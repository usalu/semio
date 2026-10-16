//! 🕹️ `shell-input` component structs: `ShellInputState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🕹️ `ShellInputState`: scroll, overlay, selection, context menu, tree drag, widget maps and the retained input geometry.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `input`.
pub struct ShellInputState {
    pub scroll_offsets: HashMap<String, f32>,
    pub overlay_state: OverlayState,
    pub collapsed_sections: HashMap<String, bool>,
    pub tree_open_states: BTreeMap<String, bool>,
    pub accessibility_focused_control_id: Option<String>,
    pub interaction_selection: HashMap<String, semio_framework::DomainSelection>,
    pub open_selects: HashMap<String, bool>,
    pub context_menu: Option<ContextMenuState>,
    pub right_click: RightClickState,
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<ActionDescriptor>,
    pub widget_maps_staging: WidgetInteractionMaps<ActionDescriptor>,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    /// 🪟️ The tab whose label is held down, selected on release — React's `onSelectTab` fires from
    /// the label BUTTON's click, while a drag can only start from the tab's own grip chip.
    /// 🖱️ The exact chrome control armed by a primary press and eligible to activate only when the
    /// matching release lands on that same control. Shared by dock tab labels and palette rows.
    pub pending_chrome_release: Option<String>,
    pub chrome_pointer_owner: Option<ui_render::PointerId>,
    /// ⌨️ The shortcut row whose next key event is a CHORD WRITE rather than a shell verb —
    /// React's `keybindingCaptureControlId` (`🏛️ShellHost/🟦️.tsx`).
    pub keybinding_capture_control_id: Option<String>,
    /// 🎯️ Control id → the retained body that minted it this frame build, and the rect that body was
    /// painted into. The registry `InputState` keeps is flat and carries no owner, so this is what
    /// tells a document hit (route it into that body's retained tree, in that body's own local
    /// coordinates) from a chrome hit (route it through the shell's own handlers). Panels and dock
    /// windows are one map: both paint a retained document, and only their rect differs
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎯️ Double-buffered against `ui_wgpu::wgpu::HitRegistry` and published in the same step: this
    /// is the owner map of the last COMPLETE chrome walk, so an id `InputState::hit_at` resolves is
    /// always one this map can name. `retained_hit_windows_staging` is the map the walk in progress
    /// is filling.
    pub retained_hit_windows: HashMap<String, (String, Rect)>,
    pub retained_hit_windows_staging: HashMap<String, (String, Rect)>,
    pub retained_scene_hits: HashMap<String, (Rect, crate::element_contracts::ScenePointerTarget)>,
    pub retained_scene_hits_staging: HashMap<String, (Rect, crate::element_contracts::ScenePointerTarget)>,
    pub presented_input_epoch: u64,
    pub next_presented_input_epoch: Option<u64>,
    pub presented_input_candidate: Option<PresentedInputCandidateWitness>,
    pub presented_input_geometry: PresentedInputGeometry,
    pub presented_input_geometry_staging: PresentedInputGeometry,
    pub retained_body_rects_staging: Vec<(String, Rect)>,
    /// 🪟️ The pane-overlay chip rows this frame's windows registered, held back until every window
    /// BODY has been walked and registered before the first docked panel.
    ///
    /// ⚖️ `InputState::hit_at` resolves in reverse registration order, so this deferral is React's
    /// own three-level stack read as an order: a chip outranks the window body and the world surface
    /// it is painted inside (`--z-pane: 20` over `--z-window: 10`) and LOSES to any docked panel
    /// floating over it (`--z-panel: 30`, `🖱️ui/🎨️styling/🖌️ui/🎨️.css:834-836`). React states the
    /// same rule in prose where it mounts the chips — "window pane toggles stay on their authored
    /// anchors behind anchored chrome panels; panels paint above `z-window` and occlude overlap
    /// without shifting pane chrome" (`🪟️Window/🟦️.tsx:200`).
    ///
    /// 🩸️ The rows used to be registered AFTER the panels, which made a covered chip outrank the
    /// panel over it — the opposite of React, where the press lands on the panel. Measured on the
    /// React reference with the Catalogue panel open over the top pane: pressing
    /// `framework.window.puzzle3dMainTop.{engagement,search}.toggle` journalled the catalogue row's
    /// own `addObjectKind` and left every fold closed
    /// (`🗑️generated/parity-run-12/steps.json` steps 9-10, ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12d-pane-chip-ids-and-projection-toggle.md` §1).
    pub pane_overlay_hits: Vec<HitTarget<ActionDescriptor>>,
    /// 👋️ The retained body a pointer move was last routed into, so the move that LEAVES it is
    /// routed there too. `events::EventRouter` is edge-triggered on the moves it receives, so a body
    /// that stops receiving them keeps `NodeFlags::HOVERED` on whatever it last resolved.
    pub retained_hover_window: Option<String>,
}
