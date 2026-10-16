//! 📂️ `shell-doc-open` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl OpeningState {
    pub fn shell_uri(&self, docs: &DocumentState) -> String {
        self.uri_history.get(self.uri_index).cloned().unwrap_or_else(|| docs.session.as_ref().map(|s| format!("os://{}/{}", s.plugin_id, s.app.id)).unwrap_or_else(|| "os://home".into()))
    }
}

//#endregion ShellChrome

