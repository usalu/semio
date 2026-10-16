//! 📄️ `shell-docs` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// 🎓️ Matches React's first-frame arming predicate with an independently owned host policy.
pub fn should_auto_start_introduction(seen_key: &str, has_introduction: bool, tutorial_active: bool, seen: bool, replay_on_load: bool, suppressed: bool) -> bool {
    !seen_key.is_empty() && has_introduction && !tutorial_active && !suppressed && (replay_on_load || !seen)
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

