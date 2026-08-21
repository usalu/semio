//! 🖥️ Shell UI state single source of truth. `ShellState` + `ShellCommand` + `ShellEvent` +
//! `ShellError` + the pure [`reduce`] function are the ONE place semantic shell UI state (which
//! windows exist, what is focused, active mode/tool/utility, panel and dock layout,
//! dialogs/overlays, sync/merge state, user-visible prefs) lives. The React `Shell/component.tsx`
//! reducer, the ShellHost `useState`s, and the wgpu `Shell/component.rs` struct are three
//! independent, drifting copies of this today; they become projections of this module in later
//! adoption packets (H1–H4). Nothing outside those host files can observe or drive shell state
//! today — that is exactly why the OS is not LLM-first. `shell_capabilities()` at the bottom of
//! this file is what lets an MCP gateway (a later packet) advertise every `ShellCommand` variant
//! as an invocable, schema-described tool.
//!
//! Pure: no I/O, no clock (callers pass `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/
//! `std::thread`/`SystemTime`/`Instant::now`/`std::fs`/`std::net`. Compiles for native AND
//! `wasm32-unknown-unknown` — both the React host and wgpu-web run this crate directly.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️luna-shellstate-audit.md`
//! for the row-by-row classification this module is built from, and `📓️terra-P9-report.md` for
//! the coverage table (which audit row each variant below subsumes) and every scope decision this
//! file makes that the audit left open.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 🧩️ Free-form staged/seed argument payload (action args, command args, dialog seed args).
/// `serde_json::Value` has no `ts_rs::TS` impl, so every field of this type is annotated
/// `#[cfg_attr(feature = "typegen", ts(type = "unknown"))]` at its use site.
pub type JsonValue = serde_json::Value;

//#region 🧭️Anchor
/// 🧭️ The four docking edges a panel can attach to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Anchor {
    Left,
    Right,
    Top,
    Bottom,
}

impl Anchor {
    pub const ALL: [Anchor; 4] = [Anchor::Left, Anchor::Right, Anchor::Top, Anchor::Bottom];
}

/// 🧭️ One value per [`Anchor`] — `Anchor` is a small closed set (4 edges), so a fixed struct
/// avoids the "enum as HashMap key" JSON-serialization pitfall (serde_json map keys must reduce
/// to strings; a 4-field struct sidesteps the question entirely and is simpler to reason about).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ByAnchor<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Clone> ByAnchor<T> {
    pub fn uniform(value: T) -> Self {
        Self { left: value.clone(), right: value.clone(), top: value.clone(), bottom: value }
    }

    pub fn get(&self, anchor: Anchor) -> &T {
        match anchor {
            Anchor::Left => &self.left,
            Anchor::Right => &self.right,
            Anchor::Top => &self.top,
            Anchor::Bottom => &self.bottom,
        }
    }

    pub fn set(&mut self, anchor: Anchor, value: T) {
        match anchor {
            Anchor::Left => self.left = value,
            Anchor::Right => self.right = value,
            Anchor::Top => self.top = value,
            Anchor::Bottom => self.bottom = value,
        }
    }
}
//#endregion 🧭️Anchor

//#region 🧱️LayoutNode
/// 🧱️ How two child regions of a split are arranged.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// 🧱️ Minimal local mirror of the real `WindowLayoutNode`/`DockSkeleton` tree types (audit
/// `📓️luna-shellstate-audit.md` §1, `shellLayout`/`dockOverride` rows). The canonical types live
/// in the React/wgpu shells' own modules, which this crate must not depend on (§4 of the packet:
/// "if depending on `semio-framework` drags in a mid-rewrite crate, define the minimal local
/// mirror and say so"). `dockOverride` and `shellLayout` are structurally the same kind of tree
/// (window ids at the leaves, nested splits), so one recursive type serves both; reconciling this
/// with the real tree types is later-packet adoption work.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum LayoutNode {
    Leaf { window_id: String },
    Split { orientation: SplitOrientation, children: Vec<LayoutNode>, sizes: Vec<f32> },
}

/// 🗂️ Restored dock UI state (`HYDRATE_DOCK_UI` row) — a layout tree plus per-anchor visibility,
/// the shape the audit's `DockUiState` payload carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DockUiState {
    pub layout: Option<LayoutNode>,
    pub panels_visible: ByAnchor<bool>,
}
//#endregion 🧱️LayoutNode

//#region 🪟️Windows
/// 🪟️ One spawned/extra window instance (audit `ExtraWindowInstance`). `params` mirrors the same
/// free-form seed shape `OpenWindow`'s kernel effect carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ExtraWindowInstance {
    pub window_id: String,
    pub kind: String,
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub params: Option<JsonValue>,
}

/// 🖼️ Stable icon identifier. Local newtype mirror — the real `IconName` lives in the UI token
/// crate this module must not depend on (§4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct IconName(pub String);
//#endregion 🪟️Windows

//#region 🔌️PluginRuntime
/// 🔌️ One loaded plugin's registry entry (audit `LoadedProgramState` — its exact field shape was
/// never captured by the audit; this is a minimal domain-shaped mirror sufficient for identity +
/// registry semantics, reconciled at adoption time).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct LoadedPlugin {
    pub plugin_id: String,
    pub module_url: String,
    pub label: Option<String>,
}

/// 🪟️ Plugin panel open/collapsed UI state (audit: "plugin panel UI state (open/collapsed)").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PluginPanelStatus {
    Open,
    Collapsed,
    Errored { message: String },
}

/// 🚑️ Plugin resource/failure monitoring summary (audit: "plugin resource/failure monitoring").
/// A minimal local mirror — the full failure ladder lives in the actor runtime crate this module
/// must not depend on (§4); this is the shell-observable projection of it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PluginSupervisorState {
    pub healthy: bool,
    pub restart_count: u32,
    /// `u64`, but the wire format is plain JSON (this module has no binary codec), so the TS
    /// mirror must be `number` not ts-rs' default `bigint` — millisecond epoch timestamps never
    /// approach `u64::MAX`, let alone JS's `Number.MAX_SAFE_INTEGER`.
    #[cfg_attr(feature = "typegen", ts(type = "number | null"))]
    pub last_signal_ms: Option<u64>,
}

/// 🎯️ The active app instance binding (audit: "active app instance binding").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DialogState {
    pub dialog_id: String,
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub seed_args: Option<JsonValue>,
}

/// 🔔️ Severity of a [`TransientNotice`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum NoticeKind {
    Info,
    Success,
    Warning,
    Error,
}

/// 🔔️ A non-blocking auto-dismiss notice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TransientNotice {
    pub message: String,
    pub kind: NoticeKind,
    /// See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// ts-rs' default `bigint`, in the TS mirror.
    #[cfg_attr(feature = "typegen", ts(type = "number | null"))]
    pub expires_at_ms: Option<u64>,
}

/// 🎭️ Which role group the Open panel should focus. Local newtype mirror (§4) — the real
/// `AppRole` enumeration lives in the plugin manifest surface this module must not depend on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct AppRole(pub String);
//#endregion 🔔️Overlays

//#region 🎨️UiPreferences
/// 🎨️ Appearance preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum UiAppearance {
    System,
    Light,
    Dark,
}

/// 📐️ UI chrome density (matches the `os.setDriver` command's declared select options
/// default/compact — audit §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum UiChromeLayout {
    Default,
    Compact,
}

/// 🌐️ Interface language. CLAUDE.md requires "no default language" as a *product* policy (a host
/// must not silently prefer a language without an explicit choice reaching the user) — that is a
/// bootstrap-sequencing concern for the host, not something a plain-old-data enum can encode; this
/// type still needs a technical fallback value for [`ShellState::default`], documented there.
/// English first, then German, per CLAUDE.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum UiLocale {
    En,
    De,
}

/// 🚗️ A user-defined UI driver (audit: `uiCustomDrivers`/`uiDriverDraft`). Its full shape was
/// never captured by the audit; `config` carries whatever driver-specific data the real type has.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct UiDriver {
    pub driver_id: String,
    pub label: String,
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub config: JsonValue,
}

/// 🎨️ A user-defined UI theme (audit: `uiCustomThemes`/`uiThemeDraft`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct UiTheme {
    pub theme_id: String,
    pub label: String,
    pub tokens: HashMap<String, String>,
}
//#endregion 🎨️UiPreferences

//#region 🔄️Sync
/// 🗂️ Check-in target type (audit: "check-in file/folder/remote type").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SyncCardKind {
    File,
    Folder,
    Remote,
}

/// 🩺️ Per-document sync health (audit: `ArtifactSyncStatus`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArtifactSyncStatus {
    Clean,
    Dirty,
    Syncing,
    Errored { message: String },
}
//#endregion 🔄️Sync

//#region 🤝️Merge
/// 🤝️ Conflict resolution strategy (audit: `MergePolicy`, persisted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MergePolicy {
    PreferLocal,
    PreferRemote,
    Manual,
}

/// ⚠️ One open conflict on the roster (audit: `Conflict`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Conflict {
    pub conflict_id: String,
    pub document_id: String,
    pub description: String,
}
//#endregion 🤝️Merge

//#region 💾️Host
/// 💾️ Storage backend for shell state persistence (ShellHost `scope` useState, audit §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ShellScope {
    LocalStorage,
    Memory,
}
//#endregion 💾️Host

