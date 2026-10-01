//! 🔗️ Authenticated hub descriptor binding for a headless MCP workspace.

use crate::{GatewayError, GatewayErrorCode};
use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::{
    client::{DirectoryClient, DirectoryClientError, DirectoryTransport, HubSocketGrantSource, LocalHubCredential},
    descriptor_digest_v1, hex_lower, DirectoryAccessChange, DirectoryEventBody, DirectorySessionKindV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, DirectoryStreamMessage, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1, DocumentScope, DocumentView, MemberSpaceViewV1, MemberView,
};
use semio_framework_os_kernel::{FromValue, ToValue};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError, RwLock};

#[path = "🧩️pair/🦀️.rs"]
mod pair;
pub use pair::{CanonicalPairBody, CanonicalPairFetchRequest, CanonicalPairHttpResponse, CanonicalPairMount, CanonicalPairMountError, CanonicalPairMountIdentity, CanonicalPairMountProgress, CanonicalPairMountStage, CanonicalPairTransport};
#[cfg(not(target_arch = "wasm32"))]
pub use pair::{NativeCanonicalPairBody, NativeCanonicalPairTransport};

pub const HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS: usize = 4_096;
/// 👥️ The most member rows a descriptor refresh reads while it looks for the authenticated principal's own row.
pub const HUB_SPACE_MEMBERS_MAX: usize = 4_096;
pub const HUB_VERIFIED_CATALOG_MAX_PACKAGES: usize = 256;
pub const HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES: usize = 32 * 1024 * 1024;
pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;
/// ✂️ The longest transport detail a hub-unavailable refusal carries (characters), schema `HubUnavailableCauseV1`.
pub const HUB_UNAVAILABLE_DETAIL_MAX_CHARS: usize = 512;
pub const HUB_BINDING_ID_MAX_BYTES: usize = 512;
pub const HUB_BINDING_OPERATION_TIMEOUT_MS: u64 = 10_000;
/// ⏳️ How long a hub-bound call waits for a descriptor refresh already in flight before it fails
/// closed: one refresh turn's own deadline. Every directory event in the space (another member's
/// new document, this agent's own commit) invalidates the authority, and without the wait every
/// tool call in that window answered a retryable `PLUGIN_UNAVAILABLE`, including an approval whose
/// edit had already landed.
pub const HUB_AUTHORITY_SETTLE_WAIT_MS: u64 = HUB_BINDING_OPERATION_TIMEOUT_MS;
/// ⏳️ How long a hub-bound call keeps waiting while the binding waits for its network byte budget to refill: one refill
/// interval on top of the ordinary settle wait, so a call spans the refill instead of refusing inside it.
pub const HUB_NETWORK_BUDGET_SETTLE_WAIT_MS: u64 = semio_framework_os_services::HTTP_BUCKET_REFILL_INTERVAL_MS + HUB_AUTHORITY_SETTLE_WAIT_MS;
/// 💰️ The budget one authority refresh waits for before it runs again after its budget ran out: the session, the space page
/// and one manifest per document, plus any descriptor it has not verified before.
pub const HUB_REFRESH_NETWORK_BUDGET_BYTES: u64 = 16 * 1024 * 1024;
/// 🔁️ How often a network budget wait re-reads the budget.
pub const HUB_NETWORK_BUDGET_POLL_MS: u64 = 250;

/// 💰️ Waits until `read` reports at least `needed` bytes of budget (a whole allowance when `needed` exceeds it), reporting
/// each wait to `report` as `(remaining, needed)`: a spent budget is a wait for its next refill turn, never a refusal.
/// Only `cancel` ends the wait early.
pub fn await_network_budget(
    read: impl Fn() -> semio_framework_os_services::HttpPackageBudget,
    needed: u64,
    cancel: &semio_framework_async::CancelToken,
    mut report: impl FnMut(u64, u64),
) -> Result<(), HubBindingError> {
    loop {
        let budget = read();
        let needed = needed.min(budget.capacity_bytes);
        if budget.remaining_bytes >= needed {
            return Ok(());
        }
        if cancel.is_cancelled_now() {
            return Err(HubBindingError::Cancelled);
        }
        report(budget.remaining_bytes, needed);
        pause_unless_cancelled(cancel, HUB_NETWORK_BUDGET_POLL_MS);
    }
}

/// 🔁️ First and largest pause between failed authority refreshes. A refresh costs the hub two
/// requests plus two per document, and one that failed (a busy hub answering `503
/// deadline-exceeded`) used to be retried back-to-back for as long as it kept failing.
pub const HUB_REFRESH_RETRY_BASE_MS: u64 = 100;
pub const HUB_REFRESH_RETRY_MAX_MS: u64 = 5_000;

/// 🔁️ `min(base · 2^(failures-1), max)` — the pause after the `failures`th refresh in a row failed.
pub fn hub_refresh_retry_ms(failures: u32) -> u64 {
    HUB_REFRESH_RETRY_BASE_MS.saturating_mul(1u64 << failures.saturating_sub(1).min(16)).min(HUB_REFRESH_RETRY_MAX_MS)
}

/// 😴️ Sleeps `ms` in short steps so a cancelled binding actor stops within one step.
fn pause_unless_cancelled(cancel: &semio_framework_async::CancelToken, ms: u64) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
    while !cancel.is_cancelled_now() {
        let Some(remaining) = deadline.checked_duration_since(std::time::Instant::now()).filter(|remaining| !remaining.is_zero()) else { return };
        std::thread::sleep(remaining.min(std::time::Duration::from_millis(25)));
    }
}
/// 🧩️ Execution-target components are static assets — immutable, catalog-verified, content-addressed — so a gateway keeps
/// them in the per-user execution-target store and fetches a missing one over a pool of its own that meters no bytes: the
/// per-minute budget is for the directory's API requests, and component traffic is bounded by [`HUB_COMPONENT_STREAMS`]
/// here and by the hub's own execution-target asset stream rule (see [`HubComponentAssets`]).
pub const HUB_COMPONENT_ASSET_BYTES_PER_MINUTE: u64 = u64::MAX;
/// 🚰️ Component streams one gateway runs at once: one stream already takes the whole link, a second only splits it.
pub const HUB_COMPONENT_STREAMS: u32 = 1;
/// ⏱️ One component stream's deadline is this allowance plus its declared length at [`HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND`].
pub const HUB_COMPONENT_STREAM_ALLOWANCE_MS: u64 = 60_000;
/// 🐢️ The slowest transfer rate a component stream is still waited for.
pub const HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND: u64 = 256 * 1024;
/// 🔁️ How often one component is asked for across interrupted streams and hub shortages (`502`/`503`/`504`) before the
/// open fails, retryably.
pub const HUB_COMPONENT_STREAM_ATTEMPTS: u32 = 4;
/// ⏳️ The wait a busy hub's asset refusal stands for when its body names none, and the longest wait any refusal is held to.
pub const HUB_COMPONENT_STREAM_RETRY_MS: u64 = 1_000;
pub const HUB_COMPONENT_STREAM_RETRY_MAX_MS: u64 = 30_000;
/// 📈️ The longest a component transfer stays silent: an unchanged progress is published again after this long.
pub const HUB_COMPONENT_PROGRESS_KEEPALIVE_MS: u64 = 2_000;
/// 💰️ The budget one execution-target lease request waits for: its ≈3 KB JSON with generous headroom.
pub const HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES: u64 = 64 * 1024;
pub const CANONICAL_CHECKPOINT_RESOURCE_SCHEMA: &str = "semio.mcp.canonical-checkpoint-resource/v1";
pub const CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES: usize = 6 * 1024 * 1024;
const CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES: usize = 16 * 1024;
static NEXT_HUB_AUTHORITY_GENERATION: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedDocumentView {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub view: DocumentView,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedDescriptorSnapshot {
    pub authenticated_user_id: String,
    pub session_expires_at_ms: i64,
    pub space: MemberSpaceViewV1,
    pub membership: MemberView,
    pub observed_event_seq: u64,
    pub documents: HashMap<DocumentScope, AuthorizedDocumentView>,
}

/// 🧾 One package descriptor selected by the authenticated Hub for at least one document.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedPackageSelection {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub lease: DocumentExecutionTargetLeaseFieldsV1,
    pub descriptor: semio_framework::PackageDescriptor,
}

/// 🔐 A bounded package roster that is publishable only while the exact descriptor authority
/// generation that selected it remains live.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedCatalogSnapshot {
    pub authority_generation: u64,
    pub selections: Vec<AuthorizedPackageSelection>,
    /// 🪢 Each document's owning-app dialect coordinate exactly as its own authenticated lease names it
    /// (`parent_dialect.artifact_kind`, the dialect the hub indexed at genesis). Kept per document
    /// because `selections` is deduplicated per package, and one package owns several kinds.
    pub dialect_kinds: HashMap<DocumentScope, String>,
}

/// 🗃️ Package descriptors this binding already verified, keyed by the SHA-256 and byte length an authenticated lease
/// names. Descriptor bytes are content-addressed, so one verified descriptor serves every document and every later refresh
/// that names it; the cache holds at most [`HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES`] and keeps only what the latest catalog
/// selects.
#[derive(Default)]
struct HubDescriptorCache {
    entries: HashMap<(String, u64), semio_framework::PackageDescriptor>,
    bytes: u64,
}

impl HubDescriptorCache {
    fn get(&self, key: &(String, u64)) -> Option<semio_framework::PackageDescriptor> {
        self.entries.get(key).cloned()
    }

    fn insert(&mut self, key: (String, u64), descriptor: semio_framework::PackageDescriptor, used: &HashSet<(String, u64)>) -> Result<(), HubBindingError> {
        if self.entries.contains_key(&key) {
            return Ok(());
        }
        if self.bytes.saturating_add(key.1) > HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64 {
            self.retain(used);
        }
        self.bytes = self.bytes.checked_add(key.1).filter(|bytes| *bytes <= HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64).ok_or(HubBindingError::CapacityExceeded)?;
        self.entries.insert(key, descriptor);
        Ok(())
    }

