//! 🪝️ `shell-window-actions` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl<'ctx> WindowActionsCtx<'ctx> {
    //#region 🔖️DirectoryAndIdentity
    //#region 💡️InferencePort
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_service_port(&mut self, owner:&str,service_id:&str,action:&str,payload:DslValue) {
        if self.docs.session.as_ref().map(|session|session.plugin_id.as_str())!=Some(owner) {return;}
        if let Some(runner)=self.directory.inference_port.as_ref().filter(|runner|runner.owner==owner && runner.service_id==service_id) {
            runner.intend(action,payload);
            return;
        }
        self.directory.cancel_inference_port();
        let Some(lease)=self.opening.document_execution_target_lease.as_ref() else {return};
        if lease.package.plugin_id!=owner {return;}
        let Some(space_id)=self.opening.open_space_id.clone() else {return};
        let Some(document_id)=self.sync.sync_channel.as_ref().map(|channel|channel.document_id.clone()) else {return};
        let scope=DocumentScope {space_id,document_id};
        if lease.scope!=scope {return;}
        let mut matches=self.opening.document_services.iter().filter(|entry|entry.owner==owner && entry.service_id==service_id);
        let Some(contribution)=matches.next().copied() else {return};
        if matches.next().is_some() {return;}
        let Some(port)=self.plugins.installed_document_http_port(owner,service_id) else {return};
        let Some(client)=self.directory.directory_client.clone().filter(|_|self.directory.identity.is_some()) else {return};
        let mut driver=(contribution.create)(scope.clone(),true);
        if driver.intend(action,payload).is_err() {return;}
        self.directory.inference_port_status=Some(InstalledServiceStatusV1 {owner:owner.into(),service_id:service_id.into(),payload:driver.status()});
        self.directory.inference_port=Some(ShellInferenceRunner::start(semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool(),client,std::sync::Arc::new(port),self.directory.directory_ctx(),contribution,scope,driver));
    }

    //#endregion 💡️InferencePort
    //#endregion 🔖️DirectoryAndIdentity
    /// 🔁️ Steps the active mode one place along the app's declared order, wrapping — what the
    /// `mod+alt+→`/`mod+alt+←` chords fire. Reuses the very same body as clicking a
    /// `playground.navbar.modes.<id>` button, so keyboard and pointer cannot diverge.
    pub async fn apply_mode_step(&mut self, step: i32) -> Result<(), String> {
        let Some(session) = self.docs.session.as_ref() else {
            return Ok(());
        };
        let mode_ids: Vec<String> = session.app.modes.iter().map(|mode| mode.id.clone()).collect();
        let Some(next) = step_mode_id(&mode_ids, session.view_state.active_mode_id.as_deref(), step) else {
            return Ok(());
        };
        self.apply_navbar_mode(&next).await
    }

    /// 🎛️ The ONE body behind a mode change, whether a navbar button or a chord asked for it.
    pub async fn apply_navbar_mode(&mut self, mode_id: &str) -> Result<(), String> {
        if let Some(session) = self.docs.session.as_mut() {
            session.view_state.active_mode_id = Some(mode_id.to_string());
            if let Some(layout) = semio_framework::resolve_layout_for_mode(&session.app, mode_id) {
                self.dock.layout_override = Some(layout);
                self.as_session().sync_dock();
                self.windows.active_window_id = self.dock.dock.active_window_id.clone();
            }
        }
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 📇️ ticket §6 — generic app switch by definition (not the host's landing/host app via
    /// `host_config()`), used for the `s.space` artifact-index route below. Kept as a close sibling of
    /// `switch_to_managed_app` (CLAUDE.md: repeated code stays close together) rather than a forced
    /// shared abstraction that would blur the two call shapes (this one has no workflow-panel state).
    /// Both targets — the lazy install it opens with is [`Self::install_plugin`]; only the retained
    /// Home hand-back below is native, because `directory_home` is a native-only projection.
    pub async fn switch_to_app(&mut self, plugin_id: &str, app: AppDefinition) -> Result<(), String> {
        self.plugins.install_plugin(plugin_id).await?;
        let program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned().ok_or("program missing")?;
        if let Some(session) = &self.docs.session {
            if session.plugin_id == plugin_id && session.app.id == app.id {
                return Ok(());
            }
        }
        let instance_id = program.create_app(&app.id).await.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome))?;
        self.windows.install_app_session(plugin_id, app, instance_id, &self.prefs, &mut self.directory, &mut self.docs);
        self.refresh_ui(UiDirtyScope::Full).await
    }
}

//#endregion ShellActions

