//! 🎚️ `shell-chrome-state` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ChromeState {
    /// 🛑️ Sends one cooperative cancellation for the gateway-owned invocation id and leaves the
    /// row in `cancelling` until its real result arrives. A disconnected shell changes nothing.
    pub(crate) fn cancel_agent_tool_call(&mut self, invocation_id: &str) -> bool {
        self.chrome_build.agent.cancel_tool_call(invocation_id)
    }

    //#region 🧭️PanelAnchorAccessors
    /// 📱️ Below React's `UI_MOBILE_MEDIA_QUERY` breakpoint (`max-width: 767px`, the same
    /// `dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX` the window dock reads) the eight anchors do not paint at
    /// all: one mobile panel carries every tab instead. Read off the LOGICAL viewport width W1g
    /// established, never the physical one.
    pub fn mobile_panel_active(&self) -> bool {
        self.screen_w <= crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX
    }
    //#endregion 🧭️PanelAnchorAccessors
}

//#endregion ShellLifecycle