    fn retain(&mut self, selected: &HashSet<(String, u64)>) {
        self.entries.retain(|key, _| selected.contains(key));
        self.bytes = self.entries.keys().map(|(_, length)| *length).sum();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HubRemoteBindingState {
    Unbound,
    Refreshing,
    Ready(Arc<AuthorizedDescriptorSnapshot>),
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubBindingPhase {
    Idle,
    Authenticating,
    LoadingSpace,
    ValidatingDocuments,
    AwaitingNetworkBudget,
    Ready,
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubBindingProgress {
    pub phase: HubBindingPhase,
    pub completed: usize,
    pub total: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubStreamObservation {
    Stable,
    RefreshRequired,
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubBindingError {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    SessionExpired,
    MembershipRequired,
    CapacityExceeded,
    InvalidResponse(&'static str),
    StaleRefresh,
    Unavailable(HubUnavailableCause),
}

/// 🔎️ What made the hub directory unavailable, so a refusal names it instead of one opaque word
/// (schema `HubUnavailableCauseV1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubUnavailableCause {
    /// 🌐️ The HTTP status and, when the hub answered its typed refusal, that refusal's `code`.
    Http { status: u16, code: Option<String> },
    /// 🔌️ The transport failed before any HTTP answer; `detail` is its own bounded cause (an exhausted byte
    /// budget, a refused connection), never dropped.
    Transport { detail: String },
    /// 💰️ This gateway's own network byte budget is spent until its next refill turn — the hub is fine.
    NetworkBudget,
    UndecodableResponse,
}

impl HubUnavailableCause {
    /// 🌐️ An HTTP refusal, keeping the hub's typed `code` (bounded) when its body carries one.
    fn http(status: u16, body: &str) -> Self {
        let code = serde_json::from_str::<serde_json::Value>(body).ok().and_then(|value| value.get("code").and_then(serde_json::Value::as_str).map(|code| code.chars().take(64).collect()));
        Self::Http { status, code }
    }

    /// 🔌️ A transport failure, keeping its own detail bounded to [`HUB_UNAVAILABLE_DETAIL_MAX_CHARS`].
    fn transport(error: &semio_framework_os_kernel::os_directory::client::TransportError) -> Self {
        if matches!(error, semio_framework_os_kernel::os_directory::client::TransportError::BudgetExhausted) {
            return Self::NetworkBudget;
        }
        let detail = match error {
            semio_framework_os_kernel::os_directory::client::TransportError::Io(detail) => detail.clone(),
            other => other.to_string(),
        };
        Self::Transport { detail: detail.chars().take(HUB_UNAVAILABLE_DETAIL_MAX_CHARS).collect() }
    }

    /// 🧾️ The typed `HubUnavailableCauseV1` a refusal's `details.cause` carries.
    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Http { status, code } => serde_json::json!({ "kind": "http", "status": status, "code": code }),
            Self::Transport { detail } => serde_json::json!({ "kind": "transport", "detail": detail }),
            Self::NetworkBudget => serde_json::json!({ "kind": "network-budget" }),
            Self::UndecodableResponse => serde_json::json!({ "kind": "undecodable-response" }),
        }
    }

    /// 🗣️ The en + de sentence a client shows for this cause (`details.summary`).
    fn summary(&self) -> serde_json::Value {
        let (en, de) = match self {
            Self::Http { status, code: Some(code) } => (
                format!("The hub answered HTTP {status} ({code}); the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status} ({code}); die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Http { status, code: None } => (
                format!("The hub answered HTTP {status}; the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status}; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Transport { detail } => (
                format!("The hub could not be reached ({detail}); the request is retried once the connection recovers."),
                format!("Der Hub war nicht erreichbar ({detail}); die Anfrage wird wiederholt, sobald die Verbindung wieder steht."),
            ),
            Self::NetworkBudget => (
                "This gateway's network allowance for this minute is used up; the request continues once it refills.".to_string(),
                "Das Netzwerkkontingent dieses Gateways für diese Minute ist aufgebraucht; die Anfrage läuft weiter, sobald es aufgefüllt ist.".to_string(),
            ),
            Self::UndecodableResponse => (
                "The hub's answer could not be read; the request is retried once the hub recovers.".to_string(),
                "Die Antwort des Hubs war nicht lesbar; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist.".to_string(),
            ),
        };
        serde_json::json!({ "en": en, "de": de })
    }
}

impl std::fmt::Display for HubBindingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("hub descriptor refresh was cancelled"),
            Self::DeadlineExceeded => formatter.write_str("hub descriptor refresh exceeded its deadline"),
            Self::Unauthorized => formatter.write_str("hub session is unauthorized"),
            Self::SessionExpired => formatter.write_str("hub session is expired"),
            Self::MembershipRequired => formatter.write_str("current hub space membership is required"),
            Self::CapacityExceeded => formatter.write_str("hub descriptor index exceeded its fixed document capacity"),
            Self::InvalidResponse(detail) => write!(formatter, "hub directory response was invalid: {detail}"),
            Self::StaleRefresh => formatter.write_str("hub descriptor refresh was superseded"),
            Self::Unavailable(HubUnavailableCause::Http { status, code: Some(code) }) => write!(formatter, "hub directory is temporarily unavailable (HTTP {status} {code})"),
            Self::Unavailable(HubUnavailableCause::Http { status, code: None }) => write!(formatter, "hub directory is temporarily unavailable (HTTP {status})"),
            Self::Unavailable(HubUnavailableCause::Transport { detail }) => write!(formatter, "hub directory is temporarily unavailable (transport: {detail})"),
            Self::Unavailable(HubUnavailableCause::NetworkBudget) => formatter.write_str("hub directory is temporarily unavailable (network byte budget exhausted until its next refill)"),
            Self::Unavailable(HubUnavailableCause::UndecodableResponse) => formatter.write_str("hub directory is temporarily unavailable (undecodable response)"),
        }
    }
}

impl std::error::Error for HubBindingError {}

pub struct HubRemoteBinding {
    hub_origin: String,
    space_id: String,
    state: RwLock<HubRemoteBindingState>,
    progress: RwLock<HubBindingProgress>,
    diagnostic: RwLock<Option<String>>,
    generation: AtomicU64,
    authority_generation: AtomicU64,
    observed_event_seq: AtomicU64,
    authenticated_user_id: RwLock<Option<String>>,
    catalog: RwLock<Option<Arc<AuthorizedCatalogSnapshot>>>,
    descriptors: Mutex<HubDescriptorCache>,
    pair_actor: Mutex<pair::CanonicalPairActor>,
    settle: (Mutex<()>, Condvar),
    #[cfg(test)]
    pair_mount_return_pause: Mutex<Option<(Arc<std::sync::Barrier>, Arc<std::sync::Barrier>)>>,
}

impl HubRemoteBinding {
    pub fn new(hub_origin: &str, space_id: impl Into<String>) -> Result<Self, HubBindingError> {
        let hub_origin = pair::normalize_hub_origin(hub_origin)?;
        let space_id = space_id.into();
        validate_identity("space id", &space_id)?;
        Ok(Self {
            pair_actor: Mutex::new(pair::CanonicalPairActor::new(hub_origin.clone())),
            #[cfg(test)]
            pair_mount_return_pause: Mutex::new(None),
            hub_origin,
            space_id,
            state: RwLock::new(HubRemoteBindingState::Unbound),
            progress: RwLock::new(HubBindingProgress { phase: HubBindingPhase::Idle, completed: 0, total: 0 }),
            diagnostic: RwLock::new(None),
            generation: AtomicU64::new(0),
            authority_generation: AtomicU64::new(0),
            observed_event_seq: AtomicU64::new(0),
            authenticated_user_id: RwLock::new(None),
            catalog: RwLock::new(None),
            descriptors: Mutex::new(HubDescriptorCache::default()),
            settle: (Mutex::new(()), Condvar::new()),
        })
    }

    /// 🚫️ The retryable refusal for `state`, naming the refresh phase and the binding's last fault.
    fn unavailable(&self, state: HubRemoteBindingState) -> GatewayError {
        unavailable_gateway_error(state, self.progress().phase, self.diagnostic())
    }

    /// 🔄️ Whether an authority refresh is in flight: not yet bound, refreshing, or bound while the
    /// catalog of that exact authority generation is still being verified.
    fn settling(&self) -> bool {
        match self.state() {
            HubRemoteBindingState::Unbound | HubRemoteBindingState::Refreshing => true,
            HubRemoteBindingState::Ready(_) => {
                let authority_generation = self.authority_generation.load(Ordering::SeqCst);
                authority_generation == 0 || self.catalog.read().unwrap_or_else(PoisonError::into_inner).as_ref().is_none_or(|catalog| catalog.authority_generation != authority_generation)
            }
            HubRemoteBindingState::Revoked => false,
        }
    }

    /// 📣️ Wakes every caller parked in [`Self::await_settled`]; called after each state, authority
    /// or catalog transition, never while one of those locks is held.
    fn announce(&self) {
        let _guard = self.settle.0.lock().unwrap_or_else(PoisonError::into_inner);
        self.settle.1.notify_all();
    }

    /// ⏳️ Parks the calling (tool) thread until the refresh in flight settles or `wait_ms` passes — or, while the refresh
    /// waits for its network byte budget to refill, until [`HUB_NETWORK_BUDGET_SETTLE_WAIT_MS`] passes, so a call spans the
    /// refill instead of refusing inside it. A call that carries a progress token hears each second of the wait as
    /// `notifications/progress` naming what it waits for. It decides nothing: the caller's own gate still fails closed if
    /// the authority is not ready. Only request threads call it; the binding actor that performs the refresh never does.
    pub fn await_settled(&self, wait_ms: u64) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let started = std::time::Instant::now();
            let mut deadline = started + std::time::Duration::from_millis(wait_ms);
            let mut guard = self.settle.0.lock().unwrap_or_else(PoisonError::into_inner);
            while self.settling() {
                let progress = self.progress();
                if progress.phase == HubBindingPhase::AwaitingNetworkBudget {
                    deadline = deadline.max(started + std::time::Duration::from_millis(HUB_NETWORK_BUDGET_SETTLE_WAIT_MS));
                }
                let now = std::time::Instant::now();
                let Some(remaining) = deadline.checked_duration_since(now).filter(|remaining| !remaining.is_zero()) else { return };
                let fraction = now.duration_since(started).as_secs_f64() / deadline.duration_since(started).as_secs_f64();
                crate::notify::publish_call_progress(fraction, &settle_progress_message(&progress));
                guard = self.settle.1.wait_timeout(guard, remaining.min(std::time::Duration::from_secs(1))).unwrap_or_else(PoisonError::into_inner).0;
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = wait_ms;
    }

    /// 💰️ Records that the refresh waits for the network byte budget: `remaining` of the `needed` bytes are available. A
    /// call parked in [`Self::await_settled`] extends its wait over the refill and names it in its progress.
    pub fn note_network_budget_wait(&self, remaining: u64, needed: u64) {
        self.set_progress(HubBindingPhase::AwaitingNetworkBudget, usize::try_from(remaining).unwrap_or(usize::MAX), usize::try_from(needed).unwrap_or(usize::MAX));
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic("the gateway's network byte budget is spent; the authority refresh continues once it refills"));
        self.announce();
    }

    pub fn state(&self) -> HubRemoteBindingState {
        self.state.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn progress(&self) -> HubBindingProgress {
        self.progress.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn diagnostic(&self) -> Option<String> {
        self.diagnostic.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn ready_snapshot(&self, wall_now_ms: i64) -> Result<Arc<AuthorizedDescriptorSnapshot>, GatewayError> {
        let ready = match self.state() {
            HubRemoteBindingState::Ready(snapshot) if snapshot.session_expires_at_ms > wall_now_ms => snapshot,
            HubRemoteBindingState::Ready(_) => {
                self.revoke(HubBindingError::SessionExpired);
                return Err(self.unavailable(HubRemoteBindingState::Revoked));
            }
            state => return Err(self.unavailable(state)),
        };
        Ok(ready)
    }

    /// 📚 Returns the exact Hub-selected package descriptors only while their authenticated
    /// descriptor authority is still current. Refreshing, expiry and revocation fail closed.
    pub fn ready_catalog_snapshot(&self, wall_now_ms: i64) -> Result<Arc<AuthorizedCatalogSnapshot>, GatewayError> {
        self.ready_snapshot(wall_now_ms)?;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        let catalog = self.catalog.read().unwrap_or_else(PoisonError::into_inner).clone();
        match catalog {
            Some(catalog) if authority_generation != 0 && catalog.authority_generation == authority_generation && self.authority_generation.load(Ordering::SeqCst) == authority_generation => Ok(catalog),
            _ => Err(self.unavailable(HubRemoteBindingState::Refreshing)),
        }
    }

    /// 🔎 Fetches every selected document's manifest and verifies the protected descriptor body it names. No repository
    /// path participates: manifest identity, bytes and descriptor contents all come from authenticated execution-target
    /// routes and are fenced by the current authority generation. A descriptor body is fetched only the first time its
    /// SHA-256 is named (see [`HubDescriptorCache`]): a space of 63 documents over 33 packages moved 16.7 MB of descriptors
    /// per refresh — every directory event triggers one — and spent the gateway's whole byte budget within a minute.
    pub async fn refresh_catalog<T: DirectoryTransport>(
        &self,
        client: &DirectoryClient<T>,
        snapshot: &Arc<AuthorizedDescriptorSnapshot>,
        ctx: &OperationContext,
    ) -> Result<Arc<AuthorizedCatalogSnapshot>, HubBindingError> {
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(HubBindingError::StaleRefresh);
        }
        let mut documents: Vec<_> = snapshot.documents.values().cloned().collect();
        documents.sort_by(|left, right| left.scope.space_id.cmp(&right.scope.space_id).then_with(|| left.scope.document_id.cmp(&right.scope.document_id)));
        let mut selected = BTreeMap::<(String, String, String, String, String), AuthorizedPackageSelection>::new();
        let mut dialect_kinds = HashMap::<DocumentScope, String>::new();
        let mut used = HashSet::<(String, u64)>::new();
        let mut used_bytes = 0u64;
        let mut catalog_generation_id: Option<String> = None;
        for (index, document) in documents.into_iter().enumerate() {
            if ctx.cancel.is_cancelled_now() {
                return Err(HubBindingError::Cancelled);
            }
            let intent = DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: document.scope.clone(),
                requested_surface_id: None,
                client_instance_id: format!("mcp-catalog-{index}"),
            };
            let lease = client.document_execution_target_manifest(ctx, &intent).await.map_err(map_catalog_client_error)?;
            if lease.scope != document.scope
                || lease.descriptor_digest_v1 != document.descriptor_digest_v1
                || lease.package.plugin_id != document.view.descriptor.owner.plugin_id
                || lease.package.package_id != document.view.descriptor.owner.package_id
                || lease.package.version != document.view.descriptor.owner.version
                || lease.package.component_sha256 != document.view.descriptor.owner.package_hash
                || lease.artifact.kind != document.view.descriptor.artifact_kind
                || lease.artifact.schema != document.view.descriptor.artifact_schema
                || lease.artifact.pack_schema_hash != document.view.descriptor.pack_schema_hash
            {
                return Err(HubBindingError::InvalidResponse("execution-target selection does not match authenticated document descriptor"));
            }
            match &catalog_generation_id {
                Some(expected) if expected != &lease.catalog.generation_id => return Err(HubBindingError::InvalidResponse("documents resolved against different catalog generations")),
                None => catalog_generation_id = Some(lease.catalog.generation_id.clone()),
                Some(_) => {}
            }
            let key = (lease.descriptor.sha256.clone(), lease.descriptor.byte_length);
            if !used.contains(&key) {
                used_bytes = used_bytes.checked_add(key.1).filter(|bytes| *bytes <= HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES as u64).ok_or(HubBindingError::CapacityExceeded)?;
            }
            let cached = self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).get(&key);
            let descriptor = match cached {
                Some(descriptor) => descriptor,
                None => {
                    let descriptor_bytes = client.document_execution_target_descriptor(ctx, &intent).await.map_err(map_catalog_client_error)?;
                    if lease.descriptor.byte_length != descriptor_bytes.len() as u64 || lease.descriptor.sha256 != framework_hash::sha256_hex(&descriptor_bytes) {
                        return Err(HubBindingError::InvalidResponse("execution-target selection does not match authenticated document descriptor"));
                    }
                    let descriptor_value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(&descriptor_bytes)
                        .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a canonical pack"))?;
                    let descriptor = semio_framework::PackageDescriptor::from_value(descriptor_value.clone())
                        .map_err(|_| HubBindingError::InvalidResponse("execution-target descriptor is not a package descriptor"))?;
                    if semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&descriptor_value) != descriptor_bytes
                        || semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&descriptor.to_value()) != descriptor_bytes
                    {
                        return Err(HubBindingError::InvalidResponse("execution-target descriptor is not its exact canonical package projection"));
                    }
                    self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).insert(key.clone(), descriptor.clone(), &used)?;
                    descriptor
                }
            };
            used.insert(key);
            if descriptor.descriptor_version != 1
                || descriptor.package_id != lease.package.package_id
                || descriptor.manifest.plugin_id != lease.package.plugin_id
                || descriptor.manifest.version != lease.package.version
                || descriptor.hashes.wasm_sha256 != lease.package.component_sha256
            {
                return Err(HubBindingError::InvalidResponse("execution-target package descriptor identity mismatch"));
            }
            let key = (
                lease.package.plugin_id.clone(),
                lease.package.package_id.clone(),
                lease.package.version.clone(),
                lease.package.component_sha256.clone(),
                lease.package.descriptor_byte_sha256.clone(),
            );
            dialect_kinds.insert(document.scope.clone(), lease.parent_dialect.artifact_kind.clone());
            let candidate = AuthorizedPackageSelection { scope: document.scope, descriptor_digest_v1: document.descriptor_digest_v1, lease, descriptor };
            if let Some(previous) = selected.get(&key) {
                if previous.descriptor != candidate.descriptor || previous.lease.catalog != candidate.lease.catalog {
                    return Err(HubBindingError::InvalidResponse("one selected package identity resolved to different descriptors"));
                }
            } else {
                if selected.len() >= HUB_VERIFIED_CATALOG_MAX_PACKAGES {
                    return Err(HubBindingError::CapacityExceeded);
                }
                selected.insert(key, candidate);
            }
        }
        if self.authority_generation.load(Ordering::SeqCst) != authority_generation
            || !matches!(self.state(), HubRemoteBindingState::Ready(current) if current.as_ref() == snapshot.as_ref())
        {
            return Err(HubBindingError::StaleRefresh);
        }
        self.descriptors.lock().unwrap_or_else(PoisonError::into_inner).retain(&used);
        let catalog = Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections: selected.into_values().collect(), dialect_kinds });
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(catalog.clone());
        self.announce();
        Ok(catalog)
    }

    pub async fn refresh<T: DirectoryTransport>(&self, client: &DirectoryClient<T>, ctx: &OperationContext, wall_now_ms: i64, operation_now_ms: u64) -> Result<Arc<AuthorizedDescriptorSnapshot>, HubBindingError> {
        let generation = self.begin_refresh(HubBindingPhase::Authenticating, 0);
        if ctx.cancel.is_cancelled_now() {
            return self.fail(generation, HubBindingError::Cancelled);
        }
        if ctx.deadline_ms.is_some_and(|deadline| deadline <= operation_now_ms) {
            return self.fail(generation, HubBindingError::DeadlineExceeded);
        }
        let session = match client.me(ctx).await {
            Ok(session) => session,
            Err(error) => return self.fail(generation, map_client_error(error)),
        };
        if session.expires_at <= wall_now_ms {
            return self.fail(generation, HubBindingError::SessionExpired);
        }
        validate_identity("authenticated user id", &session.user_id).and_then(|_| validate_identity("authenticated email", &session.email)).map_err(|error| {
            let _ = self.fail::<Arc<AuthorizedDescriptorSnapshot>>(generation, error.clone());
            error
        })?;
        self.set_progress(HubBindingPhase::LoadingSpace, 0, 0);
        let (space, members, documents) = match self.read_space_administration(client, ctx, &session.user_id).await {
            Ok(projection) => projection,
            Err(error) => return self.fail(generation, error),
        };
        let observed_event_seq = self.observed_event_seq.load(Ordering::SeqCst);
        let snapshot = match self.validate_snapshot(session.user_id, session.session_kind, session.expires_at, space, members, documents, observed_event_seq, ctx) {
            Ok(snapshot) => Arc::new(snapshot),
            Err(error) => return self.fail(generation, error),
        };
        if self.generation.load(Ordering::SeqCst) != generation {
            return Err(HubBindingError::StaleRefresh);
        }
        let authority_generation = next_authority_generation()?;
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = Some(snapshot.authenticated_user_id.clone());
        self.authority_generation.store(authority_generation, Ordering::SeqCst);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Ready(snapshot.clone());
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).descriptor_ready(authority_generation);
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(HubBindingPhase::Ready, snapshot.documents.len(), snapshot.documents.len());
        self.announce();
        Ok(snapshot)
    }

    pub fn observe_stream_message(&self, message: &DirectoryStreamMessage) -> HubStreamObservation {
        match message {
            DirectoryStreamMessage::Event { event } => {
                self.observed_event_seq.fetch_max(event.seq, Ordering::SeqCst);
                if event.space_id.as_deref() != Some(self.space_id.as_str()) {
                    return HubStreamObservation::Stable;
                }
                if let DirectoryEventBody::MemberRemoved { space_id, user_id } = &event.body {
                    let subject = self.authenticated_user_id.read().unwrap_or_else(PoisonError::into_inner).clone();
                    if space_id == &self.space_id && subject.as_deref() == Some(user_id.as_str()) {
                        self.revoke(HubBindingError::MembershipRequired);
                        return HubStreamObservation::Revoked;
                    }
                }
                if matches!(&event.body, DirectoryEventBody::SpaceDeleted { space_id } if space_id == &self.space_id) {
                    self.revoke(HubBindingError::MembershipRequired);
                    return HubStreamObservation::Revoked;
                }
                self.invalidate("hub directory event requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::RebootstrapRequired { control } if control.scope.space_id == self.space_id => {
                self.invalidate("hub directory rebootstrap requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange::Revoked } if space_id == &self.space_id => {
                self.revoke(HubBindingError::MembershipRequired);
                HubStreamObservation::Revoked
            }
            DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange::Granted } if space_id == &self.space_id => {
                self.invalidate("hub directory access grant requires an authenticated descriptor refresh");
                HubStreamObservation::RefreshRequired
            }
            DirectoryStreamMessage::Heartbeat { head_seq } => {
                self.observed_event_seq.fetch_max(*head_seq, Ordering::SeqCst);
                HubStreamObservation::Stable
            }
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } | DirectoryStreamMessage::AccessChanged { .. } => HubStreamObservation::Stable,
        }
    }

    pub fn invalidate_stream(&self) {
        self.invalidate("hub directory stream continuity was lost");
    }

    /// 🩺️ Records why the live directory stream's last dial failed, without starting a refresh: the stream's own reconnect
    /// ladder retries the dial, and a hub-bound refusal names this cause as its `lastFault`.
    pub fn note_stream_fault(&self, cause: &str) {
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(cause));
        self.announce();
    }

    fn begin_refresh(&self, phase: HubBindingPhase, total: usize) -> u64 {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        let generation = self.generation.fetch_add(1, Ordering::SeqCst).saturating_add(1);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(phase, 0, total);
        self.announce();
        generation
    }

    /// 📚️ The space's administration projection across every bounded window. The hub bounds each window to
    /// `DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS` rows and a cursor advances only the window it was issued for, so the
    /// documents are followed to their last page, then the members until the authenticated principal's own row, every
    /// page bound to the first page's authorization generation and at most [`HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS`]
    /// documents / [`HUB_SPACE_MEMBERS_MAX`] members. Reading only the first page refused every space past 64 documents
    /// (live hub coverage on 7800/p33, 66 documents: every `artifact_open` answered "descriptor index is refreshing").
    async fn read_space_administration<T: DirectoryTransport>(&self, client: &DirectoryClient<T>, ctx: &OperationContext, authenticated_user_id: &str) -> Result<(MemberSpaceViewV1, Vec<MemberView>, Vec<DocumentView>), HubBindingError> {
        let windows = |page: &DirectorySpaceAdministrationPageV1| match page.clone() {
            DirectorySpaceAdministrationPageV1::Member { authorization_generation, space, members, documents, .. } | DirectorySpaceAdministrationPageV1::Author { authorization_generation, space, members, documents, .. } => {
                Ok((authorization_generation, space, members, documents))
            }
            DirectorySpaceAdministrationPageV1::Public { .. } => Err(HubBindingError::MembershipRequired),
        };
        let first = client.space_administration_page(ctx, &self.space_id, None).await.map_err(map_client_error)?;
        let (authorization, space, first_members, first_documents) = windows(first.page())?;
        let declared = usize::try_from(space.document_count).unwrap_or(usize::MAX);
        let (mut members, mut documents) = (first_members.rows, first_documents.rows);
        let member_cursor = first_members.next_cursor.filter(|_| !members.iter().any(|row| row.user_id == authenticated_user_id));
        for (documents_window, mut cursor) in [(true, first_documents.next_cursor), (false, member_cursor)] {
            while let Some(next) = cursor.take() {
                if ctx.cancel.is_cancelled_now() {
                    return Err(HubBindingError::Cancelled);
                }
                let page = client.space_administration_page(ctx, &self.space_id, Some(&next)).await.map_err(map_client_error)?;
                let (page_authorization, _, page_members, page_documents) = windows(page.page())?;
                if page_authorization != authorization {
                    return Err(HubBindingError::InvalidResponse("space administration pages crossed an authorization change"));
                }
                if documents_window {
                    documents.extend(page_documents.rows);
                    cursor = page_documents.next_cursor;
                } else {
                    let found = page_members.rows.iter().any(|row| row.user_id == authenticated_user_id);
                    members.extend(page_members.rows);
                    cursor = page_members.next_cursor.filter(|_| !found);
                }
                if documents.len() > HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS || members.len() > HUB_SPACE_MEMBERS_MAX {
                    return Err(HubBindingError::CapacityExceeded);
                }
                self.set_progress(HubBindingPhase::LoadingSpace, documents.len(), declared);
            }
        }
        Ok((space, members.into_iter().map(|row| MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect(), documents))
    }

    fn set_progress(&self, phase: HubBindingPhase, completed: usize, total: usize) {
        *self.progress.write().unwrap_or_else(PoisonError::into_inner) = HubBindingProgress { phase, completed, total };
    }

    fn invalidate(&self, diagnostic: &str) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(diagnostic));
        self.set_progress(HubBindingPhase::Idle, 0, 0);
        self.announce();
    }

    fn revoke(&self, error: HubBindingError) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = None;
        actor.invalidate(pair::CanonicalPairActorState::Revoked);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Revoked;
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = None;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
        self.set_progress(HubBindingPhase::Revoked, 0, 0);
        self.announce();
    }

    fn fail<T>(&self, generation: u64, error: HubBindingError) -> Result<T, HubBindingError> {
        if self.generation.load(Ordering::SeqCst) != generation {
            return Err(HubBindingError::StaleRefresh);
        }
        if matches!(error, HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) {
            self.revoke(error.clone());
        } else {
            *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
            *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
            self.set_progress(HubBindingPhase::Idle, 0, 0);
            self.announce();
        }
        Err(error)
    }

    fn validate_snapshot(
        &self,
        authenticated_user_id: String,
        session_kind: DirectorySessionKindV1,
        session_expires_at_ms: i64,
        space: MemberSpaceViewV1,
        members: Vec<MemberView>,
        documents: Vec<DocumentView>,
        observed_event_seq: u64,
        ctx: &OperationContext,
    ) -> Result<AuthorizedDescriptorSnapshot, HubBindingError> {
        if space.id != self.space_id {
            return Err(HubBindingError::InvalidResponse("selected space id mismatch"));
        }
        validate_identity("space id", &space.id)?;
        let membership = members.into_iter().find(|member| member.user_id == authenticated_user_id).ok_or(HubBindingError::MembershipRequired)?;
        if !principal_role_admitted(session_kind, space.role, membership.role) {
            return Err(HubBindingError::MembershipRequired);
        }
        validate_document_count(documents.len(), space.document_count)?;
        let total = documents.len();
        self.set_progress(HubBindingPhase::ValidatingDocuments, 0, total);
        let mut indexed = HashMap::with_capacity(total);
        for (index, view) in documents.into_iter().enumerate() {
            if index % 32 == 0 && ctx.cancel.is_cancelled_now() {
                return Err(HubBindingError::Cancelled);
            }
            if view.descriptor.space_id != self.space_id {
                return Err(HubBindingError::InvalidResponse("document escaped the selected space"));
            }
            validate_identity("document id", &view.descriptor.document_id)?;
            if view.commit_seq > view.head_seq {
                return Err(HubBindingError::InvalidResponse("document commit exceeds head"));
            }
            let digest = descriptor_digest_v1(&view.descriptor).map_err(|_| HubBindingError::InvalidResponse("document descriptor digest is invalid"))?;
            let scope = DocumentScope::new(self.space_id.clone(), view.descriptor.document_id.clone());
            let document = AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: hex_lower(digest.as_bytes()), view };
            if indexed.insert(scope, document).is_some() {
                return Err(HubBindingError::InvalidResponse("duplicate document scope"));
            }
            if index % 32 == 31 || index + 1 == total {
                self.set_progress(HubBindingPhase::ValidatingDocuments, index + 1, total);
            }
        }
        Ok(AuthorizedDescriptorSnapshot { authenticated_user_id, session_expires_at_ms, space, membership, observed_event_seq, documents: indexed })
    }

    #[cfg(test)]
    pub(crate) fn install_snapshot_for_test(&self, snapshot: AuthorizedDescriptorSnapshot) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        let authority_generation = next_authority_generation().expect("test authority generation capacity");
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = Some(snapshot.authenticated_user_id.clone());
        self.authority_generation.store(authority_generation, Ordering::SeqCst);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Ready(Arc::new(snapshot));
        self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner).descriptor_ready(authority_generation);
        self.announce();
    }

    #[cfg(test)]
    pub(crate) fn install_catalog_for_test(&self, selections: Vec<AuthorizedPackageSelection>) {
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        assert_ne!(authority_generation, 0);
        let dialect_kinds = selections.iter().map(|selection| (selection.scope.clone(), selection.lease.parent_dialect.artifact_kind.clone())).collect();
        *self.catalog.write().unwrap_or_else(PoisonError::into_inner) = Some(Arc::new(AuthorizedCatalogSnapshot { authority_generation, selections, dialect_kinds }));
        self.announce();
    }
}

