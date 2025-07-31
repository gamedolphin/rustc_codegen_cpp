use rustc_middle::ty::{layout::TyAndLayout, Instance, TyCtxt, TypingEnv};
use rustc_type_ir::ClosureArgs;

use crate::cpp::{
    fields::get_field_offsets,
    function::get_lines,
    typ::{get_type, get_type_hash},
};

use super::{
    function::{FunctionContext, FunctionSignature},
    project::Project,
    statements::Line,
    typ::{Type, TypeVal},
};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Closure {
    pub signature: FunctionSignature,
    pub fields: Vec<(TypeVal, usize)>, // (type, offset)
    pub size: usize,
    pub alignment: usize,
}

pub fn get_closure_type<'tcx>(
    hash: u128,
    name: &str,
    closure: ClosureArgs<TyCtxt<'tcx>>,
    fn_ctx: &FunctionContext<'tcx>,
    tcx: TyCtxt<'tcx>,
    project: &mut Project,
    instance: Instance<'tcx>,
    layout: TyAndLayout<'tcx>,
) -> Type {
    if project.closures.contains_key(&hash) {
        return Type::Closure(hash);
    }
    let mut sig = closure.sig();
    sig = fn_ctx.monomorphize(sig);

    let sig = tcx.normalize_erasing_late_bound_regions(TypingEnv::fully_monomorphized(), sig);
    let inputs: Vec<_> = sig
        .inputs()
        .iter()
        .map(|ty| TypeVal {
            hash: get_type_hash(tcx, *ty),
            ty: get_type(project, tcx, instance, fn_ctx, *ty),
            debug: Some(format!("{:?}", ty)),
        })
        .collect();
    let output = get_type(project, tcx, instance, fn_ctx, sig.output());
    let output_hash = get_type_hash(tcx, sig.output());
    let output = TypeVal {
        hash: output_hash,
        ty: output,
        debug: Some(format!("{:?}", sig.output())),
    };

    let fields: Vec<_> = closure
        .upvar_tys()
        .iter()
        .map(|ty| TypeVal {
            hash: get_type_hash(tcx, ty),
            ty: get_type(project, tcx, instance, fn_ctx, ty),
            debug: Some(format!("{:?}", ty)),
        })
        .collect();

    let fields_iter = get_field_offsets(&layout.layout.0.fields);

    let fields = fields
        .iter()
        .zip(fields_iter)
        .filter(|(ty, _)| !matches!(ty.ty, Type::Void))
        .map(|(ty, offset)| (ty.clone(), offset))
        .collect::<Vec<_>>();

    let closure = Closure {
        signature: FunctionSignature {
            name: name.to_string(), // closures dont have names
            args: inputs,
            return_type: output,
        },
        fields,
        size: layout.size.bytes_usize(),
        alignment: layout.align.abi.bytes_usize(),
    };

    project.add_closure(hash, closure.clone());

    Type::Closure(hash)
}