//#region 📋️ShellState
/// 📋️ The shell's entire semantic UI state — the single source of truth. Render caches (UiNode
/// trees, engagements, measure overlays, tutorial playback transport state) are deliberately
/// absent; see `📓️terra-P9-report.md` "Rows excluded" for the row-by-row justification. `revision`
/// increments on every successfully applied [`ShellCommand`]; a rejected command never changes it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ShellState {
    /// See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// ts-rs' default `bigint`, in the TS mirror.
    #[cfg_attr(feature = "typegen", ts(type = "number"))]
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
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub staged_action_args: HashMap<String, HashMap<String, HashMap<String, JsonValue>>>,
    pub active_utility_by_window: HashMap<String, Option<String>>,
    pub active_tool_id: Option<String>,
    //#endregion 🎛️ActionRail

    //#region 🎮️CommandPalette
    pub command_panel_expanded: Option<String>,
    /// command_id -> arg_id -> value.
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub staged_command_args: HashMap<String, HashMap<String, JsonValue>>,
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

    //#region 🎨️UiPreferences
    pub ui_appearance: UiAppearance,
    pub ui_layout: UiChromeLayout,
    pub ui_driver_id: String,
    pub ui_custom_drivers: HashMap<String, UiDriver>,
    pub ui_driver_draft: Option<UiDriver>,
    pub ui_locale: UiLocale,
    pub ui_terminology: String,
    pub ui_theme_id: String,
    pub ui_custom_themes: HashMap<String, UiTheme>,
    pub ui_theme_draft: Option<UiTheme>,
    pub ui_keybinding_overrides: HashMap<String, String>,
    //#endregion 🎨️UiPreferences

    //#region 🔄️Sync
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<SyncCardKind>,
    pub sync_draft_path: String,
    pub sync_status_by_document: HashMap<String, ArtifactSyncStatus>,
    //#endregion 🔄️Sync

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
    /// choice of language, theme, or driver. CLAUDE.md requires "no default language"; a host
    /// integrating this module MUST issue an explicit `SetUiLocale`/`SetUiAppearance`/… during its
    /// own bootstrap sequence before presenting UI, exactly as it must do today for the React
    /// reducer's own initial state.
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
            ui_appearance: UiAppearance::System,
            ui_layout: UiChromeLayout::Default,
            ui_driver_id: String::new(),
            ui_custom_drivers: HashMap::new(),
            ui_driver_draft: None,
            ui_locale: UiLocale::En,
            ui_terminology: String::new(),
            ui_theme_id: String::new(),
            ui_custom_themes: HashMap::new(),
            ui_theme_draft: None,
            ui_keybinding_overrides: HashMap::new(),
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_draft_path: String::new(),
            sync_status_by_document: HashMap::new(),
            merge_policy: MergePolicy::Manual,
            conflicts: Vec::new(),
            selected_conflict_id: None,
            storage_scope: ShellScope::Memory,
            opening_preferences: HashMap::new(),
        }
    }
}
//#endregion 📋️ShellState

//#region 🎮️ShellCommand
/// 🎮️ The vocabulary the LLM (and every other shell client) speaks. Every variant is plain
/// serializable data — no `Updatable<T>`/functional-updater payload (a React idiom that cannot
/// cross a wire); every Record/HashMap-shaped mutation from the TS reducer is decomposed into a
/// single-key upsert/remove command (`value: None` removes) rather than a whole-map replacement,
/// which is both wire-safe and small-diff-friendly for an LLM caller.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellCommand {
    // ── Plugin runtime — audit UPSERT_LOADED_PLUGIN/REMOVE_LOADED_PLUGIN/SET_PLUGIN_STATUS/SET_PLUGIN_SUPERVISOR/SET_SESSION/SET_ERROR
    RegisterLoadedPlugin {
        plugin: LoadedPlugin,
    },
    UnregisterLoadedPlugin {
        plugin_id: String,
    },
    SetPluginStatus {
        plugin_id: String,
        status: PluginPanelStatus,
    },
    SetPluginSupervisorState {
        plugin_id: String,
        state: PluginSupervisorState,
    },
    SetActiveSession {
        session: Option<ActiveSession>,
    },
    SetSessionError {
        error: Option<String>,
    },

    // ── App labels — audit SET_APP_LABELS_OVERLAY
    SetAppLabelOverride {
        app_id: String,
        label_key: String,
        value: Option<String>,
    },

    // ── Action rail — audit SET_ACTION_PANE_FOLDED/SET_ACTION_PANE_EXPANDED/STAGE_ACTION_ARG/RESET_ACTION_ARGS/SET_ACTIVE_UTILITY/SET_ACTIVE_TOOL
    SetActionPaneFolded {
        window_id: String,
        folded: bool,
    },
    SetActionPaneExpanded {
        window_id: String,
        action_id: Option<String>,
    },
    StageActionArg {
        window_id: String,
        action_id: String,
        arg_id: String,
        #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
        value: JsonValue,
    },
    ResetActionArgs {
        window_id: String,
        action_id: String,
    },
    SetActiveUtility {
        window_id: String,
        utility_id: Option<String>,
    },
    SetActiveTool {
        tool_id: Option<String>,
    },

    // ── Command palette — audit SET_COMMAND_EXPANDED/STAGE_COMMAND_ARG/RESET_COMMAND_ARGS
    SetCommandExpanded {
        command_id: Option<String>,
    },
    StageCommandArg {
        command_id: String,
        arg_id: String,
        #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
        value: JsonValue,
    },
    ResetCommandArgs {
        command_id: String,
    },

    // ── Panel layout — audit SET_PANEL_VISIBLE/SET_PANEL_SIZE/SET_PANEL_PATH/SET_DOCK_OVERRIDE/SET_PANEL_PATH_MEMORY/SET_TREE_OPEN_STATE/HYDRATE_DOCK_UI/RESET_DOCK/SET_ACTIVE_WINDOW_ID/SET_SHELL_LAYOUT/SET_ACTIVE_EXAMPLE_ID/SET_MOBILE_PANEL_PATH/SET_MOBILE_PANEL_VISIBLE/SET_EXTRA_WINDOW_INSTANCES/SET_WINDOW_TITLE/SET_WINDOW_ICON
    SetPanelVisible {
        anchor: Anchor,
        visible: bool,
    },
    SetPanelSize {
        anchor: Anchor,
        size: f32,
    },
    SetPanelPath {
        anchor: Anchor,
        path: Vec<String>,
    },
    SetDockOverride {
        dock: Option<LayoutNode>,
    },
    SetPanelPathMemory {
        panel_key: String,
        path: Option<String>,
    },
    SetTreeOpenState {
        tree_id: String,
        open: bool,
    },
    HydrateDockUi {
        dock: Option<DockUiState>,
    },
    ResetDock,
    FocusWindow {
        window_id: Option<String>,
    },
    SetShellLayout {
        layout: Option<LayoutNode>,
    },
    SetActiveExample {
        example_id: String,
    },
    SetMobilePanelPath {
        path: Vec<String>,
    },
    SetMobilePanelVisible {
        visible: bool,
    },
    SetExtraWindows {
        windows: Vec<ExtraWindowInstance>,
    },
    SetWindowTitle {
        window_id: String,
        title: String,
    },
    SetWindowIcon {
        window_id: String,
        icon: IconName,
    },

    // ── Overlays / dialogs — audit SET_SEARCH_OPEN/SET_FIND_OPEN/AUTO_START_INTRODUCTION/SET_INTRODUCTION_STEP/COMPLETE_INTRODUCTION_INTERACTION/SET_DIALOG/SET_TRANSIENT_NOTICE/SET_OPEN_WITH_FOCUS_ROLE
    SetSearchOpen {
        open: bool,
    },
    SetFindOpen {
        open: bool,
    },
    AutoStartIntroduction {
        key: String,
    },
    SetIntroductionStep {
        step_index: Option<u32>,
    },
    CompleteIntroductionInteraction {
        interaction_index: u32,
    },
    OpenDialog {
        dialog_id: String,
        #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
        seed_args: Option<JsonValue>,
    },
    CloseDialog {
        dialog_id: Option<String>,
    },
    ShowTransientNotice {
        notice: TransientNotice,
    },
    DismissTransientNotice,
    SetOpenWithFocusRole {
        role: Option<AppRole>,
    },

    // ── Tutorial (semantic subset) — audit SET_TUTORIAL
    SetActiveTutorial {
        tutorial_id: Option<String>,
    },

    // ── UI preferences — audit SET_UI_APPEARANCE/SET_UI_LAYOUT/SET_UI_DRIVER_ID/SET_UI_CUSTOM_DRIVERS/SET_UI_DRIVER_DRAFT/SET_UI_LOCALE/SET_UI_TERMINOLOGY/SET_UI_THEME_ID/SET_UI_CUSTOM_THEMES/SET_UI_THEME_DRAFT/SET_UI_KEYBINDING_OVERRIDES
    SetUiAppearance {
        appearance: UiAppearance,
    },
    SetUiLayout {
        layout: UiChromeLayout,
    },
    SetUiDriver {
        driver_id: String,
    },
    SetUiCustomDriver {
        driver_id: String,
        driver: Option<UiDriver>,
    },
    SetUiDriverDraft {
        draft: Option<UiDriver>,
    },
    SetUiLocale {
        locale: UiLocale,
    },
    SetUiTerminology {
        terminology_id: String,
    },
    SetUiTheme {
        theme_id: String,
    },
    SetUiCustomTheme {
        theme_id: String,
        theme: Option<UiTheme>,
    },
    SetUiThemeDraft {
        draft: Option<UiTheme>,
    },
    SetUiKeybindingOverride {
        control_id: String,
        keys: Option<String>,
    },

    // ── Sync — audit SET_SYNC_BACKBONE_URI/SET_SYNC_CARD_KIND/SET_SYNC_DRAFT_PATH/SET_SYNC_STATUS_FOR_DOCUMENT
    SetSyncBackboneUri {
        uri: Option<String>,
    },
    SetSyncCardKind {
        kind: Option<SyncCardKind>,
    },
    SetSyncDraftPath {
        path: String,
    },
    SetDocumentSyncStatus {
        document_id: String,
        status: ArtifactSyncStatus,
    },

    // ── Merge / conflicts — audit SET_MERGE_POLICY/SET_CONFLICTS/SET_SELECTED_CONFLICT_ID
    SetMergePolicy {
        policy: MergePolicy,
    },
    SetConflicts {
        conflicts: Vec<Conflict>,
    },
    SelectConflict {
        conflict_id: Option<String>,
    },

    // ── Host prefs — ShellHost `scope`/`openingPreferences` useState (audit §2)
    SetStorageScope {
        scope: ShellScope,
    },
    SetOpeningPreference {
        role: String,
        dialect_id: Option<String>,
    },
}
//#endregion 🎮️ShellCommand

