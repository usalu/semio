// #region 🔖️Presentation
const USAGE: &str = "semio — semio monorepo orchestrator\n\nUsage:\n  semio                 interactive TUI dashboard (requires a TTY)\n  semio dev <variant…>  start a plugin dev session\n  semio catalog         list playgrounds\n  semio plugin registry generate|check\n  semio daemon …        start|stop|status|attach dashboard daemon\n  semio <verb> …        forwarded to `bun ./📜️script.ts <verb> …`";

/// 🧭️ Presents the non-interactive Semio CLI usage reference.
pub fn print() {
    eprintln!("{USAGE}");
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::USAGE;

    #[test]
    fn usage_reference_preserves_every_registered_command() {
        assert_eq!(
            USAGE,
            "semio — semio monorepo orchestrator\n\nUsage:\n  semio                 interactive TUI dashboard (requires a TTY)\n  semio dev <variant…>  start a plugin dev session\n  semio catalog         list playgrounds\n  semio plugin registry generate|check\n  semio daemon …        start|stop|status|attach dashboard daemon\n  semio <verb> …        forwarded to `bun ./📜️script.ts <verb> …`"
        );
    }
}
// #endregion 🔖️Tests
