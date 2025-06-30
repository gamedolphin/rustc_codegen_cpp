#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_chains)]

extern crate rustc_driver;
extern crate rustc_middle;
extern crate rustc_codegen_ssa;
extern crate rustc_session;
extern crate rustc_metadata;
extern crate rustc_data_structures;
extern crate rustc_target;
extern crate rustc_span;
extern crate rustc_ast;
extern crate rustc_hir;

use rustc_codegen_ssa::{traits::CodegenBackend, CodegenResults, CrateInfo, TargetConfig};
use rustc_metadata::EncodedMetadata;
use rustc_middle::ty::TyCtxt;
use rustc_session::{config::OutputFilenames, Session};

pub struct CppCodegenBackend;

pub struct CppCodegenResults {}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(CppCodegenBackend)
}

impl CodegenBackend for CppCodegenBackend {
    // taken from codegen_clr
    fn target_config(&self, sess: &Session) -> TargetConfig {
        use rustc_span::sym;
        let target_features = if sess.target.arch == "x86_64" && sess.target.os != "none" {
            vec![
                sym::fxsr,
                sym::sse,
                sym::sse2,
                rustc_span::Symbol::intern("x87"),
            ]
        } else if sess.target.arch == "aarch64" {
            match &*sess.target.os {
                "none" => vec![],
                // On macOS the aes, sha2 and sha3 features are enabled by default and ring
                // fails to compile on macOS when they are not present.
                "macos" => vec![sym::neon, sym::aes, sym::sha2, sym::sha3],
                // AArch64 mandates Neon support
                _ => vec![sym::neon],
            }
        } else {
            vec![]
        };
        let unstable_target_features = target_features.clone();
        TargetConfig {
            target_features,
            unstable_target_features,
            has_reliable_f16: true,
            has_reliable_f16_math: true,
            has_reliable_f128: true,
            has_reliable_f128_math: true,
        }
    }

    fn locale_resource(&self) -> &'static str {
        ""
    }

    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn std::any::Any> {
        Box::new(CppCodegenResults {})
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn std::any::Any>,
        sess: &rustc_session::Session,
        outputs: &rustc_session::config::OutputFilenames,
    ) -> (CodegenResults, rustc_data_structures::fx::FxIndexMap<rustc_middle::dep_graph::WorkProductId, rustc_middle::dep_graph::WorkProduct>) {
        todo!()
    }

    fn link(
        &self,
        _sess: &Session,
        _codegen_results: CodegenResults,
        _metadata: EncodedMetadata,
        _outputs: &OutputFilenames,
    ) {
        // no linking
    }
}
