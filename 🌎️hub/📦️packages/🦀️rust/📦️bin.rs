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

use axum::body::Bytes;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use db::db_storage::PayloadStorage as _;
use directory::os_directory::{
    self, ConnectionView, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryConnectionPhase, DirectoryEvent, DirectoryPresenceActor, DirectoryReadModel, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentView,
    InviteView, MemberView, SpaceView,
};
use directory::{DslValue, FromValue, ToValue};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use protocol::{decode_client_frame, encode_server_frame, AckStage, ActorId, ApplyOutcome, ArtifactId as ProtocolArtifactId, ClientFrame, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
use semio_framework_async::ShardedMap;
use semio_hub::directory::error::DirectoryError;
use semio_hub::directory::model::{AuthSessionKind, DocumentScope, InviteRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceRole, SyncSessionRecord};
#[cfg(test)]
use semio_hub::directory::model::AuthSessionIssue;
use semio_hub::directory::{identity_subject_digest, HubCapability, IdentityAssertionVerifier, IdentityVerificationControl, InviteCapability, LocalBootstrapTransport, SessionCapability, SocketGrantCapability, AUTH_TEXT_MAX_BYTES};
use semio_hub::lag_rebootstrap::{append_canonical_pair_data, append_canonical_pair_header, append_canonical_pair_terminal, canonical_pair_etag, CanonicalPairTerminal, RebootstrapContext, RebootstrapError, RebootstrapProgress, RebootstrapProgressStage, RebootstrapTransferControl, VerifiedRebootstrapSource, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, REBOOTSTRAP_DEADLINE_MS};
#[cfg(test)]
use semio_hub::lag_rebootstrap::decode_canonical_checkpoint_pair;
use semio_hub::local_bootstrap::{serve_local_bootstrap, InheritedLocalBootstrapTransport, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS};
use semio_hub::artifact_authority::trusted_catalog::{NativeCodecBinding, TrustedCatalogLoader, VerifiedTrustedCatalog};
use semio_hub::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage, ArtifactChunkCasStores, FsArtifactChunkCasStorage};
#[cfg(test)]
use semio_hub::artifact_authority::chunk_cas::{artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1, MemoryArtifactChunkCasStorage};
#[cfg(feature = "neo4j")]
use semio_hub::artifact_authority::chunk_cas::Neo4jArtifactChunkCasStorage;
#[cfg(feature = "postgres")]
use semio_hub::artifact_authority::chunk_cas::PostgresArtifactChunkCasStorage;
#[cfg(feature = "sqlite")]
use semio_hub::artifact_authority::chunk_cas::SqliteArtifactChunkCasStorage;
use semio_hub::artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, CheckpointPublicationOrchestrator, OperationContext, ValidatingCanonicalArtifactAuthority};
#[cfg(test)]
use semio_hub::artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, ImmutableArtifactBlobStore};
#[cfg(feature = "sqlite")]
use semio_hub::directory::sqlite::SqliteDirectory;
use semio_hub::directory::{ArtifactCasSweepContinuation, ArtifactCasSweepRequest, ArtifactCasSweepResult, CommandResult, DirectoryService, HubDirectories, HubDirectory, HubVerifiedCheckpointPublisher, DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS};
use serde::{Deserialize, Serialize};
use semio_framework_hash::Sha256;
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
                    match service
                        .sweep_artifact_cas(storage.as_ref(), checkpoint.request(execute, semio_hub::directory::ARTIFACT_CAS_SWEEP_OBJECT_MAX), &context)
                        .await
                    {
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
        Arc::new(Self {
            cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            healthy: Arc::new(std::sync::atomic::AtomicBool::new(true)),
            wake: Arc::new(tokio::sync::Notify::new()),
            task: std::sync::Mutex::new(None),
        })
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

async fn configured_artifact_authority(bundle_path: Option<std::path::PathBuf>, profile: Option<String>, bindings: &[NativeCodecBinding]) -> Result<Option<Arc<HubArtifactAuthority>>, AuthorityError> {
    let (bundle_path, profile) = match (bundle_path, profile) {
        (None, None) => return Ok(None),
        (Some(bundle_path), Some(profile)) => (bundle_path, profile),
        _ => return Err(AuthorityError::Catalog("OS_HUB_TRUSTED_CATALOG_BUNDLE and OS_HUB_TRUSTED_CATALOG_PROFILE must be configured together".to_string())),
    };
    let control = StartupCatalogControl;
    let started = control.now_ms();
    let context = OperationContext::new(started.saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let catalog = Arc::new(TrustedCatalogLoader::load(&bundle_path, &profile, bindings, &context).await?);
    Ok(Some(Arc::new(ValidatingCanonicalArtifactAuthority::new(catalog))))
}

fn linked_native_codec_bindings() -> Vec<NativeCodecBinding> {
    Vec::new()
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

/// @emoji 👤️ One connected actor's presence in a document (contract §C7.3) — inserted with `peer:
/// None` right after the hub sends `ServerFrame::Session`, updated to `peer: Some(bytes)` on
/// `ClientFrame::Presence`, removed at handler exit. `surface`/`color` are fixed for the life of the
/// connection (stamped at handshake time); the hub never decodes `peer` — it stores and forwards it
/// opaquely.
struct PresenceSession {
    surface: String,
    user_id: Option<String>,
    color: u8,
    peer: Option<Vec<u8>>,
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
        }
    }
}

const SOCKET_GRANT_TTL_MS: i64 = 30_000;
const SOCKET_GRANT_LEDGER_CAPACITY: usize = 4_096;
const SOCKET_GRANT_BINDING_PENDING_CAPACITY: usize = 64;
const SOCKET_PROTOCOL_V1: &str = "semio.socket.v1";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SocketBindingKeyV1 {
    User(String),
    Session(String),
    Share(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SocketAudienceV1 {
    Document(DocumentScope),
    Directory { auth_session_id: String, authorization_generation: u64 },
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
            (Self::Share { share_id, selector, scope, expires_at_ms }, SocketAudienceV1::Document(audience_scope)) if scope == audience_scope => {
                match directory.socket_share_binding(share_id, selector, scope, at_ms).await {
                    Ok(SocketShareBindingStatus::Active { expires_at_ms: current_expiry }) if current_expiry == *expires_at_ms => SocketBindingValidityV1::Active,
                    Ok(SocketShareBindingStatus::Unavailable) | Err(_) => SocketBindingValidityV1::Unavailable,
                    _ => SocketBindingValidityV1::Unauthorized,
                }
            }
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
    issued_at_ms: i64,
    expires_at_ms: i64,
    state: SocketGrantStateV1,
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

    async fn acquire_subject(&self, subject: &SocketSubjectV1) -> Vec<tokio::sync::OwnedMutexGuard<()>> {
        let mut admissions = Vec::with_capacity(2);
        for binding in subject.admission_bindings() {
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
    fn sweep_expired(inner: &mut SocketGrantLedgerInnerV1, at_ms: i64) {
        let live_selectors = inner
            .live_by_binding
            .values()
            .flat_map(BTreeMap::values)
            .map(|(selector, _)| selector.clone())
            .collect::<BTreeSet<_>>();
        let expired: Vec<String> = inner
            .records
            .iter()
            .filter_map(|(selector, record)| {
                (record.expires_at_ms <= at_ms && (record.state == SocketGrantStateV1::Pending || !live_selectors.contains(selector))).then(|| selector.clone())
            })
            .collect();
        for selector in expired {
            if let Some(record) = inner.records.remove(&selector) {
                let binding = record.subject.binding();
                if let Some(selectors) = inner.pending_by_binding.get_mut(&binding) {
                    selectors.remove(&selector);
                    if selectors.is_empty() {
                        inner.pending_by_binding.remove(&binding);
                    }
                }
            }
        }
    }

    fn issue(&self, capability: &SocketGrantCapability, audience: SocketAudienceV1, actor_id: String, subject: SocketSubjectV1, issued_at_ms: i64, expires_at_ms: i64) -> Result<(), SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, issued_at_ms);
        let binding = subject.binding();
        if inner.records.len() >= SOCKET_GRANT_LEDGER_CAPACITY || inner.pending_by_binding.get(&binding).map_or(0, BTreeSet::len) >= SOCKET_GRANT_BINDING_PENDING_CAPACITY {
            return Err(SocketGrantLedgerErrorV1::Capacity);
        }
        let selector = capability.selector().to_string();
        if inner.records.contains_key(&selector) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        inner.pending_by_binding.entry(binding).or_default().insert(selector.clone());
        inner.records.insert(
            selector.clone(),
            SocketGrantRecordV1 { selector, secret_digest: capability.secret_digest(), audience, actor_id, subject, issued_at_ms, expires_at_ms, state: SocketGrantStateV1::Pending },
        );
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
        if record.state != SocketGrantStateV1::Pending
            || !matches!(record.audience, SocketAudienceV1::Directory { .. })
            || !semio_hub::directory::constant_time_digest_eq(&record.secret_digest, &capability.secret_digest())
        {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        Ok(record.clone())
    }

    fn consume(&self, candidate: &SocketGrantRecordV1, at_ms: i64) -> Result<SocketGrantRecordV1, SocketGrantLedgerErrorV1> {
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        Self::sweep_expired(&mut inner, at_ms);
        let binding = candidate.subject.binding();
        if !inner.pending_by_binding.get(&binding).is_some_and(|selectors| selectors.contains(&candidate.selector)) {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        let record = inner.records.get_mut(&candidate.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if record.state != SocketGrantStateV1::Pending
            || record.audience != candidate.audience
            || record.secret_digest != candidate.secret_digest
            || record.issued_at_ms != candidate.issued_at_ms
            || record.expires_at_ms <= at_ms
        {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        record.state = SocketGrantStateV1::Consumed;
        let consumed = record.clone();
        if let Some(selectors) = inner.pending_by_binding.get_mut(&binding) {
            selectors.remove(&candidate.selector);
            if selectors.is_empty() {
                inner.pending_by_binding.remove(&binding);
            }
        }
        Ok(consumed)
    }

    fn register_live(&self, record: &SocketGrantRecordV1) -> Result<(String, Arc<tokio::sync::Notify>), SocketGrantLedgerErrorV1> {
        let id = directory::os_identity::time_ordered_id();
        let notify = Arc::new(tokio::sync::Notify::new());
        let mut inner = self.inner.lock().map_err(|_| SocketGrantLedgerErrorV1::Rejected)?;
        let stored = inner.records.get(&record.selector).ok_or(SocketGrantLedgerErrorV1::Rejected)?;
        if stored.state != SocketGrantStateV1::Consumed || stored.secret_digest != record.secret_digest || stored.audience != record.audience || stored.subject != record.subject {
            return Err(SocketGrantLedgerErrorV1::Rejected);
        }
        inner.live_by_binding.entry(record.subject.binding()).or_default().insert(id.clone(), (record.selector.clone(), notify.clone()));
        Ok((id, notify))
    }

    fn unregister_live(&self, record: &SocketGrantRecordV1, live_id: &str) {
        let Ok(mut inner) = self.inner.lock() else { return };
        let binding = record.subject.binding();
        if let Some(live) = inner.live_by_binding.get_mut(&binding) {
            live.remove(live_id);
            if live.is_empty() {
                inner.live_by_binding.remove(&binding);
            }
        }
        inner.records.remove(&record.selector);
    }

    fn is_live(&self, record: &SocketGrantRecordV1, live_id: &str) -> bool {
        let Ok(inner) = self.inner.lock() else { return false };
        inner.records.get(&record.selector).is_some_and(|stored| {
            stored.state == SocketGrantStateV1::Consumed
                && stored.secret_digest == record.secret_digest
                && stored.audience == record.audience
                && stored.subject == record.subject
                && inner
                    .live_by_binding
                    .get(&record.subject.binding())
                    .and_then(|live| live.get(live_id))
                    .is_some_and(|(selector, _)| selector == &record.selector)
        })
    }

    fn invalidate_binding(&self, binding: SocketBindingKeyV1) {
        let notifiers = {
            let Ok(mut inner) = self.inner.lock() else { return };
            if let Some(selectors) = inner.pending_by_binding.remove(&binding) {
                for selector in selectors {
                    inner.records.remove(&selector);
                }
            }
            let invalidated = inner
                .records
                .iter()
                .filter_map(|(selector, record)| (record.subject.binding() == binding).then(|| selector.clone()))
                .collect::<Vec<_>>();
            for selector in invalidated {
                inner.records.remove(&selector);
            }
            inner.live_by_binding.remove(&binding).into_iter().flat_map(BTreeMap::into_values).map(|(_, notify)| notify).collect::<Vec<_>>()
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
        let binding = record.subject.binding();
        inner.records.remove(selector);
        if let Some(selectors) = inner.pending_by_binding.get_mut(&binding) {
            selectors.remove(selector);
            if selectors.is_empty() {
                inner.pending_by_binding.remove(&binding);
            }
        }
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

async fn socket_live_authority(
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
) -> Result<Vec<tokio::sync::OwnedMutexGuard<()>>, SocketBindingValidityV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&record.subject))
        .await
        .map_err(|_| SocketBindingValidityV1::Unavailable)?;
    let validity = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()),
    )
    .await
    .unwrap_or(SocketBindingValidityV1::Unavailable);
    if validity != SocketBindingValidityV1::Active {
        return Err(validity);
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
    _artifact_publication: Arc<HubArtifactPublication>,
    artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    /// @emoji 🏭️ Wave 1.B: the single serialized directory writer (contract §C1's decider laws +
    /// dense event `seq`) built once over `directory` at startup — see `semio_hub::directory::
    /// DirectoryService`'s own doc. `/directory/commands` and `/directory/invites/{token}/redeem`
    /// go through this; every other `/directory/*` route reads `directory` directly.
    directory_service: Arc<DirectoryService>,
    admin_subjects: Arc<[AdminSubject]>,
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
    /// @emoji 👥️ `(document_scope_key_v1, actor)` -> that actor's presence session (contract §C7.3) — ephemeral,
    /// never durable (mirrors the preview lane's own law), rebuilt from nothing on hub restart. The
    /// roster is document-wide now (contract §C7.0): `ServerFrame::Presence` fans out on `fanout`, not
    /// a surface-scoped channel; a peer's `surface` travels INSIDE its `PresencePeer` bytes, stamped
    /// by the client actor, never decoded by this hub.
    presence: Arc<ShardedMap<(String, String), PresenceSession>>,
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
    socket_binding_gates: Arc<SocketBindingGatesV1>,
    /// @emoji 🧩️ Wave 1.B: installed runtime extensions mirrored from dev `/extensions` static dir —
    /// populated by hub deploy copy / sideload; `GET /extensions` lists `install.json` rows.
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

    /// @emoji 👥️ The document-wide roster's raw peer bytes (contract §C7.3) — entries whose `peer` is
    /// still `None` (handshake-only, no `ClientFrame::Presence` published yet) are excluded.
    fn presence_peers(&self, key: &str) -> Vec<Vec<u8>> {
        let mut peers = Vec::new();
        self.presence.for_each(|(scope, _), session| {
            if scope == key {
                peers.extend(session.peer.clone());
            }
        });
        peers
    }

    /// @emoji 📡️ Amendment 3 to C1: the SAME roster as `presence_peers`, shaped as
    /// `DirectoryPresenceActor`s the hub already knows without ever decoding a peer's bytes.
    fn directory_presence_actors(&self, key: &str) -> Vec<DirectoryPresenceActor> {
        let mut actors = Vec::new();
        self.presence.for_each(|(scope, actor), session| {
            if scope == key && session.peer.is_some() {
                actors.push(DirectoryPresenceActor { actor: actor.clone(), user_id: session.user_id.clone(), surface: session.surface.clone(), color: session.color });
            }
        });
        actors
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
/// @emoji 🔎️ What a bearer token resolved to: an authenticated space member, an anonymous
/// share-token viewer, an anonymous spectator admitted only because the space is `visibility ==
/// "public"`, or nothing.
enum AuthOutcome {
    Session {
        user_id: String,
        role: SpaceRole,
        session_id: String,
        authorization_generation: u64,
    },
    ShareToken,
    /// @emoji 👁️ Public-visibility fallback — an implicit anonymous `SpaceRole::Spectator`, never
    /// persisted as a membership row. Granted only when no session/share-token access resolved AND
    /// the space's own `visibility` is `"public"` (design ruling: "an implicit anonymous spectator
    /// role granted to unauthenticated requests" — deliberately a HANDLER-level fallback, not a
    /// policy-engine concept, since `db_security`'s `RoleBasedPolicy` stays purely mechanical).
    Public,
    Denied,
}

/// @emoji 🔐️ Tries the bearer as an `AuthSessionRecord` (session id -> user -> space role) first;
/// falls back to an active space/document share grant when session resolution fails; and
/// finally falls back to `AuthOutcome::Public` when the space itself is `visibility == "public"`.
/// Tokenless documents in a non-public space are always denied.
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
    match state.directory.get_space(space_id).await {
        Ok(Some(space)) if space.visibility == "public" => AuthOutcome::Public,
        _ => AuthOutcome::Denied,
    }
}

async fn authorized(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, space_id, document_id, token).await, AuthOutcome::Denied)
}

/// @emoji 📦️ Space-scoped blobs have no owning document, so this borrows `resolve_auth`'s
/// session -> role branch as-is (space role lookup never touches `document_id`) by passing the
/// blob hash in the document-id slot; the share-token branch then degrades to `Denied` unless a
/// document happens to share the blob's hash as its id, which content hashes never do in
/// practice. A session with any space role is required — a document's share token intentionally
/// does not widen into read access over the whole space's content-addressed blob store.
async fn authorized_for_blob(state: &HubState, space_id: &str, hash: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, space_id, hash, token).await, AuthOutcome::Denied)
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
    provider: String,
    subject_digest: [u8; 32],
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
    rebootstrap: bool,
    mcp_workspace: bool,
    inference: bool,
}

fn hub_readiness(mode: HubMode, bind_scope: &'static str, run_id: String, bootstrap_ready: bool, artifact_authority_ready: bool, admin_assets_ready: bool, artifact_cas_barrier_ready: bool, artifact_cas_sweep_execute: bool) -> HubReadinessV1 {
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
        features: HubFeatureReadinessV1 { open_plan: false, rebootstrap: true, mcp_workspace: false, inference: false },
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
        if subjects.iter().any(|existing: &AdminSubject| existing.provider == provider && semio_hub::directory::constant_time_digest_eq(&existing.subject_digest, &subject_digest)) {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ADMIN_SUBJECTS contains a duplicate identity".into()));
        }
        subjects.push(AdminSubject { provider: provider.to_string(), subject_digest });
    }
    Ok(subjects.into())
}

fn validate_auth_startup(
    mode: HubMode,
    bind: std::net::IpAddr,
    verifier: Option<&Arc<dyn IdentityAssertionVerifier>>,
    local_bootstrap: Option<&Arc<dyn LocalBootstrapTransport>>,
    admin_subjects: &[AdminSubject],
) -> Result<(), HubError> {
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

/// @emoji 🛡️ Administrator authority is a verified session identity policy, never a static token
/// or proximity to a loopback interface.
async fn is_admin(state: &HubState, headers: &HeaderMap, _peer: Option<SocketAddr>) -> bool {
    let Some(encoded) = bearer(headers) else { return false };
    let Ok(capability) = SessionCapability::parse(&encoded) else { return false };
    let Ok(Some(session)) = state.directory.authenticate_session(&capability).await else { return false };
    state.admin_subjects.iter().any(|subject| {
        subject.provider == session.identity_provider && semio_hub::directory::constant_time_digest_eq(&subject.subject_digest, &session.identity_subject_digest)
    })
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
struct ShareResponse {
    id: String,
    token: String,
    expires_at: i64,
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

async fn issue_socket_grant(
    state: &HubState,
    subject: SocketSubjectV1,
    audience: SocketAudienceV1,
    stable_actor_material: Option<[u8; 32]>,
) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    let binding = subject.binding();
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&subject))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = tokio::time::timeout(std::time::Duration::from_secs(2), subject.revalidate(state.directory.as_ref(), &audience, now_ms()))
        .await
        .unwrap_or(SocketBindingValidityV1::Unavailable);
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
    let validity = tokio::time::timeout(std::time::Duration::from_secs(2), record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()))
        .await
        .unwrap_or(SocketBindingValidityV1::Unavailable);
    match validity {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => {
            state.socket_grants.invalidate_binding(binding);
            return Err(StatusCode::UNAUTHORIZED);
        }
        SocketBindingValidityV1::Unavailable => {
            state.socket_grants.reject_pending(&record.selector);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    }
    Ok(Json(SocketGrantReceiptV1 { schema: "semio.hub.socket-grant/v1", protocol: SOCKET_PROTOCOL_V1, grant: capability.expose_once(), actor_id, expires_at_ms }))
}

async fn issue_document_socket_grant(
    Path((space_id, document_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let scope = DocumentScope::new(space_id, document_id);
    let capability = HubCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let (subject, stable_actor_material) = match capability {
        HubCapability::Session(capability) => {
            let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability))
                .await
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let role = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_role(&scope.space_id, &session.user_id))
                .await
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let material = session.secret_digest;
            let subject = SocketSubjectV1::Session {
                session_id: session.id,
                user_id: session.user_id,
                authorization_generation: session.authorization_generation,
                role: Some(role),
                expires_at_ms: session.expires_at,
            };
            (subject, Some(material))
        }
        HubCapability::Share(capability) => {
            let share = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_share_binding(&scope, &capability))
                .await
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            (SocketSubjectV1::Share { share_id: share.id, selector: share.selector, scope: scope.clone(), expires_at_ms: share.expires_at }, None)
        }
        HubCapability::Invite(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    let descriptor = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(&scope))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(directory_error_status)?;
    if descriptor.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    issue_socket_grant(&state, subject, audience, stable_actor_material).await
}

async fn issue_directory_socket_grant(headers: HeaderMap, State(state): State<HubState>) -> Result<Json<SocketGrantReceiptV1>, StatusCode> {
    let capability = SessionCapability::parse(&socket_issue_bearer(&headers)?).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&capability))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let material = session.secret_digest;
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let subject = SocketSubjectV1::Session {
        session_id: session.id,
        user_id: session.user_id,
        authorization_generation: session.authorization_generation,
        role: None,
        expires_at_ms: session.expires_at,
    };
    issue_socket_grant(&state, subject, audience, Some(material)).await
}

