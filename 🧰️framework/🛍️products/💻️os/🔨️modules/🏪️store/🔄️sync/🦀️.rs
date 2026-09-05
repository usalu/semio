//! 🔁️ Local-first sync actor layer: a schema-agnostic per-document backbone actor that runs all IO
//! (persist, semio_hub WebSocket sync, file watching) off the UI thread, plus the causal {@link SyncSession}
//! that feeds remote {@link MutationEnvelope}s into a document's vcs edit timeline.
//!
//! # Threading model
//! - **Native** (wgpu native host, tests): {@link ArtifactHost::open} schedules bounded actor turns
//!   on the injected process `WorkerPool`; WebSocket readiness remains on the ambient Tokio I/O
//!   reactor and all actor deadlines use the pool's `TimerWheel`.
//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on the owned browser-local
//!   executor with a `web_sys::WebSocket` semio_hub transport (no threads, no filesystem). The
//!   production browser shell instead uses a TS twin (`🧵️backbone-worker.ts`, WS-E); this wasm actor
//!   keeps the crate coherent for a future in-wasm host.
//! - **WASI-P2 plugins never link this crate** — inside the sandbox a store attaches vcs's pure
//!   `PortBackbone` (an in-memory queue relayed to the host). This actor is a host-side concern only.

use crate::os_spr::PresencePeer;
use crate::os_spr::{decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes, AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, RuntimeFrontierSummary, ServerFrame};
use crate::os_spr::{ActorId, MutationId};
use crate::os_store::{ArtifactPackFiles, ArtifactStore, ArtifactTextFiles, BackboneMessage, Backbones, ChannelBackbone, ChannelBackboneRemote};
use crate::os_dsl::{DslValue, FromValue as FromValueTrait, ToValue as ToValueTrait, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use tokio::sync::{broadcast, mpsc};

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

//#region 🔖️EnvelopeSerde
/// @emoji 🧵️ JSON worker seam: `MutationEnvelope` vectors as `encode_envelopes` bytes (not struct JSON).
mod envelope_serde {
    use crate::os_spr::{decode_envelopes, encode_envelopes, MutationEnvelope};

    /// @emoji 🧵️ Byte framing for the `ArtifactActorMsg::LocalMutations`/`ArtifactEvent::
    /// RemoteMutations` `#[value(serialize_with = ..., deserialize_with = ...)]` field bridge —
    /// wire shape is a `DslValue::Array` of `DslValue::Number` (one per byte), matching what the
    /// former serde `serialize_seq` path produced.
    pub fn to_value(envelopes: &Vec<MutationEnvelope>) -> super::DslValue {
        let bytes = encode_envelopes(envelopes);
        super::DslValue::Array(bytes.into_iter().map(|byte| super::DslValue::uint(byte as u64)).collect())
    }

    /// @emoji 🧵️ `FromValue` twin of `deserialize` above.
    pub fn from_value(value: super::DslValue) -> Result<Vec<MutationEnvelope>, super::ValueError> {
        let super::DslValue::Array(items) = value else {
            return Err(super::ValueError::new("expected array of bytes"));
        };
        let mut bytes = Vec::with_capacity(items.len());
        for item in items {
            let super::DslValue::Number(n) = item else {
                return Err(super::ValueError::new("expected byte number"));
            };
            let Some(n) = n.as_u64() else {
                return Err(super::ValueError::new("expected byte number"));
            };
            bytes.push(n as u8);
        }
        decode_envelopes(&bytes).map_err(|error| super::ValueError::new(error.to_string()))
    }
}
//#endregion 🔖️EnvelopeSerde

//#region 🔖️Protocol
/// @emoji 🗃️ A durable place a document synchronizes with. A document may bind to several at once
/// (folder-only, semio_hub-only, or both); the actor treats each as an independent peer.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PersistenceBinding {
    /// @emoji 📁️ Local canonical store. A directory uses the multi-document `folder://` event log;
    /// a `*.json` path uses the single-blob `file://` export format.
    Folder { path: std::path::PathBuf },
    /// @emoji ☁️ A semio_hub node reachable over WebSocket
    /// (`remote://host:port` → `ws://host:port/spaces/{space_id}/documents/{id}/socket/v1`).
    Hub {
        base_url: String,
        space_id: String,
        /// @emoji 🎭️ Out-of-band presence scope (ticket 26/08/16/HUB-SPACES-…, contract §C0): rides
        /// as `?surface=` on the WS URL rather than a wire field — `PresencePeer`'s flag byte is
        /// already full. `<kind>@<standard>/<subset>#<role>`, e.g. `s.space.home@1/*#editor`.
        #[value(default, skip_serializing_if = "Option::is_none")]
        surface: Option<String>,
    },
}

/// @emoji 🧾️ Everything {@link ArtifactHost::open} needs to spawn one document's actor.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactActorConfig {
    pub document_id: String,
    pub schema: String,
    pub bindings: Vec<PersistenceBinding>,
    /// @emoji 👁️ Watch the folder binding for external edits (other processes writing the file).
    #[value(default)]
    pub watch_external: bool,
    /// @emoji 🖋️ The authoring actor id used for semio_hub `Hello`/presence and operation origin filtering.
    pub actor: String,
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

/// @emoji 📨️ Caller → actor control messages, sent on the {@link ArtifactChannels} command channel.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactActorMsg {
    /// @emoji ⬆️ Wakes the actor to drain the store's outbound operations promptly. `envelopes` is a
    /// direct-injection fallback used only when no store is attached to the channel (empty = pure wake).
    LocalMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// @emoji 📡️ Broadcasts this peer's presence/selection to the semio_hub.
    PresenceHeartbeat { peer: Box<PresencePeer> },
    /// @emoji 👻️ Publishes an ephemeral, best-effort UI-state blob on the semio_hub's uncredited preview
    /// lane (`crate::os_spr::wire::ClientFrame::PreviewPublish`) — e.g. a drag ghost or live cursor;
    /// `seq` is a per-`key` monotone counter so a receiver can drop stale-arriving previews.
    PublishPreview { key: String, seq: u64, payload: Vec<u8> },
    /// @emoji 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,
    /// @emoji ✂️ Flushes any pending outbound operations, then stops the actor.
    Detach,
}

pub const ARTIFACT_MAILBOX_ITEMS: usize = 64;
pub const ARTIFACT_MAILBOX_BYTES: usize = 1_048_576;

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
    generation: u64,
    bytes: usize,
    message: ArtifactActorMsg,
}

struct ArtifactMailboxState {
    slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS],
    head: usize,
    len: usize,
    bytes: usize,
    generation: u64,
    closed: bool,
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
    state: std::sync::Mutex<ArtifactMailboxState>,
}

#[derive(Clone)]
pub struct ArtifactMailboxSender {
    authority: std::sync::Arc<ArtifactMailboxAuthority>,
    generation: u64,
}

struct ArtifactMailboxReceiver {
    authority: std::sync::Arc<ArtifactMailboxAuthority>,
    generation: u64,
}

#[derive(Clone)]
struct ArtifactMailboxClose {
    authority: std::sync::Arc<ArtifactMailboxAuthority>,
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
    fn try_recv(&self) -> Option<ArtifactActorMsg> {
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

    async fn recv(&self) -> Option<ArtifactActorMsg> {
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
    fn set_wake(&self, wake: std::sync::Arc<dyn Fn() + Send + Sync>) {
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

    fn close(&self) {
        self.close_handle().close();
    }

    fn close_one(&self) -> bool {
        self.close_handle().close_one()
    }

    fn close_handle(&self) -> ArtifactMailboxClose {
        ArtifactMailboxClose { authority: self.authority.clone(), generation: self.generation }
    }
}

impl ArtifactMailboxClose {
    fn close(&self) {
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

    fn close_one(&self) -> bool {
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

    fn has_pending(&self) -> bool {
        self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len != 0
    }
}

fn artifact_mailbox_pair() -> (ArtifactMailboxSender, ArtifactMailboxReceiver) {
    let authority = std::sync::Arc::new(ArtifactMailboxAuthority { state: std::sync::Mutex::new(ArtifactMailboxState::new()) });
    (ArtifactMailboxSender { authority: authority.clone(), generation: 1 }, ArtifactMailboxReceiver { authority, generation: 1 })
}

fn artifact_actor_message_bytes(message: &ArtifactActorMsg) -> Option<usize> {
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
                text(&mut bytes, &envelope.diff.schema.0)?;
                add(&mut bytes, field(envelope.diff.payload.len())?)?;
                text(&mut bytes, &envelope.inverse.schema.0)?;
                add(&mut bytes, field(envelope.inverse.payload.len())?)?;
                add(&mut bytes, 24)?;
            }
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
                        crate::os_spr::PresenceViewKind::Canvas { .. } => 24,
                        crate::os_spr::PresenceViewKind::Orbit { .. } => 80,
                        crate::os_spr::PresenceViewKind::Geo { .. } => 40,
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
        }
        ArtifactActorMsg::PublishPreview { key, seq: _, payload } => {
            text(&mut bytes, key)?;
            add(&mut bytes, 8)?;
            add(&mut bytes, field(payload.len())?)?;
        }
        ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
    }
    (bytes <= ARTIFACT_MAILBOX_BYTES).then_some(bytes)
}

/// @emoji 📶️ Connection state of a document's remote (semio_hub) transport.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RemoteState {
    Detached,
    Connecting,
    Live { peer_count: usize },
    Backoff { retry_in_ms: u64 },
}

/// @emoji 🚦️ Snapshot of a document's sync health for status badges.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactEvent {
    /// @emoji 🕸️ Remote operations (semio_hub fan-out or appended external edits) — also pushed into the store's
    /// inbound queue so `store.tick()` materializes them.
    RemoteMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// @emoji 📸️ The whole document was replaced (divergent external history / semio_hub snapshot swap),
    /// as real pack+spr bytes — no JSON envelope anywhere in this actor's own path.
    SnapshotReplaced { pack: Vec<u8>, spr: Vec<u8> },
    /// @emoji 📈️ Monotonic, bounded progress for one descriptor-bound artifact bootstrap.
    /// A new transfer starts at zero after reconnect; no progress event implies a committed frontier.
    BootstrapProgress { received_bytes: u64, total_bytes: u64, received_chunks: u32, total_chunks: u32 },
    /// @emoji 🚦️ Sync status changed.
    Status(ArtifactSyncStatus),
    /// @emoji 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// @emoji 🎨️ The hub assigned (or re-confirmed, on reconnect) this connection's session color —
    /// `crate::os_spr::wire::ServerFrame::Session`, sent once per connection after `Welcome`. The
    /// actor stores it and stamps it onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
    Session { actor: String, color: u8 },
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
    /// @emoji ⚠️ A structural conflict (external divergence with local pending operations / semio_hub
    /// protocol-level reject), on the frozen diagnostic-bag vocabulary (contract freeze `26/08/16/
    /// MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10) rather than the deleted
    /// per-domain conflict-bag type this used to wrap: `level` is `Error` (this actor has no finer
    /// severity signal from either source), `target` carries the diverged folder uri / hub base url.
    /// This is a transport-level diagnostic, never a first-class `crate::os_spr::Conflict` (no
    /// `id`/`status`/`actors`/`timestamp` exists for either source event).
    Conflict(MutationMessage),
}

const ARTIFACT_BOOTSTRAP_DEADLINE_MS: u64 = 15_000;

/// @emoji 🧬️ Binds a public artifact bootstrap to the already-selected document codec before
/// allocating or accepting any payload bytes.
fn validate_artifact_bootstrap_identity(bootstrap: &ArtifactBootstrap, document_id: &str, schema: &str, pack_schema_hash: [u8; 32], server_frontier: &RuntimeFrontierSummary) -> Result<(), String> {
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

fn frontier_reaches(actual: &RuntimeFrontierSummary, required: &RuntimeFrontierSummary) -> bool {
    actual == required
}

/// @emoji ⚖️ The client-side twin of `crate::os_spr::wire::ApplyOutcome`, minus the `Transformed`
/// envelope payload (already delivered separately as {@link ArtifactEvent::RemoteMutations} by
/// the time this fires — see {@link ArtifactEvent::CommandOutcome}).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CommandAckOutcome {
    Accepted,
    Transformed,
    Rejected { reason: String, messages: Vec<u8> },
}
//#endregion 🔖️Protocol

//#region 🔖️BackboneWorkerWire
/// @emoji 🧵️ Binary worker seam: `MAGIC` + `crate::os_store::pack_rt::encode_wire_value` over a `DslValue`
/// tree (serde-shaped), shared by the wasm `store_worker` and `🧵️backbone-worker.ts`.
pub mod backbone_worker_wire {
    use super::{ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, PersistenceBinding};
    use crate::os_dsl::{from_dsl_value, to_dsl_value};
    use semio_framework_value_derive::{FromValue, ToValue};

    pub const MAGIC: u8 = 0x01;

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerRequest {
        Open {
            document_id: String,
            schema: String,
            bindings: Vec<PersistenceBinding>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            watch_external: Option<bool>,
            actor: String,
        },
        Close {
            document_id: String,
        },
        Send {
            document_id: String,
            message: Box<ArtifactActorMsg>,
        },
    }

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerResponse {
        Event { document_id: String, event: ArtifactEvent },
        Ready,
    }

    impl BackboneWorkerRequest {
        pub async fn actor_config(&self) -> Option<ArtifactActorConfig> {
            let Self::Open { document_id, schema, bindings, watch_external, actor } = self else {
                return None;
            };
            Some(ArtifactActorConfig { document_id: document_id.clone(), schema: schema.clone(), bindings: bindings.clone(), watch_external: watch_external.unwrap_or(true), actor: actor.clone() })
        }
    }

    pub async fn encode_request(request: &BackboneWorkerRequest) -> Result<Vec<u8>, String> {
        let dsl = to_dsl_value(request)?;
        let mut bytes = vec![MAGIC];
        bytes.extend(crate::os_store::pack_rt::encode_wire_value(&dsl));
        Ok(bytes)
    }

    pub async fn decode_request(bytes: &[u8]) -> Result<BackboneWorkerRequest, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = crate::os_store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        from_dsl_value(dsl)
    }

    pub async fn encode_response(response: &BackboneWorkerResponse) -> Result<Vec<u8>, String> {
        let dsl = to_dsl_value(response)?;
        let mut bytes = vec![MAGIC];
        bytes.extend(crate::os_store::pack_rt::encode_wire_value(&dsl));
        Ok(bytes)
    }

    pub async fn decode_response(bytes: &[u8]) -> Result<BackboneWorkerResponse, String> {
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
async fn op_ids_of(edit: &crate::os_spr::HistoryEdit) -> Vec<String> {
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
async fn spr_op_ids(spr: &[u8]) -> Result<std::collections::HashSet<String>, String> {
    let reader = crate::os_spr::HistoryReader::open(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| error.to_string())?;
    let mut ids = std::collections::HashSet::new();
    for edit in reader.edits().await {
        let edit = edit.map_err(|error| error.to_string())?;
        ids.extend(op_ids_of(&edit).await);
    }
    Ok(ids)
}

/// @emoji 📦️ Rebuilds real {@link MutationEnvelope}s (one per forward op, genuine `OpBinary`
/// payloads straight from the edit's own binary `OpPayload`s — no codec, no JSON) from one
/// `HistoryEdit` decoded off the spr bytes, so an appended external edit can flow through the
/// store's causal DAG (`ingest_remote` → `edit_from_operation_envelope`). A binary-less op payload
/// is a hard error — `.spr` is binary-only since B1, so every real op has one.
#[cfg(not(target_arch = "wasm32"))]
async fn envelopes_from_history_edit(edit: &crate::os_spr::HistoryEdit, document_id: &str, schema: &str) -> Result<Vec<MutationEnvelope>, String> {
    let op_ids = op_ids_of(edit).await;
    let mut envelopes = Vec::with_capacity(edit.ops.len());
    for (index, op) in edit.ops.iter().enumerate() {
        let payload = op.binary.clone().ok_or_else(|| format!("edit {} op {index} has no binary payload", edit.id))?;
        let meta = edit.meta.as_ref().and_then(|metas| metas.get(index));
        let dependencies = meta.map(|m| m.dependencies.iter().cloned().map(MutationId).collect()).unwrap_or_default();
        let actor = meta.and_then(|m| m.author_id.clone()).or_else(|| edit.actor.clone()).unwrap_or_else(|| "unknown".to_string());
        let timestamp = match meta.and_then(|meta| meta.hlt) {
            Some((actor, physical_ms, logical)) => crate::os_spr::HybridLogicalTimestamp { actor, physical_ms: u64::try_from(physical_ms).map_err(|_| format!("edit {} op {index} has a negative hybrid-clock physical time", edit.id))?, logical },
            None => crate::os_spr::HybridLogicalTimestamp::new(0, 0),
        };
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
async fn history_edit_from_envelope(envelope: &MutationEnvelope) -> crate::os_spr::HistoryEdit {
    crate::os_spr::HistoryEdit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        started_at: now_ms().await.to_string(),
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
            // 🔀️ A remote-ingested envelope carries no provenance of its own: the authoring peer
            // already resolved any contribution or transaction locally and shipped the resulting
            // OWNER ops, so this side records exactly what it receives — an owner edit.
            origin: crate::os_spr::command::MutationOrigin::Owner,
            messages: Vec::new(),
        }]),
    }
}

/// @emoji 🔗️ Derives a semio_hub WebSocket URL: `remote://host:port` (or `http(s)://`, `ws(s)://`) →
/// `ws(s)://host:port/spaces/{space_id}/documents/{document_id}/socket/v1`, with an out-of-band
/// `?surface=` appended when the binding carries one (contract §C0's presence scope, ticket
/// 26/08/16/HUB-SPACES-…: `(space_id, document_id, surface)` — `surface` rides outside the wire
/// protocol rather than widening `PresencePeer`'s already-full flag byte).
async fn hub_ws_url(base_url: &str, space_id: &str, document_id: &str, surface: Option<&str>) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    let space_id = crate::os_directory::client::encode_url_component(space_id);
    let document_id = crate::os_directory::client::encode_url_component(document_id);
    match surface {
        Some(surface) => format!("{scheme}://{authority}/spaces/{space_id}/documents/{document_id}/socket/v1?surface={}", crate::os_directory::client::encode_url_component(surface)),
        None => format!("{scheme}://{authority}/spaces/{space_id}/documents/{document_id}/socket/v1"),
    }
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
async fn rollback_envelope(envelope: &MutationEnvelope) -> MutationEnvelope {
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    MutationEnvelope {
        mutation_id: undo_id,
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
async fn presence_to_bytes(peer: &PresencePeer) -> Vec<u8> {
    crate::os_spr::encode_presence_peer(peer).await
}

/// @emoji 📡️ The inverse of {@link presence_to_bytes}, for `ServerFrame::Presence`'s peer roster.
async fn presence_from_bytes(bytes: &[u8]) -> Option<PresencePeer> {
    crate::os_spr::decode_presence_peer(bytes).await.ok()
}

// 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
// `assemble_presence_interaction` MOVED to `crate::os_spr::wire`'s `🔖️PresenceInteraction` region —
// guests never enable this crate's `sync` feature, and `VcsArtifactApp` must be able to call it
// without pulling in the whole actor layer. Re-imported at the top of this module (`use
// crate::os_spr::assemble_presence_interaction`-equivalent via the `crate::os_spr::{...}` import
// list) rather than re-exported here — every call site below already goes through
// `crate::os_spr::assemble_presence_interaction` directly.

/// @emoji 🎨️ Stamps a peer's hub-assigned session color and canonical surface onto an outbound
/// `PresencePeer` right before {@link presence_to_bytes} — the ONE place either field is ever
/// filled; shells never set `peer.color`/`peer.surface` themselves (contract-freeze §C7.4). Pure so
/// both the native and wasm actors (and their tests) can share it.
async fn stamp_session(peer: &mut PresencePeer, session_color: Option<u8>, surface: Option<&str>) {
    peer.color = session_color;
    peer.surface = surface.map(str::to_string);
}

/// @emoji ⏰️ Millisecond wall-clock reads for {@link next_timestamp}: `SystemTime` on native AND
/// `wasm32-wasip2` (WASI's clock backs it fine), `js_sys::Date` only in the actual browser wasm
/// build (`target_arch = "wasm32"` is TRUE for wasip2 too, so that arm is narrowed to exclude it).
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
async fn now_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// @emoji 🧮️ A stable, deterministic `u64` seed for an actor id string, for
/// `crate::os_spr::HybridLogicalTimestamp::actor` (which is `u64`-shaped; this actor's own id is a
/// free-form `String`).
async fn actor_seed(actor: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.hash(&mut hasher);
    hasher.finish()
}

/// @emoji ⏰️ Advances `counter` and stamps a fresh {@link crate::os_spr::HybridLogicalTimestamp} for an
/// outbound envelope — freshly stamped on every send (this actor never round-trips a locally-
/// authored envelope's own timestamp back in; a remote-delivered envelope's `timestamp` is simply
/// carried through unchanged).
async fn next_timestamp(seed: u64, counter: &mut u64) -> crate::os_spr::HybridLogicalTimestamp {
    *counter = counter.wrapping_add(1);
    crate::os_spr::HybridLogicalTimestamp { actor: seed, physical_ms: now_ms().await, logical: *counter }
}
//#endregion 🔖️WireBridge

//#region 🔖️SyncSession
/// @emoji 🔁️ Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Mutation>
where
    P: Clone + crate::os_dsl::ToValue + crate::os_dsl::FromValue + crate::os_store::ArtifactPack + Send + Sync + 'static,
    Mutation: Clone + crate::os_spr::Mutation<P> + crate::os_spr::OpBinary + crate::os_spr::OpText + Send + 'static,
{
    pub store: ArtifactStore<P, Mutation>,
    cmd_tx: Option<ArtifactMailboxSender>,
    events: Option<broadcast::Receiver<ArtifactEvent>>,
    status: ArtifactSyncStatus,
}

impl<P, Mutation> SyncSession<P, Mutation>
where
    P: Clone + crate::os_dsl::ToValue + crate::os_dsl::FromValue + crate::os_store::ArtifactPack + Send + Sync + 'static,
    Mutation: Clone + crate::os_spr::Mutation<P> + crate::os_spr::OpBinary + crate::os_spr::OpText + Send + 'static,
{
    pub async fn new(store: ArtifactStore<P, Mutation>) -> Self {
        Self { store, cmd_tx: None, events: None, status: ArtifactSyncStatus::default() }
    }

    /// @emoji 🔌️ Attaches this session's store to a document actor: the actor's `ChannelBackbone` end
    /// is wired into the store, and the command/event channels are retained for wake + status.
    pub async fn attach(&mut self, channels: ArtifactChannels, events: broadcast::Receiver<ArtifactEvent>) -> Result<(), SyncError> {
        self.store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.map_err(|error| SyncError::Vcs(error.to_string()))?;
        self.cmd_tx = Some(channels.cmd_tx);
        self.events = Some(events);
        Ok(())
    }

    /// @emoji ✂️ Detaches from the actor (asking it to flush + stop) and unbinds the store's backbone.
    pub async fn detach(&mut self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::Detach);
        }
        let _ = self.store.detach_backbone();
        self.cmd_tx = None;
        self.events = None;
    }

    /// @emoji 🔔️ Nudges the actor to drain the store's outbound queue without waiting for its poll tick.
    pub async fn wake(&self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        }
    }

    /// @emoji 👻️ Publishes an ephemeral preview blob on the semio_hub's preview lane. See
    /// {@link ArtifactActorMsg::PublishPreview}.
    pub async fn publish_preview(&self, key: String, seq: u64, payload: Vec<u8>) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::PublishPreview { key, seq, payload });
        }
    }

    /// @emoji 📥️ Advances one buffered sync event, then gives the store one inbound pump opportunity.
    pub async fn tick(&mut self) -> Result<bool, SyncError> {
        if let Some(events) = &mut self.events {
            if let Ok(event) = events.try_recv() {
                if let ArtifactEvent::Status(status) = &event {
                    self.status = status.clone();
                }
            }
        }
        self.store.tick().await.map_err(|error| SyncError::Vcs(error.to_string()))
    }

    /// @emoji 🚦️ The latest sync status seen on the event stream (updated by {@link SyncSession::tick}).
    pub async fn status(&self) -> ArtifactSyncStatus {
        self.status.clone()
    }

    /// @emoji 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline. Kept for direct/test injection.
    pub async fn receive(&mut self, envelope: MutationEnvelope) -> Result<(), SyncError> {
        self.store.dispatch(crate::os_store::ArtifactCommand::IngestRemote { envelope }).await.map(|_| ()).map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub async fn reconcile_branch(&mut self, _alternative_name: &str, _message: Option<String>, _authors: Vec<vcs::Author>) -> Result<String, SyncError> {
        Err(SyncError::Vcs("branch reconciliation requires the retained envelope candidate job; synchronous whole-envelope duplication is forbidden".into()))
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
    // 🚫️async: E1 pure struct-literal builder consumed by `impl Default` (sync-only external
    // trait) — see R9. No I/O, no suspension point.
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
    pub cmd_tx: ArtifactMailboxSender,
    pub document_key: ArtifactDocumentKey,
    #[cfg(not(target_arch = "wasm32"))]
    pub runner: ArtifactActorRunnerTicket,
    /// @emoji 🔗️ The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Backbones::Channel(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

struct OpenDocument {
    generation: u64,
    cancel: semio_framework_async::CancelToken,
    cmd_tx: ArtifactMailboxSender,
    events: broadcast::Sender<ArtifactEvent>,
    presence: PresenceHeartbeatProducer,
    #[cfg(not(target_arch = "wasm32"))]
    runner: ArtifactActorRunnerHandle,
}

struct ArtifactHostState {
    documents: std::collections::HashMap<ArtifactDocumentKey, OpenDocument>,
    document_socket_surfaces: std::collections::HashMap<ArtifactDocumentKey, crate::os_directory::client::DocumentSocketSurfaceExpectationV1>,
    #[cfg(not(target_arch = "wasm32"))]
    closing: std::collections::HashMap<u64, ArtifactActorRunnerHandle>,
    next_generation: u64,
    host_references: usize,
}

impl ArtifactHostState {
    fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
            document_socket_surfaces: std::collections::HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            closing: std::collections::HashMap::new(),
            next_generation: 1,
            host_references: 1,
        }
    }

    fn claim_generation(&mut self) -> u64 {
        let generation = self.next_generation;
        self.next_generation = self.next_generation.checked_add(1).expect("artifact actor generation exhausted");
        generation
    }
}

