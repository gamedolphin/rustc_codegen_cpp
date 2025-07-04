use rustc_const_eval::interpret::{GlobalAlloc, Scalar};
use rustc_middle::{
    mir::{ConstValue, Operand},
    ty::{Ty, TyKind, TypingEnv},
};
use rustc_type_ir::IntTy;
use std::hash::Hash;

use crate::cpp::{
    constants::Constant,
    function::get_fn_name,
    typ::{get_type, Type, UIntType},
};

use super::{function::FunctionContext, project::Project};

pub enum Op {
    Constant(String),
    Copy(String),
    Move(String),
}

pub fn translate_operand<'tcx>(
    operand: &Operand<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    proj: &mut Project,
) -> Op {
    // let operand_ty = fn_ctx.monomorphize(operand.ty(&fn_ctx.body, fn_ctx.tcx));
    match operand {
        Operand::Copy(place) => Op::Copy(format!("Copy({:?})", place)),
        Operand::Move(place) => Op::Move(format!("Move({:?})", place)),
        Operand::Constant(val) => {
            let constant = val.const_;
            let constant = fn_ctx.monomorphize(constant);
            let evaluated = constant
                .eval(fn_ctx.tcx, TypingEnv::fully_monomorphized(), val.span)
                .expect("Could not evaluate constant!");

            let const_ty = constant.ty();

            match evaluated {
                ConstValue::Scalar(scalar) => translate_scalar(scalar, const_ty, fn_ctx, proj),
                ConstValue::Slice { data, meta } => todo!(),
                ConstValue::Indirect { alloc_id, offset } => todo!(),
                ConstValue::ZeroSized => todo!(),
            }
        }
    }
}

fn translate_scalar<'tcx>(
    val: Scalar,
    ty: Ty<'tcx>,
    fn_ctx: &FunctionContext<'tcx>,
    project: &mut Project,
) -> Op {
    let ty = fn_ctx.monomorphize(ty);
    let size = val.size();
    let val = match val {
        Scalar::Int(int) => int.to_uint(size).to_ne_bytes()[0..size.bytes() as usize].to_vec(),
        Scalar::Ptr(pointer, size) => {
            let (alloc_id, offset) = pointer.into_parts();
            let global_alloc = fn_ctx.tcx.global_alloc(alloc_id.alloc_id());

            match global_alloc {
                GlobalAlloc::Function { instance } => todo!(),
                GlobalAlloc::VTable(ty, raw_list) => todo!(),
                GlobalAlloc::Static(def_id) => todo!(),
                GlobalAlloc::Memory(const_allocation) => {
                    let const_alloc = const_allocation.inner();
                    let bytes: &[u8] = const_alloc
                        .inspect_with_uninit_and_ptr_outside_interpreter(0..const_alloc.len());
                    bytes.to_vec()
                }
            }
        }
    };

    let const_type = get_type(project, fn_ctx.tcx, fn_ctx.instance, fn_ctx, ty);
    let constant = Constant {
        typ: const_type,
        value: val,
        size: size.bytes(),
    };

    Op::Constant(project.add_constant(fn_ctx.tcx, ty, constant.clone()))
}

pub fn get_constant_name(hash: u128, value: &[u8], size: u64) -> String {
    use std::hash::{DefaultHasher, Hasher};
    let mut s = DefaultHasher::new();
    value.hash(&mut s);
    let value_hash = s.finish();
    format!("constant_{hash:x}_{value_hash:x}_{size}")
}

pub fn is_unint_operand<'tcx>(operand: &Operand<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> bool {
    match operand {
        Operand::Copy(_) | Operand::Move(_) => false,
        Operand::Constant(val) => {
            let constant = val.const_;
            let constant = fn_ctx.monomorphize(constant);
            let evaluated = constant
                .eval(fn_ctx.tcx, TypingEnv::fully_monomorphized(), val.span)
                .expect("Could not evaluate constant!");
            match evaluated {
                ConstValue::Scalar(_) => false,
                ConstValue::ZeroSized => true, // maybe not? empty structs are zero-sized
                ConstValue::Slice { data, .. } => {
                    let mask = data.inner().init_mask();
                    let mut chunks =
                        mask.range_as_init_chunks(rustc_const_eval::interpret::AllocRange {
                            start: rustc_abi::Size::ZERO,
                            size: data.0.size(),
                        });
                    let Some(only) = chunks.next() else {
                        return false;
                    };

                    if chunks.next().is_some() {
                        return false;
                    }
                    !only.is_init()
                }
                ConstValue::Indirect { alloc_id, .. } => {
                    let data = fn_ctx.tcx.global_alloc(alloc_id);
                    let GlobalAlloc::Memory(data) = data else {
                        return false;
                    };

                    let mask = data.0.init_mask();
                    let mut chunks =
                        mask.range_as_init_chunks(rustc_const_eval::interpret::AllocRange {
                            start: rustc_abi::Size::ZERO,
                            size: data.0.size(),
                        });
                    let Some(only) = chunks.next() else {
                        return false;
                    };
                    // If this is not the only chunk, then the init mask must not be fully uninitialized
                    if chunks.next().is_some() {
                        return false;
                    }
                    !only.is_init()
                }
            }
        }
    }
}

pub fn is_zero_const<'tcx>(operand: &Operand<'tcx>, fn_ctx: &FunctionContext<'tcx>) -> bool {
    match operand {
        Operand::Copy(_) | Operand::Move(_) => false,
        Operand::Constant(val) => {
            let constant = val.const_;
            let constant = fn_ctx.monomorphize(constant);
            let evaluated = constant
                .eval(fn_ctx.tcx, TypingEnv::fully_monomorphized(), val.span)
                .expect("Could not evaluate constant!");
            match evaluated {
                ConstValue::Scalar(scalar) => match scalar {
                    Scalar::Int(scalar_int) => scalar_int.is_null(),
                    Scalar::Ptr(..) => false,
                },
                ConstValue::ZeroSized => true,
                _ => false,
            }
        }
    }
}
