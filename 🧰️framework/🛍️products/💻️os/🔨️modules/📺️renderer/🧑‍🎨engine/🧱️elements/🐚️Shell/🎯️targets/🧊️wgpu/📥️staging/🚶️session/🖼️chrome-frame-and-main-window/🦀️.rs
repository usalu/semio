//! 🚶️ `shell-session` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> SessionCtx<'ctx> {
    pub fn body_rect(&self, theme: &Theme) -> Rect {
        let top = theme.navbar_height + self.tutorial.tutorial_bar_reserve(theme);
        Rect::new(0.0, top, self.chrome.screen_w, self.chrome.screen_h - top - theme.footer_height - self.subfooter_height(theme))
    }
}

//#endregion ShellChrome

