//! 🪪️ The headless workspace state, plugin artifact binding, drop and provenance helpers.

use super::{ActionAdapter, Arc, Catalog, GatewayError, GatewayErrorCode, HashMap, HubRemoteBinding, Mutex, OnceLock, PathBuf, ResourceContent, UntrustedAuthors, UntrustedProvenance, UntrustedRevision, UntrustedSource, untrusted_content, untrusted_content_sha256};
#[cfg(not(target_arch = "wasm32"))]
use super::NativeHubBindingDriver;
use super::hub_relay::{HubRelay, SessionDocumentPair};
use super::plugin_binding_and_activation::WorkspaceOrigin;
use super::probe_document::{PROBE_SCHEMA, ProbeStore, close_probe_store_to_terminal};
use super::routing_and_shell_routed::WorkspaceCatalog;
#[cfg(not(target_arch = "wasm32"))]
use super::routing_and_shell_routed::PluginComponentSource;

//#region 🔖️HeadlessWorkspace
/// 🏠️ The real `GatewayBackend` — see this file's module doc for the shape and the honest gaps.
pub struct HeadlessWorkspace {
    pub(super) host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy,
    #[cfg(test)]
    pub(crate) test_discovery:Option<Vec<semio_framework::PackageDescriptor>>,
    pub(super) artifact_host: store::sync::ArtifactHost,
    pub(super) origin: WorkspaceOrigin,
    pub(super) principal: String,
    pub(super) session_id: String,
    pub(super) scopes: Vec<String>,
    pub(super) repo_root: Option<PathBuf>,
    /// 🗂️ See [`WorkspaceCatalog`]: fixed for a folder, following the hub's descriptor authority for a hub.
    pub(super) catalog: Arc<WorkspaceCatalog>,
    /// 🧵️ Every document this workspace has opened a live `ProbeStore`/backbone attachment for —
    /// `artifact_open`/`ensure_probe_artifact` populate this; `resolve_context`/`read_resource` read
    /// through it first before falling back to a cold `FolderEventLogStorage` read.
    pub(super) open_probes: Mutex<HashMap<String, ProbeStore>>,
    /// 🎬️ Lazily-built, cached real `ActionAdapter` — `🔀️dispatch`'s own already-real Observe→
    /// Prepare→Preview→Commit→Undo/Redo two-phase protocol, wrapping this workspace's own real
    /// `RoutingArtifactChannel` (`open_routing_channel`). `prepare_action`/`invoke_action` delegate to
    /// it rather than duplicating that protocol — see those methods' own doc for why.
    pub(super) action_adapter: Mutex<Option<Arc<ActionAdapter>>>,
    /// 🗿️ The ROOT-owned adapter `action_prepare`/`action_invoke` actually drive
    /// (`🦀️.rs`'s `build_server_with_workspace`), bound once before the server serves. It is the
    /// only holder of the guest instances a committed mutation lives in, so it is the only place
    /// `artifact_snapshot` can read a document as the agent just left it.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) root_actions: OnceLock<Arc<ActionAdapter>>,
    /// 🗂️ `artifact_id` → the plugin-typed binding [`HeadlessWorkspace::create_plugin_artifact`]
    /// minted for it. The probe lane (`open_probes`) covers this crate's own generic document; this
    /// covers every artifact created for a REAL plugin artifact kind, and is what finally gives
    /// `artifact_export`/`semio://artifact/{id}/schema` an artifact → plugin mapping instead of the
    /// "exactly one registered plugin or bust" guess they used to make.
    ///
    /// 🗿️ Shared with this workspace's [`RoutingArtifactChannel`] (`Arc`, one map, never a copy):
    /// the channel needs the INVERSE lookup — which artifact a plugin's live session document is —
    /// so `AppFrame::HistorySnapshot` can stamp the artifact rather than the plugin id.
    pub(super) plugin_artifacts: Arc<Mutex<HashMap<String, PluginArtifactBinding>>>,
    /// 🐚️ The session's channel decision, published once by `server_for_workspace_options`. The
    /// artifact-level verbs that open a channel of their OWN (`create_plugin_artifact`,
    /// `export_artifact_media` — they never pass through `ActionAdapter`) consult it, so a session
    /// that resolved `shell` has ONE document owner for every verb rather than a split brain
    /// (`📓️lb1-live-bridge-action-routing.md` §7.3). Unset is the ordinary headless tier.
    pub(super) shell_route: OnceLock<(Arc<crate::shell_channel::SessionChannelBinding>, Arc<Catalog>)>,
    pub(super) hub_binding: Option<Arc<HubRemoteBinding>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) hub_driver: Option<Arc<NativeHubBindingDriver>>,
    /// 🧩️ Where this workspace's plugin components come from — see [`PluginComponentSource`].
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) plugin_components: Option<PluginComponentSource>,
}

