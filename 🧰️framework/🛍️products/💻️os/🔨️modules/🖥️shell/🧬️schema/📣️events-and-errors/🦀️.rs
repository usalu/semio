//! 📣️ Events and errors of the shell schema: the shell event vocabulary and the shell error with its display. See [`super`].

use super::*;

//#region 📣️ShellEvent
/// 📣️ What [`reduce`] reports happened. Every accepted command always emits [`ShellEvent::Applied`]
/// (a deterministic, always-present baseline every fixture can assert on); commands whose
/// acceptance also triggers an automatic side effect (focus reassignment, mutual-exclusion
/// clearing, a dock reset) additionally emit the matching specific variant below. This is
/// deliberately NOT a 1:1 mirror of `ShellCommand` — most setters have no side effect beyond the
/// field they set, so a second event carrying the same payload as the command would be pure
/// duplication; the specific variants exist only where `reduce` does something beyond the literal
/// field write the command names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellEvent {
    /// Always emitted, exactly once, as the last event of every accepted command. `revision` is
    /// `number` — see [`PluginSupervisorState::last_signal_ms`].
    Applied {
        capability_id: String,
        revision: u64,
    },
    /// Focus moved — either directly (`FocusWindow`) or automatically (the previously-focused
    /// window disappeared from `SetExtraWindows`'s new list).
    WindowFocusChanged {
        previous: Option<String>,
        current: Option<String>,
    },
    /// `SetActiveTool` cleared a window's active utility for mode↔tool mutual exclusion.
    ActiveUtilityChanged {
        window_id: String,
        previous: Option<String>,
        current: Option<String>,
    },
    /// `SetActiveUtility` cleared the active tool for mode↔tool mutual exclusion.
    ActiveToolChanged {
        previous: Option<String>,
        current: Option<String>,
    },
    DockReset,
    DialogOpened {
        dialog_id: String,
    },
    DialogClosed {
        dialog_id: String,
    },
}
//#endregion 📣️ShellEvent

//#region 🚨️ShellError
/// 🚨️ Why a command was rejected. `reduce` never panics; every invalid transition returns one of
/// these instead of silently no-oping (packet §4: "invalid transitions return `ShellError` rather
/// than silently no-oping").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellError {
    EmptyIdentifier { field: String },
    UnknownPlugin { plugin_id: String },
    UnknownDialog { dialog_id: String },
    UnknownConflict { conflict_id: String },
    InvalidPanelSize { anchor: Anchor, size: f32 },
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => write!(formatter, "empty identifier for {field}"),
            Self::UnknownPlugin { plugin_id } => write!(formatter, "unknown plugin: {plugin_id}"),
            Self::UnknownDialog { dialog_id } => write!(formatter, "unknown dialog: {dialog_id}"),
            Self::UnknownConflict { conflict_id } => write!(formatter, "unknown conflict: {conflict_id}"),
            Self::InvalidPanelSize { anchor, size } => write!(formatter, "invalid panel size for {anchor:?}: {size}"),
        }
    }
}

impl std::error::Error for ShellError {}
//#endregion 🚨️ShellError
