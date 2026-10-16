//! 📒️ `shell-directory` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl DirectoryState {
    /// 🛂️ Applies one click on an administration control. The two affordances the CHROME owns land
    /// here; every mutating control (`role`, `remove`, `issue`, `revoke`) still has to seal a
    /// `DirectoryCommand`, which is the operation's own funnel, and is left to the funnel rather than
    /// re-derived here — see the packet report's hand-offs.
    ///
    /// 📄️ The two paging controls carry their own cursor as the control's VALUE, so the page they
    /// advance is exactly the window the hub issued it for.
    pub fn resolve_space_administration_control(&mut self, control_id: &str, prefs: &PrefsState) {
                use semio_framework_os_renderer_wgpu_hub_workspace::space_administration::SPACE_ADMINISTRATION_CLOSE_CONTROL_ID;
        if control_id == SPACE_ADMINISTRATION_CLOSE_CONTROL_ID {
            self.close_space_administration();
            return;
        }
        if control_id == "os.space-administration.members.more" || control_id == "os.space-administration.invites.more" {
            let german = prefs.locale_id == "de";
            let cursor =
                self.space_administration.as_ref().map(|operation| shell_space_administration_controls(operation, german)).and_then(|controls| controls.into_iter().find(|control| control.control_id == control_id).map(|control| control.value));
            if let (Some(cursor), Some(operation)) = (cursor.filter(|cursor| !cursor.is_empty()), self.space_administration.as_mut()) {
                operation.request_page(Some(cursor));
            }
            return;
        }
    }
}

//#endregion ShellChrome

