//! 🚀️ `startup` — process startup: errors, catalog control, supervisors, instance composition.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::Router;
use directory::os_directory::DocumentDescriptor;
use futures::StreamExt;
use semio_framework_trace::record::{TraceLevel, TraceOutcome, TraceRecord, Tracer};
use semio_hub::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores};
use semio_hub::artifact_authority::trusted_catalog::{NativeCodecProviderSourceV1, TrustedCatalogLoader, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedTrustedCatalog, TrustedCatalogLoadProgressCellV1};
#[cfg(test)]
use semio_hub::artifact_authority::ImmutableArtifactBlobStore;
use semio_hub::artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, CheckpointPublicationOrchestrator, OperationContext, ValidatingCanonicalArtifactAuthority};
use semio_hub::directory::error::DirectoryError;
use semio_hub::directory::{ArtifactCasSweepContinuation, ArtifactCasSweepRequest, ArtifactCasSweepResult, DirectoryService, HubDirectories, HubVerifiedCheckpointPublisher, ProjectionRebuildControl, IdentityVerificationControl, LocalBootstrapTransport};
use semio_hub::inference::runtime::GisMapApprovalCommitErrorV1;
use semio_hub::lag_rebootstrap::RebootstrapTransferControl;
use semio_hub::stores::{HubAuthModule, HubDirectoryModule, HubInstance, HubModules};
use server::gateway::ServerState;
use server::storage::StorageProfile;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::support::now_ms;
use crate::readiness::HubStartupProgressV1;
use db::db_storage::PayloadStorage as _;
use server::gateway::Server as FrameworkServer;

//#region ⚠️ Errors
/// 🧯️ Top-level startup error — the only fallible paths outside a document/WS session are
/// opening `db::Database`'s storage backend, connecting the directory backend, and binding the
/// HTTP listener.
#[derive(Debug)]
pub(crate) enum HubError {
    ArtifactAuthority(AuthorityError),
    Directory(DirectoryError),
    Db(db::DbError),
    DatabaseOpen(db::engine::DatabaseOpenAtRejected),
    StorageOpen(db::db_storage::DbStorageOpenRejected),
    InferenceShutdown(GisMapApprovalCommitErrorV1),
    Io(std::io::Error),
    UnknownStorageBackend(String),
    UnknownDirectoryBackend(String),
    UnsafeAuthConfiguration(String),
    InstanceStorage(String),
}

