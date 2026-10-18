//! 🐘️ Postgres storage backend: the public postgres storage, its drop, the document, output, unit, page and list helpers, the storage trait implementations and the backend capabilities. See [`super`].

use super::*;

/// 🐘️ Typed PostgreSQL facade; only the registered executor owns the external driver.
pub struct PostgresStorage {
    pub control: DbIoBackendControl,
    pub(super) worker_pool: Arc<WorkerPool>,
    pub(super) closed: std::sync::atomic::AtomicBool,
}

impl PostgresStorage {
    pub async fn connect(worker_pool: Arc<WorkerPool>, database_url: &str) -> Result<Self, DbStorageOpenRejected> {
        db_io_driver::open_db_io_backend_admitted(&worker_pool, || Self::connect_once(worker_pool.clone(), database_url)).await
    }

    /// 🎯️ One PostgreSQL backend open attempt, refused at once when the backend capacity is taken.
    async fn connect_once(worker_pool: Arc<WorkerPool>, database_url: &str) -> Result<Self, DbStorageOpenRejected> {
        let database_url = DbIoText::try_from_str(database_url)?;
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = worker_pool.acquire_use().map_err(|error| DbError::Unavailable(format!("PostgreSQL DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let executor = Box::new(PostgresDbIoExecutor::new(database_url.clone())?);
        let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Postgres, executor, worker_pool.clone(), pool_use, rollback)?;
        let storage = Self { control, worker_pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = storage.execute(DbIoTask::BackendOpen { backend: control, path: database_url }).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    pub(super) async fn execute(&self, task: DbIoTask) -> Result<DbIoResult, DbError> {
        let mut operation = submit_db_io_task_admitted(task).await?;
        operation.start_async_native_on_lane_io().await?;
        operation.finish().await
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = postgres_unit(self.execute(DbIoTask::BackendClose { backend: self.control }).await?);
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }
}

impl Drop for PostgresStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

fn postgres_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn postgres_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("PostgreSQL output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn postgres_unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-unit result".to_string())),
    }
}

fn postgres_pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(pages) => Ok(pages),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-page result".to_string())),
    }
}

fn postgres_list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(list) => Ok(list),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-list result".to_string())),
    }
}

impl WalStorage for PostgresStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_io_core::WalWriterPermit, DbError> {
        let document = postgres_document(document)?;
        db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
            match self.execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await? {
                DbIoResult::WalWriter(writer) => Ok(writer),
                _ => Err(DbError::Internal("remote WAL writer result taxonomy".to_string())),
            }
        })
        .await
    }
    async fn create_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
    async fn append(&self, writer: &db_io_core::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
    async fn sync(&self, writer: &db_io_core::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
    }
    async fn seal(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::WalRead { backend: self.control, document: postgres_document(document)?, index, range, output: postgres_output(range.len)? }).await?)
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalLength { backend: self.control, document: postgres_document(document)?, index }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match self.execute(DbIoTask::WalState { backend: self.control, document: postgres_document(document)?, index }).await? {
            DbIoResult::WalSegmentState(state) => Ok(state),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-WAL-state result".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::WalList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn truncate_tail(&self, writer: &db_io_core::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await?)
    }
    async fn delete_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
}

impl SnapshotStorage for PostgresStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::SnapshotWrite { backend: self.control, document: postgres_document(document)?, generation, input: bytes }).await?)
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::SnapshotRead { backend: self.control, document: postgres_document(document)?, generation, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self.execute(DbIoTask::SnapshotLatest { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(generation) => Ok(generation),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-generation result".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::SnapshotList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::SnapshotDelete { backend: self.control, document: postgres_document(document)?, generation }).await?)
    }
}

impl PayloadStorage for PostgresStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self.execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-hash result".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self.execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(exists) => Ok(exists),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-exists result".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self.execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
}

impl CatalogStorage for PostgresStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self.execute(DbIoTask::CatalogRead { backend: self.control, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(catalog) => Ok(catalog),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-catalog result".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-fence result".to_string())),
        }
    }
}

impl IndexStorage for PostgresStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::IndexWrite { backend: self.control, document: postgres_document(document)?, run_id, input: bytes }).await?)
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::IndexRead { backend: self.control, document: postgres_document(document)?, run_id, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::IndexList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::IndexDelete { backend: self.control, document: postgres_document(document)?, run_id }).await?)
    }
}

impl LeaseStorage for PostgresStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-fence result".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self.execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(lease) => Ok(lease),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-lease result".to_string())),
        }
    }
}

//#region 🔖️DbBackend
/// 🎚️ `PostgresStorage`'s fixed capability set — extracted to a free fn so the unit tests
/// below can assert on it without opening a real connection.
pub(super) fn postgres_capabilities() -> StorageCapabilities {
    StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
}

impl PostgresStorage {
    /// 🎚️ What this backend actually supports — see [`postgres_capabilities`].
    pub async fn capabilities(&self) -> StorageCapabilities {
        postgres_capabilities()
    }
}
//#endregion 🔖️DbBackend
