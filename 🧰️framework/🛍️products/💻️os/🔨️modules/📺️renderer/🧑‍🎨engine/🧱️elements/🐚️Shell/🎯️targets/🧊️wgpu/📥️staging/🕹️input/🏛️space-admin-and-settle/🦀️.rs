//! 🕹️ `shell-input` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 🎯️ The two layers a pointer can belong to. Chrome is everything the SHELL paints — the navbar,
/// the dock's structure, a pane's own overlay chips, an anchored panel, a menu, a dropdown, a dialog
/// and the tour veil; Surface is the engine canvas under it. React gets this partition for free from
/// the DOM (its chrome is elements ABOVE the `<canvas>`), and the wgpu target has to state it, which
/// is what [`ShellState::pointer_hit_owner`] does. One owner per point, and — through
/// `AppInteractionState`'s capture — one owner per pointer SEQUENCE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerHitOwner {
    Chrome,
    Surface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PresentedInputCandidateWitness(pub(crate) u64);

pub(crate) struct PresentedInputGeometry {
    pub(crate) retained_body_rects: Vec<(String, Rect)>,
    pub(crate) media_owners: Vec<(String, Option<crate::media_slots::PresentedMediaOwner>)>,
    pub(crate) media_blocked: bool,
    pub(crate) retained_surfaces: Vec<(String, ui_wgpu::wgpu::UiSurfaceToken)>,
    pub(crate) dock_canvas_bounds: Rect,
    pub(crate) dock_axis_separator: f32,
    pub(crate) dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub(crate) dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub(crate) dock_window_plan: Vec<(String, Rect)>,
    pub(crate) window_content_rects: HashMap<String, Rect>,
    pub(crate) window_silhouettes: HashMap<String, WindowSilhouette>,
    pub(crate) modal: bool,
    pub(crate) panel_rects: Vec<Rect>,
}

impl Default for PresentedInputGeometry {
    fn default() -> Self {
        Self {
            retained_body_rects: Vec::new(),
            media_owners: Vec::new(),
            media_blocked: false,
            retained_surfaces: Vec::new(),
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_axis_separator: 0.0,
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            dock_window_plan: Vec::new(),
            window_content_rects: HashMap::new(),
            window_silhouettes: HashMap::new(),
            modal: false,
            panel_rects: Vec::new(),
        }
    }
}

//#endregion ShellTypes

