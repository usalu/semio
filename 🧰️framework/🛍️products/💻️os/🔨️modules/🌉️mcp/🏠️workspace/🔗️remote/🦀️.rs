//! 🔗️ Authenticated hub descriptor binding for a headless MCP workspace.

use crate::{GatewayError, GatewayErrorCode};
use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::{
    client::{DirectoryClient, DirectoryClientError, DirectoryTransport, HubSocketGrantSource, LocalHubCredential},
    descriptor_digest_v1, hex_lower, DirectoryEventBody, DirectorySpaceDetailV1, DirectoryStreamMessage, DocumentScope, DocumentView, MemberSpaceViewV1, MemberView,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

#[path = "🧩️pair/🦀️.rs"]
mod pair;
pub use pair::{CanonicalPairBody, CanonicalPairFetchRequest, CanonicalPairHttpResponse, CanonicalPairMount, CanonicalPairMountError, CanonicalPairMountIdentity, CanonicalPairMountProgress, CanonicalPairMountStage, CanonicalPairTransport};
#[cfg(not(target_arch = "wasm32"))]
pub use pair::{NativeCanonicalPairBody, NativeCanonicalPairTransport};

pub const HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS: usize = 4_096;
pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;
pub const HUB_BINDING_ID_MAX_BYTES: usize = 512;
pub const HUB_BINDING_OPERATION_TIMEOUT_MS: u64 = 10_000;
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
    Unavailable,
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
            Self::Unavailable => formatter.write_str("hub directory is temporarily unavailable"),
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
    pair_actor: Mutex<pair::CanonicalPairActor>,
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
        })
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
                return Err(unavailable_gateway_error(HubRemoteBindingState::Revoked));
            }
            state => return Err(unavailable_gateway_error(state)),
        };
        Ok(ready)
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
        if session.expires_at_ms <= wall_now_ms {
            return self.fail(generation, HubBindingError::SessionExpired);
        }
        validate_identity("authenticated user id", &session.user_id).and_then(|_| validate_identity("authenticated email", &session.email)).map_err(|error| {
            let _ = self.fail::<Arc<AuthorizedDescriptorSnapshot>>(generation, error.clone());
            error
        })?;
        self.set_progress(HubBindingPhase::LoadingSpace, 0, 0);
        let detail = match client.space(ctx, &self.space_id).await {
            Ok(detail) => detail,
            Err(error) => return self.fail(generation, map_client_error(error)),
        };
        let (space, members, documents) = match detail {
            DirectorySpaceDetailV1::Member { space, members, documents } | DirectorySpaceDetailV1::Author { space, members, documents, .. } => (space, members, documents),
            DirectorySpaceDetailV1::Public { .. } => return self.fail(generation, HubBindingError::MembershipRequired),
        };
        let observed_event_seq = self.observed_event_seq.load(Ordering::SeqCst);
        let snapshot = match self.validate_snapshot(session.user_id, session.expires_at_ms, space, members, documents, observed_event_seq, ctx) {
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
            DirectoryStreamMessage::Heartbeat { head_seq } => {
                self.observed_event_seq.fetch_max(*head_seq, Ordering::SeqCst);
                HubStreamObservation::Stable
            }
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } => HubStreamObservation::Stable,
        }
    }

    pub fn invalidate_stream(&self) {
        self.invalidate("hub directory stream continuity was lost");
    }

    fn begin_refresh(&self, phase: HubBindingPhase, total: usize) -> u64 {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        let generation = self.generation.fetch_add(1, Ordering::SeqCst).saturating_add(1);
        self.authority_generation.store(0, Ordering::SeqCst);
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = None;
        self.set_progress(phase, 0, total);
        generation
    }

    fn set_progress(&self, phase: HubBindingPhase, completed: usize, total: usize) {
        *self.progress.write().unwrap_or_else(PoisonError::into_inner) = HubBindingProgress { phase, completed, total };
    }

    fn invalidate(&self, diagnostic: &str) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        actor.invalidate(pair::CanonicalPairActorState::Refreshing);
        drop(actor);
        let mut state = self.state.write().unwrap_or_else(PoisonError::into_inner);
        *state = HubRemoteBindingState::Refreshing;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(diagnostic));
        self.set_progress(HubBindingPhase::Idle, 0, 0);
    }

    fn revoke(&self, error: HubBindingError) {
        let mut actor = self.pair_actor.lock().unwrap_or_else(PoisonError::into_inner);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.authority_generation.store(0, Ordering::SeqCst);
        actor.invalidate(pair::CanonicalPairActorState::Revoked);
        drop(actor);
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = HubRemoteBindingState::Revoked;
        *self.authenticated_user_id.write().unwrap_or_else(PoisonError::into_inner) = None;
        *self.diagnostic.write().unwrap_or_else(PoisonError::into_inner) = Some(bounded_diagnostic(&error.to_string()));
        self.set_progress(HubBindingPhase::Revoked, 0, 0);
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
        }
        Err(error)
    }

    fn validate_snapshot(
        &self,
        authenticated_user_id: String,
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
        if space.role != membership.role {
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
    }
}

