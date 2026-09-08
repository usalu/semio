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
use axum::extract::{DefaultBodyLimit, OriginalUri, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use db::db_storage::PayloadStorage as _;
use directory::os_directory::{
    self, AdminConnectionSnapshotV1, AdminIntentOutcomeV1, AdminIntentReceiptV1, AdminIntentResultV1, AdminIntentStateV1, AdminIntentV1, AdminOperationAuditPhaseV1, AdminOperationAuditV1, AdminOperationProgressV1, AdminOperationStatusV1,
    AdminPageV1, AdminRecordedConnectionV1, ArtifactFrontier, ArtifactHash, CHECKPOINT_PUBLICATION_COMMAND_MAX_BYTES, CHECKPOINT_PUBLICATION_DEADLINE_MS, CHECKPOINT_PUBLICATION_PAIR_MAX_BYTES, CheckpointPublicationBlobV1,
    CheckpointPublicationCommandV1, CheckpointPublicationCurrentV1, CheckpointPublicationFrontierV1, CheckpointPublicationReceiptV1, ConnectionView, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES,
    DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES,
    DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER, DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryConnectionPhase,
    DirectoryEvent, DirectoryEventPageErrorV1, DirectoryEventPageV1, DirectoryPresenceActor, DirectoryReadModel, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteRowV1,
    DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceAdministrationPublicDocumentWindowV1,
    DirectorySpaceAdministrationSectionV1, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentExecutionTargetComponentV1, DocumentExecutionTargetDescriptorV1,
    DocumentExecutionTargetLeaseFieldsV1, DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1,
    DocumentOpenPlanErrorV1, DocumentOpenPlanV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceV1, DocumentPlanSocketGrantIntentV1, DocumentView, MemberSpaceViewV1, MemberView, PublicDocumentCatalogEntryV1, PublicSpaceViewV1,
    PublishedArtifactCheckpoint, SpaceView, descriptor_digest_v1, directory_command_sha256, same_lease_fields_v1, validate_directory_event_page_event,
};
use directory::os_spr::channel::{PRESENCE_ROSTER_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_ITEMS};
use directory::{DslValue, FromValue, ToValue};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use protocol::{AckStage, ActorId, ApplyOutcome, ArtifactId as ProtocolArtifactId, ClientFrame, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame, decode_client_frame, encode_server_frame};
use semio_framework_async::ShardedMap;
use semio_framework_hash::Sha256;
#[cfg(feature = "neo4j")]
use semio_hub::artifact_authority::chunk_cas::Neo4jArtifactChunkCasStorage;
#[cfg(feature = "postgres")]
use semio_hub::artifact_authority::chunk_cas::PostgresArtifactChunkCasStorage;
#[cfg(feature = "sqlite")]
use semio_hub::artifact_authority::chunk_cas::SqliteArtifactChunkCasStorage;
use semio_hub::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage, ArtifactChunkCasStores, FsArtifactChunkCasStorage};
#[cfg(test)]
use semio_hub::artifact_authority::chunk_cas::{MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
#[cfg(feature = "native-artifact-execution")]
use semio_hub::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
use semio_hub::artifact_authority::trusted_catalog::{NativeCodecProviderSourceV1, TrustedCatalogLoader, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedTrustedCatalog};
#[cfg(test)]
use semio_hub::artifact_authority::{ArtifactBlobIntegrity, ImmutableArtifactBlobStore};
use semio_hub::artifact_authority::{
    ArtifactPair, AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, CanonicalArtifactAuthority, CheckpointPublicationOrchestrator, CheckpointRequest, OperationContext, ValidatingCanonicalArtifactAuthority,
    VerifiedCheckpointPublisher,
};
#[cfg(any(test, feature = "native-artifact-execution"))]
use semio_hub::artifact_authority::creation::{ArtifactCreationActorV1, ArtifactCreationCommitAuthorityV1, ArtifactCreationCommitFutureV1, ArtifactCreationCommitLeaseV1, ArtifactCreationServiceV1};
#[cfg(any(test, feature = "native-artifact-execution"))]
use directory::os_directory::schema::space_artifact_creation::{SPACE_ARTIFACT_CREATION_MAX_BYTES, SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1};
use semio_hub::directory::error::DirectoryError;
#[cfg(test)]
use semio_hub::directory::model::AuthSessionIssue;
use semio_hub::directory::model::{
    AdminEffectCommitV1, AdminOperationAuditRecord, AuthSessionKind, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CheckpointPublicationDispositionV1, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion,
    DirectoryCommandReceiptRecord, DirectoryCommandResultKindV1, DocumentScope, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt,
    SocketSessionBindingStatus, SocketShareBindingStatus, SpaceRole, SyncSessionRecord,
};
#[cfg(feature = "sqlite")]
use semio_hub::directory::sqlite::SqliteDirectory;
use semio_hub::directory::{
    ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_INTENT_REQUEST_MAX_BYTES, ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, ArtifactCasSweepContinuation, ArtifactCasSweepRequest, ArtifactCasSweepResult, CAPABILITY_MAX_TTL_SECS, CommandResult,
    DIRECTORY_EVENT_READ_MAX, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DirectoryCommandExecutionV1, DirectoryService, HubDirectories, HubDirectory, HubVerifiedCheckpointPublisher, ProjectionRebuildControl, ProjectionRebuildProgress,
    SPACE_ADMINISTRATION_PAGE_FETCH_MAX, SPACE_ADMINISTRATION_PAGE_MAX, directory_command_result_kind, published_artifact_checkpoint, replay_directory_command_receipt,
};
use semio_hub::directory::{AUTH_TEXT_MAX_BYTES, HubCapability, IdentityAssertionVerifier, IdentityVerificationControl, InviteCapability, LocalBootstrapTransport, SessionCapability, SocketGrantCapability, identity_subject_digest};
#[cfg(all(test, feature = "sqlite", feature = "test-support"))]
use semio_hub::inference::runtime::UnavailableGisMapApprovalCommitterV1;
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
use semio_hub::inference::runtime::{
    GisMapApprovalCheckpointPublisherV1, GisMapApprovalCheckpointRequestV1, GisMapApprovalCommitErrorV1, GisMapApprovalIngressAuthorityV1, GisMapDocumentWriteAuthorityV1, HubInferenceRuntimeV1, InferenceApprovalRouteContextV1, InferenceRouteErrorV1,
    RetainedGisMapApprovalCommitterV1,
};
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution", feature = "test-support"))]
use semio_hub::inference::runtime::InferenceCheckpointTestGateV1;
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
use semio_hub::inference::sqlite::InferenceJobLedgerV1;
#[cfg(feature = "native-artifact-execution")]
use semio_hub::inference::{VerifiedGisMapArtifactBindingV1, verified_gis_map_binding};
#[cfg(test)]
use semio_hub::lag_rebootstrap::decode_canonical_checkpoint_pair;
use semio_hub::lag_rebootstrap::{
    CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, CanonicalPairTerminal, REBOOTSTRAP_DEADLINE_MS, RebootstrapContext, RebootstrapError, RebootstrapProgress, RebootstrapProgressStage, RebootstrapTransferControl, VerifiedRebootstrapSource,
    append_canonical_pair_data, append_canonical_pair_header, append_canonical_pair_terminal, canonical_pair_etag,
};
use semio_hub::local_bootstrap::{InheritedLocalBootstrapTransport, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS, serve_local_bootstrap};
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
    DatabaseOpen(db::engine::DatabaseOpenAtRejected),
    StorageOpen(db::db_storage::DbStorageOpenRejected),
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    InferenceShutdown(GisMapApprovalCommitErrorV1),
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
            Self::DatabaseOpen(error) => std::fmt::Display::fmt(error.error(), formatter),
            Self::StorageOpen(error) => std::fmt::Display::fmt(error.error(), formatter),
            #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
            Self::InferenceShutdown(error) => write!(formatter, "inference runtime shutdown failed: {error:?}"),
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
            Self::DatabaseOpen(error) => Some(error.error()),
            Self::StorageOpen(error) => Some(error.error()),
            #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
            Self::InferenceShutdown(_) => None,
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

impl From<db::engine::DatabaseOpenAtRejected> for HubError {
    fn from(error: db::engine::DatabaseOpenAtRejected) -> Self {
        Self::DatabaseOpen(error)
    }
}

impl From<db::db_storage::DbStorageOpenRejected> for HubError {
    fn from(error: db::db_storage::DbStorageOpenRejected) -> Self {
        Self::StorageOpen(error)
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
    /// 🧱 Exact-selection asset accessor. It never accepts a package, digest, path or receipt
    /// selector, and answers only while `current_generation` is still the catalog's own.
    fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets>;
}

impl DocumentOpenCatalogAuthorityV1 for VerifiedTrustedCatalog {
    fn generation_id(&self) -> &str {
        self.generation_id()
    }

    fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        self.resolve_document_open(descriptor, requested_surface_id, writable)
    }

    fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        self.assets_for_current_selection(descriptor, requested_surface_id, writable, current_generation)
    }
}

