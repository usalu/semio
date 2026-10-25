//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The coordinator binary: nothing but a delegation to the crate entry point.

//#endregion 🧲️Header

/// ▶️ Starts the coordinator from the ambient environment and blocks until the process is killed.
fn main() {
    if let Err(failure) = semio_framework_repo_coordinator::main_blocking() {
        eprintln!("repo coordinator: {failure}");
        std::process::exit(1);
    }
}
