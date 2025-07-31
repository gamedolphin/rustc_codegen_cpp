use rustc_middle::mir::{BinOp, Operand};

use crate::cpp::ops::translate_operand;

use super::{function::FunctionContext, ops::Op, project::Project, value::Value};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Dyad {
    pub operation: BinOp,
    pub left: Op,
    pub right: Op,
}

pub fn translate_binary_op<'tcx>(
    bin_op: &BinOp,
    ops: &Box<(Operand<'tcx>, Operand<'tcx>)>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Value {
    let left = translate_operand(&ops.0, fn_ctx, project);
    let right = translate_operand(&ops.1, fn_ctx, project);

    // let ty_a = ops.0.ty(&fn_ctx.body, fn_ctx.tcx);
    // let ty_b = ops.1.ty(&fn_ctx.body, fn_ctx.tcx);

    Value::BinaryOp(Dyad {
        operation: *bin_op,
        left,
        right,
    })
}
