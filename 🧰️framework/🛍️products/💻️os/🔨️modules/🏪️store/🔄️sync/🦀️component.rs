//! 🔁️ Local-first sync actor layer: a schema-agnostic per-document backbone actor that runs all IO
//! (persist, semio_hub WebSocket sync, file watching) off the UI thread, plus the causal {@link SyncSession}
//! that feeds remote {@link MutationEnvelope}s into a document's vcs edit timeline.
//!
//! # Threading model
//! - **Native** (wgpu native host, tests): {@link ArtifactHost::open} spawns a dedicated `std::thread`
//!   running a current-thread tokio runtime; the actor `select!`s over the store's outbound queue, a
//!   semio_hub WebSocket, a `notify` file watcher, and reconnect/debounce timers.
//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on `wasm_bindgen_futures::
//!   spawn_local` with a `web_sys::WebSocket` semio_hub transport (no threads, no filesystem). The
//!   production browser shell instead uses a TS twin (`🟦️backbone-worker.ts`, WS-E); this wasm actor
//!   keeps the crate coherent for a future in-wasm host.
//! - **WASI-P2 plugins never link this crate** — inside the sandbox a store attaches vcs's pure
//!   `PortBackbone` (an in-memory queue relayed to the host). This actor is a host-side concern only.

use crate::os_spr::{decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes, AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, MutationEnvelope, ServerFrame};
use crate::os_spr::{ActorId, MutationId};
use crate::os_spr::PresencePeer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::os_store::{reconcile_alternative, BackboneMessage, ChannelBackbone, ChannelBackboneRemote, ArtifactPackFiles, ArtifactStore, ArtifactTextFiles, SpaceConflict};
use tokio::sync::{broadcast, mpsc};

//#region 🔖️Errors
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("vcs error: {0}")]
    Vcs(String),
    #[error("actor error: {0}")]
    Actor(String),
}
//#endregion 🔖️Errors

//#region 🔖️EnvelopeSerde
/// @emoji 🧵️ JSON worker seam: `MutationEnvelope` vectors as `encode_envelopes` bytes (not struct JSON).
mod envelope_serde {
    use crate::os_spr::{decode_envelopes, encode_envelopes, MutationEnvelope};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(envelopes: &[MutationEnvelope], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = encode_envelopes(envelopes);
        let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
        for byte in bytes {
            seq.serialize_element(&byte)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<MutationEnvelope>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> serde::de::Visitor<'de> for BytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a byte sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = Vec::new();
                while let Some(byte) = seq.next_element()? {
                    bytes.push(byte);
                }
                Ok(bytes)
            }
        }

        let bytes = deserializer.deserialize_seq(BytesVisitor)?;
        decode_envelopes(&bytes).map_err(serde::de::Error::custom)
    }
}
//#endregion 🔖️EnvelopeSerde

//#region 🔖️Protocol
/// @emoji 🗃️ A durable place a document synchronizes with. A document may bind to several at once
/// (folder-only, semio_hub-only, or both); the actor treats each as an independent peer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PersistenceBinding {
    /// @emoji 📁️ Local canonical store. A directory uses the multi-document `folder://` sqlite store;
    /// a `*.json` path uses the single-blob `file://` export format.
    Folder { path: std::path::PathBuf },
    /// @emoji ☁️ A semio_hub node reachable over WebSocket
    /// (`remote://host:port` → `ws://host:port/spaces/{space_id}/documents/{id}/ws`).
    Hub {
        base_url: String,
        space_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        token: Option<String>,
    },
}

/// @emoji 🧾️ Everything {@link ArtifactHost::open} needs to spawn one document's actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactActorConfig {
    pub document_id: String,
    pub schema: String,
    pub bindings: Vec<PersistenceBinding>,
    /// @emoji 👁️ Watch the folder binding for external edits (other processes writing the file).
    #[serde(default)]
    pub watch_external: bool,
    /// @emoji 🖋️ The authoring actor id used for semio_hub `Hello`/presence and operation origin filtering.
    pub actor: String,
}

/// @emoji 📨️ Caller → actor control messages, sent on the {@link ArtifactChannels} command channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactActorMsg {
    /// @emoji ⬆️ Wakes the actor to drain the store's outbound operations promptly. `envelopes` is a
    /// direct-injection fallback used only when no store is attached to the channel (empty = pure wake).
    LocalMutations {
        #[serde(with = "envelope_serde")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// @emoji 📡️ Broadcasts this peer's presence/selection to the semio_hub.
    PresenceHeartbeat { peer: PresencePeer },
    /// @emoji 👻️ Publishes an ephemeral, best-effort UI-state blob on the semio_hub's uncredited preview
    /// lane (`crate::os_spr::wire::ClientFrame::PreviewPublish`) — e.g. a drag ghost or live cursor;
    /// `seq` is a per-`key` monotone counter so a receiver can drop stale-arriving previews.
    PublishPreview { key: String, seq: u64, payload: Vec<u8> },
    /// @emoji 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,
    /// @emoji ✂️ Flushes any pending outbound operations, then stops the actor.
    Detach,
}

/// @emoji 📶️ Connection state of a document's remote (semio_hub) transport.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemoteState {
    Detached,
    Connecting,
    Live { peer_count: usize },
    Backoff { retry_in_ms: u64 },
}

/// @emoji 🚦️ Snapshot of a document's sync health for status badges.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactSyncStatus {
    pub persisted: bool,
    pub pending_mutations: usize,
    pub remote: RemoteState,
}

impl Default for ArtifactSyncStatus {
    fn default() -> Self {
        Self { persisted: false, pending_mutations: 0, remote: RemoteState::Detached }
    }
}

/// @emoji 📬️ Actor → subscriber events, delivered on the broadcast channel from {@link ArtifactHost::subscribe}.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactEvent {
    /// @emoji 🕸️ Remote operations (semio_hub fan-out or appended external edits) — also pushed into the store's
    /// inbound queue so `store.tick()` materializes them.
    RemoteMutations {
        #[serde(with = "envelope_serde")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// @emoji 📸️ The whole document was replaced (divergent external history / semio_hub snapshot swap),
    /// as real pack+spr bytes — no JSON envelope anywhere in this actor's own path.
    SnapshotReplaced { pack: Vec<u8>, spr: Vec<u8> },
    /// @emoji 🚦️ Sync status changed.
    Status(ArtifactSyncStatus),
    /// @emoji 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// @emoji 👻️ A peer published an ephemeral preview blob (`crate::os_spr::wire::ServerFrame::Preview`)
    /// on the uncredited, loss-tolerant preview lane — the counterpart of
    /// {@link ArtifactActorMsg::PublishPreview}.
    Preview { actor: String, key: String, seq: u64, payload: Vec<u8> },
    /// @emoji 📮️ The semio_hub's terminal disposition for one outbound `Commands` batch
    /// (`crate::os_spr::wire::ServerFrame::Ack`'s `Applied` stage) — accepted as-is, transformed against
    /// concurrent history (the transformed envelope is already delivered as a
    /// {@link ArtifactEvent::RemoteMutations} replacing the speculative local one), or rejected
    /// (the speculative local head is rolled back via {@link rollback_envelope} before this fires).
    CommandOutcome { batch_id: u64, outcome: CommandAckOutcome },
    /// @emoji ⚠️ A structural conflict (external divergence with local pending operations / semio_hub CAS reject).
    Conflict(SpaceConflict),
}

/// @emoji ⚖️ The client-side twin of `crate::os_spr::wire::ApplyOutcome`, minus the `Transformed`
/// envelope payload (already delivered separately as {@link ArtifactEvent::RemoteMutations} by
/// the time this fires — see {@link ArtifactEvent::CommandOutcome}).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CommandAckOutcome {
    Accepted,
    Transformed,
    Rejected { reason: String },
}
//#endregion 🔖️Protocol

//#region 🔖️BackboneWorkerWire
/// @emoji 🧵️ Binary worker seam: `MAGIC` + `crate::os_store::pack_rt::encode_wire_value` over a `DslValue`
/// tree (serde-shaped), shared by the wasm `store_worker` and `🟦️backbone-worker.ts`.
pub mod backbone_worker_wire {
    use super::{ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, PersistenceBinding};
    use crate::os_dsl::{from_dsl_value, to_dsl_value};

    pub const MAGIC: u8 = 0x01;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerRequest {
        Open {
            document_id: String,
            schema: String,
            bindings: Vec<PersistenceBinding>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            watch_external: Option<bool>,
            actor: String,
        },
        Close {
            document_id: String,
        },
        Send {
            document_id: String,
            message: ArtifactActorMsg,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerResponse {
        Event { document_id: String, event: ArtifactEvent },
        Ready,
    }

    impl BackboneWorkerRequest {
        pub fn actor_config(&self) -> Option<ArtifactActorConfig> {
            let Self::Open { document_id, schema, bindings, watch_external, actor } = self else {
                return None;
            };
            Some(ArtifactActorConfig { document_id: document_id.clone(), schema: schema.clone(), bindings: bindings.clone(), watch_external: watch_external.unwrap_or(true), actor: actor.clone() })
        }
    }

    pub fn encode_request(request: &BackboneWorkerRequest) -> Result<Vec<u8>, String> {
        let dsl = to_dsl_value(request)?;
        let mut bytes = vec![MAGIC];
        bytes.extend(crate::os_store::pack_rt::encode_wire_value(&dsl));
        Ok(bytes)
    }

    pub fn decode_request(bytes: &[u8]) -> Result<BackboneWorkerRequest, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = crate::os_store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        from_dsl_value(dsl)
    }

    pub fn encode_response(response: &BackboneWorkerResponse) -> Result<Vec<u8>, String> {
        let dsl = to_dsl_value(response)?;
        let mut bytes = vec![MAGIC];
        bytes.extend(crate::os_store::pack_rt::encode_wire_value(&dsl));
        Ok(bytes)
    }

    pub fn decode_response(bytes: &[u8]) -> Result<BackboneWorkerResponse, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = crate::os_store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        from_dsl_value(dsl)
    }
}
//#endregion 🔖️BackboneWorkerWire

//#region 🔖️Endpoints
// 🧱️ `ArtifactId` is used only by native-only test helpers below (the wire bridge and production
// folder-reconstruction path move `crate::os_spr::MutationEnvelope`s without spelling this type), so it
// stays a native-only import to avoid an unused-import warning on the wasm32 build.
#[cfg(not(target_arch = "wasm32"))]
use crate::os_spr::ArtifactId;

/// @emoji 🆔️ One `HistoryEdit`'s op ids, matching `crate::os_spr::mutation_envelope_from_edit`'s own
/// fallback convention (`meta[i].op_id` when present, else `"{edit.id}#{i}"`) so this is the SAME
/// id a live-dispatched envelope for this edit already carries. This is the ONE id domain the
/// actor's dedup set uses — computed here and ONLY here, so a locally-flushed edit's spr entry
/// and the envelope built from re-reading that same spr always agree, fixing the actor's old
/// self-re-ingest bug (mixing envelope op ids with raw JSON edit ids) by construction.
#[cfg(not(target_arch = "wasm32"))]
fn op_ids_of(edit: &crate::os_spr::HistoryEdit) -> Vec<String> {
    match &edit.meta {
        Some(metas) if metas.len() == edit.ops.len() => metas.iter().enumerate().map(|(index, meta)| meta.op_id.clone().unwrap_or_else(|| format!("{}#{index}", edit.id))).collect(),
        _ => (0..edit.ops.len()).map(|index| format!("{}#{index}", edit.id)).collect(),
    }
}

/// @emoji #⃣ Content hash over the concatenated pack+spr bytes, for the actor's self-write
/// suppression check (was a hash over the JSON envelope string; same purpose, real bytes now).
#[cfg(not(target_arch = "wasm32"))]
fn backbone_pack_hash(pack: &[u8], spr: &[u8]) -> String {
    let mut combined = Vec::with_capacity(pack.len() + spr.len());
    combined.extend_from_slice(pack);
    combined.extend_from_slice(spr);
    semio_framework_hash::hash_bytes(&combined)
}

/// @emoji 🆔️ Every op id across every edit in an spr byte log — the actor's dedup/known-ids set,
/// read directly off the binary history (NEVER via `parse_document_spr`, whose meta-absent branch
/// mints fresh random ids on every read and would make dedup unstable across reads).
#[cfg(not(target_arch = "wasm32"))]
fn spr_op_ids(spr: &[u8]) -> Result<std::collections::HashSet<String>, String> {
    let reader = crate::os_spr::HistoryReader::open(spr, &crate::os_spr::DecodeOptions::default()).map_err(|error| error.to_string())?;
    let mut ids = std::collections::HashSet::new();
    for edit in reader.edits() {
        let edit = edit.map_err(|error| error.to_string())?;
        ids.extend(op_ids_of(&edit));
    }
    Ok(ids)
}

/// @emoji 📦️ Rebuilds real {@link MutationEnvelope}s (one per forward op, genuine `OpBinary`
/// payloads straight from the edit's own binary `OpPayload`s — no codec, no JSON) from one
/// `HistoryEdit` decoded off the spr bytes, so an appended external edit can flow through the
/// store's causal DAG (`ingest_remote` → `edit_from_operation_envelope`). A binary-less op payload
/// is a hard error — `.spr` is binary-only since B1, so every real op has one.
#[cfg(not(target_arch = "wasm32"))]
fn envelopes_from_history_edit(edit: &crate::os_spr::HistoryEdit, document_id: &str, schema: &str) -> Result<Vec<MutationEnvelope>, String> {
    let op_ids = op_ids_of(edit);
    let mut envelopes = Vec::with_capacity(edit.ops.len());
    for (index, op) in edit.ops.iter().enumerate() {
        let payload = op.binary.clone().ok_or_else(|| format!("edit {} op {index} has no binary payload", edit.id))?;
        let meta = edit.meta.as_ref().and_then(|metas| metas.get(index));
        let dependencies = meta.map(|m| m.dependencies.iter().cloned().map(MutationId).collect()).unwrap_or_default();
        let actor = meta.and_then(|m| m.author_id.clone()).or_else(|| edit.actor.clone()).unwrap_or_else(|| "unknown".to_string());
        let timestamp = meta.and_then(|m| m.hlt).map(|(actor, physical_ms, logical)| crate::os_spr::HybridLogicalTimestamp { actor, physical_ms: physical_ms as u64, logical }).unwrap_or_else(|| crate::os_spr::HybridLogicalTimestamp::new(0, 0));
        let inverse_payload = edit.inverse.get(index).and_then(|p| p.binary.clone()).unwrap_or_default();
        envelopes.push(MutationEnvelope {
            mutation_id: MutationId(op_ids[index].clone()),
            document_id: ArtifactId(document_id.to_string()),
            actor: ActorId(actor),
            dependencies,
            diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId(schema.to_string()), payload },
            inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId(schema.to_string()), payload: inverse_payload },
            timestamp,
        });
    }
    Ok(envelopes)
}

/// @emoji 📦️ The inverse of {@link envelopes_from_history_edit}: one `MutationEnvelope` -> one
/// `HistoryEdit` with a real binary `OpPayload` and populated meta (`op_id` == the envelope's own
/// id, so a later `op_ids_of` re-read agrees) — the byte-level twin of
/// `crate::os_store::edit_from_operation_envelope`, for appending a locally-flushed envelope to the spr log.
#[cfg(not(target_arch = "wasm32"))]
fn history_edit_from_envelope(envelope: &MutationEnvelope) -> crate::os_spr::HistoryEdit {
    crate::os_spr::HistoryEdit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        started_at: now_ms().to_string(),
        finished_at: None,
        coalesce_key: None,
        description: None,
        ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(envelope.diff.payload.clone()) }],
        inverse: if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![crate::os_spr::OpPayload { text: None, binary: Some(envelope.inverse.payload.clone()) }] },
        meta: Some(vec![crate::os_spr::HistoryOpMeta {
            op_id: Some(envelope.mutation_id.0.clone()),
            dependencies: envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect(),
            base_version: 0,
            author_id: Some(envelope.actor.0.clone()),
            hlt: Some((envelope.timestamp.actor, envelope.timestamp.physical_ms as i64, envelope.timestamp.logical)),
            undo_policy: 0,
            payload_hash: None,
            // 🎞️ `crate::os_spr::causal::MutationEnvelope` (this fn's input) carries no group_id —
            // same precedent as its already-absent `semantic_kind`/`label`/`undo_policy`, see
            // `command::MutationMeta.group_id`'s doc comment. A remote-ingested edit is therefore
            // never a recognized composite-gesture member; group undo degrades it to a foreign/
            // solitary edit, matching how this whole envelope is already `undo_policy: 0`-flattened.
            group_id: None,
        }]),
    }
}

