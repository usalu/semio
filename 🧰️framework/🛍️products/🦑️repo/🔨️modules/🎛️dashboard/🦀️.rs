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

#[path = "./🔌️plugin-registry/🦀️.rs"]
pub mod plugin_registry;

#[path = "./🌀️daemon/⌨️command/🦀️.rs"]
pub mod daemon;

#[path = "./⌨️usage/🦀️.rs"]
pub mod usage;

#[path = "./📇️playground-catalog/🦀️.rs"]
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
    if argv.first().is_some_and(|first| matches!(first.as_str(), "--help" | "-h" | "help")) { show_usage(&root, &invocation(argv), usage::help); return 0; }
    let parsed = invocation(argv);
    let rest = without_language(argv.get(1..).unwrap_or_default());
    let rest = rest.as_slice();
    match parsed.verb.as_str() {
        "dashboard" => dashboard_terminal::run_with(&root, &parsed),
        "preferences" => dashboard_preferences::run(&root, &parsed),
        "repo-view" => dashboard_repo_domain::run_action(&root, &parsed),
        "daemon" => daemon::run(&root, &parsed),
        "catalog" => playground_catalog::run(&root, &parsed),
        "command-tree" => dashboard_inventory::command_tree::run(&root, &parsed),
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => plugin_registry::run(&root, parsed.segments.get(1).map_or("generate", String::as_str)),
        "commands" => {
            let locale = dashboard_preferences::cli_locale(&root, &parsed);
            if locale.is_none() && rest.iter().any(|word| word.as_str() == "--check") { eprintln!("{}", dashboard_preferences::language::declared().message("required")); return 2; }
            dashboard_cli::commands(&root, rest, locale, &voice_of(&root, &parsed))
        }
        "run" => dashboard_cli::run(&root, rest, &voice_of(&root, &parsed)),
        "tasks" => dashboard_cli::tasks(&root, rest, &voice_of(&root, &parsed)),
        "logs" => dashboard_cli::logs(&root, rest, &voice_of(&root, &parsed)),
        "stop" | "restart" | "kill" => dashboard_cli::act(&root, &parsed.verb, rest, &voice_of(&root, &parsed)),
        "open" => dashboard_cli::open(&root, rest, &voice_of(&root, &parsed)),
        _ => {
            say(&root, &parsed, |text| text.cli_unknown_verb.fill(&[("verb", argv.first().map_or("", String::as_str))]).into_string());
            show_usage(&root, &parsed, usage::print);
            2
        }
    }
}

/// ✂️ The arguments of a command line verb without the `--language` choice, which the voice already carries; the tail after `--` stays verbatim.
pub fn without_language(rest: &[String]) -> Vec<String> {
    let mut kept = Vec::new();
    let mut words = rest.iter();
    while let Some(word) = words.next() {
        if word == "--" { kept.push(word.clone()); kept.extend(words.by_ref().cloned()); break; }
        if word == "--language" { words.next(); continue; }
        if word.starts_with("--language=") { continue; }
        kept.push(word.clone());
    }
    kept
}

/// 🗣️ The voice of the command line verbs: the explicitly chosen language, or every declared language while none is chosen.
fn voice_of(root: &std::path::Path, parsed: &ParsedArgs) -> dashboard_cli::Voice {
    let selected = dashboard_preferences::cli_locale(root, parsed);
    let locales = dashboard_preferences::language::declared().choices().into_iter().map(|choice| dashboard_preferences::language::negotiate(&choice.language).expect("validated choice")).filter(|locale| selected.is_none_or(|selected| selected == *locale));
    dashboard_cli::Voice::new(locales.map(dashboard_labels::labels).collect(), selected)
}

/// 📢️ Prints one catalogue text on standard error in the explicitly chosen language, or in every declared language while none is chosen.
fn say(root: &std::path::Path, parsed: &ParsedArgs, render: impl Fn(&dashboard_labels::DashboardLabels) -> String) {
    let selected = dashboard_preferences::cli_locale(root, parsed);
    for choice in dashboard_preferences::language::declared().choices() {
        let locale = dashboard_preferences::language::negotiate(&choice.language).expect("validated choice");
        if selected.is_none_or(|selected| selected == locale) { eprintln!("{}", render(dashboard_labels::labels(locale))); }
    }
}

/// 📖️ Shows the usage reference in the explicitly chosen language, or in every declared language while none is chosen.
fn show_usage(root: &std::path::Path, parsed: &ParsedArgs, show: fn(&str)) {
    let selected = dashboard_preferences::cli_locale(root, parsed);
    for choice in dashboard_preferences::language::declared().choices() {
        let locale = dashboard_preferences::language::negotiate(&choice.language).expect("validated choice");
        if selected.is_none_or(|selected| selected == locale) { if let Some(text) = usage::text(&choice.language) { show(text); } }
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
    use semio_framework_repo_workspace::spawn_command;
    use std::path::Path;

    /// 🏃️ Spawns `cmd` with inherited stdio, extending (not replacing) the current environment.
    pub fn spawn_inherit(cmd: &str, args: &[&str], cwd: &Path, env: &[(String, String)]) -> i32 {
        let status = spawn_command(cmd).args(args).current_dir(cwd).envs(env.iter().cloned()).status();
        match status {
            Ok(s) => s.code().unwrap_or(1),
            Err(e) => {
                super::say(cwd, &dashboard_core::args::ParsedArgs::default(), |text| text.cli_run_failed.fill(&[("command", cmd), ("error", &e.to_string())]).into_string());
                1
            }
        }
    }
}
// #endregion 🔖️Proc

// #region 🔖️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

