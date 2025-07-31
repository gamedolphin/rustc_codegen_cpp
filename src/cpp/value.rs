use rustc_middle::{
    mir::{CastKind, Operand, Place, Rvalue},
    ty::{Instance, Ty, TyKind},
};

use crate::cpp::{
    aggregate::translate_aggregate,
    dyad::translate_binary_op,
    function::{add_function, get_fn_name, get_fn_name_from_def_id},
    ops::translate_operand,
    typ::{get_type, get_type_hash, IntType, Type, UIntType},
    variables::get_variable_type,
};

use super::{
    dyad::Dyad, function::FunctionContext, ops::Op, project::Project, typ::TypeVal,
    variables::get_variable,
};

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Value {
    Use(Op),
    BinaryOp(Dyad),
    Reference(String),
    Dereference(Op),
    CastTo(Op, Type),
    Reinterpret(Op, Type),
    Strukt(u128, Vec<(usize, Op)>),
    Enum(u128, u128, Vec<(usize, Op)>),
    Closure(u128, Vec<(usize, Op)>),
    Tuple(Vec<(usize, Op)>),
    Array(Vec<(usize, Op)>),
    FnPtr(String),
    FatPtr(u128, Op),
    Todo(String),
}

pub fn translate_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Value {
    match rvalue {
        Rvalue::Use(operand) => Value::Use(translate_operand(operand, fn_ctx, project)),
        Rvalue::BinaryOp(bin_op, ops) => translate_binary_op(bin_op, ops, fn_ctx, project),
        Rvalue::RawPtr(_, place) | Rvalue::Ref(_, _, place) => {
            translate_ref(place, fn_ctx, project)
        }
        Rvalue::Cast(kind, op, ty) => translate_cast(kind, op, ty, fn_ctx, project),
        Rvalue::Aggregate(aggregate_kind, index_vec) => {
            translate_aggregate(aggregate_kind, index_vec, fn_ctx, project)
        }
        Rvalue::Repeat(operand, _) => {
            Value::Todo(format!("Repeat rvalue not implemented: {:?}", operand))
        }
        Rvalue::ThreadLocalRef(def_id) => {
            Value::Todo(format!("ThreadLocalRef not implemented: {:?}", def_id))
        }
        Rvalue::Len(place) => Value::Todo(format!("Len rvalue not implemented: {:?}", place)),
        Rvalue::NullaryOp(null_op, ty) => Value::Todo(format!(
            "NullaryOp {:?} with type {:?} not implemented",
            null_op, ty
        )),
        Rvalue::UnaryOp(un_op, operand) => Value::Todo(format!(
            "UnaryOp {:?} with operand {:?} not implemented",
            un_op, operand
        )),
        Rvalue::Discriminant(place) => {
            Value::Todo(format!("Discriminant rvalue not implemented: {:?}", place))
        }
        Rvalue::ShallowInitBox(operand, ty) => Value::Todo(format!(
            "ShallowInitBox with operand {:?} and type {:?} not implemented",
            operand, ty
        )),
        Rvalue::CopyForDeref(place) => {
            let typ = get_variable_type(place, fn_ctx, project);
            Value::Use(Op::Copy(get_variable(place, fn_ctx), typ))
        }
        Rvalue::WrapUnsafeBinder(operand, ty) => Value::Todo(format!(
            "WrapUnsafeBinder with operand {:?} and type {:?} not implemented",
            operand, ty
        )),
    }
}