/// 🪜️ Whether the hub's role for the authenticated principal is one its account's membership admits: a human session
/// holds exactly its member row's role; an agent session holds at most it (the hub caps an agent at its delegation's
/// audience, so a `read` agent of an author account is a spectator), and no principal ever holds more.
fn principal_role_admitted(session_kind: DirectorySessionKindV1, principal: DirectorySpaceRole, membership: DirectorySpaceRole) -> bool {
    principal == membership || (session_kind == DirectorySessionKindV1::Agent && principal == DirectorySpaceRole::Spectator)
}

fn next_authority_generation() -> Result<u64, HubBindingError> {
    NEXT_HUB_AUTHORITY_GENERATION.try_update(Ordering::SeqCst, Ordering::SeqCst, |current| current.checked_add(1)).map_err(|_| HubBindingError::CapacityExceeded)
}

/// 🔑️ Exchanges an explicitly selected credential protocol with the shared bounded native transport.
#[cfg(not(target_arch = "wasm32"))]
pub fn exchange_delegated_session_v1(base_url: &str, credential: &crate::agent_credential::DelegatedCredentialV1, protocol: crate::agent_credential::CredentialExchangeProtocolV1) -> Result<crate::agent_credential::DelegatedSessionGrantV1, GatewayError> {
    use semio_framework_actor::{ActorId, PackageId};
    use semio_framework_async::{ProcessKind, ScopeOwner, TraceId, WorkerPoolConfig};
    use semio_framework_os_kernel::os_directory::client::{native::NativeDirectoryTransport, DirectoryTransport, HttpMethod};
    use semio_framework_os_services::{ComputePool, TokioHostRuntime};

    let origin = base_url.trim_end_matches('/');
    if origin != credential.origin().trim_end_matches('/') {
        return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "--hub origin does not match the origin this agent credential was issued for"));
    }
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("mcp-agent-session-exchange"), None);
    let compute = Arc::new(ComputePool::with_pool(1, pool));
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime.clone(), scope, compute, 1024 * 1024, 1, PackageId("semio-framework-os-mcp".to_string()), ActorId(0x4d43_5002));
    let cancel = semio_framework_async::CancelToken::root_now();
    let operation_now = runtime.block_on(runtime.now_ms());
    let ctx = OperationContext {
        actor: 0x4d43_5002,
        generation: 0,
        trace: TraceId(operation_now),
        lane: 1,
        deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
        cancel: cancel.child_now(),
        capability: None,
    };
    let request = (protocol.request)(credential, &agent_instance_id())?;
    let request = crate::agent_credential::CredentialExchangeRequestV1::new(request.path, request.body)?;
    let url = format!("{origin}{}", request.path);
    let mut response = runtime
        .block_on(transport.http(&ctx, HttpMethod::Post, &url, Some(credential.expose_for_exchange()), Some(request.body)))
        .map_err(|error| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("the hub at {origin} did not answer the agent-session exchange: {error:?}")).retryable())?;
    let outcome = if response.status != 200 { Err((protocol.decode_error)(response.status, &response.body)) } else { (protocol.decode_grant)(&response.body, credential) };
    response.body.fill(0);
    outcome
}

