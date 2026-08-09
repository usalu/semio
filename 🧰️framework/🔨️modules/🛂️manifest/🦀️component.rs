// #region 🛂️Manifest
//! 🧩️ App manifest (`AppDefinition`/`ModeDefinition`/`WindowKindDefinition`/`PluginManifest`/`ViewModel`)
//! and kernel types shared by plugins and renderers; the declarative `UiNode` component model itself
//! lives in `ui_wgpu`'s `component` region.

use serde::{Deserialize, Serialize};
use dsl::DslValue;
use ui_wgpu::wgpu::{ActionDescriptor, Locale, LocalizedLabel, NamedLayout, SurfaceKind, Terminology, WindowLayout, WindowOptions};
use crate::mesh::{MediaPortSpec, ArtifactKindSpec, ConfigSpec, CommandGrammar, AppIo};
use crate::IconName;

//#region 🔖️Manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// @emoji 🗂️ Classifies a declared action by how it interacts with VCS history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ActionKind {
    /// Mutates the document — dispatched as VCS mutations with a true inverse, recorded in history.
    Mutation,
    /// Ephemeral view state (camera, selection, hover, active utility) — recorded in the session
    /// command log, never as a VCS edit.
    View,
    /// Framework-provided undo/redo/checkpoint/alternative — auto-injected, never app-declared.
    History,
    /// Framework-provided copy/cut/paste — auto-injected, never app-declared (mirrors `History`).
    Clipboard,
    /// Shell-only effect (navigate, export, spawn) — recorded in the session command log via
    /// dispatch or the `noteShellCommand` mechanism, no document mutation.
    Shell,
}

//#region 🔖️ActionArgs
/// @emoji 🔘️ One selectable option of a `Select` argument control — the persisted `value` and its
/// human `label`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionArgOption {
    pub value: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`. Not yet ts-rs-mirrored
    /// (follow-up: `LocalizedLabel` itself has no `TS` impl).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
}

impl ActionArgOption {
    pub fn new(value: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

/// @emoji 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
/// deliberately NOT `ui_wgpu::wgpu::UiControlNode` (whose variants embed live values and immediate-dispatch
/// wiring). Renderers map each variant onto a staged form field. Tagged with `kind` to mirror the
/// sibling `UtilityNode`/`UiControlNode` declarative-tree convention.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActionArgControl {
    Text {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        placeholder: Option<String>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
    },
    Slider {
        min: f64,
        max: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        unit: Option<String>,
    },
    Toggle,
    Select {
        options: Vec<ActionArgOption>,
    },
    Vec3,
    IconSelect {
        classifier_kind: String,
    }
}

/// @emoji 📝️ Declares one argument of an action: its `id` (the JSON key sent in `ActionDescriptor.args`),
/// human `label`, input `control`, whether it is `required`, an optional `default` value, and an optional
/// `description`. An empty `ActionDefinition.args` (the common case) means a no-argument action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionArgDef {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub control: ActionArgControl,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub default: Option<DslValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub description: Option<String>,
}

impl ActionArgDef {
    fn with_control(id: impl Into<String>, label: impl Into<LocalizedLabel>, control: ActionArgControl) -> Self {
        Self { id: id.into(), label: label.into(), control, required: false, default: None, description: None }
    }

    /// @emoji 🔤️ A free-text argument.
    pub fn text(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_control(id, label, ActionArgControl::Text { placeholder: None })
    }

    /// @emoji 🔢️ A numeric argument (unbounded stepper by default).
    pub fn number(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_control(id, label, ActionArgControl::Number { min: None, max: None, step: None })
    }

    /// @emoji 🎚️ A bounded slider argument.
    pub fn slider(id: impl Into<String>, label: impl Into<LocalizedLabel>, min: f64, max: f64) -> Self {
        Self::with_control(id, label, ActionArgControl::Slider { min, max, step: None, unit: None })
    }

    /// @emoji 🔘️ A boolean toggle argument.
    pub fn toggle(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_control(id, label, ActionArgControl::Toggle)
    }

    /// @emoji 🔽️ A single-choice select argument.
    pub fn select(id: impl Into<String>, label: impl Into<LocalizedLabel>, options: Vec<ActionArgOption>) -> Self {
        Self::with_control(id, label, ActionArgControl::Select { options })
    }

    /// @emoji 🧭️ A three-component vector argument.
    pub fn vec3(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_control(id, label, ActionArgControl::Vec3)
    }

    /// @emoji ❗️ Marks the argument as required — execution is blocked until it has an effective value.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// @emoji 🎁️ Sets the default effective value used when nothing is staged.
    pub fn default_value(mut self, value: impl Serialize) -> Self {
        self.default = dsl::to_dsl_value(&value).ok();
        self
    }

    /// @emoji 💬️ Attaches a description shown alongside the field.
    pub fn describe(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
//#endregion 🔖️ActionArgs

/// @emoji 🎛️ Canonical catalog icon for a declared app mode id.
pub fn catalog_mode_icon_id(id: &str) -> IconName {
    match id {
        "edit" | "main" => "pencil".into(),
        "paint" => "paintbrush".into(),
        "generate" => "sparkles".into(),
        "explore" => "focus".into(),
        "builder" => "component".into(),
        "curate" => "folder-open".into(),
        "blueprint" => "cad-shape".into(),
        "review" => "search".into(),
        "report" => "bar-chart-3".into(),
        "view" => "eye".into(),
        "capture" => "camera".into(),
        "model" => "box".into(),
        "analyze" => "search".into(),
        _ => "layers".into(),
    }
}

/// @emoji 🧪️ Canonical catalog icon for a playground example id (content-specific ids override at declaration).
pub fn catalog_example_icon_id(id: &str) -> IconName {
    match id {
        "empty" | "default" => "file".into(),
        "demo" => "cylinder".into(),
        "semio" => "sparkles".into(),
        _ if id.contains("capsule") || id.contains("nakagin") => "building".into(),
        _ if id.contains("forest") || id.contains("concrete") => "list-tree".into(),
        _ if id.contains("hex") => "hexagon".into(),
        _ => "file-text".into(),
    }
}

/// @emoji 🎯️ Canonical catalog icon for a declared action id (view/shell/operation/history/clipboard).
pub fn catalog_action_icon_id(id: &str, kind: ActionKind) -> IconName {
    match id {
        "undo" => "undo-2".into(),
        "redo" => "redo-2".into(),
        "commitCheckpoint" => "git-commit".into(),
        "createAlternative" => "git-branch".into(),
        "switchAlternative" => "git-branch".into(),
        "checkoutCheckpoint" => "git-branch".into(),
        "revertToCommand" => "clock".into(),
        "copy" => "copy".into(),
        "cut" => "scissors".into(),
        "paste" => "clipboard".into(),
        "setHistoryCommandFilter" => "list".into(),
        "noteShellCommand" => "book-open".into(),
        "setActiveUtility" => "wrench".into(),
        "setActiveTool" => "hammer".into(),
        "startIntroduction" => "graduation-cap".into(),
        "setSelection" | "documentSelect" | "selectNode" | "nodeGraphSelect" | "setNodeSelection" | "setFeatureSelection"
        | "setReferenceSelection" | "setMediaNodeSelection" | "setAppInstanceSelection" | "selectRegister"
        | "selectInstance" | "selectSameKind" | "selectSameKindSelection" | "worldSelect" | "worldVortexSelect" => {
            "mouse-pointer".into()
        }
        "clearSelection" | "deselect" => "mouse-pointer-2".into(),
        "selectAll" => "mouse-pointer-2".into(),
        "setCamera" | "setCamera2d" | "setCamera3d" | "nodeGraphViewport" => "camera".into(),
        "setProjection" | "setProjectionParam" => "scan".into(),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "graphPointerDown" | "worldPointerDown" => {
            "mouse-pointer".into()
        }
        "worldHover" | "setHover" | "nodeGraphHover" | "textHover" | "referenceHover" => "eye".into(),
        "worldPick" => "crosshair".into(),
        "engagementInput" | "engagementAbort" | "engagementControlSelect" | "editorEngagementInput"
        | "graphEngagementInput" | "resultsEngagementInput" | "workflowEngagementInput"
        | "compiledDagEngagementInput" => "hand".into(),
        "setLodMode" => "layers".into(),
        "toggleGrid" | "setGridSnapEnabled" | "setGridFactor" => "grid-3x3".into(),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => "sun".into(),
        "run" | "stop" => "play".into(),
        "search" => "search".into(),
        "exportProgram" | "exportRegistersCsv" | "exportMedia" | "exportStudioPack" | "exportStudioDsl"
        | "exportVideoFromDeck" => "download".into(),
        "importMedia" | "importSpacePack" | "importFrames" | "importVideo" | "openSource" => "hard-drive".into(),
        "goHome" => "home".into(),
        "openSpace" | "openInstance" => "folder-open".into(),
        "navigateVirtualFileSystemNode" => "folder".into(),
        "setActiveExample" | "setActivePanelTab" => "panel-left".into(),
        "copyPrompt" => "copy".into(),
        "evaluate" => "hash".into(),
        "recomputeRewrite" | "reorganize" => "rotate-cw".into(),
        "textEdit" | "formatDocument" | "requestCompletions" => "typography".into(),
        "textSelect" => "text-cursor".into(),
        "paintStrokeBegin" | "paintStroke" | "paintAt" | "paintSample" => "paintbrush".into(),
        "transformBegin" => "move".into(),
        "incrementViaCommand" | "setLabelViaCommand" => "plus".into(),
        _ => match kind {
            ActionKind::View => "eye".into(),
            ActionKind::Shell => "code".into(),
            ActionKind::Mutation => "sparkles".into(),
            ActionKind::History => "clock".into(),
            ActionKind::Clipboard => "clipboard".into(),
        },
    }
}

/// @emoji 🎛️ Canonical catalog icon for a footer command id.
pub fn catalog_command_icon_id(id: &str) -> IconName {
    match id {
        id if id.starts_with("os.set") => "settings".into(),
        "os.resetDock" => "panel-left".into(),
        "os.toggleCompact" => "minimize-2".into(),
        "app.export" | "incrementViaCommand" | "setLabelViaCommand" => "download".into(),
        "mode.focus" => "focus".into(),
        "animate.resetGrid" => "grid-3x3".into(),
        _ => "code".into(),
    }
}

/// @emoji 📇️ Declares one action an app can receive via `ActionDescriptor.action`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub kind: ActionKind,
    pub icon_id: IconName,
    /// 📝️ Typed argument declarations. Empty (the common case) = a no-argument action.
    pub args: Vec<ActionArgDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    #[serde(default)]
    pub in_palette: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub category: Option<String>,
}

impl ActionDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind, icon_id: impl Into<IconName>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            icon_id: icon_id.into(),
            args: Vec::new(),
            keys: None,
            in_palette: true,
            category: None,
        }
    }

    /// @emoji 🎯️ Declares an action whose icon is resolved from {@link catalog_action_icon_id}.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        let id = id.into();
        Self::new(id.clone(), label, kind, catalog_action_icon_id(&id, kind))
    }

    /// @emoji 📝️ Attaches typed argument declarations to this action.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// @emoji 🎨️ Sets palette visibility for this action.
    pub fn with_in_palette(mut self, in_palette: bool) -> Self {
        self.in_palette = in_palette;
        self
    }

    /// @emoji 🎨️ Sets palette visibility for this action.
    pub fn in_palette(self, in_palette: bool) -> Self {
        self.with_in_palette(in_palette)
    }

    /// @emoji 🗂️ Sets this action's ribbon-parent-taxonomy category (a `ui_wgpu::wgpu::RIBBON_PARENT_CATEGORIES`
    /// id) — read back by `AppActionRegistry::category_of` and fed into `organize_context_menu`'s
    /// `category_of` lookup at the context-menu funnel, so an overflowing flat menu buckets this
    /// action's row into `menu.group.<category>` instead of `menu.group.actions`.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// @emoji 🗂️ Sets this action's ribbon-parent-taxonomy category — see `with_category`.
    pub fn category(self, category: impl Into<String>) -> Self {
        self.with_category(category)
    }
}

/// @emoji ⏪️ The framework-owned action id apps dispatch to revert to a past command-log entry —
/// auto-injected as the 7th `history_action_definitions()` entry (never in the palette; needs a
/// concrete `entrySeq` from the history panel's "backwards" button).
pub const REVERT_TO_COMMAND_ACTION_ID: &str = "revertToCommand";

/// @emoji 🕹️ The seven framework-owned History actions, auto-injected into every `AppDefinition`.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition {
            keys: Some("mod+z".into()),
            ..ActionDefinition::new_catalog("undo", LocalizedLabel::native("Undo", "Rückgängig"), ActionKind::History)
        },
        ActionDefinition {
            keys: Some("mod+shift+z".into()),
            ..ActionDefinition::new_catalog("redo", LocalizedLabel::native("Redo", "Wiederholen"), ActionKind::History)
        },
        ActionDefinition::new_catalog("commitCheckpoint", LocalizedLabel::native("Commit Checkpoint", "Checkpoint festschreiben"), ActionKind::History),
        ActionDefinition::new_catalog("createAlternative", LocalizedLabel::native("Create Alternative", "Alternative erstellen"), ActionKind::History),
        ActionDefinition::new_catalog("switchAlternative", LocalizedLabel::native("Switch Alternative", "Alternative wechseln"), ActionKind::History),
        ActionDefinition::new_catalog("checkoutCheckpoint", LocalizedLabel::native("Checkout Checkpoint", "Checkpoint auschecken"), ActionKind::History),
        ActionDefinition {
            in_palette: false,
            ..ActionDefinition::new_catalog(
                REVERT_TO_COMMAND_ACTION_ID,
                LocalizedLabel::native("Revert to Command", "Auf Befehl zurücksetzen"),
                ActionKind::History,
            )
        }
        .with_args([ActionArgDef::number("entrySeq", LocalizedLabel::native("Entry", "Eintrag")).required()]),
    ]
}

/// @emoji 🎚️ The framework-owned action id apps dispatch to change the history panel's operations
/// filter — auto-injected unconditionally (mirrors `RECORD_TUTORIAL_ACTION_ID`).
pub const SET_HISTORY_COMMAND_FILTER_ACTION_ID: &str = "setHistoryCommandFilter";

/// @emoji 🎚️ The framework-injected `setHistoryCommandFilter` View action (never in the palette):
/// switches the history panel's tri-state operations filter. Ephemeral UI state, never an
/// operation — `ActionKind::View`. Arg id is `"value"` (not `"filter"`) — a top-level `UiNode::Select`
/// always dispatches its picked option merged into `args` under the `"value"` key (both renderers'
/// `Select` interpreters hardcode that key; see `with_item_value_arg` in ui_wgpu).
pub fn set_history_command_filter_action_definition() -> ActionDefinition {
    let options = vec![
        ActionArgOption::new("all", LocalizedLabel::native("All", "Alle")),
        ActionArgOption::new("withoutOperations", LocalizedLabel::native("Without Operations", "Ohne Operationen")),
        ActionArgOption::new("onlyOperations", LocalizedLabel::native("Only Operations", "Nur Operationen")),
    ];
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(
            SET_HISTORY_COMMAND_FILTER_ACTION_ID,
            LocalizedLabel::native("Set History Filter", "Verlaufsfilter festlegen"),
            ActionKind::View,
        )
    }
    .with_args([ActionArgDef::select("value", LocalizedLabel::native("Filter", "Filter"), options).default_value(serde_json::json!("all"))])
}

/// @emoji 🗒️ The framework-owned action id apps dispatch to note a shell effect (navigate, export,
/// spawn, …) into the session command log without any document mutation — mirrors
/// `SET_HISTORY_COMMAND_FILTER_ACTION_ID`'s auto-injected-constant pattern.
pub const NOTE_SHELL_COMMAND_ACTION_ID: &str = "noteShellCommand";

/// @emoji 🗒️ The framework-injected `noteShellCommand` Shell action (never in the palette): records a
/// shell-kind effect that already happened into the session command log, for effects dispatched
/// outside the normal `ActionDescriptor` path. `commandId` and `label` are required; `detail` is an
/// optional free-text elaboration shown in the history panel.
pub fn note_shell_command_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(
            NOTE_SHELL_COMMAND_ACTION_ID,
            LocalizedLabel::native("Note Shell Command", "Shell-Befehl vermerken"),
            ActionKind::Shell,
        )
    }
    .with_args([
        ActionArgDef::text("commandId", LocalizedLabel::native("Command", "Befehl")).required(),
        ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")).required(),
        ActionArgDef::text("detail", LocalizedLabel::native("Detail", "Detail")),
    ])
}

//#region 🔖️Clipboard
/// 🕹️ The three framework-owned Clipboard actions, auto-injected into every `AppDefinition` —
/// mirrors `history_action_definitions`. `paste` carries a staged `anchoring` choice (defaulting to
/// `original`) plus an optional `position` override, both consumed as a `PastePlacement`.
pub fn clipboard_action_definitions() -> Vec<ActionDefinition> {
    let anchoring_options = vec![
        ActionArgOption::new("original", LocalizedLabel::native("Original", "Original")),
        ActionArgOption::new("middle", LocalizedLabel::native("Middle", "Mitte")),
        ActionArgOption::new("centroid", LocalizedLabel::native("Centroid", "Schwerpunkt")),
        ActionArgOption::new("bottomLeft", LocalizedLabel::native("Bottom Left", "Unten links")),
        ActionArgOption::new("bottomRight", LocalizedLabel::native("Bottom Right", "Unten rechts")),
        ActionArgOption::new("topLeft", LocalizedLabel::native("Top Left", "Oben links")),
        ActionArgOption::new("topRight", LocalizedLabel::native("Top Right", "Oben rechts")),
    ];
    vec![
        ActionDefinition {
            keys: Some("mod+c".into()),
            ..ActionDefinition::new_catalog("copy", LocalizedLabel::native("Copy", "Kopieren"), ActionKind::Clipboard)
        },
        ActionDefinition {
            keys: Some("mod+x".into()),
            ..ActionDefinition::new_catalog("cut", LocalizedLabel::native("Cut", "Ausschneiden"), ActionKind::Clipboard)
        },
        ActionDefinition {
            keys: Some("mod+v".into()),
            ..ActionDefinition::new_catalog("paste", LocalizedLabel::native("Paste", "Einfügen"), ActionKind::Clipboard)
        }
        .with_args([
            ActionArgDef::select("anchor", LocalizedLabel::native("Anchoring", "Verankerung"), anchoring_options)
                .default_value(serde_json::json!("original")),
            ActionArgDef::vec3("position", LocalizedLabel::native("Position", "Position")),
        ]),
    ]
}
//#endregion 🔖️Clipboard

/// @emoji 🧰️ The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// @emoji 🧰️ The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(
            SET_ACTIVE_UTILITY_ACTION_ID,
            LocalizedLabel::native("Set Active Utility", "Aktives Hilfsmittel festlegen"),
            ActionKind::View,
        )
    }
    .with_args([
        ActionArgDef::text("utilityId", LocalizedLabel::native("Utility", "Hilfsmittel")).required(),
        ActionArgDef::text("windowKindId", LocalizedLabel::native("Window", "Fenster")),
    ])
}

/// @emoji 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — auto-injected
/// as a View action into any `AppDefinition` that declares tools (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const SET_ACTIVE_TOOL_ACTION_ID: &str = "setActiveTool";

/// @emoji 🛠️ The framework-injected `setActiveTool` View action (never in the palette): switches the
/// host-owned active tool of the active mode. Unlike `setActiveUtility` this takes no `windowKindId` —
/// tools are windowless, scoped to the whole mode.
pub fn set_active_tool_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(
            SET_ACTIVE_TOOL_ACTION_ID,
            LocalizedLabel::native("Set Active Tool", "Aktives Werkzeug festlegen"),
            ActionKind::View,
        )
    }
    .with_args([ActionArgDef::text("toolId", LocalizedLabel::native("Tool", "Werkzeug")).required()])
}

/// @emoji 🎓️ The framework-owned action id apps dispatch to (re)start an app's introduction —
/// auto-injected as a shell-intercepted View action into any
/// `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const START_INTRODUCTION_ACTION_ID: &str = "startIntroduction";

