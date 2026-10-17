//! 🔁️ Local-first sync actor layer: a schema-agnostic per-document backbone actor that runs all IO
//! (persist, semio_hub WebSocket sync, file watching) off the UI thread, plus the causal {@link SyncSession}
//! that feeds remote {@link MutationEnvelope}s into a document's vcs edit timeline.
//!
//! # Threading model
//! - **Native** (wgpu native host, tests): {@link ArtifactHost::open} schedules bounded actor turns
//!   on the injected process `WorkerPool`; WebSocket readiness remains on the ambient Tokio I/O
//!   reactor and all actor deadlines use the pool's `TimerWheel`.
//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on the owned browser-local
//!   executor and dials its hub through the same `semio.session.v1` admission as the native actor, over
//!   the socket the host's injected `DocumentSocketDialer` opens (no threads, no filesystem). The React
//!   shell's browser host keeps its TS twin (`🏪️store/👷️worker/🟦️.ts`, WS-E).
//! - **WASI-P2 plugins never link this crate** — inside the sandbox a store attaches vcs's pure
//!   `PortBackbone` (an in-memory queue relayed to the host). This actor is a host-side concern only.

use semio_framework_process_routing_workspace as workspace_identity;

/// 🏠️ Admits only a declared package workspace with regular build authority.
pub fn is_repository_root(path: &std::path::Path) -> Result<bool, String> {
    workspace_identity::is_workspace_root(path, |source| semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| matches!(value.get("workspaces"), Some(semio_framework_pack_json::Value::Array(_) | semio_framework_pack_json::Value::Object(_)))).map_err(|error| error.to_string()))
}

use semio_framework_value::DslValue;
use semio_framework_value::FromValue as FromValueTrait;
use semio_framework_value::ToValue as ToValueTrait;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{PresencePeer, PresenceToolRun};
use semio_framework_os_spr::{decode_document_backbone_envelopes_exact, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes, AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, OpBinary, RuntimeFrontierSummary, ServerFrame};
use semio_framework_os_spr::{ActorId, MutationId};
use semio_framework_os_store_backbone::{decode_hot_backbone_message_exact, BackboneMessage, Backbones, ChannelBackbone, ChannelBackboneRemote, BACKBONE_CHANNEL_MAXIMUM_BYTES, BACKBONE_CHANNEL_MAXIMUM_MESSAGES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES};
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_text_io::text_format::ArtifactTextFiles;
use semio_framework_value_derive::{FromValue, ToValue};
use tokio::sync::broadcast;

//#region 🔖️Errors
#[derive(Debug, PartialEq, Eq)]
pub enum SyncError {
    Vcs(String),
    Actor(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vcs(detail) => write!(formatter, "vcs error: {detail}"),
            Self::Actor(detail) => write!(formatter, "actor error: {detail}"),
        }
    }
}

impl std::error::Error for SyncError {}
//#endregion 🔖️Errors

pub fn actor_identity_admission_admits(grant:semio_framework_value::RetainedCloneGrant)->bool{let demand=semio_framework_value::SharedUtf8::admission_demand();grant.maximum_items>=1&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_depth>=demand.depth}
fn actor_identity_batch_admits(count:usize,grant:semio_framework_value::RetainedCloneGrant)->bool{let demand=semio_framework_value::SharedUtf8::lease_demand();count<=grant.maximum_items&&count.checked_mul(demand.copy_bytes).is_some_and(|bytes|bytes<=grant.maximum_copy_bytes)&&grant.maximum_depth>=demand.depth}
pub fn add_actor_identity_progress(total:&mut semio_framework_value::RetainedCloneProgress,receipt:semio_framework_value::RetainedCloneProgress){total.copied_items=total.copied_items.checked_add(receipt.copied_items).expect("original identity item conservation overflow");total.copied_bytes=total.copied_bytes.checked_add(receipt.copied_bytes).expect("original identity copy conservation overflow");total.retained_capacity_bytes=total.retained_capacity_bytes.checked_add(receipt.retained_capacity_bytes).expect("original identity capacity conservation overflow");total.released_bytes=total.released_bytes.checked_add(receipt.released_bytes).expect("original identity release conservation overflow");}
pub fn wire_envelopes_with_original_actor(envelopes:&[MutationEnvelope],actor:&semio_framework_value::SharedUtf8,grant:semio_framework_value::RetainedCloneGrant)->Result<(Vec<MutationEnvelope>,semio_framework_value::RetainedCloneProgress),ValueError>{if !actor_identity_batch_admits(envelopes.len(),grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original socket identity batch exceeds caller lease authority"));}let mut total=semio_framework_value::RetainedCloneProgress::default();let mut rows=Vec::with_capacity(envelopes.len());for original in envelopes{let(actor,receipt)=actor.admit_clone(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:grant.maximum_copy_bytes-total.copied_bytes,..grant})?;add_actor_identity_progress(&mut total,receipt);rows.push(MutationEnvelope{mutation_id:original.mutation_id.clone(),document_id:original.document_id.clone(),actor:ActorId(actor),dependencies:original.dependencies.clone(),observed:original.observed.clone(),target:original.target.clone(),diff:original.diff.clone(),inverse:original.inverse.clone(),timestamp:original.timestamp,transaction:original.transaction.clone(),verb:original.verb.clone(),line:original.line.clone()});}Ok((rows,total))}

//#region 🔖️EnvelopeSerde
/// 🧵️ JSON worker seam: `MutationEnvelope` vectors as `encode_envelopes` bytes (not struct JSON).
mod envelope_serde {
    use semio_framework_os_spr::{decode_envelopes, encode_envelopes, MutationEnvelope};

    /// 🧵️ Byte framing for the `ArtifactActorMsg::LocalMutations`/`ArtifactEvent::
    /// RemoteMutations` `#[value(serialize_with = ..., deserialize_with = ...)]` field bridge —
    /// wire shape is a `DslValue::Array` of `DslValue::Number` (one per byte), matching what the
    /// former serde `serialize_seq` path produced.
    pub fn to_value(envelopes: &[MutationEnvelope]) -> semio_framework_value::DslValue {
        let bytes = encode_envelopes(envelopes);
        semio_framework_value::DslValue::Array(bytes.into_iter().map(|byte| semio_framework_value::DslValue::uint(byte as u64)).collect())
    }

    /// 🧵️ `FromValue` twin of `deserialize` above.
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<Vec<MutationEnvelope>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Array(items) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected array of bytes"));
        };
        let mut bytes = Vec::with_capacity(items.len());
        for item in items {
            let semio_framework_value::DslValue::Number(n) = item else {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected byte number"));
            };
            let Some(n) = n.as_u64() else {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected byte number"));
            };
            bytes.push(n as u8);
        }
        decode_envelopes(&bytes).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}
//#endregion 🔖️EnvelopeSerde

//#region 🔖️Protocol
/// 🗃️ A durable place a document synchronizes with. A document may bind to several at once
/// (folder-only, semio_hub-only, or both); the actor treats each as an independent peer.
/// 🗃️ Durability × sharing class for persistence bindings and replication lanes.
/// Preview/presence lanes are always `EphemeralShared` and must never be mistaken for durable WAL state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum PersistenceDataClass {
    PersistedLocalOnly,
    PersistedShared,
    EphemeralLocalOnly,
    EphemeralShared,
}

impl PersistenceDataClass {
    pub const fn is_durable(self) -> bool {
        matches!(self, Self::PersistedLocalOnly | Self::PersistedShared)
    }

    pub const fn is_shared(self) -> bool {
        matches!(self, Self::PersistedShared | Self::EphemeralShared)
    }

    pub const fn allows_share(self) -> bool {
        matches!(self, Self::PersistedShared)
    }

    pub const fn allows_collaboration(self) -> bool {
        matches!(self, Self::PersistedShared)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistedLocalOnly => "persistedLocalOnly",
            Self::PersistedShared => "persistedShared",
            Self::EphemeralLocalOnly => "ephemeralLocalOnly",
            Self::EphemeralShared => "ephemeralShared",
        }
    }
}

/// 🛤️ Classifies a wire lane. Preview and presence are broadcast-only (never WAL).
pub fn wire_lane_data_class(lane: &str) -> PersistenceDataClass {
    match lane {
        "preview" | "presence" => PersistenceDataClass::EphemeralShared,
        "command" => PersistenceDataClass::PersistedShared,
        _ => PersistenceDataClass::EphemeralLocalOnly,
    }
}

/// 🗃️ Resolves the data class of an actor's bindings. Empty bindings are ephemeral local-only
/// (in-memory draft / studio with no backbone).
pub fn bindings_data_class(bindings: &[PersistenceBinding]) -> PersistenceDataClass {
    match bindings.first() {
        None => PersistenceDataClass::EphemeralLocalOnly,
        Some(binding) => binding.data_class(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PersistenceBinding {
    /// 📁️ Local canonical store. A directory uses the multi-document `folder://` event log;
    /// a `*.json` path uses the single-blob `file://` export format.
    Folder { path: std::path::PathBuf },
    /// ☁️ A semio_hub node reachable over WebSocket
    /// (`remote://host:port` → `ws://host:port/scopes/{space_id}%2F{id}/document/ws`).
    Hub {
        base_url: String,
        space_id: String,
        /// 🎭️ Out-of-band presence scope (ticket 26/08/16/HUB-SPACES-…, contract §C0): rides
        /// as `?surface=` on the WS URL rather than a wire field — `PresencePeer`'s flag byte is
        /// already full. `<kind>@<standard>/<subset>#<role>`, e.g. `s.space.home@1/*#editor`.
        #[value(default, skip_serializing_if = "Option::is_none")]
        surface: Option<String>,
    },
}

impl PersistenceBinding {
    pub const fn data_class(&self) -> PersistenceDataClass {
        match self {
            Self::Folder { .. } => PersistenceDataClass::PersistedLocalOnly,
            Self::Hub { .. } => PersistenceDataClass::PersistedShared,
        }
    }
}

/// 🧾️ Everything {@link ArtifactHost::open} needs to spawn one document's actor.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactActorConfig {
    pub document_id: String,
    pub schema: String,
    pub bindings: Vec<PersistenceBinding>,
    /// 👁️ Watch the folder binding for external edits (other processes writing the file).
    #[value(default)]
    pub watch_external: bool,
    /// 🖋️ The authoring actor id used for semio_hub `Hello`/presence and operation origin filtering.
    pub actor: String,
    pub actor_identity_grant: semio_framework_value::RetainedCloneGrant,
}

/// 🗝️ Exact process-local identity for one open artifact actor. Hub documents are keyed by
/// their complete authority scope; local-only documents occupy a separate namespace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactDocumentKey {
    Local { document_id: String },
    Hub { space_id: String, document_id: String },
}

impl ArtifactDocumentKey {
    pub fn local(document_id: impl Into<String>) -> Self {
        Self::Local { document_id: document_id.into() }
    }

    pub fn hub(space_id: impl Into<String>, document_id: impl Into<String>) -> Self {
        Self::Hub { space_id: space_id.into(), document_id: document_id.into() }
    }

    pub fn for_config(config: &ArtifactActorConfig) -> Self {
        let mut hub_spaces = config.bindings.iter().filter_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.as_str()),
            PersistenceBinding::Folder { .. } => None,
        });
        let Some(space_id) = hub_spaces.next() else { return Self::local(config.document_id.clone()) };
        assert!(hub_spaces.all(|candidate| candidate == space_id), "one artifact actor cannot span hub spaces");
        Self::hub(space_id, config.document_id.clone())
    }

    pub fn document_id(&self) -> &str {
        match self {
            Self::Local { document_id } | Self::Hub { document_id, .. } => document_id,
        }
    }
}