//#region 📣️ShellEvent
/// 📣️ What [`reduce`] reports happened. Every accepted command always emits [`ShellEvent::Applied`]
/// (a deterministic, always-present baseline every fixture can assert on); commands whose
/// acceptance also triggers an automatic side effect (focus reassignment, mutual-exclusion
/// clearing, a dock reset) additionally emit the matching specific variant below. This is
/// deliberately NOT a 1:1 mirror of `ShellCommand` — most setters have no side effect beyond the
/// field they set, so a second event carrying the same payload as the command would be pure
/// duplication; the specific variants exist only where `reduce` does something beyond the literal
/// field write the command names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellEvent {
    /// Always emitted, exactly once, as the last event of every accepted command. `revision` is
    /// `number`, not ts-rs' default `bigint` — see [`PluginSupervisorState::last_signal_ms`].
    Applied {
        capability_id: String,
        #[cfg_attr(feature = "typegen", ts(type = "number"))]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, thiserror::Error, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellError {
    #[error("empty identifier for {field}")]
    EmptyIdentifier { field: String },
    #[error("unknown plugin: {plugin_id}")]
    UnknownPlugin { plugin_id: String },
    #[error("unknown dialog: {dialog_id}")]
    UnknownDialog { dialog_id: String },
    #[error("unknown conflict: {conflict_id}")]
    UnknownConflict { conflict_id: String },
    #[error("invalid panel size for {anchor:?}: {size}")]
    InvalidPanelSize { anchor: Anchor, size: f32 },
}
//#endregion 🚨️ShellError

//#region 🧮️reduce
fn require_non_empty(value: &str, field: &str) -> Result<(), ShellError> {
    if value.trim().is_empty() {
        Err(ShellError::EmptyIdentifier { field: field.to_string() })
    } else {
        Ok(())
    }
}

