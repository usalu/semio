//! 🔗️ Hot-swappable backbones: the backbone message protocol, memory, channel and port backbones and their registry.
//! The engine attaches them; their retirement lives beside them in `♻️retirement`.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use crate::os_spr::{DiffRegions, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use crate::os_vcs::VcsError;
use crate::os_vcs::{
    Identified, Patchable,
};
use super::store_schema::{ArtifactBackboneRef, document_backbone_ref};
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use crate::os_pack::pack_rt;
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree,
};

/// 📨️ Wire message exchanged over an attached backbone channel. B-R6 "kill hand-rolled binary
/// codecs": `#[derive(crate::os_dsl::DslOps)]` generates `OpBinary::encode_op`/`decode_op` (`format u8 (=1) |
/// variant ordinal varint | record body`, `crate::os_dsl::op_rt`) — this is the one real binary encoding for
/// every caller, including the wasm-sandbox `BackboneChannelPort` seam (see that trait's doc) — the
/// WIT `backbone-send`/`backbone-poll` host functions carry these exact bytes as `list<u8>`.
/// `Operations.envelopes` carries `crate::os_spr::encode_envelopes`/`decode_envelopes` bytes rather than a
/// real `Vec<crate::os_spr::MutationEnvelope>` field: `MutationEnvelope` lives in `protocol_causal`,
/// which sits BELOW `dsl` in the dependency graph (`dsl` → `protocol` → `protocol_causal`), so it
/// cannot implement `crate::os_dsl::DslField` without a dependency cycle — the derive can only lower fields
/// shaped from types it can see. `#[dsl(base64)]` marks each `Vec<u8>` field `Shape::Bytes64`
/// (otherwise a bare `Vec<u8>` lowers to a `List<UInt>`, one DSL list element per byte).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum BackboneMessage {
    /// 🧬️ The document's initial snapshot pack — its identity, never its current state. A
    /// replica announces it before its event log; an empty persistence end adopts it as the genesis
    /// it stores, every other end verifies it names the same document.
    Genesis {
        #[dsl(base64)]
        pack: Vec<u8>,
    },
    /// ✉️ Semantic events — edit operations and history transitions — as causal envelopes.
    Mutations {
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
    /// ✅️ Acknowledges inbound operations the store has ingested (store→actor). Lets a future actor
    /// implement at-least-once redelivery with id-based dedupe — safe across store crashes/reloads.
    Ack { op_ids: Vec<String> },
    /// 🪆️ Semantic events authored on ONE composed member of this document, carrying that
    /// member's exact lane identity: the owner-path text of the member that owns it (`owner`, empty for the document
    /// itself; design §21.9) and its `(slot, child_id)` step there. A composed document has exactly one replica
    /// endpoint — the parent's backbone — so a child lane's edits can only cross under this tag, and
    /// the receiving replica folds them into ITS own member of the same lane, never into the parent.
    Member {
        owner: String,
        slot: String,
        child_id: String,
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
    /// 🔙️ Retracts history transitions this replica authored and the hub refused (actor→store): each leaves the store's
    /// event log with every local transition causally depending on it ([`transition_retraction_closure`]), and the
    /// projection follows — the author converges with the hub, which never took them. No inverse exists for a
    /// transition; a retraction is the refusal's exact compensation.
    Retract { mutation_ids: Vec<String> },
}

/// 🎞️ Handcrafted OpText (P6).
impl OpText for BackboneMessage {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for BackboneMessage {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::decode_op(bytes)
    }
}

pub const BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES: usize = crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES;

pub const BACKBONE_GENESIS_MESSAGE_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;

pub const BACKBONE_CHANNEL_MAXIMUM_MESSAGES: usize = crate::os_spr::DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES;

pub const BACKBONE_CHANNEL_MAXIMUM_BYTES: usize = crate::os_spr::DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES;

pub fn decode_backbone_message_exact(bytes: &[u8]) -> Result<BackboneMessage, VcsError> {
    if bytes.len() > BACKBONE_GENESIS_MESSAGE_MAXIMUM_BYTES {
        return Err(VcsError::Deserialize("backbone message exceeds its fixed byte cap".into()));
    }
    BackboneMessage::decode_op(bytes).map_err(|error| VcsError::Deserialize(error.to_string()))
}

pub fn decode_hot_backbone_message_exact(bytes: &[u8]) -> Result<BackboneMessage, VcsError> {
    if bytes.len() > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES {
        return Err(VcsError::Deserialize("hot backbone message exceeds its fixed byte cap".into()));
    }
    let message = decode_backbone_message_exact(bytes)?;
    if matches!(message, BackboneMessage::Genesis { .. }) {
        return Err(VcsError::ValidationFailed("hot backbone transport refuses genesis packs; a hot replica is bootstrapped through the cold document pair".into()));
    }
    Ok(message)
}

/// 🧵️ Non-blocking, IO-free in-memory queue contract between a `ArtifactStore` and its
/// sync actor. `send`/`receive` MUST return immediately: implementations only enqueue/dequeue
/// `BackboneMessage`s — never HTTP, never filesystem, never a blocking wait. All IO (persistence,
/// semio_hub sync, file watching, presence) lives behind this queue in `framework/sync`'s actor layer,
/// which owns the other end; the store's `pump()`/`flush_outbound()` run synchronously on the
/// caller's thread and must never be blocked by transport work.
///
/// URI schemes are resolved by the host actor (`framework/sync`): `temp://` (in-memory),
/// `file://` (single JSON blob), `folder://` (sqlite `.semio/document.db`), `remote://` (OS semio_hub).
// 🧵️ No `Send`/`Sync` supertrait bound (R7 coordinator ruling 2026-08-19): this crate is
// guest-reachable and Send-ness comes structurally from the concrete `Backbones` enum at each spawn
// site, never from a bound named here.
pub trait Backbone {
    async fn descriptor(&self) -> ArtifactBackboneRef;
    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError>;
    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

pub(crate) static HOST_BACKBONE_PORT: Mutex<Option<Arc<BackbonePorts>>> = Mutex::new(None);

/// 🔌️ Injects the browser or dev-server backbone port for wasm file/folder IO.
pub async fn set_host_backbone_port(port: Arc<BackbonePorts>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

pub(crate) async fn host_backbone_port() -> Option<Arc<BackbonePorts>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub async fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.get(uri).cloned().ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub(crate) async fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// 💾️ Browser `localStorage` backbone port with in-memory fallback for native tests.
#[derive(Default)]
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub async fn new() -> Self {
        Self { fallback: MemoryBackbonePort::new().await }
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port().await {
            if let Ok(value) = Box::pin(port.read(uri)).await {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri).await) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri).await
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload).await?;
        if let Some(port) = host_backbone_port().await {
            let _ = Box::pin(port.write(uri, payload)).await;
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(&local_storage_backbone_key(uri).await, payload);
                }
            }
        }
        Ok(())
    }
}