/// @emoji 🎓️ The framework-injected `startIntroduction` View action: fully shell-intercepted (never
/// forwarded to the program), it resets playback to the first step of `AppDefinition.introduction`.
/// Unlike ordinary app actions this stays out of the action palette because the shell exposes the
/// dedicated `Introduce App` command.
pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(START_INTRODUCTION_ACTION_ID, LocalizedLabel::native("Introduce App", "App vorstellen"), ActionKind::View)
    }
}

/// 📇️ A validated reference into an app's `AppDefinition.actions` registry — prevents windows/modes
/// from silently inheriting "every app action" by making the scoping explicit and typed. Distinct
/// from `kernel::ActionId` (a dispatched-invocation identifier); this one names a *declaration*.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct ActionRef(String);

impl ActionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ActionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ActionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

//#region 🔖️Utilities
/// @emoji 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
/// an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
/// host-owned session view state (`ViewModel.active_utility_id`), never a document field or VCS operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct UtilityDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🧺️ Visual ribbon collection this utility groups into; `None` = a flat top-level ribbon entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    /// 🖱️ CSS/winit cursor name applied to the window body while this utility is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub category: Option<ui_wgpu::wgpu::UtilityCategory>,
    /// 🚦️ Whether window-scoped actions stay enabled while this utility is active. Defaults to `false`
    /// (matching today's whitelist-based gating where an active utility suppresses the action panel);
    /// set `true` for passive view utilities (e.g. cad `cad.play.view.*`) that should not gate actions.
    #[serde(default)]
    pub allows_actions_while_active: bool,
}

impl UtilityDefinition {
    /// @emoji 🧰️ A utility with sensible defaults (no group/keys/cursor/category, gates actions while active).
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon_id: icon_id.into(),
            group: None,
            keys: None,
            cursor: None,
            category: None,
            allows_actions_while_active: false,
        }
    }
}

/// @emoji 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
/// `ActionRef`, scoping utilities to window kinds/modes with a typed, resolvable id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct UtilityRef(String);

impl UtilityRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for UtilityRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for UtilityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Utilities

//#region 🔖️Commands
/// @emoji 🗂️ Where a command is offered. There are no window-level commands — window-scoped verbs
/// stay `ActionDefinition`/`UtilityDefinition`; a command is scoped to the os shell, a program, an app, or
/// one of an app's modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum CommandScope {
    Os,
    Plugin,
    App,
    Mode,
}

/// @emoji 🎛️ Declares one command: a scoped, categorized verb offered in the footer command panel.
/// Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
/// `DocumentApp::handle_command`/`ActionEmit`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub scope: CommandScope,
    /// 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
    pub category: String,
    pub icon_id: IconName,
    /// 📝️ Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
    pub args: Vec<ActionArgDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    #[serde(default)]
    pub in_palette: bool,
}

impl CommandDefinition {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<LocalizedLabel>,
        scope: CommandScope,
        category: impl Into<String>,
        icon_id: impl Into<IconName>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            scope,
            category: category.into(),
            icon_id: icon_id.into(),
            args: Vec::new(),
            keys: None,
            in_palette: true,
        }
    }

    /// @emoji 🎛️ Declares a command whose icon is resolved from {@link catalog_command_icon_id}.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, scope: CommandScope, category: impl Into<String>) -> Self {
        let id = id.into();
        Self::new(id.clone(), label, scope, category, catalog_command_icon_id(&id))
    }

    /// @emoji 📝️ Attaches typed argument declarations to this command.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }
}

/// 🎛️ A validated reference into an app's `AppDefinition.commands` registry — the command mirror of
/// `ActionRef`/`UtilityRef`. Only ever names a `Mode`-scope command (see `ModeDefinition.commands`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct CommandRef(String);

impl CommandRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CommandRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CommandRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Commands

//#region 🔖️Tools
/// @emoji 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
/// Distinct from `UtilityDefinition` (a per-window pointer mode — a utility is a tool for a specific
/// window) and `CommandDefinition` (a fire-once verb): exactly one tool is active per app at a time,
/// and activation is host-owned session view state (`ViewModel.active_tool_id`), never a document
/// field or VCS operation. A tool's live options are supplied dynamically via `DocumentApp::tool_measures`,
/// keyed by tool id — not part of this static declaration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
}

impl ToolDefinition {
    /// @emoji 🛠️ A tool with sensible defaults (no keybinding).
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), keys: None }
    }
}

/// @emoji 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
/// `UtilityRef`/`CommandRef`, scoping tools to modes with a typed, resolvable id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct ToolRef(String);

impl ToolRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ToolRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Tools

//#region 🆔️ElementId
/// @emoji 🆔️ Whether `id` matches the renderer-agnostic UI element id grammar: dot-separated segments,
/// each starting with a lowercase letter and continuing with letters/digits only (camelCase, no
/// hyphens/underscores) — e.g. `framework.window.main.action.addLayer`. This id is the single
/// integration key across i18n, tooltips, hotkeys, command origin tracking, tutorials, E2E selectors,
/// and introduction anchors; each renderer maps it onto its own element (React → DOM `id` attribute,
/// wgpu → hit-target `control_id`), so no renderer-specific shape leaks into the grammar itself.
pub fn is_element_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.split('.').all(|segment| {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) if first.is_ascii_lowercase() => chars.all(|c| c.is_ascii_alphanumeric()),
            _ => false,
        }
    })
}

/// @emoji 🆔️ Normalizes arbitrary input (a domain object's own id, a free-text label, an already
/// grammar-safe word) into a single camelCase element-id segment: splits on `-`/`_`/` `/`.`, lowercases
/// the very first character, capitalizes the first character after each separator, and drops any other
/// non-alphanumeric character. Idempotent on input that is already a valid segment. Used as the last
/// resort by `child_element_id` when a child id is derived from something not already grammar-safe (e.g.
/// a runtime label) — prefer a real semantic key first, then this, then a numeric index.
pub fn element_id_segment(raw: &str) -> String {
    let mut segment = String::new();
    let mut capitalize_next = false;
    for ch in raw.chars() {
        if ch == '-' || ch == '_' || ch == ' ' || ch == '.' {
            capitalize_next = true;
            continue;
        }
        if !ch.is_ascii_alphanumeric() {
            continue;
        }
        if segment.is_empty() {
            segment.push(ch.to_ascii_lowercase());
        } else if capitalize_next {
            segment.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            segment.push(ch);
        }
    }
    segment
}

/// @emoji 🆔️ Derives a child element id by suffixing `parent` with one or more segments, each normalized
/// through `element_id_segment` — the hierarchical mechanism every composite element uses to name its
/// parts instead of a context/registry: `child_element_id("ui.chat", &["send"])` → `"ui.chat.send"`.
pub fn child_element_id(parent: &str, segments: &[&str]) -> String {
    let mut id = parent.to_string();
    for segment in segments {
        id.push('.');
        id.push_str(&element_id_segment(segment));
    }
    id
}

/// @emoji 🆔️ Element id of the app shell's navbar — singular, shell-owned chrome.
pub const UI_NAVBAR_ELEMENT_ID: &str = "ui.navbar";
/// @emoji 🆔️ Element id of the app shell's footer — singular, shell-owned chrome.
pub const UI_FOOTER_ELEMENT_ID: &str = "ui.footer";

/// @emoji 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`.
pub fn window_element_id(kind_id: &str) -> String {
    child_element_id("framework.window", &[kind_id])
}

/// @emoji 🆔️ Element id of a panel tab's uncollapsed panel body. `tab_id` is already a dotted
/// `PanelTabDefinition.id()` (e.g. `puzzle.catalogue`) — appended verbatim rather than through
/// `child_element_id`, which would collapse its dots into camelCase.
pub fn panel_tab_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

/// @emoji 🆔️ Alias id of the first draggable tree row inside a panel tab (document order within that
/// uncollapsed panel) — stamped via `data-element-alias` since no single tree row has a stable semantic
/// id at authoring time. Used to teach catalogue drag-and-drop without hardcoding a kind id.
pub fn panel_tab_first_draggable_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}.firstDraggable")
}
//#endregion 🆔️ElementId

//#region 🔖️Introduction
/// @emoji 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
/// first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
/// glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
/// document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionDefinition {
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub title: LocalizedLabel,
    pub steps: Vec<IntroductionStepDefinition>,
}

/// @emoji 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
/// raising extra elements above the glass veil and `interactions` completing the step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionStepDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub title: LocalizedLabel,
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub body: LocalizedLabel,
    /// 🎯️ The single element id raised above the glass, pulsing `data-introduced`, that the info box
    /// anchors to. `None` = a screen-style step: full veil, centered info box.
    #[serde(default)]
    pub introduce: Option<String>,
    /// 🕳️ Additional element ids raised above the glass — interactive, no pulse — e.g. every 3D window
    /// that accepts a catalogue drop while `introduce` teaches the drag source.
    #[serde(default)]
    pub show: Vec<String>,
    #[serde(default)]
    pub placement: IntroductionPlacement,
    /// ✅️ Interactions completing this step; empty means purely informational (Next-button-only).
    #[serde(default)]
    pub interactions: Vec<IntroductionInteraction>,
    /// 🔢️ Whether `interactions` must complete in declaration order — out-of-order completions are
    /// ignored. Unordered: the first incomplete matching interaction completes.
    #[serde(default)]
    pub ordered: bool,
    /// 🏛️ Institution/partner logos shown in the info box below the body — e.g. funding acknowledgements.
    #[serde(default)]
    pub logos: Vec<IntroductionLogo>,
    /// 🎬️ Ghost-cursor demonstrations played in order, one after another, then looping back to the first —
    /// e.g. a viewport step showing zoom, then pan, then orbit. When the step also declares `interactions`,
    /// `demonstrations[i]` previews `interactions[i]` and completed interactions are omitted from replay.
    /// Empty means no demonstration.
    #[serde(default)]
    pub demonstrations: Vec<IntroductionDemonstration>,
}

impl IntroductionStepDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, body: impl Into<LocalizedLabel>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            introduce: None,
            show: Vec::new(),
            placement: IntroductionPlacement::default(),
            interactions: Vec::new(),
            ordered: false,
            logos: Vec::new(),
            demonstrations: Vec::new(),
        }
    }

    /// @emoji 🎯️ Sets the single element id raised above the glass and anchoring the info box.
    pub fn introduce(mut self, element_id: impl Into<String>) -> Self {
        self.introduce = Some(element_id.into());
        self
    }

    /// @emoji 🕳️ Additional element ids raised above the glass alongside `introduce` (no pulse).
    pub fn show(mut self, element_ids: Vec<String>) -> Self {
        self.show = element_ids;
        self
    }

    /// @emoji 📍️ Overrides where the info box is placed relative to `introduce`.
    pub fn placement(mut self, placement: IntroductionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// @emoji ✅️ Makes the step complete when the user performs all `interactions` (any order) instead of
    /// pressing Next.
    pub fn interact(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self
    }

    /// @emoji 🔢️ Like `interact`, but `interactions` must complete in declaration order.
    pub fn interact_ordered(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self.ordered = true;
        self
    }

    /// @emoji 🏛️ Attaches institution/partner logos to the step's info box.
    pub fn logos(mut self, logos: Vec<IntroductionLogo>) -> Self {
        self.logos = logos;
        self
    }

    /// @emoji 🎬️ Attaches ghost-cursor demonstrations played in order, then looping back to the first.
    pub fn demonstrate(mut self, demonstrations: Vec<IntroductionDemonstration>) -> Self {
        self.demonstrations = demonstrations;
        self
    }
}

/// @emoji 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
/// URL pair (no DOM/CSS types), optionally linking out when clicked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionLogo {
    pub src: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_src: Option<String>,
    pub alt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

/// @emoji 📍️ Where the info box is placed relative to its anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// @emoji 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
/// complete as soon as the user activates that utility/tool, opens that panel tab, or expands that tree
/// section — teaching by doing. `Pan`/`Zoom`/`Orbit` complete on that camera-navigation gesture over the
/// 3D window named by the payload (a window-kind id) — classified from camera-state deltas by the shell
/// that renders the window, so only shells that render a 3D world (the React shell) can complete them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum IntroductionInteractionKind {
    /// 📇️ References `AppDefinition.actions`.
    Action(ActionRef),
    /// 🧰️ References `AppDefinition.utilities`.
    Utility(UtilityRef),
    /// 🛠️ References `AppDefinition.tools` (mode-level tools such as fill).
    Tool(ToolRef),
    /// 📑️ Shell panel tab id (e.g. `framework.panel.catalogue`) — completes when that panel opens.
    Panel(String),
    /// 🌲️ Tree section/item id (e.g. `puzzle3d-play-kinds.objects`) — completes when the user expands it.
    Expand(String),
    /// 🖐️ Completes when the user pans the named 3D window.
    Pan(String),
    /// 🔍️ Completes when the user zooms (scroll or dolly) the named 3D window.
    Zoom(String),
    /// 🌐️ Completes when the user orbits the named 3D window.
    Orbit(String),
}

/// @emoji ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
/// rendered as a checklist row in the info box and celebrated individually on completion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionInteraction {
    pub on: IntroductionInteractionKind,
    /// 🏷️ Short checklist label shown in the step's info box.
    pub label: String,
    /// 🎉️ Element id stamped `data-celebrated` on completion; `None` falls back to the step's `introduce`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub celebrate: Option<String>,
}

impl IntroductionInteraction {
    fn new(on: IntroductionInteractionKind, label: impl Into<String>) -> Self {
        Self { on, label: label.into(), celebrate: None }
    }

    /// @emoji 📇️ An interaction completing when the user activates action `id`.
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Action(ActionRef::new(id.into())), label)
    }

    /// @emoji 🧰️ An interaction completing when the user activates utility `id`.
    pub fn utility(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Utility(UtilityRef::new(id.into())), label)
    }

    /// @emoji 🛠️ An interaction completing when the user activates tool `id`.
    pub fn tool(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Tool(ToolRef::new(id.into())), label)
    }

    /// @emoji 📑️ An interaction completing when panel tab `id` opens.
    pub fn panel(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Panel(id.into()), label)
    }

    /// @emoji 🌲️ An interaction completing when tree section/item `id` expands.
    pub fn expand(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Expand(id.into()), label)
    }

    /// @emoji 🖐️ An interaction completing when the user pans 3D window `window_kind_id`.
    pub fn pan(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Pan(window_kind_id.into()), label)
    }

    /// @emoji 🔍️ An interaction completing when the user zooms 3D window `window_kind_id`.
    pub fn zoom(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Zoom(window_kind_id.into()), label)
    }

    /// @emoji 🌐️ An interaction completing when the user orbits 3D window `window_kind_id`.
    pub fn orbit(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Orbit(window_kind_id.into()), label)
    }

    /// @emoji 🎉️ Overrides which element id is stamped `data-celebrated` on completion.
    pub fn celebrate(mut self, element_id: impl Into<String>) -> Self {
        self.celebrate = Some(element_id.into());
        self
    }
}

/// @emoji 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
/// point type covers click targets and drag endpoints across every addressing scheme the shell needs:
/// element-relative, absolute/normalized screen space, absolute/normalized window(pane)-local space, and
/// a 3D scene world position projected through that window's live camera.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
// 🐢️ `rename_all_fields` is required alongside `rename_all` — the latter only renames the *variant* tag
// values; without the former, a future multi-word field inside a variant would silently serialize
// snake_case and desync from the generated TS type (see `UiDirtyScope`'s comment for the full story).
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionPoint {
    /// 🎯️ Center (or `offset`, normalized 0–1 within the element's rect) of the element `id` resolves to.
    Element {
        id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        offset: Option<[f64; 2]>,
    },
    /// 🖥️ Absolute viewport pixel.
    Screen { x: f64, y: f64 },
    /// 🖥️ Normalized 0–1 of the viewport.
    ScreenNormalized { x: f64, y: f64 },
    /// 🪟️ Pixel local to window/pane element `id`'s rect (top-left origin).
    Window { id: String, x: f64, y: f64 },
    /// 🪟️ Normalized 0–1 within window/pane element `id`'s rect.
    WindowNormalized { id: String, x: f64, y: f64 },
    /// 🧊️ 3D world-space position in the scene shown by window `id`, projected through its live camera.
    Scene { id: String, position: [f64; 3] },
    /// 🗺️ 2D world-space coordinates (camera x/y/zoom) on the infinite-canvas surface shown by window
    /// `id` — the 2D sibling of `Scene`. On a 3D window this resolves via the ground plane (z = 0).
    Canvas { id: String, x: f64, y: f64 },
    /// 🏷️ A live entity addressed semantically in the shell's established pick-target grammar (see
    /// `CanvasPickTarget`): `domain` is the surface's target domain ("vortex", "object", "attraction",
    /// "node", "edge", "handle", "position", "route", "block", "layer", …), `entity` its id verbatim
    /// (compound forms like `"objectId:vortexId"` or `"widgetId:port"` included; `"*"` = any — the
    /// surface picks a representative, nearest the viewport center). `offset` is normalized 0–1 within
    /// the entity's bounds, default center.
    Entity {
        id: String,
        domain: String,
        entity: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        offset: Option<[f64; 2]>,
    },
    /// 🪡️ A parametric point along an entity's curve geometry (an attraction segment, graph edge, ink
    /// stroke, or canvas path layer) — `t` in 0–1 by arc length.
    Curve { id: String, domain: String, entity: String, t: f64 },
    /// 🎚️ A value mapped through an entity's live value domain (e.g. a graph slider's min..max onto its
    /// track), resolved to the corresponding point along the entity's geometry.
    Domain { id: String, domain: String, entity: String, value: f64 },
}

