//! 🗄️ `db_engine` — the `Database` supervisor and catalog actor: the crate that assembles every
//! other `db_*` crate into the stable, contract-frozen `Database`/`ArtifactHandle` API
//! (`Database::{open, open_at, create_document, document, catalog, health, shutdown}`;
//! `ArtifactHandle::{submit, query, subscribe, frontier, preview, history, snapshot_now}`).
//! Frozen contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_engine` row + "Stable API" block).
//!
//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape byte-for-byte stable specifically
//! because THIS crate constructs every one of those verbatim. `SubmitOptions{durability, policy}`
//! and `CommandReceipt{.., messages}` both gained a field under
//! `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C9 — every construction site below
//! and `to_engine_receipt`'s field-for-field bridge were updated in lockstep (never a `..Default`
//! spread over a frozen-shape struct, so a future field addition here fails loud, not silent).
//!
//! 🎯️ Design choice (scope): per this wave's instructions, this crate makes `Database::open_at`
//! (zero-touch `FsStorage`) and a full submit → durable → query round trip over a REAL
//! `db_artifact::ArtifactAuthority` genuinely work end to end (see `//#region 🧪️Tests`), composing
//! the guaranteed-complete `db_state`/`db_wal`/`db_storage`/`db_artifact` crates against their real,
//! current APIs throughout. `db_cluster` is still an unimplemented stub upstream of this wave (its
//! `lib.rs` declares no public items at all) — nothing in this crate can call into it yet; every
//! cluster-shaped concern (sharding, ownership leases, quorum durability, split-brain repair) is
//! deferred wholesale, documented here rather than faked. `db_compact`/`db_sync`/`db_security`/
//! `db_observe` ARE genuinely wired, but narrowly: `Database::compact_document` drives a real
//! retained compaction pass, and `Database::hello` mounts `db_sync::DatabaseSyncHelloFuture` for the wire-v2
//! handshake (no transport of its own — that is CW5/CW6's job), and `Database::open`/`open_at` wire a
//! real `db_observe::StructuredSink`/`HealthRegistry` pair by default. `ArtifactHandle::preview`/
//! `subscribe` return `DbError::Unimplemented` (not a panic, not a fake success): `db_artifact`'s
//! own `ArtifactAuthority` mailbox (`db/document/rs/lib.rs`'s `ArtifactMessage` enum) only carries
//! `Submit`/`Query`/`Frontier` variants — there is no way to drive its preview/commit-log machinery
//! through the actor boundary without editing `db_artifact` itself, which is out of this crate's
//! ownership this wave. `snapshot_now` is likewise `Unimplemented`: `db_artifact`'s own module doc
//! documents that `DocumentState` materializes purely from the WAL suffix with no full-state
//! enumeration to serialize into a pack snapshot, and `db_snapshot` is not even a direct dependency
//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` replays the WAL through the
//! document authority's retained cursor: the WAL is the only record of a document's history.
#![allow(async_fn_in_trait)]
#![cfg(not(target_arch = "wasm32"))]

use db_durability::EpochFence;
use db_engine_base::{CatalogState, CatalogView, DatabaseDocumentMountFailure, DatabaseDocumentMountFuture, DatabaseDocumentMountOwner, DatabaseDocumentMountPolicy, DatabaseDocumentMountPublishedHook, DatabaseDocumentMountRegistry, DatabaseDocumentMountReply, DatabaseDocumentMountSlot, DatabaseDocumentMountWait, DatabaseDocumentMountWaiter, database_document_mount_work_live_slots, decode_catalog, default_emit, encode_catalog_pages, now_ms, to_core_document_id};
use db_engine_capability_open::{DATABASE_CAPABILITY_OPEN_LIVE, DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenRejected, database_capability_open_registry};
use db_engine_catalog_bootstrap::{DATABASE_CATALOG_BOOTSTRAP_LIVE, DatabaseCatalogBootstrapFuture, DatabaseCatalogBootstrapKey, DatabaseCatalogBootstrapRejected, database_catalog_bootstrap_registry};
use db_engine_catalog_create::{DATABASE_CREATE_CATALOG_LIVE, DatabaseCreateCatalogFuture, DatabaseCreateCatalogRejected, database_create_catalog_registry};
use db_engine_catalog_read::{DATABASE_CATALOG_READ_LIVE, DatabaseCatalogReadFuture, DatabaseCatalogReadRejected, DatabaseCatalogRootKey, database_catalog_read_registry};
use db_engine_handle::{ArtifactCompactionFuture, ArtifactHandle, ArtifactHandleLease};
use db_ids::{ActorId, ArtifactId, DbError};
use db_io_core::CatalogStorage as _;
use db_observe::{Emit, EmitEvent, EmitField};
use db_policy::MailboxCapacities;
use semio_framework_async::{WorkerPool, WorkerPoolUse};
use std::{sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::{Consistency, DATABASE_DOCUMENT_MOUNT_WAITERS, DatabaseDocumentMountCleanupFaultHook, DatabaseDocumentMountDriver, DatabaseDocumentMountParkedHook, DatabaseDocumentMountWork, LiveQuerySpec, Query, QueryResultEntry, QueryStream, SnapshotKind, test_worker_pool};
#[cfg(test)]
use db_engine_handle::{ARTIFACT_HISTORY_ADMISSION, ARTIFACT_HISTORY_OPERATION_SLOTS, ArtifactHistoryAdmission, ArtifactHistoryAdmissionError, ArtifactHistoryState, HistoryFuture};
#[cfg(test)]
use db_ids::GenerationId;
#[cfg(test)]
use semio_framework_async::Lane;
#[cfg(test)]
use std::future::Future;

//#region 🔖️Reexports
pub use db_durability::DurabilityClass;

pub use db_policy::{DbCapabilities, DbConfig, Profile};
//#endregion 🔖️Reexports
//#region 🔖️ArtifactSpec
/// 📄️ What `Database::create_document` needs to mint a brand-new document — this crate's own
/// choice (the contract fixes `create_document`'s signature, not `ArtifactSpec`'s shape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactSpec {
    pub document: protocol::ArtifactId,
}

impl ArtifactSpec {
    pub async fn new(document: protocol::ArtifactId) -> ArtifactSpec {
        ArtifactSpec { document }
    }
}
//#endregion 🔖️ArtifactSpec
//#region 🔖️Health
/// 🩺️ The frozen `Database::health()` return shape, wrapping a real
/// `db_observe::HealthRegistry` snapshot plus this crate's own catalog-level fact (open document
/// count) that no lower crate could know.
#[derive(Clone, Debug)]
pub struct DbHealth {
    pub report: db_observe::HealthReport,
    pub open_artifacts: usize,
}
//#endregion 🔖️Health

