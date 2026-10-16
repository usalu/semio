//! ⚓️ `shell-dock` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ShellDockState {
    pub(crate) fn document_body_key(&self, surface_id: &str, docs: &DocumentState) -> Option<String> {
        let session = docs.session.as_ref()?;
        let window_kind_id = self.live_window_kind_id(session, surface_id);
        session
            .app
            .window_kinds
            .iter()
            .find(|kind| Some(kind.id.as_str()) == window_kind_id)
            .map(|kind| kind.body_key.clone())
            .or_else(|| flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().find(|tab| tab.id() == surface_id).and_then(|tab| tab.body_key.clone()))
    }

    /// 🖼️ Applies React's renderer-local `SetWindowIconContext` command only to an exact live dock
    /// instance. The owner token prevents a late command from one app instance mutating its successor,
    /// while the shared view-context capacity bounds both entry count and identifier size.
    pub(crate) fn apply_window_icon_host_command(&mut self, window_id: &str, icon_id: &str, docs: &DocumentState, windows: &mut WindowState) -> bool {
        let Some(session) = docs.session.as_ref() else { return false };
        if self.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id)
            || self.dock.window_kind_id(window_id).is_none()
            || icon_id.is_empty()
            || icon_id.chars().count() > semio_framework::VIEW_CONTEXT_IDENTIFIER_CHARS
            || (!windows.window_icon_overrides.contains_key(window_id) && windows.window_icon_overrides.len() >= semio_framework::VIEW_CONTEXT_WINDOW_INSTANCES)
        {
            return false;
        }
        windows.window_icon_overrides.insert(window_id.to_string(), WindowIconOverride { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), app_instance_id: session.instance_id, icon_id: icon_id.to_string() });
        true
    }

    /// 🏷️ Applies an explicit title to one live pane within its current session owner.
    pub(crate) fn apply_window_title_host_command(&mut self, window_id: &str, title: &str, docs: &DocumentState, windows: &mut WindowState) -> bool {
        let Some(session) = docs.session.as_ref() else { return false };
        if self.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id)
            || self.dock.window_kind_id(window_id).is_none()
            || (!windows.window_title_overrides.contains_key(window_id) && windows.window_title_overrides.len() >= semio_framework::VIEW_CONTEXT_WINDOW_INSTANCES)
        {
            return false;
        }
        windows.window_title_overrides.insert(window_id.to_string(), WindowTitleOverride { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), app_instance_id: session.instance_id, title: title.to_string() });
        true
    }

    pub(crate) fn live_window_kind_id<'a>(&'a self, session: &'a ActiveSession, window_id: &str) -> Option<&'a str> {
        self.dock.window_kind_id(window_id).or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_id).map(|kind| kind.id.as_str()))
    }

    pub(crate) fn persist_dock_layout(&mut self) {
        self.layout_override = Some(self.dock.to_window_layout());
    }

    pub(crate) fn allocate_window_instance_id(&self, window_kind_id: &str) -> String {
        let taken: Vec<String> = self.dock.window_instances().into_iter().map(|(id, _)| id).collect();
        (2..).map(|index| format!("{window_kind_id}-{index}")).find(|candidate| !taken.contains(candidate)).expect("an unbounded counter always clears a finite dock roster")
    }

    /// 🌫️ React's `deactivateActiveWindow` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1455-1472`), reached by
    /// pressing the canvas background or a resize gutter: the mode keeps its layout and simply has no
    /// focused window any more.
    pub(crate) fn deactivate_active_window(&mut self, windows: &mut WindowState) {
        if !self.dock.deactivate_active_window() && windows.active_window_id.is_none() {
            return;
        }
        windows.active_window_id = None;
    }
}

//#endregion ShellLifecycle