const DEFAULT_SHARE_TTL_SECS: i64 = 7 * 24 * 60 * 60;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateShareQuery {
    ttl_secs: Option<i64>,
}

#[derive(Serialize)]
struct BlobRecord {
    hash: String,
    media_type: String,
    size: i64,
}

fn canonical_pair_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    if headers.get_all(axum::http::header::AUTHORIZATION).iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    bearer(headers).filter(|value| !value.is_empty()).ok_or(StatusCode::UNAUTHORIZED)
}

fn canonical_pair_request_admission(uri: &axum::http::Uri, headers: &HeaderMap) -> Result<String, StatusCode> {
    if uri.query().is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if headers.contains_key(axum::http::header::RANGE) {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    if headers.get_all(axum::http::header::ACCEPT).iter().count() != 1
        || headers.get(axum::http::header::ACCEPT).and_then(|value| value.to_str().ok()) != Some(CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE)
    {
        return Err(StatusCode::NOT_ACCEPTABLE);
    }
    canonical_pair_bearer(headers)
}

struct CanonicalPairHttpControl;

impl RebootstrapTransferControl for CanonicalPairHttpControl {
    fn now_ms(&self) -> u64 {
        u64::try_from(now_ms()).unwrap_or(u64::MAX)
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: RebootstrapProgress) {}
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
async fn get_active_checkpoint_pair(
    Path((space_id, document_id)): Path<(String, String)>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Response {
    let token = match canonical_pair_request_admission(&uri, &headers) {
        Ok(token) => token,
        Err(status) => return status.into_response(),
    };
    let scope = DocumentScope::new(space_id, document_id);
    let control = CanonicalPairHttpControl;
    let deadline = control.now_ms().saturating_add(REBOOTSTRAP_DEADLINE_MS);
    let context = RebootstrapContext::new(deadline, &control);
    control.report(RebootstrapProgress { stage: RebootstrapProgressStage::Authorize, completed_units: 1, total_units: 1 });
    if context.checkpoint().is_err() || !authorized_for_canonical_pair(&state, &scope, &token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let pair = match state.rebootstrap.active_pair(&scope, &context).await {
        Ok(pair) => pair,
        Err(error) => return canonical_pair_error_status(error).into_response(),
    };
    if !authorized_for_canonical_pair(&state, &scope, &token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let mut body = Vec::new();
    if let Err(error) = append_canonical_pair_header(&mut body, &pair.selection) {
        return canonical_pair_error_status(error).into_response();
    }
    for ordinal in 0..pair.data_record_count() {
        if !authorized_for_canonical_pair(&state, &scope, &token).await {
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
    if !authorized_for_canonical_pair(&state, &scope, &token).await {
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

async fn create_share(
    Path((space_id, document_id)): Path<(String, String)>,
    axum::extract::Query(query): axum::extract::Query<CreateShareQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<Json<ShareResponse>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let scope = DocumentScope::new(space_id, document_id);
    if state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let issued = state.directory.issue_share_token(&scope, query.ttl_secs.unwrap_or(DEFAULT_SHARE_TTL_SECS), &directory::os_identity::time_ordered_id()).await.map_err(directory_error_status)?;
    Ok(Json(ShareResponse { id: issued.record.id, token: issued.capability.expose_once(), expires_at: issued.record.expires_at }))
}

async fn revoke_share(Path((space_id, document_id, share_id)): Path<(String, String, String)>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> StatusCode {
    if !is_admin(&state, &headers, Some(peer)).await {
        return StatusCode::UNAUTHORIZED;
    }
    let scope = DocumentScope::new(space_id, document_id);
    let binding = SocketBindingKeyV1::Share(share_id.clone());
    let gate = state.socket_binding_gates.gate(binding.clone());
    let _admission = match tokio::time::timeout(std::time::Duration::from_secs(2), gate.lock_owned()).await {
        Ok(admission) => admission,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE,
    };
    match tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.directory.revoke_share_token(&scope, &share_id, "administrator-revoked", &directory::os_identity::time_ordered_id()),
    )
    .await
    {
        Ok(Ok(())) => {
            state.socket_grants.invalidate_binding(binding);
            StatusCode::NO_CONTENT
        }
        Ok(Err(error)) => directory_error_status(error),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
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

async fn consume_socket_grant(state: &HubState, headers: &HeaderMap, audience: SocketAudienceV1) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending(&capability, &audience, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&candidate.subject)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        candidate.subject.revalidate(state.directory.as_ref(), &candidate.audience, now_ms()),
    )
    .await
    .unwrap_or(SocketBindingValidityV1::Unavailable);
    match validity {
        SocketBindingValidityV1::Active => state
            .socket_grants
            .consume(&candidate, now_ms())
            .map(|record| SocketGrantAdmissionV1 { record })
            .map_err(|_| StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn consume_directory_socket_grant(state: &HubState, headers: &HeaderMap) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending_directory(&capability, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&candidate.subject)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        candidate.subject.revalidate(state.directory.as_ref(), &candidate.audience, now_ms()),
    )
    .await
    .unwrap_or(SocketBindingValidityV1::Unavailable);
    match validity {
        SocketBindingValidityV1::Active => state
            .socket_grants
            .consume(&candidate, now_ms())
            .map(|record| SocketGrantAdmissionV1 { record })
            .map_err(|_| StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// @emoji 🎯️ Contract §C0: presence scope is `(space_id, document_id, surface)`, `surface` travels
/// out of band as `?surface=` on the document WS URL — missing ⇒ `""` (a document with no surface
/// concept still gets one scope, just the empty-string one).
#[derive(Deserialize)]
struct DocumentWsQuery {
    #[serde(default)]
    surface: Option<String>,
}

async fn document_ws(ws: WebSocketUpgrade, Path((space_id, document_id)): Path<(String, String)>, axum::extract::Query(query): axum::extract::Query<DocumentWsQuery>, State(state): State<HubState>) -> impl IntoResponse {
    let surface = query.surface.unwrap_or_default();
    ws.on_upgrade(move |socket| handle_ws(socket, space_id, document_id, surface, state, None))
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DocumentWsV1Query {
    surface: Option<String>,
}

async fn document_ws_v1(
    ws: WebSocketUpgrade,
    Path((space_id, document_id)): Path<(String, String)>,
    axum::extract::Query(query): axum::extract::Query<DocumentWsV1Query>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Response {
    let surface = query.surface.unwrap_or_default();
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) || surface.len() > AUTH_TEXT_MAX_BYTES {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(&space_id, &document_id);
    let admission = match consume_socket_grant(&state, &headers, SocketAudienceV1::Document(scope)).await {
        Ok(admission) => admission,
        Err(status) => return (status, "socket grant rejected").into_response(),
    };
    ws.protocols([SOCKET_PROTOCOL_V1]).on_upgrade(move |socket| handle_ws(socket, space_id, document_id, surface, state, Some(admission))).into_response()
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

async fn send_socket_document_rebootstrap(
    sender: &mut SplitSink<WebSocket, Message>,
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
    scope: &DocumentScope,
) -> SocketBindingValidityV1 {
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

async fn close_document_for_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, scope: &DocumentScope) {
    if let Some(control) = verified_rebootstrap_control(state, scope).await {
        let _ = sender.send(encode(&ServerFrame::RebootstrapRequired { control: wire_rebootstrap(&control) }).await).await;
    }
    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "rebootstrap-required".into() }))).await;
}

async fn close_directory_for_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, scope: Option<&DocumentScope>, caller: &AuthedUser) {
    if let Some(scope) = scope {
        if caller_is_space_member(state, &scope.space_id, Some(caller)).await {
            if let Some(control) = verified_rebootstrap_control(state, scope).await {
                let _ = send_directory_message(sender, &DirectoryStreamMessage::RebootstrapRequired { control }).await;
            }
        }
    }
    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "rebootstrap-required".into() }))).await;
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

/// @emoji 📨️ Handles one decoded `ClientFrame` for an already-authenticated, already-`Hello`'d
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
    gate: &db::security::SecurityGate,
    principal: &db::security::Principal,
    tenant: &db::security::TenantId,
    bound_actor: Option<&ActorId>,
    frame: ClientFrame,
    sender: &mut SplitSink<WebSocket, Message>,
) -> bool {
    match frame {
        ClientFrame::Commands { batch_id, envelopes } => {
            if bound_actor.is_some_and(|bound| envelopes.iter().any(|envelope| &envelope.actor != bound)) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack {
                    batch_id,
                    stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "socket subject actor mismatch".into(), messages: Vec::new() }) }],
                    frontier,
                };
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
            state.presence.with_mut(&(key.to_string(), actor.0.clone()), |entry| {
                if let Some(entry) = entry {
                    entry.peer = Some(peer);
                }
            });
            let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(key) });
            state.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.to_string(), document_id: document_id.to_string(), actors: state.directory_presence_actors(key) });
            true
        }
        // 🪙️ Command-lane credit-based flow control: no server-side congestion control implemented
        // this wave (matches `framework/sync`'s client, which also accepts and ignores this frame).
        ClientFrame::CreditGrant { .. } => true,
        ClientFrame::Bye => false,
        // A second `Hello` mid-session has nothing to negotiate beyond the first — ignored rather
        // than torn down, matching this crate's generally forgiving-of-redundant-frames stance.
        ClientFrame::Hello { .. } if bound_actor.is_some() => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            false
        }
        ClientFrame::Hello { .. } | ClientFrame::SocketHelloV1 { .. } => true,
    }
}