/// 🔖️ A per-process instance id, so two agents sharing one delegation still get separate sessions
/// and therefore separate presence rows.
#[cfg(not(target_arch = "wasm32"))]
fn agent_instance_id() -> String {
    format!("mcp.{}", std::process::id())
}

pub fn validate_hub_origin(base_url: &str, space_id: &str) -> Result<(), GatewayError> {
    pair::normalize_hub_origin(base_url).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "--hub requires a bounded origin-only http(s) URL"))?;
    validate_identity("space id", space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?;
    Ok(())
}

pub fn descriptor_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/descriptor", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_descriptor_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "descriptor" || parts.next().is_some() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

pub fn checkpoint_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/checkpoint", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_checkpoint_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "checkpoint" || parts.next().is_some() || space_id.is_empty() || document_id.is_empty() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

fn checked_base64_length(length: usize) -> Result<usize, CanonicalPairMountError> {
    length.checked_add(2).and_then(|value| value.checked_div(3)).and_then(|value| value.checked_mul(4)).ok_or(CanonicalPairMountError::ResourceLimit)
}

fn base64_encode_exact(bytes: &[u8], output_length: usize) -> Result<String, CanonicalPairMountError> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if checked_base64_length(bytes.len())? != output_length {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let mut output = String::with_capacity(output_length);
    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = *chunk.get(1).unwrap_or(&0);
        let third = *chunk.get(2).unwrap_or(&0);
        output.push(ALPHABET[(first >> 2) as usize] as char);
        output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
        output.push(if chunk.len() > 1 { ALPHABET[(((second & 15) << 2) | (third >> 6)) as usize] as char } else { '=' });
        output.push(if chunk.len() > 2 { ALPHABET[(third & 63) as usize] as char } else { '=' });
    }
    Ok(output)
}