/// 🧬️ Enum dispatch over every `BackbonePort` implementor (O1 — dyn dispatch is dropped
/// repo-wide in favor of enum/generated dispatch). The trait stays as the contract; this enum
/// implements it by match-delegation so 🪐️space's blanket `impl<T: BackbonePort>
/// SpaceBackbonePort for T` keeps covering every concrete port through the enum too.
pub enum BackbonePorts {
    Memory(MemoryBackbonePort),
    LocalStorage(LocalStorageBackbonePort),
}

impl BackbonePort for BackbonePorts {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        match self {
            Self::Memory(port) => port.read(uri).await,
            Self::LocalStorage(port) => port.read(uri).await,
        }
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        match self {
            Self::Memory(port) => port.write(uri, payload).await,
            Self::LocalStorage(port) => port.write(uri, payload).await,
        }
    }
}

/// 🕸️ Injectable duplex transport across the wasm sandbox boundary (program ↔ host process).
/// `message`/the `poll` result are `BackboneMessage::encode_op`/`decode_op` (`crate::os_spr::OpBinary`) bytes.
pub trait BackboneChannelPort: Send + Sync {
    async fn send(&self, uri: &str, message: &[u8]) -> Result<(), VcsError>;
    async fn poll(&self, uri: &str) -> Result<Vec<Vec<u8>>, VcsError>;
}

pub(crate) struct ActorBackboneChannelState {
    uri: String,
    inbound: VecDeque<Vec<u8>>,
    outbound: VecDeque<Vec<u8>>,
    inbound_bytes: usize,
    outbound_bytes: usize,
    accepting: bool,
}

pub struct ActorBackboneChannelPort {
    state: Arc<Mutex<ActorBackboneChannelState>>,
}

#[derive(Clone)]
pub struct ActorBackboneChannelOwner {
    state: Arc<Mutex<ActorBackboneChannelState>>,
}

pub(crate) fn push_actor_backbone_outbound(state: &Arc<Mutex<ActorBackboneChannelState>>, uri: &str, message: &[u8]) -> Result<(), VcsError> {
    decode_hot_backbone_message_exact(message)?;
    let mut state = state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
    if !state.accepting || state.uri != uri {
        return Err(VcsError::Backbone("actor backbone channel is retired or addressed to another URI".into()));
    }
    let next_bytes = state.outbound_bytes.checked_add(message.len()).ok_or_else(|| VcsError::Backbone("actor backbone outbound byte accounting overflow".into()))?;
    if state.outbound.len() >= BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > BACKBONE_CHANNEL_MAXIMUM_BYTES {
        return Err(VcsError::Backbone("actor backbone outbound queue is saturated".into()));
    }
    state.outbound.push_back(message.to_vec());
    state.outbound_bytes = next_bytes;
    Ok(())
}

