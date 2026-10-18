//! ⚙️ Typed executor: the typed operation execution and the task executor impl. See [`super`].

use super::*;

//#region 🔖️TypedExecutor
impl Neo4jDbIoExecutor {
    async fn wal_read_into(&self, document: &str, index: u64, range: ByteRange, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        check_len(range.len, DB_IO_MAX_READ_BYTES, "Neo4j WAL task read")?;
        let index = u64_to_i64(index, "wal segment index")?;
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document).param("index", index)).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j WAL segment not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        if range.offset.checked_add(range.len).is_none_or(|end| end > length) {
            return Err(DbError::InvalidArgument("Neo4j WAL task range exceeds segment".to_string()));
        }
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, range.offset as usize, range.len as usize, output).await
    }

    async fn named_blob_read_into(&self, kind: &'static str, document: &str, ordinal: u64, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let (query_text, key, value) = match kind {
            "snapshot" => (CYPHER_SNAPSHOT_READ, "generation", u64_to_i64(ordinal, "snapshot generation")?),
            "index" => (CYPHER_INDEX_READ, "runId", u64_to_i64(ordinal, "index run")?),
            _ => return Err(DbError::Internal("Neo4j named blob taxonomy mismatch".to_string())),
        };
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(query_text).param("document", document).param(key, value)).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j named blob not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "Neo4j named blob length")?;
        check_len(length, DB_IO_MAX_READ_BYTES, "Neo4j named blob")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, 0, length as usize, output).await
    }

    async fn payload_read_into(&self, hash: &ContentHash, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_PAYLOAD_GET).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j payload not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")?;
        check_len(length, DB_IO_MAX_READ_BYTES, "Neo4j payload")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, 0, length as usize, output).await
    }

    async fn catalog_read_into(&self, output: &mut DbIoPageWriter) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let mut reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let Some(row) = self.fetch_one(query(CYPHER_CATALOG_READ)).await? else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "catalog epoch")?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "catalog length")?;
        check_len(length, DB_IO_MAX_READ_BYTES, "Neo4j catalog")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        Ok(Some((write_driver_bytes(reservation, bytes, 0, length as usize, output).await?, EpochFence { epoch })))
    }

    async fn drive_task(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        self.active_operation = operation;
        match task {
            DbIoTask::WalWriterAcquire { backend, document } => {
                self.writer_table()?.ensure_capacity(document)?;
                let graph = self.graph()?.clone();
                let lease = Neo4jWalWriterLease::claim(&graph, document.as_str()).await?;
                let renewal = lease.renew_periodically(graph.clone(), document.as_str().to_string());
                let backend = *backend;
                let held = Neo4jWalWriterGuard::Held { lease, graph, document: document.as_str().to_string(), backend, renewal };
                let permit = self.writer_table()?.acquire_with(document, move || Ok(held))?;
                Ok(DbIoResult::WalWriter(permit))
            }
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != self.uri.as_str() {
                    return Err(DbError::InvalidArgument("Neo4j URI authority mismatch".to_string()));
                }
                let config = self.config.take().ok_or_else(|| DbError::Internal("Neo4j connection configuration consumed twice".to_string()))?;
                self.graph = Some(Graph::connect(config).await.map_err(map_neo4rs_error)?);
                self.bootstrap_schema().await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalCreate { .. } | DbIoTask::WalAppend { .. } | DbIoTask::WalSync { .. } | DbIoTask::WalSeal { .. } | DbIoTask::WalTruncate { .. } | DbIoTask::WalDelete { .. } => self.fenced_wal_mutation(operation, task).await,
            DbIoTask::WalRead { document, index, range, output, .. } => Ok(DbIoResult::Pages(self.wal_read_into(document.as_str(), *index, *range, output).await?)),
            DbIoTask::WalLength { document, index, .. } => Ok(DbIoResult::Length(with_admitted_artifact!(operation, document, artifact, self.segment_len(artifact, *index))?)),
            DbIoTask::WalState { document, index, .. } => Ok(DbIoResult::WalSegmentState(with_admitted_artifact!(operation, document, artifact, self.segment_state(artifact, *index))?)),
            DbIoTask::WalList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, self.list_segments(artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                let input = input.take_for_async_driver();
                with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::write_generation(self, artifact, *generation, input))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("snapshot", document.as_str(), *generation, output).await?)),
            DbIoTask::SnapshotLatest { document, .. } => Ok(DbIoResult::OptionalLength(with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::latest_generation(self, artifact))?)),
            DbIoTask::SnapshotList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::list_generations(self, artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::delete_generation(self, artifact, *generation))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::PayloadPut { input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Hash(<Self as PayloadStorage>::put(self, input).await?))
            }
            DbIoTask::PayloadGet { hash, output, .. } => Ok(DbIoResult::Pages(self.payload_read_into(hash, output).await?)),
            DbIoTask::PayloadExists { hash, .. } => Ok(DbIoResult::Exists(<Self as PayloadStorage>::contains(self, hash).await?)),
            DbIoTask::PayloadLength { hash, .. } => Ok(DbIoResult::Length(<Self as PayloadStorage>::len(self, hash).await?)),
            DbIoTask::PayloadDelete { hash, .. } => {
                <Self as PayloadStorage>::delete(self, hash).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::CatalogRead { output, .. } => Ok(DbIoResult::OptionalCatalog(self.catalog_read_into(output).await?)),
            DbIoTask::CatalogCas { expected, input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Fence(<Self as CatalogStorage>::cas_root(self, *expected, input).await?))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => {
                let input = input.take_for_async_driver();
                with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::write_run(self, artifact, *run_id, input))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("index", document.as_str(), *run_id, output).await?)),
            DbIoTask::IndexList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::list_runs(self, artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::delete_run(self, artifact, *run_id))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseAcquire { document, holder, ttl_ms, now_ms, .. } => Ok(DbIoResult::Fence(<Self as LeaseStorage>::acquire(self, document.as_str(), holder.as_str(), *ttl_ms, *now_ms).await?)),
            DbIoTask::LeaseRenew { document, holder, fence, ttl_ms, now_ms, .. } => {
                <Self as LeaseStorage>::renew(self, document.as_str(), holder.as_str(), *fence, *ttl_ms, *now_ms).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                <Self as LeaseStorage>::release(self, document.as_str(), holder.as_str(), *fence).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let lease = <Self as LeaseStorage>::current(self, document.as_str(), *now_ms).await?;
                Ok(DbIoResult::OptionalLease(lease))
            }
            DbIoTask::BackendClose { .. } => Ok(DbIoResult::Unit),
        }
    }
}