fn canonical_checkpoint_resource_text(
    identity: &CanonicalPairMountIdentity,
    frontier: &semio_framework_os_kernel::os_directory::ArtifactFrontier,
    pack: &[u8],
    spr: &[u8],
) -> Result<String, CanonicalPairMountError> {
    let raw_length = pack.len().checked_add(spr.len()).ok_or(CanonicalPairMountError::ResourceLimit)?;
    if raw_length > pair::HUB_PAIR_MAX_VERIFIED_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    if frontier.document_id != identity.scope.document_id {
        return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint projection identity mismatch"));
    }
    let pack_base64_length = checked_base64_length(pack.len())?;
    let spr_base64_length = checked_base64_length(spr.len())?;
    let projected_upper_bound = pack_base64_length
        .checked_add(spr_base64_length)
        .and_then(|value| value.checked_add(CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES))
        .ok_or(CanonicalPairMountError::ResourceLimit)?;
    if projected_upper_bound > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let pack_base64 = base64_encode_exact(pack, pack_base64_length)?;
    let spr_base64 = base64_encode_exact(spr, spr_base64_length)?;
    let provenance = crate::schema::UntrustedProvenance {
        source: crate::schema::UntrustedSource::HubCheckpoint,
        artifact_id: Some(identity.scope.document_id.clone()),
        artifact_kind: None,
        space_id: Some(identity.scope.space_id.clone()),
        revision: crate::schema::UntrustedRevision { content_sha256: crate::schema::untrusted_content_sha256(&[pack, spr]), head_edit_id: Some(frontier.head_edit_id.clone()), commit_seq: Some(frontier.last_commit_seq) },
        authors: crate::schema::UntrustedAuthors::SpaceWriters { space_id: identity.scope.space_id.clone() },
    };
    let value = serde_json::json!({
        "schema": CANONICAL_CHECKPOINT_RESOURCE_SCHEMA,
        "scope": { "spaceId": identity.scope.space_id, "documentId": identity.scope.document_id },
        "descriptorDigestV1": identity.descriptor_digest_v1,
        "activeCheckpointId": identity.active_checkpoint_id,
        "etag": identity.etag,
        "authorityGeneration": identity.authority_generation,
        "catalogGeneration": identity.catalog_generation,
        "frontier": {
            "documentId": frontier.document_id,
            "headEditOrdinal": frontier.head_edit_ordinal,
            "headEditId": frontier.head_edit_id,
            "lastCommitSeq": frontier.last_commit_seq,
            "chainHash": hex_lower(&frontier.chain_hash.0)
        },
        "pack": { "byteLength": pack.len(), "sha256": framework_hash::sha256_hex(pack) },
        "spr": { "byteLength": spr.len(), "sha256": framework_hash::sha256_hex(spr) },
        "untrusted": crate::schema::untrusted_content(&provenance, serde_json::json!({ "packBase64": pack_base64, "sprBase64": spr_base64 }))
    });
    let text = serde_json::to_string(&value).map_err(|_| CanonicalPairMountError::InvalidResponse("canonical checkpoint resource serialization failed"))?;
    if text.len() > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    Ok(text)
}

fn validate_identity(field: &'static str, value: &str) -> Result<(), HubBindingError> {
    if value.is_empty() || value.len() > HUB_BINDING_ID_MAX_BYTES || value.chars().any(char::is_control) {
        return Err(HubBindingError::InvalidResponse(field));
    }
    Ok(())
}

fn validate_document_count(actual: usize, declared: u32) -> Result<(), HubBindingError> {
    if actual > HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS {
        return Err(HubBindingError::CapacityExceeded);
    }
    if usize::try_from(declared).ok() != Some(actual) {
        return Err(HubBindingError::InvalidResponse("space document count mismatch"));
    }
    Ok(())
}

fn map_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403 | 404, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) => HubBindingError::Unavailable(HubUnavailableCause::UndecodableResponse),
        DirectoryClientError::Http { status, body } => HubBindingError::Unavailable(HubUnavailableCause::http(status, &body)),
        DirectoryClientError::Transport(error) => HubBindingError::Unavailable(HubUnavailableCause::transport(&error)),
    }
}

fn map_catalog_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) => HubBindingError::InvalidResponse("execution-target response failed validation"),
        DirectoryClientError::Http { status: 404 | 409, .. } => HubBindingError::StaleRefresh,
        DirectoryClientError::Http { status, body } => HubBindingError::Unavailable(HubUnavailableCause::http(status, &body)),
        DirectoryClientError::Transport(error) => HubBindingError::Unavailable(HubUnavailableCause::transport(&error)),
    }
}

fn bounded_diagnostic(message: &str) -> String {
    if message.len() <= HUB_BINDING_DIAGNOSTIC_MAX_BYTES {
        return message.to_string();
    }
    let mut end = HUB_BINDING_DIAGNOSTIC_MAX_BYTES;
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message[..end].to_string()
}

/// 🏷️ The wire label of one refresh phase.
fn phase_label(phase: HubBindingPhase) -> &'static str {
    match phase {
        HubBindingPhase::Idle => "idle",
        HubBindingPhase::Authenticating => "authenticating",
        HubBindingPhase::LoadingSpace => "loading-space",
        HubBindingPhase::ValidatingDocuments => "validating-documents",
        HubBindingPhase::AwaitingNetworkBudget => "awaiting-network-budget",
        HubBindingPhase::Ready => "ready",
        HubBindingPhase::Revoked => "revoked",
    }
}

/// 📈️ What a call parked on an authority refresh hears while it waits, in English and German.
fn settle_progress_message(progress: &HubBindingProgress) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    match progress.phase {
        HubBindingPhase::AwaitingNetworkBudget => format!(
            "waiting for the network byte budget to refill ({:.1} of {:.1} MiB available) — warte auf das Auffüllen des Netzwerkkontingents ({:.1} von {:.1} MiB verfügbar)",
            progress.completed as f64 / MIB,
            progress.total as f64 / MIB,
            progress.completed as f64 / MIB,
            progress.total as f64 / MIB
        ),
        phase => format!(
            "waiting for the hub authority refresh ({} {}/{}) — warte auf die Aktualisierung der Hub-Berechtigungen ({} {}/{})",
            phase_label(phase),
            progress.completed,
            progress.total,
            phase_label(phase),
            progress.completed,
            progress.total
        ),
    }
}

/// 🔁️ Runs one canonical pair `attempt` until it stops failing for a reason a wait cures: an authority refresh in flight
/// (the attempt raced a directory event: waits for it to settle, `wait_ms` in all) or a spent network byte budget (waits
/// through `await_budget`). Every other outcome is the attempt's own. The first `artifact_open` right after a gateway
/// starts used to answer "canonical pair descriptor is unavailable" while the refresh a directory event started was still
/// running (ticket 26/09/23, session 14c).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn settled_pair_attempt<T>(
    binding: &HubRemoteBinding,
    wait_ms: u64,
    mut await_budget: impl FnMut() -> Result<(), CanonicalPairMountError>,
    mut attempt: impl FnMut() -> Result<T, CanonicalPairMountError>,
) -> Result<T, CanonicalPairMountError> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
    loop {
        let authority = binding.authority_generation.load(Ordering::SeqCst);
        match attempt() {
            Err(CanonicalPairMountError::BudgetExhausted) => await_budget()?,
            Err(error @ (CanonicalPairMountError::DescriptorUnavailable | CanonicalPairMountError::StaleCompletion)) if binding.settling() || binding.authority_generation.load(Ordering::SeqCst) != authority => {
                let Some(remaining) = deadline.checked_duration_since(std::time::Instant::now()).filter(|remaining| !remaining.is_zero()) else { return Err(error) };
                binding.await_settled(u64::try_from(remaining.as_millis()).unwrap_or(u64::MAX));
            }
            outcome => return outcome,
        }
    }
}

fn unavailable_gateway_error(state: HubRemoteBindingState, phase: HubBindingPhase, diagnostic: Option<String>) -> GatewayError {
    let label = match state {
        HubRemoteBindingState::Unbound => "unbound",
        HubRemoteBindingState::Refreshing => "refreshing",
        HubRemoteBindingState::Ready(_) => "expired",
        HubRemoteBindingState::Revoked => "revoked",
    };
    let phase = phase_label(phase);
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated hub descriptor index is {label}; retry after authority refresh"))
        .with_details(serde_json::json!({ "bindingState": label, "phase": phase, "lastFault": diagnostic }))
        .retryable()
}

pub(crate) fn percent_encode(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => output.push(char::from(byte)),
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = hex_digit(*bytes.get(index + 1)?)?;
            let low = hex_digit(*bytes.get(index + 2)?)?;
            decoded.push(high << 4 | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

//#region 🧩️ComponentAssets
/// 📶️ Where one execution-target component transfer is (schema `HubComponentTransferPhaseV1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubComponentTransferPhase {
    AwaitingStream,
    Receiving,
    Retrying,
}

impl HubComponentTransferPhase {
    pub fn label(self) -> &'static str {
        match self {
            Self::AwaitingStream => "awaiting-stream",
            Self::Receiving => "receiving",
            Self::Retrying => "retrying",
        }
    }
}

/// 📶️ One progress reading of a component transfer: its phase and how many of its bytes arrived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubComponentTransfer {
    pub phase: HubComponentTransferPhase,
    pub received_bytes: u64,
    pub total_bytes: u64,
}

impl HubComponentTransfer {
    pub fn fraction(&self) -> f64 {
        (self.received_bytes as f64 / self.total_bytes.max(1) as f64).clamp(0.0, 1.0)
    }

    /// 🗣️ What the call transferring `plugin_id`'s component hears, phase first, in English and German.
    pub fn message(&self, plugin_id: &str) -> String {
        const MIB: f64 = 1024.0 * 1024.0;
        let (received, total) = (self.received_bytes as f64 / MIB, self.total_bytes as f64 / MIB);
        let label = self.phase.label();
        match self.phase {
            HubComponentTransferPhase::AwaitingStream => format!("{label}: plugin component `{plugin_id}` ({total:.1} MiB) waits for a hub asset stream — Plugin-Komponente `{plugin_id}` ({total:.1} MiB) wartet auf einen Asset-Stream des Hubs"),
            HubComponentTransferPhase::Receiving => format!("{label}: plugin component `{plugin_id}` {received:.1} of {total:.1} MiB — Plugin-Komponente `{plugin_id}` {received:.1} von {total:.1} MiB"),
            HubComponentTransferPhase::Retrying => format!("{label}: plugin component `{plugin_id}` transfer interrupted, asking again — Übertragung der Plugin-Komponente `{plugin_id}` unterbrochen, neuer Versuch"),
        }
    }
}

/// 🚦️ Why one component stream attempt did not deliver the component.
#[cfg(not(target_arch = "wasm32"))]
enum ComponentStreamFault {
    Busy(u64),
    Transient(GatewayError),
    Final(GatewayError),
}

/// 🧩️ How a gateway gets the execution-target components its opens need, as static assets: from the per-user
/// execution-target store when it holds exactly the lease's bytes (no network at all — a restarted gateway opens every
/// kind it opened before with zero component bytes), else streamed from the hub over a pool of its own that the per-minute
/// API budget never meters, one stream at a time, verified against the lease and admitted to the store. The first open of a
/// plugin is therefore bound by bandwidth alone and reports its bytes as it goes.
#[cfg(not(target_arch = "wasm32"))]
pub struct HubComponentAssets {
    runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    credential: Arc<LocalHubCredential>,
    store: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore,
    serial: Mutex<()>,
}

