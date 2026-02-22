#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod logic;
mod max_lines_per_file;
mod max_lines_per_function;
mod no_expect;
mod no_unwrap;
mod too_many_lines;

dylint_linting::dylint_library!();

#[allow(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    max_lines_per_file::register_lints(sess, lint_store);
    max_lines_per_function::register_lints(sess, lint_store);
    no_expect::register_lints(sess, lint_store);
    no_unwrap::register_lints(sess, lint_store);
    too_many_lines::register_lints(sess, lint_store);
}

#[test]
fn ui() {
    // Unset RUSTC_WRAPPER so dylint_testing can identify real rustc invocations in
    // cargo's verbose output (the wrapper replaces rustc as the first argument).
    unsafe { std::env::remove_var("RUSTC_WRAPPER") };
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}

/// Runs the compiler in-process with our lint registered, so that check_crate
/// and register_lints execute inside the test binary and are captured by cargo-llvm-cov.
#[cfg(test)]
fn run_compiler_with_lint(source_path: &str) {
    struct LintCallbacks;
    impl rustc_driver::Callbacks for LintCallbacks {
        fn config(&mut self, config: &mut rustc_interface::interface::Config) {
            config.register_lints = Some(Box::new(|sess, lint_store| {
                register_lints(sess, lint_store);
            }));
        }
    }

    let sysroot = std::str::from_utf8(
        &std::process::Command::new("rustc")
            .args(["--print", "sysroot"])
            .output()
            .expect("rustc --print sysroot failed")
            .stdout,
    )
    .expect("invalid utf8")
    .trim()
    .to_owned();

    rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(
            &[
                "rustc".to_string(),
                "--edition=2024".to_string(),
                "--sysroot".to_string(),
                sysroot,
                source_path.to_string(),
            ],
            &mut LintCallbacks,
        );
    });
}

/// dylint_internal initializes a CONFIG_TABLE OnceLock during the first compiler run.
/// Both cases must run in the same test to avoid re-initialization across threads.
#[test]
fn lint_runs_in_process() {
    run_compiler_with_lint("examples/main.rs");
    run_compiler_with_lint("examples/too_many_lines.rs");
}