#[derive(Clone)]
pub struct PluginArtifactBinding {
    pub schema: String,
    pub plugin_id: String,
    pub app_id: String,
    pub surface_id: Option<String>,
    pub document: Option<Arc<SessionDocumentPair>>,
    /// 🧵️ The hub document actor's own mailbox, when this binding opened one. Every
    /// `MessageEndpoint::Backbone` message the plugin's guest publishes for this document is handed
    /// to it as `ArtifactActorMsg::DocumentBackbone`, which is the actor path that persists the
    /// envelopes and relays them to the hub as `ClientFrame::Commands` — the SAME route the wgpu
    /// shell's `route_document_backbone_effects` drives for a human's edit.
    #[cfg(not(target_arch = "wasm32"))]
    pub backbone: Option<store::sync::ArtifactMailboxSender>,
    /// 🚧️ Why this binding opened no document actor, when it did not. A hub document whose artifact
    /// schema has no registered `store::os_store::serialization::ArtifactCodec` in this process cannot open a document socket
    /// at all: `connect_hub` reads `document_codec(schema).pack_schema_hash` to build its
    /// `SocketHelloV1` and silently reschedules a reconnect forever when there is none. Recording
    /// the reason here is what turns that silence into something a reader can see.
    pub backbone_blocked_by: Option<String>,
    /// 🧾️ How many document-backbone messages this binding has handed to its document actor. The
    /// ONE fact that separates "the guest published nothing" from "the actor has it and the socket
    /// has not flushed yet", and the agent is owed it: `action_invoke` answers `SUCCEEDED` for a
    /// transaction the guest committed, which is true and is not the same as "anyone else can see
    /// it". Published by `artifact_open`'s `sessionDocument.relayedBatches`.
    pub relayed: Arc<std::sync::atomic::AtomicU64>,
    /// 🚦️ The document actor's own sync reports, for a binding that opened one.
    pub relay: Arc<HubRelay>,
    /// 🕰️ When the agent last opened or edited this hub document ([`hub_link_tick`]; 0 for a folder artifact): its app's
    /// session document is the one with the latest, and the least recent live link closes first past the bound.
    pub used: u64,
}

impl std::fmt::Debug for PluginArtifactBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PluginArtifactBinding")
            .field("schema", &self.schema)
            .field("plugin_id", &self.plugin_id)
            .field("app_id", &self.app_id)
            .field("surface_id", &self.surface_id)
            .field("document_bytes", &self.document.as_ref().map(|pair| pair.pack.len() + pair.spr.len()))
            .field("backbone_blocked_by", &self.backbone_blocked_by)
            .field("relayed", &self.relayed.load(std::sync::atomic::Ordering::Relaxed))
            .field("used", &self.used)
            .finish()
    }
}

