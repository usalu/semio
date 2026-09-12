//! 🚪️ Binary entry point for `semio`; all logic lives in the `semio_framework_repo_cli` godfile.
fn main() {
    std::process::exit(semio_framework_repo_cli::run(std::env::args().skip(1).collect()));
}