/// 🧮️ Total, pure state transition: same inputs → same outputs, never panics. `now_ms` is the
/// caller's clock reading (this crate never reads a clock itself); it is threaded through only to
/// timestamp events/notices that need a wall-clock moment (`TransientNotice::expires_at_ms`, etc.)
/// — it is the caller's responsibility to advance it monotonically. On success, `state.revision`
/// in the returned state is exactly `state.revision + 1`; on error, the input `state` is
/// unchanged (this function takes `&ShellState` and always returns a fresh value, so "unchanged"
/// is enforced by never touching the borrowed input).
pub fn reduce(state: &ShellState, command: &ShellCommand, now_ms: u64) -> Result<(ShellState, Vec<ShellEvent>), ShellError> {
    let mut next = state.clone();
    let mut events: Vec<ShellEvent> = Vec::new();
    let capability_id = capability_id_for(command);

    match command {
        ShellCommand::RegisterLoadedPlugin { plugin } => {
            require_non_empty(&plugin.plugin_id, "plugin.plugin_id")?;
            next.loaded_plugins.retain(|existing| existing.plugin_id != plugin.plugin_id);
            next.loaded_plugins.push(plugin.clone());
        }
        ShellCommand::UnregisterLoadedPlugin { plugin_id } => {
            require_non_empty(plugin_id, "plugin_id")?;
            let before = next.loaded_plugins.len();
            next.loaded_plugins.retain(|existing| &existing.plugin_id != plugin_id);
            if next.loaded_plugins.len() == before {
                return Err(ShellError::UnknownPlugin { plugin_id: plugin_id.clone() });
            }
            next.plugin_status_by_id.remove(plugin_id);
            next.plugin_supervisor_by_id.remove(plugin_id);
        }
        ShellCommand::SetPluginStatus { plugin_id, status } => {
            require_non_empty(plugin_id, "plugin_id")?;
            next.plugin_status_by_id.insert(plugin_id.clone(), status.clone());
        }
        ShellCommand::SetPluginSupervisorState { plugin_id, state: supervisor } => {
            require_non_empty(plugin_id, "plugin_id")?;
            next.plugin_supervisor_by_id.insert(plugin_id.clone(), supervisor.clone());
        }
        ShellCommand::SetActiveSession { session } => {
            next.active_session = session.clone();
        }
        ShellCommand::SetSessionError { error } => {
            next.session_error = error.clone();
        }

        ShellCommand::SetAppLabelOverride { app_id, label_key, value } => {
            require_non_empty(app_id, "app_id")?;
            require_non_empty(label_key, "label_key")?;
            let entry = next.app_labels_overlay.entry(app_id.clone()).or_default();
            match value {
                Some(v) => {
                    entry.insert(label_key.clone(), v.clone());
                }
                None => {
                    entry.remove(label_key);
                    if entry.is_empty() {
                        next.app_labels_overlay.remove(app_id);
                    }
                }
            }
        }

        ShellCommand::SetActionPaneFolded { window_id, folded } => {
            require_non_empty(window_id, "window_id")?;
            next.action_pane_folded_by_window.insert(window_id.clone(), *folded);
        }
        ShellCommand::SetActionPaneExpanded { window_id, action_id } => {
            require_non_empty(window_id, "window_id")?;
            next.action_pane_expanded_by_window.insert(window_id.clone(), action_id.clone());
        }
        ShellCommand::StageActionArg { window_id, action_id, arg_id, value } => {
            require_non_empty(window_id, "window_id")?;
            require_non_empty(action_id, "action_id")?;
            require_non_empty(arg_id, "arg_id")?;
            next.staged_action_args.entry(window_id.clone()).or_default().entry(action_id.clone()).or_default().insert(arg_id.clone(), value.clone());
        }
        ShellCommand::ResetActionArgs { window_id, action_id } => {
            require_non_empty(window_id, "window_id")?;
            require_non_empty(action_id, "action_id")?;
            if let Some(by_action) = next.staged_action_args.get_mut(window_id) {
                by_action.remove(action_id);
                if by_action.is_empty() {
                    next.staged_action_args.remove(window_id);
                }
            }
        }
        ShellCommand::SetActiveUtility { window_id, utility_id } => {
            require_non_empty(window_id, "window_id")?;
            next.active_utility_by_window.insert(window_id.clone(), utility_id.clone());
            // Mode↔tool mutual exclusion (tricky path, packet §5): a utility taking control of the
            // currently-focused window releases the global active tool.
            if utility_id.is_some() && next.active_window_id.as_deref() == Some(window_id.as_str()) && next.active_tool_id.is_some() {
                let previous = next.active_tool_id.take();
                events.push(ShellEvent::ActiveToolChanged { previous, current: None });
            }
        }
        ShellCommand::SetActiveTool { tool_id } => {
            let previous_tool = next.active_tool_id.clone();
            next.active_tool_id = tool_id.clone();
            // Mode↔tool mutual exclusion: a global tool taking control releases the focused
            // window's active utility.
            if tool_id.is_some() {
                if let Some(window_id) = next.active_window_id.clone() {
                    if let Some(slot) = next.active_utility_by_window.get_mut(&window_id) {
                        if slot.is_some() {
                            let previous = slot.take();
                            events.push(ShellEvent::ActiveUtilityChanged { window_id, previous, current: None });
                        }
                    }
                }
            }
            let _ = previous_tool;
        }

        ShellCommand::SetCommandExpanded { command_id } => {
            next.command_panel_expanded = command_id.clone();
        }
        ShellCommand::StageCommandArg { command_id, arg_id, value } => {
            require_non_empty(command_id, "command_id")?;
            require_non_empty(arg_id, "arg_id")?;
            next.staged_command_args.entry(command_id.clone()).or_default().insert(arg_id.clone(), value.clone());
        }
        ShellCommand::ResetCommandArgs { command_id } => {
            require_non_empty(command_id, "command_id")?;
            next.staged_command_args.remove(command_id);
        }

        ShellCommand::SetPanelVisible { anchor, visible } => {
            next.panels_visible.set(*anchor, *visible);
        }
        ShellCommand::SetPanelSize { anchor, size } => {
            if !size.is_finite() || *size < 0.0 {
                return Err(ShellError::InvalidPanelSize { anchor: *anchor, size: *size });
            }
            next.panels_size.set(*anchor, *size);
        }
        ShellCommand::SetPanelPath { anchor, path } => {
            next.panels_path.set(*anchor, path.clone());
        }
        ShellCommand::SetDockOverride { dock } => {
            next.dock_override = dock.clone();
        }
        ShellCommand::SetPanelPathMemory { panel_key, path } => {
            require_non_empty(panel_key, "panel_key")?;
            match path {
                Some(p) => {
                    next.panel_path_memory.insert(panel_key.clone(), p.clone());
                }
                None => {
                    next.panel_path_memory.remove(panel_key);
                }
            }
        }
        ShellCommand::SetTreeOpenState { tree_id, open } => {
            require_non_empty(tree_id, "tree_id")?;
            next.tree_open_states.insert(tree_id.clone(), *open);
        }
        ShellCommand::HydrateDockUi { dock } => {
            next.dock_override = dock.as_ref().and_then(|d| d.layout.clone());
            if let Some(d) = dock {
                next.panels_visible = d.panels_visible.clone();
            }
        }
        ShellCommand::ResetDock => {
            next.dock_override = None;
            events.push(ShellEvent::DockReset);
        }
        ShellCommand::FocusWindow { window_id } => {
            let previous = next.active_window_id.clone();
            next.active_window_id = window_id.clone();
            if previous != next.active_window_id {
                events.push(ShellEvent::WindowFocusChanged { previous, current: next.active_window_id.clone() });
            }
        }
        ShellCommand::SetShellLayout { layout } => {
            next.shell_layout = layout.clone();
        }
        ShellCommand::SetActiveExample { example_id } => {
            next.active_example_id = example_id.clone();
        }
        ShellCommand::SetMobilePanelPath { path } => {
            next.mobile_panel_path = path.clone();
        }
        ShellCommand::SetMobilePanelVisible { visible } => {
            next.mobile_panel_visible = *visible;
        }
        ShellCommand::SetExtraWindows { windows } => {
            next.extra_windows = windows.clone();
            // Focus-after-close (tricky path, packet §5): if the focused window was an extra
            // window and it is no longer present, refocus the last remaining extra window, or
            // clear focus if none remain.
            if let Some(active) = next.active_window_id.clone() {
                let was_extra = state.extra_windows.iter().any(|w| w.window_id == active);
                let still_present = next.extra_windows.iter().any(|w| w.window_id == active);
                if was_extra && !still_present {
                    let fallback = next.extra_windows.last().map(|w| w.window_id.clone());
                    next.active_window_id = fallback.clone();
                    events.push(ShellEvent::WindowFocusChanged { previous: Some(active), current: fallback });
                }
            }
        }
        ShellCommand::SetWindowTitle { window_id, title } => {
            require_non_empty(window_id, "window_id")?;
            next.window_titles_by_id.insert(window_id.clone(), title.clone());
        }
        ShellCommand::SetWindowIcon { window_id, icon } => {
            require_non_empty(window_id, "window_id")?;
            next.window_icons_by_id.insert(window_id.clone(), icon.clone());
        }

        ShellCommand::SetSearchOpen { open } => {
            next.search_open = *open;
        }
        ShellCommand::SetFindOpen { open } => {
            next.find_open = *open;
        }
        ShellCommand::AutoStartIntroduction { key } => {
            require_non_empty(key, "key")?;
            if !next.introduction_auto_started_keys.iter().any(|k| k == key) {
                next.introduction_auto_started_keys.push(key.clone());
            }
        }
        ShellCommand::SetIntroductionStep { step_index } => {
            next.introduction_step_index = *step_index;
        }
        ShellCommand::CompleteIntroductionInteraction { interaction_index } => {
            if !next.introduction_completed_interactions.contains(interaction_index) {
                next.introduction_completed_interactions.push(*interaction_index);
            }
        }
        ShellCommand::OpenDialog { dialog_id, seed_args } => {
            require_non_empty(dialog_id, "dialog_id")?;
            next.dialog_stack.push(DialogState { dialog_id: dialog_id.clone(), seed_args: seed_args.clone() });
            events.push(ShellEvent::DialogOpened { dialog_id: dialog_id.clone() });
        }
        ShellCommand::CloseDialog { dialog_id } => {
            let closed_id = match dialog_id {
                Some(id) => {
                    require_non_empty(id, "dialog_id")?;
                    let position = next.dialog_stack.iter().position(|d| &d.dialog_id == id).ok_or_else(|| ShellError::UnknownDialog { dialog_id: id.clone() })?;
                    next.dialog_stack.remove(position).dialog_id
                }
                None => next.dialog_stack.pop().ok_or_else(|| ShellError::UnknownDialog { dialog_id: String::new() })?.dialog_id,
            };
            events.push(ShellEvent::DialogClosed { dialog_id: closed_id });
        }
        ShellCommand::ShowTransientNotice { notice } => {
            next.transient_notice = Some(notice.clone());
        }
        ShellCommand::DismissTransientNotice => {
            next.transient_notice = None;
        }
        ShellCommand::SetOpenWithFocusRole { role } => {
            next.open_with_focus_role = role.clone();
        }

        ShellCommand::SetActiveTutorial { tutorial_id } => {
            next.active_tutorial_id = tutorial_id.clone();
        }

        ShellCommand::SetUiAppearance { appearance } => {
            next.ui_appearance = *appearance;
        }
        ShellCommand::SetUiLayout { layout } => {
            next.ui_layout = *layout;
        }
        ShellCommand::SetUiDriver { driver_id } => {
            next.ui_driver_id = driver_id.clone();
        }
        ShellCommand::SetUiCustomDriver { driver_id, driver } => {
            require_non_empty(driver_id, "driver_id")?;
            match driver {
                Some(d) => {
                    next.ui_custom_drivers.insert(driver_id.clone(), d.clone());
                }
                None => {
                    next.ui_custom_drivers.remove(driver_id);
                }
            }
        }
        ShellCommand::SetUiDriverDraft { draft } => {
            next.ui_driver_draft = draft.clone();
        }
        ShellCommand::SetUiLocale { locale } => {
            next.ui_locale = *locale;
        }
        ShellCommand::SetUiTerminology { terminology_id } => {
            next.ui_terminology = terminology_id.clone();
        }
        ShellCommand::SetUiTheme { theme_id } => {
            next.ui_theme_id = theme_id.clone();
        }
        ShellCommand::SetUiCustomTheme { theme_id, theme } => {
            require_non_empty(theme_id, "theme_id")?;
            match theme {
                Some(t) => {
                    next.ui_custom_themes.insert(theme_id.clone(), t.clone());
                }
                None => {
                    next.ui_custom_themes.remove(theme_id);
                }
            }
        }
        ShellCommand::SetUiThemeDraft { draft } => {
            next.ui_theme_draft = draft.clone();
        }
        ShellCommand::SetUiKeybindingOverride { control_id, keys } => {
            require_non_empty(control_id, "control_id")?;
            match keys {
                Some(k) => {
                    next.ui_keybinding_overrides.insert(control_id.clone(), k.clone());
                }
                None => {
                    next.ui_keybinding_overrides.remove(control_id);
                }
            }
        }

        ShellCommand::SetSyncBackboneUri { uri } => {
            next.sync_backbone_uri = uri.clone();
        }
        ShellCommand::SetSyncCardKind { kind } => {
            next.sync_card_kind = *kind;
        }
        ShellCommand::SetSyncDraftPath { path } => {
            next.sync_draft_path = path.clone();
        }
        ShellCommand::SetDocumentSyncStatus { document_id, status } => {
            require_non_empty(document_id, "document_id")?;
            next.sync_status_by_document.insert(document_id.clone(), status.clone());
        }

        ShellCommand::SetMergePolicy { policy } => {
            next.merge_policy = *policy;
        }
        ShellCommand::SetConflicts { conflicts } => {
            next.conflicts = conflicts.clone();
            if let Some(selected) = &next.selected_conflict_id {
                if !next.conflicts.iter().any(|c| &c.conflict_id == selected) {
                    next.selected_conflict_id = None;
                }
            }
        }
        ShellCommand::SelectConflict { conflict_id } => {
            if let Some(id) = conflict_id {
                require_non_empty(id, "conflict_id")?;
                if !next.conflicts.iter().any(|c| &c.conflict_id == id) {
                    return Err(ShellError::UnknownConflict { conflict_id: id.clone() });
                }
            }
            next.selected_conflict_id = conflict_id.clone();
        }

        ShellCommand::SetStorageScope { scope } => {
            next.storage_scope = *scope;
        }
        ShellCommand::SetOpeningPreference { role, dialect_id } => {
            require_non_empty(role, "role")?;
            match dialect_id {
                Some(id) => {
                    next.opening_preferences.insert(role.clone(), id.clone());
                }
                None => {
                    next.opening_preferences.remove(role);
                }
            }
        }
    }

    let _ = now_ms;
    next.revision = state.revision + 1;
    events.push(ShellEvent::Applied { capability_id, revision: next.revision });
    Ok((next, events))
}
//#endregion 🧮️reduce

//#region 🛰️ShellCapability
/// 🛰️ One machine-readable descriptor per [`ShellCommand`] variant — what an MCP gateway (a later
/// packet) compiles into its tool catalog. Defined locally (packet §3: "do NOT depend on the
/// gateway crate or on `🛂️manifest`").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ShellCapability {
    /// Stable dotted id, e.g. `ui.window.focus`. Reuses the wgpu shell's existing informal
    /// `shell.*` verb strings where one already names this exact mutation (see
    /// `📓️terra-P9-report.md` "existing-verb → variant mapping" for the full table); coined fresh,
    /// domain-shaped, where no verb existed yet.
    pub id: String,
    pub title: String,
    pub description: String,
    /// JSON Schema for this variant's payload (schemars-derived from [`ShellCommand`]).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub schema: JsonValue,
    /// True for commands that only stage/observe data (no mutation with externally-visible
    /// consequence beyond the field itself) — none of today's commands are observable-only, this
    /// flag exists for the gateway to distinguish future read-modeling commands if any are added.
    pub observable_only: bool,
}

struct CommandMeta {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    observable_only: bool,
}