/// 📨️ Caller → actor control messages, sent on the {@link ArtifactChannels} command channel.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactActorMsg {
    /// ⬆️ Wakes the actor to drain the store's outbound operations promptly. `envelopes` is a
    /// direct-injection fallback used only when no store is attached to the channel (empty = pure wake).
    LocalMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// 🪢️ One canonical Store `BackboneMessage::Mutations` retained byte-for-byte by a
    /// mounted document port until its authoritative Hub command acknowledgment.
    DocumentBackbone { message: Vec<u8> },
    /// 🗃️ One exact versioned root plus recursive-owned-member archive for durable persistence.
    LocalDocumentArchive { archive: Vec<u8> },
    /// 📡️ Broadcasts this peer's presence/selection to the semio_hub.
    PresenceHeartbeat { peer: Box<PresencePeer> },
    /// 👻️ Publishes an ephemeral, best-effort UI-state blob on the semio_hub's uncredited preview
    /// lane (`crate::os_spr::wire::ClientFrame::PreviewPublish`) — e.g. a drag ghost or live cursor;
    /// `seq` is a per-`key` monotone counter so a receiver can drop stale-arriving previews.
    PublishPreview { key: String, seq: u64, payload: Vec<u8> },
    /// 🛟️ The host re-seeded a rebootstrapping hub document ([`ArtifactEvent::RebootstrapRequired`]): the canonical
    /// checkpoint pair it verified against the control and already loaded into the guest. The actor adopts it as its baseline,
    /// hands its unacked local operations back to the guest and dials at once, saying Hello at the pair's baseline.
    Reseed { pack: Vec<u8>, spr: Vec<u8>, baseline: semio_framework_os_directory::ArtifactFrontier },
    /// 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,
    /// ✂️ Flushes any pending outbound operations, then stops the actor.
    Detach,
}

pub const ARTIFACT_MAILBOX_ITEMS: usize = 64;
pub const ARTIFACT_MAILBOX_BYTES: usize = semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 65_536;

#[derive(Debug)]
pub enum ArtifactMailboxSendError {
    Full { message: ArtifactActorMsg },
    Bytes { message: ArtifactActorMsg },
    Closed { message: ArtifactActorMsg },
    Stale { message: ArtifactActorMsg },
}

impl ArtifactMailboxSendError {
    pub fn into_message(self) -> ArtifactActorMsg {
        match self {
            Self::Full { message } | Self::Bytes { message } | Self::Closed { message } | Self::Stale { message } => message,
        }
    }
}

struct ArtifactMailboxSlot {
    pub generation: u64,
    pub bytes: usize,
    pub message: ArtifactActorMsg,
}

struct ArtifactMailboxState {
    pub slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS],
    pub head: usize,
    pub len: usize,
    pub bytes: usize,
    pub generation: u64,
    pub closed: bool,
    wake_armed: bool,
    waker: Option<std::task::Waker>,
    wake: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}

impl ArtifactMailboxState {
    fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0, bytes: 0, generation: 1, closed: false, wake_armed: false, waker: None, wake: None }
    }
}

struct ArtifactMailboxAuthority {
    pub state: std::sync::Mutex<ArtifactMailboxState>,
}

#[derive(Clone)]
pub struct ArtifactMailboxSender {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    generation: u64,
}

pub struct ArtifactMailboxReceiver {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    pub generation: u64,
}

#[derive(Clone)]
pub struct ArtifactMailboxClose {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    generation: u64,
}

impl ArtifactMailboxSender {
    pub fn send(&self, message: ArtifactActorMsg) -> Result<(), ArtifactMailboxSendError> {
        let Some(bytes) = artifact_actor_message_bytes(&message) else { return Err(ArtifactMailboxSendError::Bytes { message }) };
        let (waker, wake) = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation {
                return Err(ArtifactMailboxSendError::Stale { message });
            }
            if state.closed {
                return Err(ArtifactMailboxSendError::Closed { message });
            }
            if state.len == ARTIFACT_MAILBOX_ITEMS {
                return Err(ArtifactMailboxSendError::Full { message });
            }
            if bytes > ARTIFACT_MAILBOX_BYTES.saturating_sub(state.bytes) {
                return Err(ArtifactMailboxSendError::Bytes { message });
            }
            let index = (state.head + state.len) % ARTIFACT_MAILBOX_ITEMS;
            state.slots[index] = Some(ArtifactMailboxSlot { generation: self.generation, bytes, message });
            state.len += 1;
            state.bytes += bytes;
            let first_ready = !state.wake_armed;
            state.wake_armed = true;
            (state.waker.take(), first_ready.then(|| state.wake.clone()).flatten())
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        if let Some(wake) = wake {
            wake();
        }
        Ok(())
    }
}

impl ArtifactMailboxReceiver {
    pub fn try_recv(&self) -> Option<ArtifactActorMsg> {
        let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.generation != state.generation || state.len == 0 {
            return None;
        }
        let head = state.head;
        let slot = state.slots[head].take().expect("artifact mailbox FIFO owner disappeared");
        state.head = (head + 1) % ARTIFACT_MAILBOX_ITEMS;
        state.len -= 1;
        state.bytes -= slot.bytes;
        if state.len == 0 {
            state.wake_armed = false;
        }
        (slot.generation == self.generation).then_some(slot.message)
    }

    pub async fn recv(&self) -> Option<ArtifactActorMsg> {
        std::future::poll_fn(|context| {
            if let Some(message) = self.try_recv() {
                return std::task::Poll::Ready(Some(message));
            }
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation || state.closed {
                return std::task::Poll::Ready(None);
            }
            if state.len != 0 {
                drop(state);
                return std::task::Poll::Ready(self.try_recv());
            }
            state.waker = Some(context.waker().clone());
            std::task::Poll::Pending
        })
        .await
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_wake(&self, wake: std::sync::Arc<dyn Fn() + Send + Sync>) {
        let ready = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation || state.closed {
                return;
            }
            state.wake = Some(wake.clone());
            state.len != 0
        };
        if ready {
            wake();
        }
    }

    pub fn close(&self) {
        self.close_handle().close();
    }

    pub fn close_one(&self) -> bool {
        self.close_handle().close_one()
    }

    pub fn close_handle(&self) -> ArtifactMailboxClose {
        ArtifactMailboxClose { authority: self.authority.clone(), generation: self.generation }
    }
}

impl ArtifactMailboxClose {
    pub fn close(&self) {
        let waker = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation {
                return;
            }
            state.closed = true;
            state.generation = state.generation.wrapping_add(1);
            state.wake_armed = false;
            state.wake = None;
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn close_one(&self) -> bool {
        let owner = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.len == 0 {
                return false;
            }
            let head = state.head;
            let slot = state.slots[head].take();
            state.head = (head + 1) % ARTIFACT_MAILBOX_ITEMS;
            state.len -= 1;
            if let Some(slot) = &slot {
                state.bytes -= slot.bytes;
            }
            if state.len == 0 {
                state.wake_armed = false;
            }
            slot
        };
        drop(owner);
        true
    }

    pub fn has_pending(&self) -> bool {
        self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len != 0
    }
}

pub fn artifact_mailbox_pair() -> (ArtifactMailboxSender, ArtifactMailboxReceiver) {
    let authority = std::sync::Arc::new(ArtifactMailboxAuthority { state: std::sync::Mutex::new(ArtifactMailboxState::new()) });
    (ArtifactMailboxSender { authority: authority.clone(), generation: 1 }, ArtifactMailboxReceiver { authority, generation: 1 })
}

pub fn artifact_actor_message_bytes(message: &ArtifactActorMsg) -> Option<usize> {
    fn field(bytes: usize) -> Option<usize> {
        4usize.checked_add(bytes)
    }
    fn add(total: &mut usize, bytes: usize) -> Option<()> {
        *total = total.checked_add(bytes)?;
        (*total <= ARTIFACT_MAILBOX_BYTES).then_some(())
    }
    fn text(total: &mut usize, value: &str) -> Option<()> {
        add(total, field(value.len())?)
    }
    fn optional_text(total: &mut usize, value: Option<&String>) -> Option<()> {
        add(total, 1)?;
        if let Some(value) = value {
            text(total, value)?;
        }
        Some(())
    }
    fn string_vec(total: &mut usize, values: &[String]) -> Option<()> {
        add(total, 4)?;
        for value in values {
            text(total, value)?;
        }
        Some(())
    }
    let mut bytes = 1usize;
    match message {
        ArtifactActorMsg::LocalMutations { envelopes } => {
            add(&mut bytes, 4)?;
            for envelope in envelopes {
                text(&mut bytes, &envelope.mutation_id.0)?;
                text(&mut bytes, &envelope.document_id.0)?;
                text(&mut bytes, &envelope.actor.0)?;
                add(&mut bytes, 4)?;
                for dependency in &envelope.dependencies {
                    text(&mut bytes, &dependency.0)?;
                }
                add(&mut bytes, 1)?;
                if let Some(observed) = &envelope.observed {
                    text(&mut bytes, &observed.0)?;
                }
                add(&mut bytes, 4)?;
                for segment in &envelope.target {
                    text(&mut bytes, segment)?;
                }
                text(&mut bytes, &envelope.diff.schema.0)?;
                add(&mut bytes, field(envelope.diff.payload.len())?)?;
                text(&mut bytes, &envelope.inverse.schema.0)?;
                add(&mut bytes, field(envelope.inverse.payload.len())?)?;
                add(&mut bytes, 24)?;
            }
        }
        ArtifactActorMsg::DocumentBackbone { message } => {
            if message.len() > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || decode_document_backbone_message_exact(message).is_err() {
                return None;
            }
            add(&mut bytes, field(message.len())?)?;
        }
        ArtifactActorMsg::LocalDocumentArchive { archive } => {
            if archive.is_empty() || archive.len() > semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES || archive[0] != 1 {
                return None;
            }
            add(&mut bytes, field(archive.len())?)?;
        }
        ArtifactActorMsg::PresenceHeartbeat { peer } => {
            text(&mut bytes, &peer.actor)?;
            add(&mut bytes, 8)?;
            optional_text(&mut bytes, peer.label.as_ref())?;
            add(&mut bytes, 1)?;
            if let Some(pack) = &peer.presence_pack {
                add(&mut bytes, field(pack.len())?)?;
            }
            optional_text(&mut bytes, peer.user_id.as_ref())?;
            optional_text(&mut bytes, peer.role.as_ref())?;
            optional_text(&mut bytes, peer.drag_ghost_json.as_ref())?;
            add(&mut bytes, 1)?;
            if let Some(interaction) = &peer.interaction {
                text(&mut bytes, &interaction.app_id)?;
                add(&mut bytes, 4)?;
                for domain in &interaction.domains {
                    text(&mut bytes, &domain.domain)?;
                    text(&mut bytes, &domain.granularity)?;
                    string_vec(&mut bytes, &domain.selected)?;
                    string_vec(&mut bytes, &domain.hovered)?;
                }
            }
            add(&mut bytes, 1)?;
            if peer.color.is_some() {
                add(&mut bytes, 1)?;
            }
            optional_text(&mut bytes, peer.surface.as_ref())?;
            add(&mut bytes, 4)?;
            for view in &peer.views {
                text(&mut bytes, &view.window_id)?;
                text(&mut bytes, &view.space)?;
                add(&mut bytes, 1)?;
                add(
                    &mut bytes,
                    match &view.kind {
                        semio_framework_os_spr::PresenceViewKind::Canvas { .. } => 24,
                        semio_framework_os_spr::PresenceViewKind::Orbit { .. } => 80,
                        semio_framework_os_spr::PresenceViewKind::Geo { .. } => 40,
                    },
                )?;
                add(&mut bytes, 17)?;
                if view.pointer.is_some() {
                    add(&mut bytes, 24)?;
                }
            }
            add(&mut bytes, 1)?;
            if let Some(ui) = &peer.ui {
                optional_text(&mut bytes, ui.hovered_path.as_ref())?;
                optional_text(&mut bytes, ui.focused_path.as_ref())?;
                optional_text(&mut bytes, ui.pressed_path.as_ref())?;
            }
            add(&mut bytes, 1)?;
            if let Some(tool_run) = &peer.tool_run {
                text(&mut bytes, &tool_run.tool_id)?;
                add(&mut bytes, 1 + 2 + 8 + 1)?;
                if tool_run.total.is_some() {
                    add(&mut bytes, 8)?;
                }
            }
        }
        ArtifactActorMsg::PublishPreview { key, seq: _, payload } => {
            text(&mut bytes, key)?;
            add(&mut bytes, 8)?;
            add(&mut bytes, field(payload.len())?)?;
        }
        ArtifactActorMsg::Reseed { pack, spr, baseline } => {
            add(&mut bytes, field(pack.len())?)?;
            add(&mut bytes, field(spr.len())?)?;
            text(&mut bytes, &baseline.document_id)?;
            text(&mut bytes, &baseline.head_edit_id)?;
            add(&mut bytes, 48)?;
        }
        ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
    }
    (bytes <= ARTIFACT_MAILBOX_BYTES).then_some(bytes)
}

