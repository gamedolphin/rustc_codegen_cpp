use rustc_middle::mir::Rvalue;

use crate::cpp::{dyad::translate_binary_op, ops::translate_operand};

use super::{dyad::Dyad, function::FunctionContext, ops::Op, project::Project};

pub enum Value {
    Use(Op),
    BinaryOp(Dyad),
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
        _ => Value::Todo(format!("{:?}", rvalue)),
        // Rvalue::Repeat(operand, _) => todo!(),
        // Rvalue::Ref(region, borrow_kind, place) => todo!(),
        // Rvalue::ThreadLocalRef(def_id) => todo!(),
        // Rvalue::RawPtr(raw_ptr_kind, place) => todo!(),
        // Rvalue::Len(place) => todo!(),
        // Rvalue::Cast(cast_kind, operand, ty) => todo!(),
        // Rvalue::NullaryOp(null_op, ty) => todo!(),
        // Rvalue::UnaryOp(un_op, operand) => todo!(),
        // Rvalue::Discriminant(place) => todo!(),
        // Rvalue::Aggregate(aggregate_kind, index_vec) => todo!(),
        // Rvalue::ShallowInitBox(operand, ty) => todo!(),
        // Rvalue::CopyForDeref(place) => todo!(),
        // Rvalue::WrapUnsafeBinder(operand, ty) => todo!(),
    }
}