/// @emoji 🔗️ Derives a semio_hub WebSocket URL: `remote://host:port` (or `http(s)://`, `ws(s)://`) →
/// `ws(s)://host:port/spaces/{space_id}/documents/{document_id}/ws`.
fn hub_ws_url(base_url: &str, space_id: &str, document_id: &str) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!("{scheme}://{authority}/spaces/{space_id}/documents/{document_id}/ws")
}
//#endregion 🔖️Endpoints

//#region 🔖️WireBridge
// 🎯️ W6 kernel unification: `to_wire_envelope`/`from_wire_envelope` are DELETED — this actor's
// local envelope shape (`LocalMutations`/`RemoteMutations`/`ChannelBackbone`) and the wire shape
// (`crate::os_spr::wire::ClientFrame::Commands`/`ServerFrame::Commands`) are now the SAME type,
// `crate::os_spr::MutationEnvelope`, throughout: `crate::os_store::BackboneMessage::Mutations` already carries
// it natively (W6's `store` repoint), so there is no local/wire boundary left to bridge across.
// The old local-only fields (`payload_hash`, `schema_version` separate from `diff.schema`) are
// dropped, not replaced — a repo-wide grep confirmed neither was ever read outside this
// now-deleted bridge (`payload_hash` was write-only; `schema_version`/`deps` were only read back
// by these same two functions and two test assertions, both updated alongside this change).

/// @emoji ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed
/// `inverse`, so a semio_hub `Ack::Applied::{Rejected,Transformed}` outcome can roll back (or replace)
/// the local speculative head without a second round trip. This actor stays JSON-payload-typed end
/// to end (never touches `vcs`/`protocol_command`'s typed `Mutation`/`MutationDiff` trait
/// machinery — see the crate doc), so "the inverse machinery" it uses is simply replaying the
/// envelope's own already-computed inverse payload as a synthetic remote operation, the same path
/// {@link ArtifactActor::deliver_remote_operations} already uses for any other remote edit. 🎯️ W6:
/// re-emits `envelope.inverse` as the rollback's forward diff; the rollback's OWN inverse is the
/// original forward diff (inverse-of-inverse) — `crate::os_spr::causal::InverseMutation` carries no
/// `target_mutation`/`base_version`/`dependencies`/`undo_policy` (a deliberately simpler shape
/// than the old kernel-local one), so those are gone, not defaulted.
fn rollback_envelope(envelope: &MutationEnvelope) -> MutationEnvelope {
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    MutationEnvelope {
        mutation_id: undo_id.clone(),
        document_id: envelope.document_id.clone(),
        actor: envelope.actor.clone(),
        dependencies: vec![envelope.mutation_id.clone()],
        diff: crate::os_spr::ArtifactDiff { schema: envelope.inverse.schema.clone(), payload: envelope.inverse.payload.clone() },
        inverse: crate::os_spr::InverseMutation { schema: envelope.diff.schema.clone(), payload: envelope.diff.payload.clone() },
        timestamp: envelope.timestamp,
    }
}

/// @emoji 📡️ `PresencePeer` -> the binary blob `crate::os_spr::wire::ClientFrame::Presence` carries
/// opaquely (`crate::os_spr::encode_presence_peer` — `protocol_wire` has no dependency on
/// this crate's `PresencePeer` type, so the frame only ever moves the pre-encoded bytes).
fn presence_to_bytes(peer: &PresencePeer) -> Vec<u8> {
    crate::os_spr::encode_presence_peer(peer)
}

/// @emoji 📡️ The inverse of {@link presence_to_bytes}, for `ServerFrame::Presence`'s peer roster.
fn presence_from_bytes(bytes: &[u8]) -> Option<PresencePeer> {
    crate::os_spr::decode_presence_peer(bytes).ok()
}

/// @emoji 📡️ Assembles `PresencePeer.interaction` from local `InteractionState` plus each domain's
/// declared hover/selection behavior — the ONE place this logic lives, so every app broadcasts
/// selection+hover with ZERO app-side code (call this wherever a `PresenceHeartbeat`'s `peer` is
/// built, right before `presence_to_bytes`, at the same cadence cursor updates already get
/// throttled at — this fn is pure/stateless, so it invents no throttle of its own). Forwards each
/// domain's already-computed `ids` verbatim (never re-derives a closure itself) — EXPLICIT ids
/// only cross the wire; a receiver re-expands any transitive closure via its own
/// `crate::os_spr::DomainTopology`. Only the `"pointer"` hover channel ever broadcasts (any other
/// channel, e.g. a drag-only one, stays local); a domain whose `HoverSpec::broadcast`/
/// `SelectionSpec::broadcast` is `false` contributes nothing for that half. A domain that ends up
/// with both halves empty is omitted from `domains` entirely.
pub fn assemble_presence_interaction(app_id: &str, state: &crate::os_spr::InteractionState, hover_specs: &std::collections::BTreeMap<String, crate::os_spr::HoverSpec>, selection_specs: &std::collections::BTreeMap<String, crate::os_spr::SelectionSpec>) -> crate::os_spr::PresenceInteraction {
    let mut domain_ids: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
    domain_ids.extend(state.selection.keys());
    domain_ids.extend(state.hover.keys());

    let mut domains = Vec::new();
    for domain_id in domain_ids {
        let selected = if selection_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.selection.get(domain_id).map(|selection| selection.ids.clone()).unwrap_or_default() } else { Vec::new() };
        let hovered = if hover_specs.get(domain_id).is_some_and(|spec| spec.broadcast) {
            state.hover.get(domain_id).filter(|hover| hover.channel == "pointer").map(|hover| hover.ids.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };
        if selected.is_empty() && hovered.is_empty() {
            continue;
        }
        let granularity = state.active_granularity.get(domain_id).cloned().unwrap_or_default();
        domains.push(crate::os_spr::PresenceDomain { domain: domain_id.clone(), granularity, selected, hovered });
    }

    crate::os_spr::PresenceInteraction { app_id: app_id.to_string(), domains }
}

/// @emoji ⏰️ Millisecond wall-clock reads for {@link next_timestamp}: `SystemTime` natively,
/// `js_sys::Date` in the browser wasm build (no `SystemTime` there).
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// @emoji 🧮️ A stable, deterministic `u64` seed for an actor id string, for
/// `crate::os_spr::HybridLogicalTimestamp::actor` (which is `u64`-shaped; this actor's own id is a
/// free-form `String`).
fn actor_seed(actor: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.hash(&mut hasher);
    hasher.finish()
}

/// @emoji ⏰️ Advances `counter` and stamps a fresh {@link crate::os_spr::HybridLogicalTimestamp} for an
/// outbound envelope — freshly stamped on every send (this actor never round-trips a locally-
/// authored envelope's own timestamp back in; a remote-delivered envelope's `timestamp` is simply
/// carried through unchanged).
fn next_timestamp(seed: u64, counter: &mut u64) -> crate::os_spr::HybridLogicalTimestamp {
    *counter = counter.wrapping_add(1);
    crate::os_spr::HybridLogicalTimestamp { actor: seed, physical_ms: now_ms(), logical: *counter }
}
//#endregion 🔖️WireBridge

//#region 🔖️SyncSession
/// @emoji 🔁️ Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Mutation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned + crate::os_store::ArtifactPack,
    Mutation: Clone + serde::Serialize + serde::de::DeserializeOwned + crate::os_spr::Mutation<P> + crate::os_spr::OpBinary + crate::os_spr::OpText,
{
    pub store: ArtifactStore<P, Mutation>,
    cmd_tx: Option<mpsc::UnboundedSender<ArtifactActorMsg>>,
    events: Option<broadcast::Receiver<ArtifactEvent>>,
    status: ArtifactSyncStatus,
}

impl<P, Mutation> SyncSession<P, Mutation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned + crate::os_store::ArtifactPack,
    Mutation: Clone + serde::Serialize + serde::de::DeserializeOwned + crate::os_spr::Mutation<P> + crate::os_spr::OpBinary + crate::os_spr::OpText,
{
    pub fn new(store: ArtifactStore<P, Mutation>) -> Self {
        Self { store, cmd_tx: None, events: None, status: ArtifactSyncStatus::default() }
    }

    /// @emoji 🔌️ Attaches this session's store to a document actor: the actor's `ChannelBackbone` end
    /// is wired into the store, and the command/event channels are retained for wake + status.
    pub fn attach(&mut self, channels: ArtifactChannels, events: broadcast::Receiver<ArtifactEvent>) -> Result<(), SyncError> {
        self.store.attach_backbone(Box::new(channels.channel_backbone)).map_err(|error| SyncError::Vcs(error.to_string()))?;
        self.cmd_tx = Some(channels.cmd_tx);
        self.events = Some(events);
        Ok(())
    }

    /// @emoji ✂️ Detaches from the actor (asking it to flush + stop) and unbinds the store's backbone.
    pub fn detach(&mut self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::Detach);
        }
        self.store.detach_backbone();
        self.cmd_tx = None;
        self.events = None;
    }

    /// @emoji 🔔️ Nudges the actor to drain the store's outbound queue without waiting for its poll tick.
    pub fn wake(&self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        }
    }

    /// @emoji 👻️ Publishes an ephemeral preview blob on the semio_hub's preview lane. See
    /// {@link ArtifactActorMsg::PublishPreview}.
    pub fn publish_preview(&self, key: String, seq: u64, payload: Vec<u8>) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::PublishPreview { key, seq, payload });
        }
    }

    /// @emoji 📥️ Drains any buffered sync status, then pumps the store's inbound backbone queue into
    /// the edit timeline (delegating to `store.tick()`/`pump()`).
    pub fn tick(&mut self) -> Result<bool, SyncError> {
        if let Some(events) = &mut self.events {
            while let Ok(event) = events.try_recv() {
                if let ArtifactEvent::Status(status) = &event {
                    self.status = status.clone();
                }
            }
        }
        self.store.tick().map_err(|error| SyncError::Vcs(error.to_string()))
    }

    /// @emoji 🚦️ The latest sync status seen on the event stream (updated by {@link SyncSession::tick}).
    pub fn status(&self) -> ArtifactSyncStatus {
        self.status.clone()
    }

    /// @emoji 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline. Kept for direct/test injection.
    pub fn receive(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<(), SyncError> {
        self.store.dispatch(crate::os_store::ArtifactCommand::IngestRemote { envelope }).map(|_| ()).map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub fn reconcile_branch(&mut self, alternative_name: &str, message: Option<String>, authors: Vec<vcs::Author>) -> Result<String, SyncError> {
        let mut envelope = self.store.envelope().clone();
        let alternative_id = reconcile_alternative(&mut envelope, alternative_name, message, authors).map_err(|error| SyncError::Vcs(error.to_string()))?;
        let applied = self.store.applied_edit_ids().to_vec();
        let redo = self.store.redo_edit_ids().to_vec();
        self.store.reset(envelope, applied, redo).map_err(|error| SyncError::Vcs(error.to_string()))?;
        Ok(alternative_id)
    }
}
//#endregion 🔖️SyncSession

//#region 🔖️Host
/// @emoji 💓️ Maximum generic host heartbeat frequency. Presence is lossy, last-writer-wins state:
/// callers may offer cursor/viewport/app-presence updates as often as input arrives, while the host
/// publishes only the newest complete peer snapshot at ten hertz.
pub const PRESENCE_HEARTBEAT_INTERVAL_MS: u64 = 100;

/// @emoji 💓️ Per-document last-writer-wins presence producer. The producer owns cadence rather than
/// every renderer/app inventing a timer: offers inside the minimum interval replace `pending`, the
/// first offer publishes immediately, and a later offer publishes the newest complete snapshot.
#[derive(Clone, Debug)]
pub struct PresenceHeartbeatProducer {
    interval_ms: u64,
    last_sent_at_ms: Option<u64>,
    pending: Option<PresencePeer>,
}

impl Default for PresenceHeartbeatProducer {
    fn default() -> Self {
        Self::new(PRESENCE_HEARTBEAT_INTERVAL_MS)
    }
}

impl PresenceHeartbeatProducer {
    pub fn new(interval_ms: u64) -> Self {
        Self { interval_ms: interval_ms.max(1), last_sent_at_ms: None, pending: None }
    }

    /// @emoji 📡️ Offers the newest whole peer snapshot and returns it only when this document's
    /// cadence permits a publish. A backward-moving clock conservatively waits for the next interval.
    pub fn offer(&mut self, now_ms: u64, peer: PresencePeer) -> Option<PresencePeer> {
        self.pending = Some(peer);
        let due = self.last_sent_at_ms.is_none_or(|last| now_ms.saturating_sub(last) >= self.interval_ms);
        if !due {
            return None;
        }
        self.last_sent_at_ms = Some(now_ms);
        self.pending.take()
    }

    pub fn pending(&self) -> Option<&PresencePeer> {
        self.pending.as_ref()
    }
}

/// @emoji 🎛️ The channels {@link ArtifactHost::open} hands back to a caller: attach `channel_backbone`
/// to your `ArtifactStore`, and send control messages (or wakes) on `cmd_tx`.
pub struct ArtifactChannels {
    pub cmd_tx: mpsc::UnboundedSender<ArtifactActorMsg>,
    /// @emoji 🔗️ The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Box::new(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

struct OpenDocument {
    cmd_tx: mpsc::UnboundedSender<ArtifactActorMsg>,
    events: broadcast::Sender<ArtifactEvent>,
    presence: PresenceHeartbeatProducer,
    #[cfg(not(target_arch = "wasm32"))]
    join: Option<std::thread::JoinHandle<()>>,
}

/// @emoji 🏛️ Registry of open per-document actors. One `ArtifactHost` per host process (wgpu native,
/// tests, or the browser wgpu build) owns every open document's actor + event fan-out.
#[derive(Clone, Default)]
pub struct ArtifactHost {
    inner: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, OpenDocument>>>,
}

impl ArtifactHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        self.close(&document_id);
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}"));
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (event_tx, _event_rx) = broadcast::channel(256);
        #[cfg(not(target_arch = "wasm32"))]
        let join = spawn_actor(config, remote, cmd_rx, event_tx.clone());
        #[cfg(target_arch = "wasm32")]
        spawn_actor(config, remote, cmd_rx, event_tx.clone());
        let entry = OpenDocument {
            cmd_tx: cmd_tx.clone(),
            events: event_tx,
            presence: PresenceHeartbeatProducer::default(),
            #[cfg(not(target_arch = "wasm32"))]
            join,
        };
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(document_id, entry);
        ArtifactChannels { cmd_tx, channel_backbone }
    }

    /// @emoji 📬️ A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        let guard = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match guard.get(document_id) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// @emoji 🔔️ Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        if let Some(document) = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(document_id) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// @emoji 💓️ Offers a generic cursor/viewport/app-presence heartbeat for one open document.
    /// Returns `true` only when the host actually queued a publish; faster offers are coalesced onto
    /// the document's producer and cannot flood the preview lane.
    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        let mut documents = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = documents.get_mut(document_id) else { return false };
        let Some(peer) = document.presence.offer(now_ms, peer) else { return false };
        document.cmd_tx.send(ArtifactActorMsg::PresenceHeartbeat { peer }).is_ok()
    }

    /// @emoji ✂️ Stops a document's actor (flushing pending outbound operations first) and, on native, joins
    /// its thread.
    pub fn close(&self, document_id: &str) {
        let document = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(document_id);
        if let Some(document) = document {
            let _ = document.cmd_tx.send(ArtifactActorMsg::Detach);
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(join) = document.join {
                let _ = join.join();
            }
        }
    }

    /// @emoji 🧹️ Ids of every currently-open document.
    pub fn open_artifacts(&self) -> Vec<String> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).keys().cloned().collect()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.inner) > 1 {
            return;
        }
        for document_id in self.open_artifacts() {
            self.close(&document_id);
        }
    }
}
//#endregion 🔖️Host

