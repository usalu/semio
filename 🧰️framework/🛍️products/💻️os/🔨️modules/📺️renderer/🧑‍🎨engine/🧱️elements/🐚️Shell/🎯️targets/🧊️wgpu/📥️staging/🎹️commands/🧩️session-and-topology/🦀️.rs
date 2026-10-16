//! 🎹️ `shell-commands` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl CommandState {
    //#region 🏷️LabelResolution
    pub fn set_search_open(&mut self, open: bool, input: &mut ShellInputState) {
        self.search_open = open;
        if open {
            self.find_open = false;
            input.overlay_state = OverlayState::Search;
        } else if input.overlay_state == OverlayState::Search {
            input.overlay_state = OverlayState::None;
        }
    }
    //#endregion 🏷️LabelResolution
}

//#endregion ShellLifecycle

