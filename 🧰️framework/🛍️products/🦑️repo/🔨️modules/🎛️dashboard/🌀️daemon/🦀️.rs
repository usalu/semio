//! 🌀️ The dashboard daemon: the process that owns the developer processes of a workspace independently of
//! the views attached to it. One folder per concern — the wire, the transport, the readiness and replay
//! of output, the journal, the views, the groups, the supervisor, the connection a view holds and the
//! control surface without a terminal — and the `semio daemon start|serve|stop|status|attach` verb.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use crate::args::ParsedArgs;
use crate::terminal::labels::labels;
use std::path::{Path, PathBuf};

#[path = "✉️ipc/🦀️.rs"]
pub mod ipc;

#[path = "🟢️ready/🦀️.rs"]
pub mod ready;

#[path = "📼️replay/🦀️.rs"]
pub mod replay;

#[path = "🚚️transport/🦀️.rs"]
pub mod transport;

#[path = "📜️journal/🦀️.rs"]
pub mod journal;

#[path = "👥️clients/🦀️.rs"]
pub mod clients;

#[path = "🧩️groups/🦀️.rs"]
pub mod groups;

#[path = "🧠️supervisor/🦀️.rs"]
pub mod supervisor;

#[path = "../📎️connection/🦀️.rs"]
pub mod client;

#[path = "🕹️control/🦀️.rs"]
pub mod control;

// #region 🔖️Command
/// 🖥️ Controls the terminal dashboard daemon lifecycle and attachment.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let subcommand = parsed.segments.first().map_or("status", String::as_str);
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let root = ipc::canonical_path(&root);
    let text = labels(crate::preferences::cli_locale(&root, parsed));
    match subcommand {
        "start" => {
            let executable = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("semio"));
            supervisor::start_detached(&root, &executable, text)
        }
        "serve" => {
            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            match supervisor::serve(&root, &running) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("{}", text.cli_daemon_serve_failed.fill(&[("error", &error.to_string())]).into_string());
                    1
                }
            }
        }
        "stop" => supervisor::stop(&root, text),
        "status" => {
            print!("{}", supervisor::status(&root, text));
            0
        }
        "attach" => crate::terminal::run(&root),
        _ => {
            eprintln!("{}", text.cli_daemon_usage.as_str());
            1
        }
    }
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧊️integration/🦀️.rs"]
mod integration;
// #endregion 🔖️Tests
