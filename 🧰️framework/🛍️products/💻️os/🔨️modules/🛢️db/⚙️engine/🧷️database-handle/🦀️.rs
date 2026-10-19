//! 🧷️ Database handle: the database, the open-at rejection and the handle operations that do not depend on the emit runtime. See [`super`].

use super::*;

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
    pub(super) emit: Arc<E>,
    pub health: Arc<db_observe::HealthRegistry>,
    pub catalog: Arc<Mutex<CatalogState>>,
    pub open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>,
    #[cfg(test)]
    pub(super) mount_catalog_published_hook: Arc<Mutex<Option<DatabaseDocumentMountPublishedHook>>>,
    #[cfg(test)]
    pub(super) mount_cleanup_fault_hook: Arc<Mutex<Option<DatabaseDocumentMountCleanupFaultHook>>>,
    #[cfg(test)]
    pub(super) mount_parked_hook: Arc<Mutex<Option<DatabaseDocumentMountParkedHook>>>,
    pub(super) closing_authority: Option<(String, Arc<db_artifact::ArtifactAuthority>)>,
    pub(super) shutdown_emit_started: bool,
    pub(super) shutdown_complete: bool,
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
    pub async fn open_with_emit<E: Emit + 'static>(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<E>, DbError> {
        Database::open_with(pool, config, storage, emit).await
    }
}
//#endregion 🔖️Database