/// 📶️ Connection state of a document's remote (semio_hub) transport.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RemoteState {
    Detached,
    Connecting,
    Live { peer_count: usize },
    Backoff { retry_in_ms: u64 },
}

/// 🚦️ Snapshot of a document's sync health for status badges.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactSyncStatus {
    pub persisted: bool,
    pub pending_mutations: usize,
    pub remote: RemoteState,
    /// 📌️ The edited head the hub acknowledged, present only while nothing local is unacknowledged —
    /// the head a hub Check In names. `None` before the first edit and while mutations are in flight.
    pub acknowledged_head: Option<semio_framework_os_directory::EditedArtifactFrontierV1>,
}

impl Default for ArtifactSyncStatus {
    fn default() -> Self {
        Self { persisted: false, pending_mutations: 0, remote: RemoteState::Detached, acknowledged_head: None }
    }
}

/// 📬️ Actor → subscriber events, delivered on the broadcast channel from {@link ArtifactHost::subscribe}.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactEvent {
    /// 🕸️ Remote operations (semio_hub fan-out or appended external edits) — also pushed into the store's
    /// inbound queue so `store.tick()` materializes them.
    RemoteMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// 🪢️ One canonical Hub mutation batch for an exact mounted guest document port.
    DocumentBackbone { message: Vec<u8> },
    /// 🗃️ The whole root plus recursive owned-member closure was replaced atomically.
    DocumentArchiveReplaced { archive: Vec<u8> },
    /// 🫙️ The bound folder holds no archive of this document: answered once per open, by the first answered folder read alone,
    /// and never after an archive was read or written.
    DocumentArchiveAbsent,
    /// 📈️ Monotonic, bounded progress for one descriptor-bound artifact bootstrap.
    /// A new transfer starts at zero after reconnect; no progress event implies a committed frontier.
    BootstrapProgress { received_bytes: u64, total_bytes: u64, received_chunks: u32, total_chunks: u32 },
    /// 🚦️ Sync status changed.
    Status(ArtifactSyncStatus),
    /// 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// 🛟️ The hub made this document rebuild from its canonical checkpoint (fan-out lag, an approval checkpoint): the actor
    /// dropped its projection and socket, keeps its unacked work and waits for the host's [`ArtifactActorMsg::Reseed`] with the
    /// pair `control` names — the pair only the host's directory client fetches and only the host can load into the guest.
    RebootstrapRequired { control: semio_framework_os_directory::RebootstrapRequired },
    /// 🎨️ The hub assigned (or re-confirmed, on reconnect) this connection's session color —
    /// `crate::os_spr::wire::ServerFrame::Session`, sent once per connection after `Welcome`. The
    /// actor stores it and stamps it onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
    Session { actor: String, color: u8 },
    /// 👻️ A peer published an ephemeral preview blob (`crate::os_spr::wire::ServerFrame::Preview`)
    /// on the uncredited, loss-tolerant preview lane — the counterpart of
    /// {@link ArtifactActorMsg::PublishPreview}.
    Preview { actor: semio_framework_value::SharedUtf8, key: String, seq: u64, payload: Vec<u8> },
    /// 📮️ The semio_hub's terminal disposition for one outbound `Commands` batch
    /// (`crate::os_spr::wire::ServerFrame::Ack`'s `Applied` stage) — accepted as-is, transformed against
    /// concurrent history (the transformed envelope is already delivered as a
    /// {@link ArtifactEvent::RemoteMutations} replacing the speculative local one), or rejected
    /// (the speculative local head is rolled back via {@link rollback_envelope} before this fires).
    CommandOutcome {
        #[value(rename = "batchId")]
        batch_id: u64,
        outcome: CommandAckOutcome,
    },
    /// ⚠️ A structural conflict (external divergence with local pending operations / semio_hub
    /// protocol-level reject), on the frozen diagnostic-bag vocabulary (contract freeze `26/08/16/
    /// MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10) rather than the deleted
    /// per-domain conflict-bag type this used to wrap: `level` is `Error` (this actor has no finer
    /// severity signal from either source), `target` carries the diverged folder uri / hub base url.
    /// This is a transport-level diagnostic, never a first-class `crate::os_spr::Conflict` (no
    /// `id`/`status`/`actors`/`timestamp` exists for either source event).
    Conflict(MutationMessage),
}

pub fn decode_document_backbone_message_exact(message: &[u8]) -> Result<Vec<MutationEnvelope>, String> {
    match decode_hot_backbone_message_exact(message).map_err(|error| error.to_string())? {
        BackboneMessage::Mutations { envelopes } => decode_document_backbone_envelopes_exact(&envelopes).map_err(|error| error.to_string()),
        BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } | BackboneMessage::Member { .. } | BackboneMessage::Retract { .. } => Err("document backbone requires a canonical mutation message".into()),
    }
}

#[derive(Default)]
pub struct DocumentBackboneRetentionV1 {
    pub entries: std::collections::HashMap<String, (usize, usize)>,
    pub bytes: usize,
    pub messages: usize,
}

impl DocumentBackboneRetentionV1 {
    pub fn retain(&mut self, message_bytes: usize, envelopes: &[MutationEnvelope]) -> Result<(), CommandAckOutcome> {
        if envelopes.is_empty() {
            return Ok(());
        }
        let refused = |code, reason: &str, limit: usize| CommandAckOutcome::local_rejected(code, reason, CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), bytes: Some(message_bytes as u64), limit: Some(limit as u64) });
        if message_bytes == 0 || message_bytes > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone message exceeds its hot byte limit", BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES));
        }
        let Some(next_bytes) = self.bytes.checked_add(message_bytes) else {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone byte accounting overflow", BACKBONE_CHANNEL_MAXIMUM_BYTES));
        };
        if self.messages >= BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > BACKBONE_CHANNEL_MAXIMUM_BYTES {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone pending capacity", BACKBONE_CHANNEL_MAXIMUM_BYTES));
        }
        let mut admitted = std::collections::HashSet::with_capacity(envelopes.len());
        if envelopes.iter().any(|envelope| !admitted.insert(envelope.mutation_id.0.as_str()) || self.entries.contains_key(&envelope.mutation_id.0)) {
            return Err(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneDuplicate, "document backbone mutation identity is already retained", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
        }
        for (index, envelope) in envelopes.iter().enumerate() {
            self.entries.insert(envelope.mutation_id.0.clone(), if index == 0 { (message_bytes, 1) } else { (0, 0) });
        }
        self.bytes = next_bytes;
        self.messages += 1;
        Ok(())
    }

    pub fn release(&mut self, envelopes: &[MutationEnvelope]) {
        for envelope in envelopes {
            if let Some((bytes, messages)) = self.entries.remove(&envelope.mutation_id.0) {
                self.bytes = self.bytes.checked_sub(bytes).expect("document backbone retained byte ledger underflow");
                self.messages = self.messages.checked_sub(messages).expect("document backbone retained message ledger underflow");
            }
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
        self.messages = 0;
    }
}

pub const ARTIFACT_BOOTSTRAP_DEADLINE_MS: u64 = 15_000;

/// 🧬️ Binds a public artifact bootstrap to the already-selected document codec before
/// allocating or accepting any payload bytes.
pub fn validate_artifact_bootstrap_identity(bootstrap: &ArtifactBootstrap, document_id: &str, schema: &str, pack_schema_hash: [u8; 32], server_frontier: &RuntimeFrontierSummary) -> Result<(), String> {
    if bootstrap.artifact_schema != schema {
        return Err(format!("artifact schema mismatch: expected {schema:?}, got {:?}", bootstrap.artifact_schema));
    }
    if bootstrap.baseline_frontier.document_id.0 != document_id || bootstrap.required_tail_frontier.document_id.0 != document_id || server_frontier.document_id.0 != document_id {
        return Err("artifact bootstrap document mismatch".into());
    }
    if bootstrap.pack_schema_hash != pack_schema_hash || pack_schema_hash == [0u8; 32] {
        return Err("artifact bootstrap pack schema mismatch".into());
    }
    if &bootstrap.required_tail_frontier != server_frontier {
        return Err("artifact bootstrap required tail does not match welcome frontier".into());
    }
    Ok(())
}


/// 📌️ The hub's frontier in the Check In head grammar; `None` for a genesis (never edited) frontier.
fn acknowledged_edited_head(frontier: &RuntimeFrontierSummary) -> Option<semio_framework_os_directory::EditedArtifactFrontierV1> {
    let chain_sha256: String = frontier.chain_hash.iter().map(|byte| format!("{byte:02x}")).collect();
    let head = semio_framework_os_directory::EditedArtifactFrontierV1 { document_id: frontier.document_id.0.clone(), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_sha256 };
    head.validate().then_some(head)
}

pub fn frontier_reaches(actual: &RuntimeFrontierSummary, required: &RuntimeFrontierSummary) -> bool {
    actual == required
}

/// 🧾️ Every class of refused command batch, one closed set for every producer: the hub (`hub.refused`, or `hub.unreadable`
/// when its diagnostics do not decode) and an actor's own refusals before any frame leaves it. Schema
/// `🧬️schema/🔣️command-rejection/🔣️.json`, TypeScript twin `COMMAND_REJECTION_CODES_V1` (`💻️os/🟦️.ts`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum CommandRejectionCode {
    #[value(rename = "hub.refused")]
    HubRefused,
    #[value(rename = "hub.unreadable")]
    HubUnreadable,
    #[value(rename = "local.read-only")]
    LocalReadOnly,
    #[value(rename = "local.queue-full")]
    LocalQueueFull,
    #[value(rename = "local.backbone-capacity")]
    LocalBackboneCapacity,
    #[value(rename = "local.backbone-duplicate")]
    LocalBackboneDuplicate,
    #[value(rename = "local.backbone-pair-unavailable")]
    LocalBackbonePairUnavailable,
    #[value(rename = "local.backbone-scope-mismatch")]
    LocalBackboneScopeMismatch,
    #[value(rename = "local.backbone-malformed")]
    LocalBackboneMalformed,
    #[value(rename = "local.socket-frame-ceiling")]
    LocalSocketFrameCeiling,
}

