//! ⚓️ `shell-dock` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ShellDockState {
    /// 🎓️ Opens+selects `tab_id`'s panel side, mirroring `select_left_panel_tab`/the `shell.panel.tab.right.*`
    /// hit handler exactly (including their differing `setActivePanelTab` dispatch conditions) so the
    /// tour's programmatic reveal behaves identically to the user clicking the tab themselves.
    pub(crate) fn chrome_tour_reveal_panel_tab(&mut self, session: &ActiveSession, tab_id: &str, settle: &mut SettleState, plugins: &PluginState) {
        let is_app_leaf = flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().any(|tab| tab.id() == tab_id);
        if plugins.host_config().is_some() && is_app_leaf {
            settle.deferred_actions.push(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) });
            return;
        }
        self.reveal_dock_tab(tab_id);
    }
}

//#endregion ShellChrome