async fn configured_artifact_authority(data_dir: &std::path::Path, providers: Option<&dyn NativeCodecProviderSourceV1>) -> Result<Option<ConfiguredArtifactAuthority>, AuthorityError> {
    if providers.is_none() && data_dir.join("trusted-catalog/current.json").try_exists().map_err(|error| AuthorityError::Catalog(error.to_string()))? {
        return Err(AuthorityError::Catalog("configured trusted catalog requires the native-artifact-execution provider".into()));
    }
    let Some(providers) = providers else {
        return Ok(None);
    };
    let control = StartupCatalogControl;
    let started = control.now_ms();
    let context = OperationContext::new(started.saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let Some(catalog) = TrustedCatalogLoader::load_current(data_dir, providers, &context).await? else {
        return Ok(None);
    };
    let catalog = Arc::new(catalog);
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
    user_id: Option<String>,
    connected_at_ms: i64,
    label: Option<String>,
    role: Option<String>,
    document_surface: Option<String>,
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
    gate_ticks: std::sync::atomic::AtomicBool,
    tick_admitted: tokio::sync::Semaphore,
    tick_release: tokio::sync::Semaphore,
    tick_evaluated: tokio::sync::Semaphore,
}

#[cfg(test)]
impl TestPresenceClock {
    fn new() -> Self {
        Self {
            origin: tokio::time::Instant::now(),
            now_ms: std::sync::atomic::AtomicU64::new(0),
            gate_ticks: std::sync::atomic::AtomicBool::new(false),
            tick_admitted: tokio::sync::Semaphore::new(0),
            tick_release: tokio::sync::Semaphore::new(0),
            tick_evaluated: tokio::sync::Semaphore::new(0),
        }
    }

    fn now(&self) -> tokio::time::Instant {
        self.origin + std::time::Duration::from_millis(self.now_ms.load(std::sync::atomic::Ordering::SeqCst))
    }

    fn advance_to(&self, now_ms: u64) {
        self.now_ms.store(now_ms, std::sync::atomic::Ordering::SeqCst);
    }

    async fn evaluate_tick(&self, ungate: bool) {
        tokio::time::timeout(std::time::Duration::from_secs(5), self.tick_admitted.acquire()).await.expect("presence tick admission deadline").expect("presence tick admission").forget();
        if ungate {
            self.gate_ticks.store(false, std::sync::atomic::Ordering::Release);
        }
        self.tick_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(5), self.tick_evaluated.acquire()).await.expect("presence tick evaluation deadline").expect("presence tick evaluation").forget();
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
    socket_global_send_pause: Mutex<Option<(String, u8)>>,
    socket_global_send_admitted: tokio::sync::Semaphore,
    socket_session_revoke_attempted: tokio::sync::Semaphore,
    socket_global_send_release: tokio::sync::Semaphore,
    socket_scoped_send_mode: std::sync::atomic::AtomicU8,
    socket_scoped_send_admitted: tokio::sync::Semaphore,
    socket_scoped_send_release: tokio::sync::Semaphore,
    socket_membership_remove_enabled: std::sync::atomic::AtomicBool,
    socket_membership_remove_admitted: tokio::sync::Semaphore,
    socket_membership_remove_release: tokio::sync::Semaphore,
    directory_command_pause_user: Mutex<Option<(String, bool)>>,
    directory_command_attempted: tokio::sync::Semaphore,
    directory_command_admitted: tokio::sync::Semaphore,
    directory_command_release: tokio::sync::Semaphore,
    admin_effect_pause_enabled: std::sync::atomic::AtomicBool,
    admin_effect_admitted: tokio::sync::Semaphore,
    admin_effect_release: tokio::sync::Semaphore,
    directory_event_page_fence_enabled: std::sync::atomic::AtomicBool,
    directory_event_page_read_admitted: tokio::sync::Semaphore,
    directory_event_page_read_release: tokio::sync::Semaphore,
    directory_event_page_control: Mutex<Option<Arc<DirectoryEventPageHttpControl>>>,
    checkpoint_publication_pause_enabled: std::sync::atomic::AtomicBool,
    checkpoint_publication_admitted: tokio::sync::Semaphore,
    checkpoint_publication_release: tokio::sync::Semaphore,
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
            socket_global_send_pause: Mutex::new(None),
            socket_global_send_admitted: tokio::sync::Semaphore::new(0),
            socket_session_revoke_attempted: tokio::sync::Semaphore::new(0),
            socket_global_send_release: tokio::sync::Semaphore::new(0),
            socket_scoped_send_mode: std::sync::atomic::AtomicU8::new(0),
            socket_scoped_send_admitted: tokio::sync::Semaphore::new(0),
            socket_scoped_send_release: tokio::sync::Semaphore::new(0),
            socket_membership_remove_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_membership_remove_admitted: tokio::sync::Semaphore::new(0),
            socket_membership_remove_release: tokio::sync::Semaphore::new(0),
            directory_command_pause_user: Mutex::new(None),
            directory_command_attempted: tokio::sync::Semaphore::new(0),
            directory_command_admitted: tokio::sync::Semaphore::new(0),
            directory_command_release: tokio::sync::Semaphore::new(0),
            admin_effect_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            admin_effect_admitted: tokio::sync::Semaphore::new(0),
            admin_effect_release: tokio::sync::Semaphore::new(0),
            directory_event_page_fence_enabled: std::sync::atomic::AtomicBool::new(false),
            directory_event_page_read_admitted: tokio::sync::Semaphore::new(0),
            directory_event_page_read_release: tokio::sync::Semaphore::new(0),
            directory_event_page_control: Mutex::new(None),
            checkpoint_publication_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            checkpoint_publication_admitted: tokio::sync::Semaphore::new(0),
            checkpoint_publication_release: tokio::sync::Semaphore::new(0),
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
    component: std::sync::Arc<[u8]>,
    descriptor: std::sync::Arc<[u8]>,
    browser_actor: Option<std::sync::Arc<[u8]>>,
}

#[cfg(test)]
impl DocumentOpenCatalogAuthorityV1 for TestDocumentOpenCatalog {
    fn generation_id(&self) -> &str {
        &self.generation_id
    }

    fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        if current_generation != self.generation_id || self.component.is_empty() || self.descriptor.is_empty() {
            return None;
        }
        let selection = DocumentOpenCatalogAuthorityV1::resolve_document_open(self, descriptor, requested_surface_id, writable)?;
        Some(VerifiedExecutionTargetAssets { selection, component: std::sync::Arc::clone(&self.component), descriptor: std::sync::Arc::clone(&self.descriptor), browser_actor: self.browser_actor.clone() })
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
const ADMIN_OPERATION_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);
const ADMIN_OPERATION_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(11);
const ADMIN_TERMINAL_RETRY_DEADLINE: std::time::Duration = std::time::Duration::from_secs(2);
const ADMIN_EFFECT_PRE_EFFECT: u8 = 0;
const ADMIN_EFFECT_ADMITTED: u8 = 1;
const ADMIN_EFFECT_CANCELLED: u8 = 2;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SocketBindingKeyV1 {
    User(String),
    Session(String),
    DirectorySpaceAuthority { space_id: String },
    Membership { user_id: String, space_id: String },
    Share(String),
    DocumentWrite(DocumentScope),
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
    if let SocketAudienceV1::Document(scope) | SocketAudienceV1::DirectoryScoped(scope) = audience {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() });
        if let SocketSubjectV1::Session { user_id, .. } = subject {
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
        self.acquire_bindings(socket_record_bindings(subject, audience)).await
    }

    async fn acquire_bindings(&self, mut bindings: Vec<SocketBindingKeyV1>) -> Vec<tokio::sync::OwnedMutexGuard<()>> {
        bindings.sort();
        bindings.dedup();
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
        if inner.records.len() >= SOCKET_GRANT_LEDGER_CAPACITY
            || bindings.iter().filter(|binding| !matches!(binding, SocketBindingKeyV1::DirectorySpaceAuthority { .. })).any(|binding| inner.pending_by_binding.get(binding).map_or(0, BTreeSet::len) >= SOCKET_GRANT_BINDING_PENDING_CAPACITY)
        {
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
    browser_actor: os_directory::DocumentOpenBrowserActorV1,
    grant: DocumentOpenGrantV1,
    checkpoint: DocumentOpenCheckpointV1,
    revalidation: DocumentOpenRevalidationV1,
    subject: SocketSubjectV1,
    server_actor_id: String,
    client_instance_id_digest: [u8; 32],
}

impl DocumentOpenPlanAuthorityV1 {
    fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Stale)?;
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
            || self.checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || self.checkpoint.baseline_frontier.document_id != self.scope.document_id
            || !(self.checkpoint.baseline_frontier.is_genesis_for(&self.scope) || self.checkpoint.baseline_frontier.is_edited_for(&self.scope))
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
            browser_actor: self.browser_actor.clone(),
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
        let digests = inner
            .records
            .iter()
            .filter_map(|(digest, record)| (socket_record_bindings(&record.authority.subject, &SocketAudienceV1::Document(record.authority.scope.clone())).contains(binding) && record.state == DocumentOpenPlanStateV1::Issued).then_some(*digest))
            .collect::<Vec<_>>();
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
    socket_live_authority_with_bindings(state, record, live_id, record.bindings()).await
}

async fn socket_live_authority_with_bindings(state: &HubState, record: &SocketGrantRecordV1, live_id: &str, bindings: Vec<SocketBindingKeyV1>) -> Result<Vec<tokio::sync::OwnedMutexGuard<()>>, SocketBindingValidityV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_bindings(bindings)).await.map_err(|_| SocketBindingValidityV1::Unavailable)?;
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
    artifact_authority: Option<Arc<HubArtifactAuthority>>,
    verified_catalog: Option<Arc<VerifiedTrustedCatalog>>,
    #[cfg(feature = "native-artifact-execution")]
    artifact_creation: Option<Arc<ArtifactCreationServiceV1>>,
    #[cfg(feature = "native-artifact-execution")]
    artifact_creation_commit_authority: Arc<HubArtifactCreationCommitAuthorityV1>,
    #[cfg(feature = "native-artifact-execution")]
    artifact_creation_tasks: Arc<ArtifactCreationHttpTaskOwnerV1>,
    #[cfg(feature = "native-artifact-execution")]
    gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>,
    /// @emoji 💡️ The sole GIS Map proposal authority — present only when a verified trusted profile
    /// froze a writable Map editor selection; otherwise every inference route fails closed with
    /// `inference.unavailable` and readiness publishes `inference: false`.
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    inference_runtime: Option<Arc<HubInferenceRuntimeV1>>,
    openable_catalog: Option<Arc<dyn DocumentOpenCatalogAuthorityV1>>,
    artifact_publication: Arc<HubArtifactPublication>,
    artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    /// @emoji 🏭️ Wave 1.B: the single serialized directory writer (contract §C1's decider laws +
    /// dense event `seq`) built once over `directory` at startup — see `semio_hub::directory::
    /// DirectoryService`'s own doc. `/directory/commands` and `/directory/invites/{token}/redeem`
    /// go through this; every other `/directory/*` route reads `directory` directly.
    directory_service: Arc<DirectoryService>,
    admin_subjects: Arc<[AdminSubject]>,
    admin_cursor_key: [u8; 32],
    /// 🏛️ Process-local MAC key authenticating one space-administration keyset cursor.
    space_administration_cursor_key: [u8; 32],
    admin_operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    admin_operation_slots: Arc<tokio::sync::Semaphore>,
    admin_operation_tasks: Arc<AdminOperationTaskOwner>,
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
    /// a surface-scoped channel; identity and the plan-bound surface are reconstructed by Hub
    /// before canonical peer bytes are stored or published.
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
                    let directory_actor = slot.document_surface.as_ref().map(|surface| DirectoryPresenceActor { actor: actor.clone(), user_id: slot.user_id.clone(), surface: surface.clone(), color: slot.color });
                    rows.push((actor.clone(), peer.clone(), directory_actor));
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
            if let Some(actor) = actor {
                actors.push(actor);
            }
        }
        PresenceSnapshot { peers, actors }
    }

    /// 📡️ Publishes one document roster before its matching member-directory projection.
    fn publish_presence_delta(&self, key: &str, space_id: &str, document_id: &str, snapshot: PresenceSnapshot) {
        let _ = self.fanout_for(key).send(ServerFrame::Presence { peers: snapshot.peers });
        self.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.to_string(), document_id: document_id.to_string(), actors: snapshot.actors });
    }

    /// 🆕️ Selects one live socket as the actor's current owner without making it visible.
    async fn install_presence_slot(&self, key: &str, space_id: &str, document_id: &str, actor: &str, slot: PresenceLeaseSlot) -> PresenceLeaseTransition {
        let Ok(_publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let replaced_visible = self.presence.with(&map_key, |slot| slot.is_some_and(|slot| slot.peer.is_some()));
        self.presence.insert(map_key, slot);
        if replaced_visible {
            self.publish_presence_delta(key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 🪪️ Retains only canonical ephemerals under the matching socket's admitted identity.
    async fn refresh_document_presence(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, peer: Vec<u8>, now: tokio::time::Instant) -> PresenceLeaseTransition {
        let Ok(input) = protocol::decode_presence_peer(&peer).await else { return PresenceLeaseTransition::Rejected };
        let normalized = self.presence.with(&(key.to_string(), actor.to_string()), |slot| {
            slot.filter(|slot| slot.socket_live_id == socket_live_id).map(|slot| protocol::PresencePeer {
                actor: actor.to_string(),
                connected_at_ms: slot.connected_at_ms,
                label: slot.label.clone(),
                user_id: slot.user_id.clone(),
                role: slot.role.clone(),
                color: Some(slot.color),
                surface: slot.document_surface.clone(),
                presence_pack: input.presence_pack,
                drag_ghost_json: input.drag_ghost_json,
                interaction: input.interaction,
                views: input.views,
                ui: input.ui,
            })
        });
        let Some(normalized) = normalized else { return PresenceLeaseTransition::NoChange };
        let peer = protocol::encode_presence_peer(&normalized).await;
        if peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || protocol::decode_presence_peer(&peer).await.is_err() {
            return PresenceLeaseTransition::Rejected;
        }
        self.refresh_presence(key, space_id, document_id, actor, socket_live_id, peer, now).await
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
        match self.db.ensure_document(id).await {
            Ok(handle) => Ok(handle),
            Err(mut rejected) => loop {
                match rejected.retry_close().await {
                    Ok(error) => return Err(error),
                    Err(retained) => {
                        rejected = retained;
                        semio_framework_async::yield_once().await;
                    }
                }
            },
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
    inference_ready: bool,
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
        features: HubFeatureReadinessV1 { open_plan: open_plan_ready, open_plan_exchange: open_plan_ready, rebootstrap: true, mcp_workspace: false, inference: inference_ready },
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
    let descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&descriptor).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale))?.0);
    let checkpoint = state
        .directory
        .get_active_artifact_checkpoint(&scope)
        .await
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?
        .ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    if checkpoint.scope != scope
        || os_directory::hex_lower(&checkpoint.descriptor_digest_v1.0) != descriptor_digest_v1
        || !(checkpoint.baseline_frontier.is_genesis_for(&scope) || checkpoint.baseline_frontier.is_edited_for(&scope))
    {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let checkpoint = document_open_checkpoint(checkpoint);
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    let writable = matches!(subject, SocketSubjectV1::Session { role: Some(SpaceRole::Author), .. });
    let selected = catalog.resolve_document_open(&descriptor, intent.requested_surface_id.as_deref(), writable).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision == 0 || directory_revision > os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    }
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
        browser_actor: selected.browser_actor,
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
        || catalog.resolve_document_open(&authority.descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selected| {
            selected.package != authority.package
                || selected.artifact != authority.artifact
                || selected.parent_dialect != authority.parent_dialect
                || selected.surface != authority.surface
                || selected.browser_actor != authority.browser_actor
                || selected.grant != authority.grant
        })
    {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision != authority.revalidation.directory_revision || directory_revision != authority.revalidation.membership_generation {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    state.document_open_plans.exchange_to_socket_grant(&intent.plan_receipt, &authority, exchange_at_ms, state.socket_grants.as_ref()).map(Json).map_err(document_open_plan_exchange_error)
}

//#region 🪪️ExecutionTargetLease
const DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES: usize = 8 * 1024;
const DOCUMENT_EXECUTION_TARGET_DEADLINE_MS: u64 = 8_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentExecutionTargetAssetV1 {
    Manifest,
    Component,
    Descriptor,
    BrowserActor,
}

/// 🛡️ One protected document-scoped execution-target read. Every call re-authenticates the exact
/// session or share binding, reloads the durable descriptor, and resolves the current trusted
/// selection through the catalog's own exact-selection accessor. It accepts only the bounded
/// `DocumentOpenIntentV1`; a package id, digest, catalog generation, local path or plan receipt is
/// never a selector. The private browser verifier rejects independently selected reads that differ
/// from its open plan; this route rejects authorization or directory changes during one selection.
async fn document_execution_target_selection(space_id: String, document_id: String, headers: HeaderMap, state: HubState, body: Bytes) -> Result<(DocumentExecutionTargetLeaseFieldsV1, VerifiedExecutionTargetAssets), DocumentOpenPlanRouteError> {
    let content_types = headers.get_all(axum::http::header::CONTENT_TYPE);
    if !socket_text_bounded(&space_id)
        || !socket_text_bounded(&document_id)
        || body.is_empty()
        || body.len() > DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES
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
    let (subject, _) = authenticate_document_socket_subject(&state, &scope, &headers).await.map_err(document_open_plan_exchange_error)?;
    let audience = SocketAudienceV1::Document(scope.clone());
    let _admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(&subject, &audience)).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    let directory_revision = state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?;
    if directory_revision == 0 || directory_revision > os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    }
    let descriptor =
        state.directory.get_document_descriptor(&scope).await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?.ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    let descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&descriptor).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale))?.0);
    let checkpoint = state
        .directory
        .get_active_artifact_checkpoint(&scope)
        .await
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))?
        .ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::NotFound))?;
    if checkpoint.scope != scope
        || os_directory::hex_lower(&checkpoint.descriptor_digest_v1.0) != descriptor_digest_v1
        || !(checkpoint.baseline_frontier.is_genesis_for(&scope) || checkpoint.baseline_frontier.is_edited_for(&scope))
    {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    let checkpoint = document_open_checkpoint(checkpoint);
    let catalog = state.openable_catalog.as_ref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::CatalogUnavailable))?;
    let writable = matches!(subject, SocketSubjectV1::Session { role: Some(SpaceRole::Author), .. });
    let generation_id = catalog.generation_id().to_string();
    let assets = catalog.assets_for_current_selection(&descriptor, intent.requested_surface_id.as_deref(), writable, &generation_id).ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let (session_generation, share_generation) = match &subject {
        SocketSubjectV1::Session { authorization_generation, .. } => (Some(*authorization_generation), None),
        SocketSubjectV1::Share { .. } => (None, Some(1)),
    };
    let component_byte_length = u64::try_from(assets.component.len()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let descriptor_byte_length = u64::try_from(assets.descriptor.len()).map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    if component_byte_length == 0 || component_byte_length > DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES || descriptor_byte_length == 0 || descriptor_byte_length > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable));
    }
    let browser_actor_byte_length = assets.browser_actor.as_ref().map(|bytes| u64::try_from(bytes.len())).transpose().map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let browser_actor = assets
        .selection
        .browser_actor
        .to_lease(
            os_directory::DocumentBrowserActorSourceV1 { component_sha256: &assets.selection.package.component_sha256, descriptor_byte_sha256: &assets.selection.package.descriptor_byte_sha256 },
            assets.selection.surface.renderer_target.as_str(),
            browser_actor_byte_length,
        )
        .map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let fields = DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".into(),
        version: 1,
        scope,
        descriptor_digest_v1,
        catalog: DocumentOpenCatalogV1 { generation_id: generation_id.clone() },
        package: assets.selection.package.clone(),
        component: DocumentExecutionTargetComponentV1 { sha256: assets.selection.package.component_sha256.clone(), blake3: assets.selection.package.component_blake3.clone(), byte_length: component_byte_length },
        descriptor: DocumentExecutionTargetDescriptorV1 { sha256: assets.selection.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
        browser_actor,
        artifact: assets.selection.artifact.clone(),
        parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: assets.selection.parent_dialect.artifact_kind.clone(), standard: assets.selection.parent_dialect.standard.clone(), subset: assets.selection.parent_dialect.subset.clone() },
        surface: assets.selection.surface.clone(),
        grant: assets.selection.grant,
        checkpoint,
        revalidation: DocumentOpenRevalidationV1 { directory_revision, membership_generation: directory_revision, session_generation, share_generation },
    };
    fields.validate().map_err(document_open_plan_exchange_error)?;
    #[cfg(test)]
    if let Some(gate) = &state.document_open_plan_issue_gate {
        gate.admitted.add_permits(1);
        gate.release.acquire().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Cancelled))?.forget();
    }
    match subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await {
        SocketBindingValidityV1::Active => {}
        SocketBindingValidityV1::Unauthorized => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Denied)),
        SocketBindingValidityV1::Unavailable => return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)),
    }
    if state.directory.head_seq().await.map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded))? != directory_revision {
        return Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::Stale));
    }
    Ok((fields, assets))
}

async fn issue_document_execution_target(
    asset: DocumentExecutionTargetAssetV1,
    uri: axum::http::Uri,
    space_id: String,
    document_id: String,
    state: HubState,
    request: axum::extract::Request,
) -> Result<axum::response::Response, DocumentOpenPlanRouteError> {
    if uri.query().is_some() {
        return Err(document_open_plan_route_error(StatusCode::BAD_REQUEST, DocumentOpenPlanErrorCodeV1::Denied));
    }
    tokio::time::timeout(std::time::Duration::from_millis(DOCUMENT_EXECUTION_TARGET_DEADLINE_MS), async move {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES).await.map_err(|_| document_open_plan_route_error(StatusCode::PAYLOAD_TOO_LARGE, DocumentOpenPlanErrorCodeV1::Denied))?;
        let (fields, assets) = document_execution_target_selection(space_id, document_id, parts.headers, state, body).await?;
        Ok(match asset {
            DocumentExecutionTargetAssetV1::Manifest => DirectoryJson(fields).into_response(),
            DocumentExecutionTargetAssetV1::Component => document_execution_target_bytes(&assets.component),
            DocumentExecutionTargetAssetV1::Descriptor => document_execution_target_bytes(&assets.descriptor),
            DocumentExecutionTargetAssetV1::BrowserActor => document_execution_target_bytes(assets.browser_actor.as_deref().ok_or_else(|| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?),
        })
    })
    .await
    .unwrap_or_else(|_| Err(document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)))
}

fn document_execution_target_bytes(bytes: &[u8]) -> axum::response::Response {
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/octet-stream".to_string()), (axum::http::header::CONTENT_LENGTH, bytes.len().to_string()), (axum::http::header::CACHE_CONTROL, "no-store".to_string())], bytes.to_vec())
        .into_response()
}

async fn issue_document_execution_target_manifest(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<axum::response::Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Manifest, uri, space_id, document_id, state, request).await
}

async fn issue_document_execution_target_component(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<axum::response::Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Component, uri, space_id, document_id, state, request).await
}

async fn issue_document_execution_target_descriptor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<axum::response::Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::Descriptor, uri, space_id, document_id, state, request).await
}
async fn issue_document_execution_target_browser_actor(
    OriginalUri(uri): OriginalUri,
    Path((space_id, document_id)): Path<(String, String)>,
    State(state): State<HubState>,
    request: axum::extract::Request,
) -> Result<axum::response::Response, DocumentOpenPlanRouteError> {
    issue_document_execution_target(DocumentExecutionTargetAssetV1::BrowserActor, uri, space_id, document_id, state, request).await
}
//#endregion 🪪️ExecutionTargetLease

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

//#region 📣️CheckpointPublication
struct CheckpointPublicationHttpControl {
    cancelled: std::sync::atomic::AtomicBool,
    progress: Mutex<Option<AuthorityProgress>>,
}

impl CheckpointPublicationHttpControl {
    fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), progress: Mutex::new(None) }
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }
}

impl AuthorityOperationControl for CheckpointPublicationHttpControl {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }

    fn report(&self, progress: AuthorityProgress) {
        if progress.completed_units <= progress.total_units {
            *self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(progress);
        }
    }
}

struct CheckpointPublicationHttpRequest {
    control: Arc<CheckpointPublicationHttpControl>,
    complete: bool,
}

struct CheckpointPublicationClaimGuardV1 {
    service: Arc<DirectoryService>,
    actor_user_id: String,
    correlation_id: String,
    command_sha256: String,
    complete: bool,
}

impl CheckpointPublicationClaimGuardV1 {
    fn new(service: Arc<DirectoryService>, claim: &NewCheckpointPublicationClaimV1) -> Self {
        Self { service, actor_user_id: claim.actor_user_id.clone(), correlation_id: claim.correlation_id.clone(), command_sha256: claim.command_sha256.clone(), complete: false }
    }

    fn complete(&mut self) {
        self.complete = true;
    }

    async fn release(&mut self) {
        if !self.complete && self.service.release_checkpoint_publication(&self.actor_user_id, &self.correlation_id, &self.command_sha256).await.is_ok() {
            self.complete = true;
        }
    }
}

impl Drop for CheckpointPublicationClaimGuardV1 {
    fn drop(&mut self) {
        if self.complete {
            return;
        }
        let service = self.service.clone();
        let actor_user_id = self.actor_user_id.clone();
        let correlation_id = self.correlation_id.clone();
        let command_sha256 = self.command_sha256.clone();
        tokio::spawn(async move {
            let _ = service.release_checkpoint_publication(&actor_user_id, &correlation_id, &command_sha256).await;
        });
    }
}

impl CheckpointPublicationHttpRequest {
    fn new(control: Arc<CheckpointPublicationHttpControl>) -> Self {
        Self { control, complete: false }
    }

    fn complete(&mut self) {
        self.complete = true;
    }
}

impl Drop for CheckpointPublicationHttpRequest {
    fn drop(&mut self) {
        if !self.complete {
            self.control.cancel();
        }
    }
}

fn checkpoint_publication_current_matches(expected: &CheckpointPublicationCurrentV1, current: Option<&PublishedArtifactCheckpoint>) -> bool {
    match (expected, current) {
        (CheckpointPublicationCurrentV1::Genesis { checkpoint_id }, Some(current)) => current.checkpoint_id.hex() == *checkpoint_id && current.parent_checkpoint_id.is_none() && current.baseline_frontier.is_genesis_for(&current.scope),
        (CheckpointPublicationCurrentV1::Active { checkpoint_id, baseline_frontier }, Some(current)) => current.checkpoint_id.hex() == *checkpoint_id && baseline_frontier.artifact_frontier().as_ref() == Some(&current.baseline_frontier),
        _ => false,
    }
}

fn checkpoint_publication_artifact_frontier(scope: &DocumentScope, snapshot: &db::CheckpointPublicationSnapshot) -> Option<ArtifactFrontier> {
    Some(ArtifactFrontier {
        document_id: scope.document_id.clone(),
        head_edit_ordinal: snapshot.frontier.head_seq,
        head_edit_id: snapshot.head_edit_id.as_ref()?.0.clone(),
        last_commit_seq: snapshot.frontier.commit_seq,
        chain_hash: ArtifactHash(snapshot.frontier.chain_hash),
    })
}

fn checkpoint_publication_snapshot_matches(scope: &DocumentScope, command: &CheckpointPublicationCommandV1, snapshot: &db::CheckpointPublicationSnapshot) -> bool {
    let Some(expected) = command.baseline_frontier.artifact_frontier() else { return false };
    snapshot.authority_generation != 0
        && snapshot.frontier.document == db_artifact_id(scope)
        && snapshot.frontier.head_seq == command.expected_document_frontier.head_seq
        && snapshot.frontier.commit_seq == command.expected_document_frontier.commit_seq
        && snapshot.frontier.epoch == command.expected_document_frontier.epoch
        && checkpoint_publication_artifact_frontier(scope, snapshot).as_ref() == Some(&expected)
}

