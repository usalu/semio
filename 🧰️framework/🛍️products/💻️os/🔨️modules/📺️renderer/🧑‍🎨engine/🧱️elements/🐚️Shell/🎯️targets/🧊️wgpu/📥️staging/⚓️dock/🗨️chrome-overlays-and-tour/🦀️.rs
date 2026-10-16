//! ⚓️ `shell-dock` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
impl ShellDockState {
    /// 🎬️ One bounded agent-bridge turn, folded into the same 100 ms `PumpSync` slot the directory
    /// lane already rides. Answers whether the chrome must repaint.
    ///
    /// 🧾️ `principal_actor` is React's own value for a shell with no signed-in identity
    /// (`"agent:unknown"`); a signed-in shell announces its hub user id, which is the `{userId}` half
    /// of the same actor grammar `shell_actor` already uses.
    pub fn pump_agent_bridge(&mut self, chrome: &mut ChromeState, directory: &mut DirectoryState, windows: &mut WindowState) -> bool {
        let mut changed = false;
        if let Some(config) = take_pending_agent_bridge_config() {
            changed = directory.set_agent_bridge_config(config, chrome);
        }
        if !directory.agent_bridge.is_armed() {
            return changed;
        }
        let session = directory.shell_session_id.clone();
        let principal = directory.identity.as_ref().map(|identity| identity.user_id.clone()).unwrap_or_else(|| "agent:unknown".to_string());
        let now = chrome_now_ms();
        changed |= directory.agent_bridge.pump(&mut chrome.chrome_build.agent, &session, &principal, now);
        changed | self.apply_inbound_agent_shell_commands(chrome, windows)
    }

    /// 🤖️ Carries out the `ui_focus`/`ui_reveal` the connected agent sent, on THIS shell's real
    /// chrome, and only then acknowledges them. Answering the gateway before the dock moved is the
    /// exact defect slice M7 measured on the React host, where the bridge reduced against a private
    /// mirror and reported `ok` while nothing on screen changed.
    pub(super) fn apply_inbound_agent_shell_commands(&mut self, chrome: &mut ChromeState, windows: &mut WindowState) -> bool {
        let mut changed = false;
        for inbound in chrome.chrome_build.agent.take_inbound_shell_commands() {
            let seq = inbound.seq();
            let (ok, fault) = match inbound {
                crate::agent_bridge::InboundShellCommand::FocusWindow { window_id, .. } => {
                    windows.active_window_id = window_id;
                    changed = true;
                    (true, None)
                }
                crate::agent_bridge::InboundShellCommand::RevealPanelTab { tab_id, .. } => match self.reveal_dock_tab(&tab_id) {
                    Some(_) => {
                        changed = true;
                        (true, None)
                    }
                    None => (false, Some(format!("this shell's dock hosts no `{tab_id}` panel tab"))),
                },
                crate::agent_bridge::InboundShellCommand::Acknowledge { .. } => (true, None),
            };
            chrome.chrome_build.agent.settle_shell_command(seq, ok, fault);
        }
        changed
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