/// 🔢️ The typed counters a local refusal reports: the refused batch's envelopes, the bytes it or its queue holds, and the
/// limit it met — TypeScript twin `CommandRejectionDetailV1`.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
pub struct CommandRejectionDetail {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub envelopes: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

/// ⚖️ The client-side twin of `crate::os_spr::wire::ApplyOutcome`, minus the `Transformed`
/// envelope payload (already delivered separately as {@link ArtifactEvent::RemoteMutations} by
/// the time this fires — see {@link ArtifactEvent::CommandOutcome}). `Rejected` is the one rejection contract every
/// producer answers (`🧬️schema/🔣️command-rejection/🔣️.json`): its class, the producer's English diagnostic `reason` (never
/// shown to a human), the structured `MutationMessage`s the hub graded the batch with (decoded once, empty for a local
/// refusal) and a local refusal's counters.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CommandAckOutcome {
    Accepted,
    Transformed,
    Rejected {
        code: CommandRejectionCode,
        reason: String,
        messages: Vec<MutationMessage>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        detail: Option<CommandRejectionDetail>,
    },
}

impl CommandAckOutcome {
    /// 🧾️ The hub's `ApplyOutcome::Rejected`, decoded exactly once: its `messages` bytes are the UTF-8 JSON of one
    /// `MutationMessage` array (`🌎️hub` `encode_messages`; empty for an admission or transport refusal), read into
    /// `hub.refused`, or `hub.unreadable` with no messages when they are not that array.
    pub fn hub_rejected(reason: String, messages: &[u8]) -> Self {
        let decoded = if messages.is_empty() { Some(Vec::new()) } else { std::str::from_utf8(messages).ok().and_then(|text| semio_framework_pack_json::from_json_str::<Vec<MutationMessage>>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()) };
        match decoded {
            Some(messages) => Self::Rejected { code: CommandRejectionCode::HubRefused, reason, messages, detail: None },
            None => Self::Rejected { code: CommandRejectionCode::HubUnreadable, reason, messages: Vec::new(), detail: None },
        }
    }

    /// 🚫️ A refusal an actor answers before any frame leaves it: no hub messages, only its typed counters (none when all
    /// are absent).
    pub fn local_rejected(code: CommandRejectionCode, reason: &str, detail: CommandRejectionDetail) -> Self {
        Self::Rejected { code, reason: reason.to_string(), messages: Vec::new(), detail: (detail != CommandRejectionDetail::default()).then_some(detail) }
    }
}
//#endregion 🔖️Protocol

//#region 🔖️BackboneWorkerWire
/// 🧵️ Binary worker seam: `MAGIC` + `crate::os_pack::pack_rt::encode_wire_value` over a `DslValue`
/// tree (serde-shaped), shared by the wasm `store_worker` and `🏪️store/👷️worker/🟦️.ts`.
pub mod backbone_worker_wire {
    use crate::{ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, PersistenceBinding};
    
    use semio_framework_value_derive::{FromValue, ToValue};

    pub const MAGIC: u8 = 0x01;

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerRequest {
        Open {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
            schema: String,
            bindings: Vec<PersistenceBinding>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            watch_external: Option<bool>,
            actor: String,
            actor_identity_grant: semio_framework_value::RetainedCloneGrant,
        },
        Close {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
        },
        Send {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
            message: Box<ArtifactActorMsg>,
        },
    }

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerResponse {
        Event { document_id: String, client_instance_id: String, event: ArtifactEvent },
        Ready,
    }

    impl BackboneWorkerRequest {
        pub async fn actor_config(&self) -> Option<ArtifactActorConfig> {
            let Self::Open { document_id, schema, bindings, watch_external, actor, actor_identity_grant, .. } = self else {
                return None;
            };
            Some(ArtifactActorConfig { document_id: document_id.clone(), schema: schema.clone(), bindings: bindings.clone(), watch_external: watch_external.unwrap_or(true), actor: actor.clone(), actor_identity_grant: *actor_identity_grant })
        }
    }

    pub fn encode_request(request: &BackboneWorkerRequest) -> Vec<u8> {
        let dsl = semio_framework_value::ToValue::to_value(request);
        let mut bytes = vec![MAGIC];
        bytes.extend(semio_framework_os_pack::pack_rt::encode_wire_value(&dsl));
        bytes
    }

    pub async fn decode_request(bytes: &[u8]) -> Result<BackboneWorkerRequest, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = semio_framework_os_pack::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        semio_framework_value::FromValue::from_value(dsl).map_err(|error| error.to_string())
    }

    pub fn encode_response(response: &BackboneWorkerResponse) -> Vec<u8> {
        let dsl = semio_framework_value::ToValue::to_value(response);
        let mut bytes = vec![MAGIC];
        bytes.extend(semio_framework_os_pack::pack_rt::encode_wire_value(&dsl));
        bytes
    }

    pub async fn decode_response(bytes: &[u8]) -> Result<BackboneWorkerResponse, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = semio_framework_os_pack::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        semio_framework_value::FromValue::from_value(dsl).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️BackboneWorkerWire

//#region 🔖️Endpoints
// 🧱️ `ArtifactId` names the document of every envelope `envelopes_from_history_edit` rebuilds off an spr
// log, which both actors (native and browser) read.
use semio_framework_os_spr::ArtifactId;

/// 🆔️ One `HistoryEdit`'s op ids, matching `crate::os_spr::mutation_envelope_from_edit`'s own
/// fallback convention (`meta[i].op_id` when present, else `"{edit.id}#{i}"`) so this is the SAME
/// id a live-dispatched envelope for this edit already carries. This is the ONE id domain the
/// actor's dedup set uses — computed here and ONLY here, so a locally-flushed edit's spr entry
/// and the envelope built from re-reading that same spr always agree, fixing the actor's old
/// self-re-ingest bug (mixing envelope op ids with raw JSON edit ids) by construction.
async fn op_ids_of(edit: &semio_framework_os_spr::HistoryEdit) -> Vec<String> {
    match &edit.meta {
        Some(metas) if metas.len() == edit.ops.len() => metas.iter().enumerate().map(|(index, meta)| meta.op_id.clone().unwrap_or_else(|| format!("{}#{index}", edit.id))).collect(),
        _ => (0..edit.ops.len()).map(|index| format!("{}#{index}", edit.id)).collect(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn document_archive_hash(archive: &[u8]) -> String {
    semio_framework_hash::hash_bytes(archive)
}

/// 🆔️ Every event id in an spr byte log — each edit's op ids and each transition's id — the
/// actor's dedup/known-ids set, read directly off the binary history (NEVER via
/// `parse_document_spr`, whose meta-absent branch mints fresh random ids on every read).
#[cfg(not(target_arch = "wasm32"))]
pub async fn spr_op_ids(spr: &[u8]) -> Result<std::collections::HashSet<String>, String> {
    let log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| error.to_string())?;
    let mut ids = std::collections::HashSet::new();
    for edit in &log.edits {
        ids.extend(op_ids_of(edit).await);
    }
    ids.extend(log.transitions.iter().map(|transition| transition.id.clone()));
    Ok(ids)
}

/// 📜️ Every event an spr byte log persists, as the causal envelopes a store ingests: each
/// edit's operations in log order, then every history transition — a persisted document IS its
/// genesis pack plus exactly these events.
pub async fn spr_events(spr: &[u8], document_id: &str, schema: &str, actor_identity_grant: semio_framework_value::RetainedCloneGrant) -> Result<Vec<MutationEnvelope>, String> {
    let log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| error.to_string())?;
    let mut events = Vec::new();
    for edit in &log.edits {
        events.extend(envelopes_from_history_edit(edit, document_id, schema, actor_identity_grant).await?);
    }
    events.extend(log.transitions.iter().map(|transition| transition.to_envelope(document_id)));
    Ok(events)
}

/// 📦️ Rebuilds real {@link MutationEnvelope}s (one per forward op, genuine `OpBinary`
/// payloads straight from the edit's own binary `OpPayload`s — no codec, no JSON) from one
/// `HistoryEdit` decoded off the spr bytes, so an appended external edit can flow through the
/// store's causal DAG (`ingest_remote` → `edit_from_operation_envelope`). A binary-less op payload
/// is a hard error — `.spr` is binary-only since B1, so every real op has one.
pub async fn envelopes_from_history_edit(edit: &semio_framework_os_spr::HistoryEdit, document_id: &str, schema: &str, actor_identity_grant: semio_framework_value::RetainedCloneGrant) -> Result<Vec<MutationEnvelope>, String> {
    if !actor_identity_batch_admits(edit.ops.len(),actor_identity_grant){return Err("original history identity batch exceeds caller lease authority".into());}
    for index in 0..edit.ops.len(){if edit.meta.as_ref().and_then(|metas|metas.get(index)).and_then(|meta|meta.author_id.as_ref()).or(edit.actor.as_ref()).is_none(){return Err(format!("edit {} op {index} has no authentic actor identity",edit.id));}}
    let op_ids = op_ids_of(edit).await;
    let mut identity_copied=0;
    let mut envelopes = Vec::with_capacity(edit.ops.len());
    for (index, op) in edit.ops.iter().enumerate() {
        let payload = op.binary.clone().ok_or_else(|| format!("edit {} op {index} has no binary payload", edit.id))?;
        let meta = edit.meta.as_ref().and_then(|metas| metas.get(index));
        let dependencies = meta.map(|m| m.dependencies.iter().cloned().map(MutationId).collect()).unwrap_or_default();
        let original_actor=meta.and_then(|meta|meta.author_id.as_ref()).or(edit.actor.as_ref()).expect("original actor presence validated before envelope birth");
        let(actor,receipt)=original_actor.admit_clone(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:actor_identity_grant.maximum_copy_bytes-identity_copied,..actor_identity_grant}).map_err(ValueError::into_message)?;identity_copied+=receipt.copied_bytes;
        let timestamp = match meta.and_then(|meta| meta.hlt) {
            Some((actor, physical_ms, logical)) => semio_framework_os_spr::HybridLogicalTimestamp { actor, physical_ms: u64::try_from(physical_ms).map_err(|_| format!("edit {} op {index} has a negative hybrid-clock physical time", edit.id))?, logical },
            None => semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0),
        };
        let inverse_payload = edit.inverse.get(index).and_then(|p| p.binary.clone()).unwrap_or_default();
        envelopes.push(MutationEnvelope {
            mutation_id: MutationId(op_ids[index].clone()),
            document_id: ArtifactId(document_id.to_string()),
            actor: ActorId(actor),
            dependencies,
            observed: None,
            target: Vec::new(),
            diff: semio_framework_os_spr::ArtifactDiff { schema: semio_framework_os_spr::SchemaId(schema.to_string()), payload },
            inverse: semio_framework_os_spr::InverseMutation { schema: semio_framework_os_spr::SchemaId(schema.to_string()), payload: inverse_payload },
            timestamp,
            transaction: None, verb: edit.verb.clone(), line: edit.line.clone(),
        });
    }
    Ok(envelopes)
}

