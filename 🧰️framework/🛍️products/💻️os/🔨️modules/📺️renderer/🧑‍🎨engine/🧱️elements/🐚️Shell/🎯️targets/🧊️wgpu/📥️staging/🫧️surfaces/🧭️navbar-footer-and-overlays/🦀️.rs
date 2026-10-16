//! 🫧️ `shell-surfaces` staging of `navbar-footer-and-overlays`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl SurfaceState {
    /// ⏳️🖼️ Each live World3d surface with the compute-status document it published — the one input
    /// both the status pill and the cancel control read.
    pub(super) fn world3d_status_rows(&self) -> Vec<(&str, Rect, Option<&str>)> {
        self.world3d_states.iter().map(|(surface_id, world)| (surface_id.as_str(), world.bounds, self.world3d_status.get(surface_id).map(String::as_str))).collect()
    }
}

//#endregion ShellChrome

