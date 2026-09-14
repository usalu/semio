//! 🚪️ Binary entry point for `semio`; all logic lives in the `semio_framework_repo_cli` godfile.
fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(semio_framework_repo_cli::run(&argv));
}