/// 🆔 Returns the [`ShellCapability::id`] for a command instance — used by [`reduce`] to stamp
/// [`ShellEvent::Applied`]. Declaration order here MUST match [`ShellCommand`]'s declaration
/// order and [`SHELL_COMMAND_CATALOG`]'s order; `shell_capabilities_declaration_order_matches_enum`
/// guards this.
fn capability_id_for(command: &ShellCommand) -> String {
    let index = match command {
        ShellCommand::RegisterLoadedPlugin { .. } => 0,
        ShellCommand::UnregisterLoadedPlugin { .. } => 1,
        ShellCommand::SetPluginStatus { .. } => 2,
        ShellCommand::SetPluginSupervisorState { .. } => 3,
        ShellCommand::SetActiveSession { .. } => 4,
        ShellCommand::SetSessionError { .. } => 5,
        ShellCommand::SetAppLabelOverride { .. } => 6,
        ShellCommand::SetActionPaneFolded { .. } => 7,
        ShellCommand::SetActionPaneExpanded { .. } => 8,
        ShellCommand::StageActionArg { .. } => 9,
        ShellCommand::ResetActionArgs { .. } => 10,
        ShellCommand::SetActiveUtility { .. } => 11,
        ShellCommand::SetActiveTool { .. } => 12,
        ShellCommand::SetCommandExpanded { .. } => 13,
        ShellCommand::StageCommandArg { .. } => 14,
        ShellCommand::ResetCommandArgs { .. } => 15,
        ShellCommand::SetPanelVisible { .. } => 16,
        ShellCommand::SetPanelSize { .. } => 17,
        ShellCommand::SetPanelPath { .. } => 18,
        ShellCommand::SetDockOverride { .. } => 19,
        ShellCommand::SetPanelPathMemory { .. } => 20,
        ShellCommand::SetTreeOpenState { .. } => 21,
        ShellCommand::HydrateDockUi { .. } => 22,
        ShellCommand::ResetDock => 23,
        ShellCommand::FocusWindow { .. } => 24,
        ShellCommand::SetShellLayout { .. } => 25,
        ShellCommand::SetActiveExample { .. } => 26,
        ShellCommand::SetMobilePanelPath { .. } => 27,
        ShellCommand::SetMobilePanelVisible { .. } => 28,
        ShellCommand::SetExtraWindows { .. } => 29,
        ShellCommand::SetWindowTitle { .. } => 30,
        ShellCommand::SetWindowIcon { .. } => 31,
        ShellCommand::SetSearchOpen { .. } => 32,
        ShellCommand::SetFindOpen { .. } => 33,
        ShellCommand::AutoStartIntroduction { .. } => 34,
        ShellCommand::SetIntroductionStep { .. } => 35,
        ShellCommand::CompleteIntroductionInteraction { .. } => 36,
        ShellCommand::OpenDialog { .. } => 37,
        ShellCommand::CloseDialog { .. } => 38,
        ShellCommand::ShowTransientNotice { .. } => 39,
        ShellCommand::DismissTransientNotice => 40,
        ShellCommand::SetOpenWithFocusRole { .. } => 41,
        ShellCommand::SetActiveTutorial { .. } => 42,
        ShellCommand::SetUiAppearance { .. } => 43,
        ShellCommand::SetUiLayout { .. } => 44,
        ShellCommand::SetUiDriver { .. } => 45,
        ShellCommand::SetUiCustomDriver { .. } => 46,
        ShellCommand::SetUiDriverDraft { .. } => 47,
        ShellCommand::SetUiLocale { .. } => 48,
        ShellCommand::SetUiTerminology { .. } => 49,
        ShellCommand::SetUiTheme { .. } => 50,
        ShellCommand::SetUiCustomTheme { .. } => 51,
        ShellCommand::SetUiThemeDraft { .. } => 52,
        ShellCommand::SetUiKeybindingOverride { .. } => 53,
        ShellCommand::SetSyncBackboneUri { .. } => 54,
        ShellCommand::SetSyncCardKind { .. } => 55,
        ShellCommand::SetSyncDraftPath { .. } => 56,
        ShellCommand::SetDocumentSyncStatus { .. } => 57,
        ShellCommand::SetMergePolicy { .. } => 58,
        ShellCommand::SetConflicts { .. } => 59,
        ShellCommand::SelectConflict { .. } => 60,
        ShellCommand::SetStorageScope { .. } => 61,
        ShellCommand::SetOpeningPreference { .. } => 62,
    };
    SHELL_COMMAND_CATALOG[index].id.to_string()
}

/// 🗂️ One entry per [`ShellCommand`] variant, in declaration order. `id`s reuse the wgpu shell's
/// existing `shell.*` verb strings where one already exists for this exact mutation (see
/// `📓️terra-P9-report.md`), and are coined fresh in the same dotted-noun style otherwise.
const SHELL_COMMAND_CATALOG: [CommandMeta; 63] = [
    CommandMeta { id: "plugin.register", title: "Register Loaded Plugin", description: "Add or replace a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.unregister", title: "Unregister Loaded Plugin", description: "Remove a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.setStatus", title: "Set Plugin Status", description: "Set a plugin panel's open/collapsed/error status.", observable_only: false },
    CommandMeta { id: "plugin.setSupervisorState", title: "Set Plugin Supervisor State", description: "Update a plugin's resource/failure monitoring summary.", observable_only: false },
    CommandMeta { id: "plugin.setActiveSession", title: "Set Active Session", description: "Switch the active app instance binding.", observable_only: false },
    CommandMeta { id: "plugin.setSessionError", title: "Set Session Error", description: "Set or clear the top-level shell error message.", observable_only: false },
    CommandMeta { id: "ui.app.setLabelOverride", title: "Set App Label Override", description: "Set or clear one app-specific label customization.", observable_only: false },
    CommandMeta { id: "shell.action.fold", title: "Set Action Pane Folded", description: "Collapse or expand a window's action rail.", observable_only: false },
    CommandMeta { id: "shell.action.expand", title: "Set Action Pane Expanded", description: "Open or close a window's action arg form.", observable_only: false },
    CommandMeta { id: "shell.action.stageArg", title: "Stage Action Arg", description: "Buffer one action argument value.", observable_only: false },
    CommandMeta { id: "shell.action.reset", title: "Reset Action Args", description: "Clear all staged args for one action.", observable_only: false },
    CommandMeta { id: "ui.window.setActiveUtility", title: "Set Active Utility", description: "Switch a window's host-owned active utility.", observable_only: false },
    CommandMeta { id: "ui.tool.setActive", title: "Set Active Tool", description: "Switch the host-owned active mode tool.", observable_only: false },
    CommandMeta { id: "ui.command.expand", title: "Set Command Expanded", description: "Open or close a command palette entry's arg form.", observable_only: false },
    CommandMeta { id: "ui.command.stageArg", title: "Stage Command Arg", description: "Buffer one command palette argument value.", observable_only: false },
    CommandMeta { id: "ui.command.reset", title: "Reset Command Args", description: "Clear all staged args for one command.", observable_only: false },
    CommandMeta { id: "shell.panelToggle", title: "Set Panel Visible", description: "Show or hide a docked panel.", observable_only: false },
    CommandMeta { id: "ui.panel.setSize", title: "Set Panel Size", description: "Resize a docked panel.", observable_only: false },
    CommandMeta { id: "shell.panel.tab", title: "Set Panel Path", description: "Navigate a docked panel's breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.dock.setOverride", title: "Set Dock Override", description: "Persist a user dock rearrangement.", observable_only: false },
    CommandMeta { id: "ui.panel.setPathMemory", title: "Set Panel Path Memory", description: "Remember (or forget) a drill-down tab per panel.", observable_only: false },
    CommandMeta { id: "ui.tree.setOpenState", title: "Set Tree Open State", description: "Expand or collapse a tree section.", observable_only: false },
    CommandMeta { id: "ui.dock.hydrate", title: "Hydrate Dock UI", description: "Restore dock layout and panel visibility from a snapshot.", observable_only: false },
    CommandMeta { id: "ui.dock.reset", title: "Reset Dock", description: "Clear the dock override, returning to the default layout.", observable_only: false },
    CommandMeta { id: "ui.window.focus", title: "Focus Window", description: "Change the focused window instance.", observable_only: false },
    CommandMeta { id: "ui.shell.setLayout", title: "Set Shell Layout", description: "Change the window split/stack arrangement.", observable_only: false },
    CommandMeta { id: "ui.example.setActive", title: "Set Active Example", description: "Switch the active catalog example.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelPath", title: "Set Mobile Panel Path", description: "Change the mobile panel breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelVisible", title: "Set Mobile Panel Visible", description: "Show or hide the mobile panel.", observable_only: false },
    CommandMeta { id: "ui.window.setExtraWindows", title: "Set Extra Windows", description: "Replace the spawned extra window list.", observable_only: false },
    CommandMeta { id: "ui.window.setTitle", title: "Set Window Title", description: "Set a live window title override.", observable_only: false },
    CommandMeta { id: "ui.window.setIcon", title: "Set Window Icon", description: "Set a live window icon override.", observable_only: false },
    CommandMeta { id: "ui.search.setOpen", title: "Set Search Open", description: "Show or hide the global search panel.", observable_only: false },
    CommandMeta { id: "ui.find.setOpen", title: "Set Find Open", description: "Show or hide the find-in-window panel.", observable_only: false },
    CommandMeta { id: "ui.introduction.autoStart", title: "Auto-Start Introduction", description: "Mark a walkthrough as auto-started this session.", observable_only: false },
    CommandMeta { id: "ui.introduction.setStep", title: "Set Introduction Step", description: "Change the walkthrough step index.", observable_only: false },
    CommandMeta { id: "ui.introduction.completeInteraction", title: "Complete Introduction Interaction", description: "Mark one walkthrough step interaction done.", observable_only: false },
    CommandMeta { id: "ui.dialog.open", title: "Open Dialog", description: "Push a dialog onto the open-dialog stack.", observable_only: false },
    CommandMeta { id: "ui.dialog.close", title: "Close Dialog", description: "Pop a dialog off the open-dialog stack (top, or by id).", observable_only: false },
    CommandMeta { id: "ui.notice.show", title: "Show Transient Notice", description: "Show a non-blocking auto-dismiss notice.", observable_only: false },
    CommandMeta { id: "ui.notice.dismiss", title: "Dismiss Transient Notice", description: "Dismiss the current transient notice.", observable_only: false },
    CommandMeta { id: "ui.open.setFocusRole", title: "Set Open-With Focus Role", description: "Set which role group the Open panel focuses.", observable_only: false },
    CommandMeta { id: "ui.tutorial.setActive", title: "Set Active Tutorial", description: "Switch the active video tutorial.", observable_only: false },
    CommandMeta { id: "os.setAppearance", title: "Set Appearance", description: "Set light/dark/system appearance.", observable_only: false },
    CommandMeta { id: "os.setDriver", title: "Set UI Layout", description: "Set default/compact UI chrome density.", observable_only: false },
    CommandMeta { id: "ui.driver.setActive", title: "Set UI Driver", description: "Select the active UI driver.", observable_only: false },
    CommandMeta { id: "ui.driver.setCustom", title: "Set Custom UI Driver", description: "Add, replace, or remove a user-defined UI driver.", observable_only: false },
    CommandMeta { id: "ui.driver.setDraft", title: "Set UI Driver Draft", description: "Set or clear the in-progress driver editor draft.", observable_only: false },
    CommandMeta { id: "os.setLocale", title: "Set UI Locale", description: "Set the interface language.", observable_only: false },
    CommandMeta { id: "os.setTerminology", title: "Set UI Terminology", description: "Set the app-specific terminology id.", observable_only: false },
    CommandMeta { id: "os.setThemeId", title: "Set UI Theme", description: "Select the active theme.", observable_only: false },
    CommandMeta { id: "ui.theme.setCustom", title: "Set Custom UI Theme", description: "Add, replace, or remove a user-defined theme.", observable_only: false },
    CommandMeta { id: "ui.theme.setDraft", title: "Set UI Theme Draft", description: "Set or clear the in-progress theme editor draft.", observable_only: false },
    CommandMeta { id: "ui.keybinding.setOverride", title: "Set Keybinding Override", description: "Set or clear a user keybinding customization.", observable_only: false },
    CommandMeta { id: "sync.setBackboneUri", title: "Set Sync Backbone URI", description: "Set or clear the hub document sync backbone URI.", observable_only: false },
    CommandMeta { id: "sync.setCardKind", title: "Set Sync Card Kind", description: "Set or clear the check-in target type.", observable_only: false },
    CommandMeta { id: "sync.setDraftPath", title: "Set Sync Draft Path", description: "Set the work-in-progress check-in path.", observable_only: false },
    CommandMeta { id: "sync.setDocumentStatus", title: "Set Document Sync Status", description: "Set one document's sync health.", observable_only: false },
    CommandMeta { id: "merge.setPolicy", title: "Set Merge Policy", description: "Set the conflict resolution strategy.", observable_only: false },
    CommandMeta { id: "merge.setConflicts", title: "Set Conflicts", description: "Replace the open conflict roster.", observable_only: false },
    CommandMeta { id: "merge.selectConflict", title: "Select Conflict", description: "Select a conflict for preview.", observable_only: false },
    CommandMeta { id: "host.setStorageScope", title: "Set Storage Scope", description: "Select the shell state storage backend.", observable_only: false },
    CommandMeta { id: "host.setOpeningPreference", title: "Set Opening Preference", description: "Set or clear the default app/dialect for a role.", observable_only: false },
];