//#region 🔖️NativeActor
#[cfg(not(target_arch = "wasm32"))]
mod native_actor {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};
    use std::time::Duration;
    use tokio::time::Instant;
    use tokio_tungstenite::tungstenite::Message;

    type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
    type WsSink = futures_util::stream::SplitSink<WsStream, Message>;
    type WsRead = futures_util::stream::SplitStream<WsStream>;

    struct HubConn {
        write: WsSink,
        read: WsRead,
    }

    /// @emoji 📁️ A folder/file binding's storage driver, keyed for multi-document sqlite or single
    /// pack-backed blob. Both variants move real `pack`+`spr` bytes end to end — this actor never
    /// touches JSON. `Sqlite` is fully codec-free (its row IS the pack+spr pair, no bridging
    /// needed). `Pack` (file) needs a codec ONLY for the `.dsl`/`.ops` text mirrors — logging, not
    /// truth — and for the hand-authored/imported fallback when no `.pack` exists yet
    /// (`compile_dsl`); a missing codec there degrades to pack+spr-only persistence (mirrors
    /// skipped, with a caller-visible warning), never a hard failure, since the mirrors are not
    /// load-bearing.
    enum FolderEndpoint {
        Sqlite { storage: FolderSqliteStorage, document_id: String, schema: String },
        Pack { storage: FolderTextStorage, document_id: String, extension: String, schema: String },
    }

    impl FolderEndpoint {
        /// @emoji 📖️ `Ok(None)` = nothing persisted yet; `Ok(Some(pack, spr))` = the authoritative
        /// binary pair, resolved pack-first (falling back to compiling the DSL mirror for
        /// hand-authored/imported documents with no `.pack` file yet — `Sqlite` has no such
        /// fallback, a row is always written pack+spr together); `Err` = a real storage failure.
        fn read(&self) -> Result<Option<(Vec<u8>, Vec<u8>)>, String> {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, .. } => storage.read(document_id).map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    if let Some(pack_files) = storage.read_pack(document_id, extension).map_err(|error| error.to_string())? {
                        return Ok(Some((pack_files.pack, pack_files.spr)));
                    }
                    let Some(text_files) = storage.read(document_id, extension).map_err(|error| error.to_string())? else {
                        return Ok(None);
                    };
                    let Some(codec) = crate::os_store::document_codec(schema) else {
                        return Err(format!("no document codec registered for schema {schema:?} — cannot compile the DSL-only fallback"));
                    };
                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).map_err(|error| error.to_string())?;
                    Ok(Some((pack_files.pack, pack_files.spr)))
                }
            }
        }

        /// @emoji ✍️ Persists the authoritative `pack`+`spr` pair. `Sqlite` needs no codec at all;
        /// `Pack` additionally writes the `.dsl`/`.ops` logging mirrors when a codec is registered
        /// (silently skipped otherwise — the pack+spr write below already succeeded).
        fn write(&self, pack: &[u8], spr: &[u8]) -> Result<(), String> {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, schema } => storage.write(document_id, schema, pack, spr).map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    let Some(codec) = crate::os_store::document_codec(schema) else {
                        let pack_files = crate::os_store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() };
                        return storage.write_pack(document_id, extension, &pack_files, "").map_err(|error| error.to_string());
                    };
                    let mirror = (codec.print_mirror)(pack, spr).map_err(|error| error.to_string())?;
                    let pack_files = crate::os_store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: mirror.ops };
                    storage.write_pack(document_id, extension, &pack_files, &mirror.dsl).map_err(|error| error.to_string())
                }
            }
        }
    }

    /// @emoji 🎭️ One document's backbone actor: drains the store's outbound queue to persist + relay,
    /// ingests semio_hub/file changes back into the store, and keeps subscribers current with status/events.
    pub(super) struct ArtifactActor {
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<ArtifactEvent>,
        cmd_rx: mpsc::UnboundedReceiver<ArtifactActorMsg>,
        folder: Option<FolderEndpoint>,
        folder_watch_path: Option<PathBuf>,
        watch_external: bool,
        hub_base_url: Option<String>,
        hub_space_id: Option<String>,
        hub_token: Option<String>,
        semio_hub: Option<HubConn>,
        /// @emoji 🏔️ Last frontier the semio_hub reported (`Welcome.server_frontier` / `Commands.frontier` /
        /// `Ack.frontier`) — the wire-v2 replacement for the old `hub_version: i64` counter.
        server_frontier: Option<crate::os_spr::RuntimeFrontierSummary>,
        /// @emoji 🎟️ The semio_hub's last `Welcome.resume_token`, echoed back on the next `Hello` after a
        /// reconnect so the semio_hub can resume rather than replay from scratch.
        resume_token: Option<String>,
        backoff_ms: u64,
        reconnect_at: Option<Instant>,
        /// @emoji 🧺️ Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`, so `Rejected`/
        /// `Transformed` can roll back exactly the envelopes that batch sent.
        pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
        next_batch_id: u64,
        /// @emoji ⏰️ This actor's `HybridLogicalTimestamp` seed (derived from `actor`) + logical tick
        /// counter, for {@link next_timestamp} on every outbound wire envelope.
        hlc_seed: u64,
        hlc_counter: u64,
        current_pack: Option<Vec<u8>>,
        current_spr: Option<Vec<u8>>,
        known_op_ids: HashSet<String>,
        last_written_hash: Option<String>,
        remote_state: RemoteState,
        last_status: Option<ArtifactSyncStatus>,
        watcher: Option<notify::RecommendedWatcher>,
        fs_rx: Option<mpsc::UnboundedReceiver<()>>,
        fs_deadline: Option<Instant>,
    }

    impl ArtifactActor {
        pub(super) fn new(config: ArtifactActorConfig, remote: ChannelBackboneRemote, cmd_rx: mpsc::UnboundedReceiver<ArtifactActorMsg>, events: broadcast::Sender<ArtifactEvent>) -> Self {
            let mut folder = None;
            let mut folder_watch_path = None;
            let mut hub_base_url = None;
            let mut hub_space_id = None;
            let mut hub_token = None;
            for binding in &config.bindings {
                match binding {
                    PersistenceBinding::Folder { path } => {
                        if folder.is_none() {
                            folder = Some(build_folder_endpoint(path, &config.document_id, &config.schema));
                            folder_watch_path = Some(folder_watch_path_for(path));
                        }
                    }
                    PersistenceBinding::Hub { base_url, space_id, token } => {
                        if hub_base_url.is_none() {
                            hub_base_url = Some(base_url.clone());
                            hub_space_id = Some(space_id.clone());
                            hub_token = token.clone();
                        }
                    }
                }
            }
            let hlc_seed = actor_seed(&config.actor);
            Self {
                document_id: config.document_id,
                schema: config.schema,
                actor: config.actor,
                remote,
                events,
                cmd_rx,
                folder,
                folder_watch_path,
                watch_external: config.watch_external,
                hub_base_url,
                hub_space_id,
                hub_token,
                semio_hub: None,
                server_frontier: None,
                resume_token: None,
                backoff_ms: 500,
                reconnect_at: None,
                pending_batches: std::collections::HashMap::new(),
                next_batch_id: 0,
                hlc_seed,
                hlc_counter: 0,
                current_pack: None,
                current_spr: None,
                known_op_ids: HashSet::new(),
                last_written_hash: None,
                remote_state: RemoteState::Detached,
                last_status: None,
                watcher: None,
                fs_rx: None,
                fs_deadline: None,
            }
        }

        pub(super) async fn run(mut self) {
            self.setup();
            self.try_connect_hub().await;
            let mut poll = tokio::time::interval(Duration::from_millis(25));
            poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                let reconnect_at = self.reconnect_at;
                let fs_deadline = self.fs_deadline;
                tokio::select! {
                    biased;
                    cmd = self.cmd_rx.recv() => {
                        match cmd {
                            None => { break; }
                            Some(message) => {
                                if self.handle_cmd(message).await {
                                    break;
                                }
                            }
                        }
                    }
                    message = hub_next(&mut self.semio_hub), if self.semio_hub.is_some() => {
                        self.on_hub_message(message).await;
                    }
                    changed = fs_next(&mut self.fs_rx), if self.fs_rx.is_some() => {
                        if changed.is_some() {
                            self.fs_deadline = Some(Instant::now() + Duration::from_millis(200));
                        }
                    }
                    _ = sleep_opt(reconnect_at), if reconnect_at.is_some() => {
                        self.reconnect_at = None;
                        self.try_connect_hub().await;
                    }
                    _ = sleep_opt(fs_deadline), if fs_deadline.is_some() => {
                        self.fs_deadline = None;
                        self.handle_external_change();
                    }
                    _ = poll.tick() => {
                        self.drain_and_relay().await;
                        self.emit_status_if_changed();
                    }
                }
            }
        }

        /// @emoji 🌱️ Seeds persistence state from any already-stored pack+spr and installs the file watcher.
        fn setup(&mut self) {
            if let Some((pack, spr)) = self.folder.as_ref().and_then(|folder| folder.read().ok().flatten()) {
                if let Ok(op_ids) = spr_op_ids(&spr) {
                    self.known_op_ids = op_ids;
                    self.last_written_hash = Some(backbone_pack_hash(&pack, &spr));
                    self.current_pack = Some(pack);
                    self.current_spr = Some(spr);
                }
            }
            if self.watch_external {
                if let Some(watch_path) = self.folder_watch_path.clone() {
                    if let Some((watcher, fs_rx)) = install_watcher(&watch_path) {
                        self.watcher = Some(watcher);
                        self.fs_rx = Some(fs_rx);
                    }
                }
            }
        }

        /// @emoji 📨️ Handles a caller control message. Returns `true` when the actor should stop.
        async fn handle_cmd(&mut self, message: ArtifactActorMsg) -> bool {
            match message {
                ArtifactActorMsg::LocalMutations { envelopes } => {
                    let drained = self.drain_and_relay().await;
                    if !drained && !envelopes.is_empty() {
                        self.persist_operations(&envelopes);
                        self.relay_operations_to_hub(&envelopes).await;
                    }
                    false
                }
                ArtifactActorMsg::PresenceHeartbeat { peer } => {
                    self.send_client_frame(ClientFrame::Presence { peer: presence_to_bytes(&peer) }, Lane::Preview).await;
                    false
                }
                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_client_frame(ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                    false
                }
                ArtifactActorMsg::ExternalChanged => {
                    self.handle_external_change();
                    false
                }
                ArtifactActorMsg::Detach => {
                    self.drain_and_relay().await;
                    true
                }
            }
        }

        /// @emoji 📤️ Drains the store's outbound queue, persisting + relaying each message. Returns
        /// whether anything was drained.
        async fn drain_and_relay(&mut self) -> bool {
            let messages = self.remote.drain().unwrap_or_default();
            let drained = !messages.is_empty();
            for message in messages {
                match message {
                    BackboneMessage::Mutations { envelopes } => {
                        let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                        self.persist_operations(&envelopes);
                        self.relay_operations_to_hub(&envelopes).await;
                    }
                    BackboneMessage::Snapshot { pack, spr } => {
                        self.persist_snapshot(pack, spr);
                        // 📸️ No client -> semio_hub whole-envelope push exists in wire v2
                        // (`crate::os_spr::wire::ClientFrame` has no snapshot-put variant — only
                        // causally-ordered `Commands`; the semio_hub -> client snapshot direction is
                        // `Bootstrap::Snapshot`/`SnapshotChunk`/`SnapshotDone`, download-only). The
                        // folder binding above still persists this snapshot; relaying a structural
                        // snapshot to the semio_hub is a CW6+ semio_hub-rebuild concern (documented deferral in
                        // the CW5 report, not a bug in this actor).
                    }
                    BackboneMessage::Ack { .. } => {}
                }
            }
            drained
        }

        //#region 🔖️Folder
        /// @emoji ✍️ Persists the current pack+spr bytes to the folder binding and records the
        /// content hash for self-write suppression. A write failure (e.g. no `crate::os_store::ArtifactCodec`
        /// registered for this document's schema on the `Pack` endpoint — see `FolderEndpoint::write`)
        /// is swallowed here the same way every other best-effort path in this actor already is, but
        /// deliberately does NOT record `last_written_hash` on failure — a false "persisted" mark
        /// would make `handle_external_change` mistake the still-stale on-disk content for a
        /// self-write and ignore a real external change.
        fn persist_write(&mut self, pack: &[u8], spr: &[u8]) {
            let Some(folder) = self.folder.as_ref() else { return };
            if folder.write(pack, spr).is_ok() {
                self.last_written_hash = Some(backbone_pack_hash(pack, spr));
            }
        }

        /// @emoji 📸️ Records a full pack+spr snapshot as the canonical persisted state.
        fn persist_snapshot(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
            if self.folder.is_none() {
                return;
            }
            if let Ok(op_ids) = spr_op_ids(&spr) {
                self.known_op_ids = op_ids;
            }
            self.persist_write(&pack, &spr);
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
        }

        /// @emoji ➕️ Appends locally-applied operations to the persisted spr log (append-only),
        /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
        fn persist_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if self.folder.is_none() {
                return;
            }
            let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
            let new_edits: Vec<crate::os_spr::HistoryEdit> = envelopes.iter().filter(|envelope| self.known_op_ids.insert(envelope.mutation_id.0.clone())).map(history_edit_from_envelope).collect();
            if new_edits.is_empty() {
                return;
            }
            let Ok(new_spr) = crate::os_store::append_history_edits_to_spr(&spr, &new_edits) else { return };
            self.persist_write(&pack, &new_spr);
            self.current_pack = Some(pack);
            self.current_spr = Some(new_spr);
        }

        /// @emoji 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`,
        /// divergence → `SnapshotReplaced`, divergence with local pending operations → `Conflict`. Self-writes
        /// (content hash match) are ignored.
        fn handle_external_change(&mut self) {
            let Some((pack, spr)) = self.folder.as_ref().and_then(|folder| folder.read().ok().flatten()) else { return };
            let hash = backbone_pack_hash(&pack, &spr);
            if self.last_written_hash.as_deref() == Some(hash.as_str()) {
                return;
            }
            let Ok(file_ids) = spr_op_ids(&spr) else { return };
            let lost: Vec<String> = self.known_op_ids.difference(&file_ids).cloned().collect();
            let new_ids: HashSet<String> = file_ids.difference(&self.known_op_ids).cloned().collect();

            if lost.is_empty() && !new_ids.is_empty() {
                let Ok(reader) = crate::os_spr::HistoryReader::open(&spr, &crate::os_spr::DecodeOptions::default()) else { return };
                let mut appended = Vec::new();
                for edit in reader.edits() {
                    let Ok(edit) = edit else { break };
                    if op_ids_of(&edit).iter().any(|id| new_ids.contains(id)) {
                        if let Ok(mut envelopes) = envelopes_from_history_edit(&edit, &self.document_id, &self.schema) {
                            appended.append(&mut envelopes);
                        }
                    }
                }
                self.known_op_ids.extend(new_ids);
                self.current_pack = Some(pack);
                self.current_spr = Some(spr);
                self.last_written_hash = Some(hash);
                self.deliver_remote_operations(appended);
            } else if !lost.is_empty() {
                if !self.pending_batches.is_empty() {
                    self.emit(ArtifactEvent::Conflict(SpaceConflict { kind: "externalDivergence".into(), uri: format!("folder://{}", self.document_id), message: "external history diverged while local operations are pending".into() }));
                } else {
                    self.known_op_ids = file_ids;
                    self.current_pack = Some(pack.clone());
                    self.current_spr = Some(spr.clone());
                    self.last_written_hash = Some(hash);
                    self.deliver_snapshot(pack, spr);
                }
            }
        }
        //#endregion 🔖️Folder

        //#region 🔖️Hub
        async fn try_connect_hub(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            let space_id = self.hub_space_id.clone().unwrap_or_default();
            let token = self.hub_token.clone();
            let url = hub_ws_url(&base_url, &space_id, &self.document_id);
            self.set_remote_state(RemoteState::Connecting);
            match tokio_tungstenite::connect_async(url).await {
                Ok((stream, _response)) => {
                    let (write, read) = stream.split();
                    self.semio_hub = Some(HubConn { write, read });
                    self.backoff_ms = 500;
                    let hello = ClientFrame::Hello {
                        wire_version: 1,
                        protocol_version: 1,
                        schema: self.schema.clone(),
                        // 🔖️ No schema pack hashing wired into this client-side actor yet (db/pack
                        // integration is a CW6+ semio_hub-rebuild concern) — the semio_hub is JSON-only until
                        // then anyway, so this placeholder is never validated this wave.
                        pack_schema_hash: [0u8; 32],
                        actor: ActorId(self.actor.clone()),
                        token,
                        resume_token: self.resume_token.clone(),
                        frontier: self.server_frontier.clone(),
                    };
                    self.send_client_frame(hello, Lane::Command).await;
                }
                Err(_error) => {
                    self.schedule_reconnect();
                }
            }
        }

        fn schedule_reconnect(&mut self) {
            let retry = self.backoff_ms;
            self.set_remote_state(RemoteState::Backoff { retry_in_ms: retry });
            self.reconnect_at = Some(Instant::now() + Duration::from_millis(retry));
            self.backoff_ms = (self.backoff_ms * 2).min(30_000);
        }

        async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
            match message {
                Some(Ok(Message::Binary(bytes))) => {
                    if let Ok((_lane, frame)) = decode_server_frame(&bytes) {
                        self.on_hub_frame(frame);
                    }
                }
                Some(Ok(Message::Ping(payload))) => {
                    self.send_raw(Message::Pong(payload)).await;
                }
                Some(Ok(_)) => {}
                Some(Err(_)) | None => {
                    self.semio_hub = None;
                    self.schedule_reconnect();
                }
            }
        }

        fn on_hub_frame(&mut self, frame: ServerFrame) {
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.resume_token = Some(resume_token);
                    self.server_frontier = Some(server_frontier);
                    // 📡️ `Welcome` no longer carries a presence roster (wire v2 splits it into its own
                    // `ServerFrame::Presence`) — `peer_count` is corrected once that frame arrives.
                    self.set_remote_state(RemoteState::Live { peer_count: 0 });
                    match bootstrap {
                        Bootstrap::None | Bootstrap::Tail => {}
                        Bootstrap::Snapshot { .. } => {
                            // 📦️ Pack-based snapshot bootstrap: no client-side pack decoder wired into
                            // this actor this wave (db/pack integration is a CW6+ semio_hub-rebuild concern)
                            // — accepted and ignored rather than erroring; catch-up instead relies on
                            // the semio_hub's follow-up `Commands` frame(s) once CW6 lands.
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                    // 📦️ See the `Bootstrap::Snapshot` note above — accepted and ignored.
                }
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    self.server_frontier = Some(frontier);
                    if origin != ActorId(self.actor.clone()) {
                        let converted = envelopes;
                        self.persist_operations(&converted);
                        self.deliver_remote_operations(converted);
                    }
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages);
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if actor != ActorId(self.actor.clone()) {
                        self.emit(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let peers: Vec<PresencePeer> = peers.iter().filter_map(|p| presence_from_bytes(p)).collect();
                    self.set_remote_state(RemoteState::Live { peer_count: peers.len() });
                    self.emit(ArtifactEvent::Presence { peers });
                }
                ServerFrame::CreditGrant { .. } => {
                    // 🪙️ Command-lane credit-based flow control: no client-side backpressure
                    // implemented this wave (scope is frame plumbing, not congestion control) —
                    // accepted and ignored.
                }
                ServerFrame::Error { code, message } => {
                    self.emit(ArtifactEvent::Conflict(SpaceConflict { kind: code, uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
            }
        }

        /// @emoji 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`
        /// just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
        /// local head first (via {@link rollback_envelope}, replayed as remote operations), and
        /// `Transformed` then delivers the semio_hub's replacement envelope the same way.
        fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let rollbacks: Vec<MutationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.persist_operations(&rollbacks);
                        self.deliver_remote_operations(rollbacks);
                        let converted = *envelope;
                        self.persist_operations(std::slice::from_ref(&converted));
                        self.deliver_remote_operations(vec![converted]);
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason } => {
                        let rollbacks: Vec<MutationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.persist_operations(&rollbacks);
                        self.deliver_remote_operations(rollbacks);
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason } });
                    }
                }
            }
            self.emit_status_if_changed();
        }

        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            if self.semio_hub.is_none() || envelopes.is_empty() {
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let wire_envelopes: Vec<crate::os_spr::MutationEnvelope> = envelopes.iter().map(|envelope| crate::os_spr::MutationEnvelope { timestamp: next_timestamp(self.hlc_seed, &mut self.hlc_counter), ..envelope.clone() }).collect();
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_client_frame(ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
            self.emit_status_if_changed();
        }

        async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(&frame, lane);
            self.send_raw(Message::Binary(bytes.into())).await;
        }

        async fn send_raw(&mut self, message: Message) {
            let mut failed = false;
            if let Some(conn) = self.semio_hub.as_mut() {
                if conn.write.send(message).await.is_err() {
                    failed = true;
                }
            }
            if failed {
                self.semio_hub = None;
                self.schedule_reconnect();
            }
        }
        //#endregion 🔖️Hub

        //#region 🔖️Deliver
        /// @emoji 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
        fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) {
            if envelopes.is_empty() {
                return;
            }
            let _ = self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) });
            self.emit(ArtifactEvent::RemoteMutations { envelopes });
        }

        /// @emoji 📸️ Pushes a full pack+spr snapshot into the store's inbound queue and notifies subscribers.
        fn deliver_snapshot(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
            let _ = self.remote.push(BackboneMessage::Snapshot { pack: pack.clone(), spr: spr.clone() });
            self.emit(ArtifactEvent::SnapshotReplaced { pack, spr });
        }

        fn emit(&self, event: ArtifactEvent) {
            let _ = self.events.send(event);
        }

        fn status(&self) -> ArtifactSyncStatus {
            ArtifactSyncStatus { persisted: self.last_written_hash.is_some() || self.server_frontier.is_some(), pending_mutations: self.pending_batches.values().map(Vec::len).sum(), remote: self.remote_state.clone() }
        }

        fn set_remote_state(&mut self, state: RemoteState) {
            self.remote_state = state;
            self.emit_status_if_changed();
        }

        fn emit_status_if_changed(&mut self) {
            let status = self.status();
            if self.last_status.as_ref() != Some(&status) {
                self.last_status = Some(status.clone());
                self.emit(ArtifactEvent::Status(status));
            }
        }
        //#endregion 🔖️Deliver
    }

    /// @emoji 🔀️ A binding path with a file extension addresses one document's text blob directly
    /// (`Text`, generalizing the deleted single-file `FileJsonStorage` beyond `.json`); an extensionless
    /// directory path is the canonical multi-document sqlite store (`Sqlite`).
    fn build_folder_endpoint(path: &std::path::Path, document_id: &str, schema: &str) -> FolderEndpoint {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(extension) => {
                let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
                FolderEndpoint::Pack { storage: FolderTextStorage::new(folder), document_id: document_id.to_string(), extension: extension.to_string(), schema: schema.to_string() }
            }
            None => FolderEndpoint::Sqlite { storage: FolderSqliteStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() },
        }
    }

    /// @emoji 📍️ The on-disk path a folder binding writes to: the `<document_id>.<extension>` text blob
    /// itself, or the multi-document sqlite db under `<folder>/.semio/documents.db`.
    fn folder_watch_path_for(path: &Path) -> PathBuf {
        if path.extension().is_some() {
            path.to_path_buf()
        } else {
            path.join(".semio").join("documents.db")
        }
    }

    /// @emoji 👁️ Installs a `notify` watcher over the binding's on-disk directory, forwarding raw
    /// change events into an async channel (debounced by the actor's 200ms deadline).
    fn install_watcher(watch_path: &Path) -> Option<(notify::RecommendedWatcher, mpsc::UnboundedReceiver<()>)> {
        use notify::Watcher;
        let watch_root = watch_path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| watch_path.to_path_buf());
        let _ = std::fs::create_dir_all(&watch_root);
        let (tx, rx) = mpsc::unbounded_channel();
        let mut watcher = notify::recommended_watcher(move |result: Result<notify::Event, notify::Error>| {
            if result.is_ok() {
                let _ = tx.send(());
            }
        })
        .ok()?;
        watcher.watch(&watch_root, notify::RecursiveMode::NonRecursive).ok()?;
        Some((watcher, rx))
    }

    async fn hub_next(conn: &mut Option<HubConn>) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        match conn {
            Some(conn) => conn.read.next().await,
            None => std::future::pending().await,
        }
    }

    async fn fs_next(rx: &mut Option<mpsc::UnboundedReceiver<()>>) -> Option<()> {
        match rx {
            Some(rx) => rx.recv().await,
            None => std::future::pending().await,
        }
    }

    async fn sleep_opt(deadline: Option<Instant>) {
        match deadline {
            Some(deadline) => tokio::time::sleep_until(deadline).await,
            None => std::future::pending().await,
        }
    }

    /// @emoji 🚀️ Spawns a dedicated OS thread running a current-thread tokio runtime that drives the actor.
    pub(super) fn spawn_actor(config: ArtifactActorConfig, remote: ChannelBackboneRemote, cmd_rx: mpsc::UnboundedReceiver<ArtifactActorMsg>, events: broadcast::Sender<ArtifactEvent>) -> Option<std::thread::JoinHandle<()>> {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().ok()?;
        std::thread::Builder::new()
            .name(format!("sync-actor-{}", config.document_id))
            .spawn(move || {
                let actor = ArtifactActor::new(config, remote, cmd_rx, events);
                runtime.block_on(actor.run());
            })
            .ok()
    }
}

