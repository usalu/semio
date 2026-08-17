use crate::args::ParsedArgs;
use std::path::{Path, PathBuf};

// #region 🔖️Command
/// 🖥️ Controls the terminal dashboard daemon lifecycle and attachment.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let subcommand = parsed.segments.first().map(String::as_str).unwrap_or("status");
    let root = parsed.flag("root").map(PathBuf::from).unwrap_or_else(|| root.to_path_buf());
    match subcommand {
        "start" => {
            let executable = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("semio"));
            crate::daemon::start_detached(&root, &executable)
        }
        "serve" => {
            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            match crate::daemon::serve(&root, running) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("daemon serve failed: {error}");
                    1
                }
            }
        }
        "stop" => crate::daemon::stop(&root),
        "status" => {
            print!("{}", crate::daemon::status(&root));
            0
        }
        "attach" => crate::terminal_dashboard::run(&root),
        _ => {
            eprintln!("usage: semio daemon start|stop|status|attach|serve");
            1
        }
    }
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn unknown_subcommand_returns_usage_without_side_effects() {
        let parsed = ParsedArgs { verb: "daemon".into(), segments: vec!["unknown".into()], flags: HashMap::new() };
        assert_eq!(run(Path::new("."), &parsed), 1);
    }
}
// #endregion 🔖️Tests
