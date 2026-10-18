//! 💾️ Storage, capabilities, backbone and metrics: the storage backend, the capability checker, the backbone transport and registry and the effect metrics recorders. See [`super`].

use super::*;

//#region 💾️StorageBackend
/// 💾️ Blocking storage transport `StorageScheduler::submit` drives — same seam discipline as
/// `HttpTransport` in `semio-framework-os-services` (this packet adds no concrete implementation;
/// see `UnwiredStorageBackend`).
// 🚫️async: R1/E0038 — `Arc<dyn StorageBackend>` (below, `AsyncEffectExecutor::storage_backend`)
// makes this an open dyn-dispatched extension point; `async fn` in a trait used via `dyn` is not
// object-safe on stable Rust. Full de-dyn (O1/R11) is a separate, repo-wide program outside this
// packet's `🔌️plugin/🖥️host` scope — reverted to the sync form this trait had before the blind
// universal-async codemod touched it (doc already called it "Blocking storage transport").
pub trait StorageBackend: Send + Sync {
    fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error>;
    fn write(&self, key: &str, bytes: &[u8]) -> Result<(), std::io::Error>;
    fn delete(&self, key: &str) -> Result<(), std::io::Error>;
}

pub struct UnwiredStorageBackend;

impl StorageBackend for UnwiredStorageBackend {
    fn read(&self, _key: &str) -> Result<Vec<u8>, std::io::Error> {
        Err(std::io::Error::other("AsyncEffectExecutor: no StorageBackend wired yet (see the packet report's honest gaps)"))
    }
    fn write(&self, _key: &str, _bytes: &[u8]) -> Result<(), std::io::Error> {
        Err(std::io::Error::other("AsyncEffectExecutor: no StorageBackend wired yet (see the packet report's honest gaps)"))
    }
    fn delete(&self, _key: &str) -> Result<(), std::io::Error> {
        Err(std::io::Error::other("AsyncEffectExecutor: no StorageBackend wired yet (see the packet report's honest gaps)"))
    }
}
//#endregion 💾️StorageBackend

//#region 🔑️CapabilityChecker
/// 🔑️ Whether `actor` currently holds a grant covering `scope` (e.g. `"messaging.backbone:<uri>"`)
/// — a seam, not a real grant table: no such table exists yet for backbone scopes specifically
/// (see `#region 📡️EffectBackbone`'s own doc and the packet report's honest gaps).
// 🚫️async: R1/E0038 — same `dyn CapabilityChecker` reasoning as `StorageBackend` above.
pub trait CapabilityChecker: Send + Sync {
    fn is_granted(&self, actor: u64, scope: &str) -> bool;
}

/// 🧪️ Permissive default/test double — grants everything.
pub struct AllowAllCapabilities;

impl CapabilityChecker for AllowAllCapabilities {
    fn is_granted(&self, _actor: u64, _scope: &str) -> bool {
        true
    }
}
//#endregion 🔑️CapabilityChecker

//#region 📡️EffectBackbone
/// 📡️ Per-instance backbone bridge (`📓️design-abi.md` §4) — replaces the deleted PROCESS-GLOBAL
/// `set_host_backbone_channel`, which left guest↔store sync with NO path at all. A `BackboneRegistry`
/// maps a backbone URI to its live [`BackboneTransport`] endpoint (bridging to the store sync
/// engine); `Effect::SendMessage { target: MessageEndpoint::Backbone { uri }, payload }` is checked
/// against capability scope `messaging.backbone:<uri>` (via [`CapabilityChecker`]) before it is
/// allowed through. Store deltas fan out through [`EventRouter`] under
/// `ChannelPolicy::Coalesced { key: uri }` (topic `backbone.delta.<uri>`) so a burst of deltas for
/// the SAME uri collapses to the latest instead of queueing — the exact backpressure vocabulary
/// `ChannelPolicy` exists for.
///
/// 🌉️ **Wire shape for the TypeScript counterpart** (a separate packet, per the mission): a
/// backbone message crossing the host↔guest boundary is
/// `{ "kind": "send" | "delta", "uri": string, "payload": <base64>, "revision"?: number }` —
/// `send` mirrors `Effect::SendMessage`'s `payload: Vec<u8>` (base64 on the JS side, raw bytes
/// host-side); `delta` mirrors a coalesced store-sync delta and carries a monotonic `revision` so
/// the TS side can detect a collapsed (skipped) delta the same way `UiPatch.base_revision` lets a
/// guest detect a stale diff. This module never itself crosses that boundary — it only defines the
/// shape precisely enough for the TS packet to conform to.
// 🚫️async: R1/E0038 — same `dyn BackboneTransport` reasoning as `StorageBackend` above
// (`BackboneRegistry::endpoints: Mutex<HashMap<String, Arc<dyn BackboneTransport>>>`).
pub trait BackboneTransport: Send + Sync {
    fn send(&self, uri: &str, payload: &[u8]) -> Result<(), std::io::Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackboneError {
    CapabilityDenied { uri: String },
    NoSuchEndpoint { uri: String },
    Transport(String),
}

impl std::fmt::Display for BackboneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackboneError::CapabilityDenied { uri } => write!(f, "capability `messaging.backbone:{uri}` not granted"),
            BackboneError::NoSuchEndpoint { uri } => write!(f, "no BackboneTransport registered for `{uri}`"),
            BackboneError::Transport(message) => write!(f, "backbone transport error: {message}"),
        }
    }
}