/// 🚪️ Shared interruption authority for a retained database shutdown.
pub struct DatabaseShutdownControl {
    pub deadline: std::time::Instant,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
}

impl DatabaseShutdownControl {
    pub fn new(deadline: std::time::Instant, cancelled: Arc<std::sync::atomic::AtomicBool>) -> DatabaseShutdownControl {
        DatabaseShutdownControl { deadline, cancelled }
    }

    pub fn for_timeout(timeout: std::time::Duration) -> DatabaseShutdownControl {
        DatabaseShutdownControl::new(std::time::Instant::now() + timeout, Arc::new(std::sync::atomic::AtomicBool::new(false)))
    }

    pub fn interrupted(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire) || std::time::Instant::now() >= self.deadline
    }

    /// 🔦️ The interruption an expired or cancelled shutdown answers with. A deadline names the
    /// phase it expired in and that phase's retained owner counts, because the caller's only other
    /// evidence — the step sequence — is already gone by the time the error reaches it.
    fn interruption_error(&self, witness: DatabaseShutdownInterruptionWitness) -> DbError {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            DbError::Closed
        } else {
            DbError::Unavailable(format!(
                "database shutdown deadline elapsed in phase {:?}: open artifacts {}, emit started {}, retained pool-use owners {}, retained slots {:?}",
                witness.phase, witness.open_artifacts, witness.emit_started, witness.pool_use_owners, witness.retained_slots
            ))
        }
    }
}

/// 🔎️ What one interrupted shutdown had reached when its deadline expired.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DatabaseShutdownInterruptionWitness {
    pub phase: Option<DatabaseShutdownPhase>,
    open_artifacts: usize,
    emit_started: bool,
    pool_use_owners: usize,
    retained_slots: DatabaseRetainedPoolUseCensus,
}

/// 🔬️ How many LIVE owners of each retained-activity family still hold a `WorkerPoolUse` clone.
/// Every slot registry in this crate is a TERMINAL table — a state registers its slot only when it
/// is orphaned — so a census built from those tables reads 0 for a family whose owner is simply
/// still alive. These four counters are incremented where the state that stores the clone is built
/// and decremented in its `Drop`, so they see the live owner the tables cannot.
static DATABASE_MOUNT_FUTURE_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ One live document-mount future. A mount future OWNS a `WorkerPoolUse` clone from the moment
/// it is built, and the clone lives exactly as long as the future — whoever holds it. Every other
/// census family counts a struct field; this one counts the futures themselves, which is the only
/// carrier left once every field reads 0.
struct DatabaseMountFutureLiveGuardV1;

impl DatabaseMountFutureLiveGuardV1 {
    pub fn new() -> Self {
        DATABASE_MOUNT_FUTURE_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self
    }
}

impl Drop for DatabaseMountFutureLiveGuardV1 {
    fn drop(&mut self) {
        DATABASE_MOUNT_FUTURE_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

static DATABASE_POOL_USE_MOUNT_WAIT_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ One live BARE `require_open_use` clone that never reaches a registered owner. `mount_document`
/// clones the Database's use into a plain local and holds it across its awaits while the mount waiter
/// parks on another caller's `Opening` slot. That clone is in no slot table, in no `live_*` struct family
/// and in no mount future, so it is invisible to every other census family while it pins the use.
/// `require_open_use` has exactly three call sites; the other two hand their clone to a counted future, so
/// this guard closes the census over every carrier of `Database::pool_use`.
struct DatabasePoolUseSiteGuardV1(&'static std::sync::atomic::AtomicUsize);

impl DatabasePoolUseSiteGuardV1 {
    pub fn new(site: &'static std::sync::atomic::AtomicUsize) -> Self {
        site.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self(site)
    }
}

impl Drop for DatabasePoolUseSiteGuardV1 {
    fn drop(&mut self) {
        self.0.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

/// 🔬️ How many retained owners of each family still hold a `WorkerPoolUse` clone of the
/// `Database`'s use, because `Arc::strong_count` names a number and never an owner. The first six
/// are process-global slot tables; `runner_handoff` is the family that has no table at all — a
/// runner handoff parks its clone inside the runner until the terminal transition takes it, so an
/// authority already absent from `open_artifacts` can still pin the use through one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DatabaseRetainedPoolUseCensus {
    capability_open: usize,
    catalog_read: usize,
    catalog_bootstrap: usize,
    create_catalog: usize,
    sync_hello: usize,
    artifact_retirement: usize,
    runner_handoff: usize,
    mount_work: usize,
    live_authority: usize,
    live_capability_open: usize,
    live_catalog_read: usize,
    live_catalog_bootstrap: usize,
    live_create_catalog: usize,
    live_mount_future: usize,
    live_sync_hello: usize,
    mount_wait: usize,
}

impl DatabaseRetainedPoolUseCensus {
    pub fn observe() -> Self {
        Self {
            capability_open: database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            catalog_read: database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            catalog_bootstrap: database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            create_catalog: database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            sync_hello: db_sync::database_sync_hello_live_slots(),
            artifact_retirement: db_artifact::artifact_runner_retirement_live_slots(),
            runner_handoff: db_artifact::artifact_runner_handoff_pool_use_live_slots(),
            mount_work: database_document_mount_work_live_slots(),
            live_authority: db_artifact::artifact_authority_live_pool_uses(),
            live_capability_open: DATABASE_CAPABILITY_OPEN_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_catalog_read: DATABASE_CATALOG_READ_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_catalog_bootstrap: DATABASE_CATALOG_BOOTSTRAP_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_create_catalog: DATABASE_CREATE_CATALOG_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_mount_future: DATABASE_MOUNT_FUTURE_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_sync_hello: db_sync::database_sync_hello_live_states(),
            mount_wait: DATABASE_POOL_USE_MOUNT_WAIT_LIVE.load(std::sync::atomic::Ordering::Acquire),
        }
    }
}

/// 🧭️ The retained owner advanced by one database shutdown step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownPhase {
    Authority,
    Emit,
    PoolUse,
}

/// 🧱️ A live shared owner that prevents destructive shutdown progress.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownBlock {
    Authorities(usize),
    Executor(semio_framework_async::WorkerSubmitErrorKind),
    ArtifactClose(db_artifact::ArtifactCloseRetryProgress),
}

/// 🚦️ One bounded retained database shutdown result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownProgress {
    Progress { phase: DatabaseShutdownPhase, remaining_authorities: usize },
    Blocked(DatabaseShutdownBlock),
    Interrupted,
    Complete,
}

/// 🚪️ A Database-owned retained admission either refuses before it copies any owner, or returns
/// the exact operation-specific rejection that must still be driven to terminal cleanup.
pub enum DatabaseRetainedActivityRejected<R> {
    Closed(DbError),
    Retained(R),
}

impl<R: std::fmt::Debug> std::fmt::Debug for DatabaseRetainedActivityRejected<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(error) => formatter.debug_tuple("Closed").field(error).finish(),
            Self::Retained(rejected) => formatter.debug_tuple("Retained").field(rejected).finish(),
        }
    }
}