impl std::fmt::Display for HubError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArtifactAuthority(error) => std::fmt::Display::fmt(error, formatter),
            Self::Directory(error) => std::fmt::Display::fmt(error, formatter),
            Self::Db(error) => std::fmt::Display::fmt(error, formatter),
            Self::DatabaseOpen(error) => std::fmt::Display::fmt(error.error(), formatter),
            Self::StorageOpen(error) => std::fmt::Display::fmt(error.error(), formatter),
            Self::InferenceShutdown(error) => write!(formatter, "inference runtime shutdown failed: {error:?}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::UnknownStorageBackend(backend) => write!(formatter, "unknown OS_HUB_STORAGE_BACKEND: {backend}"),
            Self::UnknownDirectoryBackend(backend) => write!(formatter, "unknown OS_HUB_DIRECTORY_BACKEND: {backend}"),
            Self::UnsafeAuthConfiguration(detail) => write!(formatter, "unsafe hub authentication configuration: {detail}"),
            Self::InstanceStorage(detail) => write!(formatter, "hub instance storage unavailable: {detail}"),
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
            Self::InferenceShutdown(_) => None,
            Self::Io(error) => Some(error),
            Self::UnknownStorageBackend(_) | Self::UnknownDirectoryBackend(_) | Self::UnsafeAuthConfiguration(_) | Self::InstanceStorage(_) => None,
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

/// 📚️ Carries the tracer the trusted-catalog load reports its progress onto. A unit struct
/// would have had nowhere to send that progress, which is the only reason this holds a value.
pub(crate) struct StartupCatalogControl {
    pub(crate) tracer: Tracer,
    pub(crate) cancellation: StartupCancellationV1,
    pub(crate) progress: StartupProgressCellV1,
    /// 🕰️ When the last IN-FLIGHT progress record was emitted, so a long load reports that it
    /// is advancing without turning a 16 k-unit catalog into 16 k trace lines.
    last_in_flight_ms: std::sync::atomic::AtomicU64,
}

/// ⏲️ The smallest gap between two in-flight startup-catalog progress records. The first and
/// last unit of a load are always emitted; everything between is rate-limited to this.
const STARTUP_CATALOG_IN_FLIGHT_TRACE_MIN_GAP_MS: u64 = 1_000;

impl StartupCatalogControl {
    /// 📚️ Reporting onto `tracer` and into `progress`, cancelled with `cancellation`.
    pub(crate) fn new(tracer: Tracer, cancellation: StartupCancellationV1, progress: StartupProgressCellV1) -> Self {
        Self { tracer, cancellation, progress, last_in_flight_ms: std::sync::atomic::AtomicU64::new(0) }
    }

    /// 🤫️ Reporting nowhere — for a caller that runs before this process has configured
    /// observability, and for every law that is not about the catalog's progress.
    pub(crate) fn silent() -> Self {
        Self { tracer: Tracer::disabled(), cancellation: StartupCancellationV1::default(), progress: StartupProgressCellV1::default(), last_in_flight_ms: std::sync::atomic::AtomicU64::new(0) }
    }
}

/// 📈️ The latest progress a booting hub's startup work reported — what `/readyz` shows as
/// `startup` while the trusted catalog loads, so a waiting caller sees a hub that is advancing — and the
/// catalog's per-package progress, which the loaded catalog keeps reporting into for its background
/// verification.
#[derive(Clone, Default)]
pub(crate) struct StartupProgressCellV1 {
    pub(crate) latest: Arc<Mutex<Option<AuthorityProgress>>>,
    pub(crate) catalog: Arc<TrustedCatalogLoadProgressCellV1>,
}

impl StartupProgressCellV1 {
    pub(crate) fn observe(&self, progress: AuthorityProgress) {
        *self.latest.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(progress);
    }

    pub(crate) fn latest(&self) -> Option<HubStartupProgressV1> {
        let latest = *self.latest.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let catalog = Some(self.catalog.snapshot()).filter(|catalog| !catalog.packages.is_empty());
        latest.map(|progress| HubStartupProgressV1 { stage: progress.stage.code(), completed_units: progress.completed_units, total_units: progress.total_units, catalog })
    }
}

/// 🔐️ The catalog's background verification: once the hub serves, every package whose codec rows no
/// memory pinned at load is pinned against its component, smallest first, reporting into the same progress the
/// admin observability route reads; a codec call that reaches a package first verifies it itself. One
/// `server.catalog.publication` record names the outcome.
pub(crate) async fn verify_pending_catalog_guests(catalog: Arc<VerifiedTrustedCatalog>, tracer: Tracer, cancellation: StartupCancellationV1, progress: StartupProgressCellV1) {
    let control = StartupCatalogControl::new(tracer.clone(), cancellation, progress);
    let started = std::time::Instant::now();
    let outcome = match OperationContext::stall_bounded(TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS, AuthorityLimits::maximum(), &control) {
        Ok(context) => catalog.verify_pending_guests(&context).await,
        Err(error) => Err(error),
    };
    let verified = catalog.load_progress();
    let mut record = TraceRecord::new("server.catalog.publication", if outcome.is_ok() && verified.packages_refused == 0 { TraceOutcome::Ok } else { TraceOutcome::Refused });
    record.level = TraceLevel::Info;
    record.duration_us = Some(started.elapsed().as_micros().min(u128::from(u64::MAX)) as u64);
    record.detail = Some(format!("background-verification packages-ready={}/{} refused={} rows-verified={} rows-pinned={} outcome={}", verified.packages_ready, verified.packages_total, verified.packages_refused, verified.rows_verified, verified.rows_pinned, outcome.map_or_else(|error| error.to_string(), |()| "done".into())));
    tracer.emit(record);
}

/// 🚪️ Raised once the launcher's pipe is gone. Every startup step that can run for minutes —
/// the trusted-catalog load above all — observes it, so a hub whose launcher left stops loading and
/// exits instead of outliving it.
#[derive(Clone, Default)]
pub(crate) struct StartupCancellationV1(pub(crate) Arc<std::sync::atomic::AtomicBool>);

impl StartupCancellationV1 {
    pub(crate) fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(crate) fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

impl AuthorityOperationControl for StartupCatalogControl {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// 📡️ The boot's only outward sign that the catalog load is ALIVE. It used to emit the
    /// first and last unit alone, so a load that took 30 s printed nothing for 30 s and an operator
    /// could not tell a slow machine from a wedged one — the very distinction the load's own
    /// no-progress bound now makes internally. In-flight units are rate-limited to
    /// [`STARTUP_CATALOG_IN_FLIGHT_TRACE_MIN_GAP_MS`] so the record count never scales with the
    /// catalog's, and are emitted at `info`, the level a launcher runs the hub at: a `started`
    /// record defaults to `debug`, so a long guest-codec step (a component's pack-schema hash)
    /// printed nothing at `info` and a launcher's readiness stall bound killed a hub that was
    /// still loading.
    fn report(&self, progress: AuthorityProgress) {
        self.progress.observe(progress);
        let terminal = progress.completed_units == 0 || progress.completed_units == progress.total_units;
        if !terminal {
            let now_ms = self.now_ms();
            let last_ms = self.last_in_flight_ms.load(std::sync::atomic::Ordering::Relaxed);
            if now_ms.saturating_sub(last_ms) < STARTUP_CATALOG_IN_FLIGHT_TRACE_MIN_GAP_MS {
                return;
            }
            self.last_in_flight_ms.store(now_ms, std::sync::atomic::Ordering::Relaxed);
        }
        let outcome = if progress.completed_units == progress.total_units { TraceOutcome::Ok } else { TraceOutcome::Started };
        let mut record = TraceRecord::new("server.catalog.publication", outcome);
        record.level = TraceLevel::Info;
        record.detail = Some(format!("stage={:?} {}/{}", progress.stage, progress.completed_units, progress.total_units));
        self.tracer.emit(record);
    }

    fn catalog_progress(&self) -> Option<Arc<TrustedCatalogLoadProgressCellV1>> {
        Some(Arc::clone(&self.progress.catalog))
    }
}

pub(crate) struct HubBootstrapControl {
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
}

impl HubBootstrapControl {
    pub(crate) fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false) }
    }

    pub(crate) fn cancel(&self) {
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

pub(crate) type HubArtifactAuthority = ValidatingCanonicalArtifactAuthority<Arc<VerifiedTrustedCatalog>>;

pub(crate) type HubArtifactPublication = CheckpointPublicationOrchestrator<ArtifactChunkBlobStore<Arc<ArtifactChunkCasStores>>, HubVerifiedCheckpointPublisher<ArtifactChunkCasStores>>;

struct ArtifactCasMaintenanceControl {
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    tracer: Tracer,
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
            let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Ok);
            record.detail = Some(format!("stage={:?} {}/{}", progress.stage, progress.completed_units, progress.total_units));
            self.tracer.emit(record);
        }
    }
}