impl std::error::Error for BackboneError {}

pub struct BackboneRegistry {
    endpoints: Mutex<HashMap<String, Arc<dyn BackboneTransport>>>,
    pub(super) events: Arc<EventRouter>,
    pub(super) capabilities: Arc<dyn CapabilityChecker>,
}

impl BackboneRegistry {
    pub async fn new(events: Arc<EventRouter>, capabilities: Arc<dyn CapabilityChecker>) -> Self {
        Self { endpoints: Mutex::new(HashMap::new()), events, capabilities }
    }

    pub async fn register(&self, uri: String, transport: Arc<dyn BackboneTransport>) {
        self.endpoints.lock().expect("BackboneRegistry endpoints mutex poisoned").insert(uri, transport);
    }

    /// 📤️ `messaging.backbone:<uri>` capability-gated send — the `Effect::SendMessage` handler for
    /// a `MessageEndpoint::Backbone` target.
    pub async fn send(&self, actor: u64, uri: &str, payload: &[u8]) -> Result<(), BackboneError> {
        if !self.capabilities.is_granted(actor, &format!("messaging.backbone:{uri}")) {
            return Err(BackboneError::CapabilityDenied { uri: uri.to_string() });
        }
        let transport = self.endpoints.lock().expect("BackboneRegistry endpoints mutex poisoned").get(uri).cloned().ok_or_else(|| BackboneError::NoSuchEndpoint { uri: uri.to_string() })?;
        transport.send(uri, payload).map_err(|error| BackboneError::Transport(error.to_string()))
    }

    /// 📥️ Fans a store-sync delta for `uri` out to every actor subscribed to `backbone.delta.<uri>`
    /// — `ChannelPolicy::Coalesced { key: uri }` collapses a burst of deltas for the SAME uri to
    /// the latest rather than queueing every one.
    pub async fn fanout_delta(&self, uri: &str, delta: Vec<u8>) -> Vec<(semio_framework_actor::ActorId, PublishOutcome)> {
        let topic = Topic(format!("backbone.delta.{uri}"));
        self.events.publish(&topic, Some(uri), &delta).await
    }
}
//#endregion 📡️EffectBackbone

//#region 📊️EffectMetrics
/// 📊️ Where a Trap/Quarantine's `ScopeDrainReport` goes instead of being discarded — mission's own
/// wording: "Record the ScopeDrainReport into actor metrics rather than discarding it."
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrainOwner {
    Actor(u64),
    Package,
}

// 🚫️async: R1/E0038 — same `dyn EffectMetricsRecorder` reasoning as `StorageBackend` above
// (`AsyncEffectExecutor::metrics: Arc<dyn EffectMetricsRecorder>`).
pub trait EffectMetricsRecorder: Send + Sync {
    fn record_drain(&self, owner: DrainOwner, report: ScopeDrainReport);
}

pub struct NullMetricsRecorder;

impl EffectMetricsRecorder for NullMetricsRecorder {
    fn record_drain(&self, _owner: DrainOwner, _report: ScopeDrainReport) {}
}

/// 🧪️ Test double recording every drain report in order.
#[derive(Clone, Default)]
pub struct RecordingMetrics(Arc<Mutex<Vec<(DrainOwner, ScopeDrainReport)>>>);

impl RecordingMetrics {
    pub async fn new() -> Self {
        Self::default()
    }
    pub async fn recorded(&self) -> Vec<(DrainOwner, ScopeDrainReport)> {
        self.0.lock().expect("RecordingMetrics mutex poisoned").clone()
    }
}

impl EffectMetricsRecorder for RecordingMetrics {
    fn record_drain(&self, owner: DrainOwner, report: ScopeDrainReport) {
        self.0.lock().expect("RecordingMetrics mutex poisoned").push((owner, report));
    }
}
//#endregion 📊️EffectMetrics