/// 🛰️ Every [`ShellCommand`] variant's machine-readable descriptor, in declaration order.
pub fn shell_capabilities() -> Vec<ShellCapability> {
    let root_schema = schemars::schema_for!(ShellCommand);
    let root_value = serde_json::to_value(&root_schema).unwrap_or(serde_json::Value::Null);
    let one_of = root_value.get("oneOf").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    SHELL_COMMAND_CATALOG
        .iter()
        .enumerate()
        .map(|(index, meta)| ShellCapability {
            id: meta.id.to_string(),
            title: meta.title.to_string(),
            description: meta.description.to_string(),
            schema: one_of.get(index).cloned().unwrap_or(serde_json::Value::Null),
            observable_only: meta.observable_only,
        })
        .collect()
}
//#endregion 🛰️ShellCapability

//#region 🧪️tests
#[cfg(test)]
mod tests {
    use super::*;

    fn window(id: &str) -> ExtraWindowInstance {
        ExtraWindowInstance { window_id: id.to_string(), kind: "app".to_string(), params: None }
    }

    #[test]
    fn reduce_is_pure_and_increments_revision() {
        let state = ShellState::default();
        let (next, events) = reduce(&state, &ShellCommand::SetSearchOpen { open: true }, 1000).expect("accepted");
        assert_eq!(next.revision, state.revision + 1);
        assert!(next.search_open);
        assert!(!state.search_open, "input state must be untouched");
        assert!(matches!(events.last(), Some(ShellEvent::Applied { .. })));
    }

    #[test]
    fn reduce_rejects_leave_state_and_revision_untouched() {
        let state = ShellState::default();
        let err = reduce(&state, &ShellCommand::SelectConflict { conflict_id: Some("missing".to_string()) }, 1000).unwrap_err();
        assert_eq!(err, ShellError::UnknownConflict { conflict_id: "missing".to_string() });
    }

    #[test]
    fn focus_after_closing_focused_window_reassigns() {
        let mut state = ShellState::default();
        state.extra_windows = vec![window("w1"), window("w2")];
        state.active_window_id = Some("w2".to_string());
        let (next, events) = reduce(&state, &ShellCommand::SetExtraWindows { windows: vec![window("w1")] }, 1000).expect("accepted");
        assert_eq!(next.active_window_id, Some("w1".to_string()));
        assert!(events.iter().any(|e| matches!(e, ShellEvent::WindowFocusChanged { previous: Some(p), current: Some(c) } if p == "w2" && c == "w1")));
    }

