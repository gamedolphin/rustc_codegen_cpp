use std::hash::{DefaultHasher, Hash, Hasher};

use rustc_abi::{CanonAbi, ExternAbi, TagEncoding, Variants};
use rustc_middle::{
    mir::{Body, Statement, StatementKind},
    ty::{
        layout::HasTypingEnv, EarlyBinder, GenericArgsRef, Instance, List, PseudoCanonicalInput,
        SymbolName, TyCtxt, TyKind, TypingEnv,
    },
};

use crate::cpp::{
    inbuilts::get_inbuilt_functions,
    statements::{get_line, get_terminator},
    typ::{get_type, get_type_hash, TypeVal},
};

use super::{project::Project, statements::Line, typ::Type};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<TypeVal>,
    pub return_type: TypeVal,
}

pub struct Function {
    pub signature: FunctionSignature,
    pub locals: Vec<(usize, TypeVal)>,
    pub body: Vec<Vec<Line>>,
    pub is_closure: bool,
}

pub struct FunctionContext<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub instance: Instance<'tcx>,
    pub body: Body<'tcx>,
}

impl<'tcx, 'proj> FunctionContext<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>, body: Body<'tcx>) -> Self {
        FunctionContext {
            tcx,
            instance,
            body,
        }
    }

    pub fn monomorphize<T: rustc_middle::ty::TypeFoldable<TyCtxt<'tcx>> + Clone>(
        &self,
        ty: T,
    ) -> T {
        self.instance.instantiate_mir_and_normalize_erasing_regions(
            self.tcx,
            TypingEnv::fully_monomorphized(),
            EarlyBinder::bind(ty),
        )
    }

    pub fn layout_of(
        &self,
        ty: rustc_middle::ty::Ty<'tcx>,
    ) -> rustc_middle::ty::layout::TyAndLayout<'tcx> {
        let ty = self.monomorphize(ty);
        self.tcx
            .layout_of(PseudoCanonicalInput {
                typing_env: TypingEnv::fully_monomorphized(),
                value: ty,
            })
            .expect("Could not get type layout!")
    }

    pub fn split_last_tuple(&self) -> Result<bool, String> {
        let fn_ty = self.instance.ty(self.tcx, TypingEnv::fully_monomorphized());
        let internal_abi = match fn_ty.kind() {
            TyKind::FnDef(_, _) => fn_ty.fn_sig(self.tcx).abi(),
            TyKind::Closure(_, args) => args.as_closure().sig().abi(),
            TyKind::Coroutine(_, _) => ExternAbi::Rust,
            _ => return Err("Unsupported function type".to_string()),
        };

        let split_last_tuple = match internal_abi {
            ExternAbi::C { unwind: _ }
            | ExternAbi::Cdecl { unwind: _ }
            | ExternAbi::Rust
            | ExternAbi::RustCold
            | ExternAbi::Unadjusted
            | ExternAbi::SysV64 { unwind: _ } => false,

            ExternAbi::RustCall => true, /*Err(CodegenError::FunctionABIUnsuported(
            "\"rust_call\" ABI, used for things like clsoures, is not supported yet!",
            ))?,*/
            _ => return Err(format!("Unsupported function ABI: {:?}", internal_abi)),
        };

        Ok(split_last_tuple)
    }
}

impl<'tcx> rustc_middle::ty::layout::HasTyCtxt<'tcx> for FunctionContext<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl rustc_abi::HasDataLayout for FunctionContext<'_> {
    fn data_layout(&self) -> &rustc_abi::TargetDataLayout {
        self.tcx.data_layout()
    }
}

impl<'tcx> HasTypingEnv<'tcx> for FunctionContext<'tcx> {
    fn typing_env(&self) -> rustc_middle::ty::TypingEnv<'tcx> {
        TypingEnv::fully_monomorphized()
    }
}

