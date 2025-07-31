use rustc_abi::FieldIdx;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{AggregateKind, Operand},
    ty::{AdtDef, GenericArg, List, PseudoCanonicalInput, Ty, TypingEnv},
};
use rustc_type_ir::TyKind;

use crate::cpp::{
    enums::parse_discriminant,
    ops::translate_operand,
    typ::{get_type, get_type_hash, Type},
};

use super::{function::FunctionContext, ops::Op, project::Project, value::Value};

pub fn translate_aggregate<'tcx>(
    kind: &AggregateKind<'tcx>,
    value_index: &IndexVec<FieldIdx, Operand<'tcx>>,
    fn_ctx: &FunctionContext<'tcx>,
    proj: &mut Project,
) -> Value {
    let values = value_index
        .iter()
        .enumerate()
        .map(|(idx, op)| (idx, translate_operand(op, fn_ctx, proj)))
        .collect::<Vec<_>>();

    match kind {
        AggregateKind::Adt(def_id, variant_idx, raw_list, _, field_idx) => {
            let subst = fn_ctx.monomorphize(*raw_list);
            let adt_def = fn_ctx
                .tcx
                .resolve_instance_raw(PseudoCanonicalInput {
                    typing_env: rustc_middle::ty::TypingEnv::fully_monomorphized(),
                    value: (*def_id, subst),
                })
                .unwrap()
                .unwrap();
            let adt_type = adt_def.ty(fn_ctx.tcx, TypingEnv::fully_monomorphized());
            let adt_type = fn_ctx.monomorphize(adt_type);
            let TyKind::Adt(adt_def, subst) = adt_type.kind() else {
                panic!("Expected ADT type, found: {:?}", adt_type);
            };

            return generate_adt(
                *adt_def,
                adt_type,
                subst,
                variant_idx.as_u32(),
                values,
                *field_idx,
                fn_ctx,
                proj,
            );
        }
        AggregateKind::Tuple => Value::Tuple(values),
        AggregateKind::Array(_) => Value::Array(values),
        AggregateKind::Closure(def_id, raw_list) => {
            let subst = fn_ctx.monomorphize(*raw_list);
            let closure_def = fn_ctx
                .tcx
                .resolve_instance_raw(PseudoCanonicalInput {
                    typing_env: TypingEnv::fully_monomorphized(),
                    value: (*def_id, subst),
                })
                .unwrap()
                .unwrap();
            let closure_type = closure_def.ty(fn_ctx.tcx, TypingEnv::fully_monomorphized());
            let closure_type = fn_ctx.monomorphize(closure_type);
            let TyKind::Closure(..) = closure_type.kind() else {
                panic!("Expected Closure type, found: {:?}", closure_type);
            };
            let hash = get_type_hash(fn_ctx.tcx, closure_type);
            Value::Closure(hash, values)
        }
        _ => Value::Todo(format!("Unsupported aggregate kind: {:?}", kind)),
        // AggregateKind::Coroutine(def_id, raw_list) => todo!(),
        // AggregateKind::CoroutineClosure(def_id, raw_list) => todo!(),
        // AggregateKind::RawPtr(ty, mutability) => todo!(),
    }
}

fn generate_adt<'tcx>(
    adt: AdtDef<'tcx>,
    adt_type: Ty<'tcx>,
    subst: &'tcx List<GenericArg<'tcx>>,
    variant_idx: u32,
    fields: Vec<(usize, Op)>,
    active_field: Option<FieldIdx>,
    fn_ctx: &FunctionContext<'tcx>,
    proj: &mut Project,
) -> Value {
    let ty = fn_ctx.monomorphize(adt_type);

    if adt.repr().transparent() {
        let field = fields
            .get(0)
            .expect("Transparent ADT should have at least one field");
        return Value::Use(field.1.clone());
    }

    let hash = get_type_hash(fn_ctx.tcx, ty);

    match adt.adt_kind() {
        rustc_middle::ty::AdtKind::Struct => Value::Strukt(hash, fields),
        rustc_middle::ty::AdtKind::Union => {
            Value::Todo(format!("Union type not implemented: {:?}", adt_type))
        }
        rustc_middle::ty::AdtKind::Enum => {
            let discr = adt.variant(variant_idx.into()).discr;
            let discr = parse_discriminant(&discr, fn_ctx.tcx);
            Value::Enum(hash, discr, fields)
        }
    }
}
