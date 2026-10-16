//! 🎚️ `shell-chrome-state` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ChromeState {
    /// 📨️ Coalesces the persisted chrome load without touching platform storage in the frame step.
    pub(super) fn request_chrome_preferences_load(&mut self) {
        if !self.chrome_present.preferences_loaded {
            self.chrome_present.maintenance.load_requested = true;
        }
    }

    /// 🎨️ Exposes the already-loaded theme id without synchronously opening preference storage.
    pub(crate) fn active_theme_id_for_frame(&self) -> &str {
        &self.chrome_build.preferences.theme_id
    }
}

//#endregion ShellChrome