impl IntroductionPoint {
    /// @emoji 🗺️ 2D world-space coordinates on the infinite-canvas surface shown by window `window_id`.
    pub fn canvas(window_id: impl Into<String>, x: f64, y: f64) -> Self {
        Self::Canvas { id: window_id.into(), x, y }
    }

    /// @emoji 🏷️ A specific entity by domain + id, centered (no `offset`).
    pub fn entity(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>) -> Self {
        Self::Entity { id: window_id.into(), domain: domain.into(), entity: entity.into(), offset: None }
    }

    /// @emoji 🏷️ Any entity in `domain` — the surface picks a representative, nearest the viewport center.
    pub fn any_entity(window_id: impl Into<String>, domain: impl Into<String>) -> Self {
        Self::entity(window_id, domain, "*")
    }

    /// @emoji 🪡️ A parametric point at `t` (0–1 by arc length) along an entity's curve geometry.
    pub fn curve(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, t: f64) -> Self {
        Self::Curve { id: window_id.into(), domain: domain.into(), entity: entity.into(), t }
    }

    /// @emoji 🎚️ A value mapped through an entity's live value domain (e.g. a slider's min..max).
    pub fn domain_value(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, value: f64) -> Self {
        Self::Domain { id: window_id.into(), domain: domain.into(), entity: entity.into(), value }
    }
}

/// @emoji 🖱️ Which mouse button a drag-like demonstration presses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionPointerButton {
    #[default]
    Left,
    Middle,
    Right,
}

/// @emoji ⌨️ Keyboard modifier held during a drag-like demonstration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionKeyModifier {
    Alt,
    Shift,
    Control,
    Meta,
}

fn introduction_pointer_button_left() -> IntroductionPointerButton {
    IntroductionPointerButton::Left
}

fn introduction_pointer_button_right() -> IntroductionPointerButton {
    IntroductionPointerButton::Right
}

fn introduction_orbit_default_modifiers() -> Vec<IntroductionKeyModifier> {
    vec![IntroductionKeyModifier::Alt]
}

/// @emoji 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
/// and performs the visual press/release affordance for the gesture kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
// 🐢️ `rename_all_fields` required alongside `rename_all` so `Scroll`'s `delta_y` field actually
// serializes/types as `deltaY` — see `IntroductionPoint`'s comment / `UiDirtyScope`'s for the full story.
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionGesture {
    LeftClick { at: IntroductionPoint },
    RightClick { at: IntroductionPoint },
    DoubleClick { at: IntroductionPoint },
    Drag {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_left", skip_serializing_if = "IntroductionPointerButton::is_left")]
        button: IntroductionPointerButton,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        modifiers: Vec<IntroductionKeyModifier>,
    },
    Scroll { at: IntroductionPoint, delta_y: f64 },
    /// 🌐️ A curved (not straight-line) drag around a pivot — camera orbit, distinct from `Drag`'s
    /// straight-line pan/reposition motion.
    Orbit {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_right", skip_serializing_if = "IntroductionPointerButton::is_right")]
        button: IntroductionPointerButton,
        #[serde(default = "introduction_orbit_default_modifiers", skip_serializing_if = "introduction_orbit_modifiers_is_default")]
        modifiers: Vec<IntroductionKeyModifier>,
    }
}

impl IntroductionPointerButton {
    fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }
}

fn introduction_orbit_modifiers_is_default(modifiers: &[IntroductionKeyModifier]) -> bool {
    modifiers == [IntroductionKeyModifier::Alt]
}

/// @emoji 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionCursor {
    #[default]
    Default,
    Pointer,
    Grab,
    Grabbing,
    Crosshair,
    Move,
}

/// @emoji 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
/// `IntroductionStepDefinition`. Plays only while the user's own pointer is idle — any real pointer
/// movement mutes it and restores the real cursor instantly; going idle again while the step is still
/// active replays it from the beginning. `cursor` overrides the glyph shown over the target; omitted, it
/// derives from `gesture` (clicks → pointer, drag → grab/grabbing).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionDemonstration {
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cursor: Option<IntroductionCursor>,
}

impl IntroductionDemonstration {
    /// @emoji 👆️ A left-click demonstration at `at`.
    pub fn left_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::LeftClick { at }, cursor: None }
    }

    /// @emoji 👆️ A right-click demonstration at `at`.
    pub fn right_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::RightClick { at }, cursor: None }
    }

    /// @emoji ✋️ A click-and-drag demonstration from `from` to `to`.
    pub fn drag(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self {
            gesture: IntroductionGesture::Drag { from, to, button: IntroductionPointerButton::Left, modifiers: vec![] },
            cursor: None,
        }
    }

    /// @emoji 🖲️ A scroll-wheel demonstration at `at`; `delta_y` sign conveys direction.
    pub fn scroll(at: IntroductionPoint, delta_y: f64) -> Self {
        Self { gesture: IntroductionGesture::Scroll { at, delta_y }, cursor: None }
    }

    /// @emoji 🌐️ A camera-orbit demonstration curving from `from` to `to`.
    pub fn orbit(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self {
            gesture: IntroductionGesture::Orbit {
                from,
                to,
                button: IntroductionPointerButton::Right,
                modifiers: vec![IntroductionKeyModifier::Alt],
            },
            cursor: None,
        }
    }
}
//#endregion 🔖️Introduction

//#region 🔖️Tutorial
/// @emoji 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
/// `IntroductionDefinition`. Where an introduction gates progression on the user performing an
/// interaction, a tutorial plays a multi-track recording (narration, video overlay, UI state, document
/// edits, camera, ghost-cursor gestures) against a sandboxed copy of the document while the user watches,
/// scrubs, or deviates and converges back. A *recording* IS a `TutorialDefinition` — the recorder simply
/// produces a densely-sampled one; nothing distinguishes a hand-authored tutorial from a captured one.
/// Distinct from the docs-tooltip `tutorial` link field in `ui/js/react`'s `UiLabelLeaf` (a URL into the
/// manual) — this is the interactive playback mechanism.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub description: Option<LocalizedLabel>,
    /// ⏱️ Total timeline length in milliseconds; every track entry's `at` (+ duration) must fit within.
    pub duration_ms: u64,
    /// 📖️ Scrub-bar markers, sorted ascending by `at`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chapters: Vec<TutorialChapter>,
    /// 🎬️ Starting conditions the player restores into its sandbox before t=0.
    pub base: TutorialBase,
    pub tracks: TutorialTracks,
    /// 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub recorded_at: Option<String>,
}

impl TutorialDefinition {
    /// @emoji 📂️ Deserializes a `TutorialDefinition` from its JSON wire format — the constructor apps use
    /// to load a hand-authored or recorded tutorial (e.g. via `include_str!`) into `.tutorial(...)`.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// @emoji 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialChapter {
    pub id: String,
    pub at: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub body: Option<LocalizedLabel>,
}

/// @emoji 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
/// state. The player snapshots the user's live document, loads this in its place, and restores the
/// snapshot on exit — a tutorial can never touch real work.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialBase {
    /// 📂️ Full document DSL text (`DocumentTextFiles.dsl`) to sandbox-load; `None` falls back to `example_id`, and both
    /// `None` falls back to the app's default/empty document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub document_dsl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub example_id: Option<String>,
    pub ui: TutorialUiSnapshot,
    /// 🎥️ Initial camera per window instance (every entry's `at` is `0`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cameras: Vec<TutorialCameraKeyframe>,
}

/// @emoji 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
/// millisecond offset from tutorial start, and each `Vec` is sorted ascending by `at`
/// (`validate_tutorial` enforces this).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialTracks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub narration: Vec<TutorialNarrationCue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<TutorialVideoCue>,
    /// 🏷️ Annotational only — drives affordance pulses and scrub-bar tick marks; playback never
    /// re-dispatches these into a plugin (see `TutorialEventKind`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<TutorialEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ui: Vec<TutorialUiKeyframe>,
    /// 🖋️ The sole source of document mutation during playback — see `TutorialDocumentEventKind`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document: Vec<TutorialDocumentEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub camera: Vec<TutorialCameraKeyframe>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gestures: Vec<TutorialGestureCue>,
}

/// @emoji 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
/// (content-addressed Blake3 hash + size + media type) — `framework/core` does not depend on
/// `semio-vcs`, so the shape is mirrored rather than reused; conversion between the two is
/// field-for-field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialAssetSrc {
    /// 🌐️ Static asset route (a brand's `assetsDir` or the shared `ui/asset` mount).
    Url { url: String },
    /// 🗄️ Content-addressed blob in the studio's `BlobStore`.
    Blob { hash: String, size: u64, media_type: String },
    /// 🧵️ Inline data URL — the recorder's default before a save destination is chosen.
    DataUrl { data: String },
}

fn tutorial_narration_default_rate() -> f64 {
    1.0
}

fn tutorial_rate_is_default(rate: &f64) -> bool {
    (*rate - 1.0).abs() < f64::EPSILON
}

/// @emoji 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
/// overrides TTS with a recorded take. The timeline is always the master clock — a still-speaking TTS
/// utterance is cancelled at the next cue's `at`; audio assets are seeked and rate-matched to the
/// playhead instead of played independently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialNarrationCue {
    pub id: String,
    pub at: u64,
    /// ⏱️ Audio duration when `audio` is set (recorder-measured); a rough TTS estimate otherwise — used
    /// for scrub-bar layout only, never to gate playback.
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub text: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub audio: Option<TutorialAssetSrc>,
    /// 🗣️ Web Speech API voice-name hint; ignored once `audio` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub voice: Option<String>,
    /// 🎚️ TTS/audio rate multiplier layered under the player's own playback-rate control.
    #[serde(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    pub rate: f64,
    /// 💬️ Timed caption sub-segments (offsets relative to this cue's `at`); empty means `text` is shown
    /// whole for the cue's `duration_ms`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub captions: Vec<TutorialCaption>,
}

/// @emoji 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialCaption {
    pub at: u64,
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub text: LocalizedLabel,
}

/// @emoji 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialOverlayRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for TutorialOverlayRect {
    /// 📌️ Bottom-right picture-in-picture, ~16:9.
    fn default() -> Self {
        Self { x: 0.72, y: 0.70, width: 0.24, height: 0.24 }
    }
}

/// @emoji 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialVideoCue {
    pub at: u64,
    pub duration_ms: u64,
    pub src: TutorialAssetSrc,
    #[serde(default)]
    pub rect: TutorialOverlayRect,
    /// 🔇️ True when narration carries the audio (a webcam take recorded muted).
    #[serde(default)]
    pub muted: bool,
    /// ⏩️ Seek offset into the source at cue start.
    #[serde(default)]
    pub source_offset_ms: u64,
}

/// @emoji 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialEvent {
    pub at: u64,
    pub kind: TutorialEventKind,
}

/// @emoji 🏷️ What one `TutorialEvent` annotates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialEventKind {
    /// 📇️ An `AppDefinition.actions` dispatch, with its effective args.
    Action {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
        args: Option<DslValue>,
    },
    /// 🎛️ A `CommandDefinition` dispatch.
    Command {
        command: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
        args: Option<DslValue>,
    },
    /// ⌨️ A keybinding press, display-only over the action it triggered.
    Key { keys: String },
}

/// @emoji 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
/// sparse list of changes since the previous sample.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialUiKeyframe {
    pub at: u64,
    pub sample: TutorialUiSample,
}

/// @emoji 🧮️ See `TutorialUiKeyframe`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiSample {
    Snapshot { state: TutorialUiSnapshot },
    Delta { changes: Vec<TutorialUiChange> }
}

/// @emoji 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
/// dock/panel/dialog state neither shell serializes today. Deliberately NOT a serialization of either
/// shell's internal store: each shell implements its own `captureUiSnapshot`/`applyUiSnapshot` against
/// this shape. Locale/terminology are excluded on purpose — a tutorial plays in the viewer's own locale.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialUiSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub focused_window_id: Option<String>,
    /// 🧰️ Mirrors `ViewModel.active_utility_by_window_id`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_tool_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub layout: Option<WindowLayout>,
    /// 📑️ Active tab id per panel group; groups absent from the map are collapsed/closed.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_panel_tab_by_group: std::collections::HashMap<String, String>,
    /// 🗂️ Opaque program vocabulary, verbatim `ViewModel.panel_json`/`selection_json`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub panel_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub selection_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub open_dialog_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expanded_tree_ids: Vec<String>,
    #[serde(default)]
    pub command_panel_open: bool,
}

/// @emoji 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
/// `TutorialUiSnapshot` to reconstruct state at any timeline offset without shipping a full snapshot at
/// every sample.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiChange {
    ActiveMode {
        id: String,
    },
    FocusedWindow {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        id: Option<String>,
    },
    /// 🧰️ `utility_id: None` deactivates — mirrors `SetActiveUtility` semantics.
    ActiveUtility {
        window_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        utility_id: Option<String>,
    },
    ActiveTool {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        id: Option<String>,
    },
    Layout {
        layout: WindowLayout,
    },
    /// 📑️ `tab_id: None` collapses/closes the group.
    PanelTab {
        group: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        tab_id: Option<String>,
    },
    PanelState {
        panel_json: String,
    },
    Selection {
        selection_json: String,
    },
    Dialog {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
        args: Option<DslValue>,
    },
    TreeExpansion {
        id: String,
        expanded: bool,
    },
    CommandPanel {
        open: bool,
    }
}

/// @emoji 🖋️ One document-track entry — mirrors `store::DocumentCommand` with `Mutation =
/// serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
/// diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
/// annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
/// (fresh ids/timestamps) and would double-apply against this track.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialDocumentEvent {
    pub at: u64,
    pub kind: TutorialDocumentEventKind,
}

/// @emoji 🖋️ See `TutorialDocumentEvent`. `Edit` carries both `forwards` and `backwards` operations
/// verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialDocumentEventKind {
    Edit {
        #[cfg_attr(feature = "typegen", ts(type = "unknown[]"))]
        forwards: Vec<DslValue>,
        #[cfg_attr(feature = "typegen", ts(type = "unknown[]"))]
        backwards: Vec<DslValue>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        coalesce_key: Option<String>,
    },
    Undo,
    Redo,
    Checkpoint {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        message: Option<String>,
    },
    CheckoutCheckpoint {
        checkpoint_id: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    /// 📂️ Wholesale document replacement (e.g. a mid-tutorial example switch) — full
    /// `DocumentEnvelope` JSON in both directions.
    Load {
        document_dsl: String,
        previous_dsl: String,
    }
}

fn tutorial_camera_up_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// @emoji 🎥️ One camera track keyframe for a specific window instance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialCameraKeyframe {
    pub at: u64,
    /// 🪟️ Window *instance* id (matches `ViewWindowInstance.id`).
    pub window_id: String,
    pub camera: TutorialCameraState,
    /// 🪄️ Easing INTO this keyframe from the previous one on the same window.
    #[serde(default)]
    pub easing: TutorialEasing,
}

/// @emoji 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
/// mirrors `Canvas2dScene`'s `cameraX`/`cameraY`/`zoom`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialCameraState {
    Orbit {
        position: [f64; 3],
        target: [f64; 3],
        #[serde(default = "tutorial_camera_up_z")]
        up: [f64; 3],
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        fov: Option<f64>,
    },
    Canvas {
        x: f64,
        y: f64,
        zoom: f64,
    }
}

/// @emoji 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum TutorialEasing {
    Linear,
    #[default]
    EaseInOut,
    /// 📌️ No interpolation — hold the previous pose until this keyframe, then snap.
    Hold,
}

/// @emoji 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
/// both shells already resolve/render `IntroductionGesture`/`IntroductionPoint`/`IntroductionCursor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TutorialGestureCue {
    pub at: u64,
    pub duration_ms: u64,
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cursor: Option<IntroductionCursor>,
}

/// @emoji 🎬️ The framework-owned action id apps dispatch to (re)start a tutorial — auto-injected as a
/// fully shell-intercepted View action into any `AppDefinition` that declares one (mirrors
/// `START_INTRODUCTION_ACTION_ID`). Distinct from an introduction: a tutorial takes a required
/// `tutorialId` argument since an app may declare more than one.
pub const START_TUTORIAL_ACTION_ID: &str = "startTutorial";

/// @emoji 🎬️ The framework-injected `startTutorial` View action: fully shell-intercepted, it sandboxes
/// the live document, loads the selected tutorial's `base`, and starts playback from t=0.
pub fn start_tutorial_action_definition(tutorials: &[TutorialDefinition]) -> ActionDefinition {
    let options = tutorials.iter().map(|t| ActionArgOption::new(t.id.clone(), t.title.clone())).collect();
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(START_TUTORIAL_ACTION_ID, LocalizedLabel::native("Play Tutorial", "Tutorial abspielen"), ActionKind::View)
    }
    .with_args([ActionArgDef::select("tutorialId", LocalizedLabel::native("Tutorial", "Tutorial"), options).required()])
}

/// @emoji ⏺️ The framework-owned action id that opens the tutorial recorder chrome — auto-injected into
/// EVERY `AppDefinition` (recording needs no app-side declaration at all).
pub const RECORD_TUTORIAL_ACTION_ID: &str = "recordTutorial";