fn checkpoint_publication_replay_matches(scope: &DocumentScope, command: &CheckpointPublicationCommandV1, checkpoint: &PublishedArtifactCheckpoint) -> bool {
    checkpoint.scope == *scope
        && checkpoint.descriptor_digest_v1.hex() == command.descriptor_digest_v1
        && command.baseline_frontier.artifact_frontier().as_ref() == Some(&checkpoint.baseline_frontier)
        && checkpoint.pack.sha256.hex() == command.pack.sha256
        && checkpoint.pack.byte_length == command.pack.byte_length
        && checkpoint.spr.sha256.hex() == command.spr.sha256
        && checkpoint.spr.byte_length == command.spr.byte_length
        && match &command.expected_current {
            CheckpointPublicationCurrentV1::Genesis { checkpoint_id } | CheckpointPublicationCurrentV1::Active { checkpoint_id, .. } => checkpoint.parent_checkpoint_id.is_some_and(|parent| parent.hex() == *checkpoint_id),
        }
}

fn checkpoint_publication_receipt(command: &CheckpointPublicationCommandV1, checkpoint: PublishedArtifactCheckpoint) -> Response {
    let receipt = CheckpointPublicationReceiptV1 { schema: "semio.hub.checkpoint-publication-receipt/v1".into(), correlation_id: command.correlation_id.clone(), checkpoint };
    let mut response = DirectoryJson(receipt).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("private, no-store"));
    response
}

async fn checkpoint_publication_blob(state: &HubState, reference: &os_directory::CheckpointPublicationBlobV1, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
    context.checkpoint()?;
    let content_hash = parse_content_hash(&reference.sha256).ok_or(AuthorityError::BlobIntegrity("input"))?;
    let pages = state.db.storage().await.payload().await.get(&content_hash).await.map_err(|_| AuthorityError::Store("checkpoint input blob unavailable".into()))?;
    let bytes = db_io_pages_into_http_bytes(pages).await.map_err(|_| AuthorityError::Store("checkpoint input blob unavailable".into()))?;
    context.checkpoint()?;
    if u64::try_from(bytes.len()).ok() != Some(reference.byte_length) || ArtifactHash(Sha256::digest(bytes.as_ref())).hex() != reference.sha256 {
        return Err(AuthorityError::BlobIntegrity("input"));
    }
    Ok(bytes.to_vec())
}

fn checkpoint_publication_error_status(error: &AuthorityError) -> StatusCode {
    match error {
        AuthorityError::Cancelled => StatusCode::SERVICE_UNAVAILABLE,
        AuthorityError::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        AuthorityError::ResourceLimit(_) | AuthorityError::PairResourceLimit(_) => StatusCode::PAYLOAD_TOO_LARGE,
        AuthorityError::InvalidDescriptor(_)
        | AuthorityError::InvalidScope
        | AuthorityError::InvalidFrontier
        | AuthorityError::InvalidParentCheckpoint
        | AuthorityError::InvalidOperationOrder
        | AuthorityError::InvalidLimits
        | AuthorityError::Codec { .. }
        | AuthorityError::CodecIdentityMismatch => StatusCode::BAD_REQUEST,
        AuthorityError::Catalog(_) | AuthorityError::Store(_) => StatusCode::SERVICE_UNAVAILABLE,
        AuthorityError::BlobIntegrity(_) | AuthorityError::Publication(_) => StatusCode::CONFLICT,
    }
}

struct FencedCheckpointPublisherV1 {
    state: HubState,
    handle: db::ArtifactHandle,
    subject: SocketSubjectV1,
    audience: SocketAudienceV1,
    descriptor: DocumentDescriptor,
    descriptor_digest: ArtifactHash,
    expected_current: CheckpointPublicationCurrentV1,
    expected_snapshot: db::CheckpointPublicationSnapshot,
    completion: CheckpointPublicationCompletionV1,
}

impl FencedCheckpointPublisherV1 {
    fn publication_error() -> AuthorityError {
        AuthorityError::Publication("checkpoint publication authority changed".into())
    }

    async fn authority_is_current(&self) -> Result<bool, AuthorityError> {
        if self.subject.revalidate(self.state.directory.as_ref(), &self.audience, now_ms()).await != SocketBindingValidityV1::Active {
            return Ok(false);
        }
        let SocketAudienceV1::Document(scope) = &self.audience else { return Ok(false) };
        let descriptor = self.state.directory.get_document_descriptor(scope).await.map_err(|_| Self::publication_error())?;
        let current = self.state.directory.get_active_artifact_checkpoint(scope).await.map_err(|_| Self::publication_error())?;
        let snapshot = self.handle.checkpoint_publication_snapshot().await.map_err(|_| Self::publication_error())?;
        Ok(descriptor.as_ref() == Some(&self.descriptor)
            && descriptor.as_ref().and_then(|value| descriptor_digest_v1(value).ok()) == Some(self.descriptor_digest)
            && checkpoint_publication_current_matches(&self.expected_current, current.as_ref())
            && snapshot == self.expected_snapshot)
    }
}

impl VerifiedCheckpointPublisher for FencedCheckpointPublisherV1 {
    async fn reserve(&self, plan: &semio_hub::artifact_authority::chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await? {
            return Err(Self::publication_error());
        }
        HubVerifiedCheckpointPublisher::new(self.state.directory_service.clone(), self.state.artifact_cas.clone(), "system:artifact-authority").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &os_directory::ArtifactCheckpoint, reservation: &semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let authorization = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.acquire_record(&self.subject, &self.audience)).await.map_err(|_| Self::publication_error())?;
        let SocketAudienceV1::Document(scope) = &self.audience else { return Err(Self::publication_error()) };
        let document_write = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(scope.clone())).lock_owned()).await.map_err(|_| Self::publication_error())?;
        context.checkpoint()?;
        if !self.authority_is_current().await?
            || checkpoint.scope != *scope
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest
            || checkpoint.baseline_frontier != checkpoint_publication_artifact_frontier(scope, &self.expected_snapshot).ok_or_else(Self::publication_error)?
        {
            return Err(Self::publication_error());
        }
        let mut completion = self.completion.clone();
        completion.checkpoint_id = checkpoint.checkpoint_id;
        completion.completed_at = i64::try_from(context.now_ms()).map_err(|_| Self::publication_error())?;
        let result = self
            .state
            .directory_service
            .publish_reserved_artifact_checkpoint_and_complete_checkpoint_publication(DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() }, checkpoint.clone(), reservation.clone(), completion, context.now_ms())
            .await
            .map(|_| ())
            .map_err(|_| AuthorityError::Publication("checkpoint publication completion failed".into()));
        drop(document_write);
        drop(authorization);
        result
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
struct GisMapApprovalPublicationControlV1 {
    deadline_ms: u64,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl AuthorityOperationControl for GisMapApprovalPublicationControlV1 {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.now_ms() > self.deadline_ms
    }

    fn report(&self, _progress: AuthorityProgress) {}
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
struct GisMapApprovalVerifiedPublisherV1 {
    directory: Arc<HubDirectories>,
    directory_service: Arc<DirectoryService>,
    artifact_cas: Arc<ArtifactChunkCasStores>,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    descriptor: DocumentDescriptor,
    descriptor_digest: ArtifactHash,
    expected_current: Option<PublishedArtifactCheckpoint>,
    expected_snapshot: db::CheckpointPublicationSnapshot,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl GisMapApprovalVerifiedPublisherV1 {
    async fn authority_is_current(&self) -> Result<bool, AuthorityError> {
        let descriptor = self.directory.get_document_descriptor(&self.scope).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        let current = self.directory.get_active_artifact_checkpoint(&self.scope).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        let snapshot = self.handle.checkpoint_publication_snapshot().await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        Ok(descriptor.as_ref() == Some(&self.descriptor) && descriptor.as_ref().and_then(|value| descriptor_digest_v1(value).ok()) == Some(self.descriptor_digest) && current == self.expected_current && snapshot == self.expected_snapshot)
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl VerifiedCheckpointPublisher for GisMapApprovalVerifiedPublisherV1 {
    async fn reserve(&self, plan: &semio_hub::artifact_authority::chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await? {
            return Err(AuthorityError::Publication("GIS Map approval checkpoint authority changed".into()));
        }
        HubVerifiedCheckpointPublisher::new(self.directory_service.clone(), self.artifact_cas.clone(), "system:gis-map-approval").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &os_directory::ArtifactCheckpoint, reservation: &semio_hub::artifact_authority::chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await?
            || checkpoint.scope != self.scope
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest
            || checkpoint.baseline_frontier != checkpoint_publication_artifact_frontier(&checkpoint.scope, &self.expected_snapshot).ok_or_else(|| AuthorityError::Publication("GIS Map approval frontier is incomplete".into()))?
        {
            return Err(AuthorityError::Publication("GIS Map approval checkpoint authority changed".into()));
        }
        HubVerifiedCheckpointPublisher::new(self.directory_service.clone(), self.artifact_cas.clone(), "system:gis-map-approval").publish_reserved(checkpoint, reservation, context).await
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
struct GisMapApprovalCheckpointPublisherV1Impl {
    directory: Arc<HubDirectories>,
    directory_service: Arc<DirectoryService>,
    artifact_cas: Arc<ArtifactChunkCasStores>,
    authority: Arc<HubArtifactAuthority>,
    fanout: Arc<ShardedMap<String, broadcast::Sender<ServerFrame>>>,
    fanout_capacity: usize,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl GisMapApprovalCheckpointPublisherV1Impl {
    fn matches_published(request: &GisMapApprovalCheckpointRequestV1, checkpoint: &PublishedArtifactCheckpoint) -> bool {
        let Some(frontier) = checkpoint_publication_artifact_frontier(&request.scope, &request.actor_snapshot) else { return false };
        checkpoint.scope == request.scope
            && checkpoint.descriptor_digest_v1.hex() == request.descriptor_digest
            && checkpoint.baseline_frontier == frontier
            && checkpoint.pack.sha256 == ArtifactHash(Sha256::digest(&request.pair.pack))
            && checkpoint.pack.byte_length == request.pair.pack.len() as u64
            && checkpoint.spr.sha256 == ArtifactHash(Sha256::digest(&request.pair.spr))
            && checkpoint.spr.byte_length == request.pair.spr.len() as u64
    }

    fn current_matches_base(scope: &DocumentScope, base: &ArtifactFrontier, current: Option<&PublishedArtifactCheckpoint>) -> bool {
        if base.document_id != scope.document_id {
            return false;
        }
        match current {
            Some(checkpoint) => checkpoint.scope == *scope && checkpoint.baseline_frontier == *base,
            None => false,
        }
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl GisMapApprovalCheckpointPublisherV1 for GisMapApprovalCheckpointPublisherV1Impl {
    fn publish<'a>(
        &'a self,
        request: GisMapApprovalCheckpointRequestV1,
        document_write: Arc<GisMapDocumentWriteAuthorityV1>,
        attempt_lifetime_ms: u64,
        _decision_now_ms: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async move {
            let _document_write = document_write;
            let descriptor = self.directory.get_document_descriptor(&request.scope).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?.ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            let descriptor_digest = descriptor_digest_v1(&descriptor).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if descriptor_digest.hex() != request.descriptor_digest {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let current = self.directory.get_active_artifact_checkpoint(&request.scope).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if current.as_ref().is_some_and(|checkpoint| Self::matches_published(&request, checkpoint)) {
                return Ok(current.expect("matching active checkpoint exists"));
            }
            if !Self::current_matches_base(&request.scope, &request.base_frontier, current.as_ref()) {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let post_frontier = checkpoint_publication_artifact_frontier(&request.scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            if attempt_lifetime_ms == 0 || attempt_lifetime_ms > semio_hub::inference::schema::JOB_MAX_LIFETIME_MS {
                return Err(GisMapApprovalCommitErrorV1::Rejected);
            }
            let deadline_ms = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)).saturating_add(attempt_lifetime_ms);
            let control = GisMapApprovalPublicationControlV1 { deadline_ms };
            let context = OperationContext::new(deadline_ms, AuthorityLimits { max_operations: 1, max_operation_bytes: 1, max_pair_bytes: CHECKPOINT_PUBLICATION_PAIR_MAX_BYTES }, &control);
            let candidate = self
                .authority
                .materialize_checkpoint(
                    CheckpointRequest {
                        descriptor: descriptor.clone(),
                        scope: request.scope.clone(),
                        parent_checkpoint_id: current.as_ref().map(|checkpoint| checkpoint.checkpoint_id),
                        base_frontier: post_frontier,
                        input_pair: request.pair,
                        operations: Vec::new(),
                    },
                    &context,
                )
                .await
                .map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let publisher = GisMapApprovalVerifiedPublisherV1 {
                directory: self.directory.clone(),
                directory_service: self.directory_service.clone(),
                artifact_cas: self.artifact_cas.clone(),
                scope: request.scope,
                handle: request.handle,
                descriptor,
                descriptor_digest,
                expected_current: current,
                expected_snapshot: request.actor_snapshot,
            };
            let publication = CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(self.artifact_cas.clone()), publisher);
            let published = publication.publish_candidate(candidate, &context).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            Ok(published_artifact_checkpoint(&published.checkpoint))
        })
    }

    fn checkpoint_applied(&self, checkpoint: &PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1> {
        publish_gis_map_checkpoint_change(self.fanout.as_ref(), self.fanout_capacity, checkpoint);
        Ok(())
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
fn publish_gis_map_checkpoint_change(
    fanout: &ShardedMap<String, broadcast::Sender<ServerFrame>>,
    fanout_capacity: usize,
    checkpoint: &PublishedArtifactCheckpoint,
) {
    let key = document_scope_key_v1(&checkpoint.scope);
    let sender = fanout.get_or_insert_with_cloned(key, || broadcast::channel(fanout_capacity).0);
    let control = os_directory::RebootstrapRequired {
        scope: checkpoint.scope.clone(),
        checkpoint_id: checkpoint.checkpoint_id,
        descriptor_digest_v1: checkpoint.descriptor_digest_v1,
        baseline_frontier: checkpoint.baseline_frontier.clone(),
    };
    let _ = sender.send(ServerFrame::RebootstrapRequired { control: wire_rebootstrap(&control) });
}

fn checkpoint_publication_response<'a>(
    state: &'a HubState,
    scope: &'a DocumentScope,
    subject: &'a SocketSubjectV1,
    audience: &'a SocketAudienceV1,
    command: CheckpointPublicationCommandV1,
    completion: CheckpointPublicationCompletionV1,
    control: &'a CheckpointPublicationHttpControl,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send + 'a>> {
    Box::pin(async move {
        let deadline_ms = control.now_ms().saturating_add(CHECKPOINT_PUBLICATION_DEADLINE_MS);
        let limits = AuthorityLimits { max_operations: 1, max_operation_bytes: 1, max_pair_bytes: CHECKPOINT_PUBLICATION_PAIR_MAX_BYTES };
        let context = OperationContext::new(deadline_ms, limits, control);
        if let Err(error) = context.checkpoint() {
            return checkpoint_publication_error_status(&error).into_response();
        }

        let authorization = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_record(subject, audience)).await {
            Ok(guards) => guards,
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        let document_write = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(scope.clone())).lock_owned()).await {
            Ok(guard) => guard,
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        if subject.revalidate(state.directory.as_ref(), audience, now_ms()).await != SocketBindingValidityV1::Active {
            return StatusCode::UNAUTHORIZED.into_response();
        }
        let descriptor = match state.directory.get_document_descriptor(scope).await {
            Ok(Some(descriptor)) => descriptor,
            Ok(None) => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        let descriptor_digest = match descriptor_digest_v1(&descriptor) {
            Ok(digest) if digest.hex() == command.descriptor_digest_v1 => digest,
            Ok(_) => return StatusCode::CONFLICT.into_response(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        let handle = match state.ensure_document(&db_artifact_id(scope)).await {
            Ok(handle) => handle,
            Err(error) => return db_error_status(&error).into_response(),
        };
        let snapshot = match handle.checkpoint_publication_snapshot().await {
            Ok(snapshot) if checkpoint_publication_snapshot_matches(scope, &command, &snapshot) => snapshot,
            Ok(_) => return StatusCode::CONFLICT.into_response(),
            Err(error) => return db_error_status(&error).into_response(),
        };
        let current = match state.directory.get_active_artifact_checkpoint(scope).await {
            Ok(current) if checkpoint_publication_current_matches(&command.expected_current, current.as_ref()) => current,
            Ok(_) => return StatusCode::CONFLICT.into_response(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
        let base_frontier = match checkpoint_publication_artifact_frontier(scope, &snapshot) {
            Some(frontier) => frontier,
            None => return StatusCode::CONFLICT.into_response(),
        };
        drop(document_write);
        drop(authorization);

        let pack = match checkpoint_publication_blob(state, &command.pack, &context).await {
            Ok(bytes) => bytes,
            Err(error) => return checkpoint_publication_error_status(&error).into_response(),
        };
        let spr = match checkpoint_publication_blob(state, &command.spr, &context).await {
            Ok(bytes) => bytes,
            Err(error) => return checkpoint_publication_error_status(&error).into_response(),
        };
        let Some(authority) = state.artifact_authority.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
        let request =
            CheckpointRequest { descriptor: descriptor.clone(), scope: scope.clone(), parent_checkpoint_id: current.as_ref().map(|checkpoint| checkpoint.checkpoint_id), base_frontier, input_pair: ArtifactPair { pack, spr }, operations: Vec::new() };
        let candidate = match authority.materialize_checkpoint(request, &context).await {
            Ok(candidate) => candidate,
            Err(error) => return checkpoint_publication_error_status(&error).into_response(),
        };
        #[cfg(test)]
        if let Some(gate) = state.live_gate.as_ref().filter(|gate| gate.checkpoint_publication_pause_enabled.load(std::sync::atomic::Ordering::Acquire)) {
            gate.checkpoint_publication_admitted.add_permits(1);
            let _ = gate.checkpoint_publication_release.acquire().await;
            if let Err(error) = context.checkpoint() {
                return checkpoint_publication_error_status(&error).into_response();
            }
        }
        let publisher = FencedCheckpointPublisherV1 {
            state: state.clone(),
            handle,
            subject: subject.clone(),
            audience: audience.clone(),
            descriptor,
            descriptor_digest,
            expected_current: command.expected_current.clone(),
            expected_snapshot: snapshot,
            completion,
        };
        let publication = CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(state.artifact_cas.clone()), publisher);
        let published = match publication.publish_candidate(candidate, &context).await {
            Ok(published) => published,
            Err(error) => return checkpoint_publication_error_status(&error).into_response(),
        };
        checkpoint_publication_receipt(&command, published_artifact_checkpoint(&published.checkpoint))
    })
}

async fn post_checkpoint_publication(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || headers.get_all(axum::http::header::CONTENT_TYPE).iter().count() != 1 || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let command_source = match std::str::from_utf8(&body) {
        Ok(source) => source,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let command = match CheckpointPublicationCommandV1::parse_canonical_json(command_source) {
        Some(command) => command,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, _) = match authenticate_document_socket_subject(&state, &scope, &headers).await {
        Ok(value) => value,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    match &subject {
        SocketSubjectV1::Session { role: Some(SpaceRole::Author), .. } => {}
        SocketSubjectV1::Session { .. } => return StatusCode::FORBIDDEN.into_response(),
        SocketSubjectV1::Share { .. } => return StatusCode::UNAUTHORIZED.into_response(),
    }
    let actor_user_id = match &subject {
        SocketSubjectV1::Session { user_id, .. } => user_id.clone(),
        SocketSubjectV1::Share { .. } => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let command_sha256 = os_directory::hex_lower(&Sha256::digest(command_source.as_bytes()));
    let claim = NewCheckpointPublicationClaimV1 { actor_user_id, correlation_id: command.correlation_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() };
    let claimed = match state.directory_service.claim_or_read_checkpoint_publication(&claim).await {
        Ok(CheckpointPublicationClaimV1::Claimed(_)) => true,
        Ok(CheckpointPublicationClaimV1::Conflict) => return StatusCode::CONFLICT.into_response(),
        Ok(CheckpointPublicationClaimV1::Existing(record)) => {
            if record.disposition != CheckpointPublicationDispositionV1::Completed {
                return StatusCode::CONFLICT.into_response();
            }
            let Some(checkpoint_id) = record.checkpoint_id else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
            let descriptor = match state.directory.get_document_descriptor(&scope).await {
                Ok(Some(descriptor)) => descriptor,
                Ok(None) => return StatusCode::NOT_FOUND.into_response(),
                Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
            };
            if descriptor_digest_v1(&descriptor).ok().map(|digest| digest.hex()).as_deref() != Some(command.descriptor_digest_v1.as_str()) {
                return StatusCode::CONFLICT.into_response();
            }
            return match state.directory.get_artifact_checkpoint(&scope, checkpoint_id).await {
                Ok(Some(checkpoint)) if checkpoint_publication_replay_matches(&scope, &command, &checkpoint) => checkpoint_publication_receipt(&command, checkpoint),
                Ok(_) => StatusCode::CONFLICT.into_response(),
                Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
            };
        }
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    debug_assert!(claimed);
    let mut claim_guard = CheckpointPublicationClaimGuardV1::new(state.directory_service.clone(), &claim);
    let completion = CheckpointPublicationCompletionV1 { actor_user_id: claim.actor_user_id.clone(), correlation_id: claim.correlation_id.clone(), command_sha256, checkpoint_id: ArtifactHash([0; 32]), completed_at: 0 };
    let audience = SocketAudienceV1::Document(scope.clone());
    let control = Arc::new(CheckpointPublicationHttpControl::new());
    let mut request = CheckpointPublicationHttpRequest::new(control.clone());
    let subject_for_operation = subject.clone();
    let audience_for_operation = audience.clone();
    let operation = checkpoint_publication_response(&state, &scope, &subject_for_operation, &audience_for_operation, command, completion, control.as_ref());
    tokio::pin!(operation);
    let monitored = async {
        loop {
            tokio::select! {
                response = &mut operation => break response,
                _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                    if subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await != SocketBindingValidityV1::Active {
                        control.cancel();
                    }
                }
            }
        }
    };
    let response = match tokio::time::timeout(std::time::Duration::from_millis(CHECKPOINT_PUBLICATION_DEADLINE_MS.saturating_add(100)), monitored).await {
        Ok(response) => response,
        Err(_) => {
            control.cancel();
            StatusCode::GATEWAY_TIMEOUT.into_response()
        }
    };
    if response.status().is_success() {
        claim_guard.complete();
    } else {
        claim_guard.release().await;
    }
    request.complete();
    response
}
//#endregion 📣️CheckpointPublication
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
            selection.package != authority.package
                || selection.artifact != authority.artifact
                || selection.parent_dialect != authority.parent_dialect
                || selection.surface != authority.surface
                || selection.browser_actor != authority.browser_actor
                || selection.grant != authority.grant
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
            let _document_write = state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(DocumentScope::new(space_id, document_id))).lock_owned().await;
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
            let _ = state.refresh_document_presence(key, space_id, document_id, &actor.0, socket_live_id, peer, state.presence_now()).await;
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
    let connected_at_ms = now_ms();
    let authenticated_user = match user_id.as_deref() {
        Some(user_id) => tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_user(user_id)).await.ok().and_then(Result::ok).flatten(),
        None => None,
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
    let slot = PresenceLeaseSlot {
        socket_live_id: socket_live.id.clone(),
        expires_at: state.presence_now() + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS),
        user_id: user_id.clone(),
        connected_at_ms,
        label: authenticated_user.as_ref().map(|user| user.display_name.clone()),
        role: role.map(|role| role.as_str().to_string()),
        document_surface: socket_grant.document_plan.as_ref().map(|plan| plan.surface.surface_id.clone()),
        color,
        peer: None,
    };
    if matches!(state.install_presence_slot(&key, &space_id, &document_id, &actor.0, slot).await, PresenceLeaseTransition::Unavailable | PresenceLeaseTransition::Rejected) {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "presence-unavailable".into() }))).await;
        state.release_color(&space_id, &actor.0);
        return;
    }
    drop(_session_authority);

    let fanout = state.fanout_for(&key);
    let mut broadcast_rx = fanout.subscribe();
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.document_subscribed.add_permits(1);
        let _ = gate.document_release.acquire().await;
    }

    let authenticated_email = authenticated_user.as_ref().map(|user| user.email.as_str());
    let sync_session = state.directory.record_sync_session_open(auth_session_id, authorization_generation, &actor.0, &space_id, &document_id, &surface, user_id.as_deref(), authenticated_email, role, &actor.0).await.ok();
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
                        #[cfg(test)]
                        let gated_clock = state.presence_clock.as_ref().filter(|clock| clock.gate_ticks.load(std::sync::atomic::Ordering::Acquire));
                        #[cfg(test)]
                        if let Some(clock) = gated_clock {
                            clock.tick_admitted.add_permits(1);
                            clock.tick_release.acquire().await.expect("presence tick release").forget();
                        }
                        let _ = state.expire_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id, state.presence_now()).await;
                        #[cfg(test)]
                        if let Some(clock) = gated_clock { clock.tick_evaluated.add_permits(1); }
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

/// 🪪️ An admitted command keeps the exact authenticated session, not a reusable user identity.
async fn revalidate_directory_caller(state: &HubState, caller: &AuthedUser) -> Result<(), StatusCode> {
    let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&caller.capability))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if session.id != caller.session_id || session.user_id != caller.user_id || session.authorization_generation != caller.authorization_generation || session.expires_at != caller.expires_at || session.expires_at <= now_ms() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}

#[cfg(feature = "native-artifact-execution")]
struct HubArtifactCreationCommitLeaseV1 {
    _guards: Vec<tokio::sync::OwnedMutexGuard<()>>,
}

#[cfg(feature = "native-artifact-execution")]
impl ArtifactCreationCommitLeaseV1 for HubArtifactCreationCommitLeaseV1 {}

#[cfg(feature = "native-artifact-execution")]
struct HubArtifactCreationCommitAuthorityV1 {
    directory: Arc<HubDirectories>,
    gates: Arc<SocketBindingGatesV1>,
}

#[cfg(feature = "native-artifact-execution")]
impl ArtifactCreationCommitAuthorityV1 for HubArtifactCreationCommitAuthorityV1 {
    fn acquire<'a>(&'a self, actor: &'a ArtifactCreationActorV1, space_id: &'a str) -> ArtifactCreationCommitFutureV1<'a> {
        Box::pin(async move {
            let guards = tokio::time::timeout(
                std::time::Duration::from_secs(2),
                self.gates.acquire_bindings(vec![
                    SocketBindingKeyV1::User(actor.user_id.clone()),
                    SocketBindingKeyV1::Session(actor.session_id.clone()),
                    SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_string() },
                    SocketBindingKeyV1::Membership { user_id: actor.user_id.clone(), space_id: space_id.to_string() },
                ]),
            )
            .await
            .map_err(|_| DirectoryError::Backend("artifact creation final authority unavailable".into()))?;
            match tokio::time::timeout(
                std::time::Duration::from_secs(2),
                self.directory.socket_session_binding(&actor.session_id, &actor.user_id, actor.authorization_generation, Some(space_id), now_ms()),
            )
            .await
            {
                Ok(Ok(SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), .. })) => Ok(Box::new(HubArtifactCreationCommitLeaseV1 { _guards: guards }) as Box<dyn ArtifactCreationCommitLeaseV1>),
                Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => Err(DirectoryError::Backend("artifact creation final authority unavailable".into())),
                _ => Err(DirectoryError::Unauthorized),
            }
        })
    }
}

#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_space_id_v1(space_id: &str) -> bool {
    !space_id.is_empty() && space_id.len() <= 256 && space_id.as_bytes()[0].is_ascii_alphanumeric() && space_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
}

