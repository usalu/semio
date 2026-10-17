//! 📋️ Shell state of the shell schema: the one shell state record with its default. See [`super`].

use super::*;

//#region 📋️ShellState
/// 📋️ The shell's entire semantic UI state — the single source of truth. Render caches (UiNode
/// trees, engagements, measure overlays, tutorial playback transport state) are deliberately
/// absent; see `📓️terra-P9-report.md` "Rows excluded" for the row-by-row justification. `revision`
/// increments on every successfully applied [`ShellCommand`]; a rejected command never changes it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ShellState {
    /// See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// a wider integer projection, in the TypeScript mirror.
    pub revision: u64,

    //#region 🔌️PluginRuntime
    pub loaded_plugins: Vec<LoadedPlugin>,
    pub plugin_status_by_id: HashMap<String, PluginPanelStatus>,
    pub plugin_supervisor_by_id: HashMap<String, PluginSupervisorState>,
    pub active_session: Option<ActiveSession>,
    pub session_error: Option<String>,
    //#endregion 🔌️PluginRuntime

    //#region 🏷️AppLabels
    /// app_id -> (label_key -> override text). Ephemeral per shell instance (not persisted) —
    /// audit classifies this SEMANTIC but never states a persistence scope; treating it as
    /// session-local is the conservative reading (it is `PluginAppLabelsOverlay`, not a document
    /// or account-level preference).
    pub app_labels_overlay: HashMap<String, HashMap<String, String>>,
    //#endregion 🏷️AppLabels

    //#region 🎛️ActionRail
    pub action_pane_folded_by_window: HashMap<String, bool>,
    pub action_pane_expanded_by_window: HashMap<String, Option<String>>,
    /// window_id -> action_id -> arg_id -> value.
    #[value(with = "staged_action_args_bridge")]
    pub staged_action_args: StagedActionArgs,
    pub active_utility_by_window: HashMap<String, Option<String>>,
    pub active_tool_id: Option<String>,
    //#endregion 🎛️ActionRail

    //#region 🎮️CommandPalette
    pub command_panel_expanded: Option<String>,
    /// command_id -> arg_id -> value.
    #[value(with = "staged_command_args_bridge")]
    pub staged_command_args: StagedCommandArgs,
    //#endregion 🎮️CommandPalette

    //#region 🗂️PanelLayout
    pub panels_visible: ByAnchor<bool>,
    pub panels_size: ByAnchor<f32>,
    pub panels_path: ByAnchor<Vec<String>>,
    pub dock_override: Option<LayoutNode>,
    pub panel_path_memory: HashMap<String, String>,
    pub tree_open_states: HashMap<String, bool>,
    pub active_window_id: Option<String>,
    pub shell_layout: Option<LayoutNode>,
    pub active_example_id: String,
    pub mobile_panel_path: Vec<String>,
    pub mobile_panel_visible: bool,
    pub extra_windows: Vec<ExtraWindowInstance>,
    pub window_titles_by_id: HashMap<String, String>,
    pub window_icons_by_id: HashMap<String, IconName>,
    //#endregion 🗂️PanelLayout

    //#region 🔔️Overlays
    pub search_open: bool,
    pub find_open: bool,
    pub introduction_step_index: Option<u32>,
    pub introduction_auto_started_keys: Vec<String>,
    pub introduction_completed_interactions: Vec<u32>,
    /// Open dialog stack, top-of-stack last. See [`DialogState`] for why this is a stack.
    pub dialog_stack: Vec<DialogState>,
    pub transient_notice: Option<TransientNotice>,
    pub open_with_focus_role: Option<AppRole>,
    //#endregion 🔔️Overlays

    //#region 🎓️Tutorial
    /// Only the persisted/semantic subset survives here — playback transport state
    /// (playing/rate/muted/captions/recording/deviated) is TRANSIENT per the audit and stays in
    /// the renderer.
    pub active_tutorial_id: Option<String>,
    //#endregion 🎓️Tutorial

    //#region 🎨️UiPreferenceDrafts
    pub ui_driver_draft: Option<UiDriver>,
    pub ui_theme_draft: Option<UiTheme>,
    //#endregion 🎨️UiPreferenceDrafts

    //#region 🔄️Sync
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<SyncCardKind>,
    pub sync_draft_path: String,
    pub sync_status_by_document: HashMap<String, ArtifactSyncStatus>,
    //#endregion 🔄️Sync

    //#region 💡️InferencePort
    /// Ephemeral per-document inference-port state, keyed by document id. Never persisted.
    pub inference_port_by_document: HashMap<String, InferencePortStatus>,
    //#endregion 💡️InferencePort

    //#region 🤝️Merge
    pub merge_policy: MergePolicy,
    pub conflicts: Vec<Conflict>,
    pub selected_conflict_id: Option<String>,
    //#endregion 🤝️Merge

    //#region 💾️Host
    pub storage_scope: ShellScope,
    /// role -> default dialect/app id (ShellHost `openingPreferences`, audit §2).
    pub opening_preferences: HashMap<String, String>,
    //#endregion 💾️Host
}

impl Default for ShellState {
    /// 🧪️ A purely technical starting point for tests/fixtures/bootstrap wiring — NOT a product
    /// choice of language, theme, or driver. Hosts supply durable UI preferences through the canonical OS config projection.
    fn default() -> Self {
        ShellState {
            revision: 0,
            loaded_plugins: Vec::new(),
            plugin_status_by_id: HashMap::new(),
            plugin_supervisor_by_id: HashMap::new(),
            active_session: None,
            session_error: None,
            app_labels_overlay: HashMap::new(),
            action_pane_folded_by_window: HashMap::new(),
            action_pane_expanded_by_window: HashMap::new(),
            staged_action_args: HashMap::new(),
            active_utility_by_window: HashMap::new(),
            active_tool_id: None,
            command_panel_expanded: None,
            staged_command_args: HashMap::new(),
            panels_visible: ByAnchor::uniform(false),
            panels_size: ByAnchor::uniform(280.0),
            panels_path: ByAnchor::uniform(Vec::new()),
            dock_override: None,
            panel_path_memory: HashMap::new(),
            tree_open_states: HashMap::new(),
            active_window_id: None,
            shell_layout: None,
            active_example_id: String::new(),
            mobile_panel_path: Vec::new(),
            mobile_panel_visible: false,
            extra_windows: Vec::new(),
            window_titles_by_id: HashMap::new(),
            window_icons_by_id: HashMap::new(),
            search_open: false,
            find_open: false,
            introduction_step_index: None,
            introduction_auto_started_keys: Vec::new(),
            introduction_completed_interactions: Vec::new(),
            dialog_stack: Vec::new(),
            transient_notice: None,
            open_with_focus_role: None,
            active_tutorial_id: None,
            ui_driver_draft: None,
            ui_theme_draft: None,
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_draft_path: String::new(),
            sync_status_by_document: HashMap::new(),
            inference_port_by_document: HashMap::new(),
            merge_policy: MergePolicy::Manual,
            conflicts: Vec::new(),
            selected_conflict_id: None,
            storage_scope: ShellScope::Memory,
            opening_preferences: HashMap::new(),
        }
    }
}
//#endregion 📋️ShellState
