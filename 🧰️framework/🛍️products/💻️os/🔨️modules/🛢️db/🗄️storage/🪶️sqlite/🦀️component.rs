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
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! `rusqlite` is a genuinely BLOCKING C client (unlike `sqlx`/`neo4rs`) — every `db_storage`
//! sub-trait method here is a plain `async fn` whose body dispatches through `db_storage`'s own
//! `run_blocking_op` bridge, which submits onto the process-wide `semio_framework_async::WorkerPool`
//! (`Lane::Io`) rather than ever running the SQLite call on the calling async task's own thread.
//! This crate names no `tokio`/executor of its own — mirrors `db_storage::FsStorage`'s identical
//! crossing.
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
    use crate::db_durability::{DurabilityClass, EpochFence};
    use crate::db_ids::{check_len, ArtifactId, DbError};
    use crate::db_storage::{run_blocking_op, CatalogStorage, DbIoPages, DbIoRequest, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use pack::{ByteRange, ContentHash};
    use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
    use semio_framework_async::WorkerPool;
    use std::sync::{Arc, Mutex};

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
    /// via `check_len` BEFORE the read buffer is allocated. Mirrors
    /// `db_storage::MemoryStorage`/`FsStorage`'s own `MAX_READ_BYTES` choice (same number, kept
    /// in lock-step deliberately: a caller swapping backends should hit the same ceiling on
    /// every backend).
    const MAX_BLOB_BYTES: u64 = 496 * 1024;
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
    // 🚫️async: E4 fn-pointer slot
    fn sqlite_err(err: rusqlite::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🔢️ SQLite's `INTEGER` column type is a signed 64-bit value; this crate's `u64`
    /// indices (segment/generation/run ids, epochs, millisecond timestamps) are validated to fit
    /// before being cast, rather than silently reinterpreting an out-of-range value's bit pattern
    /// as negative.
    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn to_sql_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
        i64::try_from(value).map_err(|_| DbError::LimitExceeded(what))
    }
    //#endregion 🔖️Errors

    //#region 🔖️Connection
    /// @emoji 🗄️ SQLite-backed `DbStorage`. One `rusqlite::Connection` behind an `Arc<Mutex<_>>`
    /// (the `Arc` is what lets every trait method's blocking closure — dispatched through
    /// `run_blocking_op`, see module doc — carry its own `'static` handle to the same connection)
    /// — `rusqlite` is synchronous. Each call is an explicitly indivisible backend residual on
    /// the shared I/O lane; the retained authority bounds admission and ownership, not syscall
    /// duration. Phase 9 replaces that residual with the owned event log.
    pub struct SqliteStorage {
        conn: Arc<Mutex<Connection>>,
        pool: Arc<WorkerPool>,
    }

    /// @emoji 🩹️ Recovers the connection mutex from a poisoned lock instead of panicking — a
    /// single panicking caller must not turn every subsequent storage call into a cascading
    /// panic (mirrors `db_storage::MemoryStorage`'s own `lock` helper).
    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn lock(conn: &Mutex<Connection>) -> std::sync::MutexGuard<'_, Connection> {
        conn.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    impl SqliteStorage {
        /// @emoji 🚀️ Opens (creating the file and its parent directories if absent) a
        /// `SqliteStorage` at `path` and bootstraps the schema. Safe to call repeatedly against
        /// the same path (schema DDL is `IF NOT EXISTS`, data is untouched). Dispatches every
        /// subsequent trait call's blocking body through `run_blocking_op` onto `pool`'s
        /// `Lane::Io`; open and schema setup use that same retained authority.
        pub async fn open(pool: Arc<WorkerPool>, path: &std::path::Path) -> Result<Self, DbError> {
            let admitted_path = path.to_path_buf();
            let conn = run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                if let Some(parent) = admitted_path.parent() {
                    if !parent.as_os_str().is_empty() {
                        std::fs::create_dir_all(parent).map_err(|err| DbError::Io(err.to_string()))?;
                    }
                }
                let conn = Connection::open(admitted_path).map_err(sqlite_err)?;
                init_connection(&conn)?;
                Ok(conn)
            })
            .await?;
            Ok(Self { conn: Arc::new(Mutex::new(conn)), pool })
        }

        /// @emoji 🧪️ Opens a private, in-memory `SqliteStorage` — never durable across process
        /// exit; exists for fast unit tests that don't care about on-disk persistence (the
        /// crash/reopen laws are exercised against a real file in `//#region 🧪️Tests` instead).
        pub async fn open_in_memory(pool: Arc<WorkerPool>) -> Result<Self, DbError> {
            let conn = run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                let conn = Connection::open_in_memory().map_err(sqlite_err)?;
                init_connection(&conn)?;
                Ok(conn)
            })
            .await?;
            Ok(Self { conn: Arc::new(Mutex::new(conn)), pool })
        }
    }

    fn init_connection(conn: &Connection) -> Result<(), DbError> {
        conn.pragma_update(None, "journal_mode", "WAL").map_err(sqlite_err)?;
        conn.pragma_update(None, "synchronous", "FULL").map_err(sqlite_err)?;
        conn.pragma_update(None, "foreign_keys", "OFF").map_err(sqlite_err)?;
        conn.execute_batch(SCHEMA).map_err(sqlite_err)
    }
    //#endregion 🔖️Connection

    //#region 🔖️WalStorage
    impl WalStorage for SqliteStorage {
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let index = to_sql_i64(index, "wal_storage::create_segment index")?;
                    let conn = lock(&conn);
                    let exists: bool = conn.query_row("SELECT EXISTS(SELECT 1 FROM wal_segment WHERE document = ?1 AND segment_index = ?2)", params![document.0, index], |row| row.get(0)).map_err(sqlite_err)?;
                    if exists {
                        return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
                    }
                    conn.execute("INSERT INTO wal_segment (document, segment_index, bytes, sealed) VALUES (?1, ?2, x'', 0)", params![document.0, index]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_BLOB_BYTES, "wal_storage::append") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            let byte_len = bytes.len() as u64;
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::write(byte_len), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::append index")?;
                    let conn = lock(&conn);
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
                    conn.execute("UPDATE wal_segment SET bytes = CAST(bytes || ?3 AS BLOB) WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index, bytes.as_slice()]).map_err(sqlite_err)?;
                    let new_len: i64 = conn.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok(new_len as u64)
                })
                .await
            }
        }

        async fn sync(&self, _document: &ArtifactId, _index: u64, _class: DurabilityClass) -> Result<(), DbError> {
            // 🎯️ See module doc's "Durability choice": `synchronous = FULL` already fsyncs every
            // commit, so every class this crate could be asked to sync to is already satisfied.
            {
                Ok(())
            }
        }

        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::seal index")?;
                    let conn = lock(&conn);
                    // 🎯️ `changes()` counts rows matched by the WHERE clause regardless of whether
                    // `sealed`'s value actually flips, so this is idempotent-if-already-sealed for free:
                    // `0` means no such row (not found), `1` means the row exists (sealed now, or
                    // already was).
                    let changed = conn.execute("UPDATE wal_segment SET sealed = 1 WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index]).map_err(sqlite_err)?;
                    if changed == 0 {
                        return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
                    }
                    Ok(())
                })
                .await
            }
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
            if let Err(err) = check_len(range.len, MAX_BLOB_BYTES, "wal_storage::read") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::read(range.len), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::read index")?;
                    let sql_start = to_sql_i64(range.offset.checked_add(1).ok_or_else(|| DbError::InvalidArgument("wal read offset overflows u64".to_string()))?, "wal_storage::read offset")?;
                    let sql_len = to_sql_i64(range.len, "wal_storage::read len")?;
                    let conn = lock(&conn);
                    let (bytes, current_len): (Vec<u8>, i64) = conn
                        .query_row("SELECT CAST(substr(bytes, ?3, ?4) AS BLOB), length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index, sql_start, sql_len], |row| Ok((row.get(0)?, row.get(1)?)))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
                    let start = range.offset as usize;
                    let end = start.checked_add(range.len as usize).ok_or_else(|| DbError::InvalidArgument("wal read range overflows usize".to_string()))?;
                    if end > current_len as usize {
                        return Err(DbError::InvalidArgument(format!("wal read range {start}..{end} out of bounds (len {current_len})")));
                    }
                    Ok(bytes)
                })
                .await
            }
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::segment_len index")?;
                    let conn = lock(&conn);
                    let len: i64 = conn
                        .query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| row.get(0))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
                    Ok(len as u64)
                })
                .await
            }
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::list(4096), move || {
                    let conn = lock(&conn);
                    let mut stmt = conn.prepare("SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC").map_err(sqlite_err)?;
                    let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
                    let mut out = Vec::new();
                    for row in rows {
                        if out.len() == 4096 {
                            return Err(DbError::LimitExceeded("db_io list item credit"));
                        }
                        out.push(row.map_err(sqlite_err)? as u64);
                    }
                    Ok(out)
                })
                .await
            }
        }

        async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::truncate_tail index")?;
                    let conn = lock(&conn);
                    let (sealed, current_len): (i64, i64) = conn
                        .query_row("SELECT sealed, length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index], |row| Ok((row.get(0)?, row.get(1)?)))
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
                    conn.execute("UPDATE wal_segment SET bytes = CAST(substr(bytes, 1, ?3) AS BLOB) WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index, sql_new_len]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_index = to_sql_i64(index, "wal_storage::delete_segment index")?;
                    let conn = lock(&conn);
                    conn.execute("DELETE FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.0, sql_index]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    impl SnapshotStorage for SqliteStorage {
        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_BLOB_BYTES, "snapshot_storage::write_generation") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            let byte_len = bytes.len() as u64;
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::write(byte_len), move || {
                    let sql_generation = to_sql_i64(generation, "snapshot_storage::write_generation generation")?;
                    let conn = lock(&conn);
                    conn.execute(
                        "INSERT INTO snapshot_generation (document, generation, bytes) VALUES (?1, ?2, ?3)
                     ON CONFLICT(document, generation) DO UPDATE SET bytes = excluded.bytes",
                        params![document.0, sql_generation, bytes.as_slice()],
                    )
                    .map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::read(MAX_BLOB_BYTES), move || {
                    let sql_generation = to_sql_i64(generation, "snapshot_storage::read_generation generation")?;
                    let conn = lock(&conn);
                    let len: i64 = conn
                        .query_row("SELECT length(bytes) FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![&document.0, sql_generation], |row| row.get(0))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?;
                    check_len(len as u64, MAX_BLOB_BYTES, "snapshot_storage::read_generation")?;
                    conn.query_row("SELECT bytes FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![&document.0, sql_generation], |row| row.get(0))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))
                })
                .await
            }
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let conn = lock(&conn);
                    let max: Option<i64> = conn.query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.0], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok(max.map(|value| value as u64))
                })
                .await
            }
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::list(4096), move || {
                    let conn = lock(&conn);
                    let mut stmt = conn.prepare("SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC").map_err(sqlite_err)?;
                    let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
                    let mut out = Vec::new();
                    for row in rows {
                        if out.len() == 4096 {
                            return Err(DbError::LimitExceeded("db_io list item credit"));
                        }
                        out.push(row.map_err(sqlite_err)? as u64);
                    }
                    Ok(out)
                })
                .await
            }
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_generation = to_sql_i64(generation, "snapshot_storage::delete_generation generation")?;
                    let conn = lock(&conn);
                    conn.execute("DELETE FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![document.0, sql_generation]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    impl PayloadStorage for SqliteStorage {
        async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_BLOB_BYTES, "payload_storage::put") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let pool = self.pool.clone();
            let byte_len = bytes.len() as u64;
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::write(byte_len), move || {
                    let hash = ContentHash(*blake3::hash(bytes.as_slice()).as_bytes());
                    let conn = lock(&conn);
                    conn.execute("INSERT INTO payload (hash, bytes, len) VALUES (?1, ?2, ?3) ON CONFLICT(hash) DO NOTHING", params![hash.to_string(), bytes.as_slice(), bytes.len() as i64]).map_err(sqlite_err)?;
                    Ok(hash)
                })
                .await
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
            let conn = self.conn.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::read(MAX_BLOB_BYTES), move || {
                    let conn = lock(&conn);
                    let len: i64 = conn.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
                    check_len(len as u64, MAX_BLOB_BYTES, "payload_storage::get")?;
                    conn.query_row("SELECT bytes FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
                })
                .await
            }
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            let conn = self.conn.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let conn = lock(&conn);
                    conn.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)
                })
                .await
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let conn = lock(&conn);
                    conn.execute("DELETE FROM payload WHERE hash = ?1", params![hash.to_string()]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            let conn = self.conn.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let conn = lock(&conn);
                    let len: i64 = conn.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
                    Ok(len as u64)
                })
                .await
            }
        }
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    impl CatalogStorage for SqliteStorage {
        async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
            let conn = self.conn.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::read(MAX_BLOB_BYTES), move || {
                    let conn = lock(&conn);
                    let len: Option<i64> = conn.query_row("SELECT length(bytes) FROM catalog_root WHERE id = 0", [], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    if let Some(len) = len {
                        check_len(len as u64, MAX_BLOB_BYTES, "catalog_storage::read_root")?;
                    }
                    let row: Option<(Vec<u8>, i64)> = conn.query_row("SELECT bytes, epoch FROM catalog_root WHERE id = 0", [], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(sqlite_err)?;
                    Ok(row.map(|(bytes, epoch)| (bytes, EpochFence { epoch: epoch as u64 })))
                })
                .await
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
            if let Err(err) = check_len(new_bytes.len() as u64, MAX_BLOB_BYTES, "catalog_storage::cas_root") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let pool = self.pool.clone();
            let byte_len = new_bytes.len() as u64;
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::write(byte_len), move || {
                    let mut conn = lock(&conn);
                    // 🎯️ `IMMEDIATE` acquires SQLite's write lock before the read, so a concurrent writer
                    // (another thread OR another OS process against the same file) can't slip a write in
                    // between this read and this write — see module doc's "CAS choice".
                    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                    let current_epoch: Option<i64> = tx.query_row("SELECT epoch FROM catalog_root WHERE id = 0", [], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    let current_fence = current_epoch.map_or(EpochFence::INITIAL, |epoch| EpochFence { epoch: epoch as u64 });
                    expected.check(current_fence)?;
                    let new_fence = expected.next();
                    let sql_epoch = to_sql_i64(new_fence.epoch, "catalog_storage::cas_root epoch")?;
                    tx.execute(
                        "INSERT INTO catalog_root (id, bytes, epoch) VALUES (0, ?1, ?2)
                     ON CONFLICT(id) DO UPDATE SET bytes = excluded.bytes, epoch = excluded.epoch",
                        params![new_bytes.as_slice(), sql_epoch],
                    )
                    .map_err(sqlite_err)?;
                    tx.commit().map_err(sqlite_err)?;
                    Ok(new_fence)
                })
                .await
            }
        }
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    impl IndexStorage for SqliteStorage {
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_BLOB_BYTES, "index_storage::write_run") {
                return { Err(err) };
            }
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            let byte_len = bytes.len() as u64;
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::write(byte_len), move || {
                    let sql_run_id = to_sql_i64(run_id, "index_storage::write_run run_id")?;
                    let conn = lock(&conn);
                    conn.execute(
                        "INSERT INTO index_run (document, run_id, bytes) VALUES (?1, ?2, ?3)
                     ON CONFLICT(document, run_id) DO UPDATE SET bytes = excluded.bytes",
                        params![document.0, sql_run_id, bytes.as_slice()],
                    )
                    .map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::read(MAX_BLOB_BYTES), move || {
                    let sql_run_id = to_sql_i64(run_id, "index_storage::read_run run_id")?;
                    let conn = lock(&conn);
                    let len: i64 = conn
                        .query_row("SELECT length(bytes) FROM index_run WHERE document = ?1 AND run_id = ?2", params![&document.0, sql_run_id], |row| row.get(0))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?;
                    check_len(len as u64, MAX_BLOB_BYTES, "index_storage::read_run")?;
                    conn.query_row("SELECT bytes FROM index_run WHERE document = ?1 AND run_id = ?2", params![&document.0, sql_run_id], |row| row.get(0))
                        .optional()
                        .map_err(sqlite_err)?
                        .ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))
                })
                .await
            }
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::list(4096), move || {
                    let conn = lock(&conn);
                    let mut stmt = conn.prepare("SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC").map_err(sqlite_err)?;
                    let rows = stmt.query_map(params![document.0], |row| row.get::<_, i64>(0)).map_err(sqlite_err)?;
                    let mut out = Vec::new();
                    for row in rows {
                        if out.len() == 4096 {
                            return Err(DbError::LimitExceeded("db_io list item credit"));
                        }
                        out.push(row.map_err(sqlite_err)? as u64);
                    }
                    Ok(out)
                })
                .await
            }
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let sql_run_id = to_sql_i64(run_id, "index_storage::delete_run run_id")?;
                    let conn = lock(&conn);
                    conn.execute("DELETE FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.0, sql_run_id]).map_err(sqlite_err)?;
                    Ok(())
                })
                .await
            }
        }
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    impl LeaseStorage for SqliteStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            let conn = self.conn.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let mut conn = lock(&conn);
                    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                    let existing: Option<(String, i64, i64)> = tx.query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(sqlite_err)?;
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
                })
                .await
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let mut conn = lock(&conn);
                    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate).map_err(sqlite_err)?;
                    let (existing_holder, epoch, expires_at_ms): (String, i64, i64) = tx
                        .query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
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
                })
                .await
            }
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            let conn = self.conn.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let mut conn = lock(&conn);
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
                })
                .await
            }
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            let conn = self.conn.clone();
            let resource = resource.to_string();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_ref(), DbIoRequest::metadata(), move || {
                    let conn = lock(&conn);
                    let existing: Option<(String, i64, i64)> =
                        conn.query_row("SELECT holder, epoch, expires_at_ms FROM lease WHERE resource = ?1", params![resource], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(sqlite_err)?;
                    Ok(existing.and_then(
                        |(holder, epoch, expires_at_ms)| {
                            if (now_ms as i64) < expires_at_ms {
                                Some(LeaseInfo { resource: resource.clone(), holder, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 })
                            } else {
                                None
                            }
                        },
                    ))
                })
                .await
            }
        }
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbBackend
    impl SqliteStorage {
        /// @emoji 🎚️ Always durable, `fsync`-capable, CAS-capable — `synchronous = FULL` mode.
        pub async fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }
    //#endregion 🔖️DbBackend

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::future::Future;

        fn pages(bytes: &[u8]) -> DbIoPages {
            DbIoPages::try_new(bytes.to_vec()).expect("test sqlite bytes must fit the fixed page owner")
        }

        /// @emoji ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every
        /// future a `SqliteStorage` driven by `semio_framework_async::testkit::ManualRuntime` hands
        /// back resolves on its first poll (`ManualRuntime::run_blocking` executes synchronously),
        /// so this drives one to completion without needing a real executor.
        async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
            fut.await
        }

        async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
            poll_once(fut).await
        }

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

        /// @emoji 🎲️ Opened on the test-owned process pool, exercising the retained path.
        async fn sqlite_scratch(name: &str) -> SqliteStorage {
            poll_once(SqliteStorage::open(crate::db_storage::db_io_test_pool(), &sqlite_scratch_path(name))).await.unwrap()
        }

        //#region 🔖️WalStorage
        #[semio_framework_async_macros::async_test]
        async fn wal_storage_append_seal_truncate_and_bounds_laws() {
            let storage = sqlite_scratch("wal_laws").await;
            let document: ArtifactId = "doc-wal".into();

            block_on_ready(storage.create_segment(&document, 0)).await.unwrap();
            assert!(matches!(block_on_ready(storage.create_segment(&document, 0)).await, Err(DbError::AlreadyExists(_))));

            let len_after_first = block_on_ready(storage.append(&document, 0, pages(b"hello "))).await.unwrap();
            assert_eq!(len_after_first, 6);
            let len_after_second = block_on_ready(storage.append(&document, 0, pages(b"world"))).await.unwrap();
            assert_eq!(len_after_second, 11);
            assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);

            let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).await.unwrap();
            assert_eq!(read_back, b"world");
            assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })).await, Err(DbError::InvalidArgument(_))));

            block_on_ready(storage.sync(&document, 0, DurabilityClass::Fsync)).await.unwrap();

            block_on_ready(storage.truncate_tail(&document, 0, 6)).await.unwrap();
            assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
            assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");

            block_on_ready(storage.create_segment(&document, 1)).await.unwrap();
            assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0, 1]);

            block_on_ready(storage.seal(&document, 0)).await.unwrap();
            block_on_ready(storage.seal(&document, 0)).await.unwrap(); // idempotent
            assert!(matches!(block_on_ready(storage.append(&document, 0, pages(b"!"))).await, Err(DbError::InvalidArgument(_))));
            assert!(matches!(block_on_ready(storage.truncate_tail(&document, 0, 0)).await, Err(DbError::InvalidArgument(_))));

            block_on_ready(storage.delete_segment(&document, 1)).await.unwrap();
            assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0]);

            assert!(matches!(block_on_ready(storage.append(&document, 99, pages(b"x"))).await, Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️WalStorage

        //#region 🔖️SnapshotStorage
        #[semio_framework_async_macros::async_test]
        async fn snapshot_storage_generations_overwrite_and_delete_laws() {
            let storage = sqlite_scratch("snapshot_laws").await;
            let document: ArtifactId = "doc-snap".into();
            assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), None);

            block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-bytes"))).await.unwrap();
            block_on_ready(storage.write_generation(&document, 1, pages(b"gen-one-bytes"))).await.unwrap();
            assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![0, 1]);
            assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), Some(1));
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-bytes");

            block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-overwritten"))).await.unwrap();
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-overwritten");

            block_on_ready(storage.delete_generation(&document, 0)).await.unwrap();
            assert!(matches!(block_on_ready(storage.read_generation(&document, 0)).await, Err(DbError::NotFound(_))));
            assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![1]);
        }
        //#endregion 🔖️SnapshotStorage

        //#region 🔖️PayloadStorage
        #[semio_framework_async_macros::async_test]
        async fn payload_storage_is_content_addressed_and_idempotent() {
            let storage = sqlite_scratch("payload_laws").await;
            let bytes = b"a payload blob that gets content-addressed";
            let hash_a = block_on_ready(storage.put(pages(bytes))).await.unwrap();
            let hash_b = block_on_ready(storage.put(pages(bytes))).await.unwrap();
            assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
            assert_eq!(hash_a, ContentHash(*blake3::hash(bytes).as_bytes()));

            assert!(block_on_ready(storage.contains(&hash_a)).await.unwrap());
            assert_eq!(block_on_ready(storage.get(&hash_a)).await.unwrap(), bytes);
            assert_eq!(block_on_ready(storage.len(&hash_a)).await.unwrap(), bytes.len() as u64);

            let other_hash = ContentHash([0xAB; 32]);
            assert!(!block_on_ready(storage.contains(&other_hash)).await.unwrap());
            assert!(matches!(block_on_ready(storage.get(&other_hash)).await, Err(DbError::NotFound(_))));

            block_on_ready(storage.delete(&hash_a)).await.unwrap();
            assert!(!block_on_ready(storage.contains(&hash_a)).await.unwrap());
            assert!(matches!(block_on_ready(storage.get(&hash_a)).await, Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️PayloadStorage

        //#region 🔖️CatalogStorage
        #[semio_framework_async_macros::async_test]
        async fn catalog_storage_cas_root_fences_stale_writers() {
            let storage = sqlite_scratch("catalog_laws").await;
            assert_eq!(block_on_ready(storage.read_root()).await.unwrap(), None);

            let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-v1"))).await.unwrap();
            assert_eq!(epoch_1, EpochFence::INITIAL.next());
            let (bytes, fence) = block_on_ready(storage.read_root()).await.unwrap().unwrap();
            assert_eq!(bytes, b"root-v1");
            assert_eq!(fence, epoch_1);

            // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
            assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-stale"))).await, Err(DbError::Fenced { .. })));

            let epoch_2 = block_on_ready(storage.cas_root(epoch_1, pages(b"root-v2"))).await.unwrap();
            assert_eq!(epoch_2, epoch_1.next());
            assert_eq!(block_on_ready(storage.read_root()).await.unwrap().unwrap().0, b"root-v2");
        }
        //#endregion 🔖️CatalogStorage

        //#region 🔖️IndexStorage
        #[semio_framework_async_macros::async_test]
        async fn index_storage_runs_list_read_and_delete_laws() {
            let storage = sqlite_scratch("index_laws").await;
            let document: ArtifactId = "doc-index".into();
            block_on_ready(storage.write_run(&document, 0, pages(b"run-zero"))).await.unwrap();
            block_on_ready(storage.write_run(&document, 1, pages(b"run-one"))).await.unwrap();
            assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![0, 1]);
            assert_eq!(block_on_ready(storage.read_run(&document, 1)).await.unwrap(), b"run-one");

            block_on_ready(storage.delete_run(&document, 0)).await.unwrap();
            assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![1]);
            assert!(matches!(block_on_ready(storage.read_run(&document, 0)).await, Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️IndexStorage

        //#region 🔖️LeaseStorage
        #[semio_framework_async_macros::async_test]
        async fn lease_storage_acquire_renew_fence_and_handoff_laws() {
            let storage = sqlite_scratch("lease_laws").await;
            let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).await.unwrap();
            assert_eq!(fence_1, EpochFence::INITIAL);

            // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
            let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).await.unwrap();
            assert_eq!(fence_reacquire, fence_1);

            // A different holder cannot acquire an unexpired lease.
            assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)).await, Err(DbError::Conflict(_))));

            block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).await.unwrap();
            assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)).await, Err(DbError::Fenced { .. })));
            assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)).await, Err(DbError::Unauthorized(_))));

            let current = block_on_ready(storage.current("shard-1", 600)).await.unwrap().unwrap();
            assert_eq!(current.holder, "node-a");
            assert_eq!(current.fence, fence_1);

            // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
            // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
            assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).await.unwrap(), None);
            let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).await.unwrap();
            assert_eq!(fence_2, fence_1.next());

            // The old holder's stale fence is now rejected.
            assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)).await, Err(DbError::Unauthorized(_))));

            block_on_ready(storage.release("shard-1", "node-b", fence_2)).await.unwrap();
            assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).await.unwrap(), None);
            assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)).await, Err(DbError::NotFound(_))));
        }
        //#endregion 🔖️LeaseStorage

        //#region 🔖️DbBackend
        #[semio_framework_async_macros::async_test]
        async fn db_backend_accessors_and_capabilities() {
            let storage: crate::db_storage::DbBackend = crate::db_storage::DbBackend::Sqlite(poll_once(SqliteStorage::open(crate::db_storage::db_io_test_pool(), &sqlite_scratch_path("umbrella"))).await.unwrap());
            let document: ArtifactId = "doc-umbrella".into();
            block_on_ready(poll_once(storage.wal()).await.create_segment(&document, 0)).await.unwrap();
            block_on_ready(poll_once(storage.catalog()).await.cas_root(EpochFence::INITIAL, pages(b"root"))).await.unwrap();
            block_on_ready(poll_once(storage.index()).await.write_run(&document, 0, pages(b"run"))).await.unwrap();
            assert_eq!(block_on_ready(poll_once(storage.index()).await.read_run(&document, 0)).await.unwrap(), b"run");

            let capabilities = poll_once(storage.capabilities()).await;
            assert!(capabilities.durable);
            assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
            assert!(capabilities.supports_fsync);
            assert!(capabilities.supports_cas);
        }
        //#endregion 🔖️DbBackend

        //#region 🔖️Connection
        #[semio_framework_async_macros::async_test]
        async fn write_survives_reopen_across_instances_against_a_real_file() {
            let path = sqlite_scratch_path("reopen");
            {
                let storage = poll_once(SqliteStorage::open(crate::db_storage::db_io_test_pool(), &path)).await.unwrap();
                let document: ArtifactId = "doc-reopen".into();
                block_on_ready(storage.write_generation(&document, 0, pages(b"persisted across reopen"))).await.unwrap();
                block_on_ready(storage.put(pages(b"payload persisted across reopen"))).await.unwrap();
            }
            {
                let storage = poll_once(SqliteStorage::open(crate::db_storage::db_io_test_pool(), &path)).await.unwrap();
                let document: ArtifactId = "doc-reopen".into();
                assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"persisted across reopen");
                let hash = ContentHash(*blake3::hash(b"payload persisted across reopen").as_bytes());
                assert_eq!(block_on_ready(storage.get(&hash)).await.unwrap(), b"payload persisted across reopen");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn in_memory_storage_works_without_a_file() {
            let storage = poll_once(SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool())).await.unwrap();
            let document: ArtifactId = "doc-mem".into();
            block_on_ready(storage.create_segment(&document, 0)).await.unwrap();
            assert_eq!(block_on_ready(storage.append(&document, 0, pages(b"in memory"))).await.unwrap(), 9);
        }
        //#endregion 🔖️Connection
    }
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite_storage::SqliteStorage;
//#endregion 🔖️SqliteStorage
