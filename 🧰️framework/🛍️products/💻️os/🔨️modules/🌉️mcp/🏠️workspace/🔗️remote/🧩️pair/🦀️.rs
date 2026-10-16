//! 🧩️ Binding-owned canonical checkpoint pair receiver, cache, and opaque mount.

#[cfg(any(test, feature = "testkit"))]
use std::sync::atomic::Ordering;
use semio_framework_async::CancelToken;
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::ArtifactFrontier;
use semio_framework_os_kernel::os_directory::ArtifactHash;
use semio_framework_os_kernel::os_directory::DocumentScope;
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::binding_errors::HubBindingError;

#[cfg(any(test, feature = "testkit"))]
static TEST_WIPED_BYTES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub const HUB_PAIR_MEDIA_TYPE: &str = "application/vnd.semio.canonical-checkpoint-pair.v1";

pub const HUB_PAIR_HEADER_MAX_BYTES: usize = 16 * 1024;

pub const HUB_PAIR_RECORD_BYTES: usize = 4 * 1024;

pub const HUB_PAIR_MAX_VERIFIED_BYTES: usize = 4 * 1024 * 1024;

pub const HUB_PAIR_CACHE_MAX_BYTES: usize = 8 * 1024 * 1024;

pub const HUB_PAIR_CACHE_MAX_ENTRIES: usize = 4;

const HUB_PAIR_SCOPE_MAX_BYTES: usize = 512;

const HUB_PAIR_ETAG_DOMAIN: &[u8] = b"semio.hub.canonical-checkpoint-pair-etag.v1\0";

const PAIR_HEADER: u8 = 1;

const PAIR_DATA: u8 = 2;

const PAIR_TERMINAL: u8 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalPairFetchRequest {
    pub hub_origin: String,
    pub path: String,
    pub accept: &'static str,
    pub maximum_response_bytes: usize,
}

pub struct CanonicalPairHttpResponse<B> {
    pub status: u16,
    pub content_type: String,
    pub etag: String,
    pub content_length: u64,
    pub body: B,
}

pub trait CanonicalPairBody: Send {
    async fn read(&mut self, context: &OperationContext, output: &mut [u8]) -> Result<usize, CanonicalPairMountError>;
}