#[cfg(feature = "native-artifact-execution")]
async fn acquire_artifact_creation_actor(
    state: &HubState,
    space_id: &str,
    token: Option<&str>,
) -> Result<(ArtifactCreationActorV1, Vec<tokio::sync::OwnedMutexGuard<()>>), StatusCode> {
    if !artifact_creation_space_id_v1(space_id) { return Err(StatusCode::BAD_REQUEST); }
    let caller = resolve_bearer_user(state, token).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let guards = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.acquire_bindings(vec![
            SocketBindingKeyV1::User(caller.user_id.clone()),
            SocketBindingKeyV1::Session(caller.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_string() },
            SocketBindingKeyV1::Membership { user_id: caller.user_id.clone(), space_id: space_id.to_string() },
        ]),
    )
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let binding = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.directory.socket_session_binding(&caller.session_id, &caller.user_id, caller.authorization_generation, Some(space_id), now_ms()),
    )
    .await;
    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), .. })) => Ok((ArtifactCreationActorV1 { user_id: caller.user_id, session_id: caller.session_id, authorization_generation: caller.authorization_generation }, guards)),
        Ok(Ok(SocketSessionBindingStatus::Active { .. } | SocketSessionBindingStatus::MembershipLost)) => Err(StatusCode::FORBIDDEN),
        Ok(Ok(SocketSessionBindingStatus::Revoked | SocketSessionBindingStatus::Expired)) => Err(StatusCode::UNAUTHORIZED),
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

#[cfg(feature = "native-artifact-execution")]
const ARTIFACT_CREATION_HTTP_CAPACITY: usize = 8;
#[cfg(feature = "native-artifact-execution")]
const ARTIFACT_CREATION_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(32);

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpControlV1 {
    deadline: std::time::Instant,
    cancelled: std::sync::atomic::AtomicBool,
    shutdown_cancelled: Option<Arc<std::sync::atomic::AtomicBool>>,
}

#[cfg(feature = "native-artifact-execution")]
impl ArtifactCreationHttpControlV1 {
    fn new() -> Self {
        Self { deadline: std::time::Instant::now() + std::time::Duration::from_millis(semio_hub::artifact_authority::creation::ARTIFACT_CREATION_DEADLINE_MS), cancelled: std::sync::atomic::AtomicBool::new(false), shutdown_cancelled: None }
    }

    fn recovery(shutdown_cancelled: Arc<std::sync::atomic::AtomicBool>) -> Self {
        Self { shutdown_cancelled: Some(shutdown_cancelled), ..Self::new() }
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(feature = "native-artifact-execution")]
impl AuthorityOperationControl for ArtifactCreationHttpControlV1 {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
            || self.shutdown_cancelled.as_ref().is_some_and(|cancelled| cancelled.load(std::sync::atomic::Ordering::Acquire))
            || std::time::Instant::now() >= self.deadline
    }

    fn report(&self, _progress: AuthorityProgress) {}
}

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpTaskV1 {
    control: Arc<ArtifactCreationHttpControlV1>,
    pending: Arc<ArtifactCreationHttpPendingV1>,
    task: tokio::task::JoinHandle<()>,
}

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpPendingV1 {
    control: Arc<ArtifactCreationHttpControlV1>,
    disposition: std::sync::atomic::AtomicU8,
    changed: tokio::sync::Notify,
}

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpTaskOwnerStateV1 {
    closing: bool,
    reservations: BTreeMap<String, Arc<ArtifactCreationHttpPendingV1>>,
    tasks: BTreeMap<String, ArtifactCreationHttpTaskV1>,
    recovery: Option<tokio::task::JoinHandle<()>>,
}

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpTaskOwnerV1 {
    state: Mutex<ArtifactCreationHttpTaskOwnerStateV1>,
    changed: Arc<tokio::sync::Notify>,
    recovery_cancelled: Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(feature = "native-artifact-execution")]
struct ArtifactCreationHttpReservationV1 {
    owner: Arc<ArtifactCreationHttpTaskOwnerV1>,
    key: String,
    pending: Arc<ArtifactCreationHttpPendingV1>,
    activated: bool,
}

#[cfg(feature = "native-artifact-execution")]
enum ArtifactCreationHttpAdmissionV1 {
    Owner(ArtifactCreationHttpReservationV1),
    Join(Arc<ArtifactCreationHttpPendingV1>),
    Unavailable,
}

#[cfg(feature = "native-artifact-execution")]
impl ArtifactCreationHttpTaskOwnerV1 {
    fn new() -> Self {
        Self {
            state: Mutex::new(ArtifactCreationHttpTaskOwnerStateV1 { closing: false, reservations: BTreeMap::new(), tasks: BTreeMap::new(), recovery: None }),
            changed: Arc::new(tokio::sync::Notify::new()),
            recovery_cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    fn reserve(self: &Arc<Self>, key: String, control: Arc<ArtifactCreationHttpControlV1>) -> ArtifactCreationHttpAdmissionV1 {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        if state.closing { return ArtifactCreationHttpAdmissionV1::Unavailable; }
        if let Some(pending) = state.reservations.get(&key).or_else(|| state.tasks.get(&key).map(|task| &task.pending)) {
            return ArtifactCreationHttpAdmissionV1::Join(pending.clone());
        }
        if state.tasks.len().saturating_add(state.reservations.len()) >= ARTIFACT_CREATION_HTTP_CAPACITY { return ArtifactCreationHttpAdmissionV1::Unavailable; }
        let pending = Arc::new(ArtifactCreationHttpPendingV1 { control, disposition: std::sync::atomic::AtomicU8::new(0), changed: tokio::sync::Notify::new() });
        state.reservations.insert(key.clone(), pending.clone());
        ArtifactCreationHttpAdmissionV1::Owner(ArtifactCreationHttpReservationV1 { owner: self.clone(), key, pending, activated: false })
    }

    fn start_recovery(self: &Arc<Self>, service: Arc<ArtifactCreationServiceV1>, authority: Arc<HubArtifactCreationCommitAuthorityV1>) {
        let cancelled = self.recovery_cancelled.clone();
        let wake = self.changed.clone();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                    _ = wake.notified() => {}
                }
                if cancelled.load(std::sync::atomic::Ordering::Acquire) { break; }
                let scan_now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX));
                let intents = match service.recovery_candidates(scan_now, 32).await {
                    Ok(intents) => intents,
                    Err(error) => {
                        eprintln!("[DEBUG] artifact creation recovery scan unavailable: {error}");
                        continue;
                    }
                };
                for intent in intents {
                    if cancelled.load(std::sync::atomic::Ordering::Acquire) { break; }
                    let control = ArtifactCreationHttpControlV1::recovery(cancelled.clone());
                    let deadline = control.now_ms().saturating_add(semio_hub::artifact_authority::creation::ARTIFACT_CREATION_DEADLINE_MS);
                    let context = OperationContext::new(deadline, AuthorityLimits::maximum(), &control);
                    if let Err(error) = service.recover(intent, authority.as_ref(), &context).await {
                        eprintln!("[DEBUG] artifact creation recovery attempt unavailable: {error}");
                    }
                }
            }
        });
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.closing || state.recovery.is_some() {
            task.abort();
        } else {
            state.recovery = Some(task);
        }
    }

    fn cancel(&self, key: &str) {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(pending) = state.reservations.get(key) { pending.control.cancel(); }
        if let Some(task) = state.tasks.get(key) { task.control.cancel(); }
    }

    async fn shutdown(&self) {
        self.shutdown_with_deadline(ARTIFACT_CREATION_SHUTDOWN_DEADLINE).await;
    }

    async fn shutdown_with_deadline(&self, shutdown_deadline: std::time::Duration) {
        self.recovery_cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.changed.notify_waiters();
        let deadline = tokio::time::Instant::now() + shutdown_deadline;
        loop {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.closing = true;
            for pending in state.reservations.values() { pending.control.cancel(); }
            for task in state.tasks.values() { task.control.cancel(); }
            if state.reservations.is_empty() { break; }
            drop(state);
            if tokio::time::timeout_at(deadline, self.changed.notified()).await.is_err() { break; }
        }
        let (mut tasks, recovery, reservations) = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tasks = std::mem::take(&mut state.tasks).into_values().map(|task| task.task).collect::<Vec<_>>();
            (tasks, state.recovery.take(), std::mem::take(&mut state.reservations).into_values().collect::<Vec<_>>())
        };
        for pending in reservations {
            pending.control.cancel();
            let _ = pending.disposition.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
            pending.changed.notify_waiters();
        }
        self.changed.notify_waiters();
        if let Some(recovery) = recovery {
            tasks.push(recovery);
        }
        if tokio::time::timeout_at(deadline, futures::future::join_all(tasks.iter_mut())).await.is_err() {
            for task in &tasks { task.abort(); }
            for task in tasks { let _ = task.await; }
        }
    }

    #[cfg(test)]
    fn task_count(&self) -> usize {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.reservations.len().saturating_add(state.tasks.values().filter(|task| !task.task.is_finished()).count())
    }
}

