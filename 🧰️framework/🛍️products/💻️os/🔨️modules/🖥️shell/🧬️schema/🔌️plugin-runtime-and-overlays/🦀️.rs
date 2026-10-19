//! 🔌️ Plugin runtime and overlays of the shell schema: loaded plugins and their supervisor state, active sessions, dialogs, notices and the app role. See [`super`].

use super::*;

//#region 🔌️PluginRuntime
/// 🔌️ One loaded plugin's registry entry (audit `LoadedProgramState` — its exact field shape was
/// never captured by the audit; this is a minimal domain-shaped mirror sufficient for identity +
/// registry semantics, reconciled at adoption time).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct LoadedPlugin {
    pub plugin_id: String,
    pub module_url: String,
    pub label: Option<String>,
}

/// 🪟️ Plugin panel open/collapsed UI state (audit: "plugin panel UI state (open/collapsed)").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PluginPanelStatus {
    Open,
    Collapsed,
    Errored { message: String },
}

/// 🚑️ Plugin resource/failure monitoring summary (audit: "plugin resource/failure monitoring").
/// A minimal local mirror — the full failure ladder lives in the actor runtime crate this module
/// must not depend on (§4); this is the shell-observable projection of it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct PluginSupervisorState {
    pub healthy: bool,
    pub restart_count: u32,
    /// 🍃️ `u64`, but the wire format is plain JSON (this module has no binary codec), so the TS
    /// mirror is explicitly `number`, since millisecond epoch timestamps never
    /// approach `u64::MAX`, let alone JS's `Number.MAX_SAFE_INTEGER`.
    pub last_signal_ms: Option<u64>,
}

/// 🎯️ The active app instance binding (audit: "active app instance binding").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ActiveSession {
    pub plugin_id: String,
    pub app_id: String,
    pub instance_id: u32,
}
//#endregion 🔌️PluginRuntime

//#region 🔔️Overlays
/// 🪟️ One entry on the open-dialog stack. Modeled as a stack (not the audit's flat
/// `Option<dialog>`) so `OpenDialog`/`CloseDialog` can express dialog-over-dialog stacking — one
/// of the tricky paths the packet brief calls out explicitly; a flat `Option` cannot represent a
/// confirm dialog opened on top of a settings dialog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct DialogState {
    pub dialog_id: String,
    #[value(with = "json_value_option_bridge")]
    pub seed_args: Option<JsonValue>,
}

/// 🔔️ Severity of a [`TransientNotice`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum NoticeKind {
    Info,
    Success,
    Warning,
    Error,
}

/// 🔔️ A non-blocking auto-dismiss notice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TransientNotice {
    pub message: String,
    pub kind: NoticeKind,
    /// 🧩️ See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// a wider integer projection, in the TypeScript mirror.
    pub expires_at_ms: Option<u64>,
}

/// 🎭️ Which role group the Open panel should focus. Local newtype mirror (§4) — the real
/// `AppRole` enumeration lives in the plugin manifest surface this module must not depend on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[value(crate = "::protocol::value", transparent)]
pub struct AppRole(pub String);
//#endregion 🔔️Overlays
