//! ⌨️ The `repo dashboard daemon start|serve|stop|status|attach` verb: it controls the lifecycle of the workspace daemon and attaches the
//! terminal dashboard to it.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🚦️lifecycle/🦀️.rs

use dashboard_core::args::ParsedArgs;
use dashboard_daemon::lifecycle;
use dashboard_labels::labels;
use dashboard_wire::ipc;
use std::path::{Path, PathBuf};

// #region 🔖️Command
/// 🖥️ Controls the terminal dashboard daemon lifecycle and attachment.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let subcommand = parsed.segments.first().map_or("status", String::as_str);
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let root = ipc::canonical_path(&root);
    if subcommand=="attach" { let mut attach=parsed.clone();attach.segments.clear();return dashboard_terminal::run_with(&root,&attach); }
    if subcommand=="serve" {
        let running=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        return match lifecycle::serve(&root,&running){Ok(())=>0,Err(error)=>{let selected=dashboard_preferences::cli_locale(&root,parsed);for choice in dashboard_preferences::language::declared().choices(){let locale=dashboard_preferences::language::negotiate(&choice.language).expect("validated choice");if selected.is_none_or(|selected|selected==locale){eprintln!("{}",labels(locale).cli_daemon_serve_failed.fill(&[("error",&error.to_string())]).into_string());}}1}};
    }
    let Some(locale)=dashboard_preferences::cli_locale(&root, parsed) else { eprintln!("{}",dashboard_preferences::language::declared().message("required")); return 2; };
    let text = labels(locale);
    match subcommand {
        "start" => {
            let executable = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("semio"));
            lifecycle::start_detached(&root, &executable, text)
        }
        "stop" => lifecycle::stop(&root, text),
        "status" => {
            print!("{}", lifecycle::status(&root, text));
            0
        }
        "attach" => dashboard_terminal::run(&root),
        _ => {
            eprintln!("{}", text.cli_daemon_usage.as_str());
            1
        }
    }
}
// #endregion 🔖️Command
