//! 📂️ `shell-doc-open` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl OpeningState {
    //#region 🔖️DirectoryAndIdentity
    //#region 💡️InferencePort
    /// 📦 Replaces the executable's finite installed owner inventory and retires prior work.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn install_document_services(&mut self, services: Vec<InstalledServiceContributionV1>, directory: &mut DirectoryState) {
        directory.cancel_inference_port();
        if services.len()>64 || services.iter().enumerate().any(|(index,entry)|services[..index].iter().any(|prior|prior.owner==entry.owner && prior.service_id==entry.service_id)) {self.document_services.clear();return;}
        self.document_services=services;
    }

    //#endregion 💡️InferencePort
    /// 🧩️ Starts a hub document's open by resolving the component it runs by the SERVING catalog
    /// generation (binding decision of ticket 26/09/23, session 11 13:1x; React resolves the same generation
    /// through the hub's plugin-module routes): the local program only when its content hash is the lease's,
    /// else the verified store entry, else the hub's own execution-target bytes, verified and stored
    /// ([`DirectoryClient::resolve_execution_target_module`]) — all detached, with a cancellable band.
    /// The lease names the plugin and app that open the document, so a relay that names none (the creation
    /// door's, a Space index row's) resolves exactly like one that does. Answers `false` for a relay that names
    /// no hub document, which opens as before.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn begin_document_resolution(&mut self, target: &OpenArtifactRelayTarget, plugins: &PluginState, prefs: &PrefsState, directory: &DirectoryState) -> bool {
        let (Some(document_id), Some(schema), Some(space_id), Some(client)) = (target.document_id.clone(), target.schema.clone(), self.open_space_id.clone(), directory.directory_client.clone()) else {
            return false;
        };
        let local_app = target.app_id.as_ref().and_then(|app_id| plugins.plugins.iter().flat_map(|program| program.manifest.apps.iter()).find(|app| app.id == *app_id).cloned());
        let surface_id = local_app.as_ref().map_or_else(|| semio_framework::manifest::surface_app_id(&target.dialect, target.role), |app| semio_framework::manifest::surface_app_id(&app.dialect, app.role));
        let label = local_app.map_or_else(|| target.app_id.clone().unwrap_or_else(|| target.artifact_ref.clone()), |app| app.label.resolve(prefs.active_terminology(), prefs.active_locale()).to_string());
        let local_shas: Vec<(String, Option<String>)> = plugins.plugins.iter().map(|program| (program.plugin_id.clone(), program.component_sha256.clone())).collect();
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id.as_str()),
            requested_surface_id: Some(surface_id),
            client_instance_id: format!("wgpu-shell-{}", directory.shell_session_id),
        };
        let ctx = directory.directory_ctx();
        let cancel = ctx.cancel.clone();
        let resolve_step = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(0));
        let reported = resolve_step.clone();
        let expected_plugin = target.plugin_id.clone();
        let pending = ShellDetached::spawn(async move {
            let store = semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user();
            reported.store(document_open_resolve_step_code(semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep::Lease), std::sync::atomic::Ordering::Release);
            let lease = client.document_execution_target_lease(&ctx, &intent).await.map_err(|error| format!("document execution target: {error}"))?;
            let plugin_id = lease.package.plugin_id.clone();
            if expected_plugin.as_ref().is_some_and(|expected| *expected != plugin_id) {
                return Err(crate::program_bridge::ProgramFault::from(format!("the hub opens this document with {plugin_id}, not {}", expected_plugin.unwrap_or_default())));
            }
            let local_sha = local_shas.into_iter().find(|(local, _)| *local == plugin_id).and_then(|(_, sha)| sha);
            let resolved = client
                .resolve_execution_target_files(&ctx, &intent, lease, local_sha.as_deref(), &store, |step| reported.store(document_open_resolve_step_code(step), std::sync::atomic::Ordering::Release))
                .await
                .map_err(|error| format!("document execution target: {error}"))?;
            let app_id = resolved.lease.surface.app_id.clone();
            let program = match resolved.files {
                Some(files) => Some(crate::program_bridge::load_resolved_program(&plugin_id, &files.component, &files.descriptor, &resolved.lease.package.component_receipt).await?),
                None => None,
            };
            Ok::<ShellDocumentOpenAnswer, crate::program_bridge::ProgramFault>(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint: resolved.lease.checkpoint.clone() })
        });
        let mut opening = ShellDocumentOpening::new(
            label,
            ShellDocumentOpenPhase::Resolving,
            target.plugin_id.clone().unwrap_or_default(),
            target.app_id.clone().unwrap_or_default(),
            Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }),
            Some(pending),
        );
        opening.cancel = Some(cancel);
        opening.resolve_step = resolve_step;
        self.document_opening = Some(opening);
        true
    }

    /// 🧯️ Settles the open as failed, out loud: its band keeps the reason and the notice names it.
    pub(super) fn fail_document_opening(&mut self, error: String, chrome: &mut ChromeState, prefs: &PrefsState) {
        debug_log(&format!("[TRACE] wgpu shell os.open-artifact relay failed: {error}"));
        let is_de = prefs.locale_id == "de";
        chrome.show_transient_notice(shell_chrome_string("open-artifact.document-failed", is_de), semio_framework::Severity::Warning, Some("open-artifact.document-failed"));
        if let Some(opening) = self.document_opening.as_mut() {
            opening.phase = ShellDocumentOpenPhase::Failed(error);
            opening.pending = None;
            opening.prepared = None;
        }
    }

    /// 🛑️ Cancels the open in flight, or clears a settled record. The phase moves only when the step in
    /// flight answers: an instance created after the cancel is destroyed, a seeded document never binds.
    pub fn cancel_document_opening(&mut self) {
        if self.document_opening.as_ref().is_some_and(|opening| !opening.running()) {
            self.document_opening = None;
            return;
        }
        if let Some(opening) = self.document_opening.as_mut() {
            opening.cancel_requested = true;
            if let Some(cancel) = opening.cancel.as_ref() {
                cancel.cancel_now();
            }
        }
    }
    //#endregion 🔖️DirectoryAndIdentity
}

//#endregion ShellActions