pub trait CanonicalPairTransport: Send + Sync {
    type Body: CanonicalPairBody;
    async fn fetch(&self, context: &OperationContext, request: &CanonicalPairFetchRequest) -> Result<CanonicalPairHttpResponse<Self::Body>, CanonicalPairMountError>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeCanonicalPairTransport<R: semio_framework_async::HostAsyncRuntime> {
    pub transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>,
    pub credential: std::sync::Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime> NativeCanonicalPairTransport<R> {
    pub fn new(transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>, credential: std::sync::Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>) -> Self {
        Self { transport, credential }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeCanonicalPairBody {
    pub body: semio_framework_os_services::HttpPoolBody,
}

#[cfg(not(target_arch = "wasm32"))]
impl CanonicalPairBody for NativeCanonicalPairBody {
    async fn read(&mut self, context: &OperationContext, output: &mut [u8]) -> Result<usize, CanonicalPairMountError> {
        if context.cancel.is_cancelled_now() {
            return Err(CanonicalPairMountError::Cancelled);
        }
        let Some(mut chunk) = self.body.next_chunk().await.map_err(|error| {
            if context.cancel.is_cancelled_now() {
                CanonicalPairMountError::Cancelled
            } else if matches!(error, semio_framework_os_services::HttpPoolError::Compute(semio_framework_os_services::ComputeError::DeadlineExceeded)) {
                CanonicalPairMountError::DeadlineExceeded
            } else if matches!(error, semio_framework_os_services::HttpPoolError::ByteBudgetExhausted { .. }) {
                CanonicalPairMountError::BudgetExhausted
            } else {
                CanonicalPairMountError::Unavailable
            }
        })?
        else {
            return Ok(0);
        };
        if chunk.len() > output.len() {
            chunk.fill(0);
            return Err(CanonicalPairMountError::ResourceLimit);
        }
        let length = chunk.len();
        output[..length].copy_from_slice(&chunk);
        chunk.fill(0);
        Ok(length)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime + 'static> CanonicalPairTransport for NativeCanonicalPairTransport<R> {
    type Body = NativeCanonicalPairBody;

    async fn fetch(&self, context: &OperationContext, request: &CanonicalPairFetchRequest) -> Result<CanonicalPairHttpResponse<Self::Body>, CanonicalPairMountError> {
        if normalize_hub_origin(self.credential.hub_origin()).map_err(|_| CanonicalPairMountError::Unauthorized)? != request.hub_origin || request.accept != HUB_PAIR_MEDIA_TYPE || request.path.contains('?') || request.path.contains('#') {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair request authority mismatch"));
        }
        let url = format!("{}{}", request.hub_origin, request.path);
        let (head, body) = self.transport.fetch_protected_stream(context, self.credential.as_ref(), &url, request.accept).await.map_err(|error| match error {
            semio_framework_os_kernel::os_directory::client::TransportError::Cancelled => CanonicalPairMountError::Cancelled,
            semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded => CanonicalPairMountError::DeadlineExceeded,
            semio_framework_os_kernel::os_directory::client::TransportError::BudgetExhausted => CanonicalPairMountError::BudgetExhausted,
            semio_framework_os_kernel::os_directory::client::TransportError::Io(_) => CanonicalPairMountError::Unavailable,
        })?;
        let (content_type, etag, content_length) = if head.status == 200 {
            let content_type = exact_header(&head.headers, "content-type")?;
            let etag = exact_header(&head.headers, "etag")?;
            let content_length = exact_header(&head.headers, "content-length")?.parse::<u64>().map_err(|_| CanonicalPairMountError::InvalidResponse("canonical pair Content-Length is invalid"))?;
            (content_type, etag, content_length)
        } else {
            (String::new(), String::new(), 0)
        };
        if content_length > request.maximum_response_bytes as u64 {
            return Err(CanonicalPairMountError::ResourceLimit);
        }
        Ok(CanonicalPairHttpResponse { status: head.status, content_type, etag, content_length, body: NativeCanonicalPairBody { body } })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn exact_header(headers: &[(String, String)], expected: &str) -> Result<String, CanonicalPairMountError> {
    let mut matches = headers.iter().filter(|(name, _)| name.eq_ignore_ascii_case(expected));
    let value = matches.next().map(|(_, value)| value.clone()).ok_or(CanonicalPairMountError::InvalidResponse("canonical pair response omitted a required header"))?;
    if matches.next().is_some() || value.is_empty() || value.bytes().any(|byte| byte == b'\r' || byte == b'\n') {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair response header is ambiguous"));
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CanonicalPairMountIdentity {
    pub hub_origin: String,
    pub authority_generation: u64,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub active_checkpoint_id: String,
    pub etag: String,
    pub catalog_generation: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalPairMount {
    pub identity: CanonicalPairMountIdentity,
    pub baseline: ArtifactFrontier,
    pub mount_id: u64,
}

impl CanonicalPairMount {
    pub fn identity(&self) -> &CanonicalPairMountIdentity {
        &self.identity
    }

    pub fn baseline(&self) -> &ArtifactFrontier {
        &self.baseline
    }

    pub fn opaque_id(&self) -> u64 {
        self.mount_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalPairMountStage {
    DescriptorReady,
    Loading,
    Receiving,
    Verifying,
    Mounted,
    Refreshing,
    Revoked,
    Closed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalPairMountProgress {
    pub stage: CanonicalPairMountStage,
    pub completed: u64,
    pub total: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalPairMountError {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    DescriptorUnavailable,
    StaleCompletion,
    ResourceLimit,
    InvalidResponse(&'static str),
    Unavailable,
    /// 💰️ The gateway's network byte budget ran out before the pair arrived; the same mount succeeds after the refill.
    BudgetExhausted,
}

impl std::fmt::Display for CanonicalPairMountError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Cancelled => "canonical pair receipt was cancelled",
            Self::DeadlineExceeded => "canonical pair receipt exceeded its deadline",
            Self::Unauthorized => "canonical pair authority was revoked",
            Self::DescriptorUnavailable => "canonical pair descriptor is unavailable",
            Self::StaleCompletion => "canonical pair receipt was superseded",
            Self::ResourceLimit => "canonical pair receipt exceeded its fixed memory budget",
            Self::InvalidResponse(detail) => detail,
            Self::Unavailable => "canonical pair transport is unavailable",
            Self::BudgetExhausted => "canonical pair transfer waits for the network byte budget to refill",
        })
    }
}

impl std::error::Error for CanonicalPairMountError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalPairActorState {
    Empty,
    DescriptorReady,
    Loading,
    Verified,
    Mounted,
    Refreshing,
    Revoked,
    Closed,
}

struct PairBytes {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

struct WipeVec(pub Vec<u8>);

impl Drop for WipeVec {
    fn drop(&mut self) {
        #[cfg(test)]
        let length = self.0.len();
        self.0.fill(0);
        #[cfg(test)]
        TEST_WIPED_BYTES.fetch_add(length as u64, Ordering::SeqCst);
    }
}

struct WipeScratch(pub [u8; 16 * 1024]);

impl Drop for WipeScratch {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

struct PairReceipt {
    pub id: u64,
    pub binding_generation: u64,
    pub scope: DocumentScope,
    pub cancel: CancelToken,
}

struct LoadingReceipt {
    pub id: u64,
    pub binding_generation: u64,
    pub expected: Option<CanonicalPairMountIdentity>,
    pub cancel: CancelToken,
    pub completion: tokio::sync::watch::Sender<PairCompletion>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PairCompletion {
    Pending,
    Published(CanonicalPairMountIdentity),
    Failed,
}

pub enum PairBegin {
    Owner(PairReceipt),
    Join(tokio::sync::watch::Receiver<PairCompletion>),
}

impl PairBytes {
    pub fn len(&self) -> usize {
        self.pack.len() + self.spr.len()
    }
}

impl Drop for PairBytes {
    fn drop(&mut self) {
        #[cfg(test)]
        let length = self.len();
        self.pack.fill(0);
        self.spr.fill(0);
        #[cfg(test)]
        TEST_WIPED_BYTES.fetch_add(length as u64, Ordering::SeqCst);
    }
}

struct VerifiedCanonicalPair {
    pub identity: CanonicalPairMountIdentity,
    pub baseline: ArtifactFrontier,
    pub bytes: PairBytes,
}

struct CacheEntry {
    pub pair: VerifiedCanonicalPair,
    pub mount_id: u64,
}

pub struct CanonicalPairActor {
    pub hub_origin: String,
    pub state: CanonicalPairActorState,
    pub binding_generation: u64,
    pub loadings: HashMap<DocumentScope, LoadingReceipt>,
    pub mounted: Option<CanonicalPairMount>,
    pub cache: VecDeque<CacheEntry>,
    pub cached_bytes: usize,
    next_mount_id: u64,
    next_receipt_id: u64,
    pub progress: CanonicalPairMountProgress,
    #[cfg(test)]
    pub cache_hits: u64,
    #[cfg(test)]
    pub cache_misses: u64,
}

impl CanonicalPairActor {
    pub fn new(hub_origin: String) -> Self {
        Self {
            hub_origin,
            state: CanonicalPairActorState::Empty,
            binding_generation: 0,
            loadings: HashMap::new(),
            mounted: None,
            cache: VecDeque::new(),
            cached_bytes: 0,
            next_mount_id: 1,
            next_receipt_id: 1,
            progress: CanonicalPairMountProgress { stage: CanonicalPairMountStage::Refreshing, completed: 0, total: 0 },
            #[cfg(test)]
            cache_hits: 0,
            #[cfg(test)]
            cache_misses: 0,
        }
    }

    pub fn descriptor_ready(&mut self, generation: u64) {
        self.cancel_loading();
        self.clear_verified();
        self.binding_generation = generation;
        self.state = CanonicalPairActorState::DescriptorReady;
        self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::DescriptorReady, completed: 0, total: 0 };
    }

    pub fn invalidate(&mut self, state: CanonicalPairActorState) {
        self.cancel_loading();
        self.clear_verified();
        self.state = state;
        self.progress = CanonicalPairMountProgress {
            stage: match state {
                CanonicalPairActorState::Revoked => CanonicalPairMountStage::Revoked,
                CanonicalPairActorState::Closed => CanonicalPairMountStage::Closed,
                _ => CanonicalPairMountStage::Refreshing,
            },
            completed: 0,
            total: 0,
        };
    }

    /// 🤝️ Opens the one receipt a scope loads under, or joins the receipt already loading it: every
    /// concurrent reader of a scope waits for the same fetch, and a reader that expects an exact identity
    /// checks it against what that receipt publishes instead of being refused while it is in flight.
    pub fn begin(&mut self, generation: u64, scope: &DocumentScope, expected: Option<&CanonicalPairMountIdentity>, parent_cancel: &CancelToken) -> Result<PairBegin, CanonicalPairMountError> {
        if let Some(loading) = self.loadings.get(scope) {
            return Ok(PairBegin::Join(loading.completion.subscribe()));
        }
        if !matches!(self.state, CanonicalPairActorState::DescriptorReady | CanonicalPairActorState::Mounted | CanonicalPairActorState::Loading) {
            return Err(CanonicalPairMountError::DescriptorUnavailable);
        }
        if self.binding_generation != generation {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let cancel = parent_cancel.child_now();
        let id = self.next_receipt_id;
        self.next_receipt_id = self.next_receipt_id.checked_add(1).ok_or(CanonicalPairMountError::ResourceLimit)?;
        let (completion, _) = tokio::sync::watch::channel(PairCompletion::Pending);
        self.loadings.insert(scope.clone(), LoadingReceipt { id, binding_generation: generation, expected: expected.cloned(), cancel: cancel.clone(), completion });
        self.mounted = None;
        self.state = CanonicalPairActorState::Loading;
        self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::Loading, completed: 0, total: 1 };
        Ok(PairBegin::Owner(PairReceipt { id, binding_generation: generation, scope: scope.clone(), cancel }))
    }

    pub fn report(&mut self, stage: CanonicalPairMountStage, completed: u64, total: u64) {
        if self.state == CanonicalPairActorState::Loading || self.state == CanonicalPairActorState::Verified {
            self.progress = CanonicalPairMountProgress { stage, completed, total };
        }
    }

    pub fn cancel_receipt(&mut self, receipt: &PairReceipt) {
        let matches = self.loadings.get(&receipt.scope).is_some_and(|loading| loading.id == receipt.id && loading.binding_generation == receipt.binding_generation);
        if !matches {
            return;
        }
        if let Some(loading) = self.loadings.remove(&receipt.scope) {
            loading.cancel.cancel_now();
            loading.completion.send_replace(PairCompletion::Failed);
        }
        self.state = if self.loadings.is_empty() {
            if self.mounted.is_some() {
                CanonicalPairActorState::Mounted
            } else {
                CanonicalPairActorState::DescriptorReady
            }
        } else {
            CanonicalPairActorState::Loading
        };
        if self.state == CanonicalPairActorState::DescriptorReady {
            self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::DescriptorReady, completed: 0, total: 0 };
        }
    }

    pub fn publish(&mut self, receipt: &PairReceipt, pair: VerifiedCanonicalPair) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        let matches = self.loadings.get(&receipt.scope).is_some_and(|loading| loading.id == receipt.id && loading.binding_generation == receipt.binding_generation);
        if !matches || receipt.scope != pair.identity.scope || self.binding_generation != pair.identity.authority_generation || self.hub_origin != pair.identity.hub_origin {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let cache_index = self.cache.iter().position(|entry| entry.pair.identity == pair.identity);
        let next_mount_id = if cache_index.is_none() {
            if pair.bytes.len() > HUB_PAIR_MAX_VERIFIED_BYTES || pair.bytes.len() > HUB_PAIR_CACHE_MAX_BYTES {
                return Err(CanonicalPairMountError::ResourceLimit);
            }
            Some(self.next_mount_id.checked_add(1).ok_or(CanonicalPairMountError::ResourceLimit)?)
        } else {
            None
        };
        let loading = self.loadings.remove(&receipt.scope).expect("matched canonical pair receipt");
        self.state = CanonicalPairActorState::Verified;
        self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::Verifying, completed: 1, total: 1 };
        let mount = if let Some(index) = cache_index {
            let entry = self.cache.remove(index).expect("located canonical pair cache entry");
            let mount = CanonicalPairMount { identity: entry.pair.identity.clone(), baseline: entry.pair.baseline.clone(), mount_id: entry.mount_id };
            self.cache.push_front(entry);
            #[cfg(test)]
            {
                self.cache_hits += 1;
            }
            mount
        } else {
            let length = pair.bytes.len();
            while self.cache.len() >= HUB_PAIR_CACHE_MAX_ENTRIES || self.cached_bytes.saturating_add(length) > HUB_PAIR_CACHE_MAX_BYTES {
                let evicted = self.cache.pop_back().ok_or(CanonicalPairMountError::ResourceLimit)?;
                self.cached_bytes = self.cached_bytes.saturating_sub(evicted.pair.bytes.len());
            }
            let mount_id = self.next_mount_id;
            self.next_mount_id = next_mount_id.expect("new canonical pair preflighted a mount id");
            let mount = CanonicalPairMount { identity: pair.identity.clone(), baseline: pair.baseline.clone(), mount_id };
            self.cached_bytes += length;
            self.cache.push_front(CacheEntry { pair, mount_id });
            #[cfg(test)]
            {
                self.cache_misses += 1;
            }
            mount
        };
        loading.completion.send_replace(PairCompletion::Published(mount.identity.clone()));
        self.mounted = Some(mount.clone());
        self.state = if self.loadings.is_empty() { CanonicalPairActorState::Mounted } else { CanonicalPairActorState::Loading };
        self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::Mounted, completed: 1, total: 1 };
        Ok(mount)
    }

    fn cancel_loading(&mut self) {
        for (_, loading) in self.loadings.drain() {
            loading.cancel.cancel_now();
            loading.completion.send_replace(PairCompletion::Failed);
        }
    }

    pub fn cached(&mut self, identity: &CanonicalPairMountIdentity) -> Option<CanonicalPairMount> {
        let index = self.cache.iter().position(|entry| &entry.pair.identity == identity)?;
        let entry = self.cache.remove(index).expect("located canonical pair cache entry");
        let mount = CanonicalPairMount { identity: entry.pair.identity.clone(), baseline: entry.pair.baseline.clone(), mount_id: entry.mount_id };
        self.cache.push_front(entry);
        self.mounted = Some(mount.clone());
        self.state = if self.loadings.is_empty() { CanonicalPairActorState::Mounted } else { CanonicalPairActorState::Loading };
        self.progress = CanonicalPairMountProgress { stage: CanonicalPairMountStage::Mounted, completed: 1, total: 1 };
        #[cfg(test)]
        {
            self.cache_hits += 1;
        }
        Some(mount)
    }

    fn clear_verified(&mut self) {
        self.mounted = None;
        self.cache.clear();
        self.cached_bytes = 0;
    }

    pub fn project_mounted<T>(
        &self,
        mount: &CanonicalPairMount,
        project: impl FnOnce(&CanonicalPairMountIdentity, &ArtifactFrontier, &[u8], &[u8]) -> Result<T, CanonicalPairMountError>,
    ) -> Result<T, CanonicalPairMountError> {
        if self.binding_generation != mount.identity.authority_generation
            || !matches!(self.state, CanonicalPairActorState::Mounted | CanonicalPairActorState::Loading)
            || self.mounted.as_ref().is_none_or(|active| active.identity != mount.identity || active.mount_id != mount.mount_id)
        {
            return Err(CanonicalPairMountError::StaleCompletion);
        }
        let entry = self
            .cache
            .iter()
            .find(|entry| entry.mount_id == mount.mount_id && entry.pair.identity == mount.identity)
            .ok_or(CanonicalPairMountError::StaleCompletion)?;
        project(&entry.pair.identity, &entry.pair.baseline, &entry.pair.bytes.pack, &entry.pair.bytes.spr)
    }
}

impl Drop for CanonicalPairActor {
    fn drop(&mut self) {
        self.invalidate(CanonicalPairActorState::Closed);
    }
}

pub fn validate_expected_identity(identity: &CanonicalPairMountIdentity, hub_origin: &str, authority_generation: u64, scope: &DocumentScope, descriptor_digest_v1: &str, catalog_generation: Option<u64>) -> Result<(), CanonicalPairMountError> {
    if identity.hub_origin != hub_origin || identity.authority_generation != authority_generation || identity.scope != *scope || identity.descriptor_digest_v1 != descriptor_digest_v1 || identity.catalog_generation != catalog_generation {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair cache identity does not match the active binding"));
    }
    Ok(())
}

/// ⏳️ A joined reader waits for the receipt's owner to publish or fail — woken by the completion itself or by its own cancel,
/// never by a timer: the gateway drives mounts on the framework executor (`TokioHostRuntime::block_on`), which runs no Tokio
/// reactor, and the 10 ms Tokio sleep this loop used to poll with panicked there, so every second concurrent reader of a
/// loading document died "without answering" (live quartet on 7800/p33, ticket 26/09/23 G12 session 14c). The owner's own
/// deadline — the same operation timeout, started earlier — bounds the wait; every wake re-checks this reader's deadline.
pub async fn wait_for_published_receipt(mut completion: tokio::sync::watch::Receiver<PairCompletion>, context: &OperationContext, operation_now_ms: u64, started: std::time::Instant) -> Result<CanonicalPairMountIdentity, CanonicalPairMountError> {
    loop {
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        match completion.borrow_and_update().clone() {
            PairCompletion::Published(identity) => return Ok(identity),
            PairCompletion::Failed => return Err(CanonicalPairMountError::StaleCompletion),
            PairCompletion::Pending => {}
        }
        tokio::select! {
            changed = completion.changed() => changed.map_err(|_| CanonicalPairMountError::StaleCompletion)?,
            () = context.cancel.cancelled() => return Err(CanonicalPairMountError::Cancelled),
        }
    }
}

pub fn checkpoint(context: &OperationContext, operation_now_ms: u64) -> Result<(), CanonicalPairMountError> {
    if context.cancel.is_cancelled_now() {
        return Err(CanonicalPairMountError::Cancelled);
    }
    if context.deadline_ms.is_some_and(|deadline| operation_now_ms >= deadline) {
        return Err(CanonicalPairMountError::DeadlineExceeded);
    }
    Ok(())
}

pub fn receipt_now(operation_now_ms: u64, started: std::time::Instant) -> u64 {
    operation_now_ms.saturating_add(u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX))
}

pub fn receipt_wall_now(wall_now_ms: i64, started: std::time::Instant) -> i64 {
    wall_now_ms.saturating_add(i64::try_from(started.elapsed().as_millis()).unwrap_or(i64::MAX))
}

pub async fn receive_body<B: CanonicalPairBody>(
    content_length: u64,
    mut body: B,
    maximum: usize,
    context: &OperationContext,
    operation_now_ms: u64,
    started: std::time::Instant,
    mut report: impl FnMut(u64, u64),
) -> Result<WipeVec, CanonicalPairMountError> {
    let length = usize::try_from(content_length).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
    if length == 0 || length > maximum {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let mut wire = WipeVec(Vec::with_capacity(length));
    let mut scratch = WipeScratch([0u8; 16 * 1024]);
    while wire.0.len() < length {
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        let remaining = length - wire.0.len();
        let limit = remaining.min(scratch.0.len());
        let read = body.read(context, &mut scratch.0[..limit]).await?;
        if read == 0 || read > limit {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair body length mismatch"));
        }
        wire.0.extend_from_slice(&scratch.0[..read]);
        scratch.0[..read].fill(0);
        report(wire.0.len() as u64, content_length);
    }
    let trailing_read = body.read(context, &mut scratch.0[..1]).await?;
    scratch.0[0] = 0;
    if trailing_read != 0 {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair body exceeds Content-Length"));
    }
    Ok(wire)
}

pub fn normalize_hub_origin(origin: &str) -> Result<String, HubBindingError> {
    if origin.is_empty() || origin.len() > 2_048 || origin.trim() != origin {
        return Err(HubBindingError::InvalidResponse("hub origin"));
    }
    let uri: axum::http::Uri = origin.parse().map_err(|_| HubBindingError::InvalidResponse("hub origin"))?;
    let scheme = uri.scheme_str().ok_or(HubBindingError::InvalidResponse("hub origin"))?;
    let authority = uri.authority().ok_or(HubBindingError::InvalidResponse("hub origin"))?;
    if !matches!(scheme, "http" | "https") || !matches!(uri.path(), "" | "/") || uri.query().is_some() {
        return Err(HubBindingError::InvalidResponse("hub origin"));
    }
    Ok(format!("{}://{}", scheme.to_ascii_lowercase(), authority.as_str().to_ascii_lowercase()))
}

struct Cursor<'a> {
    pub bytes: &'a [u8],
    pub offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn take(&mut self, length: usize) -> Result<&'a [u8], CanonicalPairMountError> {
        let end = self.offset.checked_add(length).ok_or(CanonicalPairMountError::ResourceLimit)?;
        let value = self.bytes.get(self.offset..end).ok_or(CanonicalPairMountError::InvalidResponse("canonical pair is truncated"))?;
        self.offset = end;
        Ok(value)
    }

    pub fn byte(&mut self) -> Result<u8, CanonicalPairMountError> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32, CanonicalPairMountError> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().map_err(|_| CanonicalPairMountError::InvalidResponse("invalid u32 field"))?))
    }

    pub fn u64(&mut self) -> Result<u64, CanonicalPairMountError> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().map_err(|_| CanonicalPairMountError::InvalidResponse("invalid u64 field"))?))
    }

    fn text_with_empty(&mut self, allow_empty: bool) -> Result<String, CanonicalPairMountError> {
        let length = usize::try_from(self.u32()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
        if (!allow_empty && length == 0) || length > HUB_PAIR_SCOPE_MAX_BYTES {
            return Err(CanonicalPairMountError::ResourceLimit);
        }
        let text = std::str::from_utf8(self.take(length)?).map_err(|_| CanonicalPairMountError::InvalidResponse("canonical pair text is not UTF-8"))?;
        if text.chars().any(char::is_control) {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair text contains a control character"));
        }
        Ok(text.to_string())
    }

    pub fn text(&mut self) -> Result<String, CanonicalPairMountError> {
        self.text_with_empty(false)
    }

    fn baseline_head(&mut self) -> Result<String, CanonicalPairMountError> {
        self.text_with_empty(true)
    }

    pub fn hash(&mut self) -> Result<[u8; 32], CanonicalPairMountError> {
        let hash: [u8; 32] = self.take(32)?.try_into().map_err(|_| CanonicalPairMountError::InvalidResponse("invalid hash width"))?;
        if hash == [0; 32] {
            Err(CanonicalPairMountError::InvalidResponse("zero hash is not canonical"))
        } else {
            Ok(hash)
        }
    }

    fn baseline_chain_hash(&mut self) -> Result<[u8; 32], CanonicalPairMountError> {
        self.take(32)?.try_into().map_err(|_| CanonicalPairMountError::InvalidResponse("invalid hash width"))
    }
}

fn frame<'a>(cursor: &mut Cursor<'a>, maximum: usize) -> Result<&'a [u8], CanonicalPairMountError> {
    let length = usize::try_from(cursor.u32()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
    if length == 0 || length > maximum {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    cursor.take(length)
}

pub fn decode_response(
    content_type: &str,
    etag: &str,
    wire: WipeVec,
    hub_origin: &str,
    authority_generation: u64,
    expected_scope: &DocumentScope,
    expected_descriptor_digest: &str,
    catalog_generation: Option<u64>,
    context: &OperationContext,
    operation_now_ms: u64,
    started: std::time::Instant,
    mut report: impl FnMut(u64, u64),
) -> Result<VerifiedCanonicalPair, CanonicalPairMountError> {
    if content_type != HUB_PAIR_MEDIA_TYPE {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair media type mismatch"));
    }
    checkpoint(context, receipt_now(operation_now_ms, started))?;
    let total_wire = wire.0.len() as u64;
    let mut input = Cursor { bytes: &wire.0, offset: 0 };
    let header = frame(&mut input, HUB_PAIR_HEADER_MAX_BYTES)?;
    let mut cursor = Cursor { bytes: header, offset: 0 };
    if cursor.byte()? != PAIR_HEADER || cursor.u32()? != 1 {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair header version mismatch"));
    }
    let scope = DocumentScope::new(cursor.text()?, cursor.text()?);
    let descriptor_digest = cursor.hash()?;
    let active_checkpoint_id = cursor.hash()?;
    let frontier_document_id = cursor.text()?;
    let head_edit_ordinal = cursor.u64()?;
    let head_edit_id = cursor.baseline_head()?;
    let last_commit_seq = cursor.u64()?;
    let chain_hash = cursor.baseline_chain_hash()?;
    let pack_hash = cursor.hash()?;
    let pack_length = usize::try_from(cursor.u64()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
    let spr_hash = cursor.hash()?;
    let spr_length = usize::try_from(cursor.u64()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
    let aggregate_hash = cursor.hash()?;
    if cursor.offset != header.len() || scope != *expected_scope || hex(&descriptor_digest) != expected_descriptor_digest {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair authority identity mismatch"));
    }
    let baseline = ArtifactFrontier { document_id: frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash: ArtifactHash::new(chain_hash) };
    if !(baseline.is_genesis_for(&scope) || baseline.is_edited_for(&scope)) {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair baseline frontier mismatch"));
    }
    let pair_length = pack_length.checked_add(spr_length).ok_or(CanonicalPairMountError::ResourceLimit)?;
    if pack_length == 0 || spr_length == 0 || pair_length > HUB_PAIR_MAX_VERIFIED_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let computed_etag = canonical_etag(header);
    if etag != computed_etag {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair ETag mismatch"));
    }
    let expected_records = pack_length.div_ceil(HUB_PAIR_RECORD_BYTES).checked_add(spr_length.div_ceil(HUB_PAIR_RECORD_BYTES)).ok_or(CanonicalPairMountError::ResourceLimit)?;
    let mut pack = WipeVec(Vec::with_capacity(pack_length));
    let mut spr = WipeVec(Vec::with_capacity(spr_length));
    for ordinal in 0..expected_records {
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        let record_frame = frame(&mut input, HUB_PAIR_RECORD_BYTES + 18)?;
        let mut record = Cursor { bytes: record_frame, offset: 0 };
        if record.byte()? != PAIR_DATA {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record order mismatch"));
        }
        let part = record.byte()?;
        if usize::try_from(record.u32()?).ok() != Some(ordinal) {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record order mismatch"));
        }
        let offset = usize::try_from(record.u64()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
        let length = usize::try_from(record.u32()?).map_err(|_| CanonicalPairMountError::ResourceLimit)?;
        if length == 0 || length > HUB_PAIR_RECORD_BYTES {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record length mismatch"));
        }
        let bytes = record.take(length)?;
        if record.offset != record_frame.len() {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record has trailing bytes"));
        }
        let target = match part {
            1 if pack.0.len() < pack_length => &mut pack,
            2 if pack.0.len() == pack_length => &mut spr,
            _ => return Err(CanonicalPairMountError::InvalidResponse("canonical pair part order mismatch")),
        };
        if offset != target.0.len() || target.0.len().checked_add(length).is_none() {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record offset mismatch"));
        }
        target.0.extend_from_slice(bytes);
        if target.0.len() > if part == 1 { pack_length } else { spr_length } {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair record exceeds declared part"));
        }
        report(input.offset as u64, total_wire);
    }
    let terminal = frame(&mut input, 2)?;
    if terminal != [PAIR_TERMINAL, 0] || input.offset != wire.0.len() || pack.0.len() != pack_length || spr.0.len() != spr_length {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair terminal or total length mismatch"));
    }
    if checked_hash(&pack.0, context, operation_now_ms, started)? != pack_hash || checked_hash(&spr.0, context, operation_now_ms, started)? != spr_hash {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair part hash mismatch"));
    }
    let mut aggregate = framework_hash::Sha256::new();
    for chunk in pack.0.chunks(16 * 1024).chain(spr.0.chunks(16 * 1024)) {
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        aggregate.update(chunk);
    }
    if aggregate.finalize() != aggregate_hash {
        return Err(CanonicalPairMountError::InvalidResponse("canonical pair aggregate hash mismatch"));
    }
    report(total_wire, total_wire);
    Ok(VerifiedCanonicalPair {
        identity: CanonicalPairMountIdentity {
            hub_origin: hub_origin.to_string(),
            authority_generation,
            scope,
            descriptor_digest_v1: hex(&descriptor_digest),
            active_checkpoint_id: hex(&active_checkpoint_id),
            etag: computed_etag,
            catalog_generation,
        },
        baseline,
        bytes: PairBytes { pack: std::mem::take(&mut pack.0), spr: std::mem::take(&mut spr.0) },
    })
}

fn checked_hash(bytes: &[u8], context: &OperationContext, operation_now_ms: u64, started: std::time::Instant) -> Result<[u8; 32], CanonicalPairMountError> {
    let mut hash = framework_hash::Sha256::new();
    for chunk in bytes.chunks(16 * 1024) {
        checkpoint(context, receipt_now(operation_now_ms, started))?;
        hash.update(chunk);
    }
    Ok(hash.finalize())
}

pub fn maximum_wire_bytes() -> usize {
    HUB_PAIR_MAX_VERIFIED_BYTES + HUB_PAIR_HEADER_MAX_BYTES + 4 + (HUB_PAIR_MAX_VERIFIED_BYTES.div_ceil(HUB_PAIR_RECORD_BYTES) + 1) * 22 + 6
}

pub fn canonical_etag(header: &[u8]) -> String {
    let mut hash = framework_hash::Sha256::new();
    hash.update(HUB_PAIR_ETAG_DOMAIN);
    hash.update(header);
    format!("\"{}\"", hex(&hash.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 15) as usize] as char);
    }
    output
}

#[cfg(any(test, feature = "testkit"))]
pub fn decode_fixture_for_test(body: Vec<u8>, scope: &DocumentScope, descriptor_digest: &str, etag: &str, context: &OperationContext) -> Result<CanonicalPairMountIdentity, CanonicalPairMountError> {
    decode_response(HUB_PAIR_MEDIA_TYPE, etag, WipeVec(body), "https://hub.invalid", 7, scope, descriptor_digest, Some(9), context, 1, std::time::Instant::now(), |_, _| {}).map(|pair| pair.identity)
}

#[cfg(any(test, feature = "testkit"))]
pub fn test_wiped_bytes() -> u64 {
    TEST_WIPED_BYTES.load(Ordering::SeqCst)
}
