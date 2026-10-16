//! 📄️ `shell-docs` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl DocumentState {
    //#region 🏷️LabelResolution
    pub fn chrome_dialog_request(&self, dialog_id: &str, args: Option<DslValue>, prefs: &PrefsState) -> Option<ChromeDialogRequest> {
        let session = self.session.as_ref()?;
        let dialog = session.app.dialogs.iter().find(|dialog| dialog.id == dialog_id)?;
        Some(ChromeDialogRequest::from_definition(&session.app.controller_id, dialog, args, prefs.active_terminology(), prefs.active_locale()))
    }

    //#endregion 🏷️LabelResolution
    //#region 🏠️🧳️PluginHostConfig
    /// 🕒️ Who a `noteShellCommand` is addressed to: the LIVE session's app first, and only then the
    /// host program's own app — the same order `document_controller_id` resolves a document's action
    /// bindings in, and the one React uses (`noteShellCommand` closes over the mounted app).
    ///
    /// 🩸️ The command journal used to ask [`Self::host_controller_id`] alone. A play serve mounts NO
    /// host app (there is no space/studio around the plugin), so `host_config()` is `None`, the
    /// controller resolved to nothing and every window-cap and split-resize command returned before it
    /// dispatched: the dock's close, focus and resize journalled NOTHING while the panel toggles right
    /// beside them journalled fine, because those had always read the session (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` steps 15–17).
    pub fn shell_command_controller_id(&self, plugins: &PluginState) -> Option<String> {
        self.session.as_ref().map(|session| session.app.controller_id.clone()).or_else(|| plugins.host_controller_id())
    }

    /// 🎬️ The controller every retained document's own action bindings dispatch to: the live
    /// session's app, falling back to the host program's own app when no session is mounted (a
    /// framework panel painted before the guest opened).
    pub fn document_controller_id(&self, plugins: &PluginState) -> String {
        self.session.as_ref().map(|session| session.app.controller_id.clone()).or_else(|| plugins.host_controller_id()).unwrap_or_default()
    }

    /// 🪟️ The session whose exact retained document occupies this dock body.
    pub fn window_document_session(&self, window_id: &str, plugins: &PluginState) -> Option<&ActiveSession> {
        if window_id == "spawned" { self.spawned_session.as_ref().filter(|_| plugins.space_mode) } else { self.session.as_ref() }
    }

    /// 🎯️ The app instance addressed by a retained action's controller and surface.
    pub fn action_session(&self, action: &ActionDescriptor, plugins: &PluginState) -> Option<&ActiveSession> {
        let requested = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str);
        let spawned = self.spawned_session.as_ref().filter(|_| plugins.space_mode);
        if requested == Some("spawned") || spawned.is_some_and(|session| requested.is_some_and(|id| session.view_state.window_instances.iter().any(|window| window.id == id))) {
            return spawned.filter(|session| session.app.controller_id == action.controller_id);
        }
        if let Some(spawned) = spawned.filter(|session| session.app.controller_id == action.controller_id && self.session.as_ref().is_none_or(|host| host.app.controller_id != action.controller_id)) {
            return Some(spawned);
        }
        self.session.as_ref().filter(|session| session.app.controller_id == action.controller_id)
    }

    /// 📨️ Commands select the declared plugin and app among the mounted sessions.
    pub fn command_session(&self, owner: &semio_framework::manifest::CommandOwnerAddress, window_id: Option<&str>, plugins: &PluginState) -> Option<&ActiveSession> {
        use semio_framework::manifest::CommandOwnerAddress;
        let matches = |session: &&ActiveSession| match owner {
            CommandOwnerAddress::Os => false,
            CommandOwnerAddress::Plugin { plugin_id } => &session.plugin_id == plugin_id,
            CommandOwnerAddress::App { plugin_id, app_id } | CommandOwnerAddress::Mode { plugin_id, app_id, .. } => &session.plugin_id == plugin_id && &session.app.id == app_id,
        };
        let spawned = self.spawned_session.as_ref().filter(|_| plugins.space_mode).filter(matches);
        if let Some(session) = spawned.filter(|session| window_id.is_some_and(|id| id == "spawned" || session.view_state.window_instances.iter().any(|window| window.id == id))) {
            return Some(session);
        }
        self.session.as_ref().filter(matches).or(spawned)
    }

    //#endregion 🏠️🧳️PluginHostConfig
    pub fn prepare_hot_reload(&mut self, plugins: Vec<ProgramBridgeEntry>, plugins_state: &mut PluginState, directory: &mut DirectoryState) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let retained_home = directory.directory_home.as_ref().map(|home| (home.plugin_id.clone(), home.instance_id));
            if let Some(mut home) = directory.directory_home.take() {
                if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                    if let Some(program) = plugins_state.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                        program.destroy_app(instance_id);
                    }
                }
            }
            if let Some(session) = self.session.take() {
                if retained_home.as_ref().is_none_or(|(plugin_id, instance_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id) {
                    if let Some(program) = plugins_state.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                        program.destroy_app(session.instance_id);
                    }
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(session) = self.session.take() {
            if let Some(program) = plugins_state.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                program.destroy_app(session.instance_id);
            }
        }
        plugins_state.plugins = plugins;
    }

    /// 📚️ The examples the OPEN surface offers, resolved by DIALECT through the one shared predicate
    /// (`manifest::examples_for_app`, twin of React's `examplesForApp`) rather than by plugin id: an
    /// example is a document of the artifact's subset, so a plugin that publishes several dialects
    /// must never offer another artifact's fixtures here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn session_example_rows(&self, plugins: &PluginState, prefs: &PrefsState) -> Vec<ShellExampleRow> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        let Some(plugin) = plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) else {
            return Vec::new();
        };
        shell_example_rows(&plugin.manifest.examples, &session.app, plugins.active_example_id.as_deref(), prefs.active_terminology(), prefs.active_locale())
    }

    /// 🧰️ The scope a host-effect pass owes ON TOP of what its dispatch declared — the Rust twin of
    /// `🛠️ShellHelpers/🟦️.tsx`'s `hostEffectRefreshScopeV1`, and unioned with the guest's own scope,
    /// never substituted for it.
    ///
    /// `SetPanel`/`LoadDocument` name no body of their own and the panel payload feeds every section,
    /// so the only honest answer for those is the widest one. `SetActiveUtility`/`SetActiveTool`
    /// rewrite a render input the guest reads back, so they earn every window body plus the
    /// host-derived sections — the defect measured on the puzzle 3d editor, where an armed utility's
    /// `UiDirtyScope::None` meant the armed body was never fetched at all
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37).
    pub fn host_effect_earned_scope(&self, effects: &[semio_framework::kernel::Effect]) -> UiDirtyScope {
        use semio_framework::kernel::Effect;
        if effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return UiDirtyScope::Full;
        }
        if !effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { .. })) {
            return UiDirtyScope::None;
        }
        let Some(session) = self.session.as_ref() else { return UiDirtyScope::Full };
        UiDirtyScope::Partial { window_bodies: session.app.window_kinds.iter().map(|kind| kind.body_key.clone()).collect(), panel_bodies: Vec::new(), utilities: true, tools: true, engagements: false, measures: true, labels: false }
    }
}

//#endregion ShellLifecycle

