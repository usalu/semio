//! 🔲️ `shell-windows` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl WindowState {
    /// 🛠️ One `tool.<id>` leaf's body — the wgpu twin of React's `buildToolTree`: a single headerless
    /// options section over the tool's own published measures. The leaf tab IS the activation control
    /// (see {@link ShellState::arm_tool_for_panel_tab}), so this renders no second activation toggle.
    /// The wgpu program bridge has no `tool_measures` reader yet, so the section carries the tool's
    /// armed state and stays otherwise empty exactly as React's does with no measures in hand.
    pub fn build_tool_panel_ui(&self, tool_id: &str, prefs: &PrefsState, docs: &DocumentState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let armed = docs.session.as_ref().and_then(|session| session.view_state.active_tool_id.as_deref()) == Some(tool_id);
        let items = self.tool_measures.get(tool_id).map(|measures| window_measure_tree_rows(measures)).unwrap_or_default();
        display_panel_body(
            &format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{tool_id}.panel"),
            vec![UiTreeSectionNode {
                header_toolbar: None,
                id: format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{tool_id}.options"),
                label: Some(Label::data(shell_chrome_string(if armed { "tool.armed" } else { "tool.idle" }, is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window: None,
            }],
        )
    }
}

//#endregion ShellLifecycle