#[cfg(not(target_arch = "wasm32"))]
use native_actor::spawn_actor;
//#endregion 🔖️NativeActor

//#region 🔖️WasmActor
/// @emoji 🌐️ Browser wgpu build: the actor runs on `spawn_local` with a `web_sys::WebSocket` semio_hub
/// transport. No filesystem, so folder bindings are ignored (the browser uses the dev-middleware
/// SSE watch instead, wired by WS-E's TS twin). Kept coherent so a future in-wasm host can link it.
#[cfg(target_arch = "wasm32")]
mod wasm_actor {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, MessageEvent, WebSocket};

    struct WasmActor {
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<ArtifactEvent>,
        hub_base_url: Option<String>,
        hub_space_id: Option<String>,
        hub_token: Option<String>,
        ws: Option<WebSocket>,
        server_frontier: Option<crate::os_spr::RuntimeFrontierSummary>,
        resume_token: Option<String>,
        pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
        next_batch_id: u64,
        hlc_seed: u64,
        hlc_counter: u64,
        incoming_tx: mpsc::UnboundedSender<Vec<u8>>,
        _closures: Vec<Closure<dyn FnMut(MessageEvent)>>,
        _open_closures: Vec<Closure<dyn FnMut()>>,
    }

    impl WasmActor {
        fn connect(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            let space_id = self.hub_space_id.clone().unwrap_or_default();
            let url = hub_ws_url(&base_url, &space_id, &self.document_id);
            let Ok(ws) = WebSocket::new(&url) else { return };
            ws.set_binary_type(BinaryType::Arraybuffer);

            let incoming = self.incoming_tx.clone();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(buffer) = event.data().dyn_ref::<js_sys::ArrayBuffer>() {
                    let _ = incoming.send(js_sys::Uint8Array::new(buffer).to_vec());
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            self._closures.push(onmessage);

            let hello = ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: self.schema.clone(),
                // 🔖️ See the native actor's matching note in `try_connect_hub` — no client-side
                // schema pack hashing wired this wave, the semio_hub is JSON-only until CW6 anyway.
                pack_schema_hash: [0u8; 32],
                actor: ActorId(self.actor.clone()),
                token: self.hub_token.clone(),
                resume_token: self.resume_token.clone(),
                frontier: self.server_frontier.clone(),
            };
            let mut hello_bytes = encode_client_frame(&hello, Lane::Command);
            let ws_for_open = ws.clone();
            let onopen = Closure::wrap(Box::new(move || {
                let _ = ws_for_open.send_with_u8_array(&mut hello_bytes);
            }) as Box<dyn FnMut()>);
            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            self._open_closures.push(onopen);

            self.ws = Some(ws);
        }

        fn send_frame(&self, frame: &ClientFrame, lane: Lane) {
            if let Some(ws) = &self.ws {
                let mut bytes = encode_client_frame(frame, lane);
                let _ = ws.send_with_u8_array(&mut bytes);
            }
        }

        /// @emoji 🧺️ Builds + sends one `Commands` batch, tracking it in `pending_batches` for
        /// {@link WasmActor::handle_ack}. Mirrors the native actor's `relay_operations_to_hub`.
        fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let wire_envelopes: Vec<crate::os_spr::MutationEnvelope> = envelopes.iter().map(|envelope| crate::os_spr::MutationEnvelope { timestamp: next_timestamp(self.hlc_seed, &mut self.hlc_counter), ..envelope.clone() }).collect();
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_frame(&ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command);
        }

        fn drain_and_relay(&mut self) -> bool {
            let messages = self.remote.drain().unwrap_or_default();
            let drained = !messages.is_empty();
            for message in messages {
                match message {
                    BackboneMessage::Mutations { envelopes } => {
                        let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                        self.relay_operations(&envelopes);
                    }
                    BackboneMessage::Snapshot { .. } => {
                        // 📸️ No client -> semio_hub whole-envelope push in wire v2 — see the native actor's
                        // matching note in `drain_and_relay` (native_actor module, above).
                    }
                    BackboneMessage::Ack { .. } => {}
                }
            }
            drained
        }

        fn handle_cmd(&mut self, message: ArtifactActorMsg) {
            match message {
                ArtifactActorMsg::LocalMutations { envelopes } => {
                    let drained = self.drain_and_relay();
                    if !drained && !envelopes.is_empty() {
                        self.relay_operations(&envelopes);
                    }
                }
                ArtifactActorMsg::PresenceHeartbeat { peer } => {
                    self.send_frame(&ClientFrame::Presence { peer: presence_to_bytes(&peer) }, Lane::Preview);
                }
                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview);
                }
                ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
            }
        }

        fn on_binary(&mut self, bytes: &[u8]) {
            let Ok((_lane, frame)) = decode_server_frame(bytes) else { return };
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.resume_token = Some(resume_token);
                    self.server_frontier = Some(server_frontier);
                    match bootstrap {
                        Bootstrap::None | Bootstrap::Tail => {}
                        // 📦️ See the native actor's matching `Bootstrap::Snapshot` note — no
                        // client-side pack decoder wired this wave, accepted and ignored.
                        Bootstrap::Snapshot { .. } => {}
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {}
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    self.server_frontier = Some(frontier);
                    if origin != ActorId(self.actor.clone()) {
                        let converted = envelopes;
                        self.deliver_remote_operations(converted);
                    }
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages);
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if actor != ActorId(self.actor.clone()) {
                        let _ = self.events.send(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let peers: Vec<PresencePeer> = peers.iter().filter_map(|p| presence_from_bytes(p)).collect();
                    let _ = self.events.send(ArtifactEvent::Presence { peers });
                }
                ServerFrame::CreditGrant { .. } => {}
                ServerFrame::Error { code, message } => {
                    let _ = self.events.send(ArtifactEvent::Conflict(SpaceConflict { kind: code, uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
            }
        }

        /// @emoji 📮️ Mirrors the native actor's `handle_ack` — see its doc comment.
        fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let rollbacks: Vec<MutationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.deliver_remote_operations(rollbacks);
                        let converted = *envelope;
                        self.deliver_remote_operations(vec![converted]);
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason } => {
                        let rollbacks: Vec<MutationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.deliver_remote_operations(rollbacks);
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason } });
                    }
                }
            }
        }

        fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) {
            if envelopes.is_empty() {
                return;
            }
            let _ = self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) });
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
        }
    }

    pub(super) fn spawn_actor(config: ArtifactActorConfig, remote: ChannelBackboneRemote, mut cmd_rx: mpsc::UnboundedReceiver<ArtifactActorMsg>, events: broadcast::Sender<ArtifactEvent>) {
        let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let mut hub_base_url = None;
        let mut hub_space_id = None;
        let mut hub_token = None;
        for binding in &config.bindings {
            if let PersistenceBinding::Hub { base_url, space_id, token } = binding {
                if hub_base_url.is_none() {
                    hub_base_url = Some(base_url.clone());
                    hub_space_id = Some(space_id.clone());
                    hub_token = token.clone();
                }
            }
        }
        let hlc_seed = actor_seed(&config.actor);
        let mut actor = WasmActor {
            document_id: config.document_id,
            schema: config.schema,
            actor: config.actor,
            remote,
            events,
            hub_base_url,
            hub_space_id,
            hub_token,
            ws: None,
            server_frontier: None,
            resume_token: None,
            pending_batches: std::collections::HashMap::new(),
            next_batch_id: 0,
            hlc_seed,
            hlc_counter: 0,
            incoming_tx,
            _closures: Vec::new(),
            _open_closures: Vec::new(),
        };
        wasm_bindgen_futures::spawn_local(async move {
            actor.connect();
            loop {
                tokio::select! {
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            None => break,
                            Some(ArtifactActorMsg::Detach) => { actor.drain_and_relay(); break; }
                            Some(message) => actor.handle_cmd(message),
                        }
                    }
                    bytes = incoming_rx.recv() => {
                        match bytes {
                            Some(bytes) => actor.on_binary(&bytes),
                            None => break,
                        }
                    }
                }
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_actor::spawn_actor;
//#endregion 🔖️WasmActor

//#region 🔖️Fixtures
/// @emoji 🎬️ A scripted actor test vector shared by cargo test (here) and vitest (WS-E's TS twin).
/// Each fixture drives inbound events at a document actor and asserts the resulting `ArtifactEvent`
/// sequence and the final persisted envelope edit ids. See `framework/sync/fixtures/README.md`.
#[derive(Clone, Debug)]
pub struct ActorFixture {
    pub name: String,
    pub schema: String,
    pub document_id: String,
    /// @emoji 📥️ Inbound stimulus applied to the actor, in order.
    pub inbound: Vec<FixtureInbound>,
    /// @emoji 📤️ The `ArtifactEvent` variant tags expected on the subscriber channel, in order.
    pub expected_events: Vec<String>,
    /// @emoji 📇️ Edit ids expected in the document's timeline after replay.
    pub expected_edit_ids: Vec<String>,
}

/// @emoji 📥️ One scripted inbound stimulus: either a semio_hub server frame or an external folder edit.
/// Document/op CONTENT lives in sibling text files (never JSON) — this is the LOADED shape
/// (content already read off disk); see `RawFixtureInbound` for the on-disk manifest shape that
/// only references filenames.
#[derive(Clone, Debug)]
pub enum FixtureInbound {
    /// @emoji 📬️ A raw `crate::os_spr::wire::ServerFrame`'s encoded bytes (`crate::os_spr::encode_server_frame`
    /// output, `lane` byte included), delivered as if received over the semio_hub WebSocket — already
    /// real binary, not document/op content, so it stays inline in the manifest as a JSON number
    /// array. Driven by `🟦️backbone-worker.ts`'s TS fallback vitest harness (which decodes these
    /// bytes with its own binary decoder); the folder-only Rust harness skips these.
    HubFrame { frame_bytes: Vec<u8> },
    /// @emoji 📁️ An external folder edit: `.ops`-grammar text (one or more `edit ...` blocks) to
    /// append to the spr log out-of-band.
    ExternalEdits { ops_text: String },
    /// @emoji ♻️ An external whole-document rewrite (divergent history): dsl + ops text compiled
    /// via `codec.compile_dsl` and written in place of the stored document.
    ReplaceDocument { dsl_text: String, ops_text: String },
}

/// @emoji 📄️ The on-disk manifest shape: `kind`-tagged like `FixtureInbound`, but content-bearing
/// variants reference a sibling filename (relative to the fixture's own directory) instead of
/// carrying the text inline — `load_fixtures` resolves these into real `FixtureInbound`s.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum RawFixtureInbound {
    HubFrame { frame_bytes: Vec<u8> },
    ExternalEdits { ops_file: String },
    ReplaceDocument { dsl_file: String, ops_file: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureManifest {
    name: String,
    schema: String,
    document_id: String,
    inbound: Vec<RawFixtureInbound>,
    expected_events: Vec<String>,
    expected_edit_ids: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_fixture_dsl_manifest(text: &str) -> Option<FixtureManifest> {
    use std::collections::BTreeMap;

    let mut name = None;
    let mut schema = None;
    let mut document_id = None;
    let mut expected_events = Vec::new();
    let mut expected_edit_ids = Vec::new();
    let mut inbound_fields: BTreeMap<(usize, String), String> = BTreeMap::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=')?;
        let value = value.trim().to_string();
        if let Some(rest) = key.strip_prefix("inbound.") {
            let (index, field) = rest.split_once('.')?;
            let index: usize = index.parse().ok()?;
            inbound_fields.insert((index, field.to_string()), value);
            continue;
        }
        match key {
            "name" => name = Some(value),
            "schema" => schema = Some(value),
            "documentId" => document_id = Some(value),
            "expectedEvent" => expected_events.push(value),
            "expectedEditId" => expected_edit_ids.push(value),
            _ => {}
        }
    }

    let mut inbound_indexes: Vec<usize> = inbound_fields.keys().map(|(index, _)| *index).collect();
    inbound_indexes.sort_unstable();
    inbound_indexes.dedup();

    let mut inbound = Vec::new();
    for index in inbound_indexes {
        let kind = inbound_fields.get(&(index, "kind".into()))?.clone();
        let raw = match kind.as_str() {
            "externalEdits" => {
                let ops_file = inbound_fields.get(&(index, "opsFile".into()))?.clone();
                RawFixtureInbound::ExternalEdits { ops_file }
            }
            "replaceDocument" => {
                let dsl_file = inbound_fields.get(&(index, "dslFile".into()))?.clone();
                let ops_file = inbound_fields.get(&(index, "opsFile".into()))?.clone();
                RawFixtureInbound::ReplaceDocument { dsl_file, ops_file }
            }
            "hubFrame" => {
                let frame_bytes = inbound_fields.get(&(index, "frameBytes".into()))?.split(',').filter_map(|part| part.trim().parse::<u8>().ok()).collect();
                RawFixtureInbound::HubFrame { frame_bytes }
            }
            _ => return None,
        };
        inbound.push(raw);
    }

    Some(FixtureManifest { name: name?, schema: schema?, document_id: document_id?, inbound, expected_events, expected_edit_ids })
}

/// @emoji 📂️ Loads every `<name>/🔣️fixture.dsl` manifest directory under `dir`, resolving each
/// content-bearing inbound entry against its sibling text file. A fixture whose manifest or
/// any referenced file is missing/unreadable is skipped (never a partial/silently-wrong fixture).
#[cfg(not(target_arch = "wasm32"))]
pub fn load_fixtures(dir: &std::path::Path) -> Vec<ActorFixture> {
    let mut fixtures = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return fixtures };
    let mut fixture_dirs: Vec<std::path::PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.is_dir()).collect();
    fixture_dirs.sort();
    for fixture_dir in fixture_dirs {
        let Ok(manifest_text) = std::fs::read_to_string(fixture_dir.join("🔣️fixture.dsl")) else { continue };
        let Some(manifest) = parse_fixture_dsl_manifest(&manifest_text) else { continue };
        let mut inbound = Vec::with_capacity(manifest.inbound.len());
        let mut all_resolved = true;
        for raw in manifest.inbound {
            let resolved = match raw {
                RawFixtureInbound::HubFrame { frame_bytes } => Some(FixtureInbound::HubFrame { frame_bytes }),
                RawFixtureInbound::ExternalEdits { ops_file } => std::fs::read_to_string(fixture_dir.join(&ops_file)).ok().map(|ops_text| FixtureInbound::ExternalEdits { ops_text }),
                RawFixtureInbound::ReplaceDocument { dsl_file, ops_file } => {
                    let dsl_text = std::fs::read_to_string(fixture_dir.join(&dsl_file)).ok();
                    let ops_text = std::fs::read_to_string(fixture_dir.join(&ops_file)).ok();
                    dsl_text.zip(ops_text).map(|(dsl_text, ops_text)| FixtureInbound::ReplaceDocument { dsl_text, ops_text })
                }
            };
            match resolved {
                Some(inbound_item) => inbound.push(inbound_item),
                None => {
                    all_resolved = false;
                    break;
                }
            }
        }
        if all_resolved {
            fixtures.push(ActorFixture { name: manifest.name, schema: manifest.schema, document_id: manifest.document_id, inbound, expected_events: manifest.expected_events, expected_edit_ids: manifest.expected_edit_ids });
        }
    }
    fixtures
}
//#endregion 🔖️Fixtures

//#region 🔖️FolderStorage
/// @emoji 🗄️ Pure multi-document sqlite persistence (`folder://`), the canonical local store. Rows
/// are keyed by document id: `document(id, schema, pack, spr, updated_at)` — `pack` (initial
/// snapshot) + `spr` (real inverse/binary op payloads/cursor, see `crate::os_store::print_document_spr`)
/// are the AUTHORITATIVE pair, both `NOT NULL`; there is no JSON column (a greenfield rule — every
/// row is written pack+spr together, never one without the other). No `Backbone` impl: the actor
/// layer drives this from its own thread; this crate only owns the sqlite schema.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderSqliteStorage {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderSqliteStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("documents.db")
    }

    fn connection(&self) -> Result<rusqlite::Connection, vcs::VcsError> {
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let conn = rusqlite::Connection::open(self.db_path()).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Self::ensure_schema(&conn)?;
        Ok(conn)
    }

    fn ensure_schema(conn: &rusqlite::Connection) -> Result<(), vcs::VcsError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS document (\
                 id TEXT PRIMARY KEY,\
                 schema TEXT,\
                 pack BLOB NOT NULL,\
                 spr BLOB NOT NULL,\
                 updated_at INTEGER NOT NULL\
             );\
             CREATE TABLE IF NOT EXISTS blobs (\
                 hash TEXT PRIMARY KEY,\
                 media_type TEXT NOT NULL,\
                 size INTEGER NOT NULL,\
                 bytes BLOB NOT NULL\
             );",
        )
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖️ Reads the stored `(pack, spr)` bytes for `document_id`, or `None` if no row exists.
    pub fn read(&self, document_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, vcs::VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT pack, spr FROM document WHERE id = ?1", [document_id], |row| Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))).optional().map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s `(pack, spr)` bytes together (schema id, `updated_at` stamp)
    /// — both are written in the same statement, never independently.
    pub fn write(&self, document_id: &str, schema: &str, pack: &[u8], spr: &[u8]) -> Result<(), vcs::VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, pack, spr, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, pack = excluded.pack, spr = excluded.spr, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, pack, spr, now_ms() as i64],
        )
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📇️ Lists every stored document id (newest write first), for a folder-wide index.
    pub fn document_ids(&self) -> Result<Vec<String>, vcs::VcsError> {
        let conn = self.connection()?;
        let mut statement = conn.prepare("SELECT id FROM document ORDER BY updated_at DESC").map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let ids = statement.query_map([], |row| row.get::<_, String>(0)).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?.collect::<Result<Vec<_>, _>>().map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(ids)
    }
}

