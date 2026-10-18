//! 🧭️ Postgres typed executor: the typed task operations and the task executor implementation of the executor. See [`super`].

use super::*;

//#region 🔖️TypedExecutor
impl PostgresDbIoExecutor {
    async fn wal_read_into(&self, document: &str, index: u64, range: ByteRange, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read")?;
        let index = to_i64(index)?;
        let current: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document).bind(index).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current = current.ok_or_else(|| DbError::NotFound("PostgreSQL WAL segment not found".to_string()))?.0 as u64;
        let (offset, len) = validate_read_range(current, range)?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1::integer FOR $2::integer) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(document).bind(index).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn named_blob_read_into(&self, table: &'static str, document: &str, ordinal: u64, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let ordinal = to_i64(ordinal)?;
        let (length_sql, read_sql) = match table {
            "snapshot" => ("SELECT octet_length(bytes)::BIGINT FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2", "SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2"),
            "index" => ("SELECT octet_length(bytes)::BIGINT FROM db_index_run WHERE document_id = $1 AND run_id = $2", "SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2"),
            _ => return Err(DbError::Internal("PostgreSQL named blob taxonomy mismatch".to_string())),
        };
        let length: Option<(i64,)> = sqlx::query_as(length_sql).bind(document).bind(ordinal).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let length = length.ok_or_else(|| DbError::NotFound("PostgreSQL named blob not found".to_string()))?.0 as u64;
        check_len(length, DB_IO_MAX_READ_BYTES, "PostgreSQL named blob")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as(read_sql).bind(document).bind(ordinal).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn payload_read_into(&self, hash: &ContentHash, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let length: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let length = length.ok_or_else(|| DbError::NotFound("PostgreSQL payload not found".to_string()))?.0 as u64;
        check_len(length, DB_IO_MAX_READ_BYTES, "PostgreSQL payload")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn catalog_read_into(&self, output: &mut DbIoPageWriter) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (epoch, bytes): (i64, Option<Vec<u8>>) = sqlx::query_as("SELECT epoch, bytes FROM db_catalog_root WHERE id = 1").fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        match bytes {
            Some(bytes) => Ok(Some((db_io_write_observed_bytes(reservation, bytes, output).await?, EpochFence { epoch: epoch as u64 }))),
            None => {
                let mut reservation = reservation;
                reservation.close_step()?;
                Ok(None)
            }
        }
    }

    async fn drive_task(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        self.active_operation = operation;
        match task {
            DbIoTask::WalWriterAcquire { backend, document } => {
                self.writer_table()?.ensure_capacity(document)?;
                let session = PostgresWalWriterSession::open(&self.pool, document).await?;
                let backend = *backend;
                let permit = self.writer_table()?.acquire_with(document, move || Ok(PostgresWalWriterGuard::Held { session: Some(session), backend }))?;
                Ok(DbIoResult::WalWriter(permit))
            }
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != self.database_url.as_str() {
                    return Err(DbError::InvalidArgument("PostgreSQL URL authority mismatch".to_string()));
                }
                bootstrap_schema(&self.pool).await?;
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
            DbIoTask::BackendClose { .. } => {
                self.pool.close().await;
                Ok(DbIoResult::Unit)
            }
        }
    }
}

impl DbIoTaskExecutor for PostgresDbIoExecutor {
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
        (size_of::<Self>() + size_of::<WalWriterTable<PostgresWalWriterGuard>>()) as u64
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
        Err(DbError::Internal("PostgreSQL async-native task entered the blocking executor".to_string()))
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
                    return Err(DbError::Internal("PostgreSQL WAL writer table returned a false terminal witness".to_string()));
                }
                owner.take();
                return Ok(false);
            }
        }
        if self.pool.is_closed() {
            self.close_future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
            return Ok(true);
        }
        let mut close = self.close_future.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if close.is_none() {
            let pool = self.pool.clone();
            *close = Some(db_storage_driver_runtime::detach_unit(Box::pin(async move { pool.close().await })));
            return Ok(false);
        }
        let terminal = match close.as_mut() {
            Some(receiver) => std::future::Future::poll(std::pin::Pin::new(receiver), context).is_ready(),
            None => false,
        };
        if terminal {
            close.take();
        }
        Ok(false)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire) && self.pool.is_closed() && self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}
//#endregion 🔖️TypedExecutor