pub fn add_function<'tcx>(
    project: &mut Project,
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    name: SymbolName<'tcx>,
) -> Result<(), String> {
    // if !instance.def_id().is_local() {
    //     return Ok(()); // skip non-local instances
    // }

    let ty = instance.ty(tcx, TypingEnv::fully_monomorphized());
    let kind = ty.kind();

    if !matches!(
        kind,
        TyKind::FnDef(..) | TyKind::Closure(..) | TyKind::Coroutine(..)
    ) {
        // nota function, skip
        return Ok(());
    }

    let name = get_name_from_symbol(name);

    let is_closure = matches!(kind, TyKind::Closure(..));
    let mir = tcx.instance_mir(instance.def);
    let ctx = FunctionContext::new(tcx, instance, mir.clone());

    // TODO: check if fn is magic

    let fn_abi = tcx
        .fn_abi_of_instance(PseudoCanonicalInput {
            typing_env: TypingEnv::fully_monomorphized(),
            value: (instance, List::empty()),
        })
        .map_err(|_| "Failed to get function ABI".to_string())?;

    if !matches!(
        fn_abi.conv,
        CanonAbi::C | CanonAbi::Rust | CanonAbi::X86(..)
    ) {
        // not a C or Rust ABI, skip
        return Err(format!("Unsupported ABI: {:?}", fn_abi.conv));
    }

    let return_type = get_type(project, tcx, instance, &ctx, fn_abi.ret.layout.ty);
    let return_hash = get_type_hash(tcx, fn_abi.ret.layout.ty);
    let return_type = TypeVal {
        hash: return_hash,
        ty: return_type,
        debug: Some(format!("{:?}", fn_abi.ret.layout.ty)),
    };
    let fn_args: Vec<_> = fn_abi
        .args
        .iter()
        .map(|arg| {
            let ty = ctx.monomorphize(arg.layout.ty);
            let ty_hash = get_type_hash(tcx, ty);
            let ty = get_type(project, tcx, instance, &ctx, ty);
            TypeVal {
                hash: ty_hash,
                ty,
                debug: Some(format!("{:?}", arg.layout.ty)),
            }
        })
        .collect();

    let signature = FunctionSignature {
        name: name.clone(),
        args: fn_args,
        return_type,
    };

    let locals_iter = mir
        .local_decls
        .iter_enumerated()
        .filter(|(id, _)| id.as_usize() == 0 || id.as_usize() > mir.arg_count);

    let mut locals = Vec::new();
    for (index, local) in locals_iter {
        let ty = ctx.monomorphize(local.ty);
        let ty = get_type(project, tcx, instance, &ctx, ty);
        let ty_hash = get_type_hash(tcx, local.ty);
        locals.push((
            index.as_usize(),
            TypeVal {
                hash: ty_hash,
                ty,
                debug: Some(format!("{:?}", local.ty)),
            },
        ));
    }

    if let Some(spread_arg) = mir.spread_arg {
        let spread_arg_ty = ctx.monomorphize(mir.local_decls[spread_arg].ty);
        let ty = get_type(project, tcx, instance, &ctx, spread_arg_ty);
        let ty_hash = get_type_hash(tcx, spread_arg_ty);

        locals.push((
            spread_arg.as_usize(),
            TypeVal {
                hash: ty_hash,
                ty,
                debug: Some(format!(
                    "{:?} (at {})",
                    spread_arg_ty,
                    spread_arg.as_usize()
                )),
            },
        ));
    }

    let blocks = get_lines(&mir, tcx, project, &ctx);

    let func = Function {
        signature,
        locals,
        body: blocks,
        is_closure,
    };

    project.functions.insert(name, func);

    Ok(())
}

pub fn get_lines<'tcx>(
    mir: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    project: &mut Project,
    ctx: &FunctionContext<'tcx>,
) -> Vec<Vec<Line>> {
    let mut blocks = Vec::new();

    for (id, block) in mir.basic_blocks.iter_enumerated() {
        let mut statements = Vec::new();
        for statement in &block.statements {
            let stmts = get_line(statement, tcx, &ctx, project);

            if stmts.iter().any(|line| !matches!(line, Line::Skip(_))) {
                statements.push(Line::Todo(format!("Debug: {:?}", statement.kind)));
            }
            statements.extend(stmts);
        }

        statements.extend(get_terminator(block.terminator(), ctx, project));

        blocks.push(statements);
    }

    blocks
}

pub fn get_fn_name_from_def_id<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: rustc_span::def_id::DefId,
    args: GenericArgsRef<'tcx>,
) -> String {
    if let Some(instance) =
        Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), def_id, args)
            .expect("Failed to resolve instance")
    {
        return get_fn_name(tcx, &instance);
    }
    let mut state = DefaultHasher::new();
    def_id.hash(&mut state);
    args.hash(&mut state);
    format!("fn_{:x}", state.finish())
    // let env = rustc_middle::ty::TypingEnv::fully_monomorphized();
    // let instance = tcx
    //     .resolve_instance_raw(rustc_middle::ty::PseudoCanonicalInput {
    //         typing_env: env,
    //         value: (def_id, args),
    //     })
    //     .expect("Failed to resolve instance")
    //     .expect("Instance resolution failed");
    // return get_fn_name(tcx, &instance);
}

pub fn get_fn_name<'tcx>(tcx: TyCtxt<'tcx>, instance: &Instance<'tcx>) -> String {
    let symbol_name = tcx.symbol_name(*instance);
    symbol_name
        .to_string()
        .replace("::", "_")
        .replace(".", "_")
        .replace("$", "_")
}

pub fn get_name_from_symbol<'tcx>(symbol: SymbolName<'tcx>) -> String {
    // Remove the leading "fn" and trailing "()" from the symbol name
    symbol
        .to_string()
        .replace("::", "_")
        .replace(".", "_")
        .replace("$", "_")
}

pub fn get_main_function_name(tcx: TyCtxt<'_>) -> Option<String> {
    tcx.entry_fn(()).map(|(def_id, _)| {
        let instance = Instance::mono(tcx, def_id);
        get_fn_name(tcx, &instance)
    })
}
