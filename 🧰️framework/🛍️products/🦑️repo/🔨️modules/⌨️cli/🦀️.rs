//! 🧭️ `semio`: the monorepo orchestrator CLI. It owns the verb dispatch and hands every dashboard
//! verb to `semio-framework-repo-dashboard`, which owns the command modules themselves.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs

use semio_framework_repo_dashboard::{args, command_tree, daemon, playground_catalog, playground_session, plugin_registry, root_delegation, terminal, usage, workflow};
use std::io::IsTerminal;
use std::path::PathBuf;

// #region 🔖️Dispatch
/// 🚦️ Runs one `semio` invocation and returns its process exit code.
pub fn run(argv: &[String]) -> i32 {
    let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if argv.is_empty() {
        if !std::io::stdout().is_terminal() {
            usage::print();
            return 1;
        }
        return terminal::run(&root);
    }
    let parsed = args::parse(argv);
    match parsed.verb.as_str() {
        "daemon" => daemon::run(&root, &parsed),
        "workflow" => workflow::run(&root, &parsed),
        "dev" => playground_session::run(&root, &parsed),
        "catalog" => playground_catalog::run(&root, &parsed),
        "command-tree" => command_tree::run(&root, &parsed),
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => plugin_registry::run(&root, parsed.segments.get(1).map_or("generate", String::as_str)),
        _ => root_delegation::run(&root, &parsed),
    }
}
// #endregion 🔖️Dispatch