/// 📦️ The inverse of {@link envelopes_from_history_edit}: one `MutationEnvelope` -> one
/// `HistoryEdit` with a real binary `OpPayload` and populated meta (`op_id` == the envelope's own
/// id, so a later `op_ids_of` re-read agrees) — the byte-level twin of
/// `crate::os_store::ledger::edit_from_operation_envelope`, for appending a locally-flushed envelope to the spr log.
///
/// 🎞️ `crate::os_spr::causal::MutationEnvelope` (this fn's input) carries no group_id —
/// same precedent as its already-absent `semantic_kind`/`label`/`undo_policy`, see
/// `command::MutationMeta.group_id`'s doc comment. A remote-ingested edit is therefore
/// never a recognized composite-gesture member; group undo degrades it to a foreign/
/// solitary edit, matching how this whole envelope is already `undo_policy: 0`-flattened.
///
/// 🔀️ A remote-ingested envelope carries no provenance of its own: the authoring peer
/// already resolved any contribution or transaction locally and shipped the resulting
/// OWNER ops, so this side records exactly what it receives — an owner edit.
#[cfg(not(target_arch = "wasm32"))]
pub async fn history_edit_from_envelope(envelope: &MutationEnvelope) -> semio_framework_os_spr::HistoryEdit {
    semio_framework_os_spr::HistoryEdit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        started_at: now_ms().await.to_string(),
        finished_at: None, verb: envelope.verb.clone(), line: envelope.line.clone(),
        ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(envelope.diff.payload.clone()) }],
        inverse: if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(envelope.inverse.payload.clone()) }] },
        meta: Some(vec![semio_framework_os_spr::HistoryOpMeta {
            op_id: Some(envelope.mutation_id.0.clone()),
            dependencies: envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect(),
            base_version: 0,
            author_id: Some(envelope.actor.0.clone()),
            hlt: Some((envelope.timestamp.actor, envelope.timestamp.physical_ms as i64, envelope.timestamp.logical)),
            undo_policy: 0,
            payload_hash: None,
            group_id: None,
            origin: semio_framework_os_spr::command::MutationOrigin::Owner,
            messages: Vec::new(),
            transaction: envelope.transaction.clone(),
        }]),
        lane: None,
    }
}

/// 🔗️ Derives a semio_hub WebSocket URL: `remote://host:port` (or `http(s)://`, `ws(s)://`) →
/// `ws(s)://host:port/scopes/{space_id}%2F{document_id}/document/ws`, with an out-of-band
/// `?surface=` appended when the binding carries one (contract §C0's presence scope, ticket
/// 26/08/16/HUB-SPACES-…: `(space_id, document_id, surface)` — `surface` rides outside the wire
/// protocol rather than widening `PresencePeer`'s already-full flag byte). Actor identity is never
/// a query parameter — the server binds it from the session credential.
pub async fn hub_ws_url(base_url: &str, space_id: &str, document_id: &str, surface: Option<&str>) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    let scope = semio_framework_os_directory_client::encode_url_component(&format!("{space_id}/{document_id}"));
    match surface {
        Some(surface) => format!("{scheme}://{authority}/scopes/{scope}/document/ws?surface={}", semio_framework_os_directory_client::encode_url_component(surface)),
        None => format!("{scheme}://{authority}/scopes/{scope}/document/ws"),
    }
}
//#endregion 🔖️Endpoints

//#region 🔖️DocumentSocketConnect
/// 🧬️ The pack schema identity a document socket's hello and bootstrap are checked against: the
/// kind's codec when this process resolves one ([`crate::os_store::codec_registry::document_kind_codec`] — the linked
/// Rust codec, else the codec of the mounted component that owns the kind, asked of that component
/// exactly as the hub's trusted catalog asks it), otherwise the verified execution-target lease's — the
/// hub-selected package a client that mounted no owning component will run. Neither is no identity at
/// all, and the caller refuses to dial rather than guess.
pub async fn document_pack_schema_hash(schema: &str, lease: Option<&semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> Option<[u8; 32]> {
    if let Ok(Some(codec)) = semio_framework_os_store_codec_registry::document_kind_codec(schema).await {
        return codec.pack_schema_hash().await.ok();
    }
    let lease = lease.filter(|lease| lease.artifact.schema == schema)?;
    semio_framework_os_directory_client::decode_lower_hex_32(&lease.artifact.pack_schema_hash)
}

/// 🎫️ What a document actor asks of the hub's open plan for its binding.
pub fn document_socket_expectation(schema: &str, pack_schema_hash: [u8; 32], surface: Option<&str>, lease: Option<&semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> semio_framework_os_directory_client::DocumentSocketExpectationV1 {
    semio_framework_os_directory_client::DocumentSocketExpectationV1 { artifact_schema: schema.to_string(), pack_schema_hash, requested_surface_id: surface.map(str::to_string), lease: lease.cloned() }
}

/// 🧷️ The one binding a document actor holds a hub socket for.
#[derive(Clone, Copy, Debug)]
pub struct DocumentSocketBinding<'a> {
    pub hub_base_url: &'a str,
    pub space_id: &'a str,
    pub document_id: &'a str,
    pub schema: &'a str,
    pub pack_schema_hash: [u8; 32],
    pub surface: Option<&'a str>,
    pub lease: Option<&'a semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
}

/// ⚖️ Whether one admitted authority may carry this binding at `now_ms`: unexpired, from the bound
/// hub origin, for exactly this scope, schema, pack identity and surface, and — when the client verified
/// an execution-target lease — for exactly that lease.
pub fn document_socket_authority_admits(authority: &semio_framework_os_directory_client::DocumentSocketAuthorityV1, binding: &DocumentSocketBinding<'_>, now_ms: u64) -> bool {
    authority.expires_at_unix_ms > now_ms
        && authority.hub_origin.trim_end_matches('/') == binding.hub_base_url.trim_end_matches('/')
        && authority.scope.space_id == binding.space_id
        && authority.scope.document_id == binding.document_id
        && authority.artifact.schema == binding.schema
        && authority.pack_schema_hash == binding.pack_schema_hash
        && binding.surface.is_none_or(|surface| surface == authority.surface.surface_id)
        && binding.lease.is_none_or(|lease| authority.matches_lease_fields(lease))
}

/// 🪢️ A canonical pair's public baseline in the runtime frontier grammar a `SocketHelloV1` advertises.
pub fn canonical_pair_baseline(frontier: &semio_framework_os_directory::ArtifactFrontier) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: ArtifactId(frontier.document_id.clone()), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_hash: frontier.chain_hash.0 }
}

/// 🛟️ A wire `RebootstrapRequired` control as the directory contract names it (its frontier already projected onto the
/// public document id by the hub).
pub fn rebootstrap_control(control: &semio_framework_os_spr::RebootstrapRequired) -> semio_framework_os_directory::RebootstrapRequired {
    let frontier = &control.baseline_frontier;
    semio_framework_os_directory::RebootstrapRequired {
        scope: semio_framework_os_directory::DocumentScope::new(control.space_id.as_str(), control.document_id.as_str()),
        checkpoint_id: semio_framework_os_directory::ArtifactHash(control.checkpoint_id),
        descriptor_digest_v1: semio_framework_os_directory::ArtifactHash(control.descriptor_hash),
        baseline_frontier: semio_framework_os_directory::ArtifactFrontier { document_id: frontier.document_id.0.clone(), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_hash: semio_framework_os_directory::ArtifactHash(frontier.chain_hash) },
    }
}

/// 👋️ The client-first frame every document socket opens with.
pub fn document_socket_hello(schema: &str, pack_schema_hash: [u8; 32], resume_token: Option<String>, frontier: Option<RuntimeFrontierSummary>) -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: schema.to_string(), pack_schema_hash, resume_token, frontier }
}

/// 🔐️ The one ordered subprotocol offer a document socket dials with — the grant receipt's
/// protocol, then the session capability — which a browser joins into the single
/// `Sec-WebSocket-Protocol` value the hub splits on `", "`.
pub fn document_socket_protocols(receipt_protocol: &str, capability: &str) -> [String; 2] {
    [receipt_protocol.to_string(), capability.to_string()]
}

/// 📦️ One `Commands` frame ready for a socket: its batch id, the local envelopes it relays (the
/// rollback owner) and its encoded bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandsFrame {
    pub batch_id: u64,
    pub local: Vec<MutationEnvelope>,
    pub bytes: Vec<u8>,
}

/// 📏️ The frames one relay sends, every one within the socket's ceiling, plus the envelopes that
/// cannot fit a frame even alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandsFramePlan {
    pub frames: Vec<CommandsFrame>,
    pub oversized: Vec<MutationEnvelope>,
}

/// ✂️ Splits one relay into `Commands` frames no larger than `max_frame_bytes`, halving a batch
/// until it fits and keeping envelope order; batch ids run consecutively from `first_batch_id`. `local`
/// and `wire` are the same envelopes before and after socket stamping.
pub async fn commands_frames_within(max_frame_bytes: usize, first_batch_id: u64, local: Vec<MutationEnvelope>, wire: Vec<MutationEnvelope>) -> CommandsFramePlan {
    let mut plan = CommandsFramePlan::default();
    let mut batch_id = first_batch_id;
    let mut pending = std::collections::VecDeque::from([(local, wire)]);
    while let Some((local, wire)) = pending.pop_front() {
        let bytes = encode_client_frame(&ClientFrame::Commands { batch_id, envelopes: wire.clone() }, Lane::Command).await;
        if bytes.len() <= max_frame_bytes {
            plan.frames.push(CommandsFrame { batch_id, local, bytes });
            batch_id = batch_id.wrapping_add(1);
        } else if local.len() > 1 {
            let middle = local.len() / 2;
            let (local_head, local_tail) = local.split_at(middle);
            let (wire_head, wire_tail) = wire.split_at(middle);
            pending.push_front((local_tail.to_vec(), wire_tail.to_vec()));
            pending.push_front((local_head.to_vec(), wire_head.to_vec()));
        } else {
            plan.oversized.extend(local);
        }
    }
    plan
}

/// 🎲️ One document actor's hybrid-logical-clock replica seed: platform entropy, so two replicas
/// of one user (two tabs, two processes) never tie on `(seed, counter)` — H4's per-replica law.
pub fn replica_hlc_seed() -> Result<u64, semio_framework_os_identity::EntropyError> {
    semio_framework_os_identity::entropy_u64()
}
//#endregion 🔖️DocumentSocketConnect

//#region 🔖️DocumentLinkShortage
/// 🔌️ The capped doubling reconnect and the longest shortage one hub document's link rides out
/// (`🧬️schema/document-link-shortage/🔣️.json` `$defs/Policy`). The bound is twice the backoff cap, so at
/// least two capped reconnect attempts fall inside it, and no retry is ever scheduled past it: the last
/// attempt of every shortage runs AT the bound. An attempt that never answers ends the link at the
/// ceiling, the bound plus one capped backoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentLinkShortagePolicy {
    pub reconnect_min_ms: u64,
    pub reconnect_max_ms: u64,
    pub shortage_bound_ms: u64,
}

impl DocumentLinkShortagePolicy {
    /// 🔁️ When the next attempt of a shortage that began at `since_ms` is due: one backoff after `now_ms`, never past the bound.
    pub fn retry_at(&self, since_ms: u64, now_ms: u64, backoff_ms: u64) -> u64 {
        now_ms.saturating_add(backoff_ms).min(since_ms.saturating_add(self.shortage_bound_ms))
    }

    /// ⏳️ The instant a shortage that began at `since_ms` ends although no attempt answered: the bound plus one capped backoff.
    pub fn ceiling_at(&self, since_ms: u64) -> u64 {
        since_ms.saturating_add(self.shortage_bound_ms).saturating_add(self.reconnect_max_ms)
    }
}

/// 🔌️ The policy every shell drives (`🧫️fixtures/document-link-shortage-v1` `policy`, the React twin
/// `DOCUMENT_LINK_SHORTAGE_POLICY` in `🏪️store/🟦️.ts`).
pub const DOCUMENT_LINK_SHORTAGE_POLICY: DocumentLinkShortagePolicy = DocumentLinkShortagePolicy {
    reconnect_min_ms: semio_framework_os_directory_client::HUB_RECONNECT_MIN_MS,
    reconnect_max_ms: semio_framework_os_directory_client::HUB_RECONNECT_MAX_MS,
    shortage_bound_ms: 2 * semio_framework_os_directory_client::HUB_RECONNECT_MAX_MS,
};

