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
    // ── Plugin runtime — audit UPSERT_LOADED_PLUGIN/REMOVE_LOADED_PLUGIN/SET_PLUGIN_STATUS/SET_PLUGIN_SUPERVISOR/SET_SESSION/SET_ERROR
    RegisterLoadedPlugin { plugin: LoadedPlugin },
    UnregisterLoadedPlugin { plugin_id: String },
    SetPluginStatus { plugin_id: String, status: PluginPanelStatus },
    SetPluginSupervisorState { plugin_id: String, state: PluginSupervisorState },
    SetActiveSession { session: Option<ActiveSession> },
    SetSessionError { error: Option<String> },

    // ── App labels — audit SET_APP_LABELS_OVERLAY
    SetAppLabelOverride { app_id: String, label_key: String, value: Option<String> },

    // ── Action rail — audit SET_ACTION_PANE_FOLDED/SET_ACTION_PANE_EXPANDED/STAGE_ACTION_ARG/RESET_ACTION_ARGS/SET_ACTIVE_UTILITY/SET_ACTIVE_TOOL
    SetActionPaneFolded { window_id: String, folded: bool },
    SetActionPaneExpanded { window_id: String, action_id: Option<String> },
    StageActionArg { window_id: String, action_id: String, arg_id: String, value: JsonValue },
    ResetActionArgs { window_id: String, action_id: String },
    SetActiveUtility { window_id: String, utility_id: Option<String> },
    SetActiveTool { tool_id: Option<String> },

    // ── Command palette — audit SET_COMMAND_EXPANDED/STAGE_COMMAND_ARG/RESET_COMMAND_ARGS
    SetCommandExpanded { command_id: Option<String> },
    StageCommandArg { command_id: String, arg_id: String, value: JsonValue },
    ResetCommandArgs { command_id: String },

    // ── Panel layout — audit SET_PANEL_VISIBLE/SET_PANEL_SIZE/SET_PANEL_PATH/SET_DOCK_OVERRIDE/SET_PANEL_PATH_MEMORY/SET_TREE_OPEN_STATE/HYDRATE_DOCK_UI/RESET_DOCK/SET_ACTIVE_WINDOW_ID/SET_SHELL_LAYOUT/SET_ACTIVE_EXAMPLE_ID/SET_MOBILE_PANEL_PATH/SET_MOBILE_PANEL_VISIBLE/SET_EXTRA_WINDOW_INSTANCES/SET_WINDOW_TITLE/SET_WINDOW_ICON
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

    // ── Overlays / dialogs — audit SET_SEARCH_OPEN/SET_FIND_OPEN/AUTO_START_INTRODUCTION/SET_INTRODUCTION_STEP/COMPLETE_INTRODUCTION_INTERACTION/SET_DIALOG/SET_TRANSIENT_NOTICE/SET_OPEN_WITH_FOCUS_ROLE
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

    // ── Tutorial (semantic subset) — audit SET_TUTORIAL
    SetActiveTutorial { tutorial_id: Option<String> },

    // ── Ephemeral UI editor drafts; durable preferences are canonical OS config mutations.
    SetUiDriverDraft { draft: Option<UiDriver> },
    SetUiThemeDraft { draft: Option<UiTheme> },

    // ── Sync — audit SET_SYNC_BACKBONE_URI/SET_SYNC_CARD_KIND/SET_SYNC_DRAFT_PATH/SET_SYNC_STATUS_FOR_DOCUMENT
    SetSyncBackboneUri { uri: Option<String> },
    SetSyncCardKind { kind: Option<SyncCardKind> },
    SetSyncDraftPath { path: String },
    SetDocumentSyncStatus { document_id: String, status: ArtifactSyncStatus },

    // ── Inference port — host-owned ephemeral proposal lifecycle, never a document mutation
    SetDocumentInferencePort { document_id: String, port: InferencePortStatus },
    ClearDocumentInferencePort { document_id: String },

    // ── Merge / conflicts — audit SET_MERGE_POLICY/SET_CONFLICTS/SET_SELECTED_CONFLICT_ID
    SetMergePolicy { policy: MergePolicy },
    SetConflicts { conflicts: Vec<Conflict> },
    SelectConflict { conflict_id: Option<String> },

    // ── Host prefs — ShellHost `scope`/`openingPreferences` useState (audit §2)
    SetStorageScope { scope: ShellScope },
    SetOpeningPreference { role: String, dialect_id: Option<String> },
}
//#endregion 🎮️ShellCommand
