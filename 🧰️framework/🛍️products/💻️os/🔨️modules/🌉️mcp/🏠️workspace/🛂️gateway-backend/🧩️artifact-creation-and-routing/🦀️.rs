//! 🧩️ Artifact creation and routing of the headless workspace: plugin artifact creation, media export, capability resolution, the routing channel and the action adapter. See [`super`].

use super::*;

impl HeadlessWorkspace {
    /// 🆕️ Creates `artifact_id` as a REAL artifact of `schema`, seeded from the owning plugin's own
    /// freshly-opened document (`AppCommand::ReadArtifact` → the guest's `ReadDocument`) and persisted
    /// under that plugin's real schema id. The bytes are host-opaque throughout — this crate never
    /// decodes a plugin document, it moves the guest's own pack+spr into the folder event log the
    /// same way `store::sync` does, which is why `kind` can finally mean something without this host
    /// owning a single plugin type.
    ///
    /// ⏳️ Every activation phase it runs is reported to `scope`, and a cancel observed at any phase
    /// boundary — the last one is just before persisting — creates nothing.
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
        scope.checkpoint().map_err(activation_fault)?;
        let storage = store::sync::FolderEventLogStorage::new(path);
        ::semio_framework_async::poll::resolve_ready(storage.write(artifact_id, &kind.schema, &pack, &spr)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("persisting `{artifact_id}`: {error}")))?;
        self.plugin_artifacts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(artifact_id.to_string(), PluginArtifactBinding { schema: kind.schema.clone(), plugin_id: kind.plugin_id.clone(), app_id: kind.app_id.clone(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
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

    /// 🧭️ `capability_id` → its owning plugin id — the routing key `open_routing_channel`'s
    /// `RoutingArtifactChannel` resolves a `PureCommand` through, and what `prepare_action` below
    /// resolves FIRST (before any channel exchange) both to fail fast on a bad capability id and to
    /// pick the right `plugin_instance_slot`. Replaces the deleted `resolve_default_plugin_id`'s
    /// "exactly one plugin or bust": multiple simultaneously-registered plugins are no longer
    /// ambiguous — each capability id routes to exactly the plugin its own catalog entry names.
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
        RoutingArtifactChannel::new(Arc::clone(&self.catalog), self.plugin_components(), self.actor_label(), Arc::clone(&self.plugin_artifacts), self.hub_binding.clone(), self.host_driver_policy)
    }

    /// 🎬️ Lazily builds (and caches) the real `ActionAdapter` `prepare_action`/`invoke_action`
    /// delegate to — fresh, in-process `HandleTable`/`IdempotencyStore`/`InMemoryAuditSink` (this
    /// workspace's own private mutation-protocol bookkeeping, never shared with `🦀️.rs`'s
    /// separate root-owned adapter the `action_prepare`/`action_invoke` MCP tools already drive
    /// directly — see this struct's own field doc for why duplicating that wiring here is fine: nothing
    /// else in this crate calls `GatewayBackend::prepare_action`/`invoke_action` today). Never fails to
    /// build on its own any more — a `RoutingArtifactChannel` opens no plugin until a command actually
    /// names one, so "which plugin" errors surface per-call (`plugin_id_for`), not at construction.
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