#[cfg(not(target_arch = "wasm32"))]
impl HubComponentAssets {
    /// 🚰️ Component assets of `credential`'s hub kept in `store`, streamed over a fresh pool on `runtime`.
    pub fn new(runtime: Arc<semio_framework_os_services::TokioHostRuntime>, credential: Arc<LocalHubCredential>, store: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore) -> Self {
        let scope = runtime.open_scope_now(semio_framework_async::ScopeOwner::Service("mcp-execution-target-assets"), None);
        let compute = Arc::new(semio_framework_os_services::ComputePool::with_pool(HUB_COMPONENT_STREAMS, runtime.worker_pool().clone()));
        let transport = semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport::with_new_http_pool_now(
            runtime.clone(),
            scope,
            compute,
            HUB_COMPONENT_ASSET_BYTES_PER_MINUTE,
            HUB_COMPONENT_STREAMS,
            semio_framework_actor::PackageId("semio-framework-os-mcp.execution-target-assets".to_string()),
            semio_framework_actor::ActorId(0x4d43_5003),
        );
        Self { runtime, transport, credential, store, serial: Mutex::new(()) }
    }

    /// 🧩️ The component `expected` names for `intent`'s document: the store's verified copy, else the hub's stream. A busy
    /// hub (`429`) is a wait for the time it names, an interrupted stream or a hub shortage is asked again up to
    /// [`HUB_COMPONENT_STREAM_ATTEMPTS`] times, and bytes off the lease are refused and never kept. `report` hears every
    /// phase and every chunk; `cancel` ends the transfer between chunks.
    pub fn component(&self, intent: &DocumentOpenIntentV1, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1, cancel: &semio_framework_async::CancelToken, mut report: impl FnMut(&HubComponentTransfer)) -> Result<Vec<u8>, GatewayError> {
        if let Some(bytes) = self.store.component(expected) {
            return Ok(bytes);
        }
        let reading = |phase| HubComponentTransfer { phase, received_bytes: 0, total_bytes: expected.byte_length };
        let _serial = loop {
            match self.serial.try_lock() {
                Ok(guard) => break guard,
                Err(std::sync::TryLockError::Poisoned(poisoned)) => break poisoned.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) if cancel.is_cancelled_now() => return Err(component_cancelled()),
                Err(std::sync::TryLockError::WouldBlock) => {
                    report(&reading(HubComponentTransferPhase::AwaitingStream));
                    pause_unless_cancelled(cancel, 50);
                }
            }
        };
        if let Some(bytes) = self.store.component(expected) {
            return Ok(bytes);
        }
        let mut faults = 0u32;
        loop {
            match self.stream_once(intent, expected, cancel, &mut report) {
                Ok(bytes) => {
                    let _ = self.store.admit_component(&expected.sha256, &bytes);
                    return Ok(bytes);
                }
                Err(ComponentStreamFault::Final(error)) => return Err(error),
                Err(ComponentStreamFault::Busy(wait_ms)) => {
                    let until = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
                    while std::time::Instant::now() < until && !cancel.is_cancelled_now() {
                        report(&reading(HubComponentTransferPhase::AwaitingStream));
                        pause_unless_cancelled(cancel, until.saturating_duration_since(std::time::Instant::now()).as_millis().min(250) as u64);
                    }
                }
                Err(ComponentStreamFault::Transient(error)) => {
                    faults += 1;
                    if faults >= HUB_COMPONENT_STREAM_ATTEMPTS {
                        return Err(error);
                    }
                    report(&reading(HubComponentTransferPhase::Retrying));
                    pause_unless_cancelled(cancel, hub_refresh_retry_ms(faults));
                }
            }
            if cancel.is_cancelled_now() {
                return Err(component_cancelled());
            }
        }
    }

    /// 🌊️ One component stream from the hub, read whole and verified against the lease.
    fn stream_once(&self, intent: &DocumentOpenIntentV1, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1, cancel: &semio_framework_async::CancelToken, report: &mut impl FnMut(&HubComponentTransfer)) -> Result<Vec<u8>, ComponentStreamFault> {
        use semio_framework_os_kernel::os_directory::client::TransportError;
        let gateway = |error: DirectoryClientError| binding_error_to_gateway(map_client_error(error));
        let operation_now = self.runtime.block_on(self.runtime.now_ms());
        let stream_ms = HUB_COMPONENT_STREAM_ALLOWANCE_MS.saturating_add(expected.byte_length.saturating_mul(1_000) / HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND);
        let ctx = OperationContext {
            actor: 0x4d43_5003,
            generation: 0,
            trace: semio_framework_async::TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(stream_ms)),
            cancel: cancel.child_now(),
            capability: None,
        };
        let (head, mut body) = match self.runtime.block_on(self.transport.execution_target_component_stream(&ctx, &self.credential, intent)) {
            Ok(started) => started,
            Err(TransportError::Cancelled) => return Err(ComponentStreamFault::Final(component_cancelled())),
            Err(error) => return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(error)))),
        };
        if head.status != 200 {
            let mut refusal = Vec::new();
            while refusal.len() < 4_096 {
                match self.runtime.block_on(body.next_chunk()) {
                    Ok(Some(chunk)) => refusal.extend_from_slice(&chunk),
                    Ok(None) | Err(_) => break,
                }
            }
            refusal.truncate(4_096);
            return Err(match head.status {
                429 => ComponentStreamFault::Busy(hub_asset_refusal_wait_ms(&refusal)),
                401 => ComponentStreamFault::Final(gateway(DirectoryClientError::Unauthorized)),
                502..=504 => ComponentStreamFault::Transient(gateway(DirectoryClientError::Http { status: head.status, body: String::from_utf8_lossy(&refusal).into_owned() })),
                status => ComponentStreamFault::Final(gateway(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&refusal).into_owned() })),
            });
        }
        let declared = head.headers.iter().find(|(name, _)| name.eq_ignore_ascii_case("content-length")).and_then(|(_, value)| value.trim().parse::<u64>().ok());
        if declared.is_some_and(|declared| declared != expected.byte_length) {
            return Err(ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("hub announced {} component bytes where its own lease declared {}", declared.unwrap_or_default(), expected.byte_length))));
        }
        let mut bytes = Vec::with_capacity(usize::try_from(expected.byte_length).unwrap_or(0));
        report(&HubComponentTransfer { phase: HubComponentTransferPhase::Receiving, received_bytes: 0, total_bytes: expected.byte_length });
        loop {
            if cancel.is_cancelled_now() {
                return Err(ComponentStreamFault::Final(component_cancelled()));
            }
            match self.runtime.block_on(body.next_chunk()) {
                Ok(Some(chunk)) => {
                    if (bytes.len() + chunk.len()) as u64 > expected.byte_length {
                        return Err(ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("hub served more than the {} component bytes its own lease declared", expected.byte_length))));
                    }
                    bytes.extend_from_slice(&chunk);
                    report(&HubComponentTransfer { phase: HubComponentTransferPhase::Receiving, received_bytes: bytes.len() as u64, total_bytes: expected.byte_length });
                }
                Ok(None) => break,
                Err(_) if cancel.is_cancelled_now() => return Err(ComponentStreamFault::Final(component_cancelled())),
                Err(error) => return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(TransportError::Io(format!("component stream interrupted after {} of {} bytes: {error}", bytes.len(), expected.byte_length)))))),
            }
        }
        if bytes.len() as u64 != expected.byte_length {
            return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(TransportError::Io(format!("component stream ended after {} of {} bytes", bytes.len(), expected.byte_length))))));
        }
        semio_framework_os_kernel::os_directory::client::verify_execution_target_bytes("component", &expected.sha256, expected.byte_length, &bytes)
            .map_err(|_| ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, "hub execution-target component does not hash to the SHA-256 its own lease declared")))?;
        Ok(bytes)
    }
}

/// ⏳️ The wait a busy hub's typed asset refusal (`semio.hub.rate-limit-refusal/v1`) names, held to
/// [`HUB_COMPONENT_STREAM_RETRY_MAX_MS`]; [`HUB_COMPONENT_STREAM_RETRY_MS`] when the body names none.
#[cfg(not(target_arch = "wasm32"))]
fn hub_asset_refusal_wait_ms(body: &[u8]) -> u64 {
    serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .filter(|refusal| refusal["schema"] == "semio.hub.rate-limit-refusal/v1")
        .and_then(|refusal| refusal["retryAfterMs"].as_u64())
        .unwrap_or(HUB_COMPONENT_STREAM_RETRY_MS)
        .clamp(1, HUB_COMPONENT_STREAM_RETRY_MAX_MS)
}

#[cfg(not(target_arch = "wasm32"))]
fn component_cancelled() -> GatewayError {
    GatewayError::new(GatewayErrorCode::Cancelled, "the execution-target component transfer was cancelled")
}
//#endregion 🧩️ComponentAssets

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHubBindingDriver {
    cancel: semio_framework_async::CancelToken,
    thread: Option<std::thread::JoinHandle<()>>,
    runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    pair_transport: Arc<NativeCanonicalPairTransport<semio_framework_os_services::TokioHostRuntime>>,
    inference_transport: Arc<crate::inference::NativeInferenceHubTransport<semio_framework_os_services::TokioHostRuntime>>,
    /// 🪪️ The SAME authenticated client the binding refreshes through, held for the per-document execution-target lease.
    client: Arc<DirectoryClient<semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>>>,
    /// 💰️ The same transport, read for its byte budget: a transfer the budget cannot admit waits for the refill turn.
    budget: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    /// 🧩️ The execution-target components, fetched on demand as static assets. The catalog refresh pulls manifest and
    /// descriptor for every selected document; a component is pulled only when an open or an action needs its plugin — it
    /// is tens of megabytes per package and a session that never dispatches an action must not pay for it.
    assets: HubComponentAssets,
}

