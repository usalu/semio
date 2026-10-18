//! 🪣️ Memory storage backend: the public in-memory storage, its task helpers, its drop and the storage trait implementations. See [`super`].

use super::*;

//#region 🔖️Memory
pub struct MemoryStorage {
    pub control: DbIoBackendControl,
    pub pool: Arc<WorkerPool>,
    pub closed: std::sync::atomic::AtomicBool,
}

async fn memory_execute(task: DbIoTask) -> Result<DbIoResult, DbError> {
    submit_db_io_task_admitted(task).await?.finish().await
}

pub(super) fn memory_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn memory_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("memory output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

impl MemoryStorage {
    /// 🧠️ Opens one memory backend through [`open_db_io_backend_admitted`] (waits while the process's backend
    /// capacity is taken by backends still retiring).
    pub async fn new(pool: Arc<WorkerPool>) -> Result<Self, DbStorageOpenRejected> {
        open_db_io_backend_admitted(&pool, || Self::open_once(pool.clone())).await
    }

    /// 🎯️ One memory backend open attempt, refused at once when the backend capacity is taken.
    pub async fn open_once(pool: Arc<WorkerPool>) -> Result<Self, DbStorageOpenRejected> {
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("memory DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let credit = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 }.checked_add(db_io_registry::controller_credit()).expect("fixed memory writer control credit");
        let owner = db_io_backend_owner_reserve(credit)?;
        let executor = Box::new(MemoryDbIoExecutor::default());
        let control = register_db_io_backend_reserved_with_use(DbIoBackendKind::Memory, executor, pool.clone(), pool_use, owner, credit, rollback).map_err(|failure| DbIoBackendRegistrationRejected {
            cause: failure.cause,
            cleanup_error: None,
            owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(failure.retirement)),
        })?;
        let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = memory_execute(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("memory://fixed")? }).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn fail_next_writer_release(&self) {
        let (slot, generation) = db_io_backend_parts(self.control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        assert_eq!(owner.generation, generation);
        let executor = owner.executor.as_mut().unwrap().as_any_mut().downcast_mut::<MemoryDbIoExecutor>().unwrap();
        executor.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().unwrap().fail_next_release();
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = match memory_execute(DbIoTask::BackendClose { backend: self.control }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory backend close returned the wrong typed result".to_string())),
        };
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true }
    }
}

impl Drop for MemoryStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

impl WalStorage for MemoryStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
        let document = memory_document(document)?;
        db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
            match memory_execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await? {
                DbIoResult::WalWriter(writer) => Ok(writer),
                _ => Err(DbError::Internal("memory WAL writer result taxonomy".to_string())),
            }
        })
        .await
    }
    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL create result taxonomy".to_string())),
        }
    }
    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL append result taxonomy".to_string())),
        }
    }
    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL sync result taxonomy".to_string())),
        }
    }
    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL seal result taxonomy".to_string())),
        }
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::WalRead { backend: self.control, document: memory_document(document)?, index, range, output: memory_output(range.len)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL read result taxonomy".to_string())),
        }
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::WalLength { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL length result taxonomy".to_string())),
        }
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match memory_execute(DbIoTask::WalState { backend: self.control, document: memory_document(document)?, index }).await? {
            DbIoResult::WalSegmentState(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL state result taxonomy".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::WalList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory WAL list result taxonomy".to_string())),
        }
    }
    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL truncate result taxonomy".to_string())),
        }
    }
    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory WAL delete result taxonomy".to_string())),
        }
    }
}

impl SnapshotStorage for MemoryStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(DbIoTask::SnapshotWrite { backend: self.control, document: memory_document(document)?, generation, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot write result taxonomy".to_string())),
        }
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::SnapshotRead { backend: self.control, document: memory_document(document)?, generation, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot read result taxonomy".to_string())),
        }
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match memory_execute(DbIoTask::SnapshotLatest { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot latest result taxonomy".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::SnapshotList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory snapshot list result taxonomy".to_string())),
        }
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::SnapshotDelete { backend: self.control, document: memory_document(document)?, generation }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory snapshot delete result taxonomy".to_string())),
        }
    }
}

impl PayloadStorage for MemoryStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match memory_execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload put result taxonomy".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload get result taxonomy".to_string())),
        }
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match memory_execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload exists result taxonomy".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match memory_execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory payload delete result taxonomy".to_string())),
        }
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match memory_execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(DbError::Internal("memory payload length result taxonomy".to_string())),
        }
    }
}

impl CatalogStorage for MemoryStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match memory_execute(DbIoTask::CatalogRead { backend: self.control, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog read result taxonomy".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match memory_execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory catalog CAS result taxonomy".to_string())),
        }
    }
}

impl IndexStorage for MemoryStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match memory_execute(DbIoTask::IndexWrite { backend: self.control, document: memory_document(document)?, run_id, input: bytes }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index write result taxonomy".to_string())),
        }
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match memory_execute(DbIoTask::IndexRead { backend: self.control, document: memory_document(document)?, run_id, output: memory_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(DbError::Internal("memory index read result taxonomy".to_string())),
        }
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match memory_execute(DbIoTask::IndexList { backend: self.control, document: memory_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::List(value) => Ok(value),
            _ => Err(DbError::Internal("memory index list result taxonomy".to_string())),
        }
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::IndexDelete { backend: self.control, document: memory_document(document)?, run_id }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory index delete result taxonomy".to_string())),
        }
    }
}

impl LeaseStorage for MemoryStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match memory_execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, ttl_ms, now_ms }).await? {
            DbIoResult::Fence(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease acquire result taxonomy".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match memory_execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, ttl_ms, now_ms }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease renew result taxonomy".to_string())),
        }
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match memory_execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await? {
            DbIoResult::Unit => Ok(()),
            _ => Err(DbError::Internal("memory lease release result taxonomy".to_string())),
        }
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match memory_execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(value) => Ok(value),
            _ => Err(DbError::Internal("memory lease get result taxonomy".to_string())),
        }
    }
}
//#endregion 🔖️Memory
