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
pub struct PresentedInputCandidateWitness(pub u64);

pub struct PresentedInputGeometry {
    pub retained_body_rects: Vec<(String, Rect)>,
    pub media_owners: Vec<(String, Option<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaOwner>)>,
    pub media_blocked: bool,
    pub retained_surfaces: Vec<(String, semio_framework_ui_wgpu_engine::UiSurfaceToken)>,
    pub dock_canvas_bounds: Rect,
    pub dock_axis_separator: f32,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub dock_window_plan: Vec<(String, Rect)>,
    pub window_content_rects: HashMap<String, Rect>,
    pub window_silhouettes: HashMap<String, WindowSilhouette>,
    pub modal: bool,
    pub panel_rects: Vec<Rect>,
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

//#region ShellTypes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerCaptureSlot {
    pub pointer_id: ui_render::PointerId,
    pub owner: PointerHitOwner,
    pub position: [f32; 2],
}

impl PointerCapture {
    pub fn press(&mut self, pointer_id: ui_render::PointerId, owner: PointerHitOwner, x: f32, y: f32) -> Option<PointerHitOwner> {
        let index = self.slots.iter().position(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).or_else(|| self.slots.iter().position(Option::is_none))?;
        self.slots[index] = Some(PointerCaptureSlot { pointer_id, owner, position: [x, y] });
        Some(owner)
    }

    /// 🖱️⬆️ Ends the sequence and answers the layer that owned it. A release with no press before it
    /// — the pointer entered the canvas already down — is the surface's, as it is in the DOM.
    pub fn release(&mut self, pointer_id: ui_render::PointerId) -> PointerHitOwner {
        self.slots.iter_mut().find(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).and_then(Option::take).map_or(PointerHitOwner::Surface, |slot| slot.owner)
    }

    pub fn owner_of_move(&self, pointer_id: ui_render::PointerId, under_pointer: PointerHitOwner) -> PointerHitOwner {
        self.holder(pointer_id).unwrap_or(under_pointer)
    }

    pub fn holder(&self, pointer_id: ui_render::PointerId) -> Option<PointerHitOwner> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.owner)
    }

    pub fn position(&self, pointer_id: ui_render::PointerId) -> Option<[f32; 2]> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.position)
    }

    pub fn advance(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32) -> Option<(f32, f32)> {
        let slot = self.slots.iter_mut().flatten().find(|slot| slot.pointer_id == pointer_id)?;
        let delta = (x - slot.position[0], y - slot.position[1]);
        slot.position = [x, y];
        Some(delta)
    }

    pub fn any_active(&self) -> bool {
        self.slots.iter().any(Option::is_some)
    }
}

/// 🎯️🖱️ The layer that owns the pointer SEQUENCE in flight — the DOM's implicit pointer capture,
/// which the wgpu target has to keep itself because it has one canvas and no event targets.
///
/// ⚖️ A press captures the layer it landed on and every move and the release belong to it until the
/// release ends the sequence. Both directions matter: a press on chrome keeps the whole gesture away
/// from the engine surface under it (React's `pointerdown` on a `<button>` means the canvas never
/// sees that `pointerup`), and a press on the surface keeps an orbit drag on the surface even where
/// it sweeps over a panel (three's `OrbitControls` calls `setPointerCapture` on the canvas).
///
/// 🩸️ Without it every chrome click ALSO reached the world pane painted under it — the reference
/// journals one `noteShellCommand` for a navbar panel tab where wgpu journalled
/// `noteShellCommand` + `interactionHover` + `interactionSelect`, and the stray
/// `interactionSelect` carried empty `targets`, i.e. pressing a panel tab CLEARED the world
/// selection (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md` §6 family A).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerCapture {
    pub slots: [Option<PointerCaptureSlot>; semio_framework_os_renderer_wgpu_contracts::SCENE_POINTER_OWNER_CAPACITY],
}
//#endregion ShellTypes