/// 💰️ Runs one authority refresh step on the binding actor, parking it on the network byte budget whenever the budget ran
/// out: the binding reports `AwaitingNetworkBudget`, and the step runs again (with a fresh operation context of its own)
/// once the refill admits a refresh — a spent budget is never a failed refresh and never clears the authority for good.
#[cfg(not(target_arch = "wasm32"))]
fn budgeted_refresh_step<T>(
    binding: &HubRemoteBinding,
    budget: &semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    cancel: &semio_framework_async::CancelToken,
    mut step: impl FnMut() -> Result<T, HubBindingError>,
) -> Result<T, HubBindingError> {
    loop {
        match step() {
            Err(HubBindingError::Unavailable(HubUnavailableCause::NetworkBudget)) => {
                await_network_budget(|| budget.network_budget(), HUB_REFRESH_NETWORK_BUDGET_BYTES, cancel, |remaining, needed| binding.note_network_budget_wait(remaining, needed))?;
            }
            outcome => return outcome,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    /// 💰️ The per-package byte budget must admit ONE authorized execution-target component plus
    /// the directory JSON around it. The previous flat 16 MiB was a directory-page budget: a real
    /// `gis` component is 47 MB, so every component fetch would have aborted mid-body with
    /// `ByteBudgetExhausted` — the budget is therefore derived from the Hub's own fixed component
    /// ceiling rather than from a number chosen when only JSON crossed this pool.
    pub fn connect(credential: Arc<LocalHubCredential>, base_url: &str, space_id: &str) -> Result<(Arc<HubRemoteBinding>, Self, Arc<dyn HubSocketGrantSource>), GatewayError> {
        use semio_framework_actor::{ActorId, PackageId};
        use semio_framework_async::{HostAsyncRuntime, ProcessKind, ScopeOwner, TraceId, WorkerPoolConfig};
        use semio_framework_os_kernel::os_directory::client::{native::NativeDirectoryTransport, DirectoryStreamTurn};
        use semio_framework_os_services::{ComputePool, TokioHostRuntime};

        validate_hub_origin(base_url, space_id)?;
        if base_url.trim_end_matches('/') != credential.hub_origin().trim_end_matches('/') {
            return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "--hub origin does not match the protected local credential"));
        }
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
        let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
        let scope = runtime.open_scope_now(ScopeOwner::Service("mcp-authenticated-hub-descriptor-index"), None);
        let compute = Arc::new(ComputePool::with_pool(2, pool));
        let transport = NativeDirectoryTransport::with_new_http_pool_now(
            runtime.clone(),
            scope,
            compute,
            semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES + 16 * 1024 * 1024,
            2,
            PackageId("semio-framework-os-mcp".to_string()),
            ActorId(0x4d43_5001),
        );
        let assets = HubComponentAssets::new(runtime.clone(), credential.clone(), semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user());
        let pair_transport = Arc::new(NativeCanonicalPairTransport::new(transport.clone(), credential.clone()));
        let inference_transport = Arc::new(crate::inference::NativeInferenceHubTransport::new(transport.clone(), credential.clone()));
        let budget = transport.clone();
        let client = Arc::new(DirectoryClient::authenticated(transport, credential));
        let grant_source: Arc<dyn HubSocketGrantSource> = client.clone();
        let binding = Arc::new(HubRemoteBinding::new(base_url, space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?);
        let cancel = semio_framework_async::CancelToken::root_now();
        let operation_now = runtime.block_on(runtime.now_ms());
        let ctx = OperationContext {
            actor: 0x4d43_5001,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
            cancel: cancel.child_now(),
            capability: None,
        };
        let descriptor_snapshot = runtime
            .block_on(binding.refresh(client.as_ref(), &ctx, wall_now_ms(), operation_now))
            .map_err(binding_error_to_gateway)?;
        runtime
            .block_on(binding.refresh_catalog(client.as_ref(), &descriptor_snapshot, &ctx))
            .map_err(binding_error_to_gateway)?;

        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { client: dial_client, since } = stream.turn(&ctx, operation_now) else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream did not enter its initial dial").retryable());
        };
        let authority_generation = binding.authority_generation.load(Ordering::SeqCst);
        let connection = dial_client.open_stream_ws(&ctx, since, 1_000).map_err(|error| {
            let refusal = directory_dial_refusal(&error);
            binding.invalidate(&refusal.message);
            refusal
        })?;
        match complete_authorized_directory_dial(&mut stream, &binding, &ctx, authority_generation, operation_now, connection) {
            DirectoryStreamTurn::Idle => {}
            DirectoryStreamTurn::Closed if ctx.cancel.is_cancelled_now() => {
                return Err(GatewayError::new(GatewayErrorCode::Cancelled, "hub directory stream activation was cancelled"));
            }
            DirectoryStreamTurn::Closed => {
                return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream authority changed before activation").retryable());
            }
            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Revoked(_) => {
                return Err(GatewayError::new(GatewayErrorCode::Internal, "hub directory stream returned an invalid activation turn"));
            }
        }

        let thread_cancel = cancel.clone();
        let thread_binding = binding.clone();
        let thread_runtime = runtime.clone();
        let thread_client = client.clone();
        let thread_budget = budget.clone();
        let thread = std::thread::Builder::new()
            .name("semio-mcp-hub-binding".to_string())
            .spawn(move || {
                let fresh_context = || {
                    let operation_now = thread_runtime.block_on(thread_runtime.now_ms());
                    let ctx = OperationContext {
                        actor: 0x4d43_5001,
                        generation: 0,
                        trace: TraceId(operation_now),
                        lane: 1,
                        deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
                        cancel: thread_cancel.child_now(),
                        capability: None,
                    };
                    (ctx, operation_now)
                };
                let refresh_authority = || {
                    budgeted_refresh_step(&thread_binding, &thread_budget, &thread_cancel, || {
                        let (ctx, operation_now) = fresh_context();
                        thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now))
                    })
                };
                let refresh_catalog = |snapshot: &Arc<AuthorizedDescriptorSnapshot>| {
                    budgeted_refresh_step(&thread_binding, &thread_budget, &thread_cancel, || {
                        let (ctx, _) = fresh_context();
                        thread_runtime.block_on(thread_binding.refresh_catalog(thread_client.as_ref(), snapshot, &ctx))
                    })
                };
                let mut needs_refresh = false;
                let mut refresh_failures: u32 = 0;
                while !thread_cancel.is_cancelled_now() {
                    let (ctx, operation_now) = fresh_context();
                    match stream.turn(&ctx, operation_now) {
                        DirectoryStreamTurn::Dial { client: dial_client, since } => {
                            if thread_binding.authority_generation.load(Ordering::SeqCst) == 0 {
                                match refresh_authority() {
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(_) => {
                                        match stream.complete_dial(
                                            operation_now,
                                            Err(semio_framework_os_kernel::os_directory::client::TransportError::Io("authenticated authority refresh failed before directory dial".to_string())),
                                        ) {
                                            DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                            DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                        }
                                        continue;
                                    }
                                    Ok(snapshot) => {
                                        if let Err(error) = refresh_catalog(&snapshot) {
                                            thread_binding.invalidate(&format!("authenticated Hub catalog refresh failed before directory dial: {error}"));
                                            refresh_failures = refresh_failures.saturating_add(1);
                                            pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                            continue;
                                        }
                                        refresh_failures = 0;
                                    }
                                }
                            }
                            let authority_generation = thread_binding.authority_generation.load(Ordering::SeqCst);
                            let (ctx, operation_now) = fresh_context();
                            match dial_client.open_stream_ws(&ctx, since, 1_000) {
                                Ok(connection) => {
                                    match complete_authorized_directory_dial(&mut stream, &thread_binding, &ctx, authority_generation, operation_now, connection) {
                                        DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) => break,
                                    }
                                }
                                Err(error) => {
                                    thread_binding.note_stream_fault(&directory_dial_refusal(&error).message);
                                    match stream.complete_dial(operation_now, Err(semio_framework_os_kernel::os_directory::client::TransportError::Io(error.to_string()))) {
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                    }
                                }
                            }
                        }
                        DirectoryStreamTurn::Message(message) => {
                            match thread_binding.observe_stream_message(&message) {
                                HubStreamObservation::Stable => {}
                                HubStreamObservation::RefreshRequired => needs_refresh = true,
                                HubStreamObservation::Revoked => break,
                            }
                        }
                        DirectoryStreamTurn::ReconnectAt(deadline) => {
                            thread_binding.invalidate_stream();
                            let wait = deadline.saturating_sub(operation_now).clamp(1, 25);
                            std::thread::sleep(std::time::Duration::from_millis(wait));
                        }
                        DirectoryStreamTurn::Idle if needs_refresh => {
                            match refresh_authority() {
                                Ok(snapshot) => match refresh_catalog(&snapshot) {
                                    Ok(_) => {
                                        needs_refresh = false;
                                        refresh_failures = 0;
                                    }
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(error) => {
                                        thread_binding.invalidate(&format!("authenticated Hub catalog refresh failed: {error}"));
                                        refresh_failures = refresh_failures.saturating_add(1);
                                        pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                    }
                                },
                                Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                Err(_) => {
                                    refresh_failures = refresh_failures.saturating_add(1);
                                    pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                }
                            }
                        }
                        DirectoryStreamTurn::Idle => std::thread::sleep(std::time::Duration::from_millis(10)),
                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) | DirectoryStreamTurn::DialScoped { .. } => break,
                    }
                }
                stream.close();
            })
            .map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "could not start the hub descriptor binding actor"))?;
        Ok((binding, Self { cancel, thread: Some(thread), runtime, pair_transport, inference_transport, client, budget, assets }, grant_source))
    }

    /// 🧩️ The plugin COMPONENT the Hub authorized for `scope`, bit-for-bit the one its lease names — exact byte length and
    /// exact SHA-256, compared before the bytes reach any runtime, because a component the Hub serves is executable code.
    /// This is the leg of the browser shell's own attach chain a native client never had: `open-plan` →
    /// `execution-target/{manifest, component, descriptor}`; it comes from the per-user store when that holds the bytes, else
    /// from the hub as a static asset ([`HubComponentAssets`]), and the calling tool hears the transfer as progress.
    pub fn fetch_execution_target_component(&self, plugin_id: &str, scope: &DocumentScope, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1) -> Result<Vec<u8>, GatewayError> {
        let intent = DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: None,
            client_instance_id: format!("mcp-component-{plugin_id}"),
        };
        let cancel = self.cancel.child_now();
        let mut published: Option<(HubComponentTransferPhase, u64, std::time::Instant)> = None;
        self.assets.component(&intent, expected, &cancel, |transfer| {
            if crate::notify::active_request_cancel_requested() {
                cancel.cancel_now();
            }
            let percent = transfer.received_bytes.saturating_mul(100) / transfer.total_bytes.max(1);
            let stale = published.is_none_or(|(phase, at, when)| phase != transfer.phase || at != percent || when.elapsed().as_millis() >= u128::from(HUB_COMPONENT_PROGRESS_KEEPALIVE_MS));
            if stale {
                published = Some((transfer.phase, percent, std::time::Instant::now()));
                crate::notify::publish_call_progress(transfer.fraction(), &transfer.message(plugin_id));
            }
        })
    }

    /// 🪪️ Fetches the execution-target LEASE the Hub minted for one exact document — the small JSON
    /// projection (≈3 KB, measured on hub 7621) that carries what the package-keyed catalog snapshot
    /// deliberately does not: this document's own `surface` (`surfaceId`/`appId`/`windowKindId`),
    /// its `artifact` kind and schema, and the grant behind them.
    ///
    /// 🧭️ `refresh_catalog` deduplicates its selections by PACKAGE identity, so its `scope` names
    /// whichever document first resolved that package — never necessarily the one a caller is
    /// opening. A per-document lease is therefore the only honest source for a per-document surface,
    /// and it is fetched here rather than derived: a surface id assembled host-side from a manifest
    /// would be this gateway's guess at what the hub authorized, which is exactly the class of
    /// fabrication `PROBE_SURFACE_ID` already was.
    pub fn fetch_execution_target_lease(&self, scope: &DocumentScope, client_instance_id: &str) -> Result<DocumentExecutionTargetLeaseFieldsV1, GatewayError> {
        let intent = DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: None,
            client_instance_id: client_instance_id.to_string(),
        };
        let lease = loop {
            self.await_transfer_budget(HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES)?;
            let (ctx, _) = self.operation_context(&self.cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
            match self.runtime.block_on(self.client.document_execution_target_manifest(&ctx, &intent)) {
                Err(DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::BudgetExhausted)) => {}
                outcome => break outcome.map_err(|error| binding_error_to_gateway(map_client_error(error)))?,
            }
        };
        if lease.scope != *scope {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "hub execution-target lease names a different document scope than the one it was requested for"));
        }
        Ok(lease)
    }

    /// 💰️ Waits on the calling tool thread until this transport's byte budget admits `needed` bytes, telling the call what
    /// it waits for as progress; the call's own cancellation or the driver's ends the wait. A transfer the budget cannot
    /// admit now starts after the refill instead of failing half-way.
    fn await_transfer_budget(&self, needed: u64) -> Result<(), GatewayError> {
        let cancel = self.cancel.child_now();
        let started = std::time::Instant::now();
        await_network_budget(|| self.budget.network_budget(), needed, &cancel, |remaining, needed| {
            if crate::notify::active_request_cancel_requested() {
                cancel.cancel_now();
            }
            let progress = HubBindingProgress { phase: HubBindingPhase::AwaitingNetworkBudget, completed: usize::try_from(remaining).unwrap_or(usize::MAX), total: usize::try_from(needed).unwrap_or(usize::MAX) };
            let refill = std::time::Duration::from_millis(semio_framework_os_services::HTTP_BUCKET_REFILL_INTERVAL_MS).as_secs_f64();
            crate::notify::publish_call_progress((started.elapsed().as_secs_f64() / refill).min(0.99), &settle_progress_message(&progress));
        })
        .map_err(binding_error_to_gateway)
    }

    /// 🧊️ One scope's canonical pair, mounted once the authority refresh in flight settled and the byte budget admits it
    /// (see [`settled_pair_attempt`]).
    fn mount_settled(&self, binding: &HubRemoteBinding, scope: &DocumentScope, cancel: &semio_framework_async::CancelToken) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        settled_pair_attempt(
            binding,
            HUB_AUTHORITY_SETTLE_WAIT_MS,
            || self.await_transfer_budget(pair::HUB_PAIR_MAX_VERIFIED_BYTES as u64).map_err(|_| CanonicalPairMountError::Cancelled),
            || {
                let (context, operation_now) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
                self.mount_canonical_pair(binding, scope, None, None, &context, wall_now_ms(), operation_now)
            },
        )
    }

    pub fn mount_canonical_pair(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        catalog_generation: Option<u64>,
        expected: Option<&CanonicalPairMountIdentity>,
        context: &OperationContext,
        wall_now_ms: i64,
        operation_now_ms: u64,
    ) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        self.runtime.block_on(binding.mount_canonical_pair(self.pair_transport.as_ref(), scope, catalog_generation, expected, context, wall_now_ms, operation_now_ms))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for NativeHubBindingDriver {
    fn drop(&mut self) {
        self.cancel.cancel_now();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// 🚪️ The refusal a hub binding answers when its directory stream cannot be dialled: retryable `PLUGIN_UNAVAILABLE` naming the
/// dial's own cause (a refused socket grant's status and body, a transport fault), so an agent and its operator see WHY the
/// authenticated snapshot was not activated instead of a bare "unavailable".
pub(crate) fn directory_dial_refusal(cause: &impl std::fmt::Display) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("hub directory stream is unavailable ({cause}); authenticated snapshot was not activated")).retryable()
}

