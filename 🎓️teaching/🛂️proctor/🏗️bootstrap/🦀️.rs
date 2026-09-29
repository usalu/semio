//! 🏗️ The `proctor` process: one multi-threaded runtime, the command line and the process
//! environment handed to `proctor::cli::run`.
//!
//! @see ../🔨️modules/⌨️cli/🦀️.rs — the commands

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(runtime) => runtime.block_on(proctor::cli::run(&arguments, |name| std::env::var(name).ok())),
        Err(error) => {
            eprintln!("[ERROR] cannot start the async runtime: {error}");
            ExitCode::FAILURE
        }
    }
}