/// @emoji 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial
/// snapshot), `<id>.<ext>.ops` holds the append-only op log (see `crate::os_store::print_document_text`/
/// `crate::os_store::parse_document_text`). No `Backbone` impl: like `FolderSqliteStorage` above, this actor
/// layer drives it from its own thread; this crate only owns the file format. Additive alongside the
/// sqlite storage today — a technology adopts it by implementing `ArtifactDsl`/`OpText` and having
/// its sync endpoint construct one of these instead; nothing currently reads or writes through it
/// automatically.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderTextStorage {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderTextStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    fn dsl_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder
            .join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Dsl))
    }

    fn ops_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder
            .join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Op))
    }

    /// @emoji 🏷️ Path of the authoritative binary pack file.
    pub fn pack_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder
            .join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Pack))
    }

    /// @emoji 🏷️ Path of the authoritative binary op-log file.
    pub fn spr_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder
            .join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Spr))
    }

    /// @emoji 📖️ Reads both files for `document_id`, or `None` if the DSL file does not exist yet.
    pub fn read(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactTextFiles>, vcs::VcsError> {
        let dsl = match std::fs::read_to_string(self.dsl_path(document_id, envelope_id)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactTextFiles { dsl, ops }))
    }

    /// @emoji ✍️ Overwrites both files wholesale — the structural-command cold path (undo/redo/
    /// checkpoint/alternative).
    pub fn write(&self, document_id: &str, envelope_id: &str, files: &ArtifactTextFiles) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id), &files.dsl).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id), &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖️ pack+spr-first read: reads the AUTHORITATIVE pair for `document_id`, or `None` if
    /// the `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone
    /// doesn't count — pack+spr are authoritative per the disk-layout LAW, the DSL file is
    /// import-only). A present `.pack` with a missing `.spr` is a hard error — no legacy: they are
    /// always written together (see `write_pack`), so a missing `.spr` means corruption or a
    /// manual edit, never a valid state to silently recover from.
    pub fn read_pack(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactPackFiles>, vcs::VcsError> {
        let pack = match std::fs::read(self.pack_path(document_id, envelope_id)) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let spr = std::fs::read(self.spr_path(document_id, envelope_id)).map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                vcs::VcsError::Backbone(format!(
                    "{document_id} pack.semio exists but spr.semio is missing for envelope {envelope_id}"
                ))
            } else {
                vcs::VcsError::Backbone(err.to_string())
            }
        })?;
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactPackFiles { pack, spr, ops }))
    }

    /// @emoji ✍️ Overwrites all four files: the AUTHORITATIVE `.pack` + `.spr` pair, the shared
    /// `.ops` text mirror, and the always-written DSL mirror `dsl_mirror` (`print_dsl` on the
    /// initial snapshot) — the pack-aware sibling of `write`.
    pub fn write_pack(&self, document_id: &str, envelope_id: &str, files: &ArtifactPackFiles, dsl_mirror: &str) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.pack_path(document_id, envelope_id), &files.pack).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.spr_path(document_id, envelope_id), &files.spr).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id), &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id), dsl_mirror).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ➕️ Appends already-printed op-log lines (one {@link print_edit_lines} block) to the `.ops`
    /// file without rewriting it — the hot-path append unit, O(new edit) instead of O(whole history).
    pub fn append_ops(&self, document_id: &str, envelope_id: &str, lines: &str) -> Result<(), vcs::VcsError> {
        use std::io::Write;
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(self.ops_path(document_id, envelope_id)).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        file.write_all(lines.as_bytes()).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📇️ Lists every stored document id (by DSL `.semio` file stem) for a given envelope id.
    pub fn document_ids(&self, envelope_id: &str) -> Result<Vec<String>, vcs::VcsError> {
        let suffix = format!(".{envelope_id}.dsl.semio");
        let entries = match std::fs::read_dir(&self.folder) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let mut ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(id) = name.strip_suffix(&suffix) {
                    ids.push(id.to_string());
                }
            }
        }
        Ok(ids)
    }
}
//#endregion 🔖️FolderStorage

//#region 🔖️BlobStoreImpl

/// @emoji 🗄️ `FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table (bootstrapped
/// alongside `document` in `ensure_schema`) — one whole-blob `BLOB` column is plenty for v1; this
/// crate's other tables don't chunk large payloads either, and the `crate::os_store::BlobStore` trait itself stays
/// whole-blob regardless of how a given backend chooses to store the bytes internally.
#[cfg(not(target_arch = "wasm32"))]
impl crate::os_store::BlobStore for FolderSqliteStorage {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<crate::os_store::BlobRef, vcs::VcsError> {
        let hash = semio_framework_hash::hash_bytes(bytes);
        let conn = self.connection()?;
        conn.execute("INSERT OR IGNORE INTO blobs (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![hash, media_type, bytes.len() as i64, bytes]).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(crate::os_store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, vcs::VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT bytes FROM blobs WHERE hash = ?1", [hash], |row| row.get(0)).optional().map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    fn has(&self, hash: &str) -> Result<bool, vcs::VcsError> {
        let conn = self.connection()?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM blobs WHERE hash = ?1", [hash], |row| row.get(0)).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(count > 0)
    }