/// 🚪️ Real teardown for a `--folder`/`--hub`-bound `semio-os-mcp` process (and every test that opens
/// a real probe): drains every store `open_probes` still owns to `ArtifactStore::drop`'s
/// terminal-empty witness instead of leaving that to `HashMap`'s own field drop, which would hit the
/// witness assert with nothing ever drained. `artifact_host.close` runs first for each id so the
/// backbone's remote side (the actor this workspace itself opened via `ArtifactHost::open`) releases
/// its shared queue reference before `close_probe_store_to_terminal` needs to `Arc::try_unwrap` it —
/// the same "release the other side before draining" order `store::sync::ArtifactHost`'s own `Drop`
/// already follows for its actor runners.
impl Drop for HeadlessWorkspace {
    /// 🧵️ Bound hub documents close FIRST, before the driver: their actors hold the socket-grant
    /// source the driver owns, and a document that is still dialling when its grant source
    /// vanishes reconnects against nothing instead of retiring its presence row.
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let bound = std::mem::take(&mut *self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
            for (artifact_id, binding) in bound {
                if binding.backbone.is_some() {
                    self.artifact_host.close_key(&self.origin.artifact_document_key(&artifact_id));
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        drop(self.hub_driver.take());
        let probes = std::mem::take(&mut *self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
        for (artifact_id, probe_store) in probes {
            self.artifact_host.close_key(&self.origin.artifact_document_key(&artifact_id));
            close_probe_store_to_terminal(probe_store);
        }
    }
}

impl HeadlessWorkspace {
    /// 🪪️ Provenance of content read from `artifact_id`: its kind as this workspace knows it, the
    /// head it was read at, and who may have written it — the folder's one local principal, or the
    /// writers of the hub space the document lives in. `parts` are the exact bytes enveloped.
    pub fn untrusted_artifact_provenance(&self, artifact_id: &str, source: UntrustedSource, parts: &[&[u8]]) -> Result<UntrustedProvenance, GatewayError> {
        let content_sha256 = untrusted_content_sha256(parts);
        if matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            let snapshot = self.hub_snapshot()?;
            let document = snapshot.documents.values().find(|document| document.scope.document_id == artifact_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?;
            let artifact_kind = self.hub_catalog_snapshot()?.dialect_kinds.get(&document.scope).cloned();
            return Ok(UntrustedProvenance {
                source,
                artifact_id: Some(artifact_id.to_string()),
                artifact_kind,
                space_id: Some(document.scope.space_id.clone()),
                revision: UntrustedRevision { content_sha256, head_edit_id: None, commit_seq: Some(document.view.commit_seq) },
                authors: UntrustedAuthors::SpaceWriters { space_id: document.scope.space_id.clone() },
            });
        }
        let probe_head = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id).map(|probe_store| {
            let applied = probe_store.applied_edit_ids();
            (applied.last().cloned(), applied.len() as u64)
        });
        let artifact_kind = match &probe_head {
            Some(_) => Some(PROBE_SCHEMA.to_string()),
            None => self.plugin_artifact_binding(artifact_id).map(|binding| binding.schema),
        };
        Ok(UntrustedProvenance {
            source,
            artifact_id: Some(artifact_id.to_string()),
            artifact_kind,
            space_id: None,
            revision: UntrustedRevision { content_sha256, head_edit_id: probe_head.as_ref().and_then(|(head, _)| head.clone()), commit_seq: probe_head.map(|(_, count)| count) },
            authors: UntrustedAuthors::LocalPrincipal { principal: self.principal.clone() },
        })
    }

    /// 🧊️ `semio://artifact/{id}`'s body: the gateway's own sizes beside the document's pack and spr,
    /// which travel only inside the untrusted envelope.
    fn artifact_body(&self, artifact_id: &str, pack: &[u8], spr: &[u8]) -> Result<serde_json::Value, GatewayError> {
        let provenance = self.untrusted_artifact_provenance(artifact_id, UntrustedSource::ArtifactBody, &[pack, spr])?;
        Ok(serde_json::json!({
            "artifactId": artifact_id,
            "packBytes": pack.len(),
            "sprBytes": spr.len(),
            "untrusted": untrusted_content(&provenance, serde_json::json!({ "packBase64": base64_encode(pack), "sprBase64": base64_encode(spr) })),
        }))
    }

    /// 🗿️ The body and the schema are both answerable for a hub document: the body from the
    /// authenticated binding's own verified canonical pair, the schema from the descriptor
    /// the binding already published (`DocumentDescriptor.artifact_schema` — the hub's own
    /// authority over what this document IS, never a guess). Only `history` and `validation`
    /// remain typed gaps, and they are gaps in the WIRE, not in this binding.
    ///
    /// 🪢 Two vocabularies, both published, neither collapsed into the other. `schema` is
    /// the declared pack schema — what the bytes are. `artifactKind` is the owning
    /// app's canonical `<domain>.<plugin>.<artifact>` identity, whose declaring owner
    /// `DocumentIndexEntryV1::validate` enforces — what a capability's `artifactKind`
    /// filter and `capabilities_search` match on. Publishing only the schema is what made
    /// an agent unable to match ANY verb to a hub document (M8 §5.4(1), measured live).
    ///
    /// 🧯️ Real answer when this workspace genuinely knows the schema (an open probe
    /// artifact — `PROBE_SCHEMA` is this crate's own, real, no fabrication needed), a typed,
    /// retryable gap otherwise: the real wire protocol (`store::AppCommand`, enumerated
    /// exhaustively at `PluginArtifactChannel::exchange`'s own doc) has no
    /// schema/describe/manifest QUERY command at all — nothing to "ask the guest" through yet
    /// for an arbitrary plugin-backed artifact, so this names precisely what's missing rather
    /// than fabricating a schema this workspace cannot see.
    ///
    /// 🧯️ Killed fabrication: this used to unconditionally answer `{"valid": true}` for ANY
    /// artifact id, including ones this workspace has never even seen. The real wire protocol
    /// has no validate query command either (same gap as `schema` above) — a live plugin
    /// instance's own `validate()` is not reachable from here yet, so this is now a typed,
    /// retryable gap naming exactly that, never a hardcoded pass.
    pub(super) fn read_artifact_resource(&self, artifact_id: &str, suffix: Option<&str>, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        if matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            let snapshot = self.hub_snapshot()?;
            if !snapshot.documents.keys().any(|scope| scope.document_id == artifact_id) {
                return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")));
            }
            match suffix {
                None => {
                    let (pack, spr) = match self.plugin_artifact_binding(artifact_id).filter(|binding| binding.document.is_some()) {
                        Some(_) => self.read_artifact_bytes(artifact_id)?.ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?,
                        None => {
                            let scope = snapshot.documents.keys().find(|scope| scope.document_id == artifact_id).cloned().ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?;
                            self.read_hub_canonical_pair(&scope)?
                        }
                    };
                    let body = self.artifact_body(artifact_id, &pack, &spr)?;
                    return Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }]);
                }
                Some("schema") => {
                    let document = snapshot.documents.values().find(|document| document.scope.document_id == artifact_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}")))?;
                    let catalog = self.hub_catalog_snapshot()?;
                    let body = serde_json::json!({ "artifactId": artifact_id, "schema": document.view.descriptor.artifact_schema, "artifactKind": catalog.dialect_kinds.get(&document.scope), "spaceId": document.scope.space_id });
                    return Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }]);
                }
                Some("validation") => return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "artifact validation remains unavailable until the wire protocol carries a validate query command").retryable()),
                Some("history") => return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "artifact history remains unavailable until a revision-bound remote history surface exists").retryable()),
                Some(_) => return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact sub-resource for {artifact_id}"))),
            }
        }
        match suffix {
            None => match self.read_artifact_bytes(artifact_id)? {
                Some((pack, spr)) => {
                    let body = self.artifact_body(artifact_id, &pack, &spr)?;
                    Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }])
                }
                None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}"))),
            },
            Some("schema") => match (self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(artifact_id), self.plugin_artifact_binding(artifact_id)) {
                (true, _) => Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(serde_json::json!({ "artifactId": artifact_id, "schema": PROBE_SCHEMA }).to_string()), blob: None }]),
                (false, Some(binding)) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("application/json".to_string()),
                    text: Some(serde_json::json!({ "artifactId": artifact_id, "schema": binding.schema, "pluginId": binding.plugin_id, "appId": binding.app_id }).to_string()),
                    blob: None,
                }]),
                (false, None) => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{artifact_id}` has no schema this workspace can answer for — the real wire protocol has no schema/describe/manifest query command yet, and this is neither an open probe artifact nor one created here with a declared kind")).retryable()),
            },
            Some("history") => match self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id) {
                Some(probe_store) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("application/json".to_string()),
                    text: Some(serde_json::json!({ "appliedEditIds": probe_store.applied_edit_ids().iter().collect::<Vec<_>>() }).to_string()),
                    blob: None,
                }]),
                None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("`{artifact_id}` has no open history in this workspace"))),
            },
            Some("validation") => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{artifact_id}` cannot be validated yet — the real wire protocol has no validate query command, and no live plugin instance is reachable from this workspace to ask")).retryable()),
            Some(other) => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact sub-resource: {other}"))),
        }
    }
}

pub(super) fn directory_json_value<T: semio_framework_value::ToValue>(value: &T) -> Result<serde_json::Value, GatewayError> {
    let text = semio_framework_pack_json::to_json_string(value);
    serde_json::from_str(&text).map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "directory value could not be projected to MCP JSON"))
}

pub(super) fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(ALPHABET[(b0 >> 2) as usize] as char);
        out.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}
//#endregion 🔖️HeadlessWorkspace
