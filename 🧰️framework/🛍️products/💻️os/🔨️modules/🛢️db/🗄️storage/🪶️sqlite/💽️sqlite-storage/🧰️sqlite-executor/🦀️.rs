//! 🧰️ Sqlite executor: the direct storage operations of the sqlite executor. See [`super`].

use super::*;

//#region 🔖️Authority
impl SqliteDbIoExecutor {
    pub(super) fn new(path: DbIoText, in_memory: bool) -> Self {
        Self {
            connection: Mutex::new(None),
            readers: [const { Mutex::new(None) }; SQLITE_READERS],
            reader_owners: Mutex::new([None; SQLITE_READERS]),
            path,
            in_memory,
            canonical_database: Mutex::new(None),
            writers: Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
            payload_hashes: [const { Mutex::new(None) }; SQLITE_OPERATION_OWNERS],
            backend_close_cursor: std::sync::atomic::AtomicUsize::new(0),
            backend_terminal: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub(super) fn operation(operation: u64) -> Result<i64, DbError> {
        to_sql_i64(operation, "sqlite DB I/O operation")
    }

    pub(super) fn physical_writer_sidecar(&self, document: &DbIoText) -> Result<std::path::PathBuf, DbError> {
        let canonical = self.canonical_database.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let canonical = canonical.as_ref().ok_or_else(|| DbError::Closed)?;
        let path = std::path::Path::new(canonical.as_str());
        let parent = path.parent().ok_or_else(|| DbError::InvalidArgument("canonical SQLite database has no parent".to_string()))?;
        let directory = parent.join(".semio-wal-writer");
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(canonical.as_str().as_bytes());
        hash.update(&[0]);
        hash.update(document.as_str().as_bytes());
        Ok(directory.join(format!("{}.lock", semio_framework_hash::hex_lower(&hash.finalize()))))
    }

    fn ensure_write_stage(connection: &Connection, operation: i64) -> Result<(), DbError> {
        connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) VALUES (?1, x'', NULL)", params![operation]).map_err(sqlite_err)?;
        Ok(())
    }

    pub(super) fn write_stage_step(connection: &Connection, operation: i64, input: &mut DbIoPages) -> Result<bool, DbError> {
        Self::ensure_write_stage(connection, operation)?;
        let Some(fragment) = input.page(0) else { return Ok(true) };
        let fragment_len = fragment.len();
        connection.execute(STAGE_APPEND_SQL, params![operation, fragment]).map_err(sqlite_err)?;
        input.advance(fragment_len)?;
        Ok(false)
    }

    pub(super) fn read_stage_step(connection: &Connection, operation: i64, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let total: i64 =
            connection.query_row("SELECT length(bytes) FROM db_io_stage WHERE operation = ?1", params![operation], |row| row.get(0)).optional().map_err(sqlite_err)?.ok_or_else(|| DbError::NotFound("SQLite DB I/O stage not found".to_string()))?;
        check_len(total as u64, MAX_BLOB_BYTES, "sqlite retained stage read")?;
        if output.len() == total as usize {
            return match output.seal_retained_step()? {
                Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
                None => Ok((DbIoExecutionStep::Yield, None)),
            };
        }
        let offset = to_sql_i64(output.len() as u64 + 1, "sqlite retained read offset")?;
        let fragment: Vec<u8> = connection.query_row("SELECT substr(bytes, ?2, ?3) FROM db_io_stage WHERE operation = ?1", params![operation, offset, DB_IO_PAGE_BYTES as i64], |row| row.get(0)).map_err(sqlite_err)?;
        if fragment.is_empty() || fragment.len() > DB_IO_PAGE_BYTES {
            return Err(DbError::Corrupt("SQLite returned an invalid fixed DB I/O fragment".to_string()));
        }
        output.write_fragment(&fragment)?;
        Ok((DbIoExecutionStep::Yield, None))
    }

    fn list_step(connection: &Connection, sql: &str, document: &DbIoText, output: &mut DbIoU64List) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let offset = to_sql_i64(output.len() as u64, "sqlite list cursor")?;
        let next: Option<i64> = connection.query_row(sql, params![document.as_str(), offset], |row| row.get(0)).optional().map_err(sqlite_err)?;
        if let Some(next) = next {
            output.push(next as u64)?;
            Ok((DbIoExecutionStep::Yield, None))
        } else {
            Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
        }
    }