/// 🚧️ Document mount rejection retaining any writer acquired by WAL or engine construction.
#[must_use = "document mount rejection must reach terminal retained cleanup"]
pub enum DatabaseDocumentOpenRejected {
    Database(DbError),
    Engine(db_artifact_engine::ArtifactEngineOpenRejected),
}

impl DatabaseDocumentOpenRejected {
    pub fn error(&self) -> &DbError {
        match self {
            Self::Database(error) => error,
            Self::Engine(rejected) => rejected.error(),
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::Database(_) => None,
            Self::Engine(rejected) => rejected.cleanup_error(),
        }
    }

    pub fn has_retained_writer(&self) -> bool {
        matches!(self, Self::Engine(rejected) if rejected.has_retained_writer())
    }

    pub async fn retry_close(self) -> Result<DbError, DatabaseDocumentOpenRejected> {
        match self {
            Self::Database(error) => Ok(error),
            Self::Engine(rejected) => rejected.retry_close().await.map_err(Self::Engine),
        }
    }
}

impl From<DbError> for DatabaseDocumentOpenRejected {
    fn from(error: DbError) -> Self {
        Self::Database(error)
    }
}

impl From<db_artifact_engine::ArtifactEngineOpenRejected> for DatabaseDocumentOpenRejected {
    fn from(rejected: db_artifact_engine::ArtifactEngineOpenRejected) -> Self {
        Self::Engine(rejected)
    }
}

impl std::fmt::Debug for DatabaseDocumentOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseDocumentOpenRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).field("has_retained_writer", &self.has_retained_writer()).finish()
    }
}
//#region 🔖️Database
/// 🗄️ The catalog: owns the storage substrate, the shared config/capabilities/authz/observability
/// wiring every document actor is constructed with, and the registry of currently-open
/// `ArtifactAuthority` actors.
///
/// 🎯️ Design choice: the catalog registry itself is a plain `Mutex`-guarded `HashMap`, not a
/// separate `db_actor::Actor`-driven process. `Database`'s own public surface (`open`/
/// `create_document`/`document`/`catalog`/`health`/`shutdown`) is already synchronous per the
/// frozen contract, and per-document concurrency is already provided by each `ArtifactAuthority`'s
/// own dedicated thread — the catalog only ever needs to serialize document-registry mutations and
/// a catalog-root CAS write, which a `Mutex` does directly without the mailbox's priority-lane/
/// backpressure machinery (that machinery matters for a document's WAL under load, not a rare
/// catalog-root swap).
// 🔀️ `E` is the pluggable `Emit` sink — dedyn-emit-runtime, O1/R11(a): `open_with_emit`'s own doc
// ("a caller-supplied Emit sink, e.g. a `db_observe::WriterSink`") is exactly R11(a)'s "trivially
// generic" shape. Default is `db_observe::StructuredSink<db_observe::MemorySink>` — the concrete
// type `default_emit()` has always constructed — so every unparameterized `Database` reference
// (this crate's own `open`/`open_at`, plus `🌎️hub` and every other external caller, none of which
// ever names this type parameter) names the default. Replaces `Arc<dyn Emit>`.
pub struct Database<E: Emit + 'static = db_observe::StructuredSink<db_observe::MemorySink>> {
    pub storage: Arc<db_storage::DbBackend>,
    pub config: DbConfig,
    pub capabilities: DbCapabilities,
    emit: Arc<E>,
    pub health: Arc<db_observe::HealthRegistry>,
    pub catalog: Arc<Mutex<CatalogState>>,
    pub open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>,
    #[cfg(test)]
    mount_catalog_published_hook: Arc<Mutex<Option<DatabaseDocumentMountPublishedHook>>>,
    #[cfg(test)]
    mount_cleanup_fault_hook: Arc<Mutex<Option<DatabaseDocumentMountCleanupFaultHook>>>,
    #[cfg(test)]
    mount_parked_hook: Arc<Mutex<Option<DatabaseDocumentMountParkedHook>>>,
    closing_authority: Option<(String, Arc<db_artifact::ArtifactAuthority>)>,
    shutdown_emit_started: bool,
    shutdown_complete: bool,
    pub pool_use: Option<Arc<WorkerPoolUse>>,
    /// 🧵️ The process WorkerPool every document authority and submit bridge uses.
    /// Construction without this owner is intentionally impossible: no database path may execute
    /// blocking storage or authority work inline on its caller.
    pub pool: Arc<WorkerPool>,
}

// 🚫️async: E5 executor bridge — every `Database` method below is plain sync and drives its async
// storage/`ArtifactEngine` calls via `db_actor::block_on` (R4 clause 2: this crate's own db-actor
// thread bridges are sanctioned; `Database` is the facade the prior `db-trait-flip` packet already
// classified as thread-owning alongside `db_artifact`, per its report's "db_engine (per-submit
// bridge threads)"). Every `.wal()`/`.snapshot()`/`.catalog()`/`.index()`/`.payload()`/`.lease()`
// accessor call is `.await`ed inside the SAME `block_on`, never a bare synchronous call.
/// 🧲️ Filesystem database-open rejection retaining storage cleanup authority.
#[must_use = "database-open rejection must close its retained storage before exposing the cause"]
pub enum DatabaseOpenAtRejected {
    Storage(db_io_driver::DbStorageOpenRejected),
    Database { cause: DbError, cleanup_error: Option<DbError>, storage: Arc<db_storage::DbBackend> },
}

impl DatabaseOpenAtRejected {
    pub fn error(&self) -> &DbError {
        match self {
            Self::Storage(rejected) => rejected.error(),
            Self::Database { cause, .. } => cause,
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::Storage(rejected) => rejected.cleanup_error(),
            Self::Database { cleanup_error, .. } => cleanup_error.as_ref(),
        }
    }

