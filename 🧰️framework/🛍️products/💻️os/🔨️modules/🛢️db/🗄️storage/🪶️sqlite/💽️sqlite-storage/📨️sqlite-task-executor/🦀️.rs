//! 📨️ Sqlite task executor: the typed task executor implementation of the sqlite executor. See [`super`].

use super::*;

//#region 🔖️Executor
impl DbIoTaskExecutor for SqliteDbIoExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }

    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        owner.as_deref_mut().ok_or(DbError::Closed)?.bind(control)
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn owner_backing_bytes(&self) -> u64 {
        (size_of::<Self>() + size_of::<WalWriterTable<SqliteWalWriterGuard>>()) as u64
    }

    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("SQLite backend has no async-native driver".to_string())))
        })
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        if let DbIoTask::BackendOpen { path, .. } = task {
            if path.as_str() != self.path.as_str() {
                return Err(DbError::InvalidArgument("SQLite path authority mismatch".to_string()));
            }
            let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if owner.is_none() {
                let connection = if self.in_memory {
                    Connection::open_in_memory().map_err(sqlite_err)?
                } else {
                    let path = std::path::Path::new(self.path.as_str());
                    if let Some(parent) = path.parent() {
                        if !parent.as_os_str().is_empty() {
                            std::fs::create_dir_all(parent).map_err(|error| DbError::Io(error.to_string()))?;
                        }
                    }
                    Connection::open(path).map_err(sqlite_err)?
                };
                init_connection(&connection)?;
                if !self.in_memory {
                    let canonical = std::fs::canonicalize(self.path.as_str()).map_err(|error| DbError::Io(error.to_string()))?;
                    let canonical = canonical.to_str().ok_or_else(|| DbError::InvalidArgument("canonical SQLite path is not UTF-8".to_string()))?;
                    *self.canonical_database.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbIoText::try_from_str(canonical)?);
                }
                *owner = Some(connection);
            }
            return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));
        }
        if matches!(task, DbIoTask::BackendClose { .. }) {
            return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));
        }
        if let Some(step) = self.reader_step(operation, task)? {
            return Ok(step);
        }
        let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let connection = owner.as_mut().ok_or(DbError::Closed)?;
        if let Some(step) = Self::query_step(connection, task)? {
            return Ok(step);
        }
        let sql_operation = Self::operation(operation)?;
        match task {
            DbIoTask::WalWriterAcquire { document, .. } => {
                let mut writers = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let table = writers.as_deref_mut().ok_or(DbError::Closed)?;
                let permit = table.acquire_with(document, || match self.in_memory {
                    true => Ok(SqliteWalWriterGuard::Local),
                    false => {
                        let path = self.physical_writer_sidecar(document)?;
                        std::fs::create_dir_all(path.parent().expect("SQLite writer sidecar has a parent")).map_err(|error| DbError::Io(error.to_string()))?;
                        WalFileWriterGuard::try_acquire(&path).map(SqliteWalWriterGuard::Physical)
                    }
                })?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalWriter(permit))))
            }
            DbIoTask::WalCreate { document, index, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let changed = connection.execute("INSERT OR IGNORE INTO wal_segment (document, segment_index, bytes, sealed) VALUES (?1, ?2, x'', 0)", params![document.as_str(), index]).map_err(sqlite_err)?;
                if changed == 0 {
                    return Err(DbError::AlreadyExists(format!("WAL segment {index} for {} already exists", document.as_str())));
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                if !Self::write_stage_step(connection, sql_operation, input)? {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                let sealed: Option<i64> = transaction.query_row("SELECT sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                match sealed.map(decode_wal_segment_state).transpose()? {
                    None => return Err(DbError::NotFound(format!("WAL segment {index} not found"))),
                    Some(WalSegmentState::Sealed) => return Err(DbError::InvalidArgument("cannot append to sealed WAL segment".to_string())),
                    Some(WalSegmentState::Active) => {}
                }
                transaction.execute(WAL_APPEND_STAGE_SQL, params![document.as_str(), index, sql_operation]).map_err(sqlite_err)?;
                let length: i64 = transaction.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).map_err(sqlite_err)?;
                transaction.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                transaction.commit().map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(length as u64))))
            }
            DbIoTask::WalSync { .. } => {
                connection.execute_batch("PRAGMA wal_checkpoint(PASSIVE)").map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalSeal { document, index, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let changed = connection.execute("UPDATE wal_segment SET sealed = 1 WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index]).map_err(sqlite_err)?;
                if changed == 0 {
                    return Err(DbError::NotFound(format!("WAL segment {index} not found")));
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalRead { document, index, range, output, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let start = to_sql_i64(range.offset.saturating_add(1), "sqlite WAL offset")?;
                let length = to_sql_i64(range.len, "sqlite WAL length")?;
                let actual: Option<i64> = connection.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                let actual = actual.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))? as u64;
                let end = range.offset.checked_add(range.len).ok_or(DbError::LimitExceeded("sqlite WAL range"))?;
                if end > actual {
                    return Err(DbError::InvalidArgument("WAL read range exceeds segment length".to_string()));
                }
                Self::stage_read(
                    connection,
                    sql_operation,
                    |connection, operation| {
                        connection.execute(
                            "INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) SELECT ?1, substr(bytes, ?4, ?5), NULL FROM wal_segment WHERE document = ?2 AND segment_index = ?3",
                            params![operation, document.as_str(), index, start, length],
                        )
                    },
                    || DbError::NotFound(format!("WAL segment {index} not found")),
                )?;
                Self::read_stage_step(connection, sql_operation, output)
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let new_len = to_sql_i64(*new_len, "sqlite WAL truncate length")?;
                let current: Option<(i64, i64)> =
                    connection.query_row("SELECT length(bytes), sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(sqlite_err)?;
                let (current, sealed) = current.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?;
                if decode_wal_segment_state(sealed)? == WalSegmentState::Sealed || new_len > current {
                    return Err(DbError::InvalidArgument("invalid sealed or growing WAL truncation".to_string()));
                }
                connection.execute("UPDATE wal_segment SET bytes = substr(bytes, 1, ?3) WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index, new_len]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalDelete { document, index, .. } => {
                connection.execute("DELETE FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), to_sql_i64(*index, "sqlite WAL index")?]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                if !Self::write_stage_step(connection, sql_operation, input)? {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
                connection
                    .execute(
                        "INSERT INTO snapshot_generation (document, generation, bytes) SELECT ?1, ?2, bytes FROM db_io_stage WHERE operation = ?3 ON CONFLICT(document, generation) DO UPDATE SET bytes = excluded.bytes",
                        params![document.as_str(), to_sql_i64(*generation, "sqlite snapshot generation")?, sql_operation],
                    )
                    .map_err(sqlite_err)?;
                connection.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => {
                let generation = to_sql_i64(*generation, "sqlite snapshot generation")?;
                Self::stage_read(
                    connection,
                    sql_operation,
                    |connection, operation| {
                        connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) SELECT ?1, bytes, NULL FROM snapshot_generation WHERE document = ?2 AND generation = ?3", params![operation, document.as_str(), generation])
                    },
                    || DbError::NotFound(format!("snapshot generation {generation} not found")),
                )?;
                Self::read_stage_step(connection, sql_operation, output)
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                connection.execute("DELETE FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![document.as_str(), to_sql_i64(*generation, "sqlite snapshot generation")?]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::PayloadPut { input, .. } => {
                let Some(hash) = self.payload_stage_step(connection, operation, input)? else {
                    return Ok((DbIoExecutionStep::Yield, None));
                };
                connection.execute("INSERT OR IGNORE INTO payload (hash, bytes, len) SELECT ?1, bytes, length(bytes) FROM db_io_stage WHERE operation = ?2", params![hash.to_string(), sql_operation]).map_err(sqlite_err)?;
                connection.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Hash(hash))))
            }
            DbIoTask::PayloadGet { hash, output, .. } => {
                Self::stage_read(
                    connection,
                    sql_operation,
                    |connection, operation| connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) SELECT ?1, bytes, NULL FROM payload WHERE hash = ?2", params![operation, hash.to_string()]),
                    || DbError::NotFound(format!("payload {hash} not found")),
                )?;
                Self::read_stage_step(connection, sql_operation, output)
            }
            DbIoTask::PayloadDelete { hash, .. } => {
                connection.execute("DELETE FROM payload WHERE hash = ?1", params![hash.to_string()]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::CatalogRead { output, .. } => {
                let inserted = connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) SELECT ?1, bytes, epoch FROM catalog_root WHERE id = 0", params![sql_operation]).map_err(sqlite_err)?;
                let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM db_io_stage WHERE operation = ?1)", params![sql_operation], |row| row.get(0)).map_err(sqlite_err)?;
                if inserted == 0 && !exists {
                    return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalCatalog(None))));
                }
                let (step, result) = Self::read_stage_step(connection, sql_operation, output)?;
                let fence: i64 = connection.query_row("SELECT number FROM db_io_stage WHERE operation = ?1", params![sql_operation], |row| row.get(0)).map_err(sqlite_err)?;
                Ok((
                    step,
                    result.map(|result| match result {
                        DbIoResult::Pages(pages) => DbIoResult::OptionalCatalog(Some((pages, EpochFence { epoch: fence as u64 }))),
                        _ => unreachable!("SQLite catalog stage returns pages"),
                    }),
                ))
            }
            DbIoTask::CatalogCas { expected, input, .. } => {
                if !Self::write_stage_step(connection, sql_operation, input)? {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
                let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                let current: Option<i64> = transaction.query_row("SELECT epoch FROM catalog_root WHERE id = 0", [], |row| row.get(0)).optional().map_err(sqlite_err)?;
                expected.check(current.map_or(EpochFence::INITIAL, |epoch| EpochFence { epoch: epoch as u64 }))?;
                let next = expected.next();
                transaction
                    .execute(
                        "INSERT INTO catalog_root (id, bytes, epoch) SELECT 0, bytes, ?2 FROM db_io_stage WHERE operation = ?1 ON CONFLICT(id) DO UPDATE SET bytes = excluded.bytes, epoch = excluded.epoch",
                        params![sql_operation, to_sql_i64(next.epoch, "sqlite catalog epoch")?],
                    )
                    .map_err(sqlite_err)?;
                transaction.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                transaction.commit().map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(next))))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => {
                if !Self::write_stage_step(connection, sql_operation, input)? {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
                connection
                    .execute(
                        "INSERT INTO index_run (document, run_id, bytes) SELECT ?1, ?2, bytes FROM db_io_stage WHERE operation = ?3 ON CONFLICT(document, run_id) DO UPDATE SET bytes = excluded.bytes",
                        params![document.as_str(), to_sql_i64(*run_id, "sqlite index run")?, sql_operation],
                    )
                    .map_err(sqlite_err)?;
                connection.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => {
                let run_id = to_sql_i64(*run_id, "sqlite index run")?;
                Self::stage_read(
                    connection,
                    sql_operation,
                    |connection, operation| {
                        connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) SELECT ?1, bytes, NULL FROM index_run WHERE document = ?2 AND run_id = ?3", params![operation, document.as_str(), run_id])
                    },
                    || DbError::NotFound(format!("index run {run_id} not found")),
                )?;
                Self::read_stage_step(connection, sql_operation, output)
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                connection.execute("DELETE FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.as_str(), to_sql_i64(*run_id, "sqlite index run")?]).map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseAcquire { document, holder, now_ms, ttl_ms, .. } => {
                let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                let existing: Option<(String, i64, i64)> =
                    transaction.query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![document.as_str()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(sqlite_err)?;
                let fence = match existing {
                    Some((existing_holder, epoch, expires)) if *now_ms < expires as u64 => {
                        if existing_holder != holder.as_str() {
                            return Err(DbError::Conflict("resource is leased by another holder".to_string()));
                        }
                        EpochFence { epoch: epoch as u64 }
                    }
                    Some((_, epoch, _)) => EpochFence { epoch: epoch as u64 }.next(),
                    None => EpochFence::INITIAL,
                };
                let expires = (*now_ms).checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("sqlite lease expiry"))?;
                transaction
                    .execute(
                        "INSERT INTO lease (resource, holder, epoch, expires_at_ms) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(resource) DO UPDATE SET holder = excluded.holder, epoch = excluded.epoch, expires_at_ms = excluded.expires_at_ms",
                        params![document.as_str(), holder.as_str(), to_sql_i64(fence.epoch, "sqlite lease epoch")?, to_sql_i64(expires, "sqlite lease expiry")?],
                    )
                    .map_err(sqlite_err)?;
                transaction.commit().map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(fence))))
            }
            DbIoTask::LeaseRenew { document, holder, fence, now_ms, ttl_ms, .. } => {
                let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                let existing: Option<(String, i64, i64)> =
                    transaction.query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![document.as_str()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(sqlite_err)?;
                let (existing_holder, epoch, expires) = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                if *now_ms >= expires as u64 {
                    return Err(DbError::Unavailable("lease expired".to_string()));
                }
                if existing_holder != holder.as_str() {
                    return Err(DbError::Unauthorized("lease holder mismatch".to_string()));
                }
                fence.check(EpochFence { epoch: epoch as u64 })?;
                let expires = (*now_ms).checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("sqlite lease expiry"))?;
                transaction.execute("UPDATE lease SET expires_at_ms = ?2 WHERE resource = ?1", params![document.as_str(), to_sql_i64(expires, "sqlite lease expiry")?]).map_err(sqlite_err)?;
                transaction.commit().map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                let existing: Option<(String, i64)> = transaction.query_row("SELECT holder, epoch FROM lease WHERE resource = ?1", params![document.as_str()], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(sqlite_err)?;
                let (existing_holder, epoch) = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                if existing_holder != holder.as_str() {
                    return Err(DbError::Unauthorized("lease holder mismatch".to_string()));
                }
                fence.check(EpochFence { epoch: epoch as u64 })?;
                transaction.execute("DELETE FROM lease WHERE resource = ?1", params![document.as_str()]).map_err(sqlite_err)?;
                transaction.commit().map_err(sqlite_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let existing: Option<(String, i64, i64)> =
                    connection.query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![document.as_str()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(sqlite_err)?;
                let lease = match existing {
                    Some((holder, epoch, expires)) if *now_ms < expires as u64 => Some(DbIoLeaseResult::new(document.clone(), DbIoText::try_from_str(&holder)?, EpochFence { epoch: epoch as u64 }, expires as u64)),
                    _ => None,
                };
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLease(lease))))
            }
            DbIoTask::WalLength { .. }
            | DbIoTask::WalState { .. }
            | DbIoTask::WalList { .. }
            | DbIoTask::SnapshotLatest { .. }
            | DbIoTask::SnapshotList { .. }
            | DbIoTask::PayloadExists { .. }
            | DbIoTask::PayloadLength { .. }
            | DbIoTask::IndexList { .. }
            | DbIoTask::BackendOpen { .. }
            | DbIoTask::BackendClose { .. } => unreachable!("SQLite query and control tasks are answered before the writer match"),
        }
    }

    fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        let held = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().position(|owner| *owner == Some(operation));
        if let Some(index) = held {
            let mut reader = self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.end_read(index, operation, &mut reader, false)?;
            return Ok(false);
        }
        let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(connection) = owner.as_mut() {
            let sql_operation = Self::operation(operation)?;
            let staged: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM db_io_stage WHERE operation = ?1)", params![sql_operation], |row| row.get(0)).map_err(sqlite_err)?;
            if staged {
                connection.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![sql_operation]).map_err(sqlite_err)?;
                return Ok(false);
            }
        }
        let owner_slot = db_io_core::db_io_operation_owner_slot(operation).ok();
        for hash in owner_slot.map_or(&self.payload_hashes[..], |slot| std::slice::from_ref(&self.payload_hashes[slot])) {
            let mut hash = hash.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if hash.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                hash.take();
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        {
            let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(table) = owner.as_deref_mut() {
                if table.close_step()? {
                    return Ok(false);
                }
                if !table.terminal_is_empty() {
                    return Err(DbError::Internal("SQLite WAL writer table returned a false terminal witness".to_string()));
                }
                owner.take();
                return Ok(false);
            }
        }
        let cursor = self.backend_close_cursor.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if cursor < self.payload_hashes.len() {
            self.payload_hashes[cursor].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            return Ok(false);
        }
        let readers_end = self.payload_hashes.len() + SQLITE_READERS;
        if cursor < readers_end {
            let index = cursor - self.payload_hashes.len();
            self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index] = None;
            return Ok(false);
        }
        if cursor == readers_end {
            self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            return Ok(false);
        }
        if cursor == readers_end + 1 {
            self.canonical_database.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            return Ok(false);
        }
        self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
        Ok(true)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire)
            && self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.readers.iter().all(|reader| reader.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
            && self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().all(Option::is_none)
            && self.canonical_database.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.payload_hashes.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
    }
}
//#endregion 🔖️Executor
