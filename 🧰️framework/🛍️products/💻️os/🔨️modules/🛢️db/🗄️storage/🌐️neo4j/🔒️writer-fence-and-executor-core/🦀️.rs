//! 🔒️ Writer fence and executor core: the WAL writer lease and guard, the executor and its WAL storage. See [`super`].

use super::*;

//#region 🔖️WriterFence
/// ⏳️ Lease lifetime on the server clock — contract `neo4j.leaseTtlMs`.
pub(super) const WAL_WRITER_LEASE_TTL_MS: i64 = 15_000;

/// 💓 Renewal period of a held lease — contract `neo4j.renewEveryMs`.
pub(super) const WAL_WRITER_RENEW_EVERY_MS: u64 = 5_000;

/// 🎫️ One claimed lease: the holder identity and the fencing token its claim produced.
#[derive(Clone)]
pub(super) struct Neo4jWalWriterLease {
    pub(super) holder: String,
    token: i64,
}

impl Neo4jWalWriterLease {
    pub(super) fn holder() -> String {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
        format!("{}:{nanos}:{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    pub(super) fn query(&self, cypher: &'static str, document: &str) -> Query {
        query(cypher).param("document", document.to_string()).param("holder", self.holder.clone()).param("token", self.token).param("ttlMs", WAL_WRITER_LEASE_TTL_MS)
    }

    pub(super) async fn claim(graph: &Graph, document: &str) -> Result<Self, DbError> {
        let holder = Self::holder();
        let mut stream = graph.execute(query(CYPHER_WAL_WRITER_ACQUIRE).param("document", document.to_string()).param("holder", holder.clone()).param("ttlMs", WAL_WRITER_LEASE_TTL_MS)).await.map_err(map_neo4rs_error)?;
        let row = stream.next().await.map_err(map_neo4rs_error)?;
        while stream.next().await.map_err(map_neo4rs_error)?.is_some() {}
        let token: i64 = row.ok_or_else(|| DbError::Conflict("WAL document already has a live Neo4j writer lease".to_string()))?.get("token").map_err(map_de_error)?;
        Ok(Self { holder, token })
    }

    pub(super) fn renew_periodically(&self, graph: Graph, document: String) -> DbIoDriverPeriodic {
        let lease = self.clone();
        db_storage_driver_runtime::detach_periodic(std::time::Duration::from_millis(WAL_WRITER_RENEW_EVERY_MS), move || {
            let graph = graph.clone();
            let renew = lease.query(CYPHER_WAL_WRITER_RENEW, &document);
            Box::pin(async move {
                let Ok(mut stream) = graph.execute(renew).await else { return true };
                let owned = match stream.next().await {
                    Ok(row) => row.is_some(),
                    Err(_) => return true,
                };
                while let Ok(Some(_)) = stream.next().await {}
                owned
            })
        })
    }
}

/// 🔔 Completion witness of a detached lease release; wakes the release controller and a parked backend close.
struct Neo4jWalWriterUnlock {
    done: std::sync::atomic::AtomicBool,
    pub(super) waker: std::sync::Mutex<Option<std::task::Waker>>,
}

/// 🔐️ One document's cross-process writer fence: a renewed lease, then an in-flight release, then terminal.
pub(super) enum Neo4jWalWriterGuard {
    Held { lease: Neo4jWalWriterLease, graph: Graph, document: String, backend: DbIoBackendControl, renewal: DbIoDriverPeriodic },
    Unlocking(Arc<Neo4jWalWriterUnlock>),
    Terminal,
}

impl WalWriterGuard for Neo4jWalWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError> {
        match std::mem::replace(self, Self::Terminal) {
            Self::Held { lease, graph, document, backend, renewal } => {
                drop(renewal);
                let unlock = Arc::new(Neo4jWalWriterUnlock { done: std::sync::atomic::AtomicBool::new(false), waker: std::sync::Mutex::new(None) });
                let signal = unlock.clone();
                drop(db_storage_driver_runtime::detach_unit(Box::pin(async move {
                    let _ = graph.run(lease.query(CYPHER_WAL_WRITER_RELEASE, &document)).await;
                    signal.done.store(true, std::sync::atomic::Ordering::Release);
                    if let Some(waker) = signal.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        waker.wake();
                    }
                    db_io_core::request_controller(backend);
                })));
                *self = Self::Unlocking(unlock);
                Ok(true)
            }
            Self::Unlocking(unlock) if !unlock.done.load(std::sync::atomic::Ordering::Acquire) => {
                *self = Self::Unlocking(unlock);
                Ok(true)
            }
            Self::Unlocking(_) | Self::Terminal => Ok(false),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self, Self::Terminal)
    }

    fn awaiting_wake(&self) -> bool {
        matches!(self, Self::Unlocking(unlock) if !unlock.done.load(std::sync::atomic::Ordering::Acquire))
    }

    fn register_wake(&self, waker: &std::task::Waker) {
        if let Self::Unlocking(unlock) = self {
            *unlock.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(waker.clone());
            if unlock.done.load(std::sync::atomic::Ordering::Acquire) {
                waker.wake_by_ref();
            }
        }
    }
}
//#endregion 🔖️WriterFence