fn next_authority_generation() -> Result<u64, HubBindingError> {
    NEXT_HUB_AUTHORITY_GENERATION.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| current.checked_add(1)).map_err(|_| HubBindingError::CapacityExceeded)
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
        DirectoryClientError::Decode(_) | DirectoryClientError::Http { .. } | DirectoryClientError::Transport(_) => HubBindingError::Unavailable,
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

fn unavailable_gateway_error(state: HubRemoteBindingState) -> GatewayError {
    let label = match state {
        HubRemoteBindingState::Unbound => "unbound",
        HubRemoteBindingState::Refreshing => "refreshing",
        HubRemoteBindingState::Ready(_) => "expired",
        HubRemoteBindingState::Revoked => "revoked",
    };
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated hub descriptor index is {label}; retry after authority refresh"))
        .with_details(serde_json::json!({ "bindingState": label }))
        .retryable()
}

fn percent_encode(value: &str) -> String {
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

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHubBindingDriver {
    cancel: semio_framework_async::CancelToken,
    thread: Option<std::thread::JoinHandle<()>>,
    runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    pair_transport: Arc<NativeCanonicalPairTransport<semio_framework_os_services::TokioHostRuntime>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
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
            16 * 1024 * 1024,
            2,
            PackageId("semio-framework-os-mcp".to_string()),
            ActorId(0x4d43_5001),
        );
        let pair_transport = Arc::new(NativeCanonicalPairTransport::new(transport.clone(), credential.clone()));
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
        runtime
            .block_on(binding.refresh(client.as_ref(), &ctx, wall_now_ms(), operation_now))
            .map_err(binding_error_to_gateway)?;

        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { client: dial_client, since } = stream.turn(&ctx, operation_now) else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream did not enter its initial dial").retryable());
        };
        let authority_generation = binding.authority_generation.load(Ordering::SeqCst);
        let connection = dial_client.open_stream_ws(&ctx, since, 1_000).map_err(|_| {
            binding.invalidate_stream();
            GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream is unavailable; authenticated snapshot was not activated").retryable()
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
        let thread = std::thread::Builder::new()
            .name("semio-mcp-hub-binding".to_string())
            .spawn(move || {
                let mut needs_refresh = false;
                while !thread_cancel.is_cancelled_now() {
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
                    match stream.turn(&ctx, operation_now) {
                        DirectoryStreamTurn::Dial { client: dial_client, since } => {
                            if thread_binding.authority_generation.load(Ordering::SeqCst) == 0 {
                                match thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now)) {
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
                                    Ok(_) => {}
                                }
                            }
                            let authority_generation = thread_binding.authority_generation.load(Ordering::SeqCst);
                            match dial_client.open_stream_ws(&ctx, since, 1_000) {
                                Ok(connection) => {
                                    match complete_authorized_directory_dial(&mut stream, &thread_binding, &ctx, authority_generation, operation_now, connection) {
                                        DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) => break,
                                    }
                                }
                                Err(error) => {
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
                            match thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now)) {
                                Ok(_) => needs_refresh = false,
                                Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                Err(_) => std::thread::sleep(std::time::Duration::from_millis(100)),
                            }
                        }
                        DirectoryStreamTurn::Idle => std::thread::sleep(std::time::Duration::from_millis(10)),
                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) | DirectoryStreamTurn::DialScoped { .. } => break,
                    }
                }
                stream.close();
            })
            .map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "could not start the hub descriptor binding actor"))?;
        Ok((binding, Self { cancel, thread: Some(thread), runtime, pair_transport }, grant_source))
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

