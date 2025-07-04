#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_chains)]
#![feature(f16)]
#![feature(f128)]
#![feature(slice_as_array)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_const_eval;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_type_ir;

mod cpp;

use std::any::Any;

use cpp::{
    function::get_main_function_name,
    project::{add_item, Project},
};
use rustc_codegen_ssa::{traits::CodegenBackend, CodegenResults, CrateInfo, TargetConfig};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
};
use rustc_session::{config::OutputFilenames, Session};

pub struct CppCodegenBackend;

pub struct CppCodegenResults {
    crate_name: String,
    crate_info: CrateInfo,
    project: Project,
}

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

    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn Any> {
        let cgus = tcx.collect_and_partition_mono_items(());

        let mut project = Project {
            main: get_main_function_name(tcx),
            ..Default::default()
        };

        for cgu in cgus.codegen_units {
            for (item, _data) in cgu.items() {
                add_item(&mut project, tcx, *item).expect("Could not add item to project");
            }
        }

        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();

        Box::new(CppCodegenResults {
            crate_name,
            crate_info: CrateInfo::new(tcx, "cpp".to_string()),
            project,
        })
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn std::any::Any>,
        sess: &rustc_session::Session,
        outputs: &rustc_session::config::OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        let cpp_results = ongoing_codegen
            .downcast::<CppCodegenResults>()
            .expect("Invalid codegen results type");

        let output = cpp_results.project.output();
        let cpp_filename = format!("{}_cpp/main.cpp", cpp_results.crate_name);

        std::fs::create_dir_all(format!("{}_cpp", cpp_results.crate_name))
            .expect("Failed to create C++ output directory");
        std::fs::write(cpp_filename, output).expect("Failed to write C++ file");

        sess.dcx().note(format!(
            "Generated C++ project in: {}_cpp/",
            cpp_results.crate_name
        ));
        sess.dcx().note("To build the C++ project:");
        sess.dcx()
            .note(format!("  cd {}_cpp && ./build.sh", cpp_results.crate_name));

        let codegen_results = CodegenResults {
            modules: vec![],
            allocator_module: None,
            crate_info: cpp_results.crate_info,
        };

        (codegen_results, FxIndexMap::default())
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
