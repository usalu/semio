//! 🎚️ `shell-chrome-state` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl ShellChromeBuildState {
    pub fn content_has_focus(&self, window_id: &str) -> bool {
        self.content_focus_node(window_id).is_some()
    }

    /// 🎯️ WHICH retained node holds focus in `window_id` — React's `surfaceActiveRoot`. A chrome verb
    /// that must address the focused content node (a panel-level Copy, an inspector commit) reads this
    /// instead of re-deriving focus from the tree.
    fn content_focus_node(&self, window_id: &str) -> Option<semio_framework_ui_arena::NodeId> {
        self.content_focus.get(window_id).and_then(Option::as_ref).map(|focus| focus.node)
    }

    pub fn note_content_focus_commands(&mut self, commands: &[semio_framework_ui_wgpu_events::UiCommand]) {
        for command in commands {
            if let semio_framework_ui_wgpu_events::UiCommand::FocusChanged { window_id, node } = command {
                debug_log(&format!("[TRACE] wgpu-shell content focus window={window_id} node={node:?}"));
                let focus = node.and_then(|node| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_node_focus_identity(window_id, node).map(|(key, kind)| RetainedContentFocus { node, key, kind }));
                if focus.is_some() {
                    if let Some(previous) = self.focused_retained_surface.replace(window_id.clone()).filter(|previous| previous != window_id) {
                        self.content_focus.insert(previous, None);
                    }
                } else if self.focused_retained_surface.as_deref() == Some(window_id.as_str()) {
                    self.focused_retained_surface = None;
                }
                self.content_focus.insert(window_id.clone(), focus);
            }
        }
    }

    pub fn clear_content_focus(&mut self, surface: &str) {
        self.content_focus.remove(surface);
        if self.focused_retained_surface.as_deref() == Some(surface) {
            self.focused_retained_surface = None;
        }
    }

    pub fn clear_sibling_pane_focus(&mut self, surface: &str) {
        let owner = window_pane_owner(surface).unwrap_or(surface);
        for sibling in [owner.to_string(), window_measures_surface_id(owner), window_actions_surface_id(owner), window_search_surface_id(owner), window_projection_surface_id(owner)] {
            if sibling != surface {
                self.clear_content_focus(&sibling);
            }
        }
    }
}

//#endregion ShellInput