impl DbIoTaskExecutor for Neo4jDbIoExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }
    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        self.writer_table()?.bind(control)
    }
    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().map_or(Ok(DbIoWriterReleaseStep::Idle), WalWriterTable::release_requested_step)
    }
    fn pin_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
        self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)?.pin_operation(key, backend, document, operation).map(|_| ())
    }
    fn finish_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
        if let Some(table) = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut() {
            table.finish_operation_if_pinned(key, backend, document, operation)?;
        }
        Ok(())
    }
    fn owner_backing_bytes(&self) -> u64 {
        (size_of::<Self>() + size_of::<WalWriterTable<Neo4jWalWriterGuard>>()) as u64
    }
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }
    fn driver_runtime(&self) -> Option<&'static dyn DbIoAsyncDriverRuntime> {
        Some(db_storage_driver_runtime::shared())
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("Neo4j async-native task entered the blocking executor".to_string()))
    }
    fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let mut executor = self;
            let mut task = task;
            let terminal = executor.drive_task(operation, &mut task).await;
            let executor: Box<dyn DbIoTaskExecutor> = executor;
            (executor, task, terminal)
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        {
            let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(table) = owner.as_deref_mut() {
                table.register_wake(context.waker());
                if table.close_step()? {
                    return Ok(false);
                }
                if !table.terminal_is_empty() {
                    return Err(DbError::Internal("Neo4j WAL writer table returned a false terminal witness".to_string()));
                }
                owner.take();
                return Ok(false);
            }
        }
        if self.graph.take().is_some() {
            return Ok(false);
        }
        self.config.take();
        self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire) && self.graph.is_none() && self.config.is_none() && self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}
//#endregion 🔖️TypedExecutor
