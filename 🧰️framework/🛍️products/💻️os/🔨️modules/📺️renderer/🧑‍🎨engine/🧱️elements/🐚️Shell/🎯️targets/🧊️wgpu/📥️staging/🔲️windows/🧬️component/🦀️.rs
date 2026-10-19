//! 🔲️ `shell-windows` component structs: `WindowState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🔲️ `WindowState`: active window, engagement and measures chrome, window topology journal and per-window overrides.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `windows`.
pub struct WindowState {
    pub active_window_id: Option<String>,
    /// 🪟️ The active window the guest's command history was last told about, so a REAL activation is
    /// noted exactly once — the wgpu twin of React's `handleActiveWindowChange` change guard
    /// (`🏛️ShellHost/🟦️.tsx`'s `if (value) noteShellCommand("shell.windowActivate", …)`), which only
    /// fires on a changed value. Re-seeded, never noted, when the session instance changes, because a
    /// successor's landing window is a mount rather than a user activation.
    pub noted_active_window: Option<(u32, String)>,
    /// 📨️ The active window the program was last told about for host events, so the pane that loses the activation is
    /// blurred exactly once ([`Self::arm_window_blur_host_event`]).
    pub host_event_active_window: Option<(u32, String)>,
    pub engagement_expanded: HashMap<String, bool>,
    pub engagement_activated: HashMap<String, bool>,
    pub measures_folded: HashMap<String, bool>,
    pub measures_expanded: HashMap<String, bool>,
    pub measures_width: HashMap<String, f32>,
    pub measures_resize_origin_width: f32,
    pub engagement_inputs: HashMap<String, String>,
    pub measures_resize_window_id: Option<String>,
    /// 🪟️ Physical window transfers journal through the ordinary guest action lane, but only after
    /// the Full refresh publishes the required retained body. Direct Display opens intentionally
    /// have no React journal. The shared queue gives every attempted note one admission or refusal.
    pub window_topology_actions: semio_framework_ui_action::BoundedActionQueue,
    /// 🪟️ Required initial-body identities, retry counts and journal outcomes bounded to the action
    /// queue's item capacity so a refusing guest cannot grow host ownership without limit.
    pub window_topology_publications: VecDeque<WindowTopologyPublication>,
    /// 🧾️ Stable logical identities parallel to the fixed-credit action queue. A later body may
    /// publish before an earlier refusing peer without stealing that peer's action.
    pub window_topology_action_tokens: VecDeque<u64>,
    pub next_window_topology_action_token: u64,
    pub window_topology_refresh_owed: bool,
    /// 📯️ A successful cohort released at least one body-gated journal. This alone permits
    /// deferred dispatch ahead of the next retry; unrelated deferred guest work never delays the
    /// first topology refresh.
    pub window_topology_journal_dispatch_owed: bool,
    pub active_utilities: Vec<UtilityNode>,
    /// 🧰️ Host-owned active utility per window kind (never a document field, never a VCS operation).
    /// Replaces the deleted `active_utility_id`/`find_active_utility_id` "first pressed toggle" heuristic.
    pub active_utility_by_window: HashMap<String, String>,
    /// 📇️ Per-window Actions-rail fold state (absent = folded, the default).
    pub action_panel_folded: HashMap<String, bool>,
    /// 🔎️ Per-window Search-pane fold state (absent = folded, the default).
    pub search_panel_folded: HashMap<String, bool>,
    /// 🧰️ Per-window Utilities-rail fold state — React's `Window` `utilityBarFolded`
    /// (`useState(true)`, `🪟️Window/🟦️.tsx:176`), so absent = folded.
    pub utility_bar_folded: HashMap<String, bool>,
    /// 🔀️ Per-window Projection-pane fold state — React's `WorldOrbitProjectionSwitchPane`
    /// `useState(true)` (`🌐️World3dHost/🟦️.tsx:5190`), so absent = folded.
    pub projection_pane_folded: HashMap<String, bool>,
    /// 🔀️ Per-window selected projection template id — the wgpu twin of React's
    /// `externalPendingProjectionSpec`, VIEW STATE ONLY and never dispatched, exactly as
    /// `handleProjectionKindChange` records (`🌐️World3dHost/🟦️.tsx:6574`). Absent =
    /// [`WORLD_PROJECTION_DEFAULT_TEMPLATE_ID`], the template React's own
    /// `worldProjectionDefaults("threePoint")` fallback resolves to for a perspective orbit.
    pub world_projection_template: HashMap<String, String>,
    /// 🖼️ Renderer-local `SetWindowIconContext` state, owned by the exact mounted app instance and
    /// bounded by the same 64-window view-context contract as the dock roster.
    pub window_icon_overrides: HashMap<String, WindowIconOverride>,
    pub window_title_overrides: HashMap<String, WindowTitleOverride>,
    /// 📇️ Per-window expanded action id (the accordion-open staged arg form).
    pub action_panel_expanded: HashMap<String, String>,
    /// 🛠️ Live tool measures keyed by TOOL id — React's `toolMeasuresByToolId` ref, read off
    /// [`ProgramBridgeEntry::tool_measures_section`] on the same refresh pass window measures ride.
    pub tool_measures: HashMap<String, Vec<WindowMeasure>>,
    pub window_engagements: HashMap<String, WindowEngagement>,
    /// 🪪️ `(revision, generation)` last minted per Measures overlay surface — the ledger
    /// `ui_document_ingress_generation` reads, kept across retirements so a reopened overlay moves forward.
    pub window_measures_minted: HashMap<String, (u64, u64)>,
    pub utility_collection_expanded: HashMap<String, bool>,
    pub contributor_instances: HashMap<String, u32>,
}
