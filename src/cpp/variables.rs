use rustc_middle::{
    mir::{Body, Place, ProjectionElem},
    ty::{Instance, TyCtxt, TyKind},
};

use crate::cpp::fields::get_field_name;

use super::{
    function::FunctionContext,
    project::Project,
    typ::{get_type, Type},
};

pub fn get_variable_type<'tcx>(
    place: &Place<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Type {
    let place_ty = place.ty(&fn_ctx.body, fn_ctx.tcx).ty;

    if let Some(_) = place.as_local() {
        return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, place_ty);
    }

    let (p, r) = place.iter_projections().last().unwrap();
    let pty = p.ty(&fn_ctx.body, fn_ctx.tcx).ty;
    match r {
        ProjectionElem::Field(field_idx, ty) => {
            let field_index = field_idx.as_usize();
            if field_index != 0 {
                return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty);
            }

            let binder = if let TyKind::Closure(_, args) = pty.kind() {
                args.as_closure().sig()
            } else if pty.is_fn_ptr() || pty.is_fn() {
                pty.fn_sig(fn_ctx.tcx)
            } else {
                // neither, just return the type
                return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty);
            };

            let sig = fn_ctx.tcx.normalize_erasing_late_bound_regions(
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
                binder,
            );
            let output_ty = fn_ctx.monomorphize(sig.output());
            return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, output_ty);
        }
        ProjectionElem::Index(_)
        | ProjectionElem::ConstantIndex { .. }
        | ProjectionElem::Subslice { .. } => {}
        _ => {
            let elem_ty = place_ty.builtin_index().unwrap();
            return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, elem_ty);
        }
    }

    return get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, place_ty);
}

pub fn get_variable<'tcx>(place: &Place<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> String {
    let place_ty = place.ty(&fn_ctx.body, fn_ctx.tcx);
    let place_ty = place_ty.ty;

    if let Some(local_index) = place.as_local() {
        return format!("_{} /*{:?}*/", local_index.index(), place_ty);
    }

    let mut output = String::new();
    place.iter_projections().for_each(|(p, r)| {
        if let Some(local_index) = p.as_local() {
            output.push_str(&format!("_{}", local_index.index()));
        }

        let pty = p.ty(&fn_ctx.body, fn_ctx.tcx).ty;
        let layout = fn_ctx.layout_of(pty);
        match r {
            ProjectionElem::Deref => output = format!("*({output})"),
            ProjectionElem::Field(field_idx, _) => {
                let field_index = field_idx.as_usize();
                if field_index == 0 && (pty.is_closure() || pty.is_fn()) {
                    // this is a return value.. probably from a function call
                    output = format!("({output})(/* TODO: function args */)");

                    return;
                }

                if layout.is_tuple::<FunctionContext>() {
                    output = format!("std::get<{field_index}>({output})");
                    return;
                }

                // if let TyKind::Tuple(_) = pty.kind() {
                //     output.push_str(&format!(".{}", field_index));
                //     return;
                // }

                let field_name = get_field_name(field_index);
                output.push_str(&format!(".{field_name}"));
            }
            ProjectionElem::Index(local) => {
                output.push_str(&format!("[{}]", local.index()));
            }
            ProjectionElem::ConstantIndex {
                offset,
                min_length,
                from_end,
            } => output.push_str(&format!("[{}..{}]", offset, min_length)),
            ProjectionElem::Subslice { from, to, from_end } => {
                // use std::slice here instead
                output.push_str(&format!("[{}..{}]", from, to));
                if from_end {
                    output.push_str(" (from end)");
                }
            }
            _ => {
                output.push_str(&format!(".unhandled-{r:?}"));
            }
        }
    });

    output
}