#[cfg(feature = "native-artifact-execution")]
impl ArtifactCreationHttpReservationV1 {
    fn activate<F>(mut self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owns_reservation = state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending));
        if !state.closing && owns_reservation {
            state.reservations.remove(&self.key);
            state.tasks.insert(self.key.clone(), ArtifactCreationHttpTaskV1 { control: self.pending.control.clone(), pending: self.pending.clone(), task: tokio::spawn(future) });
            self.activated = true;
        }
        drop(state);
        let disposition = if self.activated { 1 } else { 2 };
        let _ = self.pending.disposition.compare_exchange(0, disposition, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }

    fn finish(mut self) {
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owns_reservation = state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending));
        if owns_reservation { state.reservations.remove(&self.key); }
        let disposition = if !state.closing && owns_reservation { 1 } else { 2 };
        drop(state);
        self.activated = true;
        let _ = self.pending.disposition.compare_exchange(0, disposition, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }
}

#[cfg(feature = "native-artifact-execution")]
impl Drop for ArtifactCreationHttpReservationV1 {
    fn drop(&mut self) {
        if self.activated { return; }
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending)) { state.reservations.remove(&self.key); }
        drop(state);
        let _ = self.pending.disposition.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }
}

#[cfg(feature = "native-artifact-execution")]
async fn await_artifact_creation_http_admission_v1(pending: &ArtifactCreationHttpPendingV1) -> Option<u8> {
    let disposition = pending.disposition.load(std::sync::atomic::Ordering::Acquire);
    if disposition != 0 { return Some(disposition); }
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            let changed = pending.changed.notified();
            let disposition = pending.disposition.load(std::sync::atomic::Ordering::Acquire);
            if disposition != 0 { return disposition; }
            changed.await;
        }
    })
    .await
    .ok()
}

#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_task_key_v1(user_id: &str, space_id: &str, request_id: &str) -> String {
    format!("{user_id}\0{space_id}\0{request_id}")
}

