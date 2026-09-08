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
mod tests {
    use super::*;
    use directory::os_directory::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1};
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
        let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false, false);
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
        let partial = hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, true, false, false);
        assert_eq!(partial.status, "not-ready");
        assert!(partial.authentication.bootstrap_ready);
        assert!(!partial.artifact_authority.ready);
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), false, false, false, true, true, false, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, false, true, false, false).status, "not-ready");
        assert_eq!(hub_readiness(HubMode::Development, "network", ready.run_id, true, true, false, true, false, false, false).status, "not-ready");
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
    use tokio_tungstenite::tungstenite::{Message as WsMessage, client::IntoClientRequest};

    /// @emoji 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
    const STUDIO: &str = "default";

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn artifact_creation_admission_cannot_activate_after_shutdown_deadline() {
        let owner = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
        let control = Arc::new(ArtifactCreationHttpControlV1::new());
        let reservation = match owner.reserve("user\0space\0request".into(), control) {
            ArtifactCreationHttpAdmissionV1::Owner(reservation) => reservation,
            _ => panic!("first exact admission owns its reservation"),
        };
        let pending = reservation.pending.clone();
        let shutdown = tokio::spawn({
            let owner = owner.clone();
            async move { owner.shutdown_with_deadline(std::time::Duration::from_millis(10)).await }
        });
        loop {
            if owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing { break; }
            tokio::task::yield_now().await;
        }
        shutdown.await.expect("bounded owner shutdown");
        let executed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        reservation.activate({
            let executed = executed.clone();
            async move { executed.store(true, std::sync::atomic::Ordering::Release) }
        });
        tokio::task::yield_now().await;
        assert!(!executed.load(std::sync::atomic::Ordering::Acquire), "a durable late acceptance cannot start its factory after close");
        assert_eq!(pending.disposition.load(std::sync::atomic::Ordering::Acquire), 2, "joiners observe the retired pre-accept owner");
        assert_eq!(owner.task_count(), 0, "shutdown clears reservations and cannot leak a post-close task");
        assert!(matches!(owner.reserve("user\0space\0request".into(), Arc::new(ArtifactCreationHttpControlV1::new())), ArtifactCreationHttpAdmissionV1::Unavailable));
    }

    #[cfg(all(feature = "native-artifact-execution", feature = "test-support"))]
    #[tokio::test]
    async fn space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed() {
        use semio_hub::artifact_authority::creation::ArtifactCreationOperationV1;
        use semio_hub::artifact_authority::trusted_catalog::test_support;

        let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("artifact-creation-http")).await.expect("verified GIS Map creation profile");
        let mut state = test_state().await;
        state.verified_catalog = Some(profile.catalog().clone());
        state.artifact_creation = Some(Arc::new(ArtifactCreationServiceV1::new(state.directory_service.clone(), profile.catalog().clone(), state.artifact_cas.clone())));
        let author = issue_test_session(&state, "creation-author@example.test").await;
        let peer = issue_test_session(&state, "creation-peer@example.test").await;
        let spectator = issue_test_session(&state, "creation-spectator@example.test").await;
        let space_id = create_space_for_test(&state, &author.user_id, "Artifact Creation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, "creation-author@example.test", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, "creation-peer@example.test", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, "creation-spectator@example.test", DirectorySpaceRole::Spectator).await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let route = format!("/spaces/{space_id}/artifact-creations");
        let author_bearer = format!("Bearer {}", author.token);
        let peer_bearer = format!("Bearer {}", peer.token);
        let spectator_bearer = format!("Bearer {}", spectator.token);
        let catalog = raw_http_request(addr, "GET", &route, &[("Authorization", author_bearer.as_str())], &[]).await;
        assert_eq!(catalog.status, 200);
        let catalog_source = std::str::from_utf8(&catalog.body).expect("creation catalog UTF-8");
        let catalog = SpaceArtifactCreationCatalogV1::parse_canonical_json(catalog_source).expect("canonical selected creation catalog");
        assert_eq!(catalog.space_id, space_id);
        assert_eq!(catalog.kinds.iter().map(|kind| kind.kind_id.as_str()).collect::<Vec<_>>(), vec!["s.gis.gismap"]);
        assert_eq!(raw_http_request(addr, "GET", &route, &[], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "GET", &route, &[("Authorization", spectator_bearer.as_str())], &[]).await.status, 403);

        let request = SpaceArtifactCreateV1 { schema: "semio.hub.space-artifact-create/v1".into(), request_id: "1234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.gismap".into(), name: "Shared Map".into() };
        let body = directory::os_pack::json::to_json_string(&request);
        let malformed = body.replacen("{", "{\"documentId\":\"caller-owned\",", 1);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], malformed.as_bytes()).await.status, 400);
        let unknown = SpaceArtifactCreateV1 { request_id: "2234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.unknown".into(), ..request.clone() };
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], directory::os_pack::json::to_json_string(&unknown).as_bytes()).await.status, 409);
        let first = raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], body.as_bytes());
        let duplicate = raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], body.as_bytes());
        let (first, duplicate) = tokio::join!(first, duplicate);
        assert!([200, 202].contains(&first.status) && [200, 202].contains(&duplicate.status), "exact concurrent duplicate never reports capacity or owns a second factory");
        for response in [&first, &duplicate] {
            let source = std::str::from_utf8(&response.body).expect("creation acceptance UTF-8");
            assert!(SpaceArtifactCreationStatusV1::parse_canonical_json(source).is_some(), "creation acceptance is canonical");
        }
        let facts = state.directory.read_artifact_creation(&author.user_id, &request.request_id).await.expect("durable creation facts");
        let operation = ArtifactCreationOperationV1::fold(&facts).expect("one durable creation operation");
        assert!(operation.intent.scope.document_id.strip_prefix("artifact-").is_some_and(artifact_creation_request_id_v1));
        let created_document_id = operation.intent.scope.document_id;

        let status_route = format!("{route}/{}", request.request_id);
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
        let ready = loop {
            let response = raw_http_request(addr, "GET", &status_route, &[("Authorization", author_bearer.as_str())], &[]).await;
            assert_eq!(response.status, 200);
            let status = SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("creation status UTF-8")).expect("canonical creation status");
            if status.phase == SpaceArtifactCreationPhaseV1::Ready { break status; }
            assert!(matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing | SpaceArtifactCreationPhaseV1::Indeterminate), "creation reached an unexpected terminal phase");
            assert!(tokio::time::Instant::now() < deadline, "actual native genesis did not become Ready");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        };
        let ready_scope = DocumentScope::new(&space_id, &ready.ready.as_ref().expect("Ready coordinates").document_id);
        assert_eq!(ready_scope.document_id, created_document_id, "both concurrent requests retain one server-minted document");
        assert!(state.directory.get_document_descriptor(&ready_scope).await.expect("created descriptor read").is_some());
        assert!(state.directory.get_active_artifact_checkpoint(&ready_scope).await.expect("created checkpoint read").is_some_and(|checkpoint| checkpoint.baseline_frontier.is_genesis_for(&ready_scope)));
        assert_eq!(raw_http_request(addr, "GET", &status_route, &[("Authorization", peer_bearer.as_str())], &[]).await.status, 404, "another Author cannot read the private request key");
        let cancelled = raw_http_request(addr, "POST", &format!("{status_route}/cancel"), &[("Authorization", author_bearer.as_str())], &[]).await;
        assert_eq!(cancelled.status, 200);
        assert_eq!(SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&cancelled.body).expect("cancel status UTF-8")).expect("canonical cancel status").phase, SpaceArtifactCreationPhaseV1::Ready, "cancel cannot overwrite Ready");
        assert_eq!(state.artifact_creation_tasks.task_count(), 0);
        let _ = shutdown.send(());
        server.await.expect("creation HTTP server stop");
        state.artifact_creation_tasks.shutdown().await;
    }

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn trusted_catalog_startup_is_selected_only_by_the_server_owned_data_root() {
        let data_root = std::fs::canonicalize(tempdir("unconfigured-trusted-catalog")).expect("canonical fixture-owned data root");
        assert!(configured_artifact_authority(&data_root, Some(&NativeCodecProviderSetV1::linked())).await.expect("unconfigured authority").is_none());
        std::fs::remove_dir_all(data_root).expect("remove unconfigured trusted catalog fixture");
    }

    #[tokio::test]
    async fn configured_catalog_without_a_native_provider_fails_closed() {
        let unconfigured = tempdir("unconfigured-headless-catalog");
        assert!(configured_artifact_authority(&unconfigured, None).await.expect("unconfigured headless authority").is_none());
        std::fs::create_dir_all(unconfigured.join("trusted-catalog")).expect("trusted catalog directory");
        std::fs::write(unconfigured.join("trusted-catalog/current.json"), b"{}\n").expect("configured current pointer");
        let error = match configured_artifact_authority(&unconfigured, None).await {
            Ok(_) => panic!("configured trusted catalog unexpectedly admitted without its native provider"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("requires the native-artifact-execution provider"));
        std::fs::remove_dir_all(unconfigured).expect("remove headless trusted catalog fixture");
    }

    #[cfg(feature = "native-artifact-execution")]
    fn native_openable_stdio_bundle() -> std::path::PathBuf {
        let root = tempdir("native-openable-stdio");
        let stage = root.join("generation-stage");
        std::fs::create_dir_all(stage.join("components")).expect("stdio component directory");
        std::fs::create_dir_all(stage.join("descriptors")).expect("stdio descriptor directory");
        let component = b"abc";
        let component_sha256 = os_directory::hex_lower(&Sha256::digest(component));
        let component_blake3 = blake3::hash(component).to_hex().to_string();
        let receipts = semio_s_plugin_stdio::registry::native_codec_factory_receipts().expect("artifact-owned stdio receipts");
        let viewer = semio_framework_plugin::Viewer::builder(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_plugin::StandardId("rfc8259"), subset: semio_framework_plugin::SubsetId::ANY })
            .document(["semio", "stdio", "json"])
            .mode("view", semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
            .default_mode_id("view")
            .window_kind_def(<semio_framework_plugin::app::TreeWindowKit as semio_framework_plugin::app::WindowKit>::window_kind())
            .build_definition();
        let mut manifest = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::new("stdio", "Stdio Fixture", receipts[0].package_version).manifest;
        manifest.artifact_kinds = semio_s_plugin_stdio::registry::native_codec_artifact_kinds();
        manifest.apps.push(viewer.clone());
        manifest.topic_contributions.push(semio_s_plugin_stdio::registry::native_artifact_catalog_contribution().expect("synthetic fixture retains full catalog semantics"));
        assert_eq!(manifest.artifact_kinds.len(), receipts.len(), "every descriptor artifact kind has one executable owner receipt");
        assert_eq!(viewer.id, "s.stdio.json@rfc8259/*#viewer", "synthetic JSON fixture keeps the canonical surface coordinate");
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
            execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
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
                "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
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
        bundle["packages"][0]["browserActor"] = serde_json::json!({
            "kind":"closed-browser-actor", "schema":"semio.os.closed-browser-actor.v1", "codegenPolicy":"semio.os.browser-jco-1.27.0-jspi.v1",
            "path":"closed-actor.mjs", "byteLength":component.len(), "sha256":component_sha256,
            "sourceComponentSha256":component_sha256, "sourceDescriptorByteSha256":descriptor_sha256, "policySha256":"41".repeat(32), "importInterfaces":[]
        });
        std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");
        let carried = serde_json::to_vec(&bundle).expect("provisional stdio bundle");
        let (selected_closure_sha256, generation_id) = semio_hub::artifact_authority::trusted_catalog::trusted_profile_digests_json(&carried, "stdio-native-openable-v1").expect("stdio profile digests");
        bundle["profiles"][0]["selectedClosureSha256"] = selected_closure_sha256.into();
        bundle["profiles"][0]["generationId"] = generation_id.into();
        std::fs::write(stage.join("components/stdio.wasm"), component).expect("write stdio component");
        std::fs::write(stage.join("descriptors/stdio.descriptor.semio"), descriptor_bytes).expect("write stdio descriptor");
        let bundle_bytes = serde_json::to_vec_pretty(&bundle).expect("stdio bundle json");
        std::fs::write(stage.join("trusted-catalog.json"), &bundle_bytes).expect("write stdio bundle");
        let trusted_root = root.join("trusted-catalog");
        let generations = trusted_root.join("generations");
        std::fs::create_dir_all(&generations).expect("trusted generation owner");
        let generation = bundle["profiles"][0]["generationId"].as_str().expect("generation id");
        std::fs::rename(stage, generations.join(generation)).expect("publish trusted generation");
        let bundle_sha256 = os_directory::hex_lower(&Sha256::digest(&bundle_bytes));
        let current_bytes = format!(
            r#"{{"profileId":"stdio-native-openable-v1","generationId":"{generation}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#
        )
        .into_bytes();
        std::fs::write(trusted_root.join("current.json"), current_bytes).expect("publish current pointer");
        std::fs::canonicalize(root).expect("canonical fixture-owned data root")
    }

    #[cfg(feature = "native-artifact-execution")]
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
        let root = native_openable_stdio_bundle();
        let configured = configured_artifact_authority(&root, Some(&providers)).await.expect("verified stdio authority").expect("configured stdio authority");
        assert_eq!(configured.catalog.codec_count(), 26);
        assert_eq!(configured.catalog.open_target_count(), 1);
        let mut ready = test_state().await;
        ready.openable_catalog = Some(configured.catalog.clone());
        ready.artifact_authority = Some(configured.authority);
        ready.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
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
        let mut dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
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
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
        test_state_with_directory(dir, directory, directory_capacity, fanout_capacity).await
    }

    async fn test_state_with_directory(dir: std::path::PathBuf, directory: SqliteDirectory, directory_capacity: usize, fanout_capacity: usize) -> HubState {
        let database = db::Database::open_at(hub_worker_pool(), &dir, db::Profile::Test).await.expect("open db");
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
        #[cfg(feature = "native-artifact-execution")]
        let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
        #[cfg(feature = "native-artifact-execution")]
        let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            artifact_authority: None,
            verified_catalog: None,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation: None,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation_commit_authority,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
            #[cfg(feature = "native-artifact-execution")]
            gis_map_binding: None,
            #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
            inference_runtime: None,
            openable_catalog: None,
            artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            admin_cursor_key: [0x5a; 32],
            space_administration_cursor_key: [0xa5; 32],
            admin_operations: Arc::new(ShardedMap::new()),
            admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false, false)),
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
            socket_binding_gates: {
                #[cfg(feature = "native-artifact-execution")]
                { socket_binding_gates }
                #[cfg(not(feature = "native-artifact-execution"))]
                { Arc::new(SocketBindingGatesV1::default()) }
            },
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
        #[cfg(feature = "native-artifact-execution")]
        let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
        #[cfg(feature = "native-artifact-execution")]
        let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
        HubState {
            db: database,
            artifact_cas,
            directory,
            rebootstrap,
            artifact_authority: None,
            verified_catalog: None,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation: None,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation_commit_authority,
            #[cfg(feature = "native-artifact-execution")]
            artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
            #[cfg(feature = "native-artifact-execution")]
            gis_map_binding: None,
            #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
            inference_runtime: None,
            openable_catalog: None,
            artifact_publication,
            artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
            directory_service,
            admin_subjects: Arc::from([]),
            admin_cursor_key: [0x5a; 32],
            space_administration_cursor_key: [0xa5; 32],
            admin_operations: Arc::new(ShardedMap::new()),
            admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
            readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false, false)),
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
            socket_binding_gates: {
                #[cfg(feature = "native-artifact-execution")]
                { socket_binding_gates }
                #[cfg(not(feature = "native-artifact-execution"))]
                { Arc::new(SocketBindingGatesV1::default()) }
            },
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
        let bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(b"document-open-genesis-pack"));
        os_directory::DocumentDescriptor {
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            artifact_kind: "test.artifact".into(),
            artifact_schema: "test.v1".into(),
            owner: os_directory::DocumentOwner { plugin_id: "test.plugin".into(), package_id: "test.package".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash,
        }
    }

    fn artifact_document_id_for_test(label: &str) -> String {
        format!("artifact-{}", &os_directory::hex_lower(&Sha256::digest(label.as_bytes()))[..32])
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
        let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis fixture clock");
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
            published_at_ms: accepted_at_ms,
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

    async fn publish_genesis_checkpoint_for_test(
        state: &HubState,
        actor: ArtifactCreationActorV1,
        catalog_generation: String,
        parent_dialect: directory::os_io::ArtifactDialect,
        descriptor: DocumentDescriptor,
        pack: &[u8],
        spr: &[u8],
    ) -> os_directory::ArtifactCheckpoint {
        use semio_hub::artifact_authority::creation::{
            ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationIntentV1, ArtifactCreationPreparedV1, artifact_creation_command_digest_v1,
        };
        let accepted_at_ms = 1;
        let scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
        let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
        let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
        let mut aggregate = Sha256::new();
        aggregate.update(pack);
        aggregate.update(spr);
        let pack_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, pack).expect("genesis pack manifest plan");
        let spr_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, spr).expect("genesis SPR manifest plan");
        let mut checkpoint = os_directory::ArtifactCheckpoint {
            scope: scope.clone(),
            checkpoint_id: os_directory::ArtifactHash([0; 32]),
            parent_checkpoint_id: None,
            descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("genesis descriptor digest"),
            baseline_frontier: os_directory::ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: os_directory::ArtifactHash([0; 32]) },
            pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
            spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
            aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
            published_at_ms: 1,
        };
        checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("genesis checkpoint identity")));
        let request = SpaceArtifactCreateV1 {
            schema: "semio.hub.space-artifact-create/v1".into(),
            request_id: scope.document_id.strip_prefix("artifact-").expect("creation-owned document id").into(),
            kind_id: descriptor.artifact_kind.clone(),
            name: "Checkpoint publication fixture".into(),
        };
        let intent = ArtifactCreationIntentV1 {
            actor,
            scope: scope.clone(),
            command_sha256: artifact_creation_command_digest_v1(&scope.space_id, &request).expect("genesis creation digest"),
            request,
            catalog_generation,
            owner: descriptor.owner.clone(),
            artifact_schema: descriptor.artifact_schema.clone(),
            pack_schema_hash: descriptor.pack_schema_hash.clone(),
            parent_dialect,
            accepted_at_ms,
            deadline_ms: accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS,
        };
        let prepared = ArtifactCreationPreparedV1 { descriptor, checkpoint: checkpoint.clone(), pack: pack.to_vec(), spr: spr.to_vec() };
        prepared.validate(&intent).expect("exact prepared publication genesis");
        assert!(matches!(state.directory.claim_artifact_creation(&intent).await.expect("claim publication genesis"), ArtifactCreationClaimV1::Accepted(_)));
        state
            .directory
            .append_artifact_creation_fact(&ArtifactCreationFactAppendV1 {
                actor: intent.actor.clone(),
                space_id: scope.space_id.clone(),
                request_id: intent.request.request_id.clone(),
                command_sha256: intent.command_sha256.clone(),
                expected_revision: 1,
                recorded_at_ms: accepted_at_ms,
                body: ArtifactCreationFactBodyV1::Prepared { candidate: prepared.clone() },
            })
            .await
            .expect("prepare publication genesis");
        let pair = ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("genesis ownership plan");
        let reservation = state
            .directory_service
            .reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:checkpoint-publication-genesis-test".into() }, ownership, intent.deadline_ms, accepted_at_ms)
            .await
            .expect("reserve genesis objects");
        let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
        let control = StartupCatalogControl;
        let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
        cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage genesis pack");
        cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage genesis SPR");
        state.directory_service.publish_document_genesis(intent, &prepared, checkpoint.clone(), reservation, accepted_at_ms).await.expect("publish dedicated creation genesis");
        checkpoint
    }

    async fn publish_openable_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str) -> (DocumentDescriptor, os_directory::ArtifactCheckpoint) {
        let pack = b"document-open-genesis-pack";
        let spr = b"document-open-genesis-spr";
        let mut descriptor = document_descriptor_for_test(space_id, document_id);
        descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(pack));
        let session = state
            .directory
            .authenticate_session(&SessionCapability::parse(token).expect("document-open author capability"))
            .await
            .expect("document-open author session read")
            .expect("document-open author session");
        let actor = ArtifactCreationActorV1 { user_id: session.user_id, session_id: session.id, authorization_generation: session.authorization_generation };
        let parent_dialect = directory::os_io::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() };
        let checkpoint = publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor.clone(), pack, spr).await;
        (descriptor, checkpoint)
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

    #[cfg(feature = "native-artifact-execution")]
    struct CheckpointPublicationFixture {
        state: HubState,
        author: TestIssuedSession,
        spectator: TestIssuedSession,
        scope: DocumentScope,
        handle: db::ArtifactHandle,
        command: CheckpointPublicationCommandV1,
        pack: Vec<u8>,
        spr: Vec<u8>,
        catalog_root: std::path::PathBuf,
    }

    #[cfg(feature = "native-artifact-execution")]
    fn checkpoint_publication_command(correlation_id: &str, descriptor: &DocumentDescriptor, snapshot: &db::CheckpointPublicationSnapshot, expected_current: CheckpointPublicationCurrentV1, pack: &[u8], spr: &[u8]) -> CheckpointPublicationCommandV1 {
        let head_edit_id = snapshot.head_edit_id.as_ref().expect("committed checkpoint tip").0.clone();
        CheckpointPublicationCommandV1 {
            schema: "semio.hub.checkpoint-publication-command/v1".into(),
            correlation_id: correlation_id.into(),
            descriptor_digest_v1: descriptor_digest_v1(descriptor).expect("descriptor digest").hex(),
            expected_document_frontier: os_directory::DocumentFrontier { head_seq: snapshot.frontier.head_seq, commit_seq: snapshot.frontier.commit_seq, epoch: snapshot.frontier.epoch },
            expected_current,
            baseline_frontier: CheckpointPublicationFrontierV1 {
                document_id: descriptor.document_id.clone(),
                head_edit_ordinal: snapshot.frontier.head_seq,
                head_edit_id,
                last_commit_seq: snapshot.frontier.commit_seq,
                chain_sha256: os_directory::hex_lower(&snapshot.frontier.chain_hash),
            },
            pack: CheckpointPublicationBlobV1 { sha256: os_directory::hex_lower(&Sha256::digest(pack)), byte_length: pack.len() as u64 },
            spr: CheckpointPublicationBlobV1 { sha256: os_directory::hex_lower(&Sha256::digest(spr)), byte_length: spr.len() as u64 },
        }
    }

    #[cfg(feature = "native-artifact-execution")]
    async fn checkpoint_publication_fixture(label: &str) -> CheckpointPublicationFixture {
        let catalog_root = native_openable_stdio_bundle();
        let providers = NativeCodecProviderSetV1::linked();
        let configured = configured_artifact_authority(&catalog_root, Some(&providers)).await.expect("load stdio publication catalog").expect("configured publication catalog");
        let selection = configured.catalog.selected_document_open().expect("selected stdio JSON target").clone();
        let mut state = test_state().await;
        let author = issue_test_session(&state, &format!("checkpoint-{label}-author@example.test")).await;
        let spectator = issue_test_session(&state, &format!("checkpoint-{label}-spectator@example.test")).await;
        let space_id = create_space_for_test(&state, &author.user_id, &format!("Checkpoint {label}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, &format!("checkpoint-{label}-author@example.test"), DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, &format!("checkpoint-{label}-spectator@example.test"), DirectorySpaceRole::Spectator).await;
        let request_id = os_directory::hex_lower(&Sha256::digest(format!("checkpoint-{label}").as_bytes()))[..32].to_string();
        let scope = DocumentScope::new(space_id, format!("artifact-{request_id}"));
        let descriptor = DocumentDescriptor {
            space_id: scope.space_id.clone(),
            document_id: scope.document_id.clone(),
            artifact_kind: selection.artifact.kind,
            artifact_schema: selection.artifact.schema,
            owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id, package_id: selection.package.package_id, version: selection.package.version, package_hash: selection.package.component_sha256 },
            pack_schema_hash: selection.artifact.pack_schema_hash,
            bootstrap_version: 1,
            bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: String::new(),
        };
        state.artifact_authority = Some(configured.authority);
        let catalog_generation = configured.catalog.generation_id().to_string();
        let parent_dialect = selection.parent_dialect;
        state.openable_catalog = Some(configured.catalog);
        let document = db_artifact_id(&scope);
        let snapshot_value = semio_s_artifact_stdio_json::schema::snapshot::demo_json_snapshot();
        let pack = <semio_s_artifact_stdio_json::JsonSnapshot as directory::os_store::ArtifactPack>::encode_pack(&snapshot_value);
        let spr = directory::os_store::empty_document_spr(&document.0, &descriptor.artifact_schema).await;
        let mut descriptor = descriptor;
        descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(&pack));
        let session = state
            .directory
            .authenticate_session(&SessionCapability::parse(&author.token).expect("publication author capability"))
            .await
            .expect("publication author session read")
            .expect("publication author session");
        let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: session.id, authorization_generation: session.authorization_generation };
        let genesis = publish_genesis_checkpoint_for_test(&state, actor, catalog_generation, parent_dialect, descriptor.clone(), &pack, &spr).await;
        let handle = state.ensure_document(&document).await.expect("publication document actor");
        let batch = db::document::CommandBatch::new(vec![sample_envelope(&format!("checkpoint-{label}-edit-1"), &WireArtifactId(document.0.clone())).await]).await.expect("publication command batch");
        handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("publication actor response").expect("publication edit accepted");
        let snapshot = handle.checkpoint_publication_snapshot().await.expect("publication actor snapshot");
        let command = checkpoint_publication_command(
            "1234567890abcdef1234567890abcdef",
            &descriptor,
            &snapshot,
            CheckpointPublicationCurrentV1::Genesis { checkpoint_id: genesis.checkpoint_id.hex() },
            &pack,
            &spr,
        );
        CheckpointPublicationFixture { state, author, spectator, scope, handle, command, pack, spr, catalog_root }
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog() {
        use semio_hub::artifact_authority::trusted_catalog::test_support;

        let artifact_root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned checkpoint process artifact root"));
        let destination = artifact_root.join("checkpoint-publication-process-fixture");
        let stage = artifact_root.join(format!(".checkpoint-publication-process-fixture-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&stage);
        let _ = std::fs::remove_dir_all(&destination);
        std::fs::create_dir_all(&stage).expect("create process fixture stage");

        let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("checkpoint-process")).await.expect("verified GIS Map process profile");
        let selection = profile.binding().selection();
        assert_eq!((selection.artifact.kind.as_str(), selection.artifact.schema.as_str()), ("s.gis.gismap", "gis.map"));
        let source = profile.bundle_path().parent().expect("profile bundle parent");
        let bundle_bytes = std::fs::read(profile.bundle_path()).expect("read verified profile bundle");
        let bundle: serde_json::Value = serde_json::from_slice(&bundle_bytes).expect("decode verified profile bundle");
        let generation_id = bundle["profiles"][0]["generationId"].as_str().expect("verified generation id");
        let generation = stage.join("data/trusted-catalog/generations").join(generation_id);
        std::fs::create_dir_all(&generation).expect("create trusted generation");
        for name in ["component.wasm", "descriptor.semio", "closed-actor.mjs", "stdio-component.wasm", "stdio-descriptor.semio", "trusted-catalog.json"] {
            std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {name}: {error}"));
        }
        let bundle_sha256 = os_directory::hex_lower(&Sha256::digest(&bundle_bytes));
        std::fs::create_dir_all(stage.join("data/trusted-catalog")).expect("create trusted current owner");
        std::fs::write(
            stage.join("data/trusted-catalog/current.json"),
            format!(
                r#"{{"profileId":"{}","generationId":"{generation_id}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#,
                test_support::GIS_MAP_TEST_PROFILE_ID
            ),
        )
        .expect("write trusted current pointer");

        let pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&gis_map_test_snapshot());
        let spr = directory::os_store::empty_document_spr("", &selection.artifact.schema).await;
        let diff = db::document::encode_pathmap_json(&serde_json::json!({ "checkpoint-process": "committed" })).await.expect("encode process mutation diff");
        let inverse = db::document::encode_pathmap_json(&serde_json::json!({ "checkpoint-process": null })).await.expect("encode process mutation inverse");
        let payload = stage.join("payload");
        std::fs::create_dir_all(&payload).expect("create process payload owner");
        for (name, bytes) in [("pack.bin", pack.as_slice()), ("spr.bin", spr.as_slice()), ("diff.bin", diff.as_slice()), ("inverse.bin", inverse.as_slice())] {
            std::fs::write(payload.join(name), bytes).unwrap_or_else(|error| panic!("write process {name}: {error}"));
        }
        let fixture = serde_json::json!({
            "schema": "semio.hub.checkpoint-publication-process-fixture/v1",
            "profileId": test_support::GIS_MAP_TEST_PROFILE_ID,
            "generationId": generation_id,
            "documentId": "mcp-cold-gis-map",
            "mutationId": "mcp-cold-gis-map-edit-1",
            "package": {
                "pluginId": selection.package.plugin_id.as_str(),
                "packageId": selection.package.package_id.as_str(),
                "version": selection.package.version.as_str(),
                "componentSha256": selection.package.component_sha256.as_str()
            },
            "artifact": {
                "kind": selection.artifact.kind.as_str(),
                "schema": selection.artifact.schema.as_str(),
                "packSchemaHash": selection.artifact.pack_schema_hash.as_str()
            },
            "surfaceId": selection.surface.surface_id.as_str(),
            "payload": {
                "pack": { "path": "payload/pack.bin", "byteLength": pack.len(), "sha256": os_directory::hex_lower(&Sha256::digest(&pack)) },
                "spr": { "path": "payload/spr.bin", "byteLength": spr.len(), "sha256": os_directory::hex_lower(&Sha256::digest(&spr)) },
                "diff": { "path": "payload/diff.bin", "schema": db::document::DB_PATHMAP_SCHEMA },
                "inverse": { "path": "payload/inverse.bin", "schema": db::document::DB_PATHMAP_SCHEMA }
            }
        });
        std::fs::write(stage.join("fixture.json"), serde_json::to_vec_pretty(&fixture).expect("encode process fixture")).expect("write process fixture receipt");
        std::fs::rename(&stage, &destination).expect("publish process fixture atomically");
        let dependency_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🔣️.json")).expect("compiled dependency fixture");
        let expected_files: std::collections::BTreeSet<_> = dependency_fixture["publicationFiles"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect();
        let retained_generation = destination.join("data/trusted-catalog/generations").join(generation_id);
        let actual_files: std::collections::BTreeSet<_> = std::fs::read_dir(&retained_generation).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect();
        assert_eq!(actual_files, expected_files);
        let control = StartupCatalogControl;
        let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
        let relocated = TrustedCatalogLoader::load_current(&destination.join("data"), &NativeCodecProviderSetV1::linked(), &context).await.expect("relocated process catalog dependency closure").expect("relocated current");
        assert_eq!(relocated.codec_count(), 28);
        assert_eq!(relocated.packages().len(), 2);
        assert_eq!(relocated.generation_id(), generation_id);
        eprintln!("[DEBUG] checkpoint process fixture relocated files=6 packages=2 codecs=28");
        assert!(destination.join("data/trusted-catalog/current.json").is_file());
        assert_eq!(std::fs::read(destination.join("payload/pack.bin")).expect("read retained GIS pack"), pack);
        assert_eq!(std::fs::read(destination.join("payload/spr.bin")).expect("read retained GIS SPR"), spr);
    }

    #[cfg(feature = "native-artifact-execution")]
    async fn put_checkpoint_publication_blob(addr: SocketAddr, scope: &DocumentScope, token: &str, bytes: &[u8]) {
        let hash = os_directory::hex_lower(&Sha256::digest(bytes));
        let authorization = format!("Bearer {token}");
        let response = raw_http_request(addr, "PUT", &format!("/spaces/{}/blobs/{hash}", scope.space_id), &[("Authorization", authorization.as_str()), ("Content-Type", "application/octet-stream")], bytes).await;
        assert_eq!(response.status, 200, "checkpoint input blob lands before publication: {}", String::from_utf8_lossy(&response.body));
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

    async fn spawn_restartable_server(state: HubState) -> (SocketAddr, tokio::sync::oneshot::Sender<()>, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state).into_make_service_with_connect_info::<SocketAddr>();
        let (shutdown, stopped) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = stopped.await;
                })
                .await
                .unwrap();
        });
        (addr, shutdown, task)
    }

    #[derive(Debug)]
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

    //#region 💡️Inference
    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    #[tokio::test]
    async fn gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write() {
        let state = test_state().await;
        let email = "approval-ingress-author@example.test";
        let caller = issue_test_session(&state, email).await;
        let space_id = create_space_for_test(&state, &caller.user_id, "Approval ingress", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, email, DirectorySpaceRole::Author).await;
        let scope = DocumentScope::new(&space_id, "approval-ingress-document");
        let authority = acquire_gis_map_approval_ingress(&state, scope.clone(), Some(&caller.token)).await.expect("exact Author ingress");
        assert_eq!(authority.scope(), &scope);
        assert_eq!(authority.user_id(), caller.user_id);
        for binding in [
            SocketBindingKeyV1::User(authority.caller.user_id.clone()),
            SocketBindingKeyV1::Session(authority.caller.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() },
            SocketBindingKeyV1::Membership { user_id: authority.caller.user_id.clone(), space_id: space_id.clone() },
        ] {
            assert!(state.socket_binding_gates.gate(binding).try_lock_owned().is_err(), "the exact Hub ingress guard remains owned");
        }
        assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(scope.clone())).try_lock_owned().is_ok(), "Hub ingress must not outer-lock the runtime document writer");
        revalidate_gis_map_approval_delivery(&state, &authority).await.expect("fresh delivery under the retained guards");
        drop(authority);
        execute_directory_command_fenced(
            &state,
            DirectoryActor { kind: DirectoryActorKind::System, id: "system:approval-ingress-law".into() },
            DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: email.into(), role: DirectorySpaceRole::Spectator },
        )
        .await
        .expect("demote after exact authority release");
        assert!(matches!(acquire_gis_map_approval_ingress(&state, scope, Some(&caller.token)).await, Err(InferenceRouteErrorV1::Denied)));
    }

    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    #[tokio::test]
    async fn gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair() {
        let fanout = ShardedMap::new();
        let fanout_capacity = 8;
        let scope = DocumentScope::new("gis-map-space", "gis-map-document");
        let sender = fanout.get_or_insert_with_cloned(document_scope_key_v1(&scope), || broadcast::channel(fanout_capacity).0);
        let mut first = sender.subscribe();
        let mut second = sender.subscribe();
        assert!(matches!(first.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
        assert!(matches!(second.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
        let checkpoint = PublishedArtifactCheckpoint {
            scope: scope.clone(),
            checkpoint_id: ArtifactHash([0x41; 32]),
            parent_checkpoint_id: Some(ArtifactHash([0x40; 32])),
            descriptor_digest_v1: ArtifactHash([0x42; 32]),
            baseline_frontier: ArtifactFrontier {
                document_id: scope.document_id.clone(),
                head_edit_ordinal: 7,
                head_edit_id: "gis-map-create-region".into(),
                last_commit_seq: 5,
                chain_hash: ArtifactHash([0x43; 32]),
            },
            pack: os_directory::PublishedArtifactBlob { sha256: ArtifactHash([0x44; 32]), byte_length: 11 },
            spr: os_directory::PublishedArtifactBlob { sha256: ArtifactHash([0x45; 32]), byte_length: 13 },
            aggregate_sha256: ArtifactHash([0x46; 32]),
            published_at_ms: 17,
        };
        let genesis = ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash([0; 32]) };
        assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &genesis, None));
        for hostile in [
            ArtifactFrontier { document_id: "substituted-document".into(), ..genesis.clone() },
            ArtifactFrontier { head_edit_ordinal: 1, ..genesis.clone() },
            ArtifactFrontier { head_edit_id: "non-genesis".into(), ..genesis.clone() },
            ArtifactFrontier { last_commit_seq: 1, ..genesis.clone() },
            ArtifactFrontier { chain_hash: ArtifactHash([1; 32]), ..genesis.clone() },
        ] {
            assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &hostile, None), "a missing Directory checkpoint is never a publication base");
        }
        assert!(GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &checkpoint.baseline_frontier, Some(&checkpoint)));
        let substituted_scope = DocumentScope::new("substituted-space", scope.document_id.clone());
        assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&substituted_scope, &checkpoint.baseline_frontier, Some(&checkpoint)));
        for hostile in [
            ArtifactFrontier { document_id: "substituted-document".into(), ..checkpoint.baseline_frontier.clone() },
            ArtifactFrontier { head_edit_ordinal: checkpoint.baseline_frontier.head_edit_ordinal + 1, ..checkpoint.baseline_frontier.clone() },
            ArtifactFrontier { head_edit_id: "substituted-edit".into(), ..checkpoint.baseline_frontier.clone() },
            ArtifactFrontier { last_commit_seq: checkpoint.baseline_frontier.last_commit_seq + 1, ..checkpoint.baseline_frontier.clone() },
            ArtifactFrontier { chain_hash: ArtifactHash([0x47; 32]), ..checkpoint.baseline_frontier.clone() },
        ] {
            assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &hostile, Some(&checkpoint)), "an active Directory checkpoint requires the exact scope and base frontier");
        }
        publish_gis_map_checkpoint_change(&fanout, fanout_capacity, &checkpoint);
        let expected = ServerFrame::RebootstrapRequired {
            control: wire_rebootstrap(&os_directory::RebootstrapRequired {
                scope,
                checkpoint_id: checkpoint.checkpoint_id,
                descriptor_digest_v1: checkpoint.descriptor_digest_v1,
                baseline_frontier: checkpoint.baseline_frontier.clone(),
            }),
        };
        assert_eq!(first.recv().await.expect("first peer checkpoint change"), expected);
        assert_eq!(second.recv().await.expect("second peer checkpoint change"), expected);
        assert!(matches!(first.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "one applied checkpoint emits exactly one control per peer");
        assert!(matches!(second.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "one applied checkpoint emits exactly one control per peer");
    }

    #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
    #[tokio::test]
    async fn gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture");
        let unavailable = fixture["errors"].as_array().expect("error vocabulary").iter().find(|row| row["name"] == "no-binding").expect("no-binding row");
        let state = test_state().await;
        assert!(state.inference_runtime.is_none(), "production today has no trusted GIS Map profile");
        assert!(state.gis_map_binding.is_none());
        let session = issue_test_session(&state, "inference-owner@example.test").await;
        let addr = spawn_server(state).await;
        let readiness: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, "/readyz", &[]).await.body).expect("readiness body");
        assert_eq!(readiness["features"]["inference"], false, "readiness publishes inference only with a frozen binding");
        let bearer = format!("Bearer {}", session.token);
        let base = "/spaces/space-a/documents/document-a/inference/gis-map/jobs";
        let intent = serde_json::json!({ "schema": "semio.hub.inference-request/v1", "version": 1, "requestId": "11111111111111111111111111111111", "serviceId": "s.gis.gismap.inference", "policyVersion": 1, "lifetimeMs": 60_000 }).to_string();
        let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": fixture["sampleJobId"], "proposalHash": fixture["proposalHash"] }).to_string();
        let job = fixture["sampleJobId"].as_str().expect("sample job");
        let calls: [(&str, String, &str); 4] =
            [("POST", base.to_string(), intent.as_str()), ("GET", format!("{base}/{job}/events?after=0"), ""), ("POST", format!("{base}/{job}/cancel"), ""), ("POST", format!("{base}/{job}/approval"), approval.as_str())];
        for (method, path, body) in &calls {
            for headers in [vec![("Authorization", bearer.as_str()), ("Content-Type", "application/json")], vec![("Content-Type", "application/json")]] {
                let response = raw_http_request(addr, method, path, &headers, body.as_bytes()).await;
                assert_eq!(u64::from(response.status), unavailable["status"].as_u64().expect("status"), "{method} {path} must fail closed");
                let published: serde_json::Value = serde_json::from_slice(&response.body).expect("closed error body");
                assert_eq!(published["schema"], "semio.hub.inference-error/v1");
                assert_eq!(published["code"], unavailable["code"], "{method} {path}");
                assert_eq!(published.as_object().expect("closed error object").len(), 2, "the closed error body never names a private object");
            }
        }
    }

    /// 🗺️ Builds a real trusted GIS Map editor profile, ledger and runtime on a live `HubState`.
    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    struct GisMapInferenceFixture {
        state: HubState,
        profile: semio_hub::artifact_authority::trusted_catalog::test_support::VerifiedGisMapTestProfileV1,
        space_id: String,
        document_id: String,
        snapshot_pack: Vec<u8>,
        ledger_path: std::path::PathBuf,
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    fn gis_map_test_snapshot() -> semio_s_artifact_gis_gismap::GisMapSnapshot {
        use directory::DslValue;
        use semio_s_artifact_gis_gismap::{GisMapSnapshot, MapFeature};
        let point = |lon: f64, lat: f64| DslValue::object([("lon".into(), DslValue::float(lon)), ("lat".into(), DslValue::float(lat))]);
        let pair = |lon: f64, lat: f64| DslValue::Array(vec![DslValue::float(lon), DslValue::float(lat)]);
        GisMapSnapshot {
            positions: vec![MapFeature { id: "point-a".into(), data: point(7.0, 47.0) }],
            routes: vec![MapFeature { id: "route-a".into(), data: DslValue::object([("points".into(), DslValue::Array(vec![pair(8.0, 46.0), pair(9.0, 48.0)]))]) }],
            regions: Vec::new(),
            ..Default::default()
        }
    }

    /// 🧭️ Seeds one space, one Author, one Spectator, a GIS Map document and its verified checkpoint.
    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    async fn gis_map_inference_fixture(author_email: &str, spectator_email: &str) -> (GisMapInferenceFixture, TestIssuedSession, TestIssuedSession) {
        use semio_hub::artifact_authority::trusted_catalog::test_support;
        let mut state = test_state().await;
        let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("routes")).await.expect("real GIS Map editor profile");
        let ledger_path = tempdir("inference").join("jobs.sqlite3");
        let ledger = semio_hub::inference::sqlite::InferenceJobLedgerV1::open(&ledger_path).expect("private job ledger");
        state.verified_catalog = Some(profile.catalog().clone());
        state.gis_map_binding = Some(profile.binding().clone());
        state.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(profile.binding().clone(), Arc::new(ledger), Arc::new(UnavailableGisMapApprovalCommitterV1))));
        let author = issue_test_session(&state, author_email).await;
        let spectator = issue_test_session(&state, spectator_email).await;
        let space_id = create_space_for_test(&state, &author.user_id, "GIS Map Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, author_email, DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space_id, spectator_email, DirectorySpaceRole::Spectator).await;
        let document_id = artifact_document_id_for_test("gis-map-inference");
        let selection = profile.binding().selection();
        let genesis_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&semio_s_artifact_gis_gismap::GisMapSnapshot::default());
        let genesis_spr = b"gis-map-genesis-spr";
        let descriptor = os_directory::DocumentDescriptor {
            space_id: space_id.clone(),
            document_id: document_id.clone(),
            artifact_kind: selection.artifact.kind.clone(),
            artifact_schema: selection.artifact.schema.clone(),
            owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id.clone(), package_id: selection.package.package_id.clone(), version: selection.package.version.clone(), package_hash: selection.package.component_sha256.clone() },
            pack_schema_hash: selection.artifact.pack_schema_hash.clone(),
            bootstrap_version: 1,
            bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: os_directory::hex_lower(&Sha256::digest(&genesis_pack)),
        };
        let authenticated = state
            .directory
            .authenticate_session(&SessionCapability::parse(&author.token).expect("GIS Map author capability"))
            .await
            .expect("GIS Map author session read")
            .expect("GIS Map author session");
        let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: authenticated.id, authorization_generation: authenticated.authorization_generation };
        let genesis = publish_genesis_checkpoint_for_test(
            &state,
            actor,
            profile.catalog().generation_id().to_string(),
            selection.parent_dialect.clone(),
            descriptor,
            &genesis_pack,
            genesis_spr,
        )
        .await;
        let snapshot_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&gis_map_test_snapshot());
        publish_gis_checkpoint_for_test(&state, &space_id, &document_id, &snapshot_pack, &genesis).await;
        (GisMapInferenceFixture { state, profile, space_id, document_id, snapshot_pack, ledger_path }, author, spectator)
    }

    /// 🧾️ Publishes one verified active checkpoint whose pack is the literal GIS Map snapshot bytes.
    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    async fn publish_gis_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str, pack: &[u8], genesis: &os_directory::ArtifactCheckpoint) {
        let spr = b"gis-map-spr";
        let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
        let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
        let mut aggregate = Sha256::new();
        aggregate.update(pack);
        aggregate.update(spr);
        let scope = DocumentScope::new(space_id, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("announced GIS Map descriptor");
        let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
        let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
        let mut checkpoint = os_directory::ArtifactCheckpoint {
            scope: scope.clone(),
            checkpoint_id: os_directory::ArtifactHash([0; 32]),
            parent_checkpoint_id: Some(genesis.checkpoint_id),
            descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
            baseline_frontier: os_directory::ArtifactFrontier { document_id: document_id.to_string(), head_edit_ordinal: 1, head_edit_id: "gis-map-edit-1".into(), last_commit_seq: 1, chain_hash: os_directory::ArtifactHash([0x44; 32]) },
            pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
            spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
            aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
            published_at_ms: genesis.published_at_ms.saturating_add(1),
        };
        checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
        let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:gis-map-inference-test".into() };
        let reservation = state.directory_service.reserve_artifact_cas(system.clone(), ownership, 1_000, 100).await.expect("reserve checkpoint objects");
        let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
        let control = StartupCatalogControl;
        let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
        cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage GIS Map pack");
        cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage GIS Map SPR");
        state.directory_service.publish_reserved_artifact_checkpoint(system, checkpoint, reservation, 100).await.expect("publish verified GIS Map checkpoint");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    fn inference_route(space_id: &str, document_id: &str, suffix: &str) -> String {
        format!("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs{suffix}")
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    fn inference_intent(request_id: &str) -> String {
        serde_json::json!({ "schema": "semio.hub.inference-request/v1", "version": 1, "requestId": request_id, "serviceId": "s.gis.gismap.inference", "policyVersion": 1, "lifetimeMs": 120_000 }).to_string()
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    async fn wait_for_inference_state(addr: SocketAddr, space_id: &str, document_id: &str, job_id: &str, headers: &[(&str, &str)], expected: &str) -> serde_json::Value {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                let response = raw_http_get(addr, &inference_route(space_id, document_id, &format!("/{job_id}/events?after=0")), headers).await;
                assert_eq!(response.status, 200, "inference event poll failed: {}", String::from_utf8_lossy(&response.body));
                let page: serde_json::Value = serde_json::from_slice(&response.body).expect("inference event page");
                if page["state"] == expected {
                    return page;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("inference reaches its bounded terminal state")
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    fn proposal_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation() {
        let fixture = proposal_fixture();
        let (bound, author, _spectator) = gis_map_inference_fixture("map-owner@example.test", "map-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
        let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
        runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
        let addr = spawn_server(bound.state.clone()).await;
        let bearer = format!("Bearer {}", author.token);
        let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
        let accepted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("11111111111111111111111111111111").as_bytes()).await;
        assert_eq!(accepted.status, 200, "an Author with a bound Map may submit: {}", String::from_utf8_lossy(&accepted.body));
        let receipt: serde_json::Value = serde_json::from_slice(&accepted.body).expect("closed job receipt");
        assert_eq!(receipt["schema"], "semio.hub.inference-job-receipt/v1");
        assert_eq!(receipt["state"], "running", "the submit response returns before retained compute completes");
        assert_eq!(receipt["proposalState"], "none");
        assert_eq!(receipt["proposalHash"], serde_json::Value::Null);
        let job_id = receipt["jobId"].as_str().expect("server-minted job id").to_owned();
        tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached its cancellation checkpoint");
        let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner event page");
        assert_eq!(page["schema"], "semio.hub.inference-job-events/v1");
        assert_eq!(page["stale"], false);
        assert_eq!(page["cancelRequested"], false);
        let kinds: Vec<&str> = page["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
        assert_eq!(kinds, vec!["accepted", "running"], "the private stream exposes Running while actual compute is paused");
        let cursors: Vec<u64> = page["progress"].as_array().expect("progress").iter().map(|row| row["cursor"].as_u64().expect("cursor")).collect();
        assert!(cursors.windows(2).all(|pair| pair[1] == pair[0] + 1), "the progress cursor is monotonic and dense: {cursors:?}");
        assert!(cursors.len() as u64 <= fixture["limits"]["progressMaxCursor"].as_u64().expect("cursor bound"), "progress is bounded");
        assert_eq!(page["nextCursor"].as_u64().expect("next cursor"), cursors.last().copied().unwrap_or(0));
        let cancelled: serde_json::Value = serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/cancel")), &headers, &[]).await.body).expect("cancel page");
        checkpoint.release();
        assert_eq!(cancelled["cancelRequested"], true, "cancellation is durably requested before any terminal effect");
        assert_eq!(cancelled["proposalState"], "none", "a cancelled running job never publishes a private proposal");
        assert_eq!(cancelled["proposalHash"], serde_json::Value::Null, "no private proposal survives cancellation");
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                if runtime.retained_operation_count_for_test().await.expect("retained worker count") == 0 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the cancelled blocking worker is joined and released");
        let after: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("retired page");
        assert!(after["events"].as_array().expect("events").iter().any(|row| row["kind"] == "cancel-requested"));
        assert_eq!(after["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "cancelled").count(), 1, "late codec completion cannot append a second terminal");
        let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": "9".repeat(64) }).to_string();
        let denied = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
        assert_eq!(denied.status, 409, "a cancelled offer can never be approved afterwards");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_inference_runtime_close_signals_and_joins_actual_codec_work() {
        let (bound, author, _spectator) = gis_map_inference_fixture("close-owner@example.test", "close-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
        let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
        runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
        let addr = spawn_server(bound.state.clone()).await;
        let bearer = format!("Bearer {}", author.token);
        let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
        let submitted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("12121212121212121212121212121212").as_bytes()).await;
        assert_eq!(submitted.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&submitted.body).expect("running job receipt");
        assert_eq!(receipt["state"], "running");
        tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the shutdown checkpoint");
        assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 1);
        tokio::time::timeout(std::time::Duration::from_secs(5), runtime.close()).await.expect("runtime close deadline").expect("runtime close");
        assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 0, "close joins the outer and blocking worker before returning");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_inference_revocation_after_compute_refuses_late_publication() {
        let (bound, author, _spectator) = gis_map_inference_fixture("revoked-owner@example.test", "revoked-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
        let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
        runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
        let session = match HubCapability::parse(&author.token).expect("session capability") {
            HubCapability::Session(capability) => bound.state.directory.authenticate_session(&capability).await.expect("session lookup").expect("live session"),
            _ => panic!("test issuer returned a non-session capability"),
        };
        let addr = spawn_server(bound.state.clone()).await;
        let bearer = format!("Bearer {}", author.token);
        let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
        let submitted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("13131313131313131313131313131313").as_bytes()).await;
        assert_eq!(submitted.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&submitted.body).expect("running job receipt");
        let job_id = receipt["jobId"].as_str().expect("job id").to_owned();
        tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the revocation checkpoint");
        let owner = semio_hub::inference::sqlite::InferenceReaderV1 {
            user_id: &session.user_id,
            session_id: &session.id,
            authorization_generation: session.authorization_generation,
            space_id: &space_id,
            document_id: &document_id,
        };
        let identity = runtime.ledger().identity_of(&job_id, &owner).expect("retained owner identity");
        bound.state.directory.revoke_auth_sessions_for_user(&author.user_id, "test-revocation", None, "gis-map-inference-test").await.expect("revoke the retained owner");
        checkpoint.release();
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                if runtime.retained_operation_count_for_test().await.expect("retained worker count") == 0 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("revoked worker reaches terminal");
        let page = runtime.ledger().events(&job_id, &semio_hub::inference::runtime::reader(&identity), 0, u64::try_from(now_ms()).expect("test clock")).expect("private terminal page");
        assert_eq!(page.state, semio_hub::inference::schema::InferenceJobStateV1::Cancelled);
        assert_eq!(page.proposal_state, semio_hub::inference::schema::InferenceProposalStateV1::None);
        assert!(page.proposal_hash.is_none(), "revocation before the final authority fence publishes no proposal");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_proposal_is_private_to_every_peer_spectator_and_stale_caller() {
        let fixture = proposal_fixture();
        let (bound, author, spectator) = gis_map_inference_fixture("private-owner@example.test", "private-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let peer = issue_test_session(&bound.state, "private-peer@example.test").await;
        upsert_member_for_test(&bound.state, &space_id, "private-peer@example.test", DirectorySpaceRole::Author).await;
        let other_space = create_space_for_test(&bound.state, &peer.user_id, "Other Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(bound.state.clone()).await;
        let owner_bearer = format!("Bearer {}", author.token);
        let owner_headers = [("Authorization", owner_bearer.as_str()), ("Content-Type", "application/json")];
        let accepted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &owner_headers, inference_intent("22222222222222222222222222222222").as_bytes()).await;
        assert_eq!(accepted.status, 200, "{}", String::from_utf8_lossy(&accepted.body));
        let receipt: serde_json::Value = serde_json::from_slice(&accepted.body).expect("job receipt");
        let job_id = receipt["jobId"].as_str().expect("job id").to_owned();
        let terminal = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &owner_headers, "succeeded").await;
        let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": terminal["proposalHash"] }).to_string();
        let peer_bearer = format!("Bearer {}", peer.token);
        let spectator_bearer = format!("Bearer {}", spectator.token);
        let denied_code = fixture["visibility"].as_array().expect("visibility").iter().find(|row| row["role"] == "peer-author-same-space").expect("peer row")["expectedCode"].clone();
        for (role, header) in [("peer-author-same-space", peer_bearer.as_str()), ("viewer", spectator_bearer.as_str())] {
            let headers = [("Authorization", header), ("Content-Type", "application/json")];
            for (method, suffix, body) in [("GET", format!("/{job_id}/events?after=0"), String::new()), ("POST", format!("/{job_id}/cancel"), String::new()), ("POST", format!("/{job_id}/approval"), approval.clone())] {
                let response = raw_http_request(addr, method, &inference_route(&space_id, &document_id, &suffix), &headers, body.as_bytes()).await;
                assert_eq!(response.status, 403, "{role} reached {method} {suffix}");
                let published: serde_json::Value = serde_json::from_slice(&response.body).expect("closed denial");
                assert_eq!(published["code"], denied_code, "{role} {method}");
                assert_eq!(published.as_object().expect("closed object").len(), 2, "a denial never names a private object");
            }
        }
        let cross = raw_http_request(addr, "GET", &inference_route(&other_space, &document_id, &format!("/{job_id}/events?after=0")), &[("Authorization", peer_bearer.as_str())], &[]).await;
        assert!(matches!(cross.status, 403 | 404 | 503), "a cross-space read never returns another space's job: {}", cross.status);
        assert_ne!(cross.status, 200);
        let anonymous = raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &[]).await;
        assert_eq!(anonymous.status, 403, "an unauthenticated caller is denied");
        bound.state.directory.revoke_auth_sessions_for_user(&author.user_id, "test-revocation", None, "gis-map-inference-test").await.expect("revoke the original owner sessions");
        let stale = raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &owner_headers).await;
        assert_eq!(stale.status, 403, "a revoked original session loses its own private stream");
        assert!(!bound.snapshot_pack.is_empty(), "the base Map pack the job froze is a real encoded snapshot");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_approval_stamps_one_create_region_and_rejects_every_frozen_drift() {
        let fixture = proposal_fixture();
        let (bound, author, _spectator) = gis_map_inference_fixture("approve-owner@example.test", "approve-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let addr = spawn_server(bound.state.clone()).await;
        let bearer = format!("Bearer {}", author.token);
        let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
        let running: serde_json::Value =
            serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("33333333333333333333333333333333").as_bytes()).await.body).expect("job receipt");
        let job_id = running["jobId"].as_str().expect("job id").to_owned();
        assert_eq!(running["state"], "running");
        let receipt = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &headers, "succeeded").await;
        let proposal_hash = receipt["proposalHash"].as_str().expect("offered hash").to_owned();
        let wrong = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": "9".repeat(64) }).to_string();
        let rejected = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, wrong.as_bytes()).await;
        assert_eq!(rejected.status, 409, "a substituted proposal hash is refused");
        let foreign_job = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": "4".repeat(32), "proposalHash": proposal_hash }).to_string();
        let mismatched = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, foreign_job.as_bytes()).await;
        assert_eq!(mismatched.status, 409, "the body's job id must equal the route's");
        let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": proposal_hash }).to_string();
        let unavailable = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
        let expected = fixture["errors"].as_array().expect("errors").iter().find(|row| row["name"] == "no-composition-transaction").expect("commit-unavailable row");
        assert_eq!(u64::from(unavailable.status), expected["status"].as_u64().expect("status"), "{}", String::from_utf8_lossy(&unavailable.body));
        let published: serde_json::Value = serde_json::from_slice(&unavailable.body).expect("closed error");
        assert_eq!(published["code"], expected["code"], "a Map with composed children must fail closed, never auto-apply");
        let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner page");
        assert_eq!(page["proposalState"], "offered", "a refused publication never marks the proposal approved");
        let kinds: Vec<&str> = page["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
        assert!(kinds.contains(&"approval-prepared"), "the outbox row is durably prepared before publication is attempted");
        assert!(!kinds.contains(&"approved"), "no committed-WAL witness, no approved event");
    }

    #[cfg(all(feature = "sqlite", feature = "test-support"))]
    #[tokio::test]
    async fn gis_map_approval_is_idempotent_across_duplicate_requests_and_restart() {
        let (bound, author, _spectator) = gis_map_inference_fixture("idempotent-owner@example.test", "idempotent-watcher@example.test").await;
        let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
        let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
        let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
        runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
        let addr = spawn_server(bound.state.clone()).await;
        let bearer = format!("Bearer {}", author.token);
        let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
        let first: serde_json::Value = serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("55555555555555555555555555555555").as_bytes()).await.body).expect("job receipt");
        assert_eq!(first["state"], "running");
        tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the duplicate-submit gate");
        let repeated: serde_json::Value =
            serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("55555555555555555555555555555555").as_bytes()).await.body).expect("repeated job receipt");
        assert_eq!(first["jobId"], repeated["jobId"], "one scoped request id can only ever mint one job");
        assert_eq!(repeated["state"], "running", "the duplicate observes the installed owner instead of replacing it");
        assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 1, "one idempotency identity owns exactly one retained worker");
        let job_id = first["jobId"].as_str().expect("job id").to_owned();
        checkpoint.release();
        let terminal = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &headers, "succeeded").await;
        let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": terminal["proposalHash"] }).to_string();
        for attempt in 0..3 {
            let response = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
            assert_eq!(response.status, 503, "attempt {attempt} must reach the same fail-closed publication boundary");
        }
        let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner page");
        let prepared = page["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "approval-prepared").count();
        assert_eq!(prepared, 1, "three duplicate approvals reconcile to exactly one prepared envelope");
        assert_eq!(page["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "succeeded").count(), 1, "the job succeeded exactly once");
        let mut restarted = bound.state.clone();
        let reopened = semio_hub::inference::sqlite::InferenceJobLedgerV1::open(&bound.ledger_path).expect("the durable ledger reopens after a restart");
        restarted.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(bound.profile.binding().clone(), Arc::new(reopened), Arc::new(UnavailableGisMapApprovalCommitterV1))));
        let restarted_addr = spawn_server(restarted).await;
        let recovered: serde_json::Value = serde_json::from_slice(&raw_http_get(restarted_addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("recovered owner page");
        let recovered_kinds: Vec<&str> = recovered["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
        assert_eq!(recovered_kinds.iter().filter(|kind| **kind == "approval-prepared").count(), 1, "restart recovery finds exactly one prepared envelope");
        assert_eq!(recovered_kinds.iter().filter(|kind| **kind == "succeeded").count(), 1, "restart never re-executes an already-offered job");
        assert!(!recovered_kinds.contains(&"approved"), "restart never invents a committed witness");
        let after_restart = raw_http_request(restarted_addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
        assert_eq!(after_restart.status, 503, "approval after restart reaches the same fail-closed publication boundary");
    }
    //#endregion 💡️Inference

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
            browser_actor: fixture.valid_plan.browser_actor.clone(),
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

    const TEST_EXECUTION_TARGET_COMPONENT_BYTES: &[u8] = b"\0asm\x01\0\0\0semio-execution-target-lease-component";
    const TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES: &[u8] = b"semio-execution-target-lease-descriptor";

    fn document_open_catalog_for_descriptor(descriptor: &DocumentDescriptor) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
        document_open_catalog_for_descriptor_with_generation(descriptor, "66".repeat(32))
    }

    fn install_document_open_catalog_for_test(state: &mut HubState, descriptor: &DocumentDescriptor) {
        state.openable_catalog = Some(document_open_catalog_for_descriptor(descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    }

    fn document_open_catalog_for_descriptor_with_generation(descriptor: &DocumentDescriptor, generation_id: String) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
        let package = DocumentOpenPackageV1 {
            plugin_id: descriptor.owner.plugin_id.clone(),
            package_id: descriptor.owner.package_id.clone(),
            version: descriptor.owner.version.clone(),
            component_sha256: descriptor.owner.package_hash.clone(),
            component_blake3: "44".repeat(32),
            descriptor_byte_sha256: "55".repeat(32),
            execution_protocol: os_directory::DocumentExecutionProtocolV1 { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        };
        let artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
        Arc::new(TestDocumentOpenCatalog {
            generation_id,
            component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
            descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
            browser_actor: None,
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
                    browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
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
                    browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
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
        authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("scoped descriptor digest").0);
        authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
        authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
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
            authority.browser_actor = selected.browser_actor;
            authority.grant = selected.grant;
        }
        authority.checkpoint = document_open_checkpoint(
            state.directory.get_active_artifact_checkpoint(&authority.scope).await.expect("checkpoint lookup").expect("committed document checkpoint"),
        );
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
            Err((status, DirectoryJson(error))) => panic!("issue document open plan: {status} {:?}", error.code),
        };
        let exchange = DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan.receipt.clone() };
        let Json(grant) = match issue_document_plan_socket_grant_inner(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), Bytes::from(directory::os_pack::json::to_json_string(&exchange))).await {
            Ok(grant) => grant,
            Err((status, DirectoryJson(error))) => panic!("exchange document open plan: {status} {:?}", error.code),
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

    /// 🪪️ Every execution-target asset route re-authenticates the exact scope and role, reloads the
    /// durable descriptor, and resolves the current trusted selection before any body: it accepts
    /// only the bounded open intent, never a package/digest/generation/path/receipt selector, serves
    /// exactly the selected bytes, and denies after a catalog rotation.
    #[tokio::test]
    async fn execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body() {
        let mut state = test_state().await;
        let document_id = artifact_document_id_for_test("execution-target-assets");
        let token = seed_author_token(&state).await;
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, &document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
        let authorization = format!("Bearer {token}");
        let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let root = format!("/spaces/{STUDIO}/documents/{document_id}/execution-target");
        let intent_body = |surface: &str| {
            directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: scope.clone(),
                requested_surface_id: Some(surface.into()),
                client_instance_id: "client:execution-target".into(),
            })
        };
        let addr = spawn_server(state.clone()).await;

        let manifest = raw_http_request(addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(manifest.status, 200, "{}", String::from_utf8_lossy(&manifest.body));
        let manifest_text = String::from_utf8(manifest.body).expect("manifest UTF-8");
        assert!(!manifest_text.contains(&token) && !manifest_text.contains("client:execution-target") && !manifest_text.contains("sessionId") && !manifest_text.contains("receipt"));
        let fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&manifest_text).expect("manifest JSON");
        fields.validate().expect("route manifest is a valid lease projection");
        assert_eq!(fields.scope, scope);
        assert_eq!(fields.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
        assert_eq!(fields.component.byte_length, TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64);
        assert_eq!(fields.descriptor.byte_length, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.len() as u64);

        let component = raw_http_request(addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(component.status, 200);
        assert_eq!(component.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES);
        let descriptor_body = raw_http_request(addr, "POST", &format!("{root}/descriptor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(descriptor_body.status, 200);
        assert_eq!(descriptor_body.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES);

        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json")).expect("closed actor neutral corpus");
        let mut closed_selection = state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap();
        closed_selection.surface.renderer_target = os_directory::DocumentOpenRendererTargetV1::Wasm;
        let mut actor_json = corpus["plan"]["browserActor"].clone();
        actor_json["sha256"] = serde_json::json!(os_directory::hex_lower(&Sha256::digest(b"abc")));
        actor_json["sourceComponentSha256"] = serde_json::json!(closed_selection.package.component_sha256);
        actor_json["sourceDescriptorByteSha256"] = serde_json::json!(closed_selection.package.descriptor_byte_sha256);
        closed_selection.browser_actor = directory::os_pack::json::from_json_str(&actor_json.to_string()).expect("closed actor fixture");
        let expected_actor = closed_selection
            .browser_actor
            .to_lease(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &closed_selection.package.component_sha256, descriptor_byte_sha256: &closed_selection.package.descriptor_byte_sha256 }, "wasm", Some(3))
            .expect("closed actor lease");
        let mut closed_state = state.clone();
        closed_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
            generation_id: "88".repeat(32),
            open_targets: vec![closed_selection.clone()].into_boxed_slice(),
            component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
            descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
            browser_actor: Some(Arc::from(&b"abc"[..])),
        }));
        let closed_addr = spawn_server(closed_state.clone()).await;
        let closed_manifest = raw_http_request(closed_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(closed_manifest.status, 200);
        let closed_text = String::from_utf8(closed_manifest.body).expect("closed manifest UTF-8");
        let closed_fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&closed_text).expect("closed manifest");
        closed_fields.validate().expect("closed manifest bound");
        assert_eq!(closed_fields.browser_actor, expected_actor);
        for private in ["path", "moduleUrl", "actorBytes", "receipt", "sessionId", "client:execution-target"] {
            assert!(!closed_text.contains(&format!("\"{private}\"")));
        }
        assert!(!closed_text.contains(&token));
        let closed_plan = raw_http_request(closed_addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}/open-plan"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(closed_plan.status, 200);
        let closed_plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&closed_plan.body).unwrap()).expect("closed plan");
        assert_eq!(closed_plan.browser_actor, closed_selection.browser_actor);
        let actor_body = raw_http_request(closed_addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(actor_body.status, 200);
        assert_eq!(actor_body.body, b"abc");
        let absent_actor = raw_http_request(addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(absent_actor.status, 503);
        assert_ne!(absent_actor.body, b"abc");
        for (selection, browser_actor) in [(closed_selection, None), (state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap(), Some(Arc::from(&b"abc"[..])))] {
            let mut invalid_state = state.clone();
            invalid_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
                generation_id: "99".repeat(32),
                open_targets: vec![selection].into_boxed_slice(),
                component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
                descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
                browser_actor,
            }));
            let invalid_addr = spawn_server(invalid_state).await;
            let invalid = raw_http_request(invalid_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
            assert_eq!(invalid.status, 503, "actor identity/body presence mismatch");
        }

        // 🚫 Unauthenticated, foreign-scope, foreign-surface, query-smuggled, oversized and
        // non-JSON requests never reach a byte.
        for asset in ["manifest", "component", "descriptor", "browser-actor"] {
            let route = format!("{root}/{asset}");
            let anonymous = raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], intent_body("surface.test.editor").as_bytes()).await;
            assert_eq!(anonymous.status, 401, "{asset} served an unauthenticated caller");
            let foreign_scope = raw_http_request(addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}-foreign/execution-target/{asset}"), &headers, intent_body("surface.test.editor").as_bytes()).await;
            assert!(foreign_scope.status == 400 || foreign_scope.status == 404, "{asset} accepted a foreign scope with status {}", foreign_scope.status);
            let foreign_surface = raw_http_request(addr, "POST", &route, &headers, intent_body("surface.foreign").as_bytes()).await;
            assert_eq!(foreign_surface.status, 503, "{asset} accepted a foreign surface");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
            let smuggled = raw_http_request(addr, "POST", &format!("{route}?package=semio:fixture"), &headers, intent_body("surface.test.editor").as_bytes()).await;
            assert_eq!(smuggled.status, 400, "{asset} accepted a query selector");
            let unknown_field =
                raw_http_request(addr, "POST", &route, &headers, br#"{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{"spaceId":"studio","documentId":"execution-target-assets"},"clientInstanceId":"c","componentSha256":"aa"}"#)
                    .await;
            assert_eq!(unknown_field.status, 400, "{asset} accepted a client-supplied digest selector");
            let oversized = raw_http_request(addr, "POST", &route, &headers, &vec![b'x'; 9 * 1024]).await;
            assert!(oversized.status == 400 || oversized.status == 413, "{asset} accepted an oversized body");
        }

        // 🔁 A rotation between reads denies every subsequent body rather than mixing generations.
        let mut rotated = state.clone();
        rotated.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
        let rotated_addr = spawn_server(rotated.clone()).await;
        let rotated_manifest = raw_http_request(rotated_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(rotated_manifest.status, 200);
        let rotated_fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&String::from_utf8(rotated_manifest.body).expect("rotated UTF-8")).expect("rotated manifest");
        assert_ne!(rotated_fields.catalog.generation_id, fields.catalog.generation_id);
        assert!(!same_lease_fields_v1(&rotated_fields, &fields));

        // 🚧 An unconfigured catalog advertises nothing and serves nothing.
        let mut unavailable = state.clone();
        unavailable.openable_catalog = None;
        let unavailable_addr = spawn_server(unavailable).await;
        let denied = raw_http_request(unavailable_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(denied.status, 503);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("catalog error")["code"], "catalog-unavailable");
    }

    #[tokio::test]
    async fn execution_target_selection_final_fence_matches_neutral_races() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1/🔣️.json")).expect("execution-target relay fixture");
        for vector in fixture["fences"].as_array().expect("neutral fences") {
            let mut state = test_state().await;
            let token = seed_author_token(&state).await;
            let document_id = artifact_document_id_for_test("execution-target-fence");
            let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
            let scope = DocumentScope::new(STUDIO, document_id);
            state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
            state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
            let mut headers = bearer_headers(&token);
            headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
            let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
            state.document_open_plan_issue_gate = Some(gate.clone());
            let body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: scope.clone(),
                requested_surface_id: Some("surface.test.editor".into()),
                client_instance_id: "client:fence".into(),
            }));
            let task = tokio::spawn(document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), body));
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("target final fence deadline").expect("target reached final fence").forget();
            match vector["mutation"].as_str().expect("mutation") {
                "unchanged" => {}
                "directory-advanced" => announce_document_for_test(&state, STUDIO, "execution-target-racing-document").await,
                "session-revoked" => {
                    let capability = SessionCapability::parse(&token).expect("session capability");
                    let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("session");
                    state.directory.revoke_auth_session(&session.id, "target-fence", None, "target-fence").await.expect("revoke session").expect("revoked row");
                }
                "cancelled" => task.abort(),
                _ => panic!("unknown target fence mutation"),
            }
            gate.release.add_permits(1);
            let outcome = match task.await {
                Ok(Ok((fields, assets))) => {
                    assert_eq!(fields.scope, scope);
                    assert_eq!(&*assets.component, TEST_EXECUTION_TARGET_COMPONENT_BYTES);
                    "selected"
                }
                Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Stale => "stale",
                Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Denied => "denied",
                Err(error) if error.is_cancelled() => "cancelled",
                _ => panic!("unexpected execution-target fence outcome"),
            };
            assert_eq!(outcome, vector["expected"].as_str().expect("expected fence outcome"));
            assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "asset reads never mint or consume plan receipts");
        }
    }

    #[tokio::test]
    async fn document_open_and_execution_target_refuse_descriptor_or_index_without_genesis() {
        for indexed in [false, true] {
            let mut state = test_state().await;
            let token = seed_author_token(&state).await;
            let document_id = artifact_document_id_for_test(if indexed { "indexed-without-genesis" } else { "descriptor-without-genesis" });
            let scope = DocumentScope::new(STUDIO, &document_id);
            announce_document_for_test(&state, STUDIO, &document_id).await;
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor-only document");
            if indexed {
                let session = state
                    .directory
                    .authenticate_session(&SessionCapability::parse(&token).expect("indexed author capability"))
                    .await
                    .expect("indexed author session read")
                    .expect("indexed author session");
                let event = semio_hub::directory::NewDirectoryEvent {
                    hlc: semio_hub::directory::HubClock::new().tick(),
                    actor: DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#indexed-without-genesis", session.user_id) },
                    space_id: Some(scope.space_id.clone()),
                    user_id: Some(session.user_id),
                    body: os_directory::DirectoryEventBody::DocumentIndexed {
                        scope: scope.clone(),
                        descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
                        entry: os_directory::DocumentIndexEntryV1 {
                            name: "Indexed without genesis".into(),
                            dialect: directory::os_io::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
                        },
                    },
                };
                state.directory.append_decided_events(&[event]).await.expect("corrupt indexed-only fixture");
            }
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let mut headers = bearer_headers(&token);
            headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
            let body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: scope.clone(),
                requested_surface_id: Some("surface.test.editor".into()),
                client_instance_id: "client:no-genesis".into(),
            }));
            let authorization = format!("Bearer {token}");
            let raw_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
            let addr = spawn_server(state.clone()).await;
            for path in [
                format!("/spaces/{}/documents/{}/open-plan", scope.space_id, scope.document_id),
                format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id),
            ] {
                let denied = raw_http_request(addr, "POST", &path, &raw_headers, &body).await;
                assert_eq!(denied.status, 404, "{path} must refuse a document without committed genesis");
                let error: serde_json::Value = serde_json::from_slice(&denied.body).expect("closed checkpoint error");
                assert_eq!(error["code"], "not-found");
                assert!(!denied.body.windows(TEST_EXECUTION_TARGET_COMPONENT_BYTES.len()).any(|window| window == TEST_EXECUTION_TARGET_COMPONENT_BYTES));
            }
            assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "pre-genesis route refusal cannot mint a plan");
            if !indexed {
                let mut no_catalog = state.clone();
                no_catalog.openable_catalog = None;
                let no_catalog_addr = spawn_server(no_catalog).await;
                let denied = raw_http_request(
                    no_catalog_addr,
                    "POST",
                    &format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id),
                    &raw_headers,
                    &body,
                )
                .await;
                assert_eq!(denied.status, 404, "checkpoint absence must be decided before catalog availability");
                assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("closed no-catalog checkpoint error")["code"], "not-found");
            }
            let Err((plan_status, DirectoryJson(plan_error))) = issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), body.clone()).await else {
                panic!("descriptor/index without genesis minted an open plan");
            };
            assert_eq!((plan_status, plan_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
            let Err((target_status, DirectoryJson(target_error))) = document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state, body).await else {
                panic!("descriptor/index without genesis minted an execution target");
            };
            assert_eq!((target_status, target_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
        }
        eprintln!("[DEBUG] required-checkpoint-open: descriptor-only and indexed-without-genesis refused before plan or lease");
    }

    #[tokio::test]
    async fn document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable() {
        let mut state = test_state().await;
        let document_id = artifact_document_id_for_test("open-plan-issue");
        let token = seed_author_token(&state).await;
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, &document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
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
        unavailable.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false, false));
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
            authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
            authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
            state.document_open_plans.issue_with_capability(authority, capacity_now, capacity_now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("fill plan capacity");
        }
        let capacity_response = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:capacity").as_bytes()).await;
        assert_eq!(capacity_response.status, 503);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&capacity_response.body).expect("capacity JSON")["code"], "deadline-exceeded");

        let mut cancelled = test_state().await;
        let cancelled_token = seed_author_token(&cancelled).await;
        let cancelled_document_id = artifact_document_id_for_test("open-plan-cancelled");
        let (cancelled_descriptor, _) = publish_openable_document_for_test(&cancelled, &cancelled_token, STUDIO, &cancelled_document_id).await;
        let cancelled_scope = DocumentScope::new(STUDIO, cancelled_document_id);
        cancelled.openable_catalog = Some(document_open_catalog_for_descriptor(&cancelled_descriptor));
        cancelled.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
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
        let document_id = artifact_document_id_for_test("open-plan-consume");
        let token = seed_author_token(&state).await;
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, &document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));

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
        publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
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
        Arc::make_mut(hostile_checkpoint.document_plan.as_mut().expect("checkpoint authority")).checkpoint.descriptor_digest_v1 = "00".repeat(32);
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
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
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
        let session_intent = AdminIntentV1::RevokeUserSessions { request_id: "request:open-plan-session-revoke".into(), user_id: session_record.user_id, reason_code: "test-revoke".into() };
        let session_digest = admin_intent_digest(&session_intent);
        let mut session_revoke_authority = None;
        let session_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-session-revoke", &session_digest, session_intent, None, &mut session_revoke_authority).await;
        drop(session_revoke_authority);
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
        let share_intent = AdminIntentV1::RevokeDocumentShare {
            request_id: "request:open-plan-share-revoke".into(),
            scope: issued_share.record.scope,
            share_id: issued_share.record.id,
            reason_code: "test-revoke".into(),
        };
        let share_digest = admin_intent_digest(&share_intent);
        let mut share_revoke_authority = None;
        let share_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-share-revoke", &share_digest, share_intent, None, &mut share_revoke_authority).await;
        drop(share_revoke_authority);
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
        assert_eq!(
            pending.bindings(),
            vec![
                SocketBindingKeyV1::User("user-a".into()),
                SocketBindingKeyV1::Session("session-a".into()),
                SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() },
                SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() },
            ]
        );
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
                        if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { ref descriptor }
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

    async fn stop_recovery_server(state: HubState, shutdown: tokio::sync::oneshot::Sender<()>, task: tokio::task::JoinHandle<()>) {
        shutdown.send(()).expect("shutdown signal");
        tokio::time::timeout(std::time::Duration::from_secs(5), task).await.expect("server shutdown deadline").expect("server shutdown join");
        state.admin_operation_tasks.shutdown().await;
        let directory = Arc::downgrade(&state.directory);
        let database = state.db.clone();
        drop(state);
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while directory.strong_count() != 0 || Arc::strong_count(&database) != 1 {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("all socket and directory owners retired before reopen");
        let mut database = Arc::try_unwrap(database).unwrap_or_else(|_| panic!("database owner remained shared"));
        let control = db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
        tokio::time::timeout(std::time::Duration::from_secs(5), database.shutdown(&control)).await.expect("database shutdown deadline").expect("database shutdown");
    }

    fn assert_recovery_denied(response: RawHttpResponse, expected: &serde_json::Value) {
        assert_eq!(u64::from(response.status), expected["removedStatus"].as_u64().unwrap());
        assert!(response.headers.lines().any(|line| line.eq_ignore_ascii_case("content-type: application/json")));
        let error: DocumentOpenPlanErrorV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("strict bounded error, not selected asset bytes");
        assert_eq!(error.schema, "semio.hub.document-open-plan-error/v1");
        assert_eq!(error.code, DocumentOpenPlanErrorCodeV1::Denied);
        assert_eq!(expected["removedCode"], "denied");
    }

    #[test]
    fn admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen() {
        run_socket_test(|| async {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json")).expect("admin recovery fixture");
            let expected = &fixture["expected"];
            let scope = DocumentScope::new(fixture["scope"]["spaceId"].as_str().unwrap(), fixture["scope"]["documentId"].as_str().unwrap());
            let dir = tempdir("admin-presence-target-recovery");
            std::fs::create_dir_all(&dir).expect("recovery fixture directory");
            let path = dir.join("directory.sqlite");
            let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("file directory");
            let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
            let admin_headers = authorize_test_admin(&mut state, "recovery-admin@example.com").await;
            let removed = issue_test_session(&state, "recovery-removed@example.com").await;
            let observer = issue_test_session(&state, "recovery-observer@example.com").await;
            for member in fixture["members"].as_array().unwrap() {
                let email = format!("recovery-{}@example.com", member["id"].as_str().unwrap());
                assert_eq!(member["role"], "author");
                upsert_member_for_test(&state, &scope.space_id, &email, DirectorySpaceRole::Author).await;
            }
            announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
            let descriptor = state.directory.get_document_descriptor(&scope).await.unwrap().unwrap();
            state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
            state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
            let (plan_b, grant_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:removed").await;
            let (plan_c, grant_c) = issue_and_exchange_document_open_plan_for_test(&state, &observer.token, &scope, "client:observer").await;
            assert_eq!(plan_b.surface.surface_id, fixture["surfaceId"].as_str().unwrap());
            assert_eq!(plan_b.surface.surface_id, plan_c.surface.surface_id);
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            let root = format!("/spaces/{}/documents/{}", scope.space_id, scope.document_id);
            let url = format!("ws://{addr}{root}/socket/v1?surface={}", plan_b.surface.surface_id);
            let (mut b, _) = connect_async(socket_request(&url, &grant_b.grant)).await.expect("member document socket");
            let (mut c, _) = connect_async(socket_request(&url, &grant_c.grant)).await.expect("observer document socket");
            b.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
            c.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
            let welcome_b = next_server_frame(&mut b).await;
            assert!(matches!(&welcome_b, ServerFrame::Welcome { .. }), "member welcome: {welcome_b:?}");
            let ServerFrame::Session { actor: actor_b, color: color_b } = next_server_frame(&mut b).await else { panic!("member session") };
            let welcome_c = next_server_frame(&mut c).await;
            assert!(matches!(&welcome_c, ServerFrame::Welcome { .. }), "observer welcome: {welcome_c:?}");
            let ServerFrame::Session { actor: actor_c, .. } = next_server_frame(&mut c).await else { panic!("observer session") };
            let raw = presence_hex_bytes(presence_normalization_fixture()["vectors"][0]["rawPeerHex"].as_str().unwrap());
            b.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.unwrap();
            let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("observer normalized presence") };
            assert_eq!(peers.len(), 1);
            let peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
            assert_eq!(peer.actor, actor_b);
            assert_eq!(peer.user_id.as_deref(), Some(removed.user_id.as_str()));
            assert_eq!(peer.label.as_deref(), Some("recovery-removed@example.com"));
            assert_eq!(peer.role.as_deref(), Some("author"));
            assert_eq!(peer.color, Some(color_b));
            assert_eq!(peer.surface.as_deref(), Some(plan_b.surface.surface_id.as_str()));
            assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Presence { .. }));
            let intent = directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
                schema: "semio.hub.document-open-intent/v1".into(),
                version: 1,
                scope: scope.clone(),
                requested_surface_id: Some(plan_b.surface.surface_id.clone()),
                client_instance_id: "client:recovery-target".into(),
            });
            let authorization_b = format!("Bearer {}", removed.token);
            let authorization_c = format!("Bearer {}", observer.token);
            let headers_b = [("Authorization", authorization_b.as_str()), ("Content-Type", "application/json")];
            let headers_c = [("Authorization", authorization_c.as_str()), ("Content-Type", "application/json")];
            assert_eq!(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_b, intent.as_bytes()).await.status, 200);
            let (_, pending_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:pending-removed").await;
            let remove = directory::os_pack::json::to_json_string(&AdminIntentV1::RemoveSpaceMember { request_id: "request:admin-presence-recovery".into(), space_id: scope.space_id.clone(), user_id: removed.user_id.clone() });
            let admin_authorization = admin_headers.get(axum::http::header::AUTHORIZATION).unwrap().to_str().unwrap();
            let receipt = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", admin_authorization), ("Content-Type", "application/json")], remove.as_bytes()).await;
            assert_eq!(receipt.status, 200);
            let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&receipt.body).unwrap()).unwrap();
            assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
            assert_eq!(u64::from(next_close_without_authority(&mut b).await), expected["removedCloseCode"].as_u64().unwrap());
            let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("removed member withdrawal") };
            assert_eq!(peers.len() as u64, expected["visibleAfterRemoval"].as_u64().unwrap());
            assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
            let exchange = directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan_b.receipt.clone() });
            let denied = raw_http_request(addr, "POST", &format!("{root}/socket-grants"), &headers_b, exchange.as_bytes()).await;
            assert_recovery_denied(denied, expected);
            assert!(connect_async(socket_request(&url, &grant_b.grant)).await.is_err(), "removed member cannot reuse its consumed grant");
            assert!(connect_async(socket_request(&url, &pending_b.grant)).await.is_err(), "removal invalidates a previously unused member grant");
            for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
                let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
                assert_recovery_denied(denied, expected);
            }
            c.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.unwrap();
            let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("observer remains live") };
            assert_eq!(peers.len(), 1);
            let observer_peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
            assert_eq!(observer_peer.actor, actor_c);
            assert_eq!(observer_peer.user_id.as_deref(), Some(observer.user_id.as_str()));
            assert_eq!(observer_peer.role.as_deref(), Some("author"));
            assert_eq!(observer_peer.surface.as_deref(), Some(plan_c.surface.surface_id.as_str()));
            assert_eq!(u64::from(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_c, intent.as_bytes()).await.status), expected["observerStatus"].as_u64().unwrap());
            c.close(None).await.unwrap();
            drop(c);
            drop(b);
            stop_recovery_server(state, shutdown, server).await;
            assert_eq!(expected["sqliteReopen"], true);
            let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("reopen exact file directory");
            let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
            state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
            state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
            assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
            assert!(state.socket_grants.inner.lock().unwrap().records.is_empty());
            assert!(state.document_open_plans.inner.lock().unwrap().records.is_empty());
            for session in [&removed, &observer] {
                assert!(state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().is_some(), "membership removal must not be mistaken for session loss");
            }
            assert_eq!(state.directory.get_role(&scope.space_id, &removed.user_id).await.unwrap(), None);
            assert_eq!(state.directory.get_role(&scope.space_id, &observer.user_id).await.unwrap(), Some(SpaceRole::Author));
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
                let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
                assert_recovery_denied(denied, expected);
                let admitted = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_c, intent.as_bytes()).await;
                assert_eq!(u64::from(admitted.status), expected["observerStatus"].as_u64().unwrap(), "reopened observer {route}");
                match route {
                    "execution-target/component" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES),
                    "execution-target/descriptor" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES),
                    "execution-target/manifest" => {
                        let manifest: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).unwrap();
                        manifest.validate().unwrap();
                        assert_eq!(manifest.scope, scope);
                    }
                    "open-plan" => {
                        let plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).unwrap();
                        plan.validate(now_ms() as u64).unwrap();
                        assert_eq!(plan.scope, scope);
                    }
                    _ => unreachable!(),
                }
            }
            stop_recovery_server(state, shutdown, server).await;
            eprintln!("[DEBUG] admin removal withdrew plan-bound presence, closed only the removed member, and survived exact file-SQLite Hub reopen for all selected target routes");
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
                    if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { ref descriptor }
                        if descriptor.space_id == STUDIO && descriptor.document_id == "scoped-order-document")
            ));
            tokio::time::timeout(std::time::Duration::from_secs(2), removal).await.expect("delivery-wins removal deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut delivery_wins, false).await, 4401, "the one admitted event precedes the terminal membership close");
        });
    }

    #[test]
    fn admin_intent_binding_wire_matrix_is_exact_sorted_and_self_deduplicated() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).unwrap();
        let principal = AdminPrincipalV1 {
            user_id: "admin".into(),
            auth_session_id: "admin-session".into(),
            authorization_generation: 1,
            identity_provider: "fixture".into(),
            identity_subject_digest: [0; 32],
            expires_at_ms: i64::MAX,
            correlation_id: "binding-fixture".into(),
            peer_class: "test",
        };
        for row in fixture["bindings"].as_array().unwrap() {
            let intent: AdminIntentV1 = directory::os_pack::json::from_json_str(row["intentJson"].as_str().unwrap()).expect("actual closed administrator intent wire");
            let keys = admin_intent_bindings(&principal, &intent).map(|bindings| {
                bindings
                    .into_iter()
                    .map(|binding| match binding {
                        SocketBindingKeyV1::User(id) => format!("user:{id}"),
                        SocketBindingKeyV1::Session(id) => format!("session:{id}"),
                        SocketBindingKeyV1::DirectorySpaceAuthority { space_id } => format!("space:{space_id}"),
                        SocketBindingKeyV1::Membership { user_id, space_id } => format!("membership:{user_id}/{space_id}"),
                        SocketBindingKeyV1::Share(id) => format!("share:{id}"),
                        SocketBindingKeyV1::DocumentWrite(_) => panic!("administrator short authority cannot acquire a document writer"),
                    })
                    .collect::<Vec<_>>()
            });
            assert_eq!(serde_json::to_value(&keys).unwrap(), row["keys"], "exact sorted binding union: {}", row["name"]);
            eprintln!("[DEBUG] admin-intent-bindings: {} keys={keys:?}", row["name"]);
        }
    }

    #[test]
    fn admin_short_effects_retain_principal_until_their_actual_side_effect() {
        run_socket_test(|| async {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin short effect fixture");
            let root = tempdir("admin-short-authority");
            std::fs::create_dir_all(&root).expect("physical directory parent");
            let path = root.join("directory.sqlite");
            let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("physical directory");
            let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state_with_directory(root.join("db"), directory, 1024, 256)).await.expect("admin effect state open deadline");
            let physical = rusqlite::Connection::open(&path).expect("independent physical directory reader");
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let email = "admin-short-authority@example.test";
            let _headers = authorize_test_admin(&mut state, email).await;
            let owner = issue_test_session(&state, "admin-short-owner@example.test").await;
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            for (index, row) in fixture["shortActions"].as_array().unwrap().iter().enumerate() {
                let admin = issue_test_session(&state, email).await;
                let target = issue_test_session(&state, &format!("admin-short-target-{index}@example.test")).await;
                let space = create_space_for_test(&state, &owner.user_id, "Short Authority", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
                let scope = DocumentScope::new(space, format!("admin-short-document-{index}"));
                announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
                let request_id = format!("request:admin-short-{index}");
                let sync_id = format!("sync:admin-short-{index}");
                let kick = Arc::new(tokio::sync::Notify::new());
                state.session_kicks.insert(sync_id.clone(), kick.clone());
                let action = row["action"].as_str().unwrap();
                let existing_share = if action == "revoke-share" { Some(state.directory.issue_share_token(&scope, 600, "fixture:existing-share").await.expect("existing target share")) } else { None };
                let intent = match action {
                    "issue-share" => AdminIntentV1::IssueDocumentShare { request_id, scope: scope.clone(), ttl_secs: 600 },
                    "revoke-share" => AdminIntentV1::RevokeDocumentShare { request_id, scope: scope.clone(), share_id: existing_share.as_ref().unwrap().record.id.clone(), reason_code: "test-revoke".into() },
                    "revoke-user" => AdminIntentV1::RevokeUserSessions { request_id, user_id: target.user_id.clone(), reason_code: "test-revoke".into() },
                    "kick" => AdminIntentV1::KickConnection { request_id, sync_session_id: sync_id.clone(), reason_code: "test-kick".into() },
                    other => panic!("unknown short action {other}"),
                };
                let principal = authenticate_admin_principal(&state, &bearer_headers(&admin.token), None).await.expect("short action principal");
                let bindings = admin_intent_bindings(&principal, &intent).expect("short effect owns an authority union");
                assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "closed short action authority keys");
                let action_first = row["first"] == "action";
                *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), action_first));
                let command = tokio::spawn({
                    let authorization = format!("Bearer {}", admin.token);
                    let body = directory::os_pack::json::to_json_string(&intent);
                    async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("short action pause deadline").expect("short action pause").forget();
                let mut revoke = tokio::spawn({
                    let state = state.clone();
                    let token = admin.token.clone();
                    async move { delete_session_me(bearer_headers(&token), State(state)).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("short action revoke attempt deadline").expect("short action revoke attempt").forget();
                if action_first {
                    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted {} must own the principal authority", row["name"]);
                    for binding in bindings {
                        assert!(state.socket_binding_gates.gate(binding).try_lock_owned().is_err(), "every short effect key stays owned through its physical side effect");
                    }
                } else {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning short action revoke deadline").expect("winning short action revoke"), StatusCode::NO_CONTENT);
                }
                *gate.directory_command_pause_user.lock().unwrap() = None;
                gate.directory_command_release.add_permits(1);
                let response = tokio::time::timeout(std::time::Duration::from_secs(5), command).await.expect("short action completion deadline").expect("short action task");
                assert_eq!(response.status, 200);
                let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("short action receipt");
                assert_eq!(receipt.state, if row["effect"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled }, "principal authority decides {}", row["name"]);
                if action_first {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing short action revoke deadline").expect("trailing short action revoke"), StatusCode::NO_CONTENT);
                }
                let effect: i64 = match action {
                    "issue-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
                    "revoke-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE id = ?1 AND revoked_at IS NOT NULL", [existing_share.as_ref().unwrap().record.id.as_str()], |row| row.get(0)).unwrap(),
                    "revoke-user" => physical.query_row("SELECT count(*) FROM hub_auth_session WHERE user_id = ?1 AND revoked_at IS NOT NULL", [target.user_id.as_str()], |row| row.get(0)).unwrap(),
                    "kick" => i64::from(tokio::time::timeout(std::time::Duration::from_millis(30), kick.notified()).await.is_ok()),
                    _ => unreachable!(),
                };
                assert_eq!(effect, i64::from(row["effect"].as_bool().unwrap()), "exact physical effect: {}", row["name"]);
                let audit_count: i64 = physical.query_row("SELECT count(*) FROM hub_auth_audit WHERE correlation_id = ?1", [&receipt.correlation_id], |row| row.get(0)).unwrap();
                assert_eq!(audit_count, if action == "kick" { 0 } else { effect }, "no auth side-effect audit under revoked authority");
                let secret = receipt.result.as_ref().and_then(|result| result.share_token.as_ref());
                assert_eq!(secret.is_some(), row["secret"].as_bool().unwrap(), "plaintext share result follows the exact admitted effect");
                if let Some(secret) = secret {
                    let capability = semio_hub::directory::ShareCapability::parse(secret).expect("minted share capability");
                    assert!(state.directory.authenticate_share(&scope, &capability).await.expect("minted share authentication"));
                }
                state.session_kicks.remove(&sync_id);
                eprintln!("[DEBUG] admin-short-authority: {} state={:?} physical-effects={effect} auth-audits={audit_count} secret={}", row["name"], receipt.state, secret.is_some());
            }
            drop(physical);
            stop_recovery_server(state, shutdown, server).await;
        });
    }

    #[test]
    fn admin_directory_commands_hold_exact_principal_without_confusing_space_role() {
        run_socket_test(|| async {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin authority fixture");
            let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("admin authority state open deadline");
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let email = "directory-admin-authority@example.test";
            let _headers = authorize_test_admin(&mut state, email).await;
            let owner = issue_test_session(&state, "directory-admin-target-owner@example.test").await;
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
                let admin = issue_test_session(&state, email).await;
                let headers = bearer_headers(&admin.token);
                let principal = authenticate_admin_principal(&state, &headers, None).await.expect("configured administrator");
                let space = create_space_for_test(&state, &owner.user_id, "Admin Target", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
                upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Author).await;
                let request_id = format!("request:admin-directory-authority-{index}");
                let name = format!("Admin Mutation {index}");
                let create = row["command"] == "create";
                let command_first = row["first"] == "command";
                let intent = if create {
                    AdminIntentV1::CreateSpace { request_id: request_id.clone(), name: name.clone(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }
                } else {
                    AdminIntentV1::RenameSpace { request_id: request_id.clone(), space_id: space.clone(), name: name.clone() }
                };
                let target = if create { admin_create_space_id(&request_id) } else { space.clone() };
                let before = state.directory.head_seq().await.expect("admin head before intent");
                *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), command_first));
                let mut command = tokio::spawn({
                    let authorization = format!("Bearer {}", admin.token);
                    let body = directory::os_pack::json::to_json_string(&intent);
                    async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("admin pause deadline").expect("admin pause").forget();
                let mut trailing_revoke = None;
                if row["transition"] == "session" {
                    let mut revoke = tokio::spawn({
                        let state = state.clone();
                        let token = admin.token.clone();
                        async move { delete_session_me(bearer_headers(&token), State(state)).await }
                    });
                    tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("admin revoke attempt deadline").expect("admin revoke attempt").forget();
                    if command_first {
                        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted admin command retains the exact session");
                        assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::User(principal.user_id.clone())).try_lock_owned().is_err(), "admitted admin command also retains user-wide revocation authority");
                        trailing_revoke = Some(revoke);
                    } else {
                        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("admin revoke deadline").expect("admin revoke"), StatusCode::NO_CONTENT);
                    }
                } else {
                    upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Spectator).await;
                    assert_eq!(state.directory.get_role(&space, &admin.user_id).await.unwrap(), Some(SpaceRole::Spectator));
                }
                *gate.directory_command_pause_user.lock().unwrap() = None;
                gate.directory_command_release.add_permits(1);
                let response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut command).await.expect("admin intent completion deadline").expect("admin intent task");
                assert_eq!(response.status, 200);
                let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("admin authority receipt");
                let expected = if row["mutated"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled };
                assert_eq!(receipt.state, expected, "exact principal, not ordinary role, decides {}", row["name"]);
                if let Some(revoke) = trailing_revoke {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing admin revoke deadline").expect("trailing admin revoke"), StatusCode::NO_CONTENT);
                }
                let events = state.directory.events_since(before, 50).await.expect("admin authority durable events");
                let own_events: Vec<_> = events.iter().filter(|event| event.actor == principal.event_actor()).collect();
                assert_eq!(own_events.len(), if row["mutated"] == true { if create { 2 } else { 1 } } else { 0 }, "no mutation escapes a revoked principal");
                let persisted = state.directory.get_space(&target).await.expect("admin target projection");
                assert_eq!(persisted.as_ref().is_some_and(|space| space.name == name), row["mutated"].as_bool().unwrap());
                if row["mutated"] == false {
                    assert!(receipt.event_seq_first.is_none() && receipt.event_seq_last.is_none());
                }
                eprintln!("[DEBUG] admin-directory-authority: {} state={:?} events={}", row["name"], receipt.state, own_events.len());
            }

            let self_session = issue_test_session(&state, email).await;
            let authorization = format!("Bearer {}", self_session.token);
            let body = directory::os_pack::json::to_json_string(&AdminIntentV1::RevokeUserSessions { request_id: "request:admin-directory-self-revoke".into(), user_id: self_session.user_id.clone(), reason_code: "test-self-revoke".into() });
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()))
                .await
                .expect("self-revocation cannot nest an administrator User fence");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("self-revocation receipt");
            assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
            let capability = SessionCapability::parse(&self_session.token).unwrap();
            assert!(state.directory.authenticate_session(&capability).await.unwrap().is_none());
            eprintln!("[DEBUG] admin-directory-authority: self-user-revocation completed without nested User ownership");
            stop_recovery_server(state, shutdown, server).await;
        });
    }

    #[test]
    fn directory_global_message_bindings_decode_wire_without_indexing_unrelated_memberships() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
        let ledger = SocketGrantLedgerV1::default();
        let capability = SocketGrantCapability::mint().expect("global capability");
        let audience = SocketAudienceV1::Directory { auth_session_id: "session".into(), authorization_generation: 1 };
        let subject = SocketSubjectV1::Session { session_id: "session".into(), user_id: "recipient".into(), authorization_generation: 1, role: None, expires_at_ms: 10_000 };
        ledger.issue(&capability, audience.clone(), "hub.v1.global".into(), subject, 1, 9_000).expect("global issue");
        let pending = ledger.pending(&capability, &audience, 2).expect("pending global");
        let record = ledger.consume(&pending, 3).expect("consumed global");
        let (live_id, _) = ledger.register_live(&record).expect("global lease");
        let principal_bindings = vec![SocketBindingKeyV1::User("recipient".into()), SocketBindingKeyV1::Session("session".into())];
        for row in fixture["messages"].as_array().unwrap() {
            let message: DirectoryStreamMessage = directory::os_pack::json::from_json_str(row["messageJson"].as_str().unwrap()).unwrap_or_else(|error| panic!("wire message {}: {error:?}", row["name"]));
            assert_eq!(directory_stream_message_space(&message), row["spaceId"].as_str(), "wire-derived space");
            let bindings = directory_message_bindings(&record, &message);
            assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "complete transient union");
            let mut expected = principal_bindings.clone();
            if let Some(space_id) = row["spaceId"].as_str() {
                expected.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
                expected.push(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: space_id.to_owned() });
            }
            assert_eq!(bindings, expected, "sorted exact principal and recipient membership union");
            assert_eq!(record.bindings(), principal_bindings, "transient keys never become permanent global indices");
            let mut scoped = record.clone();
            scoped.audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("scoped", "doc"));
            assert_eq!(directory_message_bindings(&scoped, &message), scoped.bindings(), "scoped audiences do not borrow unrelated scopes");
        }
        ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: "space-a".into() });
        ledger.invalidate_binding(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() });
        assert!(ledger.is_live(&record, &live_id), "space A revocation cannot invalidate a global lease for B");
        ledger.invalidate_binding(SocketBindingKeyV1::Session("session".into()));
        assert!(!ledger.is_live(&record, &live_id), "principal revocation remains terminal");
        eprintln!("[DEBUG] global-directory-message-bindings: six real wire kinds, transient union, scoped isolation, global indices, principal invalidation");
    }

    #[test]
    fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
        run_socket_test(|| async {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
            let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("global directory state open deadline");
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
                let owner_email = format!("global-owner-{index}@example.test");
                let recipient_email = format!("global-recipient-{index}@example.test");
                let owner = issue_test_session(&state, &owner_email).await;
                let recipient = issue_test_session(&state, &recipient_email).await;
                let space_a = create_space_for_test(&state, &owner.user_id, "A", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
                let space_b = create_space_for_test(&state, &owner.user_id, "B", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
                for space in [&space_a, &space_b] {
                    upsert_member_for_test(&state, space, &recipient_email, DirectorySpaceRole::Spectator).await;
                }
                let receipt = issue_directory_socket_grant(bearer_headers(&recipient.token), State(state.clone())).await.expect("global grant").0;
                let since = state.directory.head_seq().await.expect("global directory head");
                let url = format!("ws://{addr}/directory/socket/v1?since={since}");
                let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("global directory socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("global socket hello");
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_directory_admitted.acquire()).await.expect("global admission deadline").expect("global admission").forget();
                gate.socket_directory_release.add_permits(1);
                let delivery_first = row["first"] == "send";
                *gate.socket_global_send_pause.lock().expect("global pause lock") = Some((recipient.user_id.clone(), if delivery_first { 2 } else { 1 }));
                let event_a = state
                    .directory_service
                    .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_a.clone(), name: format!("A-{index}") })
                    .await
                    .expect("durable A event")
                    .0
                    .into_iter()
                    .next()
                    .expect("A event");
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_global_send_admitted.acquire()).await.expect("message pause deadline").expect("message pause").forget();
                let membership = row["revocation"] == "membership";
                let mut revoke = tokio::spawn({
                    let state = state.clone();
                    let token = if membership { owner.token.clone() } else { recipient.token.clone() };
                    let command = DirectoryCommand::RemoveMember { space_id: space_a.clone(), user_id: recipient.user_id.clone() };
                    async move { if membership { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await.status } else { delete_session_me(bearer_headers(&token), State(state)).await.as_u16() } }
                });
                let attempted = if membership { &gate.directory_command_attempted } else { &gate.socket_session_revoke_attempted };
                tokio::time::timeout(std::time::Duration::from_secs(5), attempted.acquire()).await.expect("revocation fence attempt deadline").expect("revocation fence attempt").forget();
                if delivery_first {
                    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted delivery must own the same authority keys as revocation: {}", row["name"]);
                } else {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning revocation deadline").expect("winning revocation"), if membership { 202 } else { 204 });
                }
                *gate.socket_global_send_pause.lock().expect("clear global pause") = None;
                gate.socket_global_send_release.add_permits(1);
                if row["a"] == true {
                    assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event == event_a), "one exact A event wins before revocation");
                }
                if delivery_first {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing revocation deadline").expect("trailing revocation"), if membership { 202 } else { 204 });
                }
                if row["b"] == true {
                    let event_b = state
                        .directory_service
                        .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_b.clone(), name: format!("B-{index}") })
                        .await
                        .expect("durable B event")
                        .0
                        .into_iter()
                        .next()
                        .expect("B event");
                    assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event == event_b), "removed A membership neither leaks A nor closes unrelated B");
                    socket.close(None).await.expect("close unaffected global socket");
                } else {
                    assert_eq!(u64::from(next_close_code(&mut socket, false).await), row["close"].as_u64().unwrap(), "revoked principal closes without a later message");
                }
                eprintln!("[DEBUG] global-directory-message-authority: {} A={} B={} close={}", row["name"], row["a"], row["b"], row["close"]);
            }
            stop_recovery_server(state, shutdown, server).await;
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
        assert_eq!(socket_directory_membership_visibility(&state, &record, &DirectoryStreamMessage::Event { event }).await, SocketBindingValidityV1::Unauthorized);
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

    //#region 🏛️SpaceAdministration
    /// 🏛️ An author receives both bounded windows, a canonical receipt over the exact response
    /// bytes, and the server's own capability flags — and no credential column anywhere.
    #[tokio::test]
    async fn space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt() {
        let state = test_state().await;
        let author = issue_test_session(&state, "administration-author@example.invalid").await;
        let space = create_space_for_test(&state, &author.user_id, "Administered", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "administration-guest@example.invalid", DirectorySpaceRole::Spectator).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
            .await
            .expect("author invite fixture");
        let addr = spawn_server(state).await;
        let authorization = format!("Bearer {}", author.token);
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(response.status, 200);
        let canonical = std::str::from_utf8(&response.body).expect("administration page UTF-8").to_string();
        assert!(canonical.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES);
        let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical author page");
        assert_eq!(page.space_id(), space);
        let DirectorySpaceAdministrationPageV1::Author { members, invites, capabilities, .. } = &page else { panic!("author projection") };
        assert_eq!(members.rows.len(), 2);
        assert!(members.rows.windows(2).all(|pair| pair[0].user_id < pair[1].user_id), "member rows are keyset-ordered");
        assert!(members.rows.iter().any(|row| row.owner), "the owner row is marked so removal can be disabled before dispatch");
        assert_eq!(invites.rows.len(), 1);
        assert!(capabilities.remove_member && capabilities.create_invite);
        for secret in ["selector", "secretDigest", "passwordHash", "ssoSubject", "ssoProvider", "inviteToken"] {
            assert!(!canonical.contains(secret), "administration page leaked {secret}");
        }
    }

    /// 🛂️ A spectator receives the member shape only: invites and capability flags are structurally
    /// absent, so no renderer can mis-gate them into existence.
    #[tokio::test]
    async fn space_administration_page_v1_route_denies_a_spectator_the_author_windows() {
        let state = test_state().await;
        let author = issue_test_session(&state, "administration-owner@example.invalid").await;
        let spectator = issue_test_session(&state, "administration-spectator@example.invalid").await;
        let outsider = issue_test_session(&state, "administration-outsider@example.invalid").await;
        let space = create_space_for_test(&state, &author.user_id, "Spectated", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "administration-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
        state
            .directory_service
            .execute(
                DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-spectator-law", author.user_id) },
                DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 },
            )
            .await
            .expect("author invite fixture");
        let addr = spawn_server(state).await;
        let spectator_authorization = format!("Bearer {}", spectator.token);
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", spectator_authorization.as_str())]).await;
        assert_eq!(response.status, 200);
        let canonical = std::str::from_utf8(&response.body).expect("spectator page UTF-8").to_string();
        let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical member page");
        assert!(matches!(page, DirectorySpaceAdministrationPageV1::Member { .. }));
        assert!(page.capabilities().is_none());
        assert!(!canonical.contains("\"invites\"") && !canonical.contains("\"capabilities\""));
        let outsider_authorization = format!("Bearer {}", outsider.token);
        assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", outsider_authorization.as_str())]).await.status, 404);
        assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[]).await.status, 404);
    }

    /// 🧯️ A membership removal takes effect on the very next page read: the response is a denial with
    /// no page bytes at all, so no member or invite row can leak past the revocation.
    #[tokio::test]
    async fn space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows() {
        let state = test_state().await;
        let author = issue_test_session(&state, "administration-remover@example.invalid").await;
        let removed = issue_test_session(&state, "administration-removed@example.invalid").await;
        let space = create_space_for_test(&state, &author.user_id, "Revoked", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "administration-removed@example.invalid", DirectorySpaceRole::Spectator).await;
        let service = state.directory_service.clone();
        let addr = spawn_server(state).await;
        let authorization = format!("Bearer {}", removed.token);
        assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
        service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-remove-law", author.user_id) }, DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: removed.user_id.clone() })
            .await
            .expect("member removal");
        let denied = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(denied.status, 404, "a removed member cannot even enumerate the private space");
        assert!(!String::from_utf8_lossy(&denied.body).contains("administration-remover@example.invalid"));
    }

    /// 🛡️ The query grammar is exact and every cursor is MAC-bound: a foreign, tampered, or
    /// differently shaped cursor is a 400 with no page bytes, never a page for another identity.
    #[tokio::test]
    async fn space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor() {
        let state = test_state().await;
        let author = issue_test_session(&state, "administration-cursor@example.invalid").await;
        let space = create_space_for_test(&state, &author.user_id, "Cursored", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state).await;
        let authorization = format!("Bearer {}", author.token);
        for query in ["?cursor=", "?cursor=a&cursor=b", "?cursor=%6d", "?cursor=m+1", "?section=members", "?cursor=m.7573.zz"] {
            let response = raw_http_get(addr, &format!("/directory/spaces/{space}{query}"), &[("Authorization", authorization.as_str())]).await;
            assert_eq!(response.status, 400, "query {query} must be refused before any read");
            assert!(response.body.is_empty() || !String::from_utf8_lossy(&response.body).contains("receiptSha256"));
        }
        let forged = format!("m.{}.{}", os_directory::hex_lower(b"user-forged"), "ab".repeat(32));
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}?cursor={forged}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(response.status, 400, "a cursor with a forged MAC is refused");
        assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
    }
    //#endregion 🏛️SpaceAdministration

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
            assert_eq!(anonymous["schema"], DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA);
            assert_eq!(anonymous["space"]["visibility"], "public");
            assert_eq!(anonymous["documents"]["rows"][0]["documentId"], "catalog-document");
            assert!(anonymous["documents"]["rows"][0].get("descriptor").is_none());
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
            assert!(member.get("invites").is_none(), "a member page structurally omits invites");
            assert!(member.get("capabilities").is_none(), "a member page structurally omits capability flags");
            assert_eq!(member["documents"]["rows"][0]["headSeq"], 0);

            let author_authorization = format!("Bearer {}", author.token);
            let authored = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await;
            assert_eq!(authored.status, 200);
            let authored: serde_json::Value = serde_json::from_slice(&authored.body).expect("author detail");
            assert_eq!(authored["access"], "author");
            assert_eq!(authored["space"]["role"], "author");
            assert_eq!(authored["invites"]["rows"].as_array().map(Vec::len), Some(1));
            assert_eq!(authored["capabilities"]["removeMember"], true);
            let authored_bytes = std::str::from_utf8(&raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await.body).expect("author page UTF-8").to_string();
            assert!(!authored_bytes.contains("selector") && !authored_bytes.contains("secretDigest") && !authored_bytes.contains("passwordHash"));
            assert_eq!(DirectorySpaceAdministrationPageV1::parse_canonical_json(&authored_bytes).map(|page| page.space_id().to_string()), Ok(public_space.clone()));

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
    async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json")).expect("retained short administrator fixture");
        assert_eq!(fixture["cases"].as_array().expect("retained cases").len(), 15);
        let root = tempdir("retained-short-admin");
        std::fs::create_dir_all(&root).expect("retained administrator root");
        let path = root.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().expect("retained directory path")).await.expect("retained directory");
        let mut state = test_state_with_directory(root.join("db"), directory, 1024, 256).await;
        let physical = rusqlite::Connection::open(&path).expect("retained administrator physical reader");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "retained-short-admin@example.test";
        let _ = authorize_test_admin(&mut state, email).await;
        let admin = issue_test_session(&state, email).await;
        let owner = issue_test_session(&state, "retained-short-owner@example.test").await;
        let space = create_space_for_test(&state, &owner.user_id, "Retained Short", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let scope = DocumentScope::new(&space, "retained-short-document");
        announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let request_id = "request:retained-short-drop";
        let intent = AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 600 };
        let body = directory::os_pack::json::to_json_string(&intent);
        *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), true));
        let dropped = tokio::spawn({
            let authorization = format!("Bearer {}", admin.token);
            let body = body.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("dropped request admission deadline").expect("dropped request admitted").forget();
        dropped.abort();
        let _ = dropped.await;
        *gate.directory_command_pause_user.lock().unwrap() = None;
        gate.directory_command_release.add_permits(1);
        let mut rows = Vec::new();
        for _ in 0..256 {
            rows = state.directory.admin_operation_audit_for_request(request_id).await.expect("dropped request audit");
            if rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(rows.len(), 2, "request cancellation cannot cancel its retained operation");
        assert_eq!(rows[1].fact.phase, "succeeded");
        assert_eq!(rows[1].fact.outcome_code, "share-issued");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(), 1);
        let authorization = format!("Bearer {}", admin.token);
        let retry = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await;
        assert_eq!(retry.status, 200);
        let retry_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&retry.body).unwrap()).expect("retry receipt");
        assert_eq!(retry_receipt.state, AdminIntentStateV1::Succeeded);
        assert!(retry_receipt.result.is_none(), "a lost one-shot share token is never stored or replayed");
        assert_eq!(
            physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
            1,
            "retry cannot execute the side effect twice"
        );
        let collision = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 601 });
        assert_eq!(raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], collision.as_bytes()).await.status, 409);

        let cancelled_scope = DocumentScope::new(&space, "retained-short-cancelled");
        announce_document_for_test(&state, &cancelled_scope.space_id, &cancelled_scope.document_id).await;
        let cancelled_request = "request:retained-short-cancelled";
        let cancelled_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: cancelled_request.into(), scope: cancelled_scope.clone(), ttl_secs: 600 });
        *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), false));
        let cancelled = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], cancelled_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("pre-effect cancellation admission deadline").expect("pre-effect cancellation admitted").forget();
        let accepted = state.directory.admin_operation_audit_for_request(cancelled_request).await.expect("cancelled acceptance");
        let operation_id = accepted.first().expect("cancelled accepted row").fact.operation_id.clone();
        cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("cancel retained operation");
        *gate.directory_command_pause_user.lock().unwrap() = None;
        gate.directory_command_release.add_permits(1);
        let cancelled = cancelled.await.expect("cancelled request task");
        let cancelled_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&cancelled.body).unwrap()).expect("cancelled receipt");
        assert_eq!(cancelled_receipt.state, AdminIntentStateV1::Cancelled);
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![cancelled_scope.space_id, cancelled_scope.document_id], |row| row.get(0)).unwrap(), 0);

        let admitted_scope = DocumentScope::new(&space, "retained-short-admitted");
        announce_document_for_test(&state, &admitted_scope.space_id, &admitted_scope.document_id).await;
        let admitted_request = "request:retained-short-admitted";
        let admitted_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: admitted_request.into(), scope: admitted_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let admitted = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], admitted_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("admitted effect deadline").expect("effect admitted").forget();
        let accepted = state.directory.admin_operation_audit_for_request(admitted_request).await.expect("admitted acceptance");
        let operation_id = accepted.first().expect("admitted accepted row").fact.operation_id.clone();
        cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("late cancellation request");
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let admitted = admitted.await.expect("admitted request task");
        let admitted_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).expect("admitted receipt");
        assert_eq!(admitted_receipt.state, AdminIntentStateV1::Succeeded, "cancellation after effect admission cannot invent rollback");
        assert!(admitted_receipt.result.as_ref().and_then(|result| result.share_token.as_ref()).is_some());
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![admitted_scope.space_id, admitted_scope.document_id], |row| row.get(0)).unwrap(), 1);

        let fenced_scope = DocumentScope::new(&space, "retained-short-admitted-deadline");
        announce_document_for_test(&state, &fenced_scope.space_id, &fenced_scope.document_id).await;
        let first_request = "request:retained-short-admitted-deadline-first";
        let second_request = "request:retained-short-admitted-deadline-second";
        let first_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: first_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
        let second_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: second_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let mut first = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], first_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("first admitted writer deadline").expect("first writer admitted").forget();
        let mut second = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], second_body.as_bytes()).await }
        });
        for _ in 0..256 {
            if state.directory.admin_operation_audit_for_request(second_request).await.expect("competing acceptance read").len() == 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), &mut second).await.is_err(), "competing same-scope writer cannot pass retained authority");
        let first_response = tokio::time::timeout(ADMIN_OPERATION_DEADLINE + std::time::Duration::from_secs(2), &mut first).await.expect("first HTTP deadline response").expect("first HTTP task");
        assert_eq!(first_response.status, 503, "the HTTP waiter expires without cancelling its admitted writer");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 0);
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let second_response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut second).await.expect("competing writer completion deadline").expect("competing HTTP task");
        assert_eq!(second_response.status, 200);
        let mut first_rows = Vec::new();
        for _ in 0..256 {
            first_rows = state.directory.admin_operation_audit_for_request(first_request).await.expect("first admitted writer audit");
            if first_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        let second_rows = state.directory.admin_operation_audit_for_request(second_request).await.expect("second admitted writer audit");
        assert_eq!(first_rows.len(), 2);
        assert_eq!(second_rows.len(), 2);
        assert_eq!(first_rows[1].fact.phase, "succeeded");
        assert_eq!(second_rows[1].fact.phase, "succeeded");
        assert!(first_rows[1].sequence < second_rows[1].sequence, "the retained first writer terminal precedes its blocked successor");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 2);
        assert_eq!(state.admin_operation_tasks.task_count(), 0);
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    }

    #[tokio::test]
    async fn retained_short_admin_shutdown_drains_before_bounded_abort_and_receipt_reconciliation_is_exact() {
        let cancelled = AdminOperationRuntime {
            deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
            cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
        };
        assert!(cancelled.request_cancel());
        assert!(!cancelled.admit_effect(), "cancellation linearized before admission refuses the effect");
        let admitted = AdminOperationRuntime {
            deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
            cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
        };
        assert!(admitted.admit_effect());
        assert!(!admitted.request_cancel(), "admission linearized before cancellation cannot claim rollback");
        ADMIN_SECRET_WIPE_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
        let invite = "invite.v1.selector.secret".to_string();
        let share = "share.v1.selector.secret".to_string();
        let expected_wipes = invite.len() + share.len();
        drop(AdminIntentSecretResult::Invite(invite));
        drop(AdminIntentSecretResult::Share(share));
        assert_eq!(ADMIN_SECRET_WIPE_BYTES.load(std::sync::atomic::Ordering::SeqCst), expected_wipes);

        let state = test_state().await;
        let principal = AdminPrincipalV1 {
            user_id: "user:admin".into(),
            auth_session_id: "session:admin".into(),
            authorization_generation: 1,
            identity_provider: "test".into(),
            identity_subject_digest: [7; 32],
            expires_at_ms: now_ms() + 60_000,
            correlation_id: "correlation:retained".into(),
            peer_class: "admin-rest",
        };
        let metadata = AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: "space:one".into(), reason_code: None };
        let mut accepted = new_admin_audit_fact(&principal, "request:stale-accepted", &"11".repeat(32), "operation:stale-accepted", &metadata, "accepted", None, "accepted");
        accepted.occurred_at = now_ms() - 60_000;
        state.directory.append_admin_operation_audit(&accepted).await.expect("stale accepted audit");
        let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("stale audit read")).await.expect("stale reconciliation");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].fact.phase, "accepted", "age alone cannot prove cancellation or rollback");
        let unresolved = admin_audit_visibility(admin_audit_receipt(&rows).expect("stale accepted receipt"), false);
        assert_eq!(unresolved.state, AdminIntentStateV1::Indeterminate);
        assert_eq!(unresolved.outcome.code, "admin-effect-outcome-indeterminate");
        assert_eq!(admin_audit_visibility(admin_audit_receipt(&rows).expect("live accepted receipt"), true).state, AdminIntentStateV1::Accepted);

        let owner = issue_test_session(&state, "retained-reconcile-owner@example.test").await;
        let space = create_space_for_test(&state, &owner.user_id, "Receipt Reconciliation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let scope = DocumentScope::new(&space, "receipt-reconciliation-document");
        announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
        let committed_digest = "22".repeat(32);
        let committed_metadata = AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None };
        let mut committed = new_admin_audit_fact(&principal, "request:committed-accepted", &committed_digest, "operation:committed-accepted", &committed_metadata, "accepted", None, "accepted");
        committed.occurred_at = now_ms() - 60_000;
        state.directory.append_admin_operation_audit(&committed).await.expect("committed acceptance audit");
        let effect = NewAdminOperationEffectReceiptV1 {
            operation_id: committed.operation_id.clone(),
            intent_digest: committed.intent_digest.clone(),
            committed_at: now_ms(),
            outcome_code: "share-issued".into(),
        };
        let AdminEffectCommitV1::Applied(issued) = state.directory.issue_share_token_as_with_admin_effect(&scope, 600, Some(&principal.user_id), &principal.correlation_id, &effect).await else {
            panic!("atomic share and effect receipt");
        };
        drop(issued);
        assert!(tokio::time::timeout(std::time::Duration::ZERO, std::future::pending::<()>()).await.is_err(), "effect commit can precede acknowledgement deadline");
        assert!(state.directory.admin_operation_effect_receipt(&committed.operation_id, &"33".repeat(32)).await.expect("mismatched receipt query").is_none());
        let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&committed.request_id).await.expect("committed audit read"))
            .await
            .expect("factual receipt reconciliation");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].fact.phase, "succeeded");
        assert_eq!(rows[1].fact.outcome_code, "share-issued");
        assert_eq!(rows[1].fact.event_seq_first, None);
        assert_eq!(rows[1].fact.event_seq_last, None);

        let owner = AdminOperationTaskOwner::new(std::time::Duration::from_secs(1));
        let runtime = Arc::new(AdminOperationRuntime {
            deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
            cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
        });
        let drained = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(
            owner
                .spawn("operation:drain".into(), runtime.clone(), {
                    let drained = drained.clone();
                    let runtime = runtime.clone();
                    async move {
                        while !runtime.progress().cancel_requested {
                            tokio::task::yield_now().await;
                        }
                        drained.store(true, std::sync::atomic::Ordering::Release);
                    }
                })
                .is_ok()
        );
        owner.shutdown().await;
        assert!(drained.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(owner.task_count(), 0);
        assert!(owner.spawn("operation:after-close".into(), runtime.clone(), async {}).is_err(), "shutdown refuses later operation tasks");

        struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
        impl Drop for DropSignal {
            fn drop(&mut self) {
                if let Some(signal) = self.0.take() {
                    let _ = signal.send(());
                }
            }
        }
        let aborting = AdminOperationTaskOwner::new(std::time::Duration::from_millis(10));
        let abort_runtime = Arc::new(AdminOperationRuntime {
            deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
            completed: std::sync::atomic::AtomicU64::new(0),
            total: std::sync::atomic::AtomicU64::new(0),
            effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
            cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
        });
        let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel();
        assert!(
            aborting
                .spawn("operation:bounded-abort".into(), abort_runtime, async move {
                    let _signal = DropSignal(Some(dropped_tx));
                    std::future::pending::<()>().await;
                })
                .is_ok()
        );
        aborting.shutdown().await;
        tokio::time::timeout(std::time::Duration::from_secs(1), dropped_rx).await.expect("aborted task drop deadline").expect("aborted task dropped");
        assert_eq!(aborting.task_count(), 0);
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
            effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
            cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
        });
        operations.insert(operation_id.into(), runtime);
        let cleanup = AdminOperationCleanup { operations: operations.clone(), operation_id: operation_id.into(), _permit: operation_slots.clone().try_acquire_owned().expect("cleanup slot") };
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
            if operations.get_cloned(operation_id).is_none() {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].fact.phase, "accepted");
        assert_eq!(rows[0].fact.outcome_code, "accepted");
        assert!(operations.get_cloned(operation_id).is_none());
        assert_eq!(operation_slots.available_permits(), 64);
        let later = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "succeeded", None, "late-success");
        assert_eq!(directory.append_admin_operation_audit(&later).await.expect("factual late terminal").fact.phase, "succeeded", "task abortion cannot invent rollback before the factual result is known");
    }

    fn presence_hex_bytes(hex: &str) -> Vec<u8> {
        assert_eq!(hex.len() % 2, 0);
        (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("presence fixture byte")).collect()
    }

    fn presence_normalization_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json")).expect("presence normalization fixture")
    }

    fn test_presence_slot(live: &str, user_id: Option<&str>, now: tokio::time::Instant) -> PresenceLeaseSlot {
        PresenceLeaseSlot {
            socket_live_id: live.into(),
            expires_at: now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS),
            user_id: user_id.map(str::to_string),
            connected_at_ms: 1000,
            label: None,
            role: None,
            document_surface: Some("surface".into()),
            color: 0,
            peer: None,
        }
    }

    async fn presence_test_peer(pack: &[u8]) -> Vec<u8> {
        let fixture = presence_normalization_fixture();
        let mut peer = protocol::decode_presence_peer(&presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"))).await.expect("fixture peer");
        peer.presence_pack = Some(pack.to_vec());
        protocol::encode_presence_peer(&peer).await
    }

    async fn presence_frame_has_pack(frame: ServerFrame, pack: &[u8]) -> bool {
        let ServerFrame::Presence { peers } = frame else { return false };
        peers.len() == 1 && protocol::decode_presence_peer(&peers[0]).await.is_ok_and(|peer| peer.presence_pack.as_deref() == Some(pack))
    }

    #[tokio::test]
    async fn presence_normalization_matches_neutral_authority_and_no_effect_rejections() {
        let state = lag_test_state(1024, 256).await;
        let fixture = presence_normalization_fixture();
        let head = state.directory.head_seq().await.expect("directory head");
        let now = tokio::time::Instant::now();
        for vector in fixture["vectors"].as_array().expect("vectors") {
            let document_id = vector["name"].as_str().expect("vector name");
            let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
            let admitted = &vector["admission"];
            let actor = admitted["actor"].as_str().expect("admitted actor");
            let mut slot = test_presence_slot("live", admitted["userId"].as_str(), now);
            slot.connected_at_ms = admitted["connectedAtMs"].as_i64().expect("admitted time");
            slot.label = admitted["label"].as_str().map(str::to_string);
            slot.role = admitted["role"].as_str().map(str::to_string);
            slot.color = u8::try_from(admitted["color"].as_u64().expect("admitted color")).expect("palette index");
            slot.document_surface = admitted["surface"].as_str().map(str::to_string);
            let deadline = slot.expires_at;
            let mut receiver = state.fanout_for(&key).subscribe();
            assert_eq!(state.install_presence_slot(&key, STUDIO, document_id, actor, slot).await, PresenceLeaseTransition::NoChange);
            let raw = presence_hex_bytes(vector["rawPeerHex"].as_str().expect("raw peer"));
            let transition = state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw.clone(), now + std::time::Duration::from_secs(1)).await;
            let map_key = (key.clone(), actor.to_string());
            if vector["expected"]["accepted"] == true {
                let expected = presence_hex_bytes(vector["expected"]["normalizedPeerHex"].as_str().expect("normalized peer"));
                assert_eq!(transition, PresenceLeaseTransition::Published, "{document_id}");
                assert_eq!(state.presence_snapshot(&key).peers, vec![expected.clone()], "{document_id}");
                assert_eq!(state.presence_snapshot(&key).actors.len(), usize::from(admitted["surface"].is_string()), "non-plan rows cannot mint a directory surface: {document_id}");
                assert!(matches!(receiver.try_recv(), Ok(ServerFrame::Presence { peers }) if peers == vec![expected]));
                assert_eq!(state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw, now + std::time::Duration::from_secs(2)).await, PresenceLeaseTransition::NoChange);
                assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline + std::time::Duration::from_secs(2));
            } else {
                assert_eq!(transition, PresenceLeaseTransition::Rejected, "{document_id}");
                assert!(state.presence_snapshot(&key).peers.is_empty(), "{document_id}");
                assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline, "{document_id}");
            }
            assert!(matches!(receiver.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "no duplicate or rejected publication: {document_id}");
            assert_eq!(vector["expected"]["durableWrites"], 0);
        }
        assert_eq!(state.directory.head_seq().await.expect("directory head"), head);
        eprintln!("[DEBUG] presence normalization: 17 exact neutral admission vectors, unchanged rejected TTL, duplicate suppression, zero directory writes");
    }

    #[test]
    fn presence_normalization_socket_overwrites_identity_and_rejects_without_refresh() {
        run_socket_test(|| async {
            let mut state = lag_test_state(1024, 256).await;
            let clock = Arc::new(TestPresenceClock::new());
            state.presence_clock = Some(clock.clone());
            let token = seed_author_token(&state).await;
            let document_id = "presence-normalized-socket";
            announce_document_for_test(&state, STUDIO, document_id).await;
            let scope = DocumentScope::new(STUDIO, document_id);
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-normalization").await;
            let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("session capability")).await.expect("session lookup").expect("session");
            let user = state.directory.get_user(&session.user_id).await.expect("user lookup").expect("user");
            let started = now_ms();
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1?surface={}", plan.surface.surface_id);
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("admitted socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            let ServerFrame::Session { actor, color } = next_server_frame(&mut socket).await else { panic!("session frame") };
            assert_eq!(actor, receipt.actor_id);
            let fixture = presence_normalization_fixture();
            let raw = presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"));
            let input = protocol::decode_presence_peer(&raw).await.expect("raw peer decode");
            socket.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.expect("forged peer send");
            let ServerFrame::Presence { peers } = next_server_frame(&mut socket).await else { panic!("presence frame") };
            assert_eq!(peers.len(), 1);
            let normalized = protocol::decode_presence_peer(&peers[0]).await.expect("normalized peer");
            assert_eq!(normalized.actor, actor);
            assert_eq!(normalized.user_id, Some(session.user_id));
            assert_eq!(normalized.label, Some(user.display_name));
            assert_eq!(normalized.role.as_deref(), Some("author"));
            assert_eq!(normalized.color, Some(color));
            assert_eq!(normalized.surface, Some(plan.surface.surface_id));
            assert!(normalized.connected_at_ms >= started && normalized.connected_at_ms <= now_ms());
            assert_eq!(normalized.presence_pack, input.presence_pack);
            assert_eq!(normalized.drag_ghost_json, input.drag_ghost_json);
            assert_eq!(normalized.interaction, input.interaction);
            assert_eq!(normalized.views, input.views);
            assert_eq!(normalized.ui, input.ui);
            assert_eq!(protocol::encode_presence_peer(&normalized).await, peers[0]);
            let key = document_scope_key_v1(&scope);
            assert_eq!(state.presence_snapshot(&key).actors[0].surface, normalized.surface.as_deref().expect("plan surface"));
            let deadline = state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at);
            let head = state.directory.head_seq().await.expect("head before rejected input");
            clock.advance_to(5_000);
            for vector in fixture["vectors"]
                .as_array()
                .expect("vectors")
                .iter()
                .filter(|vector| !vector["expected"]["accepted"].as_bool().expect("accepted") && !vector["name"].as_str().expect("name").starts_with("authoritative-") && !vector["name"].as_str().expect("name").starts_with("normalized-"))
            {
                socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("hostile peer")) }, Lane::Preview).await).await.expect("hostile send");
            }
            socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("rejection fence");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 1, .. } if key == "presence-fifo"), "every rejected frame precedes the FIFO marker without an interim roster");
            assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
            assert_eq!(state.presence_snapshot(&key).peers, peers);
            assert_eq!(state.directory.head_seq().await.expect("head after rejected input"), head);
            for (sequence, name, label_bytes) in [(2, "authoritative-field-over-limit", 1025), (3, "normalized-entry-over-limit", 1024)] {
                let vector = fixture["vectors"].as_array().expect("vectors").iter().find(|vector| vector["name"] == name).expect("authority expansion vector");
                let saved_label = state.presence.with_mut(&(key.clone(), actor.clone()), |slot| std::mem::replace(&mut slot.expect("visible slot").label, Some("l".repeat(label_bytes))));
                socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("expansion raw peer")) }, Lane::Preview).await).await.expect("authority-expanded input");
                socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: sequence, payload: vec![] }, Lane::Preview).await).await.expect("expansion fence");
                assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq, .. } if key == "presence-fifo" && seq == sequence), "authority-expanded output is rejected before publication: {name}");
                assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
                assert_eq!(state.presence_snapshot(&key).peers, peers);
                state.presence.with_mut(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").label = saved_label);
            }
            socket.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.expect("identical refresh");
            socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 4, payload: vec![] }, Lane::Preview).await).await.expect("duplicate fence");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 4, .. } if key == "presence-fifo"), "identical normalized peer does not republish");
            assert_eq!(state.presence.with(&(key, actor), |slot| slot.expect("visible slot").expires_at), deadline + std::time::Duration::from_secs(5));
            socket.close(None).await.expect("close socket");
            eprintln!("[DEBUG] admitted plan-backed socket overwrote all identity fields, preserved five ephemerals, and rejected malformed input without visibility, TTL or directory changes");
        });
    }

    #[test]
    fn presence_lease_reconnect_rejects_old_live_refresh_and_close() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
            let token = seed_author_token(&state).await;
            let document_id = "presence-reconnect";
            announce_document_for_test(&state, STUDIO, document_id).await;
            let scope = DocumentScope::new(STUDIO, document_id);
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let observer_token = seed_author_token(&state).await;
            let (plan, first) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-first").await;
            let (_, second) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-second").await;
            let (_, observer_grant) = issue_and_exchange_document_open_plan_for_test(&state, &observer_token, &scope, "client:presence-observer").await;
            let first_record = state.socket_grants.pending(&SocketGrantCapability::parse(&first.grant).expect("first capability"), &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("first pending record");
            assert_eq!(first.actor_id, second.actor_id);
            assert_ne!(first.actor_id, observer_grant.actor_id);
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1?surface={}", plan.surface.surface_id);
            let (mut observer, _) = connect_async(socket_request(&url, &observer_grant.grant)).await.expect("observer socket");
            observer.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("observer hello");
            assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Session { .. }));
            let (mut socket_a, _) = connect_async(socket_request(&url, &first.grant)).await.expect("first socket");
            socket_a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("first hello");
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Session { actor, .. } if actor == first.actor_id));
            socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"old-live").await }, Lane::Preview).await).await.expect("first presence");
            assert!(presence_frame_has_pack(next_server_frame(&mut socket_a).await, b"old-live").await);
            assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"old-live").await);
            let key = document_scope_key_v1(&scope);
            let first_live = state.presence.with(&(key.clone(), first.actor_id.clone()), |slot| slot.expect("first slot").socket_live_id.clone());

            let (mut socket_b, _) = connect_async(socket_request(&url, &second.grant)).await.expect("replacement socket");
            socket_b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("replacement hello");
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Session { actor, .. } if actor == second.actor_id));
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Presence { peers } if peers.is_empty()), "replacement removes the old visible row");
            assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Presence { peers } if peers.is_empty()));
            socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"stale-refresh").await }, Lane::Preview).await).await.expect("stale refresh");
            socket_a.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-stale-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("stale owner fence");
            assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Preview { seq: 1, .. }));
            assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Preview { seq: 1, .. }));
            assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Preview { seq: 1, .. }));
            socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live").await }, Lane::Preview).await).await.expect("current refresh");
            assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live").await);
            assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live").await);
            socket_a.close(None).await.expect("stale socket close");
            tokio::time::timeout(std::time::Duration::from_secs(2), async {
                while state.socket_grants.is_live(&first_record, &first_live) {
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                }
            })
            .await
            .expect("old socket cleanup completed");
            socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live-2").await }, Lane::Preview).await).await.expect("current refresh after stale close");
            assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live-2").await);
            assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live-2").await);
            let snapshot = state.presence_snapshot(&key);
            assert_eq!(snapshot.actors.len(), 1);
            assert_eq!(snapshot.actors[0].surface, plan.surface.surface_id);
            assert_eq!(snapshot.actors[0].actor, second.actor_id);
            observer.close(None).await.expect("observer close");
            socket_b.close(None).await.expect("current socket close");
            eprintln!("[DEBUG] plan-backed reconnect retained only the new live owner across stale refresh and close, observed by an independent admitted socket");
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
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"visible").await }, Lane::Preview).await).await.expect("visible presence");
            assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"visible").await);
            let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
            let mut observed = state.fanout_for(&key).subscribe();
            clock.gate_ticks.store(true, std::sync::atomic::Ordering::Release);
            clock.advance_to(PRESENCE_LEASE_TTL_MS - 1);
            clock.evaluate_tick(false).await;
            assert_eq!(state.presence_snapshot(&key).peers.len(), 1, "an evaluated server tick preserves the lease immediately before its deadline");
            assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "the evaluated early tick publishes no expiry");
            clock.advance_to(PRESENCE_LEASE_TTL_MS);
            clock.evaluate_tick(true).await;
            assert_eq!(clock.tick_release.available_permits(), 0, "ungating leaves no stale release permit");
            assert!(state.presence_snapshot(&key).peers.is_empty());
            assert!(matches!(observed.try_recv(), Ok(ServerFrame::Presence { peers }) if peers.is_empty()));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers.is_empty()), "the server tick publishes exact-deadline expiry");
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"revived").await }, Lane::Preview).await).await.expect("live socket refresh after visibility expiry");
            assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"revived").await, "expiry does not close or unregister the authenticated socket");
            eprintln!("[DEBUG] server tick barriers preserved visibility at TTL-1, published expiry at TTL, and kept the live socket refreshable");
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
            assert_eq!(state.install_presence_slot(&ordered_key, STUDIO, &ordered_scope.document_id, actor, test_presence_slot(live, None, now)).await, PresenceLeaseTransition::NoChange);
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
            assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, &actor, test_presence_slot(&live, None, now)).await, PresenceLeaseTransition::NoChange);
            assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, &actor, &live, vec![0; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES], now).await, PresenceLeaseTransition::Published);
        }
        assert_eq!(state.presence_snapshot(&full_key).peers.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
        assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", test_presence_slot("live-overflow", None, now)).await, PresenceLeaseTransition::NoChange);
        let deadline = state.presence.with(&(full_key.clone(), "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at);
        assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1], now + std::time::Duration::from_secs(1)).await, PresenceLeaseTransition::Rejected);
        assert_eq!(
            state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES + 1], now + std::time::Duration::from_secs(2)).await,
            PresenceLeaseTransition::Rejected
        );
        assert_eq!(state.presence.with(&(full_key, "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at), deadline, "rejection cannot refresh the lease deadline");

        let normalized_document = "presence-normalized-capacity";
        let normalized_key = document_scope_key_v1(&DocumentScope::new(STUDIO, normalized_document));
        let raw = presence_test_peer(b"canonical-capacity").await;
        let mut observed = state.fanout_for(&normalized_key).subscribe();
        for index in 0..=PRESENCE_ROSTER_MAXIMUM_ITEMS {
            let actor = format!("normalized-{index:03}");
            let live = format!("normalized-live-{index:03}");
            let slot = test_presence_slot(&live, Some("normalized-user"), now);
            let deadline = slot.expires_at;
            assert_eq!(state.install_presence_slot(&normalized_key, STUDIO, normalized_document, &actor, slot).await, PresenceLeaseTransition::NoChange);
            let before = state.presence_snapshot(&normalized_key).peers;
            let result = state.refresh_document_presence(&normalized_key, STUDIO, normalized_document, &actor, &live, raw.clone(), now + std::time::Duration::from_secs(1)).await;
            if index < PRESENCE_ROSTER_MAXIMUM_ITEMS {
                assert_eq!(result, PresenceLeaseTransition::Published);
                let ServerFrame::Presence { peers } = observed.try_recv().expect("normalized publication") else { panic!("normalized roster frame") };
                assert_eq!(peers.len(), index + 1);
                let peer = protocol::decode_presence_peer(peers.last().unwrap()).await.expect("canonical admitted row");
                assert_eq!(peer.actor, actor);
                assert_eq!(peer.user_id.as_deref(), Some("normalized-user"));
            } else {
                assert_eq!(result, PresenceLeaseTransition::Rejected);
                assert_eq!(state.presence_snapshot(&normalized_key).peers, before);
                assert_eq!(state.presence.with(&(normalized_key.clone(), actor), |slot| slot.expect("rejected normalized slot").expires_at), deadline);
                assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
            }
        }
        eprintln!("[DEBUG] canonical presence ingress admitted 64 bounded actors and rejected the 65th without TTL, roster or fanout effects");
    }

    #[tokio::test]
    async fn presence_lease_restart_is_empty_and_directory_presence_is_member_only() {
        let state = test_state().await;
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, "presence-restart"));
        let before = state.directory.head_seq().await.expect("directory head");
        let now = tokio::time::Instant::now();
        assert_eq!(state.install_presence_slot(&key, STUDIO, "presence-restart", "actor-a", test_presence_slot("live-a", Some("seed"), now)).await, PresenceLeaseTransition::NoChange);
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
            .map(|(space_id, name)| semio_hub::directory::NewDirectoryEvent {
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

    fn directory_command_body(request_id: &str, command: DirectoryCommand) -> Vec<u8> {
        DirectoryCommandRequestV1::new(request_id, command).canonical_json().into_bytes()
    }

    async fn post_directory_command_for_test(addr: SocketAddr, token: &str, request_id: &str, command: DirectoryCommand) -> RawHttpResponse {
        let authorization = format!("Bearer {token}");
        raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], &directory_command_body(request_id, command)).await
    }

    fn parse_directory_command_receipt_for_test(response: &RawHttpResponse, request_id: &str, command: &DirectoryCommand) -> DirectoryCommandReceiptV1 {
        let canonical = std::str::from_utf8(&response.body).expect("receipt UTF-8");
        DirectoryCommandReceiptV1::parse_canonical_json(canonical, &DirectoryCommandRequestV1::new(request_id, command.clone())).expect("canonical receipt")
    }

    #[tokio::test]
    async fn directory_command_authority_revalidates_after_durable_revocation_before_fence() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["appended"] == 0) {
            let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("authority state open deadline");
            let owner = issue_test_session(&state, "authority-owner@example.com").await;
            let author = issue_test_session(&state, "authority-author@example.com").await;
            let target = issue_test_session(&state, "authority-target@example.com").await;
            let space = create_space_for_test(&state, &owner.user_id, "Authority Race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
            upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
            let gate = Arc::new(TestLiveGate::default());
            *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), false));
            state.live_gate = Some(gate.clone());
            let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
            let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
            let request_id = "a00102030405060708090a0b0c0d0e0f";
            let pending = {
                let token = author.token.clone();
                let command = command.clone();
                tokio::spawn(async move { post_directory_command_for_test(addr, &token, request_id, command).await })
            };
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
            match row["revocation"].as_str().unwrap() {
                "demote" | "remove" => {
                    let revocation = if row["revocation"] == "demote" {
                        DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator }
                    } else {
                        DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: author.user_id.clone() }
                    };
                    assert_eq!(post_directory_command_for_test(addr, &owner.token, "b00102030405060708090a0b0c0d0e0f", revocation).await.status, 202);
                }
                "session" => assert_eq!(delete_session_me(bearer_headers(&author.token), State(state.clone())).await, StatusCode::NO_CONTENT),
                _ => unreachable!(),
            }
            let head = state.directory.head_seq().await.unwrap();
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
            assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["id"]);
            assert!(response.body.is_empty());
            assert_eq!(state.directory.head_seq().await.unwrap(), head);
            assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), Some(SpaceRole::Spectator));
            let claim =
                NewDirectoryCommandReceipt { actor_user_id: author.user_id.clone(), request_id: request_id.into(), command_sha256: directory_command_sha256(&command), result_kind: directory_command_result_kind(&command), claimed_at: now_ms() };
            assert!(matches!(state.directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Claimed(_)), "denied command must leave no durable claim");
            state.directory.release_directory_command_receipt(&author.user_id, request_id, &claim.command_sha256).await.unwrap();
            stop_recovery_server(state, shutdown, server).await;
            eprintln!("[DEBUG] directory authority case={} status={} appended=0 receipt=0 target-retained=1", row["id"], response.status);
        }
    }

    #[tokio::test]
    async fn directory_command_authority_holds_admitted_command_until_receipt_before_demotion() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
        let mut state = test_state().await;
        let owner = issue_test_session(&state, "authority-owner@example.com").await;
        let author = issue_test_session(&state, "authority-author@example.com").await;
        let target = issue_test_session(&state, "authority-target@example.com").await;
        let space = create_space_for_test(&state, &owner.user_id, "Authority Order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
        let gate = Arc::new(TestLiveGate::default());
        *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), true));
        state.live_gate = Some(gate.clone());
        let head = state.directory.head_seq().await.unwrap();
        let addr = spawn_server(state.clone()).await;
        let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
        let pending = {
            let token = author.token.clone();
            let command = command.clone();
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        gate.directory_command_attempted.acquire().await.unwrap().forget();
        let demotion = DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
        let revoking = {
            let token = owner.token.clone();
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, "d00102030405060708090a0b0c0d0e0f", demotion).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
        assert!(!revoking.is_finished());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
        let receipt = parse_directory_command_receipt_for_test(&response, "c00102030405060708090a0b0c0d0e0f", &command);
        assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted);
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
        let events = state.directory.events_since(head, 8).await.unwrap();
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::MemberRemoved { user_id, .. } if user_id == &target.user_id));
        assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), None);
        assert_eq!(state.directory.get_role(&space, &author.user_id).await.unwrap(), Some(SpaceRole::Spectator));
        eprintln!("[DEBUG] directory authority case={} status=202 appended=1 receipt=accepted revocation-ordered=1", row["id"]);
    }

    #[tokio::test]
    async fn directory_command_authority_invite_revocation_requires_the_exact_owned_space() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).expect("authority fixture");
        let state = test_state().await;
        let caller = issue_test_session(&state, "invite-scope-owner@example.test").await;
        let other = issue_test_session(&state, "invite-scope-other@example.test").await;
        let owned = create_space_for_test(&state, &caller.user_id, "Owned space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let foreign = create_space_for_test(&state, &other.user_id, "Foreign space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state.clone()).await;
        for (index, row) in fixture["inviteScopes"].as_array().expect("invite scope rows").iter().enumerate() {
            let issued = state.directory.issue_invite(&foreign, SpaceRole::Spectator, 600, "invite-scope-law").await.expect("foreign invite");
            let invite = issued.record.clone();
            if row["accepted"].as_bool().expect("accepted fixture") {
                let recipient = issue_test_session(&state, &format!("invite-recipient-{index}@example.test")).await;
                state.directory_service.redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-scope", recipient.user_id) }, &issued.capability, &recipient.user_id).await.expect("accepted invite fixture");
            }
            let exact = row["commandSpace"] == row["inviteSpace"];
            let (space, token) = if exact { (&foreign, &other.token) } else { (&owned, &caller.token) };
            let head = state.directory.head_seq().await.expect("before revoke head");
            let request_id = format!("{:032x}", 1000 + index);
            let response = post_directory_command_for_test(addr, token, &request_id, DirectoryCommand::RevokeInvite { space_id: space.clone(), invite_id: invite.id.clone() }).await;
            assert_eq!(u64::from(response.status), row["status"].as_u64().expect("expected status"), "{}", row["id"]);
            let current = state.directory.list_invites(&foreign).await.expect("foreign invite after request").into_iter().find(|candidate| candidate.id == invite.id).expect("retained foreign invite");
            assert_eq!(current.revoked_at.is_some(), row["revoked"].as_bool().expect("expected revocation"));
            assert_eq!(state.directory.head_seq().await.expect("after revoke head"), head, "invite revocation changes capability authority, not membership events");
            if !exact {
                assert!(response.body.is_empty(), "cross-space response carries no foreign metadata");
            }
            println!("[DEBUG] directory invite authority case={} status={} revoked={}", row["id"], response.status, current.revoked_at.is_some());
        }
    }

    /// 🎟️ Uses real space commands and HTTP redemption to preserve archive reads without restoring authorship.
    #[tokio::test]
    async fn directory_invite_redemption_obeys_current_space_state_and_readonly_replay() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).expect("invite state fixture");
        let state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite state open deadline");
        let owner = issue_test_session(&state, "invite-state-owner@example.test").await;
        let addr = spawn_server(state.clone()).await;
        for (index, row) in fixture["spaceStates"].as_array().expect("state rows").iter().enumerate() {
            let caller = issue_test_session(&state, &format!("invite-state-{index}@example.test")).await;
            let other = issue_test_session(&state, &format!("invite-state-other-{index}@example.test")).await;
            let space = create_space_for_test(&state, &owner.user_id, "Invite state", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let role = if row["inviteRole"] == "author" { SpaceRole::Author } else { SpaceRole::Spectator };
            let issued = state.directory.issue_invite(&space, role, 600, "invite-state-law").await.expect("pending invite");
            let accepted_user = if row["accepted"] == "other" { &other } else { &caller };
            let accepted = if row["accepted"] != "none" {
                state
                    .directory_service
                    .redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-state", accepted_user.user_id) }, &issued.capability, &accepted_user.user_id)
                    .await
                    .map(|commit| vec![commit.into_event()])
                    .expect("initial acceptance")
            } else {
                Vec::new()
            };
            let transition = match row["spaceState"].as_str().unwrap() {
                "archived" => Some(DirectoryCommand::ArchiveSpace { space_id: space.clone() }),
                "deleted" => Some(DirectoryCommand::DeleteSpace { space_id: space.clone() }),
                "writable" => None,
                _ => unreachable!(),
            };
            if let Some(command) = transition {
                assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 2000 + index), command).await.status, 202);
            }
            let replay_grant = if row["accepted"] == "same" && row["spaceState"] != "deleted" {
                let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
                let capability = SocketGrantCapability::mint().unwrap();
                let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new(&space, "invite-replay"));
                let subject =
                    SocketSubjectV1::Session { session_id: session.session_id, user_id: caller.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Spectator), expires_at_ms: session.expires_at };
                state.socket_grants.issue(&capability, audience.clone(), "hub.v1.invite-replay".into(), subject, now_ms(), now_ms() + 30_000).unwrap();
                Some((capability, audience))
            } else {
                None
            };
            let before = state.directory.head_seq().await.unwrap();
            let mut stream = state.directory_service.subscribe();
            let token = issued.capability.expose_once();
            let authorization = format!("Bearer {}", caller.token);
            let response = raw_http_request(addr, "POST", &format!("/directory/invites/{token}/redeem"), &[("Authorization", &authorization)], &[]).await;
            assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
            if let Some((capability, audience)) = replay_grant {
                assert!(state.socket_grants.pending(&capability, &audience, now_ms()).is_ok(), "read-only replay preserves fresh admission");
            }
            let appended = state.directory.events_since(before, 16).await.unwrap();
            assert_eq!(appended.len() as u64, row["appended"].as_u64().unwrap(), "{}", row["name"]);
            assert!(appended.iter().all(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })));
            let current = state.directory.get_role(&space, &caller.user_id).await.unwrap();
            let expected_role = match row["membershipRole"].as_str().unwrap() {
                "author" => Some(SpaceRole::Author),
                "spectator" => Some(SpaceRole::Spectator),
                "none" => None,
                _ => unreachable!(),
            };
            assert_eq!(current, expected_role, "{}", row["name"]);
            let invitations = state.directory.list_invites(&space).await.unwrap();
            if row["spaceState"] == "deleted" {
                assert!(invitations.is_empty());
            } else {
                let retained = invitations.iter().find(|invite| invite.id == issued.record.id).expect("retained invitation");
                assert_eq!(retained.accepted_at.is_some(), !accepted.is_empty() || !appended.is_empty());
                if !accepted.is_empty() {
                    assert_eq!(retained.accepted_event_id.as_deref(), Some(accepted[0].id.as_str()));
                }
            }
            if row["appended"] == 0 {
                assert!(matches!(stream.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "replay and denial publish no new event");
            } else {
                assert!(stream.try_recv().is_ok());
            }
            if response.status == 200 && !accepted.is_empty() {
                let expected = axum::body::to_bytes(DirectoryJson(accepted).into_response().into_body(), 1 << 20).await.unwrap();
                assert_eq!(response.body.as_slice(), expected.as_ref(), "replay returns the exact original immutable event");
            } else if response.status != 200 {
                assert!(response.body.is_empty());
            }
            eprintln!("[DEBUG] invite space state case={} status={} appended={} role={:?}", row["name"], response.status, appended.len(), current);
        }
    }

    /// 🧭️ Scope selection accepts only the exact stored secret and existing authenticated subject.
    #[tokio::test]
    async fn directory_invite_redemption_scope_hint_is_capability_bound() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
        let state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite hint state deadline");
        let owner = issue_test_session(&state, "invite-hint-owner@example.test").await;
        let caller = issue_test_session(&state, "invite-hint-caller@example.test").await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite hint", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-hint").await.unwrap();
        let other = InviteCapability::mint().unwrap();
        let other_encoded = other.expose_once();
        let wrong_secret = InviteCapability::parse(&other_encoded.replacen(other.selector(), issued.capability.selector(), 1)).unwrap();
        let head = state.directory.head_seq().await.unwrap();
        for row in fixture["vectors"].as_array().unwrap().iter().filter(|row| ["fresh-single", "wrong-selector", "wrong-secret", "actor-mismatch", "missing-user"].contains(&row["name"].as_str().unwrap())) {
            let user_id = if row["name"] == "missing-user" { "nonexistent-user" } else { &caller.user_id };
            let actor_user = if row["name"] == "actor-mismatch" { &owner.user_id } else { user_id };
            let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{actor_user}#hint") };
            let capability = match row["name"].as_str().unwrap() {
                "wrong-selector" => &other,
                "wrong-secret" => &wrong_secret,
                _ => &issued.capability,
            };
            let result = state.directory.invite_redemption_scope_hint(capability, &actor, user_id).await;
            if row["expected"]["outcomes"][0] == "newly-committed" {
                assert_eq!(result.unwrap().space_id(), space);
            } else {
                assert!(matches!(result, Err(DirectoryError::Unauthorized)), "{}", row["name"]);
            }
            assert_eq!(state.directory.head_seq().await.unwrap(), head, "a hint writes no event");
            assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None, "a hint grants no membership");
            assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
            eprintln!("[DEBUG] invite scope hint case={} capability-bound=1 mutation=0", row["name"]);
        }
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint", caller.user_id) };
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint-owner", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: space }).await.unwrap();
        assert!(matches!(state.directory.invite_redemption_scope_hint(&issued.capability, &actor, &caller.user_id).await, Err(DirectoryError::Unauthorized)));
    }

    /// 🔒️ A paused redemption cannot spend the identity captured before a durable revocation.
    #[tokio::test]
    async fn directory_invite_redemption_revalidates_after_hint_before_fence() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite race state deadline");
        let owner = issue_test_session(&state, "invite-race-owner@example.test").await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["authorityRaces"].as_array().unwrap().iter().filter(|row| row["appended"] == 0).enumerate() {
            let caller = issue_test_session(&state, &format!("invite-race-{index}@example.test")).await;
            let space = create_space_for_test(&state, &owner.user_id, "Invite race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-race").await.unwrap();
            *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), false));
            let pending = {
                let token = caller.token.clone();
                let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
                tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
            };
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
            if row["revocation"] == "session" {
                assert_eq!(delete_session_me(bearer_headers(&caller.token), State(state.clone())).await, StatusCode::NO_CONTENT);
            } else {
                let command = if row["revocation"] == "archive" { DirectoryCommand::ArchiveSpace { space_id: space.clone() } } else { DirectoryCommand::DeleteSpace { space_id: space.clone() } };
                assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 3000 + index), command).await.status, 202);
            }
            let head = state.directory.head_seq().await.unwrap();
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
            assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
            assert!(response.body.is_empty());
            assert_eq!(state.directory.head_seq().await.unwrap(), head);
            assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None);
            assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
            eprintln!("[DEBUG] invite authority race={} status={} appended=0", row["name"], response.status);
        }
        shutdown.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
    }

    /// 🧱️ All session and space keys stay owned through one irreversible invitation event.
    #[tokio::test]
    async fn directory_invite_redemption_admitted_fence_precedes_archive() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
        let row = fixture["authorityRaces"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite order state deadline");
        let owner = issue_test_session(&state, "invite-order-owner@example.test").await;
        let caller = issue_test_session(&state, "invite-order-caller@example.test").await;
        let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
        let space = create_space_for_test(&state, &owner.user_id, "Invite order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-order").await.unwrap();
        let gate = Arc::new(TestLiveGate::default());
        *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), true));
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let head = state.directory.head_seq().await.unwrap();
        let pending = {
            let token = caller.token.clone();
            let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
            tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        for key in [
            SocketBindingKeyV1::User(caller.user_id.clone()),
            SocketBindingKeyV1::Session(session.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space.clone() },
            SocketBindingKeyV1::Membership { space_id: space.clone(), user_id: caller.user_id.clone() },
        ] {
            assert!(state.socket_binding_gates.gate(key).try_lock_owned().is_err(), "the admitted redemption must own every exact authority key");
        }
        gate.directory_command_attempted.acquire().await.unwrap().forget();
        let revoking = {
            let token = owner.token.clone();
            let command = DirectoryCommand::ArchiveSpace { space_id: space.clone() };
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, "f10102030405060708090a0b0c0d0e0f", command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
        assert!(!revoking.is_finished());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
        let events = state.directory.events_since(head, 16).await.unwrap();
        assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::InviteRedeemed { user_id, role: DirectorySpaceRole::Author, .. } if user_id == &caller.user_id));
        assert!(matches!(&events.last().unwrap().body, os_directory::DirectoryEventBody::SpaceArchived { .. }));
        assert_eq!(events.iter().filter(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })).count(), row["appended"].as_u64().unwrap() as usize);
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), Some(SpaceRole::Spectator));
        shutdown.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
        eprintln!("[DEBUG] invite admitted-before-archive event-first=1 final-role=spectator");
    }

    #[tokio::test]
    async fn directory_command_authority_demotion_invalidates_only_affected_scope_once() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
        let shared = SocketGrantLedgerV1::default();
        let capacity = &fixture["capacity"];
        assert_eq!(capacity["subjectLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_BINDING_PENDING_CAPACITY);
        assert_eq!(capacity["ledgerLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_LEDGER_CAPACITY);
        for index in 0..capacity["subjects"].as_u64().unwrap() {
            let subject = SocketSubjectV1::Session { session_id: format!("capacity-session-{index}"), user_id: format!("capacity-user-{index}"), authorization_generation: 1, role: Some(SpaceRole::Author), expires_at_ms: 10_000 };
            shared.issue(&SocketGrantCapability::mint().unwrap(), SocketAudienceV1::DirectoryScoped(DocumentScope::new("shared-space", "document")), format!("hub.v1.capacity-{index}"), subject, 1, 9_000).unwrap();
        }
        assert_eq!(shared.inner.lock().unwrap().records.len() as u64, capacity["accepted"].as_u64().unwrap());
        eprintln!("[DEBUG] directory authority shared-space pending-subjects=65 subject-limit=64 global-limit=4096");
        let state = test_state().await;
        let owner = issue_test_session(&state, "authority-owner@example.com").await;
        let author = issue_test_session(&state, "authority-author@example.com").await;
        let space = create_space_for_test(&state, &owner.user_id, "Authority Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let other = create_space_for_test(&state, &author.user_id, "Other Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
        let mut records = Vec::new();
        for row in fixture["bindings"].as_array().unwrap() {
            let user = if row["user"] == "author" { &author } else { &owner };
            let session = state.directory.authenticate_session(&SessionCapability::parse(&user.token).unwrap()).await.unwrap().unwrap();
            let scope = DocumentScope::new(if row["space"] == "changed" { &space } else { &other }, "authority-document");
            let subject = SocketSubjectV1::Session { session_id: session.id.clone(), user_id: user.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Author), expires_at_ms: session.expires_at };
            let audience = match row["audience"].as_str().unwrap() {
                "document" => SocketAudienceV1::Document(scope),
                "scoped" => SocketAudienceV1::DirectoryScoped(scope),
                "global" => SocketAudienceV1::Directory { auth_session_id: session.id, authorization_generation: session.authorization_generation },
                _ => unreachable!(),
            };
            let pending = SocketGrantCapability::mint().unwrap();
            let live = SocketGrantCapability::mint().unwrap();
            for capability in [&pending, &live] {
                state.socket_grants.issue(capability, audience.clone(), format!("hub.v1.{}", row["id"].as_str().unwrap()), subject.clone(), now_ms(), now_ms() + 30_000).unwrap();
            }
            let record = state.socket_grants.pending(&live, &audience, now_ms()).unwrap();
            let record = state.socket_grants.consume(&record, now_ms()).unwrap();
            let (live_id, notify) = state.socket_grants.register_live(&record).unwrap();
            records.push((row, pending, record, live_id, notify));
        }
        let addr = spawn_server(state.clone()).await;
        let command = DirectoryCommand::UpsertMember { space_id: space, email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
        let request_id = "e00102030405060708090a0b0c0d0e0f";
        assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
        for (row, pending, record, live_id, notify) in &records {
            let invalidated = row["invalidated"].as_bool().unwrap();
            assert_eq!(state.socket_grants.pending(pending, &record.audience, now_ms()).is_err(), invalidated, "pending {}", row["id"]);
            assert_eq!(!state.socket_grants.is_live(record, live_id), invalidated, "live {}", row["id"]);
            if invalidated {
                tokio::time::timeout(std::time::Duration::from_secs(1), notify.notified()).await.unwrap();
                let fresh = SocketGrantCapability::mint().unwrap();
                let mut subject = record.subject.clone();
                if let SocketSubjectV1::Session { role, .. } = &mut subject {
                    *role = Some(SpaceRole::Spectator);
                }
                state.socket_grants.issue(&fresh, record.audience.clone(), record.actor_id.clone(), subject, now_ms(), now_ms() + 30_000).unwrap();
                assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
                assert!(state.socket_grants.pending(&fresh, &record.audience, now_ms()).is_ok(), "receipt replay must not invalidate fresh admission");
            }
            eprintln!("[DEBUG] directory authority binding={} invalidated={} replay-preserved=1", row["id"], invalidated);
        }
    }

    #[tokio::test]
    async fn directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids() {
        let state = test_state().await;
        let author = issue_test_session(&state, "command-receipt-author@example.com").await;
        let space_id = create_space_for_test(&state, &author.user_id, "Receipt Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state.clone()).await;
        let mut live = state.directory_service.subscribe();
        let command = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
        let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";
        let (first, second) = tokio::join!(post_directory_command_for_test(addr, &author.token, request_id, command.clone()), post_directory_command_for_test(addr, &author.token, request_id, command.clone()),);
        assert_eq!((first.status, second.status), (202, 202), "an idempotent duplicate is answered, never failed");
        let receipts = [parse_directory_command_receipt_for_test(&first, request_id, &command), parse_directory_command_receipt_for_test(&second, request_id, &command)];
        let tokens: Vec<String> = receipts
            .iter()
            .filter_map(|receipt| match &receipt.result {
                os_directory::DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
                os_directory::DirectoryCommandResultV1::None => None,
            })
            .collect();
        assert_eq!(tokens.len(), 1, "exactly one response carries the one-shot capability");
        assert_eq!(receipts.iter().filter(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::Accepted).count(), 1);
        assert!(receipts.iter().any(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::SecretUndeliverable && receipt.result == os_directory::DirectoryCommandResultV1::None));
        let token = tokens.into_iter().next().expect("issued capability");

        let invites = state.directory.list_invites(&space_id).await.expect("invite rows");
        assert_eq!(invites.len(), 1, "two concurrent identical request ids mint exactly one invitation");
        assert!(!format!("{invites:?}").contains(&token), "no invite row retains the capability plaintext");

        let retry = post_directory_command_for_test(addr, &author.token, request_id, command.clone()).await;
        let replayed = parse_directory_command_receipt_for_test(&retry, request_id, &command);
        assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
        assert!(!std::str::from_utf8(&retry.body).expect("retry UTF-8").contains(&token), "a later resolution of the same id is redacted");
        assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows after retry").len(), 1);

        let events = state.directory.events_since(0, 1_000).await.expect("durable log");
        assert!(!format!("{events:?}").contains(&token), "the capability never enters the durable event log");
        let mut broadcast = Vec::new();
        while let Ok(message) = live.try_recv() {
            broadcast.push(format!("{message:?}"));
        }
        assert!(!broadcast.join("\n").contains(&token), "the capability never enters the live broadcast");
    }

    #[tokio::test]
    async fn directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution() {
        let state = test_state().await;
        let author = issue_test_session(&state, "command-receipt-owner@example.com").await;
        let spectator = issue_test_session(&state, "command-receipt-spectator@example.com").await;
        let space_id = create_space_for_test(&state, &author.user_id, "Denial Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, "command-receipt-spectator@example.com", DirectorySpaceRole::Spectator).await;
        let addr = spawn_server(state.clone()).await;
        let invite = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
        let request_id = "0a1b2c3d4e5f60718293a4b5c6d7e8f9";

        let accepted = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
        assert_eq!(accepted.status, 202);
        let token = match parse_directory_command_receipt_for_test(&accepted, request_id, &invite).result {
            DirectoryCommandResultV1::Invite { invite_token } => invite_token,
            DirectoryCommandResultV1::None => panic!("the first accepted create-invite carries its capability"),
        };

        let denied = post_directory_command_for_test(addr, &spectator.token, request_id, invite.clone()).await;
        assert_eq!(denied.status, 403, "a spectator is denied before any stored completion is consulted");
        assert!(denied.body.is_empty(), "a denial carries no body, so no receipt or capability leaks");

        let cross_user =
            post_directory_command_for_test(addr, &spectator.token, request_id, DirectoryCommand::CreateSpace { name: "Spectator Space".into(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private })
                .await;
        assert_eq!(cross_user.status, 202, "the idempotency key is scoped to the authenticated user");
        assert!(!std::str::from_utf8(&cross_user.body).expect("cross-user UTF-8").contains(&token), "another user's equal request id never discovers a stored capability");

        let substituted = post_directory_command_for_test(addr, &author.token, request_id, DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Substituted".into() }).await;
        assert_eq!(substituted.status, 409, "an equal key with an unequal command digest is a generic conflict");
        assert!(substituted.body.is_empty());
        assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows").len(), 1, "no denial or conflict executed a second time");

        let capability = SessionCapability::parse(&author.token).expect("author capability");
        let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
        state.directory.revoke_auth_session(&record.id, "command-receipt-test", None, "command-receipt-test").await.expect("revoke session").expect("revoked row");
        let revoked = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
        assert_eq!(revoked.status, 401, "authentication re-runs before any stored completion is returned");
        assert!(revoked.body.is_empty());
    }

    #[tokio::test]
    async fn directory_command_receipt_v1_route_bounds_request_and_receipt_bytes() {
        let state = test_state().await;
        let author = issue_test_session(&state, "command-receipt-bounds@example.com").await;
        let space_id = create_space_for_test(&state, &author.user_id, "Bounds Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", author.token);

        let fitting = DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "n".repeat(64) };
        let request = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a0", fitting.clone());
        assert!(request.canonical_json().len() <= DIRECTORY_COMMAND_REQUEST_MAX_BYTES);
        let accepted = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], request.canonical_json().as_bytes()).await;
        assert_eq!(accepted.status, 202);
        assert!(accepted.body.len() <= os_directory::DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, "an admitted request can never produce an over-ceiling receipt");

        let padding = DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1 - DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: String::new() }).canonical_json().len();
        let oversize = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "x".repeat(padding) });
        assert_eq!(oversize.canonical_json().len(), DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1);
        let rejected = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], oversize.canonical_json().as_bytes()).await;
        assert!(matches!(rejected.status, 400 | 413), "one byte past the request ceiling is refused, got {}", rejected.status);

        for hostile in [
            "{\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"schema\":\"semio.directory.command-request.v1\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
            "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9F8E7D6C5B4A39281706F5E4D3C2B1A2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
            "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"00000000000000000000000000000000\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
            "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"},\"extra\":1}",
            "{\"kind\":\"archive-space\",\"spaceId\":\"x\"}",
        ] {
            let response = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], hostile.as_bytes()).await;
            assert_eq!(response.status, 400, "a noncanonical, mis-cased, zero-id, unknown-field, or bare-command body is refused: {hostile}");
            assert!(response.body.is_empty());
        }
    }

    #[tokio::test]
    async fn directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart() {
        let path = tempdir("command-receipt-store");
        std::fs::create_dir_all(&path).expect("receipt store dir");
        let database = path.join("directory.sqlite");
        let database_path = database.to_str().expect("receipt store path").to_string();
        let claim = |request_id: &str, digest: &str, kind: DirectoryCommandResultKindV1, actor: &str| NewDirectoryCommandReceipt {
            actor_user_id: actor.to_string(),
            request_id: request_id.to_string(),
            command_sha256: digest.to_string(),
            result_kind: kind,
            claimed_at: 1_700_000_000_000,
        };
        let invite_digest = directory_command_sha256(&DirectoryCommand::CreateInvite { space_id: "space".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 });
        let rename_digest = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space".into(), name: "Renamed".into() });
        let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";

        let directory = SqliteDirectory::connect(&database_path).await.expect("connect receipt store");
        directory.seed().await.expect("seed receipt store");
        let pending_claim = claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-a");
        assert!(matches!(directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("first claim"), DirectoryCommandClaimV1::Claimed(_)));
        let lost = match directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("lost-reply resolution") {
            DirectoryCommandClaimV1::Existing(record) => record,
            other => panic!("a claimed key never re-executes: {other:?}"),
        };
        assert_eq!(lost.disposition, DirectoryCommandDispositionV1::Pending);
        assert_eq!(replay_directory_command_receipt(&lost).outcome, DirectoryCommandOutcomeV1::SecretUndeliverable, "a reply lost between durable command and response is honestly undeliverable");
        assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &rename_digest, DirectoryCommandResultKindV1::None, "user-a")).await.expect("digest substitution"), DirectoryCommandClaimV1::Conflict));
        assert!(
            matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-b")).await.expect("cross-user claim"), DirectoryCommandClaimV1::Claimed(_)),
            "the key is scoped to the authenticated user"
        );

        let replay_digest = replay_directory_command_receipt(&DirectoryCommandReceiptRecord { disposition: DirectoryCommandDispositionV1::Completed, ..lost.clone() }).receipt_sha256;
        let completed = directory
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: "user-a".into(),
                request_id: request_id.into(),
                event_seq_first: None,
                event_seq_last: None,
                receipt_sha256: replay_digest.clone(),
                completed_at: 1_700_000_000_001,
            })
            .await
            .expect("durable completion");
        assert_eq!(completed.disposition, DirectoryCommandDispositionV1::Completed);
        assert!(
            directory
                .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                    actor_user_id: "user-a".into(),
                    request_id: request_id.into(),
                    event_seq_first: None,
                    event_seq_last: None,
                    receipt_sha256: replay_digest.clone(),
                    completed_at: 1_700_000_000_002
                })
                .await
                .is_err(),
            "one claim completes exactly once"
        );
        drop(directory);

        let restarted = SqliteDirectory::connect(&database_path).await.expect("reconnect receipt store");
        restarted.seed().await.expect("reseed receipt store");
        let resolved = match restarted.claim_or_read_directory_command_receipt(&pending_claim).await.expect("restart resolution") {
            DirectoryCommandClaimV1::Existing(record) => record,
            other => panic!("a completed key survives restart: {other:?}"),
        };
        assert_eq!(resolved.disposition, DirectoryCommandDispositionV1::Completed);
        assert_eq!(resolved.receipt_sha256.as_deref(), Some(replay_digest.as_str()), "the durable row carries the canonical redacted receipt digest");
        let replayed = replay_directory_command_receipt(&resolved);
        assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
        assert_eq!(replayed.receipt_sha256, replay_digest);
        assert!(replayed.validate().is_ok() && replayed.events.is_empty() && replayed.result == DirectoryCommandResultV1::None);

        let plain = NewDirectoryCommandReceipt { result_kind: DirectoryCommandResultKindV1::None, command_sha256: rename_digest.clone(), request_id: "9f8e7d6c5b4a39281706f5e4d3c2b1a0".into(), ..pending_claim.clone() };
        assert!(matches!(restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain claim"), DirectoryCommandClaimV1::Claimed(_)));
        restarted
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: plain.actor_user_id.clone(),
                request_id: plain.request_id.clone(),
                event_seq_first: Some(7),
                event_seq_last: Some(8),
                receipt_sha256: "0".repeat(64),
                completed_at: 1_700_000_000_003,
            })
            .await
            .expect("plain completion");
        let plain_record = match restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain resolution") {
            DirectoryCommandClaimV1::Existing(record) => record,
            other => panic!("a completed plain key replays: {other:?}"),
        };
        assert_eq!((plain_record.event_seq_first, plain_record.event_seq_last), (Some(7), Some(8)));
        assert_eq!(replay_directory_command_receipt(&plain_record).outcome, DirectoryCommandOutcomeV1::PreviouslyAccepted, "a completed secret-free command resolves as previously accepted");
        drop(restarted);
        let _ = std::fs::remove_dir_all(&path);
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
        let rejected = semio_hub::directory::NewDirectoryEvent {
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

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe() {
        let fixture = checkpoint_publication_fixture("idempotency").await;
        let addr = spawn_server(fixture.state.clone()).await;
        put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.pack).await;
        put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.spr).await;
        let route = format!("/spaces/{}/documents/{}/checkpoint-publications", fixture.scope.space_id, fixture.scope.document_id);
        let body = directory::os_pack::json::to_json_string(&fixture.command);
        let author = format!("Bearer {}", fixture.author.token);
        let spectator = format!("Bearer {}", fixture.spectator.token);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], body.as_bytes()).await.status, 401);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", spectator.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await.status, 403);

        let accepted = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await;
        assert_eq!(accepted.status, 200, "author checkpoint publication: {}", String::from_utf8_lossy(&accepted.body));
        assert!(accepted.headers.to_ascii_lowercase().contains("cache-control: private, no-store"));
        let receipt: CheckpointPublicationReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&accepted.body).expect("publication receipt UTF-8")).expect("canonical publication receipt");
        assert_eq!(receipt.correlation_id, fixture.command.correlation_id);
        assert_eq!(receipt.checkpoint.scope, fixture.scope);
        assert_eq!(receipt.checkpoint.pack.sha256.hex(), fixture.command.pack.sha256);
        assert_eq!(receipt.checkpoint.spr.sha256.hex(), fixture.command.spr.sha256);
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1);

        let replay = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await;
        assert_eq!(replay.status, 200);
        assert_eq!(replay.body, accepted.body, "a lost-response retry returns the identical durable receipt");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("replay checkpoint count"), 1, "retry emits no second checkpoint event");
        let command_sha256 = os_directory::hex_lower(&Sha256::digest(body.as_bytes()));
        let durable = fixture
            .state
            .directory
            .claim_or_read_checkpoint_publication(&NewCheckpointPublicationClaimV1 {
                actor_user_id: fixture.author.user_id.clone(),
                correlation_id: fixture.command.correlation_id.clone(),
                command_sha256: command_sha256.clone(),
                claimed_at: now_ms(),
            })
            .await
            .expect("durable publication receipt");
        let CheckpointPublicationClaimV1::Existing(durable) = durable else { panic!("completed publication must be durable") };
        assert_eq!(durable.disposition, CheckpointPublicationDispositionV1::Completed);
        assert_eq!(durable.checkpoint_id, Some(receipt.checkpoint.checkpoint_id));

        let mut substituted = fixture.command.clone();
        substituted.spr.byte_length += 1;
        let substituted = directory::os_pack::json::to_json_string(&substituted);
        let conflict = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], substituted.as_bytes()).await;
        assert_eq!(conflict.status, 409, "same author/correlation with a different exact command conflicts");
        assert!(conflict.body.is_empty());
        std::fs::remove_dir_all(fixture.catalog_root).expect("remove publication catalog fixture");
    }

    #[cfg(feature = "native-artifact-execution")]
    #[tokio::test]
    async fn checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication() {
        let mut fixture = checkpoint_publication_fixture("fence").await;
        let gate = Arc::new(TestLiveGate::default());
        gate.checkpoint_publication_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        fixture.state.live_gate = Some(gate.clone());
        let other_space = create_space_for_test(&fixture.state, &fixture.author.user_id, "Checkpoint other scope", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&fixture.state, &other_space, "checkpoint-fence-author@example.test", DirectorySpaceRole::Author).await;
        let descriptor = fixture.state.directory.get_document_descriptor(&fixture.scope).await.expect("publication descriptor read").expect("publication descriptor");
        let mut other_descriptor = descriptor.clone();
        other_descriptor.space_id = other_space.clone();
        fixture
            .state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: other_descriptor })
            .await
            .expect("announce same-id other-space document");
        let addr = spawn_server(fixture.state.clone()).await;
        put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.pack).await;
        put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.spr).await;
        let authorization = format!("Bearer {}", fixture.author.token);
        let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];

        let mut cross_scope = fixture.command.clone();
        cross_scope.correlation_id = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
        let cross_scope = directory::os_pack::json::to_json_string(&cross_scope);
        let cross = raw_http_request(addr, "POST", &format!("/spaces/{other_space}/documents/{}/checkpoint-publications", fixture.scope.document_id), &headers, cross_scope.as_bytes()).await;
        assert_eq!(cross.status, 409, "route scope cannot borrow another space's selected descriptor/frontier");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("source scope checkpoint count"), 0);
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&DocumentScope::new(&other_space, &fixture.scope.document_id)).await.expect("other scope checkpoint count"), 0);

        let route = format!("/spaces/{}/documents/{}/checkpoint-publications", fixture.scope.space_id, fixture.scope.document_id);
        let body = directory::os_pack::json::to_json_string(&fixture.command);
        let queued = tokio::spawn({
            let route = route.clone();
            let body = body.clone();
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("publication fence admission deadline").expect("publication fence admission").forget();
        let write = fixture.state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(fixture.scope.clone())).lock_owned().await;
        let document = db_artifact_id(&fixture.scope);
        let batch = db::document::CommandBatch::new(vec![sample_envelope("checkpoint-fence-edit-2", &WireArtifactId(document.0)).await]).await.expect("queued write batch");
        fixture.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("queued write actor response").expect("queued write accepted");
        drop(write);
        gate.checkpoint_publication_release.add_permits(1);
        let queued = queued.await.expect("queued publication response");
        assert_eq!(queued.status, 409, "the final actor snapshot fence rejects a write committed during materialization");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("stale publication count"), 0);
        let failed_digest = os_directory::hex_lower(&Sha256::digest(body.as_bytes()));
        let failed_claim = NewCheckpointPublicationClaimV1 { actor_user_id: fixture.author.user_id.clone(), correlation_id: fixture.command.correlation_id.clone(), command_sha256: failed_digest.clone(), claimed_at: now_ms() };
        assert!(
            matches!(fixture.state.directory.claim_or_read_checkpoint_publication(&failed_claim).await.expect("reclaim failed publication"), CheckpointPublicationClaimV1::Claimed(_)),
            "a returned failure synchronously releases its durable claim for a corrected retry"
        );
        fixture.state.directory.release_checkpoint_publication(&failed_claim.actor_user_id, &failed_claim.correlation_id, &failed_digest).await.expect("release test reclaim");

        let current = fixture.handle.checkpoint_publication_snapshot().await.expect("current publication snapshot");
        let descriptor_command = checkpoint_publication_command("cccccccccccccccccccccccccccccccc", &descriptor, &current, fixture.command.expected_current.clone(), &fixture.pack, &fixture.spr);
        let descriptor_body = directory::os_pack::json::to_json_string(&descriptor_command);
        let descriptor_swap = tokio::spawn({
            let route = route.clone();
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], descriptor_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("descriptor fence admission deadline").expect("descriptor fence admission").forget();
        let mut changed_descriptor = descriptor.clone();
        changed_descriptor.bootstrap_version = changed_descriptor.bootstrap_version.saturating_add(1);
        fixture
            .state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: changed_descriptor })
            .await
            .expect("replace publication descriptor");
        gate.checkpoint_publication_release.add_permits(1);
        let descriptor_swap = descriptor_swap.await.expect("descriptor-swapped publication response");
        assert_eq!(descriptor_swap.status, 409, "the final selected-descriptor fence rejects a replacement during materialization");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("descriptor-swapped publication count"), 0);
        fixture
            .state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() })
            .await
            .expect("restore publication descriptor");

        let mut cancellation = checkpoint_publication_command("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", &descriptor, &current, fixture.command.expected_current.clone(), &fixture.pack, &fixture.spr);
        cancellation.schema = "semio.hub.checkpoint-publication-command/v1".into();
        let cancellation = directory::os_pack::json::to_json_string(&cancellation);
        let capability = SessionCapability::parse(&fixture.author.token).expect("publication session capability");
        let session = fixture.state.directory.authenticate_session(&capability).await.expect("publication session lookup").expect("publication session");
        let revoked = tokio::spawn({
            let route = route.clone();
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], cancellation.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("revocation fence admission deadline").expect("revocation fence admission").forget();
        fixture.state.directory.revoke_auth_session(&session.id, "checkpoint-publication-test", None, "checkpoint-publication-test").await.expect("revoke publication session").expect("revoked publication session");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        gate.checkpoint_publication_release.add_permits(1);
        let revoked = revoked.await.expect("revoked publication response");
        assert_eq!(revoked.status, 503, "revocation cancels the request-local authority operation");
        assert!(revoked.body.is_empty());
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("cancelled publication count"), 0);
        std::fs::remove_dir_all(fixture.catalog_root).expect("remove publication fence catalog fixture");
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
