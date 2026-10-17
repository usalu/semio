//! 🗃️ Utility, tool and document actions: the active utility and tool actions, the document transfer actions and the default action icon. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🧰️ The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// 🧰️ The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_UTILITY_ACTION_ID, LocalizedLabel::native("Set Active Utility", "Aktives Hilfsmittel festlegen"), ActionKind::View, "wrench") }
        .with_args([ActionArgDef::text("utilityId", LocalizedLabel::native("Utility", "Hilfsmittel")).required(), ActionArgDef::text("windowKindId", LocalizedLabel::native("Window", "Fenster"))])
}

/// 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — auto-injected
/// as a View action into any `AppDefinition` that declares tools (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const SET_ACTIVE_TOOL_ACTION_ID: &str = "setActiveTool";

/// 🛠️ The framework-injected `setActiveTool` View action (never in the palette): switches the
/// host-owned active tool of the active mode. Unlike `setActiveUtility` this takes no `windowKindId` —
/// tools are windowless, scoped to the whole mode.
pub fn set_active_tool_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_TOOL_ACTION_ID, LocalizedLabel::native("Set Active Tool", "Aktives Werkzeug festlegen"), ActionKind::View, "hammer") }.with_args([ActionArgDef::text(
        "toolId",
        LocalizedLabel::native("Tool", "Werkzeug"),
    )
    .required()])
}

/// 📤️ The framework-owned action id of Export Document: injected into every app window and fully
/// shell-intercepted (never forwarded to the program) — the shell writes the document's canonical archive (root
/// envelope, its op log, every owned member) as one `.semio-archive` file, for every artifact kind alike.
pub const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "exportArtifactDocument";

/// 📥️ The framework-owned action id of Import Document, injected beside [`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`]
/// and shell-intercepted: the shell opens a picked archive as a NEW document of the same program (a cancellable,
/// progress-reporting load of its op log), never overwriting the focused one.
pub const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "importArtifactDocument";

/// 💾️ The shell-intercepted action that writes the focused editor's declared natural file bytes.
pub const SAVE_ARTIFACT_FILE_ACTION_ID: &str = "saveArtifactFile";

/// 📂️ The shell-intercepted action that opens one declared natural file as a new document owner.
pub const OPEN_ARTIFACT_FILE_ACTION_ID: &str = "openArtifactFile";

/// 📄️ The ABI media port reserved for an editor's declared natural-file representation.
pub const NATURAL_FILE_PORT_ID: &str = "artifact:native";

/// 🗃️ The framework-injected Export/Import Document pair: `Shell` verbs in the palette and the `transfer` ribbon
/// category, shell chrome ([`CapabilityAudience::Chrome`]) — agents export through the MCP's own artifact export.
pub fn document_transfer_action_definitions() -> [ActionDefinition; 2] {
    [
        ActionDefinition::resumable_framework(EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Export Document", "Dokument exportieren"), ActionKind::Shell, "export").with_category("transfer").audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Import Document…", "Dokument importieren…"), ActionKind::Shell, "import").with_category("transfer").audience(CapabilityAudience::Chrome),
    ]
}

/// 🎯️ Neutral icon for an action or command whose owner does not declare presentation metadata.
pub fn default_action_icon_id(kind: ActionKind) -> IconName {
    match kind {
        ActionKind::View => "eye".into(),
        ActionKind::Shell => "code".into(),
        ActionKind::Mutation => "sparkles".into(),
        ActionKind::History => "clock".into(),
        ActionKind::Clipboard => "clipboard".into(),
        ActionKind::Interaction => "mouse-pointer".into(),
    }
}
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
