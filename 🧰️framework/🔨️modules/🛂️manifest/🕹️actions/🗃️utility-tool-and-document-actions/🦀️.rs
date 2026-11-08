//! 🗃️ Utility, tool and document actions: the active utility and tool actions, the document transfer actions and the default action icon. See [`super`].

use super::*;
use semio_framework_manifest_arg_schema::ArgPresentation;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🧰️ The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// 👁️ The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_UTILITY_ACTION_ID, LocalizedLabel::native("Set Active Utility", "Aktives Hilfsmittel festlegen"), ActionKind::View, "wrench") }
        .with_args([ActionArgDef::text("utilityId", LocalizedLabel::native("Utility", "Hilfsmittel")).required(), ActionArgDef::text("windowKindId", LocalizedLabel::native("Window", "Fenster"))])
}

/// 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — auto-injected
/// as a View action into any `AppDefinition` that declares tools (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const SET_ACTIVE_TOOL_ACTION_ID: &str = "setActiveTool";

/// 🔧️ The framework-injected `setActiveTool` View action (never in the palette): switches the
/// host-owned active tool of the active mode. Unlike `setActiveUtility` this takes no `windowKindId` —
/// tools are windowless, scoped to the whole mode.
pub fn set_active_tool_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_TOOL_ACTION_ID, LocalizedLabel::native("Set Active Tool", "Aktives Werkzeug festlegen"), ActionKind::View, "hammer") }.with_args([ActionArgDef::text(
        "toolId",
        LocalizedLabel::native("Tool", "Werkzeug"),
    )
    .required()])
}

/// 🎯️ The framework-owned action id of "focus the window of kind X", shell-intercepted in every app (never forwarded to the program): the shell opens the window if it is closed, raises it, moves the keyboard focus into it and announces it politely to assistive technology in the interface language. A chord (`ShellKeyOutcome::Verb`), a menu row and the palette all raise this one verb; the window kind arrives as the text argument [`FOCUS_WINDOW_ARG_KIND`].
pub const FOCUS_WINDOW_ACTION_ID: &str = "focusWindow";

/// 🎯️ The text argument of [`FOCUS_WINDOW_ACTION_ID`]: the id of a window kind the app declares.
pub const FOCUS_WINDOW_ARG_KIND: &str = "windowKindId";

/// 📱️ The optional text argument of [`FOCUS_WINDOW_ACTION_ID`]: the window kind that stands in for the requested one on a device whose layout does not contain it (a mobile tab, a tablet side pane); without it, or when the stand-in is missing there too, the focus is refused with a localized notice.
pub const FOCUS_WINDOW_ARG_FALLBACK: &str = "fallbackWindowKindId";

/// ✏️ The framework-owned action id of "focus the editable control inside the row R of a tree or grid", shell-intercepted in every app (never forwarded to the program): the shell moves the keyboard focus into the first editable control the row holds (a text, number, date or multi-line field) so typing replaces the cell, and announces a refusal politely when the row holds none. `F2` on a focused row raises it ([`FOCUS_ROW_CONTROL_KEY`]); the row arrives as the text argument [`FOCUS_ROW_CONTROL_ARG_ROW`]. Escape in the cell drops the typed draft and shows the committed value again (the shared text keyboard law); Enter or leaving the cell commits.
pub const FOCUS_ROW_CONTROL_ACTION_ID: &str = "focusRowControl";

/// ✏️ The text argument of [`FOCUS_ROW_CONTROL_ACTION_ID`]: the authored key of the row (the id its author gave the tree item).
pub const FOCUS_ROW_CONTROL_ARG_ROW: &str = "rowId";

/// ⌨️ The key that raises [`FOCUS_ROW_CONTROL_ACTION_ID`] on a focused row (the spreadsheet and grid convention for "edit this cell").
pub const FOCUS_ROW_CONTROL_KEY: &str = "F2";

/// ✏️ The input kinds a row cell edits by typing; a colour or file input opens a picker instead and a toggle, button or select is not typed into, so none of them takes the focus of [`FOCUS_ROW_CONTROL_ACTION_ID`].
pub const FOCUS_ROW_CONTROL_EDITABLE_KINDS: [&str; 4] = ["text", "longText", "number", "date"];

/// ✏️ One control inside a row, reduced to what [`plan_row_control_focus`] reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowControl<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub disabled: bool,
}

/// ✏️ The control a [`FOCUS_ROW_CONTROL_ACTION_ID`] lands on: the first enabled control of the row, in row order, whose kind is one of [`FOCUS_ROW_CONTROL_EDITABLE_KINDS`]; `None` when the row holds no such control (the focus is refused). Shared rule of every host, pinned by `🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-row-control/🔣️.json`.
pub fn plan_row_control_focus<'a>(controls: &[RowControl<'a>]) -> Option<&'a str> {
    controls.iter().find(|control| !control.disabled && FOCUS_ROW_CONTROL_EDITABLE_KINDS.contains(&control.kind)).map(|control| control.id)
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

/// 📤️ The shell-intercepted action that downloads a declared output through the checked media broker.
pub const EXPORT_MEDIA_FILE_ACTION_ID: &str = "exportMediaFile";

/// 📸️ The shell-intercepted action that downloads the current materialized snapshot through its declared broker port.
pub const EXPORT_SNAPSHOT_FILE_ACTION_ID: &str = "exportSnapshotFile";

/// 🔌️ Declares a binary output download with static representation metadata and localized chrome.
pub fn media_file_export_action_definition(action_id: &str, label: LocalizedLabel, output_port: &str, filename: &str, media_type: &str, format_kind: &str) -> ActionDefinition {
    let args = [("outputPort", output_port), ("filename", filename), ("mediaType", media_type), ("formatKind", format_kind)].map(|(id, value)| {
        let mut argument = ActionArgDef::text(id, LocalizedLabel::native(id, id)).default_value(&value);
        argument.presentation = Some(ArgPresentation::Hidden);
        argument
    });
    ActionDefinition { in_palette: true, ..ActionDefinition::resumable_framework(action_id, label, ActionKind::Shell, "export").with_category("transfer").with_args(args).audience(CapabilityAudience::Chrome) }
}

/// 📂️ The shell-intercepted action that opens one declared natural file as a new document owner.
pub const OPEN_ARTIFACT_FILE_ACTION_ID: &str = "openArtifactFile";

/// 📄️ The ABI media port reserved for an editor's declared natural-file representation.
pub const NATURAL_FILE_PORT_ID: &str = "artifact:native";

/// 🎬️ The framework-injected Export/Import Document pair: `Shell` verbs in the palette and the `transfer` ribbon
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