/// 🚫️ The HTTP statuses of a document admission that mean the hub withdrew access (`accessRefusedStatuses`):
/// the link is revoked, never retried.
pub const DOCUMENT_LINK_ACCESS_REFUSED_STATUSES: [u16; 4] = [401, 403, 404, 410];

/// 🚫️ Whether a document admission failed because the hub withdrew access rather than because the link is short.
pub fn document_admission_refuses_access(error: &semio_framework_os_directory_client::DirectoryClientError) -> bool {
    match error {
        semio_framework_os_directory_client::DirectoryClientError::Unauthorized => true,
        semio_framework_os_directory_client::DirectoryClientError::Http { status, .. } => DOCUMENT_LINK_ACCESS_REFUSED_STATUSES.contains(status),
        _ => false,
    }
}

/// 📣️ The message a document actor emits once its link turns terminal: the status code is the fault code a
/// shell localizes ([`DocumentLinkStatus::text`]), the message the English line for logs.
pub fn document_link_terminal_message(document_id: &str, status: DocumentLinkStatus) -> MutationMessage {
    MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new(status.code()), message: status.text(false).unwrap_or_default().to_string(), target: vec![document_id.to_string()], op_index: None }
}

/// 🔌️ A hub document's link, the ONE state machine the native actor, the browser actor and the
/// React worker drive (ticket 26/09/23 audit P2-2): while `Unlinked` the document stays mounted, local
/// edits keep applying and queue, and the link retries at `retry_at_ms`; a shortage that outlasts the
/// policy bound becomes `Expired` (AGENTS.md: short shortages, never long offline periods) and a hub that
/// withdraws access makes it `Revoked`. Both terminal states admit no local edit and never relink — the
/// human reopens. Schema: `🧬️schema/document-link-shortage/🔣️.json`; law: `🧫️fixtures/document-link-shortage-v1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLink {
    Linked,
    Unlinked { since_ms: u64, backoff_ms: u64, retry_at_ms: u64 },
    Expired { since_ms: u64, at_ms: u64 },
    Revoked { at_ms: u64 },
}

/// 🔌️ What happened to a link: an attempt failed or the socket was lost, the link is live again, time
/// passed, or the hub refused access (401/403/404/410).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLinkEvent {
    Failed { now_ms: u64 },
    Restored { now_ms: u64 },
    Tick { now_ms: u64 },
    Refused { now_ms: u64 },
}

/// 🚦️ The status a shell shows for a link (`$defs/Status`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLinkStatus {
    Linked,
    Reconnecting,
    LinkExpired,
    AccessRevoked,
}

impl DocumentLinkStatus {
    /// 🏷️ The schema's status code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Reconnecting => "reconnecting",
            Self::LinkExpired => "link-expired",
            Self::AccessRevoked => "access-revoked",
        }
    }

    /// 🌐️ The localized line a shell shows for a shortage status (the fixture's `texts`); `None` while linked.
    pub fn text(self, is_de: bool) -> Option<&'static str> {
        match (self, is_de) {
            (Self::Linked, _) => None,
            (Self::Reconnecting, false) => Some("Connection lost. Your edits are kept and sent when the hub is reachable again."),
            (Self::Reconnecting, true) => Some("Verbindung unterbrochen. Ihre Änderungen bleiben erhalten und werden gesendet, sobald der Hub wieder erreichbar ist."),
            (Self::LinkExpired, false) => Some("The connection was lost for too long. Reconnect to keep editing this document."),
            (Self::LinkExpired, true) => Some("Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten."),
            (Self::AccessRevoked, false) => Some("Your access to this document was removed."),
            (Self::AccessRevoked, true) => Some("Ihr Zugriff auf dieses Dokument wurde entfernt."),
        }
    }
}

impl DocumentLink {
    /// 🌱️ A link opened at `now_ms`: not yet live, first attempt due at once, the shortage bound running.
    pub fn opened(now_ms: u64) -> Self {
        Self::Unlinked { since_ms: now_ms, backoff_ms: 0, retry_at_ms: now_ms }
    }

    /// ⚙️ The one transition function (`$defs/Event` → `$defs/State`).
    #[must_use]
    pub fn apply(self, policy: &DocumentLinkShortagePolicy, event: DocumentLinkEvent) -> Self {
        match (self, event) {
            (Self::Expired { .. } | Self::Revoked { .. }, _) => self,
            (_, DocumentLinkEvent::Refused { now_ms }) => Self::Revoked { at_ms: now_ms },
            (_, DocumentLinkEvent::Restored { .. }) => Self::Linked,
            (Self::Linked, DocumentLinkEvent::Failed { now_ms }) => Self::Unlinked { since_ms: now_ms, backoff_ms: policy.reconnect_min_ms, retry_at_ms: policy.retry_at(now_ms, now_ms, policy.reconnect_min_ms) },
            (Self::Unlinked { since_ms, backoff_ms, .. }, DocumentLinkEvent::Failed { now_ms }) => {
                if now_ms.saturating_sub(since_ms) >= policy.shortage_bound_ms {
                    return Self::Expired { since_ms, at_ms: now_ms };
                }
                let backoff_ms = backoff_ms.saturating_mul(2).clamp(policy.reconnect_min_ms, policy.reconnect_max_ms);
                Self::Unlinked { since_ms, backoff_ms, retry_at_ms: policy.retry_at(since_ms, now_ms, backoff_ms) }
            }
            (Self::Unlinked { since_ms, .. }, DocumentLinkEvent::Tick { now_ms }) if now_ms >= policy.ceiling_at(since_ms) => Self::Expired { since_ms, at_ms: now_ms },
            (_, DocumentLinkEvent::Tick { .. }) => self,
        }
    }

    /// 🚦️ The status a shell shows.
    pub fn status(self) -> DocumentLinkStatus {
        match self {
            Self::Linked => DocumentLinkStatus::Linked,
            Self::Unlinked { .. } => DocumentLinkStatus::Reconnecting,
            Self::Expired { .. } => DocumentLinkStatus::LinkExpired,
            Self::Revoked { .. } => DocumentLinkStatus::AccessRevoked,
        }
    }

    /// ✍️ Whether a local edit applies (and queues while unlinked); a terminal link admits none.
    pub fn admits_local_edits(self) -> bool {
        matches!(self, Self::Linked | Self::Unlinked { .. })
    }

    /// ⏳️ The latest instant an unlinked link can live: its ceiling. A failed attempt at the bound ends it earlier.
    pub fn expires_at_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match self {
            Self::Unlinked { since_ms, .. } => Some(policy.ceiling_at(since_ms)),
            _ => None,
        }
    }

    /// 🔁️ When the next reconnect attempt is due; `None` while linked or terminal.
    pub fn retry_at_ms(self) -> Option<u64> {
        match self {
            Self::Unlinked { retry_at_ms, .. } => Some(retry_at_ms),
            _ => None,
        }
    }

    /// 🔁️ Whether a reconnect attempt may start now.
    pub fn retry_due(self, now_ms: u64) -> bool {
        self.retry_at_ms().is_some_and(|at| at <= now_ms)
    }

    /// ⏰️ The earliest instant this link needs a turn of its own: its retry or its expiry.
    pub fn next_deadline_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match (self.retry_at_ms(), self.expires_at_ms(policy)) {
            (Some(retry), Some(expiry)) => Some(retry.min(expiry)),
            (retry, expiry) => retry.or(expiry),
        }
    }
}
//#endregion 🔖️DocumentLinkShortage

//#region 🔁️DocumentEchoSuppression
/// 🔁️ The envelopes of one server `Commands` frame a replica applies, in order: echo suppression is by operation identity,
/// never by frame origin. An envelope is applied unless its id is one this replica authored or already applied, and every admitted id
/// is recorded in `applied`. A frame is never discarded whole — the hub's hello catch-up tail carries anyone's edits, the joiner's own
/// earlier device included (ticket 26/09/23 session 12, run s12i: late joiners saw no history). Schema
/// `🧬️schema/document-echo-suppression`; law `🧫️fixtures/document-echo-suppression-v1`; TS twin `admitRemoteEnvelopes`.
pub fn admit_remote_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: impl IntoIterator<Item = MutationEnvelope>) -> Vec<MutationEnvelope> {
    envelopes.into_iter().filter(|envelope| applied.insert(envelope.mutation_id.0.clone())).collect()
}

/// 🔁️ Records the operations this replica authored, so their echo is never applied a second time.
pub fn note_authored_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: &[MutationEnvelope]) {
    applied.extend(envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()));
}
/// ✅️ Settles this replica's own operations the hub's log already holds: a `Commands` frame (a catch-up tail or a relay)
/// that carries an operation still in `outbox` or a `pending` batch proves the hub committed it although its `Ack` was lost with the
/// socket, so it leaves both (a batch left empty leaves too) and is never resent — a resend is re-stamped and the hub refuses it as a
/// replayed operation (ticket 26/09/23 session 13, run s13b). Answers the settled envelopes, whose backbone retention the caller
/// releases.
pub fn settle_committed_envelopes(outbox: &mut Vec<MutationEnvelope>, pending: &mut std::collections::HashMap<u64, Vec<MutationEnvelope>>, committed: &[MutationEnvelope]) -> Vec<MutationEnvelope> {
    let committed: std::collections::HashSet<&str> = committed.iter().map(|envelope| envelope.mutation_id.0.as_str()).collect();
    let mut settled = Vec::new();
    let mut keep = |envelope: &MutationEnvelope| !committed.contains(envelope.mutation_id.0.as_str()) || {
        settled.push(envelope.clone());
        false
    };
    outbox.retain(&mut keep);
    pending.retain(|_, batch| {
        batch.retain(&mut keep);
        !batch.is_empty()
    });
    settled
}
//#endregion 🔁️DocumentEchoSuppression

//#region 🔖️DocumentSocketDoor
/// 📬️ One observation of a browser document socket's receive side.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentSocketPoll {
    Frame(Vec<u8>),
    Pending,
    Closed(Option<u16>),
    Lost(u32),
}

/// 🔌️ One page-owned document socket as the browser actor drives it: never blocking, polled on
/// the actor's own cadence, and honest about loss — a dropped frame is [`DocumentSocketPoll::Lost`],
/// which the actor answers with a reconnect and a hub catch-up rather than a silent gap.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait DocumentSocket {
    fn is_open(&self) -> bool;
    /// 📏️ The largest frame this socket carries; the actor splits a `Commands` batch to fit it.
    fn max_frame_bytes(&self) -> usize;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String>;
    fn poll(&mut self) -> DocumentSocketPoll;
    fn close(&mut self);
}

/// ☎️ Opens document sockets for the browser actor. The host injects it (the wgpu renderer's
/// duplex socket door), so the kernel names no page API and every browser socket has one owner.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait DocumentSocketDialer {
    fn dial(&self, url: &str, protocols: &[String]) -> Result<Box<dyn DocumentSocket>, String>;
}
//#endregion 🔖️DocumentSocketDoor

//#region 🔖️WireBridge
// 🎯️ W6 kernel unification: `to_wire_envelope`/`from_wire_envelope` are DELETED — this actor's
// local envelope shape (`LocalMutations`/`RemoteMutations`/`ChannelBackbone`) and the wire shape
// (`crate::os_spr::wire::ClientFrame::Commands`/`ServerFrame::Commands`) are now the SAME type,
// `crate::os_spr::MutationEnvelope`, throughout: `crate::os_store::backbone::BackboneMessage::Mutations` already carries
// it natively (W6's `store` repoint), so there is no local/wire boundary left to bridge across.
// The old local-only fields (`payload_hash`, `schema_version` separate from `diff.schema`) are
// dropped, not replaced — a repo-wide grep confirmed neither was ever read outside this
// now-deleted bridge (`payload_hash` was write-only; `schema_version`/`deps` were only read back
// by these same two functions and two test assertions, both updated alongside this change).

/// ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed
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
pub async fn rollback_envelope(envelope: &MutationEnvelope) -> Option<MutationEnvelope> {
    if semio_framework_os_spr::is_history_transition(envelope) {
        return None;
    }
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    Some(MutationEnvelope {
        mutation_id: undo_id,
        document_id: envelope.document_id.clone(),
        actor: envelope.actor.clone(),
        dependencies: vec![envelope.mutation_id.clone()],
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: envelope.inverse.schema.clone(), payload: envelope.inverse.payload.clone() },
        inverse: semio_framework_os_spr::InverseMutation { schema: envelope.diff.schema.clone(), payload: envelope.diff.payload.clone() },
        timestamp: envelope.timestamp,
        transaction: None, verb: None, line: None,
    })
}

/// 🛟️ The code of the typed refusal an actor raises when the hub refused a batch holding history transitions.
pub const HISTORY_TRANSITION_REFUSED_CODE: &str = "history.transition-refused";

/// 🪧️ The typed refusal of `refused`, a batch the hub did not take as sent (`reason`), when it holds history transitions
/// (a refused `Supersede`, a foreign undo): no inverse rolls a transition back ([`rollback_envelope`]), so the actor
/// retracts them from its store (`BackboneMessage::Retract`, which drops every local transition depending on them too)
/// and from its persisted log, and the author converges with the hub, which never took them. The message targets every
/// refused transition, so the runtime can say which history step the hub did not take (ticket 26/09/30
/// NON-DESTRUCTIVE-HISTORY-EDITING).
pub fn transition_refusal(refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
    let transitions: Vec<String> = refused.iter().filter(|envelope| semio_framework_os_spr::is_history_transition(envelope)).map(|envelope| envelope.mutation_id.0.clone()).collect();
    (!transitions.is_empty()).then(|| MutationMessage {
        level: semio_framework_diagnostic::Severity::Error,
        code: semio_framework_diagnostic::FaultCode::new(HISTORY_TRANSITION_REFUSED_CODE),
        message: format!("the hub refused {} history step(s) ({reason}); they are withdrawn", transitions.len()),
        target: transitions,
        op_index: None,
    })
}

/// 📡️ `PresencePeer` -> the binary blob `crate::os_spr::wire::ClientFrame::Presence` carries
/// opaquely (`crate::os_spr::encode_presence_peer` — `protocol_wire` has no dependency on
/// this crate's `PresencePeer` type, so the frame only ever moves the pre-encoded bytes).
pub async fn presence_to_bytes(peer: &PresencePeer) -> Vec<u8> {
    semio_framework_os_spr::encode_presence_peer(peer).await
}

/// 📡️ The inverse of {@link presence_to_bytes}, for `ServerFrame::Presence`'s peer roster.
pub async fn presence_from_bytes(bytes: &[u8]) -> Option<PresencePeer> {
    semio_framework_os_spr::decode_presence_peer(bytes).await.ok()
}

// 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
// `assemble_presence_interaction` MOVED to `crate::os_spr::wire`'s `🔖️PresenceInteraction` region —
// guests never enable this crate's `sync` feature, and `VcsArtifactApp` must be able to call it
// without pulling in the whole actor layer. Re-imported at the top of this module (`use
// crate::os_spr::assemble_presence_interaction`-equivalent via the `crate::os_spr::{...}` import
// list) rather than re-exported here — every call site below already goes through
// `crate::os_spr::assemble_presence_interaction` directly.

/// 🎨️ Stamps a peer's hub-assigned session color and canonical surface onto an outbound
/// `PresencePeer` right before {@link presence_to_bytes} — the ONE place either field is ever
/// filled; shells never set `peer.color`/`peer.surface` themselves (contract-freeze §C7.4). Pure so
/// both the native and wasm actors (and their tests) can share it.
pub async fn stamp_session(peer: &mut PresencePeer, session_color: Option<u8>, surface: Option<&str>) {
    peer.color = session_color;
    peer.surface = surface.map(str::to_string);
}

/// ⏰️ Millisecond wall-clock reads for {@link next_timestamp} and the native folder endpoint — the only
/// readers, both native: the browser actor never stamps a clock, it carries the author's.
#[cfg(not(target_arch = "wasm32"))]
pub async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

/// 🧮️ A stable, deterministic `u64` seed for an actor id string, for
/// `crate::os_spr::HybridLogicalTimestamp::actor` (which is `u64`-shaped; this actor's own id is a
/// free-form `String`).
pub async fn actor_seed(actor: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.hash(&mut hasher);
    hasher.finish()
}

/// ⏰️ Advances `counter` and stamps a fresh {@link crate::os_spr::HybridLogicalTimestamp} for an
/// external folder envelope that carries no clock. An authored envelope's `(hlc, id)` is canonical: it
/// crosses the wire exactly as the author folded it, so every replica orders it — and resolves the last
/// supersession of an operation — by the same key (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING).
#[cfg(not(target_arch = "wasm32"))]
pub async fn next_timestamp(seed: u64, counter: &mut u64) -> semio_framework_os_spr::HybridLogicalTimestamp {
    *counter = counter.wrapping_add(1);
    semio_framework_os_spr::HybridLogicalTimestamp { actor: seed, physical_ms: now_ms().await, logical: *counter }
}
//#endregion 🔖️WireBridge

//#region 🔖️SyncSession
/// 🔁️ Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_os_spr::Mutation<P> + OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub store: ArtifactStore<P, Mutation>,
    pub cmd_tx: Option<ArtifactMailboxSender>,
    pub events: Option<broadcast::Receiver<ArtifactEvent>>,
    pub status: ArtifactSyncStatus,
}

impl<P, Mutation> SyncSession<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_os_spr::Mutation<P> + OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub async fn new(store: ArtifactStore<P, Mutation>) -> Self {
        Self { store, cmd_tx: None, events: None, status: ArtifactSyncStatus::default() }
    }

    /// 🔌️ Attaches this session's store to a document actor: the actor's `ChannelBackbone` end
    /// is wired into the store, and the command/event channels are retained for wake + status.
    pub async fn attach<Runner>(&mut self, channels: ArtifactChannelsOf<Runner>, events: broadcast::Receiver<ArtifactEvent>) -> Result<(), SyncError> {
        self.store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.map_err(|error| SyncError::Vcs(error.to_string()))?;
        self.cmd_tx = Some(channels.cmd_tx);
        self.events = Some(events);
        Ok(())
    }

    /// ✂️ Detaches from the actor (asking it to flush + stop) and unbinds the store's backbone.
    pub async fn detach(&mut self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::Detach);
        }
        let _ = self.store.detach_backbone();
        self.cmd_tx = None;
        self.events = None;
    }

    /// 🔔️ Nudges the actor to drain the store's outbound queue without waiting for its poll tick.
    pub async fn wake(&self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        }
    }

    /// 👻️ Publishes an ephemeral preview blob on the semio_hub's preview lane. See
    /// {@link ArtifactActorMsg::PublishPreview}.
    pub async fn publish_preview(&self, key: String, seq: u64, payload: Vec<u8>) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::PublishPreview { key, seq, payload });
        }
    }

    /// 📥️ Advances one buffered sync event, then gives the store one inbound pump opportunity.
    pub async fn tick(&mut self) -> Result<bool, SyncError> {
        if let Some(events) = &mut self.events {
            if let Ok(ArtifactEvent::Status(status)) = events.try_recv() {
                self.status = status;
            }
        }
        self.store.tick().await.map_err(|error| SyncError::Vcs(error.to_string()))
    }

    /// 🚦️ The latest sync status seen on the event stream (updated by {@link SyncSession::tick}).
    pub async fn status(&self) -> ArtifactSyncStatus {
        self.status.clone()
    }

    /// 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline. Kept for direct/test injection.
    pub async fn receive(&mut self, envelope: MutationEnvelope, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), SyncError> {
        self.store.dispatch(semio_framework_os_store_space_schema::ArtifactCommand::IngestRemote { envelope }, identity).await.map(|_| ()).map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub async fn reconcile_branch(&mut self, _alternative_name: &str, _message: Option<String>, _authors: Vec<semio_framework_os_vcs::Author>) -> Result<String, SyncError> {
        Err(SyncError::Vcs("branch reconciliation requires the retained envelope candidate job; synchronous whole-envelope duplication is forbidden".into()))
    }
}
//#endregion 🔖️SyncSession

//#region 🔖️Host
/// 💓️ Maximum generic host heartbeat frequency. Presence is lossy, last-writer-wins state:
/// callers may offer cursor/viewport/app-presence updates as often as input arrives, while the host
/// publishes only the newest complete peer snapshot at ten hertz.
pub const PRESENCE_HEARTBEAT_INTERVAL_MS: u64 = 100;

/// 🧮️ The summary a peer may publish: `completed` never above `total`, so the wire decoder never refuses it.
pub fn presence_tool_run_clamped(tool_run: PresenceToolRun) -> PresenceToolRun {
    let completed = tool_run.total.map_or(tool_run.completed, |total| tool_run.completed.min(total));
    PresenceToolRun { completed, ..tool_run }
}

/// 💓️ Per-document last-writer-wins presence producer. The producer owns cadence rather than
/// every renderer/app inventing a timer: offers inside the minimum interval replace `pending`, the
/// first offer publishes immediately, and a later offer publishes the newest complete snapshot.
#[derive(Clone, Debug)]
pub struct PresenceHeartbeatProducer {
    interval_ms: u64,
    last_sent_at_ms: Option<u64>,
    pub pending: Option<PresencePeer>,
    observed_tool_run: Option<Option<PresenceToolRun>>,
}

impl Default for PresenceHeartbeatProducer {
    fn default() -> Self {
        Self::new(PRESENCE_HEARTBEAT_INTERVAL_MS)
    }
}

impl PresenceHeartbeatProducer {
    /// 🚫️async: E1 pure struct-literal builder consumed by `impl Default` (sync-only external
    /// trait) — see R9. No I/O, no suspension point.
    pub fn new(interval_ms: u64) -> Self {
        Self { interval_ms: interval_ms.max(1), last_sent_at_ms: None, pending: None, observed_tool_run: None }
    }

    /// ⏯️ Records the document instance's latest tool run summary (a guest's `AppFrame::Ephemeral.tool_run`,
    /// `📋️tool-run-contract.md` §3.4). Once observed, the guest's summary is authoritative: every later offer
    /// carries it (or its absence) instead of whatever the offering renderer assembled.
    pub fn observe_tool_run(&mut self, tool_run: Option<PresenceToolRun>) {
        let tool_run = tool_run.map(presence_tool_run_clamped);
        if let Some(pending) = self.pending.as_mut() {
            pending.tool_run = tool_run.clone();
        }
        self.observed_tool_run = Some(tool_run);
    }