pub(crate) struct ArtifactCasMaintenanceSupervisor {
    pub(crate) cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) healthy: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) wake: Arc<tokio::sync::Notify>,
    pub(crate) task: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

#[derive(Default)]
pub(crate) struct ArtifactCasMaintenanceCheckpoint {
    continuation: Option<ArtifactCasSweepContinuation>,
}

impl ArtifactCasMaintenanceCheckpoint {
    pub(crate) fn request(&self, execute: bool, max_objects: usize) -> ArtifactCasSweepRequest {
        ArtifactCasSweepRequest { execute, max_objects, continuation: self.continuation }
    }

    pub(crate) fn accept(&mut self, result: &ArtifactCasSweepResult) -> bool {
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
    pub(crate) fn start(service: Arc<DirectoryService>, storage: Arc<ArtifactChunkCasStores>, execute: bool, tracer: Tracer) -> Arc<Self> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let healthy = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let wake = Arc::new(tokio::sync::Notify::new());
        let control_cancelled = cancelled.clone();
        let task_healthy = healthy.clone();
        let task_wake = wake.clone();
        let task = tokio::spawn(async move {
            let control = ArtifactCasMaintenanceControl { cancelled: control_cancelled, tracer };
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
                            let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Ok);
                            record.detail = Some(format!(
                                "examined={} protected={} eligible={} deleted={} missing={} continued={}",
                                result.examined_objects,
                                result.protected_objects,
                                result.eligible_objects,
                                result.deleted_objects,
                                result.missing_objects,
                                result.continuation.is_some()
                            ));
                            control.tracer.emit(record);
                            if checkpoint.accept(&result) {
                                break;
                            }
                        }
                        Err(AuthorityError::Cancelled) if control.is_cancelled() => return,
                        Err(error) => {
                            task_healthy.store(false, std::sync::atomic::Ordering::Release);
                            checkpoint.fail(&error);
                            let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Failed);
                            record.detail = Some(format!("failed-closed:{error}"));
                            control.tracer.emit(record);
                            break;
                        }
                    }
                }
            }
        });
        Arc::new(Self { cancelled, healthy, wake, task: Mutex::new(Some(task)) })
    }

    #[cfg(test)]
    pub(crate) fn disabled() -> Arc<Self> {
        Arc::new(Self { cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)), healthy: Arc::new(std::sync::atomic::AtomicBool::new(true)), wake: Arc::new(tokio::sync::Notify::new()), task: std::sync::Mutex::new(None) })
    }

    pub(crate) fn healthy(&self) -> bool {
        self.healthy.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(crate) async fn shutdown(&self) {
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

/// 🚰️ How often the hub hands its instance's committed outbox rows to the saga runner.
pub(crate) const SAGA_DRAIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

/// 🚰️ How many outbox rows one drain may move. Bounded so a backlog is worked off over
/// several turns instead of one unbounded pass holding both store locks.
const SAGA_DRAIN_BATCH: usize = 64;

/// 🚰️ Runs [`ServerState::drain_sagas`] on a fixed cadence for as long as this hub serves.
///
/// The framework's own doc for that call states the reason this exists: "a forgotten runner is a
/// queue that grows forever while every event looks delivered". Hub commits into the instance's
/// authority store the moment any of its subsystems moves behind the command bus, and the outbox
/// row is committed in the same transaction as the event — a row nobody drains is a state change
/// the world is never told about, and it fails silently, which is the one failure mode an outbox is
/// built to make impossible.
///
/// It is silent unless it moves something. A cadence that emitted a record per tick would produce
/// two records a second forever and bury the ones that matter; a drain that moved rows is a fact
/// worth a line, so only that is reported (`server.saga.drain`, `ok` when every follow-up turn
/// succeeded, `failed` with the count when some did not).
pub(crate) struct SagaDrainSupervisor {
    pub(crate) cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) wake: Arc<tokio::sync::Notify>,
    pub(crate) task: Mutex<Option<tokio::task::JoinHandle<()>>>,
    pub(crate) instance: ServerState<HubInstance>,
    pub(crate) tracer: Tracer,
}

impl SagaDrainSupervisor {
    pub(crate) fn start(instance: ServerState<HubInstance>, tracer: Tracer, interval: std::time::Duration) -> Arc<Self> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let wake = Arc::new(tokio::sync::Notify::new());
        let task_cancelled = cancelled.clone();
        let task_wake = wake.clone();
        let task_instance = instance.clone();
        let task_tracer = tracer.clone();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(interval) => {}
                    _ = task_wake.notified() => {}
                }
                if task_cancelled.load(std::sync::atomic::Ordering::Acquire) {
                    return;
                }
                drain_instance_sagas(&task_instance, &task_tracer).await;
            }
        });
        Arc::new(Self { cancelled, wake, task: Mutex::new(Some(task)), instance, tracer })
    }

    /// 🚰️ Stops the cadence and then drains once more by hand. The last pass is the point:
    /// the requests that were still in flight when the router stopped accepting may have committed
    /// outbox rows after the final tick, and leaving those to the next boot is exactly the
    /// acknowledged-but-never-delivered window the outbox exists to close.
    pub(crate) async fn shutdown(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.wake.notify_waiters();
        let task = self.task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(mut task) = task {
            if tokio::time::timeout(std::time::Duration::from_secs(5), &mut task).await.is_err() {
                task.abort();
                let _ = task.await;
            }
        }
        drain_instance_sagas(&self.instance, &self.tracer).await;
    }
}

