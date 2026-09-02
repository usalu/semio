//! 🗄️ SQLite storage behind the process-wide typed, retained database I/O lane.

//#region 🔖️SqliteStorage
#[cfg(not(target_arch = "wasm32"))]
mod sqlite_storage {
    use crate::db_durability::{DurabilityClass, EpochFence};
    use crate::db_ids::{check_len, ArtifactId, DbError};
    use crate::db_storage::{
        close_db_io_backend, register_db_io_backend, retire_db_io_backend, submit_db_io_task, CatalogStorage, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected,
        DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage, DB_IO_PAGE_BYTES,
    };
    use pack::{ByteRange, ContentHash};
    use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
    use semio_framework_async::WorkerPool;
    use std::sync::{Arc, Mutex};

    //#region 🔖️Schema
    const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS wal_segment (
    document TEXT NOT NULL,
    segment_index INTEGER NOT NULL,
    bytes BLOB NOT NULL,
    sealed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (document, segment_index)
);
CREATE TABLE IF NOT EXISTS snapshot_generation (
    document TEXT NOT NULL,
    generation INTEGER NOT NULL,
    bytes BLOB NOT NULL,
    PRIMARY KEY (document, generation)
);
CREATE TABLE IF NOT EXISTS payload (
    hash TEXT PRIMARY KEY,
    bytes BLOB NOT NULL,
    len INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS catalog_root (
    id INTEGER PRIMARY KEY CHECK (id = 0),
    bytes BLOB NOT NULL,
    epoch INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS index_run (
    document TEXT NOT NULL,
    run_id INTEGER NOT NULL,
    bytes BLOB NOT NULL,
    PRIMARY KEY (document, run_id)
);
CREATE TABLE IF NOT EXISTS lease (
    resource TEXT PRIMARY KEY,
    holder TEXT NOT NULL,
    epoch INTEGER NOT NULL,
    expires_at_ms INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS db_io_stage (
    operation INTEGER PRIMARY KEY,
    bytes BLOB NOT NULL,
    number INTEGER
);
";
    //#endregion 🔖️Schema

    //#region 🔖️Authority
    const MAX_BLOB_BYTES: u64 = 496 * 1024;
    const SQLITE_OPERATION_OWNERS: usize = 64;

    fn sqlite_err(error: rusqlite::Error) -> DbError {
        DbError::Io(error.to_string())
    }

    fn to_sql_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
        i64::try_from(value).map_err(|_| DbError::LimitExceeded(what))
    }

    fn init_connection(connection: &Connection) -> Result<(), DbError> {
        connection.pragma_update(None, "journal_mode", "WAL").map_err(sqlite_err)?;
        connection.pragma_update(None, "synchronous", "FULL").map_err(sqlite_err)?;
        connection.pragma_update(None, "foreign_keys", "OFF").map_err(sqlite_err)?;
        connection.execute_batch(SCHEMA).map_err(sqlite_err)
    }

    struct SqliteDbIoExecutor {
        connection: Mutex<Option<Connection>>,
        path: DbIoText,
        in_memory: bool,
        payload_hashes: [Mutex<Option<(u64, semio_framework_hash::Hasher)>>; SQLITE_OPERATION_OWNERS],
        backend_close_cursor: std::sync::atomic::AtomicUsize,
        backend_terminal: std::sync::atomic::AtomicBool,
    }

    impl SqliteDbIoExecutor {
        fn new(path: DbIoText, in_memory: bool) -> Self {
            Self {
                connection: Mutex::new(None),
                path,
                in_memory,
                payload_hashes: [const { Mutex::new(None) }; SQLITE_OPERATION_OWNERS],
                backend_close_cursor: std::sync::atomic::AtomicUsize::new(0),
                backend_terminal: std::sync::atomic::AtomicBool::new(false),
            }
        }

        fn operation(operation: u64) -> Result<i64, DbError> {
            to_sql_i64(operation, "sqlite DB I/O operation")
        }

        fn ensure_write_stage(connection: &Connection, operation: i64) -> Result<(), DbError> {
            connection.execute("INSERT OR IGNORE INTO db_io_stage (operation, bytes, number) VALUES (?1, x'', NULL)", params![operation]).map_err(sqlite_err)?;
            Ok(())
        }

        fn write_stage_step(connection: &Connection, operation: i64, input: &mut DbIoPages) -> Result<bool, DbError> {
            Self::ensure_write_stage(connection, operation)?;
            let Some(fragment) = input.page(0) else { return Ok(true) };
            let fragment_len = fragment.len();
            connection.execute("UPDATE db_io_stage SET bytes = bytes || ?2 WHERE operation = ?1", params![operation, fragment]).map_err(sqlite_err)?;
            input.advance(fragment_len)?;
            Ok(false)
        }

        fn read_stage_step(connection: &Connection, operation: i64, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
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

        fn payload_stage_step(&self, connection: &Connection, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
            let sql_operation = Self::operation(operation)?;
            Self::ensure_write_stage(connection, sql_operation)?;
            let slot = operation as usize % self.payload_hashes.len();
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
                return Err(DbError::Unavailable("SQLite payload hash cursor capacity exhausted".to_string()));
            }
            let (_, hasher) = state.get_or_insert_with(|| (operation, semio_framework_hash::Hasher::new()));
            if let Some(fragment) = input.page(0) {
                let fragment_len = fragment.len();
                connection.execute("UPDATE db_io_stage SET bytes = bytes || ?2 WHERE operation = ?1", params![sql_operation, fragment]).map_err(sqlite_err)?;
                hasher.update(fragment);
                input.advance(fragment_len)?;
                return Ok(None);
            }
            let (_, hasher) = state.take().expect("SQLite payload hash cursor retained");
            Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
        }

        fn stage_read(connection: &Connection, operation: i64, insert: impl FnOnce(&Connection, i64) -> Result<usize, rusqlite::Error>, missing: impl FnOnce() -> DbError) -> Result<(), DbError> {
            let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM db_io_stage WHERE operation = ?1)", params![operation], |row| row.get(0)).map_err(sqlite_err)?;
            if !exists && insert(connection, operation).map_err(sqlite_err)? == 0 {
                return Err(missing());
            }
            Ok(())
        }
    }
    //#endregion 🔖️Authority

    //#region 🔖️Executor
    impl DbIoTaskExecutor for SqliteDbIoExecutor {
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
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
                    *owner = Some(connection);
                }
                return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));
            }
            if matches!(task, DbIoTask::BackendClose { .. }) {
                return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));
            }
            let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let connection = owner.as_mut().ok_or(DbError::Closed)?;
            let sql_operation = Self::operation(operation)?;
            match task {
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
                    match sealed {
                        None => return Err(DbError::NotFound(format!("WAL segment {index} not found"))),
                        Some(1) => return Err(DbError::InvalidArgument("cannot append to sealed WAL segment".to_string())),
                        _ => {}
                    }
                    transaction
                        .execute("UPDATE wal_segment SET bytes = bytes || (SELECT bytes FROM db_io_stage WHERE operation = ?3) WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index, sql_operation])
                        .map_err(sqlite_err)?;
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
                DbIoTask::WalLength { document, index, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let length: Option<i64> = connection.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))? as u64))))
                }
                DbIoTask::WalList { document, output, .. } => Self::list_step(connection, "SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC LIMIT 1 OFFSET ?2", document, output),
                DbIoTask::WalTruncate { document, index, new_len, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let new_len = to_sql_i64(*new_len, "sqlite WAL truncate length")?;
                    let current: Option<(i64, i64)> =
                        connection.query_row("SELECT length(bytes), sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(sqlite_err)?;
                    let (current, sealed) = current.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?;
                    if sealed != 0 || new_len > current {
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
                DbIoTask::SnapshotLatest { document, .. } => {
                    let latest: Option<i64> = connection.query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.as_str()], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(latest.map(|value| value as u64)))))
                }
                DbIoTask::SnapshotList { document, output, .. } => Self::list_step(connection, "SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC LIMIT 1 OFFSET ?2", document, output),
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
                DbIoTask::PayloadExists { hash, .. } => {
                    let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(exists))))
                }
                DbIoTask::PayloadLength { hash, .. } => {
                    let length: Option<i64> = connection.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))? as u64))))
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
                DbIoTask::IndexList { document, output, .. } => Self::list_step(connection, "SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC LIMIT 1 OFFSET ?2", document, output),
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
                DbIoTask::BackendOpen { .. } | DbIoTask::BackendClose { .. } => unreachable!("SQLite control tasks handled before connection lock"),
            }
        }

        fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
            let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(connection) = owner.as_mut() {
                if connection.execute("DELETE FROM db_io_stage WHERE operation = ?1", params![Self::operation(operation)?]).map_err(sqlite_err)? != 0 {
                    return Ok(false);
                }
            }
            let slot = operation as usize % self.payload_hashes.len();
            let mut hash = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if hash.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                hash.take();
                return Ok(false);
            }
            Ok(true)
        }

        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            let cursor = self.backend_close_cursor.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if cursor < self.payload_hashes.len() {
                self.payload_hashes[cursor].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            if cursor == self.payload_hashes.len() {
                self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
            Ok(true)
        }

        fn backend_terminal_is_empty(&self) -> bool {
            self.backend_terminal.load(std::sync::atomic::Ordering::Acquire)
                && self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
                && self.payload_hashes.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
        }
    }
    //#endregion 🔖️Executor

    //#region 🔖️Facade
    pub struct SqliteStorage {
        control: DbIoBackendControl,
        pool: Arc<WorkerPool>,
        closed: std::sync::atomic::AtomicBool,
    }

    async fn execute(pool: &WorkerPool, task: DbIoTask) -> Result<DbIoResult, DbError> {
        submit_db_io_task(pool, task).map_err(|(error, _)| error)?.await.map_err(crate::db_storage::DbIoFault::into_db_error)?.into_result()
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

    fn length(result: DbIoResult) -> Result<u64, DbError> {
        match result {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(result_fault("length")),
        }
    }

    fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
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

    impl SqliteStorage {
        async fn open_owned(pool: Arc<WorkerPool>, path: DbIoText, in_memory: bool) -> Result<Self, DbError> {
            let executor = Box::new(SqliteDbIoExecutor::new(path.clone(), in_memory));
            let control = register_db_io_backend(DbIoBackendKind::Sqlite, executor, pool.clone())?;
            if let Err(error) = execute(pool.as_ref(), DbIoTask::BackendOpen { backend: control, path }).await {
                let _ = execute(pool.as_ref(), DbIoTask::BackendClose { backend: control }).await;
                return Err(error);
            }
            Ok(Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) })
        }

        pub async fn open(pool: Arc<WorkerPool>, path: &std::path::Path) -> Result<Self, DbError> {
            let path = path.to_str().ok_or_else(|| DbError::InvalidArgument("SQLite path is not UTF-8".to_string()))?;
            Self::open_owned(pool, DbIoText::try_from_str(path)?, false).await
        }

        pub async fn open_in_memory(pool: Arc<WorkerPool>) -> Result<Self, DbError> {
            Self::open_owned(pool, DbIoText::try_from_str(":memory:")?, true).await
        }

        pub async fn close(&self) -> Result<(), DbError> {
            let result = unit(execute(self.pool.as_ref(), DbIoTask::BackendClose { backend: self.control }).await?);
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
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalCreate { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite WAL append")?;
            length(execute(self.pool.as_ref(), DbIoTask::WalAppend { backend: self.control, document: document_text(document)?, index, input: bytes }).await?)
        }

        async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSync { backend: self.control, document: document_text(document)?, index, class }).await?)
        }

        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalSeal { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
            check_len(range.len, MAX_BLOB_BYTES, "sqlite WAL read")?;
            pages(execute(self.pool.as_ref(), DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output: output_writer(range.len)? }).await?)
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::WalList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalTruncate { backend: self.control, document: document_text(document)?, index, new_len }).await?)
        }

        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::WalDelete { backend: self.control, document: document_text(document)?, index }).await?)
        }
    }

    impl SnapshotStorage for SqliteStorage {
        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite snapshot write")?;
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(MAX_BLOB_BYTES)? }).await?)
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await? {
                DbIoResult::OptionalLength(value) => Ok(value),
                _ => Err(result_fault("optional generation")),
            }
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
        }
    }

    impl PayloadStorage for SqliteStorage {
        async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite payload put")?;
            match execute(self.pool.as_ref(), DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
                DbIoResult::Hash(hash) => Ok(hash),
                _ => Err(result_fault("hash")),
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(MAX_BLOB_BYTES)? }).await?)
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
                DbIoResult::Exists(value) => Ok(value),
                _ => Err(result_fault("existence")),
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            length(execute(self.pool.as_ref(), DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
        }
    }

    impl CatalogStorage for SqliteStorage {
        async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::CatalogRead { backend: self.control, output: output_writer(MAX_BLOB_BYTES)? }).await? {
                DbIoResult::OptionalCatalog(value) => Ok(value),
                _ => Err(result_fault("optional catalog")),
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
            check_len(new_bytes.len() as u64, MAX_BLOB_BYTES, "sqlite catalog CAS")?;
            match execute(self.pool.as_ref(), DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
                DbIoResult::Fence(value) => Ok(value),
                _ => Err(result_fault("fence")),
            }
        }
    }

    impl IndexStorage for SqliteStorage {
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "sqlite index write")?;
            unit(execute(self.pool.as_ref(), DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
            pages(execute(self.pool.as_ref(), DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(MAX_BLOB_BYTES)? }).await?)
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(self.pool.as_ref(), DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
        }
    }

    impl LeaseStorage for SqliteStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
                DbIoResult::Fence(value) => Ok(value),
                _ => Err(result_fault("fence")),
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            unit(execute(self.pool.as_ref(), DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            match execute(self.pool.as_ref(), DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
                DbIoResult::OptionalLease(value) => Ok(value),
                _ => Err(result_fault("optional lease")),
            }
        }
    }
    //#endregion 🔖️Facade

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        async fn pages(bytes: &[u8]) -> DbIoPages {
            crate::db_storage::db_io_copy_pages(bytes).unwrap().await.unwrap()
        }

        #[semio_framework_async_macros::async_test]
        async fn typed_lane_is_lossless_at_page_boundary_and_zero() {
            let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();
            let document: ArtifactId = "typed-sqlite".into();
            storage.create_segment(&document, 0).await.unwrap();
            let bytes = vec![0x5a; DB_IO_PAGE_BYTES + 1];
            assert_eq!(storage.append(&document, 0, pages(&bytes).await).await.unwrap(), bytes.len() as u64);
            assert_eq!(storage.read(&document, 0, ByteRange { offset: 0, len: bytes.len() as u64 }).await.unwrap(), bytes);
            let hash = storage.put(pages(&[]).await).await.unwrap();
            assert_eq!(storage.get(&hash).await.unwrap(), b"");
            storage.close().await.unwrap();
        }

        #[semio_framework_async_macros::async_test]
        async fn typed_list_and_catalog_cas_are_stable() {
            let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();
            let document: ArtifactId = "typed-list".into();
            storage.write_generation(&document, 2, pages(b"two").await).await.unwrap();
            storage.write_generation(&document, 1, pages(b"one").await).await.unwrap();
            assert_eq!(storage.list_generations(&document).await.unwrap(), [1, 2]);
            assert_eq!(storage.latest_generation(&document).await.unwrap(), Some(2));
            let fence = storage.cas_root(EpochFence::INITIAL, pages(b"root").await).await.unwrap();
            assert_eq!(storage.read_root().await.unwrap().unwrap().1, fence);
            assert!(matches!(storage.cas_root(EpochFence::INITIAL, pages(b"stale").await).await, Err(DbError::Fenced { .. })));
        }
    }
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite_storage::SqliteStorage;
//#endregion 🔖️SqliteStorage
