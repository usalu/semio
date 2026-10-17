//! ⚓️ `shell-dock` component structs: `ShellDockState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// ⚓️ `ShellDockState`: the dock model, anchors, drags, splits, layouts and window content geometry.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `dock`.
pub struct ShellDockState {
    /// 🧭️ Every anchor's own open/size/active-path chrome, slotted by `PanelAnchor::index` — the
    /// wgpu twin of React's `panels[anchor]` reducer state.
    pub panel_anchors: [PanelAnchorState; 8],
    /// 🧭️ The live per-anchor tab trees, `apply_dock_skeleton(default_dock, dock_override)`.
    pub dock_tabs: ShellDock,
    /// 🗄️ The persisted arrangement override, `None` while the dock equals its computed default.
    pub dock_override: Option<DockSkeleton>,
    /// 🌳️ Branch ids folded shut in an anchor's tab bar, keyed `{anchor}/{branchId}`.
    pub dock_collapsed_branches: HashMap<String, bool>,
    /// 🎯️ The tab id a pointer is dragging between anchors, plus the anchor it started in.
    pub dock_tab_drag: Option<(String, PanelAnchor)>,
    /// 🗄️ The app id whose persisted dock layer is currently loaded — the change detector that reloads
    /// the override on an app switch without clobbering a reveal on every other chrome sync.
    pub dock_loaded_app_id: Option<String>,
    pub panel_resize_origin_width: f32,
    pub dock: DockState,
    pub split_resize_path: Option<Vec<usize>>,
    pub split_resize_index: usize,
    pub split_resize_axis_total: f32,
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    /// 🖼️ The render-only dock tree this frame paints, hit-tests and solves drop zones against —
    /// `DockState::render_view` of the committed [`Self::dock`]: the mobile flat stack below React's
    /// breakpoint, the docked-out tree while a drag floats, and the committed tree otherwise. Written
    /// once per frame by `plan_dock_windows`, so the plan, the chrome and the pointer never disagree.
    pub dock_view: DockState,
    /// 🪪️ App instance whose layout was last reconciled into `dock`; commands are refused while a
    /// successor session is mounted but its dock has not crossed `sync_dock` yet.
    pub dock_instance_owner: Option<(String, u32, String)>,
    /// 🪟️ Effective incoming layout and ordered window IDs last accepted by the dock.
    pub dock_input_identity: Option<DockInputIdentity>,
    /// 🪟️ Host-owned tree windows of every body this shell paints — what crosses as `ViewModel.tree_windows`.
    pub tree_windows: TreeWindowScheduler,
    /// 🧠️ Per body, what the observer learned from the guest's answers (`TreeWindowServedMemoryV1`).
    pub tree_window_served: HashMap<String, Vec<(String, ui_wgpu::wgpu::tree_window::TreeWindowServedMemory)>>,
    pub dock_canvas_bounds: Rect,
    /// 📐️ Separator metric captured with this frame's dock plan so later pointer resize math uses
    /// the same themed physical geometry that paint, drop targets and body publication used.
    pub dock_axis_separator: f32,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    /// 🪟️ The windows this frame's chrome walk paints, in dock order, each with the content rect the
    /// dock solved for it. Written once per frame by `render_main_window_step`'s plan phase and read
    /// by its body loop — the wgpu twin of React's `shellLayout` pane walk. Before this existed the
    /// walk painted `active_window_id` alone over the whole body, so an app whose authored mode
    /// layout places two windows (generation3d's edit mode = Flow + Preview, `[68, 32]`) only ever
    /// laid out one of them and the other's engine surface was never created
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub dock_window_plan: Vec<(String, Rect)>,
    /// 🐛️ Last `[TRACE] ` dock-plan line, so the trace prints on CHANGE rather than once per frame.
    pub dock_plan_trace: Option<String>,
    pub layout_override: Option<ui_wgpu::wgpu::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    /// 🖥️ The `framework.display.layout` leaf's save-name box — React's `layoutSaveLabel`
    /// (`📌️ChromePanels/🟦️.tsx`'s `DisplayHostApi`). Session-local, exactly as React's `useState` is.
    pub layout_save_label: String,
    /// 🖥️ Layouts the user saved through `framework.display.save` — React's `displayUserLayouts`, the
    /// `origin: "user"` half of the roster, projected from the `os.config.ui-preferences` log and written back as
    /// `setNamedLayout` events ([`ShellState::load_persisted_named_layouts`]/[`ShellState::persist_named_layouts`]).
    /// The app's own `named_layouts` supply the `origin: "builtin"` half.
    pub user_layouts: Vec<ui_wgpu::wgpu::NamedLayout>,
    /// 📱️ The single mobile panel's own chrome — React's `mobilePanelVisible`/`mobilePanelPath`,
    /// the flattened counterpart of the eight `PanelAnchorState`s.
    pub mobile_panel_visible: bool,
    pub mobile_panel_path: Vec<String>,
    /// 🖱️ Last-rendered full window content bounds per window id — used to apply the active utility's
    /// cursor while the pointer is inside that window's silhouette (Architecture Decision 8, P5).
    pub window_content_rects: HashMap<String, Rect>,
    /// 🪟️ Last-rendered dock-stack silhouette per active window id (tabs + gap cutout + controls + body).
    pub window_silhouettes: HashMap<String, WindowSilhouette>,
}