    fn delete(&self, hash: &str) -> Result<(), vcs::VcsError> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM blobs WHERE hash = ?1", [hash]).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(())
    }
}
//#endregion 🔖️BlobStoreImpl

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{Edit, OpBinary, OpText, Mutation, MutationDiff};
    use serde::{Deserialize, Serialize};
    use crate::os_store::{create_document_envelope, parse_document_pack, parse_document_text, print_document_pack, print_document_text, print_edit_lines, register_document_codec, BlobStore, ArtifactCodec, ArtifactCommand, ArtifactDsl, ArtifactPack, pack_rt, PackEncodeOptions, PackDecodeOptions, PackError, ParsedDocumentText};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslArtifact)]
    #[dsl(extension = "demo")]
    struct DemoSnapshot {
        n: i32,
    }

    impl ArtifactDsl for DemoSnapshot {
        const EXTENSION: &'static str = Self::__DSL_EXTENSION;
        fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
        fn parse_dsl(text: &str) -> Result<Self, crate::os_dsl::TextError> {
            let body = match semio_format::split_text_preamble(text) { Ok((_, rest)) => rest, Err(_) => text };
            let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
            Self::__dsl_from_record(&record)
        }
        fn print_dsl(&self) -> String {
            let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
            let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
            semio_format::wrap_text(&envelope, &body)
        }
    }

    impl ArtifactPack for DemoSnapshot {
        fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
            let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
            let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
            Ok(semio_format::wrap_binary(&envelope, &inner))
        }
        fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
            let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
            if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
                return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
            }
            let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
            Self::__dsl_from_record(&record).map_err(|err| PackError::Schema(err.to_string()))
        }
        fn record_spec() -> Option<crate::os_dsl::RecordSpec> { Some(Self::__dsl_spec()) }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl MutationDiff<DemoSnapshot> for DemoDiff {
        fn apply(&self, snapshot: &DemoSnapshot) -> DemoSnapshot {
            DemoSnapshot { n: self.n.unwrap_or(snapshot.n) }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    enum DemoMutation {
        #[dsl(key = "set-n")]
        SetN { n: i32 },
    }

    impl OpText for DemoMutation {
        fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{} ", keyword);
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let record = crate::os_dsl::parse(
                        line,
                        &spec_fn(),
                        &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                    )?;
                    return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    impl OpBinary for DemoMutation {
        fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (idx, (_, spec_fn)) = variants.iter().enumerate().find(|(_, (k, _))| k == &keyword).expect("variant spec must exist");
            let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;
            let mut out = Vec::with_capacity(2 + body.len());
            out.push(pack_rt::OP_BINARY_FORMAT);
            out.push(idx as u8);
            out.extend_from_slice(&body);
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            let mut reader = crate::os_pack::ByteReader::new(bytes);
            let format = reader.read_u8().map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: e.to_string() })?;
            if format != pack_rt::OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op binary format: {format}") });
            }
            let ordinal = reader.read_u8().map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: e.to_string() })?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or_else(|| crate::os_spr::ProtocolError::Malformed {
                what: "op ordinal",
                offset: 1,
                detail: format!("op ordinal {ordinal} out of range for {}", variants.len()),
            })?;
            let spec = spec_fn();
            let body = &bytes[reader.position()..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
                what: "op record",
                offset: reader.position() as u64,
                detail: error.to_string(),
            })
        }
    }

    impl Mutation<DemoSnapshot> for DemoMutation {
        type Diff = DemoDiff;

        fn diff(&self, _snapshot: &DemoSnapshot) -> DemoDiff {
            match self {
                DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![DemoMutation::SetN { n: snapshot.n }]
        }
    }

    /// @emoji 🎯️ Idempotently registers the `demo/v1` codec (process-global `OnceLock` registry,
    /// shared across every test in this binary) — needed by any test exercising `FolderEndpoint`
    /// end-to-end (both `Sqlite` and `Pack` now go through `document_codec` per the pack+spr flip),
    /// mirroring a real app's program-init-time `register_document_codec_for_app` call.
    fn ensure_demo_codec_registered() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            register_document_codec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1"));
        });
    }

    fn sample_operation_envelope(edit_id: &str, n: i32) -> crate::os_spr::MutationEnvelope {
        let edit = Edit {
            id: edit_id.into(),
            actor: None,
            forwards: vec![DemoMutation::SetN { n }],
            inverse: vec![DemoMutation::SetN { n: 0 }],
            mutation_meta: Vec::new(),
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "0".into(),
            finished_at: None,
        };
        let document_id = crate::os_spr::ArtifactId("demo".to_string());
        let schema = crate::os_spr::SchemaId("demo/v1".to_string());
        let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
        envelopes.pop().expect("exactly one op envelope for a single-op edit")
    }

    //#region 🧪️SyncSession
    #[test]
    fn receive_materializes_remote_envelope_into_the_edit_timeline() {
        let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope);
        let mut session = SyncSession::new(store);
        session.receive(sample_operation_envelope("edit-1", 5)).expect("receive");
        assert_eq!(session.store.snapshot().expect("snapshot").n, 5);
        assert_eq!(session.store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
        let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope);
        let mut session = SyncSession::new(store);
        let first = sample_operation_envelope("edit-1", 5);
        let mut second = sample_operation_envelope("edit-2", 9);
        second.dependencies = vec![first.mutation_id.clone()];
        session.receive(second).expect("receive second first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
        session.receive(first).expect("receive first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
        assert_eq!(session.store.snapshot().expect("snapshot").n, 9);
    }
    //#endregion 🧪️SyncSession

    //#region 🧪️Helpers
    #[test]
    fn hub_ws_url_derives_ws_endpoint_from_remote_uri() {
        assert_eq!(hub_ws_url("remote://host:6070", "studio-1", "doc-1"), "ws://host:6070/spaces/studio-1/documents/doc-1/ws");
        assert_eq!(hub_ws_url("https://semio_hub.example.com", "studio-1", "doc-2"), "wss://semio_hub.example.com/spaces/studio-1/documents/doc-2/ws");
        assert_eq!(hub_ws_url("ws://127.0.0.1:5000/prefix", "studio-1", "d"), "ws://127.0.0.1:5000/spaces/studio-1/documents/d/ws");
    }
    //#endregion 🧪️Helpers

    //#region 🧪️WireBridge
    // 🎯️ W6: `wire_bridge_round_trips_identity_and_diff_through_protocol_causal` is DELETED — the
    // local/wire bridge it tested (`to_wire_envelope`/`from_wire_envelope`) no longer exists; local
    // and wire envelopes are the same `crate::os_spr::MutationEnvelope` type now, an identity the type
    // system enforces, not something a round-trip test needs to prove.
    #[test]
    fn rollback_envelope_synthesizes_an_undo_from_the_original_inverse() {
        let envelope = sample_operation_envelope("edit-1", 5);
        let rollback = rollback_envelope(&envelope);
        assert_eq!(rollback.dependencies, vec![envelope.mutation_id.clone()], "the undo depends on the operation it undoes");
        assert_eq!(rollback.diff.payload, envelope.inverse.payload, "the undo's forward diff IS the original's inverse");
        assert_ne!(rollback.mutation_id, envelope.mutation_id, "the undo gets its own operation id");
    }

    /// @emoji 🎬️ Canonical wire-frame byte fixtures shared with `🟦️backbone-worker.ts`'s vitest suite
    /// (`framework/product/os/core/js/🟦️backbone-worker.ts` `WireBridge` region / `index.ts`'s
    /// `encodeClientFrame`/`decodeServerFrame` twins) — both sides decode the exact same committed
    /// bytes under `store/sync/fixtures/wire/`, proving `protocol_wire`'s binary lane+tag codec
    /// round-trips identically across Rust and TS. Regenerated deterministically by this test (every
    /// value below is a fixed constant, never a clock/random read) rather than hand-authored, so a
    /// `protocol_wire` field-order/shape change fails loudly here instead of silently diverging from
    /// the TS twin. 🎯️ W5: extended to cover every `ClientFrame`/`ServerFrame` variant (19 fixtures
    /// total, one per variant plus one extra each for `Welcome`'s `Bootstrap` and `Ack`'s
    /// `ApplyOutcome` sub-variants) — the previous 4-fixture set (`client-hello`, `client-commands`,
    /// `server-welcome`, `server-ack`) is superseded; the first two names are reused (byte-identical
    /// role), the latter two are replaced by the more specific names below and deleted here.
    #[test]
    fn wire_fixtures_stay_byte_identical_across_rust_and_ts() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/wire");
        std::fs::create_dir_all(&fixtures_dir).expect("fixtures dir");
        for stale in ["server-welcome.bin", "server-ack.bin"] {
            let _ = std::fs::remove_file(fixtures_dir.join(stale));
        }

        fn write_client(dir: &std::path::Path, name: &str, frame: &ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(frame, lane);
            std::fs::write(dir.join(name), &bytes).unwrap_or_else(|error| panic!("write {name}: {error}"));
            let (decoded_lane, decoded) = crate::os_spr::decode_client_frame(&bytes).unwrap_or_else(|error| panic!("decode {name}: {error}"));
            assert_eq!(decoded_lane, lane, "{name} lane round trip");
            assert_eq!(&decoded, frame, "{name} frame round trip");
        }

        fn write_server(dir: &std::path::Path, name: &str, frame: &ServerFrame, lane: Lane) {
            let bytes = crate::os_spr::encode_server_frame(frame, lane);
            std::fs::write(dir.join(name), &bytes).unwrap_or_else(|error| panic!("write {name}: {error}"));
            let (decoded_lane, decoded) = decode_server_frame(&bytes).unwrap_or_else(|error| panic!("decode {name}: {error}"));
            assert_eq!(decoded_lane, lane, "{name} lane round trip");
            assert_eq!(&decoded, frame, "{name} frame round trip");
        }

        let frontier = crate::os_spr::RuntimeFrontierSummary { document_id: crate::os_spr::ArtifactId("doc-1".to_string()), head_edit_ordinal: 1, head_edit_id: "op-1".to_string(), last_commit_seq: 1, chain_hash: [9u8; 32] };
        let wire_envelope = crate::os_spr::MutationEnvelope {
            mutation_id: crate::os_spr::MutationId("op-1".to_string()),
            document_id: crate::os_spr::ArtifactId("doc-1".to_string()),
            actor: crate::os_spr::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: crate::os_spr::OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
            inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: crate::os_spr::OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).expect("encode demo op") },
            timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1000, logical: 0 },
        };

        //#region 🔖️ClientFrame
        write_client(
            &fixtures_dir,
            "📦️client-hello.bin",
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "demo/v1".to_string(),
                pack_schema_hash: [7u8; 32],
                actor: crate::os_spr::ActorId("actor-1".to_string()),
                token: Some("token-1".to_string()),
                resume_token: None,
                frontier: None,
            },
            Lane::Command,
        );
        write_client(&fixtures_dir, "📦️client-commands.bin", &ClientFrame::Commands { batch_id: 1, envelopes: vec![wire_envelope.clone()] }, Lane::Command);
        write_client(&fixtures_dir, "📦️client-frontier-advertise.bin", &ClientFrame::FrontierAdvertise { frontier: frontier.clone() }, Lane::Command);
        write_client(&fixtures_dir, "📦️client-preview-publish.bin", &ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview);
        write_client(&fixtures_dir, "📦️client-presence.bin", &ClientFrame::Presence { peer: presence_to_bytes(&sample_presence_peer_with_interaction()) }, Lane::Preview);
        write_client(&fixtures_dir, "📦️client-credit-grant.bin", &ClientFrame::CreditGrant { n: 16 }, Lane::Command);
        write_client(&fixtures_dir, "📦️client-bye.bin", &ClientFrame::Bye, Lane::Command);
        //#endregion 🔖️ClientFrame

        //#region 🔖️ServerFrame
        write_server(&fixtures_dir, "📦️server-welcome-tail.bin", &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail }, Lane::Command);
        write_server(
            &fixtures_dir,
            "📦️server-welcome-snapshot-inline.bin",
            &ServerFrame::Welcome { session_id: "session-2".to_string(), resume_token: "resume-2".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9, 9]) } },
            Lane::Command,
        );
        write_server(&fixtures_dir, "📦️server-snapshot-chunk.bin", &ServerFrame::SnapshotChunk { seq: 0, bytes: vec![1, 2, 3, 4] }, Lane::Command);
        write_server(&fixtures_dir, "📦️server-snapshot-done.bin", &ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command);
        write_server(&fixtures_dir, "📦️server-commands.bin", &ServerFrame::Commands { envelopes: vec![wire_envelope], origin: crate::os_spr::ActorId("actor-1".to_string()), frontier: frontier.clone() }, Lane::Command);
        write_server(
            &fixtures_dir,
            "📦️server-ack-accepted.bin",
            &ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() },
            Lane::Command,
        );
        write_server(
            &fixtures_dir,
            "📦️server-ack-transformed.bin",
            &ServerFrame::Ack {
                batch_id: 2,
                stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Transformed { envelope: Box::new(sample_wire_envelope_for_fixtures()) }) }],
                frontier: frontier.clone(),
            },
            Lane::Command,
        );
        write_server(
            &fixtures_dir,
            "📦️server-ack-rejected.bin",
            &ServerFrame::Ack { batch_id: 3, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "conflict".to_string() }) }], frontier: frontier.clone() },
            Lane::Command,
        );
        write_server(&fixtures_dir, "📦️server-preview.bin", &ServerFrame::Preview { actor: crate::os_spr::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview);
        write_server(&fixtures_dir, "📦️server-presence.bin", &ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), presence_to_bytes(&sample_presence_peer_with_interaction())] }, Lane::Preview);
        write_server(&fixtures_dir, "📦️server-credit-grant.bin", &ServerFrame::CreditGrant { n: 32 }, Lane::Command);
        write_server(&fixtures_dir, "📦️server-error.bin", &ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command);
        //#endregion 🔖️ServerFrame
    }

    /// @emoji 🧸️ A second, distinct `MutationEnvelope` for `📦️server-ack-transformed.bin`'s
    /// `ApplyOutcome::Transformed` payload — must differ from the primary `wire_envelope` fixture so
    /// the vitest canary can assert it decodes as its own value, not an accidental copy.
    fn sample_wire_envelope_for_fixtures() -> crate::os_spr::MutationEnvelope {
        crate::os_spr::MutationEnvelope {
            mutation_id: crate::os_spr::MutationId("op-2".to_string()),
            document_id: crate::os_spr::ArtifactId("doc-1".to_string()),
            actor: crate::os_spr::ActorId("actor-2".to_string()),
            dependencies: vec![crate::os_spr::MutationId("op-1".to_string())],
            diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: crate::os_spr::OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).expect("encode demo op") },
            inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: crate::os_spr::OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
            timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1001, logical: 0 },
        }
    }

    /// @emoji 🕹️ A `PresencePeer` whose `interaction` carries THREE domains (one selection-only, one
    /// hover-only, one with both) — `📦️client-presence.bin`/`📦️server-presence.bin` regenerate off
    /// this so the TS vitest twin exercises bit 7 with a realistic multi-domain payload, not just the
    /// placeholder JSON blob the fixtures carried before this field existed.
    fn sample_presence_peer_with_interaction() -> PresencePeer {
        PresencePeer {
            actor: "actor-1".to_string(),
            label: Some("Ada".to_string()),
            presence_pack: None,
            connected_at_ms: 1_700_000_000_000,
            user_id: Some("user-9".to_string()),
            role: Some("owner".to_string()),
            cursor: Some(crate::os_spr::PresencePoint { x: 12.5, y: -4.0 }),
            viewport: Some(crate::os_spr::PresenceViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
            drag_ghost_json: None,
            interaction: Some(crate::os_spr::PresenceInteraction {
                app_id: "space".to_string(),
                domains: vec![
                    crate::os_spr::PresenceDomain { domain: "outline".to_string(), granularity: "task".to_string(), selected: vec!["t1".to_string(), "t2".to_string()], hovered: vec![] },
                    crate::os_spr::PresenceDomain { domain: "board".to_string(), granularity: "card".to_string(), selected: vec![], hovered: vec!["c1".to_string()] },
                    crate::os_spr::PresenceDomain { domain: "canvas".to_string(), granularity: "node".to_string(), selected: vec!["n9".to_string()], hovered: vec!["n9".to_string(), "n10".to_string()] },
                ],
            }),
        }
    }
    //#endregion 🧪️WireBridge

    //#region 🧪️PresenceInteraction
    fn selection(ids: &[&str]) -> crate::os_spr::DomainSelection {
        crate::os_spr::DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None }
    }

    fn hover(channel: &str, ids: &[&str]) -> crate::os_spr::DomainHover {
        crate::os_spr::DomainHover { channel: channel.into(), ids: ids.iter().map(|id| id.to_string()).collect() }
    }

    fn broadcasting_hover_spec() -> crate::os_spr::HoverSpec {
        crate::os_spr::HoverSpec { enabled: true, transitive: false, channels: vec!["pointer".into()], broadcast: true }
    }

    fn broadcasting_selection_spec() -> crate::os_spr::SelectionSpec {
        crate::os_spr::SelectionSpec { modes: vec![crate::os_spr::SelectionMode::Multiple], methods: vec![crate::os_spr::SelectionMethod::Pick], merges: vec![crate::os_spr::MergeMode::Replace], transitive: false, broadcast: true }
    }

    #[test]
    fn assemble_presence_interaction_includes_broadcasting_domains() {
        let mut state = crate::os_spr::InteractionState::default();
        state.selection.insert("graph".into(), selection(&["n1", "n2"]));
        state.hover.insert("graph".into(), hover("pointer", &["n3"]));
        state.active_granularity.insert("graph".into(), "node".into());

        let hover_specs = std::collections::BTreeMap::from([("graph".to_string(), broadcasting_hover_spec())]);
        let selection_specs = std::collections::BTreeMap::from([("graph".to_string(), broadcasting_selection_spec())]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs);
        assert_eq!(interaction.app_id, "draw");
        assert_eq!(interaction.domains.len(), 1);
        let domain = &interaction.domains[0];
        assert_eq!(domain.domain, "graph");
        assert_eq!(domain.granularity, "node");
        assert_eq!(domain.selected, vec!["n1".to_string(), "n2".to_string()]);
        assert_eq!(domain.hovered, vec!["n3".to_string()]);
    }

    #[test]
    fn assemble_presence_interaction_omits_domains_with_broadcast_disabled() {
        let mut state = crate::os_spr::InteractionState::default();
        state.selection.insert("private".into(), selection(&["secret"]));
        state.hover.insert("private".into(), hover("pointer", &["secret"]));

        let hover_specs = std::collections::BTreeMap::from([("private".to_string(), crate::os_spr::HoverSpec { broadcast: false, ..broadcasting_hover_spec() })]);
        let selection_specs = std::collections::BTreeMap::from([("private".to_string(), crate::os_spr::SelectionSpec { broadcast: false, ..broadcasting_selection_spec() })]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs);
        assert!(interaction.domains.is_empty(), "broadcast:false on both halves drops the domain entirely");
    }

    #[test]
    fn assemble_presence_interaction_only_broadcasts_the_pointer_hover_channel() {
        let mut state = crate::os_spr::InteractionState::default();
        state.hover.insert("graph".into(), hover("drag-preview", &["n1"]));

        let hover_specs = std::collections::BTreeMap::from([("graph".to_string(), broadcasting_hover_spec())]);
        let selection_specs = std::collections::BTreeMap::new();

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs);
        assert!(interaction.domains.is_empty(), "a non-pointer hover channel never broadcasts");
    }

    #[test]
    fn assemble_presence_interaction_respects_each_half_independently() {
        let mut state = crate::os_spr::InteractionState::default();
        state.selection.insert("graph".into(), selection(&["n1"]));
        state.hover.insert("graph".into(), hover("pointer", &["n2"]));

        let hover_specs = std::collections::BTreeMap::from([("graph".to_string(), crate::os_spr::HoverSpec { broadcast: false, ..broadcasting_hover_spec() })]);
        let selection_specs = std::collections::BTreeMap::from([("graph".to_string(), broadcasting_selection_spec())]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs);
        assert_eq!(interaction.domains.len(), 1);
        assert_eq!(interaction.domains[0].selected, vec!["n1".to_string()], "selection still broadcasts");
        assert!(interaction.domains[0].hovered.is_empty(), "hover suppressed by its own broadcast:false");
    }

    #[test]
    fn presence_heartbeat_producer_publishes_immediately_then_coalesces_to_latest() {
        let mut producer = PresenceHeartbeatProducer::new(100);
        let mut first = sample_presence_peer_with_interaction();
        first.cursor = Some(crate::os_spr::PresencePoint { x: 1.0, y: 2.0 });
        assert_eq!(producer.offer(1_000, first.clone()), Some(first));

        let mut intermediate = sample_presence_peer_with_interaction();
        intermediate.cursor = Some(crate::os_spr::PresencePoint { x: 3.0, y: 4.0 });
        assert_eq!(producer.offer(1_040, intermediate), None);

        let mut latest = sample_presence_peer_with_interaction();
        latest.cursor = Some(crate::os_spr::PresencePoint { x: 5.0, y: 6.0 });
        assert_eq!(producer.offer(1_099, latest.clone()), None);
        assert_eq!(producer.pending(), Some(&latest));
        assert_eq!(producer.offer(1_100, latest.clone()), Some(latest));
        assert!(producer.pending().is_none());
    }

    #[test]
    fn artifact_host_presence_heartbeat_owns_cadence_per_document() {
        let host = ArtifactHost::new();
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();
        let (events, _) = broadcast::channel(1);
        host.inner.lock().unwrap().insert(
            "doc".into(),
            OpenDocument {
                cmd_tx,
                events,
                presence: PresenceHeartbeatProducer::default(),
                #[cfg(not(target_arch = "wasm32"))]
                join: None,
            },
        );

        let first = sample_presence_peer_with_interaction();
        assert!(host.presence_heartbeat("doc", 500, first.clone()));
        assert!(matches!(cmd_rx.try_recv(), Ok(ArtifactActorMsg::PresenceHeartbeat { peer }) if peer == first));

        let mut latest = sample_presence_peer_with_interaction();
        latest.viewport = Some(crate::os_spr::PresenceViewport { x: 2.0, y: 3.0, zoom: 4.0 });
        assert!(!host.presence_heartbeat("doc", 550, latest.clone()));
        assert!(cmd_rx.try_recv().is_err(), "sub-interval offer must not publish");
        assert!(host.presence_heartbeat("doc", 600, latest.clone()));
        assert!(matches!(cmd_rx.try_recv(), Ok(ArtifactActorMsg::PresenceHeartbeat { peer }) if peer == latest));
        assert!(!host.presence_heartbeat("missing", 700, sample_presence_peer_with_interaction()));
    }
    //#endregion 🧪️PresenceInteraction

    //#region 🧪️Helpers

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn op_envelope_from_stored_edit_round_trips_through_ingest() {
        let edit = crate::os_spr::HistoryEdit {
            id: "ext-1".into(),
            actor: Some("peer".into()),
            started_at: "0".into(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
            inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 0 }.encode_op().expect("encode")) }],
            meta: None,
        };
        let envelopes = envelopes_from_history_edit(&edit, "demo", "demo/v1").expect("envelopes from history edit");
        assert_eq!(envelopes.len(), 1, "single-op edit yields one envelope");
        assert_eq!(envelopes[0].mutation_id.0, "ext-1#0", "meta-less fallback: edit id # op index");
        let recovered = <DemoMutation as crate::os_spr::OpBinary>::decode_op(&envelopes[0].diff.payload).expect("decode op");
        assert_eq!(recovered, DemoMutation::SetN { n: 42 });
    }
    //#endregion 🧪️Helpers

    //#region 🧪️Actor
    #[cfg(not(target_arch = "wasm32"))]
    mod actor_tests {
        use super::*;
        use futures_util::{SinkExt, StreamExt};
        use crate::os_spr::{decode_client_frame, encode_server_frame};
        use std::sync::Arc;
        use std::time::Duration;
        use tokio::sync::{broadcast as tokio_broadcast, Mutex};
        use tokio_tungstenite::tungstenite::Message as WsMessage;

        fn demo_envelope(document_id: &str) -> crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> {
            create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None)
        }

        async fn wait_until(label: &str, mut predicate: impl FnMut() -> bool) {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            while !predicate() {
                if tokio::time::Instant::now() >= deadline {
                    panic!("{label} not satisfied before 5s deadline");
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }

        async fn wait_until_value<T>(label: &str, mut predicate: impl FnMut() -> Option<T>) -> T {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                if let Some(value) = predicate() {
                    return value;
                }
                if tokio::time::Instant::now() >= deadline {
                    panic!("{label} not satisfied before 5s deadline");
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }

        async fn wait_for_event(events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                match tokio::time::timeout_at(deadline, events.recv()).await {
                    Ok(Ok(event)) => {
                        if predicate(&event) {
                            return event;
                        }
                    }
                    Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => continue,
                    other => panic!("no matching event before deadline: {other:?}"),
                }
            }
        }

        // 🔬️ External folder edit → RemoteMutations event + the store timeline grows on tick().
        #[tokio::test]
        async fn folder_external_edit_delivers_remote_operations() {
            ensure_demo_codec_registered();
            let dir = tempfile::tempdir().expect("tempdir");
            let host = ArtifactHost::new();
            let channels = host.open(ArtifactActorConfig { document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() });
            let mut events = host.subscribe("doc-a");
            let mut store = ArtifactStore::new(demo_envelope("doc-a"));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");

            // A local apply establishes a persisted edit on disk.
            store.dispatch(crate::os_store::ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
            channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

            // Wait until the actor has persisted the local edit to the folder db as real pack+spr bytes.
            let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
            let (pack, spr) = wait_until_value("persisted edit on disk", || {
                let (pack, spr) = storage.read("doc-a").expect("read")?;
                if spr_op_ids(&spr).ok()?.is_empty() {
                    None
                } else {
                    Some((pack, spr))
                }
            })
            .await;

            // Out-of-band: append a foreign edit directly to the spr bytes (real binary op
            // payloads, no codec, no JSON) before writing pack+spr back.
            let external_edit = crate::os_spr::HistoryEdit {
                id: "external-1".into(),
                actor: Some("peer".into()),
                started_at: "0".into(),
                finished_at: None,
                coalesce_key: None,
                description: None,
                ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
                inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().expect("encode")) }],
                meta: None,
            };
            let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &[external_edit]).expect("append external edit");
            storage.write("doc-a", "demo/v1", &pack, &new_spr).expect("out-of-band write");

            // Deterministically poke the actor to re-read (notify also wired, but timing-independent here).
            channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");

            let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            match event {
                ArtifactEvent::RemoteMutations { envelopes } => {
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].mutation_id.0, "external-1#0", "single-op edit -> mutation_id is edit.id#0 (crate::os_spr::mutation_envelope_from_edit's ordinal-suffix convention)");
                }
                other => panic!("expected RemoteMutations, got {other:?}"),
            }

            // The store ingests the pushed operation on tick(); the timeline grows and snapshot updates.
            store.tick().expect("tick");
            assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
            assert_eq!(store.snapshot().expect("snapshot").n, 42);
            host.close("doc-a");
        }

        //#region 🔖️MockHub
        /// @emoji 🧪️ A minimal in-process semio_hub speaking the real, binary `crate::os_spr::wire::ClientFrame`/
        /// `ServerFrame` protocol, so the semio_hub endpoint is exercised end-to-end without linking a real
        /// `db`-backed semio_hub (that's CW6's job — this mock never touches `db`). Ordinal-indexed log,
        /// mirroring `db_sync`'s replica-catch-up shape (`Hello.frontier` -> filtered backlog ->
        /// `Welcome` then a follow-up `Commands`), but with a placeholder `chain_hash`/`resume_token`
        /// (this mock has no durable log to derive a real chain hash from).
        struct MockHub {
            log: Arc<Mutex<Vec<(u64, crate::os_spr::MutationEnvelope)>>>,
            broadcast: tokio_broadcast::Sender<ServerFrame>,
        }

        fn mock_frontier(ordinal: u64) -> crate::os_spr::RuntimeFrontierSummary {
            crate::os_spr::RuntimeFrontierSummary { document_id: ArtifactId("mock".to_string()), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: [0u8; 32] }
        }

        async fn spawn_mock_hub() -> (std::net::SocketAddr, Arc<MockHub>) {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
            let addr = listener.local_addr().expect("addr");
            let (broadcast, _rx) = tokio_broadcast::channel(256);
            let semio_hub = Arc::new(MockHub { log: Arc::new(Mutex::new(Vec::new())), broadcast });
            let accept_hub = semio_hub.clone();
            tokio::spawn(async move {
                loop {
                    let Ok((stream, _)) = listener.accept().await else { break };
                    let conn_hub = accept_hub.clone();
                    tokio::spawn(async move {
                        if let Ok(ws) = tokio_tungstenite::accept_async(stream).await {
                            mock_hub_connection(ws, conn_hub).await;
                        }
                    });
                }
            });
            (addr, semio_hub)
        }

        async fn mock_hub_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, semio_hub: Arc<MockHub>) {
            let (mut write, mut read) = ws.split();
            // Expect Hello first.
            let requested_ordinal = match read.next().await {
                Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes) {
                    Ok((_, ClientFrame::Hello { frontier, .. })) => frontier.map_or(0, |frontier| frontier.head_edit_ordinal),
                    _ => return,
                },
                _ => return,
            };
            let (frontier, backlog) = {
                let log = semio_hub.log.lock().await;
                let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
                let backlog: Vec<crate::os_spr::MutationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
                (mock_frontier(ordinal), backlog)
            };
            let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
            if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).into())).await.is_err() {
                return;
            }
            if !backlog.is_empty() {
                let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("semio_hub-backlog".to_string()), frontier: frontier.clone() };
                if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).into())).await.is_err() {
                    return;
                }
            }
            let mut broadcast_rx = semio_hub.broadcast.subscribe();
            loop {
                tokio::select! {
                    incoming = read.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Binary(bytes))) => {
                                match decode_client_frame(&bytes) {
                                    Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                        let mut assigned_frontier = frontier.clone();
                                        for envelope in envelopes {
                                            let (ordinal, origin) = {
                                                let mut log = semio_hub.log.lock().await;
                                                let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                                log.push((next, envelope.clone()));
                                                (next, envelope.actor.clone())
                                            };
                                            assigned_frontier = mock_frontier(ordinal);
                                            let _ = semio_hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                        }
                                        let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                        let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).into())).await;
                                    }
                                    Ok((_, ClientFrame::PreviewPublish { key, seq, payload })) => {
                                        // 👻️ Best-effort fan-out on the uncredited preview lane — this mock
                                        // semio_hub doesn't track per-connection actor identity beyond `Hello`, so
                                        // it stamps a fixed sentinel origin (fine for the round-trip test
                                        // this drives, which only asserts the *other* peer receives it).
                                        let _ = semio_hub.broadcast.send(ServerFrame::Preview { actor: ActorId("mock-semio_hub-peer".to_string()), key, seq, payload });
                                    }
                                    Ok((_, ClientFrame::Bye)) | Err(_) => {}
                                    Ok(_) => {}
                                }
                            }
                            Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => break,
                            Some(Ok(_)) => {}
                        }
                    }
                    frame = broadcast_rx.recv() => {
                        match frame {
                            Ok(frame) => {
                                if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).into())).await.is_err() {
                                    break;
                                }
                            }
                            Err(tokio_broadcast::error::RecvError::Lagged(_)) => {}
                            Err(tokio_broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }
            }
        }
        //#endregion 🔖️MockHub

        // 🔬️ Two ArtifactHosts converge through a semio_hub: A's operation fans out to B, whose store materializes it.
        #[tokio::test]
        async fn two_hosts_converge_through_hub() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = ArtifactHost::new();
            let channels_a = host_a.open(ArtifactActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "A".into(),
            });
            let mut store_a = ArtifactStore::new(demo_envelope("shared"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");

            let host_b = ArtifactHost::new();
            let channels_b = host_b.open(ArtifactActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "B".into(),
            });
            let mut events_b = host_b.subscribe("shared");
            let mut store_b = ArtifactStore::new(demo_envelope("shared"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            // Give both actors time to connect + Hello.
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(crate::os_store::ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).expect("apply on a");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            match event {
                ArtifactEvent::RemoteMutations { envelopes } => assert_eq!(envelopes.len(), 1),
                other => panic!("expected RemoteMutations on B, got {other:?}"),
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 7, "B converged on A's operation");

            host_a.close("shared");
            host_b.close("shared");
        }

        // 🔬️ Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
        // its Welcome backlog carries only the operations it missed.
        #[tokio::test]
        async fn reconnect_since_catch_up_replays_backlog() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = ArtifactHost::new();
            let channels_a = host_a.open(ArtifactActorConfig {
                document_id: "catchup".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "A".into(),
            });
            let mut store_a = ArtifactStore::new(demo_envelope("catchup"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            // A applies two operations while nobody else is connected.
            for n in [3, 4] {
                store_a.dispatch(crate::os_store::ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n }], description: None }).expect("apply on a");
                channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
                tokio::time::sleep(Duration::from_millis(80)).await;
            }

            // B connects fresh (since_version 0) and its Welcome backlog replays both operations.
            let host_b = ArtifactHost::new();
            let channels_b =
                host_b.open(ArtifactActorConfig { document_id: "catchup".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), token: None }], watch_external: false, actor: "B".into() });
            let mut events_b = host_b.subscribe("catchup");
            let mut store_b = ArtifactStore::new(demo_envelope("catchup"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            if let ArtifactEvent::RemoteMutations { envelopes } = event {
                assert_eq!(envelopes.len(), 2, "backlog replays both missed operations");
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);

            host_a.close("catchup");
            host_b.close("catchup");
        }

        // 🔬️ Detach drains the outbox: an operation applied right before close still reaches the semio_hub (and B).
        #[tokio::test]
        async fn detach_drains_pending_outbound_operations() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            // Observer B stays connected to witness A's last operation.
            let host_b = ArtifactHost::new();
            let channels_b = host_b.open(ArtifactActorConfig {
                document_id: "drain".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "B".into(),
            });
            let mut events_b = host_b.subscribe("drain");
            let mut store_b = ArtifactStore::new(demo_envelope("drain"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            let host_a = ArtifactHost::new();
            let channels_a =
                host_a.open(ArtifactActorConfig { document_id: "drain".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), token: None }], watch_external: false, actor: "A".into() });
            let mut store_a = ArtifactStore::new(demo_envelope("drain"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(crate::os_store::ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply on a");
            // Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
            host_a.close("drain");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            if let ArtifactEvent::RemoteMutations { envelopes } = event {
                assert_eq!(envelopes.len(), 1, "the operation applied before detach was not lost");
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
            host_b.close("drain");
        }

        // 🔬️ The mock semio_hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
        // `ArtifactEvent::CommandOutcome` wiring actually fires (not just that it compiles).
        #[tokio::test]
        async fn command_outcome_accepted_fires_after_hub_ack() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");
            let host = ArtifactHost::new();
            let channels =
                host.open(ArtifactActorConfig { document_id: "outcome".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), token: None }], watch_external: false, actor: "A".into() });
            let mut events = host.subscribe("outcome");
            let mut store = ArtifactStore::new(demo_envelope("outcome"));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store.dispatch(crate::os_store::ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
            channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

            let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await;
            match event {
                ArtifactEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
                other => panic!("expected CommandOutcome, got {other:?}"),
            }
            host.close("outcome");
        }

        // 🔬️ `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock semio_hub's
        // preview-lane fan-out -> `ServerFrame::Preview` -> `ArtifactEvent::Preview` on another peer.
        #[tokio::test]
        async fn publish_preview_round_trips_through_hub() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = ArtifactHost::new();
            let channels_a = host_a.open(ArtifactActorConfig {
                document_id: "preview".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "A".into(),
            });

            let host_b = ArtifactHost::new();
            host_b.open(ArtifactActorConfig { document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), token: None }], watch_external: false, actor: "B".into() });
            let mut events_b = host_b.subscribe("preview");
            tokio::time::sleep(Duration::from_millis(300)).await;

            channels_a.cmd_tx.send(ArtifactActorMsg::PublishPreview { key: "cursor".into(), seq: 1, payload: vec![1, 2, 3] }).expect("publish preview");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::Preview { .. })).await;
            match event {
                ArtifactEvent::Preview { key, seq, payload, .. } => {
                    assert_eq!(key, "cursor");
                    assert_eq!(seq, 1);
                    assert_eq!(payload, vec![1, 2, 3]);
                }
                other => panic!("expected Preview, got {other:?}"),
            }
            host_a.close("preview");
            host_b.close("preview");
        }

        // 🔬️ Shared fixtures replay: each fixture's inbound stimuli produce the expected ArtifactEvent
        // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
        #[tokio::test]
        async fn fixtures_replay_matches_expected_events() {
            let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("🧫️fixtures");
            let fixtures = load_fixtures(&fixtures_dir);
            assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
            for fixture in fixtures {
                replay_fixture(&fixture).await;
            }
        }

        async fn replay_fixture(fixture: &ActorFixture) {
            ensure_demo_codec_registered();
            let codec = crate::os_store::document_codec(&fixture.schema).unwrap_or_else(|| panic!("no codec registered for fixture schema {:?}", fixture.schema));
            let dir = tempfile::tempdir().expect("tempdir");
            let host = ArtifactHost::new();
            let channels =
                host.open(ArtifactActorConfig { document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() });
            let mut events = host.subscribe(&fixture.document_id);
            let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");
            let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
            wait_until(&format!("seed snapshot for {} on disk", fixture.document_id), || storage.read(&fixture.document_id).expect("read").is_some()).await;

            // Lockstep: apply each stimulus, then wait for its paired expected event before the next
            // (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
            assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
            let mut observed: Vec<String> = Vec::new();
            for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
                match inbound {
                    FixtureInbound::ExternalEdits { ops_text } => {
                        let (pack, spr) = storage.read(&fixture.document_id).expect("read").expect("some");
                        let parsed = crate::os_spr::parse_ops_text(ops_text).unwrap_or_else(|error| panic!("fixture {} parse_ops_text: {error}", fixture.name));
                        let new_edits: Vec<crate::os_spr::HistoryEdit> = parsed
                            .edits
                            .into_iter()
                            .map(|edit| {
                                let ops: Vec<crate::os_spr::OpPayload> = edit
                                    .ops
                                    .iter()
                                    .map(|op| {
                                        let text = op.text.as_deref().unwrap_or_else(|| panic!("fixture {} op line has no text", fixture.name));
                                        let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
                                        crate::os_spr::OpPayload { text: None, binary: Some(concrete.encode_op().expect("encode demo op")) }
                                    })
                                    .collect();
                                crate::os_spr::HistoryEdit { ops, meta: None, ..edit }
                            })
                            .collect();
                        let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &new_edits).expect("append fixture edits");
                        storage.write(&fixture.document_id, &fixture.schema, &pack, &new_spr).expect("write");
                        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::ReplaceDocument { dsl_text, ops_text } => {
                        let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
                        storage.write(&fixture.document_id, &fixture.schema, &pack_files.pack, &pack_files.spr).expect("replace write");
                        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::HubFrame { .. } => {
                        panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                    }
                }
                let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
                observed.push(document_event_tag(&event).to_string());
                store.tick().expect("tick");
            }
            assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
            let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            for expected_id in &fixture.expected_edit_ids {
                assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
            }
            host.close(&fixture.document_id);
        }

        fn document_event_tag(event: &ArtifactEvent) -> &'static str {
            match event {
                ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
                ArtifactEvent::SnapshotReplaced { .. } => "snapshotReplaced",
                ArtifactEvent::Status(_) => "status",
                ArtifactEvent::Presence { .. } => "presence",
                ArtifactEvent::Preview { .. } => "preview",
                ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
                ArtifactEvent::Conflict(_) => "conflict",
            }
        }
    }
    //#endregion 🧪️Actor

    /// @emoji 🎯️ `FolderSqliteStorage` is now a pure `(pack, spr)` byte-blob store — schema-agnostic,
    /// no JSON/codec involvement at this layer (that lives one level up, in `FolderEndpoint`, tested
    /// via `folder_external_edit_delivers_remote_operations`). This test exercises exactly the
    /// storage mechanics: per-id keying, upsert-in-place, and the folder-wide index.
    #[test]
    fn folder_sqlite_storage_round_trips_by_document_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("doc-a").expect("read empty"), None, "absent document reads as None");

        storage.write("doc-a", "demo/v1", b"pack-a", b"spr-a").expect("write a");
        storage.write("doc-b", "demo/v1", b"pack-b", b"spr-b").expect("write b");
        assert_eq!(storage.read("doc-a").expect("read a").expect("some a"), (b"pack-a".to_vec(), b"spr-a".to_vec()), "documents are keyed independently");
        assert_eq!(storage.read("doc-b").expect("read b").expect("some b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));

        storage.write("doc-a", "demo/v1", b"pack-a2", b"spr-a2").expect("upsert a");
        assert_eq!(storage.read("doc-a").expect("reread a").expect("some a2"), (b"pack-a2".to_vec(), b"spr-a2".to_vec()), "writing the same id upserts pack+spr together in place");

        let mut ids = storage.document_ids().expect("document ids");
        ids.sort();
        assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
    }

    /// @emoji 🔐️ The endpoint-level save→load→undo proof: a store's undo/redo position survives a
    /// full write/read cycle through the ACTUAL `FolderSqliteStorage` byte storage (`store`'s own
    /// `save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position` proves the pure
    /// in-memory pack/spr encoding; this proves the folder persistence layer built on top of it).
    #[test]
    fn folder_sqlite_storage_round_trips_undo_position_through_pack_spr() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());

        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply e1");
        let post_e1 = store.snapshot().expect("post-e1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply e2");
        store.dispatch(ArtifactCommand::Undo).expect("undo e2");
        assert_eq!(store.snapshot().expect("live"), post_e1, "precondition: live store is back at post-e1");

        let files = print_document_pack(store.envelope()).expect("print document pack");
        storage.write("doc-a", "demo/v1", &files.pack, &files.spr).expect("write");

        let (pack, spr) = storage.read("doc-a").expect("read").expect("some");
        let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack, &spr).unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1 through the folder storage layer");
        let mut reloaded = ArtifactStore::new(parsed.envelope);
        assert_eq!(reloaded.snapshot().expect("reloaded"), post_e1);

        reloaded.dispatch(ArtifactCommand::Redo).expect("redo e2 after folder reload");
        assert_eq!(reloaded.snapshot().expect("post-redo"), DemoSnapshot { n: 2 });
    }

    /// @emoji 🎯️ Seeds the write from a ZERO-edit envelope (no cursor line — a cursor is only
    /// synced once an edit is dispatched, see `ArtifactStore::sync_cursor`) so both edits are then
    /// added purely via the raw `append_ops` hot path with no cursor line ever written; a cursor
    /// pinned to an earlier edit count would otherwise cap the reconstructed snapshot at that
    /// edit (see `document_text_round_trips_a_cursor_after_undo_then_apply_interleaving` in
    /// `store`'s own test suite for that law, exercised correctly there).
    #[test]
    fn folder_text_storage_round_trips_dsl_and_appends_ops() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("demo", "demo").expect("read empty"), None, "absent document reads as None");

        let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        let files = print_document_text(seed.envelope()).expect("print document text");
        storage.write("demo", "demo", &files).expect("write");

        let mut store = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply 1");
        let first_edit = store.envelope().vcs.edits.last().expect("first edit");
        storage.append_ops("demo", "demo", &print_edit_lines(first_edit).expect("print edit lines")).expect("append ops 1");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage.append_ops("demo", "demo", &print_edit_lines(second_edit).expect("print edit lines")).expect("append ops 2");

        let reloaded = storage.read("demo", "demo").expect("read").expect("some");
        let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded.dsl, &reloaded.ops).unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.snapshot.n, 2, "write + append reconstructs every edit in order");

        assert_eq!(storage.document_ids("demo").expect("document ids"), vec!["demo".to_string()]);
    }

    /// @emoji 🎯️ Unlike the `.ops`-text hot path (`append_ops`, tested above), `.pack`+`.spr` have
    /// no incremental-append primitive wired up yet (that's `crate::os_spr::HistoryAppender`, a future
    /// wave's job to thread through this storage layer) — `write_pack` is the whole-file cold path
    /// for the AUTHORITATIVE pair, called again after every edit. `append_ops` still keeps the
    /// `.ops` TEXT MIRROR current independently (it is never read by the pack+spr-first
    /// `parse_document_pack`/`read_pack` path — see `ArtifactPackFiles`'s doc — only by
    /// `parse_document_text`), which this test verifies explicitly: appending ops text alone,
    /// without an accompanying `write_pack`, does NOT change what `read_pack`/`parse_document_pack`
    /// reconstructs, because pack+spr (not ops text) are authoritative for that path.
    #[test]
    fn folder_text_storage_round_trips_pack() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read_pack("demo", "demo").expect("read empty"), None, "absent pack reads as None");

        let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        let files = print_document_pack(seed.envelope()).expect("print document pack");
        let dsl_mirror = seed.envelope().vcs.initial_snapshot.print_dsl();
        storage.write_pack("demo", "demo", &files, &dsl_mirror).expect("write pack");

        let mut store = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply 1");
        let first_edit = store.envelope().vcs.edits.last().expect("first edit");
        storage.append_ops("demo", "demo", &print_edit_lines(first_edit).expect("print edit lines")).expect("append ops 1");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage.append_ops("demo", "demo", &print_edit_lines(second_edit).expect("print edit lines")).expect("append ops 2");

        // Text mirror is current (both edits landed via append_ops).
        let reloaded_text = storage.read("demo", "demo").expect("read text").expect("some text");
        let parsed_text: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded_text.dsl, &reloaded_text.ops).unwrap_or_else(|error| panic!("parse text: {error}"));
        assert_eq!(parsed_text.snapshot.n, 2, "the .ops text mirror reflects every appended edit");

        // pack+spr are unaffected by ops-text-only appends — still the zero-edit snapshot from
        // the initial write_pack, proving read_pack/parse_document_pack never reads .ops.
        let reloaded_pack = storage.read_pack("demo", "demo").expect("read pack").expect("some pack");
        let parsed_pack: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack.pack, &reloaded_pack.spr).unwrap_or_else(|error| panic!("parse pack: {error}"));
        assert_eq!(parsed_pack.snapshot.n, 0, "pack+spr are authoritative and independent of ops-text-only appends");

        // A fresh whole-file write_pack (the actual cold-path persistence flow) brings pack+spr
        // current with the live store.
        let files2 = print_document_pack(store.envelope()).expect("print document pack 2");
        let dsl_mirror2 = store.envelope().vcs.initial_snapshot.print_dsl();
        storage.write_pack("demo", "demo", &files2, &dsl_mirror2).expect("write pack 2");
        let reloaded_pack2 = storage.read_pack("demo", "demo").expect("read pack 2").expect("some pack 2");
        let parsed_pack2: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack2.pack, &reloaded_pack2.spr).unwrap_or_else(|error| panic!("parse pack 2: {error}"));
        assert_eq!(parsed_pack2.snapshot.n, 2, "a fresh write_pack brings pack+spr current with the live store");

        // The always-written DSL mirror must also be on disk and agree with the initial-snapshot.
        let mirror = std::fs::read_to_string(storage.pack_path("demo", "demo").with_extension("")).expect("dsl mirror on disk");
        assert_eq!(DemoSnapshot::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
    }

    #[test]
    fn blob_store_put_get_dedupes_idempotently() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        let bytes = b"hello content-addressed world";
        assert!(!storage.has("not-a-real-hash").expect("has on empty store"));

        let first = storage.put(bytes, "text/plain").expect("first put");
        let second = storage.put(bytes, "text/plain").expect("second put");
        assert_eq!(first, second, "putting identical bytes twice is idempotent and dedupes by hash");
        assert_eq!(first.size, bytes.len() as u64);
        assert_eq!(first.media_type, "text/plain");

        assert!(storage.has(&first.hash).expect("has after put"));
        let fetched = storage.get(&first.hash).expect("get").expect("blob present");
        assert_eq!(fetched, bytes);

        let other = storage.put(b"different content", "text/plain").expect("put other");
        assert_ne!(other.hash, first.hash, "different bytes hash differently");

        storage.delete(&first.hash).expect("delete");
        assert!(!storage.has(&first.hash).expect("has after delete"));
        assert_eq!(storage.get(&first.hash).expect("get after delete"), None);
    }
}
