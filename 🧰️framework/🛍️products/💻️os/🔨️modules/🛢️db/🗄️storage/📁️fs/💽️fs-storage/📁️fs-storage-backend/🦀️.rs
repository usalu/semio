//! 📁️ Fs storage backend: the task helpers, the public filesystem storage, its drop and the storage trait implementations. See [`super`].

use super::*;

async fn execute(task: DbIoTask) -> Result<DbIoResult, DbError> {
    submit_db_io_task_admitted(task).await?.finish().await
}

fn document_text(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn output_writer(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("db_io output writer bytes"))?.div_ceil(super::super::DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn typed_result_fault(expected: &'static str) -> DbError {
    DbError::Internal(format!("DB I/O filesystem executor did not return {expected}"))
}

fn unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(typed_result_fault("unit")),
    }
}

fn length(result: DbIoResult) -> Result<u64, DbError> {
    match result {
        DbIoResult::Length(value) => Ok(value),
        _ => Err(typed_result_fault("length")),
    }
}

fn optional_length(result: DbIoResult) -> Result<Option<u64>, DbError> {
    match result {
        DbIoResult::OptionalLength(value) => Ok(value),
        _ => Err(typed_result_fault("optional length")),
    }
}

pub(super) fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(value) => Ok(value),
        _ => Err(typed_result_fault("pages")),
    }
}

pub(super) fn list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(value) => Ok(value),
        _ => Err(typed_result_fault("list")),
    }
}

fn wal_writer(result: DbIoResult) -> Result<WalWriterPermit, DbError> {
    match result {
        DbIoResult::WalWriter(writer) => Ok(writer),
        _ => Err(typed_result_fault("WAL writer")),
    }
}

/// 📁️ The zero-touch default `DbStorage` backend — see module doc for the on-disk
/// layout. `catalog_lock`/`lease_lock` serialize this-process's compare-and-swap operations;
/// see `CatalogStorage`/`LeaseStorage` impls below for why a bare read-verify-write over
/// `write_atomic` isn't itself enough across OS processes (documented extension seam). `pool`
/// is what every trait method dispatches its blocking body through (see module doc); its
/// `Arc` is cloned into each method's `'static` blocking closure.
pub struct FsStorage {
    pub(super) control: DbIoBackendControl,
    pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

impl FsStorage {
    /// 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
    /// subsequent trait call's blocking body through the typed task owner onto `pool`'s
    /// `Lane::Io`. The constructor's directory creation uses that same retained authority;
    /// callers never prepare the root synchronously or through a pool-less fallback.
    pub async fn open(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbStorageOpenRejected> {
        super::super::open_db_io_backend_admitted(&pool, || Self::open_once(pool.clone(), root)).await
    }

    /// 📊️ This backend's writer admission line so far — how many acquisitions had to wait for a writer, how many
    /// waits were refused, and how many wait now.
    pub(crate) fn writer_admission_census(&self) -> db_io_core::WalWriterAdmissionCensus {
        db_io_driver::admission_census(self.control)
    }

    /// 🎯️ One filesystem backend open attempt, refused at once when the backend capacity is taken.
    async fn open_once(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbStorageOpenRejected> {
        let root = root.to_str().ok_or_else(|| DbError::InvalidArgument("filesystem storage root is not UTF-8".to_string())).and_then(DbIoText::try_from_str)?;
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("filesystem DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Filesystem, Box::new(FsDbIoExecutor::new(root.clone())), pool.clone(), pool_use, rollback)?;
        let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
        let task = DbIoTask::BackendOpen { backend: control, path: root };
        if let Err(error) = execute(task).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = match execute(DbIoTask::BackendClose { backend: self.control }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("filesystem backend close returned the wrong typed result".to_string())),
        };
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }

    /// 🎚️ Always durable, `fsync`-capable, CAS-capable — the on-disk default.
    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
    }
}

impl Drop for FsStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

impl WalStorage for FsStorage {
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
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "wal_storage::append")?;
        length(execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await?)
    }

    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
    }

    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        if let Err(err) = check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read") {
            return Err(err);
        }
        let output = output_writer(range.len)?;
        pages(execute(DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output }).await?)
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        length(execute(DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match execute(DbIoTask::WalState { backend: self.control, document: document_text(document)?, index }).await? {
            DbIoResult::WalSegmentState(state) => Ok(state),
            _ => Err(DbError::Internal("filesystem WAL state result taxonomy".to_string())),
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

impl SnapshotStorage for FsStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "snapshot_storage::write_generation")?;
        unit(execute(DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        optional_length(execute(DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        list(execute(DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
    }
}

impl PayloadStorage for FsStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        if let Err(err) = check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "payload_storage::put") {
            return Err(err);
        }
        match execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(typed_result_fault("content hash")),
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(exists) => Ok(exists),
            _ => Err(typed_result_fault("existence")),
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        unit(execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        length(execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
    }
}

impl CatalogStorage for FsStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match execute(DbIoTask::CatalogRead { backend: self.control, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(root) => Ok(root),
            _ => Err(typed_result_fault("optional catalog root")),
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        if let Err(err) = check_len(new_bytes.len() as u64, DB_IO_MAX_READ_BYTES, "catalog_storage::cas_root") {
            return Err(err);
        }
        match execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(typed_result_fault("catalog fence")),
        }
    }
}

/// 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
/// `cas_root` can reuse it under `catalog_lock` without recursive task submission.
// 🚫️async: E1 pure-shaped typed-task accessor.
pub(super) fn read_root_fence(root: &Path) -> Result<Option<EpochFence>, DbError> {
    let path = catalog_path(root);
    if !path.exists() {
        return Ok(None);
    }
    let metadata = std::fs::metadata(&path).map_err(io_err)?;
    if metadata.len() < 8 {
        return Err(DbError::Corrupt("catalog root file is shorter than its 8-byte epoch header".to_string()));
    }
    let mut epoch_bytes = [0u8; 8];
    std::fs::File::open(&path).map_err(io_err)?.read_exact(&mut epoch_bytes).map_err(io_err)?;
    Ok(Some(EpochFence { epoch: u64::from_le_bytes(epoch_bytes) }))
}

impl IndexStorage for FsStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "index_storage::write_run")?;
        unit(execute(DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        pages(execute(DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        list(execute(DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        unit(execute(DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
    }
}

impl LeaseStorage for FsStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(typed_result_fault("lease fence")),
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
            DbIoResult::OptionalLease(lease) => Ok(lease),
            _ => Err(typed_result_fault("optional lease")),
        }
    }
}