#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_request_id_v1(request_id: &str) -> bool {
    request_id.len() == 32 && request_id.bytes().any(|byte| byte != b'0') && request_id.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_error_status(error: DirectoryError) -> StatusCode {
    match error {
        DirectoryError::NotFound(_) => StatusCode::NOT_FOUND,
        DirectoryError::Conflict(_) => StatusCode::CONFLICT,
        DirectoryError::Unauthorized => StatusCode::UNAUTHORIZED,
        DirectoryError::Backend(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_status_response(status: SpaceArtifactCreationStatusV1) -> Response {
    let code = match status.phase {
        SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing => StatusCode::ACCEPTED,
        SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Indeterminate | SpaceArtifactCreationPhaseV1::Failed | SpaceArtifactCreationPhaseV1::Cancelled => StatusCode::OK,
    };
    let mut response = (code, DirectoryJson(status)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    response
}

#[cfg(feature = "native-artifact-execution")]
async fn get_space_artifact_creation_catalog(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() { return StatusCode::BAD_REQUEST.into_response(); }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.creation_catalog(&actor, &space_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(catalog) => {
            let mut response = DirectoryJson(catalog).into_response();
            response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
            response
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}

#[cfg(feature = "native-artifact-execution")]
async fn post_space_artifact_creation(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some()
        || body.is_empty()
        || body.len() > SPACE_ARTIFACT_CREATION_MAX_BYTES
        || headers.get_all(axum::http::header::CONTENT_TYPE).iter().count() != 1
        || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json")
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let request = match std::str::from_utf8(&body).ok().and_then(SpaceArtifactCreateV1::parse_canonical_json) {
        Some(request) => request,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };
    let Some(service) = state.artifact_creation.as_ref().cloned() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    let key = artifact_creation_task_key_v1(&actor.user_id, &space_id, &request.request_id);
    let control = Arc::new(ArtifactCreationHttpControlV1::new());
    let mut predecessor_failures = 0u8;
    let reservation = loop {
        match state.artifact_creation_tasks.reserve(key.clone(), control.clone()) {
            ArtifactCreationHttpAdmissionV1::Owner(reservation) => break Some(reservation),
            ArtifactCreationHttpAdmissionV1::Join(pending) => match await_artifact_creation_http_admission_v1(pending.as_ref()).await {
                Some(1) => break None,
                Some(2) if predecessor_failures < 2 => {
                    predecessor_failures += 1;
                    continue;
                }
                _ => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
            },
            ArtifactCreationHttpAdmissionV1::Unavailable => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    };
    let deadline = control.now_ms().saturating_add(semio_hub::artifact_authority::creation::ARTIFACT_CREATION_DEADLINE_MS);
    let context = OperationContext::new(deadline, AuthorityLimits::maximum(), control.as_ref());
    let acceptance = match service.accept(&actor, &space_id, request, &context).await {
        Ok(acceptance) => acceptance,
        Err(error) => return artifact_creation_error_status(error).into_response(),
    };
    drop(guards);
    if let Some(execution) = acceptance.execution {
        let Some(reservation) = reservation else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        let authority = state.artifact_creation_commit_authority.clone();
        reservation.activate(async move {
            let deadline = control.now_ms().saturating_add(semio_hub::artifact_authority::creation::ARTIFACT_CREATION_DEADLINE_MS);
            let context = OperationContext::new(deadline, AuthorityLimits::maximum(), control.as_ref());
            if let Err(error) = service.execute(execution, authority.as_ref(), &context).await {
                eprintln!("[DEBUG] artifact creation retained execution unavailable: {error}");
            }
        });
    } else if let Some(reservation) = reservation { reservation.finish(); }
    artifact_creation_status_response(acceptance.status)
}

#[cfg(feature = "native-artifact-execution")]
async fn get_space_artifact_creation_status(Path((space_id, request_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() || !artifact_creation_request_id_v1(&request_id) { return StatusCode::BAD_REQUEST.into_response(); }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.status(&actor, &space_id, &request_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(status) => artifact_creation_status_response(status),
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}

#[cfg(feature = "native-artifact-execution")]
async fn post_space_artifact_creation_cancel(Path((space_id, request_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || !body.is_empty() || !artifact_creation_request_id_v1(&request_id) { return StatusCode::BAD_REQUEST.into_response(); }
    let Some(service) = state.artifact_creation.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    let (actor, _guards) = match acquire_artifact_creation_actor(&state, &space_id, bearer(&headers).as_deref()).await {
        Ok(actor) => actor,
        Err(status) => return status.into_response(),
    };
    match service.cancel(&actor, &space_id, &request_id, u64::try_from(now_ms()).unwrap_or_default()).await {
        Ok(status) => {
            state.artifact_creation_tasks.cancel(&artifact_creation_task_key_v1(&actor.user_id, &space_id, &request_id));
            artifact_creation_status_response(status)
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
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
            if space.owner_user_id == actor_user_id { Ok(()) } else { Err(StatusCode::FORBIDDEN) }
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

#[derive(Debug)]
enum FencedDirectoryCommandErrorV1 {
    Directory(DirectoryError),
    Denied(StatusCode),
    Unavailable,
}

/// 🛡️ Command scope is derived from the closed command, never from a second client claim.
fn directory_command_space(command: &DirectoryCommand) -> Option<&str> {
    match command {
        DirectoryCommand::CreateSpace { .. } => None,
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::ArchiveSpace { space_id }
        | DirectoryCommand::DeleteSpace { space_id }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => Some(space_id),
        DirectoryCommand::AnnounceDocument { descriptor } => Some(&descriptor.space_id),
    }
}

/// 🔗️ Holds ordered principal, space, and exact member gates before the directory writer.
async fn acquire_directory_command_fence(state: &HubState, mut bindings: Vec<SocketBindingKeyV1>, command: &DirectoryCommand) -> Result<Vec<tokio::sync::OwnedMutexGuard<()>>, FencedDirectoryCommandErrorV1> {
    if let Some(space_id) = directory_command_space(command) {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
    }
    if let DirectoryCommand::RemoveMember { space_id, user_id } = command {
        bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() });
    }
    tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_bindings(bindings)).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)
}

/// 🔒️ Durable role and space transitions retire only the authority they changed.
fn invalidate_directory_event_authority(state: &HubState, events: &[DirectoryEvent]) {
    let mut bindings = BTreeSet::new();
    for event in events {
        let binding = match &event.body {
            os_directory::DirectoryEventBody::MemberUpserted { space_id, user_id, .. } | os_directory::DirectoryEventBody::MemberRemoved { space_id, user_id } | os_directory::DirectoryEventBody::InviteRedeemed { space_id, user_id, .. } => {
                SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() }
            }
            os_directory::DirectoryEventBody::SpaceArchived { space_id } | os_directory::DirectoryEventBody::SpaceDeleted { space_id } => SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() },
            _ => continue,
        };
        bindings.insert(binding);
    }
    for binding in bindings {
        state.socket_grants.invalidate_binding(binding.clone());
        state.document_open_plans.invalidate_binding(&binding);
    }
}

#[cfg(test)]
async fn pause_directory_command_authority(state: &HubState, user_id: &str, fenced: bool) {
    if let Some(gate) = &state.live_gate {
        if !fenced {
            gate.directory_command_attempted.add_permits(1);
        }
        let pause = gate.directory_command_pause_user.lock().unwrap().as_ref().is_some_and(|(user, phase)| user == user_id && *phase == fenced);
        if pause {
            gate.directory_command_admitted.add_permits(1);
            gate.directory_command_release.acquire().await.expect("directory command test release").forget();
        }
    }
}

#[cfg(not(test))]
async fn pause_directory_command_authority(_state: &HubState, _user_id: &str, _fenced: bool) {}

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

#[cfg(test)]
async fn pause_admin_effect_started(state: &HubState) {
    if let Some(gate) = &state.live_gate {
        if gate.admin_effect_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
            gate.admin_effect_admitted.add_permits(1);
            gate.admin_effect_release.acquire().await.expect("administrator effect test release").forget();
        }
    }
}

#[cfg(not(test))]
async fn pause_admin_effect_started(_state: &HubState) {}

#[cfg(test)]
async fn execute_directory_command_fenced(state: &HubState, actor: DirectoryActor, command: DirectoryCommand) -> Result<(Vec<DirectoryEvent>, Option<CommandResult>), FencedDirectoryCommandErrorV1> {
    let _authority = acquire_directory_command_fence(state, Vec::new(), &command).await?;
    if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
        pause_directory_command_membership_fence(state).await;
    }
    let result = state.directory_service.execute(actor, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    invalidate_directory_event_authority(state, &result.0);
    Ok(result)
}

/// 🆔️ Reauthenticates inside the mutation fence before consulting any durable receipt.
async fn execute_directory_command_receipt_fenced(
    state: &HubState,
    user: &AuthedUser,
    headers: &HeaderMap,
    peer: SocketAddr,
    claim: NewDirectoryCommandReceipt,
    command: DirectoryCommand,
) -> Result<DirectoryCommandExecutionV1, FencedDirectoryCommandErrorV1> {
    let mut bindings = vec![SocketBindingKeyV1::User(user.user_id.clone()), SocketBindingKeyV1::Session(user.session_id.clone())];
    if let Some(space_id) = directory_command_space(&command) {
        bindings.push(SocketBindingKeyV1::Membership { user_id: user.user_id.clone(), space_id: space_id.to_owned() });
    }
    let _authority = acquire_directory_command_fence(state, bindings, &command).await?;
    revalidate_directory_caller(state, user).await.map_err(FencedDirectoryCommandErrorV1::Denied)?;
    let admin = is_admin(state, headers, Some(peer)).await;
    authorize_directory_command(state, &user.user_id, admin, &command).await.map_err(FencedDirectoryCommandErrorV1::Denied)?;
    pause_directory_command_authority(state, &user.user_id, true).await;
    if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
        pause_directory_command_membership_fence(state).await;
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let execution = state.directory_service.execute_idempotent(actor, claim, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    if let DirectoryCommandExecutionV1::Receipt(receipt) = &execution {
        if receipt.outcome == os_directory::DirectoryCommandOutcomeV1::Accepted {
            invalidate_directory_event_authority(state, &receipt.events);
        }
    }
    Ok(execution)
}

/// 🧾️ `POST /directory/commands` — the closed request/receipt command wire. Authentication and the
/// full §C2 authorization matrix run BEFORE any stored completion is returned, so knowing a request
/// id never resurrects a result for an expired, revoked, or differently-scoped session. The one-shot
/// invite capability is returned to this live call alone; every later resolution of the same id is
/// redacted, proving no duplicate invitation was minted.
async fn post_directory_commands(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, body: Bytes) -> Result<(StatusCode, DirectoryJson<DirectoryCommandReceiptV1>), StatusCode> {
    if body.len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let request = std::str::from_utf8(&body).ok().and_then(|json| DirectoryCommandRequestV1::parse_canonical_json(json).ok()).ok_or(StatusCode::BAD_REQUEST)?;
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let admin = is_admin(&state, &headers, Some(peer)).await;
    authorize_directory_command(&state, &user.user_id, admin, &request.command).await?;
    pause_directory_command_authority(&state, &user.user_id, false).await;
    let claim = NewDirectoryCommandReceipt {
        actor_user_id: user.user_id.clone(),
        request_id: request.request_id.clone(),
        command_sha256: directory_command_sha256(&request.command),
        result_kind: directory_command_result_kind(&request.command),
        claimed_at: now_ms(),
    };
    let execution = execute_directory_command_receipt_fenced(&state, &user, &headers, peer, claim, request.command).await.map_err(|error| match error {
        FencedDirectoryCommandErrorV1::Directory(error) => directory_error_status(error),
        FencedDirectoryCommandErrorV1::Denied(status) => status,
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

const DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS: u64 = 5_000;

/// 🛂️ Strict query admission: nothing, or exactly one opaque `cursor` token.
fn space_administration_request_admission(uri: &axum::http::Uri) -> Result<Option<String>, StatusCode> {
    let Some(query) = uri.query() else { return Ok(None) };
    if query.is_empty() {
        return Ok(None);
    }
    if query.contains('&') || query.contains('%') || query.contains('+') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (name, value) = query.split_once('=').ok_or(StatusCode::BAD_REQUEST)?;
    if name != "cursor" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Some(value.to_owned()))
}

/// 🔐️ Session binding of one administration page — the client only ever receives this digest.
fn space_administration_session_binding_v1(caller: Option<&AuthedUser>, space_id: &str) -> Result<[u8; 32], StatusCode> {
    let Some(caller) = caller else { return Ok([0u8; 32]) };
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/directory-space-administration/session-binding/v1\0");
    hash.update(&u32::try_from(caller.session_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?.to_be_bytes());
    hash.update(caller.session_id.as_bytes());
    hash.update(&u32::try_from(caller.user_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?.to_be_bytes());
    hash.update(caller.user_id.as_bytes());
    hash.update(&caller.authorization_generation.to_be_bytes());
    hash.update(&caller.expires_at.to_be_bytes());
    hash.update(&u32::try_from(space_id.len()).map_err(|_| StatusCode::BAD_REQUEST)?.to_be_bytes());
    hash.update(space_id.as_bytes());
    Ok(hash.finalize().into())
}

fn space_administration_cursor_mac(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, section: DirectorySpaceAdministrationSectionV1, payload: &[u8]) -> [u8; 32] {
    let (session_id, user_id, generation) = caller.map_or(("", "", 0u64), |caller| (caller.session_id.as_str(), caller.user_id.as_str(), caller.authorization_generation));
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/space-administration-cursor/v1\0");
    hash.update(key);
    hash.update(&(session_id.len() as u32).to_be_bytes());
    hash.update(session_id.as_bytes());
    hash.update(&(user_id.len() as u32).to_be_bytes());
    hash.update(user_id.as_bytes());
    hash.update(&generation.to_be_bytes());
    hash.update(&(space_id.len() as u32).to_be_bytes());
    hash.update(space_id.as_bytes());
    hash.update(section.as_str().as_bytes());
    hash.update(&(payload.len() as u32).to_be_bytes());
    hash.update(payload);
    hash.finalize().into()
}

fn space_administration_cursor_encode(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, section: DirectorySpaceAdministrationSectionV1, payload: &[u8]) -> Result<String, StatusCode> {
    let cursor = format!("{}.{}.{}", section.as_str(), os_directory::hex_lower(payload), os_directory::hex_lower(&space_administration_cursor_mac(key, caller, space_id, section, payload)));
    if cursor.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(cursor)
}

fn space_administration_cursor_decode(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, cursor: &str) -> Result<(DirectorySpaceAdministrationSectionV1, Vec<u8>), StatusCode> {
    let mut parts = cursor.split('.');
    let (Some(section), Some(payload), Some(mac), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let section = DirectorySpaceAdministrationSectionV1::parse(section).ok_or(StatusCode::BAD_REQUEST)?;
    let payload = decode_lower_hex(payload).ok_or(StatusCode::BAD_REQUEST)?;
    let mac: [u8; 32] = decode_lower_hex(mac).ok_or(StatusCode::BAD_REQUEST)?.try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    if !semio_hub::directory::constant_time_digest_eq(&space_administration_cursor_mac(key, caller, space_id, section, &payload), &mac) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok((section, payload))
}

fn decode_lower_hex(value: &str) -> Option<Vec<u8>> {
    if value.len() % 2 != 0 || !value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) {
        return None;
    }
    (0..value.len() / 2).map(|index| u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()).collect()
}

fn space_administration_member_key(payload: &[u8]) -> Result<String, StatusCode> {
    String::from_utf8(payload.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)
}

fn space_administration_invite_key(payload: &[u8]) -> Result<(i64, String), StatusCode> {
    if payload.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let created_at = i64::from_be_bytes(payload[..8].try_into().map_err(|_| StatusCode::BAD_REQUEST)?);
    Ok((created_at, String::from_utf8(payload[8..].to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?))
}

fn space_administration_document_key(payload: &[u8]) -> Result<usize, StatusCode> {
    let bytes: [u8; 8] = payload.try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    usize::try_from(u64::from_be_bytes(bytes)).map_err(|_| StatusCode::BAD_REQUEST)
}

fn space_administration_invite_payload(row: &DirectorySpaceAdministrationInviteRowV1) -> Vec<u8> {
    let mut payload = row.created_at_ms.to_be_bytes().to_vec();
    payload.extend_from_slice(row.invite_id.as_bytes());
    payload
}

struct SpaceAdministrationWindows {
    members: Vec<DirectorySpaceAdministrationMemberRowV1>,
    member_storage_more: bool,
    invites: Vec<DirectorySpaceAdministrationInviteRowV1>,
    invite_storage_more: bool,
    documents: Vec<DocumentView>,
    public_documents: Vec<PublicDocumentCatalogEntryV1>,
    document_storage_more: bool,
    document_offset: usize,
}

/// 🧾️ Seals one candidate page over the retained window prefixes and stamps its canonical receipt.
fn seal_space_administration_page_v1(
    state: &HubState,
    caller: Option<&AuthedUser>,
    space_id: &str,
    binding: [u8; 32],
    generation: u64,
    access: DirectorySpaceAccessDecisionV1,
    space: &SpaceView,
    windows: &SpaceAdministrationWindows,
    members: usize,
    invites: usize,
    documents: usize,
) -> Result<DirectorySpaceAdministrationPageV1, StatusCode> {
    let key = &state.space_administration_cursor_key;
    let member_rows = windows.members[..members].to_vec();
    let member_more = windows.member_storage_more || members < windows.members.len();
    let member_window = DirectorySpaceAdministrationMemberWindowV1 {
        next_cursor: match (member_more, member_rows.last()) {
            (true, Some(last)) => Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Members, last.user_id.as_bytes())?),
            (true, None) => return Err(StatusCode::PAYLOAD_TOO_LARGE),
            (false, _) => None,
        },
        rows: member_rows,
    };
    let invite_rows = windows.invites[..invites].to_vec();
    let invite_more = windows.invite_storage_more || invites < windows.invites.len();
    let invite_window = DirectorySpaceAdministrationInviteWindowV1 {
        next_cursor: match (invite_more, invite_rows.last()) {
            (true, Some(last)) => Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Invites, &space_administration_invite_payload(last))?),
            (true, None) => return Err(StatusCode::PAYLOAD_TOO_LARGE),
            (false, _) => None,
        },
        rows: invite_rows,
    };
    let document_more = windows.document_storage_more || documents < windows.documents.len().max(windows.public_documents.len());
    let document_cursor = if document_more {
        if documents == 0 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Documents, &u64::try_from(windows.document_offset + documents).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_be_bytes())?)
    } else {
        None
    };
    let binding_hex = os_directory::hex_lower(&binding);
    let mut page = match access {
        DirectorySpaceAccessDecisionV1::Hidden => return Err(StatusCode::NOT_FOUND),
        DirectorySpaceAccessDecisionV1::Public => DirectorySpaceAdministrationPageV1::Public {
            schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
            session_binding_sha256: binding_hex,
            authorization_generation: generation,
            space_id: space_id.to_owned(),
            space: PublicSpaceViewV1 {
                id: space.id.clone(),
                name: space.name.clone(),
                kind: space.kind,
                visibility: space.visibility,
                member_count: space.member_count,
                document_count: space.document_count,
                created_at_ms: space.created_at_ms,
                updated_at_ms: space.updated_at_ms,
            },
            documents: DirectorySpaceAdministrationPublicDocumentWindowV1 { rows: windows.public_documents[..documents].to_vec(), next_cursor: document_cursor },
            receipt_sha256: String::new(),
        },
        DirectorySpaceAccessDecisionV1::Member | DirectorySpaceAccessDecisionV1::Author => {
            let role = if matches!(access, DirectorySpaceAccessDecisionV1::Author) { DirectorySpaceRole::Author } else { DirectorySpaceRole::Spectator };
            let member_space = MemberSpaceViewV1 {
                id: space.id.clone(),
                name: space.name.clone(),
                kind: space.kind,
                visibility: space.visibility,
                owner_user_id: space.owner_user_id.clone(),
                role,
                member_count: space.member_count,
                document_count: space.document_count,
                active_connections: space.active_connections,
                created_at_ms: space.created_at_ms,
                updated_at_ms: space.updated_at_ms,
            };
            let document_window = DirectorySpaceAdministrationDocumentWindowV1 { rows: windows.documents[..documents].to_vec(), next_cursor: document_cursor };
            if matches!(access, DirectorySpaceAccessDecisionV1::Author) {
                DirectorySpaceAdministrationPageV1::Author {
                    schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
                    session_binding_sha256: binding_hex,
                    authorization_generation: generation,
                    space_id: space_id.to_owned(),
                    space: member_space,
                    members: member_window,
                    documents: document_window,
                    invites: invite_window,
                    capabilities: DirectorySpaceAdministrationCapabilitiesV1 {
                        rename_space: true,
                        set_visibility: true,
                        delete_space: caller.is_some_and(|caller| caller.user_id == space.owner_user_id),
                        upsert_member: true,
                        remove_member: true,
                        create_invite: true,
                        revoke_invite: true,
                    },
                    receipt_sha256: String::new(),
                }
            } else {
                DirectorySpaceAdministrationPageV1::Member {
                    schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
                    session_binding_sha256: binding_hex,
                    authorization_generation: generation,
                    space_id: space_id.to_owned(),
                    space: member_space,
                    members: member_window,
                    documents: document_window,
                    receipt_sha256: String::new(),
                }
            }
        }
    };
    let receipt = os_directory::hex_lower(&Sha256::digest(page.canonical_unsigned_json().as_bytes()));
    match &mut page {
        DirectorySpaceAdministrationPageV1::Public { receipt_sha256, .. } | DirectorySpaceAdministrationPageV1::Member { receipt_sha256, .. } | DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } => *receipt_sha256 = receipt,
    }
    Ok(page)
}

/// 🏛️ One bounded, receipt-bound administration projection of exactly one space. Every row is read
/// through a safe backend projection; the caller's role is re-read after the page reads and before
/// the response is handed out, so a revocation or downgrade answers 401/403 instead of a stale page.
async fn get_directory_space(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectorySpaceAdministrationPageV1>, StatusCode> {
    let cursor = space_administration_request_admission(&uri)?;
    let operation = build_directory_space_administration_page_v1(&state, &space_id, cursor.as_deref(), &headers);
    match tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS), operation).await {
        Ok(result) => result.map(DirectoryJson),
        Err(_) => Err(StatusCode::GATEWAY_TIMEOUT),
    }
}

async fn build_directory_space_administration_page_v1(state: &HubState, space_id: &str, cursor: Option<&str>, headers: &HeaderMap) -> Result<DirectorySpaceAdministrationPageV1, StatusCode> {
    let caller = resolve_bearer_user(state, bearer(headers).as_deref()).await;
    let binding = space_administration_session_binding_v1(caller.as_ref(), space_id)?;
    let generation = caller.as_ref().map_or(0, |caller| caller.authorization_generation);
    let section = match cursor {
        Some(cursor) => Some(space_administration_cursor_decode(&state.space_administration_cursor_key, caller.as_ref(), space_id, cursor)?),
        None => None,
    };
    let summary = state.directory.list_admin_space_summaries_page(Some(space_id), 0, 1).await.map_err(directory_error_status)?.into_iter().next().ok_or(StatusCode::NOT_FOUND)?;
    let space = admin_space_summary_view(summary)?;
    let role = match caller.as_ref() {
        Some(caller) => state.directory.get_role(space_id, &caller.user_id).await.map_err(directory_error_status)?.map(role_wire),
        None => None,
    };
    let access = directory_space_access_decision(space.visibility == DirectorySpaceVisibility::Public, role);
    if matches!(access, DirectorySpaceAccessDecisionV1::Hidden) {
        return Err(StatusCode::NOT_FOUND);
    }
    let member_after = match section {
        Some((DirectorySpaceAdministrationSectionV1::Members, ref payload)) => Some(space_administration_member_key(payload)?),
        _ => None,
    };
    let invite_after = match section {
        Some((DirectorySpaceAdministrationSectionV1::Invites, ref payload)) => Some(space_administration_invite_key(payload)?),
        _ => None,
    };
    let document_offset = match section {
        Some((DirectorySpaceAdministrationSectionV1::Documents, ref payload)) => space_administration_document_key(payload)?,
        _ => 0,
    };
    let mut windows = SpaceAdministrationWindows { members: Vec::new(), member_storage_more: false, invites: Vec::new(), invite_storage_more: false, documents: Vec::new(), public_documents: Vec::new(), document_storage_more: false, document_offset };
    if access.is_member() {
        let mut rows = state.directory.list_space_administration_members_page(space_id, member_after.as_deref(), SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(directory_error_status)?;
        windows.member_storage_more = rows.len() > SPACE_ADMINISTRATION_PAGE_MAX;
        rows.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
        windows.members = rows.into_iter().map(|row| DirectorySpaceAdministrationMemberRowV1 { owner: row.user_id == space.owner_user_id, user_id: row.user_id, email: row.email, display_name: row.display_name, role: role_wire(row.role) }).collect();
    }
    if matches!(access, DirectorySpaceAccessDecisionV1::Author) {
        let mut rows =
            state.directory.list_space_administration_invites_page(space_id, invite_after.as_ref().map(|(created_at, invite_id)| (*created_at, invite_id.as_str())), SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(directory_error_status)?;
        windows.invite_storage_more = rows.len() > SPACE_ADMINISTRATION_PAGE_MAX;
        rows.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
        windows.invites = rows
            .into_iter()
            .map(|row| DirectorySpaceAdministrationInviteRowV1 { invite_id: row.invite_id, role: role_wire(row.role), created_at_ms: row.created_at_ms, expires_at_ms: row.expires_at_ms, revoked: row.revoked, accepted: row.accepted })
            .collect();
    }
    let mut descriptors = state.directory.list_document_descriptors_page(Some(space_id), document_offset, SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(directory_error_status)?;
    windows.document_storage_more = descriptors.len() > SPACE_ADMINISTRATION_PAGE_MAX;
    descriptors.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
    if access.is_member() {
        for descriptor in descriptors {
            windows.documents.push(document_view(state, descriptor).await);
        }
    } else {
        windows.public_documents = descriptors
            .into_iter()
            .map(|descriptor| PublicDocumentCatalogEntryV1 {
                document_id: descriptor.document_id,
                artifact_kind: descriptor.artifact_kind,
                artifact_schema: descriptor.artifact_schema,
                owner: descriptor.owner,
                pack_schema_hash: descriptor.pack_schema_hash,
            })
            .collect();
    }
    let access = revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access).await?;
    let mut members = windows.members.len();
    let mut invites = windows.invites.len();
    let mut documents = windows.documents.len().max(windows.public_documents.len());
    loop {
        let page = seal_space_administration_page_v1(state, caller.as_ref(), space_id, binding, generation, access, &space, &windows, members, invites, documents)?;
        if directory::os_pack::json::to_json_string(&page).len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            page.validate().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Ok(page);
        }
        if documents >= members && documents >= invites && documents > 1 {
            documents -= 1;
        } else if members >= invites && members > 1 {
            members -= 1;
        } else if invites > 1 {
            invites -= 1;
        } else {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
    }
}

/// 🛡️ Re-reads the exact session and current membership after the page reads: a revoked session
/// answers 401 and a role downgrade or removal answers 403/404 instead of a stale administration page.
async fn revalidate_space_administration_caller(state: &HubState, caller: Option<&AuthedUser>, space_id: &str, binding: [u8; 32], space: &SpaceView, observed: DirectorySpaceAccessDecisionV1) -> Result<DirectorySpaceAccessDecisionV1, StatusCode> {
    let Some(caller) = caller else { return Ok(observed) };
    let session = state.directory.authenticate_session(&caller.capability).await.map_err(|_| StatusCode::UNAUTHORIZED)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let current = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability: caller.capability.clone() };
    if current.session_id != caller.session_id || current.user_id != caller.user_id || current.authorization_generation != caller.authorization_generation || space_administration_session_binding_v1(Some(&current), space_id)? != binding {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let role = state.directory.get_role(space_id, &current.user_id).await.map_err(directory_error_status)?.map(role_wire);
    let access = directory_space_access_decision(space.visibility == DirectorySpaceVisibility::Public, role);
    match (observed, access) {
        (DirectorySpaceAccessDecisionV1::Hidden, _) | (_, DirectorySpaceAccessDecisionV1::Hidden) => Err(StatusCode::NOT_FOUND),
        (observed, current) if observed == current => Ok(current),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

async fn post_redeem_invite(Path(token): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let capability = InviteCapability::parse(&token).map_err(directory_error_status)?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let hint = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.invite_redemption_scope_hint(&capability, &actor, &user.user_id)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(directory_error_status)?;
    pause_directory_command_authority(&state, &user.user_id, false).await;
    let _authority = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.acquire_bindings(vec![
            SocketBindingKeyV1::User(user.user_id.clone()),
            SocketBindingKeyV1::Session(user.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: hint.space_id().to_owned() },
            SocketBindingKeyV1::Membership { user_id: user.user_id.clone(), space_id: hint.space_id().to_owned() },
        ]),
    )
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    revalidate_directory_caller(&state, &user).await?;
    pause_directory_command_authority(&state, &user.user_id, true).await;
    let committed = state.directory_service.redeem_invite(actor, &capability, &user.user_id).await.map_err(directory_error_status)?;
    if matches!(committed, semio_hub::directory::model::InviteRedemptionCommit::NewlyCommitted { .. }) {
        invalidate_directory_event_authority(&state, std::slice::from_ref(committed.event()));
    }
    revalidate_directory_caller(&state, &user).await?;
    Ok(DirectoryJson(vec![committed.into_event()]))
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
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) { Err(StatusCode::SERVICE_UNAVAILABLE) } else { Ok(()) }
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

/// 🌐️ Outbound global telemetry borrows a space authority without indexing the global lease by it.
fn directory_stream_message_space(message: &DirectoryStreamMessage) -> Option<&str> {
    match message {
        DirectoryStreamMessage::Event { event } => event.space_id.as_deref(),
        DirectoryStreamMessage::Connection { connection, .. } => Some(&connection.space_id),
        DirectoryStreamMessage::Presence { space_id, .. } => Some(space_id),
        DirectoryStreamMessage::RebootstrapRequired { control } => Some(&control.scope.space_id),
        DirectoryStreamMessage::Heartbeat { .. } => None,
    }
}

fn directory_space_message_bindings(record: &SocketGrantRecordV1, space_id: Option<&str>) -> Vec<SocketBindingKeyV1> {
    let mut bindings = record.bindings();
    if let (SocketAudienceV1::Directory { .. }, SocketSubjectV1::Session { user_id, .. }, Some(space_id)) = (&record.audience, &record.subject, space_id) {
        bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
        bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.to_owned() });
    }
    bindings.sort();
    bindings.dedup();
    bindings
}

fn directory_message_bindings(record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> Vec<SocketBindingKeyV1> {
    directory_space_message_bindings(record, directory_stream_message_space(message))
}

async fn socket_directory_membership_visibility(state: &HubState, record: &SocketGrantRecordV1, message: &DirectoryStreamMessage) -> SocketBindingValidityV1 {
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return SocketBindingValidityV1::Unauthorized };
    let Some(space_id) = directory_stream_message_space(message) else {
        return if matches!(message, DirectoryStreamMessage::Event { event } if event.user_id.as_deref() == Some(user_id.as_str())) { SocketBindingValidityV1::Active } else { SocketBindingValidityV1::Unauthorized };
    };
    match state.directory.get_space(space_id).await {
        Ok(Some(_)) => {}
        Ok(None) => return SocketBindingValidityV1::Unauthorized,
        Err(_) => return SocketBindingValidityV1::Unavailable,
    }
    match state.directory.get_role(space_id, user_id).await {
        Ok(Some(_)) => SocketBindingValidityV1::Active,
        Ok(None) => SocketBindingValidityV1::Unauthorized,
        Err(_) => SocketBindingValidityV1::Unavailable,
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
            os_directory::DirectoryEventBody::DocumentIndexed { scope: indexed, .. } => indexed == scope,
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

#[cfg(test)]
async fn pause_global_directory_send_for_test(state: &HubState, record: &SocketGrantRecordV1, mode: u8) {
    let Some(gate) = &state.live_gate else { return };
    let SocketSubjectV1::Session { user_id, .. } = &record.subject else { return };
    let pause = gate.socket_global_send_pause.lock().expect("global send pause").clone();
    if matches!(&record.audience, SocketAudienceV1::Directory { .. }) && pause.as_ref().is_some_and(|(recipient, point)| recipient == user_id && *point == mode) {
        gate.socket_global_send_admitted.add_permits(1);
        gate.socket_global_send_release.acquire().await.expect("global send release").forget();
    }
}

async fn send_socket_directory_message(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, delivery_space_id: Option<&str>, delivery_epoch: u64, message: &DirectoryStreamMessage) -> ScopedDirectoryFrameDecisionV1 {
    #[cfg(test)]
    pause_global_directory_send_for_test(state, record, 1).await;
    #[cfg(test)]
    if matches!(&record.audience, SocketAudienceV1::DirectoryScoped(_)) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_scoped_send_mode.load(std::sync::atomic::Ordering::Acquire) == 1 {
                gate.socket_scoped_send_admitted.add_permits(1);
                let _ = gate.socket_scoped_send_release.acquire().await;
            }
        }
    }
    let _admission = match socket_live_authority_with_bindings(state, record, live_id, directory_message_bindings(record, message)).await {
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
        SocketAudienceV1::Directory { .. } => match tokio::time::timeout(std::time::Duration::from_secs(2), socket_directory_membership_visibility(state, record, message)).await.unwrap_or(SocketBindingValidityV1::Unavailable) {
            SocketBindingValidityV1::Active => ScopedDirectoryFrameDecisionV1::Deliver,
            SocketBindingValidityV1::Unauthorized => ScopedDirectoryFrameDecisionV1::SkipUnrelated,
            SocketBindingValidityV1::Unavailable => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
        },
        SocketAudienceV1::Document(_) => ScopedDirectoryFrameDecisionV1::CloseUnauthorized,
    };
    if decision != ScopedDirectoryFrameDecisionV1::Deliver {
        return decision;
    }
    let _delivery = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory_service.acquire_delivery_lease(delivery_space_id, delivery_epoch)).await {
        Ok(Some(delivery)) => delivery,
        Ok(None) | Err(_) => return ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    };
    #[cfg(test)]
    pause_global_directory_send_for_test(state, record, 2).await;
    match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, message)).await {
        Ok(true) => ScopedDirectoryFrameDecisionV1::Deliver,
        _ => ScopedDirectoryFrameDecisionV1::CloseUnavailable,
    }
}

async fn send_socket_directory_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, delivery_space_id: Option<&str>, delivery_epoch: u64, scope: &DocumentScope) -> SocketBindingValidityV1 {
    let _admission = match socket_live_authority_with_bindings(state, record, live_id, directory_space_message_bindings(record, Some(&scope.space_id))).await {
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
        Some(control) => {
            let _delivery = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory_service.acquire_delivery_lease(delivery_space_id, delivery_epoch)).await {
                Ok(Some(delivery)) => delivery,
                Ok(None) | Err(_) => return SocketBindingValidityV1::Unavailable,
            };
            match tokio::time::timeout(std::time::Duration::from_secs(2), send_directory_message(sender, &DirectoryStreamMessage::RebootstrapRequired { control })).await {
                Ok(true) => SocketBindingValidityV1::Active,
                _ => SocketBindingValidityV1::Unavailable,
            }
        }
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
    let mut delivery_invalidations = state.directory_service.subscribe_delivery_invalidations();
    let delivery_space_id = match &record.audience {
        SocketAudienceV1::DirectoryScoped(scope) => Some(scope.space_id.as_str()),
        SocketAudienceV1::Directory { .. } | SocketAudienceV1::Document(_) => None,
    };
    let delivery_epoch = state.directory_service.delivery_epoch(delivery_space_id);
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
        let message = DirectoryStreamMessage::Event { event: Box::new(event) };
        match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, &message).await {
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
    if state.directory_service.delivery_epoch(delivery_space_id) != delivery_epoch {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-replay-invalidated".into() }))).await;
        return;
    }
    let mut authorization_tick = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            invalidation = delivery_invalidations.recv() => match invalidation {
                Ok(_) => {
                    if state.directory_service.delivery_epoch(delivery_space_id) != delivery_epoch {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-delivery-invalidated".into() }))).await;
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_) | broadcast::error::RecvError::Closed) => {
                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "directory-delivery-invalidated".into() }))).await;
                    break;
                }
            },
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
                    match send_socket_directory_message(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, &message).await {
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
                        Some(scope) => send_socket_directory_rebootstrap(&mut sender, &state, &record, &live_lease.id, delivery_space_id, delivery_epoch, scope).await,
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
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_session_revoke_attempted.add_permits(1);
    }
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

