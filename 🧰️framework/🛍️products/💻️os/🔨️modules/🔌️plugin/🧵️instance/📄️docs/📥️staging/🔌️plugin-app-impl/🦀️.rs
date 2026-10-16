//! 🥁️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.
use super::*;
use crate::tool_jobs::tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

/// 📣️ Signals the shell that the document's checkpoint/alternative history changed (after an
/// undo/redo/checkpoint/alternative command) so it can re-render history-dependent surfaces.
pub(crate) async fn history_changed_event() -> AppEvent {
    AppEvent { kind: "history-changed".into(), payload: DslValue::Null }
}