    /// 📡️ Offers the newest whole peer snapshot and returns it only when this document's
    /// cadence permits a publish. A backward-moving clock conservatively waits for the next interval.
    /// The tool run summary is the observed one when there is one, and `completed ≤ total` either way.
    pub fn offer(&mut self, now_ms: u64, mut peer: PresencePeer) -> Option<PresencePeer> {
        peer.tool_run = match &self.observed_tool_run {
            Some(observed) => observed.clone(),
            None => peer.tool_run.map(presence_tool_run_clamped),
        };
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

/// 🎛️ The channels {@link ArtifactHost::open} hands back to a caller: attach `channel_backbone`
/// to your `ArtifactStore`, and send control messages (or wakes) on `cmd_tx`. `Runner` is the
/// platform's per-document runner ticket (`()` in the browser, which has no runner).
pub struct ArtifactChannelsOf<Runner> {
    pub cmd_tx: ArtifactMailboxSender,
    pub document_key: ArtifactDocumentKey,
    pub runner: Runner,
    /// 🔗️ The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Backbones::Channel(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

pub struct OpenDocument<Runner> {
    pub generation: u64,
    pub cancel: semio_framework_async::CancelToken,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: broadcast::Sender<ArtifactEvent>,
    pub presence: PresenceHeartbeatProducer,
    pub runner: Runner,
}

pub struct ArtifactHostState<Runner> {
    pub documents: std::collections::HashMap<ArtifactDocumentKey, OpenDocument<Runner>>,
    pub document_execution_target_leases: std::collections::HashMap<ArtifactDocumentKey, semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub document_seeds: std::collections::HashMap<ArtifactDocumentKey, semio_framework_os_directory::CanonicalCheckpointPairV1>,
    pub closing: std::collections::HashMap<u64, Runner>,
    pub next_generation: u64,
    pub host_references: usize,
}

impl<Runner> ArtifactHostState<Runner> {
    pub fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
            document_execution_target_leases: std::collections::HashMap::new(),
            document_seeds: std::collections::HashMap::new(),
            closing: std::collections::HashMap::new(),
            next_generation: 1,
            host_references: 1,
        }
    }

    pub fn claim_generation(&mut self) -> u64 {
        let generation = self.next_generation;
        self.next_generation = self.next_generation.checked_add(1).expect("artifact actor generation exhausted");
        generation
    }
}

/// 🏛️ The platform-neutral half of `ArtifactHost`: the registry of open per-document actors, their
/// event fan-out, presence cadence and the late-bound hub binding. Each platform crate owns the
/// `ArtifactHost` that composes this core with the actor it spawns, so this core never names an actor.
pub struct ArtifactHostCore<Runner> {
    pub inner: std::sync::Arc<std::sync::Mutex<ArtifactHostState<Runner>>>,
    pub pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub credential: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
    pub socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
    pub cancel: semio_framework_async::CancelToken,
}

impl<Runner> ArtifactHostCore<Runner> {
    /// 🧩️ Creates a core on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(ArtifactHostState::new())),
            pool,
            credential: std::sync::Arc::new(std::sync::RwLock::new(None)),
            socket_grant_source: std::sync::Arc::new(std::sync::RwLock::new(None)),
            cancel: semio_framework_async::CancelToken::root_now(),
        }
    }

    pub fn state(&self) -> std::sync::MutexGuard<'_, ArtifactHostState<Runner>> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🔗️ Counts one more owner of the shared registry and returns its handle.
    pub fn retain(&self) -> Self {
        let mut state = self.state();
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self { inner: self.inner.clone(), pool: self.pool.clone(), credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), cancel: self.cancel.clone() }
    }

    /// 🔓️ Releases one owner; `true` when it was the last.
    pub fn release(&self) -> bool {
        let mut state = self.state();
        state.host_references = state.host_references.checked_sub(1).expect("artifact host reference underflow");
        state.host_references == 0
    }

    pub fn set_local_hub_credential(&self, credential: std::sync::Arc<semio_framework_os_directory_client::LocalHubCredential>) {
        *self.credential.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: std::sync::Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>) {
        *self.socket_grant_source.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(source);
    }

    /// 🪪️ Binds one complete verified execution-target lease before this document actor is opened.
    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        if !matches!(document_key, ArtifactDocumentKey::Hub { .. }) {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_execution_target_leases.contains_key(document_key) {
            return false;
        }
        state.document_execution_target_leases.insert(document_key.clone(), surface);
        true
    }

    /// 🪢️ Hands the next actor of a hub document the canonical checkpoint pair its shell verified and loaded into the
    /// guest ([`crate::os_directory::DirectoryClient::document_canonical_checkpoint_pair`]): the actor starts at that
    /// pair — its history known, its first `SocketHelloV1` naming the pair's baseline frontier, so the hub tails only
    /// what came after it. Consumed by the next open of the same key.
    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) -> bool {
        let ArtifactDocumentKey::Hub { space_id, document_id } = document_key else {
            return false;
        };
        if pair.scope.space_id != *space_id || pair.scope.document_id != *document_id {
            return false;
        }
        let mut state = self.state();
        if state.documents.contains_key(document_key) || state.document_seeds.contains_key(document_key) {
            return false;
        }
        state.document_seeds.insert(document_key.clone(), pair);
        true
    }

    /// 🟢️ Whether a credential and a grant source are bound; a browser adds its socket dialer on top.
    pub fn hub_binding_present(&self) -> bool {
        self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// 🎫️ Takes the lease and the seed bound for the next open of `document_key`.
    pub fn take_open_inputs(&self, document_key: &ArtifactDocumentKey) -> (Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>, Option<semio_framework_os_directory::CanonicalCheckpointPairV1>) {
        let mut state = self.state();
        (state.document_execution_target_leases.remove(document_key), state.document_seeds.remove(document_key))
    }

    pub fn insert_document(&self, document_key: ArtifactDocumentKey, document: OpenDocument<Runner>) {
        self.state().documents.insert(document_key, document);
    }

    pub fn remove_document(&self, document_key: &ArtifactDocumentKey) -> Option<OpenDocument<Runner>> {
        self.state().documents.remove(document_key)
    }

    /// 📬️ A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.subscribe_key(&ArtifactDocumentKey::local(document_id)).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        let guard = self.state();
        match guard.documents.get(document_key) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// 🔔️ Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.send_key(&ArtifactDocumentKey::local(document_id), message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        if let Some(document) = self.state().documents.get(document_key) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// 💓️ Offers a generic cursor/viewport/app-presence heartbeat for one open document.
    /// Returns `true` only when the host actually queued a publish; faster offers are coalesced onto
    /// the document's producer and cannot flood the preview lane.
    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.presence_heartbeat_key(&ArtifactDocumentKey::local(document_id), now_ms, peer)
    }

    /// ⏯️ Records an open document's latest tool run summary for its presence heartbeats — the host
    /// calls this with each `AppFrame::Ephemeral.tool_run` it decodes. `false` when the document is not open.
    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        document.presence.observe_tool_run(tool_run);
        true
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        let mut state = self.state();
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        let Some(peer) = document.presence.offer(now_ms, peer) else { return false };
        document.cmd_tx.send(ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(peer) }).is_ok()
    }

    /// 🧹️ Ids of every currently-open document.
    /// 🚫️async: E1 pure lock-and-collect consumed by `impl Drop` (sync-only external trait) — see
    /// R9. No I/O, no suspension point.
    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.state().documents.keys().cloned().collect()
    }
}

//#endregion 🔖️Host
//#region 🔖️NativeActor



//#endregion 🔖️NativeActor
//#region 🔖️WasmActor



//#endregion 🔖️WasmActor

#[cfg(any(test, feature = "testing"))]
pub mod fixtures {
//#region 🔖️Fixtures
/// 🎬️ A scripted actor test vector shared by cargo test (here) and vitest (WS-E's TS twin).
/// Each fixture drives inbound events at a document actor and asserts the resulting `ArtifactEvent`
/// sequence and the final persisted envelope edit ids. See `framework/sync/fixtures/README.md`.
#[derive(Clone, Debug)]
pub struct ActorFixture {
    pub name: String,
    pub schema: String,
    pub document_id: String,
    /// 📥️ Inbound stimulus applied to the actor, in order.
    pub inbound: Vec<FixtureInbound>,
    /// 📤️ The `ArtifactEvent` variant tags expected on the subscriber channel, in order.
    pub expected_events: Vec<String>,
    /// 📇️ Edit ids expected in the document's timeline after replay.
    pub expected_edit_ids: Vec<String>,
}

/// 📥️ One scripted inbound stimulus: either a semio_hub server frame or an external folder edit.
/// Document/op CONTENT lives in sibling text files (never JSON) — this is the LOADED shape
/// (content already read off disk); see `RawFixtureInbound` for the on-disk manifest shape that
/// only references filenames.
#[derive(Clone, Debug)]
pub enum FixtureInbound {
    /// 📬️ A raw `crate::os_spr::wire::ServerFrame`'s encoded bytes (`crate::os_spr::encode_server_frame`
    /// output, `lane` byte included), delivered as if received over the semio_hub WebSocket — already
    /// real binary, not document/op content, so it stays inline in the manifest as a JSON number
    /// array. Driven by `🏪️store/👷️worker/🟦️.ts`'s TS fallback vitest harness (which decodes these
    /// bytes with its own binary decoder); the folder-only Rust harness skips these.
    HubFrame { frame_bytes: Vec<u8> },
    /// 📁️ An external folder edit: `.ops`-grammar text (one or more `edit ...` blocks) to
    /// append to the spr log out-of-band.
    ExternalEdits { ops_text: String },
    /// ♻️ An external whole-document rewrite (divergent history): dsl + ops text compiled
    /// via `codec.compile_dsl` and written in place of the stored document.
    ReplaceDocument { dsl_text: String, ops_text: String },
}

/// 📄️ The on-disk manifest shape: `kind`-tagged like `FixtureInbound`, but content-bearing
/// variants reference a sibling filename (relative to the fixture's own directory) instead of
/// carrying the text inline — `load_fixtures` resolves these into real `FixtureInbound`s.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum RawFixtureInbound {
    HubFrame { frame_bytes: Vec<u8> },
    ExternalEdits { ops_file: String },
    ReplaceDocument { dsl_file: String, ops_file: String },
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
struct TestEvidence {
    name: String,
    schema: String,
    document_id: String,
    inbound: Vec<RawFixtureInbound>,
    expected_events: Vec<String>,
    expected_edit_ids: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn parse_fixture_dsl_manifest(text: &str) -> Option<TestEvidence> {
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

    Some(TestEvidence { name: name?, schema: schema?, document_id: document_id?, inbound, expected_events, expected_edit_ids })
}

/// 📂️ Loads every `<name>/🔣️fixture.dsl` manifest directory under `dir`, resolving each
/// content-bearing inbound entry against its sibling text file. A fixture whose manifest or
/// any referenced file is missing/unreadable is skipped (never a partial/silently-wrong fixture).
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_fixtures(dir: &std::path::Path) -> Vec<ActorFixture> {
    let mut fixtures = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return fixtures };
    let mut fixture_dirs: Vec<std::path::PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.is_dir()).collect();
    fixture_dirs.sort();
    for fixture_dir in fixture_dirs {
        let Ok(manifest_text) = std::fs::read_to_string(fixture_dir.join("🔣️fixture.dsl")) else { continue };
        let Some(manifest) = parse_fixture_dsl_manifest(&manifest_text).await else { continue };
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

}
#[cfg(any(test, feature = "testing"))]
use fixtures::*;
//#endregion 🔖️Fixtures
//#region 🔖️FolderStorage

























//#endregion 🔖️FolderStorage
//#region 🔖️BlobStoreImpl

//#endregion 🔖️BlobStoreImpl

#[cfg(test)]
#[path = "🧪️tests/🔬️persistence-data-class/🦀️.rs"]
mod persistence_data_class_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-socket-connect/🦀️.rs"]
mod document_socket_connect_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-link-shortage/🦀️.rs"]
mod document_link_shortage_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-echo-suppression/🦀️.rs"]
mod document_echo_suppression_tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️command-rejection/🦀️.rs"]
mod command_rejection_tests;
