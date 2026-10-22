//! 🔌️ Plugin activation and bindings of the headless workspace: activation attempts, artifact channel opening, shell route and action adapter binding, cancellable activation jobs and installed artifact bindings. See [`super`].

use super::*;

impl HeadlessWorkspace {
    /// 🔌️ Best-effort real activation attempt for `plugin_id` — see `activate_plugin_instance`'s own
    /// doc for exactly what this proves and what it deliberately does not.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attempt_plugin_activation(&self, plugin_id: &str) -> Result<PluginActivationOutcome, GatewayError> {
        let repo_root = self.repo_root.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "repo root not found — cannot locate the plugin registry or compiled wasm"))?;
        let registry = load_plugin_registry(&repo_root)?;
        let entry = find_plugin_entry(&registry, plugin_id)?;
        let descriptor = load_package_descriptor(&entry.owner_root)?;
        let editor_app = descriptor.manifest.apps.iter().find(|app| app.role == semio_framework::AppRole::Editor).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` declares no editor app")))?;
        let app_ref = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: editor_app.id.clone() };
        let runtime = shared_plugin_runtime()?;
        let (guest, outcome) = activate_plugin_instance(runtime.as_ref(), &repo_root, entry, &descriptor, &app_ref, &self.actor_label(), 1)?;
        semio_framework_async::block_on(runtime.drop_instance(guest));
        Ok(outcome)
    }

    /// 🔌️ Constructs a real `crate::actions::ArtifactChannel` for `plugin_id` — what
    /// `P6-actions-policy`'s `ActionAdapter::new` takes in place of `MockArtifactChannel` once wired
    /// (bin.rs's job, once a real `--folder`/`--hub` workspace names which plugin its default
    /// instance targets). Thin wrapper over `open_plugin_artifact_channel` — the SAME body
    /// `RoutingArtifactChannel::exchange` calls per plugin, kept here too for direct low-level tests
    /// and any caller that already knows exactly which plugin it wants (never duplicated).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_artifact_channel(&self, plugin_id: &str) -> Result<PluginArtifactChannel, GatewayError> {
        open_plugin_artifact_channel(self.plugin_components().as_ref(), plugin_id, None, &self.actor_label(), self.host_driver_policy)
    }

    /// 🐚️ Publishes this session's channel decision into the workspace. Called once, by
    /// `server_for_workspace_options`, with the SAME binding `ShellRoutedArtifactChannel` and
    /// `context_resolve` hold — never a second, divergent one.
    pub fn bind_shell_route(&self, binding: Arc<SessionChannelBinding>, catalog: Arc<Catalog>) {
        let _ = self.shell_route.set((binding, catalog));
    }

    /// 🔀️ The channel an artifact-level verb executes on. `create_plugin_artifact` and
    /// `export_artifact_media` used to hardcode `open_artifact_channel`, which meant a session that
    /// had resolved `shell` still created and exported against THIS process's interpreter — two
    /// document owners for one session. They now ask here, so the binding governs every verb.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_session_artifact_channel(&self, plugin_id: &str, app_id: Option<&str>) -> Result<ArtifactChannels, GatewayError> {
        self.open_session_artifact_channel_scoped(plugin_id, app_id, &ActivationScope::detached())
    }

    /// 🎚️ [`Self::open_session_artifact_channel`] under a caller's progress and cancel: a headless
    /// plugin channel resolves, reads, hashes and loads or compiles its component in `scope`.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_session_artifact_channel_scoped(&self, plugin_id: &str, app_id: Option<&str>, scope: &ActivationScope) -> Result<ArtifactChannels, GatewayError> {
        if let Some((binding, catalog)) = self.shell_route.get() {
            if binding.resolve() == ChannelKind::Shell {
                return Ok(ArtifactChannels::ShellDirect(ShellArtifactChannel::new(Arc::clone(binding), Arc::clone(catalog)).for_plugin(plugin_id)));
            }
        }
        Ok(ArtifactChannels::Plugin(open_plugin_artifact_channel_scoped(self.plugin_components().as_ref(), plugin_id, app_id, &self.actor_label(), scope, self.host_driver_policy)?))
    }

    /// 🔌️ Binds the root-owned `ActionAdapter` once, before the server serves. Idempotent: a second
    /// bind is ignored rather than swapping the adapter under an in-flight call.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_root_action_adapter(&self, actions: Arc<ActionAdapter>) {
        let _ = self.root_actions.set(actions);
    }

    /// 📖️ `plugin_id`'s LIVE headless session document, off the root adapter's own guest instance —
    /// the one a committed `action_invoke` mutated. `None` when no adapter is bound (a bare
    /// workspace in a unit test) or when this workspace's catalog gives the plugin no instance slot.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn read_live_session_artifact_bytes(&self, plugin_id: &str, app_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        let Some(actions) = self.root_actions.get() else { return Ok(None) };
        let Some(instance) = app_instance_slot(&self.catalog.current(), plugin_id, app_id) else { return Ok(None) };
        actions.read_session_artifact(instance).map(Some)
    }

    /// 🔗️ `artifact_id`'s live document for an inference binding, under `scope`. A headless plugin
    /// session is activated first — component resolved, read, content-hashed and compiled, guest
    /// opened — reporting each phase it actually runs, and the document is read from it after that.
    /// Every other lane (an open probe, a shell-owned session, a hub or folder row) only reads.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_artifact_document(&self, artifact_id: &str, scope: &ActivationScope) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        if let Some(binding) = self.plugin_artifact_binding(artifact_id) {
            let shell_owned = self.shell_route.get().is_some_and(|(route, _)| route.resolve() == ChannelKind::Shell);
            if let (false, Some(actions), Some(instance)) = (shell_owned, self.root_actions.get(), app_instance_slot(&self.catalog.current(), &binding.plugin_id, &binding.app_id)) {
                actions.activate_session(instance, scope)?;
            }
        }
        scope.enter(ActivationPhase::ReadingDocument).map_err(activation_fault)?;
        self.read_artifact_bytes(artifact_id)
    }

    /// ⏳️ [`Self::create_plugin_artifact`] as a job — see [`Self::run_activation_job`].
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_plugin_artifact_cancellably(self: &Arc<Self>, artifact_id: &str, kind: &InstalledArtifactKind, scope: ActivationScope) -> Result<(usize, usize), GatewayError> {
        let artifact_id = artifact_id.to_string();
        let kind = kind.clone();
        self.run_activation_job(scope, move |workspace, scope| workspace.create_plugin_artifact(&artifact_id, &kind, scope))
    }

    /// ⏳️ [`Self::bind_artifact_document`] as a job — see [`Self::run_activation_job`].
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_artifact_document_cancellably(self: &Arc<Self>, artifact_id: &str, scope: ActivationScope) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        let artifact_id = artifact_id.to_string();
        self.run_activation_job(scope, move |workspace, scope| workspace.bind_artifact_document(&artifact_id, scope))
    }

    /// 🔌️ Activates `plugin_id`'s headless session — the first half of every binding — as a job, so a
    /// caller can bring a plugin up ahead of its first artifact with the same progress and cancel.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn activate_plugin_session_cancellably(self: &Arc<Self>, plugin_id: &str, scope: ActivationScope) -> Result<(), GatewayError> {
        let actions = self.root_actions.get().cloned().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "no action adapter is bound to this workspace yet").retryable())?;
        let instance = plugin_instance_slot(&self.catalog.current(), plugin_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` owns no instance slot in this workspace's catalog")))?;
        self.run_activation_job(scope, move |_, scope| actions.activate_session(instance, scope))
    }

    /// ⏳️ Runs `work` on the process pool's I/O lane while the caller waits for its answer or for the
    /// scope's cancel, whichever comes first, so a cancel is answered the moment it lands rather than
    /// when the phase in flight ends. The job itself stops at its next interruptible boundary and
    /// drops whatever it half-opened.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn run_activation_job<T: Send + 'static>(self: &Arc<Self>, scope: ActivationScope, work: impl FnOnce(&HeadlessWorkspace, &ActivationScope) -> Result<T, GatewayError> + Send + 'static) -> Result<T, GatewayError> {
        let cancel = scope.cancel().clone();
        let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
        let workspace = Arc::clone(self);
        workspace_worker_pool().submit(
            semio_framework_async::Lane::Io,
            Box::new(move || {
                let _ = sender.send(work(&workspace, &scope));
            }),
        );
        match semio_framework_async::block_on(semio_framework_async::select2(receiver, cancel.cancelled())) {
            semio_framework_async::Either::Left(Ok(answer)) => answer,
            semio_framework_async::Either::Left(Err(_)) => Err(GatewayError::new(GatewayErrorCode::Internal, "the activation job ended without answering")),
            semio_framework_async::Either::Right(()) => Err(GatewayError::new(GatewayErrorCode::Cancelled, "the caller cancelled while its plugin session was being activated")),
        }
    }

    pub(crate) fn read_session_artifact_bytes(&self, plugin_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        let Some((binding, catalog)) = self.shell_route.get() else { return Ok(None) };
        if binding.resolve() != ChannelKind::Shell {
            return Ok(None);
        }
        let mut channel = ArtifactChannels::ShellDirect(ShellArtifactChannel::new(Arc::clone(binding), Arc::clone(catalog)).for_plugin(plugin_id));
        let frames = channel
            .exchange(0, vec![AppCommand::ReadArtifact])
            .map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` refused ReadArtifact ({}): {}", fault.code, fault.message)))?;
        match frames.into_iter().next() {
            Some(AppFrame::Artifact { pack, spr }) => Ok(Some((pack, spr))),
            Some(AppFrame::Error(fault)) => Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{plugin_id}` rejected ReadArtifact ({}): {}", fault.code, fault.message))),
            other => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` answered ReadArtifact with {other:?}"))),
        }
    }

    /// 🗂️ Every artifact kind the INSTALLED plugins actually declare, read off their descriptors
    /// (`AppIo.artifact_schema` of each editor app, plus every `ArtifactKindSpec.kind_id` that app
    /// declares) — the real vocabulary `artifact_create{kind}` validates against and `artifact_export`
    /// reads its ports from, so an unknown kind is a typed refusal naming the installed set instead of a
    /// silently generic document. Never invented: a folder workspace reads the repository's committed
    /// descriptors (a plugin with no decodable descriptor contributes nothing); a hub workspace reads the
    /// packages the hub's live descriptor authority selected — the same descriptors its guests run.
    ///
    /// 📤️ `media_out_ports` is [`app_media_out_ports`], not `AppDefinition.media_outputs` alone —
    /// see that function for the measurement.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn installed_artifact_kinds(&self) -> Result<Vec<InstalledArtifactKind>, GatewayError> {
        let descriptors: Vec<semio_framework::PackageDescriptor> = match &self.origin {
            WorkspaceOrigin::Hub { .. } => self.verified_hub_catalog_selections()?.selections.iter().map(|selection| selection.descriptor.clone()).collect(),
            WorkspaceOrigin::Folder { .. } => {
                let repo_root = self.repo_root.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "repo root not found — cannot locate the plugin registry"))?;
                load_plugin_registry(&repo_root)?.into_iter().filter_map(|entry| load_package_descriptor(&entry.owner_root).ok()).collect()
            }
        };
        let mut kinds: std::collections::BTreeMap<String, InstalledArtifactKind> = std::collections::BTreeMap::new();
        for descriptor in &descriptors {
            for app in &descriptor.manifest.apps {
                if app.role != semio_framework::AppRole::Editor {
                    continue;
                }
                let ports = app_media_out_ports(app);
                let formats: Vec<String> = app.artifact_kinds.iter().flat_map(|kind| kind.export_formats.iter().cloned()).collect();
                for schema in std::iter::once(app.dialect.artifact_kind.clone())
                    .chain(std::iter::once(app.io.artifact_schema.clone()))
                    .chain(app.artifact_kinds.iter().map(|kind| kind.schema.clone()))
                    .filter(|schema| !schema.is_empty())
                {
                    kinds.entry(schema.clone()).or_insert_with(|| InstalledArtifactKind {
                        schema,
                        plugin_id: descriptor.manifest.plugin_id.clone(),
                        app_id: app.id.clone(),
                        media_out_ports: ports.clone(),
                        export_formats: formats.clone(),
                    });
                }
            }
        }
        Ok(kinds.into_values().collect())
    }

    /// 🗂️ The plugin binding this workspace minted for `artifact_id`, if any — see
    /// [`PluginArtifactBinding`].
    pub fn plugin_artifact_binding(&self, artifact_id: &str) -> Option<PluginArtifactBinding> {
        self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id).cloned()
    }
}