/// @emoji 🏛️ Registry of open per-document actors. One `ArtifactHost` per host process (wgpu native,
/// tests, or the browser wgpu build) owns every open document's actor + event fan-out.
pub struct ArtifactHost {
    inner: std::sync::Arc<std::sync::Mutex<ArtifactHostState>>,
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    credential: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<crate::os_directory::client::LocalHubCredential>>>>,
    socket_grant_source: std::sync::Arc<std::sync::RwLock<Option<std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
    cancel: semio_framework_async::CancelToken,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.host_references = state.host_references.checked_add(1).expect("artifact host reference capacity exhausted");
        drop(state);
        Self { inner: self.inner.clone(), pool: self.pool.clone(), credential: self.credential.clone(), socket_grant_source: self.socket_grant_source.clone(), cancel: self.cancel.clone() }
    }
}

impl ArtifactHost {
    /// @emoji 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
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

    pub fn set_local_hub_credential(&self, credential: std::sync::Arc<crate::os_directory::client::LocalHubCredential>) {
        *self.credential.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: std::sync::Arc<dyn crate::os_directory::client::HubSocketGrantSource>) {
        *self.socket_grant_source.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(source);
    }

    /// 🎯 Binds the exact native plugin/surface selection before this document actor is opened.
    pub fn set_document_socket_surface(&self, document_key: &ArtifactDocumentKey, surface: crate::os_directory::client::DocumentSocketSurfaceExpectationV1) -> bool {
        if !matches!(document_key, ArtifactDocumentKey::Hub { .. }) {
            return false;
        }
        let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.documents.contains_key(document_key) || state.document_socket_surfaces.contains_key(document_key) {
            return false;
        }
        state.document_socket_surfaces.insert(document_key.clone(), surface);
        true
    }

    pub fn local_hub_ready(&self) -> bool {
        self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            && self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    /// @emoji 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub async fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        let document_key = ArtifactDocumentKey::for_config(&config);
        let document_socket_surface = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).document_socket_surfaces.remove(&document_key);
        let _ = self.close_key(&document_key);
        let generation = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).claim_generation();
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}")).await;
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (event_tx, _event_rx) = broadcast::channel(256);
        let document_cancel = self.cancel.child_now();
        #[cfg(not(target_arch = "wasm32"))]
        let runner = spawn_actor(
            self.pool.clone(),
            generation,
            config,
            remote,
            cmd_rx,
            event_tx.clone(),
            self.credential.clone(),
            self.socket_grant_source.clone(),
            document_socket_surface,
            document_cancel.clone(),
        )
        .await;
        // 🌉️ Narrowed to match `mod wasm_actor`'s own gate: it is a browser WebSocket/`web_sys`
        // bridge, and `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too. On the WASI
        // component target neither actor exists — `native_actor` is `tokio_tungstenite`/
        // `tokio::net::TcpStream`, which a component cannot open — so `open` registers the document
        // and hands back its channels WITHOUT a sync actor, which is the only thing a component
        // with no socket of its own can do. That is already the documented story: no plugin
        // activates the `sync`/`worker` features that reach this module (see `mod wasm_actor`).
        // `OpenDocument::runner` and `ArtifactChannels::runner` are themselves
        // `cfg(not(target_arch = "wasm32"))`, so nothing downstream expects a runner here.
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        spawn_actor(self.pool.clone(), config, remote, cmd_rx, event_tx.clone()).await;
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        let _ = (&self.pool, config, remote, cmd_rx);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let weak_host = std::sync::Arc::downgrade(&self.inner);
            runner.set_terminal_empty_callback(std::sync::Arc::new(move |generation| {
                if let Some(host) = weak_host.upgrade() {
                    host.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.remove(&generation);
                }
            }));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let runner_ticket = runner.issue_ticket(self.inner.clone());
        let entry = OpenDocument {
            generation,
            cancel: document_cancel,
            cmd_tx: cmd_tx.clone(),
            events: event_tx,
            presence: PresenceHeartbeatProducer::default(),
            #[cfg(not(target_arch = "wasm32"))]
            runner: runner.clone(),
        };
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.insert(document_key.clone(), entry);
        #[cfg(not(target_arch = "wasm32"))]
        runner.start();
        ArtifactChannels {
            cmd_tx,
            document_key,
            #[cfg(not(target_arch = "wasm32"))]
            runner: runner_ticket,
            channel_backbone,
        }
    }

    /// @emoji 📬️ A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.subscribe_key(&ArtifactDocumentKey::local(document_id)).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        let guard = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match guard.documents.get(document_key) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// @emoji 🔔️ Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.send_key(&ArtifactDocumentKey::local(document_id), message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        if let Some(document) = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.get(document_key) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// @emoji 💓️ Offers a generic cursor/viewport/app-presence heartbeat for one open document.
    /// Returns `true` only when the host actually queued a publish; faster offers are coalesced onto
    /// the document's producer and cannot flood the preview lane.
    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.presence_heartbeat_key(&ArtifactDocumentKey::local(document_id), now_ms, peer)
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = state.documents.get_mut(document_key) else { return false };
        let Some(peer) = document.presence.offer(now_ms, peer) else { return false };
        let sent = document.cmd_tx.send(ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(peer) }).is_ok();
        sent
    }

    /// @emoji ✂️ Transfers a document into generation-keyed retained close ownership. The runner
    /// rejects later mailbox ingress and drains one mailbox, backbone, actor, or job owner per grant.
    // 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    // `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.remove(document_key);
        if let Some(document) = document {
            document.cancel.cancel_now();
            #[cfg(not(target_arch = "wasm32"))]
            {
                let generation = document.generation;
                let runner = document.runner;
                self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.insert(generation, runner.clone());
                if matches!(runner.terminal_state(), ArtifactActorTerminalState::Complete(_)) {
                    self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.remove(&generation);
                    return Some(generation);
                }
                runner.request_close();
                return Some(generation);
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = document.cmd_tx.send(ArtifactActorMsg::Detach);
                return Some(document.generation);
            }
        }
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn closing_runner(&self, generation: u64) -> Option<ArtifactActorRunnerHandle> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.get(&generation).cloned()
    }

    /// @emoji 🧹️ Ids of every currently-open document.
    // 🚫️async: E1 pure lock-and-collect consumed by `impl Drop` (sync-only external trait) — see
    // R9. No I/O, no suspension point.
    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.keys().cloned().collect()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        let is_last_host = {
            let mut state = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.host_references = state.host_references.checked_sub(1).expect("artifact host reference underflow");
            state.host_references == 0
        };
        if !is_last_host {
            return;
        }
        self.cancel.cancel_now();
        for document_key in self.open_artifacts() {
            self.close_key(&document_key);
        }
    }
}
//#endregion 🔖️Host

//#region 🔖️NativeActor
#[cfg(not(target_arch = "wasm32"))]
mod native_actor {
    use super::*;
    use futures::{SinkExt, StreamExt};
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::Instant;
    use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

    type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
    type WsSink = futures::stream::SplitSink<WsStream, Message>;
    type WsRead = futures::stream::SplitStream<WsStream>;
    struct ConnectedDocumentSocket {
        stream: WsStream,
        socket_actor: String,
        authority: crate::os_directory::client::DocumentSocketAuthorityV1,
    }

    struct WipeSocketHeader(String);

    impl Drop for WipeSocketHeader {
        fn drop(&mut self) {
            unsafe { self.0.as_mut_vec().fill(0) };
        }
    }

    type ConnectFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ConnectedDocumentSocket, ()>> + Send>>;

    struct HubConn {
        write: WsSink,
        read: WsRead,
    }

    struct PendingArtifactBootstrap {
        assembler: ArtifactBootstrapAssembler,
        started_at: Instant,
        resume_token: String,
        baseline_frontier: RuntimeFrontierSummary,
        required_tail_frontier: RuntimeFrontierSummary,
        pack_schema_hash: [u8; 32],
    }

    struct NativeBootstrapControl {
        cancelled: bool,
        now_ms: u64,
        events: broadcast::Sender<ArtifactEvent>,
    }

    impl ArtifactBootstrapControl for NativeBootstrapControl {
        fn is_cancelled(&mut self) -> bool {
            self.cancelled
        }

        fn now_ms(&mut self) -> u64 {
            self.now_ms
        }

        fn on_progress(&mut self, progress: ArtifactBootstrapProgress) {
            let _ = self.events.send(ArtifactEvent::BootstrapProgress {
                received_bytes: progress.received_bytes,
                total_bytes: progress.total_bytes,
                received_chunks: progress.received_chunks,
                total_chunks: progress.total_chunks,
            });
        }
    }

    #[derive(Clone, Copy)]
    enum ArtifactDrivePhase {
        Connect,
        Hub,
        ConnectResult,
        Watch,
        Reconnect,
        Folder,
        Backbone,
        Status,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum ArtifactDrive {
        MoreWork,
        Idle { deadline: Option<Instant> },
        Terminal,
    }

    /// @emoji 📁️ A folder/file binding's storage driver, keyed for multi-document sqlite or single
    /// pack-backed blob. Both variants move real `pack`+`spr` bytes end to end — this actor never
    /// touches JSON. `Sqlite` is fully codec-free (its row IS the pack+spr pair, no bridging
    /// needed). `Pack` requires the schema codec for both authoritative DSL fallback validation
    /// and synchronized text mirrors.
    enum FolderEndpoint {
        EventLog { storage: FolderEventLogStorage, document_id: String, schema: String },
        Pack { storage: FolderTextStorage, document_id: String, extension: String, schema: String },
    }

    impl FolderEndpoint {
        /// @emoji 📖️ `Ok(None)` = nothing persisted yet; `Ok(Some(pack, spr))` = the authoritative
        /// binary pair, resolved pack-first (falling back to compiling the DSL mirror for
        /// hand-authored/imported documents with no `.pack` file yet — `Sqlite` has no such
        /// fallback, a row is always written pack+spr together); `Err` = a real storage failure.
        async fn read(&self) -> Result<Option<(Vec<u8>, Vec<u8>)>, String> {
            match self {
                FolderEndpoint::EventLog { storage, document_id, .. } => storage.read(document_id).await.map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    if let Some(pack_files) = storage.read_pack(document_id, extension).await.map_err(|error| error.to_string())? {
                        return Ok(Some((pack_files.pack, pack_files.spr)));
                    }
                    let Some(text_files) = storage.read(document_id, extension).await.map_err(|error| error.to_string())? else {
                        return Ok(None);
                    };
                    let codec = crate::os_store::document_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {schema:?} — cannot compile the DSL-only fallback"))?;
                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.map_err(|error| error.to_string())?;
                    Ok(Some((pack_files.pack, pack_files.spr)))
                }
            }
        }

