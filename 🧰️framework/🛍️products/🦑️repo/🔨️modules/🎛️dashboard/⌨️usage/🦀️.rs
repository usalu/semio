//! ⌨️ The non-interactive `semio` usage reference.

// #region 🔖️Presentation
/// 📖️ The usage reference: every verb with every flag it reads. `FLAGS` states the same flags as data; a test fails
/// when the two, the dispatch table, the argument readers and the README verb table drift apart.
pub const USAGE: &str = "semio — the developer control plane of the monorepo\n\nUsage:\n  semio                          native developer dashboard (requires a TTY)\n  semio dashboard [--language en|de] [--appearance dark|light] [--terminology native|reuse]\n                  [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows] [--prefix KEY]\n                  [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace] [--help]\n                                 the interactive dashboard; --help prints its keyboard help\n  semio commands [words…] [--json] [--all] [--check] [--refresh] [--root PATH] [--snapshot PATH]\n                                 list or search the command registry; --check proves every declaration\n  semio run <id> [--param k=v|flag]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--raw]\n                 [--timeout S] [--refresh] [--root PATH] [--snapshot PATH] [-- args…]\n                                 run a registry command as a daemon task\n  semio tasks [--json] [--root PATH]\n                                 list the daemon tasks\n  semio logs <task> [--follow] [--raw] [--root PATH]\n                                 print the output of a task\n  semio stop|restart|kill <task> [--group] [--timeout S] [--root PATH]\n  semio open <task> [--print] [--root PATH]\n                                 open the ready address of a task in the browser\n  semio daemon start|status|stop|attach|serve [--root PATH]\n                                 manage the workspace daemon\n  semio preferences show|set [--language L] [--appearance A] [--terminology T] [--renderer R] [--layout L]\n                  [--prefix KEY] [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace]\n                                 read or record preferences\n  semio catalog [--json]         list the playground catalog\n  semio plugin registry generate|check\n  semio command-tree [--dump-tree] [--root PATH]\n                                 print the workspace command tree\n  semio --help                   this text\n\nSemio has no other verbs: every task of the monorepo is a registry command. Find an id with\n`semio commands <words> --json`, start it with `semio run <id>`; the routes of the root script are the targets\nof the root project, for example `semio run workspace:test` or `semio run workspace:dev -- storybook`.\nAgents start servers with `semio run <id> --detach --wait-ready`, which prints the ready address last, and\nattach previews to it. A task is named by the session id every verb prints, or by its command id.";

/// 🚩️ Every flag each verb reads, in the order `USAGE` lists them (`--help` of `semio` itself excluded).
pub const FLAGS: &[(&str, &[&str])] = &[
    ("dashboard", &["language", "appearance", "terminology", "renderer", "layout", "prefix", "bindings", "root", "config", "workspace", "help"]),
    ("commands", &["json", "all", "check", "refresh", "root", "snapshot"]),
    ("run", &["param", "env", "detach", "wait-ready", "dry-run", "json", "raw", "timeout", "refresh", "root", "snapshot"]),
    ("tasks", &["json", "root"]),
    ("logs", &["follow", "raw", "root"]),
    ("stop", &["group", "timeout", "root"]),
    ("restart", &["group", "timeout", "root"]),
    ("kill", &["group", "timeout", "root"]),
    ("open", &["print", "root"]),
    ("daemon", &["root"]),
    ("preferences", &["language", "appearance", "terminology", "renderer", "layout", "prefix", "bindings", "root", "config", "workspace"]),
    ("catalog", &["json"]),
    ("command-tree", &["dump-tree", "root"]),
];

/// 🧭️ Presents the non-interactive Semio CLI usage reference on standard error, where a refusal belongs.
pub fn print() {
    eprintln!("{USAGE}");
}

/// 📣️ Presents the usage reference on standard output, for `semio --help`.
pub fn help() {
    println!("{USAGE}");
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