fn binding_error_to_gateway(error: HubBindingError) -> GatewayError {
    let code = match error {
        HubBindingError::Cancelled => GatewayErrorCode::Cancelled,
        HubBindingError::CapacityExceeded => GatewayErrorCode::BudgetExceeded,
        HubBindingError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired => GatewayErrorCode::PermissionDenied,
        HubBindingError::DeadlineExceeded | HubBindingError::StaleRefresh | HubBindingError::Unavailable => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = GatewayError::new(code, error.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_async::{CancelToken, TraceId};
    use semio_framework_os_kernel::os_directory::client::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use std::collections::VecDeque;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Mutex;

    #[derive(Clone)]
    struct RecordingTransport {
        responses: Arc<Mutex<VecDeque<HttpResponse>>>,
        requests: Arc<Mutex<Vec<(HttpMethod, String, bool)>>>,
    }

    struct NoopWs;

    #[derive(Clone)]
    struct ObservedTransport {
        closes: Arc<AtomicUsize>,
    }

    struct ObservedWs {
        closes: Arc<AtomicUsize>,
    }

    impl DirectoryWsConnection for NoopWs {
        fn send_text(&mut self, _text: String) -> Result<(), TransportError> { Ok(()) }
        fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> { Ok(()) }
        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> { Ok(DirectoryWsPoll::Pending) }
        fn close(&mut self) {}
    }

    impl DirectoryWsConnection for ObservedWs {
        fn send_text(&mut self, _text: String) -> Result<(), TransportError> { Ok(()) }
        fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> { Ok(()) }
        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> { Ok(DirectoryWsPoll::Pending) }
        fn close(&mut self) { self.closes.fetch_add(1, Ordering::SeqCst); }
    }

    impl DirectoryTransport for RecordingTransport {
        type Ws = NoopWs;

        async fn http(&self, _ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            self.requests.lock().unwrap().push((method, url.to_string(), bearer.is_some()));
            self.responses.lock().unwrap().pop_front().ok_or_else(|| TransportError::Io("fixture response exhausted".to_string()))
        }

        fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
            Err(TransportError::Io("fixture socket grants are not exercised".into()))
        }

        fn open_ws(&self, _ctx: &OperationContext, _url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> { Ok(NoopWs) }
    }

    impl DirectoryTransport for ObservedTransport {
        type Ws = ObservedWs;

        async fn http(&self, _ctx: &OperationContext, _method: HttpMethod, _url: &str, _bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            Err(TransportError::Io("observed transport has no HTTP fixture".to_string()))
        }

        fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
            Err(TransportError::Io("observed transport has no socket grant fixture".to_string()))
        }

        fn open_ws(&self, _ctx: &OperationContext, _url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            Ok(ObservedWs { closes: self.closes.clone() })
        }
    }

    fn context(deadline_ms: Option<u64>) -> OperationContext {
        OperationContext { actor: 7, generation: 0, trace: TraceId(9), lane: 1, deadline_ms, cancel: CancelToken::root_now(), capability: None }
    }

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧫️fixtures/🔣️authenticated-hub-descriptor-index.json")).unwrap()
    }

    fn client_for(case: &serde_json::Value) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
        let responses = case["responses"].as_array().unwrap().iter().map(|response| HttpResponse {
            status: response["status"].as_u64().unwrap() as u16,
            body: serde_json::to_vec(&response["body"]).unwrap(),
        }).collect();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let transport = RecordingTransport { responses: Arc::new(Mutex::new(responses)), requests: requests.clone() };
        let client = DirectoryClient::new(transport, "http://hub.invalid");
        (client, requests)
    }

    #[tokio::test]
    async fn authenticated_hub_workspace_fixture_reaches_ready_without_retaining_bearer() {
        let contract = fixture();
        let case = &contract["cases"]["memberReady"];
        let (client, requests) = client_for(case);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
        assert_eq!(snapshot.authenticated_user_id, "user-a");
        assert_eq!(snapshot.documents.len(), 1);
        assert!(snapshot.documents.contains_key(&DocumentScope::new("space-a", "shared-doc")));
        assert_eq!(binding.progress(), HubBindingProgress { phase: HubBindingPhase::Ready, completed: 1, total: 1 });
        assert_eq!(requests.lock().unwrap().as_slice(), &[
            (HttpMethod::Get, "http://hub.invalid/auth/sessions/me".to_string(), false),
            (HttpMethod::Get, "http://hub.invalid/directory/spaces/space-a".to_string(), false),
        ]);
        let rendered = format!("{:?} {:?} {:?}", binding.state(), binding.progress(), binding.diagnostic());
        assert!(!rendered.contains("session.v1."));
    }

    #[tokio::test]
    async fn authenticated_hub_workspace_rejects_public_nonmember_and_cross_space_atomically() {
        let contract = fixture();
        for (case_name, expected) in [("publicWithoutMembership", HubBindingError::MembershipRequired), ("sameDocumentOtherSpace", HubBindingError::InvalidResponse("document escaped the selected space"))] {
            let (client, _) = client_for(&contract["cases"][case_name]);
            let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
            let error = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err();
            assert_eq!(error, expected);
            assert!(!matches!(binding.state(), HubRemoteBindingState::Ready(_)));
        }
    }

    #[tokio::test]
    async fn authenticated_hub_workspace_unauthorized_cancelled_and_deadline_never_publish() {
        let contract = fixture();
        let (client, _) = client_for(&contract["cases"]["expiredToken"]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        assert_eq!(binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::Unauthorized);
        assert!(matches!(binding.state(), HubRemoteBindingState::Revoked));

        let (client, requests) = client_for(&contract["cases"]["memberReady"]);
        let cancelled = context(Some(20_000));
        cancelled.cancel.cancel_now();
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        assert_eq!(binding.refresh(&client, &cancelled, 1_000, 10_000).await.unwrap_err(), HubBindingError::Cancelled);
        assert!(requests.lock().unwrap().is_empty());

        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        assert_eq!(binding.refresh(&client, &context(Some(10_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::DeadlineExceeded);
        assert!(requests.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn authenticated_hub_workspace_revocation_and_stream_loss_invalidate_ready_snapshot() {
        let contract = fixture();
        let (client, _) = client_for(&contract["cases"]["memberReady"]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
        let message = semio_framework_os_kernel::os_pack::json::from_json_str::<DirectoryStreamMessage>(&contract["cases"]["memberRevoked"]["streamMessage"].to_string()).unwrap();
        assert_eq!(binding.observe_stream_message(&message), HubStreamObservation::Revoked);
        assert!(matches!(binding.state(), HubRemoteBindingState::Revoked));
        assert!(binding.ready_snapshot(1_000).unwrap_err().retryable);

        let (client, _) = client_for(&contract["cases"]["memberReady"]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
        let refreshing_generation = binding.begin_refresh(HubBindingPhase::Authenticating, 0);
        binding.invalidate_stream();
        assert_ne!(binding.generation.load(Ordering::SeqCst), refreshing_generation);
        assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing));
        assert!(binding.ready_snapshot(1_000).unwrap_err().retryable);
    }

    #[tokio::test]
    async fn native_driver_closes_post_open_cancelled_and_stale_authority_dials_once_without_refresh() {
        let contract = fixture();
        let (client, requests) = client_for(&contract["cases"]["memberReady"]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
        let initial_requests = requests.lock().unwrap().len();
        let initial_generation = binding.generation.load(Ordering::SeqCst);
        let authority_generation = binding.authority_generation.load(Ordering::SeqCst);

        let cancelled_closes = Arc::new(AtomicUsize::new(0));
        let cancelled_client = Arc::new(DirectoryClient::new(ObservedTransport { closes: cancelled_closes.clone() }, "http://hub.invalid"));
        let mut cancelled_stream = cancelled_client.stream(0);
        let cancelled = context(Some(20_000));
        assert!(matches!(cancelled_stream.turn(&cancelled, 10_001), semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Dial { .. }));
        cancelled.cancel.cancel_now();
        let cancelled_turn = complete_authorized_directory_dial(
            &mut cancelled_stream,
            &binding,
            &cancelled,
            authority_generation,
            10_002,
            ObservedWs { closes: cancelled_closes.clone() },
        );
        assert!(matches!(cancelled_turn, semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Closed));
        assert_eq!(cancelled_closes.load(Ordering::SeqCst), 1);
        assert_eq!(binding.generation.load(Ordering::SeqCst), initial_generation);
        assert_eq!(binding.authority_generation.load(Ordering::SeqCst), authority_generation);
        assert!(matches!(binding.state(), HubRemoteBindingState::Ready(_)));
        assert_eq!(requests.lock().unwrap().len(), initial_requests);

        let stale_closes = Arc::new(AtomicUsize::new(0));
        let stale_client = Arc::new(DirectoryClient::new(ObservedTransport { closes: stale_closes.clone() }, "http://hub.invalid"));
        let mut stale_stream = stale_client.stream(0);
        let active = context(Some(20_000));
        assert!(matches!(stale_stream.turn(&active, 10_003), semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Dial { .. }));
        binding.invalidate_stream();
        let invalidated_generation = binding.generation.load(Ordering::SeqCst);
        let stale_turn = complete_authorized_directory_dial(
            &mut stale_stream,
            &binding,
            &active,
            authority_generation,
            10_004,
            ObservedWs { closes: stale_closes.clone() },
        );
        assert!(matches!(stale_turn, semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Closed));
        assert_eq!(stale_closes.load(Ordering::SeqCst), 1);
        assert_eq!(binding.generation.load(Ordering::SeqCst), invalidated_generation);
        assert_eq!(binding.authority_generation.load(Ordering::SeqCst), 0);
        assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing));
        assert_eq!(requests.lock().unwrap().len(), initial_requests);
    }

    #[test]
    fn authenticated_hub_workspace_fixed_caps_and_scoped_uri_laws() {
        assert!(validate_hub_origin("https://hub.invalid", "space-a").is_ok());
        assert!(validate_hub_origin("file:///tmp/hub", "space-a").is_err());
        assert!(validate_hub_origin(&format!("http://{}", "h".repeat(2_048)), "space-a").is_err());
        let scope = DocumentScope::new("space/a", "dokument/ä");
        let uri = descriptor_resource_uri(&scope);
        assert_eq!(parse_descriptor_resource_uri(&uri), Some(scope));
        assert_eq!(parse_descriptor_resource_uri("semio://workspace/scopes/space-a/shared-doc/schema"), None);
        assert!(bounded_diagnostic(&"é".repeat(HUB_BINDING_DIAGNOSTIC_MAX_BYTES)).len() <= HUB_BINDING_DIAGNOSTIC_MAX_BYTES);
        assert_eq!(validate_document_count(HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS, HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS as u32), Ok(()));
        assert_eq!(validate_document_count(HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS + 1, (HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS + 1) as u32), Err(HubBindingError::CapacityExceeded));
    }
}