        /// @emoji ✍️ Persists the authoritative `pack`+`spr` pair. `Sqlite` needs no codec at all;
        /// `Pack` additionally writes the `.dsl`/`.ops` logging mirrors when a codec is registered
        /// and schema codec; a missing or unavailable codec aborts the write before storage changes.
        async fn write(&self, pack: &[u8], spr: &[u8]) -> Result<(), String> {
            match self {
                FolderEndpoint::EventLog { storage, document_id, schema } => storage.write(document_id, schema, pack, spr).await.map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    let codec = crate::os_store::document_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {schema:?} — cannot persist synchronized pack mirrors"))?;
                    let mirror = (codec.print_mirror)(pack, spr).await.map_err(|error| error.to_string())?;
                    let pack_files = ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: mirror.ops };
                    storage.write_pack(document_id, extension, &pack_files, &mirror.dsl).await.map_err(|error| error.to_string())
                }
            }
        }
    }

    /// @emoji 🎭️ One document's backbone actor: drains the store's outbound queue to persist + relay,
    /// ingests semio_hub/file changes back into the store, and keeps subscribers current with status/events.
    pub(super) struct ArtifactActor {
        pool: Arc<semio_framework_async::WorkerPool>,
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<ArtifactEvent>,
        cmd_rx: ArtifactMailboxReceiver,
        folder: Option<FolderEndpoint>,
        folder_watch_path: Option<PathBuf>,
        watch_external: bool,
        hub_base_url: Option<String>,
        hub_space_id: Option<String>,
        credential: Arc<std::sync::RwLock<Option<Arc<crate::os_directory::client::LocalHubCredential>>>>,
        socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
        document_socket_surface: Option<crate::os_directory::client::DocumentSocketSurfaceExpectationV1>,
        operation_cancel: semio_framework_async::CancelToken,
        hub_surface: Option<String>,
        socket_actor: Option<String>,
        socket_actor_confirmed: bool,
        socket_authority: Option<crate::os_directory::client::DocumentSocketAuthorityV1>,
        socket_authority_deadline: Option<Instant>,
        /// @emoji 🎨️ This connection's hub-assigned session color (`ServerFrame::Session.color`) —
        /// `None` until the hub sends it (or for a folder-only document, which never connects to a
        /// hub). Stamped onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
        session_color: Option<u8>,
        semio_hub: Option<HubConn>,
        connect_future: Option<ConnectFuture>,
        /// @emoji 🏔️ Last frontier the semio_hub reported (`Welcome.server_frontier` / `Commands.frontier` /
        /// `Ack.frontier`) — the wire-v2 replacement for the old `hub_version: i64` counter.
        server_frontier: Option<crate::os_spr::RuntimeFrontierSummary>,
        /// @emoji 🎟️ The semio_hub's last `Welcome.resume_token`, echoed in the next `SocketHelloV1` after a
        /// reconnect so the semio_hub can resume rather than replay from scratch.
        resume_token: Option<String>,
        pending_resume_token: Option<String>,
        required_tail_frontier: Option<RuntimeFrontierSummary>,
        artifact_bootstrap: Option<PendingArtifactBootstrap>,
        backoff_ms: u64,
        reconnect_at: Option<Instant>,
        /// @emoji 🧺️ Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`, so `Rejected`/
        /// `Transformed` can roll back exactly the envelopes that batch sent.
        pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
        outbox: Vec<MutationEnvelope>,
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
        watcher: Option<semio_framework_os_services::OwnedFileChangeWatcher>,
        fs_deadline: Option<Instant>,
        started: bool,
        command_turns: u8,
        drive_phase: ArtifactDrivePhase,
        closing: bool,
        readiness: Option<Arc<dyn Fn() + Send + Sync>>,
        #[cfg(test)]
        fail_bootstrap_local_replay_once: bool,
    }

    impl ArtifactActor {
        pub(super) async fn new(
            pool: Arc<semio_framework_async::WorkerPool>,
            config: ArtifactActorConfig,
            remote: ChannelBackboneRemote,
            cmd_rx: ArtifactMailboxReceiver,
            events: broadcast::Sender<ArtifactEvent>,
            credential: Arc<std::sync::RwLock<Option<Arc<crate::os_directory::client::LocalHubCredential>>>>,
            socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
            document_socket_surface: Option<crate::os_directory::client::DocumentSocketSurfaceExpectationV1>,
            operation_cancel: semio_framework_async::CancelToken,
        ) -> Self {
            let mut folder = None;
            let mut folder_watch_path = None;
            let mut hub_base_url = None;
            let mut hub_space_id = None;
            let mut hub_surface = None;
            for binding in &config.bindings {
                match binding {
                    PersistenceBinding::Folder { path } => {
                        if folder.is_none() {
                            folder = Some(build_folder_endpoint(path, &config.document_id, &config.schema).await);
                            folder_watch_path = Some(folder_watch_path_for(path).await);
                        }
                    }
                    PersistenceBinding::Hub { base_url, space_id, surface } => {
                        if hub_base_url.is_none() {
                            hub_base_url = Some(base_url.clone());
                            hub_space_id = Some(space_id.clone());
                            hub_surface = surface.clone();
                        }
                    }
                }
            }
            let hlc_seed = actor_seed(&config.actor).await;
            Self {
                pool,
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
                credential,
                socket_grant_source,
                document_socket_surface,
                operation_cancel,
                hub_surface,
                socket_actor: None,
                socket_actor_confirmed: false,
                socket_authority: None,
                socket_authority_deadline: None,
                session_color: None,
                semio_hub: None,
                connect_future: None,
                server_frontier: None,
                resume_token: None,
                pending_resume_token: None,
                required_tail_frontier: None,
                artifact_bootstrap: None,
                backoff_ms: 500,
                reconnect_at: None,
                pending_batches: std::collections::HashMap::new(),
                outbox: Vec::new(),
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
                fs_deadline: None,
                started: false,
                command_turns: 0,
                drive_phase: ArtifactDrivePhase::Connect,
                closing: false,
                readiness: None,
                #[cfg(test)]
                fail_bootstrap_local_replay_once: false,
            }
        }

        fn set_readiness(&mut self, readiness: Arc<dyn Fn() + Send + Sync>) {
            self.readiness = Some(readiness);
        }

        /// @emoji 🏃️ Advances exactly one command, readiness source, timer, backbone owner, or status turn.
        pub(super) async fn drive_one(&mut self) -> ArtifactDrive {
            if !self.started {
                self.started = true;
                self.setup().await;
                return ArtifactDrive::MoreWork;
            }
            if self.closing {
                self.operation_cancel.cancel_now();
                if let Some(mut pending) = self.artifact_bootstrap.take() {
                    pending.assembler.abort();
                }
                if self.cmd_rx.close_one() {
                    return ArtifactDrive::MoreWork;
                }
                match self.relay_one_backbone().await {
                    Ok(true) | Err(_) => return ArtifactDrive::MoreWork,
                    Ok(false) => {}
                }
                self.connect_future = None;
                return ArtifactDrive::Terminal;
            }
            if self.command_turns < 32 {
                if let Some(message) = self.cmd_rx.try_recv() {
                    self.command_turns += 1;
                    if self.handle_cmd(message).await {
                        self.closing = true;
                        self.cmd_rx.close();
                    }
                    return ArtifactDrive::MoreWork;
                }
            }
            self.command_turns = 0;
            let phase = self.drive_phase;
            self.drive_phase = match phase {
                ArtifactDrivePhase::Connect => ArtifactDrivePhase::Hub,
                ArtifactDrivePhase::Hub => ArtifactDrivePhase::ConnectResult,
                ArtifactDrivePhase::ConnectResult => ArtifactDrivePhase::Watch,
                ArtifactDrivePhase::Watch => ArtifactDrivePhase::Reconnect,
                ArtifactDrivePhase::Reconnect => ArtifactDrivePhase::Folder,
                ArtifactDrivePhase::Folder => ArtifactDrivePhase::Backbone,
                ArtifactDrivePhase::Backbone => ArtifactDrivePhase::Status,
                ArtifactDrivePhase::Status => ArtifactDrivePhase::Connect,
            };
            match phase {
                ArtifactDrivePhase::Connect => self.start_connect_hub().await,
                ArtifactDrivePhase::Hub => {
                    let message = std::future::poll_fn(|context| {
                        let polled = self.semio_hub.as_mut().map(|connection| connection.read.poll_next_unpin(context));
                        std::task::Poll::Ready(match polled {
                            Some(std::task::Poll::Ready(message)) => Some(message),
                            Some(std::task::Poll::Pending) | None => None,
                        })
                    })
                    .await;
                    if let Some(message) = message {
                        self.on_hub_message(message).await;
                    }
                }
                ArtifactDrivePhase::ConnectResult => {
                    let connection = std::future::poll_fn(|context| {
                        let polled = self.connect_future.as_mut().map(|future| future.as_mut().poll(context));
                        std::task::Poll::Ready(match polled {
                            Some(std::task::Poll::Ready(connection)) => Some(connection),
                            Some(std::task::Poll::Pending) | None => None,
                        })
                    })
                    .await;
                    if let Some(connection) = connection {
                        self.connect_future = None;
                        self.finish_connect_hub(connection).await;
                    }
                }
                ArtifactDrivePhase::Watch => {
                    if self.watcher.as_mut().is_some_and(|watcher| watcher.poll_changed()) {
                        self.fs_deadline = Some(Instant::now() + Duration::from_millis(200));
                    }
                }
                ArtifactDrivePhase::Reconnect => {
                    if self.reconnect_at.is_some_and(|deadline| deadline <= Instant::now()) {
                        self.reconnect_at = None;
                        self.start_connect_hub().await;
                    }
                }
                ArtifactDrivePhase::Folder => {
                    if self.fs_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                        self.fs_deadline = None;
                        self.handle_external_change().await;
                    }
                }
                ArtifactDrivePhase::Backbone => {
                    let _ = self.relay_one_backbone().await;
                }
                ArtifactDrivePhase::Status => {
                    if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                        self.invalidate_socket_authority().await;
                        return ArtifactDrive::MoreWork;
                    }
                    self.emit_status_if_changed().await;
                    return ArtifactDrive::Idle { deadline: [self.reconnect_at, self.fs_deadline, self.socket_authority_deadline].into_iter().flatten().min() };
                }
            }
            ArtifactDrive::MoreWork
        }

        /// @emoji 🌱️ Seeds persistence state from any already-stored pack+spr and installs the file watcher.
        async fn setup(&mut self) {
            let seeded = match self.folder.as_ref() {
                Some(folder) => folder.read().await.ok().flatten(),
                None => None,
            };
            if let Some((pack, spr)) = seeded {
                if let Ok(op_ids) = spr_op_ids(&spr).await {
                    self.known_op_ids = op_ids;
                    self.last_written_hash = Some(backbone_pack_hash(&pack, &spr));
                    self.current_pack = Some(pack);
                    self.current_spr = Some(spr);
                }
            }
            if self.watch_external {
                if let Some(watch_path) = self.folder_watch_path.clone() {
                    if let Some(readiness) = self.readiness.clone() {
                        self.watcher = Some(install_watcher(&watch_path, self.pool.clone(), readiness));
                    }
                }
            }
        }

        /// @emoji 📨️ Handles a caller control message. Returns `true` when the actor should stop.
        async fn handle_cmd(&mut self, message: ArtifactActorMsg) -> bool {
            match message {
                ArtifactActorMsg::LocalMutations { envelopes } => {
                    let drained = self.relay_one_backbone().await.unwrap_or(false);
                    if !drained && !envelopes.is_empty() {
                        self.persist_operations(&envelopes).await;
                        self.relay_operations_to_hub(&envelopes).await;
                    }
                    false
                }
                ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                    stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                    self.send_client_frame(ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
                    false
                }
                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_client_frame(ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                    false
                }
                ArtifactActorMsg::ExternalChanged => {
                    self.handle_external_change().await;
                    false
                }
                ArtifactActorMsg::Detach => true,
            }
        }

        /// @emoji 📤️ Pops and advances exactly one store-to-actor FIFO owner.
        async fn relay_one_backbone(&mut self) -> Result<bool, vcs::VcsError> {
            let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
            match message {
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                    self.persist_operations(&envelopes).await;
                    self.relay_operations_to_hub(&envelopes).await;
                }
                BackboneMessage::Snapshot { pack, spr } => self.persist_snapshot(pack, spr).await,
                BackboneMessage::Ack { .. } => {}
            }
            Ok(true)
        }

        //#region 🔖️Folder
        /// @emoji ✍️ Persists the current pack+spr bytes to the folder binding and records the
        /// content hash for self-write suppression. A write failure (e.g. no `crate::os_store::ArtifactCodec`
        /// registered for this document's schema on the `Pack` endpoint — see `FolderEndpoint::write`)
        /// is swallowed here the same way every other best-effort path in this actor already is, but
        /// deliberately does NOT record `last_written_hash` on failure — a false "persisted" mark
        /// would make `handle_external_change` mistake the still-stale on-disk content for a
        /// self-write and ignore a real external change.
        async fn persist_write(&mut self, pack: &[u8], spr: &[u8]) {
            let Some(folder) = self.folder.as_ref() else { return };
            if folder.write(pack, spr).await.is_ok() {
                self.last_written_hash = Some(backbone_pack_hash(pack, spr));
            }
        }

        /// @emoji 📸️ Records a full pack+spr snapshot as the canonical persisted state.
        async fn persist_snapshot(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
            if self.folder.is_none() {
                return;
            }
            if let Ok(op_ids) = spr_op_ids(&spr).await {
                self.known_op_ids = op_ids;
            }
            self.persist_write(&pack, &spr).await;
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
        }

        /// @emoji ➕️ Appends locally-applied operations to the persisted spr log (append-only),
        /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
        async fn persist_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if self.folder.is_none() {
                return;
            }
            let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
            let mut new_edits: Vec<crate::os_spr::HistoryEdit> = Vec::new();
            for envelope in envelopes.iter().filter(|envelope| self.known_op_ids.insert(envelope.mutation_id.0.clone())) {
                new_edits.push(history_edit_from_envelope(envelope).await);
            }
            if new_edits.is_empty() {
                return;
            }
            let Ok(new_spr) = crate::os_store::append_history_edits_to_spr(&spr, &new_edits).await else { return };
            self.persist_write(&pack, &new_spr).await;
            self.current_pack = Some(pack);
            self.current_spr = Some(new_spr);
        }

        /// @emoji 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`,
        /// divergence → `SnapshotReplaced`, divergence with local pending operations → `Conflict`. Self-writes
        /// (content hash match) are ignored.
        async fn handle_external_change(&mut self) {
            let seeded = match self.folder.as_ref() {
                Some(folder) => folder.read().await.ok().flatten(),
                None => None,
            };
            let Some((pack, spr)) = seeded else { return };
            let hash = backbone_pack_hash(&pack, &spr);
            if self.last_written_hash.as_deref() == Some(hash.as_str()) {
                return;
            }
            let Ok(file_ids) = spr_op_ids(&spr).await else { return };
            let lost: Vec<String> = self.known_op_ids.difference(&file_ids).cloned().collect();
            let new_ids: HashSet<String> = file_ids.difference(&self.known_op_ids).cloned().collect();

            if lost.is_empty() && !new_ids.is_empty() {
                let Ok(reader) = crate::os_spr::HistoryReader::open(&spr, &crate::os_spr::DecodeOptions::default()).await else { return };
                let mut appended = Vec::new();
                for edit in reader.edits().await {
                    let Ok(edit) = edit else { break };
                    if op_ids_of(&edit).await.iter().any(|id| new_ids.contains(id)) {
                        if let Ok(mut envelopes) = envelopes_from_history_edit(&edit, &self.document_id, &self.schema).await {
                            appended.append(&mut envelopes);
                        }
                    }
                }
                self.known_op_ids.extend(new_ids);
                self.current_pack = Some(pack);
                self.current_spr = Some(spr);
                self.last_written_hash = Some(hash);
                let _ = self.deliver_remote_operations(appended).await;
            } else if !lost.is_empty() {
                if !self.pending_batches.is_empty() {
                    self.emit(ArtifactEvent::Conflict(MutationMessage {
                        level: crate::os_dsl::Severity::Error,
                        code: crate::os_dsl::FaultCode::new("externalDivergence"),
                        message: "external history diverged while local operations are pending".into(),
                        target: vec![format!("folder://{}", self.document_id)],
                        op_index: None,
                    }));
                } else {
                    self.known_op_ids = file_ids;
                    self.current_pack = Some(pack.clone());
                    self.current_spr = Some(spr.clone());
                    self.last_written_hash = Some(hash);
                    self.deliver_snapshot(pack, spr).await;
                }
            }
        }
        //#endregion 🔖️Folder

        //#region 🔖️Hub
        fn bootstrap_control(&self, started_at: Instant) -> NativeBootstrapControl {
            NativeBootstrapControl { cancelled: self.closing, now_ms: started_at.elapsed().as_millis().min(u128::from(u64::MAX)) as u64, events: self.events.clone() }
        }

        fn abort_artifact_bootstrap(&mut self) {
            if let Some(mut pending) = self.artifact_bootstrap.take() {
                pending.assembler.abort();
            }
            self.pending_resume_token = None;
            self.required_tail_frontier = None;
        }

        fn requeue_pending_batches(&mut self) {
            let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
            batches.sort_by_key(|(batch_id, _)| *batch_id);
            for (_, envelopes) in batches {
                self.queue_outbox(envelopes);
            }
        }

        fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
            let mut queued: HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
            for envelope in envelopes {
                if queued.insert(envelope.mutation_id.0.clone()) {
                    self.outbox.push(envelope);
                }
            }
        }

        fn clear_socket_epoch(&mut self) {
            self.socket_actor = None;
            self.socket_actor_confirmed = false;
            self.socket_authority = None;
            self.socket_authority_deadline = None;
            self.session_color = None;
        }

        async fn invalidate_socket_authority(&mut self) {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            if let Some(mut connection) = self.semio_hub.take() {
                let _ = tokio::time::timeout(Duration::from_millis(4), connection.write.close()).await;
            }
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
        }

        async fn fail_artifact_bootstrap(&mut self, detail: impl Into<String>) {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.emit(ArtifactEvent::Conflict(MutationMessage {
                level: crate::os_dsl::Severity::Error,
                code: crate::os_dsl::FaultCode::new("artifactBootstrap"),
                message: detail.into(),
                target: vec![self.document_id.clone()],
                op_index: None,
            }));
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
        }

        async fn flush_outbox(&mut self) {
            if self.outbox.is_empty() || self.semio_hub.is_none() {
                return;
            }
            let envelopes = std::mem::take(&mut self.outbox);
            self.relay_operations_to_hub(&envelopes).await;
        }

        async fn finish_catchup_if_ready(&mut self) {
            let Some(required) = self.required_tail_frontier.clone() else { return };
            let Some(actual) = self.server_frontier.as_ref() else { return };
            if !frontier_reaches(actual, &required) {
                return;
            }
            self.required_tail_frontier = None;
            if let Some(resume_token) = self.pending_resume_token.take() {
                self.resume_token = Some(resume_token);
            }
            self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
            self.flush_outbox().await;
        }

        async fn start_connect_hub(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.connect_future.is_some() {
                return;
            }
            if self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                self.schedule_reconnect().await;
                return;
            }
            let Some(source) = self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone() else {
                self.schedule_reconnect().await;
                return;
            };
            let space_id = self.hub_space_id.clone().unwrap_or_default();
            let document_id = self.document_id.clone();
            let surface = self.hub_surface.clone();
            let schema = self.schema.clone();
            let Some(pack_schema_hash) = crate::os_store::document_codec(&schema).await.ok().flatten().map(|codec| codec.pack_schema_hash) else {
                self.schedule_reconnect().await;
                return;
            };
            let expectation = crate::os_directory::client::DocumentSocketExpectationV1 {
                artifact_schema: schema,
                pack_schema_hash,
                requested_surface_id: surface,
                surface: self.document_socket_surface.clone(),
            };
            let client_instance_id = format!("native-document-{:016x}", self.hlc_seed);
            let operation_cancel = self.operation_cancel.child_now();
            self.set_remote_state(RemoteState::Connecting).await;
            self.connect_future = Some(Box::pin(async move {
                let ctx = semio_framework_async::OperationContext {
                    actor: 0,
                    generation: 0,
                    trace: semio_framework_async::TraceId(0),
                    lane: 1,
                    deadline_ms: None,
                    cancel: operation_cancel,
                    capability: None,
                };
                let admission_ctx = ctx.clone();
                let admission =
                    tokio::task::spawn_blocking(move || source.admit_document_socket(&admission_ctx, &space_id, &document_id, &expectation, &client_instance_id, 5_000)).await.map_err(|_| ())?.map_err(|_| ())?;
                if ctx.cancel.is_cancelled_now() || admission.authority.expires_at_unix_ms <= now_ms().await {
                    return Err(());
                }
                if base_url.trim_end_matches('/') != admission.authority.hub_origin.trim_end_matches('/') {
                    return Err(());
                }
                let url = hub_ws_url(&admission.authority.hub_origin, &admission.authority.scope.space_id, &admission.authority.scope.document_id, Some(&admission.authority.surface.surface_id)).await;
                let crate::os_directory::client::DocumentSocketAdmissionV1 { mut socket, authority } = admission;
                let mut request = url.into_client_request().map_err(|_| ())?;
                let protocol_header = WipeSocketHeader(format!("{}, {}", socket.protocol, socket.grant));
                request.headers_mut().insert("Sec-WebSocket-Protocol", protocol_header.0.parse().map_err(|_| ())?);
                let (mut stream, response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| ())?.map_err(|_| ())?;
                if response.headers().get("Sec-WebSocket-Protocol").and_then(|value| value.to_str().ok()) != Some("semio.socket.v1") {
                    let _ = stream.close(None).await;
                    return Err(());
                }
                if ctx.cancel.is_cancelled_now() || authority.expires_at_unix_ms <= now_ms().await {
                    let _ = stream.close(None).await;
                    return Err(());
                }
                let socket_actor = std::mem::take(&mut socket.actor_id);
                Ok(ConnectedDocumentSocket { stream, socket_actor, authority })
            }));
        }

        async fn finish_connect_hub(&mut self, connection: Result<ConnectedDocumentSocket, ()>) {
            match connection {
                Ok(ConnectedDocumentSocket { mut stream, socket_actor, authority }) => {
                    let local_schema_hash = crate::os_store::document_codec(&self.schema).await.ok().flatten().map(|codec| codec.pack_schema_hash);
                    let now = now_ms().await;
                    if self.operation_cancel.is_cancelled_now()
                        || authority.expires_at_unix_ms <= now
                        || authority.hub_origin.trim_end_matches('/') != self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/')
                        || authority.scope.space_id != self.hub_space_id.as_deref().unwrap_or_default()
                        || authority.scope.document_id != self.document_id
                        || authority.artifact.schema != self.schema
                        || local_schema_hash != Some(authority.pack_schema_hash)
                        || self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id)
                        || self.document_socket_surface.as_ref().is_some_and(|surface| !authority.matches_surface(surface))
                    {
                        let _ = stream.close(None).await;
                        self.clear_socket_epoch();
                        self.schedule_reconnect().await;
                        return;
                    }
                    let pack_schema_hash = authority.pack_schema_hash;
                    let (write, read) = stream.split();
                    self.semio_hub = Some(HubConn { write, read });
                    self.socket_actor = Some(socket_actor);
                    self.socket_actor_confirmed = false;
                    self.hub_surface = Some(authority.surface.surface_id.clone());
                    self.socket_authority_deadline = Some(Instant::now() + Duration::from_millis(authority.expires_at_unix_ms.saturating_sub(now)));
                    self.socket_authority = Some(authority);
                    self.session_color = None;
                    self.backoff_ms = 500;
                    let hello = ClientFrame::SocketHelloV1 {
                        wire_version: 1,
                        protocol_version: 1,
                        schema: self.schema.clone(),
                        pack_schema_hash,
                        resume_token: self.resume_token.clone(),
                        frontier: self.server_frontier.clone(),
                    };
                    self.send_client_frame(hello, Lane::Command).await;
                }
                Err(()) => {
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                }
            }
        }

        async fn schedule_reconnect(&mut self) {
            let retry = self.backoff_ms;
            self.set_remote_state(RemoteState::Backoff { retry_in_ms: retry }).await;
            self.reconnect_at = Some(Instant::now() + Duration::from_millis(retry));
            self.backoff_ms = (self.backoff_ms * 2).min(30_000);
        }

        async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.invalidate_socket_authority().await;
                return;
            }
            match message {
                Some(Ok(Message::Binary(bytes))) => {
                    match decode_server_frame(&bytes).await {
                        Ok((_lane, frame)) => self.on_hub_frame(frame).await,
                        Err(error) => self.fail_artifact_bootstrap(format!("malformed hub frame: {error}")).await,
                    }
                }
                Some(Ok(Message::Ping(payload))) => {
                    self.send_raw(Message::Pong(payload)).await;
                }
                Some(Ok(_)) => {}
                Some(Err(_)) | None => {
                    self.abort_artifact_bootstrap();
                    self.requeue_pending_batches();
                    self.semio_hub = None;
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                }
            }
        }

        async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
            self.abort_artifact_bootstrap();
            let codec = match crate::os_store::document_codec(&self.schema).await {
                Ok(Some(codec)) => codec,
                Ok(None) => {
                    self.fail_artifact_bootstrap(format!("no document codec registered for schema {:?}", self.schema)).await;
                    return;
                }
                Err(error) => {
                    self.fail_artifact_bootstrap(error.to_string()).await;
                    return;
                }
            };
            if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, codec.pack_schema_hash, &server_frontier) {
                self.fail_artifact_bootstrap(error).await;
                return;
            }
            let inline = bootstrap.inline.is_some();
            let started_at = Instant::now();
            let baseline_frontier = bootstrap.baseline_frontier.clone();
            let required_tail_frontier = bootstrap.required_tail_frontier.clone();
            let pack_schema_hash = bootstrap.pack_schema_hash;
            let mut control = self.bootstrap_control(started_at);
            let assembler = match ArtifactBootstrapAssembler::new(
                bootstrap.clone(),
                bootstrap.descriptor_hash,
                ArtifactBootstrapLimits::default(),
                Some(ARTIFACT_BOOTSTRAP_DEADLINE_MS),
                &mut control,
            ) {
                Ok(assembler) => assembler,
                Err(error) => {
                    self.fail_artifact_bootstrap(error.to_string()).await;
                    return;
                }
            };
            let mut pending = PendingArtifactBootstrap { assembler, started_at, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
            if !inline {
                self.artifact_bootstrap = Some(pending);
                return;
            }
            let mut control = self.bootstrap_control(started_at);
            match pending.assembler.finish(None, &mut control) {
                Ok(pair) => {
                    if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                        self.fail_artifact_bootstrap(error).await;
                    }
                }
                Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
            }
        }

        async fn install_artifact_bootstrap(&mut self, pending: PendingArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
            let codec = crate::os_store::document_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {:?}", self.schema))?;
            if codec.pack_schema_hash != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;
            let op_ids = spr_op_ids(&pair.spr).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
            let previous = self.current_pack.clone().zip(self.current_spr.clone());
            if let Some(folder) = self.folder.as_ref() {
                folder.write(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap persistence failed: {error}"))?;
            }
            if let Err(error) = self.remote.push(BackboneMessage::Snapshot { pack: pair.pack.clone(), spr: pair.spr.clone() }).await {
                if let (Some(folder), Some((pack, spr))) = (self.folder.as_ref(), previous) {
                    let _ = folder.write(&pack, &spr).await;
                }
                return Err(format!("artifact bootstrap store replacement failed: {error}"));
            }
            self.known_op_ids = op_ids;
            self.current_pack = Some(pair.pack.clone());
            self.current_spr = Some(pair.spr.clone());
            if self.folder.is_some() {
                self.last_written_hash = Some(backbone_pack_hash(&pair.pack, &pair.spr));
            }
            self.emit(ArtifactEvent::SnapshotReplaced { pack: pair.pack, spr: pair.spr });
            self.server_frontier = Some(pending.baseline_frontier);
            self.pending_resume_token = Some(pending.resume_token);
            self.required_tail_frontier = Some(pending.required_tail_frontier);
            if !self.outbox.is_empty() && self.replay_local_outbox_after_bootstrap().await.is_err() {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: crate::os_dsl::Severity::Error,
                    code: crate::os_dsl::FaultCode::new("artifactBootstrapLocalReplay"),
                    message: "artifact baseline committed; pending local replay will retry after reconnect".into(),
                    target: vec![self.document_id.clone()],
                    op_index: None,
                }));
                self.semio_hub = None;
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
                return Ok(());
            }
            self.finish_catchup_if_ready().await;
            Ok(())
        }

        async fn replay_local_outbox_after_bootstrap(&mut self) -> Result<(), vcs::VcsError> {
            #[cfg(test)]
            if std::mem::take(&mut self.fail_bootstrap_local_replay_once) {
                return Err(vcs::VcsError::Backbone("injected bootstrap local replay failure".into()));
            }
            self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&self.outbox) }).await
        }

        #[cfg(test)]
        pub(super) async fn inject_hub_frame(&mut self, frame: ServerFrame) {
            self.on_hub_frame(frame).await;
        }

        #[cfg(test)]
        pub(super) fn inject_bootstrap_local_replay_failure(&mut self) {
            self.fail_bootstrap_local_replay_once = true;
        }

        #[cfg(test)]
        pub(super) fn cancel_test_bootstrap(&mut self) {
            self.abort_artifact_bootstrap();
        }

        #[cfg(test)]
        pub(super) fn queue_test_outbox(&mut self, envelopes: Vec<MutationEnvelope>) {
            self.queue_outbox(envelopes);
        }

        #[cfg(test)]
        pub(super) fn install_test_socket_actor(&mut self, actor: &str) {
            self.socket_actor = Some(actor.into());
            self.socket_actor_confirmed = false;
        }

        #[cfg(test)]
        pub(super) async fn connect_test_socket(&mut self, url: &str, actor: &str) {
            let (stream, _) = tokio_tungstenite::connect_async(url).await.expect("test socket connects");
            let (write, read) = stream.split();
            self.semio_hub = Some(HubConn { write, read });
            self.socket_actor = Some(actor.into());
            self.socket_actor_confirmed = false;
            self.session_color = None;
            self.send_client_frame(
                ClientFrame::SocketHelloV1 {
                    wire_version: 1,
                    protocol_version: 1,
                    schema: self.schema.clone(),
                    pack_schema_hash: crate::os_store::document_codec(&self.schema).await.ok().flatten().map_or([0u8; 32], |codec| codec.pack_schema_hash),
                    resume_token: self.resume_token.clone(),
                    frontier: self.server_frontier.clone(),
                },
                Lane::Command,
            )
            .await;
        }

        #[cfg(test)]
        pub(super) fn socket_epoch_test_state(&self) -> (Option<String>, bool, usize, Vec<String>) {
            (self.socket_actor.clone(), self.socket_actor_confirmed, self.pending_batches.len(), self.outbox.iter().map(|envelope| envelope.actor.0.clone()).collect())
        }

        #[cfg(test)]
        pub(super) fn expire_test_socket_authority(&mut self) {
            self.socket_authority_deadline = Some(Instant::now());
        }

        #[cfg(test)]
        pub(super) async fn fail_test_connection(&mut self) {
            self.on_hub_message(None).await;
        }

        #[cfg(test)]
        pub(super) async fn fail_test_bootstrap(&mut self) {
            self.fail_artifact_bootstrap("injected bootstrap failure").await;
        }

        #[cfg(test)]
        pub(super) async fn run_test_connect_attempt(&mut self) {
            self.start_connect_hub().await;
            if let Some(future) = self.connect_future.take() {
                let outcome = future.await;
                self.finish_connect_hub(outcome).await;
            }
        }

        #[cfg(test)]
        pub(super) async fn relay_test_envelope(&mut self, envelope: MutationEnvelope) {
            self.relay_operations_to_hub(std::slice::from_ref(&envelope)).await;
        }

        #[cfg(test)]
        pub(super) fn bootstrap_test_state(&self) -> (Option<Vec<u8>>, Option<Vec<u8>>, Option<RuntimeFrontierSummary>, Option<RuntimeFrontierSummary>, Option<String>, Option<String>, RemoteState, Vec<String>) {
            (
                self.current_pack.clone(),
                self.current_spr.clone(),
                self.server_frontier.clone(),
                self.required_tail_frontier.clone(),
                self.resume_token.clone(),
                self.pending_resume_token.clone(),
                self.remote_state.clone(),
                self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect(),
            )
        }

        async fn on_hub_frame(&mut self, frame: ServerFrame) {
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.requeue_pending_batches();
                    match bootstrap {
                        Bootstrap::None => {
                            self.abort_artifact_bootstrap();
                            self.resume_token = Some(resume_token);
                            self.server_frontier = Some(server_frontier);
                            self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
                            self.flush_outbox().await;
                        }
                        Bootstrap::Tail => {
                            self.abort_artifact_bootstrap();
                            self.pending_resume_token = Some(resume_token);
                            self.required_tail_frontier = Some(server_frontier);
                            self.finish_catchup_if_ready().await;
                        }
                        Bootstrap::Snapshot { .. } => {
                            self.fail_artifact_bootstrap("database-private snapshot cannot seed an artifact client").await;
                        }
                        Bootstrap::ArtifactBootstrap(bootstrap) => {
                            self.start_artifact_bootstrap(bootstrap, resume_token, server_frontier).await;
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                    self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client").await;
                }
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.fail_artifact_bootstrap("rebootstrap control scope mismatch").await;
                    } else {
                        self.fail_artifact_bootstrap("rebootstrap-required").await;
                    }
                }
                ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                    let Some(mut pending) = self.artifact_bootstrap.take() else {
                        self.fail_artifact_bootstrap("artifact bootstrap chunk arrived without an active transfer").await;
                        return;
                    };
                    let mut control = self.bootstrap_control(pending.started_at);
                    match pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control) {
                        Ok(_) => self.artifact_bootstrap = Some(pending),
                        Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                    }
                }
                ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                    let Some(mut pending) = self.artifact_bootstrap.take() else {
                        self.fail_artifact_bootstrap("artifact bootstrap completion arrived without an active transfer").await;
                        return;
                    };
                    let mut control = self.bootstrap_control(pending.started_at);
                    match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                        Ok(pair) => {
                            if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                                self.fail_artifact_bootstrap(error).await;
                            }
                        }
                        Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                    }
                }
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    if self.artifact_bootstrap.is_some() {
                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion").await;
                        return;
                    }
                    if self.socket_actor.as_deref() != Some(origin.0.as_str()) {
                        let converted = envelopes;
                        self.persist_operations(&converted).await;
                        if !self.deliver_remote_operations(converted).await {
                            self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                            return;
                        }
                    }
                    self.server_frontier = Some(frontier);
                    self.finish_catchup_if_ready().await;
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                        self.fail_artifact_bootstrap("ack arrived before artifact catch-up completion").await;
                        return;
                    }
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages).await;
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if self.socket_actor.as_deref() != Some(actor.0.as_str()) {
                        self.emit(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let mut decoded: Vec<PresencePeer> = Vec::new();
                    for p in &peers {
                        if let Some(peer) = presence_from_bytes(p).await {
                            decoded.push(peer);
                        }
                    }
                    let peers = decoded;
                    if matches!(self.remote_state, RemoteState::Live { .. }) && self.artifact_bootstrap.is_none() && self.required_tail_frontier.is_none() {
                        self.set_remote_state(RemoteState::Live { peer_count: peers.len() }).await;
                    }
                    self.emit(ArtifactEvent::Presence { peers });
                }
                ServerFrame::Session { actor, color } => {
                    if self.socket_actor.as_deref() != Some(actor.as_str()) {
                        self.fail_artifact_bootstrap("socket receipt actor mismatch").await;
                        return;
                    }
                    self.socket_actor_confirmed = true;
                    self.session_color = Some(color);
                    self.emit(ArtifactEvent::Session { actor, color });
                    self.flush_outbox().await;
                }
                ServerFrame::CreditGrant { .. } => {
                    // 🪙️ Command-lane credit-based flow control: no client-side backpressure
                    // implemented this wave (scope is frame plumbing, not congestion control) —
                    // accepted and ignored.
                }
                ServerFrame::Error { code, message } => {
                    self.emit(ArtifactEvent::Conflict(MutationMessage { level: crate::os_dsl::Severity::Error, code: crate::os_dsl::FaultCode::new(code), message, target: vec![self.hub_base_url.clone().unwrap_or_default()], op_index: None }));
                }
            }
        }

        /// @emoji 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`
        /// just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
        /// local head first (via {@link rollback_envelope}, replayed as remote operations), and
        /// `Transformed` then delivers the semio_hub's replacement envelope the same way.
        async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                        for envelope in sent.iter().rev() {
                            rollbacks.push(rollback_envelope(envelope).await);
                        }
                        self.persist_operations(&rollbacks).await;
                        let _ = self.deliver_remote_operations(rollbacks).await;
                        let converted = *envelope;
                        self.persist_operations(std::slice::from_ref(&converted)).await;
                        let _ = self.deliver_remote_operations(vec![converted]).await;
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason, messages } => {
                        let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                        for envelope in sent.iter().rev() {
                            rollbacks.push(rollback_envelope(envelope).await);
                        }
                        self.persist_operations(&rollbacks).await;
                        let _ = self.deliver_remote_operations(rollbacks).await;
                        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason, messages } });
                    }
                }
            }
            self.emit_status_if_changed().await;
        }

        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.queue_outbox(envelopes.iter().cloned());
                self.invalidate_socket_authority().await;
                return;
            }
            let Some(socket_actor) = self.socket_actor.clone() else {
                self.queue_outbox(envelopes.iter().cloned());
                return;
            };
            if !self.socket_actor_confirmed || self.semio_hub.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                self.queue_outbox(envelopes.iter().cloned());
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let mut wire_envelopes: Vec<MutationEnvelope> = Vec::new();
            for envelope in envelopes {
                let timestamp = next_timestamp(self.hlc_seed, &mut self.hlc_counter).await;
                wire_envelopes.push(MutationEnvelope { actor: ActorId(socket_actor.clone()), timestamp, ..envelope.clone() });
            }
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_client_frame(ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
            self.emit_status_if_changed().await;
        }

        async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(&frame, lane).await;
            self.send_raw(Message::Binary(bytes.into())).await;
        }

        async fn send_raw(&mut self, message: Message) {
            if self.socket_authority_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                self.invalidate_socket_authority().await;
                return;
            }
            let mut failed = false;
            if let Some(conn) = self.semio_hub.as_mut() {
                if !matches!(tokio::time::timeout(Duration::from_millis(4), conn.write.send(message)).await, Ok(Ok(()))) {
                    failed = true;
                }
            }
            if failed {
                self.abort_artifact_bootstrap();
                self.requeue_pending_batches();
                self.semio_hub = None;
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
            }
        }
        //#endregion 🔖️Hub

        //#region 🔖️Deliver
        /// @emoji 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
        async fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            if self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).await.is_err() {
                return false;
            }
            self.emit(ArtifactEvent::RemoteMutations { envelopes });
            true
        }

        /// @emoji 📸️ Pushes a full pack+spr snapshot into the store's inbound queue and notifies subscribers.
        async fn deliver_snapshot(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
            let _ = self.remote.push(BackboneMessage::Snapshot { pack: pack.clone(), spr: spr.clone() }).await;
            self.emit(ArtifactEvent::SnapshotReplaced { pack, spr });
        }

        /// 🚫️async: E1 pure sync body — `broadcast::Sender::send` never suspends. Declaring this
        /// `async fn` forced `&ArtifactActor` to live across an await, which demanded
        /// `ArtifactActor: Sync` from every `!Sync` field it owns (the folder watcher's `Receiver`,
        /// `connect_future`, the codec table's `dyn Fn`) and made the whole actor turn non-`Send`.
        fn emit(&self, event: ArtifactEvent) {
            let _ = self.events.send(event);
        }

        /// 🚫️async: E1 pure sync body — see `emit` above; same `&self`-across-await mechanism.
        fn status(&self) -> ArtifactSyncStatus {
            ArtifactSyncStatus { persisted: self.last_written_hash.is_some() || self.server_frontier.is_some(), pending_mutations: self.pending_batches.values().map(Vec::len).sum(), remote: self.remote_state.clone() }
        }

        async fn set_remote_state(&mut self, state: RemoteState) {
            self.remote_state = state;
            self.emit_status_if_changed().await;
        }

        async fn emit_status_if_changed(&mut self) {
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
    /// directory path is the canonical multi-document append-only store (`EventLog`).
    async fn build_folder_endpoint(path: &Path, document_id: &str, schema: &str) -> FolderEndpoint {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(extension) => {
                let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
                FolderEndpoint::Pack { storage: FolderTextStorage::new(folder).await, document_id: document_id.to_string(), extension: extension.to_string(), schema: schema.to_string() }
            }
            None => FolderEndpoint::EventLog { storage: FolderEventLogStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() },
        }
    }

    /// @emoji 📍️ The on-disk path a folder binding writes to: the `<document_id>.<extension>` text blob
    /// itself, or the multi-document sqlite db under `<folder>/.semio/documents.db`.
    async fn folder_watch_path_for(path: &Path) -> PathBuf {
        if path.extension().is_some() {
            path.to_path_buf()
        } else {
            path.join(".semio").join("documents.db")
        }
    }

    /// @emoji 👁️ Creates the owned non-recursive watcher; its first probe establishes a
    /// baseline and later snapshots feed the actor's existing 200 ms debounce.
    fn install_watcher(watch_path: &Path, pool: Arc<semio_framework_async::WorkerPool>, readiness: Arc<dyn Fn() + Send + Sync>) -> semio_framework_os_services::OwnedFileChangeWatcher {
        semio_framework_os_services::OwnedFileChangeWatcher::new(watch_path, pool, readiness)
    }

    const ACTOR_RUNNER_RETRY_MS: u64 = 1;
    const ACTOR_RUNNER_RETRY_LIMIT: u8 = 8;

    type ActorTurnFuture = std::pin::Pin<Box<dyn std::future::Future<Output = (ArtifactActor, ArtifactDrive)> + Send>>;

    enum ActorTurnOwner {
        Actor(ArtifactActor),
        Future(ActorTurnFuture),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ArtifactActorTerminalReason {
        Detached,
        Cancelled,
        TurnFault,
        Pool(semio_framework_async::WorkerSubmitErrorKind),
    }

    struct ActorTerminalJob {
        reason: ArtifactActorTerminalReason,
        job: semio_framework_async::Job,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ArtifactActorTerminalState {
        Live,
        Closing(ArtifactActorTerminalReason),
        Complete(ArtifactActorTerminalReason),
    }

    #[derive(Clone)]
    pub struct ArtifactActorRunnerHandle {
        generation: u64,
        runner: Arc<ActorRunner>,
    }

    pub struct ArtifactActorRunnerTicket {
        generation: u64,
        runner: std::sync::Weak<ActorRunner>,
        host: Option<Arc<std::sync::Mutex<ArtifactHostState>>>,
        returned: bool,
    }

    pub struct ArtifactActorTerminalJob {
        handle: ArtifactActorRunnerHandle,
        owner: Option<ActorTerminalJob>,
    }

    struct ActorRunner {
        pool: Arc<semio_framework_async::WorkerPool>,
        generation: u64,
        turn: std::sync::Mutex<Option<ActorTurnOwner>>,
        terminal_turn: std::sync::Mutex<Option<ActorTurnOwner>>,
        mailbox: ArtifactMailboxClose,
        scheduled: std::sync::atomic::AtomicBool,
        wake_requested: std::sync::atomic::AtomicBool,
        turn_generation: std::sync::atomic::AtomicU64,
        deadline_armed: std::sync::atomic::AtomicBool,
        deadline_generation: std::sync::atomic::AtomicU64,
        deadline_at_ms: std::sync::atomic::AtomicU64,
        retry_armed: std::sync::atomic::AtomicBool,
        retry_generation: std::sync::atomic::AtomicU64,
        retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
        terminal_job: std::sync::Mutex<Option<ActorTerminalJob>>,
        terminal_reason: std::sync::Mutex<Option<ArtifactActorTerminalReason>>,
        terminal_empty_callback: std::sync::Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>,
        self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>>,
        external_tickets: std::sync::atomic::AtomicUsize,
        close_requested: std::sync::atomic::AtomicBool,
        terminal: std::sync::atomic::AtomicBool,
        complete: std::sync::atomic::AtomicBool,
    }

    impl ArtifactActorRunnerHandle {
        pub fn generation(&self) -> u64 {
            self.generation
        }

        pub fn cancel(&self) {
            self.runner.cancel();
        }

        pub fn terminal_state(&self) -> ArtifactActorTerminalState {
            let reason = *self.runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match reason {
                None => ArtifactActorTerminalState::Live,
                Some(reason) if self.runner.complete.load(std::sync::atomic::Ordering::Acquire) => ArtifactActorTerminalState::Complete(reason),
                Some(reason) => ArtifactActorTerminalState::Closing(reason),
            }
        }

        pub fn close_step(&self) -> bool {
            if !self.runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
                self.runner.request_close();
                return false;
            }
            let advanced = self.runner.close_one_terminal_owner();
            if !advanced && self.runner.terminal_is_empty() {
                self.runner.finish_terminal();
            }
            advanced
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.runner.terminal_is_empty()
        }

        pub fn take_terminal_job(&self) -> Option<ArtifactActorTerminalJob> {
            self.runner.take_terminal_job().map(|owner| ArtifactActorTerminalJob { handle: self.clone(), owner: Some(owner) })
        }

        pub(super) fn start(&self) {
            self.runner.schedule();
        }

        pub(super) fn request_close(&self) {
            self.runner.request_close();
        }

        pub(super) fn issue_ticket(&self, host: Arc<std::sync::Mutex<ArtifactHostState>>) -> ArtifactActorRunnerTicket {
            self.runner.external_tickets.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| count.checked_add(1)).expect("artifact actor ticket capacity exhausted");
            ArtifactActorRunnerTicket { generation: self.generation, runner: Arc::downgrade(&self.runner), host: Some(host), returned: false }
        }

        pub(super) fn set_terminal_empty_callback(&self, callback: Arc<dyn Fn(u64) + Send + Sync>) {
            *self.runner.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(callback);
            if self.runner.terminal_is_empty() {
                self.runner.finish_terminal();
            }
        }
    }

    impl ArtifactActorRunnerTicket {
        pub fn generation(&self) -> u64 {
            self.generation
        }

        pub fn terminal_state(&self) -> Option<ArtifactActorTerminalState> {
            self.runner.upgrade().map(|runner| ArtifactActorRunnerHandle { generation: self.generation, runner }.terminal_state())
        }

        pub fn return_to_host(mut self) {
            self.return_once();
        }

        fn return_once(&mut self) {
            if self.returned {
                return;
            }
            self.returned = true;
            if let Some(runner) = self.runner.upgrade() {
                if runner.generation == self.generation {
                    runner.return_ticket();
                }
            }
            self.host.take();
        }
    }

    impl Drop for ArtifactActorRunnerTicket {
        fn drop(&mut self) {
            self.return_once();
        }
    }

    impl ArtifactActorTerminalJob {
        pub fn reason(&self) -> ArtifactActorTerminalReason {
            self.owner.as_ref().expect("terminal job owner already resolved").reason
        }

        pub fn resume(mut self) {
            let owner = self.owner.take().expect("terminal job owner already resolved");
            self.handle.runner.resume_terminal_job(owner);
        }

        pub fn close(mut self) {
            let owner = self.owner.take().expect("terminal job owner already resolved");
            self.handle.runner.close_terminal_job(owner);
        }
    }

    impl Drop for ArtifactActorTerminalJob {
        fn drop(&mut self) {
            if let Some(owner) = self.owner.take() {
                self.handle.runner.return_terminal_job(owner);
            }
        }
    }

    struct ActorTurnWake {
        runner: std::sync::Weak<ActorRunner>,
        generation: u64,
    }

    impl std::task::Wake for ActorTurnWake {
        fn wake(self: Arc<Self>) {
            if let Some(runner) = self.runner.upgrade() {
                runner.request_wake(self.generation);
            }
        }

        fn wake_by_ref(self: &Arc<Self>) {
            if let Some(runner) = self.runner.upgrade() {
                runner.request_wake(self.generation);
            }
        }
    }

    impl ActorRunner {
        fn schedule(self: &Arc<Self>) {
            self.request_wake(self.turn_generation.load(std::sync::atomic::Ordering::Acquire));
        }

        fn request_wake(self: &Arc<Self>, generation: u64) {
            if self.complete.load(std::sync::atomic::Ordering::Acquire) || self.terminal.load(std::sync::atomic::Ordering::Acquire) || generation != self.turn_generation.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            let _ = self.wake_requested.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
            self.enqueue(false);
        }

        fn enqueue(self: &Arc<Self>, terminal_close: bool) {
            if self.complete.load(std::sync::atomic::Ordering::Acquire) || (!terminal_close && self.terminal.load(std::sync::atomic::Ordering::Acquire)) {
                return;
            }
            if self.scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                return;
            }
            let runner = self.clone();
            self.submit_exact(Box::new(move || runner.run_job()), 0);
        }

        fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
            match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
                Ok(()) => {}
                Err(error) => match error.kind() {
                    kind @ (semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated) if attempt < ACTOR_RUNNER_RETRY_LIMIT => {
                        *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                        self.arm_retry();
                    }
                    kind => {
                        let job = error.into_job();
                        self.begin_terminal(ArtifactActorTerminalReason::Pool(kind));
                        *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason: ArtifactActorTerminalReason::Pool(kind), job });
                    }
                },
            }
        }

        fn arm_retry(self: &Arc<Self>) {
            if self.retry_armed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                return;
            }
            let generation = self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
            let runner = self.clone();
            self.pool.callback_at(self.pool.now_ms().saturating_add(ACTOR_RUNNER_RETRY_MS), move || {
                if generation != runner.retry_generation.load(std::sync::atomic::Ordering::Acquire) {
                    return;
                }
                runner.retry_armed.store(false, std::sync::atomic::Ordering::Release);
                let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if let Some((job, attempt)) = retry {
                    if runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
                        let reason = runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner).unwrap_or(ArtifactActorTerminalReason::Cancelled);
                        *runner.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
                    } else {
                        runner.submit_exact(job, attempt);
                    }
                }
            });
        }

        fn run_job(self: Arc<Self>) {
            if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                self.close_one_terminal_owner();
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if self.has_terminal_owner() {
                    self.enqueue(true);
                } else {
                    self.finish_terminal();
                }
                return;
            }
            self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
            let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            let Some(owner) = owner else {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                return;
            };
            let mut future = match owner {
                ActorTurnOwner::Actor(mut actor) => {
                    if self.close_requested.load(std::sync::atomic::Ordering::Acquire) {
                        actor.closing = true;
                        actor.cmd_rx.close();
                    }
                    Box::pin(async move {
                        let outcome = actor.drive_one().await;
                        (actor, outcome)
                    }) as ActorTurnFuture
                }
                ActorTurnOwner::Future(future) => future,
            };
            let generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
            let waker = std::task::Waker::from(Arc::new(ActorTurnWake { runner: Arc::downgrade(&self), generation }));
            let mut context = std::task::Context::from_waker(&waker);
            let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
            match polled {
                Ok(std::task::Poll::Pending) => {
                    if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                        *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                        self.enqueue(true);
                        return;
                    }
                    *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    if self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel) {
                        self.enqueue(false);
                    }
                }
                Ok(std::task::Poll::Ready((mut actor, mut outcome))) => {
                    self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
                    self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
                    if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                        *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                        self.enqueue(true);
                        return;
                    }
                    if self.close_requested.load(std::sync::atomic::Ordering::Acquire) && outcome != ArtifactDrive::Terminal {
                        actor.closing = true;
                        actor.cmd_rx.close();
                        outcome = ArtifactDrive::MoreWork;
                    }
                    match outcome {
                        ArtifactDrive::Terminal => {
                            *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                            self.begin_terminal(ArtifactActorTerminalReason::Detached);
                            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                            self.enqueue(true);
                        }
                        ArtifactDrive::MoreWork | ArtifactDrive::Idle { .. } => {
                            *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                            let woke = self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel);
                            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                            if outcome == ArtifactDrive::MoreWork || woke {
                                self.schedule();
                            } else if let ArtifactDrive::Idle { deadline } = outcome {
                                self.arm_deadline(deadline);
                            }
                        }
                    }
                }
                Err(_) => {
                    *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                    self.begin_terminal(ArtifactActorTerminalReason::TurnFault);
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    self.enqueue(true);
                }
            }
        }

        fn arm_deadline(self: &Arc<Self>, deadline: Option<Instant>) {
            let Some(deadline) = deadline else {
                self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
                self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
                return;
            };
            if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            let delay_ms = u64::try_from(deadline.saturating_duration_since(Instant::now()).as_millis()).unwrap_or(u64::MAX).max(1);
            let at_ms = self.pool.now_ms().saturating_add(delay_ms);
            if self.deadline_armed.load(std::sync::atomic::Ordering::Acquire) && self.deadline_at_ms.load(std::sync::atomic::Ordering::Acquire) <= at_ms {
                return;
            }
            self.deadline_at_ms.store(at_ms, std::sync::atomic::Ordering::Release);
            self.deadline_armed.store(true, std::sync::atomic::Ordering::Release);
            let generation = self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
            let turn_generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
            let runner = self.clone();
            self.pool.callback_at(at_ms, move || {
                if generation != runner.deadline_generation.load(std::sync::atomic::Ordering::Acquire) {
                    return;
                }
                runner.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
                runner.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
                runner.request_wake(turn_generation);
            });
        }

        fn begin_terminal(&self, reason: ArtifactActorTerminalReason) {
            if self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
                return;
            }
            self.mailbox.close();
            self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
            self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            *self.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(reason);
            if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
            }
            if self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = owner;
            }
        }

        fn cancel(self: &Arc<Self>) {
            self.begin_terminal(ArtifactActorTerminalReason::Cancelled);
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            self.enqueue(true);
        }

        fn request_close(self: &Arc<Self>) {
            if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            self.close_requested.store(true, std::sync::atomic::Ordering::Release);
            self.mailbox.close();
            self.schedule();
        }

        fn close_one_terminal_owner(&self) -> bool {
            if let Some(owner) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                drop(owner);
                return true;
            }
            if self.mailbox.close_one() {
                return true;
            }
            let owner = self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if owner.is_none() {
                return false;
            }
            drop(owner);
            true
        }

        fn has_terminal_owner(&self) -> bool {
            self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
                || self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
                || self.mailbox.has_pending()
                || self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
        }

        fn terminal_is_empty(&self) -> bool {
            self.terminal.load(std::sync::atomic::Ordering::Acquire)
                && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
                && self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0
                && !self.has_terminal_owner()
                && self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
        }

        fn finish_terminal(&self) {
            if !self.terminal_is_empty() {
                return;
            }
            if self.complete.swap(true, std::sync::atomic::Ordering::AcqRel) {
                return;
            }
            if let Some(callback) = self.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                callback(self.generation);
            }
            self.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        }

        fn return_ticket(&self) {
            let previous = self.external_tickets.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            debug_assert!(previous > 0, "artifact actor ticket returned twice");
            if previous == 1 && self.terminal_is_empty() {
                self.finish_terminal();
            }
        }

        fn take_terminal_job(&self) -> Option<ActorTerminalJob> {
            self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
        }

        fn resume_terminal_job(self: &Arc<Self>, terminal: ActorTerminalJob) {
            self.scheduled.store(true, std::sync::atomic::Ordering::Release);
            self.submit_exact(terminal.job, 0);
        }

        fn return_terminal_job(&self, terminal: ActorTerminalJob) {
            let mut slot = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            debug_assert!(slot.is_none(), "terminal job handback slot already occupied");
            if slot.is_none() {
                *slot = Some(terminal);
            }
        }

        fn close_terminal_job(&self, terminal: ActorTerminalJob) {
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            drop(terminal);
            if self.terminal_is_empty() {
                self.finish_terminal();
            }
        }
    }

    /// @emoji 🚀️ Creates one finite-turn actor on the process WorkerPool. Tokio remains only the
    /// platform I/O reactor; it never owns an actor or timer thread.
    pub(super) async fn spawn_actor(
        pool: Arc<semio_framework_async::WorkerPool>,
        generation: u64,
        config: ArtifactActorConfig,
        remote: ChannelBackboneRemote,
        cmd_rx: ArtifactMailboxReceiver,
        events: broadcast::Sender<ArtifactEvent>,
        credential: Arc<std::sync::RwLock<Option<Arc<crate::os_directory::client::LocalHubCredential>>>>,
        socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn crate::os_directory::client::HubSocketGrantSource>>>>,
        document_socket_surface: Option<crate::os_directory::client::DocumentSocketSurfaceExpectationV1>,
        operation_cancel: semio_framework_async::CancelToken,
    ) -> ArtifactActorRunnerHandle {
        let mailbox = cmd_rx.close_handle();
        let actor = ArtifactActor::new(pool.clone(), config, remote, cmd_rx, events, credential, socket_grant_source, document_socket_surface, operation_cancel).await;
        let runner = Arc::new(ActorRunner {
            pool,
            generation,
            turn: std::sync::Mutex::new(Some(ActorTurnOwner::Actor(actor))),
            terminal_turn: std::sync::Mutex::new(None),
            mailbox,
            scheduled: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            turn_generation: std::sync::atomic::AtomicU64::new(1),
            deadline_armed: std::sync::atomic::AtomicBool::new(false),
            deadline_generation: std::sync::atomic::AtomicU64::new(1),
            deadline_at_ms: std::sync::atomic::AtomicU64::new(u64::MAX),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            terminal_reason: std::sync::Mutex::new(None),
            terminal_empty_callback: std::sync::Mutex::new(None),
            self_retained: std::sync::Mutex::new(None),
            external_tickets: std::sync::atomic::AtomicUsize::new(0),
            close_requested: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
            complete: std::sync::atomic::AtomicBool::new(false),
        });
        *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
        let weak = Arc::downgrade(&runner);
        let schedule: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            if let Some(runner) = weak.upgrade() {
                runner.schedule();
            }
        });
        if let Some(ActorTurnOwner::Actor(actor)) = runner.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut() {
            actor.cmd_rx.set_wake(schedule.clone());
            actor.set_readiness(schedule);
        }
        ArtifactActorRunnerHandle { generation, runner }
    }

    #[cfg(test)]
    pub(super) mod retained_turn_fixtures {
        use super::*;

        fn runner_with(pool: Arc<semio_framework_async::WorkerPool>, owner: Option<ActorTurnOwner>, mailbox: ArtifactMailboxClose) -> Arc<ActorRunner> {
            Arc::new(ActorRunner {
                pool,
                generation: 1,
                turn: std::sync::Mutex::new(owner),
                terminal_turn: std::sync::Mutex::new(None),
                mailbox,
                scheduled: std::sync::atomic::AtomicBool::new(false),
                wake_requested: std::sync::atomic::AtomicBool::new(false),
                turn_generation: std::sync::atomic::AtomicU64::new(1),
                deadline_armed: std::sync::atomic::AtomicBool::new(false),
                deadline_generation: std::sync::atomic::AtomicU64::new(1),
                deadline_at_ms: std::sync::atomic::AtomicU64::new(u64::MAX),
                retry_armed: std::sync::atomic::AtomicBool::new(false),
                retry_generation: std::sync::atomic::AtomicU64::new(1),
                retry_job: std::sync::Mutex::new(None),
                terminal_job: std::sync::Mutex::new(None),
                terminal_reason: std::sync::Mutex::new(None),
                terminal_empty_callback: std::sync::Mutex::new(None),
                self_retained: std::sync::Mutex::new(None),
                external_tickets: std::sync::atomic::AtomicUsize::new(0),
                close_requested: std::sync::atomic::AtomicBool::new(false),
                terminal: std::sync::atomic::AtomicBool::new(false),
                complete: std::sync::atomic::AtomicBool::new(false),
            })
        }

        pub(in super::super) fn fixture_runner_handle(pool: Arc<semio_framework_async::WorkerPool>, generation: u64, mailbox: ArtifactMailboxClose) -> ArtifactActorRunnerHandle {
            let runner = runner_with(pool, None, mailbox);
            let runner = Arc::new(ActorRunner { generation, ..Arc::try_unwrap(runner).ok().expect("fixture runner has one owner") });
            *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
            ArtifactActorRunnerHandle { generation, runner }
        }

        #[test]
        fn stale_generation_wake_cannot_schedule_or_mutate_current_turn() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, receiver) = artifact_mailbox_pair();
            let runner = runner_with(pool, None, receiver.close_handle());
            runner.request_wake(0);
            assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
            assert!(!runner.terminal.load(std::sync::atomic::Ordering::Acquire));
            runner.request_wake(1);
            assert_eq!(*runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
            assert!(runner.take_terminal_job().is_some(), "shutdown returns the exact rejected closure to observable terminal ownership");
        }

        #[test]
        fn turn_fault_and_cancel_retain_then_close_one_owner_per_grant() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (sender, receiver) = artifact_mailbox_pair();
            sender.send(ArtifactActorMsg::ExternalChanged).expect("first terminal mailbox owner");
            sender.send(ArtifactActorMsg::Detach).expect("second terminal mailbox owner");
            let fault: ActorTurnFuture = Box::pin(async { panic!("fixture turn fault") });
            let runner = runner_with(pool, Some(ActorTurnOwner::Future(fault)), receiver.close_handle());
            runner.scheduled.store(true, std::sync::atomic::Ordering::Release);
            runner.clone().run_job();
            assert_eq!(*runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::TurnFault));
            assert!(runner.close_one_terminal_owner());
            assert_eq!(runner.mailbox.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);
            assert!(runner.close_one_terminal_owner());
            assert_eq!(runner.mailbox.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 0);
            assert!(runner.close_one_terminal_owner(), "retained fault future is a distinct terminal owner");

            let (_, receiver) = artifact_mailbox_pair();
            let cancelled: ActorTurnFuture = Box::pin(async { std::future::pending::<(ArtifactActor, ArtifactDrive)>().await });
            let cancelled_runner = runner_with(runner.pool.clone(), Some(ActorTurnOwner::Future(cancelled)), receiver.close_handle());
            cancelled_runner.cancel();
            assert_eq!(*cancelled_runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Cancelled));
        }

        #[test]
        fn quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            let started = Arc::new(std::sync::Barrier::new(2));
            let release = Arc::new(std::sync::Barrier::new(2));
            let worker_started = started.clone();
            let worker_release = release.clone();
            pool.submit(
                semio_framework_async::Lane::UserVisible,
                Box::new(move || {
                    worker_started.wait();
                    worker_release.wait();
                }),
            );
            started.wait();
            let mut rejected = None;
            for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
                if let Err(error) = pool.try_submit(semio_framework_async::Lane::UserVisible, Box::new(|| {})) {
                    rejected = Some(error);
                    break;
                }
            }
            if let Some(error) = rejected {
                let kind = error.kind();
                let job = error.into_job();
                release.wait();
                pool.shutdown();
                job();
                panic!("fill exact quiet queue slot: {kind:?}");
            }
            let (_, receiver) = artifact_mailbox_pair();
            let runner = runner_with(pool, None, receiver.close_handle());
            runner.enqueue(false);
            assert!(runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            assert!(runner.retry_armed.load(std::sync::atomic::Ordering::Acquire));
            release.wait();
        }

        #[test]
        fn idle_runner_is_strongly_retained_and_quiet_late_wake_schedules_once() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, receiver) = artifact_mailbox_pair();
            let handle = fixture_runner_handle(pool, 41, receiver.close_handle());
            let weak = Arc::downgrade(&handle.runner);
            let runner = handle.runner.clone();
            drop(handle);
            drop(runner);
            let retained = weak.upgrade().expect("idle runner must retain itself until terminal close");
            for _ in 0..8 {
                retained.request_wake(retained.turn_generation.load(std::sync::atomic::Ordering::Acquire));
            }
            assert!(matches!(*retained.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown))));
            assert!(retained.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "wake storm retains one exact rejected job");
            let cleanup = ArtifactActorRunnerHandle { generation: 41, runner: retained };
            while cleanup.close_step() {}
            assert!(cleanup.terminal_is_empty());
        }

        #[test]
        fn external_ticket_held_across_close_delays_completion_until_exact_return() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, receiver) = artifact_mailbox_pair();
            let handle = fixture_runner_handle(pool, 73, receiver.close_handle());
            let host = Arc::new(std::sync::Mutex::new(ArtifactHostState::new()));
            let ticket = handle.issue_ticket(host);
            handle.cancel();
            while handle.close_step() {}
            assert!(!handle.terminal_is_empty(), "external ticket is a retained close authority");
            assert!(!handle.runner.complete.load(std::sync::atomic::Ordering::Acquire));
            ticket.return_to_host();
            assert!(handle.terminal_is_empty());
            assert!(handle.runner.complete.load(std::sync::atomic::Ordering::Acquire));
        }

        #[test]
        fn external_ticket_dropped_before_close_and_generation_aba_are_exact() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, old_receiver) = artifact_mailbox_pair();
            let old = fixture_runner_handle(pool.clone(), 91, old_receiver.close_handle());
            let host = Arc::new(std::sync::Mutex::new(ArtifactHostState::new()));
            let old_ticket = old.issue_ticket(host.clone());
            assert_eq!(old_ticket.generation(), 91);
            drop(old_ticket);
            old.cancel();
            while old.close_step() {}
            assert!(old.terminal_is_empty(), "returned-before-close ticket cannot delay retirement");

            let (_, current_receiver) = artifact_mailbox_pair();
            let current = fixture_runner_handle(pool, 92, current_receiver.close_handle());
            let current_ticket = current.issue_ticket(host);
            assert_eq!(current.runner.external_tickets.load(std::sync::atomic::Ordering::Acquire), 1);
            assert_eq!(old.runner.external_tickets.load(std::sync::atomic::Ordering::Acquire), 0, "old ticket return cannot decrement the reused generation");
            drop(current_ticket);
            current.cancel();
            while current.close_step() {}
            assert!(current.terminal_is_empty());
        }

        #[test]
        fn terminal_job_take_resume_and_close_preserve_exact_owner() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, receiver) = artifact_mailbox_pair();
            let handle = fixture_runner_handle(pool, 101, receiver.close_handle());
            handle.cancel();
            let job = handle.take_terminal_job().expect("host retrieves exact rejected terminal job");
            assert!(matches!(job.reason(), ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
            job.resume();
            let resumed = handle.take_terminal_job().expect("failed resume hands the same job back to terminal ownership");
            resumed.close();
            while handle.close_step() {}
            assert!(handle.terminal_is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn idle_then_late_send_upgrades_the_host_retained_runner_once() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            let host = ArtifactHost::new(pool);
            let channels = host.open(ArtifactActorConfig { document_id: "quiet".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
            let runner = channels.runner.runner.upgrade().expect("host retains the quiet runner");
            for _ in 0..10_000 {
                if !runner.scheduled.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
                std::thread::yield_now();
            }
            assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire), "runner reaches wake-driven idle without polling");
            channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("late owner admitted");
            for _ in 0..10_000 {
                if !runner.mailbox.has_pending() {
                    break;
                }
                std::thread::yield_now();
            }
            assert!(!runner.mailbox.has_pending(), "late send wakes and consumes exactly its mailbox owner");
            let generation = host.close("quiet").expect("host transfers the runner into closing ownership");
            channels.runner.return_to_host();
            assert_eq!(generation, runner.generation);
        }

        #[semio_framework_async_macros::async_test]
        async fn hub_document_actor_and_surface_authority_are_isolated_by_full_scope() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let host = ArtifactHost::new(pool);
            let space_a = ArtifactDocumentKey::hub("space-a", "shared-document");
            let space_b = ArtifactDocumentKey::hub("space-b", "shared-document");
            let surface = crate::os_directory::client::DocumentSocketSurfaceExpectationV1 {
                artifact_kind: "fixture".into(),
                plugin_id: "fixture.plugin".into(),
                package_id: "fixture.package".into(),
                version: "1.0.0".into(),
                surface_id: "fixture.editor".into(),
                app_id: "fixture.app".into(),
                window_kind_id: "fixture.window".into(),
                role: crate::os_directory::DocumentOpenSurfaceRoleV1::Editor,
                renderer_target: crate::os_directory::DocumentOpenRendererTargetV1::Wgpu,
            };
            assert!(host.set_document_socket_surface(&space_a, surface.clone()));
            assert!(host.set_document_socket_surface(&space_b, surface));
            assert!(!host.set_document_socket_surface(&ArtifactDocumentKey::local("shared-document"), crate::os_directory::client::DocumentSocketSurfaceExpectationV1 {
                artifact_kind: "fixture".into(), plugin_id: "fixture.plugin".into(), package_id: "fixture.package".into(), version: "1.0.0".into(), surface_id: "fixture.editor".into(), app_id: "fixture.app".into(), window_kind_id: "fixture.window".into(), role: crate::os_directory::DocumentOpenSurfaceRoleV1::Editor, renderer_target: crate::os_directory::DocumentOpenRendererTargetV1::Wgpu,
            }));
            let channels_a = host
                .open(ArtifactActorConfig {
                    document_id: "shared-document".into(),
                    schema: "fixture/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: "http://127.0.0.1:1".into(), space_id: "space-a".into(), surface: Some("fixture.editor".into()) }],
                    watch_external: false,
                    actor: "fixture-a".into(),
                })
                .await;
            let channels_b = host
                .open(ArtifactActorConfig {
                    document_id: "shared-document".into(),
                    schema: "fixture/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: "http://127.0.0.1:1".into(), space_id: "space-b".into(), surface: Some("fixture.editor".into()) }],
                    watch_external: false,
                    actor: "fixture-b".into(),
                })
                .await;
            assert_eq!(channels_a.document_key, space_a);
            assert_eq!(channels_b.document_key, space_b);
            {
                let state = host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert_eq!(state.documents.len(), 2);
                assert!(state.documents.contains_key(&space_a));
                assert!(state.documents.contains_key(&space_b));
                assert!(state.document_socket_surfaces.is_empty());
            }
            assert!(host.close_key(&space_a).is_some());
            assert!(!host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_a));
            assert!(host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_b));
            assert!(host.close_key(&space_b).is_some());
        }

        #[semio_framework_async_macros::async_test]
        async fn host_close_registry_survives_external_ticket_until_return() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let host = ArtifactHost::new(pool);
            let channels = host.open(ArtifactActorConfig { document_id: "held".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
            let generation = channels.runner.generation();
            assert_eq!(host.close("held"), Some(generation));
            let control = host.closing_runner(generation).expect("closing registry owns the runner before cancellation progression");
            while control.close_step() {}
            assert!(!control.terminal_is_empty(), "held external ticket delays terminal-empty callback");
            assert!(host.closing_runner(generation).is_some());
            channels.runner.return_to_host();
            assert!(control.terminal_is_empty());
            assert!(host.closing_runner(generation).is_none(), "ticket return clears exactly the matching generation");
        }

        #[semio_framework_async_macros::async_test]
        async fn ticket_return_before_host_close_allows_immediate_retirement() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let host = ArtifactHost::new(pool);
            let channels = host.open(ArtifactActorConfig { document_id: "returned".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
            let generation = channels.runner.generation();
            channels.runner.return_to_host();
            assert_eq!(host.close("returned"), Some(generation));
            let control = host.closing_runner(generation).expect("host retains terminal control");
            while control.close_step() {}
            assert!(control.terminal_is_empty());
            assert!(host.closing_runner(generation).is_none());
        }

        #[test]
        fn detach_while_pending_retains_future_then_cancel_closes_one_owner() {
            let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
            pool.shutdown();
            let (_, receiver) = artifact_mailbox_pair();
            let pending: ActorTurnFuture = Box::pin(async { std::future::pending::<(ArtifactActor, ArtifactDrive)>().await });
            let runner = runner_with(pool, Some(ActorTurnOwner::Future(pending)), receiver.close_handle());
            *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
            runner.request_close();
            assert!(runner.close_requested.load(std::sync::atomic::Ordering::Acquire));
            assert!(runner.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "fatal close scheduling retains the pending future");
            let handle = ArtifactActorRunnerHandle { generation: 1, runner };
            while handle.close_step() {}
            assert!(handle.terminal_is_empty());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
use native_actor::spawn_actor;
#[cfg(not(target_arch = "wasm32"))]
pub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState};
//#endregion 🔖️NativeActor

//#region 🔖️WasmActor
/// @emoji 🌐️ Browser wgpu build: the actor runs on `spawn_local` with a `web_sys::WebSocket` semio_hub
/// transport. No filesystem, so folder bindings are ignored (the browser uses the dev-middleware
/// SSE watch instead, wired by WS-E's TS twin). Kept coherent so a future in-wasm host can link it.
/// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this is a browser-only WebSocket
/// bridge, so it is narrowed to exclude the WASI component target — no plugin currently activates
/// the `sync`/`worker` features that reach this module at all.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_actor {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, MessageEvent, WebSocket};

    enum WasmIncoming {
        Binary(Vec<u8>),
        Closed,
    }

    struct PendingWasmArtifactBootstrap {
        assembler: ArtifactBootstrapAssembler,
        started_ms: u64,
        resume_token: String,
        baseline_frontier: RuntimeFrontierSummary,
        required_tail_frontier: RuntimeFrontierSummary,
        pack_schema_hash: [u8; 32],
    }

    struct WasmBootstrapControl {
        cancelled: bool,
        now_ms: u64,
        events: broadcast::Sender<ArtifactEvent>,
    }

    impl ArtifactBootstrapControl for WasmBootstrapControl {
        fn is_cancelled(&mut self) -> bool {
            self.cancelled
        }

        fn now_ms(&mut self) -> u64 {
            self.now_ms
        }

        fn on_progress(&mut self, progress: ArtifactBootstrapProgress) {
            let _ = self.events.send(ArtifactEvent::BootstrapProgress {
                received_bytes: progress.received_bytes,
                total_bytes: progress.total_bytes,
                received_chunks: progress.received_chunks,
                total_chunks: progress.total_chunks,
            });
        }
    }

    struct WasmActor {
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<ArtifactEvent>,
        hub_base_url: Option<String>,
        hub_space_id: Option<String>,
        hub_surface: Option<String>,
        /// @emoji 🎨️ See the native actor's matching field — same role, wasm side.
        session_color: Option<u8>,
        ws: Option<WebSocket>,
        server_frontier: Option<crate::os_spr::RuntimeFrontierSummary>,
        resume_token: Option<String>,
        pending_resume_token: Option<String>,
        required_tail_frontier: Option<RuntimeFrontierSummary>,
        artifact_bootstrap: Option<PendingWasmArtifactBootstrap>,
        pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
        outbox: Vec<MutationEnvelope>,
        next_batch_id: u64,
        hlc_seed: u64,
        hlc_counter: u64,
        incoming_tx: mpsc::UnboundedSender<WasmIncoming>,
        _closures: Vec<Closure<dyn FnMut(MessageEvent)>>,
        _open_closures: Vec<Closure<dyn FnMut()>>,
        _close_closures: Vec<Closure<dyn FnMut()>>,
    }

    impl WasmActor {
        async fn connect(&mut self) {
            let _ = (&self.hub_base_url, &self.hub_space_id, &self.hub_surface);
        }

        async fn send_frame(&self, frame: &ClientFrame, lane: Lane) {
            if let Some(ws) = &self.ws {
                let mut bytes = encode_client_frame(frame, lane).await;
                let _ = ws.send_with_u8_array(&mut bytes);
            }
        }

        /// @emoji 🧺️ Builds + sends one `Commands` batch, tracking it in `pending_batches` for
        /// {@link WasmActor::handle_ack}. Mirrors the native actor's `relay_operations_to_hub`.
        async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            if self.ws.as_ref().is_none_or(|socket| socket.ready_state() != WebSocket::OPEN) || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                self.queue_outbox(envelopes.iter().cloned());
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let mut wire_envelopes: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
            for envelope in envelopes {
                let timestamp = next_timestamp(self.hlc_seed, &mut self.hlc_counter).await;
                wire_envelopes.push(crate::os_spr::MutationEnvelope { timestamp, ..envelope.clone() });
            }
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_frame(&ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
        }

        async fn relay_one_backbone(&mut self) -> Result<bool, vcs::VcsError> {
            let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
            match message {
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                    self.relay_operations(&envelopes).await;
                }
                BackboneMessage::Snapshot { .. } | BackboneMessage::Ack { .. } => {}
            }
            Ok(true)
        }

        async fn handle_cmd(&mut self, message: ArtifactActorMsg) {
            match message {
                ArtifactActorMsg::LocalMutations { envelopes } => {
                    let drained = self.relay_one_backbone().await.unwrap_or(false);
                    if !drained && !envelopes.is_empty() {
                        self.relay_operations(&envelopes).await;
                    }
                }
                ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                    stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                    self.send_frame(&ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
                }
                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                }
                ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
            }
        }

        fn bootstrap_control(&self, _started_ms: u64, cancelled: bool) -> WasmBootstrapControl {
            WasmBootstrapControl { cancelled, now_ms: js_sys::Date::now() as u64, events: self.events.clone() }
        }

        fn abort_artifact_bootstrap(&mut self) {
            if let Some(mut pending) = self.artifact_bootstrap.take() {
                pending.assembler.abort();
            }
            self.pending_resume_token = None;
            self.required_tail_frontier = None;
        }

        fn requeue_pending_batches(&mut self) {
            let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
            batches.sort_by_key(|(batch_id, _)| *batch_id);
            for (_, envelopes) in batches {
                self.queue_outbox(envelopes);
            }
        }

        fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
            let mut queued: std::collections::HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
            for envelope in envelopes {
                if queued.insert(envelope.mutation_id.0.clone()) {
                    self.outbox.push(envelope);
                }
            }
        }

        async fn flush_outbox(&mut self) {
            if self.outbox.is_empty() {
                return;
            }
            let envelopes = std::mem::take(&mut self.outbox);
            self.relay_operations(&envelopes).await;
        }

        async fn finish_catchup_if_ready(&mut self) {
            let Some(required) = self.required_tail_frontier.clone() else { return };
            let Some(actual) = self.server_frontier.as_ref() else { return };
            if !frontier_reaches(actual, &required) {
                return;
            }
            self.required_tail_frontier = None;
            if let Some(resume_token) = self.pending_resume_token.take() {
                self.resume_token = Some(resume_token);
            }
            self.flush_outbox().await;
        }

        fn disconnect(&mut self) {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            if let Some(socket) = self.ws.take() {
                let _ = socket.close();
            }
        }

        async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
            self.abort_artifact_bootstrap();
            let codec = match crate::os_store::document_codec(&self.schema).await {
                Ok(Some(codec)) => codec,
                _ => {
                    self.disconnect();
                    return;
                }
            };
            if validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, codec.pack_schema_hash, &server_frontier).is_err() {
                self.disconnect();
                return;
            }
            let inline = bootstrap.inline.is_some();
            let started_ms = js_sys::Date::now() as u64;
            let baseline_frontier = bootstrap.baseline_frontier.clone();
            let required_tail_frontier = bootstrap.required_tail_frontier.clone();
            let pack_schema_hash = bootstrap.pack_schema_hash;
            let mut control = self.bootstrap_control(started_ms, false);
            let assembler = match ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, ArtifactBootstrapLimits::default(), Some(started_ms.saturating_add(ARTIFACT_BOOTSTRAP_DEADLINE_MS)), &mut control) {
                Ok(assembler) => assembler,
                Err(_) => {
                    self.disconnect();
                    return;
                }
            };
            let mut pending = PendingWasmArtifactBootstrap { assembler, started_ms, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
            if !inline {
                self.artifact_bootstrap = Some(pending);
                return;
            }
            let mut control = self.bootstrap_control(started_ms, false);
            match pending.assembler.finish(None, &mut control) {
                Ok(pair) => {
                    if self.install_artifact_bootstrap(pending, pair).await.is_err() {
                        self.disconnect();
                    }
                }
                Err(_) => self.disconnect(),
            }
        }

        async fn install_artifact_bootstrap(&mut self, pending: PendingWasmArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
            let codec = crate::os_store::document_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {:?}", self.schema))?;
            if codec.pack_schema_hash != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| error.to_string())?;
            self.remote.push(BackboneMessage::Snapshot { pack: pair.pack.clone(), spr: pair.spr.clone() }).await.map_err(|error| error.to_string())?;
            let _ = self.events.send(ArtifactEvent::SnapshotReplaced { pack: pair.pack, spr: pair.spr });
            self.server_frontier = Some(pending.baseline_frontier);
            self.pending_resume_token = Some(pending.resume_token);
            self.required_tail_frontier = Some(pending.required_tail_frontier);
            if !self.outbox.is_empty() && self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&self.outbox) }).await.is_err() {
                self.disconnect();
                return Ok(());
            }
            self.finish_catchup_if_ready().await;
            Ok(())
        }

        async fn on_binary(&mut self, bytes: &[u8]) {
            let Ok((_lane, frame)) = decode_server_frame(bytes).await else {
                self.disconnect();
                return;
            };
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.requeue_pending_batches();
                    match bootstrap {
                        Bootstrap::None => {
                            self.abort_artifact_bootstrap();
                            self.resume_token = Some(resume_token);
                            self.server_frontier = Some(server_frontier);
                            self.flush_outbox().await;
                        }
                        Bootstrap::Tail => {
                            self.abort_artifact_bootstrap();
                            self.pending_resume_token = Some(resume_token);
                            self.required_tail_frontier = Some(server_frontier);
                            self.finish_catchup_if_ready().await;
                        }
                        Bootstrap::Snapshot { .. } => self.disconnect(),
                        Bootstrap::ArtifactBootstrap(bootstrap) => self.start_artifact_bootstrap(bootstrap, resume_token, server_frontier).await,
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => self.disconnect(),
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.disconnect();
                        return;
                    }
                    self.disconnect();
                }
                ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                    let Some(mut pending) = self.artifact_bootstrap.take() else {
                        self.disconnect();
                        return;
                    };
                    let mut control = self.bootstrap_control(pending.started_ms, false);
                    if pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control).is_ok() {
                        self.artifact_bootstrap = Some(pending);
                    } else {
                        self.disconnect();
                    }
                }
                ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                    let Some(mut pending) = self.artifact_bootstrap.take() else {
                        self.disconnect();
                        return;
                    };
                    let mut control = self.bootstrap_control(pending.started_ms, false);
                    match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                        Ok(pair) => {
                            if self.install_artifact_bootstrap(pending, pair).await.is_err() {
                                self.disconnect();
                            }
                        }
                        Err(_) => self.disconnect(),
                    }
                }
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    if self.artifact_bootstrap.is_some() {
                        self.disconnect();
                        return;
                    }
                    if origin != ActorId(self.actor.clone()) {
                        let converted = envelopes;
                        if !self.deliver_remote_operations(converted).await {
                            self.disconnect();
                            return;
                        }
                    }
                    self.server_frontier = Some(frontier);
                    self.finish_catchup_if_ready().await;
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                        self.disconnect();
                        return;
                    }
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages).await;
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if actor != ActorId(self.actor.clone()) {
                        let _ = self.events.send(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let mut decoded: Vec<PresencePeer> = Vec::new();
                    for p in &peers {
                        if let Some(peer) = presence_from_bytes(p).await {
                            decoded.push(peer);
                        }
                    }
                    let peers = decoded;
                    let _ = self.events.send(ArtifactEvent::Presence { peers });
                }
                ServerFrame::Session { actor, color } => {
                    self.session_color = Some(color);
                    let _ = self.events.send(ArtifactEvent::Session { actor, color });
                }
                ServerFrame::CreditGrant { .. } => {}
                ServerFrame::Error { code, message } => {
                    let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage {
                        level: crate::os_dsl::Severity::Error,
                        code: crate::os_dsl::FaultCode::new(code),
                        message,
                        target: vec![self.hub_base_url.clone().unwrap_or_default()],
                        op_index: None,
                    }));
                }
            }
        }

        /// @emoji 📮️ Mirrors the native actor's `handle_ack` — see its doc comment.
        async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                        for envelope in sent.iter().rev() {
                            rollbacks.push(rollback_envelope(envelope).await);
                        }
                        let _ = self.deliver_remote_operations(rollbacks).await;
                        let converted = *envelope;
                        let _ = self.deliver_remote_operations(vec![converted]).await;
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason, messages } => {
                        let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                        for envelope in sent.iter().rev() {
                            rollbacks.push(rollback_envelope(envelope).await);
                        }
                        let _ = self.deliver_remote_operations(rollbacks).await;
                        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason, messages } });
                    }
                }
            }
        }

        async fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            if self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).await.is_err() {
                return false;
            }
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
            true
        }
    }

    pub(super) async fn spawn_actor(_pool: std::sync::Arc<semio_framework_async::WorkerPool>, config: ArtifactActorConfig, remote: ChannelBackboneRemote, cmd_rx: ArtifactMailboxReceiver, events: broadcast::Sender<ArtifactEvent>) {
        let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<WasmIncoming>();
        let mut hub_base_url = None;
        let mut hub_space_id = None;
        let mut hub_surface = None;
        for binding in &config.bindings {
            if let PersistenceBinding::Hub { base_url, space_id, surface } = binding {
                if hub_base_url.is_none() {
                    hub_base_url = Some(base_url.clone());
                    hub_space_id = Some(space_id.clone());
                    hub_surface = surface.clone();
                }
            }
        }
        let hlc_seed = actor_seed(&config.actor).await;
        let mut actor = WasmActor {
            document_id: config.document_id,
            schema: config.schema,
            actor: config.actor,
            remote,
            events,
            hub_base_url,
            hub_space_id,
            hub_surface,
            session_color: None,
            ws: None,
            server_frontier: None,
            resume_token: None,
            pending_resume_token: None,
            required_tail_frontier: None,
            artifact_bootstrap: None,
            pending_batches: std::collections::HashMap::new(),
            outbox: Vec::new(),
            next_batch_id: 0,
            hlc_seed,
            hlc_counter: 0,
            incoming_tx,
            _closures: Vec::new(),
            _open_closures: Vec::new(),
            _close_closures: Vec::new(),
        };
        semio_framework_async::browser::spawn_local(async move {
            actor.connect().await;
            loop {
                tokio::select! {
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            None => break,
                            Some(ArtifactActorMsg::Detach) => { let _ = actor.relay_one_backbone().await; break; }
                            Some(message) => actor.handle_cmd(message).await,
                        }
                    }
                    incoming = incoming_rx.recv() => {
                        match incoming {
                            Some(WasmIncoming::Binary(bytes)) => actor.on_binary(&bytes).await,
                            Some(WasmIncoming::Closed) => {
                                actor.abort_artifact_bootstrap();
                                actor.requeue_pending_batches();
                                actor.ws = None;
                                actor.connect().await;
                            }
                            None => break,
                        }
                    }
                }
            }
        });
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
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
    /// array. Driven by `🧵️backbone-worker.ts`'s TS fallback vitest harness (which decodes these
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
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum RawFixtureInbound {
    HubFrame { frame_bytes: Vec<u8> },
    ExternalEdits { ops_file: String },
    ReplaceDocument { dsl_file: String, ops_file: String },
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
struct FixtureManifest {
    name: String,
    schema: String,
    document_id: String,
    inbound: Vec<RawFixtureInbound>,
    expected_events: Vec<String>,
    expected_edit_ids: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn parse_fixture_dsl_manifest(text: &str) -> Option<FixtureManifest> {
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
//#endregion 🔖️Fixtures

//#region 🔖️FolderStorage
/// @emoji 📜️ Owned append-only folder event log. Documents are written as indivisible
/// `(schema, pack, spr)` snapshot events; blobs are content-addressed put/delete events. Reads fold
/// the log deterministically, so persistence follows the repo's event-sourced model without a CRUD
/// database. Every record is length-delimited and checksummed; an incomplete final record from a
/// process interruption is ignored, while corruption in a complete record is reported.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct FolderEventLogStorage {
    folder: std::path::PathBuf,
    writer: std::sync::Arc<std::sync::Mutex<()>>,
}

#[cfg(not(target_arch = "wasm32"))]
const FOLDER_EVENT_MAGIC: &[u8; 8] = b"SEMIOEL1";

#[cfg(not(target_arch = "wasm32"))]
const MAX_FOLDER_EVENT_BYTES: u64 = 16 * 1024 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
const DOCUMENT_PUT_EVENT: u8 = 1;

#[cfg(not(target_arch = "wasm32"))]
const BLOB_PUT_EVENT: u8 = 2;

#[cfg(not(target_arch = "wasm32"))]
const BLOB_DELETE_EVENT: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
struct FolderEvent {
    kind: u8,
    updated_at_ms: u64,
    key: String,
    metadata: String,
    primary: Vec<u8>,
    secondary: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
struct FolderEventReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> FolderEventReader<'a> {
    fn take(&mut self, len: usize) -> Result<&'a [u8], vcs::VcsError> {
        let end = self.cursor.checked_add(len).ok_or_else(|| vcs::VcsError::Backbone("folder event length overflow".into()))?;
        let value = self.bytes.get(self.cursor..end).ok_or_else(|| vcs::VcsError::Backbone("truncated folder event".into()))?;
        self.cursor = end;
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8, vcs::VcsError> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, vcs::VcsError> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    fn u64(&mut self) -> Result<u64, vcs::VcsError> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn text(&mut self) -> Result<String, vcs::VcsError> {
        let len = self.u32()? as usize;
        String::from_utf8(self.take(len)?.to_vec()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))
    }

    fn data(&mut self) -> Result<Vec<u8>, vcs::VcsError> {
        let len = usize::try_from(self.u64()?).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        Ok(self.take(len)?.to_vec())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderEventLogStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        static WRITERS: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, std::sync::Weak<std::sync::Mutex<()>>>>> = std::sync::OnceLock::new();
        let canonical_key = std::fs::canonicalize(&folder).unwrap_or_else(|_| folder.clone());
        let mut writers = WRITERS.get_or_init(Default::default).lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let writer = writers.get(&canonical_key).and_then(std::sync::Weak::upgrade).unwrap_or_else(|| {
            let writer = std::sync::Arc::new(std::sync::Mutex::new(()));
            writers.insert(canonical_key, std::sync::Arc::downgrade(&writer));
            writer
        });
        Self { folder, writer }
    }

    fn event_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("events.semio")
    }

    fn checksum(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
    }

    fn encode_event(event: &FolderEvent) -> Result<Vec<u8>, vcs::VcsError> {
        let mut bytes = Vec::new();
        bytes.push(event.kind);
        bytes.extend_from_slice(&event.updated_at_ms.to_le_bytes());
        Self::encode_text(&mut bytes, &event.key)?;
        Self::encode_text(&mut bytes, &event.metadata)?;
        Self::encode_data(&mut bytes, &event.primary)?;
        Self::encode_data(&mut bytes, &event.secondary)?;
        Ok(bytes)
    }

    fn encode_text(output: &mut Vec<u8>, value: &str) -> Result<(), vcs::VcsError> {
        let len = u32::try_from(value.len()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn encode_data(output: &mut Vec<u8>, value: &[u8]) -> Result<(), vcs::VcsError> {
        let len = u64::try_from(value.len()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(value);
        Ok(())
    }

    fn decode_event(bytes: &[u8]) -> Result<FolderEvent, vcs::VcsError> {
        let mut reader = FolderEventReader { bytes, cursor: 0 };
        let event = FolderEvent { kind: reader.u8()?, updated_at_ms: reader.u64()?, key: reader.text()?, metadata: reader.text()?, primary: reader.data()?, secondary: reader.data()? };
        if !matches!(event.kind, DOCUMENT_PUT_EVENT | BLOB_PUT_EVENT | BLOB_DELETE_EVENT) {
            return Err(vcs::VcsError::Backbone(format!("unknown folder event kind {}", event.kind)));
        }
        if reader.cursor != bytes.len() {
            return Err(vcs::VcsError::Backbone("folder event has trailing bytes".into()));
        }
        Ok(event)
    }

    fn append(&self, event: &FolderEvent) -> Result<(), vcs::VcsError> {
        use std::io::Write;
        let payload = Self::encode_event(event)?;
        if payload.len() as u64 > MAX_FOLDER_EVENT_BYTES {
            return Err(vcs::VcsError::Backbone("folder event exceeds the 16 GiB record boundary".into()));
        }
        let _guard = self.writer.lock().map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(self.event_path()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(FOLDER_EVENT_MAGIC).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&(payload.len() as u64).to_le_bytes()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&Self::checksum(&payload).to_le_bytes()).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&payload).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        file.sync_data().map_err(|error| vcs::VcsError::Backbone(error.to_string()))
    }

    fn events(&self) -> Result<Vec<FolderEvent>, vcs::VcsError> {
        use std::io::Read;
        let _guard = self.writer.lock().map_err(|error| vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = match std::fs::File::open(self.event_path()) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(vcs::VcsError::Backbone(error.to_string())),
        };
        let mut events = Vec::new();
        loop {
            let mut magic = [0; 8];
            match file.read_exact(&mut magic) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(vcs::VcsError::Backbone(error.to_string())),
            }
            if &magic != FOLDER_EVENT_MAGIC {
                return Err(vcs::VcsError::Backbone("invalid folder event magic".into()));
            }
            let mut header = [0; 16];
            match file.read_exact(&mut header) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(vcs::VcsError::Backbone(error.to_string())),
            }
            let len = u64::from_le_bytes(header[..8].try_into().unwrap());
            if len > MAX_FOLDER_EVENT_BYTES {
                return Err(vcs::VcsError::Backbone(format!("folder event length {len} exceeds the record boundary")));
            }
            let expected_checksum = u64::from_le_bytes(header[8..].try_into().unwrap());
            let mut payload = vec![0; usize::try_from(len).map_err(|error| vcs::VcsError::Backbone(error.to_string()))?];
            match file.read_exact(&mut payload) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(vcs::VcsError::Backbone(error.to_string())),
            }
            if Self::checksum(&payload) != expected_checksum {
                return Err(vcs::VcsError::Backbone("folder event checksum mismatch".into()));
            }
            events.push(Self::decode_event(&payload)?);
        }
        Ok(events)
    }

    /// @emoji 📖️ Folds the latest stored `(pack, spr)` event for `document_id`.
    pub async fn read(&self, document_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, vcs::VcsError> {
        Ok(self.events()?.into_iter().rev().find(|event| event.kind == DOCUMENT_PUT_EVENT && event.key == document_id).map(|event| (event.primary, event.secondary)))
    }

    /// @emoji ✍️ Appends an indivisible document snapshot event.
    pub async fn write(&self, document_id: &str, schema: &str, pack: &[u8], spr: &[u8]) -> Result<(), vcs::VcsError> {
        self.append(&FolderEvent { kind: DOCUMENT_PUT_EVENT, updated_at_ms: now_ms().await, key: document_id.into(), metadata: schema.into(), primary: pack.into(), secondary: spr.into() })
    }

    /// @emoji 📇️ Lists latest document events in newest-write-first order.
    pub async fn document_ids(&self) -> Result<Vec<String>, vcs::VcsError> {
        let mut latest = std::collections::HashMap::<String, u64>::new();
        for event in self.events()? {
            if event.kind == DOCUMENT_PUT_EVENT {
                latest.insert(event.key, event.updated_at_ms);
            }
        }
        let mut ids = latest.into_iter().collect::<Vec<_>>();
        ids.sort_by(|(left_id, left_time), (right_id, right_time)| right_time.cmp(left_time).then_with(|| left_id.cmp(right_id)));
        Ok(ids.into_iter().map(|(id, _)| id).collect())
    }
}

