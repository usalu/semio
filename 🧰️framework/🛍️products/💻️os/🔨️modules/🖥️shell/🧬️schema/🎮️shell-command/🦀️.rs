//! 🎮️ Shell command of the shell schema: the command vocabulary every host dispatches. See [`super`].

use super::*;

//#region 🎮️ShellCommand
/// 🎮️ The vocabulary the LLM (and every other shell client) speaks. Every variant is plain
/// serializable data — no `Updatable<T>`/functional-updater payload (a React idiom that cannot
/// cross a wire); every Record/HashMap-shaped mutation from the TS reducer is decomposed into a
/// single-key upsert/remove command (`value: None` removes) rather than a whole-map replacement,
/// which is both wire-safe and small-diff-friendly for an LLM caller.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellCommand {
    RegisterLoadedPlugin { plugin: LoadedPlugin },
    UnregisterLoadedPlugin { plugin_id: String },
    SetPluginStatus { plugin_id: String, status: PluginPanelStatus },
    SetPluginSupervisorState { plugin_id: String, state: PluginSupervisorState },
    SetActiveSession { session: Option<ActiveSession> },
    SetSessionError { error: Option<String> },

    SetAppLabelOverride { app_id: String, label_key: String, value: Option<String> },

    SetActionPaneFolded { window_id: String, folded: bool },
    SetActionPaneExpanded { window_id: String, action_id: Option<String> },
    StageActionArg { window_id: String, action_id: String, arg_id: String, value: JsonValue },
    ResetActionArgs { window_id: String, action_id: String },
    SetActiveUtility { window_id: String, utility_id: Option<String> },
    SetActiveTool { tool_id: Option<String> },

    SetCommandExpanded { command_id: Option<String> },
    StageCommandArg { command_id: String, arg_id: String, value: JsonValue },
    ResetCommandArgs { command_id: String },

    SetPanelVisible { anchor: Anchor, visible: bool },
    SetPanelSize { anchor: Anchor, size: f32 },
    SetPanelPath { anchor: Anchor, path: Vec<String> },
    SetDockOverride { dock: Option<LayoutNode> },
    SetPanelPathMemory { panel_key: String, path: Option<String> },
    SetTreeOpenState { tree_id: String, open: bool },
    HydrateDockUi { dock: Option<DockUiState> },
    ResetDock,
    FocusWindow { window_id: Option<String> },
    SetShellLayout { layout: Option<LayoutNode> },
    SetActiveExample { example_id: String },
    SetMobilePanelPath { path: Vec<String> },
    SetMobilePanelVisible { visible: bool },
    SetExtraWindows { windows: Vec<ExtraWindowInstance> },
    SetWindowTitle { window_id: String, title: String },
    SetWindowIcon { window_id: String, icon: IconName },

    SetSearchOpen { open: bool },
    SetFindOpen { open: bool },
    AutoStartIntroduction { key: String },
    SetIntroductionStep { step_index: Option<u32> },
    CompleteIntroductionInteraction { interaction_index: u32 },
    OpenDialog { dialog_id: String, seed_args: Option<JsonValue> },
    CloseDialog { dialog_id: Option<String> },
    ShowTransientNotice { notice: TransientNotice },
    DismissTransientNotice,
    SetOpenWithFocusRole { role: Option<AppRole> },

    SetActiveTutorial { tutorial_id: Option<String> },

    SetUiDriverDraft { draft: Option<UiDriver> },
    SetUiThemeDraft { draft: Option<UiTheme> },

    SetSyncBackboneUri { uri: Option<String> },
    SetSyncCardKind { kind: Option<SyncCardKind> },
    SetSyncDraftPath { path: String },
    SetDocumentSyncStatus { document_id: String, status: ArtifactSyncStatus },

    SetDocumentInferencePort { document_id: String, port: InferencePortStatus },
    ClearDocumentInferencePort { document_id: String },

    SetMergePolicy { policy: MergePolicy },
    SetConflicts { conflicts: Vec<Conflict> },
    SelectConflict { conflict_id: Option<String> },

    SetStorageScope { scope: ShellScope },
    SetOpeningPreference { role: String, dialect_id: Option<String> },
}
//#endregion 🎮️ShellCommand
