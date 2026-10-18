//! 🧬️ Sqlite schema and connection: the schema statements, the limits and statements, the error and integer conversions, the connection initialization, the wal writer guard and the executor type. See [`super`].

use super::*;

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
pub(super) const MAX_BLOB_BYTES: u64 = 496 * 1024;

pub(super) const SQLITE_OPERATION_OWNERS: usize = db_io_core::DB_IO_LEDGER_ITEMS;

/// 📖️ WAL reader connections of a file database: a read that starts while one is free runs on it, beside the writer and
/// the other readers, inside its own read transaction, and never writes.
pub(super) const SQLITE_READERS: usize = 16;

/// 📖️ Page-cache ceiling of one reader in KiB (`PRAGMA cache_size` takes it negated): blob reads stream each page once.
pub(super) const SQLITE_READER_CACHE_KIB: i64 = 1024;

pub(super) const STAGE_APPEND_SQL: &str = "UPDATE db_io_stage SET bytes = CAST(bytes || ?2 AS BLOB) WHERE operation = ?1";

pub(super) const WAL_APPEND_STAGE_SQL: &str = "UPDATE wal_segment SET bytes = CAST(bytes || (SELECT bytes FROM db_io_stage WHERE operation = ?3) AS BLOB) WHERE document = ?1 AND segment_index = ?2";

pub(super) fn sqlite_err(error: rusqlite::Error) -> DbError {
    DbError::Io(error.to_string())
}

pub(super) fn to_sql_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
    i64::try_from(value).map_err(|_| DbError::LimitExceeded(what))
}

pub(super) fn decode_wal_segment_state(sealed: i64) -> Result<WalSegmentState, DbError> {
    match sealed {
        0 => Ok(WalSegmentState::Active),
        1 => Ok(WalSegmentState::Sealed),
        value => Err(DbError::Corrupt(format!("SQLite WAL sealed flag is {value}"))),
    }
}

pub(super) fn init_connection(connection: &Connection) -> Result<(), DbError> {
    connection.pragma_update(None, "journal_mode", "WAL").map_err(sqlite_err)?;
    connection.pragma_update(None, "synchronous", "FULL").map_err(sqlite_err)?;
    connection.pragma_update(None, "foreign_keys", "OFF").map_err(sqlite_err)?;
    connection.execute_batch(SCHEMA).map_err(sqlite_err)
}

pub(super) enum SqliteWalWriterGuard {
    Local,
    Physical(WalFileWriterGuard),
}

impl WalWriterGuard for SqliteWalWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError> {
        match self {
            Self::Local => Ok(false),
            Self::Physical(guard) => guard.close_step(),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Local => true,
            Self::Physical(guard) => guard.terminal_is_empty(),
        }
    }
}

pub(super) struct SqliteDbIoExecutor {
    pub(super) connection: Mutex<Option<Connection>>,
    pub(super) readers: [Mutex<Option<Connection>>; SQLITE_READERS],
    pub(super) reader_owners: Mutex<[Option<u64>; SQLITE_READERS]>,
    pub(super) path: DbIoText,
    pub(super) in_memory: bool,
    pub(super) canonical_database: Mutex<Option<DbIoText>>,
    pub(super) writers: Mutex<Option<Box<WalWriterTable<SqliteWalWriterGuard>>>>,
    pub(super) payload_hashes: [Mutex<Option<(u64, semio_framework_hash::Hasher)>>; SQLITE_OPERATION_OWNERS],
    pub(super) backend_close_cursor: std::sync::atomic::AtomicUsize,
    pub(super) backend_terminal: std::sync::atomic::AtomicBool,
}
//#endregion 🔖️Authority