impl Drop for SagaDrainSupervisor {
    fn drop(&mut self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.wake.notify_waiters();
        if let Some(task) = self.task.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            task.abort();
        }
    }
}

/// 🚰️ One drain pass, reported only when it moved rows. Returns how many follow-up turns ran
/// so a law can assert on the pass itself without owning the supervisor's clock.
pub(crate) async fn drain_instance_sagas(instance: &ServerState<HubInstance>, tracer: &Tracer) -> usize {
    let span = tracer.span("server.saga.drain").request(tracer.allocate_request_id());
    let outcomes = instance.drain_sagas(SAGA_DRAIN_BATCH).await;
    if outcomes.is_empty() {
        return 0;
    }
    let rejected = outcomes.iter().filter(|outcome| matches!(outcome, server::contract::CommandOutcome::Rejected { .. })).count();
    if rejected == 0 {
        span.finish(TraceOutcome::Ok, Some(format!("drained-{}", outcomes.len())));
    } else {
        span.finish(TraceOutcome::Failed, Some(format!("drained-{} rejected-{rejected}", outcomes.len())));
    }
    outcomes.len()
}

pub(crate) fn artifact_cas_sweep_execute_from_env() -> Result<bool, HubError> {
    match std::env::var("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE").as_deref() {
        Err(std::env::VarError::NotPresent) | Ok("") | Ok("false") | Ok("0") => Ok(false),
        Ok("true") | Ok("1") => Ok(true),
        Err(error) => Err(HubError::UnsafeAuthConfiguration(format!("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE is unreadable: {error}"))),
        Ok(_) => Err(HubError::UnsafeAuthConfiguration("OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE must be true, false, 1, or 0".into())),
    }
}