async fn handle_ws(socket: WebSocket, space_id: String, document_id: String, surface: String, state: HubState, socket_admission: Option<SocketGrantAdmissionV1>) {
    let (mut sender, mut receiver) = socket.split();
    let socket_grant = socket_admission.map(|admission| admission.record);

    let hello = match tokio::time::timeout(std::time::Duration::from_secs(2), receiver.next()).await {
        Ok(Some(Ok(Message::Binary(bytes)))) => decode_client_frame(&bytes).await.ok().map(|(_lane, frame)| frame),
        _ => None,
    };
    let (schema, pack_schema_hash, actor, token, frontier, auth) = match (socket_grant.as_ref(), hello) {
        (None, Some(ClientFrame::Hello { schema, pack_schema_hash, actor, token, frontier, .. })) => {
            let auth = resolve_auth(&state, &space_id, &document_id, token.as_deref()).await;
            (schema, pack_schema_hash, actor, token, frontier, auth)
        }
        (Some(record), Some(ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema, pack_schema_hash, resume_token, frontier }))
            if socket_text_bounded(&schema) && resume_token.as_ref().is_none_or(|value| value.len() <= AUTH_TEXT_MAX_BYTES) =>
        {
            let actor = ActorId(record.actor_id.clone());
            let auth = match &record.subject {
                SocketSubjectV1::Session { session_id, user_id, authorization_generation, role: Some(role), .. } => AuthOutcome::Session {
                    user_id: user_id.clone(),
                    role: *role,
                    session_id: session_id.clone(),
                    authorization_generation: *authorization_generation,
                },
                SocketSubjectV1::Share { .. } => AuthOutcome::ShareToken,
                SocketSubjectV1::Session { role: None, .. } => AuthOutcome::Denied,
            };
            (schema, pack_schema_hash, actor, None, frontier, auth)
        }
        _ => {
            let _ = sender.send(error_frame("protocol", "expected socket hello").await).await;
            return;
        }
    };

    let (user_id, role, auth_session_id, authorization_generation) = match &auth {
        AuthOutcome::Session { user_id, role, session_id, authorization_generation } => (Some(user_id.clone()), Some(*role), Some(session_id.as_str()), *authorization_generation),
        AuthOutcome::ShareToken => (None, None, None, 0),
        // 👁️ Public-visibility fallback: an implicit anonymous spectator, never a persisted
        // membership row (see `AuthOutcome::Public`'s doc).
        AuthOutcome::Public => (None, Some(SpaceRole::Spectator), None, 0),
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
    // 🎨️ Contract §C7.3: acquired after successful Hello/auth and before `Welcome`, released at
    // handler exit (every early-return path below releases it explicitly; the loop-exit cleanup
    // releases it on a clean disconnect).
    let color = state.acquire_color(&space_id, &actor.0);

    // 🔒️ Per-connection `SecurityGate`: `space_grants` compiles this space's `kind` into
    // author=rw/spectator=ro grants (archive additionally deny-overrides author writes), a fresh
    // `RoleBasedPolicy` from them, and a `Principal` carrying the caller's resolved role. A share-
    // token/public-visibility caller (no session role) is admitted as `"spectator"` — read-only,
    // the least-privilege default for a connection this crate cannot attribute to a real member.
    // `TenantId` reuses the space id: this crate has no separate tenant concept yet, and every
    // scope this gate ever evaluates already belongs to exactly this one space/document connection.
    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map_or_else(|| "studio".to_string(), |space| space.kind);
    let policy = db::security::space_grants(&space_id, &space_kind).await.into_iter().fold(db::security::RoleBasedPolicy::new(), db::security::RoleBasedPolicy::with_grant);
    let gate = db::security::SecurityGate::new(policy, db::security::ReplayGuard::new(60_000, 256), db::security::BudgetRegistry::new(240, 60), Arc::new(db::NullEmit));
    let tenant = db::security::TenantId::from(space_id.clone());
    // 🎯️ Role mapping: anonymous public and share-grant callers are both least-privilege
    // spectators. Only an authenticated directory membership can confer author authority.
    let role_str = match &auth {
        AuthOutcome::Session { role, .. } => role.as_str().to_string(),
        AuthOutcome::ShareToken => "spectator".to_string(),
        AuthOutcome::Public => "spectator".to_string(),
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
    let mut socket_binding_gates = if let Some(record) = socket_grant.as_ref() {
        match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&record.subject)).await {
            Ok(admission) => Some(admission),
            Err(_) => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        }
    } else {
        None
    };
    let socket_live = if let Some(record) = socket_grant.as_ref() {
        let (id, notify) = match state.socket_grants.register_live(record) {
            Ok(live) => live,
            Err(_) => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        };
        let lease = SocketLiveLeaseV1 { ledger: state.socket_grants.clone(), record: record.clone(), id, notify };
        let validity = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()),
        )
        .await
        .unwrap_or(SocketBindingValidityV1::Unavailable);
        match validity {
            SocketBindingValidityV1::Active => Some(lease),
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
    } else {
        None
    };
    #[cfg(test)]
    if socket_grant.is_some() {
        if let Some(gate) = &state.live_gate {
            gate.socket_before_welcome.add_permits(1);
            let _ = gate.socket_welcome_release.acquire().await;
        }
    }
    let welcome_sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(welcome_bytes)).await;
    let welcome_acknowledged = welcome.acknowledge();
    if !matches!(welcome_sent, Ok(Ok(()))) || welcome_acknowledged.is_err() {
        hello_session.cancel();
        state.release_color(&space_id, &actor.0);
        return;
    }
    drop(socket_binding_gates.take());
    #[cfg(test)]
    if socket_grant.is_some() {
        if let Some(gate) = &state.live_gate {
            gate.socket_after_welcome.add_permits(1);
            let _ = gate.socket_bootstrap_release.acquire().await;
        }
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
                let _authority = match (socket_grant.as_ref(), socket_live.as_ref()) {
                    (Some(record), Some(live)) => match socket_live_authority(&state, record, &live.id).await {
                        Ok(admission) => Some(admission),
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
                    },
                    _ => None,
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
    let _session_authority = match (socket_grant.as_ref(), socket_live.as_ref()) {
        (Some(record), Some(live)) => match socket_live_authority(&state, record, &live.id).await {
            Ok(admission) => Some(admission),
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
        },
        _ => None,
    };
    if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(session_frame)).await, Ok(Ok(()))) {
        state.release_color(&space_id, &actor.0);
        return;
    }
    state.presence.insert((key.clone(), actor.0.clone()), PresenceSession { surface: surface.clone(), user_id: user_id.clone(), color, peer: None });
    drop(_session_authority);

    let fanout = state.fanout_for(&key);
    let mut broadcast_rx = fanout.subscribe();
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.document_subscribed.add_permits(1);
        let _ = gate.document_release.acquire().await;
    }

    let sync_session = state
        .directory
        .record_sync_session_open(auth_session_id, authorization_generation, &actor.0, &space_id, &document_id, &surface, user_id.as_deref(), role, &actor.0)
        .await
        .ok();
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
                if let (Some(record), Some(live)) = (socket_grant.as_ref(), socket_live.as_ref()) {
                    match socket_live_authority(&state, record, &live.id).await {
                        Ok(_) => {}
                        Err(SocketBindingValidityV1::Unauthorized) => {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        }
                        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                            break;
                        }
                    }
                } else if matches!(resolve_auth(&state, &space_id, &document_id, token.as_deref()).await, AuthOutcome::Denied) {
                    break;
                }
            }
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => {
                        if let Ok((_lane, frame)) = decode_client_frame(&bytes).await {
                            #[cfg(test)]
                            if socket_grant.is_some() {
                                if let Some(live_gate) = &state.live_gate {
                                    live_gate.socket_command_received.add_permits(1);
                                    let _ = live_gate.socket_command_release.acquire().await;
                                }
                            }
                            let _authority = if let (Some(record), Some(live)) = (socket_grant.as_ref(), socket_live.as_ref()) {
                                match socket_live_authority(&state, record, &live.id).await {
                                    Ok(admission) => Some(admission),
                                    Err(SocketBindingValidityV1::Unauthorized) => {
                                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                        break;
                                    }
                                    Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                        break;
                                    }
                                }
                            } else {
                                None
                            };
                            match tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &gate, &principal, &tenant, socket_grant.as_ref().map(|_| &actor), frame, &mut sender),
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
                        if socket_grant.is_some() {
                            if let Some(live_gate) = &state.live_gate {
                                live_gate.socket_broadcast_received.add_permits(1);
                                let _ = live_gate.socket_broadcast_release.acquire().await;
                            }
                        }
                        let frame = encode(&frame).await;
                        let _authority = if let (Some(record), Some(live)) = (socket_grant.as_ref(), socket_live.as_ref()) {
                            match socket_live_authority(&state, record, &live.id).await {
                                Ok(admission) => Some(admission),
                                Err(SocketBindingValidityV1::Unauthorized) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                    break;
                                }
                                Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                    break;
                                }
                            }
                        } else {
                            None
                        };
                        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        if let (Some(record), Some(live)) = (socket_grant.as_ref(), socket_live.as_ref()) {
                            #[cfg(test)]
                            if let Some(live_gate) = &state.live_gate {
                                live_gate.socket_lag_received.add_permits(1);
                                let _ = live_gate.socket_lag_release.acquire().await;
                            }
                            match send_socket_document_rebootstrap(&mut sender, &state, record, &live.id, &scope).await {
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
                        } else if !matches!(resolve_auth(&state, &space_id, &document_id, token.as_deref()).await, AuthOutcome::Denied) {
                            close_document_for_rebootstrap(&mut sender, &state, &scope).await;
                        }
                        break;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = kick.notified() => break,
            _ = async {
                match socket_live.as_ref() {
                    Some(lease) => lease.notify.notified().await,
                    None => std::future::pending::<()>().await,
                }
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
    state.presence.remove(&(key.clone(), actor.0.clone()));
    let _ = fanout.send(ServerFrame::Presence { peers: state.presence_peers(&key) });
    state.release_color(&space_id, &actor.0);
    state.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.clone(), document_id: document_id.clone(), actors: state.directory_presence_actors(&key) });
}
//#endregion 🔖️WebSocket

//#region 🔖️Directory
/// @emoji 🙋️ A bearer token resolved to a live, unexpired `AuthSessionRecord`'s user — every
/// `/directory/*`/`/auth/sessions/me` route that needs a caller identity resolves through this
/// (distinct from `AuthOutcome`, which is the document-WS auth-lite scheme with its share-token/
/// public-visibility fallbacks; the directory control plane has no such fallbacks — a command with
/// no valid session is simply unauthenticated).
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
        let db_id = db_artifact_id(&DocumentScope::new(space_id, &descriptor.document_id));
        let frontier = match state.db.document(&db_id).await {
            Ok(handle) => handle.frontier().await.ok().map(|frontier| (frontier.head_seq, frontier.commit_seq, frontier.epoch)),
            Err(_) => None,
        }
        .unwrap_or((descriptor.bootstrap_frontier.head_seq, descriptor.bootstrap_frontier.commit_seq, descriptor.bootstrap_frontier.epoch));
        views.push(DocumentView { descriptor, head_seq: frontier.0, commit_seq: frontier.1, epoch: frontier.2 });
    }
    views
}

/// @emoji 🏠️ Fills a folded `DirectorySpace`'s `SpaceView` with the two fields the pure fold cannot
/// know: the CALLING user's own `role` (server-filled per request, never derived by `fold`) and the
/// live `document_count`/`active_connections` (owned by `db`'s catalog and the directory's sync
/// sessions respectively, neither of which the directory event log itself tracks).
async fn space_view(state: &HubState, space: &os_directory::DirectorySpace, caller: Option<&AuthedUser>) -> SpaceView {
    let mut view = space.view.clone();
    view.role = caller.and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id).map(|member| member.role));
    view.document_count = documents_for_space(state, &view.id).await.len() as u32;
    view.active_connections = state.directory.list_active_sync_sessions(Some(&view.id)).await.map(|sessions| sessions.len() as u32).unwrap_or(0);
    view
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

struct DirectoryCommandResponse {
    events: Vec<DirectoryEvent>,
    result: Option<DslValue>,
}

impl ToValue for DirectoryCommandResponse {
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("events".to_string(), self.events.to_value())];
        if let Some(result) = &self.result {
            entries.push(("result".to_string(), result.clone()));
        }
        DslValue::Object(entries)
    }
}

fn command_result_value(result: Option<CommandResult>) -> Option<DslValue> {
    result.and_then(|value| value.invite_token).map(|token| DslValue::object([("inviteToken".into(), DslValue::String(token))]))
}

async fn post_directory_commands(
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
    Json(DirectoryJson(command)): Json<DirectoryJson<DirectoryCommand>>,
) -> Result<(StatusCode, DirectoryJson<DirectoryCommandResponse>), StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let admin = is_admin(&state, &headers, Some(peer)).await;
    authorize_directory_command(&state, &user.user_id, admin, &command).await?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let (events, result) = state.directory_service.execute(actor, command).await.map_err(directory_error_status)?;
    Ok((StatusCode::ACCEPTED, DirectoryJson(DirectoryCommandResponse { events, result: command_result_value(result) })))
}

async fn get_directory_spaces(headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<SpaceView>>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let mut views = Vec::new();
    for space in model.spaces.values() {
        let visible = space.view.visibility == DirectorySpaceVisibility::Public || caller.as_ref().is_some_and(|user| space.members.iter().any(|member| member.user_id == user.user_id));
        if visible {
            views.push(space_view(&state, space, caller.as_ref()).await);
        }
    }
    views.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(DirectoryJson(views))
}

struct SpaceDetailResponse {
    view: SpaceView,
    members: Vec<MemberView>,
    documents: Vec<DocumentView>,
    invites: Option<Vec<InviteView>>,
}

impl ToValue for SpaceDetailResponse {
    fn to_value(&self) -> DslValue {
        let mut entries = match self.view.to_value() {
            DslValue::Object(entries) => entries,
            other => vec![("space".into(), other)],
        };
        entries.push(("members".into(), self.members.to_value()));
        entries.push(("documents".into(), self.documents.to_value()));
        if let Some(invites) = &self.invites {
            entries.push(("invites".into(), invites.to_value()));
        }
        DslValue::Object(entries)
    }
}

