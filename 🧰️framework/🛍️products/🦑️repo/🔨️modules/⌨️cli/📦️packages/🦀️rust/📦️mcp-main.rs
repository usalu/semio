//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🚀️ `repo` — the repo Model Context Protocol server over stdio, backed by the same
//! production repository `semio mcp` serves. The profile comes from `SEMIO_REPO_MCP_CLIENT`; no
//! command arguments are accepted.

//#endregion 🧲️Header

//#region 🦀️Entrypoint

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
    std::process::exit(semio_framework_repo_mcp::run_with(&argv, semio_framework_repo_cli::mcp_verb::real_repository(root)));
}

//#endregion 🦀️Entrypoint
