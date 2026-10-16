//! 🫧️ `shell-surfaces` component structs: `SurfaceState`, `TutorialState`, `SettleState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🫧️ `SurfaceState`: the surfaces the shell hosts: world3d, node graph, tiled map, icon render and board2d states with their retirement ledger.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `surfaces`.
pub struct SurfaceState {
    pub world3d_states: AdmittedSurfaceMap<World3dState>,
    pub world3d_window_ids: HashMap<String, String>,
    /// 🪦 Closed World3d owners, detached from input immediately and drained one bounded step per
    /// frame before Drop. Keys include a shell epoch so reopening the same window id is independent.
    pub retired_world3d_states: VecDeque<(String, World3dState)>,
    /// 🧯️ Closed World3d owners whose `cancel(CaptureLost)` waits for one drive of their world authority before they retire,
    /// with the sightings so far ([`ShellState::world3d_close_cancel_settled`]).
    pub world3d_close_cancels: BTreeMap<String, u8>,
    pub component_world3d_retirement: Option<semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceCloseOwner<World3dState>>,
    pub world3d_retirement_epoch: u64,
    pub world3d_retirement_sequence: u64,
    pub world3d_retirement_fault: Option<infinite_world::world::WorldDynamicFault>,
    /// 🛑️ Per-World3d-surface compute status, mirrored out of the scene by the same per-frame attach
    /// walk that mirrors bounds. The CANCEL CONTRACT (`cancellable` + `cancelAction`) is read from
    /// here and from nowhere else, so the shell offers a stop affordance without ever learning a
    /// domain verb from code — see `world3d_cancel_affordance`.
    pub world3d_status: HashMap<String, String>,
    /// ⏳️ The last compute-status pill label announced per World3d surface, so the trace prints once
    /// per CHANGE rather than once per frame — the pill itself is recomputed from `world3d_status`
    /// every grant and holds no state of its own.
    pub world3d_status_pill_trace: HashMap<String, String>,
    pub node_graph_states: AdmittedSurfaceMap<NodeGraphSurface>,
    pub tiled_map_states: AdmittedSurfaceMap<TiledMapSurface>,
    pub icon_render_states: HashMap<String, World3dState>,
    pub board2d_states: AdmittedSurfaceMap<Board2dSurface>,
}

/// 🎓️ `TutorialState`: the tutorial runtime and its deferred document operations.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `tutorial`.
pub struct TutorialState {
    /// 🎬️ Active tutorial playback/recording runtime, if any — see `//#region 🎬️Tutorial` (below
    /// `ShellChrome`) for `TutorialRuntime`'s full shape, lifecycle, and the player/recorder it drives.
    pub tutorial: Option<TutorialRuntime>,
    /// 🎬️ Document-track operations queued by a tutorial tick/seek this frame, drained and applied
    /// asynchronously right after `render_chrome` returns (mirrors how `AppRuntime::frame` already defers
    /// `scene_events`/wheel actions through `spawn_app_task` for the same reason: the plugin bridge's
    /// `apply_mutations`/`handle_action` calls are async, but chrome rendering isn't).
    pub tutorial_pending_document_ops: Vec<TutorialPendingDocOp>,
}

/// 🫀️ `SettleState`: the settle lane: deferred actions, owed refresh scope, the pump and pending shell I/O.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `settle`.
pub struct SettleState {
    pub deferred_actions: Vec<ActionDescriptor>,
    /// 🔁️ Guards `flush_deferred_actions`'s convergence loop against re-entry: a nested flush (one a
    /// dispatched action itself asks for) stays a plain drain so the outer loop keeps the single
    /// authority over how many times the window bodies are re-entered.
    pub settling: bool,
    /// 🐢️ The union of every [`UiDirtyScope`] declared since the last refresh pass drained it — this
    /// shell's half of React's `createUiRefreshCoalescerV1` owed slot. Producers (a dispatched
    /// action's `InvocationResult::ui_scope`, a command's, an extension answer's, plus whatever the
    /// HOST dirtied by applying that pass's own effects) only DECLARE dirt here; `refresh_ui` is the
    /// one place that takes it. Starts [`UiDirtyScope::None`], so a refresh nobody owes anything to
    /// re-renders nothing at all.
    pub owed_refresh_scope: UiDirtyScope,
    /// 🫀️ The runtime-owned settle lane's shell half — see [`ShellSettlePump`]. Producers only ARM it
    /// ([`Self::owe_settle`]); the frame loop is the one caller of [`Self::settle_pump_step`].
    pub settle_pump: ShellSettlePump,
    #[cfg(not(target_arch = "wasm32"))]
    pub shell_io_pending: std::collections::VecDeque<PendingShellIo>,
}