async fn get_directory_space(Path(space_id): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<SpaceDetailResponse>, StatusCode> {
    let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await;
    let model = load_read_model(&state).await?;
    let space = model.spaces.get(&space_id).ok_or(StatusCode::NOT_FOUND)?;
    let membership = caller.as_ref().and_then(|user| space.members.iter().find(|member| member.user_id == user.user_id));
    if space.view.visibility != DirectorySpaceVisibility::Public && membership.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let is_author = matches!(membership, Some(member) if member.role == DirectorySpaceRole::Author);
    let documents = documents_for_space(&state, &space_id).await;
    let invites = if is_author { state.directory.list_invites(&space_id).await.ok().map(|records| records.into_iter().map(invite_view).collect()) } else { None };
    let view = space_view(&state, space, caller.as_ref()).await;
    Ok(DirectoryJson(SpaceDetailResponse { view, members: space.members.clone(), documents, invites }))
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

async fn caller_is_space_member(state: &HubState, space_id: &str, caller: Option<&AuthedUser>) -> bool {
    let Some(caller) = caller else { return false };
    caller_active(state, caller).await && matches!(state.directory.get_role(space_id, &caller.user_id).await, Ok(Some(_)))
}

/// @emoji 👁️ One event's visibility for `caller`: a global identity event is visible only to the
/// identity it names; other member identities come from the authorized space-detail projection.
/// A space-scoped event is visible when the space is public or the active caller is a member.
async fn event_visible(state: &HubState, event: &DirectoryEvent, caller: Option<&AuthedUser>) -> bool {
    let Some(space_id) = &event.space_id else {
        return match (caller, event.user_id.as_deref()) {
            (Some(caller), Some(user_id)) if caller.user_id == user_id => caller_active(state, caller).await,
            _ => false,
        };
    };
    match state.directory.get_space(space_id).await {
        Ok(Some(space)) if space.visibility == "public" => true,
        Ok(Some(_)) => caller_is_space_member(state, space_id, caller).await,
        _ => false,
    }
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
        DirectoryStreamMessage::Connection { connection, .. } => caller_is_space_member(state, &connection.space_id, Some(caller)).await,
        DirectoryStreamMessage::Presence { space_id, .. } => caller_is_space_member(state, space_id, Some(caller)).await,
        DirectoryStreamMessage::Heartbeat { .. } => true,
        DirectoryStreamMessage::RebootstrapRequired { control } => caller_is_space_member(state, &control.scope.space_id, Some(caller)).await,
    }
}

async fn visibility_filter_events(state: &HubState, events: Vec<DirectoryEvent>, caller: Option<&AuthedUser>) -> Vec<DirectoryEvent> {
    let mut visible = Vec::with_capacity(events.len());
    for event in events {
        if event_visible(state, &event, caller).await {
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

async fn socket_user_space_member(state: &HubState, space_id: &str, user_id: &str) -> bool {
    matches!(state.directory.get_role(space_id, user_id).await, Ok(Some(_)))
}

async fn socket_directory_message_visible(state: &HubState, record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> SocketBindingValidityV1 {
    let validity = record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()).await;
    if validity != SocketBindingValidityV1::Active {
        return validity;
    }
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return SocketBindingValidityV1::Unauthorized };
    let visible = match message {
        DirectoryStreamMessage::Event { event } => match event.space_id.as_deref() {
            Some(space_id) => socket_user_space_member(state, space_id, user_id).await,
            None => event.user_id.as_deref() == Some(user_id.as_str()),
        },
        DirectoryStreamMessage::Connection { connection, .. } => socket_user_space_member(state, &connection.space_id, user_id).await,
        DirectoryStreamMessage::Presence { space_id, .. } => socket_user_space_member(state, space_id, user_id).await,
        DirectoryStreamMessage::Heartbeat { .. } => true,
        DirectoryStreamMessage::RebootstrapRequired { control } => socket_user_space_member(state, &control.scope.space_id, user_id).await,
    };
    if visible { SocketBindingValidityV1::Active } else { SocketBindingValidityV1::Unauthorized }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DirectoryWsV1Query {
    #[serde(default)]
    since: u64,
    space_id: Option<String>,
    document_id: Option<String>,
}

async fn directory_ws_v1(
    ws: WebSocketUpgrade,
    axum::extract::Query(query): axum::extract::Query<DirectoryWsV1Query>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Response {
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryWsQuery {
    token: Option<String>,
    #[serde(default)]
    since: u64,
    space_id: Option<String>,
    document_id: Option<String>,
}

async fn directory_ws(ws: WebSocketUpgrade, axum::extract::Query(query): axum::extract::Query<DirectoryWsQuery>, State(state): State<HubState>) -> impl IntoResponse {
    let scope = query.space_id.zip(query.document_id).map(|(space_id, document_id)| DocumentScope::new(space_id, document_id));
    ws.on_upgrade(move |socket| handle_directory_ws(socket, query.token, query.since, scope, state))
}

async fn send_directory_message(sender: &mut SplitSink<WebSocket, Message>, message: &DirectoryStreamMessage) -> bool {
    let text = directory::os_pack::json::to_json_string(message);
    sender.send(Message::Text(text.into())).await.is_ok()
}

async fn send_socket_directory_message(
    sender: &mut SplitSink<WebSocket, Message>,
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
    message: &DirectoryStreamMessage,
) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority(state, record, live_id).await {
        Ok(admission) => admission,
        Err(validity) => return validity,
    };
    let validity = tokio::time::timeout(std::time::Duration::from_secs(2), socket_directory_message_visible(state, record, message))
        .await
        .unwrap_or(SocketBindingValidityV1::Unavailable);
    if validity != SocketBindingValidityV1::Active {
        return validity;
    }
    match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, message)).await {
        Ok(true) => SocketBindingValidityV1::Active,
        _ => SocketBindingValidityV1::Unavailable,
    }
}

async fn send_socket_directory_rebootstrap(
    sender: &mut SplitSink<WebSocket, Message>,
    state: &HubState,
    record: &SocketGrantRecordV1,
    live_id: &str,
    scope: &DocumentScope,
) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority(state, record, live_id).await {
        Ok(admission) => admission,
        Err(validity) => return validity,
    };
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
        Some(control) => match tokio::time::timeout(
            std::time::Duration::from_secs(2),
            send_directory_message(sender, &DirectoryStreamMessage::RebootstrapRequired { control }),
        )
        .await
        {
            Ok(true) => SocketBindingValidityV1::Active,
            _ => SocketBindingValidityV1::Unavailable,
        },
        None => SocketBindingValidityV1::Active,
    }
}

/// @emoji 📡️ Contract §C2's "subscribe, then replay, gap-free": subscribes to `DirectoryService`'s
/// live broadcast FIRST (so nothing published between "read events_since" and "start listening" is
/// ever missed), THEN replays `events_since(since)`, THEN forwards live messages — dropping any
/// already-replayed `Event` (`seq <= last_replayed`) so a reconnecting client's stream is both
/// gap-free and duplicate-free.
async fn handle_directory_ws(socket: WebSocket, token: Option<String>, since: u64, scope: Option<DocumentScope>, state: HubState) {
    let (mut sender, mut receiver) = socket.split();
    let Some(caller) = resolve_bearer_user(&state, token.as_deref()).await else { return };
    let mut live = state.directory_service.subscribe();
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.directory_subscribed.add_permits(1);
        let _ = gate.directory_release.acquire().await;
    }

    let replay = match load_all_directory_events(state.directory.as_ref(), since).await {
        Ok(events) => visibility_filter_events(&state, events, Some(&caller)).await,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1011, reason: "directory_replay_failed".into() }))).await;
            return;
        }
    };
    let mut last_replayed = since;
    for event in replay {
        last_replayed = last_replayed.max(event.seq);
        if !send_directory_message(&mut sender, &DirectoryStreamMessage::Event { event }).await {
            return;
        }
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = live.recv() => {
                match event {
                    Ok(DirectoryStreamMessage::Event { event }) => {
                        let seq = event.seq;
                        let message = DirectoryStreamMessage::Event { event };
                        if seq <= last_replayed || !directory_message_visible(&state, &message, Some(&caller)).await {
                            continue;
                        }
                        last_replayed = last_replayed.max(seq);
                        if !send_directory_message(&mut sender, &message).await {
                            break;
                        }
                    }
                    Ok(message) => {
                        if !directory_message_visible(&state, &message, Some(&caller)).await {
                            continue;
                        }
                        if !send_directory_message(&mut sender, &message).await {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        close_directory_for_rebootstrap(&mut sender, &state, scope.as_ref(), &caller).await;
                        break;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
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
    let binding_gates = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_subject(&record.subject)).await {
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
    let validity = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        record.subject.revalidate(state.directory.as_ref(), &record.audience, now_ms()),
    )
    .await
    .unwrap_or(SocketBindingValidityV1::Unavailable);
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
            SocketBindingValidityV1::Active => {
                last_replayed = last_replayed.max(seq);
            }
            SocketBindingValidityV1::Unauthorized => {}
            SocketBindingValidityV1::Unavailable => {
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
                        SocketBindingValidityV1::Active => {
                            if let Some(seq) = seq { last_replayed = last_replayed.max(seq); }
                        }
                        SocketBindingValidityV1::Unauthorized => {}
                        SocketBindingValidityV1::Unavailable => {
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
    Ok(Json(SessionMeResponse {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        expires_at: session.expires_at,
        session_kind: session.session_kind,
        authorization_generation: session.authorization_generation,
    }))
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
    match tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.directory.revoke_auth_session(&session.id, "self-revoked", Some(&session.user_id), &directory::os_identity::time_ordered_id()),
    )
    .await
    {
        Ok(Ok(Some(revoked))) => {
            debug_assert_eq!(revoked.id, session.id);
            state.socket_grants.invalidate_binding(binding);
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

async fn admin_overview(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    let connections = state.directory.list_active_sync_sessions(None).await.map_err(directory_error_status)?.len();
    let users = state.directory.list_users(i64::MAX, 0).await.map_err(directory_error_status)?.len();
    // 🌵️ `extensions_root` is `{data_dir}/extension-modules` (see `main`'s own construction) — its
    // parent is `data_dir` itself, the nearest thing `HubState` carries to `OS_HUB_DATA`'s root.
    let data_dir_bytes = state.extensions_root.parent().map(dir_size).unwrap_or(0);
    Ok(Json(serde_json::json!({
        "counts": { "spaces": model.spaces.len(), "users": users, "connections": connections },
        "backends": { "sqlite": cfg!(feature = "sqlite"), "postgres": cfg!(feature = "postgres"), "neo4j": cfg!(feature = "neo4j") },
        "dataDirBytes": data_dir_bytes,
        "headSeq": head_seq,
        "openArtifacts": state.db.catalog().await.artifacts.len(),
    })))
}

async fn admin_spaces(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<Vec<SpaceView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let mut views = Vec::new();
    for space in model.spaces.values() {
        views.push(space_view(&state, space, None).await);
    }
    views.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(DirectoryJson(views))
}

async fn admin_space(Path(space_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<SpaceDetailResponse>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let model = load_read_model(&state).await?;
    let space = model.spaces.get(&space_id).ok_or(StatusCode::NOT_FOUND)?;
    let documents = documents_for_space(&state, &space_id).await;
    let invites = state.directory.list_invites(&space_id).await.map_err(directory_error_status)?.into_iter().map(invite_view).collect();
    let view = space_view(&state, space, None).await;
    Ok(DirectoryJson(SpaceDetailResponse { view, members: space.members.clone(), documents, invites: Some(invites) }))
}

async fn admin_users(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<Vec<os_directory::UserView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let users = state.directory.list_users(i64::MAX, 0).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(users.into_iter().map(|user| os_directory::UserView { id: user.id, email: user.email, display_name: user.display_name, created_at_ms: user.created_at }).collect()))
}

async fn admin_connections(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<Vec<ConnectionView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let sessions = state.directory.list_active_sync_sessions(None).await.map_err(directory_error_status)?;
    let mut views = Vec::with_capacity(sessions.len());
    for session in &sessions {
        views.push(connection_view(&state, session).await);
    }
    Ok(DirectoryJson(views))
}

#[derive(Deserialize)]
struct DocumentsQuery {
    space: Option<String>,
}

async fn admin_documents(
    axum::extract::Query(query): axum::extract::Query<DocumentsQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<Vec<DocumentView>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match query.space {
        Some(space_id) => Ok(DirectoryJson(documents_for_space(&state, &space_id).await)),
        None => {
            let mut views = Vec::new();
            for space in state.directory.list_spaces(i64::MAX, 0).await.map_err(directory_error_status)? {
                views.extend(documents_for_space(&state, &space.id).await);
            }
            Ok(DirectoryJson(views))
        }
    }
}

async fn admin_events(
    axum::extract::Query(query): axum::extract::Query<EventsQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<Vec<DirectoryEvent>>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let events = state.directory.events_since(query.since.unwrap_or(0), query.limit.unwrap_or(500)).await.map_err(directory_error_status)?;
    Ok(DirectoryJson(events))
}

/// @emoji 🛡️ Contract §C2: actor kind `admin`, bypasses `authorize_directory_command` entirely
/// (unlike `POST /directory/commands`, this route never resolves a bearer user — `is_admin` alone
/// gates it). `create-space` still rejects an `Admin`-kind actor (`decide`'s own "create-space
/// requires a user actor" law) — an admin operator creating a space needs a real user session and
/// belongs on `/directory/commands` instead; this route is for acting ON existing spaces/members.
async fn admin_commands(
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
    Json(DirectoryJson(command)): Json<DirectoryJson<DirectoryCommand>>,
) -> Result<(StatusCode, DirectoryJson<DirectoryCommandResponse>), StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::Admin, id: "admin".to_string() };
    let (events, result) = state.directory_service.execute(actor, command).await.map_err(directory_error_status)?;
    Ok((StatusCode::ACCEPTED, DirectoryJson(DirectoryCommandResponse { events, result: command_result_value(result) })))
}

async fn admin_rebuild(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    if !is_admin(&state, &headers, Some(peer)).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let replayed = state.directory.rebuild_projections().await.map_err(directory_error_status)?;
    Ok(Json(serde_json::json!({ "eventsReplayed": replayed })))
}

async fn admin_close_connection(Path(sync_session_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> StatusCode {
    if !is_admin(&state, &headers, Some(peer)).await {
        return StatusCode::UNAUTHORIZED;
    }
    match state.session_kicks.get_cloned(&sync_session_id) {
        Some(notify) => {
            notify.notify_one();
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

/// @emoji 🦵️ Durably revokes every user capability first, then separately signals matching live
/// connections. A failed kick cannot resurrect the already-revoked generation.
async fn admin_revoke_user_sessions(Path(user_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> StatusCode {
    if !is_admin(&state, &headers, Some(peer)).await {
        return StatusCode::UNAUTHORIZED;
    }
    let user_gate = state.socket_binding_gates.gate(SocketBindingKeyV1::User(user_id.clone()));
    let user_admission = match tokio::time::timeout(std::time::Duration::from_secs(2), user_gate.lock_owned()).await {
        Ok(admission) => admission,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE,
    };
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_admin_revoke_admitted.add_permits(1);
        let _ = gate.socket_admin_revoke_release.acquire().await;
    }
    let correlation_id = directory::os_identity::time_ordered_id();
    let revoked = match tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.directory.revoke_auth_sessions_for_user(&user_id, "administrator-revoked", None, &correlation_id),
    )
    .await
    {
        Ok(Ok(revoked)) => revoked,
        Ok(Err(_)) => return StatusCode::INTERNAL_SERVER_ERROR,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE,
    };
    for session in &revoked {
        state.socket_grants.invalidate_binding(SocketBindingKeyV1::Session(session.id.clone()));
    }
    drop(user_admission);
    let revoked_ids: std::collections::BTreeSet<&str> = revoked.iter().map(|session| session.id.as_str()).collect();
    let Ok(sessions) = state.directory.list_active_sync_sessions(None).await else { return StatusCode::INTERNAL_SERVER_ERROR };
    for session in sessions.iter().filter(|session| session.auth_session_id.as_deref().is_some_and(|id| revoked_ids.contains(id))) {
        if let Some(notify) = state.session_kicks.get_cloned(&session.id) {
            notify.notify_one();
        }
    }
    StatusCode::NO_CONTENT
}
//#endregion 🔖️Admin

//#region 🔖️Extensions
/// @emoji 🧩️ Hub mirror of dev `staticDirVitePlugin` `/extensions` — lists installed extension metadata.
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
/// route, e.g. `/admin/spaces/sp-1`) falls back to `index.html`; a genuinely missing admin build
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
    let path = if requested.is_file() { requested } else { root.join("index.html") };
    match tokio::fs::read(&path).await {
        Ok(bytes) => ([(axum::http::header::CONTENT_TYPE, admin_asset_content_type(&path))], bytes).into_response(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_admin_root(State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, "index.html").await
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
        .route("/directory/commands", post(post_directory_commands))
        .route("/directory/spaces", get(get_directory_spaces))
        .route("/directory/spaces/{id}", get(get_directory_space))
        .route("/directory/invites/{token}/redeem", post(post_redeem_invite))
        .route("/directory/events", get(get_directory_events))
        .route("/directory/socket-grants", post(issue_directory_socket_grant))
        .route("/directory/socket/v1", get(directory_ws_v1))
        .route("/directory/ws", get(directory_ws))
        .route("/admin/api/overview", get(admin_overview))
        .route("/admin/api/spaces", get(admin_spaces))
        .route("/admin/api/spaces/{id}", get(admin_space))
        .route("/admin/api/users", get(admin_users))
        .route("/admin/api/connections", get(admin_connections))
        .route("/admin/api/documents", get(admin_documents))
        .route("/admin/api/events", get(admin_events))
        .route("/admin/api/commands", post(admin_commands))
        .route("/admin/api/directory/rebuild", post(admin_rebuild))
        .route("/admin/api/connections/{sync_session_id}/close", post(admin_close_connection))
        .route("/admin/api/users/{id}/sessions/revoke", post(admin_revoke_user_sessions))
        .route("/extensions", get(list_extensions))
        .route("/extensions/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/admin", get(get_admin_root))
        .route("/admin/{*path}", get(get_admin_asset))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair", get(get_active_checkpoint_pair))
        .route("/spaces/{space_id}/documents/{id}/share", post(create_share))
        .route("/spaces/{space_id}/documents/{id}/share/{share_id}", delete(revoke_share))
        .route("/spaces/{space_id}/documents/{id}/socket-grants", post(issue_document_socket_grant))
        .route("/spaces/{space_id}/documents/{id}/socket/v1", get(document_ws_v1))
        .route("/spaces/{space_id}/documents/{id}/ws", get(document_ws))
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
    let native_codec_bindings = linked_native_codec_bindings();
    let artifact_authority = configured_artifact_authority(
        std::env::var("OS_HUB_TRUSTED_CATALOG_BUNDLE").ok().filter(|value| !value.is_empty()).map(std::path::PathBuf::from),
        std::env::var("OS_HUB_TRUSTED_CATALOG_PROFILE").ok().filter(|value| !value.is_empty()),
        &native_codec_bindings,
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
    let artifact_publication = Arc::new(CheckpointPublicationOrchestrator::new(
        ArtifactChunkBlobStore::new(artifact_cas.clone()),
        HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority"),
    ));
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
    let readiness = Arc::new(hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority.is_some(), admin_dir.is_dir(), true, artifact_cas_sweep_execute));
    let state = HubState {
        db,
        artifact_cas,
        directory: directory.clone(),
        rebootstrap,
        _artifact_authority: artifact_authority,
        _artifact_publication: artifact_publication,
        artifact_maintenance: artifact_maintenance.clone(),
        directory_service,
        admin_subjects,
        readiness,
        admin_dir,
        fanout: Arc::new(ShardedMap::new()),
        fanout_capacity: 256,
        #[cfg(test)]
        live_gate: None,
        #[cfg(test)]
        canonical_pair_authorization_gate: None,
        presence: Arc::new(ShardedMap::new()),
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
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
        fn verify<'a>(
            &'a self,
            _assertion: &'a semio_hub::directory::IdentityAssertion,
            _context: &'a semio_hub::directory::IdentityVerificationContext<'a>,
        ) -> semio_hub::directory::IdentityVerificationFuture<'a> {
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

        fn accept<'a>(&'a self, _context: &'a semio_hub::directory::IdentityVerificationContext<'a>) -> semio_hub::directory::LocalBootstrapAcceptFuture<'a> {
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

        fn reject<'a>(
            &'a self,
            _request_id: &'a str,
            _code: semio_hub::directory::LocalBootstrapRejectCode,
            _context: &'a semio_hub::directory::IdentityVerificationContext<'a>,
        ) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
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
        let admin = AdminSubject { provider: "oidc.example".into(), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
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
        let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, false);
        let encoded = serde_json::to_string(&ready).expect("readiness json");
        assert_eq!(ready.status, "ready");
        assert!(!ready.authentication.public_session_issuance);
        assert!(ready.artifact_authority.ready);
        assert!(!encoded.contains("session.v1"));
        assert!(!encoded.contains("subject"));
        assert!(!encoded.contains("channel"));
        assert!(!encoded.contains("sessionKind"));
        assert!(!encoded.contains("authorizationGeneration"));
        let partial = hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, true, true, false);
        assert_eq!(partial.status, "not-ready");
        assert!(partial.authentication.bootstrap_ready);
        assert!(!partial.artifact_authority.ready);
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), false, false, true, true, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "network", ready.run_id, true, true, true, false, false).status, "not-ready");
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
            if checkpoint.accept(&result) { break; }
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
        assert!(configured_artifact_authority(None, None, &[]).await.expect("unconfigured authority").is_none());
        let error = match configured_artifact_authority(Some(std::path::PathBuf::from("bundle.json")), None, &[]).await {
            Ok(_) => panic!("partial trusted-catalog configuration unexpectedly succeeded"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("must be configured together"));
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
        let artifact_publication = Arc::new(CheckpointPublicationOrchestrator::new(
            ArtifactChunkBlobStore::new(artifact_cas.clone()),
            HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test"),
        ));
        let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            _artifact_authority: None,
            _artifact_publication: artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, true, true, false)),
            admin_dir: dir.join("admin-dist"),
            fanout: Arc::new(ShardedMap::new()),
            fanout_capacity,
            live_gate: None,
            canonical_pair_authorization_gate: None,
            presence: Arc::new(ShardedMap::new()),
            session_colors: Arc::new(ShardedMap::new()),
            session_kicks: Arc::new(ShardedMap::new()),
            socket_grants: Arc::new(SocketGrantLedgerV1::default()),
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
        let artifact_publication = Arc::new(CheckpointPublicationOrchestrator::new(
            ArtifactChunkBlobStore::new(artifact_cas.clone()),
            HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test"),
        ));
        let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            _artifact_authority: None,
            _artifact_publication: artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, true, true, false)),
            admin_dir: dir.join("admin-dist"),
            fanout: Arc::new(ShardedMap::new()),
            fanout_capacity,
            live_gate: None,
            canonical_pair_authorization_gate: None,
            presence: Arc::new(ShardedMap::new()),
            session_colors: Arc::new(ShardedMap::new()),
            session_kicks: Arc::new(ShardedMap::new()),
            socket_grants: Arc::new(SocketGrantLedgerV1::default()),
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
            baseline_frontier: os_directory::ArtifactFrontier {
                document_id: document_id.to_string(),
                head_edit_ordinal: 1,
                head_edit_id: "verified-edit-1".into(),
                last_commit_seq: 1,
                chain_hash: os_directory::ArtifactHash([0x44; 32]),
            },
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
        let staged_pack = cas
            .stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &authority_context)
            .await
            .expect("stage reserved pack manifest");
        let staged_spr = cas
            .stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &authority_context)
            .await
            .expect("stage reserved SPR manifest");
        assert_eq!(staged_pack.storage_key, checkpoint.pack.storage_key);
        assert_eq!(staged_spr.storage_key, checkpoint.spr.storage_key);
        state
            .directory_service
            .publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, checkpoint.clone(), reservation, 100)
            .await
            .expect("publish verified checkpoint");
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
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

    async fn raw_http_get(addr: SocketAddr, path: &str, headers: &[(&str, &str)]) -> RawHttpResponse {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("HTTP connect");
        let mut request = format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n");
        for (name, value) in headers {
            request.push_str(name);
            request.push_str(": ");
            request.push_str(value);
            request.push_str("\r\n");
        }
        request.push_str("\r\n");
        stream.write_all(request.as_bytes()).await.expect("HTTP write");
        let mut response = Vec::new();
        tokio::time::timeout(std::time::Duration::from_secs(5), stream.read_to_end(&mut response)).await.expect("HTTP deadline").expect("HTTP read");
        let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").expect("HTTP header boundary");
        let head = std::str::from_utf8(&response[..boundary]).expect("HTTP headers").to_string();
        let status = head.split_whitespace().nth(1).expect("HTTP status").parse().expect("numeric HTTP status");
        RawHttpResponse { status, headers: head, body: response[boundary + 4..].to_vec() }
    }

    #[test]
    fn canonical_pair_route_rejects_non_path_and_ambiguous_headers_before_work() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer session.v1.selector.proof".parse().expect("authorization"));
        assert!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers).is_ok());
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair?checkpoint=other".parse().expect("URI"), &headers), Err(StatusCode::BAD_REQUEST));
        headers.insert(axum::http::header::RANGE, "bytes=0-1".parse().expect("range"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::RANGE_NOT_SATISFIABLE));
        headers.remove(axum::http::header::RANGE);
        headers.insert(axum::http::header::ACCEPT, "application/octet-stream".parse().expect("accept"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::NOT_ACCEPTABLE));
        headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
        headers.append(axum::http::header::AUTHORIZATION, "Bearer duplicate".parse().expect("duplicate"));
        assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
        assert!(!canonical_pair_auth_outcome_allowed(&AuthOutcome::Public));
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
        state.admin_subjects = Arc::from([AdminSubject {
            provider: "test-verifier".into(),
            subject_digest: identity_subject_digest("test-verifier", "canonical-outsider@example.com").expect("admin subject digest"),
        }]);
        let other_space = create_space_for_test(&state, "another-owner", "Canonical Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &other_space, document_id).await;
        publish_checkpoint_for_test(&state, &other_space, document_id).await;
        let public_space = create_space_for_test(&state, "public-owner", "Canonical Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
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

    fn hello(actor: &str, token: Option<&str>) -> ClientFrame {
        ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0x11u8; 32], actor: ActorId(actor.to_string()), token: token.map(str::to_string), resume_token: None, frontier: None }
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
        state.admin_subjects = Arc::from([AdminSubject {
            provider: "test-verifier".into(),
            subject_digest: identity_subject_digest("test-verifier", email).expect("test admin subject digest"),
        }]);
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
        let subject = SocketSubjectV1::Session {
            session_id: "session-a".into(),
            user_id: "user-a".into(),
            authorization_generation: 7,
            role: Some(SpaceRole::Author),
            expires_at_ms: 10_000,
        };
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

    #[test]
    fn socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live() {
        run_socket_test(|| async {
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-a").await;
            announce_document_for_test(&state, STUDIO, "socket-b").await;
            let nonmember = issue_test_session(&state, "socket-nonmember@example.com").await;
            let unauthorized_existing = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-a".to_string())),
                bearer_headers(&nonmember.token),
                State(state.clone()),
            )
            .await
            .err();
            let unauthorized_missing = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-missing".to_string())),
                bearer_headers(&nonmember.token),
                State(state.clone()),
            )
            .await
            .err();
            assert_eq!(unauthorized_existing, Some(StatusCode::UNAUTHORIZED));
            assert_eq!(unauthorized_missing, unauthorized_existing, "unauthorized callers cannot enumerate descriptor existence");
            let receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-a".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue document socket grant")
            .0;
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

            let legacy_receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-a".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue legacy-carrier rejection grant")
            .0;
            let (mut legacy, _) = connect_async(socket_request(&url, &legacy_receipt.grant)).await.expect("legacy rejection socket");
            legacy.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("initial socket hello");
            assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Session { .. }));
            legacy.send(client_binary(&hello("forged-legacy-actor", Some(&token)), Lane::Command).await).await.expect("legacy credential frame");
            assert_eq!(next_close_code(&mut legacy, false).await, 4401, "v1 rejects the legacy actor/token carrier after upgrade");

            let replay = connect_async(socket_request(&url, &receipt.grant)).await.expect_err("consumed grant replay rejected");
            assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
            let pending = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-a".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue pending grant")
            .0;
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
            let receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-linearized".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue socket grant")
            .0;
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
            let receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-command-revoke".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue socket grant")
            .0;
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
            socket
                .send(client_binary(&ClientFrame::Commands { batch_id: 90, envelopes: vec![accepted] }, Lane::Command).await)
                .await
                .expect("control command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("control command boundary deadline").expect("control command boundary");
            live_gate.socket_command_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Ack { batch_id: 90, .. }));
            let accepted_frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("accepted frontier");
            assert_eq!(accepted_frontier.head_seq, 1, "an actor-matching command persists while authorized");

            let mut revoked = sample_envelope("revoked-op", &document).await;
            revoked.actor = ActorId(receipt.actor_id.clone());
            socket
                .send(client_binary(&ClientFrame::Commands { batch_id: 91, envelopes: vec![revoked] }, Lane::Command).await)
                .await
                .expect("revoked command received by server");
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
            let receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-lag-revoke".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue socket grant")
            .0;
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
            let receipt = issue_document_socket_grant(
                Path((STUDIO.to_string(), "socket-broadcast-revoke".to_string())),
                bearer_headers(&token),
                State(state.clone()),
            )
            .await
            .expect("issue socket grant")
            .0;
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
            state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: since });
            assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Heartbeat { head_seq } if head_seq == since));
            assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
            assert_eq!(next_close_code(&mut socket, false).await, 4401);
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
            assert_eq!(
                tokio::time::timeout(std::time::Duration::from_secs(2), delete_session_me(bearer_headers(&token), State(state))).await.expect("bounded revoke"),
                StatusCode::NO_CONTENT,
            );
            gate.socket_directory_release.add_permits(1);
            assert_eq!(next_close_code(&mut socket, false).await, 4401, "no replay text crosses a winning revoke");
        });
    }

    #[tokio::test]
    async fn socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke() {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let admin_headers = authorize_test_admin(&mut state, "socket-admin@example.com").await;
        let target = issue_test_session(&state, "socket-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "socket-target@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, STUDIO, "socket-admin-race").await;
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let user_id = target.user_id.clone();
            async move { admin_revoke_user_sessions(Path(user_id), admin_headers, loopback_peer(), State(state)).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_admin_revoke_admitted.acquire()).await.expect("admin gate deadline").expect("admin gate");
        let mut issue = tokio::spawn({
            let state = state.clone();
            let token = target.token.clone();
            async move {
                issue_document_socket_grant(Path((STUDIO.to_string(), "socket-admin-race".to_string())), bearer_headers(&token), State(state)).await
            }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut issue).await.is_err(), "same-user grant waits behind batch revoke");
        gate.socket_admin_revoke_release.add_permits(1);
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("bounded admin revoke").expect("admin task"), StatusCode::NO_CONTENT);
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
        let event = state
            .directory
            .events_since(0, 100)
            .await
            .expect("directory events")
            .into_iter()
            .find(|event| event.space_id.as_deref() == Some(public_space.as_str()))
            .expect("public-space event");
        let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
        let record = SocketGrantRecordV1 {
            selector: "visibility".into(),
            secret_digest: [0; 32],
            audience,
            actor_id: "hub.v1.visibility".into(),
            subject: SocketSubjectV1::Session {
                session_id: session.id,
                user_id: session.user_id,
                authorization_generation: session.authorization_generation,
                role: None,
                expires_at_ms: session.expires_at,
            },
            issued_at_ms: session.issued_at,
            expires_at_ms: session.expires_at,
            state: SocketGrantStateV1::Consumed,
        };
        assert_eq!(socket_directory_message_visible(&state, &record, &DirectoryStreamMessage::Event { event }).await, SocketBindingValidityV1::Unauthorized);
    }

    // 🔬️ WS duplex fan-out over the real wire-v2 protocol: A's committed command reaches B on its
    // own socket as a `ServerFrame::Commands`, and B's Ack for A's own submit never round-trips
    // back to A as a duplicate Commands frame (origin filtering is the caller's job — this test
    // only asserts B observes it, matching `framework/sync`'s own origin check).
    #[tokio::test]
    async fn ws_duplex_fan_out() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "default").await;
        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/default/ws");

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));

        let (mut b, _) = connect_async(&url).await.unwrap();
        b.send(client_binary(&hello("B", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Session { .. }));

        let document = db_artifact_id(&DocumentScope::new(STUDIO, "default"));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document).await] }, Lane::Command).await).await.unwrap();

        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { batch_id: 1, .. }));

        loop {
            match next_server_frame(&mut b).await {
                ServerFrame::Commands { envelopes, origin, .. } => {
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].mutation_id.0, "op-1");
                    assert_eq!(origin, ActorId("A".to_string()));
                    break;
                }
                ServerFrame::Presence { .. } => continue,
                other => panic!("unexpected frame on B: {other:?}"),
            }
        }
    }

    #[test]
    fn document_socket_forced_lag_sends_verified_control_then_closes_1013() {
        run_socket_test(|| async {
        let mut state = lag_test_state(1024, 1).await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "lagged-document").await;
        let checkpoint = publish_checkpoint_for_test(&state, STUDIO, "lagged-document").await;
        let addr = spawn_server(state.clone()).await;
        let (mut ws, _) = connect_async(format!("ws://{addr}/spaces/{STUDIO}/documents/lagged-document/ws")).await.expect("connect document socket");
        ws.send(client_binary(&hello("lagged-actor", Some(&token)), Lane::Command).await).await.expect("send hello");
        assert!(matches!(next_server_frame(&mut ws).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut ws).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.document_subscribed.acquire()).await.expect("document subscription deadline").expect("document subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "lagged-document")));
        fanout.send(ServerFrame::Presence { peers: vec![b"first".to_vec()] }).expect("first fanout");
        fanout.send(ServerFrame::Presence { peers: vec![b"second".to_vec()] }).expect("second fanout");
        gate.document_release.add_permits(1);
        match next_server_frame(&mut ws).await {
            ServerFrame::RebootstrapRequired { control } => {
                assert_eq!(control.space_id, STUDIO);
                assert_eq!(control.document_id, "lagged-document");
                assert_eq!(control.checkpoint_id, checkpoint.checkpoint_id.0);
                assert_eq!(control.descriptor_hash, checkpoint.descriptor_digest_v1.0);
                assert_eq!(control.baseline_frontier.document_id.0, "lagged-document");
            }
            other => panic!("expected verified rebootstrap control, got {other:?}"),
        }
        assert_eq!(next_close_code(&mut ws, false).await, 1013);
        });
    }

    // 🔬️ A reconnecting client whose `Hello.frontier` is stale gets the missing commands replayed
    // via `Welcome`'s `Bootstrap::Tail` follow-up — the `db::Database::hello` integration.
    #[tokio::test]
    async fn reconnect_replays_missing_commands_via_bootstrap_tail() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "default").await;
        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/default/ws");
        let document = db_artifact_id(&DocumentScope::new(STUDIO, "default"));

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document).await] }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { .. }));

        // A fresh connection with no prior frontier must see the already-committed op-1 in its
        // Welcome bootstrap follow-up, sent BEFORE the connection's own `Session` frame (contract
        // §C7.3: Session is sent after Welcome AND its follow-up bootstrap frames).
        let (mut c, _) = connect_async(&url).await.unwrap();
        c.send(client_binary(&hello("C", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Welcome { bootstrap: Bootstrap::Tail, .. }));
        match next_server_frame(&mut c).await {
            ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes[0].mutation_id.0, "op-1"),
            other => panic!("expected the Tail bootstrap's Commands follow-up, got {other:?}"),
        }
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Session { .. }));
    }

    // 🔬️ Space-scoped documents: the same document id in two different studios lands in two
    // independent `db` documents through the structural v1 scope key — a peer on
    // space-b's `shared-doc` never observes space-a's commands.
    #[tokio::test]
    async fn space_scoped_documents_are_isolated() {
        let state = test_state().await;
        let space_a = create_space_for_test(&state, "seed", "Space A", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let space_b = create_space_for_test(&state, "seed", "Space B", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, &space_a, "shared-doc").await;
        announce_document_for_test(&state, &space_b, "shared-doc").await;
        let addr = spawn_server(state).await;

        let url_a = format!("ws://{addr}/spaces/{space_a}/documents/shared-doc/ws");
        let (mut a, _) = connect_async(&url_a).await.unwrap();
        a.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));
        let document = db_artifact_id(&DocumentScope::new(&space_a, "shared-doc"));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("only-in-a", &document).await] }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { .. }));

        let url_b = format!("ws://{addr}/spaces/{space_b}/documents/shared-doc/ws");
        let (mut b, _) = connect_async(&url_b).await.unwrap();
        b.send(client_binary(&hello("B", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { bootstrap: Bootstrap::None, .. }), "space-b's document must not see space-a's committed op");
    }

    // 🔬️ A share grant is bound to one space/document, read-only, and revocable through its
    // non-secret id. A private document never admits a tokenless caller.
    #[tokio::test]
    async fn share_token_is_scoped_read_only_and_revocable() {
        let state = test_state().await;
        let other_space = create_space_for_test(&state, "seed", "Other Share Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, STUDIO, "guarded").await;
        let mut admin_state = state;
        let headers = authorize_test_admin(&mut admin_state, "share-admin@example.com").await;
        let addr = spawn_server(admin_state.clone()).await;
        let share = create_share(Path((STUDIO.to_string(), "guarded".to_string())), axum::extract::Query(CreateShareQuery::default()), headers.clone(), loopback_peer(), State(admin_state.clone())).await.expect("share");

        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/guarded/ws");
        let (mut denied, _) = connect_async(&url).await.unwrap();
        denied.send(client_binary(&hello("intruder", None), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut denied).await, ServerFrame::Error { code, .. } if code == "unauthorized"));

        let other_url = format!("ws://{addr}/spaces/{other_space}/documents/guarded/ws");
        let (mut cross_space, _) = connect_async(&other_url).await.unwrap();
        cross_space.send(client_binary(&hello("cross-space", Some(&share.0.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut cross_space).await, ServerFrame::Error { code, .. } if code == "unauthorized"));

        let (mut allowed, _) = connect_async(&url).await.unwrap();
        allowed.send(client_binary(&hello("holder", Some(&share.0.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut allowed).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut allowed).await, ServerFrame::Session { .. }));
        let document = db_artifact_id(&DocumentScope::new(STUDIO, "guarded"));
        allowed.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("share-write", &document).await] }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut allowed).await, ServerFrame::Ack { stages, .. } if matches!(&stages[0], AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Rejected { .. }))));

        assert_eq!(revoke_share(Path((STUDIO.to_string(), "guarded".to_string(), share.0.id.clone())), headers, loopback_peer(), State(admin_state)).await, StatusCode::NO_CONTENT);
        let closed = tokio::time::timeout(std::time::Duration::from_secs(3), allowed.next()).await.expect("revoked share socket closes promptly");
        assert!(matches!(closed, Some(Ok(WsMessage::Close(_))) | None | Some(Err(_))));
        let (mut revoked, _) = connect_async(&url).await.unwrap();
        revoked.send(client_binary(&hello("revoked", Some(&share.0.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut revoked).await, ServerFrame::Error { code, .. } if code == "unauthorized"));
    }

    // 🔬️ `SecurityGate` wiring: a `Spectator` session may `Hello` into a document (reads are
    // unaffected) but a `Commands` submission is rejected — `admit_writes` catches it via
    // `space_grants`'s role-scoped `RoleBasedPolicy` before `db::ArtifactHandle::submit` ever
    // runs — while an `Author` session on the same space succeeds, proving the gate is wired into
    // the real write path rather than merely unit-tested in isolation.
    #[tokio::test]
    async fn security_gate_rejects_spectator_writes_and_allows_author_writes() {
        let state = test_state().await;
        let space = create_space_for_test(&state, "seed", "Gated Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;

        let spectator_session = issue_test_session(&state, "spectator@example.com").await;
        upsert_member_for_test(&state, &space, "spectator@example.com", DirectorySpaceRole::Spectator).await;

        let author_session = issue_test_session(&state, "author@example.com").await;
        upsert_member_for_test(&state, &space, "author@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, &space, "gated-doc").await;

        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{space}/documents/gated-doc/ws");
        let document = db_artifact_id(&DocumentScope::new(&space, "gated-doc"));
        let hello_with_token = |actor: &str, token: String| ClientFrame::Hello {
            wire_version: 1,
            protocol_version: 1,
            schema: "test.v1".to_string(),
            pack_schema_hash: [0x11u8; 32],
            actor: ActorId(actor.to_string()),
            token: Some(token),
            resume_token: None,
            frontier: None,
        };

        let (mut spectator, _) = connect_async(&url).await.unwrap();
        spectator.send(client_binary(&hello_with_token("spectator", spectator_session.token), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut spectator).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut spectator).await, ServerFrame::Session { .. }));
        spectator.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-spectator", &document).await] }, Lane::Command).await).await.unwrap();
        match next_server_frame(&mut spectator).await {
            ServerFrame::Ack { stages, .. } => {
                assert_eq!(stages.len(), 1);
                match &stages[0] {
                    AckStage::Applied { outcome } => assert!(matches!(outcome.as_ref(), ApplyOutcome::Rejected { .. }), "a spectator's write must be rejected by the security gate"),
                    _ => panic!("expected a single Applied stage"),
                }
            }
            _ => panic!("expected an ack frame"),
        }

        let (mut author, _) = connect_async(&url).await.unwrap();
        author.send(client_binary(&hello_with_token("author", author_session.token), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Session { .. }));
        author.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-author", &document).await] }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut author).await, ServerFrame::Ack { batch_id: 1, .. }));
    }

    // 🔬️ Blob round-trip: PUT then GET returns identical bytes and HEAD reports found, through
    // `db::Database`'s own content-addressed payload store; a hash that was never PUT is reported
    // missing by both GET and HEAD.
    #[tokio::test]
    async fn blob_put_get_head_round_trip() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
        let bytes = Bytes::copy_from_slice(fixture["blobUtf8"].as_str().expect("blob fixture string").as_bytes());
        let mut expected_pages = db::db_storage::db_io_copy_pages(bytes.as_ref()).unwrap().await.unwrap();
        let expected_hash = db::db_storage::db_io_hash_pages(&expected_pages).await.to_string();
        while !expected_pages.terminal_is_empty() {
            expected_pages.close_step().unwrap();
            semio_framework_async::yield_once().await;
        }
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_TYPE, "text/plain".parse().unwrap());
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let put = put_blob(Path((STUDIO.to_string(), expected_hash.clone())), headers.clone(), State(state.clone()), bytes.clone()).await.expect("put blob");
        assert_eq!(put.0.hash, expected_hash);
        assert_eq!(put.0.size, bytes.len() as i64);

        let response = get_blob(Path((STUDIO.to_string(), expected_hash.clone())), headers.clone(), State(state.clone())).await.expect("get blob").into_response();
        let got = axum::body::to_bytes(response.into_body(), HUB_BLOB_MAX_BYTES).await.expect("read bounded body");
        assert_eq!(got.as_ref(), bytes.as_ref());

        assert_eq!(head_blob(Path((STUDIO.to_string(), expected_hash.clone())), headers.clone(), State(state.clone())).await, StatusCode::OK);

        let missing = "0".repeat(64);
        assert_eq!(head_blob(Path((STUDIO.to_string(), missing.clone())), headers.clone(), State(state.clone())).await, StatusCode::NOT_FOUND);
        assert_eq!(get_blob(Path((STUDIO.to_string(), missing)), headers, State(state)).await.err(), Some(StatusCode::NOT_FOUND));
    }

    #[test]
    fn db_errors_lower_to_exact_http_status_classes() {
        assert_eq!(db_error_status(&db::DbError::InvalidArgument("invalid".into())), StatusCode::BAD_REQUEST);
        assert_eq!(db_error_status(&db::DbError::NotFound("missing".into())), StatusCode::NOT_FOUND);
        assert_eq!(db_error_status(&db::DbError::Conflict("conflict".into())), StatusCode::CONFLICT);
        assert_eq!(db_error_status(&db::DbError::Unavailable("offline".into())), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(db_error_status(&db::DbError::Internal("internal".into())), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // 🔬️ A client-provided hash that doesn't match the computed content hash is a bad request.
    #[tokio::test]
    async fn blob_put_rejects_hash_mismatch() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let bytes = Bytes::from_static(b"mismatched content");
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let result = put_blob(Path((STUDIO.to_string(), "0".repeat(64))), headers, State(state), bytes).await;
        assert_eq!(result.err(), Some(StatusCode::BAD_REQUEST));
    }

    // 🔬️ A `visibility == "public"` space grants an anonymous caller an implicit
    // `AuthOutcome::Public` — the hub-handler-level fallback, never a policy-engine concept (see
    // `AuthOutcome::Public`'s doc); a private space with the same shape stays denied.
    #[tokio::test]
    async fn public_visibility_grants_anonymous_spectator_fallback() {
        let state = test_state().await;
        let public_space = create_space_for_test(&state, "seed", "Public Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let private_space = create_space_for_test(&state, "seed", "Private Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        assert!(matches!(resolve_auth(&state, &public_space, "guarded-doc", None).await, AuthOutcome::Public));
        assert!(matches!(resolve_auth(&state, &private_space, "guarded-doc", None).await, AuthOutcome::Denied));
    }

    // 🔬️ A verifier-issued test session resolves the caller's space role and grants access even
    // to a document a share capability has otherwise closed.
    #[tokio::test]
    async fn auth_session_grants_role_and_bypasses_share_gate() {
        let state = test_state().await;
        // `hub_space_membership.space_id` is FK-bound to `hub_space(id)` — a real studio, not a
        // bare string; the session subject must resolve to a real projected user row.
        let studio = create_space_for_test(&state, "seed", "Space X", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let document = "closed-doc";
        let scope = DocumentScope::new(&studio, document);
        state.directory.issue_share_token(&scope, 60, "auth-session-test").await.expect("close with share token");

        let minted = issue_test_session(&state, "dev@example.com").await;
        upsert_member_for_test(&state, &studio, "dev@example.com", DirectorySpaceRole::Spectator).await;

        assert!(!authorized(&state, &studio, document, None).await, "tokenless request still denied");
        assert!(authorized(&state, &studio, document, Some(&minted.token)).await, "session token authorized despite no share token");

        match resolve_auth(&state, &studio, document, Some(&minted.token)).await {
            AuthOutcome::Session { user_id, role, .. } => {
                assert_eq!(user_id, minted.user_id);
                assert_eq!(role, SpaceRole::Spectator);
            }
            _ => panic!("expected a resolved session"),
        }
    }

    // 🔬️ GET .../documents/{id} reports a durably announced document's current frontier.
    #[tokio::test]
    async fn document_status_reports_frontier_and_lazily_mints() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "fresh").await;
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let status = get_document_status(Path((STUDIO.to_string(), "fresh".to_string())), headers.clone(), State(state.clone())).await.expect("status");
        assert_eq!(status.0.head_seq, 0);

        let scope = DocumentScope::new(STUDIO, "fresh");
        let handle = state.ensure_document(&db_artifact_id(&scope)).await.expect("ensure");
        let batch = db::document::CommandBatch::new(vec![sample_envelope("op-1", &db_artifact_id(&scope)).await]).await.unwrap();
        handle.submit(batch, db::document::SubmitOptions::default()).await.unwrap().unwrap();

        let status = get_document_status(Path((STUDIO.to_string(), "fresh".to_string())), headers, State(state)).await.expect("status after submit");
        assert_eq!(status.0.head_seq, 1);
    }

    async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, ws.next()).await {
                Ok(Some(Ok(WsMessage::Text(text)))) => return directory::os_pack::json::from_json_str(&text).expect("directory stream message decodes"),
                Ok(Some(Ok(_))) => continue,
                Ok(Some(other)) => panic!("expected a text frame, got {other:?}"),
                Ok(None) => panic!("stream ended before a directory message"),
                Err(_) => panic!("no directory message before the 5s deadline"),
            }
        }
    }

    async fn assert_no_directory_message<S>(ws: &mut S)
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        match tokio::time::timeout(std::time::Duration::from_millis(250), ws.next()).await {
            Err(_) => {}
            Ok(other) => panic!("private directory frame leaked: {other:?}"),
        }
    }

    // 🔬️ `POST /directory/commands` -> `DirectoryService::execute` -> `HubDirectory::append_events`
    // -> `GET /directory/spaces` re-folds the log and projects the caller's own role/member count.
    #[tokio::test]
    async fn directory_commands_append_events_and_project() {
        let state = test_state().await;
        let space_id = create_space_for_test(&state, "seed", "Atelier Alpha", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;

        let events = load_all_directory_events(state.directory.as_ref(), 0).await.expect("events");
        assert!(events.iter().any(|event| matches!(&event.body, os_directory::DirectoryEventBody::SpaceCreated { space_id: id, .. } if id == &space_id)));
        assert!(events.iter().any(|event| matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id: id, user_id, .. } if id == &space_id && user_id == "seed")));

        let owner_session = issue_test_session(&state, "owner-user@example.com").await;
        upsert_member_for_test(&state, &space_id, "owner-user@example.com", DirectorySpaceRole::Author).await;

        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", owner_session.token).parse().unwrap());
        let spaces = get_directory_spaces(headers, State(state)).await.expect("list spaces");
        let projected = spaces.0.iter().find(|space| space.id == space_id).expect("projected space");
        assert_eq!(projected.name, "Atelier Alpha");
        assert_eq!(projected.role, Some(DirectorySpaceRole::Author));
        assert_eq!(projected.member_count, 2, "the synthetic owner actor plus the newly granted author");
    }

    #[tokio::test]
    async fn complete_directory_event_reads_cross_the_fixed_page_boundary_without_gaps() {
        let count = DIRECTORY_EVENT_READ_MAX + 1;
        let source = SyntheticDirectoryEventSource { head: u64::try_from(count).expect("bounded count"), requests: std::sync::Mutex::new(Vec::new()) };
        let loaded = load_all_directory_events(&source, 0).await.expect("load all paged events");
        assert_eq!(loaded.len(), count);
        assert_eq!(loaded.first().map(|event| event.seq), Some(1));
        assert_eq!(loaded.last().map(|event| event.seq), Some(u64::try_from(count).expect("bounded count")));
        assert_eq!(source.requests.into_inner().expect("request lock"), vec![(0, DIRECTORY_EVENT_READ_MAX), (u64::try_from(DIRECTORY_EVENT_READ_MAX).expect("bounded page size"), DIRECTORY_EVENT_READ_MAX)]);
    }

    #[tokio::test]
    async fn document_announcement_requires_author_and_descriptor_reads_revalidate_membership() {
        let mut state = test_state().await;
        let admin_headers = authorize_test_admin(&mut state, "descriptor-admin@example.com").await;
        let author_token = seed_author_token(&state).await;
        let spectator = issue_test_session(&state, "descriptor-reader@example.com").await;
        upsert_member_for_test(&state, STUDIO, "descriptor-reader@example.com", DirectorySpaceRole::Spectator).await;
        let descriptor = document_descriptor_for_test(STUDIO, "authorized-document");

        let mut spectator_headers = HeaderMap::new();
        spectator_headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", spectator.token).parse().unwrap());
        let denied = post_directory_commands(spectator_headers.clone(), loopback_peer(), State(state.clone()), Json(DirectoryJson(DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }))).await;
        assert_eq!(denied.err(), Some(StatusCode::FORBIDDEN));

        let mut author_headers = HeaderMap::new();
        author_headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {author_token}").parse().unwrap());
        assert_eq!(get_document_status(Path((STUDIO.into(), "unannounced".into())), author_headers.clone(), State(state.clone())).await.err(), Some(StatusCode::NOT_FOUND));
        assert_eq!(create_share(Path((STUDIO.into(), "unannounced".into())), axum::extract::Query(CreateShareQuery::default()), admin_headers, loopback_peer(), State(state.clone())).await.err(), Some(StatusCode::NOT_FOUND));
        let announced = post_directory_commands(author_headers.clone(), loopback_peer(), State(state.clone()), Json(DirectoryJson(DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }))).await.expect("author announces document");
        assert_eq!(announced.1 .0.events.len(), 1);
        let replay =
            post_directory_commands(author_headers.clone(), loopback_peer(), State(state.clone()), Json(DirectoryJson(DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() }))).await.expect("identical descriptor is idempotent");
        assert!(replay.1 .0.events.is_empty());

        let mut conflict = descriptor.clone();
        conflict.pack_schema_hash = "44".repeat(32);
        let rejected = post_directory_commands(author_headers.clone(), loopback_peer(), State(state.clone()), Json(DirectoryJson(DirectoryCommand::AnnounceDocument { descriptor: conflict }))).await;
        assert_eq!(rejected.err(), Some(StatusCode::CONFLICT));

        let detail = get_directory_space(Path(STUDIO.to_string()), spectator_headers.clone(), State(state.clone())).await.expect("member reads space detail");
        assert_eq!(detail.0.documents.len(), 1);
        assert_eq!(detail.0.documents[0].descriptor, descriptor);

        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".into() };
        state.directory_service.execute(actor, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id: spectator.user_id }).await.expect("revoke membership");
        assert_eq!(get_directory_space(Path(STUDIO.to_string()), spectator_headers, State(state)).await.err(), Some(StatusCode::NOT_FOUND));
    }

    #[tokio::test]
    async fn document_open_rejects_missing_or_conflicting_descriptor_before_db_creation() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "known-document").await;
        let addr = spawn_server(state.clone()).await;

        let missing_url = format!("ws://{addr}/spaces/{STUDIO}/documents/missing-document/ws");
        let (mut missing, _) = connect_async(&missing_url).await.unwrap();
        missing.send(client_binary(&hello("missing", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut missing).await, ServerFrame::Error { code, .. } if code == "document-not-announced"));
        assert!(state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "missing-document"))).await.is_err());

        let known_url = format!("ws://{addr}/spaces/{STUDIO}/documents/known-document/ws");
        let (mut conflict, _) = connect_async(&known_url).await.unwrap();
        let mut conflicting_hello = hello("conflict", Some(&token));
        if let ClientFrame::Hello { pack_schema_hash, .. } = &mut conflicting_hello {
            *pack_schema_hash = [0x44; 32];
        }
        conflict.send(client_binary(&conflicting_hello, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut conflict).await, ServerFrame::Error { code, .. } if code == "schema-hash-mismatch"));
        assert!(state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "known-document"))).await.is_err());

        let (mut valid, _) = connect_async(&known_url).await.unwrap();
        valid.send(client_binary(&hello("valid", Some(&token)), Lane::Command).await).await.unwrap();
        let frame = next_server_frame(&mut valid).await;
        assert!(matches!(frame, ServerFrame::Welcome { .. }), "expected valid descriptor welcome, got {frame:?}");
        assert_eq!(state.directory.get_document_descriptor(&DocumentScope::new(STUDIO, "known-document")).await.unwrap(), Some(document_descriptor_for_test(STUDIO, "known-document")));
    }

    // 🔬️ `/directory/ws?since=0`: subscribe-then-replay is visibility-filtered exactly like `GET
    // /directory/events` — B only ever sees events for spaces B belongs to, both in the replay
    // (events already committed before B connects) and in the live tail (events committed after).
    #[tokio::test]
    async fn directory_ws_replays_then_streams_live() {
        let state = test_state().await;
        let space_mine = create_space_for_test(&state, "seed", "B's Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let space_other = create_space_for_test(&state, "seed", "Other Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let b_session = issue_test_session(&state, "b@example.com").await;
        upsert_member_for_test(&state, &space_other, "someone-else@example.com", DirectorySpaceRole::Spectator).await;
        upsert_member_for_test(&state, &space_mine, "b@example.com", DirectorySpaceRole::Author).await;

        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/ws?token={}&since=0", b_session.token);
        let (mut ws, _) = connect_async(&url).await.unwrap();

        // Replay contains only `space_mine`'s creation/owner membership and B's own membership.
        // Global identity events and all `space_other` events are private.
        let mut seen_spaces = std::collections::HashSet::new();
        let mut saw_own_membership = false;
        for _ in 0..3u32 {
            match next_directory_message(&mut ws).await {
                DirectoryStreamMessage::Event { event } => {
                    if let Some(space_id) = &event.space_id {
                        seen_spaces.insert(space_id.clone());
                    }
                    if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { user_id, .. } if user_id == &b_session.user_id) {
                        saw_own_membership = true;
                    }
                }
                other => panic!("expected an Event during replay, got {other:?}"),
            }
        }
        assert!(saw_own_membership, "B must see the replayed member.upserted naming them");
        assert_eq!(seen_spaces, std::collections::HashSet::from([space_mine.clone()]), "B must never see space_other's events");

        // Live: the same filter holds after B connects. Other identities and `space_other` remain
        // invisible; the next received event belongs to `space_mine`.
        upsert_member_for_test(&state, &space_other, "yet-another@example.com", DirectorySpaceRole::Spectator).await;
        upsert_member_for_test(&state, &space_mine, "second-member@example.com", DirectorySpaceRole::Spectator).await;
        loop {
            match next_directory_message(&mut ws).await {
                DirectoryStreamMessage::Event { event } if event.space_id.is_none() => continue,
                DirectoryStreamMessage::Event { event } => {
                    assert_eq!(event.space_id.as_deref(), Some(space_mine.as_str()), "space_other's live event must be dropped, not delivered");
                    break;
                }
                other => panic!("expected an Event message, got {other:?}"),
            }
        }
    }

    #[test]
    fn directory_socket_forced_lag_is_scope_authorized_and_closes_1013() {
        run_socket_test(|| async {
        let mut state = lag_test_state(1, 256).await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "authorized-lag").await;
        let authorized_checkpoint = publish_checkpoint_for_test(&state, STUDIO, "authorized-lag").await;
        let unrelated = issue_test_session(&state, "unrelated-owner@example.com").await;
        let denied_space = create_space_for_test(&state, &unrelated.user_id, "Denied Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &denied_space, "denied-lag").await;
        publish_checkpoint_for_test(&state, &denied_space, "denied-lag").await;
        let since = state.directory.head_seq().await.expect("directory head");
        let addr = spawn_server(state.clone()).await;
        let (mut authorized, _) = connect_async(format!("ws://{addr}/directory/ws?token={token}&since={since}&spaceId={STUDIO}&documentId=authorized-lag")).await.expect("authorized directory socket");
        let (mut denied, _) = connect_async(format!("ws://{addr}/directory/ws?token={token}&since={since}&spaceId={denied_space}&documentId=denied-lag")).await.expect("denied directory socket");
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_subscribed.acquire_many(2)).await.expect("directory subscription deadline").expect("directory subscriptions");
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: since });
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: since });
        gate.directory_release.add_permits(2);

        match next_directory_message(&mut authorized).await {
            DirectoryStreamMessage::RebootstrapRequired { control } => {
                assert_eq!(control.scope, DocumentScope::new(STUDIO, "authorized-lag"));
                assert_eq!(control.checkpoint_id, authorized_checkpoint.checkpoint_id);
                assert_eq!(control.descriptor_digest_v1, authorized_checkpoint.descriptor_digest_v1);
                assert_eq!(control.baseline_frontier, authorized_checkpoint.baseline_frontier);
                assert!(!directory::os_pack::json::to_json_string(&DirectoryStreamMessage::RebootstrapRequired { control }).contains("storageKey"));
            }
            other => panic!("expected authorized rebootstrap control, got {other:?}"),
        }
        assert_eq!(next_close_code(&mut authorized, false).await, 1013);
        assert_eq!(next_close_code(&mut denied, false).await, 1013);
        });
    }

    // 🔬️ The real directory socket never exposes another private space's connection,
    // presence, membership, or account-creation frames. An unauthenticated socket receives no
    // subscription at all, while an authorized member still receives their own space's activity.
    #[tokio::test]
    async fn directory_ws_isolates_private_realtime_activity_and_global_identity() {
        let state = test_state().await;
        let mine = create_space_for_test(&state, "seed", "Mine", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let private = create_space_for_test(&state, "seed", "Private", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let observer = issue_test_session(&state, "member-a@example.com").await;
        upsert_member_for_test(&state, &mine, "member-a@example.com", DirectorySpaceRole::Spectator).await;
        let other = issue_test_session(&state, "member-b@example.com").await;
        upsert_member_for_test(&state, &private, "member-b@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, &private, "shared").await;
        announce_document_for_test(&state, &mine, "shared").await;
        let since = state.directory.head_seq().await.unwrap();
        let addr = spawn_server(state.clone()).await;

        let (mut anonymous, _) = connect_async(format!("ws://{addr}/directory/ws?since={since}")).await.unwrap();
        match tokio::time::timeout(std::time::Duration::from_secs(1), anonymous.next()).await {
            Ok(Some(Ok(WsMessage::Text(text)))) => panic!("anonymous directory subscription leaked {text}"),
            Ok(_) => {}
            Err(_) => panic!("anonymous directory subscription was left open"),
        }

        let (mut stream, _) = connect_async(format!("ws://{addr}/directory/ws?token={}&since={since}", observer.token)).await.unwrap();
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: since });
        assert!(matches!(next_directory_message(&mut stream).await, DirectoryStreamMessage::Heartbeat { head_seq } if head_seq == since));

        let (mut private_doc, _) = connect_async(format!("ws://{addr}/spaces/{private}/documents/shared/ws")).await.unwrap();
        private_doc.send(client_binary(&hello("private-actor", Some(&other.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut private_doc).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut private_doc).await, ServerFrame::Session { .. }));
        private_doc.send(client_binary(&ClientFrame::Presence { peer: b"private-presence".to_vec() }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut private_doc).await, ServerFrame::Presence { .. }));
        upsert_member_for_test(&state, &private, "private-person@example.com", DirectorySpaceRole::Spectator).await;
        assert_no_directory_message(&mut stream).await;

        let (mut mine_doc, _) = connect_async(format!("ws://{addr}/spaces/{mine}/documents/shared/ws")).await.unwrap();
        mine_doc.send(client_binary(&hello("mine-actor", Some(&observer.token)), Lane::Command).await).await.unwrap();
        let mine_first = next_server_frame(&mut mine_doc).await;
        if !matches!(mine_first, ServerFrame::Welcome { .. }) {
            match &mine_first {
                ServerFrame::Error { code, message } => panic!("expected mine document Welcome, got frame={mine_first:?}, code={code:?}, message={message:?}"),
                _ => panic!("expected mine document Welcome, got frame={mine_first:?}"),
            }
        }
        assert!(matches!(next_server_frame(&mut mine_doc).await, ServerFrame::Session { .. }));
        match next_directory_message(&mut stream).await {
            DirectoryStreamMessage::Connection { connection, .. } => {
                assert_eq!(connection.space_id, mine);
                assert_eq!(connection.email.as_deref(), Some("member-a@example.com"));
            }
            other => panic!("expected authorized connection frame, got {other:?}"),
        }
    }

    // 🔬️ Contract §C7.0/§C7.3: the roster is document-wide now — A (`surface=editor`) and C
    // (`surface=viewer`) on the SAME document see each other's presence bytes; `surface` no longer
    // scopes any broadcast channel (`surface_fanout` is deleted, `ServerFrame::Presence` fans out on
    // the document-wide `fanout` alongside `Commands`) — it travels only INSIDE each peer's opaque
    // `PresencePeer` bytes, which this hub stores and forwards without ever decoding.
    #[tokio::test]
    async fn presence_roster_is_document_wide_and_frames_carry_surface_only_inside_peer() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "shared").await;
        let addr = spawn_server(state).await;
        let url_editor = format!("ws://{addr}/spaces/{STUDIO}/documents/shared/ws?surface=editor");
        let url_viewer = format!("ws://{addr}/spaces/{STUDIO}/documents/shared/ws?surface=viewer");

        let (mut a, _) = connect_async(&url_editor).await.unwrap();
        a.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Session { .. }));

        let (mut c, _) = connect_async(&url_viewer).await.unwrap();
        c.send(client_binary(&hello("C", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut c).await, ServerFrame::Session { .. }));

        a.send(client_binary(&ClientFrame::Presence { peer: b"A-presence".to_vec() }, Lane::Command).await).await.unwrap();
        loop {
            match next_server_frame(&mut c).await {
                ServerFrame::Presence { peers } => {
                    assert!(peers.contains(&b"A-presence".to_vec()), "C (different surface, SAME document) must see A's presence — the roster is document-wide, not surface-scoped");
                    break;
                }
                other => panic!("unexpected frame on C: {other:?}"),
            }
        }

        // A also subscribes to the SAME document-wide `fanout` it just published on, so its own
        // presence publish loops back to it too — proving there is only one channel now, not a
        // second surface-scoped one A alone would be on.
        match next_server_frame(&mut a).await {
            ServerFrame::Presence { peers } => assert!(peers.contains(&b"A-presence".to_vec()), "A observes its own publish via the document-wide fanout"),
            other => panic!("unexpected frame on A: {other:?}"),
        }
    }

    // 🔬️ Contract §C7.3 session colors: the lowest free index in `0..=255` per SPACE (not per
    // document) — A gets 0, B gets 1; A's second document socket in the SAME space reuses A's
    // existing lease (still 0, ref-counted) rather than minting a new one; once BOTH of A's sockets
    // close, color 0 is freed and a brand-new actor C is assigned it (B, still connected, keeps 1).
    #[tokio::test]
    async fn session_frame_assigns_lowest_free_color_per_space_and_releases_on_last_disconnect() {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "doc1").await;
        announce_document_for_test(&state, STUDIO, "doc2").await;
        let addr = spawn_server(state).await;
        let url = |document: &str| format!("ws://{addr}/spaces/{STUDIO}/documents/{document}/ws");

        async fn welcome_and_session<S>(ws: &mut S) -> (String, u8)
        where
            S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
        {
            assert!(matches!(next_server_frame(ws).await, ServerFrame::Welcome { .. }));
            match next_server_frame(ws).await {
                ServerFrame::Session { actor, color } => (actor, color),
                other => panic!("expected Session, got {other:?}"),
            }
        }

        let (mut a1, _) = connect_async(&url("doc1")).await.unwrap();
        a1.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert_eq!(welcome_and_session(&mut a1).await, ("A".to_string(), 0));

        let (mut b, _) = connect_async(&url("doc1")).await.unwrap();
        b.send(client_binary(&hello("B", Some(&token)), Lane::Command).await).await.unwrap();
        assert_eq!(welcome_and_session(&mut b).await, ("B".to_string(), 1));

        // A's second document socket, same space: the existing lease is reused (still 0), not a new
        // lowest-free index (which would otherwise be 2).
        let (mut a2, _) = connect_async(&url("doc2")).await.unwrap();
        a2.send(client_binary(&hello("A", Some(&token)), Lane::Command).await).await.unwrap();
        assert_eq!(welcome_and_session(&mut a2).await, ("A".to_string(), 0));

        drop(a1);
        drop(a2);
        // Let both of A's handler tasks observe the socket close and release their color lease's ref.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let (mut c, _) = connect_async(&url("doc1")).await.unwrap();
        c.send(client_binary(&hello("C", Some(&token)), Lane::Command).await).await.unwrap();
        assert_eq!(welcome_and_session(&mut c).await, ("C".to_string(), 0), "color 0 is freed once BOTH of A's document sockets disconnect, and is the lowest free index (B still holds 1)");
    }

    // 🔬️ Amendment 3 to C1: `DirectoryStreamMessage::Presence` is actually published (it used to be
    // defined but never sent) — `spaceId`/`documentId` name the roster, and each
    // `DirectoryPresenceActor` carries the `surface`/`color` this hub knows without ever decoding the
    // actor's opaque `PresencePeer` bytes.
    #[tokio::test]
    async fn directory_ws_publishes_presence_roster_with_surface_and_color() {
        let state = test_state().await;
        let observer_session = issue_test_session(&state, "presence-observer@example.com").await;
        upsert_member_for_test(&state, STUDIO, "presence-observer@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "watched-presence").await;
        let since = state.directory.head_seq().await.unwrap();
        let addr = spawn_server(state.clone()).await;
        let dir_url = format!("ws://{addr}/directory/ws?token={}&since={since}", observer_session.token);
        let (mut observer, _) = connect_async(&dir_url).await.unwrap();

        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/watched-presence/ws?surface=editor");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("presence-actor", Some(&observer_session.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));
        let color = match next_server_frame(&mut doc).await {
            ServerFrame::Session { color, .. } => color,
            other => panic!("expected Session, got {other:?}"),
        };

        doc.send(client_binary(&ClientFrame::Presence { peer: b"presence-actor-bytes".to_vec() }, Lane::Command).await).await.unwrap();

        loop {
            match next_directory_message(&mut observer).await {
                DirectoryStreamMessage::Connection { .. } => continue,
                DirectoryStreamMessage::Presence { space_id, document_id, actors } => {
                    assert_eq!(space_id, STUDIO);
                    assert_eq!(document_id, "watched-presence");
                    let actor = actors.iter().find(|actor| actor.actor == "presence-actor").expect("presence-actor in the published roster");
                    assert_eq!(actor.surface, "editor");
                    assert_eq!(actor.color, color);
                    break;
                }
                other => panic!("expected Presence, got {other:?}"),
            }
        }
    }

    // 🔬️ `record_sync_session_open`/`_close` publish `DirectoryStreamMessage::Connection{phase}` —
    // any `/directory/ws` observer sees a document WS session's open and close in real time.
    #[tokio::test]
    async fn connection_events_reach_admin_stream() {
        let state = test_state().await;
        let observer_session = issue_test_session(&state, "observer@example.com").await;
        upsert_member_for_test(&state, STUDIO, "observer@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "watched").await;
        let since = state.directory.head_seq().await.unwrap();
        let addr = spawn_server(state).await;
        let dir_url = format!("ws://{addr}/directory/ws?token={}&since={since}", observer_session.token);
        let (mut observer, _) = connect_async(&dir_url).await.unwrap();

        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/watched/ws");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("watched-actor", Some(&observer_session.token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));

        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Connection { phase, connection } => {
                assert_eq!(phase, DirectoryConnectionPhase::Opened);
                assert_eq!(connection.document_id, "watched");
                assert_eq!(connection.actor, "watched-actor");
            }
            other => panic!("expected Connection(Opened), got {other:?}"),
        }

        drop(doc);
        match next_directory_message(&mut observer).await {
            DirectoryStreamMessage::Connection { phase, .. } => assert_eq!(phase, DirectoryConnectionPhase::Closed),
            other => panic!("expected Connection(Closed), got {other:?}"),
        }
    }

    // 🔬️ Admin API round trip: spaces/users/connections list what setup created, `presenceKnown`
    // tracks whether that connection has published a `ClientFrame::Presence` yet, and closing a
    // listed connection's `syncSessionId` actually kicks the live document WS session.
    #[tokio::test]
    async fn admin_api_lists_spaces_users_connections_and_kicks() {
        let mut state = test_state().await;
        let admin_headers = authorize_test_admin(&mut state, "hub-admin@example.com").await;
        let space_id = create_space_for_test(&state, "seed", "Admin Visible Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let _ = issue_test_session(&state, "someone@example.com").await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "kickable").await;

        let addr = spawn_server(state.clone()).await;
        let doc_url = format!("ws://{addr}/spaces/{STUDIO}/documents/kickable/ws");
        let (mut doc, _) = connect_async(&doc_url).await.unwrap();
        doc.send(client_binary(&hello("kick-me", Some(&token)), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Session { .. }));
        // Let the server side finish recording the sync session before the admin reads it back.
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let spaces = admin_spaces(admin_headers.clone(), loopback_peer(), State(state.clone())).await.expect("admin spaces");
        assert!(spaces.0.iter().any(|space| space.id == space_id));

        let users = admin_users(admin_headers.clone(), loopback_peer(), State(state.clone())).await.expect("admin users");
        assert!(users.0.iter().any(|user| user.email == "someone@example.com"));

        let connections = admin_connections(admin_headers.clone(), loopback_peer(), State(state.clone())).await.expect("admin connections");
        let connection = connections.0.iter().find(|connection| connection.actor == "kick-me").expect("kickable connection listed");
        assert!(!connection.presence_known, "no ClientFrame::Presence published yet");

        doc.send(client_binary(&ClientFrame::Presence { peer: b"kick-me-presence".to_vec() }, Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut doc).await, ServerFrame::Presence { .. }));
        let connections = admin_connections(admin_headers.clone(), loopback_peer(), State(state.clone())).await.expect("admin connections after presence");
        let connection = connections.0.iter().find(|connection| connection.actor == "kick-me").expect("kickable connection still listed");
        assert!(connection.presence_known, "presenceKnown flips true once the actor's PresenceSession carries a peer");

        assert_eq!(admin_close_connection(Path(connection.sync_session_id.clone()), admin_headers, loopback_peer(), State(state.clone())).await, StatusCode::NO_CONTENT);
        let closed = tokio::time::timeout(std::time::Duration::from_secs(5), doc.next()).await.expect("connection closes before the 5s deadline");
        // The kicked session's task just `break`s its select loop and drops the socket — no clean WS
        // close handshake is sent, so the client observes either a `Close` frame, a stream end, or
        // (most commonly, since the TCP connection drops mid-handshake) a protocol error. Any of the
        // three means the connection is gone, which is what the kick promises.
        assert!(matches!(closed, Some(Ok(WsMessage::Close(_))) | None | Some(Err(_))), "the kicked connection must close, got {closed:?}");
    }

    // 🔬️ Administrator authority is attached only to a verified subject. Loopback proximity
    // and arbitrary bearer strings never confer it.
    #[tokio::test]
    async fn admin_requires_verified_subject_policy() {
        let mut state = test_state().await;
        assert!(!is_admin(&state, &HeaderMap::new(), Some(SocketAddr::from(([127, 0, 0, 1], 0)))).await);
        let headers = authorize_test_admin(&mut state, "admin-policy@example.com").await;
        assert!(is_admin(&state, &headers, Some(SocketAddr::from(([10, 0, 0, 5], 0)))).await);
        state.admin_subjects = Arc::from([]);
        assert!(!is_admin(&state, &headers, Some(SocketAddr::from(([127, 0, 0, 1], 0)))).await);
    }

    // 🔬️ `space.deleted` closes the space's document WS to a later Hello — `get_space` returns
    // `None` post-deletion, so `resolve_auth` falls straight past even its public-visibility
    // fallback into `AuthOutcome::Denied` (the space was deliberately created PUBLIC, so this proves
    // it is the deletion, not privacy, doing the denying).
    #[tokio::test]
    async fn deleted_space_denies_ws_hello() {
        let state = test_state().await;
        let space_id = create_space_for_test(&state, "seed", "Doomed Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
        state.directory_service.execute(actor, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete space");
        let addr = spawn_server(state).await;
        let url = format!("ws://{addr}/spaces/{space_id}/documents/gone/ws");
        let (mut ws, _) = connect_async(&url).await.unwrap();
        ws.send(client_binary(&hello("late-comer", None), Lane::Command).await).await.unwrap();
        assert!(matches!(next_server_frame(&mut ws).await, ServerFrame::Error { code, .. } if code == "unauthorized"));
    }

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
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
