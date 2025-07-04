use rustc_middle::{
    mir::{Rvalue, Statement, StatementKind},
    ty::TyCtxt,
};

use crate::cpp::{value::translate_rvalue, variables::get_variable};

use super::{
    function::FunctionContext,
    ops::{is_unint_operand, is_zero_const},
    project::Project,
    value::Value,
};

pub enum Line {
    Assignment { lhs: String, rhs: Value },
    Todo(String),
}

pub fn get_line<'tcx>(
    stmt: &Statement<'tcx>,
    tcx: TyCtxt<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Vec<Line> {
    let kind = &stmt.kind;
    match kind {
        StatementKind::Assign(place_rvalue) => {
            let place = &place_rvalue.0;
            let rvalue = &place_rvalue.1;

            if rvalue_is_unint(rvalue, fn_ctx) {
                return vec![Line::Todo(format!("Uninitialized value: {stmt:?}"))];
            }

            let ty = fn_ctx.monomorphize(place.ty(&fn_ctx.body, tcx).ty);
            let layout = fn_ctx.layout_of(ty);
            if layout.is_zst() {
                return vec![Line::Todo(format!("zero sized type assignmenet: {stmt:?}"))];
            }

            let lhs = get_variable(place);

            let rhs = translate_rvalue(rvalue, fn_ctx, project);
            vec![Line::Assignment { lhs, rhs }]
        }
        _ => vec![Line::Todo(format!("{:?}", kind))],
    }
}

fn rvalue_is_const_0<'tcx>(rvalue: &Rvalue<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> bool {
    match rvalue {
        Rvalue::Use(operand) | Rvalue::Repeat(operand, _) => is_zero_const(operand, fn_ctx),
        _ => false,
    }
}

fn rvalue_is_unint<'tcx>(rvalue: &Rvalue<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> bool {
    match rvalue {
        Rvalue::Repeat(operand, _) | Rvalue::Use(operand) => is_unint_operand(operand, fn_ctx),
        _ => false,
    }
}