pub(crate) struct ConfiguredArtifactAuthority {
    pub(crate) catalog: Arc<VerifiedTrustedCatalog>,
    pub(crate) authority: Arc<HubArtifactAuthority>,
}

pub(crate) trait DocumentOpenCatalogAuthorityV1: Send + Sync {
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

/// ⏳️ The span of NO PROGRESS AT ALL that ends the startup trusted-catalog load. It is not a total
/// budget and never was a good one: a total wall budget charges the load for every millisecond the
/// machine spends on other work, so a hub on a busy machine refused a catalog that a hub on an idle
/// machine loaded fine (measured 2026-09-21 by M8 with a warm 612 MB catalog, and again by CE2 on
/// two consecutive 7621 starts while 17 rustc ran). The load reports progress per package phase and
/// checkpoints per 64 KiB hashed chunk, and those checkpoints only restamp the span when THIS task
/// is scheduled — under fleet load the async runtime can starve the load for tens of seconds between
/// two live chunks (measured 2026-09-22 by HC1 on 7682, and 2026-09-23 by M10b on 7681 at load ≈ 67
/// with a three-package catalog whose largest component is ≈ 48 MB). Thirty seconds therefore named
/// a busy scheduler as a wedged load. Five minutes is still a no-progress bound: a load that reaches
/// no checkpoint for that long has genuinely stopped, and eight stall-retries still apply above.
pub(crate) const TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS: u64 = 300_000;

/// 📚️ What the startup trusted-catalog load produced. Three outcomes, not two, because "the load
/// ran out of budget" and "this data root has no catalog" are different facts and the hub owes an
/// operator the difference. A structural refusal — a corrupt catalog, a missing provider, a
/// signature that does not verify — stays an `Err` and still aborts the boot: that one IS a
/// statement about the data root.
pub(crate) enum StartupArtifactAuthority {
    Configured(ConfiguredArtifactAuthority),
    Absent,
    /// ⏳️ The load reached no checkpoint for [`TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS`]. Before this
    /// variant the hub EXITED on it — the opposite of every other gate's behaviour, which is to name
    /// a reason in `blocked_by` and still bind — so a wedged load looked exactly like a corrupt data
    /// root to whoever read the exit.
    Stalled,
}

impl StartupArtifactAuthority {
    /// 📚️ The loaded authority, or `None` for either of the two "no authority" outcomes — the shape
    /// every reader that only cares whether a catalog is live wants.
    pub(crate) fn configured(self) -> Option<ConfiguredArtifactAuthority> {
        match self {
            Self::Configured(configured) => Some(configured),
            Self::Absent | Self::Stalled => None,
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::Absent | Self::Stalled)
    }
}

/// 🧾️ The reason `artifactAuthority` is closed, from the three facts that decide it. A load that
/// stalled is NOT "pointer present but not loadable": nothing was found wrong with the catalog, so
/// the reason must not read as if something had been.
pub fn artifact_authority_closed_reason(native_artifact_execution: bool, trusted_catalog_stalled: bool, catalog_pointer_present: bool) -> &'static str {
    if !native_artifact_execution {
        "native-artifact-execution-feature-not-compiled"
    } else if trusted_catalog_stalled {
        "trusted-catalog-load-stalled-before-it-finished"
    } else if catalog_pointer_present {
        "trusted-catalog-pointer-present-but-not-loadable"
    } else {
        "trusted-catalog-never-published-in-this-data-root"
    }
}

