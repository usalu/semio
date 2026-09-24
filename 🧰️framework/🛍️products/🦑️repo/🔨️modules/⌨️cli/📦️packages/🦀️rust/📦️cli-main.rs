//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🧭️ `semio-repo` — the repo command line, twin of the Go `semio-repo` binary: every verb of the
//! declared repo command tree, `mcp` included.

//#endregion 🧲️Header

//#region 🦀️Entrypoint

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(semio_framework_repo_cli::repo_cli::run(&argv));
}

//#endregion 🦀️Entrypoint