/// @emoji ⏺️ The framework-injected `recordTutorial` View action: fully shell-intercepted, arms the
/// recorder against the live document (never a sandboxed copy — a recording IS the user's work).
pub fn record_tutorial_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new_catalog(RECORD_TUTORIAL_ACTION_ID, LocalizedLabel::native("Record Tutorial", "Tutorial aufzeichnen"), ActionKind::View)
    }
}

/// ⏱️ Real-time (not timeline-time, not rate-scaled) duration of the camera glide the player performs
/// when the user presses Play after deviating from an active tutorial's recorded state.
pub const TUTORIAL_CONVERGE_MS: u64 = 600;

//#region 🔖️TutorialEngine
/// @emoji ✅️ Structural validation shared by the plugin builder and both recorders before save: every
/// track sorted ascending by `at`, every entry within `[0, durationMs]`, chapter/narration-cue ids
/// unique, `base.cameras` all at `at == 0`. Does NOT check that referenced action/command/element ids
/// exist — the plugin builder's validation (which has the full `AppDefinition` in scope) does that.
pub fn validate_tutorial(def: &TutorialDefinition) -> Result<(), String> {
    fn sorted_by_at<T>(label: &str, items: &[T], at: impl Fn(&T) -> u64, duration_ms: u64) -> Result<(), String> {
        let mut last: Option<u64> = None;
        for item in items {
            let at = at(item);
            if at > duration_ms {
                return Err(format!("tutorial track `{label}` has an entry at {at}ms beyond durationMs {duration_ms}"));
            }
            if let Some(last) = last {
                if at < last {
                    return Err(format!("tutorial track `{label}` is not sorted ascending by `at` ({last}ms then {at}ms)"));
                }
            }
            last = Some(at);
        }
        Ok(())
    }

    sorted_by_at("chapters", &def.chapters, |c| c.at, def.duration_ms)?;
    sorted_by_at("narration", &def.tracks.narration, |c| c.at, def.duration_ms)?;
    sorted_by_at("video", &def.tracks.video, |c| c.at, def.duration_ms)?;
    sorted_by_at("events", &def.tracks.events, |e| e.at, def.duration_ms)?;
    sorted_by_at("ui", &def.tracks.ui, |k| k.at, def.duration_ms)?;
    sorted_by_at("document", &def.tracks.document, |e| e.at, def.duration_ms)?;
    sorted_by_at("camera", &def.tracks.camera, |k| k.at, def.duration_ms)?;
    sorted_by_at("gestures", &def.tracks.gestures, |c| c.at, def.duration_ms)?;

    let mut chapter_ids = std::collections::HashSet::new();
    for chapter in &def.chapters {
        if !chapter_ids.insert(chapter.id.as_str()) {
            return Err(format!("duplicate tutorial chapter id `{}`", chapter.id));
        }
    }
    let mut cue_ids = std::collections::HashSet::new();
    for cue in &def.tracks.narration {
        if !cue_ids.insert(cue.id.as_str()) {
            return Err(format!("duplicate tutorial narration cue id `{}`", cue.id));
        }
    }
    for camera in &def.base.cameras {
        if camera.at != 0 {
            return Err(format!("tutorial base camera keyframe for window `{}` must have at == 0", camera.window_id));
        }
    }
    Ok(())
}

fn tutorial_ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

fn tutorial_lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// @emoji 🎥️ Interpolates between two camera keyframes at timeline offset `at_ms` (clamped into
/// `[prev.at, next.at]`). Position/target/up/fov lerp componentwise; `Canvas.zoom` interpolates in log
/// space so zooming reads as constant visual speed. `next.easing` governs the curve; `Hold` snaps to
/// `prev` until `next.at`, then jumps. Mismatched camera kinds between the two keyframes (`Orbit` vs
/// `Canvas` on the same window) never interpolate — the result snaps to whichever side `t` is closer to.
pub fn interpolate_tutorial_camera(prev: &TutorialCameraKeyframe, next: &TutorialCameraKeyframe, at_ms: f64) -> TutorialCameraState {
    let span = (next.at as f64 - prev.at as f64).max(1.0);
    let raw = ((at_ms - prev.at as f64) / span).clamp(0.0, 1.0);
    let t = match next.easing {
        TutorialEasing::Linear => raw,
        TutorialEasing::EaseInOut => tutorial_ease_in_out(raw),
        TutorialEasing::Hold => {
            if raw >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
    };
    match (&prev.camera, &next.camera) {
        (
            TutorialCameraState::Orbit { position: p0, target: t0, up: u0, fov: f0 },
            TutorialCameraState::Orbit { position: p1, target: t1, up: u1, fov: f1 },
        ) => TutorialCameraState::Orbit {
            position: tutorial_lerp3(*p0, *p1, t),
            target: tutorial_lerp3(*t0, *t1, t),
            up: tutorial_lerp3(*u0, *u1, t),
            fov: match (f0, f1) {
                (Some(a), Some(b)) => Some(a + (b - a) * t),
                (Some(a), None) => Some(*a),
                (None, Some(b)) => Some(*b),
                (None, None) => None,
            },
        },
        (TutorialCameraState::Canvas { x: x0, y: y0, zoom: z0 }, TutorialCameraState::Canvas { x: x1, y: y1, zoom: z1 }) => {
            TutorialCameraState::Canvas { x: x0 + (x1 - x0) * t, y: y0 + (y1 - y0) * t, zoom: (z0.ln() + (z1.ln() - z0.ln()) * t).exp() }
        }
        _ => {
            if t < 0.5 {
                prev.camera.clone()
            } else {
                next.camera.clone()
            }
        }
    }
}

/// @emoji 🎥️ Finds the camera pose for `window_id` at `at_ms`: exact if `at_ms` lands on or before the
/// first keyframe (falling back to `base.cameras`), interpolated between the bracketing pair otherwise,
/// held at the last pose past the final keyframe. `None` when the window has no camera keyframes at all.
pub fn tutorial_camera_at(def: &TutorialDefinition, window_id: &str, at_ms: f64) -> Option<TutorialCameraState> {
    let keyframes: Vec<&TutorialCameraKeyframe> =
        def.base.cameras.iter().chain(def.tracks.camera.iter()).filter(|k| k.window_id == window_id).collect();
    let first = keyframes.first()?;
    if at_ms <= first.at as f64 {
        return Some(first.camera.clone());
    }
    for pair in keyframes.windows(2) {
        let (prev, next) = (pair[0], pair[1]);
        if at_ms <= next.at as f64 {
            return Some(interpolate_tutorial_camera(prev, next, at_ms));
        }
    }
    Some(keyframes.last().unwrap().camera.clone())
}

/// @emoji 🩹️ Applies one `TutorialUiChange` onto a `TutorialUiSnapshot` in place — the pure core both
/// `compose_tutorial_ui` and each shell's live director share.
pub fn apply_tutorial_ui_change(state: &mut TutorialUiSnapshot, change: &TutorialUiChange) {
    match change {
        TutorialUiChange::ActiveMode { id } => state.active_mode_id = Some(id.clone()),
        TutorialUiChange::FocusedWindow { id } => state.focused_window_id = id.clone(),
        TutorialUiChange::ActiveUtility { window_id, utility_id } => match utility_id {
            Some(id) => {
                state.active_utility_by_window_id.insert(window_id.clone(), id.clone());
            }
            None => {
                state.active_utility_by_window_id.remove(window_id);
            }
        },
        TutorialUiChange::ActiveTool { id } => state.active_tool_id = id.clone(),
        TutorialUiChange::Layout { layout } => state.layout = Some(layout.clone()),
        TutorialUiChange::PanelTab { group, tab_id } => match tab_id {
            Some(id) => {
                state.active_panel_tab_by_group.insert(group.clone(), id.clone());
            }
            None => {
                state.active_panel_tab_by_group.remove(group);
            }
        },
        TutorialUiChange::PanelState { panel_json } => state.panel_json = Some(panel_json.clone()),
        TutorialUiChange::Selection { selection_json } => state.selection_json = Some(selection_json.clone()),
        TutorialUiChange::Dialog { id, .. } => state.open_dialog_id = id.clone(),
        TutorialUiChange::TreeExpansion { id, expanded } => {
            if *expanded {
                if !state.expanded_tree_ids.iter().any(|existing| existing == id) {
                    state.expanded_tree_ids.push(id.clone());
                }
            } else {
                state.expanded_tree_ids.retain(|existing| existing != id);
            }
        }
        TutorialUiChange::CommandPanel { open } => state.command_panel_open = *open,
    }
}

/// @emoji 🧮️ Reconstructs the full `TutorialUiSnapshot` at `at_ms`: starts from `base.ui`, then the
/// latest `Snapshot` sample with `at <= at_ms` (if any, replacing the base), then replays every `Delta`
/// sample after that snapshot up to and including `at_ms`, in order. This is the one place seeking (and
/// the deviation-then-play converge step) source their target UI state.
pub fn compose_tutorial_ui(def: &TutorialDefinition, at_ms: f64) -> TutorialUiSnapshot {
    let mut state = def.base.ui.clone();
    let mut deltas: Vec<&TutorialUiChange> = Vec::new();
    for keyframe in &def.tracks.ui {
        if keyframe.at as f64 > at_ms {
            break;
        }
        match &keyframe.sample {
            TutorialUiSample::Snapshot { state: snapshot } => {
                state = snapshot.clone();
                deltas.clear();
            }
            TutorialUiSample::Delta { changes } => {
                deltas.extend(changes.iter());
            }
        }
    }
    for change in deltas {
        apply_tutorial_ui_change(&mut state, change);
    }
    state
}

/// @emoji ✂️ Everything a live director's tick from `from_ms` to `to_ms` must apply: annotational
/// events, document edits, and UI deltas within the half-open interval on the crossing direction (empty
/// when `from_ms == to_ms`). Backward direction (scrubbing left) reverses entry order so callers apply
/// each `TutorialDocumentEventKind::Edit`'s `backwards` ops from most-recent to least-recent. Plain Rust
/// struct (not ts-rs mirrored) — the TS port lives in `framework/renderer/react/index.tsx` and is pinned
/// to this one via shared golden fixtures, not a wasm call per frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TutorialSlice {
    pub forward: bool,
    pub events: Vec<TutorialEvent>,
    pub document: Vec<TutorialDocumentEvent>,
    pub ui_changes: Vec<TutorialUiChange>,
}

/// @emoji ✂️ Computes the `TutorialSlice` for advancing the playhead from `from_ms` to `to_ms` (`to_ms`
/// may be less than `from_ms` when scrubbing backward).
///
/// 🐢️ A `TutorialUiSample::Snapshot` crossed mid-slice is intentionally NOT flattened into deltas here:
/// recomposing state across a snapshot boundary is exactly what `compose_tutorial_ui` already does
/// correctly and cheaply. This function is for the live per-tick advance, which never spans a snapshot
/// in practice (ticks run far more often than the multi-second snapshot cadence); any caller that jumps
/// across a snapshot boundary (a seek/scrub) should call `compose_tutorial_ui` wholesale instead of
/// accumulating through this slice.
pub fn tutorial_slice(def: &TutorialDefinition, from_ms: f64, to_ms: f64) -> TutorialSlice {
    let forward = to_ms >= from_ms;
    let (lo, hi) = if forward { (from_ms, to_ms) } else { (to_ms, from_ms) };
    let in_range = |at: u64| (at as f64) > lo && (at as f64) <= hi;

    let mut events: Vec<TutorialEvent> = def.tracks.events.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut document: Vec<TutorialDocumentEvent> = def.tracks.document.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut ui_changes: Vec<TutorialUiChange> = Vec::new();
    for keyframe in def.tracks.ui.iter().filter(|k| in_range(k.at)) {
        if let TutorialUiSample::Delta { changes } = &keyframe.sample {
            ui_changes.extend(changes.iter().cloned());
        }
    }
    if !forward {
        events.reverse();
        document.reverse();
        ui_changes.reverse();
    }
    TutorialSlice { forward, events, document, ui_changes }
}
//#endregion 🔖️TutorialEngine
//#endregion 🔖️Tutorial

//#region 🔖️Dialog
/// @emoji 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
/// identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
/// presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
/// args; empty `args` degenerates to a message/confirm dialog. Opened only via
/// `HostEffect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DialogDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub title: LocalizedLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub body: Option<LocalizedLabel>,
    pub args: Vec<ActionArgDef>,
    /// 📇️ References `AppDefinition.actions` — dispatched with the merged effective args on submit.
    pub submit_action: ActionRef,
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub submit_label: LocalizedLabel,
    /// 📇️ Optional `AppDefinition.actions` reference dispatched on any dismissal (Escape, veil
    /// click, or the Cancel button).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cancel_action: Option<ActionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub cancel_label: Option<LocalizedLabel>,
}

impl DialogDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, submit_action: ActionRef) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: None,
            args: Vec::new(),
            submit_action,
            // 🌐️ "OK" is identical in both locales — a real (not placeholder) translation choice.
            submit_label: LocalizedLabel::native("OK", "OK"),
            cancel_action: None,
            cancel_label: None,
        }
    }

    /// @emoji 📝️ Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<LocalizedLabel>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// @emoji 🧾️ Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// @emoji ✅️ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// @emoji ❌️ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// @emoji 🚪️ Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}
//#endregion 🔖️Dialog

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🛠️ Tools available while this mode is active — references `AppDefinition.tools` ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub layout_id: Option<String>,
    /// 🎛️ Mode-scope commands active while this mode is active — references `AppDefinition.commands`
    /// ids (each of which must declare `scope: CommandScope::Mode`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandRef>,
}

/// 🚫️ A non-empty, order-preserving list — construction-time enforcement replaces what used to be a
/// runtime `assert!` deep inside `AppBuilder::build_definition`. The first entry is the implicit
/// fallback default when nothing else specifies one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Vec<T>", into = "Vec<T>", bound = "T: Clone + Serialize + serde::de::DeserializeOwned")]
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> Self {
        Self { first, rest: Vec::new() }
    }

    pub fn new(first: T, rest: Vec<T>) -> Self {
        Self { first, rest }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1]
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Chain<std::iter::Once<&'a T>, std::slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = String;
    fn try_from(mut values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err("expected a non-empty list, got zero entries".to_string());
        }
        let first = values.remove(0);
        Ok(Self { first, rest: values })
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    fn from(value: NonEmptyVec<T>) -> Self {
        std::iter::once(value.first).chain(value.rest).collect()
    }
}

/// 🚫️ Every app has at least one mode — `playbook/module/procedural` and any other single-purpose app
/// must declare an explicit mode (e.g. `"default"`) instead of the zero-mode state the type system
/// now makes unrepresentable.
pub type Modes = NonEmptyVec<ModeDefinition>;

/// 🚫️ Every app has at least one window kind — mirrors `Modes`, formerly a runtime `assert!` in
/// `AppBuilder::build_definition`.
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
    pub icon_id: IconName,
    /// 🎛️ Always-present chrome facets (was: separately-optional `measures`/`engagement`).
    #[serde(default)]
    pub options: WindowOptions,
    /// 📇️ Actions this window kind accepts — references `AppDefinition.actions` ids. Mandatory,
    /// may be empty, never absent; replaces the previous implicit "every app action applies to
    /// every window" behavior.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionRef>,
    /// 🧰️ Utilities this window kind accepts — references `AppDefinition.utilities` ids. Empty = no utilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilities: Vec<UtilityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub document_snapshot_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub output_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    /// 🧭️ The dock anchor this group defaults to. Groups only ever map to the four corner anchors —
    /// the four edge-middle anchors (`top-middle`/`right-middle`/`bottom-middle`/`left-middle`) start
    /// empty and are user-populated via drag-and-drop or a dock skeleton override, never via a `PanelGroup`.
    pub fn anchor(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "top-left",
            PanelGroup::Details => "top-right",
            PanelGroup::Display => "bottom-left",
            PanelGroup::Settings => "bottom-right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

/// 🌳️ Closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention that used to
/// live in the renderer: every panel tab is either a framework-predefined kind (compile-time
/// exhaustive) or an app-declared custom tab (open id, still required to be unique/non-empty,
/// validated at construction by `AppBuilder`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum PanelTabKind {
    WorkbenchCategory,
    DisplayCategory,
    DetailsCategory,
    SettingsCategory,
    DisplayWindows,
    DisplayLayout,
    SettingsGeneral,
    SettingsTheme,
    /// 🧩️ App-declared tab — id is app-namespaced (e.g. `"puzzle.catalogue"`).
    App(String),
}

impl PanelTabKind {
    /// 🔤️ Flat string key for code that needs one, e.g. React `key=` props.
    pub fn id_str(&self) -> &str {
        match self {
            PanelTabKind::WorkbenchCategory => "framework.category.workbench",
            PanelTabKind::DisplayCategory => "framework.category.display",
            PanelTabKind::DetailsCategory => "framework.category.details",
            PanelTabKind::SettingsCategory => "framework.category.settings",
            PanelTabKind::DisplayWindows => "framework.display.windows",
            PanelTabKind::DisplayLayout => "framework.display.layout",
            PanelTabKind::SettingsGeneral => "framework.settings.general",
            PanelTabKind::SettingsTheme => "framework.settings.theme",
            PanelTabKind::App(id) => id.as_str(),
        }
    }
}

/// 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub kind: PanelTabKind,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub group: PanelGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub body_key: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PanelTabDefinition>,
}

