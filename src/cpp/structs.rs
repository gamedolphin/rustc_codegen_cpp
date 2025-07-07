use rustc_abi::VariantIdx;
use rustc_middle::ty::{AdtDef, List, Ty, TyCtxt};

use crate::cpp::{fields::get_field_offsets, typ::get_type};

use super::{
    function::FunctionContext,
    project::Project,
    typ::{get_type_hash, Type, TypeVal},
};

pub struct Strukt {
    pub name: String,
    pub fields: Vec<(TypeVal, Option<usize>)>,
    pub layout: u64,
    pub alignment: u64,
}

pub fn get_struct_name(hash: u128) -> String {
    format!("struct_{hash:x}")
}

pub fn get_struct_type<'tcx>(
    ty: Ty<'tcx>,
    def: AdtDef<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    tcx: TyCtxt<'tcx>,
    generics: &'tcx List<rustc_middle::ty::GenericArg<'tcx>>,
    instance: rustc_middle::ty::Instance<'tcx>,
    project: &mut Project,
) -> Type {
    let hash = tcx.type_id_hash(ty).as_u128();
    if project.strukts.contains_key(&hash) {
        return Type::Struct(hash);
    }

    let layout = fn_ctx.layout_of(ty);
    let offsets = get_field_offsets(&layout.fields);
    let fields_iter = def
        .variant(VariantIdx::from_u32(0))
        .fields
        .iter()
        .zip(offsets);

    let mut fields = Vec::new();
    for (field, offset) in fields_iter {
        let field_ty = fn_ctx.monomorphize(field.ty(tcx, generics));
        let field_type = get_type(project, tcx, instance, fn_ctx, field_ty);
        let field_hash = get_type_hash(tcx, field_ty);
        if field_type == Type::Void {
            continue;
        }
        fields.push((
            TypeVal {
                ty: field_type,
                hash: field_hash,
                debug: Some(format!("{:?}", field_ty)),
            },
            Some(offset),
        ));
    }

    let size = layout.layout.size().bytes();
    let alignment = layout.layout.align().abi.bytes();

    project.strukts.insert(
        hash,
        Strukt {
            name: get_struct_name(hash),
            fields,
            layout: size,
            alignment,
        },
    );

    Type::Struct(hash)
}