fn translate_cast<'tcx>(
    kind: &CastKind,
    op: &Operand<'tcx>,
    ty: &Ty<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Value {
    let operand = translate_operand(op, fn_ctx, project);
    let ty = fn_ctx.monomorphize(*ty);
    match kind {
        CastKind::PointerExposeProvenance => {
            let ty = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty);
            match ty {
                Type::UInt(UIntType::Usize)
                | Type::Int(IntType::Isize)
                | Type::FnPtr(_)
                | Type::RawPtr(_, _) => Value::CastTo(operand, ty),
                Type::Int(IntType::I64) | Type::UInt(UIntType::U64) => {
                    Value::CastTo(operand, Type::UInt(UIntType::U64))
                }
                _ => todo!("Unsupported cast kind: PointerExposeProvenance"),
            }
        }
        CastKind::PtrToPtr => Value::CastTo(
            operand,
            get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty),
        ),
        CastKind::IntToInt => Value::CastTo(
            operand,
            get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty),
        ),
        CastKind::PointerCoercion(pointer_coercion, _coercion_source) => match pointer_coercion {
            rustc_middle::ty::adjustment::PointerCoercion::ReifyFnPointer => {
                let operand_ty = op.ty(&fn_ctx.body, fn_ctx.tcx);
                op.constant()
                    .expect("Expected constant function for reify_fn_pointer");
                let operand_ty = fn_ctx.monomorphize(operand_ty);
                let (instance, subst) = if let TyKind::FnDef(def_id, subst) = operand_ty.kind() {
                    let subst = fn_ctx.monomorphize(*subst);
                    let env = rustc_middle::ty::TypingEnv::fully_monomorphized();
                    let Some(instance) = fn_ctx
                        .tcx
                        .resolve_instance_raw(rustc_middle::ty::PseudoCanonicalInput {
                            typing_env: env,
                            value: (*def_id, subst),
                        })
                        .expect("invalid function def")
                    else {
                        todo!("Could not resolve instance for reify_fn_pointer");
                    };

                    (instance, subst)
                } else {
                    todo!(
                        "Expected FnDef type for reify_fn_pointer, found: {:?}",
                        operand_ty
                    );
                };

                let fn_name = get_fn_name_from_def_id(fn_ctx.tcx, instance.def_id(), subst);

                if !instance.def_id().is_local() {
                    let type_hash = get_type_hash(fn_ctx.tcx, ty);
                    project.add_external_function(type_hash, fn_name.clone());
                }

                return Value::FnPtr(fn_name);
            }
            rustc_middle::ty::adjustment::PointerCoercion::UnsafeFnPointer => Value::Use(operand),
            rustc_middle::ty::adjustment::PointerCoercion::ClosureFnPointer(_) => {
                let op_ty = fn_ctx.monomorphize(op.ty(&fn_ctx.body, fn_ctx.tcx));
                if let TyKind::Closure(def_id, subst) = op_ty.kind() {
                    let instance = Instance::resolve_closure(
                        fn_ctx.tcx,
                        *def_id,
                        subst,
                        rustc_middle::ty::ClosureKind::FnOnce,
                    );
                    let fn_name = get_fn_name_from_def_id(fn_ctx.tcx, instance.def_id(), subst);
                    return Value::FnPtr(fn_name);
                } else {
                    panic!(
                        "Expected Closure type for ClosureFnPointer, found: {:?}",
                        op_ty
                    );
                }
            }
            rustc_middle::ty::adjustment::PointerCoercion::MutToConstPointer
            | rustc_middle::ty::adjustment::PointerCoercion::ArrayToPointer => Value::CastTo(
                operand,
                get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty),
            ),

            rustc_middle::ty::adjustment::PointerCoercion::Unsize => {
                let source = op.ty(&fn_ctx.body.local_decls, fn_ctx.tcx);
                let source = fn_ctx.monomorphize(source);
                let source = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, source);
                let target = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty);

                match &target {
                    Type::Span(..) => {
                        // just derefence the source
                        return Value::Dereference(operand);
                    }
                    Type::Dynamic => {
                        if let Type::RawPtr(hash, _) = source {
                            let inner = project
                                .typs
                                .get(&hash)
                                .expect("Expected type to be in project types");
                            if let Type::Closure(_) = inner.ty {
                                return Value::FatPtr(hash, operand);
                            }
                        }
                    }
                    _ => {}
                };

                return Value::Todo(format!(
                    "Unsize cast not implemented for source: {:?}, target: {:?}",
                    source.get_mangled(),
                    target.get_mangled()
                ));
            }
            rustc_middle::ty::adjustment::PointerCoercion::DynStar => {
                Value::Todo(format!("DynStar cast not implemented for type: {:?}", ty))
            }
        },
        CastKind::PointerWithExposedProvenance => Value::Todo(format!(
            "PointerWithExposedProvenance cast not implemented for type: {:?}",
            ty
        )),
        CastKind::Transmute => {
            let dst = fn_ctx.monomorphize(ty);
            let src = op.ty(&fn_ctx.body.local_decls, fn_ctx.tcx);
            let src = fn_ctx.monomorphize(src);
            let dst = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, dst);
            let src = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, src);

            match (&src, &dst) {
                (
                    Type::Int(IntType::Isize)
                    | Type::UInt(UIntType::Usize)
                    | Type::RawPtr(..)
                    | Type::FnPtr(_),
                    Type::Int(IntType::Isize)
                    | Type::UInt(UIntType::Usize)
                    | Type::RawPtr(..)
                    | Type::FnPtr(_),
                ) => Value::CastTo(translate_operand(op, fn_ctx, project), dst),

                (Type::UInt(UIntType::U8), Type::Char) => {
                    Value::Use(translate_operand(op, fn_ctx, project))
                }
                (_, _) => Value::Reinterpret(translate_operand(op, fn_ctx, project), dst),
            }
        }
        _ => Value::Todo(format!(
            "Unsupported cast kind: {:?}, {:?}, {:?}",
            kind, op, ty
        )),
        // CastKind::PointerCoercionWithExposedProvenance => todo!(),
        // CastKind::PointerToPointer => todo!(),
        // CastKind::PointerToInt => todo!(),
        // CastKind::IntToPointer => todo!(),
        // CastKind::FloatToPointer => todo!(),
        // CastKind::PointerToFloat => todo!(),
        // CastKind::PointerWithExposedProvenance => todo!(),
        // CastKind::PointerCoercion(pointer_coercion, coercion_source) => todo!(),

        // CastKind::FloatToInt => todo!(),
        // CastKind::FloatToFloat => todo!(),
        // CastKind::IntToFloat => todo!(),
        // CastKind::PtrToPtr => todo!(),
        // CastKind::FnPtrToPtr => todo!(),
    }
}

fn translate_ref<'tcx>(
    place: &Place<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &Project,
) -> Value {
    let var = get_variable(place, fn_ctx);

    Value::Reference(var)
}
