//! OS hub backend — thin axum shell over `db::Database` (document authority + content-addressed
//! blobs) and `HubDirectory` (identity/tenancy), speaking `protocol_wire`'s binary frames over
//! WebSocket.
//!
//! The WebSocket endpoint speaks `protocol_wire`'s binary lane-tagged `ClientFrame`/`ServerFrame`
//! frames directly (see `protocol/wire/rs/lib.rs`) — the server-side counterpart to
//! `framework/sync`'s client actors (CW5). Command-lane persistence/ordering flows through
//! `db::Database::hello`/`ArtifactHandle::submit`/`db::sync::handle_frontier_advertise`;
//! preview-lane and presence frames are ephemeral, best-effort fan-out this crate owns directly via
//! a per-document `tokio::sync::broadcast` registry (never durable, matching the preview lane's
//! contract). "Space" is a namespacing convention this crate applies on top of `db`'s flat document
//! catalog (`{space_id}:{document_id}`), not hub-internal state.

#[cfg(test)]
extern crate directory as semio_framework_os_kernel;

use axum::body::Bytes;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::{DefaultBodyLimit, OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use db::db_storage::PayloadStorage as _;
use directory::os_directory::{
    self, directory_command_sha256, validate_directory_event_page_event, AdminConnectionSnapshotV1, AdminIntentOutcomeV1, AdminIntentReceiptV1, AdminIntentResultV1, AdminIntentStateV1, AdminIntentV1, AdminOperationAuditPhaseV1, AdminOperationAuditV1,
    AdminOperationProgressV1, AdminOperationStatusV1, AdminPageV1, AdminRecordedConnectionV1, ConnectionView, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryConnectionPhase, DirectoryEvent, DirectoryEventPageErrorV1,
    DirectoryEventPageV1, DirectoryPresenceActor, DirectoryReadModel, DirectorySpaceDetailV1, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentOpenArtifactV1,
    DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanErrorV1, DocumentOpenPlanV1, DocumentOpenRevalidationV1,
    DocumentOpenSurfaceV1, DocumentPlanSocketGrantIntentV1, DocumentView, InviteView, MemberSpaceViewV1, MemberView, PublicDocumentCatalogEntryV1, PublicSpaceViewV1, SpaceView, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS,
    DOCUMENT_OPEN_MAX_SAFE_INTEGER, DOCUMENT_OPEN_PLAN_MAX_TTL_MS,
};
use directory::os_spr::channel::{PRESENCE_ROSTER_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_ITEMS};
use directory::{DslValue, FromValue, ToValue};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use protocol::{decode_client_frame, encode_server_frame, AckStage, ActorId, ApplyOutcome, ArtifactId as ProtocolArtifactId, ClientFrame, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
use semio_framework_async::ShardedMap;
use semio_framework_hash::Sha256;
#[cfg(feature = "neo4j")]
use semio_hub::artifact_authority::chunk_cas::Neo4jArtifactChunkCasStorage;
#[cfg(feature = "postgres")]
use semio_hub::artifact_authority::chunk_cas::PostgresArtifactChunkCasStorage;
#[cfg(feature = "sqlite")]
use semio_hub::artifact_authority::chunk_cas::SqliteArtifactChunkCasStorage;
#[cfg(test)]
use semio_hub::artifact_authority::chunk_cas::{artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1, MemoryArtifactChunkCasStorage};
use semio_hub::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage, ArtifactChunkCasStores, FsArtifactChunkCasStorage};
use semio_hub::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
use semio_hub::artifact_authority::trusted_catalog::{TrustedCatalogLoader, VerifiedDocumentOpenSelectionV1, VerifiedTrustedCatalog};
#[cfg(test)]
use semio_hub::artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, ImmutableArtifactBlobStore};
use semio_hub::artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, CheckpointPublicationOrchestrator, OperationContext, ValidatingCanonicalArtifactAuthority};
use semio_hub::directory::error::DirectoryError;
#[cfg(test)]
use semio_hub::directory::model::AuthSessionIssue;
use semio_hub::directory::model::{AdminOperationAuditRecord, AuthSessionKind, DocumentScope, InviteRecord, NewAdminOperationAuditRecord, NewDirectoryCommandReceipt, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceRole, SyncSessionRecord};
#[cfg(feature = "sqlite")]
use semio_hub::directory::sqlite::SqliteDirectory;
use semio_hub::directory::{identity_subject_digest, HubCapability, IdentityAssertionVerifier, IdentityVerificationControl, InviteCapability, LocalBootstrapTransport, SessionCapability, SocketGrantCapability, AUTH_TEXT_MAX_BYTES};
use semio_hub::directory::{
    directory_command_result_kind, ArtifactCasSweepContinuation, ArtifactCasSweepRequest, ArtifactCasSweepResult, CommandResult, DirectoryCommandExecutionV1, DirectoryService, HubDirectories, HubDirectory, HubVerifiedCheckpointPublisher,
    ProjectionRebuildControl, ProjectionRebuildProgress,
    ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_INTENT_REQUEST_MAX_BYTES, ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, CAPABILITY_MAX_TTL_SECS, DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS,
};
use semio_hub::inference::{verified_gis_map_binding, VerifiedGisMapArtifactBindingV1};
#[cfg(test)]
use semio_hub::lag_rebootstrap::decode_canonical_checkpoint_pair;
use semio_hub::lag_rebootstrap::{
    append_canonical_pair_data, append_canonical_pair_header, append_canonical_pair_terminal, canonical_pair_etag, CanonicalPairTerminal, RebootstrapContext, RebootstrapError, RebootstrapProgress, RebootstrapProgressStage,
    RebootstrapTransferControl, VerifiedRebootstrapSource, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, REBOOTSTRAP_DEADLINE_MS,
};
use semio_hub::local_bootstrap::{serve_local_bootstrap, InheritedLocalBootstrapTransport, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

//#region ⚠️ Errors
/// @emoji 🧯️ Top-level startup error — the only fallible paths outside a document/WS session are
/// opening `db::Database`'s storage backend, connecting the directory backend, and binding the
/// HTTP listener.
#[derive(Debug)]
enum HubError {
    ArtifactAuthority(AuthorityError),
    Directory(DirectoryError),
    Db(db::DbError),
    Io(std::io::Error),
    UnknownStorageBackend(String),
    UnknownDirectoryBackend(String),
    UnsafeAuthConfiguration(String),
}

impl std::fmt::Display for HubError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArtifactAuthority(error) => std::fmt::Display::fmt(error, formatter),
            Self::Directory(error) => std::fmt::Display::fmt(error, formatter),
            Self::Db(error) => std::fmt::Display::fmt(error, formatter),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::UnknownStorageBackend(backend) => write!(formatter, "unknown OS_HUB_STORAGE_BACKEND: {backend}"),
            Self::UnknownDirectoryBackend(backend) => write!(formatter, "unknown OS_HUB_DIRECTORY_BACKEND: {backend}"),
            Self::UnsafeAuthConfiguration(detail) => write!(formatter, "unsafe hub authentication configuration: {detail}"),
        }
    }
}

impl std::error::Error for HubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ArtifactAuthority(error) => Some(error),
            Self::Directory(error) => Some(error),
            Self::Db(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::UnknownStorageBackend(_) | Self::UnknownDirectoryBackend(_) | Self::UnsafeAuthConfiguration(_) => None,
        }
    }
}

impl From<AuthorityError> for HubError {
    fn from(error: AuthorityError) -> Self {
        Self::ArtifactAuthority(error)
    }
}

impl From<DirectoryError> for HubError {
    fn from(error: DirectoryError) -> Self {
        Self::Directory(error)
    }
}

impl From<db::DbError> for HubError {
    fn from(error: db::DbError) -> Self {
        Self::Db(error)
    }
}

impl From<std::io::Error> for HubError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
//#endregion ⚠️ Errors

/// @emoji 📦️ Axum JSON boundary for first-party `ToValue`/`FromValue` directory contracts.
struct DirectoryJson<T>(T);

impl<'de, T: FromValue> Deserialize<'de> for DirectoryJson<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        T::from_value(DslValue::from(value)).map(Self).map_err(serde::de::Error::custom)
    }
}

impl<T: ToValue> IntoResponse for DirectoryJson<T> {
    fn into_response(self) -> axum::response::Response {
        ([(axum::http::header::CONTENT_TYPE, "application/json")], directory::os_pack::json::to_json_string(&self.0)).into_response()
    }
}

fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

struct StartupCatalogControl;

impl AuthorityOperationControl for StartupCatalogControl {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, progress: AuthorityProgress) {
        if progress.completed_units == 0 || progress.completed_units == progress.total_units {
            eprintln!("[INFO] trusted catalog {:?}: {}/{}", progress.stage, progress.completed_units, progress.total_units);
        }
    }
}

struct HubBootstrapControl {
    cancelled: std::sync::atomic::AtomicBool,
}

impl HubBootstrapControl {
    fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false) }
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }
}

impl IdentityVerificationControl for HubBootstrapControl {
    fn now_ms(&self) -> i64 {
        now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }

    fn report(&self, _progress: semio_hub::directory::IdentityVerificationProgress) {}
}

type HubArtifactAuthority = ValidatingCanonicalArtifactAuthority<Arc<VerifiedTrustedCatalog>>;
type HubArtifactPublication = CheckpointPublicationOrchestrator<ArtifactChunkBlobStore<Arc<ArtifactChunkCasStores>>, HubVerifiedCheckpointPublisher<ArtifactChunkCasStores>>;

struct ArtifactCasMaintenanceControl {
    cancelled: Arc<std::sync::atomic::AtomicBool>,
}

impl AuthorityOperationControl for ArtifactCasMaintenanceControl {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }

    fn report(&self, progress: AuthorityProgress) {
        if progress.completed_units == progress.total_units {
            eprintln!("[INFO] artifact CAS maintenance {:?}: {}/{}", progress.stage, progress.completed_units, progress.total_units);
        }
    }
}

struct ArtifactCasMaintenanceSupervisor {
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    healthy: Arc<std::sync::atomic::AtomicBool>,
    wake: Arc<tokio::sync::Notify>,
    task: std::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}

#[derive(Default)]
struct ArtifactCasMaintenanceCheckpoint {
    continuation: Option<ArtifactCasSweepContinuation>,
}

impl ArtifactCasMaintenanceCheckpoint {
    fn request(&self, execute: bool, max_objects: usize) -> ArtifactCasSweepRequest {
        ArtifactCasSweepRequest { execute, max_objects, continuation: self.continuation }
    }

    fn accept(&mut self, result: &ArtifactCasSweepResult) -> bool {
        self.continuation = result.continuation;
        self.continuation.is_none()
    }

    fn fail(&mut self, error: &AuthorityError) {
        if matches!(error, AuthorityError::Store(message) if message.contains("continuation generation changed") || message.contains("continuation is invalid")) {
            self.continuation = None;
        }
    }
}

impl ArtifactCasMaintenanceSupervisor {
    fn start(service: Arc<DirectoryService>, storage: Arc<ArtifactChunkCasStores>, execute: bool) -> Arc<Self> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let healthy = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let wake = Arc::new(tokio::sync::Notify::new());
        let control_cancelled = cancelled.clone();
        let task_healthy = healthy.clone();
        let task_wake = wake.clone();
        let task = tokio::spawn(async move {
            let control = ArtifactCasMaintenanceControl { cancelled: control_cancelled };
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut checkpoint = ArtifactCasMaintenanceCheckpoint::default();
            loop {
                tokio::select! {
                    _ = interval.tick() => {}
                    _ = task_wake.notified() => {
                        if control.is_cancelled() { return; }
                        continue;
                    }
                }
                if control.is_cancelled() {
                    return;
                }
                let run_deadline_ms = control.now_ms().saturating_add(30_000);
                for _ in 0..16 {
                    if control.is_cancelled() {
                        return;
                    }
                    let context = OperationContext::new(run_deadline_ms, AuthorityLimits::maximum(), &control);
                    match service.sweep_artifact_cas(storage.as_ref(), checkpoint.request(execute, semio_hub::directory::ARTIFACT_CAS_SWEEP_OBJECT_MAX), &context).await {
                        Ok(result) => {
                            task_healthy.store(true, std::sync::atomic::Ordering::Release);
                            eprintln!(
                                "[INFO] artifact CAS maintenance complete: examined={} protected={} eligible={} deleted={} missing={} continued={}",
                                result.examined_objects,
                                result.protected_objects,
                                result.eligible_objects,
                                result.deleted_objects,
                                result.missing_objects,
                                result.continuation.is_some()
                            );
                            if checkpoint.accept(&result) {
                                break;
                            }
                        }
                        Err(AuthorityError::Cancelled) if control.is_cancelled() => return,
                        Err(error) => {
                            task_healthy.store(false, std::sync::atomic::Ordering::Release);
                            checkpoint.fail(&error);
                            eprintln!("[WARN] artifact CAS maintenance failed closed: {error}");
                            break;
                        }
                    }
                }
            }
        });
        Arc::new(Self { cancelled, healthy, wake, task: std::sync::Mutex::new(Some(task)) })
    }

    #[cfg(test)]
    fn disabled() -> Arc<Self> {
        Arc::new(Self { cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)), healthy: Arc::new(std::sync::atomic::AtomicBool::new(true)), wake: Arc::new(tokio::sync::Notify::new()), task: std::sync::Mutex::new(None) })
    }

    fn healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::Acquire)
    }

    async fn shutdown(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.wake.notify_waiters();
        let task = self.task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(mut task) = task {
            if tokio::time::timeout(std::time::Duration::from_secs(31), &mut task).await.is_err() {
                task.abort();
                let _ = task.await;
            }
        }
    }
}

impl Drop for ArtifactCasMaintenanceSupervisor {
    fn drop(&mut self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.wake.notify_waiters();
        if let Some(task) = self.task.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            task.abort();
        }
    }
}

fn artifact_cas_sweep_execute_from_env() -> Result<bool, HubError> {
    match std::env::var("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE").as_deref() {
        Err(std::env::VarError::NotPresent) | Ok("") | Ok("false") | Ok("0") => Ok(false),
        Ok("true") | Ok("1") => Ok(true),
        Err(error) => Err(HubError::UnsafeAuthConfiguration(format!("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE is unreadable: {error}"))),
        Ok(_) => Err(HubError::UnsafeAuthConfiguration("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE must be true, false, 1, or 0".into())),
    }
}

struct ConfiguredArtifactAuthority {
    catalog: Arc<VerifiedTrustedCatalog>,
    authority: Arc<HubArtifactAuthority>,
}

trait DocumentOpenCatalogAuthorityV1: Send + Sync {
    fn generation_id(&self) -> &str;
    fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1>;
}

impl DocumentOpenCatalogAuthorityV1 for VerifiedTrustedCatalog {
    fn generation_id(&self) -> &str {
        self.generation_id()
    }

    fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        self.resolve_document_open(descriptor, requested_surface_id, writable)
    }
}

async fn configured_artifact_authority(bundle_path: Option<std::path::PathBuf>, profile: Option<String>, providers: &NativeCodecProviderSetV1) -> Result<Option<ConfiguredArtifactAuthority>, AuthorityError> {
    let (bundle_path, profile) = match (bundle_path, profile) {
        (None, None) => return Ok(None),
        (Some(bundle_path), Some(profile)) => (bundle_path, profile),
        _ => return Err(AuthorityError::Catalog("OS_HUB_TRUSTED_CATALOG_BUNDLE and OS_HUB_TRUSTED_CATALOG_PROFILE must be configured together".to_string())),
    };
    let control = StartupCatalogControl;
    let started = control.now_ms();
    let context = OperationContext::new(started.saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let catalog = Arc::new(TrustedCatalogLoader::load(&bundle_path, &profile, providers, &context).await?);
    let authority = Arc::new(ValidatingCanonicalArtifactAuthority::new(catalog.clone()));
    Ok(Some(ConfiguredArtifactAuthority { catalog, authority }))
}

//#region 🔖️State
/// @emoji 🎫️ Unambiguous v1 key for the flat DB/fanout catalogs: ASCII `v1:`, both UTF-8 byte
/// lengths in decimal, separators, then the exact adjacent UTF-8 scope payloads. Both lengths make
/// colon-containing and non-ASCII identifiers structural without a fallback decoder.
fn document_scope_key_v1(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

fn db_artifact_id(scope: &DocumentScope) -> ProtocolArtifactId {
    ProtocolArtifactId(document_scope_key_v1(scope))
}

fn db_core_document_id(id: &ProtocolArtifactId) -> db::ArtifactId {
    db::ArtifactId(id.0.clone())
}

/// 👤️ One private server-local owner slot; only the matching live socket may refresh, expire, or remove it.
struct PresenceLeaseSlot {
    socket_live_id: String,
    expires_at: tokio::time::Instant,
    surface: String,
    user_id: Option<String>,
    color: u8,
    peer: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
enum PresenceLeaseTransition {
    Published,
    NoChange,
    Rejected,
    Unavailable,
}

struct PresenceSnapshot {
    peers: Vec<Vec<u8>>,
    actors: Vec<DirectoryPresenceActor>,
}

#[cfg(test)]
struct TestPresenceClock {
    origin: tokio::time::Instant,
    now_ms: std::sync::atomic::AtomicU64,
}

#[cfg(test)]
impl TestPresenceClock {
    fn new() -> Self {
        Self { origin: tokio::time::Instant::now(), now_ms: std::sync::atomic::AtomicU64::new(0) }
    }

    fn now(&self) -> tokio::time::Instant {
        self.origin + std::time::Duration::from_millis(self.now_ms.load(std::sync::atomic::Ordering::SeqCst))
    }

    fn advance_to(&self, now_ms: u64) {
        self.now_ms.store(now_ms, std::sync::atomic::Ordering::SeqCst);
    }
}

/// @emoji 🎨️ One actor's held palette index within a space, ref-counted across that actor's
/// concurrently open document sockets in the same space (contract §C7.3: "A's second document socket
/// keeps 0").
struct ColorLease {
    index: u8,
    refs: u32,
}

/// @emoji 🌈️ One space's live session-color leases (contract §C7.3) — never persisted, rebuilt from
/// nothing on hub restart, mirroring `presence`'s own ephemeral law.
#[derive(Default)]
struct SpaceColors {
    by_actor: std::collections::BTreeMap<String, ColorLease>,
}

#[cfg(test)]
struct TestLiveGate {
    document_subscribed: tokio::sync::Semaphore,
    document_release: tokio::sync::Semaphore,
    directory_subscribed: tokio::sync::Semaphore,
    directory_release: tokio::sync::Semaphore,
    socket_before_welcome: tokio::sync::Semaphore,
    socket_welcome_release: tokio::sync::Semaphore,
    socket_after_welcome: tokio::sync::Semaphore,
    socket_bootstrap_release: tokio::sync::Semaphore,
    socket_command_received: tokio::sync::Semaphore,
    socket_command_release: tokio::sync::Semaphore,
    socket_lag_received: tokio::sync::Semaphore,
    socket_lag_release: tokio::sync::Semaphore,
    socket_broadcast_received: tokio::sync::Semaphore,
    socket_broadcast_release: tokio::sync::Semaphore,
    socket_rebootstrap_read: tokio::sync::Semaphore,
    socket_directory_admitted: tokio::sync::Semaphore,
    socket_directory_release: tokio::sync::Semaphore,
    socket_admin_revoke_admitted: tokio::sync::Semaphore,
    socket_admin_revoke_release: tokio::sync::Semaphore,
    socket_scoped_send_mode: std::sync::atomic::AtomicU8,
    socket_scoped_send_admitted: tokio::sync::Semaphore,
    socket_scoped_send_release: tokio::sync::Semaphore,
    socket_membership_remove_enabled: std::sync::atomic::AtomicBool,
    socket_membership_remove_admitted: tokio::sync::Semaphore,
    socket_membership_remove_release: tokio::sync::Semaphore,
    directory_event_page_fence_enabled: std::sync::atomic::AtomicBool,
    directory_event_page_read_admitted: tokio::sync::Semaphore,
    directory_event_page_read_release: tokio::sync::Semaphore,
    directory_event_page_control: Mutex<Option<Arc<DirectoryEventPageHttpControl>>>,
}

#[cfg(test)]
impl Default for TestLiveGate {
    fn default() -> Self {
        Self {
            document_subscribed: tokio::sync::Semaphore::new(0),
            document_release: tokio::sync::Semaphore::new(0),
            directory_subscribed: tokio::sync::Semaphore::new(0),
            directory_release: tokio::sync::Semaphore::new(0),
            socket_before_welcome: tokio::sync::Semaphore::new(0),
            socket_welcome_release: tokio::sync::Semaphore::new(0),
            socket_after_welcome: tokio::sync::Semaphore::new(0),
            socket_bootstrap_release: tokio::sync::Semaphore::new(0),
            socket_command_received: tokio::sync::Semaphore::new(0),
            socket_command_release: tokio::sync::Semaphore::new(0),
            socket_lag_received: tokio::sync::Semaphore::new(0),
            socket_lag_release: tokio::sync::Semaphore::new(0),
            socket_broadcast_received: tokio::sync::Semaphore::new(0),
            socket_broadcast_release: tokio::sync::Semaphore::new(0),
            socket_rebootstrap_read: tokio::sync::Semaphore::new(0),
            socket_directory_admitted: tokio::sync::Semaphore::new(0),
            socket_directory_release: tokio::sync::Semaphore::new(0),
            socket_admin_revoke_admitted: tokio::sync::Semaphore::new(0),
            socket_admin_revoke_release: tokio::sync::Semaphore::new(0),
            socket_scoped_send_mode: std::sync::atomic::AtomicU8::new(0),
            socket_scoped_send_admitted: tokio::sync::Semaphore::new(0),
            socket_scoped_send_release: tokio::sync::Semaphore::new(0),
            socket_membership_remove_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_membership_remove_admitted: tokio::sync::Semaphore::new(0),
            socket_membership_remove_release: tokio::sync::Semaphore::new(0),
            directory_event_page_fence_enabled: std::sync::atomic::AtomicBool::new(false),
            directory_event_page_read_admitted: tokio::sync::Semaphore::new(0),
            directory_event_page_read_release: tokio::sync::Semaphore::new(0),
            directory_event_page_control: Mutex::new(None),
        }
    }
}

#[cfg(test)]
struct TestDocumentOpenPlanIssueGate {
    admitted: tokio::sync::Semaphore,
    release: tokio::sync::Semaphore,
}

#[cfg(test)]
impl Default for TestDocumentOpenPlanIssueGate {
    fn default() -> Self {
        Self { admitted: tokio::sync::Semaphore::new(0), release: tokio::sync::Semaphore::new(0) }
    }
}

#[cfg(test)]
struct TestDocumentOpenCatalog {
    generation_id: String,
    open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
}

#[cfg(test)]
impl DocumentOpenCatalogAuthorityV1 for TestDocumentOpenCatalog {
    fn generation_id(&self) -> &str {
        &self.generation_id
    }

    fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let mut matches = self.open_targets.iter().filter(|target| {
            target.package.plugin_id == descriptor.owner.plugin_id
                && target.package.package_id == descriptor.owner.package_id
                && target.package.version == descriptor.owner.version
                && target.package.component_sha256 == descriptor.owner.package_hash
                && target.artifact.kind == descriptor.artifact_kind
                && target.artifact.schema == descriptor.artifact_schema
                && target.artifact.pack_schema_hash == descriptor.pack_schema_hash
                && target.grant.write == writable
                && requested_surface_id.is_none_or(|surface_id| target.surface.surface_id == surface_id)
        });
        let selected = matches.next()?.clone();
        matches.next().is_none().then_some(selected)
    }
}

const SOCKET_GRANT_TTL_MS: i64 = 30_000;
const SOCKET_GRANT_LEDGER_CAPACITY: usize = 4_096;
const SOCKET_GRANT_BINDING_PENDING_CAPACITY: usize = 64;
const PRESENCE_LEASE_TTL_MS: u64 = 15_000;
const SOCKET_PROTOCOL_V1: &str = "semio.socket.v1";
const DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES: usize = 8 * 1024;
const DOCUMENT_OPEN_PLAN_DEADLINE_MS: u64 = 10_000;
const DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES: usize = 8 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SocketBindingKeyV1 {
    User(String),
    Session(String),
    Membership { user_id: String, space_id: String },
    Share(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SocketAudienceV1 {
    Document(DocumentScope),
    Directory { auth_session_id: String, authorization_generation: u64 },
    DirectoryScoped(DocumentScope),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SocketSubjectV1 {
    Session { session_id: String, user_id: String, authorization_generation: u64, role: Option<SpaceRole>, expires_at_ms: i64 },
    Share { share_id: String, selector: String, scope: DocumentScope, expires_at_ms: i64 },
}

impl SocketSubjectV1 {
    fn binding(&self) -> SocketBindingKeyV1 {
        match self {
            Self::Session { session_id, .. } => SocketBindingKeyV1::Session(session_id.clone()),
            Self::Share { share_id, .. } => SocketBindingKeyV1::Share(share_id.clone()),
        }
    }

    fn admission_bindings(&self) -> Vec<SocketBindingKeyV1> {
        match self {
            Self::Session { session_id, user_id, .. } => vec![SocketBindingKeyV1::User(user_id.clone()), SocketBindingKeyV1::Session(session_id.clone())],
            Self::Share { share_id, .. } => vec![SocketBindingKeyV1::Share(share_id.clone())],
        }
    }

    async fn revalidate(&self, directory: &HubDirectories, audience: &SocketAudienceV1, at_ms: i64) -> SocketBindingValidityV1 {
        match (self, audience) {
            (Self::Session { session_id, user_id, authorization_generation, role, expires_at_ms }, SocketAudienceV1::Document(scope)) => {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, Some(&scope.space_id), at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: current, expires_at_ms: current_expiry }) if current == *role && current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Session { session_id, user_id, authorization_generation, expires_at_ms, .. }, SocketAudienceV1::Directory { auth_session_id, authorization_generation: audience_generation })
                if session_id == auth_session_id && authorization_generation == audience_generation =>
            {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, None, at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms: current_expiry }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Session { session_id, user_id, authorization_generation, expires_at_ms, .. }, SocketAudienceV1::DirectoryScoped(scope)) => {
                match directory.socket_session_binding(session_id, user_id, *authorization_generation, Some(&scope.space_id), at_ms).await {
                    Ok(SocketSessionBindingStatus::Active { role: Some(_), expires_at_ms: current_expiry }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketSessionBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
            (Self::Share { share_id, selector, scope, expires_at_ms }, SocketAudienceV1::Document(audience_scope)) if scope == audience_scope => match directory.socket_share_binding(share_id, selector, scope, at_ms).await {
                Ok(SocketShareBindingStatus::Active { expires_at_ms: current_expiry }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                Ok(SocketShareBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                _ => SocketBindingValidityV1::Unauthorized,
            },
            _ => SocketBindingValidityV1::Unauthorized,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SocketBindingValidityV1 {
    Active,
    Unauthorized,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SocketGrantStateV1 {
    Pending,
    Consumed,
}

#[derive(Clone)]
struct SocketGrantRecordV1 {
    selector: String,
    secret_digest: [u8; 32],
    audience: SocketAudienceV1,
    actor_id: String,
    subject: SocketSubjectV1,
    document_plan: Option<Arc<DocumentOpenPlanAuthorityV1>>,
    issued_at_ms: i64,
    expires_at_ms: i64,
    state: SocketGrantStateV1,
}

fn socket_record_bindings(subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> Vec<SocketBindingKeyV1> {
    let mut bindings = subject.admission_bindings();
    if let SocketSubjectV1::Session { user_id, .. } = subject {
        let scope = match audience {
            SocketAudienceV1::Document(scope) | SocketAudienceV1::DirectoryScoped(scope) => Some(scope),
            SocketAudienceV1::Directory { .. } => None,
        };
        if let Some(scope) = scope {
            bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: scope.space_id.clone() });
        }
    }
    bindings.sort();
    bindings.dedup();
    bindings
}

impl SocketGrantRecordV1 {
    fn bindings(&self) -> Vec<SocketBindingKeyV1> {
        socket_record_bindings(&self.subject, &self.audience)
    }
}

struct SocketGrantAdmissionV1 {
    record: SocketGrantRecordV1,
}

#[derive(Default)]
struct SocketGrantLedgerInnerV1 {
    records: BTreeMap<String, SocketGrantRecordV1>,
    pending_by_binding: BTreeMap<SocketBindingKeyV1, BTreeSet<String>>,
    live_by_binding: BTreeMap<SocketBindingKeyV1, BTreeMap<String, (String, Arc<tokio::sync::Notify>)>>,
}

#[derive(Default)]
struct SocketGrantLedgerV1 {
    inner: Mutex<SocketGrantLedgerInnerV1>,
}

#[derive(Default)]
struct SocketBindingGatesV1 {
    inner: Mutex<BTreeMap<SocketBindingKeyV1, std::sync::Weak<tokio::sync::Mutex<()>>>>,
}

impl SocketBindingGatesV1 {
    fn gate(&self, binding: SocketBindingKeyV1) -> Arc<tokio::sync::Mutex<()>> {
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.retain(|_, gate| gate.strong_count() > 0);
        if let Some(gate) = inner.get(&binding).and_then(std::sync::Weak::upgrade) {
            return gate;
        }
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        inner.insert(binding, Arc::downgrade(&gate));
        gate
    }

    async fn acquire_record(&self, subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> Vec<tokio::sync::OwnedMutexGuard<()>> {
        let bindings = socket_record_bindings(subject, audience);
        let mut admissions = Vec::with_capacity(bindings.len());
        for binding in bindings {
            admissions.push(self.gate(binding).lock_owned().await);
        }
        admissions
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SocketGrantLedgerErrorV1 {
    Rejected,
    Capacity,
}

impl SocketGrantLedgerV1 {
    fn remove_pending_indexes(inner: &mut SocketGrantLedgerInnerV1, record: &SocketGrantRecordV1) {
        for binding in record.bindings() {
            let empty = if let Some(selectors) = inner.pending_by_binding.get_mut(&binding) {
                selectors.remove(&record.selector);
                selectors.is_empty()
            } else {
                false
            };
            if empty {
                inner.pending_by_binding.remove(&binding);
            }
        }
    }

    fn remove_live_indexes(inner: &mut SocketGrantLedgerInnerV1, record: &SocketGrantRecordV1, live_id: &str) {
        for binding in record.bindings() {
            let empty = if let Some(live) = inner.live_by_binding.get_mut(&binding) {
                live.remove(live_id);
                live.is_empty()
            } else {
                false
            };
            if empty {
                inner.live_by_binding.remove(&binding);
            }
        }
    }

    fn sweep_expired(inner: &mut SocketGrantLedgerInnerV1, at_ms: i64) {
        let live_selectors = inner.live_by_binding.values().flat_map(BTreeMap::values).map(|(selector, _)| selector.clone()).collect::<BTreeSet<_>>();
        let expired: Vec<String> = inner.records.iter().filter_map(|(selector, record)| (record.expires_at_ms <= at_ms && (record.state == SocketGrantStateV1::Pending || !live_selectors.contains(selector))).then(|| selector.clone())).collect();
        for selector in expired {
            if let Some(record) = inner.records.remove(&selector) {
                Self::remove_pending_indexes(inner, &record);
            }
        }
    }

    fn issue(&self, capability: &SocketGrantCapability, audience: SocketAudienceV1, actor_id: String, subject: SocketSubjectV1, issued_at_ms: i64, expires_at_ms: i64) -> Result<(), SocketGrantLedgerErrorV1> {
        self.issue_with_document_plan(capability, audience, actor_id, subject, issued_at_ms, expires_at_ms, None)
    }

    fn issue_with_document_plan(
        &self,
        capability: &SocketGrantCapability,
        audience: SocketAudienceV1,
        actor_id: String,
        subject: SocketSubjectV1,
        issued_at_ms: i64,
        expires_at_ms: i64,
        document_plan: Option<Arc<DocumentOpenPlanAuthorityV1>>,
    ) -> Result<(), SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, issued_at_ms);
        let bindings = socket_record_bindings(&subject, &audience);
        if inner.records.len() >= SOCKET_GRANT_LEDGER_CAPACITY || bindings.iter().any(|binding| inner.pending_by_binding.get(binding).map_or(0, BTreeSet::len) >= SOCKET_GRANT_BINDING_PENDING_CAPACITY) {
            return Err(SocketGrantLedgerErrorV1::Capacity);
        }
        let selector = capability.selector().to_string();
        if inner.records.contains_key(&selector) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        for binding in bindings {
            inner.pending_by_binding.entry(binding).or_default().insert(selector.clone());
        }
        inner.records.insert(selector.clone(), SocketGrantRecordV1 { selector, secret_digest: capability.secret_digest(), audience, actor_id, subject, document_plan, issued_at_ms, expires_at_ms, state: SocketGrantStateV1::Pending });
        Ok(())
    }

    fn pending(&self, capability: &SocketGrantCapability, audience: &SocketAudienceV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let record = inner.records.get(capability.selector()).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending || record.audience != *audience || !semio_hub::directory::constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        Ok(record.clone())
    }

    fn pending_directory(&self, capability: &SocketGrantCapability, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let record = inner.records.get(capability.selector()).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending || !matches!(record.audience, SocketAudienceV1::Directory { .. }) || !semio_hub::directory::constant_time_digest_eq(&record.secret_digest, &capability.secret_digest()) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        Ok(record.clone())
    }

    fn consume(&self, candidate: &SocketGrantRecordV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let bindings = candidate.bindings();
        if !bindings.iter().all(|binding| inner.pending_by_binding.get(binding).is_some_and(|selectors| selectors.contains(&candidate.selector))) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        let record = inner.records.get_mut(&candidate.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending
            || record.audience != candidate.audience
            || record.actor_id != candidate.actor_id
            || record.subject != candidate.subject
            || record.document_plan != candidate.document_plan
            || record.secret_digest != candidate.secret_digest
            || record.issued_at_ms != candidate.issued_at_ms
            || record.expires_at_ms <= at_ms
        {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        record.state = SocketGrantStateV1::Consumed;
        let consumed = record.clone();
        Self::remove_pending_indexes(&mut inner, &consumed);
        Ok(consumed)
    }

    fn register_live(&self, record: &SocketGrantRecordV1) -> Result<(String, Arc<tokio::sync::Notify>), SocketGrantLedgerErrorV1> {
        let id = directory::os_identity::time_ordered_id();
        let notify = Arc::new(tokio::sync::Notify::new());
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        let stored = inner.records.get(&record.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if stored.state != SocketGrantStateV1::Consumed || stored.secret_digest != record.secret_digest || stored.audience != record.audience || stored.subject != record.subject || stored.document_plan != record.document_plan {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        for binding in record.bindings() {
            inner.live_by_binding.entry(binding).or_default().insert(id.clone(), (record.selector.clone(), notify.clone()));
        }
        Ok((id, notify))
    }

    fn unregister_live(&self, record: &SocketGrantRecordV1, live_id: &str) {
        let Ok(mut inner) = self.inner.lock() else { return };
        Self::remove_live_indexes(&mut inner, record, live_id);
        inner.records.remove(&record.selector);
    }

    fn is_live(&self, record: &SocketGrantRecordV1, live_id: &str) -> bool {
        let Ok(inner) = self.inner.lock() else { return false };
        inner.records.get(&record.selector).is_some_and(|stored| {
            stored.state == SocketGrantStateV1::Consumed
                && stored.secret_digest == record.secret_digest
                && stored.audience == record.audience
                && stored.subject == record.subject
                && stored.document_plan == record.document_plan
                && record.bindings().iter().all(|binding| inner.live_by_binding.get(binding).and_then(|live| live.get(live_id)).is_some_and(|(selector, _)| selector == &record.selector))
        })
    }

    fn invalidate_binding(&self, binding: SocketBindingKeyV1) {
        let notifiers = {
            let Ok(mut inner) = self.inner.lock() else { return };
            let pending = inner.pending_by_binding.get(&binding).cloned().unwrap_or_default();
            for selector in pending {
                if let Some(record) = inner.records.remove(&selector) {
                    Self::remove_pending_indexes(&mut inner, &record);
                }
            }
            let live = inner.live_by_binding.get(&binding).cloned().unwrap_or_default();
            let mut notifiers = Vec::with_capacity(live.len());
            for (live_id, (selector, notify)) in live {
                if let Some(record) = inner.records.remove(&selector) {
                    Self::remove_pending_indexes(&mut inner, &record);
                    Self::remove_live_indexes(&mut inner, &record, &live_id);
                }
                notifiers.push(notify);
            }
            notifiers
        };
        for notify in notifiers {
            notify.notify_one();
        }
    }

    fn reject_pending(&self, selector: &str) {
        let Ok(mut inner) = self.inner.lock() else { return };
        let Some(record) = inner.records.get(selector) else { return };
        if record.state != SocketGrantStateV1::Pending {
            return;
        }
        if let Some(record) = inner.records.remove(selector) {
            Self::remove_pending_indexes(&mut inner, &record);
        }
    }
}

const DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY: usize = 1_024;
const DOCUMENT_OPEN_PLAN_BINDING_CAPACITY: usize = 64;
const DOCUMENT_OPEN_PLAN_RECEIPT_DOMAIN: &[u8] = b"semio/hub/document-open-plan-receipt/v1\0";

struct DocumentOpenPlanCapabilityV1(Box<[u8; 32]>);

impl Drop for DocumentOpenPlanCapabilityV1 {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl std::fmt::Debug for DocumentOpenPlanCapabilityV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("DocumentOpenPlanCapabilityV1(<redacted>)")
    }
}

fn document_open_plan_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

fn document_open_plan_base64url_encode(bytes: &[u8; 32]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut encoded = String::with_capacity(43);
    let mut accumulator = 0u32;
    let mut bits = 0u8;
    for byte in bytes {
        accumulator = (accumulator << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            encoded.push(ALPHABET[((accumulator >> bits) & 0x3f) as usize] as char);
        }
        accumulator &= if bits == 0 { 0 } else { (1u32 << bits) - 1 };
    }
    if bits != 0 {
        encoded.push(ALPHABET[((accumulator << (6 - bits)) & 0x3f) as usize] as char);
    }
    encoded
}

struct DocumentOpenPlanDecodedSecretV1([u8; 32]);

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct DocumentOpenPlanDecodeWipeObservationV1 {
    nonzero_before: usize,
    after: [u8; 32],
}

#[cfg(test)]
static DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS: Mutex<Vec<DocumentOpenPlanDecodeWipeObservationV1>> = Mutex::new(Vec::new());

#[cfg(test)]
std::thread_local! {
    static DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

impl Drop for DocumentOpenPlanDecodedSecretV1 {
    fn drop(&mut self) {
        #[cfg(test)]
        let nonzero_before = self.0.iter().filter(|byte| **byte != 0).count();
        self.0.fill(0);
        #[cfg(test)]
        DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| {
            if observing.get() {
                if let Ok(mut observations) = DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock() {
                    observations.push(DocumentOpenPlanDecodeWipeObservationV1 { nonzero_before, after: self.0 });
                }
            }
        });
    }
}

impl DocumentOpenPlanDecodedSecretV1 {
    fn into_box(mut self) -> Box<[u8; 32]> {
        let secret = Box::new(self.0);
        self.0.fill(0);
        secret
    }
}

fn document_open_plan_base64url_decode(encoded: &str) -> Result<Box<[u8; 32]>, DocumentOpenPlanErrorCodeV1> {
    if encoded.len() != 43 {
        return Err(DocumentOpenPlanErrorCodeV1::Denied);
    }
    let mut decoded = DocumentOpenPlanDecodedSecretV1([0u8; 32]);
    let mut offset = 0usize;
    let mut accumulator = 0u64;
    let mut bits = 0u8;
    for byte in encoded.bytes() {
        let Some(value) = document_open_plan_base64_value(byte) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        accumulator = (accumulator << 6) | u64::from(value);
        bits += 6;
        while bits >= 8 {
            bits -= 8;
            if offset >= decoded.0.len() {
                return Err(DocumentOpenPlanErrorCodeV1::Denied);
            }
            decoded.0[offset] = ((accumulator >> bits) & 0xff) as u8;
            offset += 1;
        }
        accumulator &= if bits == 0 { 0 } else { (1u64 << bits) - 1 };
    }
    if offset != decoded.0.len() || bits != 2 || accumulator != 0 {
        return Err(DocumentOpenPlanErrorCodeV1::Denied);
    }
    Ok(decoded.into_box())
}

impl DocumentOpenPlanCapabilityV1 {
    fn mint() -> Result<Self, DocumentOpenPlanErrorCodeV1> {
        let mut secret = Box::new([0u8; 32]);
        directory::os_identity::fill_entropy(secret.as_mut()).map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        Ok(Self(secret))
    }

    fn parse(receipt: &str) -> Result<Self, DocumentOpenPlanErrorCodeV1> {
        let encoded = receipt.strip_prefix("open.v1.").ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
        document_open_plan_base64url_decode(encoded).map(Self)
    }

    #[cfg(test)]
    fn from_secret(secret: [u8; 32]) -> Self {
        Self(Box::new(secret))
    }

    fn digest(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(DOCUMENT_OPEN_PLAN_RECEIPT_DOMAIN);
        hash.update(self.0.as_ref());
        hash.finalize()
    }

    fn expose_once(&self) -> String {
        format!("open.v1.{}", document_open_plan_base64url_encode(self.0.as_ref()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DocumentOpenPlanAuthorityV1 {
    scope: DocumentScope,
    descriptor: DocumentDescriptor,
    descriptor_digest_v1: String,
    catalog: DocumentOpenCatalogV1,
    package: DocumentOpenPackageV1,
    artifact: DocumentOpenArtifactV1,
    parent_dialect: semio_framework::ArtifactDialect,
    surface: DocumentOpenSurfaceV1,
    grant: DocumentOpenGrantV1,
    checkpoint: Option<DocumentOpenCheckpointV1>,
    revalidation: DocumentOpenRevalidationV1,
    subject: SocketSubjectV1,
    server_actor_id: String,
    client_instance_id_digest: [u8; 32],
}

impl DocumentOpenPlanAuthorityV1 {
    fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        let descriptor_digest = os_directory::descriptor_digest_v1(&self.descriptor).map_err(|_| DocumentOpenPlanErrorCodeV1::Stale)?;
        let descriptor_digest = os_directory::hex_lower(&descriptor_digest.0);
        let descriptor_matches = self.descriptor.space_id == self.scope.space_id
            && self.descriptor.document_id == self.scope.document_id
            && descriptor_digest == self.descriptor_digest_v1
            && self.package.plugin_id == self.descriptor.owner.plugin_id
            && self.package.package_id == self.descriptor.owner.package_id
            && self.package.version == self.descriptor.owner.version
            && self.package.component_sha256 == self.descriptor.owner.package_hash
            && self.artifact.kind == self.descriptor.artifact_kind
            && self.parent_dialect.artifact_kind == self.artifact.kind
            && [&self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().all(|value| !value.is_empty() && value.len() <= 256 && value.trim() == value.as_str() && !value.chars().any(char::is_control))
            && self.artifact.schema == self.descriptor.artifact_schema
            && self.artifact.pack_schema_hash == self.descriptor.pack_schema_hash;
        let binding_matches = match &self.subject {
            SocketSubjectV1::Session { authorization_generation, role: Some(role), .. } => {
                self.revalidation.session_generation == Some(*authorization_generation) && self.revalidation.share_generation.is_none() && self.grant.write == matches!(role, SpaceRole::Author)
            }
            SocketSubjectV1::Share { scope, .. } => self.revalidation.session_generation.is_none() && self.revalidation.share_generation.is_some() && scope == &self.scope && !self.grant.write,
            SocketSubjectV1::Session { role: None, .. } => false,
        };
        if !descriptor_matches
            || !binding_matches
            || self.server_actor_id.is_empty()
            || self.client_instance_id_digest == [0; 32]
            || self.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1 || checkpoint.baseline_frontier.document_id != self.scope.document_id)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
        }
        Ok(())
    }

    fn public_plan(&self, receipt: String, expires_at_unix_ms: u64) -> DocumentOpenPlanV1 {
        DocumentOpenPlanV1 {
            schema: "semio.hub.document-open-plan/v1".into(),
            version: 1,
            receipt,
            expires_at_unix_ms,
            scope: self.scope.clone(),
            descriptor_digest_v1: self.descriptor_digest_v1.clone(),
            catalog: self.catalog.clone(),
            package: self.package.clone(),
            artifact: self.artifact.clone(),
            parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: self.parent_dialect.artifact_kind.clone(), standard: self.parent_dialect.standard.clone(), subset: self.parent_dialect.subset.clone() },
            surface: self.surface.clone(),
            grant: self.grant,
            checkpoint: self.checkpoint.clone(),
            revalidation: self.revalidation,
        }
    }

    fn binding_scope(&self) -> (SocketBindingKeyV1, String, String) {
        (self.subject.binding(), self.scope.space_id.clone(), self.scope.document_id.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentOpenPlanStateV1 {
    Issued,
    Consumed,
    Invalidated,
}

#[derive(Clone, Debug)]
struct DocumentOpenPlanRecordV1 {
    receipt_digest: [u8; 32],
    issued_at_ms: u64,
    expires_at_ms: u64,
    state: DocumentOpenPlanStateV1,
    authority: DocumentOpenPlanAuthorityV1,
    socket_grant_selector: Option<String>,
}

#[derive(Default)]
struct DocumentOpenPlanLedgerInnerV1 {
    records: BTreeMap<[u8; 32], DocumentOpenPlanRecordV1>,
    issued_by_binding: BTreeMap<SocketBindingKeyV1, BTreeSet<[u8; 32]>>,
    issued_by_binding_scope: BTreeMap<(SocketBindingKeyV1, String, String), [u8; 32]>,
}

#[derive(Default)]
struct DocumentOpenPlanLedgerV1 {
    inner: Mutex<DocumentOpenPlanLedgerInnerV1>,
}

impl DocumentOpenPlanLedgerV1 {
    fn remove_issued_indexes(inner: &mut DocumentOpenPlanLedgerInnerV1, record: &DocumentOpenPlanRecordV1) {
        let binding = record.authority.subject.binding();
        if let Some(receipts) = inner.issued_by_binding.get_mut(&binding) {
            receipts.remove(&record.receipt_digest);
            if receipts.is_empty() {
                inner.issued_by_binding.remove(&binding);
            }
        }
        let binding_scope = record.authority.binding_scope();
        if inner.issued_by_binding_scope.get(&binding_scope) == Some(&record.receipt_digest) {
            inner.issued_by_binding_scope.remove(&binding_scope);
        }
    }

    fn sweep_expired(inner: &mut DocumentOpenPlanLedgerInnerV1, now_ms: u64) {
        let expired = inner.records.iter().filter_map(|(digest, record)| (record.expires_at_ms <= now_ms).then_some(*digest)).collect::<Vec<_>>();
        for digest in expired {
            if let Some(record) = inner.records.remove(&digest) {
                Self::remove_issued_indexes(inner, &record);
            }
        }
    }

    fn issue(&self, authority: DocumentOpenPlanAuthorityV1, now_ms: u64, expires_at_ms: u64) -> Result<DocumentOpenPlanV1, DocumentOpenPlanErrorCodeV1> {
        self.issue_with_capability(authority, now_ms, expires_at_ms, DocumentOpenPlanCapabilityV1::mint()?)
    }

    fn issue_with_capability(&self, authority: DocumentOpenPlanAuthorityV1, now_ms: u64, expires_at_ms: u64, capability: DocumentOpenPlanCapabilityV1) -> Result<DocumentOpenPlanV1, DocumentOpenPlanErrorCodeV1> {
        authority.validate()?;
        let ttl = expires_at_ms.checked_sub(now_ms).ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
        let binding_expiry_ms = match &authority.subject {
            SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => u64::try_from(*expires_at_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?,
        };
        if ttl == 0 || ttl > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || expires_at_ms > binding_expiry_ms {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let receipt = capability.expose_once();
        let public = authority.public_plan(receipt, expires_at_ms);
        public.validate(now_ms)?;
        let receipt_digest = capability.digest();
        let binding = authority.subject.binding();
        let binding_scope = authority.binding_scope();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        Self::sweep_expired(&mut inner, now_ms);
        if inner.records.contains_key(&receipt_digest) {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        if let Some(previous_digest) = inner.issued_by_binding_scope.remove(&binding_scope) {
            if let Some(previous) = inner.records.get_mut(&previous_digest) {
                previous.state = DocumentOpenPlanStateV1::Invalidated;
            }
            if let Some(receipts) = inner.issued_by_binding.get_mut(&binding) {
                receipts.remove(&previous_digest);
            }
        }
        if inner.records.len() >= DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY || inner.issued_by_binding.get(&binding).map_or(0, BTreeSet::len) >= DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
            return Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded);
        }
        inner.issued_by_binding.entry(binding).or_default().insert(receipt_digest);
        inner.issued_by_binding_scope.insert(binding_scope, receipt_digest);
        inner.records.insert(receipt_digest, DocumentOpenPlanRecordV1 { receipt_digest, issued_at_ms: now_ms, expires_at_ms, state: DocumentOpenPlanStateV1::Issued, authority, socket_grant_selector: None });
        Ok(public)
    }

    fn authority_for_authenticated_exchange(&self, receipt: &str, scope: &DocumentScope, subject: &SocketSubjectV1, now_ms: u64) -> Result<DocumentOpenPlanAuthorityV1, DocumentOpenPlanErrorCodeV1> {
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let Some(record) = inner.records.get(&digest) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        if record.expires_at_ms <= now_ms {
            let expired = inner.records.remove(&digest).expect("record was present");
            Self::remove_issued_indexes(&mut inner, &expired);
            return Err(DocumentOpenPlanErrorCodeV1::Expired);
        }
        match record.state {
            DocumentOpenPlanStateV1::Consumed => return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed),
            DocumentOpenPlanStateV1::Invalidated => return Err(DocumentOpenPlanErrorCodeV1::Stale),
            DocumentOpenPlanStateV1::Issued => {}
        }
        if &record.authority.scope != scope || &record.authority.subject != subject {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let authority = record.authority.clone();
        authority.validate()?;
        Ok(authority)
    }

    fn exchange_record<R>(
        &self,
        receipt: &str,
        current: &DocumentOpenPlanAuthorityV1,
        now_ms: u64,
        complete: impl FnOnce(&DocumentOpenPlanAuthorityV1, u64) -> Result<(String, R), DocumentOpenPlanErrorCodeV1>,
    ) -> Result<R, DocumentOpenPlanErrorCodeV1> {
        current.validate()?;
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let Some(record) = inner.records.get(&digest) else { return Err(DocumentOpenPlanErrorCodeV1::Denied) };
        if record.expires_at_ms <= now_ms {
            let expired = inner.records.remove(&digest).expect("record was present");
            Self::remove_issued_indexes(&mut inner, &expired);
            return Err(DocumentOpenPlanErrorCodeV1::Expired);
        }
        match record.state {
            DocumentOpenPlanStateV1::Consumed => return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed),
            DocumentOpenPlanStateV1::Invalidated => return Err(DocumentOpenPlanErrorCodeV1::Stale),
            DocumentOpenPlanStateV1::Issued => {}
        }
        if &record.authority != current {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
        }
        let authority = record.authority.clone();
        let expires_at_ms = record.expires_at_ms;
        let (socket_grant_selector, output) = complete(&authority, expires_at_ms)?;
        if socket_grant_selector.is_empty() || socket_grant_selector.len() > AUTH_TEXT_MAX_BYTES {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let consumed = inner.records.get_mut(&digest).expect("record was present");
        consumed.state = DocumentOpenPlanStateV1::Consumed;
        consumed.socket_grant_selector = Some(socket_grant_selector);
        let consumed = consumed.clone();
        Self::remove_issued_indexes(&mut inner, &consumed);
        Ok(output)
    }

    #[cfg(test)]
    fn exchange(&self, receipt: &str, current: &DocumentOpenPlanAuthorityV1, now_ms: u64, socket_grant_selector: &str) -> Result<DocumentOpenPlanAuthorityV1, DocumentOpenPlanErrorCodeV1> {
        self.exchange_record(receipt, current, now_ms, |authority, _| Ok((socket_grant_selector.to_string(), authority.clone())))
    }

    fn exchange_to_socket_grant(&self, receipt: &str, current: &DocumentOpenPlanAuthorityV1, now_ms: u64, socket_grants: &SocketGrantLedgerV1) -> Result<SocketGrantReceiptV1, DocumentOpenPlanErrorCodeV1> {
        self.exchange_record(receipt, current, now_ms, |authority, plan_expires_at_ms| {
            let issued_at_ms = i64::try_from(now_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
            let plan_expires_at_ms = i64::try_from(plan_expires_at_ms).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
            let binding_expires_at_ms = match &authority.subject {
                SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
            };
            let expires_at_ms = issued_at_ms.checked_add(SOCKET_GRANT_TTL_MS).ok_or(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?.min(plan_expires_at_ms).min(binding_expires_at_ms);
            if expires_at_ms <= issued_at_ms {
                return Err(DocumentOpenPlanErrorCodeV1::Expired);
            }
            let capability = SocketGrantCapability::mint().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
            let selector = capability.selector().to_string();
            let audience = SocketAudienceV1::Document(authority.scope.clone());
            socket_grants.issue_with_document_plan(&capability, audience, authority.server_actor_id.clone(), authority.subject.clone(), issued_at_ms, expires_at_ms, Some(Arc::new(authority.clone()))).map_err(|error| match error {
                SocketGrantLedgerErrorV1::Capacity => DocumentOpenPlanErrorCodeV1::DeadlineExceeded,
                SocketGrantLedgerErrorV1::Rejected => DocumentOpenPlanErrorCodeV1::Denied,
            })?;
            let response = SocketGrantReceiptV1 { schema: "semio.hub.socket-grant/v1", protocol: SOCKET_PROTOCOL_V1, grant: capability.expose_once(), actor_id: authority.server_actor_id.clone(), expires_at_ms };
            Ok((selector, response))
        })
    }

    fn invalidate_binding(&self, binding: &SocketBindingKeyV1) -> usize {
        let Ok(mut inner) = self.inner.lock() else { return 0 };
        let digests = inner.records.iter().filter_map(|(digest, record)| (&record.authority.subject.binding() == binding && record.state == DocumentOpenPlanStateV1::Issued).then_some(*digest)).collect::<Vec<_>>();
        for digest in &digests {
            let indexed = if let Some(record) = inner.records.get_mut(digest) {
                record.state = DocumentOpenPlanStateV1::Invalidated;
                Some(record.clone())
            } else {
                None
            };
            if let Some(indexed) = indexed {
                Self::remove_issued_indexes(&mut inner, &indexed);
            }
        }
        digests.len()
    }

    fn invalidate_receipt(&self, receipt: &str) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        let capability = DocumentOpenPlanCapabilityV1::parse(receipt)?;
        let digest = capability.digest();
        let mut inner = self.inner.lock().map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?;
        let indexed = {
            let record = inner.records.get_mut(&digest).ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            if record.state != DocumentOpenPlanStateV1::Issued {
                return Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed);
            }
            record.state = DocumentOpenPlanStateV1::Invalidated;
            record.clone()
        };
        Self::remove_issued_indexes(&mut inner, &indexed);
        Ok(())
    }
}

struct SocketLiveLeaseV1 {
    ledger: Arc<SocketGrantLedgerV1>,
    record: SocketGrantRecordV1,
    id: String,
    notify: Arc<tokio::sync::Notify>,
}

impl Drop for SocketLiveLeaseV1 {
    fn drop(&mut self) {
        self.ledger.unregister_live(&self.record, &self.id);
    }
}

async fn socket_live_authority(state: &HubState, record: &SocketGrantRecordV1, live_id: &str) -> Result<Vec<tokio::sync::OwnedMutexGuard<()>>, SocketBindingValidityV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&record.subject, &record.audience)).await.map_err(|_| SocketBindingValidityV1::Unavailable)?;
    let validity = socket_binding_validity(state, &record.subject, &record.audience).await;
    if validity != SocketBindingValidityV1::Active {
        return Err(validity);
    }
    let plan_validity = document_plan_socket_validity(state, record, None).await;
    if plan_validity != SocketBindingValidityV1::Active {
        return Err(plan_validity);
    }
    if !state.socket_grants.is_live(record, live_id) {
        return Err(SocketBindingValidityV1::Unauthorized);
    }
    Ok(admission)
}

#[derive(Clone)]
struct HubState {
    db: Arc<db::Database>,
    artifact_cas: Arc<ArtifactChunkCasStores>,
    directory: Arc<HubDirectories>,
    rebootstrap: Arc<VerifiedRebootstrapSource>,
    _artifact_authority: Option<Arc<HubArtifactAuthority>>,
    verified_catalog: Option<Arc<VerifiedTrustedCatalog>>,
    gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>,
    openable_catalog: Option<Arc<dyn DocumentOpenCatalogAuthorityV1>>,
    _artifact_publication: Arc<HubArtifactPublication>,
    artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    /// @emoji 🏭️ Wave 1.B: the single serialized directory writer (contract §C1's decider laws +
    /// dense event `seq`) built once over `directory` at startup — see `semio_hub::directory::
    /// DirectoryService`'s own doc. `/directory/commands` and `/directory/invites/{token}/redeem`
    /// go through this; every other `/directory/*` route reads `directory` directly.
    directory_service: Arc<DirectoryService>,
    admin_subjects: Arc<[AdminSubject]>,
    admin_cursor_key: [u8; 32],
    admin_operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    admin_operation_slots: Arc<tokio::sync::Semaphore>,
    readiness: Arc<HubReadinessV1>,
    /// @emoji 🛡️ Contract §C0 `OS_HUB_ADMIN_DIR`: the admin SPA's static asset root. Lane 2-E owns
    /// the actual `/admin` file-serving handler (and its 503-if-missing stub) — this lane only
    /// carries the resolved path through `HubState` so that handler has something to read.
    // 🌵️ Unread until 2-E's handler lands and calls `state.admin_dir` — not dead code, just not
    // wired to a route yet (explicitly out of this lane's scope, see the doc above).
    #[allow(dead_code)]
    admin_dir: std::path::PathBuf,
    /// @emoji 📡️ Command-lane + preview-lane fan-out, one `broadcast::Sender` per v1 scope key —
    /// `db::Database`'s own `ArtifactHandle` exposes no live-subscription seam yet (see
    /// `db_engine`'s module doc: `subscribe`/`preview` are honest `Unimplemented` extension seams),
    /// so relaying newly-committed commands / preview blobs / presence updates to other connected
    /// sessions on the same document is this crate's own, deliberately thin responsibility — it
    /// never itself decides ordering or durability, only re-broadcasts what `db` already committed
    /// or what a preview/presence frame carries verbatim.
    fanout: Arc<ShardedMap<String, broadcast::Sender<ServerFrame>>>,
    fanout_capacity: usize,
    #[cfg(test)]
    live_gate: Option<Arc<TestLiveGate>>,
    #[cfg(test)]
    canonical_pair_authorization_gate: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    #[cfg(test)]
    canonical_pair_request_gate: Option<Arc<TestCanonicalPairRequestGate>>,
    #[cfg(test)]
    canonical_pair_deadline_ms: Option<u64>,
    #[cfg(test)]
    document_open_plan_issue_gate: Option<Arc<TestDocumentOpenPlanIssueGate>>,
    #[cfg(test)]
    document_open_plan_deadline_ms: Option<u64>,
    /// @emoji 👥️ `(document_scope_key_v1, actor)` -> that actor's presence session (contract §C7.3) — ephemeral,
    /// never durable (mirrors the preview lane's own law), rebuilt from nothing on hub restart. The
    /// roster is document-wide now (contract §C7.0): `ServerFrame::Presence` fans out on `fanout`, not
    /// a surface-scoped channel; a peer's `surface` travels INSIDE its `PresencePeer` bytes, stamped
    /// by the client actor, never decoded by this hub.
    presence: Arc<ShardedMap<(String, String), PresenceLeaseSlot>>,
    presence_publication_gate: Arc<tokio::sync::Mutex<()>>,
    #[cfg(test)]
    presence_clock: Option<Arc<TestPresenceClock>>,
    /// @emoji 🎨️ Contract §C7.3 session colors: `space_id` -> that space's live `(actor -> palette
    /// index)` leases. `acquire_color`/`release_color` below are the only mutators. Never persisted.
    session_colors: Arc<ShardedMap<String, SpaceColors>>,
    /// @emoji 🦵️ Wave 1.B admin kick: `syncSessionId` (the `SyncSessionRecord.id`/`ConnectionView.
    /// syncSessionId` the directory hands out on connect) -> a `Notify` the WS loop `select!`s on
    /// alongside its socket/broadcast reads. `POST /admin/api/connections/{syncSessionId}/close`
    /// fires it; the loop observes the wake-up and closes the connection on its own next tick —
    /// this map never itself closes a socket, only signals the session that owns it to.
    session_kicks: Arc<ShardedMap<String, Arc<tokio::sync::Notify>>>,
    socket_grants: Arc<SocketGrantLedgerV1>,
    document_open_plans: Arc<DocumentOpenPlanLedgerV1>,
    socket_binding_gates: Arc<SocketBindingGatesV1>,
    /// @emoji 🧩️ Installed runtime extensions mirrored from dev `/🧩️extension-modules` —
    /// populated by hub deploy copy / sideload; `GET /🧩️extension-modules` lists `install.json` rows.
    extensions_root: std::path::PathBuf,
    /// @emoji ⚖️ Contract §C9 ("hub `📦️bin.rs`: policy from config → `SubmitOptions.policy`"): the
    /// authority-local `protocol::MergePolicy` every `submit_commands` call on this hub instance
    /// judges a batch's worst graded conflict/message level against — read once at startup from
    /// `OS_HUB_MERGE_POLICY` (see `merge_policy_from_env`'s doc), never per-connection/per-space,
    /// matching `protocol::MergePolicy`'s own "local/authority state, never on the wire" law.
    merge_policy: protocol::MergePolicy,
}

impl HubState {
    fn fanout_for(&self, key: &str) -> broadcast::Sender<ServerFrame> {
        self.fanout.get_or_insert_with_cloned(key.to_string(), || broadcast::channel(self.fanout_capacity).0)
    }

    fn presence_now(&self) -> tokio::time::Instant {
        #[cfg(test)]
        if let Some(clock) = &self.presence_clock {
            return clock.now();
        }
        tokio::time::Instant::now()
    }

    /// 👥️ Produces one actor-sorted, defensively bounded roster snapshot from one map traversal.
    fn presence_snapshot(&self, key: &str) -> PresenceSnapshot {
        let mut rows = Vec::new();
        self.presence.for_each(|(scope, actor), slot| {
            if scope == key {
                if let Some(peer) = &slot.peer {
                    rows.push((actor.clone(), peer.clone(), DirectoryPresenceActor { actor: actor.clone(), user_id: slot.user_id.clone(), surface: slot.surface.clone(), color: slot.color }));
                }
            }
        });
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        let mut peers = Vec::with_capacity(rows.len().min(PRESENCE_ROSTER_MAXIMUM_ITEMS));
        let mut actors = Vec::with_capacity(peers.capacity());
        let mut bytes = 0usize;
        for (_, peer, actor) in rows {
            if peers.len() == PRESENCE_ROSTER_MAXIMUM_ITEMS || peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || bytes.checked_add(peer.len()).is_none_or(|next| next > PRESENCE_ROSTER_MAXIMUM_BYTES) {
                continue;
            }
            bytes += peer.len();
            peers.push(peer);
            actors.push(actor);
        }
        PresenceSnapshot { peers, actors }
    }

    /// 📡️ Publishes one document roster before its matching member-directory projection.
    fn publish_presence_delta(&self, key: &str, space_id: &str, document_id: &str, snapshot: PresenceSnapshot) {
        let _ = self.fanout_for(key).send(ServerFrame::Presence { peers: snapshot.peers });
        self.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.to_string(), document_id: document_id.to_string(), actors: snapshot.actors });
    }

    /// 🆕️ Selects one live socket as the actor's current owner without making it visible.
    async fn install_presence_slot(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, surface: &str, user_id: Option<&str>, color: u8, now: tokio::time::Instant) -> PresenceLeaseTransition {
        let Ok(_publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let replaced_visible = self.presence.with(&map_key, |slot| slot.is_some_and(|slot| slot.peer.is_some()));
        self.presence.insert(
            map_key,
            PresenceLeaseSlot { socket_live_id: socket_live_id.to_string(), expires_at: now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS), surface: surface.to_string(), user_id: user_id.map(str::to_string), color, peer: None },
        );
        if replaced_visible {
            self.publish_presence_delta(key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 💓️ Refreshes only the matching live owner and admits only a bounded visible roster.
    async fn refresh_presence(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, peer: Vec<u8>, now: tokio::time::Instant) -> PresenceLeaseTransition {
        if peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES {
            return PresenceLeaseTransition::Rejected;
        }
        let Ok(_publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let Some((was_visible, old_len, changed)) =
            self.presence.with(&map_key, |slot| slot.filter(|slot| slot.socket_live_id == socket_live_id).map(|slot| (slot.peer.is_some(), slot.peer.as_ref().map_or(0, Vec::len), slot.peer.as_ref() != Some(&peer))))
        else {
            return PresenceLeaseTransition::NoChange;
        };
        let mut visible = 0usize;
        let mut bytes = 0usize;
        self.presence.for_each(|(scope, _), slot| {
            if scope == key {
                if let Some(peer) = &slot.peer {
                    visible += 1;
                    bytes = bytes.saturating_add(peer.len());
                }
            }
        });
        if (!was_visible && visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS) || bytes.checked_sub(old_len).and_then(|base| base.checked_add(peer.len())).is_none_or(|next| next > PRESENCE_ROSTER_MAXIMUM_BYTES) {
            return PresenceLeaseTransition::Rejected;
        }
        self.presence.with_mut(&map_key, |slot| {
            if let Some(slot) = slot.filter(|slot| slot.socket_live_id == socket_live_id) {
                slot.expires_at = now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS);
                if changed {
                    slot.peer = Some(peer);
                }
            }
        });
        if changed {
            self.publish_presence_delta(key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// ⏳️ Hides a due visible peer while retaining the matching live owner slot.
    async fn expire_presence_for_live(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, now: tokio::time::Instant) -> PresenceLeaseTransition {
        let Ok(_publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let expired = self.presence.with_mut(&map_key, |slot| {
            let Some(slot) = slot.filter(|slot| slot.socket_live_id == socket_live_id && slot.peer.is_some() && now >= slot.expires_at) else { return false };
            slot.peer = None;
            true
        });
        if expired {
            self.publish_presence_delta(key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 🧹️ Removes only the matching live owner; a stale handler cannot erase its replacement.
    async fn close_presence_for_live(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str) -> PresenceLeaseTransition {
        let Ok(_publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let visible = self.presence.with(&map_key, |slot| slot.filter(|slot| slot.socket_live_id == socket_live_id).is_some_and(|slot| slot.peer.is_some()));
        if !self.presence.remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id) {
            return PresenceLeaseTransition::NoChange;
        }
        if visible {
            self.publish_presence_delta(key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// @emoji 🎨️ Contract §C7.3: an existing lease for `actor` in `space` is ref-counted and its
    /// index reused; otherwise the lowest index in `0..=255` not currently held by any live actor of
    /// `space`, wrapping `n % 256` once all 256 are taken.
    fn acquire_color(&self, space: &str, actor: &str) -> u8 {
        self.session_colors.mutate_or_default(space.to_string(), |colors| {
            if let Some(lease) = colors.by_actor.get_mut(actor) {
                lease.refs += 1;
                return lease.index;
            }
            let used: std::collections::BTreeSet<u8> = colors.by_actor.values().map(|lease| lease.index).collect();
            let index = (0..=255u8).find(|candidate| !used.contains(candidate)).unwrap_or((colors.by_actor.len() as u32 % 256) as u8);
            colors.by_actor.insert(actor.to_string(), ColorLease { index, refs: 1 });
            index
        })
    }

    /// @emoji 🎨️ `refs -= 1`, dropping the lease at 0 — freed on the last disconnect of that actor's
    /// shell session across all of its document sockets in `space`.
    fn release_color(&self, space: &str, actor: &str) {
        self.session_colors.with_mut(space, |colors| {
            let Some(colors) = colors else { return };
            let drop_lease = match colors.by_actor.get_mut(actor) {
                Some(lease) => {
                    lease.refs = lease.refs.saturating_sub(1);
                    lease.refs == 0
                }
                None => false,
            };
            if drop_lease {
                colors.by_actor.remove(actor);
            }
        });
    }

    /// @emoji 🗂️ Get-or-create: after the caller has authenticated and validated the durable
    /// descriptor, a document is lazily minted in `db`'s catalog on its first open. Concurrent
    /// opens resolve to the same live handle.
    async fn ensure_document(&self, id: &ProtocolArtifactId) -> Result<db::ArtifactHandle, db::DbError> {
        match self.db.document(id).await {
            Ok(handle) => Ok(handle),
            Err(db::DbError::NotFound(_)) => match self.db.create_document(db::ArtifactSpec::new(id.clone()).await).await {
                Ok(handle) => Ok(handle),
                Err(db::DbError::AlreadyExists(_)) => self.db.document(id).await,
                Err(other) => Err(other),
            },
            Err(other) => Err(other),
        }
    }
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
}

fn db_error_status(error: &db::DbError) -> StatusCode {
    match error {
        db::DbError::NotFound(_) => StatusCode::NOT_FOUND,
        db::DbError::AlreadyExists(_) | db::DbError::Conflict(_) => StatusCode::CONFLICT,
        db::DbError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        db::DbError::InvalidArgument(_) | db::DbError::LimitExceeded(_) => StatusCode::BAD_REQUEST,
        db::DbError::Unavailable(_) | db::DbError::Timeout(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// @emoji #⃣ Decodes a 64-hex-char blob URL path segment into a `db::ContentHash` — the inverse of
/// `ContentHash`'s `Display` (see `pack_core::ContentHash`), never trusted as-is (a malformed path
/// is `BAD_REQUEST`, not a panic).
fn parse_content_hash(hex: &str) -> Option<db::ContentHash> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let raw = hex.as_bytes();
    for (index, slot) in bytes.iter_mut().enumerate() {
        let byte_str = std::str::from_utf8(&raw[index * 2..index * 2 + 2]).ok()?;
        *slot = u8::from_str_radix(byte_str, 16).ok()?;
    }
    Some(db::ContentHash(bytes))
}
//#endregion 🔖️State

//#region 🔖️Auth
/// @emoji 🔎️ What a bearer token resolved to for document authority: an authenticated space
/// member, an exact document share, or nothing. Public discovery never enters this boundary.
enum AuthOutcome {
    Session { user_id: String, role: SpaceRole, session_id: String, authorization_generation: u64 },
    ShareToken,
    Denied,
}

/// @emoji 🔐️ Tries the bearer as an `AuthSessionRecord` (session id -> user -> space role) first;
/// falls back to an active exact-space/document share grant when session resolution fails.
async fn resolve_auth(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> AuthOutcome {
    let capability = token.and_then(|value| HubCapability::parse(value).ok());
    if let Some(HubCapability::Session(capability)) = &capability {
        if let Ok(Some(session)) = state.directory.authenticate_session(capability).await {
            if let Ok(Some(role)) = state.directory.get_role(space_id, &session.user_id).await {
                return AuthOutcome::Session { user_id: session.user_id, role, session_id: session.id, authorization_generation: session.authorization_generation };
            }
        }
    }
    let scope = DocumentScope::new(space_id, document_id);
    if let Some(HubCapability::Share(capability)) = &capability {
        if let Ok(true) = state.directory.authenticate_share(&scope, capability).await {
            return AuthOutcome::ShareToken;
        }
    }
    AuthOutcome::Denied
}

async fn authorized(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, space_id, document_id, token).await, AuthOutcome::Denied)
}

/// @emoji 📦️ A space-scoped blob requires a current persisted membership. Public discovery and
/// exact-document shares never widen into the whole space's content-addressed store.
async fn authorized_for_blob(state: &HubState, space_id: &str, hash: &str, token: Option<&str>) -> bool {
    matches!(resolve_auth(state, space_id, hash, token).await, AuthOutcome::Session { .. })
}

fn canonical_pair_auth_outcome_allowed(outcome: &AuthOutcome) -> bool {
    matches!(outcome, AuthOutcome::Session { .. } | AuthOutcome::ShareToken)
}

async fn authorized_for_canonical_pair(state: &HubState, scope: &DocumentScope, token: &str) -> bool {
    #[cfg(test)]
    if state.canonical_pair_authorization_gate.as_ref().is_some_and(|gate| !gate()) {
        return false;
    }
    canonical_pair_auth_outcome_allowed(&resolve_auth(state, &scope.space_id, &scope.document_id, Some(token)).await)
}
//#endregion 🔖️Auth

//#region 🔖️AdminAuth
#[derive(Clone)]
struct AdminSubject {
    provider_digest: [u8; 32],
    subject_digest: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AdminPrincipalV1 {
    user_id: String,
    auth_session_id: String,
    authorization_generation: u64,
    identity_provider: String,
    identity_subject_digest: [u8; 32],
    expires_at_ms: i64,
    correlation_id: String,
    peer_class: &'static str,
}

impl AdminPrincipalV1 {
    fn event_actor(&self) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#admin-session:{}", self.user_id, self.auth_session_id) }
    }

    fn same_authority(&self, other: &Self) -> bool {
        self.user_id == other.user_id
            && self.auth_session_id == other.auth_session_id
            && self.authorization_generation == other.authorization_generation
            && self.identity_provider == other.identity_provider
            && semio_hub::directory::constant_time_digest_eq(&self.identity_subject_digest, &other.identity_subject_digest)
            && other.expires_at_ms > now_ms()
    }
}

fn admin_provider_digest(provider: &str) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-provider/v1\0");
    hash.update(&(provider.len() as u32).to_be_bytes());
    hash.update(provider.as_bytes());
    hash.finalize()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HubMode {
    Production,
    Development,
}

impl HubMode {
    fn from_environment(bind: std::net::IpAddr) -> Result<Self, HubError> {
        match std::env::var("OS_HUB_MODE").ok().as_deref() {
            Some("production") => Ok(Self::Production),
            Some("development") => Ok(Self::Development),
            Some(_) => Err(HubError::UnsafeAuthConfiguration("OS_HUB_MODE must be development or production".into())),
            None if bind.is_loopback() => Ok(Self::Development),
            None => Ok(Self::Production),
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubReadinessV1 {
    schema: &'static str,
    status: &'static str,
    run_id: String,
    mode: &'static str,
    bind_scope: &'static str,
    authentication: HubAuthenticationReadinessV1,
    directory: HubComponentReadinessV1,
    storage: HubComponentReadinessV1,
    artifact_cas_barrier: HubComponentReadinessV1,
    artifact_publication: HubComponentReadinessV1,
    artifact_cas_sweeper: HubArtifactCasSweeperReadinessV1,
    artifact_authority: HubComponentReadinessV1,
    admin_assets: HubComponentReadinessV1,
    features: HubFeatureReadinessV1,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubAuthenticationReadinessV1 {
    kind: &'static str,
    bootstrap_ready: bool,
    public_session_issuance: bool,
}

#[derive(Clone, Serialize)]
struct HubComponentReadinessV1 {
    ready: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubArtifactCasSweeperReadinessV1 {
    ready: bool,
    execute: bool,
    default_mode: &'static str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HubFeatureReadinessV1 {
    open_plan: bool,
    open_plan_exchange: bool,
    rebootstrap: bool,
    mcp_workspace: bool,
    inference: bool,
}

fn hub_readiness(
    mode: HubMode,
    bind_scope: &'static str,
    run_id: String,
    bootstrap_ready: bool,
    artifact_authority_ready: bool,
    open_plan_ready: bool,
    admin_assets_ready: bool,
    artifact_cas_barrier_ready: bool,
    artifact_cas_sweep_execute: bool,
) -> HubReadinessV1 {
    let authentication_kind = match mode {
        HubMode::Development => "local-bootstrap-pipe-v1",
        HubMode::Production => "identity-assertion-verifier",
    };
    let required_ready = bootstrap_ready && artifact_authority_ready && admin_assets_ready && artifact_cas_barrier_ready;
    HubReadinessV1 {
        schema: "semio.hub.readiness/v1",
        status: if required_ready { "ready" } else { "not-ready" },
        run_id,
        mode: match mode {
            HubMode::Development => "development",
            HubMode::Production => "production",
        },
        bind_scope,
        authentication: HubAuthenticationReadinessV1 { kind: authentication_kind, bootstrap_ready, public_session_issuance: false },
        directory: HubComponentReadinessV1 { ready: true },
        storage: HubComponentReadinessV1 { ready: true },
        artifact_cas_barrier: HubComponentReadinessV1 { ready: artifact_cas_barrier_ready },
        artifact_publication: HubComponentReadinessV1 { ready: artifact_cas_barrier_ready },
        artifact_cas_sweeper: HubArtifactCasSweeperReadinessV1 { ready: artifact_cas_barrier_ready, execute: artifact_cas_sweep_execute, default_mode: "dry-run" },
        artifact_authority: HubComponentReadinessV1 { ready: artifact_authority_ready },
        admin_assets: HubComponentReadinessV1 { ready: admin_assets_ready },
        features: HubFeatureReadinessV1 { open_plan: open_plan_ready, open_plan_exchange: open_plan_ready, rebootstrap: true, mcp_workspace: false, inference: false },
    }
}

fn configured_admin_subjects() -> Result<Arc<[AdminSubject]>, HubError> {
    const ADMIN_SUBJECTS_MAX: usize = 64;
    let Some(encoded) = std::env::var("OS_HUB_ADMIN_SUBJECTS").ok().filter(|value| !value.is_empty()) else { return Ok(Arc::from([])) };
    let mut subjects = Vec::new();
    for entry in encoded.split(',') {
        if subjects.len() == ADMIN_SUBJECTS_MAX {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS exceeds 64 entries".into()));
        }
        let (provider, subject) = entry.split_once(':').ok_or_else(|| HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS entries must be provider:subject".into()))?;
        let subject_digest = identity_subject_digest(provider, subject).map_err(HubError::Directory)?;
        let provider_digest = admin_provider_digest(provider);
        if subjects.iter().any(|existing: &AdminSubject| semio_hub::directory::constant_time_digest_eq(&existing.provider_digest, &provider_digest) && semio_hub::directory::constant_time_digest_eq(&existing.subject_digest, &subject_digest)) {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS contains a duplicate identity".into()));
        }
        subjects.push(AdminSubject { provider_digest, subject_digest });
    }
    Ok(subjects.into())
}

fn validate_auth_startup(mode: HubMode, bind: std::net::IpAddr, verifier: Option<&Arc<dyn IdentityAssertionVerifier>>, local_bootstrap: Option<&Arc<dyn LocalBootstrapTransport>>, admin_subjects: &[AdminSubject]) -> Result<(), HubError> {
    match mode {
        HubMode::Production => {
            if verifier.is_none() {
                return Err(HubError::UnsafeAuthConfiguration("production requires an IdentityAssertionVerifier adapter".into()));
            }
            if admin_subjects.is_empty() {
                return Err(HubError::UnsafeAuthConfiguration("production requires OS_HUB_ADMIN_SUBJECTS".into()));
            }
            if !bind.is_loopback() {
                return Err(HubError::UnsafeAuthConfiguration("production cleartext HTTP/WebSocket may bind only to loopback".into()));
            }
        }
        HubMode::Development => {
            if !bind.is_loopback() {
                return Err(HubError::UnsafeAuthConfiguration("development mode must bind to loopback".into()));
            }
            if local_bootstrap.is_none() {
                return Err(HubError::UnsafeAuthConfiguration("development requires a protected LocalBootstrapTransport adapter".into()));
            }
        }
    }
    Ok(())
}

fn exact_admin_session_bearer(headers: &HeaderMap) -> Result<SessionCapability, StatusCode> {
    let mut values = headers.get_all(axum::http::header::AUTHORIZATION).iter();
    let value = values.next().ok_or(StatusCode::UNAUTHORIZED)?;
    if values.next().is_some() || value.as_bytes().len() > "Bearer ".len() + AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.is_empty() || encoded.len() > AUTH_TEXT_MAX_BYTES || encoded.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    SessionCapability::parse(encoded).map_err(|_| StatusCode::UNAUTHORIZED)
}

/// @emoji 🛡️ Resolves one request-owned verified administrator principal from a live durable session.
async fn authenticate_admin_principal(state: &HubState, headers: &HeaderMap, _peer: Option<SocketAddr>) -> Result<AdminPrincipalV1, StatusCode> {
    let capability = exact_admin_session_bearer(headers)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let provider_digest = admin_provider_digest(&session.identity_provider);
    if session.authorization_generation == 0
        || session.expires_at <= now_ms()
        || !state
            .admin_subjects
            .iter()
            .any(|subject| semio_hub::directory::constant_time_digest_eq(&subject.provider_digest, &provider_digest) && semio_hub::directory::constant_time_digest_eq(&subject.subject_digest, &session.identity_subject_digest))
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(AdminPrincipalV1 {
        user_id: session.user_id,
        auth_session_id: session.id,
        authorization_generation: session.authorization_generation,
        identity_provider: session.identity_provider,
        identity_subject_digest: session.identity_subject_digest,
        expires_at_ms: session.expires_at,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "admin-rest",
    })
}

async fn is_admin(state: &HubState, headers: &HeaderMap, peer: Option<SocketAddr>) -> bool {
    authenticate_admin_principal(state, headers, peer).await.is_ok()
}
//#endregion 🔖️AdminAuth

//#region 🔖️Rest
#[derive(Serialize)]
struct DocumentStatusResponse {
    document_id: String,
    head_seq: u64,
    commit_seq: u64,
    epoch: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SocketGrantReceiptV1 {
    schema: &'static str,
    protocol: &'static str,
    grant: String,
    actor_id: String,
    expires_at_ms: i64,
}

fn socket_issue_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    let values = headers.get_all(axum::http::header::AUTHORIZATION);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let capability = encoded.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if capability.is_empty() || capability.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(capability.to_string())
}

fn socket_actor_id(material: &[u8; 32], stable_session: bool) -> String {
    let mut digest = Sha256::new();
    digest.update(b"semio/hub/socket/actor/v1\0");
    digest.update(if stable_session { b"session" } else { b"share" });
    digest.update(material);
    format!("hub.v1.{}", semio_framework_hash::hex_lower(&digest.finalize()))
}

fn socket_text_bounded(value: &str) -> bool {
    !value.is_empty() && value.len() <= AUTH_TEXT_MAX_BYTES
}

async fn socket_binding_validity(state: &HubState, subject: &SocketSubjectV1, audience: &SocketAudienceV1) -> SocketBindingValidityV1 {
    let validity = tokio::time::timeout(std::time::Duration::from_secs(2), subject.revalidate(state.directory.as_ref(), audience, now_ms())).await.unwrap_or(SocketBindingValidityV1::Unavailable);
    if validity != SocketBindingValidityV1::Active {
        return validity;
    }
    let SocketAudienceV1::DirectoryScoped(scope) = audience else { return SocketBindingValidityV1::Active };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(scope)).await {
        Ok(Ok(Some(_))) => SocketBindingValidityV1::Active,
        Ok(Ok(None)) => SocketBindingValidityV1::Unauthorized,
        Ok(Err(_)) | Err(_) => SocketBindingValidityV1::Unavailable,
    }
}

async fn issue_socket_grant(state: &HubState, subject: SocketSubjectV1, audience: SocketAudienceV1, stable_actor_material: Option<[u8; 32]>) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    let binding = subject.binding();
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&subject, &audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &subject, &audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => return Err(StatusCode::SERVICE_UNAVAILABLE),
    }
    let capability = SocketGrantCapability::mint().map_err(directory_error_status)?;
    let now = now_ms();
    let binding_expiry = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => *expires_at_ms,
    };
    let expires_at_ms = now.checked_add(SOCKET_GRANT_TTL_MS).ok_or(StatusCode::SERVICE_UNAVAILABLE)?.min(binding_expiry);
    if expires_at_ms <= now {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let actor_id = socket_actor_id(&stable_actor_material.unwrap_or_else(|| capability.secret_digest()), stable_actor_material.is_some());
    state.socket_grants.issue(&capability, audience.clone(), actor_id.clone(), subject, now, expires_at_ms).map_err(|error| match error {
        SocketGrantLedgerErrorV1::Capacity => StatusCode::SERVICE_UNAVAILABLE,
        SocketGrantLedgerErrorV1::Rejected => StatusCode::UNAUTHORIZED,
    })?;
    let record = state.socket_grants.pending(&capability, &audience, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let validity = socket_binding_validity(state, &record.subject, &record.audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => {
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            return Err(StatusCode::UNAUTHORIZED);
        }
        SocketBindingValidityV1::Unavailable => {
            state.socket_grants.reject_pending(&record.selector);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    }
    Ok(Json(SocketGrantReceiptV1 { schema: "semio.hub.socket-grant/v1", protocol: SOCKET_PROTOCOL_V1, grant: capability.expose_once(), actor_id, expires_at_ms }))
}

async fn authenticate_document_socket_subject(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, Option<[u8; 32]>), DocumentOpenPlanErrorCodeV1> {
    let bearer = socket_issue_bearer(headers).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
    let capability = HubCapability::parse(&bearer).map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
    let (subject, stable_actor_material) = match capability {
        HubCapability::Session(capability) => {
            let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            let role = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_role(&scope.space_id, &session.user_id))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            let material = session.secret_digest;
            let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at };
            (subject, Some(material))
        }
        HubCapability::Share(capability) => {
            let share = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_share_binding(&scope, &capability))
                .await
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .map_err(|_| DocumentOpenPlanErrorCodeV1::DeadlineExceeded)?
                .ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
            (SocketSubjectV1::Share { share_id: share.id, selector: share.selector, scope: scope.clone(), expires_at_ms: share.expires_at }, None)
        }
        HubCapability::Invite(_) => return Err(DocumentOpenPlanErrorCodeV1::Denied),
    };
    Ok((subject, stable_actor_material))
}

type DocumentOpenPlanRouteError = (StatusCode, DirectoryJson<DocumentOpenPlanErrorV1>);

fn document_open_plan_route_error(status: StatusCode, code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    (status, DirectoryJson(DocumentOpenPlanErrorV1 { schema: "semio.hub.document-open-plan-error/v1".into(), code }))
}

fn document_open_plan_exchange_error(code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    let status = match code {
        DocumentOpenPlanErrorCodeV1::Denied => StatusCode::UNAUTHORIZED,
        DocumentOpenPlanErrorCodeV1::NotFound => StatusCode::NOT_FOUND,
        DocumentOpenPlanErrorCodeV1::Expired => StatusCode::GONE,
        DocumentOpenPlanErrorCodeV1::Stale | DocumentOpenPlanErrorCodeV1::AlreadyConsumed => StatusCode::CONFLICT,
        DocumentOpenPlanErrorCodeV1::Cancelled => StatusCode::REQUEST_TIMEOUT,
        DocumentOpenPlanErrorCodeV1::CatalogUnavailable | DocumentOpenPlanErrorCodeV1::ComponentUnavailable | DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
    };
    document_open_plan_route_error(status, code)
}

fn document_open_checkpoint(checkpoint: os_directory::PublishedArtifactCheckpoint) -> DocumentOpenCheckpointV1 {
    DocumentOpenCheckpointV1 {
        checkpoint_id: os_directory::hex_lower(&checkpoint.checkpoint_id.0),
        descriptor_digest_v1: os_directory::hex_lower(&checkpoint.descriptor_digest_v1.0),
        baseline_frontier: checkpoint.baseline_frontier,
        aggregate_sha256: os_directory::hex_lower(&checkpoint.aggregate_sha256.0),
    }
}

async fn issue_document_open_plan_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES
        || content_types.iter().count() != 1
        || content_types.iter().next().and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let intent: DocumentOpenIntentV1 = directory::os_pack::json::from_json_str(encoded).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    intent.validate().map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let scope = DocumentScope::new(space_id, document_id);
    if intent.scope != scope {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    if !state.readiness.features.open_plan {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable));
    }
    let (subject, stable_actor_material) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&subject, &audience)).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let descriptor =
        state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    let writable = matches!(subject, SocketSubjectV1::Session { role: Some(SpaceRole::Author), .. });
    let selected = catalog.resolve_document_open(&descriptor, intent.requested_surface_id.as_deref(), writable).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision == 0 || directory_revision > os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    }
    let descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&descriptor).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale))?.0);
    let checkpoint = state.directory.get_active_artifact_checkpoint(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.map(document_open_checkpoint);
    let (session_generation, share_generation) = match &subject {
        SocketSubjectV1::Session { authorization_generation, .. } => (Some(*authorization_generation), None),
        SocketSubjectV1::Share { .. } => (None, Some(1)),
    };
    let mut client_instance_id_digest = Sha256::new();
    client_instance_id_digest.update(b"semio/hub/document-open/client-instance/v1\0");
    client_instance_id_digest.update(intent.client_instance_id.as_bytes());
    let client_instance_id_digest = client_instance_id_digest.finalize();
    let mut ephemeral_actor_material = [0u8; 32];
    let (actor_material, stable_session) = if let Some(material) = stable_actor_material {
        (material, true)
    } else {
        directory::os_identity::fill_entropy(&mut ephemeral_actor_material).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
        (ephemeral_actor_material, false)
    };
    let authority = DocumentOpenPlanAuthorityV1 {
        scope: scope.clone(),
        descriptor,
        descriptor_digest_v1,
        catalog: DocumentOpenCatalogV1 { generation_id: catalog.generation_id().to_string() },
        package: selected.package,
        artifact: selected.artifact,
        parent_dialect: selected.parent_dialect,
        surface: selected.surface,
        grant: selected.grant,
        checkpoint,
        revalidation: DocumentOpenRevalidationV1 { directory_revision, membership_generation: directory_revision, session_generation, share_generation },
        subject: subject.clone(),
        server_actor_id: socket_actor_id(&actor_material, stable_session),
        client_instance_id_digest,
    };
    ephemeral_actor_material.fill(0);
    authority.validate().map_err(document_open_plan_exchange_error)?;
    #[cfg(test)]
    if let Some(gate) = &state.document_open_plan_issue_gate {
        gate.admitted.add_permits(1);
        gate.release.acquire().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Cancelled))?.forget();
    }
    if state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))? != directory_revision {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let issued_at_ms = u64::try_from(now_ms()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    let binding_expiry_ms = match &subject {
        SocketSubjectV1::Session { expires_at_ms, .. } | SocketSubjectV1::Share { expires_at_ms, .. } => u64::try_from(*expires_at_ms).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Expired))?,
    };
    let expires_at_ms = issued_at_ms.checked_add(DOCUMENT_OPEN_PLAN_MAX_TTL_MS).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.min(binding_expiry_ms);
    if expires_at_ms <= issued_at_ms {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Expired));
    }
    state.document_open_plans.issue(authority, issued_at_ms, expires_at_ms).map(DirectoryJson).map_err(document_open_plan_exchange_error)
}

async fn issue_document_open_plan(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<DirectoryJson<DocumentOpenPlanV1>, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    #[cfg(test)]
    let deadline_ms = state.document_open_plan_deadline_ms.unwrap_or(DOCUMENT_OPEN_PLAN_DEADLINE_MS);
    #[cfg(not(test))]
    let deadline_ms = DOCUMENT_OPEN_PLAN_DEADLINE_MS;
    tokio::time::timeout(std::time::Duration::from_millis(deadline_ms), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        issue_document_open_plan_inner(space_id, document_id, parts.headers, state, body).await
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))
}

async fn issue_document_plan_socket_grant_inner(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES
        || content_types.iter().count() != 1
        || content_types.iter().next().and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    let intent: DocumentPlanSocketGrantIntentV1 = directory::os_pack::json::from_json_str(encoded).map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    intent.validate().map_err(|_| document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied))?;
    if !state.readiness.features.open_plan_exchange {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable));
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, _) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = state.socket_binding_gates.acquire_record(&subject, &audience).await;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let exchange_at_ms = u64::try_from(now_ms()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    let authority = state.document_open_plans.authority_for_authenticated_exchange(&intent.plan_receipt, &scope, &subject, exchange_at_ms).map_err(document_open_plan_exchange_error)?;
    let descriptor = state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if descriptor.as_ref() != Some(&authority.descriptor) {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    if catalog.generation_id() != authority.catalog.generation_id
        || catalog
            .resolve_document_open(&authority.descriptor, Some(&authority.surface.surface_id), authority.grant.write)
            .is_none_or(|selected| selected.package != authority.package || selected.artifact != authority.artifact || selected.parent_dialect != authority.parent_dialect || selected.surface != authority.surface || selected.grant != authority.grant)
    {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision != authority.revalidation.directory_revision || directory_revision != authority.revalidation.membership_generation {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    state.document_open_plans.exchange_to_socket_grant(&intent.plan_receipt, &authority, exchange_at_ms, state.socket_grants.as_ref()).map(Json).map_err(document_open_plan_exchange_error)
}

async fn issue_document_plan_socket_grant(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<Json<SocketGrantReceiptV1>, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    tokio::time::timeout(std::time::Duration::from_secs(2), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        issue_document_plan_socket_grant_inner(space_id, document_id, parts.headers, state, body).await
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))
}

#[cfg(test)]
async fn issue_document_socket_grant_fixture(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, stable_actor_material) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(|error| match error {
        DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::UNAUTHORIZED,
    })?;
    let descriptor = state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?;
    if descriptor.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    issue_socket_grant(&state, subject, SocketAudienceV1::Document(scope), stable_actor_material).await
}

async fn issue_directory_socket_grant(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    let capability = SessionCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let material = session.secret_digest;
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at };
    issue_socket_grant(&state, subject, audience, Some(material)).await
}

async fn issue_scoped_directory_socket_grant(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) || !body.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let capability = SessionCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let session =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let scope = DocumentScope::new(space_id, document_id);
    let role = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.socket_session_binding(&session.id, &session.user_id, session.authorization_generation, Some(&scope.space_id), now_ms())).await {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), expires_at_ms })) if expires_at_ms == session.expires_at => role,
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
        _ => return Err(StatusCode::UNAUTHORIZED),
    };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(&scope)).await {
        Ok(Ok(Some(_))) => {}
        Ok(Ok(None)) => return Err(StatusCode::NOT_FOUND),
        Ok(Err(_)) | Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    }
    let material = session.secret_digest;
    let subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at };
    issue_socket_grant(&state, subject, SocketAudienceV1::DirectoryScoped(scope), Some(material)).await
}

#[derive(Serialize)]
struct BlobRecord {
    hash: String,
    media_type: String,
    size: i64,
}

fn canonical_pair_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    let values = headers.get_all(axum::http::header::AUTHORIZATION);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let capability = encoded.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if capability.is_empty() || capability.len() > AUTH_TEXT_MAX_BYTES || !matches!(HubCapability::parse(capability), Ok(HubCapability::Session(_)) | Ok(HubCapability::Share(_))) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(capability.to_string())
}

fn canonical_pair_request_admission(uri: &axum::http::Uri, headers: &HeaderMap) -> Result<String, StatusCode> {
    if uri.query().is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if headers.contains_key(axum::http::header::RANGE) {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    if headers.get_all(axum::http::header::ACCEPT).iter().count() != 1 || headers.get(axum::http::header::ACCEPT).and_then(|value| value.to_str().ok()) != Some(CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE) {
        return Err(StatusCode::NOT_ACCEPTABLE);
    }
    canonical_pair_bearer(headers)
}

const CANONICAL_PAIR_PROGRESS_STAGES: usize = 8;

fn canonical_pair_progress_index(stage: RebootstrapProgressStage) -> usize {
    match stage {
        RebootstrapProgressStage::Authorize => 0,
        RebootstrapProgressStage::Metadata => 1,
        RebootstrapProgressStage::VerifyPack => 2,
        RebootstrapProgressStage::VerifySpr => 3,
        RebootstrapProgressStage::StreamPack => 4,
        RebootstrapProgressStage::StreamSpr => 5,
        RebootstrapProgressStage::Ready => 6,
        RebootstrapProgressStage::Chunk => 7,
    }
}

struct CanonicalPairHttpControl {
    cancelled: std::sync::atomic::AtomicBool,
    active: std::sync::atomic::AtomicBool,
    progress: Mutex<[Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES]>,
}

impl CanonicalPairHttpControl {
    fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true), progress: Mutex::new([None; CANONICAL_PAIR_PROGRESS_STAGES]) }
    }

    fn request_cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    fn is_active(&self) -> bool {
        self.active.load(std::sync::atomic::Ordering::Acquire)
    }

    #[cfg(test)]
    fn progress_snapshot(&self) -> [Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES] {
        *self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

struct CanonicalPairHttpRequest {
    control: Arc<CanonicalPairHttpControl>,
    response_owned: bool,
}

impl CanonicalPairHttpRequest {
    fn new(control: Arc<CanonicalPairHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    fn finish_response_owned(&mut self) {
        self.response_owned = true;
        self.control.release();
    }
}

impl Drop for CanonicalPairHttpRequest {
    fn drop(&mut self) {
        if !self.response_owned {
            self.control.request_cancel();
            self.control.release();
        }
    }
}

#[cfg(test)]
struct TestCanonicalPairRequestGate {
    entered: tokio::sync::Semaphore,
    release: tokio::sync::Semaphore,
    control: Mutex<Option<Arc<CanonicalPairHttpControl>>>,
}

#[cfg(test)]
impl TestCanonicalPairRequestGate {
    fn new(release_permits: usize) -> Self {
        Self { entered: tokio::sync::Semaphore::new(0), release: tokio::sync::Semaphore::new(release_permits), control: Mutex::new(None) }
    }

    async fn enter(&self, control: Arc<CanonicalPairHttpControl>) {
        *self.control.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(control);
        self.entered.add_permits(1);
        let _permit = self.release.acquire().await.expect("canonical pair request gate open");
    }

    fn control(&self) -> Arc<CanonicalPairHttpControl> {
        self.control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("canonical pair request control captured")
    }
}

impl RebootstrapTransferControl for CanonicalPairHttpControl {
    fn now_ms(&self) -> u64 {
        u64::try_from(now_ms()).unwrap_or(u64::MAX)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }

    fn report(&self, progress: RebootstrapProgress) {
        if progress.total_units == 0 || progress.completed_units > progress.total_units {
            return;
        }
        let mut stages = self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut stages[canonical_pair_progress_index(progress.stage)];
        if slot.is_none_or(|prior| prior.total_units == progress.total_units && prior.completed_units <= progress.completed_units) {
            *slot = Some(progress);
        }
    }
}

fn canonical_pair_error_status(error: RebootstrapError) -> StatusCode {
    match error {
        RebootstrapError::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        RebootstrapError::Unavailable => StatusCode::NOT_FOUND,
        RebootstrapError::ResourceLimit => StatusCode::PAYLOAD_TOO_LARGE,
        RebootstrapError::Cancelled => StatusCode::SERVICE_UNAVAILABLE,
        RebootstrapError::AuthorityIdentityChanged | RebootstrapError::Integrity => StatusCode::CONFLICT,
    }
}

/// 🧭️ Exact authenticated, path-only public projection of one active canonical checkpoint pair.
async fn get_active_checkpoint_pair(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = match canonical_pair_request_admission(&uri, &headers) {
        Ok(token) => token,
        Err(status) => return status.into_response(),
    };
    let scope = DocumentScope::new(space_id, document_id);
    let control = Arc::new(CanonicalPairHttpControl::new());
    let mut request = CanonicalPairHttpRequest::new(control.clone());
    let deadline_ms = REBOOTSTRAP_DEADLINE_MS;
    #[cfg(test)]
    let deadline_ms = state.canonical_pair_deadline_ms.unwrap_or(deadline_ms);
    let operation = async {
        #[cfg(test)]
        if let Some(gate) = &state.canonical_pair_request_gate {
            gate.enter(control.clone()).await;
        }
        canonical_pair_response(&state, &scope, &token, control.as_ref(), deadline_ms).await
    };
    let response = match tokio::time::timeout(std::time::Duration::from_millis(deadline_ms), operation).await {
        Ok(response) => response,
        Err(_) => {
            control.request_cancel();
            StatusCode::GATEWAY_TIMEOUT.into_response()
        }
    };
    request.finish_response_owned();
    response
}

async fn canonical_pair_response(state: &HubState, scope: &DocumentScope, token: &str, control: &CanonicalPairHttpControl, deadline_ms: u64) -> Response {
    let deadline = control.now_ms().saturating_add(deadline_ms);
    let context = RebootstrapContext::new(deadline, control);
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    control.report(RebootstrapProgress { stage: RebootstrapProgressStage::Authorize, completed_units: 1, total_units: 1 });
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    let pair = match state.rebootstrap.active_pair(scope, &context).await {
        Ok(pair) => pair,
        Err(error) => return canonical_pair_error_status(error).into_response(),
    };
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let mut body = Vec::new();
    if let Err(error) = append_canonical_pair_header(&mut body, &pair.selection) {
        return canonical_pair_error_status(error).into_response();
    }
    for ordinal in 0..pair.data_record_count() {
        if !authorized_for_canonical_pair(state, scope, token).await {
            return StatusCode::UNAUTHORIZED.into_response();
        }
        let record = match pair.data_record(ordinal, &context) {
            Ok(Some(record)) => record,
            Ok(None) => return StatusCode::CONFLICT.into_response(),
            Err(error) => return canonical_pair_error_status(error).into_response(),
        };
        if let Err(error) = append_canonical_pair_data(&mut body, &record) {
            return canonical_pair_error_status(error).into_response();
        }
    }
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if let Err(error) = append_canonical_pair_terminal(&mut body, CanonicalPairTerminal::Complete) {
        return canonical_pair_error_status(error).into_response();
    }
    let etag = match canonical_pair_etag(&pair.selection).ok().and_then(|value| value.parse().ok()) {
        Some(value) => value,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let mut response = Bytes::from(body).into_response();
    response.headers_mut().insert(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static(CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE));
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("private, no-store"));
    response.headers_mut().insert(axum::http::header::VARY, axum::http::HeaderValue::from_static("Authorization"));
    response.headers_mut().insert(axum::http::header::ETAG, etag);
    control.report(RebootstrapProgress { stage: RebootstrapProgressStage::Ready, completed_units: 4, total_units: 4 });
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    response
}

/// @emoji 🧭️ A durably announced document's current frontier.
async fn get_document_status(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentStatusResponse>, StatusCode> {
    if !authorized(&state, &space_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let scope = DocumentScope::new(space_id, document_id);
    if state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let handle = state.ensure_document(&db_artifact_id(&scope)).await.map_err(|e| db_error_status(&e))?;
    let frontier = handle.frontier().await.map_err(|e| db_error_status(&e))?;
    Ok(Json(DocumentStatusResponse { document_id: scope.document_id, head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch }))
}

//#region Blobs
const HUB_BLOB_MAX_BYTES: usize = db::db_storage::DB_IO_PAGE_BYTES * db::db_storage::DB_IO_OPERATION_PAGES;

async fn db_io_pages_into_http_bytes(mut pages: db::db_storage::DbIoPages) -> Result<Bytes, db::DbError> {
    if pages.len() > HUB_BLOB_MAX_BYTES {
        while !pages.terminal_is_empty() {
            pages.close_step()?;
            semio_framework_async::yield_once().await;
        }
        return Err(db::DbError::LimitExceeded("hub blob response bytes"));
    }
    let mut body = Vec::with_capacity(pages.len());
    for fragment in pages.fragments() {
        body.extend_from_slice(fragment);
    }
    while !pages.terminal_is_empty() {
        pages.close_step()?;
        semio_framework_async::yield_once().await;
    }
    Ok(Bytes::from(body))
}

async fn put_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<BlobRecord>, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let media_type = headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").to_string();
    let size = body.len();
    let pages = db::db_storage::db_io_copy_pages(body.as_ref()).map_err(|error| db_error_status(&error))?.await.map_err(|error| db_error_status(&error))?;
    let computed = state.db.storage().await.payload().await.put(pages).await.map_err(|error| db_error_status(&error))?;
    let computed_hex = computed.to_string();
    // The path hash is client-supplied (content-addressed URL); a mismatch against the
    // storage-computed hash means the client sent the wrong bytes for that address — a bad
    // request, distinct from a document CAS conflict.
    if computed_hex != hash {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(BlobRecord { hash: computed_hex, media_type, size: size as i64 }))
}

async fn get_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let content_hash = parse_content_hash(&hash).ok_or(StatusCode::BAD_REQUEST)?;
    match state.db.storage().await.payload().await.get(&content_hash).await {
        Ok(pages) => {
            let bytes = db_io_pages_into_http_bytes(pages).await.map_err(|error| db_error_status(&error))?;
            Ok(([(axum::http::header::CONTENT_TYPE, "application/octet-stream")], bytes))
        }
        Err(error) => Err(db_error_status(&error)),
    }
}

async fn head_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref()).await {
        return StatusCode::UNAUTHORIZED;
    }
    let Some(content_hash) = parse_content_hash(&hash) else { return StatusCode::BAD_REQUEST };
    match state.db.storage().await.payload().await.contains(&content_hash).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion Blobs
//#endregion 🔖️Rest

//#region 🔖️WebSocket
fn socket_grant_from_protocol_header(headers: &HeaderMap) -> Result<SocketGrantCapability, StatusCode> {
    if headers.contains_key(axum::http::header::AUTHORIZATION) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let values = headers.get_all(axum::http::header::SEC_WEBSOCKET_PROTOCOL);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let offered = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if offered.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (protocol, grant) = offered.split_once(", ").ok_or(StatusCode::UNAUTHORIZED)?;
    if protocol != SOCKET_PROTOCOL_V1 || grant.contains(',') {
        return Err(StatusCode::UNAUTHORIZED);
    }
    SocketGrantCapability::parse(grant).map_err(|_| StatusCode::UNAUTHORIZED)
}

async fn document_plan_socket_validity(state: &HubState, record: &SocketGrantRecordV1, surface: Option<&str>) -> SocketBindingValidityV1 {
    let Some(authority) = record.document_plan.as_deref() else { return SocketBindingValidityV1::Active };
    if !state.readiness.features.open_plan || !state.readiness.features.open_plan_exchange {
        return SocketBindingValidityV1::Unavailable;
    }
    if authority.validate().is_err() {
        return SocketBindingValidityV1::Unauthorized;
    }
    if record.subject != authority.subject || record.actor_id != authority.server_actor_id || record.audience != SocketAudienceV1::Document(authority.scope.clone()) || surface.is_some_and(|surface| surface != authority.surface.surface_id) {
        return SocketBindingValidityV1::Unauthorized;
    }
    let descriptor = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(&authority.scope)).await {
        Ok(Ok(Some(descriptor))) => descriptor,
        Ok(Ok(None)) => return SocketBindingValidityV1::Unauthorized,
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if descriptor != authority.descriptor {
        return SocketBindingValidityV1::Unauthorized;
    }
    let Some(catalog) = state.openable_catalog.as_ref() else { return SocketBindingValidityV1::Unavailable };
    if catalog.generation_id() != authority.catalog.generation_id
        || catalog.resolve_document_open(&descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selection| {
            selection.package != authority.package || selection.artifact != authority.artifact || selection.parent_dialect != authority.parent_dialect || selection.surface != authority.surface || selection.grant != authority.grant
        })
    {
        return SocketBindingValidityV1::Unauthorized;
    }
    let directory_revision = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.head_seq()).await {
        Ok(Ok(revision)) => revision,
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if directory_revision != authority.revalidation.directory_revision || directory_revision != authority.revalidation.membership_generation {
        return SocketBindingValidityV1::Unauthorized;
    }
    let checkpoint = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_active_artifact_checkpoint(&authority.scope)).await {
        Ok(Ok(checkpoint)) => checkpoint.map(document_open_checkpoint),
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if checkpoint != authority.checkpoint {
        return SocketBindingValidityV1::Unauthorized;
    }
    SocketBindingValidityV1::Active
}

async fn consume_socket_grant(state: &HubState, headers: &HeaderMap, audience: SocketAudienceV1, surface: Option<&str>) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending(&capability, &audience, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&candidate.subject, &candidate.audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &candidate.subject, &candidate.audience).await;
    match validity {
        SocketBindingValidityV1::Active => match document_plan_socket_validity(state, &candidate, surface).await {
            SocketBindingValidityV1::Active => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| StatusCode::UNAUTHORIZED),
            SocketBindingValidityV1::Unauthorized => {
                state.socket_grants.reject_pending(&candidate.selector);
                Err(StatusCode::UNAUTHORIZED)
            }
            SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
        },
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn consume_directory_socket_grant(state: &HubState, headers: &HeaderMap) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending_directory(&capability, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&candidate.subject, &candidate.audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &candidate.subject, &candidate.audience).await;
    match validity {
        SocketBindingValidityV1::Active => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DocumentWsV1Query {
    surface: Option<String>,
}

async fn document_ws_v1(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, axum::extract::Query(query): axum::extract::Query<DocumentWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let surface = query.surface.unwrap_or_default();
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) || surface.len() > AUTH_TEXT_MAX_BYTES {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(&space_id, &document_id);
    let admission = match consume_socket_grant(&state, &headers, SocketAudienceV1::Document(scope), Some(&surface)).await {
        Ok(admission) => admission,
        Err(status) => return (status, "socket grant rejected").into_response(),
    };
    ws.protocols([SOCKET_PROTOCOL_V1]).on_upgrade(move |socket| handle_ws(socket, space_id, document_id, surface, state, admission)).into_response()
}

async fn encode(frame: &ServerFrame) -> Message {
    Message::Binary(encode_server_frame(frame, Lane::Command).await.into())
}

async fn error_frame(code: &str, message: impl Into<String>) -> Message {
    encode(&ServerFrame::Error { code: code.to_string(), message: message.into() }).await
}

struct SocketRebootstrapControl;

impl RebootstrapTransferControl for SocketRebootstrapControl {
    fn now_ms(&self) -> u64 {
        now_ms().max(0) as u64
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: RebootstrapProgress) {}
}

fn wire_rebootstrap(control: &os_directory::RebootstrapRequired) -> protocol::RebootstrapRequired {
    protocol::RebootstrapRequired {
        space_id: control.scope.space_id.clone(),
        document_id: control.scope.document_id.clone(),
        checkpoint_id: control.checkpoint_id.0,
        descriptor_hash: control.descriptor_digest_v1.0,
        baseline_frontier: RuntimeFrontierSummary {
            document_id: ProtocolArtifactId(control.baseline_frontier.document_id.clone()),
            head_edit_ordinal: control.baseline_frontier.head_edit_ordinal,
            head_edit_id: control.baseline_frontier.head_edit_id.clone(),
            last_commit_seq: control.baseline_frontier.last_commit_seq,
            chain_hash: control.baseline_frontier.chain_hash.0,
        },
    }
}

async fn verified_rebootstrap_control(state: &HubState, scope: &DocumentScope) -> Option<os_directory::RebootstrapRequired> {
    let control = SocketRebootstrapControl;
    let deadline = control.now_ms().saturating_add(REBOOTSTRAP_DEADLINE_MS);
    state.rebootstrap.control(scope, &RebootstrapContext::new(deadline, &control)).await.ok()
}

async fn send_socket_document_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, scope: &DocumentScope) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority(state, record, live_id).await {
        Ok(admission) => admission,
        Err(validity) => return validity,
    };
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_rebootstrap_read.add_permits(1);
    }
    let control = match tokio::time::timeout(std::time::Duration::from_secs(2), verified_rebootstrap_control(state, scope)).await {
        Ok(control) => control,
        Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if let Some(control) = control {
        let frame = encode(&ServerFrame::RebootstrapRequired { control: wire_rebootstrap(&control) }).await;
        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
            return SocketBindingValidityV1::Unavailable;
        }
    }
    SocketBindingValidityV1::Active
}

/// @emoji 🧭️ Best-effort `RuntimeFrontierSummary` for an `Ack` when the triggering `submit` itself
/// failed — re-reads the document's current (unaffected) frontier so the client still learns
/// "where the server actually is", falling back to an all-zero genesis summary only if even that
/// read fails (a document wedged badly enough that this happens has bigger problems than one Ack).
async fn best_effort_frontier(handle: &db::ArtifactHandle) -> RuntimeFrontierSummary {
    match handle.frontier().await {
        Ok(frontier) => engine_frontier_to_wire(&frontier, String::new()),
        Err(_) => RuntimeFrontierSummary { document_id: handle.document_id().await.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] },
    }
}

fn engine_frontier_to_wire(frontier: &db::db_engine::Frontier, head_edit_id: String) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: frontier.document.clone(), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// @emoji ⚖️ `OS_HUB_MERGE_POLICY=laissez-faire|normal|vigilant` (default `normal`) — read once at
/// startup into `HubState.merge_policy` (see its own doc). An unrecognized value is a non-fatal
/// misconfiguration (logged, falls back to the default) rather than refusing to boot, matching this
/// crate's generally-forgiving stance on env parsing elsewhere in `main`.
fn merge_policy_from_env() -> protocol::MergePolicy {
    match std::env::var("OS_HUB_MERGE_POLICY").ok().as_deref() {
        None => protocol::MergePolicy::default(),
        Some("laissez-faire") => protocol::MergePolicy::LaissezFaire,
        Some("normal") => protocol::MergePolicy::Normal,
        Some("vigilant") => protocol::MergePolicy::Vigilant,
        Some(other) => {
            eprintln!("[WARN] unknown OS_HUB_MERGE_POLICY '{other}' (expected laissez-faire|normal|vigilant), defaulting to normal");
            protocol::MergePolicy::default()
        }
    }
}

/// @emoji 🧾️ `ApplyOutcome::Rejected.messages`'s canonical JSON payload, encoded from the
/// first-party `ToValue` shape shared by every replication wire consumer.
fn encode_messages(messages: &[protocol::MutationMessage]) -> Vec<u8> {
    let value = DslValue::Array(messages.iter().map(ToValue::to_value).collect());
    directory::os_pack::json::to_json_string(&value).into_bytes()
}

/// @emoji 🧾️ Every `protocol::MutationMessage` `error` carries, if any — non-empty only for
/// `db::DbError::Rejected` (the outcome-step gate `db_artifact::ArtifactEngine::submit` returns per
/// contract §C9); every other `DbError` variant has nothing to add here.
fn messages_for_error(error: &db::DbError) -> Vec<u8> {
    match error {
        db::DbError::Rejected { messages, .. } => encode_messages(messages),
        _ => Vec::new(),
    }
}

/// @emoji ✍️ Submits `envelopes` as one `db_artifact::CommandBatch` through `handle`, returning the
/// `Ack` to send the submitter plus (on acceptance) the `Commands` frame to fan out to every other
/// session on the same document. `Fsync` durability: a hub session's `submit` genuinely committing
/// is the promise `AckStage::Persisted` makes to the client. `policy` is `HubState.merge_policy`
/// (contract §C9) — the outcome-step gate `handle.submit` runs before any WAL append.
///
/// 🎯️ Design choice (accepted-but-degraded messages have no relay carrier yet): when `policy`
/// admits a batch whose worst graded level is still `Warning`-or-above (a "degraded merge", contract
/// §C5), `receipt.messages` is non-empty but neither `ApplyOutcome::Accepted` nor
/// `ServerFrame::Commands` (both fieldless/message-less in the CURRENTLY LANDED `📡️wire` shape —
/// verified against `📡️spr/📡️wire/🦀️.rs`) has anywhere to carry them to the submitter's
/// peers. `📡️wire` is lane 1-C's lease, already landed `ApplyOutcome::Rejected{reason, messages}`
/// for this exact contract clause's rejected half; widening `Accepted`/`Commands` further is a wire
/// change this lane is not authorized to make unilaterally (per the worker brief's "if you must
/// touch a file outside your lease, STOP and report instead"), so `receipt.messages` is deliberately
/// dropped here rather than silently faked onto a field that doesn't exist — see this ticket's
/// report for the gap.
async fn submit_commands(handle: &db::ArtifactHandle, actor: &ActorId, batch_id: u64, envelopes: Vec<MutationEnvelope>, policy: protocol::MergePolicy) -> (ServerFrame, Option<ServerFrame>) {
    let batch = match db::document::CommandBatch::new(envelopes.clone()).await {
        Ok(batch) => batch,
        Err(error) => {
            let frontier = best_effort_frontier(handle).await;
            return (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages: Vec::new() }) }], frontier }, None);
        }
    };
    match handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy }).await {
        Ok(Ok(receipt)) => {
            let frontier = engine_frontier_to_wire(&receipt.frontier, receipt.command_id.0.clone());
            let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() };
            let commands = ServerFrame::Commands { envelopes, origin: actor.clone(), frontier };
            (ack, Some(commands))
        }
        Ok(Err(error)) | Err(error) => {
            let frontier = best_effort_frontier(handle).await;
            let messages = messages_for_error(&error);
            (ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages }) }], frontier }, None)
        }
    }
}

/// @emoji 🚪️ Runs `envelopes` through `gate.admit_command` one at a time (tenant isolation, then
/// `Action::Write` authz on `AuthzScope::CommandKind`, DoS budget, replay dedupe — see
/// `SecurityGate::admit_command`'s own doc) before any of them reach `db::ArtifactHandle::submit`.
/// Returns the first rejection reason, or `None` once every envelope is admitted. `kind` is a
/// constant ("write") rather than a per-envelope command-kind string: this crate sits above
/// `db_artifact`'s pipeline and never interprets an operation's schema/diff semantics (matches
/// `db_security`'s own module doc — payload interpretation stays out of this layer), so command-kind
/// granularity inside one document is not this wave's concern.
async fn admit_writes(gate: &db::security::SecurityGate, principal: &db::security::Principal, tenant: &db::security::TenantId, document: &ProtocolArtifactId, envelopes: &[MutationEnvelope], physical_ms: u64) -> Option<String> {
    for envelope in envelopes {
        if let Err(error) = gate.admit_command(principal, tenant, document, "write", &envelope.actor, &envelope.mutation_id, physical_ms).await {
            return Some(error.to_string());
        }
    }
    None
}

/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session.
/// session. Returns `false` when the session should close (`Bye`, or a send failure).
#[allow(clippy::too_many_arguments)]
async fn handle_client_frame(
    state: &HubState,
    handle: &db::ArtifactHandle,
    db_id: &ProtocolArtifactId,
    key: &str,
    space_id: &str,
    document_id: &str,
    fanout: &broadcast::Sender<ServerFrame>,
    actor: &ActorId,
    socket_live_id: &str,
    gate: &db::security::SecurityGate,
    principal: &db::security::Principal,
    tenant: &db::security::TenantId,
    frame: ClientFrame,
    sender: &mut SplitSink<WebSocket, Message>,
) -> bool {
    match frame {
        ClientFrame::Commands { batch_id, envelopes } => {
            if envelopes.iter().any(|envelope| &envelope.actor != actor) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "socket subject actor mismatch".into(), messages: Vec::new() }) }], frontier };
                return sender.send(encode(&ack).await).await.is_ok();
            }
            if let Some(reason) = admit_writes(gate, principal, tenant, db_id, &envelopes, now_ms().max(0) as u64).await {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason, messages: Vec::new() }) }], frontier };
                return sender.send(encode(&ack).await).await.is_ok();
            }
            let (ack, relay) = submit_commands(handle, actor, batch_id, envelopes, state.merge_policy).await;
            if let Some(commands_frame) = relay {
                let _ = fanout.send(commands_frame);
            }
            sender.send(encode(&ack).await).await.is_ok()
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            let core_document = db_core_document_id(db_id);
            match db::sync::handle_frontier_advertise(&state.db.storage().await.wal().await, core_document, &frontier, actor.clone()).await {
                Ok(Some(catch_up)) => sender.send(encode(&catch_up).await).await.is_ok(),
                Ok(None) => true,
                Err(_) => true,
            }
        }
        ClientFrame::PreviewPublish { key: preview_key, seq, payload } => {
            let _ = fanout.send(ServerFrame::Preview { actor: actor.clone(), key: preview_key, seq, payload });
            true
        }
        ClientFrame::Presence { peer } => {
            let _ = state.refresh_presence(key, space_id, document_id, &actor.0, socket_live_id, peer, state.presence_now()).await;
            true
        }
        // 🪙️ Command-lane credit-based flow control: no server-side congestion control implemented
        // this wave (matches `framework/sync`'s client, which also accepts and ignores this frame).
        ClientFrame::CreditGrant { .. } => true,
        ClientFrame::Bye => false,
        ClientFrame::SocketHelloV1 { .. } => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            false
        }
    }
}

async fn handle_ws(socket: WebSocket, space_id: String, document_id: String, surface: String, state: HubState, socket_admission: SocketGrantAdmissionV1) {
    let (mut sender, mut receiver) = socket.split();
    let socket_grant = socket_admission.record;

    let hello = match tokio::time::timeout(std::time::Duration::from_secs(2), receiver.next()).await {
        Ok(Some(Ok(Message::Binary(bytes)))) => decode_client_frame(&bytes).await.ok().map(|(_lane, frame)| frame),
        _ => None,
    };
    let (schema, pack_schema_hash, actor, frontier, auth) = match hello {
        Some(ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema, pack_schema_hash, resume_token, frontier }) if socket_text_bounded(&schema) && resume_token.as_ref().is_none_or(|value| value.len() <= AUTH_TEXT_MAX_BYTES) => {
            let actor = ActorId(socket_grant.actor_id.clone());
            let auth = match &socket_grant.subject {
                SocketSubjectV1::Session { session_id, user_id, authorization_generation, role: Some(role), .. } => {
                    AuthOutcome::Session { user_id: user_id.clone(), role: *role, session_id: session_id.clone(), authorization_generation: *authorization_generation }
                }
                SocketSubjectV1::Share { .. } => AuthOutcome::ShareToken,
                SocketSubjectV1::Session { role: None, .. } => AuthOutcome::Denied,
            };
            (schema, pack_schema_hash, actor, frontier, auth)
        }
        _ => {
            let _ = sender.send(error_frame("protocol", "expected socket hello").await).await;
            return;
        }
    };

    let (user_id, role, auth_session_id, authorization_generation) = match &auth {
        AuthOutcome::Session { user_id, role, session_id, authorization_generation } => (Some(user_id.clone()), Some(*role), Some(session_id.as_str()), *authorization_generation),
        AuthOutcome::ShareToken => (None, None, None, 0),
        AuthOutcome::Denied => {
            let _ = sender.send(error_frame("unauthorized", "unauthorized").await).await;
            return;
        }
    };
    let scope = DocumentScope::new(&space_id, &document_id);
    let descriptor = match state.directory.get_document_descriptor(&scope).await {
        Ok(Some(descriptor)) => descriptor,
        Ok(None) => {
            let _ = sender.send(error_frame("document-not-announced", "document has no durable descriptor").await).await;
            return;
        }
        Err(error) => {
            let _ = sender.send(error_frame("directory", error.to_string()).await).await;
            return;
        }
    };
    let announced_hash = pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    if schema != descriptor.artifact_schema || announced_hash != descriptor.pack_schema_hash {
        let _ = sender.send(error_frame("schema-hash-mismatch", "hello codec identity does not match the durable document descriptor").await).await;
        return;
    }
    let key = document_scope_key_v1(&scope);
    // 🎨️ Contract §C7.3: acquired after successful SocketHelloV1 admission and before `Welcome`, released at
    // handler exit (every early-return path below releases it explicitly; the loop-exit cleanup
    // releases it on a clean disconnect).
    let color = state.acquire_color(&space_id, &actor.0);

    // 🔒️ Per-connection `SecurityGate`: `space_grants` compiles this space's `kind` into
    // author=rw/spectator=ro grants (archive additionally deny-overrides author writes), a fresh
    // `RoleBasedPolicy` from them, and a `Principal` carrying the caller's resolved role. A share-
    // token caller (no session role) is admitted as `"spectator"` — read-only, the
    // least-privilege default for a connection this crate cannot attribute to a real member.
    // `TenantId` reuses the space id: this crate has no separate tenant concept yet, and every
    // scope this gate ever evaluates already belongs to exactly this one space/document connection.
    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map_or_else(|| "studio".to_string(), |space| space.kind);
    let policy = db::security::space_grants(&space_id, &space_kind).await.into_iter().fold(db::security::RoleBasedPolicy::new(), db::security::RoleBasedPolicy::with_grant);
    let gate = db::security::SecurityGate::new(policy, db::security::ReplayGuard::new(60_000, 256), db::security::BudgetRegistry::new(240, 60), Arc::new(db::NullEmit));
    let tenant = db::security::TenantId::from(space_id.clone());
    // 🎯️ Role mapping: share-grant callers are least-privilege spectators. Only an
    // authenticated directory membership can confer author authority.
    let role_str = match &auth {
        AuthOutcome::Session { role, .. } => role.as_str().to_string(),
        AuthOutcome::ShareToken => "spectator".to_string(),
        AuthOutcome::Denied => unreachable!("Denied already returned above"),
    };
    let principal = db::security::Principal::new(actor.clone(), tenant.clone(), vec![role_str]);

    let db_id = db_artifact_id(&scope);
    let handle = match state.ensure_document(&db_id).await {
        Ok(handle) => handle,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };

    let session_id = directory::os_identity::time_ordered_id();
    let mut hello_session = match state.db.hello(db_id.clone(), frontier, session_id, actor.clone(), 64 * 1024).await {
        Ok(session) => session,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let welcome = match hello_session.take_welcome() {
        Ok(welcome) => welcome,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let welcome_bytes = match welcome.frame() {
        Ok(frame) => encode(frame).await,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let socket_binding_gates = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&socket_grant.subject, &socket_grant.audience)).await {
        Ok(admission) => admission,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let socket_live = {
        let (id, notify) = match state.socket_grants.register_live(&socket_grant) {
            Ok(live) => live,
            Err(_) => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        };
        let lease = SocketLiveLeaseV1 { ledger: state.socket_grants.clone(), record: socket_grant.clone(), id, notify };
        let validity = tokio::time::timeout(std::time::Duration::from_secs(2), socket_grant.subject.revalidate(state.directory.as_ref(), &socket_grant.audience, now_ms())).await.unwrap_or(SocketBindingValidityV1::Unavailable);
        let validity = if validity == SocketBindingValidityV1::Active { document_plan_socket_validity(&state, &socket_grant, Some(&surface)).await } else { validity };
        match validity {
            SocketBindingValidityV1::Active => lease,
            SocketBindingValidityV1::Unauthorized => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
            SocketBindingValidityV1::Unavailable => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        }
    };
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_before_welcome.add_permits(1);
        let _ = gate.socket_welcome_release.acquire().await;
    }
    let welcome_sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(welcome_bytes)).await;
    let welcome_acknowledged = welcome.acknowledge();
    if !matches!(welcome_sent, Ok(Ok(()))) || welcome_acknowledged.is_err() {
        hello_session.cancel();
        state.release_color(&space_id, &actor.0);
        return;
    }
    drop(socket_binding_gates);
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_after_welcome.add_permits(1);
        let _ = gate.socket_bootstrap_release.acquire().await;
    }
    loop {
        match hello_session.next_frame().await {
            Ok(Some(frame)) => {
                let frame_bytes = match frame.frame() {
                    Ok(owner) => encode(owner).await,
                    Err(error) => {
                        let _ = sender.send(error_frame("storage", error.to_string()).await).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                };
                let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                    Ok(admission) => admission,
                    Err(SocketBindingValidityV1::Unauthorized) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                    Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                };
                let sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame_bytes)).await;
                let acknowledged = frame.acknowledge();
                if !matches!(sent, Ok(Ok(()))) || acknowledged.is_err() {
                    hello_session.cancel();
                    state.release_color(&space_id, &actor.0);
                    return;
                }
            }
            Ok(None) => break,
            Err(error) => {
                let _ = sender.send(error_frame("storage", error.to_string()).await).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        }
    }
    // 🎨️ Contract §C7.3: sent exactly once per connection, after `Welcome` (and its follow-up
    // bootstrap frames) and before any `Presence` frame.
    let session_frame = encode(&ServerFrame::Session { actor: actor.0.clone(), color }).await;
    let _session_authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
        Ok(admission) => admission,
        Err(SocketBindingValidityV1::Unauthorized) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(session_frame)).await, Ok(Ok(()))) {
        state.release_color(&space_id, &actor.0);
        return;
    }
    let _ = state.install_presence_slot(&key, &space_id, &document_id, &actor.0, &socket_live.id, &surface, user_id.as_deref(), color, state.presence_now()).await;
    drop(_session_authority);

    let fanout = state.fanout_for(&key);
    let mut broadcast_rx = fanout.subscribe();
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.document_subscribed.add_permits(1);
        let _ = gate.document_release.acquire().await;
    }

    let authenticated_email = match user_id.as_deref() {
        Some(user_id) => state.directory.get_user(user_id).await.ok().flatten().map(|user| user.email),
        None => None,
    };
    let sync_session = state.directory.record_sync_session_open(auth_session_id, authorization_generation, &actor.0, &space_id, &document_id, &surface, user_id.as_deref(), authenticated_email.as_deref(), role, &actor.0).await.ok();
    if let Some(session) = &sync_session {
        let view = connection_view(&state, session).await;
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Opened, connection: view });
    }
    // 🦵️ Admin kick: only a session the directory actually recorded gets a live `Notify` registered
    // under its `syncSessionId` (see `session_kicks`' own doc) — a session that failed to record
    // (e.g. directory hiccup) falls back to a `Notify` nobody can ever reach, i.e. un-kickable, which
    // matches this crate's generally forgiving stance on directory-write failures elsewhere in this
    // handler.
    let kick = match &sync_session {
        Some(session) => {
            let notify = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(session.id.clone(), notify.clone());
            notify
        }
        None => Arc::new(tokio::sync::Notify::new()),
    };
    let mut authorization_tick = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = authorization_tick.tick() => {
                match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                    Ok(_) => {
                        let _ = state.expire_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id, state.presence_now()).await;
                    }
                    Err(SocketBindingValidityV1::Unauthorized) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                        break;
                    }
                    Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                        break;
                    }
                }
            }
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => {
                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        };
                            #[cfg(test)]
                            if let Some(live_gate) = &state.live_gate {
                                live_gate.socket_command_received.add_permits(1);
                                let _ = live_gate.socket_command_release.acquire().await;
                            }
                            let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                                Ok(admission) => admission,
                                Err(SocketBindingValidityV1::Unauthorized) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                    break;
                                }
                                Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                    break;
                                }
                            };
                            match tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &socket_live.id, &gate, &principal, &tenant, frame, &mut sender),
                            )
                            .await
                            {
                                Ok(true) => {}
                                Ok(false) => break,
                                Err(_) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                    break;
                                }
                            }
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = broadcast_rx.recv() => {
                match event {
                    Ok(frame) => {
                        #[cfg(test)]
                        if let Some(live_gate) = &state.live_gate {
                            live_gate.socket_broadcast_received.add_permits(1);
                            let _ = live_gate.socket_broadcast_release.acquire().await;
                        }
                        let frame = encode(&frame).await;
                        let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                            Ok(admission) => admission,
                            Err(SocketBindingValidityV1::Unauthorized) => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                break;
                            }
                            Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                break;
                            }
                        };
                        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        #[cfg(test)]
                        if let Some(live_gate) = &state.live_gate {
                            live_gate.socket_lag_received.add_permits(1);
                            let _ = live_gate.socket_lag_release.acquire().await;
                        }
                        match send_socket_document_rebootstrap(&mut sender, &state, &socket_grant, &socket_live.id, &scope).await {
                            SocketBindingValidityV1::Active => {
                                let _ = tokio::time::timeout(
                                    std::time::Duration::from_secs(2),
                                    sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "rebootstrap-required".into() }))),
                                )
                                .await;
                            }
                            SocketBindingValidityV1::Unauthorized => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            }
                            SocketBindingValidityV1::Unavailable => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                            }
                        }
                        break;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = kick.notified() => break,
            _ = async {
                socket_live.notify.notified().await
            } => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                break;
            }
        }
    }

    if let Some(session) = sync_session {
        let view = connection_view(&state, &session).await;
        let _ = state.directory.record_sync_session_close(&session.id).await;
        state.session_kicks.remove(&session.id);
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Closed, connection: view });
    }
    let _ = state.close_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id).await;
    state.release_color(&space_id, &actor.0);
}
//#endregion 🔖️WebSocket

//#region 🔖️Directory
/// @emoji 🙋️ A bearer token resolved to a live, unexpired `AuthSessionRecord`'s user — every
/// `/directory/*`/`/auth/sessions/me` route that needs a caller identity resolves through this
/// (distinct from `AuthOutcome`, which can also carry an exact document share; the directory
/// control plane has no such fallback — a command with no valid session is unauthenticated).
#[derive(Clone)]
struct AuthedUser {
    user_id: String,
    session_id: String,
    expires_at: i64,
    authorization_generation: u64,
    capability: SessionCapability,
}

async fn resolve_bearer_user(state: &HubState, token: Option<&str>) -> Option<AuthedUser> {
    let capability = SessionCapability::parse(token?).ok()?;
    let session = state.directory.authenticate_session(&capability).await.ok().flatten()?;
    Some(AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability })
}

fn directory_error_status(error: DirectoryError) -> StatusCode {
    match error {
        DirectoryError::NotFound(_) => StatusCode::NOT_FOUND,
        DirectoryError::Conflict(_) => StatusCode::CONFLICT,
        DirectoryError::Unauthorized => StatusCode::UNAUTHORIZED,
        DirectoryError::Backend(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

trait DirectoryEventPageSource {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError>;
    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError>;
}

impl DirectoryEventPageSource for HubDirectories {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
        HubDirectory::head_seq(self).await
    }

    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
        HubDirectory::events_since(self, since, limit).await
    }
}

/// 📖️ Reads a bounded complete suffix in fixed pages, cursoring by observed sequence.
async fn load_all_directory_events<S: DirectoryEventPageSource + ?Sized>(directory: &S, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryError> {
    let head = directory.directory_event_head().await?;
    let expected = head.saturating_sub(since);
    if expected > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
        return Err(DirectoryError::Conflict(format!("directory event suffix exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
    }
    let mut cursor = since;
    let capacity = usize::try_from(expected).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    let mut events = Vec::with_capacity(capacity);
    loop {
        let page = directory.directory_event_page(cursor, DIRECTORY_EVENT_READ_MAX).await?;
        let page_len = page.len();
        if page_len == 0 {
            break;
        }
        for event in &page {
            if event.seq <= cursor {
                return Err(DirectoryError::Backend("directory event page did not strictly advance its cursor".into()));
            }
            cursor = event.seq;
        }
        events.extend(page);
        if u64::try_from(events.len()).map_err(|error| DirectoryError::Conflict(error.to_string()))? > DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS {
            return Err(DirectoryError::Conflict(format!("directory event suffix exceeds fixed maximum {DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS}")));
        }
        if page_len < DIRECTORY_EVENT_READ_MAX {
            break;
        }
    }
    Ok(events)
}

/// @emoji 📇️ Rebuilds `DirectoryReadModel` from the bounded complete public event suffix.
async fn load_read_model(state: &HubState) -> Result<DirectoryReadModel, StatusCode> {
    let events = load_all_directory_events(state.directory.as_ref(), 0).await.map_err(directory_error_status)?;
    Ok(os_directory::fold_all(DirectoryReadModel::default(), &events).await)
}

fn role_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

fn invite_view(record: InviteRecord) -> InviteView {
    InviteView { id: record.id, space_id: record.space_id, role: role_wire(record.role), created_at_ms: record.created_at, expires_at_ms: record.expires_at, revoked: record.revoked_at.is_some() }
}

/// @emoji 🔴️ `ConnectionView` for one live `SyncSessionRecord` — `presenceKnown` cross-references
/// `state.presence` (contract: "connections = `list_active_sync_sessions()` joined with the
/// in-memory presence map").
async fn connection_view(state: &HubState, session: &SyncSessionRecord) -> ConnectionView {
    let email = match &session.user_id {
        Some(id) => state.directory.get_user(id).await.ok().flatten().map(|user| user.email),
        None => None,
    };
    connection_view_with_email(state, session, email)
}

fn connection_view_with_email(state: &HubState, session: &SyncSessionRecord, email: Option<String>) -> ConnectionView {
    let scope = document_scope_key_v1(&DocumentScope::new(&session.space_id, &session.document_id));
    let presence_known = state.presence.with(&(scope, session.client_label.clone()), |entry| entry.is_some_and(|entry| entry.peer.is_some()));
    ConnectionView {
        sync_session_id: session.id.clone(),
        space_id: session.space_id.clone(),
        document_id: session.document_id.clone(),
        surface: session.surface.clone(),
        actor: session.client_label.clone(),
        user_id: session.user_id.clone(),
        email,
        role: session.space_role.map(role_wire).unwrap_or(DirectorySpaceRole::Spectator),
        connected_at_ms: session.connected_at,
        presence_known,
    }
}

/// @emoji 📄️ Durable directory descriptors enriched with each opened DB handle's current
/// frontier; unopened documents retain the descriptor's authoritative bootstrap frontier.
async fn documents_for_space(state: &HubState, space_id: &str) -> Vec<DocumentView> {
    let mut views = Vec::new();
    let Ok(descriptors) = state.directory.list_document_descriptors(space_id).await else { return views };
    for descriptor in descriptors {
        views.push(document_view(state, descriptor).await);
    }
    views
}

/// 📖️ Builds the public catalog directly from durable descriptors, never from the
/// current-frontier [`DocumentView`] used by members and D1.
async fn public_documents_for_space(state: &HubState, space_id: &str) -> Result<Vec<PublicDocumentCatalogEntryV1>, StatusCode> {
    Ok(state
        .directory
        .list_document_descriptors(space_id)
        .await
        .map_err(directory_error_status)?
        .into_iter()
        .map(|descriptor| PublicDocumentCatalogEntryV1 {
            document_id: descriptor.document_id,
            artifact_kind: descriptor.artifact_kind,
            artifact_schema: descriptor.artifact_schema,
            owner: descriptor.owner,
            pack_schema_hash: descriptor.pack_schema_hash,
        })
        .collect())
}

async fn document_view(state: &HubState, descriptor: DocumentDescriptor) -> DocumentView {
    let db_id = db_artifact_id(&DocumentScope::new(&descriptor.space_id, &descriptor.document_id));
    let frontier = match state.db.document(&db_id).await {
        Ok(handle) => handle.frontier().await.ok().map(|frontier| (frontier.head_seq, frontier.commit_seq, frontier.epoch)),
        Err(_) => None,
    }
    .unwrap_or((descriptor.bootstrap_frontier.head_seq, descriptor.bootstrap_frontier.commit_seq, descriptor.bootstrap_frontier.epoch));
    DocumentView { descriptor, head_seq: frontier.0, commit_seq: frontier.1, epoch: frontier.2 }
}

/// @emoji 🏠️ Fills a folded `DirectorySpace`'s `SpaceView` with the two fields the pure fold cannot
/// know: the CALLING user's own `role` (server-filled per request, never derived by `fold`) and the
/// live `document_count`/`active_connections` (owned by `db`'s catalog and the directory's sync
/// sessions respectively, neither of which the directory event log itself tracks).
async fn space_view(state: &HubState, space: &os_directory::DirectorySpace, caller: Option<&AuthedUser>) -> SpaceView {
    let mut view = space.view.clone();
    view.role = caller.and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id).map(|member| member.role));
    view.document_count = documents_for_space(state, &view.id).await.len() as u32;
    view.active_connections = state.directory.list_active_sync_sessions(Some(&view.id), ACTIVE_SYNC_SESSION_READ_MAX).await.map(|sessions| sessions.len() as u32).unwrap_or(0);
    view
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectorySpaceAccessDecisionV1 {
    Hidden,
    Public,
    Member,
    Author,
}

impl DirectorySpaceAccessDecisionV1 {
    fn is_member(self) -> bool {
        matches!(self, Self::Member | Self::Author)
    }
}

/// 🛡️ The single public/member/author decision used by REST reads and both stream paths.
fn directory_space_access_decision(public: bool, role: Option<DirectorySpaceRole>) -> DirectorySpaceAccessDecisionV1 {
    match role {
        Some(DirectorySpaceRole::Author) => DirectorySpaceAccessDecisionV1::Author,
        Some(DirectorySpaceRole::Spectator) => DirectorySpaceAccessDecisionV1::Member,
        None if public => DirectorySpaceAccessDecisionV1::Public,
        None => DirectorySpaceAccessDecisionV1::Hidden,
    }
}

fn public_space_view(space: &os_directory::DirectorySpace, document_count: usize) -> PublicSpaceViewV1 {
    PublicSpaceViewV1 {
        id: space.view.id.clone(),
        name: space.view.name.clone(),
        kind: space.view.kind,
        visibility: space.view.visibility,
        member_count: space.view.member_count,
        document_count: u32::try_from(document_count).unwrap_or(u32::MAX),
        created_at_ms: space.view.created_at_ms,
        updated_at_ms: space.view.updated_at_ms,
    }
}

async fn member_space_view(state: &HubState, space: &os_directory::DirectorySpace, role: DirectorySpaceRole) -> MemberSpaceViewV1 {
    let document_count = documents_for_space(state, &space.view.id).await.len() as u32;
    let active_connections = state.directory.list_active_sync_sessions(Some(&space.view.id), ACTIVE_SYNC_SESSION_READ_MAX).await.map(|sessions| sessions.len() as u32).unwrap_or(0);
    MemberSpaceViewV1 {
        id: space.view.id.clone(),
        name: space.view.name.clone(),
        kind: space.view.kind,
        visibility: space.view.visibility,
        owner_user_id: space.view.owner_user_id.clone(),
        role,
        member_count: space.view.member_count,
        document_count,
        active_connections,
        created_at_ms: space.view.created_at_ms,
        updated_at_ms: space.view.updated_at_ms,
    }
}

/// @emoji ⚖️ Contract §C2's command authorization matrix: `create-space` any session; `delete-space`/
/// `archive-space` owner or admin; everything else any AUTHOR of the named space or admin. `decide`
/// itself performs zero authorization (its own doc) — this is that check, run before `execute`.
async fn authorize_directory_command(state: &HubState, actor_user_id: &str, admin: bool, command: &DirectoryCommand) -> Result<(), StatusCode> {
    if admin {
        return Ok(());
    }
    match command {
        DirectoryCommand::CreateSpace { .. } => Ok(()),
        DirectoryCommand::DeleteSpace { space_id } | DirectoryCommand::ArchiveSpace { space_id } => {
            let space = state.directory.get_space(space_id).await.map_err(directory_error_status)?.ok_or(StatusCode::NOT_FOUND)?;
            if space.owner_user_id == actor_user_id {
                Ok(())
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => match state.directory.get_role(space_id, actor_user_id).await {
            Ok(Some(SpaceRole::Author)) => Ok(()),
            Ok(_) => Err(StatusCode::FORBIDDEN),
            Err(error) => Err(directory_error_status(error)),
        },
        DirectoryCommand::AnnounceDocument { descriptor } => match state.directory.get_role(&descriptor.space_id, actor_user_id).await {
            Ok(Some(SpaceRole::Author)) => Ok(()),
            Ok(_) => Err(StatusCode::FORBIDDEN),
            Err(error) => Err(directory_error_status(error)),
        },
    }
}

enum FencedDirectoryCommandErrorV1 {
    Directory(DirectoryError),
    Unavailable,
}

/// 🔗️ The live membership binding one command invalidates, held across its whole execution.
fn directory_command_socket_binding(command: &DirectoryCommand) -> Option<SocketBindingKeyV1> {
    match command {
        DirectoryCommand::RemoveMember { space_id, user_id } => Some(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() }),
        _ => None,
    }
}

#[cfg(test)]
async fn pause_directory_command_membership_fence(state: &HubState) {
    if let Some(test_gate) = &state.live_gate {
        if test_gate.socket_membership_remove_enabled.load(std::sync::atomic::Ordering::Acquire) {
            test_gate.socket_membership_remove_admitted.add_permits(1);
            let _ = test_gate.socket_membership_remove_release.acquire().await;
        }
    }
}

#[cfg(not(test))]
async fn pause_directory_command_membership_fence(_state: &HubState) {}

async fn execute_directory_command_fenced(state: &HubState, actor: DirectoryActor, command: DirectoryCommand) -> Result<(Vec<DirectoryEvent>, Option<CommandResult>), FencedDirectoryCommandErrorV1> {
    let Some(binding) = directory_command_socket_binding(&command) else {
        return state.directory_service.execute(actor, command).await.map_err(FencedDirectoryCommandErrorV1::Directory);
    };
    let gate = state.socket_binding_gates.gate(binding.clone());
    let _guard = tokio::time::timeout(std::time::Duration::from_secs(2), gate.lock_owned()).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?;
    pause_directory_command_membership_fence(state).await;
    let result = state.directory_service.execute(actor, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    state.socket_grants.invalidate_binding(binding);
    Ok(result)
}

/// 🆔️ The idempotent twin of `execute_directory_command_fenced`: the same live-membership fence,
/// around the durable claim-or-read command pipeline that owns the request receipt.
async fn execute_directory_command_receipt_fenced(state: &HubState, actor: DirectoryActor, claim: NewDirectoryCommandReceipt, command: DirectoryCommand) -> Result<DirectoryCommandExecutionV1, FencedDirectoryCommandErrorV1> {
    let Some(binding) = directory_command_socket_binding(&command) else {
        return state.directory_service.execute_idempotent(actor, claim, command).await.map_err(FencedDirectoryCommandErrorV1::Directory);
    };
    let gate = state.socket_binding_gates.gate(binding.clone());
    let _guard = tokio::time::timeout(std::time::Duration::from_secs(2), gate.lock_owned()).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?;
    pause_directory_command_membership_fence(state).await;
    let execution = state.directory_service.execute_idempotent(actor, claim, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    state.socket_grants.invalidate_binding(binding);
    Ok(execution)
}

/// 🧾️ `POST /directory/commands` — the closed request/receipt command wire. Authentication and the
/// full §C2 authorization matrix run BEFORE any stored completion is returned, so knowing a request
/// id never resurrects a result for an expired, revoked, or differently-scoped session. The one-shot
/// invite capability is returned to this live call alone; every later resolution of the same id is
/// redacted, proving no duplicate invitation was minted.
async fn post_directory_commands(
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
    body: Bytes,
) -> Result<(StatusCode, DirectoryJson<DirectoryCommandReceiptV1>), StatusCode> {
    if body.len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let request = std::str::from_utf8(&body).ok().and_then(|json| DirectoryCommandRequestV1::parse_canonical_json(json).ok()).ok_or(StatusCode::BAD_REQUEST)?;
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let admin = is_admin(&state, &headers, Some(peer)).await;
    authorize_directory_command(&state, &user.user_id, admin, &request.command).await?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let claim = NewDirectoryCommandReceipt {
        actor_user_id: user.user_id.clone(),
        request_id: request.request_id.clone(),
        command_sha256: directory_command_sha256(&request.command),
        result_kind: directory_command_result_kind(&request.command),
        claimed_at: now_ms(),
    };
    let execution = execute_directory_command_receipt_fenced(&state, actor, claim, request.command).await.map_err(|error| match error {
        FencedDirectoryCommandErrorV1::Directory(error) => directory_error_status(error),
        FencedDirectoryCommandErrorV1::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
    })?;
    let receipt = match execution {
        DirectoryCommandExecutionV1::Conflict => return Err(StatusCode::CONFLICT),
        DirectoryCommandExecutionV1::Receipt(receipt) => receipt,
    };
    if receipt.validate().is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok((StatusCode::ACCEPTED, DirectoryJson(receipt)))
}

async fn get_directory_spaces(headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectorySpaceListEntryV1>>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let mut views = Vec::new();
    for space in model.spaces.values() {
        let role = caller.as_ref().and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id).map(|member| member.role));
        match directory_space_access_decision(space.view.visibility == DirectorySpaceVisibility::Public, role) {
            DirectorySpaceAccessDecisionV1::Hidden => {}
            DirectorySpaceAccessDecisionV1::Public => {
                let documents = public_documents_for_space(&state, &space.view.id).await?;
                views.push(DirectorySpaceListEntryV1::Public { space: public_space_view(space, documents.len()) });
            }
            DirectorySpaceAccessDecisionV1::Member => {
                views.push(DirectorySpaceListEntryV1::Member { space: member_space_view(&state, space, DirectorySpaceRole::Spectator).await });
            }
            DirectorySpaceAccessDecisionV1::Author => {
                views.push(DirectorySpaceListEntryV1::Author { space: member_space_view(&state, space, DirectorySpaceRole::Author).await });
            }
        }
    }
    views.sort_by(|left, right| {
        let left = match left {
            DirectorySpaceListEntryV1::Public { space } => &space.id,
            DirectorySpaceListEntryV1::Member { space } | DirectorySpaceListEntryV1::Author { space } => &space.id,
        };
        let right = match right {
            DirectorySpaceListEntryV1::Public { space } => &space.id,
            DirectorySpaceListEntryV1::Member { space } | DirectorySpaceListEntryV1::Author { space } => &space.id,
        };
        left.cmp(right)
    });
    Ok(DirectoryJson(views))
}

async fn get_directory_space(Path(space_id): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectorySpaceDetailV1>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let space = model.spaces.get(&space_id).ok_or(StatusCode::NOT_FOUND)?;
    let role = caller.as_ref().and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id).map(|member| member.role));
    let detail = match directory_space_access_decision(space.view.visibility == DirectorySpaceVisibility::Public, role) {
        DirectorySpaceAccessDecisionV1::Hidden => return Err(StatusCode::NOT_FOUND),
        DirectorySpaceAccessDecisionV1::Public => {
            let documents = public_documents_for_space(&state, &space_id).await?;
            DirectorySpaceDetailV1::Public { space: public_space_view(space, documents.len()), documents }
        }
        DirectorySpaceAccessDecisionV1::Member => {
            DirectorySpaceDetailV1::Member { space: member_space_view(&state, space, DirectorySpaceRole::Spectator).await, members: space.members.clone(), documents: documents_for_space(&state, &space_id).await }
        }
        DirectorySpaceAccessDecisionV1::Author => DirectorySpaceDetailV1::Author {
            space: member_space_view(&state, space, DirectorySpaceRole::Author).await,
            members: space.members.clone(),
            documents: documents_for_space(&state, &space_id).await,
            invites: state.directory.list_invites(&space_id).await.map_err(directory_error_status)?.into_iter().map(invite_view).collect(),
        },
    };
    Ok(DirectoryJson(detail))
}

async fn post_redeem_invite(Path(token): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let capability = InviteCapability::parse(&token).map_err(directory_error_status)?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let events = state.directory_service.redeem_invite(actor, &capability, &user.user_id).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(events))
}

#[derive(Deserialize)]
struct EventsQuery {
    since: Option<u64>,
    limit: Option<usize>,
}

fn directory_event_page_request_admission(uri: &axum::http::Uri) -> Result<u64, StatusCode> {
    let query = uri.query().ok_or(StatusCode::BAD_REQUEST)?;
    if query.contains('&') || query.contains('%') || query.contains('+') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (name, value) = query.split_once('=').ok_or(StatusCode::BAD_REQUEST)?;
    if name != "after" || value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) || (value.len() > 1 && value.starts_with('0')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let after = value.parse::<u64>().map_err(|_| StatusCode::BAD_REQUEST)?;
    if after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(after)
}

fn directory_event_page_session_binding_v1(caller: &AuthedUser) -> Result<[u8; 32], StatusCode> {
    let session_len = u32::try_from(caller.session_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_len = u32::try_from(caller.user_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/directory-event-page/session-binding/v1\0");
    hash.update(&session_len.to_be_bytes());
    hash.update(caller.session_id.as_bytes());
    hash.update(&user_len.to_be_bytes());
    hash.update(caller.user_id.as_bytes());
    hash.update(&caller.authorization_generation.to_be_bytes());
    hash.update(&caller.expires_at.to_be_bytes());
    Ok(hash.finalize().into())
}

const DIRECTORY_EVENT_PAGE_DEADLINE_MS: u64 = 5_000;

struct DirectoryEventPageHttpControl {
    cancelled: std::sync::atomic::AtomicBool,
    active: std::sync::atomic::AtomicBool,
}

impl DirectoryEventPageHttpControl {
    fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true) }
    }

    fn checkpoint(&self) -> Result<(), StatusCode> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            Err(StatusCode::SERVICE_UNAVAILABLE)
        } else {
            Ok(())
        }
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }
}

struct DirectoryEventPageHttpRequest {
    control: Arc<DirectoryEventPageHttpControl>,
    response_owned: bool,
}

impl DirectoryEventPageHttpRequest {
    fn new(control: Arc<DirectoryEventPageHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    fn finish_response_owned(&mut self) {
        self.response_owned = true;
        self.control.release();
    }
}

impl Drop for DirectoryEventPageHttpRequest {
    fn drop(&mut self) {
        if !self.response_owned {
            self.control.cancel();
            self.control.release();
        }
    }
}

async fn revalidate_directory_event_page_caller(state: &HubState, caller: &AuthedUser, binding: [u8; 32]) -> Result<AuthedUser, StatusCode> {
    let session = state.directory.authenticate_session(&caller.capability).await.map_err(|_| StatusCode::UNAUTHORIZED)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let current = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability: caller.capability.clone() };
    if current.session_id != caller.session_id || current.user_id != caller.user_id || current.authorization_generation != caller.authorization_generation || directory_event_page_session_binding_v1(&current)? != binding {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(current)
}

async fn directory_event_page_event_visible(state: &HubState, event: &DirectoryEvent, caller: &AuthedUser) -> Result<bool, StatusCode> {
    let Some(space_id) = event.space_id.as_deref() else { return Ok(event.user_id.as_deref() == Some(caller.user_id.as_str())) };
    let Some(space) = state.directory.get_space(space_id).await.map_err(directory_error_status)? else { return Ok(false) };
    let role = state.directory.get_role(space_id, &caller.user_id).await.map_err(directory_error_status)?.map(role_wire);
    Ok(directory_space_access_decision(space.visibility == "public", role).is_member())
}

fn seal_directory_event_page_v1(binding: [u8; 32], generation: u64, after: u64, through: u64, has_more: bool, events: Vec<DirectoryEvent>) -> Result<DirectoryEventPageV1, DirectoryEventPageErrorV1> {
    let mut page = DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: os_directory::hex_lower(&binding),
        authorization_generation: generation,
        after_seq_exclusive: after,
        through_seq_inclusive: through,
        has_more,
        events,
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = os_directory::hex_lower(&Sha256::digest(page.canonical_unsigned_json().as_bytes()));
    if directory::os_pack::json::to_json_string(&page).len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
        return Err(DirectoryEventPageErrorV1::TooLarge);
    }
    page.validate()?;
    Ok(page)
}

async fn build_directory_event_page_v1(state: &HubState, caller: &AuthedUser, after: u64, control: &DirectoryEventPageHttpControl) -> Result<DirectoryEventPageV1, StatusCode> {
    control.checkpoint()?;
    let binding = directory_event_page_session_binding_v1(caller)?;
    let raw = state.directory.events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    control.checkpoint()?;
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        if gate.directory_event_page_fence_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.directory_event_page_read_admitted.add_permits(1);
            let _ = gate.directory_event_page_read_release.acquire().await;
        }
    }
    control.checkpoint()?;
    let caller = revalidate_directory_event_page_caller(state, caller, binding).await?;
    control.checkpoint()?;
    let raw_len = raw.len();
    let mut through = after;
    let mut events = Vec::new();
    let mut stopped_for_bytes = false;
    for event in raw {
        control.checkpoint()?;
        if event.seq <= through || validate_directory_event_page_event(&event).is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        let visible = directory_event_page_event_visible(state, &event, &caller).await?;
        control.checkpoint()?;
        if !visible {
            match seal_directory_event_page_v1(binding, caller.authorization_generation, after, event.seq, true, events.clone()) {
                Ok(_) => through = event.seq,
                Err(DirectoryEventPageErrorV1::TooLarge) => {
                    stopped_for_bytes = true;
                    break;
                }
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
            continue;
        }
        let mut candidate = events.clone();
        candidate.push(event.clone());
        match seal_directory_event_page_v1(binding, caller.authorization_generation, after, event.seq, true, candidate) {
            Ok(_) => {
                events.push(event);
                through = events.last().map_or(through, |event| event.seq);
            }
            Err(DirectoryEventPageErrorV1::TooLarge) => {
                stopped_for_bytes = true;
                break;
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    seal_directory_event_page_v1(binding, caller.authorization_generation, after, through, stopped_for_bytes || raw_len == DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, events).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_directory_event_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    let after = directory_event_page_request_admission(&uri)?;
    let control = Arc::new(DirectoryEventPageHttpControl::new());
    let mut request = DirectoryEventPageHttpRequest::new(control.clone());
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        if gate.directory_event_page_fence_enabled.load(std::sync::atomic::Ordering::Acquire) {
            *gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(control.clone());
        }
    }
    let operation = async {
        let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
        build_directory_event_page_v1(&state, &caller, after, control.as_ref()).await.map(DirectoryJson)
    };
    let response = match tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_EVENT_PAGE_DEADLINE_MS), operation).await {
        Ok(result) => result,
        Err(_) => {
            control.cancel();
            Err(StatusCode::GATEWAY_TIMEOUT)
        }
    };
    request.finish_response_owned();
    response
}

/// @emoji 🪪️ Revalidates the exact browser session behind a long-lived directory stream. Revocation and
/// expiry therefore take effect on the next outbound frame instead of leaving a previously opened
/// socket privileged indefinitely.
async fn caller_active(state: &HubState, caller: &AuthedUser) -> bool {
    if caller.expires_at <= now_ms() {
        return false;
    }
    matches!(
        state.directory.authenticate_session(&caller.capability).await,
        Ok(Some(session)) if session.id == caller.session_id && session.user_id == caller.user_id && session.authorization_generation == caller.authorization_generation
    )
}

async fn directory_space_access_for_user(state: &HubState, space_id: &str, user_id: Option<&str>) -> DirectorySpaceAccessDecisionV1 {
    let Ok(Some(space)) = state.directory.get_space(space_id).await else { return DirectorySpaceAccessDecisionV1::Hidden };
    let role = match user_id {
        Some(user_id) => state.directory.get_role(space_id, user_id).await.ok().flatten().map(role_wire),
        None => None,
    };
    directory_space_access_decision(space.visibility == "public", role)
}

/// @emoji 👁️ Raw durable events are member-only. Public discovery has no event stream, so
/// no redaction of actor, HLC, sequence, or identity-bearing event bodies can be forgotten.
async fn event_visible(state: &HubState, event: &DirectoryEvent, caller: Option<&AuthedUser>) -> bool {
    let Some(space_id) = &event.space_id else {
        return match (caller, event.user_id.as_deref()) {
            (Some(caller), Some(user_id)) if caller.user_id == user_id => true,
            _ => false,
        };
    };
    directory_space_access_for_user(state, space_id, caller.map(|caller| caller.user_id.as_str())).await.is_member()
}

/// @emoji 🛡️ The single privacy boundary for every directory WebSocket frame. Realtime connection
/// and presence telemetry requires current membership even for public spaces; public visibility
/// exposes directory metadata, not who is online or their account email.
async fn directory_message_visible(state: &HubState, message: &DirectoryStreamMessage, caller: Option<&AuthedUser>) -> bool {
    let Some(caller) = caller else { return false };
    if !caller_active(state, caller).await {
        return false;
    }
    match message {
        DirectoryStreamMessage::Event { event } => event_visible(state, event, Some(caller)).await,
        DirectoryStreamMessage::Connection { connection, .. } => directory_space_access_for_user(state, &connection.space_id, Some(&caller.user_id)).await.is_member(),
        DirectoryStreamMessage::Presence { space_id, .. } => directory_space_access_for_user(state, space_id, Some(&caller.user_id)).await.is_member(),
        DirectoryStreamMessage::Heartbeat { .. } => false,
        DirectoryStreamMessage::RebootstrapRequired { control } => directory_space_access_for_user(state, &control.scope.space_id, Some(&caller.user_id)).await.is_member(),
    }
}

async fn visibility_filter_events(state: &HubState, events: Vec<DirectoryEvent>, caller: Option<&AuthedUser>) -> Vec<DirectoryEvent> {
    let mut visible = Vec::with_capacity(events.len());
    let mut access_by_space = BTreeMap::new();
    for event in events {
        let allowed = match event.space_id.as_deref() {
            None => caller.is_some_and(|caller| event.user_id.as_deref() == Some(caller.user_id.as_str())),
            Some(space_id) => {
                let access = match access_by_space.get(space_id) {
                    Some(access) => *access,
                    None => {
                        let access = directory_space_access_for_user(state, space_id, caller.map(|caller| caller.user_id.as_str())).await;
                        access_by_space.insert(space_id.to_string(), access);
                        access
                    }
                };
                access.is_member()
            }
        };
        if allowed {
            visible.push(event);
        }
    }
    visible
}

async fn get_directory_events(axum::extract::Query(query): axum::extract::Query<EventsQuery>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(visibility_filter_events(&state, events, caller.as_ref()).await))
}

async fn socket_directory_message_visible(state: &HubState, record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> SocketBindingValidityV1 {
    let validity = record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()).await;
    if validity != SocketBindingValidityV1::Active {
        return validity;
    }
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return SocketBindingValidityV1::Unauthorized };
    let visible = match message {
        DirectoryStreamMessage::Event { event } => match event.space_id.as_deref() {
            Some(space_id) => directory_space_access_for_user(state, space_id, Some(user_id)).await.is_member(),
            None => event.user_id.as_deref() == Some(user_id.as_str()),
        },
        DirectoryStreamMessage::Connection { connection, .. } => directory_space_access_for_user(state, &connection.space_id, Some(user_id)).await.is_member(),
        DirectoryStreamMessage::Presence { space_id, .. } => directory_space_access_for_user(state, space_id, Some(user_id)).await.is_member(),
        DirectoryStreamMessage::Heartbeat { .. } => false,
        DirectoryStreamMessage::RebootstrapRequired { control } => directory_space_access_for_user(state, &control.scope.space_id, Some(user_id)).await.is_member(),
    };
    if visible {
        SocketBindingValidityV1::Active
    } else {
        SocketBindingValidityV1::Unauthorized
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScopedDirectoryFrameDecisionV1 {
    Deliver,
    SkipUnrelated,
    CloseUnauthorized,
    CloseUnavailable,
}

fn directory_message_matches_scope(scope: &DocumentScope, message: &DirectoryStreamMessage) -> bool {
    match message {
        DirectoryStreamMessage::Event { event } => match &event.body {
            os_directory::DirectoryEventBody::DocumentAnnounced { descriptor } => descriptor.space_id == scope.space_id && descriptor.document_id == scope.document_id,
            os_directory::DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => checkpoint.scope == *scope,
            os_directory::DirectoryEventBody::ArtifactRetentionAdvanced { retention } => retention.scope == *scope,
            os_directory::DirectoryEventBody::UserCreated { .. }
            | os_directory::DirectoryEventBody::SpaceCreated { .. }
            | os_directory::DirectoryEventBody::SpaceRenamed { .. }
            | os_directory::DirectoryEventBody::SpaceVisibilityChanged { .. }
            | os_directory::DirectoryEventBody::SpaceArchived { .. }
            | os_directory::DirectoryEventBody::SpaceDeleted { .. }
            | os_directory::DirectoryEventBody::MemberUpserted { .. }
            | os_directory::DirectoryEventBody::MemberRemoved { .. }
            | os_directory::DirectoryEventBody::InviteRedeemed { .. } => false,
        },
        DirectoryStreamMessage::Connection { connection, .. } => connection.space_id == scope.space_id && connection.document_id == scope.document_id,
        DirectoryStreamMessage::Presence { space_id, document_id, .. } => space_id == &scope.space_id && document_id == &scope.document_id,
        DirectoryStreamMessage::Heartbeat { .. } => false,
        DirectoryStreamMessage::RebootstrapRequired { control } => control.scope == *scope,
    }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DirectoryWsV1Query {
    #[serde(default)]
    since: u64,
    space_id: Option<String>,
    document_id: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DirectoryScopedWsV1Query {
    #[serde(default)]
    since: u64,
}

async fn directory_ws_v1(ws: WebSocketUpgrade, axum::extract::Query(query): axum::extract::Query<DirectoryWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let scope = match (query.space_id, query.document_id) {
        (Some(space_id), Some(document_id)) if socket_text_bounded(&space_id) && socket_text_bounded(&document_id) => Some(DocumentScope::new(space_id, document_id)),
        (None, None) => None,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    let admission = match consume_directory_socket_grant(&state, &headers).await {
        Ok(admission) => admission,
        Err(status) => return (status, "socket grant rejected").into_response(),
    };
    ws.protocols([SOCKET_PROTOCOL_V1]).on_upgrade(move |socket| handle_directory_ws_v1(socket, query.since, scope, state, admission)).into_response()
}

async fn directory_scoped_ws_v1(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, axum::extract::Query(query): axum::extract::Query<DirectoryScopedWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let admission = match consume_socket_grant(&state, &headers, SocketAudienceV1::DirectoryScoped(scope.clone()), None).await {
        Ok(admission) => admission,
        Err(status) => return (status, "socket grant rejected").into_response(),
    };
    ws.protocols([SOCKET_PROTOCOL_V1]).on_upgrade(move |socket| handle_directory_ws_v1(socket, query.since, Some(scope), state, admission)).into_response()
}

async fn send_directory_message(sender: &mut SplitSink<WebSocket, Message>, message: &DirectoryStreamMessage) -> bool {
    let text = directory::os_pack::json::to_json_string(message);
    sender.send(Message::Text(text.into())).await.is_ok()
}

async fn send_socket_directory_message(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, message: &DirectoryStreamMessage) -> ScopedDirectoryFrameDecisionV1 {
    #[cfg(test)]
    if matches!(&record.audience, SocketAudienceV1::DirectoryScoped(_)) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_scoped_send_mode.load(std::sync::atomic::Ordering::Acquire) == 1 {
                gate.socket_scoped_send_admitted.add_permits(1);
                let _ = gate.socket_scoped_send_release.acquire().await;
            }
        }
    }
    let _admission = match socket_live_authority(state, record, live_id).await {
        Ok(admission) => admission,
        Err(SocketBindingValidityV1::Unauthorized) => return ScopedDirectoryFrameDecisionV1::CloseUnauthorized,
        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => return ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    };
    #[cfg(test)]
    if matches!(&record.audience, SocketAudienceV1::DirectoryScoped(_)) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_scoped_send_mode.load(std::sync::atomic::Ordering::Acquire) == 2 {
                gate.socket_scoped_send_admitted.add_permits(1);
                let _ = gate.socket_scoped_send_release.acquire().await;
            }
        }
    }
    let decision = match &record.audience {
        SocketAudienceV1::DirectoryScoped(scope) => {
            if directory_message_matches_scope(scope, message) {
                ScopedDirectoryFrameDecisionV1::Deliver
            } else {
                ScopedDirectoryFrameDecisionV1::SkipUnrelated
            }
        }
        SocketAudienceV1::Directory { .. } => match tokio::time::timeout(std::time::Duration::from_secs(2), socket_directory_message_visible(state, record, message)).await.unwrap_or(SocketBindingValidityV1::Unavailable) {
            SocketBindingValidityV1::Active => ScopedDirectoryFrameDecisionV1::Deliver,
            SocketBindingValidityV1::Unauthorized => ScopedDirectoryFrameDecisionV1::SkipUnrelated,
            SocketBindingValidityV1::Unavailable => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
        },
        SocketAudienceV1::Document(_) => ScopedDirectoryFrameDecisionV1::CloseUnauthorized,
    };
    if decision != ScopedDirectoryFrameDecisionV1::Deliver {
        return decision;
    }
    match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, message)).await {
        Ok(true) => ScopedDirectoryFrameDecisionV1::Deliver,
        _ => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    }
}

async fn send_socket_directory_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, scope: &DocumentScope) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority(state, record, live_id).await {
        Ok(admission) => admission,
        Err(validity) => return validity,
    };
    if !matches!(&record.audience, SocketAudienceV1::DirectoryScoped(audience_scope) if audience_scope == scope) && !matches!(&record.audience, SocketAudienceV1::Directory { .. }) {
        return SocketBindingValidityV1::Unauthorized;
    }
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return SocketBindingValidityV1::Unauthorized };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_role(&scope.space_id, user_id)).await {
        Ok(Ok(Some(_))) => {}
        Ok(Ok(None)) => return SocketBindingValidityV1::Unauthorized,
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    }
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_rebootstrap_read.add_permits(1);
    }
    let control = match tokio::time::timeout(std::time::Duration::from_secs(2), verified_rebootstrap_control(state, scope)).await {
        Ok(control) => control,
        Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    match control {
        Some(control) => match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, &DirectoryStreamMessage::RebootstrapRequired { control })).await {
            Ok(true) => SocketBindingValidityV1::Active,
            _ => SocketBindingValidityV1::Unavailable,
        },
        None => SocketBindingValidityV1::Active,
    }
}

async fn handle_directory_ws_v1(socket: WebSocket, since: u64, scope: Option<DocumentScope>, state: HubState, admission: SocketGrantAdmissionV1) {
    let (mut sender, mut receiver) = socket.split();
    let SocketGrantAdmissionV1 { record } = admission;
    let hello = match tokio::time::timeout(std::time::Duration::from_secs(2), receiver.next()).await {
        Ok(Some(Ok(Message::Binary(bytes)))) => decode_client_frame(&bytes).await.ok().map(|(_, frame)| frame),
        _ => None,
    };
    let Some(ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema, resume_token, .. }) = hello else {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
        return;
    };
    if !socket_text_bounded(&schema) || resume_token.as_ref().is_some_and(|value| value.len() > AUTH_TEXT_MAX_BYTES) {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
        return;
    }
    let binding_gates = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&record.subject, &record.audience)).await {
        Ok(admission) => admission,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            return;
        }
    };
    let (live_id, notify) = match state.socket_grants.register_live(&record) {
        Ok(live) => live,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
    };
    let live_lease = SocketLiveLeaseV1 { ledger: state.socket_grants.clone(), record: record.clone(), id: live_id, notify };
    let validity = socket_binding_validity(&state, &record.subject, &record.audience).await;
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
        SocketBindingValidityV1::Unavailable => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            return;
        }
    }
    drop(binding_gates);
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_directory_admitted.add_permits(1);
        let _ = gate.socket_directory_release.acquire().await;
    }
    let mut live = state.directory_service.subscribe();
    let replay = match load_all_directory_events(state.directory.as_ref(), since).await {
        Ok(events) => events,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-replay-unavailable".into() }))).await;
            return;
        }
    };
    let mut last_replayed = since;
    for event in replay {
        let seq = event.seq;
        let message = DirectoryStreamMessage::Event { event };
        match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, &message).await {
            ScopedDirectoryFrameDecisionV1::Deliver => {
                last_replayed = last_replayed.max(seq);
            }
            ScopedDirectoryFrameDecisionV1::SkipUnrelated => {}
            ScopedDirectoryFrameDecisionV1::CloseUnauthorized => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                return;
            }
            ScopedDirectoryFrameDecisionV1::CloseUnavailable => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                return;
            }
        }
    }
    let mut authorization_tick = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = authorization_tick.tick() => match socket_live_authority(&state, &record, &live_lease.id).await {
                Ok(_) => {}
                Err(SocketBindingValidityV1::Unauthorized) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                    break;
                }
                Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                    break;
                }
            },
            incoming = receiver.next() => match incoming {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Ping(payload))) => if sender.send(Message::Pong(payload)).await.is_err() { break },
                Some(Ok(_)) => {}
                Some(Err(_)) => break,
            },
            message = live.recv() => match message {
                Ok(message) => {
                    let seq = match &message { DirectoryStreamMessage::Event { event } => Some(event.seq), _ => None };
                    if seq.is_some_and(|seq| seq <= last_replayed) {
                        continue;
                    }
                    match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, &message).await {
                        ScopedDirectoryFrameDecisionV1::Deliver => {
                            if let Some(seq) = seq { last_replayed = last_replayed.max(seq); }
                        }
                        ScopedDirectoryFrameDecisionV1::SkipUnrelated => {}
                        ScopedDirectoryFrameDecisionV1::CloseUnauthorized => {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        }
                        ScopedDirectoryFrameDecisionV1::CloseUnavailable => {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                            break;
                        }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    let validity = match scope.as_ref() {
                        Some(scope) => send_socket_directory_rebootstrap(&mut sender, &state, &record, &live_lease.id, scope).await,
                        None => match socket_live_authority(&state, &record, &live_lease.id).await {
                            Ok(_) => SocketBindingValidityV1::Active,
                            Err(validity) => validity,
                        },
                    };
                    match validity {
                        SocketBindingValidityV1::Active => {
                            let _ = tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "rebootstrap-required".into() }))),
                            )
                            .await;
                        }
                        SocketBindingValidityV1::Unauthorized => { let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await; }
                        SocketBindingValidityV1::Unavailable => { let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await; }
                    }
                    break;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            },
            _ = live_lease.notify.notified() => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                break;
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionMeResponse {
    user_id: String,
    email: String,
    display_name: String,
    expires_at: i64,
    session_kind: AuthSessionKind,
    authorization_generation: u64,
}

async fn get_session_me(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SessionMeResponse>, StatusCode> {
    let capability = SessionCapability::parse(&bearer(&headers).ok_or(StatusCode::UNAUTHORIZED)?).map_err(directory_error_status)?;
    let session = state.directory.authenticate_session(&capability).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let user = state.directory.get_user(&session.user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Json(SessionMeResponse { user_id: user.id, email: user.email, display_name: user.display_name, expires_at: session.expires_at, session_kind: session.session_kind, authorization_generation: session.authorization_generation }))
}

async fn delete_session_me(headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    let Some(token) = bearer(&headers) else { return StatusCode::UNAUTHORIZED };
    let Ok(capability) = SessionCapability::parse(&token) else { return StatusCode::UNAUTHORIZED };
    let Ok(Ok(Some(session))) = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability)).await else {
        return StatusCode::UNAUTHORIZED;
    };
    let binding = SocketBindingKeyV1::Session(session.id.clone());
    let gate = state.socket_binding_gates.gate(binding.clone());
    let _admission = match tokio::time::timeout(std::time::Duration::from_secs(2), gate.lock_owned()).await {
        Ok(admission) => admission,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE,
    };
    match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.revoke_auth_session(&session.id, "self-revoked", Some(&session.user_id), &directory::os_identity::time_ordered_id())).await {
        Ok(Ok(Some(revoked))) => {
            debug_assert_eq!(revoked.id, session.id);
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            StatusCode::NO_CONTENT
        }
        Ok(Ok(None)) => StatusCode::UNAUTHORIZED,
        Ok(Err(_)) => StatusCode::INTERNAL_SERVER_ERROR,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

/// @emoji 🌐️ Applies the hub's explicit cross-origin response policy. Authentication issuance
/// is absent from the public router; protected routes still require their typed capability.
async fn cors_middleware(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response {
    let origin = request.headers().get(axum::http::header::ORIGIN).cloned();
    if request.method() == axum::http::Method::OPTIONS {
        let mut response = axum::response::Response::builder().status(StatusCode::NO_CONTENT).body(axum::body::Body::empty()).unwrap_or_default();
        apply_cors_headers(response.headers_mut(), origin.as_ref());
        return response;
    }
    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref());
    response
}

/// @emoji 🌐️ See {@link cors_middleware}. Reflects the request's own `Origin` (never `*`) plus the
/// bearer/JSON headers and verbs this control plane actually uses.
fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&axum::http::HeaderValue>) {
    if let Some(origin) = origin {
        headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, axum::http::HeaderValue::from_static("true"));
        headers.append(axum::http::header::VARY, axum::http::HeaderValue::from_static("Origin"));
    }
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_METHODS, axum::http::HeaderValue::from_static("GET, POST, PUT, HEAD, DELETE, OPTIONS"));
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, axum::http::HeaderValue::from_static("authorization, content-type"));
}
//#endregion 🔖️Directory

//#region 🔖️Admin
/// @emoji 🧮️ Best-effort recursive directory size in bytes — used only for the admin overview's
/// `dataDirBytes`; any unreadable entry is silently skipped rather than failing the whole overview.
fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total += if metadata.is_dir() { dir_size(&entry.path()) } else { metadata.len() };
            }
        }
    }
    total
}

#[derive(Clone)]
struct AdminIntentMetadata {
    intent_kind: &'static str,
    target_kind: &'static str,
    target_id: String,
    reason_code: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminPageQuery {
    cursor: Option<String>,
    limit: Option<usize>,
}

fn admin_cursor_mac(key: &[u8; 32], principal: &AdminPrincipalV1, payload: &[u8; 10], scope: Option<&str>) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-page-cursor/v1\0");
    hash.update(key);
    hash.update(&(principal.auth_session_id.len() as u32).to_be_bytes());
    hash.update(principal.auth_session_id.as_bytes());
    hash.update(&principal.authorization_generation.to_be_bytes());
    hash.update(payload);
    hash.update(&[u8::from(scope.is_some())]);
    if let Some(scope) = scope {
        hash.update(&(scope.len() as u32).to_be_bytes());
        hash.update(scope.as_bytes());
    }
    hash.finalize()
}

fn admin_cursor_decode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, encoded: Option<&str>) -> Result<usize, StatusCode> {
    let Some(encoded) = encoded else {
        return Ok(0);
    };
    if encoded.len() != 84 || !encoded.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut raw = [0u8; 42];
    for (index, byte) in raw.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&encoded[index * 2..index * 2 + 2], 16).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    if raw[0] != 1 || raw[1] != route {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut payload = [0u8; 10];
    payload.copy_from_slice(&raw[..10]);
    let mut supplied = [0u8; 32];
    supplied.copy_from_slice(&raw[10..]);
    if !semio_hub::directory::constant_time_digest_eq(&admin_cursor_mac(key, principal, &payload, scope), &supplied) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let offset = u64::from_be_bytes(payload[2..].try_into().map_err(|_| StatusCode::BAD_REQUEST)?);
    usize::try_from(offset).map_err(|_| StatusCode::BAD_REQUEST)
}

fn admin_cursor_decode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, encoded: Option<&str>) -> Result<usize, StatusCode> {
    admin_cursor_decode_scoped(key, principal, route, None, encoded)
}

fn admin_cursor_encode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, offset: usize) -> Result<String, StatusCode> {
    let mut payload = [0u8; 10];
    payload[0] = 1;
    payload[1] = route;
    payload[2..].copy_from_slice(&u64::try_from(offset).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_be_bytes());
    let mut raw = [0u8; 42];
    raw[..10].copy_from_slice(&payload);
    raw[10..].copy_from_slice(&admin_cursor_mac(key, principal, &payload, scope));
    Ok(os_directory::hex_lower(&raw))
}

fn admin_cursor_encode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, offset: usize) -> Result<String, StatusCode> {
    admin_cursor_encode_scoped(key, principal, route, None, offset)
}

fn admin_page_limit(query: &AdminPageQuery) -> Result<usize, StatusCode> {
    match query.limit.unwrap_or(ADMIN_PAGE_MAX) {
        1..=ADMIN_PAGE_MAX => Ok(query.limit.unwrap_or(ADMIN_PAGE_MAX)),
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

fn admin_fit_page<T, F>(mut rows: Vec<T>, storage_has_more: bool, observed_at_ms: i64, cursor: F) -> Result<AdminPageV1<T>, StatusCode>
where
    T: Clone + ToValue,
    F: Fn(&[T]) -> Result<String, StatusCode>,
{
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| cursor(&rows)).transpose()?;
        let candidate = AdminPageV1 { rows: rows.clone(), next_cursor, observed_at_ms };
        if directory::os_pack::json::to_json_string(&candidate).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(AdminPageV1 { rows, next_cursor: candidate.next_cursor, observed_at_ms });
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

fn admin_fit_connection_snapshot(
    mut rows: Vec<AdminRecordedConnectionV1>,
    storage_has_more: bool,
    observed_at_ms: i64,
    head_seq: u64,
    cursor_key: &[u8; 32],
    principal: &AdminPrincipalV1,
    offset: usize,
) -> Result<AdminConnectionSnapshotV1, StatusCode> {
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| admin_cursor_encode(cursor_key, principal, 1, offset + rows.len())).transpose()?;
        let candidate = AdminConnectionSnapshotV1 { rows: rows.clone(), next_cursor, observed_at_ms, source: "recorded-sync-sessions".into(), head_seq };
        if directory::os_pack::json::to_json_string(&candidate).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(candidate);
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

fn admin_directory_command(intent: &AdminIntentV1) -> Option<DirectoryCommand> {
    Some(match intent {
        AdminIntentV1::CreateSpace { .. } => return None,
        AdminIntentV1::RenameSpace { space_id, name, .. } => DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: name.clone() },
        AdminIntentV1::SetSpaceVisibility { space_id, visibility, .. } => DirectoryCommand::SetVisibility { space_id: space_id.clone(), visibility: *visibility },
        AdminIntentV1::ArchiveSpace { space_id, .. } => DirectoryCommand::ArchiveSpace { space_id: space_id.clone() },
        AdminIntentV1::DeleteSpace { space_id, .. } => DirectoryCommand::DeleteSpace { space_id: space_id.clone() },
        AdminIntentV1::UpsertSpaceMember { space_id, email, role, .. } => DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: email.clone(), role: *role },
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => DirectoryCommand::RemoveMember { space_id: space_id.clone(), user_id: user_id.clone() },
        AdminIntentV1::CreateSpaceInvite { space_id, role, ttl_secs, .. } => DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: *role, ttl_secs: u64::from(*ttl_secs) },
        AdminIntentV1::RevokeSpaceInvite { space_id, invite_id, .. } => DirectoryCommand::RevokeInvite { space_id: space_id.clone(), invite_id: invite_id.clone() },
        _ => return None,
    })
}

fn admin_create_space_id(request_id: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-create-space/v1\0");
    hash.update(&(request_id.len() as u32).to_be_bytes());
    hash.update(request_id.as_bytes());
    format!("admin-space:{}", os_directory::hex_lower(&hash.finalize()))
}

fn admin_intent_metadata(intent: &AdminIntentV1) -> AdminIntentMetadata {
    match intent {
        AdminIntentV1::CreateSpace { request_id, .. } => AdminIntentMetadata { intent_kind: "create-space", target_kind: "space", target_id: admin_create_space_id(request_id), reason_code: None },
        AdminIntentV1::RenameSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "rename-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::SetSpaceVisibility { space_id, .. } => AdminIntentMetadata { intent_kind: "set-space-visibility", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::ArchiveSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "archive-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::DeleteSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::UpsertSpaceMember { space_id, .. } => AdminIntentMetadata { intent_kind: "upsert-space-member", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::RemoveSpaceMember { space_id, .. } => AdminIntentMetadata { intent_kind: "remove-space-member", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::CreateSpaceInvite { space_id, .. } => AdminIntentMetadata { intent_kind: "create-space-invite", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::RevokeSpaceInvite { space_id, .. } => AdminIntentMetadata { intent_kind: "revoke-space-invite", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::IssueDocumentShare { scope, .. } => AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None },
        AdminIntentV1::RevokeDocumentShare { share_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "revoke-document-share", target_kind: "share", target_id: share_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::RevokeUserSessions { user_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "revoke-user-sessions", target_kind: "user", target_id: user_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::KickConnection { sync_session_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "kick-connection", target_kind: "sync-session", target_id: sync_session_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::RebuildDirectoryProjections { .. } => AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None },
    }
}

fn validate_admin_intent(intent: &AdminIntentV1) -> Result<(), StatusCode> {
    semio_hub::directory::validate_bounded_auth_text(intent.request_id(), "admin request id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let metadata = admin_intent_metadata(intent);
    semio_hub::directory::validate_bounded_auth_text(&metadata.target_id, "admin target", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    if let Some(reason) = metadata.reason_code.as_deref() {
        semio_hub::directory::validate_bounded_auth_text(reason, "admin reason", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    match intent {
        AdminIntentV1::CreateSpace { name, .. } => {
            semio_hub::directory::validate_bounded_auth_text(name, "admin space name", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RenameSpace { space_id, name, .. } => {
            for value in [space_id, name] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin space field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::SetSpaceVisibility { space_id, .. } | AdminIntentV1::ArchiveSpace { space_id, .. } | AdminIntentV1::DeleteSpace { space_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(space_id, "admin space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::UpsertSpaceMember { space_id, email, .. } => {
            for value in [space_id, email] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => {
            for value in [space_id, user_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::CreateSpaceInvite { space_id, ttl_secs, .. } => {
            semio_hub::directory::validate_bounded_auth_text(space_id, "admin invite space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeSpaceInvite { space_id, invite_id, .. } => {
            for value in [space_id, invite_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin invite field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            semio_hub::directory::validate_bounded_auth_text(&scope.space_id, "admin scope space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            semio_hub::directory::validate_bounded_auth_text(&scope.document_id, "admin scope document", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, .. } => {
            for value in [&scope.space_id, &scope.document_id, share_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin share scope", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RevokeUserSessions { user_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(user_id, "admin user", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(sync_session_id, "admin sync session", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RebuildDirectoryProjections { .. } => {}
    }
    Ok(())
}

fn admin_intent_digest(intent: &AdminIntentV1) -> String {
    let encoded = directory::os_pack::json::to_json_string(intent);
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-intent/v1\0");
    hash.update(&(encoded.len() as u32).to_be_bytes());
    hash.update(encoded.as_bytes());
    os_directory::hex_lower(&hash.finalize())
}

fn new_admin_audit_fact(principal: &AdminPrincipalV1, request_id: &str, intent_digest: &str, operation_id: &str, metadata: &AdminIntentMetadata, phase: &str, event_range: Option<(u64, u64)>, outcome_code: &str) -> NewAdminOperationAuditRecord {
    NewAdminOperationAuditRecord {
        request_id: request_id.into(),
        intent_digest: intent_digest.into(),
        operation_id: operation_id.into(),
        occurred_at: now_ms(),
        phase: phase.into(),
        intent_kind: metadata.intent_kind.into(),
        target_kind: metadata.target_kind.into(),
        target_id: metadata.target_id.clone(),
        principal_user_id: principal.user_id.clone(),
        principal_session_id: principal.auth_session_id.clone(),
        principal_generation: principal.authorization_generation,
        correlation_id: principal.correlation_id.clone(),
        event_seq_first: event_range.map(|range| range.0),
        event_seq_last: event_range.map(|range| range.1),
        outcome_code: outcome_code.into(),
        reason_code: metadata.reason_code.clone(),
    }
}

fn admin_audit_receipt(rows: &[AdminOperationAuditRecord]) -> Option<AdminIntentReceiptV1> {
    let row = rows.iter().find(|row| row.fact.phase != "accepted").or_else(|| rows.first())?;
    let state = match row.fact.phase.as_str() {
        "succeeded" => AdminIntentStateV1::Succeeded,
        "failed" => AdminIntentStateV1::Failed,
        "cancelled" => AdminIntentStateV1::Cancelled,
        _ => AdminIntentStateV1::Accepted,
    };
    Some(AdminIntentReceiptV1 {
        operation_id: row.fact.operation_id.clone(),
        correlation_id: row.fact.correlation_id.clone(),
        state,
        event_seq_first: row.fact.event_seq_first,
        event_seq_last: row.fact.event_seq_last,
        result: None,
        outcome: AdminIntentOutcomeV1 { code: row.fact.outcome_code.clone(), durable: row.fact.intent_kind != "kick-connection" && row.fact.phase == "succeeded", kick_attempted: None, kick_signalled: None },
    })
}

fn public_admin_audit(row: AdminOperationAuditRecord) -> Result<AdminOperationAuditV1, StatusCode> {
    let phase = match row.fact.phase.as_str() {
        "accepted" => AdminOperationAuditPhaseV1::Accepted,
        "succeeded" => AdminOperationAuditPhaseV1::Succeeded,
        "failed" => AdminOperationAuditPhaseV1::Failed,
        "cancelled" => AdminOperationAuditPhaseV1::Cancelled,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(AdminOperationAuditV1 {
        sequence: row.sequence,
        operation_id: row.fact.operation_id,
        occurred_at_ms: row.fact.occurred_at,
        phase,
        intent_kind: row.fact.intent_kind,
        target_kind: row.fact.target_kind,
        target_id: row.fact.target_id,
        principal_user_id: row.fact.principal_user_id,
        principal_session_id: row.fact.principal_session_id,
        principal_generation: row.fact.principal_generation,
        correlation_id: row.fact.correlation_id,
        event_seq_first: row.fact.event_seq_first,
        event_seq_last: row.fact.event_seq_last,
        outcome_code: row.fact.outcome_code,
        reason_code: row.fact.reason_code,
    })
}

async fn reconcile_stale_admin_acceptance(state: &HubState, rows: Vec<AdminOperationAuditRecord>) -> Result<Vec<AdminOperationAuditRecord>, StatusCode> {
    let Some(accepted) = rows.first() else {
        return Ok(rows);
    };
    if rows.iter().any(|row| row.fact.phase != "accepted") || state.admin_operations.with(&accepted.fact.operation_id, |runtime| runtime.is_some()) || now_ms().saturating_sub(accepted.fact.occurred_at) <= 15_000 {
        return Ok(rows);
    }
    let terminal = NewAdminOperationAuditRecord {
        request_id: accepted.fact.request_id.clone(),
        intent_digest: accepted.fact.intent_digest.clone(),
        operation_id: accepted.fact.operation_id.clone(),
        occurred_at: now_ms(),
        phase: "cancelled".into(),
        intent_kind: accepted.fact.intent_kind.clone(),
        target_kind: accepted.fact.target_kind.clone(),
        target_id: accepted.fact.target_id.clone(),
        principal_user_id: accepted.fact.principal_user_id.clone(),
        principal_session_id: accepted.fact.principal_session_id.clone(),
        principal_generation: accepted.fact.principal_generation,
        correlation_id: accepted.fact.correlation_id.clone(),
        event_seq_first: None,
        event_seq_last: None,
        outcome_code: "interrupted-before-terminal".into(),
        reason_code: accepted.fact.reason_code.clone(),
    };
    state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
    state.directory.admin_operation_audit_for_request(&accepted.fact.request_id).await.map_err(directory_error_status)
}

async fn admin_operation(Path(operation_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let rows = reconcile_stale_admin_acceptance(&state, rows).await?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let progress = state.admin_operations.with(&receipt.operation_id, |runtime| runtime.map(|runtime| runtime.progress()));
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress }))
}

async fn cancel_admin_operation(
    Path(operation_id): Path<String>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let runtime = state.admin_operations.get_cloned(&receipt.operation_id).ok_or(StatusCode::CONFLICT)?;
    runtime.cancel_requested.store(true, std::sync::atomic::Ordering::Release);
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress: Some(runtime.progress()) }))
}

async fn admin_operation_audit(
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<AdminOperationAuditV1>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let after = admin_cursor_decode(&state.admin_cursor_key, &principal, 4, query.cursor.as_deref())?;
    let mut rows = state.directory.list_admin_operation_audit(u64::try_from(after).map_err(|_| StatusCode::BAD_REQUEST)?, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = rows.len() > limit;
    rows.truncate(limit);
    let rows = rows.into_iter().map(public_admin_audit).collect::<Result<Vec<_>, _>>()?;
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| {
        let next = rows.last().map(|row| usize::try_from(row.sequence).map_err(|_| StatusCode::BAD_REQUEST)).transpose()?.unwrap_or(after);
        admin_cursor_encode(&state.admin_cursor_key, &principal, 4, next)
    })?;
    Ok(DirectoryJson(page))
}

struct AdminIntentExecution {
    phase: &'static str,
    event_range: Option<(u64, u64)>,
    result: Option<AdminIntentResultV1>,
    outcome: AdminIntentOutcomeV1,
}

struct AdminOperationRuntime {
    deadline: std::time::Instant,
    completed: std::sync::atomic::AtomicU64,
    total: std::sync::atomic::AtomicU64,
    cancel_requested: std::sync::atomic::AtomicBool,
}

impl AdminOperationRuntime {
    fn progress(&self) -> AdminOperationProgressV1 {
        AdminOperationProgressV1 {
            completed_events: self.completed.load(std::sync::atomic::Ordering::Acquire),
            total_events: self.total.load(std::sync::atomic::Ordering::Acquire),
            cancel_requested: self.cancel_requested.load(std::sync::atomic::Ordering::Acquire),
        }
    }
}

impl ProjectionRebuildControl for AdminOperationRuntime {
    fn is_cancelled(&self) -> bool {
        self.cancel_requested.load(std::sync::atomic::Ordering::Acquire) || std::time::Instant::now() >= self.deadline
    }

    fn report(&self, progress: ProjectionRebuildProgress) {
        self.completed.store(progress.completed_events, std::sync::atomic::Ordering::Release);
        self.total.store(progress.total_events, std::sync::atomic::Ordering::Release);
    }
}

struct AdminOperationCleanup {
    directory: Arc<HubDirectories>,
    operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    operation_id: String,
    terminal: Option<NewAdminOperationAuditRecord>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl AdminOperationCleanup {
    fn terminal(&self) -> &NewAdminOperationAuditRecord {
        self.terminal.as_ref().expect("admin operation cleanup terminal is armed")
    }

    fn disarm(&mut self) {
        self.terminal = None;
    }
}

impl Drop for AdminOperationCleanup {
    fn drop(&mut self) {
        self.operations.remove(&self.operation_id);
        let Some(terminal) = self.terminal.take() else {
            return;
        };
        let directory = self.directory.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                let _ = directory.append_admin_operation_audit(&terminal).await;
            });
        }
    }
}

async fn execute_admin_intent(state: &HubState, principal: &AdminPrincipalV1, intent: AdminIntentV1, operation_runtime: Option<Arc<AdminOperationRuntime>>) -> AdminIntentExecution {
    if let AdminIntentV1::CreateSpace { request_id, name, space_kind, visibility } = &intent {
        let target_id = admin_create_space_id(request_id);
        return match state.directory_service.execute_create_space_with_id(principal.event_actor(), target_id, name.clone(), *space_kind, *visibility).await {
            Ok(events) => AdminIntentExecution {
                phase: "succeeded",
                event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                result: None,
                outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
            },
            Err(_) => AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-command-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
        };
    }
    if let Some(command) = admin_directory_command(&intent) {
        if authorize_directory_command(state, &principal.user_id, true, &command).await.is_err() {
            return AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-command-denied".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        }
        return match execute_directory_command_fenced(state, principal.event_actor(), command).await {
            Ok((events, result)) => AdminIntentExecution {
                phase: "succeeded",
                event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                result: result.and_then(|result| result.invite_token).map(|invite_token| AdminIntentResultV1 { invite_token: Some(invite_token), share_token: None }),
                outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
            },
            Err(_) => AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-command-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
        };
    }
    match intent {
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            if !matches!(state.directory.get_document_descriptor(&scope).await, Ok(Some(_))) {
                return AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "document-unavailable".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            }
            match state.directory.issue_share_token_as(&scope, i64::from(ttl_secs), Some(&principal.user_id), &principal.correlation_id).await {
                Ok(issued) => AdminIntentExecution {
                    phase: "succeeded",
                    event_range: None,
                    result: Some(AdminIntentResultV1 { invite_token: None, share_token: Some(issued.capability.expose_once()) }),
                    outcome: AdminIntentOutcomeV1 { code: "share-issued".into(), durable: true, kick_attempted: None, kick_signalled: None },
                },
                Err(_) => AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "share-issue-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, reason_code, .. } => {
            let binding = SocketBindingKeyV1::Share(share_id.clone());
            let gate = state.socket_binding_gates.gate(binding.clone());
            let Ok(admission) = tokio::time::timeout(std::time::Duration::from_secs(2), gate.lock_owned()).await else {
                return AdminIntentExecution { phase: "cancelled", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "share-revoke-timeout".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            };
            let revoked = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.revoke_share_token_as(&scope, &share_id, &reason_code, Some(&principal.user_id), &principal.correlation_id)).await;
            if !matches!(revoked, Ok(Ok(()))) {
                return AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "share-revoke-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            }
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            drop(admission);
            AdminIntentExecution { phase: "succeeded", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "share-revoked".into(), durable: true, kick_attempted: None, kick_signalled: None } }
        }
        AdminIntentV1::RevokeUserSessions { user_id, reason_code, .. } => {
            let user_gate = state.socket_binding_gates.gate(SocketBindingKeyV1::User(user_id.clone()));
            let Ok(admission) = tokio::time::timeout(std::time::Duration::from_secs(2), user_gate.lock_owned()).await else {
                return AdminIntentExecution { phase: "cancelled", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "session-revoke-timeout".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            };
            let revoked = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.revoke_auth_sessions_for_user(&user_id, &reason_code, Some(&principal.user_id), &principal.correlation_id)).await;
            let Ok(Ok(revoked)) = revoked else {
                return AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "session-revoke-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            };
            for session in &revoked {
                let binding = SocketBindingKeyV1::Session(session.id.clone());
                state.socket_grants.invalidate_binding(binding.clone());
                state.document_open_plans.invalidate_binding(&binding);
            }
            drop(admission);
            let revoked_ids: BTreeSet<&str> = revoked.iter().map(|session| session.id.as_str()).collect();
            let sessions = state.directory.list_active_sync_sessions(None, ACTIVE_SYNC_SESSION_READ_MAX).await.unwrap_or_default();
            let mut attempted = 0u32;
            let mut signalled = 0u32;
            for session in sessions.iter().filter(|session| session.auth_session_id.as_deref().is_some_and(|id| revoked_ids.contains(id))) {
                attempted = attempted.saturating_add(1);
                if let Some(notify) = state.session_kicks.get_cloned(&session.id) {
                    notify.notify_one();
                    signalled = signalled.saturating_add(1);
                }
            }
            AdminIntentExecution { phase: "succeeded", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "sessions-revoked".into(), durable: true, kick_attempted: Some(attempted), kick_signalled: Some(signalled) } }
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => match state.session_kicks.get_cloned(&sync_session_id) {
            Some(notify) => {
                notify.notify_one();
                AdminIntentExecution { phase: "succeeded", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "connection-kick-signalled".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(1) } }
            }
            None => AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "connection-not-live".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(0) } },
        },
        AdminIntentV1::RebuildDirectoryProjections { expected_head_seq, .. } => {
            if !matches!(state.directory.head_seq().await, Ok(head) if head == expected_head_seq) {
                return AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-head-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            }
            let control = operation_runtime.unwrap_or_else(|| {
                Arc::new(AdminOperationRuntime {
                    deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
                    completed: std::sync::atomic::AtomicU64::new(0),
                    total: std::sync::atomic::AtomicU64::new(0),
                    cancel_requested: std::sync::atomic::AtomicBool::new(false),
                })
            });
            match state.directory.rebuild_projections_controlled(control.as_ref()).await {
                Ok(_) => AdminIntentExecution { phase: "succeeded", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-projections-rebuilt".into(), durable: true, kick_attempted: None, kick_signalled: None } },
                Err(_) if control.is_cancelled() => {
                    AdminIntentExecution { phase: "cancelled", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-cancelled".into(), durable: false, kick_attempted: None, kick_signalled: None } }
                }
                Err(_) => AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
            }
        }
        _ => unreachable!("closed admin directory intents were handled before dispatch"),
    }
}

async fn admin_intents(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, body: Bytes) -> Result<(StatusCode, DirectoryJson<AdminIntentReceiptV1>), StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    if body.is_empty() || body.len() > ADMIN_INTENT_REQUEST_MAX_BYTES || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| StatusCode::BAD_REQUEST)?;
    let intent: AdminIntentV1 = directory::os_pack::json::from_json_str(encoded).map_err(|_| StatusCode::BAD_REQUEST)?;
    validate_admin_intent(&intent)?;
    let metadata = admin_intent_metadata(&intent);
    let digest = admin_intent_digest(&intent);
    let request_id = intent.request_id().to_string();
    let prior = state.directory.admin_operation_audit_for_request(&request_id).await.map_err(directory_error_status)?;
    let prior = reconcile_stale_admin_acceptance(&state, prior).await?;
    if let Some(receipt) = admin_audit_receipt(&prior) {
        let same = prior
            .first()
            .is_some_and(|row| row.fact.intent_digest == digest && row.fact.principal_user_id == principal.user_id && row.fact.principal_session_id == principal.auth_session_id && row.fact.principal_generation == principal.authorization_generation);
        return if same { Ok((StatusCode::OK, DirectoryJson(receipt))) } else { Err(StatusCode::CONFLICT) };
    }
    let proposed_operation_id = directory::os_identity::time_ordered_id();
    let accepted = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, "accepted", None, "accepted");
    let established = state.directory.append_admin_operation_audit(&accepted).await.map_err(directory_error_status)?;
    if established.fact.operation_id != proposed_operation_id {
        let joined = state.directory.admin_operation_audit_for_request(&request_id).await.map_err(directory_error_status)?;
        return admin_audit_receipt(&joined).map(|receipt| (StatusCode::OK, DirectoryJson(receipt))).ok_or(StatusCode::CONFLICT);
    }
    let still_authorized = authenticate_admin_principal(&state, &headers, Some(peer)).await.ok().is_some_and(|fresh| principal.same_authority(&fresh));
    if !still_authorized {
        let execution = AdminIntentExecution { phase: "cancelled", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "admin-authority-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
        state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
        return Ok((
            StatusCode::OK,
            DirectoryJson(AdminIntentReceiptV1 {
                operation_id: proposed_operation_id,
                correlation_id: principal.correlation_id,
                state: AdminIntentStateV1::Cancelled,
                event_seq_first: None,
                event_seq_last: None,
                result: None,
                outcome: execution.outcome,
            }),
        ));
    }
    if matches!(&intent, AdminIntentV1::RebuildDirectoryProjections { .. }) {
        let operation_permit = match state.admin_operation_slots.clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                let execution = AdminIntentExecution { phase: "failed", event_range: None, result: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-capacity".into(), durable: false, kick_attempted: None, kick_signalled: None } };
                let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
                state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
                return Ok((
                    StatusCode::OK,
                    DirectoryJson(AdminIntentReceiptV1 {
                        operation_id: proposed_operation_id,
                        correlation_id: principal.correlation_id,
                        state: AdminIntentStateV1::Failed,
                        event_seq_first: None,
                        event_seq_last: None,
                        result: None,
                        outcome: execution.outcome,
                    }),
                ));
            }
        };
        let runtime = Arc::new(AdminOperationRuntime {
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            cancel_requested: std::sync::atomic::AtomicBool::new(false),
        });
        state.admin_operations.insert(proposed_operation_id.clone(), runtime.clone());
        let task_state = state.clone();
        let task_principal = principal.clone();
        let task_request_id = request_id.clone();
        let task_digest = digest.clone();
        let task_operation_id = proposed_operation_id.clone();
        let task_metadata = metadata.clone();
        tokio::spawn(async move {
            let interrupted = new_admin_audit_fact(&task_principal, &task_request_id, &task_digest, &task_operation_id, &task_metadata, "cancelled", None, "interrupted-before-terminal");
            let mut cleanup = AdminOperationCleanup { directory: task_state.directory.clone(), operations: task_state.admin_operations.clone(), operation_id: task_operation_id.clone(), terminal: Some(interrupted), _permit: operation_permit };
            let execution = execute_admin_intent(&task_state, &task_principal, intent, Some(runtime)).await;
            let terminal = new_admin_audit_fact(&task_principal, &task_request_id, &task_digest, &task_operation_id, &task_metadata, execution.phase, execution.event_range, &execution.outcome.code);
            cleanup.terminal = Some(terminal);
            if task_state.directory.append_admin_operation_audit(cleanup.terminal()).await.is_ok() {
                cleanup.disarm();
            }
        });
        return Ok((
            StatusCode::ACCEPTED,
            DirectoryJson(AdminIntentReceiptV1 {
                operation_id: proposed_operation_id,
                correlation_id: principal.correlation_id,
                state: AdminIntentStateV1::Accepted,
                event_seq_first: None,
                event_seq_last: None,
                result: None,
                outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-running".into(), durable: false, kick_attempted: None, kick_signalled: None },
            }),
        ));
    }
    let execution = execute_admin_intent(&state, &principal, intent, None).await;
    let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, execution.event_range, &execution.outcome.code);
    state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
    Ok((
        StatusCode::OK,
        DirectoryJson(AdminIntentReceiptV1 {
            operation_id: proposed_operation_id,
            correlation_id: principal.correlation_id,
            state: match execution.phase {
                "succeeded" => AdminIntentStateV1::Succeeded,
                "cancelled" => AdminIntentStateV1::Cancelled,
                _ => AdminIntentStateV1::Failed,
            },
            event_seq_first: execution.event_range.map(|range| range.0),
            event_seq_last: execution.event_range.map(|range| range.1),
            result: execution.result,
            outcome: execution.outcome,
        }),
    ))
}

async fn admin_overview(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let _principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let counts = state.directory.admin_overview_counts().await.map_err(directory_error_status)?;
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    // 🌵️ `extensions_root` is `{data_dir}/extension-modules` (see `main`'s own construction) — its
    // parent is `data_dir` itself, the nearest thing `HubState` carries to `OS_HUB_DATA`'s root.
    let data_dir_bytes = state.extensions_root.parent().map(dir_size).unwrap_or(0);
    let response = serde_json::json!({
        "counts": { "spaces": counts.spaces, "users": counts.users, "connections": counts.connections },
        "backends": { "sqlite": cfg!(feature = "sqlite"), "postgres": cfg!(feature = "postgres"), "neo4j": cfg!(feature = "neo4j") },
        "dataDirBytes": data_dir_bytes,
        "headSeq": head_seq,
        "openArtifacts": state.db.catalog().await.artifacts.len(),
    });
    if serde_json::to_vec(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.len() > ADMIN_RESPONSE_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(Json(response))
}

fn admin_space_summary_view(summary: semio_hub::directory::model::AdminSpaceSummaryRecord) -> Result<SpaceView, StatusCode> {
    let kind = match summary.space.kind.as_str() {
        "atelier" => os_directory::DirectorySpaceKind::Atelier,
        "studio" => os_directory::DirectorySpaceKind::Studio,
        "archive" => os_directory::DirectorySpaceKind::Archive,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let visibility = match summary.space.visibility.as_str() {
        "private" => DirectorySpaceVisibility::Private,
        "public" => DirectorySpaceVisibility::Public,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(SpaceView {
        id: summary.space.id,
        name: summary.space.name,
        kind,
        visibility,
        owner_user_id: summary.space.owner_user_id,
        role: None,
        member_count: u32::try_from(summary.member_count).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        document_count: u32::try_from(summary.document_count).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        active_connections: u32::try_from(summary.active_connections).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        created_at_ms: summary.space.created_at,
        updated_at_ms: summary.updated_at,
    })
}

async fn admin_spaces(
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<SpaceView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 6, query.cursor.as_deref())?;
    let mut summaries = state.directory.list_admin_space_summaries_page(None, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = summaries.len() > limit;
    summaries.truncate(limit);
    let mut rows = Vec::with_capacity(summaries.len());
    for summary in summaries {
        rows.push(admin_space_summary_view(summary)?);
    }
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode(&state.admin_cursor_key, &principal, 6, offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

struct AdminSpaceDetailResponse {
    view: SpaceView,
    members: AdminPageV1<MemberView>,
}

impl ToValue for AdminSpaceDetailResponse {
    fn to_value(&self) -> DslValue {
        let mut entries = match self.view.to_value() {
            DslValue::Object(entries) => entries,
            other => vec![("space".into(), other)],
        };
        entries.push(("members".into(), self.members.to_value()));
        DslValue::Object(entries)
    }
}

fn admin_fit_space_detail(view: SpaceView, mut rows: Vec<MemberView>, storage_has_more: bool, observed_at_ms: i64, cursor_key: &[u8; 32], principal: &AdminPrincipalV1, space_id: &str, offset: usize) -> Result<AdminSpaceDetailResponse, StatusCode> {
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| admin_cursor_encode_scoped(cursor_key, principal, 7, Some(space_id), offset + rows.len())).transpose()?;
        let response = AdminSpaceDetailResponse { view: view.clone(), members: AdminPageV1 { rows: rows.clone(), next_cursor, observed_at_ms } };
        if directory::os_pack::json::to_json_string(&response).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(response);
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

async fn admin_space(
    Path(space_id): Path<String>,
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminSpaceDetailResponse>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&space_id, "admin detail space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode_scoped(&state.admin_cursor_key, &principal, 7, Some(&space_id), query.cursor.as_deref())?;
    let summary = state.directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.map_err(directory_error_status)?.pop().ok_or(StatusCode::NOT_FOUND)?;
    let view = admin_space_summary_view(summary)?;
    let mut records = state.directory.list_admin_space_members_page(&space_id, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = records.len() > limit;
    records.truncate(limit);
    let rows: Vec<MemberView> = records.into_iter().map(|(user, role)| MemberView { user_id: user.id, email: user.email, display_name: user.display_name, role: role_wire(role) }).collect();
    Ok(DirectoryJson(admin_fit_space_detail(view, rows, storage_has_more, now_ms(), &state.admin_cursor_key, &principal, &space_id, offset)?))
}

async fn admin_users(
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<os_directory::UserView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 2, query.cursor.as_deref())?;
    let mut users = state.directory.list_users(i64::try_from(limit.saturating_add(1)).map_err(|_| StatusCode::BAD_REQUEST)?, i64::try_from(offset).map_err(|_| StatusCode::BAD_REQUEST)?).await.map_err(directory_error_status)?;
    let storage_has_more = users.len() > limit;
    users.truncate(limit);
    let rows = users.into_iter().map(|user| os_directory::UserView { id: user.id, email: user.email, display_name: user.display_name, created_at_ms: user.created_at }).collect();
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode(&state.admin_cursor_key, &principal, 2, offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

async fn admin_connections(
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminConnectionSnapshotV1>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 1, query.cursor.as_deref())?;
    let mut sessions = state.directory.list_active_sync_sessions_page(None, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = sessions.len() > limit;
    sessions.truncate(limit);
    let rows = sessions
        .into_iter()
        .map(|session| AdminRecordedConnectionV1 {
            sync_session_id: session.id,
            scope: DocumentScope::new(&session.space_id, &session.document_id),
            authenticated_user_id: session.user_id,
            email: session.authenticated_email,
            role: session.space_role.map(|role| match role {
                SpaceRole::Author => DirectorySpaceRole::Author,
                SpaceRole::Spectator => DirectorySpaceRole::Spectator,
            }),
            connected_at_ms: session.connected_at,
            source: "recorded-sync-session".into(),
        })
        .collect();
    let observed_at_ms = now_ms();
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    Ok(DirectoryJson(admin_fit_connection_snapshot(rows, storage_has_more, observed_at_ms, head_seq, &state.admin_cursor_key, &principal, offset)?))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DocumentsQuery {
    space: Option<String>,
    cursor: Option<String>,
    limit: Option<usize>,
}

async fn admin_documents(
    axum::extract::Query(query): axum::extract::Query<DocumentsQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<DocumentView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let page = AdminPageQuery { cursor: query.cursor, limit: query.limit };
    let limit = admin_page_limit(&page)?;
    if let Some(space_id) = query.space.as_deref() {
        semio_hub::directory::validate_bounded_auth_text(space_id, "admin document space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    let offset = admin_cursor_decode_scoped(&state.admin_cursor_key, &principal, 5, query.space.as_deref(), page.cursor.as_deref())?;
    let mut descriptors = state.directory.list_document_descriptors_page(query.space.as_deref(), offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = descriptors.len() > limit;
    descriptors.truncate(limit);
    let mut rows = Vec::with_capacity(descriptors.len());
    for descriptor in descriptors {
        rows.push(document_view(&state, descriptor).await);
    }
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode_scoped(&state.admin_cursor_key, &principal, 5, query.space.as_deref(), offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

async fn admin_events(
    axum::extract::Query(query): axum::extract::Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<DirectoryEvent>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let since = admin_cursor_decode(&state.admin_cursor_key, &principal, 3, query.cursor.as_deref())?;
    let mut events = state.directory.events_since(u64::try_from(since).map_err(|_| StatusCode::BAD_REQUEST)?, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = events.len() > limit;
    events.truncate(limit);
    let page = admin_fit_page(events, storage_has_more, now_ms(), |rows| {
        let next = rows.last().map(|event| usize::try_from(event.seq).map_err(|_| StatusCode::BAD_REQUEST)).transpose()?.unwrap_or(since);
        admin_cursor_encode(&state.admin_cursor_key, &principal, 3, next)
    })?;
    Ok(DirectoryJson(page))
}

//#endregion 🔖️Admin

//#region 🔖️Extensions
/// @emoji 🧩️ Hub mirror of dev `staticDirVitePlugin` `/🧩️extension-modules` — lists installed extension metadata.
#[derive(Serialize)]
struct ExtensionListResponse {
    extensions: Vec<serde_json::Value>,
}

fn extension_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("js") | Some("mjs") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

fn extension_asset_path(root: &std::path::Path, extension_id: &str, rest: &str) -> Option<std::path::PathBuf> {
    if extension_id.is_empty() || extension_id.contains('/') || extension_id.contains('\\') || extension_id.contains("..") {
        return None;
    }
    if rest.is_empty() || rest.contains("..") {
        return None;
    }
    let base = root.join(extension_id);
    let path = base.join(rest);
    if !path.starts_with(&base) {
        return None;
    }
    Some(path)
}

async fn list_extensions(State(state): State<HubState>) -> Result<Json<ExtensionListResponse>, StatusCode> {
    let mut extensions = Vec::new();
    let read_dir = tokio::fs::read_dir(&state.extensions_root).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut entries = read_dir;
    while let Ok(Some(entry)) = entries.next_entry().await {
        if !entry.file_type().await.map(|kind| kind.is_dir()).unwrap_or(false) {
            continue;
        }
        let meta_path = entry.path().join("install.json");
        let bytes = match tokio::fs::read(&meta_path).await {
            Ok(value) => value,
            Err(_) => continue,
        };
        let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        extensions.push(value);
    }
    extensions.sort_by(|left, right| left.get("extensionId").and_then(|value| value.as_str()).unwrap_or_default().cmp(right.get("extensionId").and_then(|value| value.as_str()).unwrap_or_default()));
    Ok(Json(ExtensionListResponse { extensions }))
}

async fn get_extension_asset(Path((extension_id, rest)): Path<(String, String)>, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    let path = extension_asset_path(&state.extensions_root, &extension_id, &rest).ok_or(StatusCode::BAD_REQUEST)?;
    let bytes = tokio::fs::read(&path).await.map_err(|error| if error.kind() == std::io::ErrorKind::NotFound { StatusCode::NOT_FOUND } else { StatusCode::INTERNAL_SERVER_ERROR })?;
    let content_type = extension_asset_content_type(&path);
    Ok(([(axum::http::header::CONTENT_TYPE, content_type)], bytes))
}
//#endregion 🔖️Extensions

//#region 🔖️AdminPage
/// @emoji 🛡️ Static SPA serving for `/admin` (contract §C2) — reads from `HubState.admin_dir`
/// (lane 1-B's field, `OS_HUB_ADMIN_DIR` else the compile-time default pointing at lane 2-E's own
/// `bun nx run os-hub-admin:build` output), mirroring `🔖️Extensions`'s `extension_asset_path`/
/// `get_extension_asset` pair exactly: plain `tokio::fs::read`, no `tower-http`, a traversal guard
/// before ever joining onto `root`, then a second `starts_with` check on the joined path as
/// defense-in-depth. SPA fallback: any requested path whose exact file is missing (a client-side
/// route, e.g. `/admin/spaces/sp-1`) falls back to `🌐️.html`; a genuinely missing admin build
/// (nobody ran the vite build yet) is a 503 with a build hint, never a confusing 404 loop.
fn admin_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript",
        Some("css") => "text/css",
        Some("svg") => "image/svg+xml",
        Some("woff2") => "font/woff2",
        Some("json") => "application/json",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

/// @emoji 🚧️ Rejects `..`, a backslash (a Windows separator smuggled into a URL path segment), and
/// strips every leading `/` before joining onto `root` — `PathBuf::join` treats an absolute second
/// argument as a full replacement of the base, which would otherwise let `rest = "/etc/passwd"` read
/// clean outside `root` entirely (the one way this guard differs from `extension_asset_path`, which
/// never sees a leading-slash `rest` in the first place).
fn admin_asset_path(root: &std::path::Path, rest: &str) -> Option<std::path::PathBuf> {
    if rest.contains("..") || rest.contains('\\') {
        return None;
    }
    let path = root.join(rest.trim_start_matches('/'));
    if !path.starts_with(root) {
        return None;
    }
    Some(path)
}

async fn admin_page(state: &HubState, rest: &str) -> axum::response::Response {
    let root = &state.admin_dir;
    if !root.is_dir() {
        return (StatusCode::SERVICE_UNAVAILABLE, "admin SPA not built — run: bun nx run os-hub-admin:build").into_response();
    }
    let Some(requested) = admin_asset_path(root, rest) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let path = if requested.is_file() { requested } else { root.join("🌐️.html") };
    match tokio::fs::read(&path).await {
        Ok(bytes) => ([(axum::http::header::CONTENT_TYPE, admin_asset_content_type(&path))], bytes).into_response(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_admin_root(State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, "🌐️.html").await
}

async fn get_admin_asset(Path(rest): Path<String>, State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, &rest).await
}

async fn get_readyz(State(state): State<HubState>) -> impl IntoResponse {
    let mut readiness = (*state.readiness).clone();
    readiness.artifact_cas_sweeper.ready = state.artifact_maintenance.healthy();
    if !readiness.artifact_cas_sweeper.ready {
        readiness.status = "not-ready";
    }
    let status = if readiness.status == "ready" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(readiness))
}
//#endregion 🔖️AdminPage

//#region 🔖️Main
fn router(state: HubState) -> Router {
    Router::new()
        .route("/readyz", get(get_readyz))
        .route("/auth/sessions/me", get(get_session_me).delete(delete_session_me))
        .route("/directory/commands", post(post_directory_commands).layer(DefaultBodyLimit::max(DIRECTORY_COMMAND_REQUEST_MAX_BYTES)))
        .route("/directory/spaces", get(get_directory_spaces))
        .route("/directory/spaces/{id}", get(get_directory_space))
        .route("/directory/invites/{token}/redeem", post(post_redeem_invite))
        .route("/directory/events", get(get_directory_events))
        .route("/directory/event-page/v1", get(get_directory_event_page_v1))
        .route("/directory/socket-grants", post(issue_directory_socket_grant))
        .route("/directory/socket/v1", get(directory_ws_v1))
        .route("/directory/spaces/{space_id}/documents/{document_id}/socket-grants", post(issue_scoped_directory_socket_grant).layer(DefaultBodyLimit::max(256)))
        .route("/directory/spaces/{space_id}/documents/{document_id}/socket/v1", get(directory_scoped_ws_v1))
        .route("/admin/api/overview", get(admin_overview))
        .route("/admin/api/spaces", get(admin_spaces))
        .route("/admin/api/spaces/{id}", get(admin_space))
        .route("/admin/api/users", get(admin_users))
        .route("/admin/api/connections", get(admin_connections))
        .route("/admin/api/documents", get(admin_documents))
        .route("/admin/api/events", get(admin_events))
        .route("/admin/api/operations/{operation_id}", get(admin_operation))
        .route("/admin/api/operations/{operation_id}/cancel", post(cancel_admin_operation))
        .route("/admin/api/audit", get(admin_operation_audit))
        .route("/admin/api/intents", post(admin_intents).layer(DefaultBodyLimit::max(ADMIN_INTENT_REQUEST_MAX_BYTES)))
        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules", get(list_extensions))
        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/admin", get(get_admin_root))
        .route("/admin/", get(get_admin_root))
        .route("/admin/{*path}", get(get_admin_asset))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair", get(get_active_checkpoint_pair))
        .route("/spaces/{space_id}/documents/{id}/open-plan", post(issue_document_open_plan))
        .route("/spaces/{space_id}/documents/{id}/socket-grants", post(issue_document_plan_socket_grant))
        .route("/spaces/{space_id}/documents/{id}/socket/v1", get(document_ws_v1))
        // 🐙️ w4-h: router-wide CORS grant — see `cors_middleware`'s doc comment (`🔖️Directory` region)
        // for why this must cover the whole router, not just `/directory/*`.
        .layer(axum::middleware::from_fn(cors_middleware))
        .with_state(state)
}

/// @emoji 🧬️ Resolves and connects `db::Database`'s storage substrate, selected by
/// `OS_HUB_STORAGE_BACKEND` (`fs` default, zero-touch, rooted at `{data_dir}/db`; `sqlite`,
/// `postgres` — requires `OS_HUB_DATABASE_URL` — or `neo4j` — requires `OS_HUB_NEO4J_URI` —
/// otherwise, each match arm compiled only when this crate's same-named feature enables `db`'s own
/// matching storage feature). Independent of `connect_directory`'s own backend choice (the
/// contract's "storage swappability" requirement applies to `db`'s substrate and the directory's
/// substrate separately, even though both now share the same three feature names).
/// @emoji 🧵️ Hub's ONE process-wide `WorkerPool` — Phase 1
/// (`26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`) replaces `HubDbRuntime` (a `HostAsyncRuntime` bridge
/// whose entire reason to exist was `run_blocking`, now deleted from that trait — see
/// `db_storage`'s module doc) with this: `db::storage_sqlite::SqliteStorage::open`'s blocking body
/// now dispatches onto `Lane::Io` here directly, and every `db::Database` receives the same pool
/// during construction so `ArtifactHandle::submit` shares it — no more per-submit
/// `"db-engine-submit-bridge"` OS thread, no more sqlite-storage
/// calls stalling their caller's own tokio worker thread. Hub is a headless server
/// (`ProcessKind::HeadlessBatch`: no UI thread to reserve a core for), sized to the process's visible
/// core count.
fn hub_worker_pool() -> Arc<db::semio_framework_async::WorkerPool> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let config = db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, cores);
    Arc::new(db::semio_framework_async::process_worker_pool(config))
}

async fn connect_db(data_dir: &std::path::Path) -> Result<db::Database, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let profile = db::Profile::Prod;
    let pool = hub_worker_pool();
    match backend.as_str() {
        "fs" | "" => {
            let root = data_dir.join("db");
            Ok(db::Database::open_at(pool, &root, profile).await?)
        }
        #[cfg(feature = "sqlite")]
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            let storage = db::storage_sqlite::SqliteStorage::open(pool.clone(), std::path::Path::new(&path)).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Sqlite(storage))).await?)
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            let storage = db::storage_postgres::PostgresStorage::connect(pool.clone(), &database_url).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Postgres(storage))).await?)
        }
        #[cfg(feature = "neo4j")]
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            let storage = db::storage_neo4j::Neo4jStorage::connect(pool.clone(), &uri, &user, &password).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Neo4j(storage))).await?)
        }
        other => Err(HubError::UnknownStorageBackend(other.to_string())),
    }
}

/// @emoji 🧩️ Opens the hub-owned artifact CAS in a namespace independent from generic DB payloads.
async fn connect_artifact_cas(data_dir: &std::path::Path) -> Result<Arc<ArtifactChunkCasStores>, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let storage = match backend.as_str() {
        "fs" | "" => ArtifactChunkCasStores::Filesystem(FsArtifactChunkCasStorage::open(&data_dir.join("artifact-cas/v1")).await?),
        #[cfg(feature = "sqlite")]
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            ArtifactChunkCasStores::Sqlite(SqliteArtifactChunkCasStorage::open(std::path::Path::new(&path)).await?)
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            ArtifactChunkCasStores::Postgres(PostgresArtifactChunkCasStorage::connect(&database_url).await?)
        }
        #[cfg(feature = "neo4j")]
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            ArtifactChunkCasStores::Neo4j(Neo4jArtifactChunkCasStorage::connect(&uri, &user, &password).await?)
        }
        other => return Err(HubError::UnknownStorageBackend(other.to_string())),
    };
    Ok(Arc::new(storage))
}

/// @emoji 🧬️ Resolves and connects the identity/tenancy directory backend, selected by
/// `OS_HUB_DIRECTORY_BACKEND` (`sqlite` default, zero-touch, `{data_dir}/directory.db`; `postgres`
/// — requires `OS_HUB_DIRECTORY_DATABASE_URL` — or `neo4j` — requires
/// `OS_HUB_DIRECTORY_NEO4J_URI` — otherwise, each match arm compiled only when this crate's
/// same-named feature is enabled).
// 🌵️ `data_dir` is only read by the `sqlite` arm below (`postgres`/`neo4j` connect via env-provided
// URIs/connection strings instead, never a local path) — whenever the `sqlite` feature is off it
// goes genuinely unused (whether or not another backend feature is on), so the allow is scoped to
// exactly that condition rather than blanket-silencing the lint for every feature set.
#[cfg_attr(not(feature = "sqlite"), allow(unused_variables))]
async fn connect_directory(data_dir: &std::path::Path) -> Result<Arc<HubDirectories>, HubError> {
    let backend = std::env::var("OS_HUB_DIRECTORY_BACKEND").unwrap_or_else(|_| "sqlite".into());
    match backend.as_str() {
        #[cfg(feature = "sqlite")]
        "sqlite" | "" => {
            let path = data_dir.join("directory.db");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let directory = SqliteDirectory::connect(&path.to_string_lossy()).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DIRECTORY_DATABASE_URL").map_err(|_| HubError::UnknownDirectoryBackend("postgres requires OS_HUB_DIRECTORY_DATABASE_URL".into()))?;
            let directory = semio_hub::directory::postgres::PostgresDirectory::connect(&database_url).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        #[cfg(feature = "neo4j")]
        "neo4j" => {
            let uri = std::env::var("OS_HUB_DIRECTORY_NEO4J_URI").map_err(|_| HubError::UnknownDirectoryBackend("neo4j requires OS_HUB_DIRECTORY_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_DIRECTORY_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_DIRECTORY_NEO4J_PASSWORD").unwrap_or_default();
            let directory = semio_hub::directory::neo4j::Neo4jDirectory::connect(&uri, &user, &password).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        other => Err(HubError::UnknownDirectoryBackend(other.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), HubError> {
    let port: u16 = std::env::var("OS_HUB_PORT").ok().and_then(|value| value.parse().ok()).unwrap_or(8787);
    let bind: std::net::IpAddr = std::env::var("OS_HUB_BIND").unwrap_or_else(|_| "0.0.0.0".into()).parse().map_err(|_| HubError::UnsafeAuthConfiguration("OS_HUB_BIND must be an IP address".into()))?;
    let mode = HubMode::from_environment(bind)?;
    let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;
    let bootstrap_control = Arc::new(HubBootstrapControl::new());
    let local_bootstrap: Option<Arc<dyn LocalBootstrapTransport>> = if mode == HubMode::Development {
        let deadline_ms = now_ms().checked_add(LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS).ok_or_else(|| HubError::UnsafeAuthConfiguration("local bootstrap deadline overflow".into()))?;
        let context = semio_hub::directory::IdentityVerificationContext { deadline_ms, control: bootstrap_control.as_ref() };
        Some(InheritedLocalBootstrapTransport::open_inherited(&context).await?)
    } else {
        None
    };
    let admin_subjects = configured_admin_subjects()?;
    validate_auth_startup(mode, bind, identity_verifier.as_ref(), local_bootstrap.as_ref(), &admin_subjects)?;
    let data_dir = std::env::var("OS_HUB_DATA").map_or_else(|_| std::path::PathBuf::from("./.🧬semio/🌐hub/"), std::path::PathBuf::from);
    let native_codec_providers = NativeCodecProviderSetV1::linked();
    let artifact_authority = configured_artifact_authority(
        std::env::var("OS_HUB_TRUSTED_CATALOG_BUNDLE").ok().filter(|value| !value.is_empty()).map(std::path::PathBuf::from),
        std::env::var("OS_HUB_TRUSTED_CATALOG_PROFILE").ok().filter(|value| !value.is_empty()),
        &native_codec_providers,
    )
    .await?;
    let db = Arc::new(connect_db(&data_dir).await?);
    let directory = connect_directory(&data_dir).await?;
    // 🧹️ Contract §C0: clear crash residue before any real connection lands — a session that never
    // got its `disconnected_at` because a previous process was killed mid-connection.
    directory.close_all_sync_sessions().await?;
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), 1024));
    let artifact_cas = connect_artifact_cas(&data_dir).await?;
    let startup_control = StartupCatalogControl;
    let startup_now_ms = startup_control.now_ms();
    let startup_context = OperationContext::new(startup_now_ms.saturating_add(30_000), AuthorityLimits::maximum(), &startup_control);
    let artifact_cas_coordinator_id = directory.artifact_cas_coordinator_id().await?;
    artifact_cas.configure_coordinator(artifact_cas_coordinator_id, &startup_context).await?;
    let artifact_publication = Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority")));
    let artifact_cas_sweep_execute = artifact_cas_sweep_execute_from_env()?;
    let artifact_maintenance = ArtifactCasMaintenanceSupervisor::start(directory_service.clone(), artifact_cas.clone(), artifact_cas_sweep_execute);
    let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
    let admin_dir = std::env::var("OS_HUB_ADMIN_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist")));
    let extensions_root = std::env::var("OS_HUB_EXTENSIONS_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| data_dir.join("extension-modules"));
    std::fs::create_dir_all(&extensions_root)?;
    let run_id = local_bootstrap.as_ref().map_or_else(|| "production".to_string(), |bootstrap| bootstrap.run_id().to_string());
    let bootstrap_ready = match mode {
        HubMode::Development => local_bootstrap.as_ref().is_some_and(|bootstrap| bootstrap.is_ready()),
        HubMode::Production => identity_verifier.is_some(),
    };
    let bind_scope = if bind.is_loopback() { "loopback" } else { "network" };
    let artifact_authority_ready = artifact_authority.is_some();
    let open_plan_ready = artifact_authority.as_ref().is_some_and(|configured| configured.catalog.open_target_count() > 0);
    let verified_catalog = artifact_authority.as_ref().map(|configured| configured.catalog.clone());
    let gis_map_binding = match verified_catalog.as_ref() {
        Some(catalog) => verified_gis_map_binding(catalog.clone()).map_err(|error| AuthorityError::Catalog(format!("verified GIS Map inference binding rejected: {error:?}")))?,
        None => None,
    };
    let openable_catalog = artifact_authority.as_ref().map(|configured| -> Arc<dyn DocumentOpenCatalogAuthorityV1> { configured.catalog.clone() });
    let readiness = Arc::new(hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority_ready, open_plan_ready, admin_dir.is_dir(), true, artifact_cas_sweep_execute));
    let admin_cursor_key = SessionCapability::mint()?.secret_digest();
    let state = HubState {
        db,
        artifact_cas,
        directory: directory.clone(),
        rebootstrap,
        _artifact_authority: artifact_authority.map(|configured| configured.authority),
        verified_catalog,
        gis_map_binding,
        openable_catalog,
        _artifact_publication: artifact_publication,
        artifact_maintenance: artifact_maintenance.clone(),
        directory_service,
        admin_subjects,
        admin_cursor_key,
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        readiness,
        admin_dir,
        fanout: Arc::new(ShardedMap::new()),
        fanout_capacity: 256,
        #[cfg(test)]
        live_gate: None,
        #[cfg(test)]
        canonical_pair_authorization_gate: None,
        #[cfg(test)]
        canonical_pair_request_gate: None,
        #[cfg(test)]
        canonical_pair_deadline_ms: None,
        #[cfg(test)]
        document_open_plan_issue_gate: None,
        #[cfg(test)]
        document_open_plan_deadline_ms: None,
        presence: Arc::new(ShardedMap::new()),
        presence_publication_gate: Arc::new(tokio::sync::Mutex::new(())),
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates: Arc::new(SocketBindingGatesV1::default()),
        extensions_root,
        merge_policy: merge_policy_from_env(),
    };
    let addr = SocketAddr::new(bind, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let bootstrap_task = local_bootstrap.clone().map(|transport| {
        let control: Arc<dyn IdentityVerificationControl> = bootstrap_control.clone();
        tokio::spawn(serve_local_bootstrap(transport, directory, control))
    });
    eprintln!("[INFO] os-hub ready at http://{addr}");
    let server = std::future::IntoFuture::into_future(axum::serve(listener, router(state).into_make_service_with_connect_info::<SocketAddr>()));
    tokio::pin!(server);
    let result = if let Some(mut bootstrap_task) = bootstrap_task {
        tokio::select! {
            result = &mut server => {
                bootstrap_control.cancel();
                if let Some(transport) = local_bootstrap {
                    let _ = transport.shutdown().await;
                }
                bootstrap_task.abort();
                let _ = bootstrap_task.await;
                result.map_err(HubError::Io)
            }
            result = &mut bootstrap_task => {
                bootstrap_control.cancel();
                match result {
                    Ok(Ok(())) => Err(HubError::UnsafeAuthConfiguration("local bootstrap endpoint closed".into())),
                    Ok(Err(error)) => Err(HubError::Directory(error)),
                    Err(_) => Err(HubError::UnsafeAuthConfiguration("local bootstrap service stopped".into())),
                }
            }
        }
    } else {
        server.await.map_err(HubError::Io)
    };
    artifact_maintenance.shutdown().await;
    result
}
//#endregion 🔖️Main

//#region 🔖️Tests
// 🪶️ Gated on the `sqlite` feature (not just `test`): every test below constructs a `HubState`
// through `SqliteDirectory` — the zero-external-dependency backend — so the full bin test suite
// naturally lives behind the same feature a plain `cargo test` already enables by default (see
// `Cargo.toml`'s `default = ["sqlite"]`). `postgres`/`neo4j` each carry their own backend-only
// tests in `📇️directory/{🐘️postgres,🌐️neo4j}/🦀️.rs` instead of duplicating this suite.
#[cfg(all(test, feature = "sqlite"))]
mod tests {
    use super::*;
    use protocol::{ArtifactId as WireArtifactId, Bootstrap};
    use semio_framework_hash::Sha256;
    use semio_hub::artifact_authority::checkpoint_id_encoding_v1;

    struct StartupVerifier;

    impl IdentityAssertionVerifier for StartupVerifier {
        fn verify<'a>(&'a self, _assertion: &'a semio_hub::directory::IdentityAssertion, _context: &'a semio_hub::directory::IdentityVerificationContext<'a>) -> semio_hub::directory::IdentityVerificationFuture<'a> {
            Box::pin(async { Err(DirectoryError::Unauthorized) })
        }
    }

    struct TestLocalBootstrap;

    impl LocalBootstrapTransport for TestLocalBootstrap {
        fn run_id(&self) -> &str {
            "00112233445566778899aabbccddeeff"
        }

        fn is_ready(&self) -> bool {
            true
        }

        fn request_cancelled(&self, _request_id: &str) -> bool {
            false
        }

        fn accept<'a>(&'a self, _control: &'a dyn IdentityVerificationControl) -> semio_hub::directory::LocalBootstrapAcceptFuture<'a> {
            Box::pin(async { Ok(None) })
        }

        fn issue<'a>(
            &'a self,
            _request: &'a semio_hub::directory::VerifiedLocalBootstrapRequest,
            _session: &'a semio_hub::directory::model::IssuedAuthSession,
            _context: &'a semio_hub::directory::IdentityVerificationContext<'a>,
        ) -> semio_hub::directory::LocalBootstrapIssueFuture<'a> {
            Box::pin(async { Ok(()) })
        }

        fn reject<'a>(&'a self, _request_id: &'a str, _code: semio_hub::directory::LocalBootstrapRejectCode, _context: &'a semio_hub::directory::IdentityVerificationContext<'a>) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
            Box::pin(async { Ok(()) })
        }

        fn cancel<'a>(&'a self, _request_id: &'a str) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
            Box::pin(async { Ok(()) })
        }

        fn shutdown<'a>(&'a self) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn startup_auth_policy_fails_closed_without_owned_adapters() {
        let loopback = std::net::IpAddr::from([127, 0, 0, 1]);
        let public = std::net::IpAddr::from([0, 0, 0, 0]);
        let verifier: Arc<dyn IdentityAssertionVerifier> = Arc::new(StartupVerifier);
        let local: Arc<dyn LocalBootstrapTransport> = Arc::new(TestLocalBootstrap);
        let admin = AdminSubject { provider_digest: admin_provider_digest("oidc.example"), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
        assert!(validate_auth_startup(HubMode::Production, loopback, None, None, &[admin.clone()]).is_err());
        assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[]).is_err());
        assert!(validate_auth_startup(HubMode::Production, public, Some(&verifier), None, &[admin.clone()]).is_err());
        assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[admin]).is_ok());
        assert!(validate_auth_startup(HubMode::Development, loopback, None, None, &[]).is_err());
        assert!(validate_auth_startup(HubMode::Development, public, None, Some(&local), &[]).is_err());
        assert!(validate_auth_startup(HubMode::Development, loopback, None, Some(&local), &[]).is_ok());
    }

    #[test]
    fn readiness_v1_is_redacted_and_never_claims_public_session_issuance() {
        let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false);
        let encoded = serde_json::to_string(&ready).expect("readiness json");
        assert_eq!(ready.status, "ready");
        assert!(!ready.authentication.public_session_issuance);
        assert!(ready.artifact_authority.ready);
        assert!(!ready.features.open_plan);
        assert!(!ready.features.open_plan_exchange);
        assert!(!encoded.contains("session.v1"));
        assert!(!encoded.contains("subject"));
        assert!(!encoded.contains("channel"));
        assert!(!encoded.contains("sessionKind"));
        assert!(!encoded.contains("authorizationGeneration"));
        let partial = hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, true, false);
        assert_eq!(partial.status, "not-ready");
        assert!(partial.authentication.bootstrap_ready);
        assert!(!partial.artifact_authority.ready);
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), false, false, false, true, true, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, false, true, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "network", ready.run_id, true, true, false, true, false, false).status, "not-ready");
    }

    #[tokio::test]
    async fn artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests() {
        let state = test_state().await;
        for index in 0..5 {
            let space_id = create_space_for_test(&state, "seed", &format!("CAS {index}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let document_id = format!("cas-maintenance-{index}");
            announce_document_for_test(&state, &space_id, &document_id).await;
            publish_checkpoint_for_test(&state, &space_id, &document_id).await;
        }
        let control = StartupCatalogControl;
        let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
        let mut checkpoint = ArtifactCasMaintenanceCheckpoint::default();
        let mut requests = 0usize;
        let mut examined = 0u64;
        loop {
            let result = state.directory_service.sweep_artifact_cas(state.artifact_cas.as_ref(), checkpoint.request(false, 1), &context).await.expect("bounded maintenance page");
            requests += 1;
            examined += result.examined_objects;
            if checkpoint.accept(&result) {
                break;
            }
            assert!(requests < 128, "maintenance cursor converges");
        }
        assert!(requests > 16);
        assert!(examined > 16);
    }
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message as WsMessage};

    /// @emoji 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
    const STUDIO: &str = "default";

    #[tokio::test]
    async fn trusted_catalog_startup_is_opt_in_and_partial_configuration_fails_closed() {
        assert!(configured_artifact_authority(None, None, &NativeCodecProviderSetV1::linked()).await.expect("unconfigured authority").is_none());
        let error = match configured_artifact_authority(Some(std::path::PathBuf::from("bundle.json")), None, &NativeCodecProviderSetV1::linked()).await {
            Ok(_) => panic!("partial trusted-catalog configuration unexpectedly succeeded"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("must be configured together"));
    }

    fn native_openable_stdio_bundle() -> (std::path::PathBuf, std::path::PathBuf) {
        let root = tempdir("native-openable-stdio");
        std::fs::create_dir_all(root.join("components")).expect("stdio component directory");
        std::fs::create_dir_all(root.join("descriptors")).expect("stdio descriptor directory");
        let component = b"abc";
        let component_sha256 = os_directory::hex_lower(&Sha256::digest(component));
        let component_blake3 = blake3::hash(component).to_hex().to_string();
        let receipts = semio_s_plugin_stdio::registry::native_codec_factory_receipts().expect("artifact-owned stdio receipts");
        let manifest = semio_s_plugin_stdio::plugin().expect("stdio descriptor source").manifest;
        assert_eq!(manifest.artifact_kinds.len(), receipts.len(), "every descriptor artifact kind has one executable owner receipt");
        let viewer = manifest.apps.iter().find(|app| app.id == "s.stdio.json@rfc8259/*#viewer").expect("descriptor-owned JSON viewer").clone();
        let viewer_id = viewer.id.clone();
        let window_id = viewer.window_kinds.iter().find(|window| window.id == "framework.window.tree").expect("descriptor-owned JSON viewer window").id.clone();
        let descriptor = semio_framework::PackageDescriptor {
            descriptor_version: 1,
            package_id: "semio:stdio".into(),
            role: semio_framework::PackageRole::Plugin,
            manifest,
            activation_events: Vec::new(),
            capability_requests: Vec::new(),
            extension_points: Vec::new(),
            execution: semio_framework::ExecutionMode::Isolated,
            quotas: semio_framework::kernel::QuotaSchema::default(),
            contributions: semio_framework::ContributionSet::default(),
            assets: Vec::new(),
            hashes: semio_framework::PackageHashes { wasm_sha256: component_sha256.clone(), core_wasm_sha256: "22".repeat(32), descriptor_sha256: "33".repeat(32) },
        };
        let descriptor_bytes = directory::os_store::pack_rt::encode_wire_value(&semio_framework::to_dsl_value(&descriptor).expect("project stdio descriptor"));
        let descriptor_sha256 = os_directory::hex_lower(&Sha256::digest(&descriptor_bytes));
        let json = receipts.iter().find(|receipt| receipt.factory_id == "stdio.native.json.v1").expect("JSON receipt");
        let native_codecs = receipts
            .iter()
            .map(|receipt| {
                serde_json::json!({
                    "artifactKind": receipt.artifact_kind,
                    "artifactSchema": receipt.schema,
                    "packSchemaHash": os_directory::hex_lower(&receipt.pack_schema_hash)
                })
            })
            .collect::<Vec<_>>();
        let version = receipts[0].package_version;
        let target = serde_json::json!({
            "artifactKind": json.artifact_kind,
            "artifactSchema": json.schema,
            "packSchemaHash": os_directory::hex_lower(&json.pack_schema_hash),
            "surfaceId": viewer_id,
            "appId": viewer.id,
            "windowKindId": window_id,
            "role": "viewer",
            "rendererTarget": "wasm",
            "parentDialect": {
                "artifactKind": viewer.dialect.artifact_kind,
                "standard": viewer.dialect.standard,
                "subset": viewer.dialect.subset
            },
            "grant": { "read": true, "write": false, "observe": true }
        });
        let mut bundle = serde_json::json!({
            "schemaVersion": 2,
            "profiles": [{
                "id": "stdio-native-openable-v1",
                "selectedClosure": [{ "pluginId": "stdio", "packageId": "semio:stdio", "version": version }],
                "selectedClosureSha256": "11".repeat(32),
                "openTarget": {
                    "package": { "pluginId": "stdio", "packageId": "semio:stdio", "version": version },
                    "target": target
                },
                "generationId": "22".repeat(32)
            }],
            "packages": [{
                "pluginId": "stdio",
                "packageId": "semio:stdio",
                "version": version,
                "role": "plugin",
                "dependencies": [],
                "component": {
                    "path": "components/stdio.wasm",
                    "byteLength": component.len(),
                    "sha256": component_sha256,
                    "blake3": component_blake3
                },
                "descriptor": {
                    "path": "descriptors/stdio.descriptor.semio",
                    "byteLength": descriptor_bytes.len(),
                    "sha256": descriptor_sha256
                },
                "nativeCodecs": native_codecs,
                "openTargets": [target]
            }]
        });
        let carried = serde_json::to_vec(&bundle).expect("provisional stdio bundle");
        let (selected_closure_sha256, generation_id) = semio_hub::artifact_authority::trusted_catalog::trusted_profile_digests_json(&carried, "stdio-native-openable-v1").expect("stdio profile digests");
        bundle["profiles"][0]["selectedClosureSha256"] = selected_closure_sha256.into();
        bundle["profiles"][0]["generationId"] = generation_id.into();
        std::fs::write(root.join("components/stdio.wasm"), component).expect("write stdio component");
        std::fs::write(root.join("descriptors/stdio.descriptor.semio"), descriptor_bytes).expect("write stdio descriptor");
        let bundle_path = root.join("trusted-catalog.json");
        std::fs::write(&bundle_path, serde_json::to_vec_pretty(&bundle).expect("stdio bundle json")).expect("write stdio bundle");
        (root, bundle_path)
    }

    #[tokio::test]
    async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition() {
        let unavailable = test_state().await;
        let unavailable_addr = spawn_server(unavailable).await;
        let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
        assert_eq!(unavailable_readiness.status, 503);
        let unavailable_json: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
        assert_eq!(unavailable_json["artifactAuthority"]["ready"], false);
        assert_eq!(unavailable_json["features"]["openPlan"], false);

        let providers = NativeCodecProviderSetV1::linked();
        let (root, bundle_path) = native_openable_stdio_bundle();
        let configured = configured_artifact_authority(Some(bundle_path), Some("stdio-native-openable-v1".into()), &providers).await.expect("verified stdio authority").expect("configured stdio authority");
        assert_eq!(configured.catalog.codec_count(), 26);
        assert_eq!(configured.catalog.open_target_count(), 1);
        let mut ready = test_state().await;
        ready.openable_catalog = Some(configured.catalog.clone());
        ready._artifact_authority = Some(configured.authority);
        ready.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false));
        let ready_addr = spawn_server(ready).await;
        let readiness = raw_http_get(ready_addr, "/readyz", &[]).await;
        assert_eq!(readiness.status, 200);
        let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("ready JSON");
        assert_eq!(readiness_json["artifactAuthority"]["ready"], true);
        assert_eq!(readiness_json["features"]["openPlan"], true);
        assert_eq!(readiness_json["features"]["openPlanExchange"], true);
        let encoded = String::from_utf8(readiness.body).expect("readiness UTF-8");
        assert!(!encoded.contains("receipt"));
        assert!(!encoded.contains("factory"));
        std::fs::remove_dir_all(root).expect("remove stdio bundle fixture");
    }

    struct SyntheticDirectoryEventSource {
        head: u64,
        requests: std::sync::Mutex<Vec<(u64, usize)>>,
    }

    impl DirectoryEventPageSource for SyntheticDirectoryEventSource {
        async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
            Ok(self.head)
        }

        async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
            self.requests.lock().expect("request lock").push((since, limit));
            let limit = u64::try_from(limit).map_err(|error| DirectoryError::Backend(error.to_string()))?;
            let end = since.saturating_add(limit).min(self.head);
            Ok(((since + 1)..=end)
                .map(|seq| DirectoryEvent {
                    seq,
                    id: format!("event-{seq}"),
                    hlc: os_directory::Hlc { physical_ms: i64::try_from(seq).expect("bounded synthetic sequence"), logical: 0 },
                    actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:paged-read-law".into() },
                    space_id: Some("default".into()),
                    user_id: None,
                    body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: "default".into(), name: format!("paged-{seq}") },
                    recorded_at_ms: 0,
                })
                .collect())
        }
    }

    /// @emoji 📁️ A fresh, never-reused temp directory per call — the owned `time_ordered_id` rather than
    /// `now_ms()` alone, since `cargo test` runs this whole module's `#[tokio::test]`s
    /// concurrently within one process: two tests calling `test_state()` in the same millisecond
    /// would otherwise collide on the identical `os-hub-test-db-<pid>-<ms>` path and open the SAME
    /// `db::Database` storage root, corrupting each other's catalog/WAL state.
    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("os-hub-test-{name}-{}", directory::os_identity::time_ordered_id()));
        dir
    }

    fn run_socket_test<F, Fut>(test: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + 'static,
    {
        std::thread::Builder::new()
            .name("hub-socket-test".into())
            .stack_size(32 * 1024 * 1024)
            .spawn(move || tokio::runtime::Builder::new_current_thread().enable_all().build().expect("test runtime").block_on(test()))
            .expect("socket test thread")
            .join()
            .expect("socket test");
    }

    async fn test_state() -> HubState {
        test_state_with_capacity(1024, 256).await
    }

    async fn test_state_with_capacity(directory_capacity: usize, fanout_capacity: usize) -> HubState {
        let dir = tempdir("db");
        let database = db::Database::open_at(hub_worker_pool(), &dir, db::Profile::Test).await.expect("open db");
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        directory.seed().await.expect("seed");
        let directory: Arc<HubDirectories> = Arc::new(directory.into());
        let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
        let database = Arc::new(database);
        let artifact_cas = Arc::new(ArtifactChunkCasStores::Filesystem(FsArtifactChunkCasStorage::open(&dir.join("artifact-cas/v1")).await.expect("open artifact CAS")));
        let control = StartupCatalogControl;
        let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
        let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
        artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
        let artifact_publication =
            Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
        let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            _artifact_authority: None,
            verified_catalog: None,
            gis_map_binding: None,
            openable_catalog: None,
            _artifact_publication: artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            admin_cursor_key: [0x5a; 32],
            admin_operations: Arc::new(ShardedMap::new()),
            admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false)),
            admin_dir: dir.join("admin-dist"),
            fanout: Arc::new(ShardedMap::new()),
            fanout_capacity,
            live_gate: None,
            canonical_pair_authorization_gate: None,
            canonical_pair_request_gate: None,
            canonical_pair_deadline_ms: None,
            document_open_plan_issue_gate: None,
            document_open_plan_deadline_ms: None,
            presence: Arc::new(ShardedMap::new()),
            presence_publication_gate: Arc::new(tokio::sync::Mutex::new(())),
            presence_clock: None,
            session_colors: Arc::new(ShardedMap::new()),
            session_kicks: Arc::new(ShardedMap::new()),
            socket_grants: Arc::new(SocketGrantLedgerV1::default()),
            document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
            socket_binding_gates: Arc::new(SocketBindingGatesV1::default()),
            extensions_root: dir.join("extension-modules"),
            merge_policy: protocol::MergePolicy::default(),
        }
    }

    async fn lag_test_state(directory_capacity: usize, fanout_capacity: usize) -> HubState {
        let dir = tempdir("lag-db");
        let pool = hub_worker_pool();
        let backend = Arc::new(db::db_storage::DbBackend::Memory(db::db_storage::MemoryStorage::new(pool.clone()).await.expect("memory storage")));
        let database = Arc::new(db::Database::open(pool, db::DbConfig::for_profile(db::Profile::Test), backend).await.expect("open memory db"));
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        directory.seed().await.expect("seed");
        let directory: Arc<HubDirectories> = Arc::new(directory.into());
        let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
        let artifact_cas = Arc::new(ArtifactChunkCasStores::Memory(MemoryArtifactChunkCasStorage::default()));
        let control = StartupCatalogControl;
        let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
        let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
        artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
        let artifact_publication =
            Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
        let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            _artifact_authority: None,
            verified_catalog: None,
            gis_map_binding: None,
            openable_catalog: None,
            _artifact_publication: artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            admin_cursor_key: [0x5a; 32],
            admin_operations: Arc::new(ShardedMap::new()),
            admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false)),
            admin_dir: dir.join("admin-dist"),
            fanout: Arc::new(ShardedMap::new()),
            fanout_capacity,
            live_gate: None,
            canonical_pair_authorization_gate: None,
            canonical_pair_request_gate: None,
            canonical_pair_deadline_ms: None,
            document_open_plan_issue_gate: None,
            document_open_plan_deadline_ms: None,
            presence: Arc::new(ShardedMap::new()),
            presence_publication_gate: Arc::new(tokio::sync::Mutex::new(())),
            presence_clock: None,
            session_colors: Arc::new(ShardedMap::new()),
            session_kicks: Arc::new(ShardedMap::new()),
            socket_grants: Arc::new(SocketGrantLedgerV1::default()),
            document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
            socket_binding_gates: Arc::new(SocketBindingGatesV1::default()),
            extensions_root: dir.join("extension-modules"),
            merge_policy: protocol::MergePolicy::default(),
        }
    }

    /// @emoji 🏗️ Test-only `create-space` through `DirectoryService::execute` (the trait's own
    /// `create_space` write method is gone — see `📓️w1-b-report.md`) — returns the minted space id.
    /// `decide` performs zero authorization of its own, so `owner_user_id` need not be a real,
    /// already-existing user for these low-level fixture setups.
    async fn create_space_for_test(state: &HubState, owner_user_id: &str, name: &str, space_kind: os_directory::DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> String {
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{owner_user_id}#test") };
        let (events, _) = state.directory_service.execute(actor, DirectoryCommand::CreateSpace { name: name.to_string(), space_kind, visibility }).await.expect("create space");
        events
            .into_iter()
            .find_map(|event| match event.body {
                os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } => Some(space_id),
                _ => None,
            })
            .expect("space.created event")
    }

    /// @emoji 🏗️ Test-only `upsert-member` through `DirectoryService::execute` — `email` must match
    /// an already-minted `AuthSessionRecord`'s user for the member to land on that SAME user rather
    /// than a freshly-created one (`decide`'s `UpsertMember` resolves-or-creates by email).
    async fn upsert_member_for_test(state: &HubState, space_id: &str, email: &str, role: DirectorySpaceRole) {
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
        state.directory_service.execute(actor, DirectoryCommand::UpsertMember { space_id: space_id.to_string(), email: email.to_string(), role }).await.expect("upsert member");
    }

    fn document_descriptor_for_test(space_id: &str, document_id: &str) -> os_directory::DocumentDescriptor {
        os_directory::DocumentDescriptor {
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            artifact_kind: "test.artifact".into(),
            artifact_schema: "test.v1".into(),
            owner: os_directory::DocumentOwner { plugin_id: "test.plugin".into(), package_id: "test.package".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: "33".repeat(32),
        }
    }

    async fn announce_document_for_test(state: &HubState, space_id: &str, document_id: &str) {
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".into() };
        state.directory_service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: document_descriptor_for_test(space_id, document_id) }).await.expect("announce document");
    }

    async fn publish_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str) -> os_directory::ArtifactCheckpoint {
        let pack = b"verified-pack";
        let spr = b"verified-spr";
        let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
        let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
        let mut aggregate = Sha256::new();
        aggregate.update(pack);
        aggregate.update(spr);
        let scope = DocumentScope::new(space_id, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("descriptor");
        let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
        let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
        let mut checkpoint = os_directory::ArtifactCheckpoint {
            scope,
            checkpoint_id: os_directory::ArtifactHash([0; 32]),
            parent_checkpoint_id: None,
            descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
            baseline_frontier: os_directory::ArtifactFrontier { document_id: document_id.to_string(), head_edit_ordinal: 1, head_edit_id: "verified-edit-1".into(), last_commit_seq: 1, chain_hash: os_directory::ArtifactHash([0x44; 32]) },
            pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
            spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
            aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
            published_at_ms: 1,
        };
        checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
        let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
        let reservation = state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, ownership, 1_000, 100).await.expect("reserve checkpoint objects");
        let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
        let authority_control = StartupCatalogControl;
        let authority_context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &authority_control);
        let staged_pack = cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &authority_context).await.expect("stage reserved pack manifest");
        let staged_spr = cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &authority_context).await.expect("stage reserved SPR manifest");
        assert_eq!(staged_pack.storage_key, checkpoint.pack.storage_key);
        assert_eq!(staged_spr.storage_key, checkpoint.spr.storage_key);
        state.directory_service.publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, checkpoint.clone(), reservation, 100).await.expect("publish verified checkpoint");
        checkpoint
    }

    async fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document.clone(),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({ "value": id })).await.unwrap() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({})).await.unwrap() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    #[test]
    fn mutation_message_payload_matches_language_neutral_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
        let messages = vec![protocol::MutationMessage::warn("mutation.clamped", "height clamped").at(["node", "height"]).at_op(2), protocol::MutationMessage::info("mutation.cascade", "dependent value updated")];
        let encoded = encode_messages(&messages);
        let parsed: serde_json::Value = serde_json::from_slice(&encoded).expect("first-party message bytes are valid JSON");
        assert_eq!(parsed, fixture["mutationMessages"]);
        assert_eq!(<Vec<protocol::MutationMessage> as FromValue>::from_value(DslValue::from(parsed)).expect("first-party message decode"), messages);
    }

    async fn spawn_server(state: HubState) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state).into_make_service_with_connect_info::<SocketAddr>();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        addr
    }

    struct RawHttpResponse {
        status: u16,
        headers: String,
        body: Vec<u8>,
    }

    async fn raw_http_request_transport(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Option<RawHttpResponse> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("HTTP connect");
        let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
        for (name, value) in headers {
            request.push_str(name);
            request.push_str(": ");
            request.push_str(value);
            request.push_str("\r\n");
        }
        request.push_str("\r\n");
        stream.write_all(request.as_bytes()).await.expect("HTTP write");
        stream.write_all(body).await.expect("HTTP body write");
        stream.flush().await.expect("HTTP request flush");
        let mut response = Vec::new();
        let read = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let mut chunk = [0_u8; 4096];
            loop {
                let read = stream.read(&mut chunk).await?;
                if read == 0 {
                    return std::io::Result::Ok(());
                }
                response.extend_from_slice(&chunk[..read]);
                if let Some(boundary) = response.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                    let head = std::str::from_utf8(&response[..boundary]).unwrap_or_default();
                    let content_length = head.lines().find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length").then(|| value.trim().parse::<usize>().ok()).flatten()
                    });
                    if content_length.is_some_and(|length| response.len() >= boundary + 4 + length) {
                        return std::io::Result::Ok(());
                    }
                }
            }
        })
        .await
        .expect("HTTP deadline");
        if let Err(error) = read {
            assert!(matches!(error.kind(), std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::BrokenPipe), "HTTP read: {error}");
        }
        if response.is_empty() {
            return None;
        }
        let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").expect("HTTP header boundary");
        let head = std::str::from_utf8(&response[..boundary]).expect("HTTP headers").to_string();
        let status = head.split_whitespace().nth(1).expect("HTTP status").parse().expect("numeric HTTP status");
        Some(RawHttpResponse { status, headers: head, body: response[boundary + 4..].to_vec() })
    }

    async fn raw_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> RawHttpResponse {
        raw_http_request_transport(addr, method, path, headers, body).await.expect("HTTP response")
    }

    async fn raw_http_get(addr: SocketAddr, path: &str, headers: &[(&str, &str)]) -> RawHttpResponse {
        raw_http_request(addr, "GET", path, headers, &[]).await
    }

    #[tokio::test]
    async fn admin_page_routes_follow_declared_html_without_alias_files() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
        let contract = &fixture["adminPageRoutes"];
        let state = test_state().await;
        tokio::fs::create_dir_all(&state.admin_dir).await.expect("admin fixture directory");
        let html = contract["htmlUtf8"].as_str().expect("HTML bytes");
        let asset = contract["assetUtf8"].as_str().expect("asset bytes");
        tokio::fs::write(state.admin_dir.join(contract["htmlPath"].as_str().expect("HTML path")), html).await.expect("HTML fixture");
        tokio::fs::write(state.admin_dir.join(contract["assetPath"].as_str().expect("asset path")), asset).await.expect("asset fixture");
        for alias in contract["absentAliases"].as_array().expect("absent aliases") {
            assert!(!state.admin_dir.join(alias.as_str().expect("alias path")).exists());
        }
        let addr = spawn_server(state).await;
        for request in contract["requests"].as_array().expect("request cases") {
            let path = request["path"].as_str().expect("request path");
            let response = raw_http_get(addr, path, &[]).await;
            assert_eq!(u64::from(response.status), request["status"].as_u64().expect("status"), "{path}");
            match request["body"].as_str().expect("body kind") {
                "html" => {
                    assert_eq!(response.body, html.as_bytes());
                    assert!(response.headers.to_ascii_lowercase().contains("content-type: text/html; charset=utf-8"));
                }
                "asset" => {
                    assert_eq!(response.body, asset.as_bytes());
                    assert!(response.headers.to_ascii_lowercase().contains("content-type: text/javascript"));
                }
                "absent" => assert!(response.body.is_empty()),
                other => panic!("unknown fixture response {other}"),
            }
        }
        let missing = test_state().await;
        let addr = spawn_server(missing).await;
        assert_eq!(raw_http_get(addr, "/admin", &[]).await.status, 503);
    }

    #[tokio::test]
    async fn extension_module_routes_accept_encoded_unicode_http_paths() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
        let contract = &fixture["extensionModuleRoutes"];
        let state = test_state().await;
        let extension_id = contract["extensionId"].as_str().expect("extension ID");
        let extension_dir = state.extensions_root.join(extension_id);
        tokio::fs::create_dir_all(&extension_dir).await.expect("extension fixture directory");
        let install = serde_json::json!({ "extensionId": extension_id });
        tokio::fs::write(extension_dir.join("install.json"), serde_json::to_vec(&install).expect("install metadata")).await.expect("install fixture");
        let asset = contract["assetUtf8"].as_str().expect("asset bytes");
        tokio::fs::write(extension_dir.join(contract["assetPath"].as_str().expect("asset path")), asset).await.expect("module fixture");
        let addr = spawn_server(state).await;
        for request in contract["requests"].as_array().expect("request cases") {
            let path = request["path"].as_str().expect("request path");
            let response = raw_http_get(addr, path, &[]).await;
            assert_eq!(u64::from(response.status), request["status"].as_u64().expect("status"), "{path}");
            match request["body"].as_str().expect("body kind") {
                "listing" => assert_eq!(serde_json::from_slice::<serde_json::Value>(&response.body).expect("listing JSON"), serde_json::json!({ "extensions": [install.clone()] })),
                "asset" => {
                    assert_eq!(response.body, asset.as_bytes());
                    assert!(response.headers.to_ascii_lowercase().contains("content-type: text/javascript"));
                }
                "absent" => assert!(response.body.is_empty()),
                other => panic!("unknown fixture response {other}"),
            }
        }
    }

    #[test]
    fn canonical_pair_route_rejects_non_path_and_ambiguous_headers_before_work() {
        let session = format!("session.v1.{}.{}", "0".repeat(32), "1".repeat(64));
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {session}").parse().expect("authorization"));
        assert!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers).is_ok());
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair?checkpoint=other".parse().expect("URI"), &headers), Err(StatusCode::BAD_REQUEST));
        headers.insert(axum::http::header::RANGE, "bytes=0-1".parse().expect("range"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::RANGE_NOT_SATISFIABLE));
        headers.remove(axum::http::header::RANGE);
        headers.insert(axum::http::header::ACCEPT, "application/octet-stream".parse().expect("accept"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::NOT_ACCEPTABLE));
        headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", "a".repeat(AUTH_TEXT_MAX_BYTES)).parse().expect("oversized authorization"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer invite.v1.{}.{}", "0".repeat(32), "1".repeat(64)).parse().expect("wrong capability kind"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer session.v1.not-hex.not-hex".parse().expect("invalid capability grammar"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {session}").parse().expect("authorization"));
        headers.append(axum::http::header::AUTHORIZATION, "Bearer duplicate".parse().expect("duplicate"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
        assert!(!canonical_pair_auth_outcome_allowed(&AuthOutcome::Denied));
        assert!(canonical_pair_auth_outcome_allowed(&AuthOutcome::ShareToken));
    }

    #[tokio::test]
    async fn canonical_pair_route_is_exact_member_or_share_and_emits_only_verified_public_pair() {
        let mut state = lag_test_state(1024, 256).await;
        let document_id = "canonical-pair-document";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let checkpoint = publish_checkpoint_for_test(&state, STUDIO, document_id).await;
        let member = issue_test_session(&state, "canonical-member@example.com").await;
        upsert_member_for_test(&state, STUDIO, "canonical-member@example.com", DirectorySpaceRole::Spectator).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let issued_share = state.directory.issue_share_token(&scope, 60, "canonical-pair-share").await.expect("share issue");
        let share_id = issued_share.record.id.clone();
        let share = issued_share.capability.expose_once();
        let outsider = issue_test_session(&state, "canonical-outsider@example.com").await;
        state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", "canonical-outsider@example.com").expect("admin subject digest") }]);
        let other_space = create_space_for_test(&state, &outsider.user_id, "Canonical Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &other_space, document_id).await;
        publish_checkpoint_for_test(&state, &other_space, document_id).await;
        let public_space = create_space_for_test(&state, &outsider.user_id, "Canonical Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        announce_document_for_test(&state, &public_space, document_id).await;
        publish_checkpoint_for_test(&state, &public_space, document_id).await;
        let other_document = "canonical-pair-other-document";
        announce_document_for_test(&state, STUDIO, other_document).await;
        publish_checkpoint_for_test(&state, STUDIO, other_document).await;
        let addr = spawn_server(state.clone()).await;
        let path = format!("/spaces/{STUDIO}/documents/{document_id}/active-checkpoint/pair");
        let accept = [("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE)];
        let member_authorization = format!("Bearer {}", member.token);
        assert_eq!(raw_http_get(addr, &format!("{path}?checkpoint=other"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await.status, 400);
        assert_eq!(raw_http_get(addr, &path, &[("Accept", "application/octet-stream"), ("Authorization", &member_authorization)]).await.status, 406);
        assert_eq!(raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Range", "bytes=0-1"), ("Authorization", &member_authorization)]).await.status, 416);
        assert_eq!(raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization), ("Authorization", &member_authorization)]).await.status, 401);
        let public = raw_http_get(addr, &path, &accept).await;
        assert_eq!(public.status, 401);
        assert!(public.body.is_empty());
        let malformed = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", "Bearer malformed")]).await;
        assert_eq!(malformed.status, 401);
        let public_fallback = raw_http_get(addr, &format!("/spaces/{public_space}/documents/{document_id}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", "Bearer malformed")]).await;
        assert_eq!(public_fallback.status, 401);
        let denied = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {}", outsider.token))]).await;
        assert_eq!(denied.status, 401);
        assert!(denied.body.is_empty());
        let cross_space = raw_http_get(addr, &format!("/spaces/{other_space}/documents/{document_id}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {}", member.token))]).await;
        assert_eq!(cross_space.status, 401);
        let cross_document_share = raw_http_get(addr, &format!("/spaces/{STUDIO}/documents/{other_document}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {share}"))]).await;
        assert_eq!(cross_document_share.status, 401);

        for token in [&member.token, &share] {
            let response = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {token}"))]).await;
            assert_eq!(response.status, 200);
            let lower_headers = response.headers.to_ascii_lowercase();
            assert!(lower_headers.contains(&format!("content-type: {CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE}")));
            assert!(lower_headers.contains("cache-control: private, no-store"));
            assert!(lower_headers.contains("vary: authorization"));
            assert!(lower_headers.contains("etag: \""));
            let verified = decode_canonical_checkpoint_pair(&response.body).expect("verified route body");
            assert_eq!(verified.selection.scope, scope);
            assert_eq!(verified.selection.active_checkpoint_id, checkpoint.checkpoint_id);
            assert_eq!(verified.pair().pack, b"verified-pack");
            assert_eq!(verified.pair().spr, b"verified-spr");
            assert!(!String::from_utf8_lossy(&response.body).contains("cas/v1"));
            assert!(!String::from_utf8_lossy(&response.body).contains("manifest"));
        }
        for authorization_checks_before_revoke in [1usize, 3, 4] {
            let checks = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let gate_checks = checks.clone();
            let mut revoked_state = state.clone();
            revoked_state.canonical_pair_authorization_gate = Some(Arc::new(move || gate_checks.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < authorization_checks_before_revoke));
            let revoked_addr = spawn_server(revoked_state).await;
            let response = raw_http_get(revoked_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await;
            assert_eq!(response.status, 401);
            assert!(response.body.is_empty(), "revocation never publishes a partial framed body");
            assert_eq!(checks.load(std::sync::atomic::Ordering::SeqCst), authorization_checks_before_revoke + 1);
        }
        let mut missing_cas_state = state.clone();
        missing_cas_state.rebootstrap = Arc::new(VerifiedRebootstrapSource::new(state.directory.clone(), Arc::new(ArtifactChunkCasStores::Memory(MemoryArtifactChunkCasStorage::default()))));
        let missing_cas_addr = spawn_server(missing_cas_state).await;
        let missing = raw_http_get(missing_cas_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await;
        assert_eq!(missing.status, 409);
        assert!(missing.body.is_empty());
        state.directory.revoke_share_token(&scope, &share_id, "test-revoked", "canonical-pair-revoke").await.expect("revoke share");
        let revoked = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {share}"))]).await;
        assert_eq!(revoked.status, 401);
        assert!(revoked.body.is_empty());
    }

    #[tokio::test]
    async fn canonical_pair_route_disconnect_deadline_and_progress_are_request_owned() {
        use tokio::io::AsyncWriteExt;

        let state = lag_test_state(1024, 256).await;
        let document_id = "canonical-pair-lifecycle-document";
        announce_document_for_test(&state, STUDIO, document_id).await;
        publish_checkpoint_for_test(&state, STUDIO, document_id).await;
        let member = issue_test_session(&state, "canonical-lifecycle@example.com").await;
        upsert_member_for_test(&state, STUDIO, "canonical-lifecycle@example.com", DirectorySpaceRole::Spectator).await;
        let path = format!("/spaces/{STUDIO}/documents/{document_id}/active-checkpoint/pair");
        let authorization = format!("Bearer {}", member.token);

        let progress_gate = Arc::new(TestCanonicalPairRequestGate::new(1));
        let mut progress_state = state.clone();
        progress_state.canonical_pair_request_gate = Some(progress_gate.clone());
        let progress_addr = spawn_server(progress_state).await;
        let response = raw_http_get(progress_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &authorization)]).await;
        assert_eq!(response.status, 200);
        let progress_control = progress_gate.control();
        assert!(!progress_control.is_cancelled());
        assert!(!progress_control.is_active());
        let progress = progress_control.progress_snapshot();
        for stage in [
            RebootstrapProgressStage::Authorize,
            RebootstrapProgressStage::Metadata,
            RebootstrapProgressStage::VerifyPack,
            RebootstrapProgressStage::VerifySpr,
            RebootstrapProgressStage::StreamPack,
            RebootstrapProgressStage::StreamSpr,
            RebootstrapProgressStage::Ready,
        ] {
            assert_eq!(progress[canonical_pair_progress_index(stage)].expect("bounded route progress").stage, stage);
        }
        assert!(progress[canonical_pair_progress_index(RebootstrapProgressStage::Chunk)].is_none());

        let deadline_gate = Arc::new(TestCanonicalPairRequestGate::new(0));
        let mut deadline_state = state.clone();
        deadline_state.canonical_pair_request_gate = Some(deadline_gate.clone());
        deadline_state.canonical_pair_deadline_ms = Some(20);
        let deadline_addr = spawn_server(deadline_state).await;
        let deadline = raw_http_get(deadline_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &authorization)]).await;
        assert_eq!(deadline.status, 504);
        assert!(deadline.body.is_empty());
        let deadline_control = deadline_gate.control();
        assert!(deadline_control.is_cancelled());
        assert!(!deadline_control.is_active());
        assert_eq!(deadline_control.progress_snapshot(), [None; CANONICAL_PAIR_PROGRESS_STAGES]);

        let disconnect_gate = Arc::new(TestCanonicalPairRequestGate::new(0));
        let mut disconnect_state = state;
        disconnect_state.canonical_pair_request_gate = Some(disconnect_gate.clone());
        let disconnect_addr = spawn_server(disconnect_state).await;
        let mut stream = tokio::net::TcpStream::connect(disconnect_addr).await.expect("disconnect HTTP connect");
        let request = format!("GET {path} HTTP/1.1\r\nHost: {disconnect_addr}\r\nAccept: {CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE}\r\nAuthorization: {authorization}\r\nConnection: keep-alive\r\n\r\n");
        stream.write_all(request.as_bytes()).await.expect("disconnect HTTP write");
        let entered = tokio::time::timeout(std::time::Duration::from_secs(2), disconnect_gate.entered.acquire()).await.expect("disconnect admission deadline").expect("disconnect admission");
        entered.forget();
        let disconnect_control = disconnect_gate.control();
        let before_disconnect = disconnect_control.progress_snapshot();
        drop(stream);
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while disconnect_control.is_active() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("disconnect cancellation deadline");
        assert!(disconnect_control.is_cancelled());
        assert!(!disconnect_control.is_active());
        let after_disconnect = disconnect_control.progress_snapshot();
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        assert_eq!(before_disconnect, after_disconnect);
        assert_eq!(after_disconnect, disconnect_control.progress_snapshot());
    }

    /// @emoji 🧪️ A loopback `ConnectInfo` for handlers called directly in tests. Network
    /// proximity confers no authorization; every protected test also supplies a verified session.
    fn loopback_peer() -> axum::extract::ConnectInfo<SocketAddr> {
        axum::extract::ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 0)))
    }

    async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Binary(bytes)))) => return protocol::decode_server_frame(&bytes).await.expect("server frame").1,
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected binary frame, got {other:?}"),
                Ok(None) => panic!("stream ended before server frame"),
                Err(_) => panic!("no server frame before 5s deadline"),
            }
        }
    }

    async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Text(text)))) => return directory::os_pack::json::from_json_str(&text).expect("directory message"),
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected directory message, got {other:?}"),
                Ok(None) => panic!("stream ended before directory message"),
                Err(_) => panic!("no directory message before 5s deadline"),
            }
        }
    }

    async fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage {
        WsMessage::Binary(protocol::encode_client_frame(frame, lane).await.into())
    }

    async fn next_close_code<S>(ws: &mut S, allow_text: bool) -> u16
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Close(Some(frame))))) => return frame.code.into(),
                Ok(Some(Ok(WsMessage::Text(text)))) if !allow_text => panic!("unauthorized rebootstrap control leaked: {text}"),
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected close frame, got {other:?}"),
                Ok(None) => panic!("stream ended before close frame"),
                Err(_) => panic!("no close frame before 5s deadline"),
            }
        }
    }

    async fn next_close_without_authority<S>(ws: &mut S) -> u16
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
            Ok(Some(Ok(WsMessage::Close(Some(frame))))) => frame.code.into(),
            Ok(Some(Ok(WsMessage::Binary(_)))) => panic!("authority-bearing binary frame crossed revocation"),
            Ok(Some(Ok(WsMessage::Text(_)))) => panic!("authority-bearing directory frame crossed revocation"),
            Ok(Some(other)) => panic!("expected close after revocation, got {other:?}"),
            Ok(None) => panic!("stream ended before revocation close"),
            Err(_) => panic!("no revocation close before 5s deadline"),
        }
    }

    fn socket_hello() -> ClientFrame {
        ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0x11; 32], resume_token: None, frontier: None }
    }

    fn bearer_headers(capability: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {capability}").parse().expect("bearer header"));
        headers
    }

    fn socket_request(url: &str, grant: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
        let mut request = url.into_client_request().expect("socket request");
        request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {grant}").parse().expect("socket protocols"));
        request
    }

    struct TestIssuedSession {
        token: String,
        user_id: String,
    }

    async fn issue_test_session(state: &HubState, email: &str) -> TestIssuedSession {
        let user = match state.directory.get_user_by_email(email).await.expect("test user lookup") {
            Some(user) => user,
            None => state.directory.create_user(email, email, None, Some(email), Some("test-verifier")).await.expect("test verified user"),
        };
        let issue = AuthSessionIssue {
            user_id: user.id.clone(),
            identity_provider: "test-verifier".into(),
            identity_subject_digest: identity_subject_digest("test-verifier", email).expect("test subject digest"),
            ttl_secs: 3_600,
            device_instance_id: "test-device".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: directory::os_identity::time_ordered_id(),
            peer_class: "test".into(),
        };
        let issued = state.directory.issue_auth_session(&issue).await.expect("test session issue");
        TestIssuedSession { token: issued.capability.expose_once(), user_id: user.id }
    }

    async fn authorize_test_admin(state: &mut HubState, email: &str) -> HeaderMap {
        let session = issue_test_session(state, email).await;
        state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", email).expect("test admin subject digest") }]);
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().expect("test bearer header"));
        headers
    }

    async fn seed_author_token(state: &HubState) -> String {
        let issue = AuthSessionIssue {
            user_id: "seed".into(),
            identity_provider: "test-verifier".into(),
            identity_subject_digest: identity_subject_digest("test-verifier", "seed").expect("seed subject digest"),
            ttl_secs: 3_600,
            device_instance_id: "seed-device".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: directory::os_identity::time_ordered_id(),
            peer_class: "test".into(),
        };
        state.directory.issue_auth_session(&issue).await.expect("seed author session").capability.expose_once()
    }

    #[tokio::test]
    async fn socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe() {
        let ledger = Arc::new(SocketGrantLedgerV1::default());
        let audience = SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a"));
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000 };
        let capability = SocketGrantCapability::mint().expect("socket grant");
        ledger.issue(&capability, audience.clone(), "hub.v1.actor".into(), subject.clone(), 1, 9_000).expect("issue grant");
        assert!(ledger.pending(&capability, &SocketAudienceV1::Document(DocumentScope::new("space-a", "document-b")), 2).is_err(), "audience mismatch never consumes");
        let candidate = ledger.pending(&capability, &audience, 2).expect("pending grant");
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let attempts = (0..2)
            .map(|_| {
                let ledger = ledger.clone();
                let candidate = candidate.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    ledger.consume(&candidate, 3).is_ok()
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("consume race")).filter(|won| *won).count(), 1, "exactly one concurrent upgrade consumes");
        assert!(ledger.pending(&capability, &audience, 4).is_err(), "consumed grants never replay");
        assert!(SocketGrantLedgerV1::default().pending(&capability, &audience, 4).is_err(), "grants are process-bound and disappear on restart");
        let (_, live_notify) = ledger.register_live(&candidate).expect("register consumed grant live");

        let pending = SocketGrantCapability::mint().expect("pending socket grant");
        ledger.issue(&pending, audience.clone(), "hub.v1.pending".into(), subject.clone(), 4, 9_000).expect("issue pending grant");
        let stale = ledger.pending(&pending, &audience, 5).expect("candidate before revoke");
        ledger.invalidate_binding(subject.binding());
        tokio::time::timeout(std::time::Duration::from_secs(1), live_notify.notified()).await.expect("live revoke notification");
        assert!(ledger.consume(&stale, 6).is_err(), "revoke between durable revalidation and consume fails closed");
        assert!(ledger.register_live(&candidate).is_err(), "consume then revoke then late register fails closed");

        let ttl_ledger = SocketGrantLedgerV1::default();
        let ttl_capability = SocketGrantCapability::mint().expect("TTL grant");
        ttl_ledger.issue(&ttl_capability, audience.clone(), "hub.v1.ttl".into(), subject.clone(), 1, 10).expect("issue TTL grant");
        let ttl_candidate = ttl_ledger.pending(&ttl_capability, &audience, 2).expect("pending TTL grant");
        let ttl_consumed = ttl_ledger.consume(&ttl_candidate, 3).expect("consume TTL grant");
        let (ttl_live_id, _) = ttl_ledger.register_live(&ttl_consumed).expect("register TTL grant live");
        let sweep_trigger = SocketGrantCapability::mint().expect("sweep trigger");
        ttl_ledger.issue(&sweep_trigger, audience.clone(), "hub.v1.sweep".into(), subject.clone(), 11, 100).expect("trigger grant sweep");
        assert!(ttl_ledger.is_live(&ttl_consumed, &ttl_live_id), "grant TTL applies to dial/consume, not a durably-authorized live socket");
        ttl_ledger.unregister_live(&ttl_consumed, &ttl_live_id);
        assert!(!ttl_ledger.inner.lock().expect("ledger").records.contains_key(ttl_capability.selector()), "last live lease reclaims its consumed grant record");

        let abandoned = SocketGrantLedgerV1::default();
        let mut first_abandoned = None;
        for index in 0..SOCKET_GRANT_LEDGER_CAPACITY {
            let capability = SocketGrantCapability::mint().expect("abandoned grant");
            abandoned.issue(&capability, audience.clone(), format!("hub.v1.abandoned.{index}"), subject.clone(), 1, 10).expect("fill ledger");
            let candidate = abandoned.pending(&capability, &audience, 2).expect("abandoned pending");
            abandoned.consume(&candidate, 3).expect("abandoned consume");
            first_abandoned.get_or_insert(capability);
        }
        assert!(abandoned.pending(first_abandoned.as_ref().expect("first abandoned"), &audience, 4).is_err(), "consumed failed-pre-live grant never replays");
        let recovered = SocketGrantCapability::mint().expect("recovered grant");
        abandoned.issue(&recovered, audience.clone(), "hub.v1.recovered".into(), subject.clone(), 11, 100).expect("expired pre-live tombstones reclaim full ledger capacity");

        let bounded = SocketGrantLedgerV1::default();
        for index in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
            let capability = SocketGrantCapability::mint().expect("bounded grant");
            bounded.issue(&capability, audience.clone(), format!("hub.v1.{index}"), subject.clone(), 1, 9_000).expect("within per-binding bound");
        }
        let overflow = SocketGrantCapability::mint().expect("overflow grant");
        assert_eq!(bounded.issue(&overflow, audience, "hub.v1.overflow".into(), subject.clone(), 1, 9_000), Err(SocketGrantLedgerErrorV1::Capacity));
        bounded.invalidate_binding(subject.binding());
        assert!(bounded.issue(&overflow, SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a")), "hub.v1.after-revoke".into(), subject, 2, 9_000).is_ok());
    }

    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct DocumentOpenPlanLedgerFixture {
        now_ms: u64,
        descriptor: DocumentDescriptor,
        descriptor_digest_v1: String,
        valid_plan: DocumentOpenPlanV1,
    }

    fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {
        DocumentOpenPlanAuthorityV1 {
            scope: fixture.valid_plan.scope.clone(),
            descriptor: fixture.descriptor.clone(),
            descriptor_digest_v1: fixture.descriptor_digest_v1.clone(),
            catalog: fixture.valid_plan.catalog.clone(),
            package: fixture.valid_plan.package.clone(),
            artifact: fixture.valid_plan.artifact.clone(),
            parent_dialect: semio_framework::ArtifactDialect {
                artifact_kind: fixture.valid_plan.parent_dialect.artifact_kind.clone(),
                standard: fixture.valid_plan.parent_dialect.standard.clone(),
                subset: fixture.valid_plan.parent_dialect.subset.clone(),
            },
            surface: fixture.valid_plan.surface.clone(),
            grant: fixture.valid_plan.grant,
            checkpoint: fixture.valid_plan.checkpoint.clone(),
            revalidation: fixture.valid_plan.revalidation,
            subject: SocketSubjectV1::Session {
                session_id: "open-plan-session".into(),
                user_id: "open-plan-user".into(),
                authorization_generation: fixture.valid_plan.revalidation.session_generation.expect("session generation"),
                role: Some(SpaceRole::Author),
                expires_at_ms: i64::MAX,
            },
            server_actor_id: "hub.v1.open-plan-actor".into(),
            client_instance_id_digest: [9; 32],
        }
    }

    fn document_open_catalog_for_descriptor(descriptor: &DocumentDescriptor) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
        document_open_catalog_for_descriptor_with_generation(descriptor, "66".repeat(32))
    }

    fn document_open_catalog_for_descriptor_with_generation(descriptor: &DocumentDescriptor, generation_id: String) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
        let package = DocumentOpenPackageV1 {
            plugin_id: descriptor.owner.plugin_id.clone(),
            package_id: descriptor.owner.package_id.clone(),
            version: descriptor.owner.version.clone(),
            component_sha256: descriptor.owner.package_hash.clone(),
            component_blake3: "44".repeat(32),
            descriptor_byte_sha256: "55".repeat(32),
        };
        let artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
        Arc::new(TestDocumentOpenCatalog {
            generation_id,
            open_targets: vec![
                VerifiedDocumentOpenSelectionV1 {
                    package: package.clone(),
                    artifact: artifact.clone(),
                    parent_dialect: semio_framework::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
                    surface: DocumentOpenSurfaceV1 {
                        surface_id: "surface.test.editor".into(),
                        app_id: "app.test".into(),
                        window_kind_id: "window.document".into(),
                        role: os_directory::DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                    },
                    grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                },
                VerifiedDocumentOpenSelectionV1 {
                    package,
                    artifact,
                    parent_dialect: semio_framework::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
                    surface: DocumentOpenSurfaceV1 {
                        surface_id: "surface.test.viewer".into(),
                        app_id: "app.test".into(),
                        window_kind_id: "window.document".into(),
                        role: os_directory::DocumentOpenSurfaceRoleV1::Viewer,
                        renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                    },
                    grant: DocumentOpenGrantV1 { read: true, write: false, observe: true },
                },
            ]
            .into_boxed_slice(),
        })
    }

    fn document_open_plan_secret(index: u32) -> [u8; 32] {
        let mut secret = [0u8; 32];
        secret[28..].copy_from_slice(&index.to_be_bytes());
        secret
    }

    fn document_open_plan_authority_for_scope(base: &DocumentOpenPlanAuthorityV1, binding: u32, document: u32) -> DocumentOpenPlanAuthorityV1 {
        let mut authority = base.clone();
        authority.scope.document_id = format!("document-{document}");
        authority.descriptor.document_id = authority.scope.document_id.clone();
        authority.checkpoint = None;
        authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("scoped descriptor digest").0);
        authority.subject = SocketSubjectV1::Session {
            session_id: format!("open-plan-session-{binding}"),
            user_id: format!("open-plan-user-{binding}"),
            authorization_generation: authority.revalidation.session_generation.expect("session generation"),
            role: Some(SpaceRole::Author),
            expires_at_ms: i64::MAX,
        };
        authority
    }

    async fn document_open_plan_authority_for_session(state: &HubState, fixture: &DocumentOpenPlanLedgerFixture, token: &str, scope: DocumentScope) -> DocumentOpenPlanAuthorityV1 {
        let capability = SessionCapability::parse(token).expect("session capability");
        let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
        let role = state.directory.get_role(&scope.space_id, &session.user_id).await.expect("role lookup").expect("space member");
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("announced descriptor");
        let mut authority = document_open_plan_test_authority(fixture);
        authority.scope = scope;
        authority.descriptor = descriptor;
        authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("descriptor digest").0);
        authority.package.plugin_id = authority.descriptor.owner.plugin_id.clone();
        authority.package.package_id = authority.descriptor.owner.package_id.clone();
        authority.package.version = authority.descriptor.owner.version.clone();
        authority.package.component_sha256 = authority.descriptor.owner.package_hash.clone();
        authority.artifact.kind = authority.descriptor.artifact_kind.clone();
        authority.artifact.schema = authority.descriptor.artifact_schema.clone();
        authority.artifact.pack_schema_hash = authority.descriptor.pack_schema_hash.clone();
        if let Some(catalog) = &state.openable_catalog {
            let selected = catalog.resolve_document_open(&authority.descriptor, None, matches!(role, SpaceRole::Author)).expect("test catalog selection");
            authority.catalog.generation_id = catalog.generation_id().into();
            authority.package = selected.package;
            authority.artifact = selected.artifact;
            authority.parent_dialect = selected.parent_dialect;
            authority.surface = selected.surface;
            authority.grant = selected.grant;
        }
        authority.checkpoint = None;
        authority.grant.write = matches!(role, SpaceRole::Author);
        let directory_revision = state.directory.head_seq().await.expect("directory revision");
        authority.revalidation.directory_revision = directory_revision;
        authority.revalidation.membership_generation = directory_revision;
        authority.revalidation.session_generation = Some(session.authorization_generation);
        authority.revalidation.share_generation = None;
        authority.subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at };
        authority.server_actor_id = socket_actor_id(&session.secret_digest, true);
        authority.validate().expect("authenticated route authority");
        authority
    }

    async fn issue_and_exchange_document_open_plan_for_test(state: &HubState, token: &str, scope: &DocumentScope, client_instance_id: &str) -> (DocumentOpenPlanV1, SocketGrantReceiptV1) {
        let mut headers = bearer_headers(token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some("surface.test.editor".into()), client_instance_id: client_instance_id.into() };
        let DirectoryJson(plan) = match issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), Bytes::from(directory::os_pack::json::to_json_string(&intent))).await {
            Ok(plan) => plan,
            Err(_) => panic!("issue document open plan"),
        };
        let exchange = DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan.receipt.clone() };
        let Json(grant) = match issue_document_plan_socket_grant_inner(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), Bytes::from(directory::os_pack::json::to_json_string(&exchange))).await {
            Ok(grant) => grant,
            Err(_) => panic!("exchange document open plan"),
        };
        (plan, grant)
    }

    #[test]
    fn document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped() {
        let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
        let authority = document_open_plan_test_authority(&fixture);
        for (field, value) in [("artifactKind", "s.foreign.document".to_owned()), ("standard", String::new()), ("subset", String::new()), ("standard", "\u{85}".to_owned()), ("subset", " * ".to_owned()), ("standard", "🌊".repeat(65))] {
            let mut hostile = authority.clone();
            match field {
                "artifactKind" => hostile.parent_dialect.artifact_kind = value,
                "standard" => hostile.parent_dialect.standard = value,
                "subset" => hostile.parent_dialect.subset = value,
                _ => unreachable!(),
            }
            assert_eq!(hostile.validate(), Err(DocumentOpenPlanErrorCodeV1::Stale), "invalid private parent {field}");
        }
        let ledger = Arc::new(DocumentOpenPlanLedgerV1::default());
        let secret = std::array::from_fn(|index| u8::try_from(index + 1).expect("fixture secret byte"));
        let public = ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DocumentOpenPlanCapabilityV1::from_secret(secret)).expect("issue fixture plan");
        assert_eq!(public.receipt, "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA");
        assert_eq!(public.parent_dialect, fixture.valid_plan.parent_dialect);
        let mut public_parent_kind = public.clone();
        public_parent_kind.parent_dialect.artifact_kind.push_str(".foreign");
        assert_eq!(public_parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let mut public_parent_control = public.clone();
        public_parent_control.parent_dialect.standard.push('\u{85}');
        assert_eq!(public_parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
        let expected_digest = [0x5d, 0x05, 0xc0, 0xd4, 0x09, 0x43, 0xe9, 0xab, 0xd9, 0x66, 0x33, 0xca, 0x62, 0xfe, 0x36, 0x2b, 0x58, 0xc6, 0x6e, 0xa8, 0x5a, 0xe0, 0xa0, 0xba, 0x1f, 0x02, 0x48, 0x8e, 0x38, 0xc0, 0xd3, 0x24];
        {
            let inner = ledger.inner.lock().expect("open plan ledger");
            let record = inner.records.get(&expected_digest).expect("digest-only record");
            assert_eq!(record.receipt_digest, expected_digest);
            assert_eq!(record.issued_at_ms, fixture.now_ms);
            assert_eq!(record.socket_grant_selector, None);
        }

        let barrier = Arc::new(std::sync::Barrier::new(9));
        let exchange_at = fixture.now_ms + 1;
        let attempts = (0..8)
            .map(|_| {
                let ledger = ledger.clone();
                let barrier = barrier.clone();
                let receipt = public.receipt.clone();
                let authority = authority.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    ledger.exchange(&receipt, &authority, exchange_at, "socket-selector").is_ok()
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("exchange race")).filter(|won| *won).count(), 1);
        assert_eq!(ledger.exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector-2"), Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed));
        assert_eq!(DocumentOpenPlanLedgerV1::default().exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Denied));

        let mismatch_ledger = DocumentOpenPlanLedgerV1::default();
        let mismatch = mismatch_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10))).expect("issue mismatch plan");
        let mut foreign = authority.clone();
        foreign.catalog.generation_id = "77".repeat(32);
        assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
        let mut foreign_parent = authority.clone();
        foreign_parent.parent_dialect.standard = "2".into();
        assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign_parent, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
        assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &authority, fixture.now_ms + 100, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Expired));

        let replacement_ledger = DocumentOpenPlanLedgerV1::default();
        let first = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(11))).expect("first outstanding plan");
        let second = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 101, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(12))).expect("replacement outstanding plan");
        assert_eq!(replacement_ledger.exchange(&first.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
        replacement_ledger.invalidate_receipt(&second.receipt).expect("cancel after publication");
        assert_eq!(replacement_ledger.exchange(&second.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

        let replacement_expiry_ledger = DocumentOpenPlanLedgerV1::default();
        let replaced_a = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(14))).expect("replacement A");
        let replaced_b = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 200, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(15))).expect("replacement B");
        let replaced_c =
            replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 101, fixture.now_ms + 201, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(16))).expect("replacement C after A expiry sweep");
        assert_eq!(replacement_expiry_ledger.exchange(&replaced_a.receipt, &authority, fixture.now_ms + 102, "socket-a"), Err(DocumentOpenPlanErrorCodeV1::Denied));
        assert_eq!(replacement_expiry_ledger.exchange(&replaced_b.receipt, &authority, fixture.now_ms + 102, "socket-b"), Err(DocumentOpenPlanErrorCodeV1::Stale));
        assert!(replacement_expiry_ledger.exchange(&replaced_c.receipt, &authority, fixture.now_ms + 102, "socket-c").is_ok());

        let revoke_ledger = DocumentOpenPlanLedgerV1::default();
        let revoked = revoke_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(13))).expect("revocable plan");
        assert_eq!(revoke_ledger.invalidate_binding(&authority.subject.binding()), 1);
        assert_eq!(revoke_ledger.exchange(&revoked.receipt, &authority, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

        let share_scope = authority.scope.clone();
        let mut share = authority.clone();
        share.subject = SocketSubjectV1::Share { share_id: "share-plan".into(), selector: "share-selector".into(), scope: share_scope, expires_at_ms: i64::MAX };
        share.revalidation.session_generation = None;
        share.revalidation.share_generation = Some(1);
        share.surface.role = directory::os_directory::DocumentOpenSurfaceRoleV1::Viewer;
        share.grant.write = false;
        assert!(DocumentOpenPlanLedgerV1::default().issue(share.clone(), fixture.now_ms, fixture.now_ms + 100).is_ok());
        share.grant.write = true;
        assert_eq!(DocumentOpenPlanLedgerV1::default().issue(share, fixture.now_ms, fixture.now_ms + 100), Err(DocumentOpenPlanErrorCodeV1::Stale));

        let mut beyond_binding = authority.clone();
        if let SocketSubjectV1::Session { expires_at_ms, .. } = &mut beyond_binding.subject {
            *expires_at_ms = i64::try_from(fixture.now_ms + 50).expect("binding expiry");
        }
        assert_eq!(DocumentOpenPlanLedgerV1::default().issue_with_capability(beyond_binding, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(22)),), Err(DocumentOpenPlanErrorCodeV1::Denied));

        let binding_bounded = DocumentOpenPlanLedgerV1::default();
        for document in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
            let scoped = document_open_plan_authority_for_scope(&authority, 1, document as u32);
            binding_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(100 + document as u32))).expect("within binding capacity");
        }
        let overflow_scope = document_open_plan_authority_for_scope(&authority, 1, DOCUMENT_OPEN_PLAN_BINDING_CAPACITY as u32);
        assert_eq!(binding_bounded.issue_with_capability(overflow_scope, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(500))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));

        let globally_bounded = DocumentOpenPlanLedgerV1::default();
        for index in 0..DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY {
            let scoped = document_open_plan_authority_for_scope(&authority, (index / DOCUMENT_OPEN_PLAN_BINDING_CAPACITY) as u32, index as u32);
            globally_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("within global capacity");
        }
        let overflow = document_open_plan_authority_for_scope(&authority, 99, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY as u32);
        assert_eq!(globally_bounded.issue_with_capability(overflow, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10_000))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
        assert!(matches!(DocumentOpenPlanCapabilityV1::parse("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB"), Err(DocumentOpenPlanErrorCodeV1::Denied)));
    }

    #[test]
    fn document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant() {
        let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
        let authority = document_open_plan_test_authority(&fixture);
        let plans = Arc::new(DocumentOpenPlanLedgerV1::default());
        let sockets = Arc::new(SocketGrantLedgerV1::default());
        let public = plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(30))).expect("route-inaccessible plan fixture");

        let barrier = Arc::new(std::sync::Barrier::new(9));
        let attempts = (0..8)
            .map(|_| {
                let plans = plans.clone();
                let sockets = sockets.clone();
                let barrier = barrier.clone();
                let authority = authority.clone();
                let receipt = public.receipt.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    plans.exchange_to_socket_grant(&receipt, &authority, fixture.now_ms + 1, sockets.as_ref())
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        let outcomes = attempts.into_iter().map(|attempt| attempt.join().expect("plan exchange race")).collect::<Vec<_>>();
        assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
        assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed))).count(), 7);
        let response = outcomes.into_iter().find_map(Result::ok).expect("one socket grant response");
        assert_eq!(response.schema, "semio.hub.socket-grant/v1");
        assert_eq!(response.protocol, SOCKET_PROTOCOL_V1);
        assert_eq!(response.actor_id, authority.server_actor_id);
        assert_eq!(response.expires_at_ms, i64::try_from(fixture.now_ms + 100).expect("fixture expiry"));
        let encoded_response = serde_json::to_string(&response).expect("socket grant response encodes");
        assert!(!encoded_response.contains(&public.receipt));
        assert!(!encoded_response.contains(&authority.descriptor_digest_v1));
        assert!(!encoded_response.contains(&authority.package.component_sha256));
        assert!(!encoded_response.contains(&authority.scope.document_id));
        assert!(!encoded_response.contains("receipt"));
        let socket_capability = SocketGrantCapability::parse(&response.grant).expect("socket grant parses");
        let audience = SocketAudienceV1::Document(authority.scope.clone());
        let pending = sockets.pending(&socket_capability, &audience, i64::try_from(fixture.now_ms + 2).expect("fixture time")).expect("exact pending document grant");
        assert_eq!(pending.actor_id, authority.server_actor_id);
        assert_eq!(pending.subject, authority.subject);
        assert_eq!(pending.document_plan.as_deref(), Some(&authority));
        assert_eq!(pending.expires_at_ms, response.expires_at_ms);
        assert_eq!(sockets.inner.lock().expect("socket ledger").records.len(), 1);
        let plan_digest = DocumentOpenPlanCapabilityV1::parse(&public.receipt).expect("plan receipt parses").digest();
        let plan_record = plans.inner.lock().expect("plan ledger").records.get(&plan_digest).expect("plan record").clone();
        assert_eq!(plan_record.state, DocumentOpenPlanStateV1::Consumed);
        assert_eq!(plan_record.socket_grant_selector.as_deref(), Some(socket_capability.selector()));

        let capacity_plans = DocumentOpenPlanLedgerV1::default();
        let capacity_sockets = SocketGrantLedgerV1::default();
        let capacity_plan = capacity_plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(31))).expect("capacity plan");
        for _ in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
            let capability = SocketGrantCapability::mint().expect("capacity socket grant");
            capacity_sockets
                .issue(
                    &capability,
                    SocketAudienceV1::Document(authority.scope.clone()),
                    authority.server_actor_id.clone(),
                    authority.subject.clone(),
                    i64::try_from(fixture.now_ms).expect("fixture time"),
                    i64::try_from(fixture.now_ms + 1_000).expect("fixture expiry"),
                )
                .expect("fill per-binding socket grant capacity");
        }
        assert!(matches!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 1, &capacity_sockets), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)));
        let capacity_digest = DocumentOpenPlanCapabilityV1::parse(&capacity_plan.receipt).expect("capacity receipt parses").digest();
        let capacity_record = capacity_plans.inner.lock().expect("capacity plan ledger").records.get(&capacity_digest).expect("capacity plan remains").clone();
        assert_eq!(capacity_record.state, DocumentOpenPlanStateV1::Issued);
        assert_eq!(capacity_record.socket_grant_selector, None);
        capacity_sockets.invalidate_binding(authority.subject.binding());
        assert!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 2, &capacity_sockets).is_ok());
    }

    #[tokio::test]
    async fn document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable() {
        let mut state = test_state().await;
        let document_id = "open-plan-issue";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false));
        let token = seed_author_token(&state).await;
        let authorization = format!("Bearer {token}");
        let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let plan_route = format!("/spaces/{STUDIO}/documents/{document_id}/open-plan");
        let grant_route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
        let intent_body = |surface: &str, client: &str| {
            directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some(surface.into()), client_instance_id: client.into() })
        };
        let grant_body = |receipt: &str| directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
        let addr = spawn_server(state.clone()).await;
        let readiness = raw_http_get(addr, "/readyz", &[]).await;
        let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
        assert_eq!(readiness_json["features"]["openPlan"], true);
        assert_eq!(readiness_json["features"]["openPlanExchange"], true);

        let success = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:private").as_bytes()).await;
        assert_eq!(success.status, 200, "{}", String::from_utf8_lossy(&success.body));
        let success_text = String::from_utf8(success.body).expect("plan UTF-8");
        assert!(!success_text.contains(&token));
        assert!(!success_text.contains("client:private"));
        assert!(!success_text.contains("sessionId"));
        assert!(!success_text.contains("descriptor\""));
        let plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&success_text).expect("plan JSON");
        assert_eq!(plan.scope, scope);
        assert_eq!(plan.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
        assert_eq!(plan.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() });
        assert_eq!(plan.surface.surface_id, "surface.test.editor");
        assert!(plan.grant.write);
        assert!(plan.expires_at_unix_ms.saturating_sub(u64::try_from(now_ms()).expect("time")) <= DOCUMENT_OPEN_PLAN_MAX_TTL_MS);
        let exchange = raw_http_request(addr, "POST", &grant_route, &headers, grant_body(&plan.receipt).as_bytes()).await;
        assert_eq!(exchange.status, 200);
        let exchange_json: serde_json::Value = serde_json::from_slice(&exchange.body).expect("exchange JSON");
        assert_eq!(exchange_json["schema"], "semio.hub.socket-grant/v1");
        assert!(!String::from_utf8(exchange.body).expect("exchange UTF-8").contains(&plan.receipt));

        let foreign_surface = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.foreign", "client:foreign").as_bytes()).await;
        assert_eq!(foreign_surface.status, 503);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
        let hostile = format!(r#"{{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{{"spaceId":"{STUDIO}","documentId":"{document_id}"}},"clientInstanceId":"client","actor":"caller"}}"#);
        assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, hostile.as_bytes()).await.status, 400);
        assert_eq!(raw_http_request(addr, "POST", &format!("{plan_route}?surface=surface.test.editor"), &headers, intent_body("surface.test.editor", "client:query").as_bytes()).await.status, 400);
        let wrong_scope = directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: DocumentScope::new(STUDIO, "other"),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:scope".into(),
        });
        assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, wrong_scope.as_bytes()).await.status, 400);
        let mut oversized = intent_body("surface.test.editor", "client:oversized").into_bytes();
        oversized.resize(DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES + 1, b' ');
        let oversized_transport = raw_http_request_transport(addr, "POST", &plan_route, &headers, &oversized).await;
        assert!(oversized_transport.is_none_or(|response| response.status == 413));
        let oversized_request =
            axum::http::Request::builder().uri(&plan_route).header(axum::http::header::AUTHORIZATION, &authorization).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(oversized)).expect("oversized request");
        assert!(matches!(issue_document_open_plan(OriginalUri(plan_route.parse().expect("plan URI")), Path((STUDIO.into(), document_id.into())), State(state.clone()), oversized_request).await, Err((StatusCode::PAYLOAD_TOO_LARGE, _))));

        let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-issue-share").await.expect("share issue");
        let share_authorization = format!("Bearer {}", issued_share.capability.expose_once());
        let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
        let share_response = raw_http_request(addr, "POST", &plan_route, &share_headers, intent_body("surface.test.viewer", "client:share").as_bytes()).await;
        assert_eq!(share_response.status, 200);
        let share_plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&String::from_utf8(share_response.body).expect("share plan UTF-8")).expect("share plan");
        assert_eq!(share_plan.parent_dialect, plan.parent_dialect);
        assert!(!share_plan.grant.write);
        assert_eq!(share_plan.surface.role, os_directory::DocumentOpenSurfaceRoleV1::Viewer);
        assert!(share_plan.revalidation.session_generation.is_none());
        assert_eq!(share_plan.revalidation.share_generation, Some(1));
        assert_eq!(raw_http_request(addr, "POST", &grant_route, &share_headers, grant_body(&share_plan.receipt).as_bytes()).await.status, 200);

        let mut unavailable = state.clone();
        unavailable.openable_catalog = None;
        unavailable.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false));
        let unavailable_addr = spawn_server(unavailable).await;
        let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
        let unavailable_readiness: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
        assert_eq!(unavailable_readiness["features"]["openPlan"], false);
        assert_eq!(unavailable_readiness["features"]["openPlanExchange"], false);
        let unavailable_response = raw_http_request(unavailable_addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:unavailable").as_bytes()).await;
        assert_eq!(unavailable_response.status, 503);
        let unavailable_text = String::from_utf8(unavailable_response.body).expect("unavailable UTF-8");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&unavailable_text).expect("unavailable JSON")["code"], "catalog-unavailable");
        assert!(!unavailable_text.contains("fixture"));

        let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open fixture");
        let mut capacity_authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
        capacity_authority.catalog.generation_id = state.openable_catalog.as_ref().expect("catalog").generation_id().into();
        capacity_authority.package = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").package;
        capacity_authority.artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
        capacity_authority.surface = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").surface;
        capacity_authority.grant = DocumentOpenGrantV1 { read: true, write: true, observe: true };
        let capacity_now = u64::try_from(now_ms()).expect("capacity time");
        for index in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
            let mut authority = capacity_authority.clone();
            authority.scope.document_id = format!("capacity-{index}");
            authority.descriptor.document_id = authority.scope.document_id.clone();
            authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("capacity digest").0);
            authority.checkpoint = None;
            state.document_open_plans.issue_with_capability(authority, capacity_now, capacity_now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("fill plan capacity");
        }
        let capacity_response = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:capacity").as_bytes()).await;
        assert_eq!(capacity_response.status, 503);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&capacity_response.body).expect("capacity JSON")["code"], "deadline-exceeded");

        let mut cancelled = test_state().await;
        announce_document_for_test(&cancelled, STUDIO, "open-plan-cancelled").await;
        let cancelled_scope = DocumentScope::new(STUDIO, "open-plan-cancelled");
        let cancelled_descriptor = cancelled.directory.get_document_descriptor(&cancelled_scope).await.expect("cancel descriptor").expect("cancel document");
        cancelled.openable_catalog = Some(document_open_catalog_for_descriptor(&cancelled_descriptor));
        cancelled.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false));
        let cancelled_token = seed_author_token(&cancelled).await;
        let mut cancelled_headers = bearer_headers(&cancelled_token);
        cancelled_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
        cancelled.document_open_plan_issue_gate = Some(gate.clone());
        let cancelled_body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: cancelled_scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:cancelled".into(),
        }));
        let cancelled_state = cancelled.clone();
        let task = tokio::spawn(issue_document_open_plan_inner(STUDIO.into(), cancelled_scope.document_id, cancelled_headers, cancelled_state, cancelled_body));
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("issuer publication fence deadline").expect("issuer reached publication fence").forget();
        task.abort();
        assert!(matches!(task.await, Err(error) if error.is_cancelled()));
        assert!(cancelled.document_open_plans.inner.lock().expect("cancelled ledger").records.is_empty());
    }

    #[tokio::test]
    async fn document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint() {
        let mut state = test_state().await;
        let document_id = "open-plan-consume";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false));
        let token = seed_author_token(&state).await;

        let (_, surface_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:surface").await;
        let surface_capability = SocketGrantCapability::parse(&surface_grant.grant).expect("surface grant");
        let mut surface_headers = HeaderMap::new();
        surface_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", surface_grant.grant).parse().expect("surface protocol"));
        assert!(matches!(consume_socket_grant(&state, &surface_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.viewer")).await, Err(StatusCode::UNAUTHORIZED)));
        assert!(state.socket_grants.pending(&surface_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "surface substitution terminally rejects the pending grant");

        let (_, checkpoint_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
        let checkpoint_capability = SocketGrantCapability::parse(&checkpoint_grant.grant).expect("checkpoint grant");
        let mut checkpoint_headers = HeaderMap::new();
        checkpoint_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", checkpoint_grant.grant).parse().expect("checkpoint protocol"));
        publish_checkpoint_for_test(&state, STUDIO, document_id).await;
        assert!(matches!(consume_socket_grant(&state, &checkpoint_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.editor")).await, Err(StatusCode::UNAUTHORIZED)));
        assert!(state.socket_grants.pending(&checkpoint_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "revision/checkpoint change terminally rejects the pending grant");

        let (_, catalog_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
        let catalog_capability = SocketGrantCapability::parse(&catalog_grant.grant).expect("catalog grant");
        let mut catalog_headers = HeaderMap::new();
        catalog_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", catalog_grant.grant).parse().expect("catalog protocol"));
        state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
        assert!(matches!(consume_socket_grant(&state, &catalog_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.editor")).await, Err(StatusCode::UNAUTHORIZED)));
        assert!(state.socket_grants.pending(&catalog_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "catalog change terminally rejects the pending grant");

        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        let (_, exact_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:exact").await;
        let exact_capability = SocketGrantCapability::parse(&exact_grant.grant).expect("exact grant");
        let pending = state.socket_grants.pending(&exact_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("exact pending grant");
        let authority = pending.document_plan.as_ref().expect("retained plan authority");
        let mut hostile_descriptor = pending.clone();
        Arc::make_mut(hostile_descriptor.document_plan.as_mut().expect("descriptor authority")).descriptor_digest_v1 = "00".repeat(32);
        assert_eq!(document_plan_socket_validity(&state, &hostile_descriptor, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
        let mut hostile_revision = pending.clone();
        Arc::make_mut(hostile_revision.document_plan.as_mut().expect("revision authority")).revalidation.directory_revision += 1;
        assert_eq!(document_plan_socket_validity(&state, &hostile_revision, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
        let mut hostile_checkpoint = pending.clone();
        Arc::make_mut(hostile_checkpoint.document_plan.as_mut().expect("checkpoint authority")).checkpoint = None;
        assert_eq!(document_plan_socket_validity(&state, &hostile_checkpoint, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
        for field in ["artifactKind", "standard", "subset"] {
            let mut hostile = pending.clone();
            let dialect = &mut Arc::make_mut(hostile.document_plan.as_mut().expect("dialect authority")).parent_dialect;
            match field {
                "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
                "standard" => dialect.standard.push_str("-foreign"),
                "subset" => dialect.subset.push_str("-foreign"),
                _ => unreachable!(),
            }
            assert_eq!(document_plan_socket_validity(&state, &hostile, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized, "foreign parent {field}");
        }
        assert_eq!(authority.scope, scope);
        let mut exact_headers = HeaderMap::new();
        exact_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", exact_grant.grant).parse().expect("exact protocol"));
        let admission = consume_socket_grant(&state, &exact_headers, SocketAudienceV1::Document(scope), Some("surface.test.editor")).await.expect("exact current authority consumes");
        assert_eq!(admission.record.document_plan.as_deref(), Some(authority.as_ref()));
        eprintln!("[DEBUG] open-plan socket full parent dialect:3 substitutions denied; exact sealed selection retained");
    }

    #[tokio::test]
    async fn document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use() {
        let mut state = test_state().await;
        let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
        let document_id = "open-plan-route";
        let foreign_document_id = "open-plan-route-foreign";
        announce_document_for_test(&state, STUDIO, document_id).await;
        announce_document_for_test(&state, STUDIO, foreign_document_id).await;
        let route_descriptor = state.directory.get_document_descriptor(&DocumentScope::new(STUDIO, document_id)).await.expect("route descriptor").expect("route document");
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&route_descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false));
        let token = seed_author_token(&state).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let mut authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
        let addr = spawn_server(state.clone()).await;
        let route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
        let authorization = format!("Bearer {token}");
        let request_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let request_body = |receipt: &str| directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
        let readiness = raw_http_get(addr, "/readyz", &[]).await;
        let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
        assert_eq!(readiness_json["features"]["openPlan"], true, "verified catalog-backed plan issuance is advertised with exchange");
        assert_eq!(readiness_json["features"]["openPlanExchange"], true);

        let now = u64::try_from(now_ms()).expect("nonnegative route time");
        let public = state.document_open_plans.issue_with_capability(authority.clone(), now, now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(40))).expect("private issuer fixture");
        let success = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
        assert_eq!(success.status, 200);
        let encoded_success = String::from_utf8(success.body.clone()).expect("success UTF-8");
        assert!(!encoded_success.contains(&public.receipt));
        assert!(!encoded_success.contains(&authority.descriptor_digest_v1));
        if let SocketSubjectV1::Session { session_id, user_id, .. } = &authority.subject {
            assert!(!encoded_success.contains(session_id));
            assert!(!encoded_success.contains(user_id));
        }
        let success_json: serde_json::Value = serde_json::from_slice(&success.body).expect("socket grant JSON");
        assert_eq!(success_json["schema"], "semio.hub.socket-grant/v1");
        assert_eq!(success_json["protocol"], SOCKET_PROTOCOL_V1);
        assert_eq!(success_json["actorId"], authority.server_actor_id);
        let socket_capability = SocketGrantCapability::parse(success_json["grant"].as_str().expect("socket grant")).expect("socket grant grammar");
        let pending = state.socket_grants.pending(&socket_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("route-bound pending grant");
        assert_eq!(pending.subject, authority.subject);
        assert_eq!(pending.document_plan.as_deref(), Some(&authority));

        let replay = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
        assert_eq!(replay.status, 409);
        let replay_json: serde_json::Value = serde_json::from_slice(&replay.body).expect("replay error JSON");
        assert_eq!(replay_json, serde_json::json!({ "schema": "semio.hub.document-open-plan-error/v1", "code": "already-consumed" }));
        assert!(!String::from_utf8(replay.body).expect("replay UTF-8").contains(&public.receipt));

        let foreign = issue_test_session(&state, "open-plan-route-foreign@example.com").await;
        upsert_member_for_test(&state, STUDIO, "open-plan-route-foreign@example.com", DirectorySpaceRole::Author).await;
        let current_revision = state.directory.head_seq().await.expect("current directory revision");
        authority.revalidation.directory_revision = current_revision;
        authority.revalidation.membership_generation = current_revision;
        let foreign_authorization = format!("Bearer {}", foreign.token);
        let foreign_headers = [("Authorization", foreign_authorization.as_str()), ("Content-Type", "application/json")];
        let bound = state.document_open_plans.issue_with_capability(authority.clone(), now + 1, now + 10_001, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(41))).expect("binding fixture plan");
        let wrong_binding = raw_http_request(addr, "POST", &route, &foreign_headers, request_body(&bound.receipt).as_bytes()).await;
        assert_eq!(wrong_binding.status, 401);
        let wrong_binding_json: serde_json::Value = serde_json::from_slice(&wrong_binding.body).expect("binding error JSON");
        assert_eq!(wrong_binding_json["code"], "denied");
        let correct_after_foreign = raw_http_request(addr, "POST", &route, &request_headers, request_body(&bound.receipt).as_bytes()).await;
        assert_eq!(correct_after_foreign.status, 200, "foreign authentication cannot consume the exact receipt");

        let scoped = state.document_open_plans.issue_with_capability(authority.clone(), now + 2, now + 10_002, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(42))).expect("scope fixture plan");
        let foreign_route = format!("/spaces/{STUDIO}/documents/{foreign_document_id}/socket-grants");
        let wrong_scope = raw_http_request(addr, "POST", &foreign_route, &request_headers, request_body(&scoped.receipt).as_bytes()).await;
        assert_eq!(wrong_scope.status, 401);
        assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&scoped.receipt).as_bytes()).await.status, 200, "foreign path cannot consume the exact receipt");

        let strict = state.document_open_plans.issue_with_capability(authority.clone(), now + 3, now + 10_003, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(43))).expect("strict request fixture plan");
        let hostile = format!(r#"{{"schema":"semio.hub.document-plan-socket-grant-intent/v1","version":1,"planReceipt":"{}","actor":"caller-selected"}}"#, strict.receipt);
        let unknown_field = raw_http_request(addr, "POST", &route, &request_headers, hostile.as_bytes()).await;
        assert_eq!(unknown_field.status, 400);
        assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&strict.receipt).as_bytes()).await.status, 200, "rejected unknown authority field cannot consume the receipt");

        let query_plan = state.document_open_plans.issue_with_capability(authority.clone(), now + 4, now + 10_004, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(44))).expect("query fixture plan");
        assert_eq!(raw_http_request(addr, "POST", &format!("{route}?receipt=forbidden"), &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 400);
        assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 200, "query rejection cannot consume the body receipt");

        let bounded = state.document_open_plans.issue_with_capability(authority.clone(), now + 5, now + 10_005, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(45))).expect("bounded request fixture plan");
        let mut oversized = request_body(&bounded.receipt).into_bytes();
        oversized.resize(DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES + 1, b' ');
        let oversized_response = raw_http_request(addr, "POST", &route, &request_headers, &oversized).await;
        assert_eq!(oversized_response.status, 413);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&oversized_response.body).expect("bounded error JSON")["code"], "denied");
        assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&bounded.receipt).as_bytes()).await.status, 200, "oversized body cannot consume the receipt");

        let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-route-share").await.expect("share issue");
        let share_token = issued_share.capability.expose_once();
        let mut share_authority = authority.clone();
        share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id, selector: issued_share.record.selector, scope: scope.clone(), expires_at_ms: issued_share.record.expires_at };
        share_authority.revalidation.session_generation = None;
        share_authority.revalidation.share_generation = Some(1);
        let share_selection = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&share_authority.descriptor, Some("surface.test.viewer"), false).expect("share selection");
        share_authority.package = share_selection.package;
        share_authority.artifact = share_selection.artifact;
        share_authority.parent_dialect = share_selection.parent_dialect;
        share_authority.surface = share_selection.surface;
        share_authority.grant = share_selection.grant;
        share_authority.server_actor_id = socket_actor_id(&[0x44; 32], false);
        share_authority.validate().expect("share route authority");
        let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), now + 6, now + 10_006, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(46))).expect("share fixture plan");
        let share_authorization = format!("Bearer {share_token}");
        let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
        let share_response = raw_http_request(addr, "POST", &route, &share_headers, request_body(&share_plan.receipt).as_bytes()).await;
        assert_eq!(share_response.status, 200);
        let share_json: serde_json::Value = serde_json::from_slice(&share_response.body).expect("share socket grant JSON");
        assert_eq!(share_json["actorId"], share_authority.server_actor_id);
        let share_socket = SocketGrantCapability::parse(share_json["grant"].as_str().expect("share grant")).expect("share grant grammar");
        let share_pending = state.socket_grants.pending(&share_socket, &SocketAudienceV1::Document(scope), now_ms()).expect("share pending grant");
        assert_eq!(share_pending.document_plan.as_deref(), Some(&share_authority));
        assert!(!share_pending.document_plan.as_ref().expect("share plan").grant.write);

        let invalid_receipt = request_body("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB");
        let invalid = raw_http_request(addr, "POST", &route, &request_headers, invalid_receipt.as_bytes()).await;
        assert_eq!(invalid.status, 400);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&invalid.body).expect("invalid error JSON")["code"], "denied");
    }

    #[test]
    fn document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes() {
        DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations").clear();
        let mut hostile = "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA".to_string();
        hostile.pop();
        hostile.push('!');
        DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(true));
        let result = DocumentOpenPlanCapabilityV1::parse(&hostile);
        DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(false));
        assert!(matches!(result, Err(DocumentOpenPlanErrorCodeV1::Denied)));
        let observations = std::mem::take(&mut *DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations"));
        assert_eq!(observations, vec![DocumentOpenPlanDecodeWipeObservationV1 { nonzero_before: 31, after: [0; 32] }]);
    }

    #[tokio::test]
    async fn document_open_plan_admin_revocation_invalidates_session_and_share_bindings() {
        let state = test_state().await;
        let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
        let principal = AdminPrincipalV1 {
            user_id: "open-plan-admin".into(),
            auth_session_id: "open-plan-admin-session".into(),
            authorization_generation: 1,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: i64::MAX,
            correlation_id: "open-plan-admin-revocation".into(),
            peer_class: "test",
        };

        let session = issue_test_session(&state, "open-plan-revoked-session@example.com").await;
        let session_capability = SessionCapability::parse(&session.token).expect("session capability");
        let session_record = state.directory.authenticate_session(&session_capability).await.expect("session lookup").expect("active session");
        let session_now = u64::try_from(now_ms()).expect("nonnegative session time");
        let mut session_authority = document_open_plan_test_authority(&fixture);
        session_authority.revalidation.session_generation = Some(session_record.authorization_generation);
        session_authority.subject = SocketSubjectV1::Session {
            session_id: session_record.id.clone(),
            user_id: session_record.user_id.clone(),
            authorization_generation: session_record.authorization_generation,
            role: Some(SpaceRole::Author),
            expires_at_ms: session_record.expires_at,
        };
        let session_plan = state.document_open_plans.issue_with_capability(session_authority.clone(), session_now, session_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(20))).expect("session plan");
        let session_revoke = execute_admin_intent(&state, &principal, AdminIntentV1::RevokeUserSessions { request_id: "request:open-plan-session-revoke".into(), user_id: session_record.user_id, reason_code: "test-revoke".into() }, None).await;
        assert_eq!(session_revoke.phase, "succeeded");
        assert_eq!(state.document_open_plans.exchange(&session_plan.receipt, &session_authority, session_now + 1, "socket-after-session-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));

        let issued_share = state.directory.issue_share_token(&fixture.valid_plan.scope, 60, "open-plan-share").await.expect("share issue");
        let mut share_authority = document_open_plan_test_authority(&fixture);
        share_authority.grant.write = false;
        share_authority.surface.role = os_directory::DocumentOpenSurfaceRoleV1::Viewer;
        share_authority.revalidation.session_generation = None;
        share_authority.revalidation.share_generation = Some(1);
        share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id.clone(), selector: issued_share.record.selector.clone(), scope: issued_share.record.scope.clone(), expires_at_ms: issued_share.record.expires_at };
        let share_now = u64::try_from(now_ms()).expect("nonnegative share time");
        let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), share_now, share_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(21))).expect("share plan");
        let share_revoke = execute_admin_intent(
            &state,
            &principal,
            AdminIntentV1::RevokeDocumentShare { request_id: "request:open-plan-share-revoke".into(), scope: issued_share.record.scope, share_id: issued_share.record.id, reason_code: "test-revoke".into() },
            None,
        )
        .await;
        assert_eq!(share_revoke.phase, "succeeded");
        assert_eq!(state.document_open_plans.exchange(&share_plan.receipt, &share_authority, share_now + 1, "socket-after-share-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    }

    #[test]
    fn socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live() {
        run_socket_test(|| async {
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-a").await;
            announce_document_for_test(&state, STUDIO, "socket-b").await;
            let nonmember = issue_test_session(&state, "socket-nonmember@example.com").await;
            let unauthorized_existing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
            let unauthorized_missing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-missing".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
            assert_eq!(unauthorized_existing, Some(StatusCode::UNAUTHORIZED));
            assert_eq!(unauthorized_missing, unauthorized_existing, "unauthorized callers cannot enumerate descriptor existence");
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue document socket grant").0;
            assert_eq!(receipt.schema, "semio.hub.socket-grant/v1");
            assert_eq!(receipt.protocol, SOCKET_PROTOCOL_V1);
            assert_eq!(receipt.grant.len(), 107);
            assert!(receipt.grant.starts_with("socket.v1."));
            assert!(receipt.actor_id.starts_with("hub.v1."));
            assert!(!receipt.actor_id.contains(receipt.grant.rsplit('.').next().expect("secret")));

            let addr = spawn_server(state.clone()).await;
            let rejected = connect_async(socket_request(&format!("ws://{addr}/spaces/{STUDIO}/documents/socket-b/socket/v1"), &receipt.grant)).await.expect_err("cross-document grant rejected");
            assert!(matches!(rejected, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));

            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-a/socket/v1");
            let (mut socket, response) = connect_async(socket_request(&url, &receipt.grant)).await.expect("upgrade socket grant");
            assert_eq!(response.headers().get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()), Some(SOCKET_PROTOCOL_V1));
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-a"));
            let mut forged = sample_envelope("forged-actor", &document).await;
            forged.actor = ActorId("client-selected-forgery".into());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 77, envelopes: vec![forged] }, Lane::Command).await).await.expect("forged command");
            match next_server_frame(&mut socket).await {
                ServerFrame::Ack { batch_id: 77, stages, .. } => match &stages[0] {
                    AckStage::Applied { outcome } => match outcome.as_ref() {
                        ApplyOutcome::Rejected { reason, .. } => {
                            assert_eq!(reason, "socket subject actor mismatch");
                            assert!(!reason.contains(&receipt.grant));
                        }
                        other => panic!("forged actor was not rejected: {other:?}"),
                    },
                    other => panic!("unexpected forged actor stage: {other:?}"),
                },
                other => panic!("expected forged actor ack, got {other:?}"),
            }

            let legacy_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue legacy-carrier rejection grant").0;
            let (mut legacy, _) = connect_async(socket_request(&url, &legacy_receipt.grant)).await.expect("legacy rejection socket");
            legacy.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("initial socket hello");
            assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Session { .. }));
            legacy.send(WsMessage::Binary(vec![0, 0].into())).await.expect("legacy tag-zero frame");
            assert_eq!(next_close_code(&mut legacy, false).await, 4401, "v1 rejects the legacy actor/token carrier after upgrade");

            let replay = connect_async(socket_request(&url, &receipt.grant)).await.expect_err("consumed grant replay rejected");
            assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
            let pending = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue pending grant").0;
            assert_eq!(pending.actor_id, receipt.actor_id, "session-derived actor is stable across grants");

            assert_eq!(delete_session_me(bearer_headers(&token), State(state.clone())).await, StatusCode::NO_CONTENT);
            assert_eq!(next_close_code(&mut socket, false).await, 4401, "successful durable revoke immediately invalidates a live socket");
            let revoked_pending = connect_async(socket_request(&url, &pending.grant)).await.expect_err("pending grant invalidated by revoke");
            assert!(matches!(revoked_pending, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
        });
    }

    #[test]
    fn socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-linearized").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-linearized".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-linearized/socket/v1");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_before_welcome.acquire()).await.expect("pre-welcome gate deadline").expect("pre-welcome gate");
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = token.clone();
                async move { delete_session_me(bearer_headers(&token), State(state)).await }
            });
            assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "revoke waits while Welcome owns the binding linearization");
            gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }), "Welcome linearizes before the waiting revoke");
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_after_welcome.acquire()).await.expect("post-Welcome boundary deadline").expect("post-Welcome boundary");
            assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("bounded revoke completion").expect("revoke task"), StatusCode::NO_CONTENT);
            gate.socket_bootstrap_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a revoke winning after Welcome suppresses bootstrap and Session authority");
        });
    }

    #[test]
    fn socket_grant_revoke_before_command_admission_has_no_storage_effect() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-command-revoke").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-command-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-command-revoke/socket/v1");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
            live_gate.document_release.add_permits(1);

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-command-revoke"));
            let mut accepted = sample_envelope("accepted-op", &document).await;
            accepted.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 90, envelopes: vec![accepted] }, Lane::Command).await).await.expect("control command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("control command boundary deadline").expect("control command boundary");
            live_gate.socket_command_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Ack { batch_id: 90, .. }));
            let accepted_frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("accepted frontier");
            assert_eq!(accepted_frontier.head_seq, 1, "an actor-matching command persists while authorized");

            let mut revoked = sample_envelope("revoked-op", &document).await;
            revoked.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 91, envelopes: vec![revoked] }, Lane::Command).await).await.expect("revoked command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary");
            assert_eq!(delete_session_me(bearer_headers(&token), State(state.clone())).await, StatusCode::NO_CONTENT);
            live_gate.socket_command_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "no Ack crosses a revoke that wins before command admission");
            let frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier");
            assert_eq!(frontier.head_seq, 1, "the revoked actor-matching command never reaches durable storage");
        });
    }

    #[test]
    fn socket_grant_revoke_before_lag_authorization_reads_no_private_control() {
        run_socket_test(|| async {
            let mut state = test_state_with_capacity(1024, 1).await;
            let live_gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-lag-revoke").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-lag-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-lag-revoke/socket/v1");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
            let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-lag-revoke")));
            fanout.send(ServerFrame::Presence { peers: vec![b"first".to_vec()] }).expect("first fanout");
            fanout.send(ServerFrame::Presence { peers: vec![b"second".to_vec()] }).expect("second fanout");
            live_gate.document_release.add_permits(1);
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_lag_received.acquire()).await.expect("lag boundary deadline").expect("lag boundary");
            assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
            live_gate.socket_lag_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "revoked lag path discloses no control frame");
            assert_eq!(live_gate.socket_rebootstrap_read.available_permits(), 0, "revoked lag path never enters the private checkpoint/control read");
        });
    }

    #[test]
    fn socket_grant_revoke_before_broadcast_authorization_suppresses_frame() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-broadcast-revoke").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-broadcast-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-broadcast-revoke/socket/v1");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
            let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-broadcast-revoke")));
            fanout.send(ServerFrame::Presence { peers: vec![b"private-presence".to_vec()] }).expect("fanout");
            live_gate.document_release.add_permits(1);
            tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_broadcast_received.acquire()).await.expect("broadcast boundary deadline").expect("broadcast boundary");
            assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
            live_gate.socket_broadcast_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a broadcast received before a winning revoke is never disclosed afterward");
        });
    }

    #[test]
    fn socket_grant_directory_route_uses_credential_free_hello_and_revokes_live() {
        run_socket_test(|| async {
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone())).await.expect("issue directory socket grant").0;
            let since = state.directory.head_seq().await.expect("directory head");
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/directory/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free hello");
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#directory-socket-law".into() }, DirectoryCommand::RenameSpace { space_id: STUDIO.into(), name: "Socket Grant Studio".into() })
                .await
                .expect("member-visible directory event");
            assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event.space_id.as_deref() == Some(STUDIO)));
            assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
            assert_eq!(next_close_code(&mut socket, false).await, 4401);
        });
    }

    #[tokio::test]
    async fn scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership() {
        let ledger = SocketGrantLedgerV1::default();
        let scope = DocumentScope::new("space-a", "document-a");
        let audience = SocketAudienceV1::DirectoryScoped(scope.clone());
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Spectator), expires_at_ms: 10_000 };
        let capability = SocketGrantCapability::mint().expect("scoped capability");
        ledger.issue(&capability, audience.clone(), "hub.v1.scoped".into(), subject.clone(), 1, 9_000).expect("scoped issue");
        let pending = ledger.pending(&capability, &audience, 2).expect("pending scoped grant");
        assert_eq!(pending.bindings(), vec![SocketBindingKeyV1::User("user-a".into()), SocketBindingKeyV1::Session("session-a".into()), SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() },]);
        let consumed = ledger.consume(&pending, 3).expect("consume scoped grant");
        let (live_id, notify) = ledger.register_live(&consumed).expect("register scoped live lease");
        assert!(ledger.is_live(&consumed, &live_id));
        ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
        assert!(!ledger.is_live(&consumed, &live_id));
        tokio::time::timeout(std::time::Duration::from_millis(50), notify.notified()).await.expect("membership invalidation notifies once");

        let later = SocketGrantCapability::mint().expect("later scoped capability");
        ledger.issue(&later, audience.clone(), "hub.v1.later".into(), subject, 4, 9_000).expect("later issue");
        ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
        assert!(ledger.pending(&later, &audience, 5).is_err(), "membership invalidation also removes pending grants");
    }

    #[test]
    fn scoped_directory_socket_message_matching_is_body_exact_and_removal_private() {
        let scope = DocumentScope::new("space-a", "document-a");
        let event = |body: os_directory::DirectoryEventBody| DirectoryStreamMessage::Event {
            event: DirectoryEvent {
                seq: 1,
                id: "event-a".into(),
                hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
                actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
                space_id: Some("space-a".into()),
                user_id: None,
                body,
                recorded_at_ms: 1,
            },
        };
        assert!(directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-a") })));
        assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-b") })));
        assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::MemberRemoved { space_id: "space-a".into(), user_id: "user-a".into() })));
        assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Heartbeat { head_seq: 99 }));
        assert!(directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-a".into(), actors: Vec::new() }));
        assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-b".into(), actors: Vec::new() }));
    }

    #[test]
    fn scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event() {
        run_socket_test(|| async {
            let state = test_state().await;
            let owner_token = seed_author_token(&state).await;
            let member = issue_test_session(&state, "scoped-member@example.com").await;
            upsert_member_for_test(&state, STUDIO, "scoped-member@example.com", DirectorySpaceRole::Spectator).await;
            announce_document_for_test(&state, STUDIO, "scoped-document-a").await;
            announce_document_for_test(&state, STUDIO, "scoped-document-b").await;
            let unaffected = issue_test_session(&state, "scoped-unaffected@example.com").await;
            let unaffected_space = create_space_for_test(&state, &unaffected.user_id, "Scoped Unaffected", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            announce_document_for_test(&state, &unaffected_space, "scoped-unaffected-document").await;
            let addr = spawn_server(state.clone()).await;
            let authorization = format!("Bearer {}", member.token);
            let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-document-a/socket-grants");
            let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
            assert_eq!(issued.status, 200);
            let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
            let grant = receipt["grant"].as_str().expect("scoped grant");
            let substituted = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-b/socket/v1?since=0");
            let error = connect_async(socket_request(&substituted, grant)).await.expect_err("scope substitution rejected before upgrade");
            assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));

            let since = state.directory.head_seq().await.expect("directory head");
            let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-a/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("exact scoped socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
            let unaffected_authorization = format!("Bearer {}", unaffected.token);
            let unaffected_issue_path = format!("/directory/spaces/{unaffected_space}/documents/scoped-unaffected-document/socket-grants");
            let unaffected_issued = raw_http_request(addr, "POST", &unaffected_issue_path, &[("Authorization", &unaffected_authorization)], &[]).await;
            assert_eq!(unaffected_issued.status, 200);
            let unaffected_receipt: serde_json::Value = serde_json::from_slice(&unaffected_issued.body).expect("unaffected scoped grant JSON");
            let unaffected_grant = unaffected_receipt["grant"].as_str().expect("unaffected scoped grant");
            let unaffected_url = format!("ws://{addr}/directory/spaces/{unaffected_space}/documents/scoped-unaffected-document/socket/v1?since={since}");
            let (mut unaffected_socket, _) = connect_async(socket_request(&unaffected_url, unaffected_grant)).await.expect("unaffected scoped socket");
            unaffected_socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("unaffected scoped hello");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
            assert_eq!(pending.status, 200);
            let pending_receipt: serde_json::Value = serde_json::from_slice(&pending.body).expect("pending scoped grant JSON");
            let pending_grant = pending_receipt["grant"].as_str().expect("pending scoped grant").to_string();
            announce_document_for_test(&state, STUDIO, "scoped-document-c").await;
            assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket.next()).await.is_err(), "same-space foreign document never serializes");

            let command = DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id: member.user_id.clone() };
            let body = directory::os_pack::json::to_json_string(&command);
            let owner_authorization = format!("Bearer {owner_token}");
            let removed = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &owner_authorization), ("Content-Type", "application/json")], body.as_bytes()).await;
            assert_eq!(removed.status, 202);
            let removed_body = String::from_utf8(removed.body).expect("remove response UTF-8");
            assert!(removed_body.contains("member.removed"));
            assert_eq!(next_close_code(&mut socket, false).await, 4401, "durable removal invalidates the scoped lease without exposing its event");
            announce_document_for_test(&state, &unaffected_space, "scoped-unaffected-document").await;
            assert!(
                matches!(
                    next_directory_message(&mut unaffected_socket).await,
                    DirectoryStreamMessage::Event { event }
                        if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { descriptor }
                            if descriptor.space_id == unaffected_space && descriptor.document_id == "scoped-unaffected-document")
                ),
                "another user's exact scoped subscription remains live"
            );

            for stale in [grant.to_string(), pending_grant] {
                let error = connect_async(socket_request(&url, &stale)).await.expect_err("consumed or pending pre-removal grant remains invalid");
                assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));
            }

            let denied = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
            assert_eq!(denied.status, 401, "removed member cannot reacquire the scoped grant");
        });
    }

    #[test]
    fn scoped_directory_socket_admin_removal_uses_the_same_membership_fence() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let mut admin_headers = authorize_test_admin(&mut state, "scoped-admin@example.com").await;
            admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
            let member = issue_test_session(&state, "scoped-admin-target@example.com").await;
            upsert_member_for_test(&state, STUDIO, "scoped-admin-target@example.com", DirectorySpaceRole::Spectator).await;
            announce_document_for_test(&state, STUDIO, "scoped-admin-document").await;
            let addr = spawn_server(state.clone()).await;
            let authorization = format!("Bearer {}", member.token);
            let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket-grants");
            let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
            assert_eq!(issued.status, 200);
            let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
            let grant = receipt["grant"].as_str().expect("scoped grant");
            let since = state.directory.head_seq().await.expect("directory head");
            let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("admin target scoped socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;

            let intent = AdminIntentV1::RemoveSpaceMember { request_id: "request:scoped-admin-removal".into(), space_id: STUDIO.into(), user_id: member.user_id };
            let body = Bytes::from(directory::os_pack::json::to_json_string(&intent));
            let (status, receipt) = admin_intents(admin_headers, loopback_peer(), State(state), body).await.expect("admin removal response");
            assert_eq!(status, StatusCode::OK);
            assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
            assert_eq!(next_close_code(&mut socket, false).await, 4401, "admin removal uses the same no-event membership fence");
        });
    }

    #[test]
    fn scoped_directory_socket_removal_and_delivery_have_one_total_membership_order() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let member = issue_test_session(&state, "scoped-order-target@example.com").await;
            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
            let addr = spawn_server(state.clone()).await;
            let authorization = format!("Bearer {}", member.token);
            let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-order-document/socket-grants");

            let open = |grant: String, since: u64| {
                let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-order-document/socket/v1?since={since}");
                async move {
                    let (mut socket, _) = connect_async(socket_request(&url, &grant)).await.expect("ordered scoped socket");
                    socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("ordered scoped hello");
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    socket
                }
            };
            let issue = || {
                let issue_path = issue_path.clone();
                let authorization = authorization.clone();
                async move {
                    let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
                    assert_eq!(issued.status, 200);
                    let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("ordered scoped grant JSON");
                    receipt["grant"].as_str().expect("ordered scoped grant").to_string()
                }
            };

            let mut removal_wins = open(issue().await, state.directory.head_seq().await.expect("removal-wins head")).await;
            gate.socket_membership_remove_enabled.store(true, std::sync::atomic::Ordering::Release);
            let mut removal = tokio::spawn({
                let state = state.clone();
                let user_id = member.user_id.clone();
                async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-removal".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_membership_remove_admitted.acquire()).await.expect("removal admission deadline").expect("removal admission");
            gate.socket_scoped_send_mode.store(1, std::sync::atomic::Ordering::Release);
            announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_scoped_send_admitted.acquire()).await.expect("removal-wins sender deadline").expect("removal-wins sender");
            gate.socket_scoped_send_release.add_permits(1);
            gate.socket_membership_remove_release.add_permits(1);
            tokio::time::timeout(std::time::Duration::from_secs(2), &mut removal).await.expect("removal-wins completion deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut removal_wins, false).await, 4401, "removal winning the membership gate exposes no scoped event");

            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            gate.socket_membership_remove_enabled.store(false, std::sync::atomic::Ordering::Release);
            gate.socket_scoped_send_mode.store(0, std::sync::atomic::Ordering::Release);
            let mut delivery_wins = open(issue().await, state.directory.head_seq().await.expect("delivery-wins head")).await;
            gate.socket_scoped_send_mode.store(2, std::sync::atomic::Ordering::Release);
            announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_scoped_send_admitted.acquire()).await.expect("delivery-wins sender deadline").expect("delivery-wins sender");
            let mut removal = tokio::spawn({
                let state = state.clone();
                let user_id = member.user_id.clone();
                async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-delivery".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
            });
            assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut removal).await.is_err(), "removal waits while an admitted scoped send owns the membership gate");
            gate.socket_scoped_send_release.add_permits(1);
            assert!(matches!(
                next_directory_message(&mut delivery_wins).await,
                DirectoryStreamMessage::Event { event }
                    if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { descriptor }
                        if descriptor.space_id == STUDIO && descriptor.document_id == "scoped-order-document")
            ));
            tokio::time::timeout(std::time::Duration::from_secs(2), removal).await.expect("delivery-wins removal deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut delivery_wins, false).await, 4401, "the one admitted event precedes the terminal membership close");
        });
    }

    #[test]
    fn socket_directory_revoke_after_admission_suppresses_replay_without_deadlock() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let token = seed_author_token(&state).await;
            let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone())).await.expect("issue directory grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/directory/socket/v1?since=0");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_directory_admitted.acquire()).await.expect("directory admission deadline").expect("directory admission");
            assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), delete_session_me(bearer_headers(&token), State(state))).await.expect("bounded revoke"), StatusCode::NO_CONTENT,);
            gate.socket_directory_release.add_permits(1);
            assert_eq!(next_close_code(&mut socket, false).await, 4401, "no replay text crosses a winning revoke");
        });
    }

    #[tokio::test]
    async fn socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke() {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let mut admin_headers = authorize_test_admin(&mut state, "socket-admin@example.com").await;
        admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let target = issue_test_session(&state, "socket-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "socket-target@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, STUDIO, "socket-admin-race").await;
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let user_id = target.user_id.clone();
            async move {
                let intent = AdminIntentV1::RevokeUserSessions { request_id: "request:socket-admin-revoke".into(), user_id, reason_code: "test-revoke".into() };
                let body = Bytes::from(directory::os_pack::json::to_json_string(&intent));
                admin_intents(admin_headers, loopback_peer(), State(state), body).await
            }
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_admin_revoke_admitted.acquire()).await.expect("admin gate deadline").expect("admin gate");
        let mut issue = tokio::spawn({
            let state = state.clone();
            let token = target.token.clone();
            async move { issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-admin-race".to_string())), bearer_headers(&token), State(state)).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut issue).await.is_err(), "same-user grant waits behind batch revoke");
        gate.socket_admin_revoke_release.add_permits(1);
        let (status, receipt) = tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("bounded admin revoke").expect("admin task").expect("admin response");
        assert_eq!(status, StatusCode::OK);
        assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
        let late_issue = tokio::time::timeout(std::time::Duration::from_secs(2), issue).await.expect("bounded late issue").expect("issue task");
        assert!(matches!(late_issue, Err(StatusCode::UNAUTHORIZED)), "revoked session cannot mint");
    }

    #[tokio::test]
    async fn socket_directory_visibility_requires_membership_even_for_public_spaces() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let capability = SessionCapability::parse(&token).expect("session capability");
        let session = state.directory.authenticate_session(&capability).await.expect("authenticate session").expect("active session");
        let other = issue_test_session(&state, "public-owner@example.com").await;
        let public_space = create_space_for_test(&state, &other.user_id, "Public Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let event = state.directory.events_since(0, 100).await.expect("directory events").into_iter().find(|event| event.space_id.as_deref() == Some(public_space.as_str())).expect("public-space event");
        let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
        let record = SocketGrantRecordV1 {
            selector: "visibility".into(),
            secret_digest: [0; 32],
            audience,
            actor_id: "hub.v1.visibility".into(),
            subject: SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at },
            document_plan: None,
            issued_at_ms: session.issued_at,
            expires_at_ms: session.expires_at,
            state: SocketGrantStateV1::Consumed,
        };
        assert_eq!(socket_directory_message_visible(&state, &record, &DirectoryStreamMessage::Event { event }).await, SocketBindingValidityV1::Unauthorized);
    }

    fn assert_public_projection_has_no_private_keys(value: &serde_json::Value) {
        const FORBIDDEN: &[&str] = &[
            "ownerUserId",
            "role",
            "activeConnections",
            "connections",
            "presence",
            "members",
            "invites",
            "email",
            "userId",
            "displayName",
            "actor",
            "hlc",
            "cursor",
            "headSeq",
            "commitSeq",
            "epoch",
            "bootstrapVersion",
            "bootstrapFrontier",
            "bootstrapSnapshotHash",
            "checkpointId",
            "storageKey",
        ];
        match value {
            serde_json::Value::Object(entries) => {
                for (key, value) in entries {
                    assert!(!FORBIDDEN.contains(&key.as_str()), "public projection disclosed forbidden key {key}");
                    assert_public_projection_has_no_private_keys(value);
                }
            }
            serde_json::Value::Array(entries) => entries.iter().for_each(assert_public_projection_has_no_private_keys),
            _ => {}
        }
    }

    #[test]
    fn space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404() {
        run_socket_test(|| async {
            let state = test_state().await;
            let author = issue_test_session(&state, "public-author@example.invalid").await;
            let spectator = issue_test_session(&state, "public-spectator@example.invalid").await;
            let outsider = issue_test_session(&state, "public-outsider@example.invalid").await;
            let public_space = create_space_for_test(&state, &author.user_id, "Discoverable", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
            upsert_member_for_test(&state, &public_space, "public-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
            announce_document_for_test(&state, &public_space, "catalog-document").await;
            state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#route-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: public_space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
                .await
                .expect("author invite fixture");
            let private_space = create_space_for_test(&state, &author.user_id, "Private", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let addr = spawn_server(state).await;

            let anonymous = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[]).await;
            assert_eq!(anonymous.status, 200);
            let anonymous: serde_json::Value = serde_json::from_slice(&anonymous.body).expect("anonymous public detail JSON");
            assert_eq!(anonymous["access"], "public");
            assert_eq!(anonymous["space"]["visibility"], "public");
            assert_eq!(anonymous["documents"][0]["documentId"], "catalog-document");
            assert!(anonymous["documents"][0].get("descriptor").is_none());
            assert_public_projection_has_no_private_keys(&anonymous);

            let outsider_authorization = format!("Bearer {}", outsider.token);
            let public_nonmember = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
            assert_eq!(public_nonmember.status, 200);
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&public_nonmember.body).expect("nonmember public detail"), anonymous);
            let private_nonmember = raw_http_get(addr, &format!("/directory/spaces/{private_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
            assert_eq!(private_nonmember.status, 404);

            let public_document_status = format!("/spaces/{public_space}/documents/catalog-document");
            assert_eq!(raw_http_get(addr, &public_document_status, &[]).await.status, 401, "public discovery is not document-currentness authority");
            assert_eq!(raw_http_get(addr, &public_document_status, &[("Authorization", outsider_authorization.as_str())]).await.status, 401);
            let blob = format!("/spaces/{public_space}/blobs/{}", "11".repeat(32));
            assert_eq!(raw_http_request(addr, "GET", &blob, &[], &[]).await.status, 401, "public discovery is not blob read authority");
            assert_eq!(raw_http_request(addr, "HEAD", &blob, &[], &[]).await.status, 401, "public discovery is not blob existence authority");
            assert_eq!(raw_http_request(addr, "PUT", &blob, &[], b"private").await.status, 401, "public discovery is not blob write authority");
            assert_eq!(raw_http_request(addr, "GET", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
            assert_eq!(raw_http_request(addr, "HEAD", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
            assert_eq!(raw_http_request(addr, "PUT", &blob, &[("Authorization", outsider_authorization.as_str())], b"private").await.status, 401);

            let spectator_authorization = format!("Bearer {}", spectator.token);
            let member = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", spectator_authorization.as_str())]).await;
            assert_eq!(member.status, 200);
            let member: serde_json::Value = serde_json::from_slice(&member.body).expect("member detail");
            assert_eq!(member["access"], "member");
            assert!(member.get("members").is_some());
            assert!(member.get("invites").is_none());
            assert_eq!(member["documents"][0]["headSeq"], 0);

            let author_authorization = format!("Bearer {}", author.token);
            let authored = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await;
            assert_eq!(authored.status, 200);
            let authored: serde_json::Value = serde_json::from_slice(&authored.body).expect("author detail");
            assert_eq!(authored["access"], "author");
            assert_eq!(authored["space"]["role"], "author");
            assert_eq!(authored["invites"].as_array().map(Vec::len), Some(1));

            let list = raw_http_get(addr, "/directory/spaces", &[]).await;
            assert_eq!(list.status, 200);
            let list: serde_json::Value = serde_json::from_slice(&list.body).expect("public list");
            let rows = list.as_array().expect("list rows");
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0]["access"], "public");
            assert_public_projection_has_no_private_keys(&rows[0]);
        });
    }

    #[test]
    fn space_public_boundary_public_event_route_denies_raw_directory_events() {
        run_socket_test(|| async {
            let state = test_state().await;
            let outsider = issue_test_session(&state, "event-outsider@example.invalid").await;
            let since = state.directory.head_seq().await.expect("pre-public head");
            let owner = issue_test_session(&state, "event-owner@example.invalid").await;
            let public_space = create_space_for_test(&state, &owner.user_id, "Event Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
            announce_document_for_test(&state, &public_space, "event-document").await;
            let addr = spawn_server(state).await;
            let anonymous = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[]).await;
            assert_eq!(anonymous.status, 200);
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&anonymous.body).expect("anonymous events"), serde_json::json!([]));
            let authorization = format!("Bearer {}", outsider.token);
            let nonmember = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[("Authorization", authorization.as_str())]).await;
            assert_eq!(nonmember.status, 200);
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&nonmember.body).expect("nonmember events"), serde_json::json!([]));
        });
    }

    #[test]
    fn space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry() {
        run_socket_test(|| async {
            let state = test_state().await;
            let outsider = issue_test_session(&state, "socket-public-outsider@example.invalid").await;
            let owner = issue_test_session(&state, "socket-public-owner@example.invalid").await;
            let public_space = create_space_for_test(&state, &owner.user_id, "Socket Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
            let since = state.directory.head_seq().await.expect("head");
            state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-replay-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Replay".into() })
                .await
                .expect("replayed raw public event");
            let receipt = issue_directory_socket_grant(bearer_headers(&outsider.token), State(state.clone())).await.expect("outsider directory grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/directory/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("public outsider directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free directory hello");
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-live-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Live".into() })
                .await
                .expect("live raw public event");
            state.directory_service.publish(DirectoryStreamMessage::Connection {
                phase: DirectoryConnectionPhase::Opened,
                connection: ConnectionView {
                    sync_session_id: "private-sync".into(),
                    space_id: public_space,
                    document_id: "private-document".into(),
                    surface: "private-surface".into(),
                    actor: "private-actor".into(),
                    user_id: Some(owner.user_id),
                    email: Some("socket-public-owner@example.invalid".into()),
                    role: DirectorySpaceRole::Author,
                    connected_at_ms: 1,
                    presence_known: true,
                },
            });
            let head = state.directory.head_seq().await.expect("post-event head");
            assert!(head > since);
            state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: head });
            assert!(tokio::time::timeout(std::time::Duration::from_millis(250), socket.next()).await.is_err(), "public nonmember received a raw event, member telemetry, or global progress cursor");
        });
    }

    #[test]
    fn admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands() {
        let valid = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private"}"#;
        assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(valid).is_ok());
        let generic = r#"{"kind":"directory","requestId":"request:one","command":{"kind":"create-space","name":"Studio","spaceKind":"studio","visibility":"private"}}"#;
        let forbidden = r#"{"kind":"announce-document","requestId":"request:one","descriptor":{}}"#;
        let unknown = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private","actor":"admin"}"#;
        assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(generic).is_err());
        assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(forbidden).is_err());
        assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(unknown).is_err());
    }

    #[test]
    fn admin_document_cursor_is_principal_route_and_exact_page_bound() {
        let cursor_key = [0x5a; 32];
        let principal = AdminPrincipalV1 {
            user_id: "user:admin".into(),
            auth_session_id: "session:admin".into(),
            authorization_generation: 7,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: now_ms() + 60_000,
            correlation_id: "correlation:admin".into(),
            peer_class: "admin-rest",
        };
        let cursor = admin_cursor_encode_scoped(&cursor_key, &principal, 5, Some("space:one"), ADMIN_PAGE_MAX).expect("document cursor");
        assert_eq!(cursor.len(), 84);
        assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:one"), Some(&cursor)), Ok(ADMIN_PAGE_MAX));
        assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:two"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
        assert_eq!(admin_cursor_decode(&cursor_key, &principal, 2, Some(&cursor)), Err(StatusCode::BAD_REQUEST));
        let mut other_principal = principal.clone();
        other_principal.auth_session_id = "session:other".into();
        assert_eq!(admin_cursor_decode_scoped(&cursor_key, &other_principal, 5, Some("space:one"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
        assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX) }), Ok(ADMIN_PAGE_MAX));
        assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(0) }), Err(StatusCode::BAD_REQUEST));
        assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX + 1) }), Err(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row() {
        let cursor_key = [0x5a; 32];
        let principal = AdminPrincipalV1 {
            user_id: "user:admin".into(),
            auth_session_id: "session:admin".into(),
            authorization_generation: 7,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: now_ms() + 60_000,
            correlation_id: "correlation:admin".into(),
            peer_class: "admin-rest",
        };
        let rows = (0..ADMIN_PAGE_MAX).map(|index| os_directory::UserView { id: format!("user:{index}:{}", "i".repeat(4_000)), email: format!("{index}@{}", "e".repeat(4_000)), display_name: "n".repeat(4_000), created_at_ms: 0 }).collect();
        let page = admin_fit_page(rows, false, 7, |rows| admin_cursor_encode(&cursor_key, &principal, 2, rows.len())).expect("byte-bounded user page");
        assert!(page.rows.len() < ADMIN_PAGE_MAX);
        assert!(page.next_cursor.is_some());
        assert!(directory::os_pack::json::to_json_string(&page).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let connections = (0..ADMIN_PAGE_MAX)
            .map(|index| AdminRecordedConnectionV1 {
                sync_session_id: format!("sync:{index}:{}", "s".repeat(4_000)),
                scope: DocumentScope::new("space", format!("document:{index}:{}", "d".repeat(4_000))),
                authenticated_user_id: Some(format!("user:{index}:{}", "u".repeat(4_000))),
                email: Some("admin@example.com".into()),
                role: Some(DirectorySpaceRole::Author),
                connected_at_ms: 0,
                source: "recorded-sync-session".into(),
            })
            .collect();
        let snapshot = admin_fit_connection_snapshot(connections, false, 7, 9, &cursor_key, &principal, 0).expect("byte-bounded connection snapshot");
        assert!(snapshot.rows.len() < ADMIN_PAGE_MAX);
        assert!(snapshot.next_cursor.is_some());
        assert!(directory::os_pack::json::to_json_string(&snapshot).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let view = SpaceView {
            id: "space:one".into(),
            name: "Space".into(),
            kind: os_directory::DirectorySpaceKind::Studio,
            visibility: DirectorySpaceVisibility::Private,
            owner_user_id: "user:owner".into(),
            role: None,
            member_count: ADMIN_PAGE_MAX as u32,
            document_count: 0,
            active_connections: 0,
            created_at_ms: 0,
            updated_at_ms: 0,
        };
        let members = (0..ADMIN_PAGE_MAX).map(|index| MemberView { user_id: format!("user:{index}:{}", "u".repeat(4_000)), email: format!("{index}@example.com"), display_name: "n".repeat(4_000), role: DirectorySpaceRole::Author }).collect();
        let detail = admin_fit_space_detail(view, members, false, 7, &cursor_key, &principal, "space:one", 0).expect("byte-bounded member detail");
        assert!(detail.members.rows.len() < ADMIN_PAGE_MAX);
        assert!(detail.members.next_cursor.is_some());
        assert!(directory::os_pack::json::to_json_string(&detail).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let oversized = vec![os_directory::UserView { id: "i".repeat(ADMIN_RESPONSE_MAX_BYTES), email: "e@example.com".into(), display_name: "name".into(), created_at_ms: 0 }];
        assert_eq!(admin_fit_page(oversized, false, 7, |_| Ok("a".repeat(84))), Err(StatusCode::PAYLOAD_TOO_LARGE));
    }

    #[tokio::test]
    async fn admin_rebuild_slots_are_atomic_and_abort_closes_once() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        directory.seed().await.expect("seed directory");
        let directory = Arc::new(HubDirectories::from(directory));
        let operations = Arc::new(ShardedMap::new());
        let operation_slots = Arc::new(tokio::sync::Semaphore::new(64));
        let barrier = Arc::new(tokio::sync::Barrier::new(129));
        let release = Arc::new(tokio::sync::Notify::new());
        let acquired = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut contenders = Vec::new();
        for _ in 0..128 {
            let barrier = barrier.clone();
            let release = release.clone();
            let acquired = acquired.clone();
            let slots = operation_slots.clone();
            contenders.push(tokio::spawn(async move {
                barrier.wait().await;
                if let Ok(_permit) = slots.try_acquire_owned() {
                    acquired.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    release.notified().await;
                }
            }));
        }
        barrier.wait().await;
        for _ in 0..128 {
            if acquired.load(std::sync::atomic::Ordering::Acquire) == 64 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(acquired.load(std::sync::atomic::Ordering::Acquire), 64);
        assert_eq!(operation_slots.available_permits(), 0);
        release.notify_waiters();
        for contender in contenders {
            contender.await.expect("slot contender");
        }
        assert_eq!(operation_slots.available_permits(), 64);

        let principal = AdminPrincipalV1 {
            user_id: "user:admin".into(),
            auth_session_id: "session:admin".into(),
            authorization_generation: 1,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: now_ms() + 60_000,
            correlation_id: "correlation:admin".into(),
            peer_class: "admin-rest",
        };
        let metadata = AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None };
        let request_id = "request:abort";
        let digest = "11".repeat(32);
        let operation_id = "operation:abort";
        let accepted = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "accepted", None, "accepted");
        directory.append_admin_operation_audit(&accepted).await.expect("accepted audit");
        let runtime = Arc::new(AdminOperationRuntime {
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            cancel_requested: std::sync::atomic::AtomicBool::new(false),
        });
        operations.insert(operation_id.into(), runtime);
        let interrupted = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "cancelled", None, "interrupted-before-terminal");
        let cleanup =
            AdminOperationCleanup { directory: directory.clone(), operations: operations.clone(), operation_id: operation_id.into(), terminal: Some(interrupted.clone()), _permit: operation_slots.clone().try_acquire_owned().expect("cleanup slot") };
        let task = tokio::spawn(async move {
            let _cleanup = cleanup;
            std::future::pending::<()>().await;
        });
        tokio::task::yield_now().await;
        task.abort();
        let _ = task.await;
        let mut rows = Vec::new();
        for _ in 0..128 {
            rows = directory.admin_operation_audit_for_request(request_id).await.expect("operation audit");
            if rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].fact.phase, "cancelled");
        assert_eq!(rows[1].fact.outcome_code, "interrupted-before-terminal");
        assert!(operations.get_cloned(operation_id).is_none());
        assert_eq!(operation_slots.available_permits(), 64);
        let later = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "succeeded", None, "late-success");
        assert!(matches!(directory.append_admin_operation_audit(&later).await, Err(DirectoryError::Conflict(_))), "a different late success cannot replace the winning interrupted cancellation");
        assert_eq!(directory.append_admin_operation_audit(&interrupted).await.expect("idempotent interrupted terminal").fact.phase, "cancelled", "the first terminal reason wins");
    }

    #[test]
    fn presence_lease_reconnect_rejects_old_live_refresh_and_close() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
            let token = seed_author_token(&state).await;
            let document_id = "presence-reconnect";
            announce_document_for_test(&state, STUDIO, document_id).await;
            let first = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id.to_string())), bearer_headers(&token), State(state.clone())).await.expect("first socket grant").0;
            let second = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id.to_string())), bearer_headers(&token), State(state.clone())).await.expect("second socket grant").0;
            assert_eq!(first.actor_id, second.actor_id);
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1");
            let (mut socket_a, _) = connect_async(socket_request(&url, &first.grant)).await.expect("first socket");
            socket_a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("first hello");
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Session { actor, .. } if actor == first.actor_id));
            socket_a.send(client_binary(&ClientFrame::Presence { peer: b"old-live".to_vec() }, Lane::Preview).await).await.expect("first presence");
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Presence { peers } if peers == vec![b"old-live".to_vec()]));

            let (mut socket_b, _) = connect_async(socket_request(&url, &second.grant)).await.expect("replacement socket");
            socket_b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("replacement hello");
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Session { actor, .. } if actor == second.actor_id));
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Presence { peers } if peers.is_empty()), "replacement removes the old visible row");
            socket_a.send(client_binary(&ClientFrame::Presence { peer: b"stale-refresh".to_vec() }, Lane::Preview).await).await.expect("stale refresh");
            assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket_b.next()).await.is_err(), "stale refresh produces no fanout");
            socket_b.send(client_binary(&ClientFrame::Presence { peer: b"current-live".to_vec() }, Lane::Preview).await).await.expect("current refresh");
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Presence { peers } if peers == vec![b"current-live".to_vec()]));
            socket_a.close(None).await.expect("stale socket close");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            socket_b.send(client_binary(&ClientFrame::Presence { peer: b"current-live-2".to_vec() }, Lane::Preview).await).await.expect("current refresh after stale close");
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Presence { peers } if peers == vec![b"current-live-2".to_vec()]));
        });
    }

    #[test]
    fn presence_lease_expires_server_clocked_visibility_without_socket_close() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let clock = Arc::new(TestPresenceClock::new());
            state.presence_clock = Some(clock.clone());
            let token = seed_author_token(&state).await;
            let document_id = "presence-expiry";
            announce_document_for_test(&state, STUDIO, document_id).await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id.to_string())), bearer_headers(&token), State(state.clone())).await.expect("socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("presence socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            socket.send(client_binary(&ClientFrame::Presence { peer: b"visible".to_vec() }, Lane::Preview).await).await.expect("visible presence");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers == vec![b"visible".to_vec()]));
            clock.advance_to(PRESENCE_LEASE_TTL_MS - 1);
            tokio::time::sleep(std::time::Duration::from_millis(1_100)).await;
            assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket.next()).await.is_err(), "the lease remains visible before its exact server deadline");
            clock.advance_to(PRESENCE_LEASE_TTL_MS);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers.is_empty()), "the server tick publishes exact-deadline expiry");
            socket.send(client_binary(&ClientFrame::Presence { peer: b"revived".to_vec() }, Lane::Preview).await).await.expect("live socket refresh after visibility expiry");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers == vec![b"revived".to_vec()]), "expiry does not close or unregister the authenticated socket");
        });
    }

    #[tokio::test]
    async fn presence_lease_enforces_shared_roster_bounds_and_actor_order() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json")).expect("presence lease fixture");
        assert_eq!(fixture["limits"]["ttlMs"].as_u64(), Some(PRESENCE_LEASE_TTL_MS));
        assert_eq!(fixture["limits"]["maximumItems"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ITEMS as u64));
        assert_eq!(fixture["limits"]["maximumEntryBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES as u64));
        assert_eq!(fixture["limits"]["maximumBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_BYTES as u64));
        let state = test_state().await;
        let ordered_scope = DocumentScope::new(STUDIO, "presence-order");
        let ordered_key = document_scope_key_v1(&ordered_scope);
        let now = tokio::time::Instant::now();
        for (actor, live, peer) in [("actor-z", "live-z", b"aaa".to_vec()), ("actor-a", "live-a", b"zzz".to_vec())] {
            assert_eq!(state.install_presence_slot(&ordered_key, STUDIO, &ordered_scope.document_id, actor, live, "surface", None, 0, now).await, PresenceLeaseTransition::NoChange);
            assert_eq!(state.refresh_presence(&ordered_key, STUDIO, &ordered_scope.document_id, actor, live, peer, now).await, PresenceLeaseTransition::Published);
        }
        let ordered = state.presence_snapshot(&ordered_key);
        assert_eq!(ordered.actors.iter().map(|actor| actor.actor.as_str()).collect::<Vec<_>>(), vec!["actor-a", "actor-z"]);
        assert_eq!(ordered.peers, vec![b"zzz".to_vec(), b"aaa".to_vec()], "opaque bytes do not select roster order");

        let full_scope = DocumentScope::new(STUDIO, "presence-full");
        let full_key = document_scope_key_v1(&full_scope);
        for index in 0..PRESENCE_ROSTER_MAXIMUM_ITEMS {
            let actor = format!("actor-{index:03}");
            let live = format!("live-{index:03}");
            assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, &actor, &live, "surface", None, 0, now).await, PresenceLeaseTransition::NoChange);
            assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, &actor, &live, vec![0; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES], now).await, PresenceLeaseTransition::Published);
        }
        assert_eq!(state.presence_snapshot(&full_key).peers.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
        assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", "surface", None, 0, now).await, PresenceLeaseTransition::NoChange);
        let deadline = state.presence.with(&(full_key.clone(), "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at);
        assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1], now + std::time::Duration::from_secs(1)).await, PresenceLeaseTransition::Rejected);
        assert_eq!(
            state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES + 1], now + std::time::Duration::from_secs(2)).await,
            PresenceLeaseTransition::Rejected
        );
        assert_eq!(state.presence.with(&(full_key, "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at), deadline, "rejection cannot refresh the lease deadline");
    }

    #[tokio::test]
    async fn presence_lease_restart_is_empty_and_directory_presence_is_member_only() {
        let state = test_state().await;
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, "presence-restart"));
        let before = state.directory.head_seq().await.expect("directory head");
        let now = tokio::time::Instant::now();
        assert_eq!(state.install_presence_slot(&key, STUDIO, "presence-restart", "actor-a", "live-a", "surface", Some("seed"), 0, now).await, PresenceLeaseTransition::NoChange);
        assert_eq!(state.refresh_presence(&key, STUDIO, "presence-restart", "actor-a", "live-a", b"opaque".to_vec(), now).await, PresenceLeaseTransition::Published);
        assert_eq!(state.directory.head_seq().await.expect("directory head"), before, "presence never appends a durable directory event");
        let member_token = seed_author_token(&state).await;
        let member = resolve_bearer_user(&state, Some(&member_token)).await.expect("member caller");
        let outsider_session = issue_test_session(&state, "presence-outsider@example.com").await;
        let outsider = resolve_bearer_user(&state, Some(&outsider_session.token)).await.expect("outsider caller");
        let message = DirectoryStreamMessage::Presence { space_id: STUDIO.into(), document_id: "presence-restart".into(), actors: state.presence_snapshot(&key).actors };
        assert!(directory_message_visible(&state, &message, Some(&member)).await);
        assert!(!directory_message_visible(&state, &message, Some(&outsider)).await);
        assert!(!directory_message_visible(&state, &message, None).await);
        let restarted = test_state().await;
        assert_eq!(restarted.presence.len(), 0, "a fresh hub has no server-local lease slots");
        assert!(restarted.presence_snapshot(&key).peers.is_empty());
    }

    async fn append_directory_page_test_events(state: &HubState, rows: &[(String, String)]) -> Vec<DirectoryEvent> {
        let events = rows
            .iter()
            .map(|(space_id, name)| semio_hub::directory::model::NewDirectoryEvent {
                hlc: os_directory::Hlc { physical_ms: now_ms(), logical: 0 },
                actor: DirectoryActor { kind: DirectoryActorKind::System, id: format!("system:event-page:{space_id}") },
                space_id: Some(space_id.clone()),
                user_id: None,
                body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: space_id.clone(), name: name.clone() },
            })
            .collect::<Vec<_>>();
        state.directory.append_events(&events).await.expect("append event-page fixture")
    }

    fn event_page_authorization(token: &str) -> String {
        format!("Bearer {token}")
    }

    #[tokio::test]
    async fn directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility() {
        let state = test_state().await;
        let caller = issue_test_session(&state, "event-page-member@example.com").await;
        upsert_member_for_test(&state, STUDIO, "event-page-member@example.com", DirectorySpaceRole::Spectator).await;
        let outsider = issue_test_session(&state, "event-page-owner@example.com").await;
        let hidden_space = create_space_for_test(&state, &outsider.user_id, "Hidden", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let after = state.directory.head_seq().await.expect("event-page start");
        let appended = append_directory_page_test_events(&state, &[(hidden_space.clone(), "hidden-11".into()), (STUDIO.into(), "visible-12".into()), (hidden_space.clone(), "hidden-13".into()), (STUDIO.into(), "visible-14".into())]).await;
        let addr = spawn_server(state.clone()).await;
        let authorization = event_page_authorization(&caller.token);
        let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
        assert_eq!(response.status, 200);
        assert!(response.headers.to_ascii_lowercase().contains("content-type: application/json"));
        let canonical = std::str::from_utf8(&response.body).expect("event-page UTF-8");
        let page = DirectoryEventPageV1::parse_canonical_json(canonical).expect("canonical event page");
        assert_eq!(page.after_seq_exclusive, after);
        assert_eq!(page.through_seq_inclusive, appended[3].seq);
        assert_eq!(page.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq, appended[3].seq]);
        assert!(!canonical.contains(&hidden_space));
        assert!(!canonical.contains("hidden-11"));
        assert!(!canonical.contains("hidden-13"));

        let hidden_after = state.directory.head_seq().await.expect("hidden scan start");
        let hidden = (0..DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).map(|index| (hidden_space.clone(), format!("hidden-saturated-{index}"))).collect::<Vec<_>>();
        let hidden_events = append_directory_page_test_events(&state, &hidden).await;
        let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={hidden_after}"), &[("Authorization", &authorization)]).await;
        let page = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("hidden page UTF-8")).expect("hidden page");
        assert!(page.events.is_empty());
        assert_eq!(page.through_seq_inclusive, hidden_events.last().expect("hidden tail").seq);
        assert!(page.has_more, "a saturated raw scan advertises the bounded follow-up even when every row is hidden");
    }

    #[tokio::test]
    async fn directory_event_page_v1_route_revalidates_session_generation_after_read_before_response() {
        let mut state = test_state().await;
        let session = issue_test_session(&state, "event-page-revoked@example.com").await;
        upsert_member_for_test(&state, STUDIO, "event-page-revoked@example.com", DirectorySpaceRole::Spectator).await;
        append_directory_page_test_events(&state, &[(STUDIO.into(), "before-revoke".into())]).await;
        let capability = SessionCapability::parse(&session.token).expect("event-page capability");
        let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
        let gate = Arc::new(TestLiveGate::default());
        gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
        state.live_gate = Some(gate.clone());
        let addr = spawn_server(state.clone()).await;
        let authorization = event_page_authorization(&session.token);
        let request = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
        let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("event-page read fence deadline").expect("event-page read fence");
        admitted.forget();
        state.directory.revoke_auth_session(&record.id, "event-page-test", None, "event-page-test").await.expect("revoke session").expect("revoked row");
        gate.directory_event_page_read_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(2), request).await.expect("revalidation response deadline").expect("revalidation request");
        assert_eq!(response.status, 401);
        assert!(response.body.is_empty());

        let session = issue_test_session(&state, "event-page-cancel@example.com").await;
        let authorization = event_page_authorization(&session.token);
        let cancelled = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
        let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("cancel read fence deadline").expect("cancel read fence");
        admitted.forget();
        let control = gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("request-owned cancellation control");
        cancelled.abort();
        assert!(cancelled.await.expect_err("request task cancelled").is_cancelled());
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while control.active.load(std::sync::atomic::Ordering::Acquire) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("server request cancellation deadline");
        assert!(control.cancelled.load(std::sync::atomic::Ordering::Acquire));
        gate.directory_event_page_read_release.add_permits(1);
    }

    #[tokio::test]
    async fn directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let after = state.directory.head_seq().await.expect("byte-prefix start");
        let appended = append_directory_page_test_events(&state, &[(STUDIO.into(), "a".repeat(32 * 1024)), (STUDIO.into(), "b".repeat(32 * 1024))]).await;
        let addr = spawn_server(state).await;
        let authorization = event_page_authorization(&token);
        let first = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
        assert_eq!(first.status, 200);
        assert!(first.body.len() <= DIRECTORY_EVENT_PAGE_MAX_BYTES);
        let first = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&first.body).expect("first page UTF-8")).expect("first page");
        assert_eq!(first.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[0].seq]);
        assert_eq!(first.through_seq_inclusive, appended[0].seq);
        assert!(first.has_more);
        let second = raw_http_get(addr, &format!("/directory/event-page/v1?after={}", first.through_seq_inclusive), &[("Authorization", &authorization)]).await;
        let second = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&second.body).expect("second page UTF-8")).expect("second page");
        assert_eq!(second.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq]);
        assert_eq!(second.through_seq_inclusive, appended[1].seq);
    }

    #[tokio::test]
    async fn directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j() {
        let state = test_state().await;
        let mut exact = DirectoryEvent {
            seq: 1,
            id: "event-boundary".into(),
            hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-boundary".into() },
            space_id: Some(STUDIO.into()),
            user_id: None,
            body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: String::new() },
            recorded_at_ms: 1,
        };
        let base = directory::os_pack::json::to_json_string(&exact).len();
        let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body else { unreachable!() };
        *name = "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
        assert_eq!(directory::os_pack::json::to_json_string(&exact).len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
        assert_eq!(validate_directory_event_page_event(&exact), Ok(()));
        if let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body {
            name.push('x');
        }
        assert_eq!(validate_directory_event_page_event(&exact), Err(DirectoryEventPageErrorV1::Invalid));

        let head = state.directory.head_seq().await.expect("head before rejected append");
        let rejected = semio_hub::directory::model::NewDirectoryEvent {
            hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-reject".into() },
            space_id: Some(STUDIO.into()),
            user_id: None,
            body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) },
        };
        assert!(matches!(state.directory.append_events(&[rejected]).await, Err(DirectoryError::Conflict(_))));
        assert_eq!(state.directory.head_seq().await.expect("head after rejected append"), head, "SQLite rolls back both row and dense sequence");
        let space = state.directory.get_space(STUDIO).await.expect("space projection").expect("seed space");
        assert_ne!(space.name.len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES, "rejected event never reaches projection");

        let postgres = include_str!("../../📇️directory/🐘️postgres/🦀️.rs");
        let neo4j = include_str!("../../📇️directory/🌐️neo4j/🦀️.rs");
        assert_eq!(postgres.matches("validate_directory_event_page_event(&").count(), 3, "all PostgreSQL full-event append seams admit before persistence");
        assert_eq!(neo4j.matches("validate_directory_event_page_event(&").count(), 3, "all Neo4j full-event append seams admit before persistence");
    }

    #[tokio::test]
    async fn directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body() {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let stale = issue_test_session(&state, "event-page-stale@example.com").await;
        let stale_capability = SessionCapability::parse(&stale.token).expect("stale capability");
        let stale_record = state.directory.authenticate_session(&stale_capability).await.expect("stale lookup").expect("stale session");
        state.directory.revoke_auth_session(&stale_record.id, "stale", None, "stale").await.expect("stale revoke").expect("stale row");
        let gate = Arc::new(TestLiveGate::default());
        gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
        state.live_gate = Some(gate.clone());
        let addr = spawn_server(state).await;
        let authorization = event_page_authorization(&token);
        for path in [
            "/directory/event-page/v1",
            "/directory/event-page/v1?",
            "/directory/event-page/v1?after=",
            "/directory/event-page/v1?after=00",
            "/directory/event-page/v1?after=1&after=2",
            "/directory/event-page/v1?since=0",
            "/directory/event-page/v1?after=%30",
            "/directory/event-page/v1?after=9007199254740992",
        ] {
            let response = raw_http_get(addr, path, &[("Authorization", &authorization)]).await;
            assert_eq!(response.status, 400, "query {path}");
            assert!(response.body.is_empty(), "query rejection is body-free");
        }
        let missing = raw_http_get(addr, "/directory/event-page/v1?after=0", &[]).await;
        assert_eq!(missing.status, 401);
        assert!(missing.body.is_empty());
        let stale_authorization = event_page_authorization(&stale.token);
        let stale = raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &stale_authorization)]).await;
        assert_eq!(stale.status, 401);
        assert!(stale.body.is_empty());
        assert_eq!(gate.directory_event_page_read_admitted.available_permits(), 0, "bad query and pre-read authentication failures perform no directory event scan");
    }

    // 🔬️ WS duplex fan-out over the real wire-v2 protocol: A's committed command reaches B on its
    // own socket as a `ServerFrame::Commands`, and B's Ack for A's own submit never round-trips
    // back to A as a duplicate Commands frame (origin filtering is the caller's job — this test
    // only asserts B observes it, matching `framework/sync`'s own origin check).

    // 🔬️ `GET`/`DELETE /auth/sessions/me`: a live session resolves the caller's identity; revoking
    // it makes the SAME token unauthorized on a subsequent call.
    #[tokio::test]
    async fn auth_sessions_me_roundtrip() {
        let state = test_state().await;
        let session = issue_test_session(&state, "me@example.com").await;
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().unwrap());

        let me = get_session_me(headers.clone(), State(state.clone())).await.expect("session me");
        assert_eq!(me.0.user_id, session.user_id);
        assert_eq!(me.0.email, "me@example.com");

        assert_eq!(delete_session_me(headers.clone(), State(state.clone())).await, StatusCode::NO_CONTENT);
        assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
    }
}
//#endregion 🔖️Tests
#[test]
fn document_scope_key_v1_is_length_prefixed_and_never_colon_ambiguous() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let vectors = fixture["documentScopeKeyV1"].as_array().expect("scope key vectors");
    let mut encoded = std::collections::HashSet::new();
    for vector in vectors {
        let scope = DocumentScope::new(vector["scope"]["spaceId"].as_str().unwrap(), vector["scope"]["documentId"].as_str().unwrap());
        let actual = document_scope_key_v1(&scope);
        assert_eq!(actual, vector["encoded"].as_str().unwrap());
        assert!(encoded.insert(actual), "scope vector aliased");
    }
    assert_ne!(db_artifact_id(&DocumentScope::new("space-a", "shared")), db_artifact_id(&DocumentScope::new("space-b", "shared")));
}