impl ActorBackboneChannelOwner {
    pub fn pair(uri: &str) -> (Arc<BackboneChannelPorts>, Self) {
        let state = Arc::new(Mutex::new(ActorBackboneChannelState { uri: uri.to_string(), inbound: VecDeque::new(), outbound: VecDeque::new(), inbound_bytes: 0, outbound_bytes: 0, accepting: true }));
        (Arc::new(BackboneChannelPorts::Actor(ActorBackboneChannelPort { state: state.clone() })), Self { state })
    }

    pub fn admits_same_channel(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }

    pub fn push_inbound(&self, uri: &str, message: &[u8]) -> Result<(), VcsError> {
        decode_hot_backbone_message_exact(message)?;
        let mut state = self.state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
        if !state.accepting || state.uri != uri {
            return Err(VcsError::Backbone("actor backbone channel is retired or addressed to another URI".into()));
        }
        let next_bytes = state.inbound_bytes.checked_add(message.len()).ok_or_else(|| VcsError::Backbone("actor backbone inbound byte accounting overflow".into()))?;
        if state.inbound.len() >= BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > BACKBONE_CHANNEL_MAXIMUM_BYTES {
            return Err(VcsError::Backbone("actor backbone inbound queue is saturated".into()));
        }
        state.inbound.push_back(message.to_vec());
        state.inbound_bytes = next_bytes;
        Ok(())
    }

    pub fn take_outbound(&self) -> Result<Option<Vec<u8>>, VcsError> {
        let mut state = self.state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
        let message = state.outbound.pop_front();
        if let Some(message) = &message {
            state.outbound_bytes = state.outbound_bytes.saturating_sub(message.len());
        }
        Ok(message)
    }

    #[cfg(test)]
    pub fn push_outbound_for_test(&self, uri: &str, message: &[u8]) -> Result<(), VcsError> {
        push_actor_backbone_outbound(&self.state, uri, message)
    }

    pub fn begin_retire(&self) -> Result<(), VcsError> {
        let mut state = self.state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
        state.accepting = false;
        state.inbound.clear();
        state.outbound.clear();
        state.inbound_bytes = 0;
        state.outbound_bytes = 0;
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> Result<bool, VcsError> {
        let state = self.state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
        Ok(!state.accepting && state.inbound.is_empty() && state.outbound.is_empty())
    }
}

pub enum BackboneChannelPorts {
    Actor(ActorBackboneChannelPort),
}

impl BackboneChannelPort for BackboneChannelPorts {
    async fn send(&self, uri: &str, message: &[u8]) -> Result<(), VcsError> {
        match self {
            Self::Actor(port) => push_actor_backbone_outbound(&port.state, uri, message),
        }
    }

    async fn poll(&self, uri: &str) -> Result<Vec<Vec<u8>>, VcsError> {
        match self {
            Self::Actor(port) => {
                let mut state = port.state.lock().map_err(|_| VcsError::Backbone("actor backbone channel lock poisoned".into()))?;
                if !state.accepting || state.uri != uri {
                    return Err(VcsError::Backbone("actor backbone channel is retired or addressed to another URI".into()));
                }
                state.inbound_bytes = 0;
                Ok(state.inbound.drain(..).collect())
            }
        }
    }
}

/// 🧵️ Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side. The
/// channel is injected per instance via [`PortBackbone::with_channel`] — a pooled multi-instance
/// actor cannot share one process-global channel (see `important.md`'s "Replace, never wrap" list,
/// `set_host_backbone_channel`). No caller constructs a real channel yet (A2, `🔌️plugin/🦀️.rs`'s
/// `PLUGIN_INIT_ONCE` doc comment: the async `EffectBackbone` bridge is deferred work), so
/// [`PortBackbone::new`] leaves it unset and `send`/`receive` surface a real "no host backbone
/// linked" error rather than silently no-op'ing.
pub struct PortBackbone {
    pub(crate) uri: String,
    pub(crate) channel: Option<Arc<BackboneChannelPorts>>,
}

impl PortBackbone {
    pub async fn new(uri: &str) -> Self {
        Self { uri: uri.to_string(), channel: None }
    }

    pub async fn with_channel(uri: &str, channel: Arc<BackboneChannelPorts>) -> Self {
        Self { uri: uri.to_string(), channel: Some(channel) }
    }
}

impl Backbone for PortBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = self.channel.as_ref().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let bytes = message.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))?;
        channel.send(&self.uri, &bytes).await
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = self.channel.as_ref().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let mut messages = Vec::new();
        for bytes in channel.poll(&self.uri).await? {
            messages.push(decode_hot_backbone_message_exact(&bytes)?);
        }
        Ok(messages)
    }
}

