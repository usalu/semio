
use super::USAGE;

#[test]
fn usage_reference_preserves_every_registered_command() {
    assert_eq!(
        USAGE,
        "semio — semio monorepo orchestrator\n\nUsage:\n  semio                 interactive TUI dashboard wizard (requires a TTY)\n  semio dev <variant…>  start a plugin dev session\n  semio catalog         list playgrounds\n  semio plugin registry generate|check\n  semio daemon …        start|stop|status|attach dashboard daemon\n  semio command-tree    print the wizard command tree (--dump-tree for JSON)\n  semio <verb> …        forwarded to `bun ./📜️script.ts <verb> …`"
    );
}
