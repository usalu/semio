//! 🚪️ Binary entry point for `semio`; all logic lives in the `repo_cli` godfile.
fn main() {
    std::process::exit(repo_cli::run(std::env::args().skip(1).collect()));
}
