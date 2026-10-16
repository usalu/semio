//! 🎛️ The semio terminal dashboard: the wizard command tree, the pseudo-terminal windows it runs
//! commands in, the session daemon behind them, and the repo-domain leaves the Rust domain crates
//! answer in process.
//!
//! The dashboard is a family of producer crates, one per layer, each rooted at a domain sub-folder of
//! `🔨️modules/🎛️dashboard` so the taxonomy tree states what the dashboard is made of: the core, the launch contract, the repo
//! domain, the command registry with its inventory and command tree, the daemon wire, the daemon, the labels, the preferences,
//! the terminal view and the command line. This crate owns the `semio` binary: `🚪️entrypoint` hands its argv to [`run`], which
//! dispatches each verb to its crate or command module.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🚪️entrypoint/🦀️.rs

use dashboard_core::args::{parse, ParsedArgs};

#[path = "../../🔌️plugin-registry/🦀️.rs"]
pub mod plugin_registry;

#[path = "../../🌀️daemon/⌨️command/🦀️.rs"]
pub mod daemon;

#[path = "../../⌨️usage/🦀️.rs"]
pub mod usage;

#[path = "../../📇️playground-catalog/🦀️.rs"]
pub mod playground_catalog;

// #region 🔖️Dispatch
/// 🗂️ The verbs `run` answers itself, in dispatch order. The usage text and the README verb table list every
/// one of them except the internal ones; a test fails when the three drift apart.
pub const NATIVE_VERBS: &[&str] = &["dashboard", "preferences", "repo-view", "daemon", "catalog", "command-tree", "plugin", "commands", "run", "tasks", "logs", "stop", "restart", "kill", "open"];
/// 🔒️ Native verbs that exist for the dashboard's own child processes and are not documented as commands.
pub const INTERNAL_VERBS: &[&str] = &["repo-view"];

/// 🚦️ Runs one `semio` invocation and returns its process exit code.
pub fn run(argv: &[String]) -> i32 {
    let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
    if argv.first().is_some_and(|first| matches!(first.as_str(), "--help" | "-h" | "help")) { usage::help(); return 0; }
    let parsed = invocation(argv);
    let rest = argv.get(1..).unwrap_or_default();
    match parsed.verb.as_str() {
        "dashboard" => dashboard_terminal::run_with(&root, &parsed),
        "preferences" => dashboard_preferences::run(&root, &parsed),
        "repo-view" => dashboard_repo_domain::run_action(&root, &parsed),
        "daemon" => daemon::run(&root, &parsed),
        "catalog" => playground_catalog::run(&root, &parsed),
        "command-tree" => dashboard_inventory::command_tree::run(&root, &parsed),
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => plugin_registry::run(&root, parsed.segments.get(1).map_or("generate", String::as_str)),
        "commands" => dashboard_cli::commands(&root, rest),
        "run" => dashboard_cli::run(&root, rest),
        "tasks" => dashboard_cli::tasks(&root, rest),
        "logs" => dashboard_cli::logs(&root, rest),
        "stop" | "restart" | "kill" => dashboard_cli::act(&root, &parsed.verb, rest),
        "open" => dashboard_cli::open(&root, rest),
        _ => {
            eprintln!("[semio] unknown verb {:?}", argv.first().map_or("", String::as_str));
            usage::print();
            2
        }
    }
}

/// 🧭️ Reads argv as one verb invocation; a bare or flag-first invocation is the dashboard itself.
fn invocation(argv: &[String]) -> ParsedArgs {
    if argv.first().is_none_or(|argument| argument.starts_with("--")) {
        return parse(&std::iter::once("dashboard".into()).chain(argv.iter().cloned()).collect::<Vec<_>>());
    }
    parse(argv)
}
// #endregion 🔖️Dispatch

// #region 🔖️Proc
pub mod proc {
    use std::path::Path;
    use std::process::Command;

    /// 🏃️ Spawns `cmd` with inherited stdio, extending (not replacing) the current environment.
    pub fn spawn_inherit(cmd: &str, args: &[&str], cwd: &Path, env: &[(String, String)]) -> i32 {
        let status = Command::new(cmd).args(args).current_dir(cwd).envs(env.iter().cloned()).status();
        match status {
            Ok(s) => s.code().unwrap_or(1),
            Err(e) => {
                let text = dashboard_labels::labels(dashboard_preferences::cli_locale(cwd, &dashboard_core::args::ParsedArgs::default()));
                eprintln!("{}", text.cli_run_failed.fill(&[("command", cmd), ("error", &e.to_string())]).into_string());
                1
            }
        }
    }
}
// #endregion 🔖️Proc

// #region 🔖️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