/// 🔗️ Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    pub(crate) uri: String,
    pub(crate) inbox: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    pub(crate) outbox: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
}

impl MemoryBackbone {
    pub async fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (Self { uri: uri_a.to_string(), inbox: Some(b_to_a.clone()), outbox: Some(a_to_b.clone()) }, Self { uri: uri_b.to_string(), inbox: Some(a_to_b), outbox: Some(b_to_a) })
    }
}

impl Backbone for MemoryBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox.as_ref().ok_or_else(|| VcsError::Backbone("memory backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbox = self.inbox.as_ref().ok_or_else(|| VcsError::Backbone("memory backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbox.drain(..).collect())
    }
}

/// 🔗️ The store-side end of a pair of crossed in-memory queues. Implements the non-blocking
/// {@link Backbone} contract; the matching {@link ChannelBackboneRemote} is held by an external sync
/// actor (built in `framework/sync`, a later workstream) that pushes inbound messages and drains the
/// store's outbound ones. This crate only provides the queue plumbing — never the actor itself.
pub struct ChannelBackbone {
    pub(crate) uri: String,
    pub(crate) inbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    pub(crate) outbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    pub(crate) outbound_wake: Arc<std::sync::OnceLock<Arc<dyn Fn() + Send + Sync>>>,
}

/// 🎛️ The actor-side end paired with a {@link ChannelBackbone}: `push` delivers a message to
/// the store's inbound queue, `try_pop_front` takes one exact outbound owner. Not a
/// `Backbone` — this is the handle an IO-owning actor endpoint holds across the store boundary.
pub struct ChannelBackboneRemote {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    pub(crate) outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound_wake: Arc<std::sync::OnceLock<Arc<dyn Fn() + Send + Sync>>>,
}

impl ChannelBackbone {
    /// 🔗️ Creates a crossed pair sharing a URI: the store attaches the `ChannelBackbone`; the
    /// actor keeps the `ChannelBackboneRemote`.
    pub async fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound_wake = Arc::new(std::sync::OnceLock::new());
        (ChannelBackbone { uri: uri.to_string(), inbound: Some(inbound.clone()), outbound: Some(outbound.clone()), outbound_wake: outbound_wake.clone() }, ChannelBackboneRemote { uri: uri.to_string(), inbound, outbound, outbound_wake })
    }
}

impl Backbone for ChannelBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound.as_ref().ok_or_else(|| VcsError::Backbone("channel backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        if let Some(wake) = self.outbound_wake.get() {
            wake();
        }
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.as_ref().ok_or_else(|| VcsError::Backbone("channel backbone is closing".into()))?.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    /// 📥️ Delivers a message to the store's inbound queue (actor→store).
    pub async fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    /// 🔔️ Installs the actor's wake: every store→actor send resumes the actor that drains it,
    /// so an idle actor never sleeps on a queued outbound owner. Only the first install is kept.
    pub fn set_outbound_wake(&self, wake: Arc<dyn Fn() + Send + Sync>) {
        let _ = self.outbound_wake.set(wake);
    }

    /// 🪄️ Takes at most one store→actor message without a bulk collection or a
    /// blocking lock wait; contention preserves the exact queued owner for a later worker turn.
    pub fn try_pop_front(&self) -> Result<Option<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.try_lock().map_err(|error| match error {
            std::sync::TryLockError::WouldBlock => VcsError::Backbone("channel backbone outbound is contended".into()),
            std::sync::TryLockError::Poisoned(_) => VcsError::Backbone("lock poisoned".into()),
        })?;
        Ok(outbound.pop_front())
    }
}

/// 🧬️ Enum dispatch over every `Backbone` implementor (O1 — see `BackbonePorts`' doc comment
/// for why: every former `Box<dyn Backbone>` seam becomes `Backbones` by value, no box needed).
pub enum Backbones {
    Port(PortBackbone),
    Memory(MemoryBackbone),
    Channel(ChannelBackbone),
}

impl Backbone for Backbones {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        match self {
            Self::Port(backbone) => backbone.descriptor().await,
            Self::Memory(backbone) => backbone.descriptor().await,
            Self::Channel(backbone) => backbone.descriptor().await,
        }
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        match self {
            Self::Port(backbone) => backbone.send(message).await,
            Self::Memory(backbone) => backbone.send(message).await,
            Self::Channel(backbone) => backbone.send(message).await,
        }
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        match self {
            Self::Port(backbone) => backbone.receive().await,
            Self::Memory(backbone) => backbone.receive().await,
            Self::Channel(backbone) => backbone.receive().await,
        }
    }
}

/// 🔌️ Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub async fn resolve_backbone(uri: &str) -> Result<Backbones, VcsError> {
    Ok(Backbones::Port(PortBackbone::new(uri).await))
}
