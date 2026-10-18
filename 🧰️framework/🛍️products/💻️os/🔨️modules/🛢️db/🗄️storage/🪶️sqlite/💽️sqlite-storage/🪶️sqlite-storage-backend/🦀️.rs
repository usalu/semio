//! 🪶️ Sqlite storage backend: the public sqlite storage, the task helpers, its drop and the storage trait implementations. See [`super`].

use super::*;

//#region 🔖️Facade
pub struct SqliteStorage {
    pub(super) control: DbIoBackendControl,
    pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

pub(super) async fn execute(task: DbIoTask) -> Result<DbIoResult, DbError> {
    submit_db_io_task_admitted(task).await?.finish().await
}

fn output_writer(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("sqlite output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn document_text(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn result_fault(expected: &'static str) -> DbError {
    DbError::Internal(format!("SQLite executor did not return {expected}"))
}

fn unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(result_fault("unit")),
    }
}

pub(super) fn length(result: DbIoResult) -> Result<u64, DbError> {
    match result {
        DbIoResult::Length(value) => Ok(value),
        _ => Err(result_fault("length")),
    }
}

pub(super) fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(value) => Ok(value),
        _ => Err(result_fault("pages")),
    }
}

fn list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(value) => Ok(value),
        _ => Err(result_fault("list")),
    }
}

fn wal_writer(result: DbIoResult) -> Result<WalWriterPermit, DbError> {
    match result {
        DbIoResult::WalWriter(writer) => Ok(writer),
        _ => Err(result_fault("WAL writer")),
    }
}

impl SqliteStorage {
    async fn open_owned(pool: Arc<WorkerPool>, path: DbIoText, in_memory: bool) -> Result<Self, DbStorageOpenRejected> {
        db_io_driver::open_db_io_backend_admitted(&pool, || Self::open_owned_once(pool.clone(), path.clone(), in_memory)).await
    }

    /// 🎯️ One SQLite backend open attempt, refused at once when the backend capacity is taken.
    async fn open_owned_once(pool: Arc<WorkerPool>, path: DbIoText, in_memory: bool) -> Result<Self, DbStorageOpenRejected> {
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("SQLite DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let executor = Box::new(SqliteDbIoExecutor::new(path.clone(), in_memory));
        let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Sqlite, executor, pool.clone(), pool_use, rollback)?;
        let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = execute(DbIoTask::BackendOpen { backend: control, path }).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    pub async fn open(pool: Arc<WorkerPool>, path: &std::path::Path) -> Result<Self, DbStorageOpenRejected> {
        let path = path.to_str().ok_or_else(|| DbError::InvalidArgument("SQLite path is not UTF-8".to_string()))?;
        Self::open_owned(pool, DbIoText::try_from_str(path)?, false).await
    }

    pub async fn open_in_memory(pool: Arc<WorkerPool>) -> Result<Self, DbStorageOpenRejected> {
        Self::open_owned(pool, DbIoText::try_from_str(":memory:")?, true).await
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = unit(execute(DbIoTask::BackendClose { backend: self.control }).await?);
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
    }
}

impl Drop for SqliteStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

impl WalStorage for SqliteStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
        let document = document_text(document)?;
        db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
            wal_writer(execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await?)
        })
        .await
    }

    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }

    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite WAL append")?;
        length(execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await?)
    }

    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
    }

    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_BLOB_BYTES, "sqlite WAL read")?;
        pages(execute(DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output: output_writer(range.len)? }).await?)
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        length(execute(DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match execute(DbIoTask::WalState { backend: self.control, document: document_text(document)?, index }).await? {
            DbIoResult::WalSegmentState(state) => Ok(state),
            _ => Err(DbError::Internal("SQLite executor returned a non-WAL-state result".to_string())),
        }
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        list(execute(DbIoTask::WalList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await?)
    }

    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
}

impl SnapshotStorage for SqliteStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite snapshot write")?;
        unit(execute(DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(MAX_BLOB_BYTES)? }).await?)
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match execute(DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(result_fault("optional generation")),
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        list(execute(DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
    }
}

impl PayloadStorage for SqliteStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite payload put")?;
        match execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(result_fault("hash")),
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(MAX_BLOB_BYTES)? }).await?)
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(value) => Ok(value),
            _ => Err(result_fault("existence")),
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        unit(execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        length(execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
    }
}

impl CatalogStorage for SqliteStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match execute(DbIoTask::CatalogRead { backend: self.control, output: output_writer(MAX_BLOB_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(value) => Ok(value),
            _ => Err(result_fault("optional catalog")),
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_BLOB_BYTES, "sqlite catalog CAS")?;
        match execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(result_fault("fence")),
        }
    }
}

impl IndexStorage for SqliteStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite index write")?;
        unit(execute(DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(MAX_BLOB_BYTES)? }).await?)
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        list(execute(DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
    }
}

impl LeaseStorage for SqliteStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(result_fault("fence")),
        }
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        unit(execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(value) => Ok(value),
            _ => Err(result_fault("optional lease")),
        }
    }
}
//#endregion 🔖️Facade
