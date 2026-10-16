//! 🧲️ `shell-plugins` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl PluginState {
    //#region 🏠️🧳️PluginHostConfig
    /// 🏠️🧳️ This filter's host config (landing/host app-id roles), or `None` when it doesn't offer a
    /// host-style multi-app experience — see `program_bridge::PluginHostConfig`.
    pub fn host_config(&self) -> Option<&'static PluginHostConfig> {
        self.host_capability
    }

    /// 🏠️🧳️ The host plugin's own host-role app, self-declaring its `controller_id`/`panel_tabs` — the
    /// generic source of truth for what were previously separate `S_PLAY_CONTROLLER_ID`/
    /// `S_PLAY_CATALOGUE_TAB_ID` literals.
    pub fn host_app(&self) -> Option<&AppDefinition> {
        let cfg = self.host_config()?;
        let program = self.plugins.iter().find(|p| p.plugin_id == cfg.plugin_id)?;
        program.manifest.apps.iter().find(|app| app.id == cfg.host_app_id)
    }

    pub fn host_controller_id(&self) -> Option<String> {
        self.host_app().map(|app| app.controller_id.clone())
    }

    pub fn host_catalogue_tab_id(&self) -> Option<String> {
        self.host_app().and_then(|app| flatten_panel_tab_leaves(&app.panel_tabs).first().map(|tab| tab.id().to_string()))
    }
    //#endregion 🏠️🧳️PluginHostConfig
}

//#endregion ShellLifecycle

pub fn flatten_panel_tab_leaves(tabs: &[PanelTabDefinition]) -> Vec<&PanelTabDefinition> {
    tabs.iter().flat_map(|tab| if tab.children.is_empty() { vec![tab] } else { flatten_panel_tab_leaves(&tab.children) }).collect()
}
