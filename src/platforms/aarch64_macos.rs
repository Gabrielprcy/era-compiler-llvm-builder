//!
//! The zkEVM LLVM arm64 `macos-aarch64` builder.
//!

use std::collections::HashSet;
use std::process::Command;

use crate::build_type::BuildType;
use crate::llvm_path::LLVMPath;
use crate::platforms::Platform;

///
/// The building sequence.
///
pub fn build(
    build_type: BuildType,
    targets: HashSet<Platform>,
    enable_tests: bool,
    enable_coverage: bool,
    extra_args: Vec<String>,
    use_ccache: bool,
    enable_assertions: bool,
) -> anyhow::Result<()> {
    crate::utils::check_presence("cmake")?;
    crate::utils::check_presence("ninja")?;

    let llvm_module_llvm = LLVMPath::llvm_module_llvm()?;
    let llvm_build_final = LLVMPath::llvm_build_final()?;
    let llvm_target_final = LLVMPath::llvm_target_final()?;

    crate::utils::command(
        Command::new("cmake")
            .args([
                "-S",
                llvm_module_llvm.to_string_lossy().as_ref(),
                "-B",
                llvm_build_final.to_string_lossy().as_ref(),
                "-G",
                "Ninja",
                format!(
                    "-DCMAKE_INSTALL_PREFIX='{}'",
                    llvm_target_final.to_string_lossy().as_ref(),
                )
                .as_str(),
                format!("-DCMAKE_BUILD_TYPE='{build_type}'").as_str(),
                format!(
                    "-DLLVM_TARGETS_TO_BUILD='{}'",
                    targets
                        .into_iter()
                        .map(|platform| platform.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                )
                .as_str(),
                "-DLLVM_ENABLE_PROJECTS='lld'",
                "-DCMAKE_OSX_DEPLOYMENT_TARGET='11.0'",
            ])
            .args(crate::platforms::shared::shared_build_opts_tests(
                enable_tests,
            ))
            .args(crate::platforms::shared::shared_build_opts_coverage(
                enable_coverage,
            ))
            .args(crate::platforms::shared::SHARED_BUILD_OPTS)
            .args(crate::platforms::shared::SHARED_BUILD_OPTS_NOT_MUSL)
            .args(extra_args)
            .args(crate::platforms::shared::shared_build_opts_ccache(
                use_ccache,
            ))
            .args(crate::platforms::shared::shared_build_opts_assertions(
                enable_assertions,
            ))
            .args(crate::platforms::shared::macos_build_opts_ignore_dupicate_libs_warnings()),
        "LLVM building cmake",
    )?;

    crate::utils::ninja(llvm_build_final.as_ref())?;

    Ok(())
}