impl PanelTabDefinition {
    pub fn id(&self) -> &str {
        self.kind.id_str()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppDefinition {
    pub id: String,
    /// 🗣️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
    /// see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub document: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub icon_id: Option<IconName>,
    pub controller_id: String,
    /// 🚧️ `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
    /// format is a flat array — not the `{ first, rest }` shape ts-rs would infer from the struct
    /// fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
    /// `NonEmptyVec` itself.
    #[cfg_attr(feature = "typegen", ts(type = "ModeDefinition[]"))]
    pub modes: Modes,
    pub default_mode_id: String,
    /// 🚧️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
    #[cfg_attr(feature = "typegen", ts(type = "WindowKindDefinition[]"))]
    pub window_kinds: WindowKinds,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionDefinition>,
    /// 🧰️ The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilities: Vec<UtilityDefinition>,
    /// 🛠️ The mode-level tools this app exposes (referenced by `ModeDefinition.tools`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    /// 🎛️ App- and mode-scope commands this app exposes (referenced by `ModeDefinition.commands` for
    /// `Mode`-scope entries; `App`-scope entries apply whenever the app is focused).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub default_layout: Option<WindowLayout>,
    /// 🗣️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminologies: Vec<String>,
    /// 🗺️ Terminology id -> full replacement document path (product + app segments), e.g. "reuse" ->
    /// ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep `document` under that terminology.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub terminology_documents: std::collections::HashMap<String, Vec<String>>,
    /// 🎓️ This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub introduction: Option<IntroductionDefinition>,
    /// 🎬️ Recorded, timed walkthroughs this app declares — see `TutorialDefinition`. A brand's own
    /// `tutorials` (if any) are shown alongside these, never replacing them (unlike `introduction`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tutorials: Vec<TutorialDefinition>,
    /// 🗨️ The modal form dialogs this app can open via `HostEffect::OpenDialog`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dialogs: Vec<DialogDefinition>,
    /// 🔌️ This app's workflow input ports — see `crate::MediaPortSpec`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media_inputs: Vec<MediaPortSpec>,
    /// 🔌️ This app's workflow output ports — see `crate::MediaPortSpec`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media_outputs: Vec<MediaPortSpec>,
    /// 🗂️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
    /// `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
    /// populate it.
    #[serde(default)]
    pub config: ConfigSpec,
    /// 🎛️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
    /// waves populate it.
    #[serde(default)]
    pub command_grammar: CommandGrammar,
    /// 🔌️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
    /// `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
    /// onto this.
    #[serde(default)]
    pub io: AppIo,
}

/// 🧭️ Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

//#region 🔖️action-args
/// @emoji 🧮️ Computes the effective argument map for an action: for each declared arg, the staged value
/// if present, else its declared `default`, else omitted. Renderers stage edits locally and pass them
/// here; the contract enforcer ({@link VcsDocumentApp}) materializes defaults before dispatch so plugins
/// never re-implement default-filling.
pub fn effective_action_args(
    defs: &[ActionArgDef],
    staged: &DslValue,
) -> DslValue {
    if defs.is_empty() {
        return staged.clone();
    }
    let mut effective = Vec::new();
    for def in defs {
        if let Some(value) = staged.get(&def.id) {
            effective.push((def.id.clone(), value.clone()));
        } else if let Some(default) = &def.default {
            effective.push((def.id.clone(), default.clone()));
        }
    }
    DslValue::Object(effective)
}

/// @emoji ❗️ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// `Null`, or an empty string (covers a blank Text/Select/IconSelect); `false`, `0`, and `[]` are
/// valid values for Toggle/Number/Slider/Vec3 and never count as unset.
pub fn missing_required_args(
    defs: &[ActionArgDef],
    effective: &DslValue,
) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None | Some(DslValue::Null) => true,
            Some(DslValue::String(text)) => text.is_empty(),
            Some(_) => false,
        })
        .map(|def| def.id.clone())
        .collect()
}

/// @emoji 🚦️ Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility`/
/// `setActiveTool` (internal View actions wired to the utility bar/tool panel, never the panel).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History
        && action.id != SET_ACTIVE_UTILITY_ACTION_ID
        && action.id != SET_ACTIVE_TOOL_ACTION_ID
}

/// @emoji 📇️ Resolves the actions a window kind presents in its panel. Explicit `window_kind.actions`
/// refs resolve in declared order; additionally, any panel-eligible app action referenced by *no*
/// window kind is an "orphan" that appears on every window (the scoping fallback that prevents blank
/// panels mid-migration — Architecture Decision 8). A window that scopes nothing therefore shows every
/// orphan; once a plugin scopes an action to some window, it stops being an orphan and appears only
/// where scoped. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_window_actions<'a>(
    app: &'a AppDefinition,
    window_kind: &WindowKindDefinition,
) -> Vec<&'a ActionDefinition> {
    let referenced: std::collections::HashSet<&str> = app
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter().map(ActionRef::as_str))
        .collect();
    let mut resolved: Vec<&'a ActionDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for action_ref in &window_kind.actions {
        if let Some(action) = app.actions.iter().find(|action| action.id == action_ref.as_str()) {
            if seen.insert(action.id.as_str()) {
                resolved.push(action);
            }
        }
    }
    for action in &app.actions {
        if action_is_panel_eligible(action)
            && !referenced.contains(action.id.as_str())
            && seen.insert(action.id.as_str())
        {
            resolved.push(action);
        }
    }
    resolved
}

/// @emoji 🛠️ Resolves the tools the active mode presents, in declared order — references into
/// `AppDefinition.tools` via `ModeDefinition.tools`. Unlike `resolve_window_actions`, unresolvable or
/// unreferenced tools have no orphan fallback: tools are opt-in per mode, not automatically shown
/// everywhere. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_mode_tools<'a>(app: &'a AppDefinition, mode_id: &str) -> Vec<&'a ToolDefinition> {
    let Some(mode) = app.modes.iter().find(|mode| mode.id == mode_id) else {
        return Vec::new();
    };
    let mut resolved: Vec<&'a ToolDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for tool_ref in &mode.tools {
        if let Some(tool) = app.tools.iter().find(|tool| tool.id == tool_ref.as_str()) {
            if seen.insert(tool.id.as_str()) {
                resolved.push(tool);
            }
        }
    }
    resolved
}
//#endregion 🔖️action-args

/// 🪜️ Formats a canonical app document for chrome.
pub fn app_document_label(document: &[String]) -> String {
    document.join(" · ")
}

/// 🗺️ Resolves the document path effective under the active terminology; unknown/native ids fall back to `document`.
pub fn resolve_app_document<'a>(app: &'a AppDefinition, terminology: &str) -> &'a [String] {
    app.terminology_documents.get(terminology).map(Vec::as_slice).unwrap_or(&app.document)
}

/// 🗂️ Formats a window tab within its canonical app document, resolved under the active terminology
/// and `locale` (needed to resolve the now-`LocalizedLabel` `app.label` for the dedup comparison below).
pub fn app_window_document_label(app: &AppDefinition, terminology: &str, locale: Locale, window_label: &str) -> String {
    let mut document = resolve_app_document(app, terminology).to_vec();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.resolve(Terminology::parse(terminology).unwrap_or_default(), locale).trim().to_lowercase();
    if !normalized_window.is_empty()
        && normalized_window != normalized_app
        && document.last().is_none_or(|segment| segment.to_lowercase() != normalized_window)
    {
        document.push(normalized_window);
    }
    app_document_label(&document)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub document_json: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Contribution {
    /// 🧩️ A module contributing an extension block kind to a block-list (Blockly-like) builder host app.
    PlaybookBlockKind {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "blockKind"))]
        block_kind: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        #[cfg_attr(feature = "typegen", ts(rename = "defaultValueJson"))]
        default_value_json: String,
        #[cfg_attr(feature = "typegen", ts(rename = "paramsBodyKey"))]
        params_body_key: String,
        #[cfg_attr(feature = "typegen", ts(rename = "previewBodyKey"))]
        preview_body_key: String,
    },
    /// 🧩️ A sourcing module contributing a typology tree and catalogue object kinds to a sourcing host app.
    SourcingModule {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "moduleId"))]
        module_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[cfg_attr(feature = "typegen", ts(rename = "typologyJson"))]
        typology_json: String,
        #[cfg_attr(feature = "typegen", ts(rename = "kindsJson"))]
        kinds_json: String,
    },
    /// 🧩️ A machine catalog contributing workshop machines (with capabilities) to a process host app.
    ProcessMachines {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "moduleId"))]
        module_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[cfg_attr(feature = "typegen", ts(rename = "machinesJson"))]
        machines_json: String,
    },
    /// 🧩️ A flow extension manifest contributing operators to a flow-backed host app catalogue/registry.
    FlowExtension {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "extensionId"))]
        extension_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[cfg_attr(feature = "typegen", ts(rename = "manifestJson"))]
        manifest_json: String,
    },
    /// 🧩️ A forms question kind contributing params/preview slots to a forms host app.
    FormsQuestionKind {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "questionKind"))]
        question_kind: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        #[cfg_attr(feature = "typegen", ts(rename = "defaultValueJson"))]
        default_value_json: String,
        #[cfg_attr(feature = "typegen", ts(rename = "paramsBodyKey"))]
        params_body_key: String,
        #[cfg_attr(feature = "typegen", ts(rename = "previewBodyKey"))]
        preview_body_key: String,
    },
    /// 🧩️ A CAD computer contributing stat/property/import profiles to a CAD host.
    CadComputer {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "moduleId"))]
        module_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[cfg_attr(feature = "typegen", ts(rename = "computersJson"))]
        computers_json: String,
    },
    /// 🧩️ An imperative module contributing operators to an imperative path host.
    ImperativeModule {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "moduleId"))]
        module_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: IconName,
        #[cfg_attr(feature = "typegen", ts(rename = "manifestJson"))]
        manifest_json: String,
    },
}

/// 🧩️ One host-aggregated plugin contribution entry (`contributionsJson` wire shape).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ProgramContributionEntry {
    pub plugin_id: String,
    pub contribution: Contribution,
}

/// 📕️ Parses host-pushed `contributionsJson` into typed entries.
pub fn parse_contributions(json: &str) -> Vec<ProgramContributionEntry> {
    serde_json::from_str(json).unwrap_or_default()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<Contribution>,
    /// 🎛️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ViewModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_window_kind_id: Option<String>,
    /// 🧰️ Per-call overlay: the host-owned active utility for the window targeted by this `render`/`handle_action`
    /// call (`window_id`). On batched `refresh-ui`, the plugin stamps this from
    /// `active_utility_by_window_id` per window entry — never from the focused window alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_utility_id: Option<String>,
    /// 🧰️ Host-owned active utility per window **instance** (never a document field, never a VCS operation). The shell
    /// sends the full map on every refresh so plugins can build per-pane scene state; tools stay mode-wide via
    /// `active_tool_id`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
    /// 🛠️ The host-owned active tool of the active mode (never a document field, never a VCS operation) —
    /// mutually exclusive with `active_utility_id`: activating one clears the other (see the React
    /// shell's `onAction` interceptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_tool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub panel_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub contributions_json: Option<String>,
    /// 🗣️ Active UI locale; plugins resolve their own label set from this via `resolve_labels`/
    /// `app_labels!`. Non-optional — the shell always resolves one (see `initUiLocaleSync`/
    /// `detectShellLocale`) before the first `render`, so "nobody set the locale" is unrepresentable.
    #[serde(default)]
    pub locale: Locale,
    /// 🗣️ Active terminology id (`Native` default, or an app-declared alternative term set).
    #[serde(default)]
    pub terminology: Terminology,
    /// 🪟️ The window instance a `render`/`handle_action` call targets — programs key all per-window
    /// option state (grid, LOD, selection mode, …) off this, never off `active_window_kind_id`, so that
    /// two window instances of the same kind (e.g. split top/perspective panes) never share options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub window_id: Option<String>,
    /// 🪟️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
    /// `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub window_instances: Vec<ViewWindowInstance>,
}

/// 🪟️ One live window instance, as seen by a plugin: `id` is the instance id (equal to `window_kind_id`
/// for a base, unsplit window), `window_kind_id` is the `AppDefinition.windowKinds` entry it renders.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ViewWindowInstance {
    pub id: String,
    pub window_kind_id: String,
}

// 🎗️ `AppLabelsOverlay` (the stringly-typed, per-id runtime label-patch map) is deleted — manifest
// labels are now `LocalizedLabel` fields resolved directly via `.resolve(terminology, locale)`, so a
// separate locale-aware overlay merged in after the fact is no longer needed. Downstream callers
// (plugin crates' `DocumentApp::app_labels()`, the OS renderer's overlay-merge call sites) are
// follow-up work owned by other agents — left broken intentionally, out of scope here.

//#region 🔖️Kernel
#[path = "../🎠️kernel/🦀️component.rs"]
pub mod kernel;
//#endregion 🔖️Kernel

#[cfg(test)]
mod app_document_tests {
    use super::app_document_label;

    //#region 🔖️UiDirtyScopeTests
    /// 🐢️ Regression: `rename_all = "camelCase"` on an enum only renames *variant* names via `tag`, not
    /// the fields inside a struct variant — those need `rename_all_fields` too, or `Partial`'s fields
    /// silently serialize as snake_case (`window_bodies`) while the TS `UiDirtyScope` type expects
    /// camelCase (`windowBodies`), desyncing the wire contract without any compile-time signal.
    #[test]
    fn ui_dirty_scope_partial_serializes_fields_as_camel_case() {
        use crate::kernel::UiDirtyScope;
        let scope = UiDirtyScope::Partial {
            window_bodies: vec!["a".into()],
            panel_bodies: vec!["b".into()],
            utilities: true,
            tools: false,
            engagements: true,
            measures: false,
            labels: false,
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("\"windowBodies\""), "{json}");
        assert!(json.contains("\"panelBodies\""), "{json}");
        assert!(!json.contains("window_bodies"), "{json}");
        assert!(!json.contains("panel_bodies"), "{json}");
    }

    #[test]
    fn ui_dirty_scope_defaults_to_full() {
        use crate::kernel::UiDirtyScope;
        assert_eq!(UiDirtyScope::default(), UiDirtyScope::Full);
        assert_eq!(serde_json::to_string(&UiDirtyScope::Full).unwrap(), "{\"kind\":\"full\"}");
        // Absent from JSON (an older program that never sets it) must also deserialize to Full.
        #[derive(serde::Deserialize)]
        struct Wrapper {
            #[serde(default)]
            ui_scope: UiDirtyScope,
        }
        let parsed: Wrapper = serde_json::from_str("{}").unwrap();
        assert_eq!(parsed.ui_scope, UiDirtyScope::Full);
    }
    //#endregion UiDirtyScopeTests

    #[test]
    fn formats_app_document_for_chrome() {
        assert_eq!(
            app_document_label(&["semio".into(), "puzzle".into(), "3d".into()]),
            "semio · puzzle · 3d"
        );
    }

    //#region 🔖️ActionArgsAndUtilitiesTests
    use crate::ui::{
        app_window_document_label, child_element_id, effective_action_args, element_id_segment, is_element_id, missing_required_args,
        resolve_app_document, resolve_layout_for_mode, resolve_mode_tools,
        resolve_window_actions, ActionArgControl, ActionArgDef,
        ActionArgOption, ActionDefinition, ActionKind, ActionRef, AppDefinition, CommandDefinition, CommandRef,
        CommandScope, DialogDefinition, IntroductionCursor, IntroductionDemonstration, IntroductionGesture, LocalizedLabel, Locale, Terminology,
        IntroductionInteraction, IntroductionInteractionKind, IntroductionKeyModifier, IntroductionPoint, IntroductionPointerButton, IntroductionStepDefinition,
        Modes, NonEmptyVec, PanelGroup, PanelTabDefinition, PanelTabKind, ToolRef, UtilityDefinition, UtilityRef, WindowKindDefinition, WindowKinds,
        SET_ACTIVE_UTILITY_ACTION_ID, UI_NAVBAR_ELEMENT_ID, UI_FOOTER_ELEMENT_ID, window_element_id, panel_tab_element_id,
        panel_tab_first_draggable_element_id,
        compose_tutorial_ui, interpolate_tutorial_camera, record_tutorial_action_definition, start_tutorial_action_definition,
        tutorial_camera_at, tutorial_slice, validate_tutorial, TutorialAssetSrc, TutorialBase, TutorialCameraKeyframe, TutorialCameraState,
        TutorialChapter, TutorialDefinition, TutorialDocumentEvent, TutorialDocumentEventKind, TutorialEasing, TutorialEvent, TutorialEventKind,
        TutorialNarrationCue, TutorialTracks, TutorialUiChange, TutorialUiKeyframe, TutorialUiSample, TutorialUiSnapshot,
        RECORD_TUTORIAL_ACTION_ID, START_TUTORIAL_ACTION_ID,
    };
    use crate::ui::kernel::HostEffect;
    use dsl::DslValue;
    use serde_json::json;

