use std::{collections::VecDeque, hash::Hash};

use rustc_hash::FxHashMap as HashMap;
use rustc_middle::ty::{self, AdtKind, Instance, Ty, TyCtxt, TyKind};
use rustc_type_ir::{FloatTy, IntTy, UintTy};

use crate::cpp::{
    closure::get_closure_type,
    enums::{get_enum_type, EnumVariant},
    fields::get_field_offsets,
    function::FunctionSignature,
    structs::get_struct_type,
};

use topological_sort::TopologicalSort;

use super::{function::FunctionContext, project::Project};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TypeVal {
    pub ty: Type,
    pub hash: u128,
}

impl TypeVal {
    pub fn get_internal_hash(&self) -> u128 {
        match self.ty {
            Type::Array(inner, _, _, _) => inner,
            Type::Span(inner, _) => inner,
            Type::RawPtr(inner, _) => inner,
            _ => self.hash,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Type {
    Void,
    Bool,
    Char,
    Int(IntType),
    UInt(UIntType),
    Float(FloatType),
    Closure(u128), // index into closure table
    Todo(String),
    String,
    StringView,
    FnPtr(u128),
    RawPtr(u128, bool),
    Struct(u128),
    Enum(u128),
    Array(u128, u64, u64, u64),
    Span(u128, bool),
    Tuple(Vec<u128>)
}

impl Type {
    pub fn get_mangled(&self) -> String {
        match self {
            Type::Void => "v".to_string(),
            Type::Bool => "b".to_string(),
            Type::Char => "c".to_string(),
            Type::Int(int_type) => match int_type {
                IntType::I8 => "i8".to_string(),
                IntType::I16 => "i16".to_string(),
                IntType::I32 => "i32".to_string(),
                IntType::I64 => "i64".to_string(),
                IntType::I128 => "i128".to_string(),
                IntType::Isize => "isize".to_string(),
            },
            Type::UInt(uint_type) => match uint_type {
                UIntType::U8 => "u8".to_string(),
                UIntType::U16 => "u16".to_string(),
                UIntType::U32 => "u32".to_string(),
                UIntType::U64 => "u64".to_string(),
                UIntType::U128 => "u128".to_string(),
                UIntType::Usize => "usize".to_string(),
            },
            Type::Float(float_type) => match float_type {
                FloatType::F16 => "f16".to_string(),
                FloatType::F32 => "f32".to_string(),
                FloatType::F64 => "f64".to_string(),
                FloatType::F128 => "f128".to_string(),
            },
            Type::Closure(index) => format!("fnclosure_{index}"),
            Type::Todo(ty) => format!("todo_{ty}"),
            Type::String => "string".to_string(),
            Type::FnPtr(index) => format!("fnptr_{index}"),
            Type::RawPtr(inner, mutability) => {
                if *mutability {
                    format!("rawptr_mut_{inner:x}")
                } else {
                    format!("rawptr_{inner:x}")
                }
            }
            Type::Struct(index) => format!("struct_{index}"),
            Type::Enum(index) => format!("enum_{index}"),
            Type::Array(inner, size, _, _) => format!("array_{inner:x}_{size}"),
            Type::StringView => format!("string_view"),
            Type::Span(inner, _) => format!("span_{inner:x}"),
            Type::Tuple(tys) => {
                let types: Vec<String> = tys.iter().map(|ty| ty.to_string()).collect();
                format!("tuple_{}", types.join("_"))
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum FloatType {
    F16,
    F32,
    F64,
    F128,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum UIntType {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl From<&FloatTy> for Type {
    fn from(float_ty: &FloatTy) -> Self {
        match float_ty {
            FloatTy::F16 => Type::Float(FloatType::F16),
            FloatTy::F32 => Type::Float(FloatType::F32),
            FloatTy::F64 => Type::Float(FloatType::F64),
            FloatTy::F128 => Type::Float(FloatType::F128),
        }
    }
}

impl From<&IntTy> for Type {
    fn from(int_ty: &IntTy) -> Self {
        match int_ty {
            IntTy::Isize => Type::Int(IntType::Isize),
            IntTy::I8 => Type::Int(IntType::I8),
            IntTy::I16 => Type::Int(IntType::I16),
            IntTy::I32 => Type::Int(IntType::I32),
            IntTy::I64 => Type::Int(IntType::I64),
            IntTy::I128 => Type::Int(IntType::I128),
        }
    }
}

impl From<&UintTy> for Type {
    fn from(uint_ty: &UintTy) -> Self {
        match uint_ty {
            UintTy::Usize => Type::UInt(UIntType::Usize),
            UintTy::U8 => Type::UInt(UIntType::U8),
            UintTy::U16 => Type::UInt(UIntType::U16),
            UintTy::U32 => Type::UInt(UIntType::U32),
            UintTy::U64 => Type::UInt(UIntType::U64),
            UintTy::U128 => Type::UInt(UIntType::U128),
        }
    }
}

pub fn get_type<'tcx>(
    project: &mut Project,
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    ty: Ty<'tcx>,
) -> Type {
    let ty = fn_ctx.monomorphize(ty);
    let layout = fn_ctx.layout_of(ty);
    if layout.is_zst() {
        return Type::Void;
    }

    let output = match ty.kind() {
        TyKind::Bound(debruijn_index, _) => Type::Void,
        TyKind::Bool => Type::Bool,
        TyKind::Char => Type::Char,
        TyKind::Int(int_ty) => Type::from(int_ty),
        TyKind::Uint(uint_ty) => Type::from(uint_ty),
        TyKind::Float(float_ty) => Type::from(float_ty),
        TyKind::Closure(def, args) => {
            let closure = args.as_closure();
            let layout = fn_ctx.layout_of(ty);
            let hash = get_type_hash(tcx, ty);
            get_closure_type(hash, closure, fn_ctx, tcx, project, instance, layout)
        }
        TyKind::FnPtr(binder, fn_header) => {
            let sig = tcx.normalize_erasing_late_bound_regions(
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
                *binder,
            );
            let output_ty = fn_ctx.monomorphize(sig.output());
            let output = TypeVal {
                hash: get_type_hash(tcx, output_ty),
                ty: get_type(project, tcx, instance, fn_ctx, output_ty),
            };
            let inputs: Vec<_> = sig
                .inputs()
                .iter()
                .map(|ty| TypeVal {
                    hash: get_type_hash(tcx, *ty),
                    ty: get_type(project, tcx, instance, fn_ctx, *ty),
                })
                .collect();
            let fn_signature = FunctionSignature {
                name: "".to_string(), // fn pointers don't have names
                args: inputs,
                return_type: output,
            };
            let hash = tcx.type_id_hash(ty).as_u128();

            project
                .function_signatures
                .insert(hash, fn_signature.clone());

            Type::FnPtr(hash)
        }
        TyKind::RawPtr(inner, mutability) | TyKind::Ref(_, inner, mutability) => {
            if is_fat_ptr(inner, tcx, fn_ctx) {
                let inner = fn_ctx.monomorphize(*inner);
                match inner.kind() {
                    TyKind::Slice(inner) => {
                        let inner_type = get_type(project, tcx, instance, fn_ctx, *inner);
                        let hash = project.add_type(tcx, *inner, inner_type);

                        return Type::Span(hash, mutability.is_mut());
                    }
                    TyKind::Str => {
                        if mutability.is_mut() {
                            return Type::String;
                        } else {
                            return Type::StringView;
                        }
                    }
                    _ => {} // the rest can just be a regular raw pointer
                }
            }

            let inner = fn_ctx.monomorphize(*inner);
            let inner_type = get_type(project, tcx, instance, fn_ctx, inner); // this adds the type to the project
            let hash = project.add_type(tcx, inner, inner_type);

            Type::RawPtr(hash, mutability.is_mut())
        }
        TyKind::Adt(def, generics) => match def.adt_kind() {
            AdtKind::Struct => get_struct_type(ty, *def, fn_ctx, tcx, generics, instance, project),
            AdtKind::Union => todo!(),
            AdtKind::Enum => get_enum_type(ty, *def, fn_ctx, tcx, generics, instance, project),
        },
        TyKind::Array(inner, length) => {
            let length = fn_ctx.monomorphize(*length);
            let length = length.try_to_target_usize(tcx).unwrap_or(0);
            let inner = fn_ctx.monomorphize(*inner);
            let inner_type = get_type(project, tcx, instance, fn_ctx, inner);

            let layout = fn_ctx.layout_of(ty);
            let arr_size = layout.layout.size.bytes();
            let arr_align = layout.layout.align.abi.bytes();
            let hash = project.add_type(tcx, inner, inner_type.clone());

            Type::Array(hash, length, arr_size, arr_align)
        }
        TyKind::Foreign(def_id) => Type::Todo(format!("Foreign: {:?}", def_id)),
        TyKind::Str => {
            if fn_ctx.monomorphize(ty).is_mutable_ptr() {
                Type::String
            } else {
                Type::StringView
            }
        } // this never happens
        TyKind::Slice(inner) => {
            let inner = fn_ctx.monomorphize(*inner);
            let inner_type = get_type(project, tcx, instance, fn_ctx, inner);
            let hash = project.add_type(tcx, inner, inner_type);
            Type::Span(hash, false) // slices are always immutable
        }
        TyKind::FnDef(..) => Type::Void,
        TyKind::Dynamic(_, _, dyn_kind) => Type::Todo(format!("Dynamic: {:?}", dyn_kind)),
        TyKind::Coroutine(def_id, gens) => {
            Type::Todo(format!("Coroutine: {:?}, gens: {:?}", def_id, gens))
        }
        TyKind::Never => Type::Void,
        TyKind::Tuple(tys) => {
            let tuple_types: Vec<_> = tys
                .iter()
                .filter_map(|ty| {
                    let ty = fn_ctx.monomorphize(ty);
                    let tyv = get_type(project, tcx, instance, fn_ctx, ty);
                    let hash = project.add_type(tcx, ty, tyv.clone());
                    Some(hash)
                })
                .collect();
            if tuple_types.is_empty() {
                Type::Void
            } else {
                Type::Tuple(tuple_types)
            }
        }
        TyKind::Alias(alias_ty_kind, alias_ty) => {
            panic!("Alias: {:?} {:?}", alias_ty_kind, alias_ty)
        }
        _ => todo!("Unhandled type: {:?}", ty.kind()),
    };

    project.add_type(tcx, ty, output.clone());

    output
}

fn is_fat_ptr<'tcx>(inner: &Ty<'tcx>, tcx: TyCtxt<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> bool {
    use rustc_abi::BackendRepr;

    let ptr = Ty::new_ptr(tcx, *inner, ty::Mutability::Mut);
    let ptr_type = fn_ctx.monomorphize(ptr);
    let layout = fn_ctx.layout_of(ptr_type).layout;
    let abi = layout.0 .0.backend_repr;
    match abi {
        BackendRepr::Scalar(_) => false,
        BackendRepr::ScalarPair(_, _) => true,
        _ => panic!("Unexpected abi of pointer to {ptr_type:?}. The ABI was:{abi:?}"),
    }
}

pub fn get_type_hash<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> u128 {
    tcx.type_id_hash(ty).as_u128()
}

pub fn cleanse_name(name: &str) -> String {
    name.replace("::", "_").replace("-", "_")
}

fn types_we_care_to_sort(val: &TypeVal) -> bool {
    matches!(
        val.ty,
        Type::Closure(_)
            | Type::Struct(_)
            | Type::Enum(_)
            | Type::FnPtr(_)
            | Type::Array(_, _, _, _)
            | Type::Span(_, _)
            | Type::RawPtr(_, _)
    )
}

pub fn sort_types(proj: &Project, types: &HashMap<u128, TypeVal>) -> Vec<(u128, Type)> {
    // make a tree of depenedencies and then sort them topologically
    // kahn's algorithm

    let mut ts = TopologicalSort::<u128>::new();
    types.iter().for_each(|(hash, _)| {
        ts.insert(*hash);
    });

    for (index, ty) in types.iter().filter(|(_, ty)| types_we_care_to_sort(ty)) {
        match ty.ty {
            Type::Closure(hash) => {
                let closure = proj.closures.get(&hash).expect("Closure not found");
                closure
                    .signature
                    .args
                    .iter()
                    .filter(|val| types_we_care_to_sort(val))
                    .for_each(|arg| {
                        ts.add_dependency(arg.get_internal_hash(), *index);
                    });
                if types_we_care_to_sort(&closure.signature.return_type) {
                    ts.add_dependency(closure.signature.return_type.get_internal_hash(), *index);
                }
                closure
                    .fields
                    .iter()
                    .filter(|(val, _)| types_we_care_to_sort(val))
                    .for_each(|(field_type, _)| {
                        ts.add_dependency(field_type.get_internal_hash(), *index);
                    });
            }
            Type::Struct(hash) => {
                let struct_def = proj.strukts.get(&hash).expect("Struct not found");
                struct_def
                    .fields
                    .iter()
                    .filter(|(val, _)| types_we_care_to_sort(val))
                    .for_each(|(field_type, _)| {
                        ts.add_dependency(field_type.get_internal_hash(), *index);
                    });
            }
            Type::Enum(hash) => {
                let enum_def = proj.enums.get(&hash).expect("Enum not found");
                enum_def.variants.iter().for_each(|variant| {
                    if let EnumVariant::Fields(fields) = &variant.1 {
                        fields
                            .iter()
                            .filter(|(val, _)| types_we_care_to_sort(val))
                            .for_each(|(field_type, _)| {
                                ts.add_dependency(field_type.get_internal_hash(), *index);
                            });
                    }
                });
            }
            Type::FnPtr(hash) => {
                if let Some(signature) = proj.function_signatures.get(&hash) {
                    signature
                        .args
                        .iter()
                        .filter(|val| types_we_care_to_sort(val))
                        .for_each(|arg| {
                            ts.add_dependency(arg.get_internal_hash(), *index);
                        });
                    if types_we_care_to_sort(&signature.return_type) {
                        ts.add_dependency(signature.return_type.get_internal_hash(), *index);
                    }
                }
            }
            Type::RawPtr(inner, _) | Type::Array(inner, ..) | Type::Span(inner, ..) => {
                if let Some(inner_type) = types.get(&inner) {
                    if types_we_care_to_sort(inner_type) {
                        ts.add_dependency(inner, *index);
                    }
                }
            }
            _ => {}
        }
    }

    let mut pop_all = ts.pop_all();
    let mut output = Vec::new();
    while !pop_all.is_empty() {
        output.extend(
            pop_all.iter().map(|v| {
                (
                    *v,
                    types
                        .get(v)
                        .unwrap_or_else(|| panic!("type not found {v:x}"))
                        .ty
                        .clone(),
                )
            }), //.filter_map(|v| types.get(v).map(|va| (*v, va.ty.clone()))),
        );

        pop_all = ts.pop_all()
    }

    output
}