pub(crate) async fn configured_artifact_authority(data_dir: &std::path::Path, providers: Option<&dyn NativeCodecProviderSourceV1>, tracer: &Tracer, cancellation: &StartupCancellationV1, progress: &StartupProgressCellV1) -> Result<StartupArtifactAuthority, AuthorityError> {
    if providers.is_none() && data_dir.join("trusted-catalog/current.json").try_exists().map_err(|error| AuthorityError::Catalog(error.to_string()))? {
        return Err(AuthorityError::Catalog("configured trusted catalog requires the native-artifact-execution provider".into()));
    }
    let Some(providers) = providers else {
        return Ok(StartupArtifactAuthority::Absent);
    };
    let mut loaded = None;
    for attempt in 0..8u8 {
        let control = StartupCatalogControl::new(tracer.clone(), cancellation.clone(), progress.clone());
        let context = OperationContext::stall_bounded(TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS, AuthorityLimits::maximum(), &control)?;
        match TrustedCatalogLoader::load_current(data_dir, providers, &context).await {
            Ok(value) => {
                loaded = Some(value);
                break;
            }
            Err(_) if cancellation.is_cancelled() => return Err(AuthorityError::Cancelled),
            Err(AuthorityError::Stalled | AuthorityError::Cancelled) if attempt + 1 < 8 => {
                let mut record = TraceRecord::new("server.catalog.publication", TraceOutcome::Ok);
                record.detail = Some(format!("startup-load-stall-retry attempt={}", attempt + 1));
                tracer.emit(record);
                continue;
            }
            Err(AuthorityError::Stalled | AuthorityError::Cancelled) => return Ok(StartupArtifactAuthority::Stalled),
            Err(error) => return Err(error),
        }
    }
    let loaded = loaded.expect("trusted catalog startup load attempts exhausted without a terminal outcome");
    let Some(catalog) = loaded else {
        return Ok(StartupArtifactAuthority::Absent);
    };
    let catalog = Arc::new(catalog);
    let authority = Arc::new(ValidatingCanonicalArtifactAuthority::new(catalog.clone()));
    Ok(StartupArtifactAuthority::Configured(ConfiguredArtifactAuthority { catalog, authority }))
}

