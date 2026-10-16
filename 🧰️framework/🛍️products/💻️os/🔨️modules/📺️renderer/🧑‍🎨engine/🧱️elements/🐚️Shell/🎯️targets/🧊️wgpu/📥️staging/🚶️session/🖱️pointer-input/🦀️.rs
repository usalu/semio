//! 🚶️ `shell-session` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> SessionCtx<'ctx> {
    pub fn retained_keyboard_focus(&self) -> Option<(String, ui_wgpu::wgpu::NodeId)> {
        let surface = self.chrome.chrome_build.focused_retained_surface.as_ref()?;
        let owner = window_pane_owner(surface).unwrap_or(surface);
        if !self.retained_surface_is_panel(surface) && self.windows.active_window_id.as_deref() != Some(owner) {
            return None;
        }
        let focus = self.chrome.chrome_build.content_focus.get(surface)?.as_ref()?;
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_node_has_visible_focus(surface, focus.node, &focus.key, focus.kind).then(|| (surface.clone(), focus.node))
    }

    pub fn retained_select_owns_keyboard(&self) -> bool {
        self.retained_keyboard_focus().is_some_and(|(surface, node)| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_node_is_select(&surface, node))
    }
}

impl<'ctx> SessionCtx<'ctx> {
    //#region 🎯️RetainedPointerRouting
    /// 🧭️ A retained panel tab owns pointer routing but is not an application window instance.
    /// 🧭️ Whether a retained surface is shell chrome rather than a plugin window: a panel open in an
    /// anchor, or any shell-owned leaf wherever it is shown (the `/hub` workspace overlay is the
    /// `framework.hub` leaf with no anchor). Focusing or pressing chrome never makes it the active
    /// window — every plugin action resolves its window kind from `active_window_id`, and a chrome id
    /// there has none, which faulted the frame.
    pub fn retained_surface_is_panel(&self, surface_id: &str) -> bool {
        self.dock.open_anchors().into_iter().any(|anchor| self.dock.anchor_state(anchor).active_tab() == Some(surface_id)) || self.docs.shell_owned_panel_leaves(&self.chrome, &self.plugins, &self.prefs).iter().any(|leaf| leaf == surface_id)
    }

    /// 🪪️ Refreshes bounded app ownership independently from authored media resources.
    pub async fn refresh_app_document_identity(&mut self, program: &ProgramBridgeEntry, instance_id: u32) -> Result<(), String> {
        let key = (program.plugin_id.clone(), instance_id);
        let spawned = self.docs.session.as_ref().and_then(|session| panel_state_from_view(&session.view_state).ok().flatten()).map(|panel| panel.spawned_apps).unwrap_or_default();
        let active = self.docs.session.as_ref().map(|session| (session.plugin_id.clone(), session.instance_id));
        let mounted = self.docs.spawned_session.as_ref().map(|session| (session.plugin_id.clone(), session.instance_id));
        self.docs.app_document_identities.retain(|owner| Some(owner) == active.as_ref() || Some(owner) == mounted.as_ref() || *owner == key || spawned.iter().any(|app| app.plugin_id == owner.0 && app.instance_id == owner.1));
        let request = self.docs.app_document_identities.begin(key.clone()).ok_or_else(|| "app document identity registry request authority unavailable".to_string())?;
        let identity = match program.read_app_document_identity(instance_id).await {
            Ok(identity) => identity,
            Err(error) => {
                self.docs.app_document_identities.remove(&key);
                return Err(self.prefs.note_refused_open(error, &mut self.chrome));
            }
        };
        if !self.docs.app_document_identities.accept(&key, request, identity.app_instance_id, identity.parent_document_id) {
            return Err("app document identity reply is stale or foreign".into());
        }
        Ok(())
    }
    //#endregion 🎯️RetainedPointerRouting
}

//#endregion ShellInput

