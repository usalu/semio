//! 📒️ `shell-directory` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
impl DirectoryState {
    /// 🔗️ Installs (or clears) the gateway config the local supervisor hands over — packet W15e.
    /// This is the ONLY way a bridge socket is ever dialled: `discover_agent_bridge_config()` answers
    /// `None` on both targets, deliberately, exactly as React's does, so a shell nobody supervises
    /// stays `Disabled` instead of guessing a url.
    pub fn set_agent_bridge_config(&mut self, config: Option<semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentBridgeConfig>, chrome: &mut ChromeState) -> bool {
        self.agent_bridge.set_config(config, &mut chrome.chrome_build.agent)
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

