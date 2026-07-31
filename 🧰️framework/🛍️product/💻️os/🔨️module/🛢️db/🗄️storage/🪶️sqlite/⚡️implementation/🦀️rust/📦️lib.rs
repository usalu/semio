//! 🗄️ `db_storage_sqlite` — a `db_storage::DbStorage` backend over `rusqlite` (bundled SQLite),
//! the single-file, zero-service storage substrate option for the `db` crate family. Implements
//! every sub-trait (`WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) directly against one `rusqlite::Connection`, matching
//! `db_storage::FsStorage`'s semantics byte-for-byte (same error taxonomy, same idempotence/CAS
//! laws) but persisted in SQLite tables instead of files. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate links the SAME bundled `rusqlite`/`libsqlite3-sys` version
//! already pinned in the workspace `Cargo.lock` by `vcs` and
//! `framework/product/os/semio_hub/storage/sqlite` (a Cargo workspace may only link one native
//! `sqlite3` — `links = "sqlite3"` — so a version drift here would be a hard build break, not a
//! style choice). Bundled sqlite's C build doesn't target `wasm32-unknown-unknown`, so — per the
//! contract's "everything storage/thread-touching is `#[cfg(not(target_arch = "wasm32"))]`
//! module-wrapped" rule, and mirroring this crate's own `Cargo.toml` gating `rusqlite` under
//! `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` — the entire implementation lives
//! in a wasm32-gated inner module; a wasm32 build of this crate is an (intentionally) empty shell.
//!
//! 🔒️ Durability choice: the connection is opened with `PRAGMA synchronous = FULL` (in `WAL`
//! journal mode, this fsyncs the WAL file on every commit — see SQLite's own docs on
//! `synchronous`/`journal_mode`). Every write in this crate is a single autocommit statement (or
//! an explicit transaction for compare-and-swap paths), so by the time a call returns, its
//! effects are already at `DurabilityClass::Fsync` regardless of the class the caller requested —
//! `WalStorage::sync` is therefore a documented no-op: it can never under-deliver relative to what
//! the caller asked for, only (harmlessly) over-deliver for `Memory`/`Os` requests. A future
//! perf-motivated revision could relax `synchronous` and make `sync` do real work for batching
//! `Memory`/`Os`-class WAL appends; left as an extension seam since correctness, not throughput,
//! is this wave's goal.
//!
//! 🔐️ CAS choice: `CatalogStorage::cas_root` and every `LeaseStorage` mutation run inside an
//! `IMMEDIATE` SQLite transaction (acquires SQLite's own write lock up front), which — unlike
//! `db_storage::FsStorage`'s `Mutex`-guarded `write_atomic` (documented there as in-process-only
//! fencing) — genuinely serializes concurrent *cross-process* writers against the same `.sqlite3`
//! file, not just concurrent threads in this process.

