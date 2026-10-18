//! 🕸️ Neo4j storage backend: `Neo4jStorage`, its result helpers and the storage trait impls. See [`super`].

use super::*;

/// 🕸️ Typed Neo4j facade; only the registered executor owns the external graph driver.
pub struct Neo4jStorage {
    pub control: DbIoBackendControl,
    pub(super) worker_pool: Arc<WorkerPool>,
    pub(super) closed: std::sync::atomic::AtomicBool,
}

impl Neo4jStorage {
    pub async fn connect(worker_pool: Arc<WorkerPool>, uri: &str, user: &str, password: &str) -> Result<Self, DbStorageOpenRejected> {
        let uri_owner = DbIoText::try_from_str(uri)?;
        let config = neo4rs::ConfigBuilder::default().uri(uri).user(user).password(password).build().map_err(map_neo4rs_error)?;
        Self::connect_owned(worker_pool, uri_owner, config).await
    }

    pub async fn connect_to_database(worker_pool: Arc<WorkerPool>, uri: &str, user: &str, password: &str, database: &str) -> Result<Self, DbStorageOpenRejected> {
        let uri_owner = DbIoText::try_from_str(uri)?;
        let config = neo4rs::ConfigBuilder::default().uri(uri).user(user).password(password).db(database).build().map_err(map_neo4rs_error)?;
        Self::connect_owned(worker_pool, uri_owner, config).await
    }

    async fn connect_owned(worker_pool: Arc<WorkerPool>, uri: DbIoText, config: neo4rs::Config) -> Result<Self, DbStorageOpenRejected> {
        db_io_driver::open_db_io_backend_admitted(&worker_pool, || Self::connect_owned_once(worker_pool.clone(), uri.clone(), config.clone())).await
    }

    /// 🎯️ One Neo4j backend open attempt, refused at once when the backend capacity is taken.
    async fn connect_owned_once(worker_pool: Arc<WorkerPool>, uri: DbIoText, config: neo4rs::Config) -> Result<Self, DbStorageOpenRejected> {
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = worker_pool.acquire_use().map_err(|error| DbError::Unavailable(format!("Neo4j DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let executor = Box::new(Neo4jDbIoExecutor::new(config, uri.clone()));
        let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Neo4j, executor, worker_pool.clone(), pool_use, rollback)?;
        let storage = Self { control, worker_pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = storage.execute(DbIoTask::BackendOpen { backend: control, path: uri }).await {
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
        let result = neo4j_unit(self.execute(DbIoTask::BackendClose { backend: self.control }).await?);
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }
}

impl Drop for Neo4jStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

fn neo4j_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn neo4j_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("Neo4j output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn neo4j_unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(DbError::Internal("Neo4j executor returned a non-unit result".to_string())),
    }
}

fn neo4j_pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(pages) => Ok(pages),
        _ => Err(DbError::Internal("Neo4j executor returned a non-page result".to_string())),
    }
}

fn neo4j_list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(list) => Ok(list),
        _ => Err(DbError::Internal("Neo4j executor returned a non-list result".to_string())),
    }
}

impl WalStorage for Neo4jStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_io_core::WalWriterPermit, DbError> {
        let document = neo4j_document(document)?;
        db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
            match self.execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await? {
                DbIoResult::WalWriter(writer) => Ok(writer),
                _ => Err(DbError::Internal("remote WAL writer result taxonomy".to_string())),
            }
        })
        .await
    }
    async fn create_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
    async fn append(&self, writer: &db_io_core::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
    async fn sync(&self, writer: &db_io_core::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
    }
    async fn seal(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::WalRead { backend: self.control, document: neo4j_document(document)?, index, range, output: neo4j_output(range.len)? }).await?)
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalLength { backend: self.control, document: neo4j_document(document)?, index }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match self.execute(DbIoTask::WalState { backend: self.control, document: neo4j_document(document)?, index }).await? {
            DbIoResult::WalSegmentState(state) => Ok(state),
            _ => Err(DbError::Internal("Neo4j executor returned a non-WAL-state result".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::WalList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn truncate_tail(&self, writer: &db_io_core::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await?)
    }
    async fn delete_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
}

impl SnapshotStorage for Neo4jStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::SnapshotWrite { backend: self.control, document: neo4j_document(document)?, generation, input: bytes }).await?)
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::SnapshotRead { backend: self.control, document: neo4j_document(document)?, generation, output: neo4j_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self.execute(DbIoTask::SnapshotLatest { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(generation) => Ok(generation),
            _ => Err(DbError::Internal("Neo4j executor returned a non-generation result".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::SnapshotList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::SnapshotDelete { backend: self.control, document: neo4j_document(document)?, generation }).await?)
    }
}

impl PayloadStorage for Neo4jStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self.execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(DbError::Internal("Neo4j executor returned a non-hash result".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: neo4j_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self.execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(exists) => Ok(exists),
            _ => Err(DbError::Internal("Neo4j executor returned a non-exists result".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self.execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
}

impl CatalogStorage for Neo4jStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self.execute(DbIoTask::CatalogRead { backend: self.control, output: neo4j_output(DB_IO_MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(catalog) => Ok(catalog),
            _ => Err(DbError::Internal("Neo4j executor returned a non-catalog result".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("Neo4j executor returned a non-fence result".to_string())),
        }
    }
}

impl IndexStorage for Neo4jStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::IndexWrite { backend: self.control, document: neo4j_document(document)?, run_id, input: bytes }).await?)
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::IndexRead { backend: self.control, document: neo4j_document(document)?, run_id, output: neo4j_output(DB_IO_MAX_READ_BYTES)? }).await?)
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::IndexList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::IndexDelete { backend: self.control, document: neo4j_document(document)?, run_id }).await?)
    }
}

impl LeaseStorage for Neo4jStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("Neo4j executor returned a non-fence result".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self.execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(lease) => Ok(lease),
            _ => Err(DbError::Internal("Neo4j executor returned a non-lease result".to_string())),
        }
    }
}

//#region 🔖️DbBackend
impl Neo4jStorage {
    /// 🎚️ What this backend actually supports. See module doc's "Durability" section: every
    /// write is already committed server-side by the time it returns, so this backend can
    /// honestly claim the strongest single-node durability class without a separate `sync` step.
    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
    }
}
//#endregion 🔖️DbBackend