/// @emoji 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial
/// snapshot), `<id>.<ext>.ops` holds the append-only op log (see `crate::os_store::print_document_text`/
/// `crate::os_store::parse_document_text`). No `Backbone` impl: like `FolderEventLogStorage` above, this actor
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
    pub async fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    async fn dsl_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Dsl))
    }

    async fn ops_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Op))
    }

    /// @emoji 🏷️ Path of the authoritative binary pack file.
    pub async fn pack_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Pack))
    }

    /// @emoji 🏷️ Path of the authoritative binary op-log file.
    pub async fn spr_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(crate::os_store::semio_format::semio_filename(document_id, envelope_id, crate::os_store::semio_format::Component::Spr))
    }

    /// @emoji 📖️ Reads both files for `document_id`, or `None` if the DSL file does not exist yet.
    pub async fn read(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactTextFiles>, vcs::VcsError> {
        let dsl = match std::fs::read_to_string(self.dsl_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactTextFiles { dsl, ops }))
    }

    /// @emoji ✍️ Overwrites both files wholesale — the structural-command cold path (undo/redo/
    /// checkpoint/alternative).
    pub async fn write(&self, document_id: &str, envelope_id: &str, files: &ArtifactTextFiles) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id).await, &files.dsl).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id).await, &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖️ pack+spr-first read: reads the AUTHORITATIVE pair for `document_id`, or `None` if
    /// the `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone
    /// doesn't count — pack+spr are authoritative per the disk-layout LAW, the DSL file is
    /// import-only). A present `.pack` with a missing `.spr` is a hard error — no legacy: they are
    /// always written together (see `write_pack`), so a missing `.spr` means corruption or a
    /// manual edit, never a valid state to silently recover from.
    pub async fn read_pack(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactPackFiles>, vcs::VcsError> {
        let pack = match std::fs::read(self.pack_path(document_id, envelope_id).await) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let spr = std::fs::read(self.spr_path(document_id, envelope_id).await).map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                vcs::VcsError::Backbone(format!("{document_id} pack.semio exists but spr.semio is missing for envelope {envelope_id}"))
            } else {
                vcs::VcsError::Backbone(err.to_string())
            }
        })?;
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactPackFiles { pack, spr, ops }))
    }

    /// @emoji ✍️ Overwrites all four files: the AUTHORITATIVE `.pack` + `.spr` pair, the shared
    /// `.ops` text mirror, and the always-written DSL mirror `dsl_mirror` (`print_dsl` on the
    /// initial snapshot) — the pack-aware sibling of `write`.
    pub async fn write_pack(&self, document_id: &str, envelope_id: &str, files: &ArtifactPackFiles, dsl_mirror: &str) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.pack_path(document_id, envelope_id).await, &files.pack).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.spr_path(document_id, envelope_id).await, &files.spr).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id).await, &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id).await, dsl_mirror).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ➕️ Appends already-printed op-log lines (one {@link print_edit_lines} block) to the `.ops`
    /// file without rewriting it — the hot-path append unit, O(new edit) instead of O(whole history).
    pub async fn append_ops(&self, document_id: &str, envelope_id: &str, lines: &str) -> Result<(), vcs::VcsError> {
        use std::io::Write;
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(self.ops_path(document_id, envelope_id).await).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        file.write_all(lines.as_bytes()).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📇️ Lists every stored document id (by DSL `.semio` file stem) for a given envelope id.
    pub async fn document_ids(&self, envelope_id: &str) -> Result<Vec<String>, vcs::VcsError> {
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

/// @emoji 🗄️ Content-addressed blob events in [`FolderEventLogStorage`].
#[cfg(not(target_arch = "wasm32"))]
impl crate::os_store::BlobStore for FolderEventLogStorage {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<crate::os_store::BlobRef, vcs::VcsError> {
        let hash = semio_framework_hash::hash_bytes(bytes);
        self.append(&FolderEvent { kind: BLOB_PUT_EVENT, updated_at_ms: now_ms().await, key: hash.clone(), metadata: media_type.into(), primary: bytes.into(), secondary: Vec::new() })?;
        Ok(crate::os_store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.into() })
    }

    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, vcs::VcsError> {
        for event in self.events()?.into_iter().rev() {
            if event.key == hash {
                return match event.kind {
                    BLOB_PUT_EVENT => Ok(Some(event.primary)),
                    BLOB_DELETE_EVENT => Ok(None),
                    _ => continue,
                };
            }
        }
        Ok(None)
    }

    async fn has(&self, hash: &str) -> Result<bool, vcs::VcsError> {
        Ok(self.get(hash).await?.is_some())
    }

    async fn delete(&self, hash: &str) -> Result<(), vcs::VcsError> {
        self.append(&FolderEvent { kind: BLOB_DELETE_EVENT, updated_at_ms: now_ms().await, key: hash.into(), metadata: String::new(), primary: Vec::new(), secondary: Vec::new() })
    }
}
//#endregion 🔖️BlobStoreImpl

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{ArtifactId, Edit, Mutation, MutationDiff, OpBinary, OpText};
    use crate::os_store::{
        create_document_envelope, pack_rt, parse_document_pack, parse_document_text, print_document_pack, print_document_text, print_edit_lines, register_document_codec, ArtifactCodec, ArtifactCommand, ArtifactDsl, ArtifactPack, BlobStore,
        PackDecodeOptions, PackEncodeOptions, PackError, ParsedDocumentText,
    };
    use serde::{Deserialize, Serialize};

    fn test_pool() -> std::sync::Arc<semio_framework_async::WorkerPool> {
        static POOL: std::sync::OnceLock<std::sync::Arc<semio_framework_async::WorkerPool>> = std::sync::OnceLock::new();
        POOL.get_or_init(|| std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)))).clone()
    }

    #[test]
    fn artifact_mailbox_item_cap_plus_one_returns_exact_owner_and_preserves_fifo() {
        let (sender, receiver) = artifact_mailbox_pair();
        for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
            sender.send(ArtifactActorMsg::PublishPreview { key: format!("key-{seq}"), seq, payload: vec![seq as u8] }).expect("admit fixed mailbox owner");
        }
        let rejected = sender.send(ArtifactActorMsg::PublishPreview { key: "cap-plus-one".into(), seq: 256, payload: vec![7, 8] }).expect_err("item cap + 1 must reject");
        assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { key, seq: 256, payload } if key == "cap-plus-one" && payload == vec![7, 8]));
        for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
            assert!(matches!(receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: actual, .. }) if actual == seq));
        }
        assert!(receiver.try_recv().is_none());
    }

    #[test]
    fn artifact_mailbox_byte_cap_and_plus_one_preflight_before_mutation() {
        let exact_payload = vec![3; ARTIFACT_MAILBOX_BYTES - 17];
        let (exact_sender, exact_receiver) = artifact_mailbox_pair();
        exact_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 1, payload: exact_payload }).expect("exact byte cap admits");
        assert_eq!(exact_sender.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).bytes, ARTIFACT_MAILBOX_BYTES);
        assert!(matches!(exact_receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: 1, .. })));

        let (overflow_sender, overflow_receiver) = artifact_mailbox_pair();
        let rejected = overflow_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 2, payload: vec![9; ARTIFACT_MAILBOX_BYTES - 16] }).expect_err("byte cap + 1 must reject");
        assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { seq: 2, payload, .. } if payload.len() == ARTIFACT_MAILBOX_BYTES - 16));
        assert!(overflow_receiver.try_recv().is_none(), "byte rejection cannot mutate FIFO state");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn artifact_mailbox_wake_storm_coalesces_until_fifo_becomes_empty() {
        let (sender, receiver) = artifact_mailbox_pair();
        let wakes = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed = wakes.clone();
        receiver.set_wake(std::sync::Arc::new(move || {
            observed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }));
        for _ in 0..ARTIFACT_MAILBOX_ITEMS {
            sender.send(ArtifactActorMsg::ExternalChanged).expect("wake-storm owner admitted");
        }
        assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 1);
        while receiver.try_recv().is_some() {}
        sender.send(ArtifactActorMsg::Detach).expect("new readiness edge admitted");
        assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 2);
    }

    #[test]
    fn artifact_mailbox_stale_late_send_hands_back_exact_owner_and_interrupted_close_drains_one_per_grant() {
        let (sender, receiver) = artifact_mailbox_pair();
        for seq in 0..3 {
            sender.send(ArtifactActorMsg::PublishPreview { key: "close".into(), seq, payload: vec![seq as u8] }).expect("close fixture admission");
        }
        let close = receiver.close_handle();
        close.close();
        let late = sender.send(ArtifactActorMsg::PublishPreview { key: "late".into(), seq: 99, payload: vec![4, 5, 6] }).expect_err("late generation rejects");
        assert!(matches!(late, ArtifactMailboxSendError::Stale { message: ArtifactActorMsg::PublishPreview { key, seq: 99, payload } } if key == "late" && payload == vec![4, 5, 6]));
        for remaining in [2, 1, 0] {
            assert!(close.close_one());
            assert_eq!(close.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, remaining);
        }
        assert!(!close.close_one());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order() {
        use crate::os_store::Backbone;
        let nested = sample_presence_peer_with_interaction().await;
        let bare = PresencePeer {
            actor: nested.actor.clone(),
            connected_at_ms: nested.connected_at_ms,
            label: None,
            presence_pack: None,
            user_id: None,
            role: None,
            drag_ghost_json: None,
            interaction: None,
            color: None,
            surface: None,
            views: Vec::new(),
            ui: None,
        };
        let nested_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(nested) }).expect("nested message fits");
        let bare_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(bare) }).expect("bare message fits");
        assert!(nested_bytes > bare_bytes, "all nested identifiers and collections contribute byte credit");

        let (mut channel, remote) = ChannelBackbone::pair("store-sync-one-pop").await;
        channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("first backbone owner");
        channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("second backbone owner");
        assert!(matches!(remote.try_pop_front().expect("first opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["first"]));
        assert!(matches!(remote.try_pop_front().expect("second opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["second"]));
        assert!(remote.try_pop_front().expect("idle opportunity").is_none());
    }

    // 🎯️ `id` must be dotted `plugin.artifact` — `os_semio`'s preamble validator rejects a bare
    // extension (this crate never compiled with `--features sync` before this packet, so the
    // mismatch was never exercised at runtime). `extension_suffix` is the id's LAST segment, so
    // `"demo.demo"` keeps `__DSL_EXTENSION` == "demo", unchanged from the old bare-extension form.
    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslArtifact)]
    #[dsl(id = "demo.demo")]
    struct DemoSnapshot {
        n: i32,
    }

    impl ArtifactDsl for DemoSnapshot {
        const EXTENSION: &'static str = Self::__DSL_EXTENSION;
        fn envelope_id() -> &'static str {
            Self::__DSL_ENVELOPE_ID
        }
        fn parse_dsl(text: &str) -> Result<Self, crate::os_dsl::TextError> {
            let body = match semio_format::split_text_preamble(text) {
                Ok((_, rest)) => rest,
                Err(_) => text,
            };
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
        fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
            Some(Self::__dsl_spec())
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl MutationDiff<DemoSnapshot> for DemoDiff {
        fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
            Ok(DemoSnapshot { n: self.n.unwrap_or(snapshot.n) })
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    #[value(tag = "operation")]
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
                    let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
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
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or_else(|| crate::os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: format!("op ordinal {ordinal} out of range for {}", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position()..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            let offset = reader.position() as u64;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset, detail: error.to_string() })
        }
    }

    impl Mutation<DemoSnapshot> for DemoMutation {
        type Diff = DemoDiff;

        fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
            crate::os_spr::MutationOutcome::new(match self {
                DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
            })
            
        }

        fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![DemoMutation::SetN { n: snapshot.n }]
        }
    }

    /// @emoji 🎯️ Idempotently registers the `demo/v1` codec (process-global `OnceLock` registry,
    /// shared across every test in this binary) — needed by any test exercising `FolderEndpoint`
    /// end-to-end (both `Sqlite` and `Pack` now go through `document_codec` per the pack+spr flip),
    /// mirroring a real app's program-init-time `register_document_codec_for_app` call.
    async fn ensure_demo_codec_registered() {
        static ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !ONCE.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = register_document_codec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1")).await.expect("register demo codec");
        }
    }

    fn bootstrap_frontier(document_id: &str, ordinal: u64, edit_id: &str, commit: u64, chain: u8) -> RuntimeFrontierSummary {
        RuntimeFrontierSummary { document_id: ArtifactId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: edit_id.into(), last_commit_seq: commit, chain_hash: [chain; 32] }
    }

    async fn demo_artifact_bootstrap(inline: bool) -> (ArtifactBootstrap, ArtifactBootstrapPair) {
        ensure_demo_codec_registered().await;
        let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 7 }, None);
        let files = print_document_pack(&envelope).await.expect("print bootstrap pair");
        let pair = ArtifactBootstrapPair { pack: files.pack, spr: files.spr };
        let pack_schema_hash = crate::os_store::document_codec("demo/v1").await.expect("codec lookup").expect("demo codec").pack_schema_hash;
        let bootstrap = ArtifactBootstrap {
            format_version: crate::os_spr::ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
            descriptor_hash: [0x11; 32],
            artifact_schema: "demo/v1".into(),
            artifact_kind: "demo".into(),
            pack_schema_hash,
            baseline_frontier: bootstrap_frontier("demo", 7, "edit-7", 3, 0x33),
            pack_hash: semio_framework_hash::Sha256::digest(&pair.pack),
            spr_hash: semio_framework_hash::Sha256::digest(&pair.spr),
            pack_length: pair.pack.len() as u64,
            spr_length: pair.spr.len() as u64,
            chunk_count: if inline { 0 } else { 3 },
            aggregate_hash: crate::os_spr::artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr),
            required_tail_frontier: bootstrap_frontier("demo", 9, "edit-9", 4, 0x44),
            inline: inline.then(|| pair.clone()),
        };
        (bootstrap, pair)
    }

    #[test]
    fn bootstrap_frontier_identity_rejects_same_ordinals_with_wrong_authenticated_head() {
        let required = bootstrap_frontier("demo", 9, "edit-9", 4, 0x44);
        let mut wrong_head = required.clone();
        wrong_head.head_edit_id = "edit-other".into();
        let mut wrong_chain = required.clone();
        wrong_chain.chain_hash = [0x55; 32];
        assert!(!frontier_reaches(&wrong_head, &required));
        assert!(!frontier_reaches(&wrong_chain, &required));
        assert!(frontier_reaches(&required, &required));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn native_terminal_connection_failure_clears_receipt_actor_before_reissue() {
        use futures::StreamExt;
        use tokio_tungstenite::tungstenite::Message;

        async fn connect(
            actor: &mut native_actor::ArtifactActor,
            receipt_actor: &str,
        ) -> tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind test socket");
            let url = format!("ws://{}", listener.local_addr().expect("test socket address"));
            let accepted = tokio::spawn(async move {
                let (stream, _) = listener.accept().await.expect("accept test socket");
                tokio_tungstenite::accept_async(stream).await.expect("upgrade test socket")
            });
            actor.connect_test_socket(&url, receipt_actor).await;
            accepted.await.expect("test socket task")
        }

        async fn receive_frame(socket: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) -> ClientFrame {
            let message = tokio::time::timeout(std::time::Duration::from_secs(1), socket.next()).await.expect("client frame deadline").expect("client frame owner").expect("client frame");
            let Message::Binary(bytes) = message else { panic!("expected binary client frame") };
            crate::os_spr::decode_client_frame(&bytes).await.expect("decode client frame").1
        }

        let (_, remote) = ChannelBackbone::pair("native-actor-epoch-test").await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, _) = broadcast::channel(8);
        let mut actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "local-only".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        let stale = "hub.v1.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let fresh = "hub.v1.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        actor.install_test_socket_actor(stale);
        actor.inject_hub_frame(ServerFrame::Session { actor: ActorId(stale.into()), color: 3 }).await;
        assert_eq!(actor.socket_epoch_test_state(), (Some(stale.into()), true, 0, Vec::new()));
        actor.fail_test_bootstrap().await;
        assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
        let first = sample_operation_envelope("after-bootstrap-failure", 1).await;
        let first_actor = first.actor.0.clone();
        actor.relay_test_envelope(first).await;
        assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor]));

        actor.install_test_socket_actor(stale);
        actor.inject_hub_frame(ServerFrame::Session { actor: ActorId(stale.into()), color: 4 }).await;
        actor.fail_test_connection().await;
        assert_eq!(actor.socket_epoch_test_state().0, None);
        assert!(!actor.socket_epoch_test_state().1);
        let second = sample_operation_envelope("after-eof", 2).await;
        let second_actor = second.actor.0.clone();
        actor.relay_test_envelope(second).await;
        assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor, second_actor]));

        let mut socket = connect(&mut actor, fresh).await;
        assert!(matches!(receive_frame(&mut socket).await, ClientFrame::SocketHelloV1 { .. }));
        let before_session = sample_operation_envelope("before-fresh-session", 3).await;
        let before_session_actor = before_session.actor.0.clone();
        actor.relay_test_envelope(before_session).await;
        assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), false, 0, vec![first_actor, second_actor, before_session_actor]));
        assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "queued mutations cannot cross the socket before Session confirms the receipt actor");
        actor.inject_hub_frame(ServerFrame::Session { actor: ActorId(fresh.into()), color: 5 }).await;
        let first_batch = receive_frame(&mut socket).await;
        let ClientFrame::Commands { batch_id, envelopes } = first_batch else { panic!("fresh Session must flush one command batch") };
        assert_eq!(envelopes.len(), 3);
        assert!(envelopes.iter().all(|envelope| envelope.actor.0 == fresh));
        assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "each queued mutation is sent exactly once after Session");
        assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 1, Vec::new()));
        actor
            .inject_hub_frame(ServerFrame::Ack {
                batch_id,
                stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }],
                frontier: bootstrap_frontier("demo", 3, "after-session", 3, 0x66),
            })
            .await;
        assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 0, Vec::new()));

        actor.fail_test_connection().await;
        let reconnected = "hub.v1.cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
        let queued_during_reconnect = sample_operation_envelope("during-reconnect", 4).await;
        actor.relay_test_envelope(queued_during_reconnect).await;
        let mut reconnected_socket = connect(&mut actor, reconnected).await;
        assert!(matches!(receive_frame(&mut reconnected_socket).await, ClientFrame::SocketHelloV1 { .. }));
        assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect cannot flush before its own Session");
        actor.inject_hub_frame(ServerFrame::Session { actor: ActorId(reconnected.into()), color: 6 }).await;
        let ClientFrame::Commands { envelopes, .. } = receive_frame(&mut reconnected_socket).await else { panic!("reconnect Session must flush queued mutation") };
        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].actor.0, reconnected);
        assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect flush is exactly once");
        assert_eq!(actor.socket_epoch_test_state(), (Some(reconnected.into()), true, 1, Vec::new()));

        actor.expire_test_socket_authority();
        let after_expiry = sample_operation_envelope("after-authority-expiry", 5).await;
        actor.relay_test_envelope(after_expiry).await;
        let (socket_actor, confirmed, pending, queued) = actor.socket_epoch_test_state();
        assert_eq!((socket_actor, confirmed, pending), (None, false, 0));
        assert_eq!(queued.len(), 2, "unacknowledged and post-expiry mutations stay queued for a fresh plan");
        let terminal = tokio::time::timeout(std::time::Duration::from_secs(1), reconnected_socket.next()).await.expect("expired authority closes promptly");
        assert!(!matches!(terminal, Some(Ok(Message::Binary(_)))), "expired plan authority cannot carry another command");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn native_bootstrap_commits_pair_before_failed_local_replay_then_restarts_without_duplicate() {
        use crate::os_store::Backbone;
        let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
        let required = bootstrap.required_tail_frontier.clone();
        let baseline = bootstrap.baseline_frontier.clone();
        let (mut channel, remote) = ChannelBackbone::pair("native-bootstrap-test").await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, mut event_rx) = broadcast::channel(32);
        let mut actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        let local = sample_operation_envelope("pending-local", 8).await;
        actor.queue_test_outbox(vec![local.clone(), local.clone()]);
        actor.inject_bootstrap_local_replay_failure();
        let welcome = |bootstrap: ArtifactBootstrap| ServerFrame::Welcome {
            session_id: "session-bootstrap".into(),
            resume_token: "resume-bootstrap".into(),
            server_frontier: required.clone(),
            bootstrap: Bootstrap::ArtifactBootstrap(bootstrap),
        };

        actor.inject_hub_frame(welcome(bootstrap.clone())).await;
        let first = channel.receive().await.expect("baseline queue");
        assert_eq!(first.len(), 1, "failed replay queues only the committed baseline");
        assert!(matches!(&first[0], BackboneMessage::Snapshot { pack, spr } if pack == &pair.pack && spr == &pair.spr));
        let (pack, spr, frontier, pending_required, resume, pending_resume, remote_state, outbox) = actor.bootstrap_test_state();
        assert_eq!(pack.as_deref(), Some(pair.pack.as_slice()));
        assert_eq!(spr.as_deref(), Some(pair.spr.as_slice()));
        assert_eq!(frontier, Some(baseline.clone()));
        assert_eq!(pending_required, Some(required.clone()));
        assert_eq!(resume, None);
        assert_eq!(pending_resume.as_deref(), Some("resume-bootstrap"));
        assert!(!matches!(remote_state, RemoteState::Live { .. }));
        assert_eq!(outbox, vec![local.mutation_id.0.clone()], "failure preserves one deduplicated local owner");
        assert!(matches!(event_rx.try_recv(), Ok(ArtifactEvent::BootstrapProgress { .. })));

        actor.inject_hub_frame(ServerFrame::Presence { peers: Vec::new() }).await;
        assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "presence cannot bypass authenticated catch-up");
        actor.inject_hub_frame(welcome(bootstrap)).await;
        let restarted = channel.receive().await.expect("restart queue");
        assert_eq!(restarted.iter().filter(|message| matches!(message, BackboneMessage::Snapshot { .. })).count(), 1);
        let replayed: Vec<MutationEnvelope> = restarted
            .iter()
            .filter_map(|message| match message {
                BackboneMessage::Mutations { envelopes } => Some(decode_envelopes(envelopes).expect("decode replay")),
                _ => None,
            })
            .flatten()
            .collect();
        assert_eq!(replayed.iter().map(|envelope| &envelope.mutation_id).collect::<Vec<_>>(), vec![&local.mutation_id], "restart performs one successful local replay");

        let mut wrong = required.clone();
        wrong.head_edit_id = "edit-wrong".into();
        wrong.chain_hash = [0x55; 32];
        actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: wrong }).await;
        assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "same ordinals cannot authenticate a different chain");
        actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
        let (_, _, frontier, pending_required, resume, _, remote_state, outbox) = actor.bootstrap_test_state();
        assert_eq!(frontier, Some(required));
        assert_eq!(pending_required, None);
        assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
        assert!(matches!(remote_state, RemoteState::Live { .. }));
        assert_eq!(outbox, vec![local.mutation_id.0], "offline hub retains the exact local owner after semantic replay");
        assert!(channel.receive().await.expect("no duplicate store replay").is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn native_inline_and_chunked_bootstrap_install_the_same_typed_pair_after_cancelled_restart() {
        use crate::os_store::Backbone;
        async fn actor_pair(uri: &str) -> (native_actor::ArtifactActor, ChannelBackbone) {
            let (channel, remote) = ChannelBackbone::pair(uri).await;
            let (_, receiver) = artifact_mailbox_pair();
            let (events, _) = broadcast::channel(32);
            let actor = native_actor::ArtifactActor::new(
                test_pool(),
                ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
                remote,
                receiver,
                events,
                Arc::new(std::sync::RwLock::new(None)),
                Arc::new(std::sync::RwLock::new(None)),
                None,
                semio_framework_async::CancelToken::root_now(),
            )
            .await;
            (actor, channel)
        }
        let (inline_bootstrap, pair) = demo_artifact_bootstrap(true).await;
        let (mut chunked_bootstrap, _) = demo_artifact_bootstrap(false).await;
        let required = inline_bootstrap.required_tail_frontier.clone();
        let welcome = |bootstrap: ArtifactBootstrap| ServerFrame::Welcome {
            session_id: "session-bootstrap".into(),
            resume_token: "resume-bootstrap".into(),
            server_frontier: required.clone(),
            bootstrap: Bootstrap::ArtifactBootstrap(bootstrap),
        };

        let (mut inline_actor, mut inline_channel) = actor_pair("native-bootstrap-inline").await;
        inline_actor.inject_hub_frame(welcome(inline_bootstrap)).await;
        let inline_messages = inline_channel.receive().await.expect("inline messages");
        assert!(matches!(&inline_messages[..], [BackboneMessage::Snapshot { pack, spr }] if pack == &pair.pack && spr == &pair.spr));

        let mut combined = pair.pack.clone();
        combined.extend_from_slice(&pair.spr);
        let chunk_size = combined.len().div_ceil(3);
        let chunks: Vec<Vec<u8>> = combined.chunks(chunk_size).map(<[u8]>::to_vec).collect();
        assert_eq!(chunks.len(), 3);
        chunked_bootstrap.chunk_count = chunks.len() as u32;
        let descriptor_hash = chunked_bootstrap.descriptor_hash;
        let (mut chunked_actor, mut chunked_channel) = actor_pair("native-bootstrap-chunked").await;
        chunked_actor.inject_hub_frame(welcome(chunked_bootstrap.clone())).await;
        chunked_actor
            .inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: 0, bytes: crate::os_spr::ArtifactBootstrapChunkBytes::try_from_slice(&chunks[0]).expect("bounded chunk") })
            .await;
        chunked_actor.cancel_test_bootstrap();
        assert!(chunked_channel.receive().await.expect("cancelled staging").is_empty(), "cancellation commits no partial pair");

        chunked_actor.inject_hub_frame(welcome(chunked_bootstrap)).await;
        for (index, chunk) in chunks.iter().enumerate() {
            chunked_actor
                .inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: index as u32, bytes: crate::os_spr::ArtifactBootstrapChunkBytes::try_from_slice(chunk).expect("bounded chunk") })
                .await;
            if index + 1 < chunks.len() {
                assert!(chunked_channel.receive().await.expect("staged chunk").is_empty(), "chunks stay invisible before done");
            }
        }
        chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count: chunks.len() as u32 }).await;
        let chunked_messages = chunked_channel.receive().await.expect("chunked messages");
        assert_eq!(chunked_messages, inline_messages, "inline and chunked replace with byte-identical typed pairs");
        chunked_actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
        let (actual_pack, actual_spr, frontier, pending_required, resume, _, remote_state, _) = chunked_actor.bootstrap_test_state();
        assert_eq!(actual_pack, Some(pair.pack));
        assert_eq!(actual_spr, Some(pair.spr));
        assert_eq!(frontier, Some(required));
        assert_eq!(pending_required, None);
        assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
        assert!(matches!(remote_state, RemoteState::Live { .. }));
    }

    async fn sample_operation_envelope(edit_id: &str, n: i32) -> MutationEnvelope {
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
        let document_id = ArtifactId("demo".to_string());
        let schema = crate::os_spr::SchemaId("demo/v1".to_string());
        let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
        envelopes.pop().expect("exactly one op envelope for a single-op edit")
    }

    //#region 🧪️SyncSession
    #[semio_framework_async_macros::async_test]
    async fn receive_materializes_remote_envelope_into_the_edit_timeline() {
        let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope).await.expect("valid receive fixture");
        let mut session = SyncSession::new(store).await;
        session.receive(sample_operation_envelope("edit-1", 5).await).await.expect("receive");
        assert_eq!(session.store.snapshot().expect("snapshot").n, 5);
        assert_eq!(session.store.envelope().vcs.edits.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
        let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope).await.expect("valid out-of-order fixture");
        let mut session = SyncSession::new(store).await;
        let first = sample_operation_envelope("edit-1", 5).await;
        let mut second = sample_operation_envelope("edit-2", 9).await;
        second.dependencies = vec![first.mutation_id.clone()];
        session.receive(second).await.expect("receive second first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
        session.receive(first).await.expect("receive first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
        assert_eq!(session.store.snapshot().expect("snapshot").n, 9);
    }
    //#endregion 🧪️SyncSession

    //#region 🧪️Helpers
    #[semio_framework_async_macros::async_test]
    async fn hub_ws_url_derives_ws_endpoint_from_remote_uri() {
        assert_eq!(hub_ws_url("remote://host:6070", "studio-1", "doc-1", None).await, "ws://host:6070/spaces/studio-1/documents/doc-1/socket/v1");
        assert_eq!(hub_ws_url("https://semio_hub.example.com", "studio-1", "doc-2", None).await, "wss://semio_hub.example.com/spaces/studio-1/documents/doc-2/socket/v1");
        assert_eq!(hub_ws_url("ws://127.0.0.1:5000/prefix", "studio-1", "d", None).await, "ws://127.0.0.1:5000/spaces/studio-1/documents/d/socket/v1");
        assert_eq!(
            hub_ws_url("remote://host:6070", "studio /東京?", "doc#ä", Some("s.space.home@1/*#editor")).await,
            "ws://host:6070/spaces/studio%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/doc%23%C3%A4/socket/v1?surface=s.space.home%401%2F%2A%23editor",
            "ticket 26/08/16/HUB-SPACES-…: surface travels out of band as ?surface= on the document WS URL"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn hostile_hub_binding_cannot_receive_a_credential_bound_document_grant() {
        use crate::os_directory::client::{
            DirectoryClientError, DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, HubSocketGrantSource, LocalHubCredential, SocketGrantReceiptV1,
        };
        use crate::os_directory::{
            DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenRendererTargetV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceRoleV1,
            DocumentOpenSurfaceV1, DocumentScope,
        };
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct BoundSource {
            admissions: AtomicUsize,
            trusted_origin: String,
        }

        impl HubSocketGrantSource for BoundSource {
            fn admit_document_socket(
                &self,
                _ctx: &semio_framework_async::OperationContext,
                space_id: &str,
                document_id: &str,
                expectation: &crate::os_directory::client::DocumentSocketExpectationV1,
                _client_instance_id: &str,
                _timeout_ms: u64,
            ) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
                self.admissions.fetch_add(1, Ordering::SeqCst);
                Ok(DocumentSocketAdmissionV1 {
                    socket: SocketGrantReceiptV1 {
                        schema: "semio.hub.socket-grant/v1".into(),
                        protocol: "semio.socket.v1".into(),
                        grant: format!("socket.v1.{}.{}", "1".repeat(32), "2".repeat(64)),
                        actor_id: format!("hub.v1.{}", "3".repeat(64)),
                        expires_at_ms: i64::MAX,
                    },
                    authority: DocumentSocketAuthorityV1 {
                        hub_origin: self.trusted_origin.clone(),
                        expires_at_unix_ms: u64::try_from(i64::MAX).expect("positive max"),
                        scope: DocumentScope::new(space_id, document_id),
                        descriptor_digest_v1: "4".repeat(64),
                        catalog: DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                        package: DocumentOpenPackageV1 {
                            plugin_id: "trusted.plugin".into(),
                            package_id: "trusted.package".into(),
                            version: "1.0.0".into(),
                            component_sha256: "6".repeat(64),
                            component_blake3: "7".repeat(64),
                            descriptor_byte_sha256: "8".repeat(64),
                        },
                        artifact: DocumentOpenArtifactV1 { kind: "trusted.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                        parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: "trusted.document".into(), standard: "1".into(), subset: "*".into() },
                        pack_schema_hash: [0x11; 32],
                        surface: DocumentOpenSurfaceV1 {
                            surface_id: "trusted.surface".into(),
                            app_id: "trusted.app".into(),
                            window_kind_id: "trusted.window".into(),
                            role: DocumentOpenSurfaceRoleV1::Editor,
                            renderer_target: DocumentOpenRendererTargetV1::Wgpu,
                        },
                        grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                        checkpoint: None,
                        revalidation: DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                    },
                })
            }
        }

        let hostile = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("hostile listener");
        let hostile_origin = format!("http://{}", hostile.local_addr().expect("hostile address"));
        let source = Arc::new(BoundSource { admissions: AtomicUsize::new(0), trusted_origin: "http://127.0.0.1:1".into() });
        let (_, remote) = ChannelBackbone::pair("hostile-binding-test").await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, _) = broadcast::channel(8);
        let mut actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig {
                document_id: "document".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: hostile_origin, space_id: "space".into(), surface: Some("trusted.surface".into()) }],
                watch_external: false,
                actor: "local-untrusted".into(),
            },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(Some(Arc::new(LocalHubCredential::test("http://127.0.0.1:1", &format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64))))))),
            Arc::new(std::sync::RwLock::new(Some(source.clone()))),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        actor.run_test_connect_attempt().await;
        assert_eq!(source.admissions.load(Ordering::SeqCst), 1);
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), hostile.accept()).await.is_err(), "hostile binding receives no dial or grant header");
        assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
    }

    /// 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4: both
    /// actors call `stamp_session` on every outbound `PresenceHeartbeat` right before
    /// `presence_to_bytes` — shells never fill `color`/`surface` themselves. This proves the pure
    /// helper both actors call: sets both fields from the actor's own `session_color`/`hub_surface`
    /// state, overwriting whatever the shell handed in, and clears `surface` to `None` when the
    /// document has no hub binding (folder-only) even if `session_color` is somehow set.
    #[semio_framework_async_macros::async_test]
    async fn actor_stamps_session_color_and_surface_on_outbound_heartbeat() {
        let mut peer = PresencePeer {
            actor: "actor-1".into(),
            connected_at_ms: 1000,
            label: None,
            presence_pack: None,
            user_id: None,
            role: None,
            drag_ghost_json: None,
            interaction: None,
            color: Some(99),
            surface: Some("shell-should-never-set-this".into()),
            views: Vec::new(),
            ui: None,
        };
        stamp_session(&mut peer, Some(7), Some("s.space.home@1/*#editor")).await;
        assert_eq!(peer.color, Some(7));
        assert_eq!(peer.surface.as_deref(), Some("s.space.home@1/*#editor"));

        stamp_session(&mut peer, None, None).await;
        assert_eq!(peer.color, None, "no session color yet (folder-only document, or hub not yet assigned one)");
        assert_eq!(peer.surface, None, "folder-only document carries no surface");
    }
    //#endregion 🧪️Helpers

    //#region 🧪️WireBridge
    // 🎯️ W6: `wire_bridge_round_trips_identity_and_diff_through_protocol_causal` is DELETED — the
    // local/wire bridge it tested (`to_wire_envelope`/`from_wire_envelope`) no longer exists; local
    // and wire envelopes are the same `crate::os_spr::MutationEnvelope` type now, an identity the type
    // system enforces, not something a round-trip test needs to prove.
    #[semio_framework_async_macros::async_test]
    async fn rollback_envelope_synthesizes_an_undo_from_the_original_inverse() {
        let envelope = sample_operation_envelope("edit-1", 5).await;
        let rollback = rollback_envelope(&envelope).await;
        assert_eq!(rollback.dependencies, vec![envelope.mutation_id.clone()], "the undo depends on the operation it undoes");
        assert_eq!(rollback.diff.payload, envelope.inverse.payload, "the undo's forward diff IS the original's inverse");
        assert_ne!(rollback.mutation_id, envelope.mutation_id, "the undo gets its own operation id");
    }

    /// 🎬️ Compares nineteen committed wire specimens with Rust encoding and decoding without
    /// modifying them. The TypeScript replication suite consumes the same semantic-owner files
    /// under `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/📡️wire`.
    /// The current socket hello round-trips in memory; the committed obsolete hello must reject.
    #[semio_framework_async_macros::async_test]
    async fn wire_fixtures_stay_byte_identical_across_rust_and_ts() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🔨️modules/📡️replication/🧫️fixtures/📡️wire");

        async fn check_client(dir: &std::path::Path, name: &str, frame: &ClientFrame, lane: Lane) {
            let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
            assert_eq!(encode_client_frame(frame, lane).await, bytes, "{name} committed bytes");
            let (decoded_lane, decoded) = crate::os_spr::decode_client_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
            assert_eq!(decoded_lane, lane, "{name} lane round trip");
            assert_eq!(&decoded, frame, "{name} frame round trip");
        }

        async fn check_server(dir: &std::path::Path, name: &str, frame: &ServerFrame, lane: Lane) {
            let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
            assert_eq!(crate::os_spr::encode_server_frame(frame, lane).await, bytes, "{name} committed bytes");
            let (decoded_lane, decoded) = decode_server_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
            assert_eq!(decoded_lane, lane, "{name} lane round trip");
            assert_eq!(&decoded, frame, "{name} frame round trip");
        }

        let frontier = crate::os_spr::RuntimeFrontierSummary { document_id: ArtifactId("doc-1".to_string()), head_edit_ordinal: 1, head_edit_id: "op-1".to_string(), last_commit_seq: 1, chain_hash: [9u8; 32] };
        let wire_envelope = MutationEnvelope {
            mutation_id: MutationId("op-1".to_string()),
            document_id: ArtifactId("doc-1".to_string()),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
            inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).expect("encode demo op") },
            timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1000, logical: 0 },
        };

        //#region 🔖️ClientFrame
        let hello = ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "demo/v1".to_string(), pack_schema_hash: [7u8; 32], resume_token: None, frontier: None };
        let hello_bytes = encode_client_frame(&hello, Lane::Command).await;
        let (hello_lane, decoded_hello) = crate::os_spr::decode_client_frame(&hello_bytes).await.expect("decode current socket hello");
        assert_eq!(hello_lane, Lane::Command, "current socket hello lane round trip");
        assert_eq!(decoded_hello, hello, "current socket hello frame round trip");
        let rejected_hello = std::fs::read(fixtures_dir.join("🚫️legacy-client-hello-rejected/💾️.bin")).expect("read rejected obsolete hello");
        assert!(crate::os_spr::decode_client_frame(&rejected_hello).await.is_err(), "obsolete hello must reject");
        check_client(&fixtures_dir, "🕹️client-commands/💾️.bin", &ClientFrame::Commands { batch_id: 1, envelopes: vec![wire_envelope.clone()] }, Lane::Command).await;
        check_client(&fixtures_dir, "🚩️client-frontier-advertise/💾️.bin", &ClientFrame::FrontierAdvertise { frontier: frontier.clone() }, Lane::Command).await;
        check_client(&fixtures_dir, "📣️client-preview-publish/💾️.bin", &ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
        check_client(&fixtures_dir, "🙋️client-presence/💾️.bin", &ClientFrame::Presence { peer: presence_to_bytes(&sample_presence_peer_with_interaction().await).await }, Lane::Preview).await;
        check_client(&fixtures_dir, "🎟️client-credit-grant/💾️.bin", &ClientFrame::CreditGrant { n: 16 }, Lane::Command).await;
        check_client(&fixtures_dir, "👋️client-bye/💾️.bin", &ClientFrame::Bye, Lane::Command).await;
        //#endregion 🔖️ClientFrame

        //#region 🔖️ServerFrame
        check_server(&fixtures_dir, "🔗️server-welcome-tail/💾️.bin", &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail }, Lane::Command).await;
        check_server(
            &fixtures_dir,
            "📸️server-welcome-snapshot-inline/💾️.bin",
            &ServerFrame::Welcome { session_id: "session-2".to_string(), resume_token: "resume-2".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9, 9]) } },
            Lane::Command,
        )
        .await;
        check_server(&fixtures_dir, "🧩️server-snapshot-chunk/💾️.bin", &ServerFrame::SnapshotChunk { seq: 0, bytes: crate::os_spr::SnapshotChunkBytes::try_from_slice(&[1, 2, 3, 4]).unwrap() }, Lane::Command).await;
        check_server(&fixtures_dir, "🏁️server-snapshot-done/💾️.bin", &ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command).await;
        check_server(&fixtures_dir, "🎮️server-commands/💾️.bin", &ServerFrame::Commands { envelopes: vec![wire_envelope], origin: ActorId("actor-1".to_string()), frontier: frontier.clone() }, Lane::Command).await;
        check_server(
            &fixtures_dir,
            "✅️server-ack-accepted/💾️.bin",
            &ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() },
            Lane::Command,
        )
        .await;
        check_server(
            &fixtures_dir,
            "🔀️server-ack-transformed/💾️.bin",
            &ServerFrame::Ack {
                batch_id: 2,
                stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Transformed { envelope: Box::new(sample_wire_envelope_for_fixtures().await) }) }],
                frontier: frontier.clone(),
            },
            Lane::Command,
        )
        .await;
        check_server(
            &fixtures_dir,
            "⛔️server-ack-rejected/💾️.bin",
            &ServerFrame::Ack {
                batch_id: 3,
                stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "conflict".to_string(), messages: vec![1, 2, 3] }) }],
                frontier: frontier.clone(),
            },
            Lane::Command,
        )
        .await;
        check_server(&fixtures_dir, "👁️server-preview/💾️.bin", &ServerFrame::Preview { actor: ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview).await;
        check_server(&fixtures_dir, "👥️server-presence/💾️.bin", &ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), presence_to_bytes(&sample_presence_peer_with_interaction().await).await] }, Lane::Preview).await;
        check_server(&fixtures_dir, "🎫️server-credit-grant/💾️.bin", &ServerFrame::CreditGrant { n: 32 }, Lane::Command).await;
        check_server(&fixtures_dir, "🚨️server-error/💾️.bin", &ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
        check_server(&fixtures_dir, "🪪️server-session/💾️.bin", &ServerFrame::Session { actor: "actor-1".to_string(), color: 5 }, Lane::Command).await;
        //#endregion 🔖️ServerFrame
    }

    /// 🧸️ A second, distinct `MutationEnvelope` for `🔀️server-ack-transformed/💾️.bin`'s
    /// `ApplyOutcome::Transformed` payload — must differ from the primary `wire_envelope` fixture so
    /// the vitest canary can assert it decodes as its own value, not an accidental copy.
    async fn sample_wire_envelope_for_fixtures() -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: MutationId("op-2".to_string()),
            document_id: ArtifactId("doc-1".to_string()),
            actor: ActorId("actor-2".to_string()),
            dependencies: vec![MutationId("op-1".to_string())],
            diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).expect("encode demo op") },
            inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
            timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1001, logical: 0 },
        }
    }

    /// 🕹️ A `PresencePeer` whose `interaction` carries THREE domains (one selection-only, one
    /// hover-only, one with both), TWO `views` (one Orbit with a pointer, one Canvas), a `color` +
    /// `surface`, and a `ui` — `🙋️client-presence/💾️.bin`/`👥️server-presence/💾️.bin` match this so
    /// the TS vitest twin exercises every `PresencePeer` v3 flag bit (§C7.1) with a realistic payload.
    async fn sample_presence_peer_with_interaction() -> PresencePeer {
        PresencePeer {
            actor: "actor-1".to_string(),
            connected_at_ms: 1_700_000_000_000,
            label: Some("Ada".to_string()),
            presence_pack: None,
            user_id: Some("user-9".to_string()),
            role: Some("owner".to_string()),
            drag_ghost_json: None,
            interaction: Some(crate::os_spr::PresenceInteraction {
                app_id: "space".to_string(),
                domains: vec![
                    crate::os_spr::PresenceDomain { domain: "outline".to_string(), granularity: "task".to_string(), selected: vec!["t1".to_string(), "t2".to_string()], hovered: vec![] },
                    crate::os_spr::PresenceDomain { domain: "board".to_string(), granularity: "card".to_string(), selected: vec![], hovered: vec!["c1".to_string()] },
                    crate::os_spr::PresenceDomain { domain: "canvas".to_string(), granularity: "node".to_string(), selected: vec!["n9".to_string()], hovered: vec!["n9".to_string(), "n10".to_string()] },
                ],
            }),
            color: Some(5),
            surface: Some("s.space.home@1/*#editor".to_string()),
            views: vec![
                crate::os_spr::PresenceWindowView {
                    window_id: "w1".to_string(),
                    space: "world".to_string(),
                    kind: crate::os_spr::PresenceViewKind::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 },
                    size: [1024.0, 768.0],
                    pointer: Some([0.5, 0.5, 0.5]),
                },
                crate::os_spr::PresenceWindowView { window_id: "w2".to_string(), space: "canvas".to_string(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 12.5, y: -4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None },
            ],
            ui: Some(crate::os_spr::PresenceUi { hovered_path: Some("row[2]#t1".to_string()), focused_path: None, pressed_path: None }),
        }
    }
    //#endregion 🧪️WireBridge

    // 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
    // `assemble_presence_interaction` and its `🧪️PresenceInteraction` tests MOVED to
    // `crate::os_spr::wire`'s `🔖️PresenceInteraction` region (`assemble_presence_interaction_tests`
    // module there) alongside the function itself — see this file's `🔖️WireBridge` region for the
    // pointer left behind.

    //#region 🧪️PresenceInteraction
    #[semio_framework_async_macros::async_test]
    async fn presence_heartbeat_producer_publishes_immediately_then_coalesces_to_latest() {
        let mut producer = PresenceHeartbeatProducer::new(100);
        let mut first = sample_presence_peer_with_interaction().await;
        first.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 1.0, y: 2.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
        assert_eq!(producer.offer(1_000, first.clone()), Some(first));

        let mut intermediate = sample_presence_peer_with_interaction().await;
        intermediate.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 3.0, y: 4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
        assert_eq!(producer.offer(1_040, intermediate), None);

        let mut latest = sample_presence_peer_with_interaction().await;
        latest.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 5.0, y: 6.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
        assert_eq!(producer.offer(1_099, latest.clone()), None);
        assert_eq!(producer.pending(), Some(&latest));
        assert_eq!(producer.offer(1_100, latest.clone()), Some(latest));
        assert!(producer.pending().is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn artifact_host_presence_heartbeat_owns_cadence_per_document() {
        let host = ArtifactHost::new(test_pool());
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (events, _) = broadcast::channel(1);
        let runner = native_actor::retained_turn_fixtures::fixture_runner_handle(host.pool.clone(), 1, cmd_rx.close_handle());
        host.inner.lock().unwrap().documents.insert(ArtifactDocumentKey::local("doc"), OpenDocument { generation: 1, cancel: semio_framework_async::CancelToken::root_now(), cmd_tx, events, presence: PresenceHeartbeatProducer::default(), runner });

        let first = sample_presence_peer_with_interaction().await;
        assert!(host.presence_heartbeat("doc", 500, first.clone()));
        assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == first));

        let mut latest = sample_presence_peer_with_interaction().await;
        latest.ui = Some(crate::os_spr::PresenceUi { hovered_path: Some("row[0]#changed".into()), focused_path: None, pressed_path: None });
        assert!(!host.presence_heartbeat("doc", 550, latest.clone()));
        assert!(cmd_rx.try_recv().is_none(), "sub-interval offer must not publish");
        assert!(host.presence_heartbeat("doc", 600, latest.clone()));
        assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == latest));
        assert!(!host.presence_heartbeat("missing", 700, sample_presence_peer_with_interaction().await));
    }
    //#endregion 🧪️PresenceInteraction

    //#region 🧪️Helpers

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn op_envelope_from_stored_edit_round_trips_through_ingest() {
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
        let envelopes = envelopes_from_history_edit(&edit, "demo", "demo/v1").await.expect("envelopes from history edit");
        assert_eq!(envelopes.len(), 1, "single-op edit yields one envelope");
        assert_eq!(envelopes[0].mutation_id.0, "ext-1#0", "meta-less fallback: edit id # op index");
        let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).expect("decode op");
        assert_eq!(recovered, DemoMutation::SetN { n: 42 });
    }
    //#endregion 🧪️Helpers

    //#region 🧪️Actor
    #[cfg(not(target_arch = "wasm32"))]
    mod actor_tests {
        use super::*;
        use crate::os_spr::{decode_client_frame, encode_server_frame};
        use futures::{SinkExt, StreamExt};
        use std::sync::Arc;
        use std::time::Duration;
        use tokio::sync::{broadcast as tokio_broadcast, Mutex};
        use tokio_tungstenite::tungstenite::Message as WsMessage;

        async fn demo_envelope(document_id: &str) -> crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> {
            create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None)
        }

        async fn wait_until<Fut>(label: &str, mut predicate: impl FnMut() -> Fut)
        where
            Fut: std::future::Future<Output = bool>,
        {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            while !predicate().await {
                if tokio::time::Instant::now() >= deadline {
                    panic!("{label} not satisfied before 5s deadline");
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }

        async fn wait_until_value<T, Fut>(label: &str, mut predicate: impl FnMut() -> Fut) -> T
        where
            Fut: std::future::Future<Output = Option<T>>,
        {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                if let Some(value) = predicate().await {
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
                    Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                    other => panic!("no matching event before deadline: {other:?}"),
                }
            }
        }

        // 🔬️ External folder edit → RemoteMutations event + the store timeline grows on tick().
        #[tokio::test]
        async fn folder_external_edit_delivers_remote_operations() {
            ensure_demo_codec_registered().await;
            let dir = crate::os_store::test_support::tempdir().expect("tempdir");
            let host = ArtifactHost::new(test_pool());
            let channels = host.open(ArtifactActorConfig { document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
            let mut events = host.subscribe("doc-a").await;
            let mut store = ArtifactStore::new(demo_envelope("doc-a").await).await.expect("valid actor fixture");
            store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");

            // A local apply establishes a persisted edit on disk.
            store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply");
            channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

            // Wait until the actor has persisted the local edit to the folder db as real pack+spr bytes.
            let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
            let (pack, spr) = wait_until_value("persisted edit on disk", || async {
                let (pack, spr) = storage.read("doc-a").await.expect("read")?;
                if spr_op_ids(&spr).await.ok()?.is_empty() {
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
            let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &[external_edit]).await.expect("append external edit");
            storage.write("doc-a", "demo/v1", &pack, &new_spr).await.expect("out-of-band write");

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
            store.tick().await.expect("tick");
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
            log: Arc<Mutex<Vec<(u64, MutationEnvelope)>>>,
            broadcast: tokio_broadcast::Sender<ServerFrame>,
        }

        async fn mock_frontier(ordinal: u64) -> crate::os_spr::RuntimeFrontierSummary {
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
                Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes).await {
                    Ok((_, ClientFrame::SocketHelloV1 { frontier, .. })) => frontier.map_or(0, |frontier| frontier.head_edit_ordinal),
                    _ => return,
                },
                _ => return,
            };
            let (frontier, backlog) = {
                let log = semio_hub.log.lock().await;
                let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
                let backlog: Vec<MutationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
                (mock_frontier(ordinal).await, backlog)
            };
            let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
            if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).await.into())).await.is_err() {
                return;
            }
            if !backlog.is_empty() {
                let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("semio_hub-backlog".to_string()), frontier: frontier.clone() };
                if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).await.into())).await.is_err() {
                    return;
                }
            }
            let mut broadcast_rx = semio_hub.broadcast.subscribe();
            loop {
                tokio::select! {
                    incoming = read.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Binary(bytes))) => {
                                match decode_client_frame(&bytes).await {
                                    Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                        let mut assigned_frontier = frontier.clone();
                                        for envelope in envelopes {
                                            let (ordinal, origin) = {
                                                let mut log = semio_hub.log.lock().await;
                                                let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                                log.push((next, envelope.clone()));
                                                (next, envelope.actor.clone())
                                            };
                                            assigned_frontier = mock_frontier(ordinal).await;
                                            let _ = semio_hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                        }
                                        let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                        let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).await.into())).await;
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
                                if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).await.into())).await.is_err() {
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

            let host_a = ArtifactHost::new(test_pool());
            let channels_a = host_a
                .open(ArtifactActorConfig {
                    document_id: "shared".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "A".into(),
                })
                .await;
            let mut store_a = ArtifactStore::new(demo_envelope("shared").await).await.expect("valid shared actor A fixture");
            let key_a = channels_a.document_key.clone();
            store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");

            let host_b = ArtifactHost::new(test_pool());
            let channels_b = host_b
                .open(ArtifactActorConfig {
                    document_id: "shared".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "B".into(),
                })
                .await;
            let key_b = channels_b.document_key.clone();
            let mut events_b = host_b.subscribe_key(&key_b).await;
            let mut store_b = ArtifactStore::new(demo_envelope("shared").await).await.expect("valid shared actor B fixture");
            store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

            // Give both actors time to connect + Hello.
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).await.expect("apply on a");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            match event {
                ArtifactEvent::RemoteMutations { envelopes } => assert_eq!(envelopes.len(), 1),
                other => panic!("expected RemoteMutations on B, got {other:?}"),
            }
            store_b.tick().await.expect("tick b");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 7, "B converged on A's operation");

            host_a.close_key(&key_a);
            host_b.close_key(&key_b);
        }

        // 🔬️ Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
        // its Welcome backlog carries only the operations it missed.
        #[tokio::test]
        async fn reconnect_since_catch_up_replays_backlog() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = ArtifactHost::new(test_pool());
            let channels_a = host_a
                .open(ArtifactActorConfig {
                    document_id: "catchup".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "A".into(),
                })
                .await;
            let mut store_a = ArtifactStore::new(demo_envelope("catchup").await).await.expect("valid catchup actor A fixture");
            let key_a = channels_a.document_key.clone();
            store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            // A applies two operations while nobody else is connected.
            for n in [3, 4] {
                store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n }], description: None }).await.expect("apply on a");
                channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
                tokio::time::sleep(Duration::from_millis(80)).await;
            }

            // B connects fresh (since_version 0) and its Welcome backlog replays both operations.
            let host_b = ArtifactHost::new(test_pool());
            let channels_b = host_b
                .open(ArtifactActorConfig {
                    document_id: "catchup".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "B".into(),
                })
                .await;
            let key_b = channels_b.document_key.clone();
            let mut events_b = host_b.subscribe_key(&key_b).await;
            let mut store_b = ArtifactStore::new(demo_envelope("catchup").await).await.expect("valid catchup actor B fixture");
            store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            if let ArtifactEvent::RemoteMutations { envelopes } = event {
                assert_eq!(envelopes.len(), 2, "backlog replays both missed operations");
            }
            store_b.tick().await.expect("tick b");
            assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);

            host_a.close_key(&key_a);
            host_b.close_key(&key_b);
        }

        // 🔬️ Detach drains the outbox: an operation applied right before close still reaches the semio_hub (and B).
        #[tokio::test]
        async fn detach_drains_pending_outbound_operations() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            // Observer B stays connected to witness A's last operation.
            let host_b = ArtifactHost::new(test_pool());
            let channels_b = host_b
                .open(ArtifactActorConfig {
                    document_id: "drain".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "B".into(),
                })
                .await;
            let key_b = channels_b.document_key.clone();
            let mut events_b = host_b.subscribe_key(&key_b).await;
            let mut store_b = ArtifactStore::new(demo_envelope("drain").await).await.expect("valid drain actor B fixture");
            store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

            let host_a = ArtifactHost::new(test_pool());
            let channels_a = host_a
                .open(ArtifactActorConfig {
                    document_id: "drain".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "A".into(),
                })
                .await;
            let mut store_a = ArtifactStore::new(demo_envelope("drain").await).await.expect("valid drain actor A fixture");
            let key_a = channels_a.document_key.clone();
            store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).await.expect("apply on a");
            // Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
            host_a.close_key(&key_a);

            let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
            if let ArtifactEvent::RemoteMutations { envelopes } = event {
                assert_eq!(envelopes.len(), 1, "the operation applied before detach was not lost");
            }
            store_b.tick().await.expect("tick b");
            assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
            host_b.close_key(&key_b);
        }

        // 🔬️ The mock semio_hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
        // `ArtifactEvent::CommandOutcome` wiring actually fires (not just that it compiles).
        #[tokio::test]
        async fn command_outcome_accepted_fires_after_hub_ack() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");
            let host = ArtifactHost::new(test_pool());
            let channels = host
                .open(ArtifactActorConfig {
                    document_id: "outcome".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "A".into(),
                })
                .await;
            let key = channels.document_key.clone();
            let mut events = host.subscribe_key(&key).await;
            let mut store = ArtifactStore::new(demo_envelope("outcome").await).await.expect("valid outcome actor fixture");
            store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply");
            channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

            let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await;
            match event {
                ArtifactEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
                other => panic!("expected CommandOutcome, got {other:?}"),
            }
            host.close_key(&key);
        }

        // 🔬️ `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock semio_hub's
        // preview-lane fan-out -> `ServerFrame::Preview` -> `ArtifactEvent::Preview` on another peer.
        #[tokio::test]
        async fn publish_preview_round_trips_through_hub() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = ArtifactHost::new(test_pool());
            let channels_a = host_a
                .open(ArtifactActorConfig {
                    document_id: "preview".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "A".into(),
                })
                .await;

            let host_b = ArtifactHost::new(test_pool());
            let channels_b = host_b
                .open(ArtifactActorConfig {
                    document_id: "preview".into(),
                    schema: "demo/v1".into(),
                    bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }],
                    watch_external: false,
                    actor: "B".into(),
                })
                .await;
            let key_a = channels_a.document_key.clone();
            let key_b = channels_b.document_key.clone();
            let mut events_b = host_b.subscribe_key(&key_b).await;
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
            host_a.close_key(&key_a);
            host_b.close_key(&key_b);
        }

        // 🔬️ Shared fixtures replay: each fixture's inbound stimuli produce the expected ArtifactEvent
        // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
        #[tokio::test]
        async fn fixtures_replay_matches_expected_events() {
            let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("🧫️fixtures");
            let fixtures = load_fixtures(&fixtures_dir).await;
            assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
            for fixture in fixtures {
                replay_fixture(&fixture).await;
            }
        }

        async fn replay_fixture(fixture: &ActorFixture) {
            ensure_demo_codec_registered().await;
            let codec = crate::os_store::document_codec(&fixture.schema).await.expect("codec registry available").unwrap_or_else(|| panic!("no codec registered for fixture schema {:?}", fixture.schema));
            let dir = crate::os_store::test_support::tempdir().expect("tempdir");
            let host = ArtifactHost::new(test_pool());
            let channels = host
                .open(ArtifactActorConfig { document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() })
                .await;
            let mut events = host.subscribe(&fixture.document_id).await;
            let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None)).await.expect("valid fixture store");
            store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
            let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
            wait_until(&format!("seed snapshot for {} on disk", fixture.document_id), || async { storage.read(&fixture.document_id).await.expect("read").is_some() }).await;

            // Lockstep: apply each stimulus, then wait for its paired expected event before the next
            // (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
            assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
            let mut observed: Vec<String> = Vec::new();
            for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
                match inbound {
                    FixtureInbound::ExternalEdits { ops_text } => {
                        let (pack, spr) = storage.read(&fixture.document_id).await.expect("read").expect("some");
                        let parsed = crate::os_spr::parse_ops_text(ops_text).unwrap_or_else(|error| panic!("fixture {} parse_ops_text: {error}", fixture.name));
                        let mut new_edits: Vec<crate::os_spr::HistoryEdit> = Vec::new();
                        for edit in parsed.edits {
                            let mut ops: Vec<crate::os_spr::OpPayload> = Vec::new();
                            for op in &edit.ops {
                                let text = op.text.as_deref().unwrap_or_else(|| panic!("fixture {} op line has no text", fixture.name));
                                let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
                                ops.push(crate::os_spr::OpPayload { text: None, binary: Some(concrete.encode_op().expect("encode demo op")) });
                            }
                            new_edits.push(crate::os_spr::HistoryEdit { ops, meta: None, ..edit });
                        }
                        let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &new_edits).await.expect("append fixture edits");
                        storage.write(&fixture.document_id, &fixture.schema, &pack, &new_spr).await.expect("write");
                        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::ReplaceDocument { dsl_text, ops_text } => {
                        let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).await.unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
                        storage.write(&fixture.document_id, &fixture.schema, &pack_files.pack, &pack_files.spr).await.expect("replace write");
                        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::HubFrame { .. } => {
                        panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                    }
                }
                let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
                observed.push(document_event_tag(&event).to_string());
                store.tick().await.expect("tick");
            }
            assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
            let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            for expected_id in &fixture.expected_edit_ids {
                assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
            }
            host.close(&fixture.document_id);
        }

        // 🚫️async: E1-adjacent — pure match with no suspension point, consumed by
        // `wait_for_event`'s sync `FnMut(&ArtifactEvent) -> bool` predicate bound — see R9.
        fn document_event_tag(event: &ArtifactEvent) -> &'static str {
            match event {
                ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
                ArtifactEvent::SnapshotReplaced { .. } => "snapshotReplaced",
                ArtifactEvent::BootstrapProgress { .. } => "bootstrapProgress",
                ArtifactEvent::Status(_) => "status",
                ArtifactEvent::Presence { .. } => "presence",
                ArtifactEvent::Session { .. } => "session",
                ArtifactEvent::Preview { .. } => "preview",
                ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
                ArtifactEvent::Conflict(_) => "conflict",
            }
        }
    }
    //#endregion 🧪️Actor

    /// @emoji 🎯️ `FolderEventLogStorage` is a pure `(pack, spr)` event store — schema-agnostic,
    /// no JSON/codec involvement at this layer (that lives one level up, in `FolderEndpoint`, tested
    /// via `folder_external_edit_delivers_remote_operations`). This test exercises exactly the
    /// storage mechanics: per-id folding, append-only replacement, and the folder-wide index.
    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn folder_event_log_storage_round_trips_by_document_id() {
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("doc-a").await.expect("read empty"), None, "absent document reads as None");

        storage.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("write a");
        storage.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("write b");
        assert_eq!(storage.read("doc-a").await.expect("read a").expect("some a"), (b"pack-a".to_vec(), b"spr-a".to_vec()), "documents are keyed independently");
        assert_eq!(storage.read("doc-b").await.expect("read b").expect("some b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));

        storage.write("doc-a", "demo/v1", b"pack-a2", b"spr-a2").await.expect("upsert a");
        assert_eq!(storage.read("doc-a").await.expect("reread a").expect("some a2"), (b"pack-a2".to_vec(), b"spr-a2".to_vec()), "the latest snapshot event replaces the projection");

        let mut ids = storage.document_ids().await.expect("document ids");
        ids.sort();
        assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
    }

    /// @emoji 🔐️ The endpoint-level save→load→undo proof: a store's undo/redo position survives a
    /// full write/read cycle through the ACTUAL `FolderEventLogStorage` byte storage (`store`'s own
    /// `save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position` proves the pure
    /// in-memory pack/spr encoding; this proves the folder persistence layer built on top of it).
    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn folder_event_log_storage_round_trips_undo_position_through_pack_spr() {
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());

        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None)).await.expect("valid folder store");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply e1");
        let post_e1 = store.snapshot().expect("post-e1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply e2");
        store.dispatch(ArtifactCommand::Undo).await.expect("undo e2");
        assert_eq!(store.snapshot().expect("live"), post_e1, "precondition: live store is back at post-e1");

        let files = print_document_pack(store.envelope()).await.expect("print document pack");
        storage.write("doc-a", "demo/v1", &files.pack, &files.spr).await.expect("write");

        let (pack, spr) = storage.read("doc-a").await.expect("read").expect("some");
        let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack, &spr).await.unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1 through the folder storage layer");
        let mut reloaded = ArtifactStore::new(parsed.envelope).await.expect("valid reloaded history");
        assert_eq!(reloaded.snapshot().expect("reloaded"), post_e1);

        reloaded.dispatch(ArtifactCommand::Redo).await.expect("redo e2 after folder reload");
        assert_eq!(reloaded.snapshot().expect("post-redo"), DemoSnapshot { n: 2 });
    }

    /// @emoji 🎯️ Seeds the write from a ZERO-edit envelope (no cursor line — a cursor is only
    /// synced once an edit is dispatched, see `ArtifactStore::sync_cursor`) so both edits are then
    /// added purely via the raw `append_ops` hot path with no cursor line ever written; a cursor
    /// pinned to an earlier edit count would otherwise cap the reconstructed snapshot at that
    /// edit (see `document_text_round_trips_a_cursor_after_undo_then_apply_interleaving` in
    /// `store`'s own test suite for that law, exercised correctly there).
    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn folder_text_storage_round_trips_dsl_and_appends_ops() {
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
        assert_eq!(storage.read("demo", "demo").await.expect("read empty"), None, "absent document reads as None");

        let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid zero-edit text fixture");
        let files = print_document_text(seed.envelope()).await.expect("print document text");
        storage.write("demo", "demo", &files).await.expect("write");

        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid text fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply 1");
        let first_edit = store.envelope().vcs.edits.last().expect("first edit");
        storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

        let reloaded = storage.read("demo", "demo").await.expect("read").expect("some");
        let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded.dsl, &reloaded.ops).await.unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.snapshot.n, 2, "write + append reconstructs every edit in order");

        assert_eq!(storage.document_ids("demo").await.expect("document ids"), vec!["demo".to_string()]);
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
    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn folder_text_storage_round_trips_pack() {
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
        assert_eq!(storage.read_pack("demo", "demo").await.expect("read empty"), None, "absent pack reads as None");

        let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack fixture");
        let files = print_document_pack(seed.envelope()).await.expect("print document pack");
        let dsl_mirror = seed.envelope().vcs.initial_snapshot.print_dsl();
        storage.write_pack("demo", "demo", &files, &dsl_mirror).await.expect("write pack");

        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack append fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply 1");
        let first_edit = store.envelope().vcs.edits.last().expect("first edit");
        storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

        // Text mirror is current (both edits landed via append_ops).
        let reloaded_text = storage.read("demo", "demo").await.expect("read text").expect("some text");
        let parsed_text: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded_text.dsl, &reloaded_text.ops).await.unwrap_or_else(|error| panic!("parse text: {error}"));
        assert_eq!(parsed_text.snapshot.n, 2, "the .ops text mirror reflects every appended edit");

        // pack+spr are unaffected by ops-text-only appends — still the zero-edit snapshot from
        // the initial write_pack, proving read_pack/parse_document_pack never reads .ops.
        let reloaded_pack = storage.read_pack("demo", "demo").await.expect("read pack").expect("some pack");
        let parsed_pack: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack.pack, &reloaded_pack.spr).await.unwrap_or_else(|error| panic!("parse pack: {error}"));
        assert_eq!(parsed_pack.snapshot.n, 0, "pack+spr are authoritative and independent of ops-text-only appends");

        // A fresh whole-file write_pack (the actual cold-path persistence flow) brings pack+spr
        // current with the live store.
        let files2 = print_document_pack(store.envelope()).await.expect("print document pack 2");
        let dsl_mirror2 = store.envelope().vcs.initial_snapshot.print_dsl();
        storage.write_pack("demo", "demo", &files2, &dsl_mirror2).await.expect("write pack 2");
        let reloaded_pack2 = storage.read_pack("demo", "demo").await.expect("read pack 2").expect("some pack 2");
        let parsed_pack2: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack2.pack, &reloaded_pack2.spr).await.unwrap_or_else(|error| panic!("parse pack 2: {error}"));
        assert_eq!(parsed_pack2.snapshot.n, 2, "a fresh write_pack brings pack+spr current with the live store");

        // The always-written DSL mirror must also be on disk and agree with the initial-snapshot.
        let mirror = std::fs::read_to_string(storage.pack_path("demo", "demo").await.with_extension("")).expect("dsl mirror on disk");
        assert_eq!(DemoSnapshot::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn blob_store_put_get_dedupes_idempotently() {
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        let bytes = b"hello content-addressed world";
        assert!(!storage.has("not-a-real-hash").await.expect("has on empty store"));

        let first = storage.put(bytes, "text/plain").await.expect("first put");
        let second = storage.put(bytes, "text/plain").await.expect("second put");
        assert_eq!(first, second, "putting identical bytes twice is idempotent and dedupes by hash");
        assert_eq!(first.size, bytes.len() as u64);
        assert_eq!(first.media_type, "text/plain");

        assert!(storage.has(&first.hash).await.expect("has after put"));
        let fetched = storage.get(&first.hash).await.expect("get").expect("blob present");
        assert_eq!(fetched, bytes);

        let other = storage.put(b"different content", "text/plain").await.expect("put other");
        assert_ne!(other.hash, first.hash, "different bytes hash differently");

        storage.delete(&first.hash).await.expect("delete");
        assert!(!storage.has(&first.hash).await.expect("has after delete"));
        assert_eq!(storage.get(&first.hash).await.expect("get after delete"), None);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[semio_framework_async_macros::async_test]
    async fn folder_event_log_ignores_an_incomplete_tail_and_coordinates_handles() {
        use std::io::Write;

        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let first = FolderEventLogStorage::new(dir.path().to_path_buf());
        let second = FolderEventLogStorage::new(dir.path().to_path_buf());
        first.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("first handle write");
        second.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("second handle write");

        let mut file = std::fs::OpenOptions::new().append(true).open(first.event_path()).expect("open event log");
        file.write_all(&FOLDER_EVENT_MAGIC[..3]).expect("partial crash tail");
        file.sync_data().expect("sync crash tail");

        assert_eq!(first.read("doc-a").await.expect("read a").expect("doc a"), (b"pack-a".to_vec(), b"spr-a".to_vec()));
        assert_eq!(second.read("doc-b").await.expect("read b").expect("doc b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));
    }
}