    pub(super) fn payload_stage_step(&self, connection: &Connection, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
        let sql_operation = Self::operation(operation)?;
        Self::ensure_write_stage(connection, sql_operation)?;
        let slot = db_io_core::db_io_operation_owner_slot(operation)?;
        let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
            return Err(DbError::Unavailable("SQLite payload hash cursor capacity exhausted".to_string()));
        }
        let (_, hasher) = state.get_or_insert_with(|| (operation, semio_framework_hash::Hasher::new()));
        if let Some(fragment) = input.page(0) {
            let fragment_len = fragment.len();
            connection.execute(STAGE_APPEND_SQL, params![sql_operation, fragment]).map_err(sqlite_err)?;
            hasher.update(fragment);
            input.advance(fragment_len)?;
            return Ok(None);
        }
        let (_, hasher) = state.take().expect("SQLite payload hash cursor retained");
        Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
    }

    pub(super) fn stage_read(connection: &Connection, operation: i64, insert: impl FnOnce(&Connection, i64) -> Result<usize, rusqlite::Error>, missing: impl FnOnce() -> DbError) -> Result<(), DbError> {
        let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM db_io_stage WHERE operation = ?1)", params![operation], |row| row.get(0)).map_err(sqlite_err)?;
        if !exists && insert(connection, operation).map_err(sqlite_err)? == 0 {
            return Err(missing());
        }
        Ok(())
    }

    /// 🔎️ The reads that never stage — one statement, or one listed row per step — answered alike on the writer and on a
    /// reader; `None` for every other task.
    pub(super) fn query_step(connection: &Connection, task: &mut DbIoTask) -> Result<Option<(DbIoExecutionStep, Option<DbIoResult>)>, DbError> {
        let step = match task {
            DbIoTask::WalLength { document, index, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let length: Option<i64> = connection.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                (DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))? as u64)))
            }
            DbIoTask::WalState { document, index, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let sealed: Option<i64> = connection.query_row("SELECT sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                let state = decode_wal_segment_state(sealed.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?)?;
                (DbIoExecutionStep::Complete, Some(DbIoResult::WalSegmentState(state)))
            }
            DbIoTask::WalList { document, output, .. } => Self::list_step(connection, "SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC LIMIT 1 OFFSET ?2", document, output)?,
            DbIoTask::SnapshotLatest { document, .. } => {
                let latest: Option<i64> = connection.query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.as_str()], |row| row.get(0)).map_err(sqlite_err)?;
                (DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(latest.map(|value| value as u64))))
            }
            DbIoTask::SnapshotList { document, output, .. } => Self::list_step(connection, "SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC LIMIT 1 OFFSET ?2", document, output)?,
            DbIoTask::PayloadExists { hash, .. } => {
                let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)?;
                (DbIoExecutionStep::Complete, Some(DbIoResult::Exists(exists)))
            }
            DbIoTask::PayloadLength { hash, .. } => {
                let length: Option<i64> = connection.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?;
                (DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))? as u64)))
            }
            DbIoTask::IndexList { document, output, .. } => Self::list_step(connection, "SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC LIMIT 1 OFFSET ?2", document, output)?,
            _ => return Ok(None),
        };
        Ok(Some(step))
    }

    /// 📍️ The rowid and byte length of the one row `sql` selects as `(rowid, length(bytes))`, inside the caller's snapshot.
    fn locate_blob(connection: &Connection, sql: &str, params: impl rusqlite::Params) -> Result<Option<(i64, u64)>, DbError> {
        connection.query_row(sql, params, |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)? as u64))).optional().map_err(sqlite_err)
    }

    /// 📖️ One page of a blob read on a reader, straight from its read transaction's snapshot: the row is located again in
    /// the same snapshot every step and only the pages the range covers are read (SQLite incremental blob I/O), so a
    /// multi-page read never mixes two states and never stages through `db_io_stage`.
    fn snapshot_blob_step(connection: &Connection, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        match task {
            DbIoTask::WalRead { document, index, range, output, .. } => {
                let index = to_sql_i64(*index, "sqlite WAL index")?;
                let (row, actual) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index])?.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?;
                let end = range.offset.checked_add(range.len).ok_or(DbError::LimitExceeded("sqlite WAL range"))?;
                if end > actual {
                    return Err(DbError::InvalidArgument("WAL read range exceeds segment length".to_string()));
                }
                Self::blob_page_step(connection, c"wal_segment", row, range.offset, range.len, output)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => {
                let generation = to_sql_i64(*generation, "sqlite snapshot generation")?;
                let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![document.as_str(), generation])?.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} not found")))?;
                Self::blob_page_step(connection, c"snapshot_generation", row, 0, total, output)
            }
            DbIoTask::PayloadGet { hash, output, .. } => {
                let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM payload WHERE hash = ?1", params![hash.to_string()])?.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
                Self::blob_page_step(connection, c"payload", row, 0, total, output)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => {
                let run_id = to_sql_i64(*run_id, "sqlite index run")?;
                let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.as_str(), run_id])?.ok_or_else(|| DbError::NotFound(format!("index run {run_id} not found")))?;
                Self::blob_page_step(connection, c"index_run", row, 0, total, output)
            }
            _ => Err(DbError::Internal("SQLite reader received a task that is not a blob read".to_string())),
        }
    }

    /// 📄️ Fills the output's current page from `[start + written, …)` of the blob in `table` row `row`, or seals the output
    /// once all `total` bytes are in.
    fn blob_page_step(connection: &Connection, table: &std::ffi::CStr, row: i64, start: u64, total: u64, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        check_len(total, MAX_BLOB_BYTES, "sqlite retained stage read")?;
        let written = output.len();
        let remaining = total.checked_sub(written as u64).ok_or_else(|| DbError::Corrupt("SQLite blob read passed the end of its row".to_string()))?;
        if remaining == 0 {
            return match output.seal_retained_step()? {
                Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
                None => Ok((DbIoExecutionStep::Yield, None)),
            };
        }
        let len = (DB_IO_PAGE_BYTES - written % DB_IO_PAGE_BYTES).min(remaining as usize);
        let offset = usize::try_from(start + written as u64).map_err(|_| DbError::LimitExceeded("sqlite blob offset"))?;
        let mut fragment = [0_u8; DB_IO_PAGE_BYTES];
        connection.blob_open(c"main", table, c"bytes", row, true).map_err(sqlite_err)?.read_at_exact(&mut fragment[..len], offset).map_err(sqlite_err)?;
        output.write_fragment(&fragment[..len])?;
        Ok((DbIoExecutionStep::Yield, None))
    }

    /// 📖️ The reader `operation` already holds, else — when `may_claim` (the read has made no progress on the writer) — a
    /// free one; `None` sends the step to the writer connection.
    fn claim_reader(&self, operation: u64, may_claim: bool) -> Option<usize> {
        let mut owners = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = owners.iter().position(|owner| *owner == Some(operation)) {
            return Some(index);
        }
        let index = owners.iter().position(Option::is_none).filter(|_| may_claim)?;
        owners[index] = Some(operation);
        Some(index)
    }

    /// 🔚️ Ends the read transaction of the reader at `index` — committed after a completed read, rolled back otherwise; a
    /// connection that cannot end it is dropped (which rolls back) — and frees the reader. Only a failed commit is an error.
    pub(super) fn end_read(&self, index: usize, operation: u64, reader: &mut Option<Connection>, completed: bool) -> Result<(), DbError> {
        let ended = match reader.as_ref() {
            Some(connection) if !connection.is_autocommit() => connection.execute_batch(if completed { "COMMIT" } else { "ROLLBACK" }).map_err(sqlite_err),
            _ => Ok(()),
        };
        if ended.is_err() {
            reader.take();
        }
        let mut owners = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if owners[index] == Some(operation) {
            owners[index] = None;
        }
        if completed {
            ended
        } else {
            Ok(())
        }
    }

    /// 📖️ One read step of a file database on a WAL reader: opened read-only on first use, a read transaction per
    /// operation (the snapshot every step of a multi-step read sees), ended when the read completes or fails. `None` for
    /// in-memory databases (one connection is the database), for tasks that are not reads, and when every reader is held.
    pub(super) fn reader_step(&self, operation: u64, task: &mut DbIoTask) -> Result<Option<(DbIoExecutionStep, Option<DbIoResult>)>, DbError> {
        let may_claim = match task {
            DbIoTask::WalRead { output, .. } | DbIoTask::SnapshotRead { output, .. } | DbIoTask::PayloadGet { output, .. } | DbIoTask::IndexRead { output, .. } => output.is_empty(),
            DbIoTask::WalList { output, .. } | DbIoTask::SnapshotList { output, .. } | DbIoTask::IndexList { output, .. } => output.is_empty(),
            DbIoTask::WalLength { .. } | DbIoTask::WalState { .. } | DbIoTask::SnapshotLatest { .. } | DbIoTask::PayloadExists { .. } | DbIoTask::PayloadLength { .. } => true,
            _ => return Ok(None),
        };
        if self.in_memory {
            return Ok(None);
        }
        let Some(index) = self.claim_reader(operation, may_claim) else { return Ok(None) };
        let mut reader = self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let outcome = Self::read_on(&mut reader, &self.path, task);
        if matches!(outcome, Ok((DbIoExecutionStep::Yield, _))) {
            return outcome.map(Some);
        }
        let ended = self.end_read(index, operation, &mut reader, outcome.is_ok());
        let step = outcome?;
        ended?;
        Ok(Some(step))
    }

    fn read_on(reader: &mut Option<Connection>, path: &DbIoText, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        if reader.is_none() {
            let connection = Connection::open_with_flags(path.as_str(), OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX | OpenFlags::SQLITE_OPEN_URI).map_err(sqlite_err)?;
            connection.pragma_update(None, "cache_size", -SQLITE_READER_CACHE_KIB).map_err(sqlite_err)?;
            *reader = Some(connection);
        }
        let connection = reader.as_ref().expect("SQLite reader connection opened");
        if connection.is_autocommit() {
            connection.execute_batch("BEGIN").map_err(sqlite_err)?;
        }
        match Self::query_step(connection, task)? {
            Some(step) => Ok(step),
            None => Self::snapshot_blob_step(connection, task),
        }
    }
}
//#endregion 🔖️Authority