//#region 🔖️Neo4jStorage
/// 🕸️ `DbStorage` over a live Neo4j server — see module doc for the schema shape, the
/// async-first `DbFuture` boundary, and the lease-node WAL writer fence.
pub(super) struct Neo4jDbIoExecutor {
    pub(super) graph: Option<Graph>,
    pub(super) config: Option<neo4rs::Config>,
    pub(super) uri: DbIoText,
    pub(super) writers: std::sync::Mutex<Option<Box<WalWriterTable<Neo4jWalWriterGuard>>>>,
    pub(super) backend_terminal: std::sync::atomic::AtomicBool,
    pub(super) active_operation: u64,
}

impl Neo4jDbIoExecutor {
    pub(super) fn new(config: neo4rs::Config, uri: DbIoText) -> Self {
        Self {
            graph: None,
            config: Some(config),
            uri,
            writers: std::sync::Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
            backend_terminal: std::sync::atomic::AtomicBool::new(false),
            active_operation: 0,
        }
    }

    pub(super) fn writer_table(&mut self) -> Result<&mut WalWriterTable<Neo4jWalWriterGuard>, DbError> {
        self.writers.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)
    }

    pub(super) fn graph(&self) -> Result<&Graph, DbError> {
        self.graph.as_ref().ok_or(DbError::Closed)
    }

    pub(super) fn reserve_driver_read(&self, maximum: u64) -> Result<DbIoDriverReservation, DbError> {
        let maximum = usize::try_from(maximum).map_err(|_| DbError::LimitExceeded("Neo4j driver byte owner"))?;
        DbIoDriverReservation::try_reserve(self.active_operation, maximum)
    }

    pub(super) async fn bootstrap_schema(&self) -> Result<(), DbError> {
        for statement in SCHEMA_STATEMENTS {
            self.graph()?.run(query(statement)).await.map_err(map_neo4rs_error)?;
        }
        Ok(())
    }

    /// 1⃣ Runs `q` (autocommit) and returns its first row, if any.
    pub(super) async fn fetch_one(&self, q: Query) -> Result<Option<neo4rs::Row>, DbError> {
        let mut stream = self.graph()?.execute(q).await.map_err(map_neo4rs_error)?;
        stream.next().await.map_err(map_neo4rs_error)
    }

    /// ▶️ Runs `q` (autocommit), discarding any result rows.
    pub(super) async fn run(&self, q: Query) -> Result<(), DbError> {
        self.graph()?.run(q).await.map_err(map_neo4rs_error)
    }
}
//#endregion 🔖️Neo4jStorage

