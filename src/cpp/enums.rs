use rustc_middle::{
    mir::ConstValue,
    ty::{AdtDef, List, Ty, TyCtxt, VariantDiscr},
};

use crate::cpp::{
    fields::{get_field_offsets, get_variant_at_index},
    typ::{get_type, get_type_hash},
};

use super::{
    function::FunctionContext,
    project::Project,
    typ::{Type, TypeVal},
};

pub enum EnumVariant {
    Tag,
    Fields(Vec<(TypeVal, Option<usize>)>),
}

pub struct Enum {
    pub name: String,
    pub variants: Vec<(u128, EnumVariant)>,
    pub size: u64,
    pub alignment: u64,
}

pub fn get_enum_type<'tcx>(
    ty: Ty<'tcx>,
    def: AdtDef<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    tcx: TyCtxt<'tcx>,
    generics: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    instance: rustc_middle::ty::Instance<'tcx>,
    project: &mut Project,
) -> Type {
    let hash = tcx.type_id_hash(ty).as_u128();
    if project.enums.contains_key(&hash) {
        return Type::Enum(hash);
    }

    let layout = fn_ctx.layout_of(ty);
    let mut variants = vec![];
    for (idx, variant) in def.variants().iter_enumerated() {
        let discriminant = parse_discriminant(&variant.discr, tcx);

        if variant.fields.is_empty() {
            variants.push((discriminant, EnumVariant::Tag));
            continue;
        }

        let variant_layout = get_variant_at_index(idx, (*layout.layout.0).clone());
        let field_offsets = get_field_offsets(&variant_layout.fields);

        let mut fields = vec![];
        for (field, offset) in variant.fields.iter().zip(field_offsets) {
            let field_ty = fn_ctx.monomorphize(field.ty(tcx, generics));
            let field_type = get_type(project, tcx, instance, fn_ctx, field_ty);
            let field_hash = get_type_hash(tcx, field_ty);
            if field_type == Type::Void {
                continue;
            }
            fields.push((
                TypeVal {
                    hash: field_hash,
                    ty: field_type,
                    debug: Some(format!("{:?}", field_ty)),
                },
                Some(offset),
            ));
        }
        variants.push((discriminant, EnumVariant::Fields(fields)));
    }

    let data = Enum {
        name: get_enum_name(hash),
        variants,
        size: layout.layout.size().bytes(),
        alignment: layout.layout.align().abi.bytes(),
    };

    project.enums.insert(hash, data);

    Type::Enum(hash)
}

fn parse_discriminant<'tcx>(val: &VariantDiscr, tcx: TyCtxt<'tcx>) -> u128 {
    match val {
        VariantDiscr::Explicit(val) => {
            let val = tcx.const_eval_poly(*val).unwrap();
            if let ConstValue::Scalar(scalar) = val {
                scalar.to_bits(scalar.size()).unwrap()
            } else {
                panic!("Failed to parse discriminant value: {:?}", val);
            }
        }
        VariantDiscr::Relative(index) => *index as u128,
    }
}

pub fn get_enum_name(hash: u128) -> String {
    format!("enum_{hash:x}")
}

pub fn get_enum_variant_name(hash: u128, discriminant: u128) -> String {
    format!("enum_{hash:x}_{discriminant:x}")
}