    pub async fn retry_close(self) -> Result<DbError, Self> {
        match self {
            Self::Storage(rejected) => rejected.retry_close().await.map_err(Self::Storage),
            Self::Database { cause, cleanup_error: _, storage } => {
                let result = match storage.as_ref() {
                    #[cfg(not(target_arch = "wasm32"))]
                    db_storage::DbBackend::Fs(storage) => storage.close().await,
                    _ => Err(DbError::Internal("database open-at rejection lost its filesystem storage".to_string())),
                };
                match result {
                    Ok(()) => Ok(cause),
                    Err(error) => Err(Self::Database { cause, cleanup_error: Some(error), storage }),
                }
            }
        }
    }
}

impl std::fmt::Debug for DatabaseOpenAtRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseOpenAtRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).finish()
    }
}

impl Database {
    /// 🚀️ The frozen entry point: opens (or initializes, if `storage` is fresh) a `Database`
    /// over an arbitrary `Arc<db_storage::DbBackend>` backend, wired with the default in-memory emit sink.
    pub async fn open(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>) -> Result<Database, DbError> {
        Database::open_with(pool, config, storage, default_emit().await).await
    }

    /// 🚀️ The zero-touch filesystem entry point. The caller supplies the process pool
    /// before storage construction, so opening can never take a pool-less inline path.
    pub async fn open_at(pool: Arc<WorkerPool>, root: &std::path::Path, profile: Profile) -> Result<Database, DatabaseOpenAtRejected> {
        let fs = db_storage_fs::FsStorage::open(pool.clone(), root).await.map_err(DatabaseOpenAtRejected::Storage)?;
        let storage: Arc<db_storage::DbBackend> = Arc::new(db_storage::DbBackend::Fs(fs));
        match Database::open(pool, DbConfig::for_profile(profile), storage.clone()).await {
            Ok(database) => Ok(database),
            Err(cause) => Err(DatabaseOpenAtRejected::Database { cause, cleanup_error: None, storage }),
        }
    }

    /// 🚀️ Like `open`, but with a caller-supplied `Emit` sink (e.g. a `db_observe::WriterSink`
    /// over a real file) instead of the default in-memory one.
    // 🔀️ dedyn-emit-runtime, O1/R11(a): generic over `E: Emit` (the function's own type param, not
    // `Database`'s default) so the returned `Database<E>` carries the caller's concrete sink type —
    // this fn has zero callers anywhere in the repo today (public, documented extension seam per
    // `open_with_emit`'s own doc).
    pub async fn open_with_emit<E: Emit + 'static>(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<E>, DbError> {
        Database::open_with(pool, config, storage, emit).await
    }
}

// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block is generic over `E: Emit`; Rust resolves `open`'s `Database::open_with(..)` call by inferring
// `E` from `default_emit()`'s concrete return type.
impl<E: Emit + 'static> Database<E> {
    fn require_open_use(&self) -> Result<Arc<WorkerPoolUse>, DbError> {
        if self.shutdown_complete {
            return Err(DbError::Closed);
        }
        self.pool_use.clone().ok_or(DbError::Closed)
    }

    /// 🧵️ Admits the exact storage owner before probing one backend capability future on I/O.
    pub fn open_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenRejected> {
        DatabaseCapabilityOpenFuture::try_submit(pool, storage)
    }

    /// 🗂️ Admits the exact storage/root-key owners before one catalog backend read on I/O.
    pub fn open_catalog_read_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCatalogReadFuture, DatabaseCatalogReadRejected> {
        DatabaseCatalogReadFuture::try_submit(pool, storage, DatabaseCatalogRootKey::root())
    }

    /// 🌱️ Admits the exact storage/catalog-page/fence owners before the initial CAS reaches I/O.
    pub fn open_catalog_bootstrap_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages) -> Result<DatabaseCatalogBootstrapFuture, DatabaseCatalogBootstrapRejected> {
        DatabaseCatalogBootstrapFuture::try_submit(pool, storage, pages, EpochFence::INITIAL)
    }

    async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<E>, DbError> {
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("database WorkerPool use rejected: {error:?}")))?;
        let capability_probe = match DatabaseCapabilityOpenFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, true) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error()),
        };
        let (storage, storage_capabilities) = capability_probe.await?.into_parts();
        let capabilities = DbCapabilities {
            // 🧩️ Extension seam: real, honest today — see module doc on why preview/live-query
            // aren't reachable through `ArtifactAuthority`'s current mailbox surface, and why
            // `db_cluster` is still an empty stub upstream of this wave.
            preview: false,
            historical_query: true,
            live_query: false,
            cluster: false,
            max_durability: std::cmp::min(storage_capabilities.max_durability, config.capabilities.max_durability),
        };

        let health = Arc::new(db_observe::HealthRegistry::new());
        health.set("db_engine.storage", if storage_capabilities.durable { db_observe::HealthState::Healthy } else { db_observe::HealthState::Degraded("storage backend is not durable".to_string()) });

        let catalog_probe = match DatabaseCatalogReadFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, DatabaseCatalogRootKey::root(), true) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error(pool.clone())),
        };
        let (storage, _catalog_key, catalog_root) = catalog_probe.await?.into_parts();
        let (storage, epoch, entries) = match catalog_root? {
            Some((bytes, epoch)) => {
                let prepared = db_io_registry::db_io_prepare_platform(&bytes)?.await?;
                (storage, epoch, decode_catalog(prepared.as_slice()).await?)
            }
            None => {
                let pages = encode_catalog_pages(&[]).await?;
                let bootstrap = match DatabaseCatalogBootstrapFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, pages, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL, true) {
                    Ok(bootstrap) => bootstrap,
                    Err(rejected) => return Err(rejected.close_and_take_error()),
                };
                let result = bootstrap.await?;
                let (retained_storage, _catalog_key, expected, actual) = match result.into_parts() {
                    Ok(parts) => parts,
                    Err(owner) => {
                        drop(owner);
                        return Err(DbError::Internal("database catalog-bootstrap result was incomplete".to_string()));
                    }
                };
                if expected != EpochFence::INITIAL {
                    return Err(DbError::Internal("database catalog-bootstrap returned the wrong expected fence".to_string()));
                }
                let epoch = actual?;
                (retained_storage, epoch, Vec::new())
            }
        };
        health.set("db_engine.catalog", db_observe::HealthState::Healthy);

        emit.emit(EmitEvent::new("db_engine.database_opened").field("documents", EmitField::U64(entries.len() as u64))).await;

        Ok(Database {
            storage,
            config,
            capabilities,
            emit,
            health,
            catalog: Arc::new(Mutex::new(CatalogState { epoch, revision: 1, entries: Arc::new(entries), pending: None })),
            open_artifacts: Arc::new(Mutex::new(DatabaseDocumentMountRegistry::new())),
            #[cfg(test)]
            mount_catalog_published_hook: Arc::new(Mutex::new(None)),
            #[cfg(test)]
            mount_cleanup_fault_hook: Arc::new(Mutex::new(None)),
            #[cfg(test)]
            mount_parked_hook: Arc::new(Mutex::new(None)),
            closing_authority: None,
            shutdown_emit_started: false,
            shutdown_complete: false,
            pool_use: Some(pool_use),
            pool,
        })
    }

    /// ⚙️ Builds one `ArtifactEngineConfig`: this database's own `limits` and preview TTL, and
    /// `..db_artifact::ArtifactEngineConfig::default()` for every other field (`security`/`emit`/
    /// `projections`) — this crate has no opinion on those (`db_artifact`'s own real
    /// `db_security::SecurityGate`-backed default policy is permissive single-tenant), and the spread keeps
    /// this call site correct across further additive growth without another coordinated edit.
    fn document_engine_config(&self) -> db_artifact_engine::ArtifactEngineConfig {
        db_artifact_engine::ArtifactEngineConfig { limits: self.config.limits.clone(), preview_ttl_ms: self.config.limits.max_preview_ttl_ms, ..db_artifact_engine::ArtifactEngineConfig::default() }
    }

    fn retained_mount_rejection(rejected: db_artifact_engine::ArtifactEngineOpenRejected, resume: Option<DatabaseDocumentMountFuture>) -> DatabaseDocumentMountFailure {
        DatabaseDocumentMountFailure::Retained { rejected, resume }
    }

    async fn refresh_catalog_document(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, catalog: Arc<Mutex<CatalogState>>, document: &protocol::ArtifactId) -> Result<bool, DbError> {
        let read = match DatabaseCatalogReadFuture::try_submit(pool.clone(), storage, DatabaseCatalogRootKey::root()) {
            Ok(read) => read,
            Err(rejected) => return Err(rejected.close_and_take_error(pool)),
        };
        let result = read.await?;
        let (_storage, _key, root) = result.into_parts();
        let Some((bytes, epoch)) = root? else { return Ok(false) };
        let prepared = db_io_registry::db_io_prepare_platform(&bytes)?.await?;
        let entries = decode_catalog(prepared.as_slice()).await?;
        let known = entries.iter().any(|entry| entry.document == *document);
        let mut state = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if epoch.epoch >= state.epoch.epoch {
            state.epoch = epoch;
            state.revision = state.revision.checked_add(1).ok_or(DbError::LimitExceeded("database catalog refresh revision"))?;
            state.entries = Arc::new(entries);
            state.pending = None;
        }
        Ok(known)
    }

    async fn publish_mount_catalog(_live: DatabaseMountFutureLiveGuardV1, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, storage: Arc<db_storage::DbBackend>, catalog: Arc<Mutex<CatalogState>>, document: protocol::ArtifactId) -> Result<(), DbError> {
        let transaction = match DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use, catalog, storage, document, true) {
            Ok(transaction) => transaction,
            Err(rejected) => return Err(rejected.close_and_take_error()),
        };
        let result = transaction.await?;
        let (_storage, _document, _expected, actual) = result.into_parts().map_err(|_| DbError::LimitExceeded("database create-catalog result owner"))?;
        actual.map(|_| ())
    }

    async fn run_open_document_mount(
        _live: DatabaseMountFutureLiveGuardV1,
        pool: Arc<WorkerPool>,
        pool_use: Arc<WorkerPoolUse>,
        storage: Arc<db_storage::DbBackend>,
        document: protocol::ArtifactId,
        config: db_artifact_engine::ArtifactEngineConfig,
        mailbox_capacities: MailboxCapacities,
        emit: Arc<E>,
    ) -> Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure> {
        let open_document = document.clone();
        let opened_at_ms = now_ms().await;
        let authority = match db_artifact::ArtifactAuthority::spawn_with_pool_use(
            pool,
            pool_use,
            move || async move { db_artifact_engine::ArtifactEngine::open_retained(open_document, storage, config, opened_at_ms).await.map(|(engine, _)| Box::new(engine)) },
            mailbox_capacities,
        )
        .await
        {
            Ok(authority) => authority,
            Err(rejected) => return Err(Self::retained_mount_rejection(rejected, None)),
        };
        emit.emit(EmitEvent::new("db_engine.document_opened").with_document(to_core_document_id(&document).await)).await;
        Ok(DatabaseDocumentMountReply { authority: Arc::new(authority), handles: Arc::new(std::sync::atomic::AtomicUsize::new(0)) })
    }

    async fn run_document_mount(
        _live: DatabaseMountFutureLiveGuardV1,
        predecessor: Option<db_actor::ReplyReceiver<()>>,
        pool: Arc<WorkerPool>,
        pool_use: Arc<WorkerPoolUse>,
        storage: Arc<db_storage::DbBackend>,
        catalog: Arc<Mutex<CatalogState>>,
        document: protocol::ArtifactId,
        policy: DatabaseDocumentMountPolicy,
        catalog_known: bool,
        create_config: db_artifact_engine::ArtifactEngineConfig,
        open_config: db_artifact_engine::ArtifactEngineConfig,
        mailbox_capacities: MailboxCapacities,
        emit: Arc<E>,
        #[cfg(test)] mount_catalog_published_hook: Arc<Mutex<Option<DatabaseDocumentMountPublishedHook>>>,
    ) -> Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure> {
        if let Some(predecessor) = predecessor {
            let _ = predecessor.await;
        }
        let mut create = !catalog_known && policy != DatabaseDocumentMountPolicy::Open;
        if create {
            if let Err(error) = Self::publish_mount_catalog(DatabaseMountFutureLiveGuardV1::new(), pool.clone(), pool_use.clone(), storage.clone(), catalog.clone(), document.clone()).await {
                if policy != DatabaseDocumentMountPolicy::Ensure || !matches!(error, DbError::AlreadyExists(_) | DbError::Fenced { .. }) {
                    return Err(error.into());
                }
                if !Self::refresh_catalog_document(pool.clone(), storage.clone(), catalog.clone(), &document).await? {
                    return Err(error.into());
                }
                create = false;
            }
            #[cfg(test)]
            if create {
                if let Some(hook) = mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    hook(&document);
                }
            }
        }

        let authority = if create {
            let create_pool = pool.clone();
            let create_storage = storage.clone();
            let create_document = document.clone();
            let created_at_ms = now_ms().await;
            match db_artifact::ArtifactAuthority::spawn_with_pool_use(create_pool, pool_use.clone(), move || async move { db_artifact_engine::ArtifactEngine::create_retained(create_document, create_storage, create_config, created_at_ms).await.map(Box::new) }, mailbox_capacities).await {
                Ok(authority) => authority,
                Err(rejected) => {
                    let retry_open = policy == DatabaseDocumentMountPolicy::Ensure && matches!(rejected.error(), DbError::AlreadyExists(_));
                    let resume = retry_open.then(|| Box::pin(Self::run_open_document_mount(DatabaseMountFutureLiveGuardV1::new(), pool.clone(), pool_use.clone(), storage.clone(), document.clone(), open_config, mailbox_capacities, emit.clone())) as DatabaseDocumentMountFuture);
                    return Err(Self::retained_mount_rejection(rejected, resume));
                }
            }
        } else {
            return Self::run_open_document_mount(DatabaseMountFutureLiveGuardV1::new(), pool, pool_use, storage, document, open_config, mailbox_capacities, emit).await;
        };
        emit.emit(EmitEvent::new("db_engine.document_created").with_document(to_core_document_id(&document).await)).await;
        Ok(DatabaseDocumentMountReply { authority: Arc::new(authority), handles: Arc::new(std::sync::atomic::AtomicUsize::new(0)) })
    }

    /// 🔁️ Mounts until a handle is adopted. A mount whose document went idle and started closing
    /// between its reply and its adoption mounts again, then as an open of the document it created.
    async fn mount_document(&self, document: protocol::ArtifactId, policy: DatabaseDocumentMountPolicy) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        let mut policy = policy;
        loop {
            if let Some(handle) = self.mount_document_once(document.clone(), policy).await? {
                return Ok(handle);
            }
            if policy == DatabaseDocumentMountPolicy::Create {
                policy = DatabaseDocumentMountPolicy::Open;
            }
        }
    }

    fn adopt_mount(&self, document: &protocol::ArtifactId, handles: &Arc<std::sync::atomic::AtomicUsize>, registry: &DatabaseDocumentMountRegistry) -> Option<ArtifactHandleLease> {
        match registry.slots.get(&document.0) {
            Some(DatabaseDocumentMountSlot::Ready(ready)) if Arc::ptr_eq(&ready.handles, handles) => Some(ArtifactHandleLease::new(handles.clone(), Arc::downgrade(&self.open_artifacts), document.0.clone())),
            _ => None,
        }
    }

    async fn mount_document_once(&self, document: protocol::ArtifactId, policy: DatabaseDocumentMountPolicy) -> Result<Option<ArtifactHandle>, DatabaseDocumentOpenRejected> {
        let pool_use = self.require_open_use()?;
        let _pool_use_site = DatabasePoolUseSiteGuardV1::new(&DATABASE_POOL_USE_MOUNT_WAIT_LIVE);
        let catalog_known = self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.iter().any(|entry| entry.document == document);
        let create_config = self.document_engine_config();
        let open_config = self.document_engine_config();
        let (reply, owner) = {
            let mut registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            if let Some(DatabaseDocumentMountSlot::Ready(completion)) = registry.slots.get(&document.0) {
                if policy == DatabaseDocumentMountPolicy::Create {
                    return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)).into());
                }
                let completion = completion.clone();
                let lease = self.adopt_mount(&document, &completion.handles, &registry);
                drop(registry);
                return Ok(lease.map(|lease| ArtifactHandle { authority: completion.authority, _lease: lease, document, pool: self.pool.clone() }));
            }
            if let Some(DatabaseDocumentMountSlot::Opening { generation, owner, waiters }) = registry.slots.get_mut(&document.0) {
                let Some(slot) = waiters.iter().position(Option::is_none) else {
                    return Err(DbError::Unavailable("database document-mount waiter capacity exhausted".to_string()).into());
                };
                let generation = *generation;
                let (reply, raw_receiver) = db_actor::oneshot();
                waiters[slot] = Some(DatabaseDocumentMountWaiter { policy, create_claim: false, reply });
                (DatabaseDocumentMountWait { receiver: raw_receiver, registry: Arc::downgrade(&self.open_artifacts), document: document.0.clone(), generation, slot, resolved: false }, Some(owner.clone()))
            } else {
                if policy == DatabaseDocumentMountPolicy::Create && catalog_known {
                    return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)).into());
                }
                if policy == DatabaseDocumentMountPolicy::Open && !catalog_known {
                    return Err(DbError::NotFound(format!("document {} not found", document.0)).into());
                }
                let generation = registry.take_generation()?;
                let predecessor = match registry.slots.remove(&document.0) {
                    Some(DatabaseDocumentMountSlot::Closing { terminal }) => Some(terminal),
                    Some(other) => {
                        registry.slots.insert(document.0.clone(), other);
                        return Err(DbError::Internal("database document-mount slot changed under its lock".to_string()).into());
                    }
                    None => None,
                };
                let (reply, raw_receiver) = db_actor::oneshot();
                let future = Box::pin(Self::run_document_mount(
                    DatabaseMountFutureLiveGuardV1::new(),
                    predecessor,
                    self.pool.clone(),
                    pool_use,
                    self.storage.clone(),
                    self.catalog.clone(),
                    document.clone(),
                    policy,
                    catalog_known,
                    create_config,
                    open_config,
                    self.config.mailbox_capacities,
                    self.emit.clone(),
                    #[cfg(test)]
                    self.mount_catalog_published_hook.clone(),
                ));
                let owner = DatabaseDocumentMountOwner::new(
                    self.pool.clone(),
                    Arc::downgrade(&self.open_artifacts),
                    document.0.clone(),
                    generation,
                    future,
                    #[cfg(test)]
                    self.mount_cleanup_fault_hook.clone(),
                    #[cfg(test)]
                    self.mount_parked_hook.clone(),
                );
                let mut waiters = std::array::from_fn(|_| None);
                waiters[0] = Some(DatabaseDocumentMountWaiter { policy, create_claim: policy == DatabaseDocumentMountPolicy::Create, reply });
                registry.slots.insert(document.0.clone(), DatabaseDocumentMountSlot::Opening { generation, owner: owner.clone(), waiters });
                (DatabaseDocumentMountWait { receiver: raw_receiver, registry: Arc::downgrade(&self.open_artifacts), document: document.0.clone(), generation, slot: 0, resolved: false }, Some(owner))
            }
        };
        if let Some(owner) = owner {
            owner.request_drive(false);
        }
        let result = reply.await.map_err(DatabaseDocumentOpenRejected::Database)?;
        let result = result.map_err(DatabaseDocumentOpenRejected::Database)?;
        let lease = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            self.adopt_mount(&document, &result.handles, &registry)
        };
        Ok(lease.map(|lease| ArtifactHandle { authority: result.authority, _lease: lease, document, pool: self.pool.clone() }))
    }

    /// 🪴️ Admits the exact create-document catalog transaction before any catalog owner is copied.
    pub fn create_document_catalog_retained(&self, document: protocol::ArtifactId) -> Result<DatabaseCreateCatalogFuture, DatabaseRetainedActivityRejected<DatabaseCreateCatalogRejected>> {
        let pool_use = self.require_open_use().map_err(DatabaseRetainedActivityRejected::Closed)?;
        DatabaseCreateCatalogFuture::try_prepare_with_use(self.pool.clone(), pool_use, self.catalog.clone(), self.storage.clone(), document, true).map_err(DatabaseRetainedActivityRejected::Retained)
    }

    /// 🌱️ The frozen `create_document`: mints a brand-new document, durably records it in the
    /// catalog root (CAS-fenced), spawns its `ArtifactAuthority`, and returns a live handle.
    pub async fn create_document(&self, spec: ArtifactSpec) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(spec.document, DatabaseDocumentMountPolicy::Create).await
    }

    /// 🪢️ Joins or installs the exact retained first-mount owner for one document.
    pub async fn ensure_document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(id.clone(), DatabaseDocumentMountPolicy::Ensure).await
    }

    /// 📄️ The frozen `document`: returns a live handle to an already-cataloged document,
    /// reusing an already-open `ArtifactAuthority` if one exists, else recovering it fresh from its
    /// WAL.
    pub async fn document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(id.clone(), DatabaseDocumentMountPolicy::Open).await
    }

    /// 📇️ The frozen `catalog`: a point-in-time read of every document this `Database`
    /// knows about.
    pub async fn catalog(&self) -> CatalogView {
        let entries = {
            let catalog = self.catalog.lock().expect("db_engine: catalog mutex poisoned");
            Arc::clone(&catalog.entries)
        };
        CatalogView { artifacts: entries.as_ref().clone() }
    }

    /// 🩺️ The frozen `health`: this `Database`'s `HealthRegistry` snapshot plus its own open
    /// document count.
    pub async fn health(&self) -> DbHealth {
        DbHealth { report: self.health.report(), open_artifacts: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").ready_count() }
    }

    /// 🚪️ Advances at most one retained shutdown owner. Cancellation, deadline, and every error
    /// return leave the exact authority mounted on this Database for retry.
    pub async fn shutdown_step(&mut self, control: &DatabaseShutdownControl) -> Result<DatabaseShutdownProgress, DbError> {
        if self.shutdown_complete {
            return Ok(DatabaseShutdownProgress::Complete);
        }
        if control.interrupted() {
            return Ok(DatabaseShutdownProgress::Interrupted);
        }
        if let Some((_, authority)) = self.closing_authority.as_ref() {
            if !authority.shutdown_step() {
                if let Some(progress) = authority.close_retry_progress().filter(db_artifact::ArtifactCloseRetryProgress::is_blocked) {
                    return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)));
                }
                return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() + 1 });
            }
            self.closing_authority.take();
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let closing = {
            let mut registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            let document = registry.slots.iter().find_map(|(document, slot)| matches!(slot, DatabaseDocumentMountSlot::Closing { .. }).then(|| document.clone()));
            document.and_then(|document| registry.slots.remove(&document).map(|slot| (document, slot)))
        };
        if let Some((document, slot)) = closing {
            let DatabaseDocumentMountSlot::Closing { mut terminal } = slot else { return Err(DbError::Internal("selected closing mount was not closing".to_string())) };
            let retired = std::future::poll_fn(|context| std::task::Poll::Ready(std::pin::Pin::new(&mut terminal).poll(context).is_ready())).await;
            if !retired {
                self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").slots.insert(document, DatabaseDocumentMountSlot::Closing { terminal });
                semio_framework_async::yield_once().await;
            }
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let opening = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            registry.slots.values().find_map(|slot| match slot {
                DatabaseDocumentMountSlot::Opening { owner, .. } => Some(owner.clone()),
                DatabaseDocumentMountSlot::Ready(_) | DatabaseDocumentMountSlot::Closing { .. } => None,
            })
        };
        if let Some(owner) = opening {
            if let Some(kind) = owner.scheduler_fault() {
                return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Executor(kind)));
            }
            owner.drive();
            semio_framework_async::yield_once().await;
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let next = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            registry
                .slots
                .iter()
                .filter_map(|(document, slot)| match slot {
                    DatabaseDocumentMountSlot::Ready(completion) if completion.handles.load(std::sync::atomic::Ordering::Acquire) == 0 => Some(document.clone()),
                    _ => None,
                })
                .min()
        };
        if let Some(document) = next {
            let slot = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").slots.remove(&document).expect("selected shutdown authority disappeared");
            let DatabaseDocumentMountSlot::Ready(completion) = slot else { return Err(DbError::Internal("selected shutdown mount was not ready".to_string())) };
            self.closing_authority = Some((document, completion.authority));
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() + 1 });
        }
        let shared_authorities = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len();
        if shared_authorities != 0 {
            return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(shared_authorities)));
        }
        if !self.shutdown_emit_started {
            self.shutdown_emit_started = true;
            self.emit.emit(EmitEvent::new("db_engine.database_shutdown")).await;
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Emit, remaining_authorities: 0 });
        }
        if self.pool_use.as_ref().is_some_and(|pool_use| Arc::strong_count(pool_use) != 1) {
            if let Some(progress) = db_artifact::artifact_close_retry_progress(&self.pool).filter(db_artifact::ArtifactCloseRetryProgress::is_blocked) {
                return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)));
            }
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::PoolUse, remaining_authorities: 0 });
        }
        self.pool_use.take();
        self.shutdown_complete = true;
        Ok(DatabaseShutdownProgress::Complete)
    }

    /// 🔂️ Grants every faulted artifact close this Database still retires (its closing authority and the
    /// dropped authorities retiring on its pool) a fresh bounded retry budget.
    pub fn readmit_artifact_close_retries(&self) -> usize {
        let closing = self.closing_authority.as_ref().is_some_and(|(_, authority)| authority.readmit_close_retry());
        usize::from(closing) + db_artifact::artifact_close_retry_readmit(&self.pool)
    }

    /// 🛑️ Stops the automatic retries of every faulted artifact close this Database still retires.
    pub fn cancel_artifact_close_retries(&self) -> usize {
        let closing = self.closing_authority.as_ref().is_some_and(|(_, authority)| authority.cancel_close_retry());
        usize::from(closing) + db_artifact::artifact_close_retry_cancel(&self.pool)
    }

    /// 🛬️ Drives bounded shutdown steps to a terminal acknowledgement while borrowing the exact
    /// Database. Error and future cancellation therefore preserve caller retry authority. Each call
    /// is the product retry owner of faulted artifact closes: it re-admits exhausted or cancelled
    /// ones once, lets the pool timer drive their bounded backoff, reports an exhausted budget as a
    /// terminal error and cancels the retries when the caller cancels.
    pub async fn shutdown(&mut self, control: &DatabaseShutdownControl) -> Result<(), DbError> {
        self.readmit_artifact_close_retries();
        let mut reached = None;
        loop {
            match self.shutdown_step(control).await? {
                DatabaseShutdownProgress::Progress { phase, .. } => {
                    reached = Some(phase);
                    semio_framework_async::yield_once().await;
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(count)) => {
                    return Err(DbError::Conflict(format!("database shutdown retains {count} shared artifact authorities")));
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Executor(kind)) => {
                    return Err(DbError::Unavailable(format!("database shutdown retains a non-runnable document-mount job: {kind:?}")));
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)) => {
                    return Err(DbError::Unavailable(format!("database shutdown retains a faulted artifact close after {} of {} retries ({:?})", progress.attempts, progress.limit, progress.state)));
                }
                DatabaseShutdownProgress::Interrupted => {
                    if control.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                        self.cancel_artifact_close_retries();
                    }
                    return Err(control.interruption_error(DatabaseShutdownInterruptionWitness {
                        phase: reached,
                        open_artifacts: self.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
                        emit_started: self.shutdown_emit_started,
                        pool_use_owners: self.pool_use.as_ref().map_or(0, Arc::strong_count),
                        retained_slots: DatabaseRetainedPoolUseCensus::observe(),
                    }));
                }
                DatabaseShutdownProgress::Complete => return Ok(()),
            }
        }
    }

    /// 🧰️ What this `Database` instance negotiated at `open` time.
    pub async fn capabilities(&self) -> DbCapabilities {
        self.capabilities
    }

    /// 🔌️ The underlying storage substrate this `Database` was opened with — an escape
    /// hatch for callers below the document-actor boundary that need direct `PayloadStorage`/
    /// `WalStorage` access (e.g. `os-semio_hub`'s content-addressed blob routes, or a wire-v2 semio_hub
    /// session driving `db_sync::handle_frontier_advertise` directly). Additive: not part of the
    /// contract-frozen `Database` API surface listed in `contract.md`'s "Stable API" block, so it
    /// carries no compatibility promise beyond this crate's own semver.
    pub async fn storage(&self) -> Arc<db_storage::DbBackend> {
        self.storage.clone()
    }

    /// 🧵️ Queues compaction on the document actor that already owns its WAL writer.
    pub async fn compact_document_retained(
        &self,
        document: &protocol::ArtifactId,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<ArtifactCompactionFuture, DatabaseDocumentOpenRejected> {
        Ok(self.document(document).await?.compact_retained(holder, consolidate_snapshots, db_compact::CompactionBudget::default(), now_ms, cancelled))
    }

    /// 🧹️ Mounts one retained compaction authority and awaits only its terminal witness.
    pub async fn compact_document(&self, document: &protocol::ArtifactId, holder: &str, consolidate_snapshots: bool) -> Result<db_compact::CompactionReport, DatabaseDocumentOpenRejected> {
        let holder = db_io_core::DbIoText::try_from_str(holder)?;
        let requested_at_ms = now_ms().await;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let result = self.compact_document_retained(document, holder, consolidate_snapshots, requested_at_ms, cancelled).await?.await.map_err(DatabaseDocumentOpenRejected::Database)?;
        result.map_err(DatabaseDocumentOpenRejected::Database)
    }

    /// 👋️ Pre-admits the exact sync-hello owners before mounting shared-pool I/O work.
    pub fn hello_retained(
        &self,
        document: ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<db_sync::DatabaseSyncHelloFuture, DatabaseRetainedActivityRejected<db_sync::DatabaseSyncHelloRejected>> {
        let pool_use = self.require_open_use().map_err(DatabaseRetainedActivityRejected::Closed)?;
        db_sync::DatabaseSyncHelloFuture::try_submit_with_use(self.pool.clone(), pool_use, self.storage.clone(), document, hello_frontier, session_id, origin, snapshot_chunk_bytes).map_err(DatabaseRetainedActivityRejected::Retained)
    }

    /// 📡️ Mounts one retained hello authority and returns its backpressured frame session. A hello
    /// that finds every admission slot taken waits for one (at most [`DATABASE_HELLO_ADMISSION_WAIT_MS`])
    /// instead of refusing its socket; any other refusal is answered at once.
    pub async fn hello(
        &self,
        document: protocol::ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<db_sync::DatabaseSyncHelloSession, DbError> {
        let document = to_core_document_id(&document).await;
        let deadline_ms = self.pool.now_ms().saturating_add(DATABASE_HELLO_ADMISSION_WAIT_MS);
        loop {
            let hello = match self.hello_retained(document.clone(), hello_frontier.clone(), session_id.clone(), origin.clone(), snapshot_chunk_bytes) {
                Ok(hello) => hello,
                Err(DatabaseRetainedActivityRejected::Closed(error)) => return Err(error),
                Err(DatabaseRetainedActivityRejected::Retained(rejected)) => {
                    let saturated = rejected.admission_saturated();
                    let error = rejected.close_and_take_error();
                    if !saturated {
                        return Err(error);
                    }
                    db_sync::DatabaseSyncHelloAdmissionReady::new(self.pool.clone(), deadline_ms).await?;
                    continue;
                }
            };
            return hello.await?.close_and_take_session();
        }
    }
}

/// ⏳️ How long [`Database::hello`] waits for a sync-hello admission slot before it answers the
/// refusal: the hub's document-socket frame deadline.
pub const DATABASE_HELLO_ADMISSION_WAIT_MS: u64 = 30_000;
//#endregion 🔖️Database

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/⏱️throughput/🦀️.rs"]
mod throughput_tests;
//#endregion 🧪️Tests
