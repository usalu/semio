//! 🎚️ `shell-chrome-state` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ChromeState {
    /// ✅️ Applies one click on an approvals-modal control: a decision button resolves that request
    /// over the bridge, the close control dismisses the dialog without deciding (the request stays
    /// parked, exactly as React's `onOpenChange` leaves it).
    pub fn resolve_agent_approval_control(&mut self, control_id: &str) {
        if control_id == semio_framework_os_renderer_wgpu_agent_elements::agent_approvals::APPROVALS_CLOSE_CONTROL_ID {
            self.chrome_build.agent_approvals.dismiss();
            return;
        }
        if let Some((approval_id, decision)) = semio_framework_os_renderer_wgpu_agent_elements::agent_approvals::parse_approval_decision_control_id(control_id) {
            self.chrome_build.agent.resolve_approval(&approval_id, decision, None);
        }
    }
}

//#endregion ShellChrome

