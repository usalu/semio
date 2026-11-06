//! 🌐️ Hub session documents of the headless workspace: binding a hub document, closing hub links, the live wait, the codec registration, the document actor and the document watch. See [`super`].

use super::*;

impl HeadlessWorkspace {
    /// 🌎️ Binds one HUB document to the plugin, app, artifact schema and SURFACE the hub's own [Design notes](README.md#source-notes-6).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_hub_session_document(&self, artifact_id: &str, pack: &[u8], spr: &[u8]) -> Result<bool, GatewayError> {
        let WorkspaceOrigin::Hub { space_id, .. } = &self.origin else {
            return Ok(false);
        };
        let snapshot = self.hub_snapshot()?;
        let Some(document) = snapshot.documents.values().find(|document| document.scope.document_id == artifact_id) else {
            return Ok(false);
        };
        let scope = document.scope.clone();
        if &scope.space_id != space_id {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("document `{artifact_id}` is not in this session's bound space")));
        }
        let driver = self.hub_driver.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub binding actor is not running").retryable())?;
        let lease = driver.fetch_execution_target_lease(&scope, &format!("mcp-session-document-{artifact_id}"))?;
        if lease.package.plugin_id != document.view.descriptor.owner.plugin_id || lease.artifact.schema != document.view.descriptor.artifact_schema || lease.artifact.kind != document.view.descriptor.artifact_kind {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "hub execution-target lease disagrees with the authenticated document descriptor"));
        }
        let established = self
            .plugin_artifacts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(artifact_id)
            .filter(|binding| binding.backbone.is_some() && binding.relay.terminal().as_deref() != Some(store::sync::DocumentLinkStatus::LinkExpired.code()))
            .cloned();
        let (backbone, backbone_blocked_by, relayed, relay) = match established {
            Some(binding) => (binding.backbone, binding.backbone_blocked_by, binding.relayed, binding.relay),
            None => {
                self.close_hub_links(artifact_id, (lease.package.plugin_id.as_str(), lease.surface.app_id.as_str()));
                let relay = Arc::new(HubRelay::default());
                let (backbone, blocked) = self.open_hub_document_actor(artifact_id, &lease, Arc::clone(&relay));
                (backbone, blocked, Arc::new(std::sync::atomic::AtomicU64::new(0)), relay)
            }
        };
        let binding = PluginArtifactBinding {
            schema: lease.artifact.schema.clone(),
            plugin_id: lease.package.plugin_id.clone(),
            app_id: lease.surface.app_id.clone(),
            surface_id: Some(lease.surface.surface_id.clone()),
            archive: None,
            document: Some(Arc::new(SessionDocumentPair { pack: pack.to_vec(), spr: spr.to_vec() })),
            backbone,
            backbone_blocked_by,
            relayed,
            relay,
            used: hub_link_tick(),
        };
        self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), binding);
        Ok(true)
    }

    /// 🔗️ Closes the live hub links [`hub_links_to_close`] names before `opening` (a document of `route` = plugin, app) [Design notes](README.md#source-notes-84).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn close_hub_links(&self, opening: &str, route: (&str, &str)) {
        let closes = {
            let mut bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let live = bound.iter().filter(|(_, binding)| binding.backbone.is_some()).map(|(artifact_id, binding)| (artifact_id.clone(), binding.plugin_id.clone(), binding.app_id.clone(), binding.used)).collect::<Vec<_>>();
            let closes = hub_links_to_close(&live, opening, route, HUB_SESSION_LIVE_LINK_LIMIT);
            for (artifact_id, close) in &closes {
                if let Some(binding) = bound.get_mut(artifact_id) {
                    binding.backbone = None;
                    binding.backbone_blocked_by = Some(match close {
                        HubLinkClose::Relinked => "its expired link is being replaced by a fresh one".to_string(),
                        HubLinkClose::Superseded => format!("its link was closed when `{opening}` became its app's open hub document; artifact_open `{artifact_id}` again to edit it"),
                        HubLinkClose::Bounded => format!("its link was closed to keep this session within {HUB_SESSION_LIVE_LINK_LIMIT} live hub documents (least recently used first); artifact_open `{artifact_id}` again to edit it"),
                    });
                }
            }
            closes
        };
        for (artifact_id, _) in closes {
            self.artifact_host.close_key(&self.origin.artifact_document_key(&artifact_id));
        }
    }

    /// 🟢️ Waits, at most [`HUB_RELAY_ACK_WAIT_MS`], until `artifact_id`'s document actor is live — its
    /// baseline and catch-up tail queued for the guest — so the agent's first command after `artifact_open`
    /// runs against the hub's head rather than its last checkpoint. Answers whether the link went live; a
    /// document with no actor answers `false` at once.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn await_hub_session_document_live(&self, artifact_id: &str) -> bool {
        self.plugin_artifact_binding(artifact_id).is_some_and(|binding| binding.backbone.is_some() && binding.relay.await_live(HUB_RELAY_ACK_WAIT_MS))
    }

    /// 🧵️ Opens the hub document's own `store::sync` actor — the one that holds the document socket [Design notes](README.md#source-notes-121).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn register_hub_document_codec(&self, lease: &semio_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1) -> Result<(), GatewayError> {
        let Some(PluginComponentSource::Hub(components)) = self.plugin_components() else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "a hub workspace with no authorized component source cannot resolve a document kind's codec").retryable());
        };
        let (component, _) = components.resolve(&lease.package.plugin_id)?;
        register_guest_document_codec(&lease.package.plugin_id, &lease.artifact.schema, semio_framework_artifact_reference::ArtifactDialect { artifact_kind: lease.parent_dialect.artifact_kind.clone(), standard: lease.parent_dialect.standard.clone(), subset: lease.parent_dialect.subset.clone() }, &component, &lease.artifact.pack_schema_hash)?;
        Ok(())
    }

    /// 🗂️ Nothing links this kind's codec, so take it from the package itself — the
    /// component the hub authorized, whose bytes this process already SHA-256-verified.
    ///
    /// 🌐️ `open` must be awaited INSIDE the reactor's context: `spawn_actor` captures
    /// `Handle::try_current()` there, once, and every later turn of this actor is polled under
    /// the handle it captured. Entering after the open would be entering nothing.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_hub_document_actor(&self, artifact_id: &str, lease: &semio_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1, relay: Arc<HubRelay>) -> (Option<store::sync::ArtifactMailboxSender>, Option<String>) {
        let schema = lease.artifact.schema.clone();
        match ::semio_framework_async::poll::resolve_ready(store::os_store::codec_registry::document_codec(&schema)) {
            Ok(Some(_)) => {}
            Ok(None) => {
                if let Err(error) = self.register_hub_document_codec(&lease) {
                    return (None, Some(error.message));
                }
            }
            Err(error) => return (None, Some(format!("document codec registry refused `{schema}`: {error}"))),
        }
        let WorkspaceOrigin::Hub { base_url, space_id } = &self.origin else {
            return (None, Some("not a hub-bound workspace".to_string()));
        };
        let document_key = self.origin.artifact_document_key(artifact_id);
        self.artifact_host.set_document_execution_target_lease(&document_key, lease.clone());
        let reactor = match hub_socket_reactor() {
            Ok(reactor) => reactor,
            Err(error) => return (None, Some(error.message)),
        };
        let _io_reactor = reactor.enter();
        let channels = semio_framework_async::block_on(self.artifact_host.open(store::sync::ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
            document_id: artifact_id.to_string(),
            schema,
            bindings: vec![store::sync::PersistenceBinding::Hub { base_url: base_url.clone(), space_id: space_id.clone(), surface: Some(lease.surface.surface_id.clone()) }],
            watch_external: false,
            actor: self.actor_label(),
        }));
        if channels.document_key != document_key {
            self.artifact_host.close_key(&channels.document_key);
            return (None, Some(format!("document `{artifact_id}` opened outside its authenticated document scope")));
        }
        self.watch_hub_document(reactor, channels.document_key.clone(), relay);
        (Some(channels.cmd_tx), None)
    }

    /// 🤖️ Makes this session a visible principal on the hub document it holds. The hub lists a [Design notes](README.md#source-notes-205).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn watch_hub_document(&self, reactor: &'static store::sync::DocumentSocketReactor, document_key: store::sync::ArtifactDocumentKey, relay: Arc<HubRelay>) {
        let host = self.artifact_host.clone();
        let actor = self.actor_label();
        let pumped_key = document_key.clone();
        self.artifact_host.pump_events_key(reactor, &pumped_key, move |event| {
            if agent_presence_moment(&event) {
                host.presence_heartbeat_key(&document_key, now_ms(), agent_presence_peer(&actor));
            }
            match event {
                store::sync::ArtifactEvent::DocumentArchiveReplaced { archive } => relay.deliver(HubInbound::Archive(archive)),
                store::sync::ArtifactEvent::DocumentBackbone { message } => relay.deliver(HubInbound::Backbone(message)),
                store::sync::ArtifactEvent::Status(status) => relay.record(Some(status), None),
                store::sync::ArtifactEvent::Conflict(message) if document_link_terminal_code(&message.code.0) => relay.record_terminal(&message.code.0, &message.message),
                store::sync::ArtifactEvent::Conflict(message) => relay.record(None, Some(format!("{}: {}", message.code.0, message.message))),
                _ => {}
            }
        });
    }
}