fn admin_audit_visibility(mut receipt: AdminIntentReceiptV1, resolver_live: bool) -> AdminIntentReceiptV1 {
    if receipt.state == AdminIntentStateV1::Accepted && !resolver_live {
        receipt.state = AdminIntentStateV1::Indeterminate;
        receipt.outcome = AdminIntentOutcomeV1 { code: "admin-effect-outcome-indeterminate".into(), durable: false, kick_attempted: None, kick_signalled: None };
    }
    receipt
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
    if rows.iter().any(|row| row.fact.phase != "accepted") {
        return Ok(rows);
    }
    let Some(accepted) = rows.iter().find(|row| row.fact.phase == "accepted") else {
        return Ok(rows);
    };
    let Some(effect) = state.directory.admin_operation_effect_receipt(&accepted.fact.operation_id, &accepted.fact.intent_digest).await.map_err(directory_error_status)? else {
        return Ok(rows);
    };
    let expected_outcome = match accepted.fact.intent_kind.as_str() {
        "create-space" | "rename-space" | "set-space-visibility" | "archive-space" | "delete-space" | "upsert-space-member" | "remove-space-member" | "create-space-invite" | "revoke-space-invite" => "directory-events-appended",
        "issue-document-share" => "share-issued",
        "revoke-document-share" => "share-revoked",
        "revoke-user-sessions" => "sessions-revoked",
        _ => return Ok(rows),
    };
    if effect.outcome_code != expected_outcome {
        return Ok(rows);
    }
    match (accepted.fact.intent_kind.as_str(), accepted.fact.target_kind.as_str()) {
        ("revoke-document-share", "share") => {
            let binding = SocketBindingKeyV1::Share(accepted.fact.target_id.clone());
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
        }
        ("revoke-user-sessions", "user") => {
            let binding = SocketBindingKeyV1::User(accepted.fact.target_id.clone());
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
        }
        _ => {}
    }
    let mut terminal = accepted.fact.clone();
    terminal.occurred_at = effect.committed_at;
    terminal.phase = "succeeded".into();
    terminal.event_seq_first = effect.event_seq_first;
    terminal.event_seq_last = effect.event_seq_last;
    terminal.outcome_code = effect.outcome_code;
    if !append_admin_terminal_with_retry(state, &terminal).await {
        return Ok(rows);
    }
    state.directory.admin_operation_audit_for_operation(&accepted.fact.operation_id).await.map_err(directory_error_status)
}

async fn append_admin_terminal_with_retry(state: &HubState, terminal: &NewAdminOperationAuditRecord) -> bool {
    let deadline = tokio::time::Instant::now() + ADMIN_TERMINAL_RETRY_DEADLINE;
    loop {
        if state.directory.append_admin_operation_audit(terminal).await.is_ok() {
            return true;
        }
        if tokio::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

async fn admin_operation(Path(operation_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let rows = reconcile_stale_admin_acceptance(&state, rows).await?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let progress = state.admin_operations.with(&receipt.operation_id, |runtime| runtime.map(|runtime| runtime.progress()));
    let receipt = admin_audit_visibility(receipt, progress.is_some());
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
    runtime.request_cancel();
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
    secret: Option<AdminIntentSecretResult>,
    outcome: AdminIntentOutcomeV1,
}

enum AdminIntentSecretResult {
    Invite(String),
    Share(String),
}

impl AdminIntentSecretResult {
    fn into_public(mut self) -> AdminIntentResultV1 {
        match &mut self {
            Self::Invite(value) => AdminIntentResultV1 { invite_token: Some(std::mem::take(value)), share_token: None },
            Self::Share(value) => AdminIntentResultV1 { invite_token: None, share_token: Some(std::mem::take(value)) },
        }
    }
}

impl Drop for AdminIntentSecretResult {
    fn drop(&mut self) {
        let value = match self {
            Self::Invite(value) | Self::Share(value) => value,
        };
        let bytes = unsafe { value.as_bytes_mut() };
        for byte in bytes {
            unsafe { std::ptr::write_volatile(byte, 0) };
            #[cfg(test)]
            ADMIN_SECRET_WIPE_BYTES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
static ADMIN_SECRET_WIPE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

struct AdminOperationRuntime {
    deadline: std::time::Instant,
    completed: std::sync::atomic::AtomicU64,
    total: std::sync::atomic::AtomicU64,
    effect_state: std::sync::atomic::AtomicU8,
    cooperative_cancel_requested: std::sync::atomic::AtomicBool,
}

impl AdminOperationRuntime {
    fn request_cancel(&self) -> bool {
        match self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_CANCELLED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire) {
            Ok(_) | Err(ADMIN_EFFECT_CANCELLED) => true,
            Err(_) => {
                self.cooperative_cancel_requested.store(true, std::sync::atomic::Ordering::Release);
                false
            }
        }
    }

    fn admit_effect(&self) -> bool {
        self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_ADMITTED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    fn cancelled_before_effect(&self) -> bool {
        self.effect_state.load(std::sync::atomic::Ordering::Acquire) == ADMIN_EFFECT_CANCELLED
    }

    fn progress(&self) -> AdminOperationProgressV1 {
        AdminOperationProgressV1 {
            completed_events: self.completed.load(std::sync::atomic::Ordering::Acquire),
            total_events: self.total.load(std::sync::atomic::Ordering::Acquire),
            cancel_requested: self.cancelled_before_effect() || self.cooperative_cancel_requested.load(std::sync::atomic::Ordering::Acquire),
        }
    }
}

impl ProjectionRebuildControl for AdminOperationRuntime {
    fn is_cancelled(&self) -> bool {
        self.cancelled_before_effect() || self.cooperative_cancel_requested.load(std::sync::atomic::Ordering::Acquire) || std::time::Instant::now() >= self.deadline
    }

    fn report(&self, progress: ProjectionRebuildProgress) {
        self.completed.store(progress.completed_events, std::sync::atomic::Ordering::Release);
        self.total.store(progress.total_events, std::sync::atomic::Ordering::Release);
    }
}

struct AdminOperationCleanup {
    operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    operation_id: String,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl Drop for AdminOperationCleanup {
    fn drop(&mut self) {
        self.operations.remove(&self.operation_id);
    }
}

struct AdminOperationTask {
    task: tokio::task::JoinHandle<()>,
    runtime: Arc<AdminOperationRuntime>,
}

struct AdminOperationTaskOwnerState {
    closing: bool,
    tasks: BTreeMap<String, AdminOperationTask>,
}

struct AdminOperationTaskOwner {
    state: Mutex<AdminOperationTaskOwnerState>,
    shutdown_deadline: std::time::Duration,
}

impl AdminOperationTaskOwner {
    fn new(shutdown_deadline: std::time::Duration) -> Self {
        Self { state: Mutex::new(AdminOperationTaskOwnerState { closing: false, tasks: BTreeMap::new() }), shutdown_deadline }
    }

    fn spawn<F>(&self, operation_id: String, runtime: Arc<AdminOperationRuntime>, future: F) -> Result<(), F>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        if state.closing || state.tasks.contains_key(&operation_id) {
            return Err(future);
        }
        let task = tokio::spawn(future);
        state.tasks.insert(operation_id, AdminOperationTask { task, runtime });
        Ok(())
    }

    async fn shutdown(&self) {
        let mut tasks = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.closing = true;
            for task in state.tasks.values() {
                task.runtime.request_cancel();
            }
            std::mem::take(&mut state.tasks).into_values().map(|task| task.task).collect::<Vec<_>>()
        };
        if tokio::time::timeout(self.shutdown_deadline, futures::future::join_all(tasks.iter_mut())).await.is_err() {
            for task in &tasks {
                task.abort();
            }
            for task in tasks {
                let _ = task.await;
            }
        }
    }

    #[cfg(test)]
    fn task_count(&self) -> usize {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).tasks.values().filter(|task| !task.task.is_finished()).count()
    }
}

/// 🗝️ Derives one sorted authority union from the closed short intent before any gate is acquired.
fn admin_intent_bindings(principal: &AdminPrincipalV1, intent: &AdminIntentV1) -> Option<Vec<SocketBindingKeyV1>> {
    let mut bindings = vec![SocketBindingKeyV1::User(principal.user_id.clone()), SocketBindingKeyV1::Session(principal.auth_session_id.clone())];
    match intent {
        AdminIntentV1::CreateSpace { request_id, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: admin_create_space_id(request_id) }),
        AdminIntentV1::RenameSpace { space_id, .. }
        | AdminIntentV1::SetSpaceVisibility { space_id, .. }
        | AdminIntentV1::ArchiveSpace { space_id, .. }
        | AdminIntentV1::DeleteSpace { space_id, .. }
        | AdminIntentV1::UpsertSpaceMember { space_id, .. }
        | AdminIntentV1::CreateSpaceInvite { space_id, .. }
        | AdminIntentV1::RevokeSpaceInvite { space_id, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() }),
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => {
            bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() });
            bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() });
        }
        AdminIntentV1::IssueDocumentShare { scope, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() }),
        AdminIntentV1::RevokeDocumentShare { scope, share_id, .. } => {
            bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() });
            bindings.push(SocketBindingKeyV1::Share(share_id.clone()));
        }
        AdminIntentV1::RevokeUserSessions { user_id, .. } => bindings.push(SocketBindingKeyV1::User(user_id.clone())),
        AdminIntentV1::KickConnection { .. } => {}
        AdminIntentV1::RebuildDirectoryProjections { .. } => return None,
    }
    bindings.sort_unstable();
    bindings.dedup();
    Some(bindings)
}

/// 🏛️ A configured administrator retains its exact durable principal through a short side effect.
async fn acquire_admin_intent_authority(state: &HubState, principal: &AdminPrincipalV1, bindings: Vec<SocketBindingKeyV1>) -> Result<Vec<tokio::sync::OwnedMutexGuard<()>>, FencedDirectoryCommandErrorV1> {
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire_bindings(bindings)).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?;
    let provider_digest = admin_provider_digest(&principal.identity_provider);
    if principal.expires_at_ms <= now_ms()
        || !state
            .admin_subjects
            .iter()
            .any(|subject| semio_hub::directory::constant_time_digest_eq(&subject.provider_digest, &provider_digest) && semio_hub::directory::constant_time_digest_eq(&subject.subject_digest, &principal.identity_subject_digest))
    {
        return Err(FencedDirectoryCommandErrorV1::Denied(StatusCode::UNAUTHORIZED));
    }
    let binding = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.socket_session_binding(&principal.auth_session_id, &principal.user_id, principal.authorization_generation, None, now_ms())).await;
    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms })) if expires_at_ms == principal.expires_at_ms => {}
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => return Err(FencedDirectoryCommandErrorV1::Unavailable),
        _ => return Err(FencedDirectoryCommandErrorV1::Denied(StatusCode::UNAUTHORIZED)),
    }
    pause_directory_command_authority(state, &principal.user_id, true).await;
    Ok(admission)
}

fn admin_directory_authority_refusal(error: FencedDirectoryCommandErrorV1) -> AdminIntentExecution {
    let code = if matches!(error, FencedDirectoryCommandErrorV1::Denied(_)) { "admin-authority-changed" } else { "admin-authority-unavailable" };
    AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: code.into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

fn admin_effect_receipt_claim(operation_id: &str, intent_digest: &str, outcome_code: &str) -> NewAdminOperationEffectReceiptV1 {
    NewAdminOperationEffectReceiptV1 { operation_id: operation_id.into(), intent_digest: intent_digest.into(), committed_at: now_ms(), outcome_code: outcome_code.into() }
}

fn admin_effect_uncertain() -> AdminIntentExecution {
    AdminIntentExecution {
        phase: "uncertain",
        event_range: None,
        secret: None,
        outcome: AdminIntentOutcomeV1 { code: "admin-effect-outcome-uncertain".into(), durable: false, kick_attempted: None, kick_signalled: None },
    }
}

fn admin_effect_rejected_before_commit() -> AdminIntentExecution {
    AdminIntentExecution {
        phase: "failed",
        event_range: None,
        secret: None,
        outcome: AdminIntentOutcomeV1 { code: "admin-effect-rejected-before-commit".into(), durable: false, kick_attempted: None, kick_signalled: None },
    }
}

async fn execute_admin_intent(
    state: &HubState,
    principal: &AdminPrincipalV1,
    operation_id: &str,
    intent_digest: &str,
    intent: AdminIntentV1,
    operation_runtime: Option<Arc<AdminOperationRuntime>>,
    authority_owner: &mut Option<Vec<tokio::sync::OwnedMutexGuard<()>>>,
) -> AdminIntentExecution {
    *authority_owner = None;
    if operation_runtime.as_ref().is_some_and(|runtime| runtime.cancelled_before_effect()) {
        return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
    }
    *authority_owner = if let Some(bindings) = admin_intent_bindings(principal, &intent) {
        pause_directory_command_authority(state, &principal.user_id, false).await;
        if operation_runtime.as_ref().is_some_and(|runtime| runtime.cancelled_before_effect()) {
            return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        }
        match acquire_admin_intent_authority(state, principal, bindings).await {
            Ok(authority) => Some(authority),
            Err(error) => return admin_directory_authority_refusal(error),
        }
    } else {
        None
    };
    if operation_runtime.as_ref().is_some_and(|runtime| !runtime.admit_effect()) {
        return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
    }
    pause_admin_effect_started(state).await;
    if let AdminIntentV1::CreateSpace { request_id, name, space_kind, visibility } = &intent {
        let target_id = admin_create_space_id(request_id);
        let effect = admin_effect_receipt_claim(operation_id, intent_digest, "directory-events-appended");
        return match state.directory_service.execute_create_space_with_id_and_admin_effect(principal.event_actor(), target_id, name.clone(), *space_kind, *visibility, &effect).await {
            AdminEffectCommitV1::Applied(events) => AdminIntentExecution {
                phase: "succeeded",
                event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                secret: None,
                outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
            },
            AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
            AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
        };
    }
    if let Some(command) = admin_directory_command(&intent) {
        if authorize_directory_command(state, &principal.user_id, true, &command).await.is_err() {
            return AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-command-denied".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        }
        if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
            pause_directory_command_membership_fence(state).await;
        }
        let effect = admin_effect_receipt_claim(operation_id, intent_digest, "directory-events-appended");
        return match state.directory_service.execute_with_admin_effect(principal.event_actor(), command, &effect).await {
            AdminEffectCommitV1::Applied((events, result)) => {
                invalidate_directory_event_authority(state, &events);
                AdminIntentExecution {
                    phase: "succeeded",
                    event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                    secret: result.and_then(|result| result.invite_token).map(AdminIntentSecretResult::Invite),
                    outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
                }
            }
            AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
            AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
        };
    }
    match intent {
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "share-issued");
            match state.directory.issue_share_token_as_with_admin_effect(&scope, i64::from(ttl_secs), Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(issued) => AdminIntentExecution {
                    phase: "succeeded",
                    event_range: None,
                    secret: Some(AdminIntentSecretResult::Share(issued.capability.expose_once())),
                    outcome: AdminIntentOutcomeV1 { code: "share-issued".into(), durable: true, kick_attempted: None, kick_signalled: None },
                },
                AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, reason_code, .. } => {
            let binding = SocketBindingKeyV1::Share(share_id.clone());
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "share-revoked");
            match state.directory.revoke_share_token_as_with_admin_effect(&scope, &share_id, &reason_code, Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(()) => {}
                AdminEffectCommitV1::RejectedBeforeCommit => return admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => return admin_effect_uncertain(),
            }
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "share-revoked".into(), durable: true, kick_attempted: None, kick_signalled: None } }
        }
        AdminIntentV1::RevokeUserSessions { user_id, reason_code, .. } => {
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "sessions-revoked");
            let revoked = match state.directory.revoke_auth_sessions_for_user_with_admin_effect(&user_id, &reason_code, Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(revoked) => revoked,
                AdminEffectCommitV1::RejectedBeforeCommit => return admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => return admin_effect_uncertain(),
            };
            for session in &revoked {
                let binding = SocketBindingKeyV1::Session(session.id.clone());
                state.socket_grants.invalidate_binding(binding.clone());
                state.document_open_plans.invalidate_binding(&binding);
            }
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
            AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "sessions-revoked".into(), durable: true, kick_attempted: Some(attempted), kick_signalled: Some(signalled) } }
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => match state.session_kicks.get_cloned(&sync_session_id) {
            Some(notify) => {
                notify.notify_one();
                AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "connection-kick-signalled".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(1) } }
            }
            None => AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "connection-not-live".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(0) } },
        },
        AdminIntentV1::RebuildDirectoryProjections { expected_head_seq, .. } => {
            if !matches!(state.directory.head_seq().await, Ok(head) if head == expected_head_seq) {
                return AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-head-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            }
            let control = operation_runtime.unwrap_or_else(|| {
                Arc::new(AdminOperationRuntime {
                    deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
                    completed: std::sync::atomic::AtomicU64::new(0),
                    total: std::sync::atomic::AtomicU64::new(0),
                    effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
                    cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
                })
            });
            match state.directory.rebuild_projections_controlled(control.as_ref()).await {
                Ok(_) => AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-projections-rebuilt".into(), durable: true, kick_attempted: None, kick_signalled: None } },
                Err(_) if control.is_cancelled() => {
                    AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-cancelled".into(), durable: false, kick_attempted: None, kick_signalled: None } }
                }
                Err(_) => AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
            }
        }
        _ => unreachable!("closed admin directory intents were handled before dispatch"),
    }
}

