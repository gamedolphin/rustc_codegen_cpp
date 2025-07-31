use rustc_middle::{
    mir::{Rvalue, Statement, StatementKind, Terminator, UnwindAction},
    ty::{Instance, TyCtxt, TyKind},
};

use crate::cpp::{
    function::get_fn_name,
    ops::{translate_operand, Op},
    typ::{get_type, get_type_hash},
    value::translate_rvalue,
    variables::{get_variable, get_variable_type},
};

use super::{
    function::FunctionContext,
    ops::{is_unint_operand, is_zero_const},
    project::Project,
    value::Value,
};

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Line {
    Assignment { lhs: String, rhs: Value },
    Skip(String),
    Todo(String),
    CallLocal(Option<String>, Op, Vec<Op>),
    CallFunc(Option<String>, String, Vec<Op>),
    GoTo(usize),
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

            let lhs = get_variable(place, fn_ctx);

            let rhs = translate_rvalue(rvalue, fn_ctx, project);
            vec![Line::Assignment { lhs, rhs }]
        }
        StatementKind::StorageDead(_) | StatementKind::StorageLive(_) => {
            // These are used for variable scope management
            vec![Line::Skip(format!("{:?}", kind))]
        }
        _ => vec![Line::Todo(format!("{:?}", kind))],
    }
}

pub fn get_terminator<'tcx>(
    terminator: &Terminator<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Vec<Line> {
    match &terminator.kind {
        rustc_middle::mir::TerminatorKind::Goto { target } => {
            vec![Line::Todo(format!("Goto to {target:?}"))]
        }
        rustc_middle::mir::TerminatorKind::SwitchInt { discr, targets } => {
            vec![Line::Todo(format!(
                "SwitchInt on {discr:?} with targets {targets:?}"
            ))]
        }
        rustc_middle::mir::TerminatorKind::UnwindResume => {
            vec![Line::Todo("UnwindResume".to_string())]
        }
        rustc_middle::mir::TerminatorKind::UnwindTerminate(unwind_terminate_reason) => {
            vec![Line::Todo(format!(
                "UnwindTerminate: {unwind_terminate_reason:?}"
            ))]
        }
        rustc_middle::mir::TerminatorKind::Return => {
            vec![Line::Todo("Return".to_string())]
        }
        rustc_middle::mir::TerminatorKind::Unreachable => {
            vec![Line::Todo("Unreachable".to_string())]
        }
        rustc_middle::mir::TerminatorKind::Drop {
            place,
            target,
            unwind,
            replace,
            drop,
            async_fut,
        } => {
            vec![Line::Todo(format!(
                "Drop: place={place:?}, target={target:?}, unwind={unwind:?}, replace={replace:?}, drop={drop:?}, async_fut={async_fut:?}"
            ))]
        }
        rustc_middle::mir::TerminatorKind::Call {
            func,
            args,
            destination,
            target,
            unwind,
            call_source,
            fn_span,
        } => {
            let op = translate_operand(func, fn_ctx, project);
            let arguments = args.iter().filter(|arg| is_unint_operand(&arg.node, fn_ctx))
                .map(|arg| translate_operand(&arg.node, fn_ctx, project)).collect::<Vec<_>>();
            let mut output = vec![
                Line::Todo(format!(
                "Call: func={func:?}, args={args:?}, destination={destination:?}, target={target:?}, unwind={unwind:?}, call_source={call_source:?}, fn_span={fn_span:?}"
            ))
            ];
            let destination_type = get_variable_type(destination, fn_ctx, project);
            let destination = if destination_type.is_void() { None } else  {Some(get_variable(&destination, fn_ctx))};
            if matches!(op, Op::Copy(..)| Op::Move(..)) {
                // If the function is a local variable, we can call it directly
                output.push(Line::CallLocal(destination, op, arguments));
            } else {
                // Otherwise, we need to translate the operand
                let op_ty = func.ty(&fn_ctx.body, fn_ctx.tcx);
                let op_ty = fn_ctx.monomorphize(op_ty);
                match op_ty.kind() {
                    TyKind::Closure(..) => {
                        output.push(Line::Todo(format!("Calling closure: {func:?}")));
                    }
                    TyKind::FnPtr(..) => {
                        output.push(Line::Todo(format!("Calling function pointer: {func:?}")));
                    }
                    TyKind::FnDef(def_id, subst) => {
                        // let env = rustc_middle::ty::TypingEnv::fully_monomorphized();
                        // let instance = fn_ctx.tcx.resolve_instance_raw(rustc_middle::ty::PseudoCanonicalInput {
                        //     typing_env: env,
                        //     value: (*def_id, subst),
                        // }).expect("Failed to resolve instance").expect("Instance resolution failed");
                        // let fn_name = get_fn_name(fn_ctx.tcx, &instance);

                        // if !instance.def_id().is_local() {
                        //     let type_hash = get_type_hash(fn_ctx.tcx, op_ty);
                        //     project.add_external_function(type_hash, fn_name.clone());
                        // }
                        // output.push(Line::CallFunc(destination, fn_name, arguments));
                    }
                    _ => {}
                };
                output.push(Line::Todo(format!("Call with operand: {func:?}")));
            }

            if let Some(target) = target {
                output.push(Line::GoTo(target.as_usize()));
            }
            output
        }
        rustc_middle::mir::TerminatorKind::TailCall {
            func,
            args,
            fn_span,
        } => vec![Line::Todo(format!(
            "TailCall: func={func:?}, args={args:?}, fn_span={fn_span:?}"
        ))],
        rustc_middle::mir::TerminatorKind::Assert {
            cond,
            expected,
            msg,
            target,
            unwind,
        } => vec![Line::Todo(format!(
            "Assert: cond={cond:?}, expected={expected:?}, msg={msg:?}, target={target:?}, unwind={unwind:?}"
        ))],
        rustc_middle::mir::TerminatorKind::Yield {
            value,
            resume,
            resume_arg,
            drop,
        } => vec![Line::Todo(format!(
            "Yield: value={value:?}, resume={resume:?}, resume_arg={resume_arg:?}, drop={drop:?}"
        ))],
        rustc_middle::mir::TerminatorKind::CoroutineDrop =>
            vec![Line::Todo("CoroutineDrop".to_string())],
        rustc_middle::mir::TerminatorKind::FalseEdge {
            real_target,
            imaginary_target,
        } => vec![Line::Todo(format!(
            "FalseEdge: real_target={real_target:?}, imaginary_target={imaginary_target:?}"
        ))],
        rustc_middle::mir::TerminatorKind::FalseUnwind {
            real_target,
            unwind,
        } => vec![Line::Todo(format!(
            "FalseUnwind: real_target={real_target:?}, unwind={unwind:?}"
        ))],
        rustc_middle::mir::TerminatorKind::InlineAsm {
            asm_macro,
            template,
            operands,
            options,
            line_spans,
            targets,
            unwind,
        } => vec![Line::Todo(format!(
            "InlineAsm: asm_macro={asm_macro:?}, template={template:?}, operands={operands:?}, options={options:?}, line_spans={line_spans:?}, targets={targets:?}, unwind={unwind:?}"
        ))],
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
