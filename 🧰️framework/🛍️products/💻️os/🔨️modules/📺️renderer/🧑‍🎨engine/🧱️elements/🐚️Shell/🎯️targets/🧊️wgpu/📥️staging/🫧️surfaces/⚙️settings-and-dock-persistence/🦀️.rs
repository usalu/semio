//! 🫧️ `shell-surfaces` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl SurfaceState {
    /// 🧯️ Whether a closed World3d window may retire now: at once unless it holds a gesture to abort
    /// (`world3d_cancel_owed`: a live gumball that streamed, an open paint stroke). Then its first sighting enqueues
    /// `cancel(CaptureLost)` (zero trace, as React's unmount does), takes the surface off the pointer's way (empty bounds) and
    /// waits for the frame's world authority to drive the cancel; it retires once its queue drained, or after
    /// [`WORLD3D_CLOSE_CANCEL_SIGHTINGS`] sightings. A cancel the full queue refuses retires at once.
    pub fn world3d_close_cancel_settled(&mut self, host_id: &str) -> bool {
        let Some(state) = self.world3d_states.get_mut(host_id) else { return true };
        let Some(sightings) = self.world3d_close_cancels.get_mut(host_id) else {
            if !infinite_world::world::world3d_cancel_owed(&state.selection, &state.authority) {
                return true;
            }
            let owed = enqueue_world3d_events(&mut state.authority, [WorldInteractionIntent::cancel(infinite_world::world::WorldCancelReason::CaptureLost)]).is_ok();
            if owed {
                state.bounds = Rect::new(0.0, 0.0, 0.0, 0.0);
                self.world3d_close_cancels.insert(host_id.to_owned(), 0);
            }
            return !owed;
        };
        *sightings = sightings.saturating_add(1);
        let settled = infinite_world::world::world3d_interaction_front_generation(&state.camera, &state.authority).is_none() || *sightings >= WORLD3D_CLOSE_CANCEL_SIGHTINGS;
        if settled {
            self.world3d_close_cancels.remove(host_id);
        }
        settled
    }
}

//#endregion ShellLifecycle