    #[test]
    fn action_arg_def_builder_chain() {
        let arg = ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0)
            .required()
            .default_value(1.0)
            .describe("scale factor");
        assert_eq!(arg.id, "scale");
        assert!(arg.required);
        assert_eq!(arg.default, Some(dsl::to_dsl_value(&1.0f64).unwrap()));
        assert_eq!(arg.description.as_deref(), Some("scale factor"));
        assert!(matches!(arg.control, ActionArgControl::Slider { min, max, .. } if min == 0.0 && max == 4.0));
    }

    #[test]
    fn effective_args_prefer_staged_then_default() {
        let defs = vec![
            ActionArgDef::text("a", LocalizedLabel::data("A")).default_value("da"),
            ActionArgDef::text("b", LocalizedLabel::data("B")).default_value("db"),
            ActionArgDef::text("c", LocalizedLabel::data("C")),
        ];
        let staged = dsl::to_dsl_value(&serde_json::json!({ "a": "staged-a" })).unwrap();
        let effective = effective_action_args(&defs, &staged);
        assert_eq!(effective.get("a"), Some(&DslValue::String("staged-a".into())), "staged wins");
        assert_eq!(effective.get("b"), Some(&DslValue::String("db".into())), "default fills in");
        assert!(!effective.as_object().is_some_and(|o| o.iter().any(|(k, _)| k == "c")), "no staged, no default ⇒ omitted");
    }

    #[test]
    fn missing_required_args_treats_unset_select_as_missing() {
        let defs = vec![
            ActionArgDef::select("mode", LocalizedLabel::data("Mode"), vec![ActionArgOption::new("x", LocalizedLabel::data("X"))]).required(),
            ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")).required(),
        ];
        // Nothing staged, no defaults: both required ids are missing.
        let empty = DslValue::Object(Vec::new());
        let effective = effective_action_args(&defs, &empty);
        let missing = missing_required_args(&defs, &effective);
        assert!(missing.contains(&"mode".to_string()));
        assert!(missing.contains(&"flag".to_string()));

        let effective = dsl::to_dsl_value(&serde_json::json!({ "mode": "", "flag": false })).unwrap();
        let missing = missing_required_args(&defs, &effective);
        assert_eq!(missing, vec!["mode".to_string()], "empty-string select is unset; false toggle is set");
    }

    #[test]
    fn utility_definition_and_utility_ref_construction() {
        let utility = UtilityDefinition::new("brush", LocalizedLabel::data("Brush"), "paintbrush");
        assert_eq!(utility.id, "brush");
        assert!(!utility.allows_actions_while_active, "default gates actions while active");
        assert_eq!(UtilityRef::new("brush").as_str(), "brush");
        assert_eq!(UtilityRef::from("brush").as_str(), "brush");
    }

    fn app_with(actions: Vec<ActionDefinition>, window_actions: Vec<ActionRef>) -> AppDefinition {
        AppDefinition {
            id: "a".into(),
            label: LocalizedLabel::data("A"),
            document: vec!["semio".into(), "a".into()],
            icon_id: None,
            controller_id: "a".into(),
            modes: Modes::one(crate::ui::ModeDefinition {
                id: "edit".into(),
                label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: WindowKinds::one(WindowKindDefinition {
                id: "main".into(),
                label: LocalizedLabel::data("Main"),
                body_key: "a.main".into(),
                surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: window_actions,
                utilities: Vec::new(),
                params_schema: None,
                document_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions,
            utilities: vec![],
            tools: vec![],
            commands: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_documents: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: crate::ConfigSpec::empty(),
            command_grammar: crate::CommandGrammar::empty(),
            io: crate::AppIo::default(),
        }
    }

    #[test]
    fn resolve_window_actions_explicit_scoping() {
        let app = app_with(
            vec![
                ActionDefinition::new_catalog("add", LocalizedLabel::data("Add"), ActionKind::Mutation),
                ActionDefinition::new_catalog("remove", LocalizedLabel::data("Remove"), ActionKind::Mutation),
            ],
            vec![ActionRef::new("add")],
        );
        let window = app.window_kinds.first();
        let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
        // `add` is explicitly scoped here; `remove` is referenced by no window ⇒ orphan ⇒ also appears.
        assert_eq!(resolved, vec!["add", "remove"]);
    }

    #[test]
    fn resolve_window_actions_excludes_history_and_set_active_utility_orphans() {
        let app = app_with(
            vec![
                ActionDefinition::new_catalog("undo", LocalizedLabel::data("Undo"), ActionKind::History),
                crate::ui::set_active_utility_action_definition(),
                ActionDefinition::new_catalog("add", LocalizedLabel::data("Add"), ActionKind::Mutation),
            ],
            vec![],
        );
        let window = app.window_kinds.first();
        let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
        assert_eq!(resolved, vec!["add"], "history + setActiveUtility are never panel-eligible orphans");
        assert!(!resolved.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
    }

    fn app_with_modes_and_tools(mut modes: Vec<crate::ui::ModeDefinition>, tools: Vec<crate::ui::ToolDefinition>) -> AppDefinition {
        let mut app = app_with(vec![], vec![]);
        let first = modes.remove(0);
        app.modes = Modes::new(first, modes);
        app.tools = tools;
        app
    }

    #[test]
    fn resolve_mode_tools_declared_order() {
        let app = app_with_modes_and_tools(
            vec![crate::ui::ModeDefinition {
                id: "edit".into(),
                label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                tools: vec![ToolRef::new("fill"), ToolRef::new("brush")],
                layout_id: None,
                commands: Vec::new(),
            }],
            vec![
                crate::ui::ToolDefinition::new("brush", LocalizedLabel::data("Brush"), "paintbrush"),
                crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket"),
            ],
        );
        let resolved: Vec<&str> = resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect();
        assert_eq!(resolved, vec!["fill", "brush"], "resolves in the mode's declared ref order, not registry order");
    }

    #[test]
    fn resolve_mode_tools_isolates_other_modes() {
        let app = app_with_modes_and_tools(
            vec![
                crate::ui::ModeDefinition {
                    id: "edit".into(),
                    label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                    tools: vec![ToolRef::new("fill")],
                    layout_id: None,
                    commands: Vec::new(),
                },
                crate::ui::ModeDefinition {
                    id: "view".into(),
                    label: LocalizedLabel::data("View"),
                icon_id: "pencil".into(),
                    tools: Vec::new(),
                    layout_id: None,
                    commands: Vec::new(),
                },
            ],
            vec![crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket")],
        );
        assert_eq!(resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect::<Vec<_>>(), vec!["fill"]);
        assert!(resolve_mode_tools(&app, "view").is_empty(), "tools are opt-in per mode, no orphan fallback");
        assert!(resolve_mode_tools(&app, "nonexistent").is_empty());
    }

    #[test]
    fn resolve_mode_tools_skips_unresolvable_refs() {
        let app = app_with_modes_and_tools(
            vec![crate::ui::ModeDefinition {
                id: "edit".into(),
                label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                tools: vec![ToolRef::new("fill"), ToolRef::new("ghost")],
                layout_id: None,
                commands: Vec::new(),
            }],
            vec![crate::ui::ToolDefinition::new("fill", LocalizedLabel::data("Fill"), "paint-bucket")],
        );
        let resolved: Vec<&str> = resolve_mode_tools(&app, "edit").iter().map(|t| t.id.as_str()).collect();
        assert_eq!(resolved, vec!["fill"]);
    }

    #[test]
    fn resolve_layout_for_mode_prefers_named_then_default_then_none() {
        fn stack_layout(active: &str) -> ui_wgpu::wgpu::WindowLayout {
            ui_wgpu::wgpu::WindowLayout {
                root: ui_wgpu::wgpu::WindowLayoutRoot::Stack(ui_wgpu::wgpu::WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: None,
                    active_window_kind_id: Some(active.into()),
                    children: vec![],
                }),
            }
        }
        let mut app = app_with(vec![], vec![]);
        app.modes.first_mut().layout_id = Some("named".into());
        app.named_layouts.push(ui_wgpu::wgpu::NamedLayout {
            id: "named".into(),
            label: "Named".into(),
            icon_id: None,
            layout: stack_layout("main"),
            origin: "app".into(),
            group_path: None,
        });
        app.default_layout = Some(stack_layout("fallback"));

        assert_eq!(resolve_layout_for_mode(&app, "edit"), Some(stack_layout("main")), "named layout referenced by the mode wins");

        app.modes.first_mut().layout_id = Some("missing".into());
        assert_eq!(
            resolve_layout_for_mode(&app, "edit"),
            Some(stack_layout("fallback")),
            "unresolved named layout id falls back to default_layout"
        );

        app.default_layout = None;
        assert_eq!(resolve_layout_for_mode(&app, "edit"), None, "no named layout and no default_layout ⇒ none");
        assert_eq!(resolve_layout_for_mode(&app, "nonexistent"), None, "unknown mode id ⇒ none");
    }

    #[test]
    fn resolve_app_document_uses_terminology_override_else_falls_back_to_native_document() {
        let mut app = app_with(vec![], vec![]);
        app.terminology_documents.insert("de".into(), vec!["semio".into(), "a-de".into()]);
        assert_eq!(resolve_app_document(&app, "de"), ["semio".to_string(), "a-de".to_string()]);
        assert_eq!(resolve_app_document(&app, "native"), app.document.as_slice());
        assert_eq!(resolve_app_document(&app, "unregistered"), app.document.as_slice());
    }

    #[test]
    fn app_window_document_label_skips_empty_app_named_and_duplicate_trailing_window_labels() {
        let mut app = app_with(vec![], vec![]);
        app.label = LocalizedLabel::data("Draw"); // document (from `app_with`) already ends in "a"
        assert_eq!(app_window_document_label(&app, "native", Locale::En, "Layers"), "semio · a · layers");
        assert_eq!(app_window_document_label(&app, "native", Locale::En, ""), "semio · a", "empty window label appends nothing");
        assert_eq!(
            app_window_document_label(&app, "native", Locale::En, "Draw"),
            "semio · a",
            "window label equal to the app label appends nothing"
        );
        assert_eq!(
            app_window_document_label(&app, "native", Locale::En, "A"),
            "semio · a",
            "window label equal to the document's trailing segment appends nothing"
        );
    }

    #[test]
    fn non_empty_vec_index_iter_first_mut_and_try_from() {
        let mut list = NonEmptyVec::new(1i32, vec![2, 3]);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], 1);
        assert_eq!(list[2], 3);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
        *list.first_mut() = 10;
        assert_eq!(list[0], 10);

        let from_vec = NonEmptyVec::try_from(vec![9, 8]).unwrap();
        assert_eq!(*from_vec.first(), 9);
        let round_tripped: Vec<i32> = from_vec.into();
        assert_eq!(round_tripped, vec![9, 8]);

        let err = NonEmptyVec::<i32>::try_from(Vec::new()).unwrap_err();
        assert!(err.contains("non-empty"));
    }

    #[test]
    fn panel_group_anchor_and_as_str_cover_all_variants() {
        assert_eq!(PanelGroup::Workbench.anchor(), "top-left");
        assert_eq!(PanelGroup::Details.anchor(), "top-right");
        assert_eq!(PanelGroup::Display.anchor(), "bottom-left");
        assert_eq!(PanelGroup::Settings.anchor(), "bottom-right");
        assert_eq!(PanelGroup::Workbench.as_str(), "workbench");
        assert_eq!(PanelGroup::Settings.as_str(), "settings");
    }

    #[test]
    fn panel_tab_kind_id_str_covers_framework_and_app_variants() {
        assert_eq!(PanelTabKind::WorkbenchCategory.id_str(), "framework.category.workbench");
        assert_eq!(PanelTabKind::DisplayWindows.id_str(), "framework.display.windows");
        assert_eq!(PanelTabKind::App("puzzle.catalogue".into()).id_str(), "puzzle.catalogue");
        let tab = PanelTabDefinition {
            kind: PanelTabKind::App("puzzle.catalogue".into()),
            label: LocalizedLabel::data("Catalogue"),
            group: PanelGroup::Workbench,
            body_key: Some("puzzle.catalogue".into()),
            children: Vec::new(),
        };
        assert_eq!(tab.id(), "puzzle.catalogue");
    }

    #[test]
    fn action_definition_requires_and_serializes_args_field() {
        let action = ActionDefinition::new_catalog("x", LocalizedLabel::data("X"), ActionKind::Mutation);
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["args"], json!([]));
        assert!(serde_json::from_value::<ActionDefinition>(json!({
            "id": "x",
            "label": {"native": {"en": "X", "de": "X"}, "reuse": {"en": "X", "de": "X"}},
            "kind": "operation",
            "inPalette": true
        }))
        .is_err());
    }

    #[test]
    fn window_kind_deserializes_without_utilities_field() {
        let window: WindowKindDefinition = serde_json::from_str(
            r#"{"id":"main","label":{"native":{"en":"Main","de":"Main"},"reuse":{"en":"Main","de":"Main"}},"bodyKey":"a.main","surfaceKind":"canvas-2d","iconId":"pen-tool"}"#,
        )
        .unwrap();
        assert!(window.utilities.is_empty());
        assert!(window.actions.is_empty());
    }

    #[test]
    fn action_arg_control_serializes_tagged() {
        let control = ActionArgControl::Select { options: vec![ActionArgOption::new("x", LocalizedLabel::data("X"))] };
        let json = serde_json::to_string(&control).unwrap();
        assert!(json.contains("\"kind\":\"select\""), "tagged with kind: {json}");
        let round: ActionArgControl = serde_json::from_str(&json).unwrap();
        assert_eq!(round, control);
    }

    #[test]
    fn is_element_id_accepts_dotted_camel_case_and_rejects_the_rest() {
        assert!(is_element_id("framework.navbar"));
        assert!(is_element_id("ui.window.main.action.addLayer"));
        assert!(is_element_id("brush"));
        assert!(!is_element_id(""));
        assert!(!is_element_id("framework.display.save-label"));
        assert!(!is_element_id("Framework.navbar"));
        assert!(!is_element_id("framework..navbar"));
        assert!(!is_element_id("framework.navbar."));
    }

    #[test]
    fn element_id_segment_normalizes_and_is_idempotent() {
        assert_eq!(element_id_segment("world-orbit-projection"), "worldOrbitProjection");
        assert_eq!(element_id_segment("Some Name"), "someName");
        assert_eq!(element_id_segment("myUtilityId"), "myUtilityId");
        assert_eq!(element_id_segment("addLayer"), element_id_segment(&element_id_segment("addLayer")));
    }

    #[test]
    fn child_element_id_suffixes_and_normalizes_segments() {
        assert_eq!(child_element_id("ui.chat", &["send"]), "ui.chat.send");
        assert_eq!(child_element_id("ui.chat", &["message-row"]), "ui.chat.messageRow");
        assert_eq!(child_element_id("ui.tree", &["row", "3"]), "ui.tree.row.3");
    }

    #[test]
    fn introduction_step_serde_defaults() {
        let step: IntroductionStepDefinition = serde_json::from_str(
            r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#,
        )
        .unwrap();
        assert_eq!(step.introduce, None);
        assert!(step.show.is_empty());
        let json = serde_json::to_string(&step).unwrap();
        let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, step);

        let with_targets = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("The Viewport"), LocalizedLabel::data("…"))
            .introduce(window_element_id("puzzle3d-main"))
            .show(vec![window_element_id("puzzle3d-secondary")]);
        let json = serde_json::to_string(&with_targets).unwrap();
        assert!(json.contains("\"introduce\":\"framework.window.puzzle3dMain\""), "{json}");
        let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, with_targets);
    }

    #[test]
    fn element_id_authoring_helpers() {
        assert_eq!(window_element_id("puzzle3d-main"), "framework.window.puzzle3dMain");
        assert_eq!(panel_tab_element_id("framework.panel.catalogue"), "framework.panelTab.framework.panel.catalogue");
        assert_eq!(
            panel_tab_first_draggable_element_id("framework.panel.catalogue"),
            "framework.panelTab.framework.panel.catalogue.firstDraggable"
        );
        assert!(is_element_id(UI_NAVBAR_ELEMENT_ID));
        assert!(is_element_id(UI_FOOTER_ELEMENT_ID));
        assert!(is_element_id(&window_element_id("puzzle3d-main")));
        assert!(is_element_id(&panel_tab_element_id("framework.panel.catalogue")));
        assert!(is_element_id(&panel_tab_first_draggable_element_id("framework.panel.catalogue")));
    }

    #[test]
    fn introduction_interaction_kind_round_trips_tagged() {
        for (kind, tag) in [
            (IntroductionInteractionKind::Action(ActionRef::new("add")), "action"),
            (IntroductionInteractionKind::Utility(UtilityRef::new("brush")), "utility"),
            (IntroductionInteractionKind::Tool(ToolRef::new("fill")), "tool"),
            (IntroductionInteractionKind::Panel("framework.panel.catalogue".into()), "panel"),
            (IntroductionInteractionKind::Expand("puzzle3d-play-kinds.objects".into()), "expand"),
            (IntroductionInteractionKind::Pan("puzzle3d-main".into()), "pan"),
            (IntroductionInteractionKind::Zoom("puzzle3d-main".into()), "zoom"),
            (IntroductionInteractionKind::Orbit("puzzle3d-main".into()), "orbit"),
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
            let round: IntroductionInteractionKind = serde_json::from_str(&json).unwrap();
            assert_eq!(round, kind);
        }
    }

    #[test]
    fn introduction_interaction_round_trips_and_defaults() {
        let interaction = IntroductionInteraction::zoom("puzzle3d-main", "Zoom in");
        assert_eq!(interaction.celebrate, None);
        let json = serde_json::to_string(&interaction).unwrap();
        assert!(!json.contains("celebrate"), "{json}");
        let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
        assert_eq!(round, interaction);

        let with_celebrate = IntroductionInteraction::pan("puzzle3d-main", "Pan").celebrate(window_element_id("puzzle3d-main"));
        let json = serde_json::to_string(&with_celebrate).unwrap();
        assert!(json.contains("\"celebrate\":\"framework.window.puzzle3dMain\""), "{json}");
        let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
        assert_eq!(round, with_celebrate);

        let step: IntroductionStepDefinition = serde_json::from_str(
            r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#,
        )
        .unwrap();
        assert!(step.interactions.is_empty());
        assert!(!step.ordered);

        let with_interactions = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).interact_ordered(vec![
            IntroductionInteraction::zoom("puzzle3d-main", "Zoom"),
            IntroductionInteraction::pan("puzzle3d-main", "Pan"),
            IntroductionInteraction::orbit("puzzle3d-main", "Orbit"),
        ]);
        assert!(with_interactions.ordered);
        assert_eq!(with_interactions.interactions.len(), 3);
        let json = serde_json::to_string(&with_interactions).unwrap();
        let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, with_interactions);
    }

    #[test]
    fn introduction_point_round_trips_tagged_camel_case() {
        for (point, tag) in [
            (IntroductionPoint::Element { id: "transform".into(), offset: None }, "element"),
            (IntroductionPoint::Element { id: "transform".into(), offset: Some([0.25, 0.75]) }, "element"),
            (IntroductionPoint::Screen { x: 10.0, y: 20.0 }, "screen"),
            (IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.5 }, "screenNormalized"),
            (IntroductionPoint::Window { id: window_element_id("puzzle3d-main"), x: 40.0, y: 60.0 }, "window"),
            (IntroductionPoint::WindowNormalized { id: window_element_id("puzzle3d-main"), x: 0.5, y: 0.55 }, "windowNormalized"),
            (IntroductionPoint::Scene { id: window_element_id("puzzle3d-main"), position: [1.0, 2.0, 3.0] }, "scene"),
            (IntroductionPoint::Canvas { id: window_element_id("puzzle3d-main"), x: 12.0, y: 34.0 }, "canvas"),
            (IntroductionPoint::entity(window_element_id("puzzle3d-main"), "vortex", "seed-left-001:v0"), "entity"),
            (IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex"), "entity"),
            (IntroductionPoint::Entity { id: window_element_id("puzzle3d-main"), domain: "node".into(), entity: "add".into(), offset: Some([0.25, 0.75]) }, "entity"),
            (IntroductionPoint::curve(window_element_id("puzzle3d-main"), "attraction", "a1", 0.5), "curve"),
            (IntroductionPoint::domain_value(window_element_id("puzzle3d-main"), "slider", "fillCount", 3.0), "domain"),
        ] {
            let json = serde_json::to_string(&point).unwrap();
            assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
            let round: IntroductionPoint = serde_json::from_str(&json).unwrap();
            assert_eq!(round, point);
        }
        // 🏷️ "*" (any-entity wildcard) must round-trip byte-for-byte, not get normalized away.
        let wildcard = IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex");
        let json = serde_json::to_string(&wildcard).unwrap();
        assert!(json.contains("\"entity\":\"*\""), "{json}");
    }

    #[test]
    fn introduction_gesture_round_trips_tagged_camel_case() {
        let at = IntroductionPoint::Element { id: "tool.fill".into(), offset: None };
        for (gesture, tag) in [
            (IntroductionGesture::LeftClick { at: at.clone() }, "leftClick"),
            (IntroductionGesture::RightClick { at: at.clone() }, "rightClick"),
            (IntroductionGesture::DoubleClick { at: at.clone() }, "doubleClick"),
            (
                IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] },
                "drag",
            ),
            (IntroductionGesture::Scroll { at: at.clone(), delta_y: 100.0 }, "scroll"),
            (
                IntroductionGesture::Orbit {
                    from: at.clone(),
                    to: at.clone(),
                    button: IntroductionPointerButton::Right,
                    modifiers: vec![IntroductionKeyModifier::Alt],
                },
                "orbit",
            ),
        ] {
            let json = serde_json::to_string(&gesture).unwrap();
            assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
            let round: IntroductionGesture = serde_json::from_str(&json).unwrap();
            assert_eq!(round, gesture);
        }

        // 🐢️ `rename_all` on an enum renames only the variant tag, not fields *within* a struct variant —
        // `rename_all_fields` is required too, or this field would silently serialize snake_case
        // (`delta_y`) and desync from the generated TS type's camelCase `deltaY` (see `UiDirtyScope`).
        let scroll_json = serde_json::to_string(&IntroductionGesture::Scroll { at, delta_y: 100.0 }).unwrap();
        assert!(scroll_json.contains("\"deltaY\":100.0"), "{scroll_json}");
        assert!(!scroll_json.contains("delta_y"), "{scroll_json}");
    }

    #[test]
    fn introduction_gesture_drag_orbit_default_button_and_modifiers() {
        let at = IntroductionPoint::Element { id: "puzzle3d-main".into(), offset: None };
        let drag: IntroductionGesture = serde_json::from_str(r#"{"kind":"drag","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
        assert_eq!(
            drag,
            IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] }
        );
        let drag_json = serde_json::to_string(&drag).unwrap();
        assert!(!drag_json.contains("button"), "{drag_json}");
        assert!(!drag_json.contains("modifiers"), "{drag_json}");

        let orbit: IntroductionGesture = serde_json::from_str(r#"{"kind":"orbit","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
        assert_eq!(
            orbit,
            IntroductionGesture::Orbit {
                from: at.clone(),
                to: at.clone(),
                button: IntroductionPointerButton::Right,
                modifiers: vec![IntroductionKeyModifier::Alt],
            }
        );
        let orbit_json = serde_json::to_string(&orbit).unwrap();
        assert!(!orbit_json.contains("button"), "{orbit_json}");
        assert!(!orbit_json.contains("modifiers"), "{orbit_json}");

        let middle_drag = IntroductionGesture::Drag {
            from: at.clone(),
            to: at.clone(),
            button: IntroductionPointerButton::Middle,
            modifiers: vec![],
        };
        let middle_json = serde_json::to_string(&middle_drag).unwrap();
        assert!(middle_json.contains("\"button\":\"middle\""), "{middle_json}");
        let round: IntroductionGesture = serde_json::from_str(&middle_json).unwrap();
        assert_eq!(round, middle_drag);
    }

    #[test]
    fn introduction_demonstration_round_trips_and_defaults() {
        let at = IntroductionPoint::Element { id: "transform".into(), offset: None };
        let demo = IntroductionDemonstration::left_click(at.clone());
        assert_eq!(demo.cursor, None);
        let json = serde_json::to_string(&demo).unwrap();
        assert!(!json.contains("cursor"), "{json}");
        let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
        assert_eq!(round, demo);

        let with_cursor = IntroductionDemonstration {
            gesture: IntroductionGesture::Drag { from: at.clone(), to: at, button: IntroductionPointerButton::Left, modifiers: vec![] },
            cursor: Some(IntroductionCursor::Grabbing),
        };
        let json = serde_json::to_string(&with_cursor).unwrap();
        assert!(json.contains("\"cursor\":\"grabbing\""), "{json}");
        let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
        assert_eq!(round, with_cursor);

        let step: IntroductionStepDefinition = serde_json::from_str(
            r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#,
        )
        .unwrap();
        assert!(step.demonstrations.is_empty());
        let json = serde_json::to_string(&step).unwrap();
        assert!(json.contains("\"demonstrations\":[]"), "{json}");

        // 🎬️ A step can sequence several demonstrations (e.g. zoom, then pan, then orbit).
        let with_demos = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).demonstrate(vec![
            IntroductionDemonstration::scroll(IntroductionPoint::Screen { x: 400.0, y: 300.0 }, -100.0),
            IntroductionDemonstration::drag(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 400.0, y: 320.0 }),
            IntroductionDemonstration::orbit(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 500.0, y: 300.0 }),
        ]);
        assert_eq!(with_demos.demonstrations.len(), 3);
        let json = serde_json::to_string(&with_demos).unwrap();
        let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, with_demos);
    }

    //#region 🔖️TutorialTests
    fn minimal_tutorial() -> TutorialDefinition {
        TutorialDefinition {
            id: "welcome-tour".into(),
            title: LocalizedLabel::data("Welcome Tour"),
            description: None,
            duration_ms: 10_000,
            chapters: vec![TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Start"), body: None }],
            base: TutorialBase { document_dsl: None, example_id: Some("concrete-forest".into()), ui: TutorialUiSnapshot::default(), cameras: vec![] },
            tracks: TutorialTracks::default(),
            recorded_at: None,
        }
    }

    #[test]
    fn tutorial_definition_serde_defaults() {
        let json = r#"{"id":"t","title":{"native":{"en":"T","de":"T"},"reuse":{"en":"T","de":"T"}},"durationMs":1000,"base":{"ui":{}},"tracks":{}}"#;
        let def: TutorialDefinition = serde_json::from_str(json).unwrap();
        assert!(def.description.is_none());
        assert!(def.chapters.is_empty());
        assert!(def.tracks.narration.is_empty());
        assert!(def.tracks.document.is_empty());
        assert!(def.base.cameras.is_empty());
        let round = serde_json::to_string(&def).unwrap();
        let round: TutorialDefinition = serde_json::from_str(&round).unwrap();
        assert_eq!(round, def);
    }

    #[test]
    fn tutorial_asset_src_round_trips_tagged_camel_case() {
        for asset in [
            TutorialAssetSrc::Url { url: "https://example.test/clip.webm".into() },
            TutorialAssetSrc::Blob { hash: "abc123".into(), size: 42, media_type: "video/webm".into() },
            TutorialAssetSrc::DataUrl { data: "data:audio/webm;base64,AA==".into() },
        ] {
            let json = serde_json::to_string(&asset).unwrap();
            assert!(json.contains("\"kind\":"), "{json}");
            let round: TutorialAssetSrc = serde_json::from_str(&json).unwrap();
            assert_eq!(round, asset);
        }
        let json = serde_json::to_string(&TutorialAssetSrc::Blob { hash: "abc".into(), size: 1, media_type: "video/webm".into() }).unwrap();
        assert!(json.contains("\"mediaType\""), "field must be camelCase: {json}");
    }

    #[test]
    fn tutorial_event_kind_round_trips_tagged_camel_case() {
        let action = TutorialEventKind::Action { action: "addObjectKind".into(), args: Some(dsl::to_dsl_value(&serde_json::json!({"kindId": "beam"})).expect("tutorial action args")) };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"kind\":\"action\""), "{json}");
        let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
        assert_eq!(round, action);

        let key = TutorialEventKind::Key { keys: "mod+z".into() };
        let json = serde_json::to_string(&key).unwrap();
        let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
        assert_eq!(round, key);
    }

    #[test]
    fn tutorial_ui_change_round_trips_tagged_camel_case() {
        let change = TutorialUiChange::ActiveUtility { window_id: "puzzle3d-main".into(), utility_id: Some("transform".into()) };
        let json = serde_json::to_string(&change).unwrap();
        assert!(json.contains("\"windowId\":\"puzzle3d-main\""), "field must be camelCase: {json}");
        assert!(json.contains("\"utilityId\":\"transform\""), "field must be camelCase: {json}");
        let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
        assert_eq!(round, change);

        let tree = TutorialUiChange::TreeExpansion { id: "puzzle3d-play-kinds.objects".into(), expanded: true };
        let json = serde_json::to_string(&tree).unwrap();
        let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
        assert_eq!(round, tree);
    }

    #[test]
    fn tutorial_document_event_kind_round_trips_tagged_camel_case() {
        let edit = TutorialDocumentEventKind::Edit {
            forwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "translate"})).expect("tutorial forward operation")],
            backwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "translate", "inverse": true})).expect("tutorial backward operation")],
            description: Some("Move object".into()),
            coalesce_key: Some("camera".into()),
        };
        let json = serde_json::to_string(&edit).unwrap();
        assert!(json.contains("\"kind\":\"edit\""), "{json}");
        assert!(json.contains("\"coalesceKey\":\"camera\""), "field must be camelCase: {json}");
        let round: TutorialDocumentEventKind = serde_json::from_str(&json).unwrap();
        assert_eq!(round, edit);

        let undo = TutorialDocumentEventKind::Undo;
        let json = serde_json::to_string(&undo).unwrap();
        assert_eq!(json, r#"{"kind":"undo"}"#);
    }

    #[test]
    fn tutorial_camera_state_round_trips_tagged_camel_case() {
        let orbit = TutorialCameraState::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(50.0) };
        let json = serde_json::to_string(&orbit).unwrap();
        assert!(json.contains("\"kind\":\"orbit\""), "{json}");
        let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
        assert_eq!(round, orbit);

        let canvas = TutorialCameraState::Canvas { x: 1.0, y: 2.0, zoom: 3.0 };
        let json = serde_json::to_string(&canvas).unwrap();
        assert!(json.contains("\"kind\":\"canvas\""), "{json}");
        let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
        assert_eq!(round, canvas);
    }

    #[test]
    fn validate_tutorial_rejects_unsorted_and_out_of_range_tracks() {
        let mut def = minimal_tutorial();
        def.tracks.narration = vec![
            TutorialNarrationCue {
                id: "b".into(),
                at: 500,
                duration_ms: 100,
                text: LocalizedLabel::data("b"),
                audio: None,
                voice: None,
                rate: 1.0,
                captions: vec![],
            },
            TutorialNarrationCue {
                id: "a".into(),
                at: 100,
                duration_ms: 100,
                text: LocalizedLabel::data("a"),
                audio: None,
                voice: None,
                rate: 1.0,
                captions: vec![],
            },
        ];
        assert!(validate_tutorial(&def).is_err(), "unsorted narration must be rejected");

        let mut def = minimal_tutorial();
        def.tracks.narration = vec![TutorialNarrationCue {
            id: "a".into(),
            at: 999_999,
            duration_ms: 100,
            text: LocalizedLabel::data("a"),
            audio: None,
            voice: None,
            rate: 1.0,
            captions: vec![],
        }];
        assert!(validate_tutorial(&def).is_err(), "entry beyond durationMs must be rejected");

        let mut def = minimal_tutorial();
        def.chapters.push(TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Dup"), body: None });
        assert!(validate_tutorial(&def).is_err(), "duplicate chapter id must be rejected");

        let mut def = minimal_tutorial();
        def.base.cameras.push(TutorialCameraKeyframe {
            at: 5,
            window_id: "w".into(),
            camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 },
            easing: TutorialEasing::default(),
        });
        assert!(validate_tutorial(&def).is_err(), "base camera keyframe must be at == 0");

        assert!(validate_tutorial(&minimal_tutorial()).is_ok());
    }

    #[test]
    fn tutorial_camera_interpolation_lerps_position_and_target() {
        let prev = TutorialCameraKeyframe {
            at: 0,
            window_id: "w".into(),
            camera: TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(40.0) },
            easing: TutorialEasing::Linear,
        };
        let next = TutorialCameraKeyframe {
            at: 1000,
            window_id: "w".into(),
            camera: TutorialCameraState::Orbit { position: [10.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(60.0) },
            easing: TutorialEasing::Linear,
        };
        let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
        match mid {
            TutorialCameraState::Orbit { position, fov, .. } => {
                assert!((position[0] - 5.0).abs() < 1e-9, "expected midpoint lerp, got {position:?}");
                assert_eq!(fov, Some(50.0));
            }
            other => panic!("expected Orbit, got {other:?}"),
        }
        let start = interpolate_tutorial_camera(&prev, &next, 0.0);
        assert_eq!(start, prev.camera);
        let end = interpolate_tutorial_camera(&prev, &next, 1000.0);
        assert_eq!(end, next.camera);
    }

    #[test]
    fn tutorial_camera_interpolation_zooms_in_log_space() {
        let prev =
            TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear };
        let next =
            TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Linear };
        let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
        match mid {
            TutorialCameraState::Canvas { zoom, .. } => assert!((zoom - 2.0).abs() < 1e-9, "log-space midpoint of 1..4 is 2, got {zoom}"),
            other => panic!("expected Canvas, got {other:?}"),
        }
    }

    #[test]
    fn tutorial_camera_interpolation_hold_snaps_at_keyframe() {
        let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Hold };
        let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Hold };
        assert_eq!(interpolate_tutorial_camera(&prev, &next, 999.0), prev.camera);
        assert_eq!(interpolate_tutorial_camera(&prev, &next, 1000.0), next.camera);
    }

    #[test]
    fn tutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after() {
        let mut def = minimal_tutorial();
        def.tracks.camera = vec![
            TutorialCameraKeyframe { at: 100, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear },
            TutorialCameraKeyframe { at: 900, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }, easing: TutorialEasing::Linear },
        ];
        assert_eq!(tutorial_camera_at(&def, "w", 0.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }));
        assert_eq!(tutorial_camera_at(&def, "w", 10_000.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }));
        assert_eq!(tutorial_camera_at(&def, "other-window", 500.0), None);
    }

    #[test]
    fn compose_tutorial_ui_applies_snapshot_then_deltas() {
        let mut def = minimal_tutorial();
        def.base.ui.active_tool_id = Some("fill".into());
        def.tracks.ui = vec![
            TutorialUiKeyframe {
                at: 100,
                sample: TutorialUiSample::Snapshot { state: TutorialUiSnapshot { active_mode_id: Some("edit".into()), ..Default::default() } },
            },
            TutorialUiKeyframe { at: 200, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("brush".into()) }] } },
            TutorialUiKeyframe {
                at: 300,
                sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::PanelTab { group: "top-left".into(), tab_id: Some("catalogue".into()) }] },
            },
        ];
        // Before any sample: the base snapshot alone.
        let at_0 = compose_tutorial_ui(&def, 0.0);
        assert_eq!(at_0.active_tool_id, Some("fill".into()));
        assert_eq!(at_0.active_mode_id, None);
        // After the snapshot but before its deltas.
        let at_100 = compose_tutorial_ui(&def, 100.0);
        assert_eq!(at_100.active_mode_id, Some("edit".into()));
        assert_eq!(at_100.active_tool_id, None, "snapshot replaces the base wholesale");
        // After one delta.
        let at_200 = compose_tutorial_ui(&def, 250.0);
        assert_eq!(at_200.active_tool_id, Some("brush".into()));
        // After both deltas.
        let at_300 = compose_tutorial_ui(&def, 300.0);
        assert_eq!(at_300.active_tool_id, Some("brush".into()));
        assert_eq!(at_300.active_panel_tab_by_group.get("top-left"), Some(&"catalogue".to_string()));
    }

    #[test]
    fn tutorial_slice_forward_and_reverse_cross_document_events() {
        let mut def = minimal_tutorial();
        def.tracks.document = vec![
            TutorialDocumentEvent {
                at: 100,
                kind: TutorialDocumentEventKind::Edit {
                    forwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "add", "id": "a"})).expect("tutorial forward operation")],
                    backwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "remove", "id": "a"})).expect("tutorial backward operation")],
                    description: None,
                    coalesce_key: None,
                },
            },
            TutorialDocumentEvent {
                at: 200,
                kind: TutorialDocumentEventKind::Edit {
                    forwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "add", "id": "b"})).expect("tutorial forward operation")],
                    backwards: vec![dsl::to_dsl_value(&serde_json::json!({"op": "remove", "id": "b"})).expect("tutorial backward operation")],
                    description: None,
                    coalesce_key: None,
                },
            },
        ];
        let forward = tutorial_slice(&def, 0.0, 250.0);
        assert!(forward.forward);
        assert_eq!(forward.document.len(), 2);
        let TutorialDocumentEventKind::Edit { forwards, .. } = &forward.document[0].kind else { panic!("expected Edit") };
        assert_eq!(forwards[0].get("id").and_then(DslValue::as_str), Some("a"), "forward order applies oldest-first");

        let backward = tutorial_slice(&def, 250.0, 0.0);
        assert!(!backward.forward);
        assert_eq!(backward.document.len(), 2);
        let TutorialDocumentEventKind::Edit { backwards, .. } = &backward.document[0].kind else { panic!("expected Edit") };
        assert_eq!(backwards[0].get("id").and_then(DslValue::as_str), Some("b"), "backward order unwinds newest-first");

        let empty = tutorial_slice(&def, 250.0, 250.0);
        assert!(empty.document.is_empty());
    }

    #[test]
    fn tutorial_slice_partitions_events_document_and_ui_by_track() {
        let mut def = minimal_tutorial();
        def.tracks.events = vec![TutorialEvent { at: 50, kind: TutorialEventKind::Action { action: "setFillCount".into(), args: None } }];
        def.tracks.ui =
            vec![TutorialUiKeyframe { at: 50, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("fill".into()) }] } }];
        let slice = tutorial_slice(&def, 0.0, 100.0);
        assert_eq!(slice.events.len(), 1);
        assert_eq!(slice.ui_changes.len(), 1);
        assert!(slice.document.is_empty());
    }

    #[test]
    fn start_tutorial_action_definition_offers_declared_tutorials_as_select_options() {
        let action = start_tutorial_action_definition(std::slice::from_ref(&minimal_tutorial()));
        assert_eq!(action.id, START_TUTORIAL_ACTION_ID);
        assert!(!action.in_palette, "shell owns palette discovery via the dedicated Play Tutorial command");
        assert_eq!(action.args.len(), 1);
        assert!(action.args[0].required);
        match &action.args[0].control {
            ActionArgControl::Select { options } => {
                assert_eq!(options.len(), 1);
                assert_eq!(options[0].value, "welcome-tour");
            }
            other => panic!("expected Select control, got {other:?}"),
        }
    }

    #[test]
    fn record_tutorial_action_definition_is_shell_intercepted_and_out_of_palette() {
        let action = record_tutorial_action_definition();
        assert_eq!(action.id, RECORD_TUTORIAL_ACTION_ID);
        assert!(!action.in_palette);
        assert_eq!(action.kind, ActionKind::View);
    }
    //#endregion 🔖️TutorialTests

    #[test]
    fn dialog_definition_round_trips_camel_case_with_defaults() {
        let dialog = DialogDefinition::new("confirm-delete", LocalizedLabel::data("Delete?"), ActionRef::new("deleteSelection"));
        let json = serde_json::to_string(&dialog).unwrap();
        assert!(json.contains("\"args\":[]"), "{json}");
        assert!(json.contains("\"submitAction\":\"deleteSelection\""), "{json}");
        assert!(json.contains("\"submitLabel\":{\"native\":{\"de\":\"OK\",\"en\":\"OK\"}"), "{json}");
        assert!(!json.contains("cancelAction"), "omitted when unset: {json}");
        let round: DialogDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, dialog);
    }

    #[test]
    fn dialog_definition_builder_chain() {
        let dialog = DialogDefinition::new("addObject", LocalizedLabel::data("Add Object"), ActionRef::new("addObjectKind"))
            .body(LocalizedLabel::data("Choose a kind"))
            .args(vec![ActionArgDef::text("objectKind", LocalizedLabel::data("Kind"))])
            .submit_label(LocalizedLabel::data("Add"))
            .cancel_label(LocalizedLabel::data("Nevermind"))
            .on_cancel(ActionRef::new("closeDialog"));
        assert_eq!(dialog.body.as_ref().map(|b| b.resolve(Terminology::Native, Locale::En)), Some("Choose a kind"));
        assert_eq!(dialog.args.len(), 1);
        assert_eq!(dialog.submit_label.resolve(Terminology::Native, Locale::En), "Add");
        assert_eq!(dialog.cancel_label.as_ref().map(|c| c.resolve(Terminology::Native, Locale::En)), Some("Nevermind"));
        assert_eq!(dialog.cancel_action, Some(ActionRef::new("closeDialog")));
    }

    #[test]
    fn command_definition_round_trips_camel_case_with_defaults() {
        let command = CommandDefinition::new_catalog("os.setThemeId", LocalizedLabel::data("Set Theme"), CommandScope::Os, "appearance");
        let json = serde_json::to_string(&command).unwrap();
        assert!(json.contains("\"args\":[]"), "{json}");
        assert!(json.contains("\"scope\":\"os\""), "{json}");
        assert!(json.contains("\"category\":\"appearance\""), "{json}");
        assert!(json.contains("\"inPalette\":true"), "{json}");
        assert!(json.contains("\"iconId\":\"settings\""), "{json}");
        assert!(!json.contains("keys"), "omitted when unset: {json}");
        let round: CommandDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, command);
    }


    #[test]
    fn command_ref_only_resolves_mode_scope_commands() {
        // 🎛️ CommandScope has no Ord/discriminant helper beyond PartialEq — this pins the four
        // variants' camelCase wire tags so a future variant addition can't silently reorder them.
        for (scope, tag) in [
            (CommandScope::Os, "\"os\""),
            (CommandScope::Plugin, "\"plugin\""),
            (CommandScope::App, "\"app\""),
            (CommandScope::Mode, "\"mode\""),
        ] {
            assert_eq!(serde_json::to_string(&scope).unwrap(), tag);
        }
        assert_eq!(CommandRef::new("mode.focus").as_str(), "mode.focus");
        assert_eq!(CommandRef::from("mode.focus").as_str(), "mode.focus");
    }

    #[test]
    fn open_dialog_effect_round_trips_camel_case() {
        let effect = HostEffect::OpenDialog { dialog_id: "addObject".into(), args: None };
        let json = serde_json::to_string(&effect).unwrap();
        assert_eq!(json, r#"{"openDialog":{"dialogId":"addObject"}}"#);
        let round: HostEffect = serde_json::from_str(&json).unwrap();
        assert_eq!(round, effect);
    }

    #[test]
    fn dispatch_action_effect_round_trips_camel_case() {
        let effect = HostEffect::DispatchAction {
            action: "advanceReconstruction".into(),
            args: Some(dsl::to_dsl_value(&json!({"jobId": "job-1"})).expect("dispatch action args")),
            delay_ms: 250,
        };
        let json = serde_json::to_string(&effect).unwrap();
        assert_eq!(json, r#"{"dispatchAction":{"action":"advanceReconstruction","args":{"jobId":"job-1"},"delayMs":250}}"#);
        let round: HostEffect = serde_json::from_str(&json).unwrap();
        assert_eq!(round, effect);
        // `args` omitted entirely when unset, not serialized as `null`.
        let bare = HostEffect::DispatchAction { action: "tick".into(), args: None, delay_ms: 0 };
        let bare_json = serde_json::to_string(&bare).unwrap();
        assert!(!bare_json.contains("\"args\""), "omitted when unset: {bare_json}");
        assert_eq!(serde_json::from_str::<HostEffect>(&bare_json).unwrap(), bare);
    }

    #[test]
    fn request_file_open_effect_round_trips_multiple() {
        let effect = HostEffect::RequestFileOpen {
            accept: ".png,.jpg".into(),
            read_as: Some("dataUrl".into()),
            import_action: "importFramePayload".into(),
            multiple: true,
        };
        let json = serde_json::to_string(&effect).unwrap();
        assert!(json.contains("\"multiple\":true"), "{json}");
        let round: HostEffect = serde_json::from_str(&json).unwrap();
        assert_eq!(round, effect);
        // `multiple` defaults to false when absent from the wire (older callers/plugins).
        let defaulted: HostEffect = serde_json::from_str(
            r#"{"requestFileOpen":{"accept":".png","importAction":"importFramePayload"}}"#,
        )
        .unwrap();
        assert_eq!(
            defaulted,
            HostEffect::RequestFileOpen {
                accept: ".png".into(),
                read_as: None,
                import_action: "importFramePayload".into(),
                multiple: false,
            }
        );
    }

    #[test]
    fn request_media_frames_effect_round_trips_camel_case() {
        let effect = HostEffect::RequestMediaFrames {
            accept: "video/mp4,video/quicktime".into(),
            frame_action: "importVideoFramePayload".into(),
            done_action: "importVideoDone".into(),
            fallback_action: "importVideoBytesPayload".into(),
            sample_stride: 5,
            max_frames: 200,
            max_long_edge_px: 1600,
            fps_hint: 30.0,
            payload: None,
            args: Some(dsl::to_dsl_value(&json!({"streamId": "s1"})).expect("media frame args")),
        };
        let json = serde_json::to_string(&effect).unwrap();
        assert!(json.contains("\"requestMediaFrames\""), "{json}");
        assert!(json.contains("\"sampleStride\":5"), "{json}");
        assert!(json.contains("\"maxLongEdgePx\":1600"), "{json}");
        assert!(!json.contains("\"payload\""), "omitted when unset: {json}");
        let round: HostEffect = serde_json::from_str(&json).unwrap();
        assert_eq!(round, effect);
        // Numeric hints default to 0 (host-default) and `payload`/`args` may be entirely absent.
        let defaulted: HostEffect = serde_json::from_str(
            r#"{"requestMediaFrames":{"accept":"video/mp4","frameAction":"f","doneAction":"d","fallbackAction":"b"}}"#,
        )
        .unwrap();
        assert_eq!(
            defaulted,
            HostEffect::RequestMediaFrames {
                accept: "video/mp4".into(),
                frame_action: "f".into(),
                done_action: "d".into(),
                fallback_action: "b".into(),
                sample_stride: 0,
                max_frames: 0,
                max_long_edge_px: 0,
                fps_hint: 0.0,
                payload: None,
                args: None,
            }
        );
        // `payload`-carrying variant (drop-zone bytes already in memory, no picker needed).
        let with_payload = HostEffect::RequestMediaFrames {
            accept: "video/*".into(),
            frame_action: "f".into(),
            done_action: "d".into(),
            fallback_action: "b".into(),
            sample_stride: 1,
            max_frames: 0,
            max_long_edge_px: 0,
            fps_hint: 0.0,
            payload: Some("data:video/mp4;base64,AAAA".into()),
            args: None,
        };
        let payload_json = serde_json::to_string(&with_payload).unwrap();
        assert!(payload_json.contains("\"payload\":\"data:video/mp4;base64,AAAA\""), "{payload_json}");
        assert_eq!(serde_json::from_str::<HostEffect>(&payload_json).unwrap(), with_payload);
    }

    #[test]
    fn os_media_format_ply_and_las_round_trip() {
        use crate::mesh::OsMediaFormat;
        for (format, ext, mime, binary) in [
            (OsMediaFormat::Ply, "ply", "model/ply", false),
            (OsMediaFormat::Las, "las", "application/vnd.las", true),
        ] {
            assert_eq!(format.as_str(), ext);
            assert_eq!(format.mime_type(), mime);
            assert_eq!(format.is_binary(), binary);
            assert_eq!(OsMediaFormat::parse(ext), Some(format));
            let json = serde_json::to_string(&format).unwrap();
            assert_eq!(json, format!("\"{ext}\""));
            let round: OsMediaFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(round, format);
        }
    }
    //#endregion 🔖️ActionArgsAndUtilitiesTests

    #[cfg(feature = "typegen")]
    #[test]
    fn exports_typescript_bindings() {
        use ts_rs::TS;
        ui_wgpu::wgpu::IconName::export().unwrap();
        ui_wgpu::wgpu::ActionDescriptor::export().unwrap();
        ui_wgpu::wgpu::WindowLayoutWindowNode::export().unwrap();
        ui_wgpu::wgpu::WindowLayoutStackNode::export().unwrap();
        ui_wgpu::wgpu::WindowLayoutAxisNode::export().unwrap();
        ui_wgpu::wgpu::WindowLayoutChild::export().unwrap();
        ui_wgpu::wgpu::WindowLayoutRoot::export().unwrap();
        ui_wgpu::wgpu::WindowLayout::export().unwrap();
        ui_wgpu::wgpu::NamedLayout::export().unwrap();
        ui_wgpu::wgpu::component::layout::MeasureSelectItem::export().unwrap();
        ui_wgpu::wgpu::WindowMeasure::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementOption::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementInput::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementStatus::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementPossible::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementRingOption::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementToggleGroupOption::export().unwrap();
        ui_wgpu::wgpu::component::layout::WindowEngagementSelectItem::export().unwrap();
        ui_wgpu::wgpu::WindowEngagementControl::export().unwrap();
        ui_wgpu::wgpu::WindowEngagement::export().unwrap();
        ui_wgpu::wgpu::WindowEngagementSlot::export().unwrap();
        ui_wgpu::wgpu::WindowOptions::export().unwrap();
        ui_wgpu::wgpu::SurfaceKind::export().unwrap();
        ui_wgpu::wgpu::UtilityCategory::export().unwrap();
        // 🧭️ The shared element-state model + every `UiNode` variant struct (closing the gap that used
        // to leave these hand-mirrored in `framework/core/js/index.ts` — see 🔖️Presence/🔖️UiNode).
        // `UiNode`/`UiComponentSceneNode` themselves are NOT yet typegen-derived: the enum's
        // `ComponentScene` variant nests ~15 scene payload types (`Canvas2dScene`, `World3dScene`, …)
        // that would each need their own `ts_rs::TS` derive first — a large, separate mechanical pass,
        // out of scope here. `framework/core/js/index.ts` hand-writes the `UiNode` union stitching
        // these generated variant interfaces together until that follow-up lands.
        ui_wgpu::wgpu::UiState::export().unwrap();
        ui_wgpu::wgpu::UiStatus::export().unwrap();
        ui_wgpu::wgpu::UiPresence::export().unwrap();
        ui_wgpu::wgpu::UiDropOverlaySpec::export().unwrap();
        ui_wgpu::wgpu::UiTextNode::export().unwrap();
        ui_wgpu::wgpu::UiButtonNode::export().unwrap();
        ui_wgpu::wgpu::UiSeparatorNode::export().unwrap();
        ui_wgpu::wgpu::UiImageNode::export().unwrap();
        ui_wgpu::wgpu::UiInputNode::export().unwrap();
        ui_wgpu::wgpu::UiSelectItem::export().unwrap();
        ui_wgpu::wgpu::UiSelectNode::export().unwrap();
        ui_wgpu::wgpu::UiToggleNode::export().unwrap();
        ui_wgpu::wgpu::UiKeyValueEntry::export().unwrap();
        ui_wgpu::wgpu::UiKeyValueNode::export().unwrap();
        ui_wgpu::wgpu::UiSliderNode::export().unwrap();
        ui_wgpu::wgpu::UiNumberStepperNode::export().unwrap();
        ui_wgpu::wgpu::UiRingNode::export().unwrap();
        ui_wgpu::wgpu::UiIconSelectNode::export().unwrap();
        ui_wgpu::wgpu::UiControlNode::export().unwrap();
        ui_wgpu::wgpu::UiTreeItemAction::export().unwrap();
        ui_wgpu::wgpu::UiTreeItemNode::export().unwrap();
        ui_wgpu::wgpu::UiTreeSectionNode::export().unwrap();
        ui_wgpu::wgpu::UiTreeNode::export().unwrap();
        ui_wgpu::wgpu::UiExternalSlotNode::export().unwrap();
        // NOT exported (recursive through `UiNode`, itself not yet typegen-derived — see comment
        // above): UiStackNode, UiGroupNode, UiFieldNode, UiSectionNode, UiInspectorFieldGroup.
        crate::ui::Keybinding::export().unwrap();
        crate::ui::ActionKind::export().unwrap();
        crate::ui::ActionArgOption::export().unwrap();
        crate::ui::ActionArgControl::export().unwrap();
        crate::ui::ActionArgDef::export().unwrap();
        crate::ui::ActionDefinition::export().unwrap();
        crate::ui::ActionRef::export().unwrap();
        crate::ui::UtilityDefinition::export().unwrap();
        crate::ui::UtilityRef::export().unwrap();
        crate::ui::ToolDefinition::export().unwrap();
        crate::ui::ToolRef::export().unwrap();
        crate::ui::CommandScope::export().unwrap();
        crate::ui::CommandDefinition::export().unwrap();
        crate::ui::CommandRef::export().unwrap();
        crate::ui::ModeDefinition::export().unwrap();
        crate::ui::WindowKindDefinition::export().unwrap();
        crate::ui::PanelGroup::export().unwrap();
        crate::ui::PanelTabKind::export().unwrap();
        crate::ui::PanelTabDefinition::export().unwrap();
        crate::ui::IntroductionDefinition::export().unwrap();
        crate::ui::IntroductionStepDefinition::export().unwrap();
        crate::ui::IntroductionPlacement::export().unwrap();
        crate::ui::IntroductionInteractionKind::export().unwrap();
        crate::ui::IntroductionInteraction::export().unwrap();
        crate::ui::IntroductionLogo::export().unwrap();
        crate::ui::IntroductionPoint::export().unwrap();
        crate::ui::IntroductionPointerButton::export().unwrap();
        crate::ui::IntroductionKeyModifier::export().unwrap();
        crate::ui::IntroductionGesture::export().unwrap();
        crate::ui::IntroductionCursor::export().unwrap();
        crate::ui::IntroductionDemonstration::export().unwrap();
        crate::ui::TutorialDefinition::export().unwrap();
        crate::ui::TutorialChapter::export().unwrap();
        crate::ui::TutorialBase::export().unwrap();
        crate::ui::TutorialTracks::export().unwrap();
        crate::ui::TutorialAssetSrc::export().unwrap();
        crate::ui::TutorialNarrationCue::export().unwrap();
        crate::ui::TutorialCaption::export().unwrap();
        crate::ui::TutorialOverlayRect::export().unwrap();
        crate::ui::TutorialVideoCue::export().unwrap();
        crate::ui::TutorialEvent::export().unwrap();
        crate::ui::TutorialEventKind::export().unwrap();
        crate::ui::TutorialUiKeyframe::export().unwrap();
        crate::ui::TutorialUiSample::export().unwrap();
        crate::ui::TutorialUiSnapshot::export().unwrap();
        crate::ui::TutorialUiChange::export().unwrap();
        crate::ui::TutorialDocumentEvent::export().unwrap();
        crate::ui::TutorialDocumentEventKind::export().unwrap();
        crate::ui::TutorialCameraKeyframe::export().unwrap();
        crate::ui::TutorialCameraState::export().unwrap();
        crate::ui::TutorialEasing::export().unwrap();
        crate::ui::TutorialGestureCue::export().unwrap();
        crate::ui::DialogDefinition::export().unwrap();
        crate::ui::AppDefinition::export().unwrap();
        crate::ui::ExampleDefinition::export().unwrap();
        crate::ui::Contribution::export().unwrap();
        crate::ui::ProgramContributionEntry::export().unwrap();
        crate::ui::PluginManifest::export().unwrap();
        crate::ui::ViewWindowInstance::export().unwrap();
        crate::ui::ViewModel::export().unwrap();
        // 🎗️ `AppLabelsOverlay` deleted — see the region comment at its former definition site.
        crate::ui::kernel::CapabilityRequirement::export().unwrap();
        crate::ui::kernel::Rights::export().unwrap();
        crate::ui::kernel::ArtifactKind::export().unwrap();
        crate::ui::kernel::Scope::export().unwrap();
        crate::mesh::OsMediaFormat::export().unwrap();
        crate::mesh::OsMediaCapability::export().unwrap();
        crate::mesh::ArtifactKindSpec::export().unwrap();
        crate::mesh::MediaClass::export().unwrap();
        crate::mesh::MediaForm::export().unwrap();
        crate::mesh::MediaType::export().unwrap();
        crate::mesh::MediaWireFormat::export().unwrap();
        crate::mesh::MediaPortDirection::export().unwrap();
        crate::mesh::PortMultiplicity::export().unwrap();
        crate::mesh::MediaPortSpec::export().unwrap();
    }
}
//#endregion 🔖️Manifest

// #endregion 🛂️Manifest