fn binding_error_to_gateway(error: HubBindingError) -> GatewayError {
    let code = match error {
        HubBindingError::Cancelled => GatewayErrorCode::Cancelled,
        HubBindingError::CapacityExceeded => GatewayErrorCode::BudgetExceeded,
        HubBindingError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired => GatewayErrorCode::PermissionDenied,
        HubBindingError::DeadlineExceeded | HubBindingError::StaleRefresh | HubBindingError::Unavailable(_) => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = match &error {
        HubBindingError::Unavailable(cause) => GatewayError::new(code, error.to_string()).with_details(serde_json::json!({ "cause": cause.to_json(), "summary": cause.summary() })),
        _ => GatewayError::new(code, error.to_string()),
    };
    if matches!(code, GatewayErrorCode::PluginUnavailable) { gateway.retryable() } else { gateway }
}

fn wall_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_millis()).ok())
        .unwrap_or(i64::MAX)
}

#[cfg(not(target_arch = "wasm32"))]
fn complete_authorized_directory_dial<T: DirectoryTransport + Clone>(
    stream: &mut semio_framework_os_kernel::os_directory::client::DirectoryStream<T>,
    binding: &HubRemoteBinding,
    ctx: &OperationContext,
    expected_authority_generation: u64,
    operation_now_ms: u64,
    connection: T::Ws,
) -> semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn<T> {
    if ctx.cancel.is_cancelled_now()
        || expected_authority_generation == 0
        || binding.authority_generation.load(Ordering::SeqCst) != expected_authority_generation
    {
        stream.close();
    }
    stream.complete_dial(operation_now_ms, Ok(connection))
}

//#region 💡️Inference
/// ⏱️ The bounded deadline every authenticated inference route call runs under. It is the hub's own
/// fixed job lifetime plus one binding-operation timeout of slack, because `POST …/jobs` runs the
/// bounded deterministic service inline before it answers.
pub const HUB_INFERENCE_OPERATION_TIMEOUT_MS: u64 = 120_000 + HUB_BINDING_OPERATION_TIMEOUT_MS;

impl HubRemoteBinding {
    /// 🌐️ The normalized origin every protected inference request is pinned to.
    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    /// 🏷️ The one space this binding was constructed for; a caller never supplies another.
    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    /// 👤️ The live authenticated subject and coarse authority fence one inference job is owned by.
    pub fn inference_subject(&self, wall_now_ms: i64) -> Result<crate::inference::HubInferenceSubjectV1, GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(self.unavailable(HubRemoteBindingState::Refreshing));
        }
        Ok(crate::inference::HubInferenceSubjectV1 { hub_origin: self.hub_origin.clone(), space_id: self.space_id.clone(), user_id: snapshot.authenticated_user_id.clone(), authority_generation })
    }

    /// 📄️ Resolves one document id inside this binding's own space against the authenticated
    /// descriptor index — never a caller-supplied scope, never a document from another space.
    pub fn inference_document(&self, document_id: &str, wall_now_ms: i64) -> Result<(DocumentScope, AuthorizedDocumentView), GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let scope = DocumentScope::new(self.space_id.clone(), document_id.to_string());
        let document = snapshot
            .documents
            .get(&scope)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("document `{document_id}` is not in this authenticated hub space")))?;
        Ok((scope, document.clone()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    fn operation_context(&self, cancel: &semio_framework_async::CancelToken, timeout_ms: u64) -> (OperationContext, u64) {
        use semio_framework_async::TraceId;
        let operation_now = self.runtime.block_on(self.runtime.now_ms());
        let context = OperationContext {
            actor: 0x4d43_5002,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(timeout_ms)),
            cancel: cancel.child_now(),
            capability: None,
        };
        (context, operation_now)
    }

    /// 🌎️ The inference services the hub executes itself, read from its own readiness body — the one
    /// place a hub declares them, so a client never keeps a table of its own.
    pub fn read_hub_inference_services(&self, hub_origin: &str, cancel: &semio_framework_async::CancelToken) -> Result<Vec<crate::inference::HubInferenceServiceV1>, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::read_hub_inference_services(self.inference_transport.as_ref(), &context, hub_origin))
    }

    /// 📥️ Submits one closed client intent through the protected inference transport and blocks the
    /// synchronous MCP tool call until the hub answers or `cancel`/the deadline interrupts it.
    pub fn submit_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, request: &crate::inference::HubInferenceSubmitRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::HubInferenceJobReceiptV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::submit_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// 📤️ Reads one owner-private bounded event page.
    pub fn read_hub_inference_job_events(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, job_id: &str, after: u64, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::HubInferenceEventPageV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::read_hub_inference_job_events(self.inference_transport.as_ref(), &context, hub_origin, scope, route, job_id, after))
    }

    /// 🛑️ Records the owner's durable cancel request.
    pub fn cancel_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, job_id: &str, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::HubInferenceEventPageV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::cancel_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, job_id))
    }

    /// ✅️ Sends one explicit approval of an exact proposal hash.
    pub fn approve_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, request: &crate::inference::HubInferenceApprovalRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<crate::inference::HubInferenceApprovalReceiptV1, crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::approve_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// ↩️ Sends one Hub-minted durable GIS approval undo through the authenticated transport.
    pub fn undo_hub_inference_approval(
        &self,
        scope: &DocumentScope,
        hub_origin: &str,
        route: &str,
        request:&semio_framework_os_kernel::DslValue,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<semio_framework_os_kernel::DslValue,crate::inference::InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(crate::inference::undo_hub_inference_approval(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// 🧊️ Mounts the P4-C canonical checkpoint pair for one scope and projects exactly the frozen
    /// base identity an inference job is compared against: descriptor digest, active checkpoint,
    /// catalog generation, etag and the verified baseline frontier.
    pub fn hub_inference_base(&self, binding: &HubRemoteBinding, scope: &DocumentScope, cancel: &semio_framework_async::CancelToken) -> Result<crate::inference::HubInferenceBaseBindingV1, CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        let identity = mount.identity();
        let baseline = mount.baseline();
        Ok(crate::inference::HubInferenceBaseBindingV1 {
            hub_origin: identity.hub_origin.clone(),
            space_id: identity.scope.space_id.clone(),
            document_id: identity.scope.document_id.clone(),
            authority_generation: identity.authority_generation,
            descriptor_digest_v1: identity.descriptor_digest_v1.clone(),
            active_checkpoint_id: identity.active_checkpoint_id.clone(),
            etag: identity.etag.clone(),
            catalog_generation: identity.catalog_generation,
            head_edit_ordinal: baseline.head_edit_ordinal,
            head_edit_id: baseline.head_edit_id.clone(),
            last_commit_seq: baseline.last_commit_seq,
            chain_hash: hex_lower(&baseline.chain_hash.0),
        })
    }

    /// 🧪 Reads one authenticated frozen checkpoint through the protected pair transport and
    /// projects it to the bounded MCP resource codec without exposing the retained cache bytes.
    pub fn read_canonical_checkpoint(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<String, CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        if mount.identity().scope != *scope {
            return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint scope does not match its mount"));
        }
        binding.project_mounted_canonical_pair(&mount, wall_now_ms(), canonical_checkpoint_resource_text)
    }

    /// 📖️ One hub document's own canonical `pack`/`spr` bytes — the SAME verified mount
    /// [`Self::read_canonical_checkpoint`] takes, projected to the bytes instead of to the
    /// checkpoint's descriptive JSON.
    ///
    /// 🧊️ This is what `artifact_open`/`artifact_snapshot` of a hub document needs. Before ticket
    /// 26/09/18 slice M8 the workspace's hub arm answered a flat `canonical artifact bodies remain
    /// unavailable until P4-B`, so an agent bound to a real hub space could list the hub's documents
    /// and read their descriptors but never read one — measured live on hub 7631
    /// (`📓️m8-mcp-agent-third-participant.md` §2.3). Nothing new is fetched or trusted here: the
    /// mount is already digest-verified against the descriptor the authenticated binding published.
    pub fn read_canonical_pair_bytes(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<(Vec<u8>, Vec<u8>), CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        if mount.identity().scope != *scope {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair scope does not match its mount"));
        }
        binding.project_mounted_canonical_pair(&mount, wall_now_ms(), |_identity, _baseline, pack, spr| Ok((pack.to_vec(), spr.to_vec())))
    }
}

/// ⚠️ The typed gateway error one canonical pair mount failure answers with — the same closed
/// mapping `binding_error_to_gateway` establishes for descriptor refresh failures.
pub fn pair_mount_error_to_gateway(error: CanonicalPairMountError) -> GatewayError {
    let code = match error {
        CanonicalPairMountError::Cancelled => GatewayErrorCode::Cancelled,
        CanonicalPairMountError::ResourceLimit => GatewayErrorCode::BudgetExceeded,
        CanonicalPairMountError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        CanonicalPairMountError::Unauthorized => GatewayErrorCode::PermissionDenied,
        CanonicalPairMountError::DeadlineExceeded | CanonicalPairMountError::DescriptorUnavailable | CanonicalPairMountError::StaleCompletion | CanonicalPairMountError::Unavailable | CanonicalPairMountError::BudgetExhausted => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = GatewayError::new(code, error.to_string());
    if matches!(code, GatewayErrorCode::PluginUnavailable) {
        gateway.retryable()
    } else {
        gateway
    }
}
//#endregion 💡️Inference

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