//#region 🔖️Directory
/// 🗄️ Opens this process's [`HubInstance`] — the server product's four durable storage roles
/// plus the command bus and the saga runner over them — rooted at `{data_dir}/instance`.
///
/// It goes through the framework's own [`FrameworkServer`] builder rather than calling
/// `HubInstance::open` directly, because the bus, the policy engine and the saga runner are
/// assembled there: opening the stores by hand would give hub four files and none of the machinery
/// that makes writing to them exactly-once.
/// Auth and directory modules register on the framework gateway; the document socket is not a
/// framework route here, because hub's own router owns `/scopes/{scope}/document/ws`.
pub(crate) async fn compose_hub_server(data_dir: &std::path::Path, directory: Arc<HubDirectories>) -> Result<(ServerState<HubInstance>, Router), HubError> {
    let root = data_dir.join("instance");
    let profile = StorageProfile::Embedded { data_dir: root.to_string_lossy().into_owned() };
    let server = FrameworkServer::<HubInstance>::builder(profile)
        .identity("os-hub", env!("CARGO_PKG_VERSION"))
        .module(HubModules::Auth(HubAuthModule::new(directory)))
        .module(HubModules::Directory(HubDirectoryModule::new()))
        .build()
        .await
        .map_err(|error| HubError::InstanceStorage(format!("{error:?}")))?;
    Ok((server.state().clone(), server.router()))
}

pub(crate) async fn instance_state(data_dir: &std::path::Path) -> Result<ServerState<HubInstance>, HubError> {
    let root = data_dir.join("instance");
    let profile = StorageProfile::Embedded { data_dir: root.to_string_lossy().into_owned() };
    let server = FrameworkServer::<HubInstance>::builder(profile)
        .identity("os-hub", env!("CARGO_PKG_VERSION"))
        .build()
        .await
        .map_err(|error| HubError::InstanceStorage(format!("{error:?}")))?;
    Ok(server.state().clone())
}

/// ⚖️ `OS_HUB_MERGE_POLICY=laissez-faire|normal|vigilant` (default `normal`) — read once at
/// startup into `HubState.merge_policy` (see its own doc). An unrecognized value is a non-fatal
/// misconfiguration (logged, falls back to the default) rather than refusing to boot, matching this
/// crate's generally-forgiving stance on env parsing elsewhere in `main`.
pub(crate) fn merge_policy_from_env(tracer: &Tracer) -> protocol::MergePolicy {
    match std::env::var("OS_HUB_MERGE_POLICY").ok().as_deref() {
        None => protocol::MergePolicy::default(),
        Some("laissez-faire") => protocol::MergePolicy::LaissezFaire,
        Some("normal") => protocol::MergePolicy::Normal,
        Some("vigilant") => protocol::MergePolicy::Vigilant,
        Some(other) => {
            let mut record = TraceRecord::new("server.boot", TraceOutcome::Refused);
            record.detail = Some(format!("unknown-merge-policy:{other}"));
            tracer.emit(record);
            protocol::MergePolicy::default()
        }
    }
}
//#endregion 🔖️Directory
