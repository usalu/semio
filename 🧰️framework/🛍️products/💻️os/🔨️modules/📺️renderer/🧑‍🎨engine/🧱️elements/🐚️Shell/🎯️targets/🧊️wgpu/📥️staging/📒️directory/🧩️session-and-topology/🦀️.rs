//! 📒️ `shell-directory` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl DirectoryState {
    /// 🪪️ The session identity a `ViewModel` carries. ONE accessor on both targets since ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1e gave the browser build a real identity of its
    /// own (`poll_browser_identity`); it used to be a cfg pair whose browser half answered a hard
    /// `None`, so every guest saw an anonymous session on the browser renderer alone.
    pub fn session_identity_view(&self) -> Option<ViewSessionIdentity> {
        self.identity.as_ref().map(|identity| ViewSessionIdentity { user_id: identity.user_id.clone(), display_name: identity.display_name.clone() })
    }
}

//#endregion ShellLifecycle

