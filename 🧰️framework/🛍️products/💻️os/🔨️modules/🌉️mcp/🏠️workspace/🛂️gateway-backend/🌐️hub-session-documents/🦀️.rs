//! 🌐️ Hub session documents of the headless workspace: binding a hub document, closing hub links, the live wait, the codec registration, the document actor and the document watch. See [`super`].

use super::*;

impl HeadlessWorkspace {
    /// 🌎️ Binds one HUB document to the plugin, app, artifact schema and SURFACE the hub's own
    /// execution-target lease names for it, and holds the canonical pair the caller already read so
    /// that plugin's guest can be seeded with THIS document instead of with its own genesis
    /// (`RoutingArtifactChannel::exchange` → `PluginArtifactChannel::load_session_document`).
    ///
    /// 🐛️ Before this existed, `plugin_artifacts` was written by `create_plugin_artifact` alone — a
    /// folder-only path — so a hub-bound agent that opened a real hub document had NO binding at
    /// all: every stamp it answered named the pseudo-id `plugin:<id>`, and every mutation it prepared
    /// ran against the plugin's genesis document rather than against the one it had open. That is
    /// step 1 of the write path M8 §5.3 / M9 §8.1 name.
    ///
    /// 🪪️ Every field comes from an authenticated route, none is derived from the artifact id or
    /// guessed from a single-plugin workspace: the lease is fetched for this exact scope and its
    /// package identity is cross-checked against the descriptor snapshot the binding already
    /// verified. `Ok(false)` means this workspace is not hub-bound or the hub authorizes no
    /// execution target for the document — an unbound artifact stays readable, it simply has no
    /// guest to seed.
    ///
    /// 🔁️ Re-opening a document this session already bound must not open a SECOND actor: the
    /// first one holds the live socket, the presence lease and the outbox that still owes the hub
    /// this agent's envelopes. Only the canonical pair is refreshed — unless its link EXPIRED (a shortage outlived its
    /// bound), which a new open relinks with a fresh actor.
    ///
    /// 🔗️ Opening a new actor first closes the live links [`hub_links_to_close`] names — the app's older document
    /// (the one opened now becomes the app's session document) and, past [`HUB_SESSION_LIVE_LINK_LIMIT`], the least
    /// recently used — so one agent never holds more of the hub's document sockets than that (each is one of the hub's
    /// bounded WAL writer slots; an agent that walked 63 documents took them all, measured on 7800/p33).
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

    /// 🔗️ Closes the live hub links [`hub_links_to_close`] names before `opening` (a document of `route` = plugin, app)
    /// links: each one's actor is closed (its socket, presence row and WAL writer slot released) and its binding keeps
    /// the reason, which a later edit of it reports until `artifact_open` relinks it. Every edit the agent was told
    /// succeeded is already acknowledged by the hub; an unacknowledged one was reverted and reported, so nothing an
    /// agent believes the hub holds rides on a closed actor.
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

    /// 🧵️ Opens the hub document's own `store::sync` actor — the one that holds the document socket
    /// (`…/socket-grants` → `semio.socket.v1`), sends `ClientFrame::Commands` for every local
    /// envelope, and beats presence for this session. Returns its mailbox, or the reason there is
    /// none.
    ///
    /// 🪪️ The binding is `PersistenceBinding::Hub { surface: Some(<the LEASE's surface>) }`, not the
    /// pinned `PROBE_SURFACE_ID` `WorkspaceOrigin::persistence_binding` mints for this crate's own
    /// probe documents. A surface id is an authorization coordinate: `finish_connect_hub` compares
    /// the socket authority's scope, schema and pack-schema hash against the client's own, and the
    /// hub issues the authority for the surface the client asked for. Claiming the probe editor's
    /// surface for a `gis` document was the pin M8 §5.3 and M9 §8.1 both name as the open unknown here
    /// (`🏠️workspace/🦀️.rs:501`). The lease is handed to the host FIRST
    /// (`set_document_execution_target_lease`), because `open` consumes it when it spawns the actor
    /// and the actor sends it inside `DocumentSocketExpectationV1`.
    ///
    /// 🚧️ No registered `ArtifactCodec` for the document's artifact schema ⇒ no socket, and this
    /// says so instead of opening an actor that would reconnect-loop in silence: `connect_hub`
    /// needs `document_codec(schema).pack_schema_hash` to build `SocketHelloV1` at all. This
    /// process links the three native codecs `📇️native-openable-provider` compiles in, so a
    /// fourth package's kind — `gis.map` among them — has none. The root fix is to route the
    /// registration through the component's own `codec` export
    /// (`🔌️plugin/🧬️schema/📜️.wit`'s `interface codec`, TC3b); it is NOT done here and the
    /// reason is reported rather than papered over.
    /// 🗂️ Registers the document kind's codec from the package the HUB authorized for it — the same
    /// component bytes `action_prepare` executes, resolved through the same verified route by the same
    /// content address the lease names, so the codec and the guest can never be two different builds
    /// of one package.
    ///
    /// 🪪️ The hub's own declared `pack_schema_hash` is passed as a CROSS-CHECK and never as the
    /// value — see [`register_guest_document_codec`] for why registering the hub's number would
    /// delete the check that reads it back.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn register_hub_document_codec(&self, lease: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1) -> Result<(), GatewayError> {
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
    pub(crate) fn open_hub_document_actor(&self, artifact_id: &str, lease: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1, relay: Arc<HubRelay>) -> (Option<store::sync::ArtifactMailboxSender>, Option<String>) {
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

    /// 🤖️ Makes this session a visible principal on the hub document it holds. The hub lists a
    /// socket in the roster only once it has beaten, and keeps the row for as long as that socket
    /// stays open (`expire_presence_for_live`), so the contract is one beat per hub connection: on
    /// the connection's `Session` and on every `Live` status after it. The beat carries no identity
    /// of its own — the hub stamps the delegation's label and the principal kind it authenticated
    /// (`agent`), so the roster can never show this session as the human who delegated it. The task
    /// ends when the document closes and drops its event channel.
    ///
    /// 🚦️ The same task records every sync status and coded fault the actor reports into `relay`, which
    /// is how a commit learns whether the hub acknowledged it and `artifact_open` reports the link.
    ///
    /// 📥️ …and queues, in order, every baseline and mutation batch the actor delivers for the guest — the
    /// bootstrap tail past the checkpoint and every other writer's edit — which the guest takes before its
    /// next command ([`RoutingArtifactChannel::exchange`], [`plan_hub_inbound`]).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn watch_hub_document(&self, reactor: &'static tokio::runtime::Runtime, document_key: store::sync::ArtifactDocumentKey, relay: Arc<HubRelay>) {
        let host = self.artifact_host.clone();
        let actor = self.actor_label();
        let mut events = ::semio_framework_async::poll::resolve_ready(host.subscribe_key(&document_key));
        reactor.spawn(async move {
            loop {
                match events.recv().await {
                    Ok(event) => {
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
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
    }
}