fn admin_execution_receipt(operation_id: String, correlation_id: String, execution: AdminIntentExecution) -> AdminIntentReceiptV1 {
    AdminIntentReceiptV1 {
        operation_id,
        correlation_id,
        state: match execution.phase {
            "succeeded" => AdminIntentStateV1::Succeeded,
            "cancelled" => AdminIntentStateV1::Cancelled,
            _ => AdminIntentStateV1::Failed,
        },
        event_seq_first: execution.event_range.map(|range| range.0),
        event_seq_last: execution.event_range.map(|range| range.1),
        result: execution.secret.map(AdminIntentSecretResult::into_public),
        outcome: execution.outcome,
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
        let resolver_live = state.admin_operations.get_cloned(&receipt.operation_id).is_some();
        return if same { Ok((StatusCode::OK, DirectoryJson(admin_audit_visibility(receipt, resolver_live)))) } else { Err(StatusCode::CONFLICT) };
    }
    let proposed_operation_id = directory::os_identity::time_ordered_id();
    let accepted = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, "accepted", None, "accepted");
    let established = state.directory.append_admin_operation_audit(&accepted).await.map_err(directory_error_status)?;
    if established.fact.operation_id != proposed_operation_id {
        let joined = state.directory.admin_operation_audit_for_request(&request_id).await.map_err(directory_error_status)?;
        return admin_audit_receipt(&joined)
            .map(|receipt| {
                let resolver_live = state.admin_operations.get_cloned(&receipt.operation_id).is_some();
                (StatusCode::OK, DirectoryJson(admin_audit_visibility(receipt, resolver_live)))
            })
            .ok_or(StatusCode::CONFLICT);
    }
    let operation_permit = match state.admin_operation_slots.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            let execution = AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-capacity".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
            state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
            return Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution))));
        }
    };
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    state.admin_operations.insert(proposed_operation_id.clone(), runtime.clone());
    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
    let task_state = state.clone();
    let task_principal = principal.clone();
    let task_request_id = request_id.clone();
    let task_digest = digest.clone();
    let task_operation_id = proposed_operation_id.clone();
    let task_metadata = metadata.clone();
    let task_runtime = runtime.clone();
    let task_headers = headers.clone();
    let asynchronous = matches!(&intent, AdminIntentV1::RebuildDirectoryProjections { .. });
    let retained = async move {
        let _cleanup = AdminOperationCleanup { operations: task_state.admin_operations.clone(), operation_id: task_operation_id.clone(), _permit: operation_permit };
        let mut authority = None;
        let execution = match authenticate_admin_principal(&task_state, &task_headers, Some(peer)).await {
            Ok(fresh) if task_principal.same_authority(&fresh) => Some(execute_admin_intent(&task_state, &task_principal, &task_operation_id, &task_digest, intent, Some(task_runtime.clone()), &mut authority).await),
            _ => Some(AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-authority-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } }),
        };
        let Some(execution) = execution else {
            let _ = response_tx.send(None);
            return;
        };
        if execution.phase == "uncertain" {
            drop(authority);
            let _ = response_tx.send(None);
            return;
        }
        let terminal = new_admin_audit_fact(&task_principal, &task_request_id, &task_digest, &task_operation_id, &task_metadata, execution.phase, execution.event_range, &execution.outcome.code);
        if !append_admin_terminal_with_retry(&task_state, &terminal).await {
            let _ = response_tx.send(None);
            return;
        }
        drop(authority);
        if response_tx.is_closed() {
            drop(execution);
            return;
        }
        if let Err(mut execution) = response_tx.send(Some(execution)) {
            execution.take();
        }
    };
    if state.admin_operation_tasks.spawn(proposed_operation_id.clone(), runtime.clone(), retained).is_err() {
        state.admin_operations.remove(&proposed_operation_id);
        let execution = AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-shutting-down".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
        state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
        return Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution))));
    }
    if asynchronous {
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
    match tokio::time::timeout_at(tokio::time::Instant::from_std(runtime.deadline), response_rx).await {
        Ok(Ok(Some(execution))) => Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution)))),
        Ok(Ok(None)) | Ok(Err(_)) | Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
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

//#region 💡️Inference
/// 🧾️ The one closed error body every inference route publishes; it never names a private object.
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InferenceErrorBodyV1 {
    schema: &'static str,
    code: &'static str,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
fn inference_error_response(error: semio_hub::inference::runtime::InferenceRouteErrorV1) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(InferenceErrorBodyV1 { schema: "semio.hub.inference-error/v1", code: error.code() })).into_response()
}

/// 🎟️ One private Hub approval admission retaining the exact sorted identity and space guards.
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
struct HubGisMapApprovalIngressAuthorityV1 {
    scope: DocumentScope,
    caller: AuthedUser,
    _guards: Vec<tokio::sync::OwnedMutexGuard<()>>,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
impl GisMapApprovalIngressAuthorityV1 for HubGisMapApprovalIngressAuthorityV1 {
    fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    fn user_id(&self) -> &str {
        &self.caller.user_id
    }

    fn session_id(&self) -> &str {
        &self.caller.session_id
    }

    fn authorization_generation(&self) -> u64 {
        self.caller.authorization_generation
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn revalidate_gis_map_approval_delivery(state: &HubState, authority: &HubGisMapApprovalIngressAuthorityV1) -> Result<(), InferenceRouteErrorV1> {
    revalidate_directory_caller(state, &authority.caller).await.map_err(|status| if status == StatusCode::SERVICE_UNAVAILABLE { InferenceRouteErrorV1::Unavailable } else { InferenceRouteErrorV1::Denied })?;
    let role =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_role(&authority.scope.space_id, &authority.caller.user_id)).await.map_err(|_| InferenceRouteErrorV1::Unavailable)?.map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    if role != Some(SpaceRole::Author) {
        return Err(InferenceRouteErrorV1::Denied);
    }
    Ok(())
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn acquire_gis_map_approval_ingress(state: &HubState, scope: DocumentScope, token: Option<&str>) -> Result<Arc<HubGisMapApprovalIngressAuthorityV1>, InferenceRouteErrorV1> {
    let caller = resolve_bearer_user(state, token).await.ok_or(InferenceRouteErrorV1::Denied)?;
    let guards = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.acquire_bindings(vec![
            SocketBindingKeyV1::User(caller.user_id.clone()),
            SocketBindingKeyV1::Session(caller.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() },
            SocketBindingKeyV1::Membership { user_id: caller.user_id.clone(), space_id: scope.space_id.clone() },
        ]),
    )
    .await
    .map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    let authority = Arc::new(HubGisMapApprovalIngressAuthorityV1 { scope, caller, _guards: guards });
    revalidate_gis_map_approval_delivery(state, &authority).await?;
    Ok(authority)
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
fn inference_context<'a>(state: &'a HubState, space_id: &str, document_id: &str, token: &'a Option<String>) -> Result<semio_hub::inference::runtime::InferenceRouteContextV1<'a>, Response> {
    let runtime = state.inference_runtime.as_ref().ok_or_else(|| inference_error_response(semio_hub::inference::runtime::InferenceRouteErrorV1::Unavailable))?;
    let scope = DocumentScope::new(space_id, document_id);
    Ok(semio_hub::inference::runtime::InferenceRouteContextV1 {
        runtime,
        directory: &state.directory,
        rebootstrap: &state.rebootstrap,
        document_write: state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(scope.clone())),
        scope,
        token: token.as_deref(),
        now_ms: u64::try_from(now_ms()).unwrap_or(0),
    })
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn post_inference_gis_map_job(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::submit_gis_map_job(context, &body).await {
        Ok(receipt) => Json(receipt).into_response(),
        Err(error) => inference_error_response(error),
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn get_inference_gis_map_job_events(Path((space_id, document_id, job_id)): Path<(String, String, String)>, Query(query): Query<InferenceEventQueryV1>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::read_gis_map_job_events(context, &job_id, query.after.unwrap_or(0)).await {
        Ok(page) => Json(page).into_response(),
        Err(error) => inference_error_response(error),
    }
}

/// 🔖️ The only inference query parameter: an exact bounded progress cursor.
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InferenceEventQueryV1 {
    after: Option<u64>,
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn post_inference_gis_map_job_cancel(Path((space_id, document_id, job_id)): Path<(String, String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::cancel_gis_map_job(context, &job_id).await {
        Ok(page) => Json(page).into_response(),
        Err(error) => inference_error_response(error),
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn post_inference_gis_map_job_approval(Path((space_id, document_id, job_id)): Path<(String, String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let route = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let ingress = match acquire_gis_map_approval_ingress(&state, route.scope.clone(), token.as_deref()).await {
        Ok(ingress) => ingress,
        Err(error) => return inference_error_response(error),
    };
    let context = InferenceApprovalRouteContextV1 { route, ingress: ingress.clone() };
    match semio_hub::inference::runtime::approve_gis_map_job(context, &job_id, &body).await {
        Ok(receipt) => match revalidate_gis_map_approval_delivery(&state, &ingress).await {
            Ok(()) => Json(receipt).into_response(),
            Err(error) => inference_error_response(error),
        },
        Err(error) => inference_error_response(error),
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
async fn post_inference_gis_map_approval_undo(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let route = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let ingress = match acquire_gis_map_approval_ingress(&state, route.scope.clone(), token.as_deref()).await {
        Ok(ingress) => ingress,
        Err(error) => return inference_error_response(error),
    };
    let context = InferenceApprovalRouteContextV1 { route, ingress: ingress.clone() };
    match semio_hub::inference::runtime::undo_gis_map_approval(context, &body).await {
        Ok(receipt) => match revalidate_gis_map_approval_delivery(&state, &ingress).await {
            Ok(()) => Json(receipt).into_response(),
            Err(error) => inference_error_response(error),
        },
        Err(error) => inference_error_response(error),
    }
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
fn inference_routes(router: Router<HubState>) -> Router<HubState> {
    router
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs", post(post_inference_gis_map_job).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/events", get(get_inference_gis_map_job_events))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/cancel", post(post_inference_gis_map_job_cancel).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/approval", post(post_inference_gis_map_job_approval).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/approval-undos", post(post_inference_gis_map_approval_undo).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
}

#[cfg(not(all(feature = "sqlite", feature = "native-artifact-execution")))]
fn inference_routes(router: Router<HubState>) -> Router<HubState> {
    router
}

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
const INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
//#endregion 💡️Inference

//#region 🔖️Main
#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_routes(router: Router<HubState>) -> Router<HubState> {
    router
        .route("/spaces/{space_id}/artifact-creations", get(get_space_artifact_creation_catalog).post(post_space_artifact_creation).layer(DefaultBodyLimit::max(SPACE_ARTIFACT_CREATION_MAX_BYTES)))
        .route("/spaces/{space_id}/artifact-creations/{request_id}", get(get_space_artifact_creation_status))
        .route("/spaces/{space_id}/artifact-creations/{request_id}/cancel", post(post_space_artifact_creation_cancel).layer(DefaultBodyLimit::max(0)))
}

#[cfg(not(feature = "native-artifact-execution"))]
fn artifact_creation_routes(router: Router<HubState>) -> Router<HubState> {
    router
}

fn router(state: HubState) -> Router {
    artifact_creation_routes(inference_routes(Router::new()))
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
        .route(
            "/spaces/{space_id}/documents/{document_id}/checkpoint-publications",
            post(post_checkpoint_publication).layer(DefaultBodyLimit::max(CHECKPOINT_PUBLICATION_COMMAND_MAX_BYTES)),
        )
        .route("/spaces/{space_id}/documents/{id}/open-plan", post(issue_document_open_plan))
        .route("/spaces/{space_id}/documents/{id}/socket-grants", post(issue_document_plan_socket_grant))
        .route("/spaces/{space_id}/documents/{id}/execution-target/manifest", post(issue_document_execution_target_manifest))
        .route("/spaces/{space_id}/documents/{id}/execution-target/component", post(issue_document_execution_target_component))
        .route("/spaces/{space_id}/documents/{id}/execution-target/descriptor", post(issue_document_execution_target_descriptor))
        .route("/spaces/{space_id}/documents/{id}/execution-target/browser-actor", post(issue_document_execution_target_browser_actor))
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
    if trusted_catalog_command::dispatch(&std::env::args_os().skip(1).collect::<Vec<_>>()).await? { return Ok(()); }
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
    #[cfg(feature = "native-artifact-execution")]
    let native_codec_providers = NativeCodecProviderSetV1::linked();
    #[cfg(feature = "native-artifact-execution")]
    let native_codec_provider: Option<&dyn NativeCodecProviderSourceV1> = Some(&native_codec_providers);
    #[cfg(not(feature = "native-artifact-execution"))]
    let native_codec_provider: Option<&dyn NativeCodecProviderSourceV1> = None;
    let artifact_authority = configured_artifact_authority(&data_dir, native_codec_provider).await?;
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
    #[cfg(feature = "native-artifact-execution")]
    let gis_map_binding = match verified_catalog.as_ref() {
        Some(catalog) => verified_gis_map_binding(catalog.clone()).map_err(|error| AuthorityError::Catalog(format!("verified GIS Map inference binding rejected: {error:?}")))?,
        None => None,
    };
    let openable_catalog = artifact_authority.as_ref().map(|configured| -> Arc<dyn DocumentOpenCatalogAuthorityV1> { configured.catalog.clone() });
    let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation_tasks = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation = verified_catalog.as_ref().map(|catalog| Arc::new(ArtifactCreationServiceV1::new(directory_service.clone(), catalog.clone(), artifact_cas.clone())));
    #[cfg(feature = "native-artifact-execution")]
    if let Some(service) = artifact_creation.as_ref() {
        artifact_creation_tasks.start_recovery(service.clone(), artifact_creation_commit_authority.clone());
    }
    let fanout = Arc::new(ShardedMap::new());
    let fanout_capacity = 256;
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    let inference_runtime = match gis_map_binding.as_ref() {
        Some(binding) => {
            let root = data_dir.join("inference");
            std::fs::create_dir_all(&root)?;
            let ledger = Arc::new(InferenceJobLedgerV1::open(&root.join("gis-map-jobs.sqlite3")).map_err(|error| AuthorityError::Catalog(format!("inference job ledger unavailable: {error:?}")))?);
            let configured = artifact_authority.as_ref().ok_or_else(|| AuthorityError::Catalog("GIS Map approval requires the configured canonical artifact authority".into()))?;
            let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(GisMapApprovalCheckpointPublisherV1Impl {
                directory: directory.clone(),
                directory_service: directory_service.clone(),
                artifact_cas: artifact_cas.clone(),
                authority: configured.authority.clone(),
                fanout: fanout.clone(),
                fanout_capacity,
            });
            let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(db.clone(), db.storage().await, ledger.clone(), publisher));
            let runtime = Arc::new(HubInferenceRuntimeV1::new(binding.clone(), ledger, committer));
            #[cfg(feature = "test-support")]
            if let Some(descriptor) = std::env::var_os("OS_HUB_TEST_INFERENCE_CHECKPOINT_FD") {
                if mode != HubMode::Development || descriptor.to_str() != Some("4") {
                    return Err(HubError::UnsafeAuthConfiguration("test inference checkpoint control requires development mode and inherited descriptor 4".into()));
                }
                runtime
                    .install_checkpoint_test_gate(Arc::new(InferenceCheckpointTestGateV1::open_inherited().map_err(|_| HubError::UnsafeAuthConfiguration("test inference checkpoint control unavailable".into()))?))
                    .map_err(|_| HubError::UnsafeAuthConfiguration("test inference checkpoint control already installed".into()))?;
            }
            Some(runtime)
        }
        None => None,
    };
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    let inference_ready = inference_runtime.is_some();
    #[cfg(not(all(feature = "sqlite", feature = "native-artifact-execution")))]
    let inference_ready = false;
    let readiness = Arc::new(hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority_ready, open_plan_ready, admin_dir.is_dir(), true, artifact_cas_sweep_execute, inference_ready));
    let admin_cursor_key = SessionCapability::mint()?.secret_digest();
    let space_administration_cursor_key = SessionCapability::mint()?.secret_digest();
    let state = HubState {
        db,
        artifact_cas,
        directory: directory.clone(),
        rebootstrap,
        artifact_authority: artifact_authority.map(|configured| configured.authority),
        verified_catalog,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_commit_authority,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_tasks: artifact_creation_tasks.clone(),
        #[cfg(feature = "native-artifact-execution")]
        gis_map_binding,
        #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
        inference_runtime: inference_runtime.clone(),
        openable_catalog,
        artifact_publication,
        artifact_maintenance: artifact_maintenance.clone(),
        directory_service,
        admin_subjects,
        admin_cursor_key,
        space_administration_cursor_key,
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
        readiness,
        admin_dir,
        fanout,
        fanout_capacity,
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
        #[cfg(test)]
        presence_clock: None,
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates,
        extensions_root,
        merge_policy: merge_policy_from_env(),
    };
    let addr = SocketAddr::new(bind, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let admin_operation_tasks = state.admin_operation_tasks.clone();
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation_tasks = state.artifact_creation_tasks.clone();
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
    admin_operation_tasks.shutdown().await;
    #[cfg(feature = "native-artifact-execution")]
    artifact_creation_tasks.shutdown().await;
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    let inference_close_result = match inference_runtime {
        Some(inference_runtime) => inference_runtime.close().await.map_err(HubError::InferenceShutdown),
        None => Ok(()),
    };
    #[cfg(not(all(feature = "sqlite", feature = "native-artifact-execution")))]
    let inference_close_result: Result<(), HubError> = Ok(());
    artifact_maintenance.shutdown().await;
    result?;
    inference_close_result
}
//#endregion 🔖️Main

#[path = "../../🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs"]
mod trusted_catalog_command;

//#region 🔖️Tests
// 🪶️ Gated on the `sqlite` feature (not just `test`): every test below constructs a `HubState`
// through `SqliteDirectory` — the zero-external-dependency backend — so the full bin test suite
// naturally lives behind the same feature a plain `cargo test` already enables by default (see
// `Cargo.toml`'s `default = ["sqlite"]`). `postgres`/`neo4j` each carry their own backend-only
// tests in `📇️directory/{🐘️postgres,🌐️neo4j}/🦀️.rs` instead of duplicating this suite.
#[cfg(all(test, feature = "sqlite"))]
#[path = "../../🧪️tests/🔬️bin-unit/🦀️.rs"]
mod tests;
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
