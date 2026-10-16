//! 🔲️ `shell-windows` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl WindowState {
    /// 🪪️ Mounts `instance_id` of `app` as this shell's session: the retained Home hands back its view
    /// state first, then the session and its first window become current. The synchronous tail of
    /// [`Self::switch_to_app`], shared with the frame-pumped open ([`Self::advance_document_opening`]),
    /// which renders on the settle lane instead of inline.
    pub(super) fn install_app_session(&mut self, plugin_id: &str, app: AppDefinition, instance_id: u32, prefs: &PrefsState, directory: &mut DirectoryState, docs: &mut DocumentState) {
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (directory.directory_home.as_mut(), docs.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: None,
            extension_input_json: None,
            session_identity: None,
            locale: prefs.active_locale(),
            terminology: prefs.active_terminology(),
            window_id: None,
            focused_window_id: Some(app.window_kinds.first().id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        docs.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });
    }
}

//#endregion ShellActions