//#region 🔖️SqliteStorage
#[cfg(not(target_arch = "wasm32"))]
mod sqlite_storage {
    use db_core::{DbError, DocumentId, DurabilityClass, EpochFence, check_len};
    use db_storage::{CatalogStorage, DbStorage, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use pack::{ByteRange, ContentHash};
    use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
    use std::sync::Mutex;

    //#region 🔖️Schema
    /// @emoji 🧱️ One document's WAL segment, keyed `(document, segment_index)`. `sealed` is
    /// stored as `0`/`1` (SQLite has no native boolean). `bytes` grows via `bytes || ?` (`append`)
    /// or shrinks via `substr(bytes, 1, ?)` (`truncate_tail`) so the full segment never
    /// round-trips through Rust memory just to grow it by a few bytes.
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
";
    //#endregion 🔖️Schema

    //#region 🔖️Limits
    /// @emoji 🛡️ Ceiling on any single blob this crate reads into memory in one call — validated
    /// via `db_core::check_len` BEFORE the read buffer is allocated. Mirrors
    /// `db_storage::MemoryStorage`/`FsStorage`'s own `MAX_READ_BYTES` choice (same number, kept
    /// in lock-step deliberately: a caller swapping backends should hit the same ceiling on
    /// every backend).
    const MAX_BLOB_BYTES: u64 = 1024 * 1024 * 1024;
    //#endregion 🔖️Limits

    //#region 🔖️Errors
    /// @emoji 🚨️ Wraps a `rusqlite::Error` into `DbError::Io` — the only place a foreign driver
    /// error type is allowed to appear, per the contract's "no foreign error type in a public
    /// signature" rule. Every not-found/already-exists/conflict distinction this crate needs is
    /// produced by an explicit existence check or an SQL `changes()` count before this ever
    /// fires, so by the time a call reaches this mapping the error is a genuine driver/IO
    /// failure, not a modeled outcome.
    // 🔒️ Used as a bare fn-pointer error mapper (`.map_err(sqlite_err)`) throughout this file —
    // `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the mapper with an owned `E`.
    #[allow(clippy::needless_pass_by_value)]
    fn sqlite_err(err: rusqlite::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🔢️ SQLite's `INTEGER` column type is a signed 64-bit value; this crate's `u64`
    /// indices (segment/generation/run ids, epochs, millisecond timestamps) are validated to fit
    /// before being cast, rather than silently reinterpreting an out-of-range value's bit pattern
    /// as negative.
    fn to_sql_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
        i64::try_from(value).map_err(|_| DbError::LimitExceeded(what))
    }
    //#endregion 🔖️Errors

    //#region 🔖️Connection
    /// @emoji 🗄️ SQLite-backed `DbStorage`. One `rusqlite::Connection` behind a `Mutex` —
    /// `rusqlite` is synchronous, and every method here is a short, bounded query/transaction, so
    /// holding the mutex for a call's duration never becomes a real bottleneck at this crate's
    /// scope.
    pub struct SqliteStorage {
        conn: Mutex<Connection>,
    }

    impl SqliteStorage {
        /// @emoji 🚀️ Opens (creating the file and its parent directories if absent) a
        /// `SqliteStorage` at `path` and bootstraps the schema. Safe to call repeatedly against
        /// the same path (schema DDL is `IF NOT EXISTS`, data is untouched).
        pub fn open(path: &std::path::Path) -> Result<Self, DbError> {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).map_err(|err| DbError::Io(err.to_string()))?;
                }
            }
            let conn = Connection::open(path).map_err(sqlite_err)?;
            Self::init(conn)
        }

        /// @emoji 🧪️ Opens a private, in-memory `SqliteStorage` — never durable across process
        /// exit; exists for fast unit tests that don't care about on-disk persistence (the
        /// crash/reopen laws are exercised against a real file in `//#region 🧪️Tests` instead).
        pub fn open_in_memory() -> Result<Self, DbError> {
            let conn = Connection::open_in_memory().map_err(sqlite_err)?;
            Self::init(conn)
        }

        fn init(conn: Connection) -> Result<Self, DbError> {
            // 🎯️ `journal_mode = WAL` is a no-op (silently stays `memory`) on an in-memory
            // connection — SQLite doesn't error on the pragma either way, so `open_in_memory`
            // shares this path.
            conn.pragma_update(None, "journal_mode", "WAL").map_err(sqlite_err)?;
            conn.pragma_update(None, "synchronous", "FULL").map_err(sqlite_err)?;
            conn.pragma_update(None, "foreign_keys", "OFF").map_err(sqlite_err)?;
            conn.execute_batch(SCHEMA).map_err(sqlite_err)?;
            Ok(Self { conn: Mutex::new(conn) })
        }

        /// @emoji 🩹️ Recovers the connection mutex from a poisoned lock instead of panicking — a
        /// single panicking caller must not turn every subsequent storage call into a cascading
        /// panic (mirrors `db_storage::MemoryStorage`'s own `lock` helper).
        fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
            self.conn.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
        }
    }
    //#endregion 🔖️Connection

    //#region 🔖️WalStorage
    impl WalStorage for SqliteStorage {
        fn create_segment(&self, document: &DocumentId, index: u64) -> Result<(), DbError> {
            let index = to_sql_i64(index, "wal_storage::create_segment index")?;
            let conn = self.lock();
            let exists: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM wal_segment WHERE document = ?1 AND segment_index = ?2)",
                    params![document.0, index],
                    |row| row.get(0),
                )
                .map_err(sqlite_err)?;
            if exists {
                return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
            }
            conn.execute("INSERT INTO wal_segment (document, segment_index, bytes, sealed) VALUES (?1, ?2, x'', 0)", params![document.0, index])
                .map_err(sqlite_err)?;
            Ok(())
        }

        fn append(&self, document: &DocumentId, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "wal_storage::append")?;
            let sql_index = to_sql_i64(index, "wal_storage::append index")?;
            let conn = self.lock();
            let sealed: i64 = conn
                .query_row("SELECT sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| row.get(0))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            if sealed != 0 {
                return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
            }
            // 🎯️ `CAST(... AS BLOB)`: SQLite's `||` operator degrades a BLOB||BLOB concatenation
            // to TEXT when the left-hand accumulator started life as the zero-length `x''`
            // literal from `create_segment` — an explicit cast keeps the column's stored type
            // BLOB regardless, so a later `row.get::<_, Vec<u8>>` never sees `Invalid column
            // type Text`.
            conn.execute(
                "UPDATE wal_segment SET bytes = CAST(bytes || ?3 AS BLOB) WHERE document = ?1 AND segment_index = ?2",
                params![document.0, sql_index, bytes],
            )
            .map_err(sqlite_err)?;
            let new_len: i64 = conn
                .query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| {
                    row.get(0)
                })
                .map_err(sqlite_err)?;
            Ok(new_len as u64)
        }

        fn sync(&self, _document: &DocumentId, _index: u64, _class: DurabilityClass) -> Result<(), DbError> {
            // 🎯️ See module doc's "Durability choice": `synchronous = FULL` already fsyncs every
            // commit, so every class this crate could be asked to sync to is already satisfied.
            Ok(())
        }

        fn seal(&self, document: &DocumentId, index: u64) -> Result<(), DbError> {
            let sql_index = to_sql_i64(index, "wal_storage::seal index")?;
            let conn = self.lock();
            // 🎯️ `changes()` counts rows matched by the WHERE clause regardless of whether
            // `sealed`'s value actually flips, so this is idempotent-if-already-sealed for free:
            // `0` means no such row (not found), `1` means the row exists (sealed now, or
            // already was).
            let changed = conn
                .execute("UPDATE wal_segment SET sealed = 1 WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index])
                .map_err(sqlite_err)?;
            if changed == 0 {
                return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
            }
            Ok(())
        }

        fn read(&self, document: &DocumentId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
            check_len(range.len, MAX_BLOB_BYTES, "wal_storage::read")?;
            let sql_index = to_sql_i64(index, "wal_storage::read index")?;
            let conn = self.lock();
            let bytes: Vec<u8> = conn
                .query_row("SELECT bytes FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| row.get(0))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            let start = range.offset as usize;
            let end = start.checked_add(range.len as usize).ok_or_else(|| DbError::InvalidArgument("wal read range overflows usize".to_string()))?;
            if end > bytes.len() {
                return Err(DbError::InvalidArgument(format!("wal read range {start}..{end} out of bounds (len {})", bytes.len())));
            }
            Ok(bytes[start..end].to_vec())
        }

        fn segment_len(&self, document: &DocumentId, index: u64) -> Result<u64, DbError> {
            let sql_index = to_sql_i64(index, "wal_storage::segment_len index")?;
            let conn = self.lock();
            let len: i64 = conn
                .query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| {
                    row.get(0)
                })
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            Ok(len as u64)
        }

        fn list_segments(&self, document: &DocumentId) -> Result<Vec<u64>, DbError> {
            let conn = self.lock();
            let mut stmt =
                conn.prepare("SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC").map_err(sqlite_err)?;
            let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row.map_err(sqlite_err)? as u64);
            }
            Ok(out)
        }

        fn truncate_tail(&self, document: &DocumentId, index: u64, new_len: u64) -> Result<(), DbError> {
            let sql_index = to_sql_i64(index, "wal_storage::truncate_tail index")?;
            let conn = self.lock();
            let (sealed, current_len): (i64, i64) = conn
                .query_row(
                    "SELECT sealed, length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2",
                    params![document.0, sql_index],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            if sealed != 0 {
                return Err(DbError::InvalidArgument(format!("cannot truncate sealed wal segment {index}")));
            }
            if new_len > current_len as u64 {
                return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
            }
            let sql_new_len = to_sql_i64(new_len, "wal_storage::truncate_tail new_len")?;
            conn.execute(
                "UPDATE wal_segment SET bytes = CAST(substr(bytes, 1, ?3) AS BLOB) WHERE document = ?1 AND segment_index = ?2",
                params![document.0, sql_index, sql_new_len],
            )
            .map_err(sqlite_err)?;
            Ok(())
        }

        fn delete_segment(&self, document: &DocumentId, index: u64) -> Result<(), DbError> {
            let sql_index = to_sql_i64(index, "wal_storage::delete_segment index")?;
            let conn = self.lock();
            conn.execute("DELETE FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index]).map_err(sqlite_err)?;
            Ok(())
        }
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    impl SnapshotStorage for SqliteStorage {
        fn write_generation(&self, document: &DocumentId, generation: u64, bytes: &[u8]) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "snapshot_storage::write_generation")?;
            let sql_generation = to_sql_i64(generation, "snapshot_storage::write_generation generation")?;
            let conn = self.lock();
            conn.execute(
                "INSERT INTO snapshot_generation (document, generation, bytes) VALUES (?1, ?2, ?3)
             ON CONFLICT(document, generation) DO UPDATE SET bytes = excluded.bytes",
                params![document.0, sql_generation, bytes],
            )
            .map_err(sqlite_err)?;
            Ok(())
        }

        fn read_generation(&self, document: &DocumentId, generation: u64) -> Result<Vec<u8>, DbError> {
            let sql_generation = to_sql_i64(generation, "snapshot_storage::read_generation generation")?;
            let conn = self.lock();
            conn.query_row(
                "SELECT bytes FROM snapshot_generation WHERE document = ?1 AND generation = ?2",
                params![document.0, sql_generation],
                |row| row.get(0),
            )
            .optional()
            .map_err(sqlite_err)?
            .ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))
        }

        fn latest_generation(&self, document: &DocumentId) -> Result<Option<u64>, DbError> {
            let conn = self.lock();
            let max: Option<i64> = conn
                .query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.0], |row| row.get(0))
                .map_err(sqlite_err)?;
            Ok(max.map(|value| value as u64))
        }

        fn list_generations(&self, document: &DocumentId) -> Result<Vec<u64>, DbError> {
            let conn = self.lock();
            let mut stmt = conn
                .prepare("SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC")
                .map_err(sqlite_err)?;
            let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row.map_err(sqlite_err)? as u64);
            }
            Ok(out)
        }

        fn delete_generation(&self, document: &DocumentId, generation: u64) -> Result<(), DbError> {
            let sql_generation = to_sql_i64(generation, "snapshot_storage::delete_generation generation")?;
            let conn = self.lock();
            conn.execute("DELETE FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![document.0, sql_generation])
                .map_err(sqlite_err)?;
            Ok(())
        }
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    impl PayloadStorage for SqliteStorage {
        fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "payload_storage::put")?;
            let hash = ContentHash(*blake3::hash(bytes).as_bytes());
            let conn = self.lock();
            conn.execute(
                "INSERT INTO payload (hash, bytes, len) VALUES (?1, ?2, ?3) ON CONFLICT(hash) DO NOTHING",
                params![hash.to_string(), bytes, bytes.len() as i64],
            )
            .map_err(sqlite_err)?;
            Ok(hash)
        }

        fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
            let conn = self.lock();
            conn.query_row("SELECT bytes FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
        }

        fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            let conn = self.lock();
            conn.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)
        }

        fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            let conn = self.lock();
            conn.execute("DELETE FROM payload WHERE hash = ?1", params![hash.to_string()]).map_err(sqlite_err)?;
            Ok(())
        }

        fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            let conn = self.lock();
            let len: i64 = conn
                .query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
            Ok(len as u64)
        }
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    impl CatalogStorage for SqliteStorage {
        fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
            let conn = self.lock();
            let row: Option<(Vec<u8>, i64)> = conn
                .query_row("SELECT bytes, epoch FROM catalog_root WHERE id = 0", [], |row| Ok((row.get(0)?, row.get(1)?)))
                .optional()
                .map_err(sqlite_err)?;
            Ok(row.map(|(bytes, epoch)| (bytes, EpochFence { epoch: epoch as u64 })))
        }

        fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError> {
            check_len(new_bytes.len() as u64, MAX_BLOB_BYTES, "catalog_storage::cas_root")?;
            let mut conn = self.lock();
            // 🎯️ `IMMEDIATE` acquires SQLite's write lock before the read, so a concurrent writer
            // (another thread OR another OS process against the same file) can't slip a write in
            // between this read and this write — see module doc's "CAS choice".
            let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
            let current_epoch: Option<i64> =
                tx.query_row("SELECT epoch FROM catalog_root WHERE id = 0", [], |row| row.get(0)).optional().map_err(sqlite_err)?;
            let current_fence = current_epoch.map_or(EpochFence::INITIAL, |epoch| EpochFence { epoch: epoch as u64 });
            expected.check(current_fence)?;
            let new_fence = expected.next();
            let sql_epoch = to_sql_i64(new_fence.epoch, "catalog_storage::cas_root epoch")?;
            tx.execute(
                "INSERT INTO catalog_root (id, bytes, epoch) VALUES (0, ?1, ?2)
             ON CONFLICT(id) DO UPDATE SET bytes = excluded.bytes, epoch = excluded.epoch",
                params![new_bytes, sql_epoch],
            )
            .map_err(sqlite_err)?;
            tx.commit().map_err(sqlite_err)?;
            Ok(new_fence)
        }
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    impl IndexStorage for SqliteStorage {
        fn write_run(&self, document: &DocumentId, run_id: u64, bytes: &[u8]) -> Result<(), DbError> {
            check_len(bytes.len() as u64, MAX_BLOB_BYTES, "index_storage::write_run")?;
            let sql_run_id = to_sql_i64(run_id, "index_storage::write_run run_id")?;
            let conn = self.lock();
            conn.execute(
                "INSERT INTO index_run (document, run_id, bytes) VALUES (?1, ?2, ?3)
             ON CONFLICT(document, run_id) DO UPDATE SET bytes = excluded.bytes",
                params![document.0, sql_run_id, bytes],
            )
            .map_err(sqlite_err)?;
            Ok(())
        }

        fn read_run(&self, document: &DocumentId, run_id: u64) -> Result<Vec<u8>, DbError> {
            let sql_run_id = to_sql_i64(run_id, "index_storage::read_run run_id")?;
            let conn = self.lock();
            conn.query_row("SELECT bytes FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.0, sql_run_id], |row| row.get(0))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))
        }

        fn list_runs(&self, document: &DocumentId) -> Result<Vec<u64>, DbError> {
            let conn = self.lock();
            let mut stmt = conn.prepare("SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC").map_err(sqlite_err)?;
            let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row.map_err(sqlite_err)? as u64);
            }
            Ok(out)
        }

        fn delete_run(&self, document: &DocumentId, run_id: u64) -> Result<(), DbError> {
            let sql_run_id = to_sql_i64(run_id, "index_storage::delete_run run_id")?;
            let conn = self.lock();
            conn.execute("DELETE FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.0, sql_run_id]).map_err(sqlite_err)?;
            Ok(())
        }
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    impl LeaseStorage for SqliteStorage {
        fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            let mut conn = self.lock();
            let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
            let existing: Option<(String, i64, i64)> = tx
                .query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })
                .optional()
                .map_err(sqlite_err)?;
            let fence = match existing {
                Some((existing_holder, epoch, expires_at_ms)) if (now_ms as i64) < expires_at_ms => {
                    if existing_holder != holder {
                        return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
                    }
                    EpochFence { epoch: epoch as u64 }
                }
                Some((_, epoch, _)) => EpochFence { epoch: epoch as u64 }.next(),
                None => EpochFence::INITIAL,
            };
            let sql_epoch = to_sql_i64(fence.epoch, "lease_storage::acquire epoch")?;
            let sql_expires_at_ms = to_sql_i64(now_ms + ttl_ms, "lease_storage::acquire expires_at_ms")?;
            tx.execute(
                "INSERT INTO lease (resource, holder, epoch, expires_at_ms) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(resource) DO UPDATE SET holder = excluded.holder, epoch = excluded.epoch, expires_at_ms = excluded.expires_at_ms",
                params![resource, holder, sql_epoch, sql_expires_at_ms],
            )
            .map_err(sqlite_err)?;
            tx.commit().map_err(sqlite_err)?;
            Ok(fence)
        }

        fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            let mut conn = self.lock();
            let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
            let (existing_holder, epoch, expires_at_ms): (String, i64, i64) = tx
                .query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
            if now_ms as i64 >= expires_at_ms {
                return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
            }
            if existing_holder != holder {
                return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
            }
            fence.check(EpochFence { epoch: epoch as u64 })?;
            let sql_expires_at_ms = to_sql_i64(now_ms + ttl_ms, "lease_storage::renew expires_at_ms")?;
            tx.execute("UPDATE lease SET expires_at_ms = ?2 WHERE resource = ?1", params![resource, sql_expires_at_ms]).map_err(sqlite_err)?;
            tx.commit().map_err(sqlite_err)?;
            Ok(())
        }

        fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            let mut conn = self.lock();
            let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
            let (existing_holder, epoch): (String, i64) = tx
                .query_row("SELECT holder, epoch FROM lease WHERE resource = ?1", params![resource], |row| Ok((row.get(0)?, row.get(1)?)))
                .optional()
                .map_err(sqlite_err)?
                .ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
            if existing_holder != holder {
                return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
            }
            fence.check(EpochFence { epoch: epoch as u64 })?;
            tx.execute("DELETE FROM lease WHERE resource = ?1", params![resource]).map_err(sqlite_err)?;
            tx.commit().map_err(sqlite_err)?;
            Ok(())
        }

        fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            let conn = self.lock();
            let existing: Option<(String, i64, i64)> = conn
                .query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })
                .optional()
                .map_err(sqlite_err)?;
            Ok(existing.and_then(|(holder, epoch, expires_at_ms)| {
                if (now_ms as i64) < expires_at_ms {
                    Some(LeaseInfo { resource: resource.to_string(), holder, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 })
                } else {
                    None
                }
            }))
        }
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbStorage
    impl DbStorage for SqliteStorage {
        fn wal(&self) -> &dyn WalStorage {
            self
        }

        fn snapshot(&self) -> &dyn SnapshotStorage {
            self
        }

        fn payload(&self) -> &dyn PayloadStorage {
            self
        }

        fn catalog(&self) -> &dyn CatalogStorage {
            self
        }

        fn index(&self) -> &dyn IndexStorage {
            self
        }

        fn lease(&self) -> &dyn LeaseStorage {
            self
        }

        fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }
    //#endregion 🔖️DbStorage

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

        /// @emoji 🎲️ A fresh `SqliteStorage` rooted at a unique scratch file under
        /// `std::env::temp_dir()` — a REAL on-disk `.sqlite3` file (not `:memory:`), so tests
        /// that reopen it exercise genuine persistence, mirroring `db_storage::FsStorage`'s own
        /// scratch helper convention (no external `tempfile` crate dependency).
        fn sqlite_scratch_path(name: &str) -> std::path::PathBuf {
            let pid = std::process::id();
            let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            std::env::temp_dir().join(format!("db_storage_sqlite_test_{name}_{pid}_{counter}.sqlite3"))
        }

        fn sqlite_scratch(name: &str) -> SqliteStorage {
            SqliteStorage::open(&sqlite_scratch_path(name)).unwrap()
        }

        //#region 🔖️WalStorage
        #[test]
        fn wal_storage_append_seal_truncate_and_bounds_laws() {
            let storage = sqlite_scratch("wal_laws");
            let document: DocumentId = "doc-wal".into();

            storage.create_segment(&document, 0).unwrap();
            assert!(matches!(storage.create_segment(&document, 0), Err(DbError::AlreadyExists(_))));

            let len_after_first = storage.append(&document, 0, b"hello ").unwrap();
            assert_eq!(len_after_first, 6);
            let len_after_second = storage.append(&document, 0, b"world").unwrap();
            assert_eq!(len_after_second, 11);
            assert_eq!(storage.segment_len(&document, 0).unwrap(), 11);

            let read_back = storage.read(&document, 0, ByteRange { offset: 6, len: 5 }).unwrap();
            assert_eq!(read_back, b"world");
            assert!(matches!(storage.read(&document, 0, ByteRange { offset: 6, len: 100 }), Err(DbError::InvalidArgument(_))));

            storage.sync(&document, 0, DurabilityClass::Fsync).unwrap();

            storage.truncate_tail(&document, 0, 6).unwrap();
            assert_eq!(storage.segment_len(&document, 0).unwrap(), 6);
            assert_eq!(storage.read(&document, 0, ByteRange { offset: 0, len: 6 }).unwrap(), b"hello ");

            storage.create_segment(&document, 1).unwrap();
            assert_eq!(storage.list_segments(&document).unwrap(), vec![0, 1]);

            storage.seal(&document, 0).unwrap();
            storage.seal(&document, 0).unwrap(); // idempotent
            assert!(matches!(storage.append(&document, 0, b"!"), Err(DbError::InvalidArgument(_))));
            assert!(matches!(storage.truncate_tail(&document, 0, 0), Err(DbError::InvalidArgument(_))));

            storage.delete_segment(&document, 1).unwrap();
            assert_eq!(storage.list_segments(&document).unwrap(), vec![0]);

            assert!(matches!(storage.append(&document, 99, b"x"), Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️WalStorage

        //#region 🔖️SnapshotStorage
        #[test]
        fn snapshot_storage_generations_overwrite_and_delete_laws() {
            let storage = sqlite_scratch("snapshot_laws");
            let document: DocumentId = "doc-snap".into();
            assert_eq!(storage.latest_generation(&document).unwrap(), None);

            storage.write_generation(&document, 0, b"gen-zero-bytes").unwrap();
            storage.write_generation(&document, 1, b"gen-one-bytes").unwrap();
            assert_eq!(storage.list_generations(&document).unwrap(), vec![0, 1]);
            assert_eq!(storage.latest_generation(&document).unwrap(), Some(1));
            assert_eq!(storage.read_generation(&document, 0).unwrap(), b"gen-zero-bytes");

            storage.write_generation(&document, 0, b"gen-zero-overwritten").unwrap();
            assert_eq!(storage.read_generation(&document, 0).unwrap(), b"gen-zero-overwritten");

            storage.delete_generation(&document, 0).unwrap();
            assert!(matches!(storage.read_generation(&document, 0), Err(DbError::NotFound(_))));
            assert_eq!(storage.list_generations(&document).unwrap(), vec![1]);
        }
        //#endregion 🔖️SnapshotStorage

        //#region 🔖️PayloadStorage
        #[test]
        fn payload_storage_is_content_addressed_and_idempotent() {
            let storage = sqlite_scratch("payload_laws");
            let bytes = b"a payload blob that gets content-addressed";
            let hash_a = storage.put(bytes).unwrap();
            let hash_b = storage.put(bytes).unwrap();
            assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
            assert_eq!(hash_a, ContentHash(*blake3::hash(bytes).as_bytes()));

            assert!(storage.contains(&hash_a).unwrap());
            assert_eq!(storage.get(&hash_a).unwrap(), bytes);
            assert_eq!(storage.len(&hash_a).unwrap(), bytes.len() as u64);

            let other_hash = ContentHash([0xAB; 32]);
            assert!(!storage.contains(&other_hash).unwrap());
            assert!(matches!(storage.get(&other_hash), Err(DbError::NotFound(_))));

            storage.delete(&hash_a).unwrap();
            assert!(!storage.contains(&hash_a).unwrap());
            assert!(matches!(storage.get(&hash_a), Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️PayloadStorage

        //#region 🔖️CatalogStorage
        #[test]
        fn catalog_storage_cas_root_fences_stale_writers() {
            let storage = sqlite_scratch("catalog_laws");
            assert_eq!(storage.read_root().unwrap(), None);

            let epoch_1 = storage.cas_root(EpochFence::INITIAL, b"root-v1").unwrap();
            assert_eq!(epoch_1, EpochFence::INITIAL.next());
            let (bytes, fence) = storage.read_root().unwrap().unwrap();
            assert_eq!(bytes, b"root-v1");
            assert_eq!(fence, epoch_1);

            // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
            assert!(matches!(storage.cas_root(EpochFence::INITIAL, b"root-stale"), Err(DbError::Fenced { .. })));

            let epoch_2 = storage.cas_root(epoch_1, b"root-v2").unwrap();
            assert_eq!(epoch_2, epoch_1.next());
            assert_eq!(storage.read_root().unwrap().unwrap().0, b"root-v2");
        }
        //#endregion 🔖️CatalogStorage

        //#region 🔖️IndexStorage
        #[test]
        fn index_storage_runs_list_read_and_delete_laws() {
            let storage = sqlite_scratch("index_laws");
            let document: DocumentId = "doc-index".into();
            storage.write_run(&document, 0, b"run-zero").unwrap();
            storage.write_run(&document, 1, b"run-one").unwrap();
            assert_eq!(storage.list_runs(&document).unwrap(), vec![0, 1]);
            assert_eq!(storage.read_run(&document, 1).unwrap(), b"run-one");

            storage.delete_run(&document, 0).unwrap();
            assert_eq!(storage.list_runs(&document).unwrap(), vec![1]);
            assert!(matches!(storage.read_run(&document, 0), Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️IndexStorage

        //#region 🔖️LeaseStorage
        #[test]
        fn lease_storage_acquire_renew_fence_and_handoff_laws() {
            let storage = sqlite_scratch("lease_laws");
            let fence_1 = storage.acquire("shard-1", "node-a", 1_000, 0).unwrap();
            assert_eq!(fence_1, EpochFence::INITIAL);

            // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
            let fence_reacquire = storage.acquire("shard-1", "node-a", 1_000, 100).unwrap();
            assert_eq!(fence_reacquire, fence_1);

            // A different holder cannot acquire an unexpired lease.
            assert!(matches!(storage.acquire("shard-1", "node-b", 1_000, 100), Err(DbError::Conflict(_))));

            storage.renew("shard-1", "node-a", fence_1, 1_000, 500).unwrap();
            assert!(matches!(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500), Err(DbError::Fenced { .. })));
            assert!(matches!(storage.renew("shard-1", "node-b", fence_1, 1_000, 500), Err(DbError::Unauthorized(_))));

            let current = storage.current("shard-1", 600).unwrap().unwrap();
            assert_eq!(current.holder, "node-a");
            assert_eq!(current.fence, fence_1);

            // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
            // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
            assert_eq!(storage.current("shard-1", 2_000).unwrap(), None);
            let fence_2 = storage.acquire("shard-1", "node-b", 1_000, 2_000).unwrap();
            assert_eq!(fence_2, fence_1.next());

            // The old holder's stale fence is now rejected.
            assert!(matches!(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100), Err(DbError::Unauthorized(_))));

            storage.release("shard-1", "node-b", fence_2).unwrap();
            assert_eq!(storage.current("shard-1", 2_100).unwrap(), None);
            assert!(matches!(storage.release("shard-1", "node-b", fence_2), Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️LeaseStorage

        //#region 🔖️DbStorage
        #[test]
        fn db_storage_accessors_and_capabilities() {
            let storage: std::sync::Arc<dyn DbStorage> = std::sync::Arc::new(sqlite_scratch("umbrella"));
            let document: DocumentId = "doc-umbrella".into();
            storage.wal().create_segment(&document, 0).unwrap();
            storage.catalog().cas_root(EpochFence::INITIAL, b"root").unwrap();
            storage.index().write_run(&document, 0, b"run").unwrap();
            assert_eq!(storage.index().read_run(&document, 0).unwrap(), b"run");

            let capabilities = storage.capabilities();
            assert!(capabilities.durable);
            assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
            assert!(capabilities.supports_fsync);
            assert!(capabilities.supports_cas);
        }
        //#endregion 🔖️DbStorage

        //#region 🔖️Connection
        #[test]
        fn write_survives_reopen_across_instances_against_a_real_file() {
            let path = sqlite_scratch_path("reopen");
            {
                let storage = SqliteStorage::open(&path).unwrap();
                let document: DocumentId = "doc-reopen".into();
                storage.write_generation(&document, 0, b"persisted across reopen").unwrap();
                storage.payload().put(b"payload persisted across reopen").unwrap();
            }
            {
                let storage = SqliteStorage::open(&path).unwrap();
                let document: DocumentId = "doc-reopen".into();
                assert_eq!(storage.read_generation(&document, 0).unwrap(), b"persisted across reopen");
                let hash = ContentHash(*blake3::hash(b"payload persisted across reopen").as_bytes());
                assert_eq!(storage.payload().get(&hash).unwrap(), b"payload persisted across reopen");
            }
        }

        #[test]
        fn in_memory_storage_works_without_a_file() {
            let storage = SqliteStorage::open_in_memory().unwrap();
            let document: DocumentId = "doc-mem".into();
            storage.create_segment(&document, 0).unwrap();
            assert_eq!(storage.append(&document, 0, b"in memory").unwrap(), 9);
        }
        //#endregion 🔖️Connection
    }
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite_storage::SqliteStorage;
//#endregion 🔖️SqliteStorage
