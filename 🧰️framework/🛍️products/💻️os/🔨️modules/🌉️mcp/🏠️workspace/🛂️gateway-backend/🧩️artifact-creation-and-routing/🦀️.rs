//! 🧩️ Artifact creation and routing of the headless workspace: plugin artifact creation, media export, capability resolution, the routing channel and the action adapter. See [`super`].

use super::*;

impl HeadlessWorkspace {

    /// 🔎️ Validates the live canonical pair with its declared owner codec.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn validate_artifact(&self, artifact_id: &str, scope: &ActivationScope) -> Result<serde_json::Value, GatewayError> {
        scope.enter(ActivationPhase::ReadingDocument).map_err(activation_fault)?;
        let (pack, spr) = self.read_artifact_bytes(artifact_id)?.ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?;
        let schema = if self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(artifact_id) {
            PROBE_SCHEMA.to_string()
        } else if let Some(binding) = self.plugin_artifact_binding(artifact_id) {
            binding.schema
        } else if matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            self.hub_snapshot()?.documents.values().find(|document| document.scope.document_id == artifact_id).map(|document| document.view.descriptor.artifact_schema.clone()).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?
        } else {
            semio_framework_async::block_on(store::os_store::codec_registry::lane_schema_from_spr(&spr)).ok_or_else(|| GatewayError::new(GatewayErrorCode::InputInvalid, "artifact history has no decodable schema identity"))?
        };
        self.validate_artifact_pair(artifact_id, &schema, &pack, &spr, scope)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn validate_artifact_pair(&self, artifact_id: &str, schema: &str, pack: &[u8], spr: &[u8], scope: &ActivationScope) -> Result<serde_json::Value, GatewayError> {
        scope.checkpoint().map_err(activation_fault)?;
        let codec = semio_framework_async::block_on(store::os_store::codec_registry::document_kind_codec(schema)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let codec = codec.filter(|codec| match codec {
            store::os_store::codec_registry::DocumentKindCodec::Linked(codec) => codec.reference.is_some(),
            store::os_store::codec_registry::DocumentKindCodec::Component(_) => true,
        });
        let checked = match codec {
            Some(codec) => semio_framework_async::block_on(codec.print_mirror(pack, spr)).map(|_| ()).map_err(|error| error.to_string()),
            None => {
                let kind = self.installed_artifact_kinds()?.into_iter().find(|kind| kind.schema == schema).ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("no owner codec is installed for artifact schema {schema}")).retryable())?;
                let channel = open_plugin_artifact_channel_scoped(self.plugin_components().as_ref(), &kind.plugin_id, Some(&kind.app_id), &self.actor_label(), scope, self.host_driver_policy, self.operation_policy)?;
                semio_framework_async::block_on(channel.runtime.codec_print_mirror(&channel.compiled, schema, pack, spr, &semio_plugin_host::GUEST_CODEC_BUDGET)).map(|_| ()).map_err(|error| error.to_string())
            }
        };
        scope.checkpoint().map_err(activation_fault)?;
        let diagnostics: Vec<String> = checked.as_ref().err().cloned().into_iter().collect();
        let provenance = self.untrusted_artifact_provenance(artifact_id, UntrustedSource::ArtifactBody, &[pack, spr])?;
        Ok(serde_json::json!({ "artifactId": artifact_id, "schema": schema, "validation": "canonical-pair", "valid": checked.is_ok(), "untrusted": untrusted_content(&provenance, serde_json::json!({ "diagnostics": diagnostics })) }))
    }

    /// ⏳️ Runs owner validation with caller-visible progress and cancellation.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn validate_artifact_cancellably(self: &Arc<Self>, artifact_id: &str, scope: ActivationScope) -> Result<serde_json::Value, GatewayError> {
        let artifact_id = artifact_id.to_string();
        self.run_activation_job(scope, move |workspace, scope| workspace.validate_artifact(&artifact_id, scope))
    }

    /// 🆕️ Creates `artifact_id` as a REAL artifact of `schema`, seeded from the owning plugin's own See [design notes](README.md).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_plugin_artifact(&self, artifact_id: &str, kind: &InstalledArtifactKind, scope: &ActivationScope) -> Result<(usize, usize), GatewayError> {
        let path = match &self.origin {
            WorkspaceOrigin::Folder { path } => path.clone(),
            WorkspaceOrigin::Hub { .. } => return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "creating a plugin-typed artifact in a hub-bound workspace needs the hub's own document-create authority — bind --folder to create one locally").retryable()),
        };
        let mut channel = self.open_session_artifact_channel_scoped(&kind.plugin_id, Some(&kind.app_id), scope)?;
        channel.activate(0, scope).map_err(activation_fault)?;
        scope.enter(ActivationPhase::ReadingDocument).map_err(activation_fault)?;
        let frames = channel.exchange(0, vec![AppCommand::ReadArtifact]).map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{}` refused ReadArtifact ({}): {}", kind.plugin_id, fault.code, fault.message)))?;
        let (pack, spr) = match frames.into_iter().next() {
            Some(AppFrame::Artifact { pack, spr }) => (pack, spr),
            Some(AppFrame::Error(fault)) => return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{}` rejected ReadArtifact ({}): {}", kind.plugin_id, fault.code, fault.message))),
            other => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{}` answered ReadArtifact with {other:?}", kind.plugin_id))),
        };
        let canonical = Self::canonical_artifact_owner(&spr, &self.installed_artifact_kinds()?)?;
        if canonical.plugin_id != kind.plugin_id || canonical.app_id != kind.app_id { return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "owner pair schema belongs to another plugin or app")); }
        scope.checkpoint().map_err(activation_fault)?;
        let storage = store::sync::FolderEventLogStorage::new(path);
        if !::semio_framework_async::poll::resolve_ready(storage.write_if_current(artifact_id, &canonical.schema, None, &pack, &spr)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("persisting `{artifact_id}`: {error}")))? { return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "artifact already exists")); }
        self.plugin_artifacts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(artifact_id.to_string(), PluginArtifactBinding { schema: canonical.schema, plugin_id: kind.plugin_id.clone(), app_id: kind.app_id.clone(), surface_id: None, archive: None, document: Some(Arc::new(SessionDocumentPair { pack: pack.clone(), spr: spr.clone() })), backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
        Ok((pack.len(), spr.len()))
    }

    /// 📤️ `artifact_id`'s real exported bytes for one of the owning app's OUT ports, produced by that
    /// plugin's own guest — the stepped archive load of the artifact's stored pair, then `MediaOut` on `port`
    /// (`AppCommand::ExportMedia`). The gateway interprets none of it: the guest's `descriptor`/`data`
    /// come back verbatim.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn export_artifact_media(&self, plugin_id: &str, artifact_id: &str, port: &str) -> Result<(Vec<u8>, Vec<u8>, String), GatewayError> {
        let (pack, spr) = self.read_artifact_bytes(artifact_id)?.ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?;
        let app_id = self.plugin_artifact_binding(artifact_id).map(|binding| binding.app_id);
        let mut channel = self.open_session_artifact_channel(plugin_id, app_id.as_deref())?;
        let frames = channel
            .exchange(0, vec![AppCommand::ExportMedia { port: port.to_string(), document: pack, document_spr: spr }])
            .map_err(|fault| GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` refused ExportMedia on `{port}` ({}): {}", fault.code, fault.message)))?;
        match frames.into_iter().next() {
            Some(AppFrame::Exported { port, descriptor, data }) => Ok((descriptor, data, port)),
            Some(AppFrame::Error(fault)) => Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("`{plugin_id}` rejected ExportMedia on `{port}` ({}): {}", fault.code, fault.message))),
            other => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` answered ExportMedia with {other:?}"))),
        }
    }

    /// 🧭️ `capability_id` → its owning plugin id — the routing key `open_routing_channel`'s See [design notes](README.md).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn resolve_plugin_for_capability(&self, capability_id: &str) -> Result<String, GatewayError> {
        resolve_plugin_for_capability_in(&self.catalog.current(), capability_id)
    }

    /// 📇️ Every distinct plugin id this workspace's own catalog names — real callers are facets
    /// (`🗿️artifact`, `💡️inference`) that need to enumerate installed plugins without duplicating the
    /// catalog scan `resolve_plugin_for_capability`/`RoutingArtifactChannel` already do.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn catalog_plugin_ids(&self) -> Vec<String> {
        distinct_plugin_ids(&self.catalog.current())
    }

    /// 🚦️ Builds this workspace's own `RoutingArtifactChannel` — the one `crate::actions::ArtifactChannel`
    /// every real mutation now goes through, whether driven by this struct's own `action_adapter`
    /// (below) or by `🦀️.rs`'s separate root-owned adapter (`server_for_workspace_options`,
    /// its ONLY call site that names this method).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_routing_channel(&self) -> RoutingArtifactChannel {
        RoutingArtifactChannel::new(Arc::clone(&self.catalog), self.plugin_components(), self.actor_label(), Arc::clone(&self.plugin_artifacts), self.hub_binding.clone(), self.host_driver_policy, self.operation_policy).with_folder(match &self.origin { WorkspaceOrigin::Folder { path } => Some(path.clone()), _ => None })
    }

    /// 🎬️ Lazily builds (and caches) the real `ActionAdapter` `prepare_action`/`invoke_action` See [design notes](README.md).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn action_adapter(&self) -> Result<Arc<ActionAdapter>, GatewayError> {
        let mut cached = self.action_adapter.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(adapter) = cached.as_ref() {
            return Ok(adapter.clone());
        }
        let adapter = Arc::new(ActionAdapter::new(
            Box::new(ArtifactChannels::Routing(self.open_routing_channel())),
            Arc::new(HandleTable::new()),
            Arc::new(IdempotencyStore::new()),
            Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())),
            AutoApprovePolicy::Never,
            ClientInfo { name: "semio-os-mcp-headless-workspace".to_string(), version: env!("CARGO_PKG_VERSION").to_string() },
        ));
        *cached = Some(adapter.clone());
        Ok(adapter)
    }

    pub(crate) fn agent_principal(&self) -> AgentPrincipal {
        AgentPrincipal::from_scope_names(self.principal.clone(), "headless workspace agent", &self.scopes, None)
    }
}
