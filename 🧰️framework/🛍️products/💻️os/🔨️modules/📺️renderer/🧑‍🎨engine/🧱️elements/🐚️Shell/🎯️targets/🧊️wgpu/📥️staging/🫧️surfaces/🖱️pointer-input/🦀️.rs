//! 🫧️ `shell-surfaces` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
//#region 🫥️HostWindowBlur
/// 🧯️ Frames a closed World3d window waits at most for its capture-lost cancel to drain before it retires regardless.
pub const WORLD3D_CLOSE_CANCEL_SIGHTINGS: u8 = 8;

//#endregion 🫥️HostWindowBlur
impl SurfaceState {
    /// 🎬️ Whether a live engine surface is painted under this point — the wheel's twin of the press
    /// path's own `state.bounds.contains(x, y)` claim, and the same set of surfaces the four wheel
    /// phases go on to offer the notch to.
    pub fn scene_surface_contains(&self, x: f32, y: f32) -> bool {
        self.world3d_states.values().any(|state| state.bounds.contains(x, y))
            || self.node_graph_states.values().any(|surface| surface.bounds.contains(x, y))
            || self.tiled_map_states.values().any(|surface| surface.bounds.contains(x, y))
            || self.board2d_states.values().any(|surface| surface.bounds.contains(x, y))
    }

    pub async fn handle_world3d_input(&mut self, x: f32, y: f32, down: bool, button: i16, shift: bool, ctrl: bool, alt: bool, meta: bool, wheel_delta: f32, drag_dx: f32, drag_dy: f32) -> Result<(), String> {
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            let wheel = WorldInteractionIntent::wheel(x, y, wheel_delta, &modifiers);
            let button_intent = WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers);
            let move_intent = WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers);
            let mut drag = move_intent;
            drag.phase = WorldInteractionPhase::PointerDrag;
            let has_wheel = wheel_delta.abs() > 0.0;
            let has_drag = down && (drag_dx.abs() > 0.0 || drag_dy.abs() > 0.0);
            let admitted = match (has_wheel, has_drag) {
                (true, true) => enqueue_world3d_events(&mut state.authority, [wheel, drag, button_intent, move_intent]).is_ok(),
                (true, false) => enqueue_world3d_events(&mut state.authority, [wheel, button_intent, move_intent]).is_ok(),
                (false, true) => enqueue_world3d_events(&mut state.authority, [drag, button_intent, move_intent]).is_ok(),
                (false, false) => enqueue_world3d_events(&mut state.authority, [button_intent, move_intent]).is_ok(),
            };
            if !admitted {
                return Err("world3d fixed interaction credits exhausted".into());
            }
        }
        Ok(())
    }
}

//#endregion ShellInput