//#region 🔖️WalStorage
impl Neo4jDbIoExecutor {
    /// 🛡️ Proves, inside the mutation's own transaction, that `lease` still owns the document.
    async fn prove_lease(txn: &mut Txn, lease: &Neo4jWalWriterLease, document: &str, generation: u64) -> Result<(), DbError> {
        let mut stream = txn.execute(lease.query(CYPHER_WAL_WRITER_FENCE, document)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        while stream.next(txn.handle()).await.map_err(map_neo4rs_error)?.is_some() {}
        let Some(row) = row else { return Err(DbError::Fenced { expected: 0, actual: generation }) };
        if row.get::<bool>("owned").map_err(map_de_error)? {
            return Ok(());
        }
        let token: i64 = row.get("token").map_err(map_de_error)?;
        Err(DbError::Fenced { expected: i64_to_u64(token, "wal writer token")?, actual: i64_to_u64(lease.token, "wal writer token")? })
    }

    pub(super) async fn create_segment(txn: &mut Txn, document: &str, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut stream = txn.execute(query(CYPHER_WAL_CREATE_SEGMENT).param("document", document.to_string()).param("index", idx).param("empty", Vec::<u8>::new())).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        while stream.next(txn.handle()).await.map_err(map_neo4rs_error)?.is_some() {}
        let fresh: bool = row.ok_or_else(|| DbError::Internal("wal create_segment returned no row".to_string()))?.get("fresh").map_err(map_de_error)?;
        if !fresh {
            return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
        }
        Ok(())
    }

    pub(super) async fn append(&self, txn: &mut Txn, document: &str, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "wal_storage::append")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut current_reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let mut stream = txn.execute(query(CYPHER_WAL_READ_ROW).param("document", document.to_string()).param("index", idx)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
        };
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, DB_IO_MAX_READ_BYTES, "wal_storage::append current length")?;
        let current: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut current = DbIoExternalBytes::new(current.value.to_vec());
        current_reservation.observe_capacity(current.capacity()?)?;
        let sealed: bool = row.get("sealed").map_err(map_de_error)?;
        if sealed {
            return Err(DbError::InvalidArgument("cannot append to sealed wal segment".to_string()));
        }
        let new_len = current.as_slice()?.len().checked_add(prepared.as_slice().len()).ok_or(DbError::LimitExceeded("Neo4j WAL append length"))?;
        check_len(new_len as u64, DB_IO_MAX_READ_BYTES, "wal_storage::append result")?;
        let combined = db_io_prepare_platform_slices(self.active_operation, current.as_slice()?, prepared.as_slice()).await?;
        let write = txn
            .run(query(CYPHER_WAL_WRITE_BYTES).param("document", document.to_string()).param("index", idx).param("bytes", combined.as_static_driver_slice().to_vec()).param("len", u64_to_i64(new_len as u64, "wal segment length")?))
            .await
            .map_err(map_neo4rs_error);
        while !current.terminal_is_empty() {
            let _ = current.close_step();
            semio_framework_async::yield_once().await;
        }
        current_reservation.close_step()?;
        db_io_close_platform(combined).await?;
        db_io_close_platform(prepared).await?;
        write?;
        Ok(new_len as u64)
    }

    pub(super) async fn seal(txn: &mut Txn, document: &str, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut stream = txn.execute(query(CYPHER_WAL_SEAL).param("document", document.to_string()).param("index", idx)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        while stream.next(txn.handle()).await.map_err(map_neo4rs_error)?.is_some() {}
        row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        Ok(())
    }

    pub(super) async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read")?;
        let idx = u64_to_i64(index, "wal segment index")?;
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, DB_IO_MAX_READ_BYTES, "wal_storage::read current length")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let _ = slice_range(&bytes.value, range)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (range.len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, range.offset as usize, range.len as usize, &mut output).await
    }

    pub(super) async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")
    }

    pub(super) async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let row = self.fetch_one(query(CYPHER_WAL_STATE).param("document", document.0.clone()).param("index", idx)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        Ok(neo4j_wal_segment_state(row.get("sealed").map_err(map_de_error)?))
    }

    pub(super) async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_WAL_LIST_SEGMENTS).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("segIndex").map_err(map_de_error)?, "wal segment index")?)?;
        }
        Ok(out)
    }

    pub(super) async fn truncate_tail(&self, txn: &mut Txn, document: &str, index: u64, new_len: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut current_reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let mut stream = txn.execute(query(CYPHER_WAL_READ_ROW).param("document", document.to_string()).param("index", idx)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
        };
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, DB_IO_MAX_READ_BYTES, "wal_storage::truncate_tail current length")?;
        let current: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut current = DbIoExternalBytes::new(current.value.to_vec());
        current_reservation.observe_capacity(current.capacity()?)?;
        let sealed: bool = row.get("sealed").map_err(map_de_error)?;
        if sealed {
            return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
        }
        if new_len > current.as_slice()?.len() as u64 {
            return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
        }
        let truncated = db_io_prepare_platform_slices(self.active_operation, &current.as_slice()?[..new_len as usize], &[]).await?;
        let write = txn
            .run(query(CYPHER_WAL_WRITE_BYTES).param("document", document.to_string()).param("index", idx).param("bytes", truncated.as_static_driver_slice().to_vec()).param("len", u64_to_i64(new_len, "wal segment length")?))
            .await
            .map_err(map_neo4rs_error);
        while !current.terminal_is_empty() {
            let _ = current.close_step();
            semio_framework_async::yield_once().await;
        }
        current_reservation.close_step()?;
        db_io_close_platform(truncated).await?;
        write
    }

    pub(super) async fn delete_segment(txn: &mut Txn, document: &str, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        txn.run(query(CYPHER_WAL_DELETE_SEGMENT).param("document", document.to_string()).param("index", idx)).await.map_err(map_neo4rs_error)
    }

    /// 🔐️ Runs one pinned WAL mutation (sync included, as the ownership barrier) in a
    /// transaction whose first statement proves the permit's lease; any failure rolls it back.
    pub(super) async fn fenced_wal_mutation(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        let (key, backend, document) = task.writer_stamp().map(|(key, backend, document)| (key, backend, document.clone())).ok_or_else(|| DbError::Internal("Neo4j WAL mutation lost its writer stamp".to_string()))?;
        let lease = match self.writer_table()?.pinned_guard_mut(key, backend, &document, operation)? {
            Neo4jWalWriterGuard::Held { lease, .. } => lease.clone(),
            _ => return Err(DbError::Closed),
        };
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let document = document.as_str();
        let result = match Self::prove_lease(&mut txn, &lease, document, key.generation()).await {
            Err(error) => Err(error),
            Ok(()) => match task {
                DbIoTask::WalCreate { index, .. } => Self::create_segment(&mut txn, document, *index).await.map(|()| DbIoResult::Unit),
                DbIoTask::WalAppend { index, input, .. } => {
                    let input = input.take_for_async_driver();
                    self.append(&mut txn, document, *index, input).await.map(DbIoResult::Length)
                }
                DbIoTask::WalSync { .. } => Ok(DbIoResult::Unit),
                DbIoTask::WalSeal { index, .. } => Self::seal(&mut txn, document, *index).await.map(|()| DbIoResult::Unit),
                DbIoTask::WalTruncate { index, new_len, .. } => self.truncate_tail(&mut txn, document, *index, *new_len).await.map(|()| DbIoResult::Unit),
                DbIoTask::WalDelete { index, .. } => Self::delete_segment(&mut txn, document, *index).await.map(|()| DbIoResult::Unit),
                _ => Err(DbError::Internal("Neo4j fenced WAL mutation taxonomy".to_string())),
            },
        };
        match result {
            Ok(value) => {
                txn.commit().await.map_err(map_neo4rs_error)?;
                Ok(value)
            }
            Err(error) => {
                let _ = txn.rollback().await;
                Err(error)
            }
        }
    }
}
//#endregion 🔖️WalStorage
