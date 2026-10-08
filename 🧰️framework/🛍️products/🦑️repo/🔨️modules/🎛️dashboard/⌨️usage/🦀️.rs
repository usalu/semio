//! ⌨️ The non-interactive `semio` usage reference.

// #region 🔖️Presentation
const USAGE: &str = "semio — semio monorepo orchestrator\n\nUsage:\n  semio                 native developer dashboard (requires a TTY)\n  semio dashboard       dashboard with optional settings (--help)\n  semio preferences …   show|set local or workspace preferences\n  semio dev <variant…>  start a plugin dev session\n  semio catalog         list playgrounds\n  semio plugin registry generate|check\n  semio daemon …        start|stop|status|attach dashboard daemon\n  semio commands …      list or search the command registry (--json, --all, --check, --refresh)\n  semio command-tree    print the workspace command tree (--dump-tree for JSON)\n  semio <verb> …        forwarded to `bun ./📜️script.ts <verb> …`";

/// 🧭️ Presents the non-interactive Semio CLI usage reference.
pub fn print() {
    eprintln!("{USAGE}");
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
