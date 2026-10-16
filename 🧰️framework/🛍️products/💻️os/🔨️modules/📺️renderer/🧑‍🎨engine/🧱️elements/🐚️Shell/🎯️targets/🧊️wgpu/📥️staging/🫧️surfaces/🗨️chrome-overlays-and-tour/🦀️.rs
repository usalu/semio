//! 🫧️ `shell-surfaces` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
impl SurfaceState {
    /// ⏰️ Translates the next epoch-based Shell chrome deadline onto the process monotonic clock.
    pub fn next_chrome_deadline(&self, monotonic_now_seconds: f64, wall_now_ms: f64, chrome_state: &ChromeState, tutorial_state: &TutorialState) -> Option<f64> {
        if !monotonic_now_seconds.is_finite() || !wall_now_ms.is_finite() {
            return None;
        }
        let chrome = chrome_state.chrome_build.next_deadline_ms(wall_now_ms);
        let tutorial = tutorial_state.next_tutorial_deadline_ms(wall_now_ms);
        match (chrome, tutorial) {
            (Some(chrome), Some(tutorial)) => Some(chrome.min(tutorial)),
            (Some(deadline), None) | (None, Some(deadline)) => Some(deadline),
            (None, None) => None,
        }
        .map(|deadline_ms| monotonic_now_seconds + ((deadline_ms - wall_now_ms).max(0.0) / 1000.0))
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