    #[test]
    fn mode_tool_mutual_exclusion_tool_clears_utility() {
        let mut state = ShellState::default();
        state.active_window_id = Some("w1".to_string());
        state.active_utility_by_window.insert("w1".to_string(), Some("inspect".to_string()));
        let (next, events) = reduce(&state, &ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) }, 1000).expect("accepted");
        assert_eq!(next.active_tool_id, Some("draw".to_string()));
        assert_eq!(next.active_utility_by_window.get("w1").cloned().flatten(), None);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::ActiveUtilityChanged { .. })));
    }

    #[test]
    fn dialog_stacking_open_and_close_top() {
        let mut state = ShellState::default();
        state.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
        let (opened, _) = reduce(&state, &ShellCommand::OpenDialog { dialog_id: "confirm".to_string(), seed_args: None }, 1000).expect("accepted");
        assert_eq!(opened.dialog_stack.iter().map(|d| d.dialog_id.clone()).collect::<Vec<_>>(), vec!["settings".to_string(), "confirm".to_string()]);
        let (closed, events) = reduce(&opened, &ShellCommand::CloseDialog { dialog_id: None }, 1000).expect("accepted");
        assert_eq!(closed.dialog_stack.iter().map(|d| d.dialog_id.clone()).collect::<Vec<_>>(), vec!["settings".to_string()]);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::DialogClosed { dialog_id } if dialog_id == "confirm")));
    }

    #[test]
    fn dock_reset_clears_override_and_emits_event() {
        let mut state = ShellState::default();
        state.dock_override = Some(LayoutNode::Leaf { window_id: "w1".to_string() });
        let (next, events) = reduce(&state, &ShellCommand::ResetDock, 1000).expect("accepted");
        assert_eq!(next.dock_override, None);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::DockReset)));
    }

    #[test]
    fn panel_path_memory_keys_do_not_clobber_each_other() {
        let state = ShellState::default();
        let (s1, _) = reduce(&state, &ShellCommand::SetPanelPathMemory { panel_key: "left".to_string(), path: Some("tab-a".to_string()) }, 1000).expect("accepted");
        let (s2, _) = reduce(&s1, &ShellCommand::SetPanelPathMemory { panel_key: "right".to_string(), path: Some("tab-b".to_string()) }, 1000).expect("accepted");
        assert_eq!(s2.panel_path_memory.get("left").cloned(), Some("tab-a".to_string()));
        assert_eq!(s2.panel_path_memory.get("right").cloned(), Some("tab-b".to_string()));
    }

    #[test]
    fn shell_capabilities_declaration_order_matches_enum() {
        let caps = shell_capabilities();
        assert_eq!(caps.len(), 63);
        let mut ids: Vec<&str> = caps.iter().map(|c| c.id.as_str()).collect();
        let unique_before = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), unique_before, "capability ids must be unique");
        for cap in &caps {
            assert_ne!(cap.schema, serde_json::Value::Null, "capability {} must have a non-null schema", cap.id);
        }
    }

    #[test]
    fn set_panel_size_rejects_invalid_values() {
        let state = ShellState::default();
        let err = reduce(&state, &ShellCommand::SetPanelSize { anchor: Anchor::Left, size: -1.0 }, 1000).unwrap_err();
        assert!(matches!(err, ShellError::InvalidPanelSize { .. }));
        let err = reduce(&state, &ShellCommand::SetPanelSize { anchor: Anchor::Left, size: f32::NAN }, 1000).unwrap_err();
        assert!(matches!(err, ShellError::InvalidPanelSize { .. }));
    }

    #[cfg(feature = "typegen")]
    #[test]
    fn exports_typescript_bindings() {
        use ts_rs::TS;
        Anchor::export_all().expect("Anchor");
        ByAnchor::<bool>::export_all().expect("ByAnchor<bool>");
        SplitOrientation::export_all().expect("SplitOrientation");
        LayoutNode::export_all().expect("LayoutNode");
        DockUiState::export_all().expect("DockUiState");
        ExtraWindowInstance::export_all().expect("ExtraWindowInstance");
        IconName::export_all().expect("IconName");
        LoadedPlugin::export_all().expect("LoadedPlugin");
        PluginPanelStatus::export_all().expect("PluginPanelStatus");
        PluginSupervisorState::export_all().expect("PluginSupervisorState");
        ActiveSession::export_all().expect("ActiveSession");
        DialogState::export_all().expect("DialogState");
        NoticeKind::export_all().expect("NoticeKind");
        TransientNotice::export_all().expect("TransientNotice");
        AppRole::export_all().expect("AppRole");
        UiAppearance::export_all().expect("UiAppearance");
        UiChromeLayout::export_all().expect("UiChromeLayout");
        UiLocale::export_all().expect("UiLocale");
        UiDriver::export_all().expect("UiDriver");
        UiTheme::export_all().expect("UiTheme");
        SyncCardKind::export_all().expect("SyncCardKind");
        ArtifactSyncStatus::export_all().expect("ArtifactSyncStatus");
        MergePolicy::export_all().expect("MergePolicy");
        Conflict::export_all().expect("Conflict");
        ShellScope::export_all().expect("ShellScope");
        ShellState::export_all().expect("ShellState");
        ShellCommand::export_all().expect("ShellCommand");
        ShellEvent::export_all().expect("ShellEvent");
        ShellError::export_all().expect("ShellError");
        ShellCapability::export_all().expect("ShellCapability");
    }

    /// 🏭️ Dev-only fixture generator — NOT part of the public API, gated by `#[ignore]` so a plain
    /// `cargo test` never writes files (this crate's normal test run stays pure/side-effect-free).
    /// Run explicitly via `cargo test --ignored write_fixtures -- --nocapture` to (re)generate
    /// `../../🧫️fixtures/*.json`. Every fixture here is a real `reduce()` output — the Rust parity
    /// test below re-derives it (a regression guard), and the independent TypeScript reducer test
    /// loads the same files and re-derives it against ITS OWN implementation (the real
    /// twin-honesty check the packet brief asks for).
    #[test]
    #[ignore]
    fn write_fixtures() {
        use std::fs;
        use std::path::PathBuf;

        #[derive(Serialize)]
        struct FixtureOk<'a> {
            name: &'a str,
            state: &'a ShellState,
            command: &'a ShellCommand,
            expected: FixtureOkExpected<'a>,
        }
        #[derive(Serialize)]
        struct FixtureOkExpected<'a> {
            state: &'a ShellState,
            events: &'a [ShellEvent],
        }
        #[derive(Serialize)]
        struct FixtureErr<'a> {
            name: &'a str,
            state: &'a ShellState,
            command: &'a ShellCommand,
            expected: FixtureErrExpected<'a>,
        }
        #[derive(Serialize)]
        struct FixtureErrExpected<'a> {
            error: &'a ShellError,
        }

        let dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("🧫️fixtures");
        fs::create_dir_all(&dir).expect("create fixtures dir");
        for entry in fs::read_dir(&dir).expect("read fixtures dir") {
            let entry = entry.expect("dir entry");
            if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
                fs::remove_file(entry.path()).expect("clear stale fixture");
            }
        }

        let write_ok = |name: &str, state: ShellState, command: ShellCommand| {
            let (result_state, result_events) = reduce(&state, &command, 1_700_000_000_000).expect(name);
            let fixture = FixtureOk { name, state: &state, command: &command, expected: FixtureOkExpected { state: &result_state, events: &result_events } };
            let path = dir.join(format!("{name}.json"));
            fs::write(&path, serde_json::to_string_pretty(&fixture).expect("serialize")).expect("write fixture");
        };
        let write_err = |name: &str, state: ShellState, command: ShellCommand| {
            let error = reduce(&state, &command, 1_700_000_000_000).expect_err(name);
            let fixture = FixtureErr { name, state: &state, command: &command, expected: FixtureErrExpected { error: &error } };
            let path = dir.join(format!("{name}.json"));
            fs::write(&path, serde_json::to_string_pretty(&fixture).expect("serialize")).expect("write fixture");
        };

        let base = ShellState::default();

        // One fixture per ShellCommand variant.
        write_ok("register-loaded-plugin", base.clone(), ShellCommand::RegisterLoadedPlugin { plugin: LoadedPlugin { plugin_id: "cad".to_string(), module_url: "https://plugins.example/cad.wasm".to_string(), label: Some("CAD".to_string()) } });
        {
            let mut s = base.clone();
            s.loaded_plugins.push(LoadedPlugin { plugin_id: "cad".to_string(), module_url: "https://plugins.example/cad.wasm".to_string(), label: None });
            write_ok("unregister-loaded-plugin", s, ShellCommand::UnregisterLoadedPlugin { plugin_id: "cad".to_string() });
        }
        write_ok("set-plugin-status", base.clone(), ShellCommand::SetPluginStatus { plugin_id: "cad".to_string(), status: PluginPanelStatus::Open });
        write_ok("set-plugin-supervisor-state", base.clone(), ShellCommand::SetPluginSupervisorState { plugin_id: "cad".to_string(), state: PluginSupervisorState { healthy: true, restart_count: 0, last_signal_ms: Some(1000) } });
        write_ok("set-active-session", base.clone(), ShellCommand::SetActiveSession { session: Some(ActiveSession { plugin_id: "cad".to_string(), app_id: "modeler".to_string(), instance_id: 1 }) });
        write_ok("set-session-error", base.clone(), ShellCommand::SetSessionError { error: Some("plugin failed to load".to_string()) });
        write_ok("set-app-label-override", base.clone(), ShellCommand::SetAppLabelOverride { app_id: "cad".to_string(), label_key: "toolbar.extrude".to_string(), value: Some("Push/Pull".to_string()) });
        write_ok("set-action-pane-folded", base.clone(), ShellCommand::SetActionPaneFolded { window_id: "w1".to_string(), folded: true });
        write_ok("set-action-pane-expanded", base.clone(), ShellCommand::SetActionPaneExpanded { window_id: "w1".to_string(), action_id: Some("translateSelection".to_string()) });
        write_ok("stage-action-arg", base.clone(), ShellCommand::StageActionArg { window_id: "w1".to_string(), action_id: "translateSelection".to_string(), arg_id: "dx".to_string(), value: serde_json::json!(1.5) });
        {
            let mut s = base.clone();
            s.staged_action_args.entry("w1".to_string()).or_default().entry("translateSelection".to_string()).or_default().insert("dx".to_string(), serde_json::json!(1.5));
            write_ok("reset-action-args", s, ShellCommand::ResetActionArgs { window_id: "w1".to_string(), action_id: "translateSelection".to_string() });
        }
        write_ok("set-active-utility", base.clone(), ShellCommand::SetActiveUtility { window_id: "w1".to_string(), utility_id: Some("inspect".to_string()) });
        write_ok("set-active-tool", base.clone(), ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) });
        write_ok("set-command-expanded", base.clone(), ShellCommand::SetCommandExpanded { command_id: Some("os.setAppearance".to_string()) });
        write_ok("stage-command-arg", base.clone(), ShellCommand::StageCommandArg { command_id: "os.setAppearance".to_string(), arg_id: "value".to_string(), value: serde_json::json!("dark") });
        {
            let mut s = base.clone();
            s.staged_command_args.entry("os.setAppearance".to_string()).or_default().insert("value".to_string(), serde_json::json!("dark"));
            write_ok("reset-command-args", s, ShellCommand::ResetCommandArgs { command_id: "os.setAppearance".to_string() });
        }
        write_ok("set-panel-visible", base.clone(), ShellCommand::SetPanelVisible { anchor: Anchor::Left, visible: true });
        write_ok("set-panel-size", base.clone(), ShellCommand::SetPanelSize { anchor: Anchor::Left, size: 320.0 });
        write_ok("set-panel-path", base.clone(), ShellCommand::SetPanelPath { anchor: Anchor::Left, path: vec!["explorer".to_string(), "documents".to_string()] });
        write_ok("set-dock-override", base.clone(), ShellCommand::SetDockOverride { dock: Some(LayoutNode::Leaf { window_id: "w1".to_string() }) });
        write_ok("set-panel-path-memory", base.clone(), ShellCommand::SetPanelPathMemory { panel_key: "left".to_string(), path: Some("tab-a".to_string()) });
        {
            let mut s = base.clone();
            s.panel_path_memory.insert("left".to_string(), "tab-a".to_string());
            s.panel_path_memory.insert("right".to_string(), "tab-b".to_string());
            write_ok("panel-path-memory-keys-independent", s, ShellCommand::SetPanelPathMemory { panel_key: "right".to_string(), path: Some("tab-c".to_string()) });
        }
        write_ok("set-tree-open-state", base.clone(), ShellCommand::SetTreeOpenState { tree_id: "layers".to_string(), open: true });
        write_ok("hydrate-dock-ui", base.clone(), ShellCommand::HydrateDockUi { dock: Some(DockUiState { layout: Some(LayoutNode::Leaf { window_id: "w1".to_string() }), panels_visible: ByAnchor::uniform(true) }) });
        {
            let mut s = base.clone();
            s.dock_override = Some(LayoutNode::Leaf { window_id: "w1".to_string() });
            write_ok("reset-dock", s, ShellCommand::ResetDock);
        }
        write_ok("focus-window", base.clone(), ShellCommand::FocusWindow { window_id: Some("w1".to_string()) });
        {
            let mut s = base.clone();
            s.extra_windows = vec![window("w1"), window("w2")];
            s.active_window_id = Some("w2".to_string());
            write_ok("focus-after-closing-focused-window", s, ShellCommand::SetExtraWindows { windows: vec![window("w1")] });
        }
        write_ok(
            "set-shell-layout",
            base.clone(),
            ShellCommand::SetShellLayout {
                layout: Some(LayoutNode::Split { orientation: SplitOrientation::Horizontal, children: vec![LayoutNode::Leaf { window_id: "w1".to_string() }, LayoutNode::Leaf { window_id: "w2".to_string() }], sizes: vec![0.5, 0.5] }),
            },
        );
        write_ok("set-active-example", base.clone(), ShellCommand::SetActiveExample { example_id: "gallery.chair".to_string() });
        write_ok("set-mobile-panel-path", base.clone(), ShellCommand::SetMobilePanelPath { path: vec!["home".to_string()] });
        write_ok("set-mobile-panel-visible", base.clone(), ShellCommand::SetMobilePanelVisible { visible: true });
        write_ok("set-extra-windows", base.clone(), ShellCommand::SetExtraWindows { windows: vec![window("w1")] });
        write_ok("set-window-title", base.clone(), ShellCommand::SetWindowTitle { window_id: "w1".to_string(), title: "Untitled Model".to_string() });
        write_ok("set-window-icon", base.clone(), ShellCommand::SetWindowIcon { window_id: "w1".to_string(), icon: IconName("cube".to_string()) });
        write_ok("set-search-open", base.clone(), ShellCommand::SetSearchOpen { open: true });
        write_ok("set-find-open", base.clone(), ShellCommand::SetFindOpen { open: true });
        write_ok("auto-start-introduction", base.clone(), ShellCommand::AutoStartIntroduction { key: "welcome".to_string() });
        write_ok("set-introduction-step", base.clone(), ShellCommand::SetIntroductionStep { step_index: Some(2) });
        write_ok("complete-introduction-interaction", base.clone(), ShellCommand::CompleteIntroductionInteraction { interaction_index: 3 });
        write_ok("open-dialog", base.clone(), ShellCommand::OpenDialog { dialog_id: "settings".to_string(), seed_args: None });
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            write_ok("close-dialog-top", s, ShellCommand::CloseDialog { dialog_id: None });
        }
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            write_ok("dialog-stacking-open-second", s, ShellCommand::OpenDialog { dialog_id: "confirm".to_string(), seed_args: Some(serde_json::json!({"prompt": "Discard changes?"})) });
        }
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            s.dialog_stack.push(DialogState { dialog_id: "confirm".to_string(), seed_args: None });
            write_ok("dialog-stacking-close-top-keeps-rest", s, ShellCommand::CloseDialog { dialog_id: None });
        }
        write_ok("show-transient-notice", base.clone(), ShellCommand::ShowTransientNotice { notice: TransientNotice { message: "Saved".to_string(), kind: NoticeKind::Success, expires_at_ms: Some(1_700_000_003_000) } });
        {
            let mut s = base.clone();
            s.transient_notice = Some(TransientNotice { message: "Saved".to_string(), kind: NoticeKind::Success, expires_at_ms: None });
            write_ok("dismiss-transient-notice", s, ShellCommand::DismissTransientNotice);
        }
        write_ok("set-open-with-focus-role", base.clone(), ShellCommand::SetOpenWithFocusRole { role: Some(AppRole("editor".to_string())) });
        write_ok("set-active-tutorial", base.clone(), ShellCommand::SetActiveTutorial { tutorial_id: Some("getting-started".to_string()) });
        write_ok("set-ui-appearance", base.clone(), ShellCommand::SetUiAppearance { appearance: UiAppearance::Dark });
        write_ok("set-ui-layout", base.clone(), ShellCommand::SetUiLayout { layout: UiChromeLayout::Compact });
        write_ok("set-ui-driver", base.clone(), ShellCommand::SetUiDriver { driver_id: "default".to_string() });
        write_ok(
            "set-ui-custom-driver",
            base.clone(),
            ShellCommand::SetUiCustomDriver { driver_id: "custom-1".to_string(), driver: Some(UiDriver { driver_id: "custom-1".to_string(), label: "My Driver".to_string(), config: serde_json::json!({}) }) },
        );
        write_ok("set-ui-driver-draft", base.clone(), ShellCommand::SetUiDriverDraft { draft: Some(UiDriver { driver_id: "draft".to_string(), label: "Draft".to_string(), config: serde_json::json!({}) }) });
        write_ok("set-ui-locale", base.clone(), ShellCommand::SetUiLocale { locale: UiLocale::De });
        write_ok("set-ui-terminology", base.clone(), ShellCommand::SetUiTerminology { terminology_id: "architecture".to_string() });
        write_ok("set-ui-theme", base.clone(), ShellCommand::SetUiTheme { theme_id: "mono".to_string() });
        write_ok(
            "set-ui-custom-theme",
            base.clone(),
            ShellCommand::SetUiCustomTheme { theme_id: "custom-1".to_string(), theme: Some(UiTheme { theme_id: "custom-1".to_string(), label: "My Theme".to_string(), tokens: HashMap::from([("accent".to_string(), "#f00".to_string())]) }) },
        );
        write_ok("set-ui-theme-draft", base.clone(), ShellCommand::SetUiThemeDraft { draft: Some(UiTheme { theme_id: "draft".to_string(), label: "Draft".to_string(), tokens: HashMap::new() }) });
        write_ok("set-ui-keybinding-override", base.clone(), ShellCommand::SetUiKeybindingOverride { control_id: "os.toggleFullscreen".to_string(), keys: Some("Cmd+Ctrl+F".to_string()) });
        write_ok("set-sync-backbone-uri", base.clone(), ShellCommand::SetSyncBackboneUri { uri: Some("hub://space/doc".to_string()) });
        write_ok("set-sync-card-kind", base.clone(), ShellCommand::SetSyncCardKind { kind: Some(SyncCardKind::Folder) });
        write_ok("set-sync-draft-path", base.clone(), ShellCommand::SetSyncDraftPath { path: "/tmp/checkin".to_string() });
        write_ok("set-document-sync-status", base.clone(), ShellCommand::SetDocumentSyncStatus { document_id: "doc-1".to_string(), status: ArtifactSyncStatus::Dirty });
        write_ok("set-merge-policy", base.clone(), ShellCommand::SetMergePolicy { policy: MergePolicy::PreferLocal });
        write_ok("set-conflicts", base.clone(), ShellCommand::SetConflicts { conflicts: vec![Conflict { conflict_id: "c1".to_string(), document_id: "doc-1".to_string(), description: "concurrent edit".to_string() }] });
        {
            let mut s = base.clone();
            s.conflicts = vec![Conflict { conflict_id: "c1".to_string(), document_id: "doc-1".to_string(), description: "concurrent edit".to_string() }];
            write_ok("select-conflict", s, ShellCommand::SelectConflict { conflict_id: Some("c1".to_string()) });
        }
        write_ok("set-storage-scope", base.clone(), ShellCommand::SetStorageScope { scope: ShellScope::LocalStorage });
        write_ok("set-opening-preference", base.clone(), ShellCommand::SetOpeningPreference { role: "editor".to_string(), dialect_id: Some("cad.modeler".to_string()) });

        // Mode↔tool mutual exclusion tricky paths.
        {
            let mut s = base.clone();
            s.active_window_id = Some("w1".to_string());
            s.active_utility_by_window.insert("w1".to_string(), Some("inspect".to_string()));
            write_ok("mode-tool-exclusion-tool-clears-utility", s, ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) });
        }
        {
            let mut s = base.clone();
            s.active_window_id = Some("w1".to_string());
            s.active_tool_id = Some("draw".to_string());
            write_ok("mode-tool-exclusion-utility-clears-tool", s, ShellCommand::SetActiveUtility { window_id: "w1".to_string(), utility_id: Some("inspect".to_string()) });
        }

        // Error fixtures.
        write_err("error-unregister-unknown-plugin", base.clone(), ShellCommand::UnregisterLoadedPlugin { plugin_id: "missing".to_string() });
        write_err("error-close-dialog-empty-stack", base.clone(), ShellCommand::CloseDialog { dialog_id: None });
        write_err("error-close-dialog-unknown-id", base.clone(), ShellCommand::CloseDialog { dialog_id: Some("missing".to_string()) });
        write_err("error-select-unknown-conflict", base.clone(), ShellCommand::SelectConflict { conflict_id: Some("missing".to_string()) });
        write_err("error-set-panel-size-negative", base.clone(), ShellCommand::SetPanelSize { anchor: Anchor::Left, size: -5.0 });
        write_err("error-set-window-title-empty-id", base.clone(), ShellCommand::SetWindowTitle { window_id: String::new(), title: "x".to_string() });

        let count = fs::read_dir(&dir).expect("read fixtures dir").filter(|e| e.as_ref().unwrap().path().extension().and_then(|x| x.to_str()) == Some("json")).count();
        println!("wrote {count} fixtures to {}", dir.display());
    }

    /// 🧪️ Loads every committed fixture and re-derives it through `reduce` — the Rust half of the
    /// twin-parity mechanism (packet §3): the TypeScript test loads the same files against its own
    /// independent reducer implementation.
    #[test]
    fn fixtures_produce_expected_output() {
        use std::fs;
        use std::path::PathBuf;

        #[derive(Deserialize)]
        struct FixtureFile {
            name: String,
            state: ShellState,
            command: ShellCommand,
            expected: serde_json::Value,
        }

        let dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("🧫️fixtures");
        let entries: Vec<PathBuf> =
            fs::read_dir(&dir).expect("fixtures dir must exist — run `cargo test --ignored write_fixtures` first").filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json")).collect();
        assert!(!entries.is_empty(), "no fixtures found in {}", dir.display());

        let mut checked = 0usize;
        for path in entries {
            let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let fixture: FixtureFile = serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
            let outcome = reduce(&fixture.state, &fixture.command, 1_700_000_000_000);
            if let Some(expected_error) = fixture.expected.get("error") {
                let error = outcome.expect_err(&format!("{}: expected error", fixture.name));
                let actual_error = serde_json::to_value(&error).expect("serialize error");
                assert_eq!(&actual_error, expected_error, "fixture {} error mismatch", fixture.name);
            } else {
                let (state, events) = outcome.unwrap_or_else(|e| panic!("{}: expected ok, got {e:?}", fixture.name));
                let expected_state = fixture.expected.get("state").cloned().expect("expected.state");
                let expected_events = fixture.expected.get("events").cloned().expect("expected.events");
                assert_eq!(serde_json::to_value(&state).expect("serialize state"), expected_state, "fixture {} state mismatch", fixture.name);
                assert_eq!(serde_json::to_value(&events).expect("serialize events"), expected_events, "fixture {} events mismatch", fixture.name);
            }
            checked += 1;
        }
        assert!(checked >= 63, "expected at least one fixture per ShellCommand variant (63), found {checked}");
    }
}
//#endregion 🧪️tests
