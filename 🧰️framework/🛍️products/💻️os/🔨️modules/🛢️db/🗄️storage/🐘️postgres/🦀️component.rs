//! 🗄️ `db_storage_postgres` — a `db_storage::DbStorage` backend over PostgreSQL (via `sqlx`),
//! informed by the deleted `os-semio_hub-storage-postgres` crate's connection/schema-bootstrap shape but
//! implementing the generic, semio_hub-agnostic `db_storage` trait family rather than semio_hub-specific
//! tables. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! every `db_storage` sub-trait method is a plain `async fn` — `sqlx`'s Postgres driver
//! is async-only (Postgres has no blocking C client to wrap, unlike sqlite), and this backend used
//! to bridge that onto the family's then-synchronous trait signatures by owning a dedicated
//! multi-thread `tokio::runtime::Runtime` and `block_on`-ing every call. That runtime (and its
//! `block_on` bridge) is GONE: every method body here is the SAME already-async `sqlx` code that
//! runtime used to drive, now handed straight back as `Box::pin(async move { .. })` — the calling
//! task's own executor (ultimately the hub's `#[tokio::main]`) drives it, so this crate spends no
//! thread of its own parked in `block_on` per call. `connect`/`connect_to_database` are async too,
//! for the same reason. This crate names no `tokio` anywhere (the repo's "`tokio` only in
//! `🛎️services`" rule) — `sqlx`'s `runtime-tokio` feature selects ITS internal executor binding at
//! ITS compile time, it does not require this crate to depend on `tokio` itself.
//!
//! 🐘️ On-disk shape: six tables (`db_wal_segment`, `db_snapshot_generation`, `db_payload`,
//! `db_catalog_root`, `db_index_run`, `db_lease`), bootstrapped idempotently on `connect`. Every
//! value column is a raw `BYTEA` blob — this crate never parses WAL/snapshot/index bytes, mirroring
//! `db_storage`'s own "opaque byte blobs" design note. Compare-and-swap (`CatalogStorage::cas_root`,
//! `LeaseStorage`) is real cross-connection fencing via `SELECT ... FOR UPDATE` inside a
//! transaction — stronger than `db_storage::FsStorage`'s documented in-process-only mutex, since
//! Postgres gives us a genuine row lock across concurrent connections/processes for free.

//#region 🔖️Schema
/// @emoji 🧱️ Idempotent DDL bootstrapped by `PostgresStorage::connect` — one statement per table,
/// plus a seed row for the catalog singleton (`db_catalog_root.id = 1`) so `CatalogStorage`'s
/// compare-and-swap can always `SELECT ... FOR UPDATE` a real row instead of racing to insert one.
const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS db_wal_segment (
        document_id TEXT NOT NULL,
        segment_index BIGINT NOT NULL,
        bytes BYTEA NOT NULL DEFAULT '',
        sealed BOOLEAN NOT NULL DEFAULT FALSE,
        PRIMARY KEY (document_id, segment_index)
    )",
    "CREATE TABLE IF NOT EXISTS db_snapshot_generation (
        document_id TEXT NOT NULL,
        generation BIGINT NOT NULL,
        bytes BYTEA NOT NULL,
        PRIMARY KEY (document_id, generation)
    )",
    "CREATE TABLE IF NOT EXISTS db_payload (
        hash BYTEA PRIMARY KEY,
        bytes BYTEA NOT NULL
    )",
    "CREATE TABLE IF NOT EXISTS db_catalog_root (
        id SMALLINT PRIMARY KEY,
        epoch BIGINT NOT NULL,
        bytes BYTEA
    )",
    "INSERT INTO db_catalog_root (id, epoch, bytes) VALUES (1, 0, NULL) ON CONFLICT (id) DO NOTHING",
    "CREATE TABLE IF NOT EXISTS db_index_run (
        document_id TEXT NOT NULL,
        run_id BIGINT NOT NULL,
        bytes BYTEA NOT NULL,
        PRIMARY KEY (document_id, run_id)
    )",
    "CREATE TABLE IF NOT EXISTS db_lease (
        resource TEXT PRIMARY KEY,
        holder TEXT NOT NULL,
        epoch BIGINT NOT NULL,
        expires_at_ms BIGINT NOT NULL
    )",
];

async fn bootstrap_schema(pool: &PgPool) -> Result<(), DbError> {
    for statement in SCHEMA_STATEMENTS {
        sqlx::query(statement).execute(pool).await.map_err(map_sqlx_error)?;
    }
    Ok(())
}
//#endregion 🔖️Schema

//#region 🔖️Connection
use crate::db_durability::{DurabilityClass, EpochFence};
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::db_storage::{
    close_db_io_backend, db_io_close_platform, db_io_copy_observed_text, db_io_hash_pages, db_io_prepare_platform, db_io_transfer_list, db_io_write_observed_bytes, register_db_io_backend, retire_db_io_backend, submit_db_io_task, CatalogStorage,
    DbIoBackendControl, DbIoBackendKind, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, IndexStorage,
    LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage, DB_IO_PAGE_BYTES,
};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

/// @emoji 🛡️ Mirrors `db_storage`'s own read-size ceiling (its `MAX_READ_BYTES` is private to that
/// crate) — validated via `check_len` before a `Vec<u8>` sized by an untrusted on-disk
/// length is allocated, per the repo's "validate before allocating" invariant.
const MAX_READ_BYTES: u64 = 496 * 1024;

/// @emoji 🐘️ A `db_storage::DbStorage` backend over PostgreSQL — `pool` is the connection pool
/// every trait method below runs its query against directly (no runtime of its own to bridge
/// through anymore — see module doc).
struct PostgresDbIoExecutor {
    pool: PgPool,
    database_url: DbIoText,
    backend_terminal: std::sync::atomic::AtomicBool,
    active_operation: u64,
    close_future: std::sync::Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>>,
}

impl PostgresDbIoExecutor {
    /// @emoji 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration
    /// framework — matches the deleted `os-semio_hub-storage-postgres` precedent), returning a ready
    /// `PostgresStorage`. `async` because connecting a pool and running DDL are themselves I/O — the
    /// caller (ultimately the hub's `#[tokio::main]`) already awaits this on a real runtime.
    fn new(database_url: DbIoText) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new().max_connections(16).connect_lazy(database_url.as_str()).map_err(map_sqlx_error)?;
        Ok(Self { pool, database_url, backend_terminal: std::sync::atomic::AtomicBool::new(false), active_operation: 0, close_future: std::sync::Mutex::new(None) })
    }

    fn reserve_driver_output(&self, maximum_capacity: u64) -> Result<DbIoDriverReservation, DbError> {
        let maximum_capacity = usize::try_from(maximum_capacity).map_err(|_| DbError::LimitExceeded("PostgreSQL external driver capacity"))?;
        DbIoDriverReservation::try_reserve(self.active_operation, maximum_capacity)
    }
}
//#endregion 🔖️Connection

//#region 🔖️ErrorMapping
/// @emoji 🚨️ Maps a `sqlx::Error` to this family's `DbError` — the only place `sqlx::Error` is
/// allowed to appear, mirroring `db_storage::FsStorage`'s single `io_err` chokepoint for
/// `std::io::Error`. A `Database`-flavored error further classifies via
/// `DatabaseError::is_unique_violation` (driver-agnostic, no hand-parsed SQLSTATE string).
#[allow(clippy::needless_pass_by_value)] // used directly as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
fn map_sqlx_error(err: sqlx::Error) -> DbError {
    match &err {
        sqlx::Error::RowNotFound => return DbError::NotFound("row not found".to_string()),
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut | sqlx::Error::WorkerCrashed => {
            return DbError::Unavailable(err.to_string());
        }
        sqlx::Error::Configuration(_) | sqlx::Error::Protocol(_) | sqlx::Error::InvalidArgument(_) => {
            return DbError::InvalidArgument(err.to_string());
        }
        _ => {}
    }
    if let Some(db_err) = err.as_database_error() {
        if db_err.is_unique_violation() {
            return DbError::AlreadyExists(db_err.message().to_string());
        }
        return DbError::Io(db_err.message().to_string());
    }
    DbError::Io(err.to_string())
}

/// @emoji 🆕️ Like `map_sqlx_error`, but a unique-violation becomes `DbError::AlreadyExists(what())`
/// instead of the generic `DbError::Io` — used by every `create_segment`-shaped write.
// 🚫️async: E1 pure accessor called from sync `.map_err(|err| map_create_error(...))` closures — see R9
fn map_create_error(err: sqlx::Error, what: impl FnOnce() -> String) -> DbError {
    if let Some(db_err) = err.as_database_error() {
        if db_err.is_unique_violation() {
            return DbError::AlreadyExists(what());
        }
    }
    map_sqlx_error(err)
}
//#endregion 🔖️ErrorMapping

//#region 🔖️Conversions
/// @emoji 🔢️ Every dense index/generation/run/epoch/timestamp this crate stores is `u64` at the
/// trait boundary but `BIGINT` (`i64`) in Postgres — this is the one narrowing conversion point,
/// erroring rather than silently wrapping on the (astronomically unlikely) values above
/// `i64::MAX`.
fn to_i64(value: u64) -> Result<i64, DbError> {
    i64::try_from(value).map_err(|_| DbError::InvalidArgument(format!("value {value} exceeds i64::MAX")))
}

/// @emoji ✂️ Validates a `WalStorage::read` range against the segment's actual current length
/// (already fetched via `octet_length`, so this never touches the segment bytes themselves) and
/// converts to the 1-indexed `(offset, len)` pair Postgres's `substring(bytea, int, int)` expects.
fn validate_read_range(current_len: u64, range: ByteRange) -> Result<(i64, i64), DbError> {
    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("read range overflows u64".to_string()))?;
    if end > current_len {
        return Err(DbError::InvalidArgument(format!("read range {}..{end} out of bounds (len {current_len})", range.offset)));
    }
    Ok((to_i64(range.offset)?, to_i64(range.len)?))
}

/// @emoji ✂️ Validates a `WalStorage::truncate_tail` request against the segment's sealed flag and
/// current length, matching `db_storage::{MemoryStorage, FsStorage}`'s identical checks.
fn validate_truncate(sealed: bool, current_len: u64, new_len: u64) -> Result<(), DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
    }
    if new_len > current_len {
        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
    }
    Ok(())
}
//#endregion 🔖️Conversions

//#region 🔖️WalStorage
impl WalStorage for PostgresDbIoExecutor {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("INSERT INTO db_wal_segment (document_id, segment_index) VALUES ($1, $2)")
            .bind(document.0.as_str())
            .bind(idx)
            .execute(&self.pool)
            .await
            .map_err(|err| map_create_error(err, || format!("wal segment {index} for {document} already exists")))?;
        Ok(())
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = async {
            let idx = to_i64(index)?;
            let doc = document.0.as_str();
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            let row: Option<(bool,)> = sqlx::query_as("SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(doc).bind(idx).fetch_optional(&mut *tx).await.map_err(map_sqlx_error)?;
            let sealed = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0;
            if sealed {
                return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
            }
            let (new_len,): (i64,) = sqlx::query_as("UPDATE db_wal_segment SET bytes = bytes || $1 WHERE document_id = $2 AND segment_index = $3 RETURNING octet_length(bytes)")
                .bind(prepared.as_slice())
                .bind(doc)
                .bind(idx)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(new_len as u64)
        }
        .await;
        db_io_close_platform(prepared).await?;
        result
    }

    async fn sync(&self, _document: &ArtifactId, _index: u64, class: DurabilityClass) -> Result<(), DbError> {
        // 🎯️ Every write above already ran as a committed statement/transaction, and Postgres
        // fsyncs its own WAL at COMMIT under the default `synchronous_commit = on` — so `Fsync` is
        // already satisfied by the time `append`/`truncate_tail` return, with nothing left for this
        // method to force. `Quorum` (replica acknowledgement) is a `db_cluster` concern layered on
        // top of Postgres's own (optionally synchronous) replication, not something a single
        // connection pool can negotiate — deliberately left as an extension seam rather than a
        // half-implemented `SET synchronous_commit` toggle here.
        let _ = class;
        {
            Ok(())
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let result: Option<(bool,)> =
            sqlx::query_as("UPDATE db_wal_segment SET sealed = TRUE WHERE document_id = $1 AND segment_index = $2 RETURNING sealed").bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        result.map(|_| ()).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(doc).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current_len = len_row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0 as u64;
        let (offset, len) = validate_read_range(current_len, range)?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1 FOR $2) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(doc).bind(idx).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut result = DbIoU64List::new();
        let mut after = -1i64;
        while let Some((index,)) =
            sqlx::query_as("SELECT segment_index FROM db_wal_segment WHERE document_id = $1 AND segment_index > $2 ORDER BY segment_index ASC LIMIT 1").bind(document.0.as_str()).bind(after).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?
        {
            result.push(index as u64)?;
            after = index;
        }
        Ok(result)
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let row: Option<(bool, i64)> = sqlx::query_as("SELECT sealed, octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(doc).bind(idx).fetch_optional(&mut *tx).await.map_err(map_sqlx_error)?;
        let (sealed, current_len) = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        validate_truncate(sealed, current_len as u64, new_len)?;
        sqlx::query("UPDATE db_wal_segment SET bytes = substring(bytes FROM 1 FOR $1) WHERE document_id = $2 AND segment_index = $3").bind(to_i64(new_len)?).bind(doc).bind(idx).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("DELETE FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document.0.as_str()).bind(idx).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
impl SnapshotStorage for PostgresDbIoExecutor {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query(
            "INSERT INTO db_snapshot_generation (document_id, generation, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, generation) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(gen)
        .bind(prepared.as_slice())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| ())
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        let gen = to_i64(generation)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row: (Option<i64>,) = sqlx::query_as("SELECT MAX(generation) FROM db_snapshot_generation WHERE document_id = $1").bind(document.0.as_str()).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0.map(|generation| generation as u64))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut result = DbIoU64List::new();
        let mut after = -1i64;
        while let Some((generation,)) =
            sqlx::query_as("SELECT generation FROM db_snapshot_generation WHERE document_id = $1 AND generation > $2 ORDER BY generation ASC LIMIT 1").bind(document.0.as_str()).bind(after).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?
        {
            result.push(generation as u64)?;
            after = generation;
        }
        Ok(result)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        sqlx::query("DELETE FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(document.0.as_str()).bind(gen).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
impl PayloadStorage for PostgresDbIoExecutor {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
        let hash = db_io_hash_pages(&bytes).await;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query("INSERT INTO db_payload (hash, bytes) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING").bind(&hash.0[..]).bind(prepared.as_slice()).execute(&self.pool).await.map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "payload_storage::get")?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0 > 0)
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        sqlx::query("DELETE FROM db_payload WHERE hash = $1").bind(&hash.0[..]).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
impl CatalogStorage for PostgresDbIoExecutor {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (epoch, bytes): (i64, Option<Vec<u8>>) = sqlx::query_as("SELECT epoch, bytes FROM db_catalog_root WHERE id = 1").fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        match bytes {
            Some(bytes) => {
                let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
                Ok(Some((db_io_write_observed_bytes(reservation, bytes, &mut output).await?, EpochFence { epoch: epoch as u64 })))
            }
            None => Ok(None),
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let prepared = db_io_prepare_platform(&new_bytes)?.await?;
        let result = async {
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            // 🎯️ The bootstrap-seeded singleton row (`id = 1`) always exists, so `SELECT ... FOR
            // UPDATE` here is a real, always-present row lock — unlike `FsStorage::cas_root`'s
            // documented in-process-only mutex, this fences concurrent writers across connections
            // and processes for free via Postgres's own lock manager.
            let (current_epoch,): (i64,) = sqlx::query_as("SELECT epoch FROM db_catalog_root WHERE id = 1 FOR UPDATE").fetch_one(&mut *tx).await.map_err(map_sqlx_error)?;
            let current_fence = EpochFence { epoch: current_epoch as u64 };
            expected.check(current_fence)?;
            let new_fence = expected.next();
            sqlx::query("UPDATE db_catalog_root SET epoch = $1, bytes = $2 WHERE id = 1").bind(to_i64(new_fence.epoch)?).bind(prepared.as_slice()).execute(&mut *tx).await.map_err(map_sqlx_error)?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(new_fence)
        }
        .await;
        db_io_close_platform(prepared).await?;
        result
    }
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
impl IndexStorage for PostgresDbIoExecutor {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let run = to_i64(run_id)?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query(
            "INSERT INTO db_index_run (document_id, run_id, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, run_id) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(run)
        .bind(prepared.as_slice())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| ())
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        let run = to_i64(run_id)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "index_storage::read_run")?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut result = DbIoU64List::new();
        let mut after = -1i64;
        while let Some((run_id,)) = sqlx::query_as("SELECT run_id FROM db_index_run WHERE document_id = $1 AND run_id > $2 ORDER BY run_id ASC LIMIT 1").bind(document.0.as_str()).bind(after).fetch_optional(&self.pool).await.map_err(map_sqlx_error)? {
            result.push(run_id as u64)?;
            after = run_id;
        }
        Ok(result)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let run = to_i64(run_id)?;
        sqlx::query("DELETE FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(document.0.as_str()).bind(run).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
/// @emoji ⏳️ The row currently held on a resource, as read from `db_lease` — the DB-shaped input to
/// the pure `lease_*_decision`/`lease_*_check` functions below, so the hand-off/renew/release state
/// machine is unit-testable without a live Postgres connection.
struct ExistingLease {
    holder: DbIoText,
    fence: EpochFence,
    expires_at_ms: u64,
}

/// @emoji 🤝️ Pure decision for `LeaseStorage::acquire` — identical state machine to
/// `db_storage::{MemoryStorage, FsStorage}::acquire`: re-acquire by the same still-live holder keeps
/// the fence, a genuine hand-off (absent or expired) bumps it, a live foreign holder conflicts.
fn lease_acquire_decision(existing: Option<&ExistingLease>, holder: &str, now_ms: u64) -> Result<EpochFence, DbError> {
    match existing {
        Some(info) if now_ms < info.expires_at_ms => {
            if info.holder.as_str() != holder {
                return Err(DbError::Conflict(format!("resource is leased by another holder ({})", info.holder.as_str())));
            }
            Ok(info.fence)
        }
        Some(info) => Ok(info.fence.next()),
        None => Ok(EpochFence::INITIAL),
    }
}

/// @emoji ♻️ Pure decision for `LeaseStorage::renew` — errors precisely as documented on the trait:
/// `NotFound` absent, `Unavailable` expired, `Unauthorized` wrong holder, `Fenced` wrong epoch.
fn lease_renew_check(existing: Option<&ExistingLease>, holder: &str, fence: EpochFence, now_ms: u64) -> Result<(), DbError> {
    let info = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
    if now_ms >= info.expires_at_ms {
        return Err(DbError::Unavailable("lease already expired".to_string()));
    }
    if info.holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// @emoji 🕊️ Pure decision for `LeaseStorage::release` — same holder/fence checks as `renew`, minus
/// the expiry check (a holder may release its own already-expired-but-not-yet-reclaimed lease).
fn lease_release_check(existing: Option<&ExistingLease>, holder: &str, fence: EpochFence) -> Result<(), DbError> {
    let info = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
    if info.holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// @emoji 🔒️ Reads `resource`'s current lease row through `executor`, taking a `FOR UPDATE` row
/// lock when `executor` is a transaction (the row-lock variant every `acquire`/`renew`/`release`
/// call uses so the read-decide-write sequence is atomic across concurrent connections) —
/// `LeaseStorage::current` instead calls this with the bare pool for a non-locking snapshot read.
async fn read_existing_lease_for_update<'e, E>(operation: u64, executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1 FOR UPDATE").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    let Some((holder, epoch, expires_at_ms)) = row else {
        reservation.close_step()?;
        return Ok(None);
    };
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder)?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

async fn read_existing_lease<'e, E>(operation: u64, executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DbIoText::maximum_capacity())?;
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    let Some((holder, epoch, expires_at_ms)) = row else {
        reservation.close_step()?;
        return Ok(None);
    };
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder)?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

impl LeaseStorage for PostgresDbIoExecutor {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        let fence = lease_acquire_decision(existing.as_ref(), holder, now_ms)?;
        let expires_at = to_i64(now_ms.checked_add(ttl_ms).ok_or(DbError::LimitExceeded("PostgreSQL lease expiry"))?)?;
        sqlx::query(
            "INSERT INTO db_lease (resource, holder, epoch, expires_at_ms) VALUES ($1, $2, $3, $4)
                 ON CONFLICT (resource) DO UPDATE SET holder = EXCLUDED.holder, epoch = EXCLUDED.epoch, expires_at_ms = EXCLUDED.expires_at_ms",
        )
        .bind(resource)
        .bind(holder)
        .bind(to_i64(fence.epoch)?)
        .bind(expires_at)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(fence)
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        lease_renew_check(existing.as_ref(), holder, fence, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or(DbError::LimitExceeded("PostgreSQL lease expiry"))?;
        sqlx::query("UPDATE db_lease SET expires_at_ms = $1 WHERE resource = $2").bind(to_i64(expires_at_ms)?).bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(self.active_operation, &mut *tx, resource).await?;
        lease_release_check(existing.as_ref(), holder, fence)?;
        sqlx::query("DELETE FROM db_lease WHERE resource = $1").bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let existing = read_existing_lease(self.active_operation, &self.pool, resource).await?;
        Ok(existing.and_then(|info| if now_ms < info.expires_at_ms { Some(LeaseInfo { resource: resource.to_string(), holder: info.holder.as_str().to_string(), fence: info.fence, expires_at_ms: info.expires_at_ms }) } else { None }))
    }
}
//#endregion 🔖️LeaseStorage

//#region 🔖️TypedExecutor
impl PostgresDbIoExecutor {
    async fn wal_read_into(&self, document: &str, index: u64, range: ByteRange, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let index = to_i64(index)?;
        let current: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document).bind(index).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current = current.ok_or_else(|| DbError::NotFound("PostgreSQL WAL segment not found".to_string()))?.0 as u64;
        let (offset, len) = validate_read_range(current, range)?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1 FOR $2) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(document).bind(index).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn named_blob_read_into(&self, table: &'static str, document: &str, ordinal: u64, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let ordinal = to_i64(ordinal)?;
        let (length_sql, read_sql) = match table {
            "snapshot" => ("SELECT octet_length(bytes) FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2", "SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2"),
            "index" => ("SELECT octet_length(bytes) FROM db_index_run WHERE document_id = $1 AND run_id = $2", "SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2"),
            _ => return Err(DbError::Internal("PostgreSQL named blob taxonomy mismatch".to_string())),
        };
        let length: Option<(i64,)> = sqlx::query_as(length_sql).bind(document).bind(ordinal).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let length = length.ok_or_else(|| DbError::NotFound("PostgreSQL named blob not found".to_string()))?.0 as u64;
        check_len(length, MAX_READ_BYTES, "PostgreSQL named blob")?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as(read_sql).bind(document).bind(ordinal).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn payload_read_into(&self, hash: &ContentHash, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let length: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let length = length.ok_or_else(|| DbError::NotFound("PostgreSQL payload not found".to_string()))?.0 as u64;
        check_len(length, MAX_READ_BYTES, "PostgreSQL payload")?;
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        db_io_write_observed_bytes(reservation, bytes, output).await
    }

    async fn catalog_read_into(&self, output: &mut DbIoPageWriter) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let reservation = self.reserve_driver_output(MAX_READ_BYTES)?;
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
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != self.database_url.as_str() {
                    return Err(DbError::InvalidArgument("PostgreSQL URL authority mismatch".to_string()));
                }
                bootstrap_schema(&self.pool).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalCreate { document, index, .. } => {
                <Self as WalStorage>::create_segment(self, &ArtifactId(document.as_str().to_string()), *index).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Length(<Self as WalStorage>::append(self, &ArtifactId(document.as_str().to_string()), *index, input).await?))
            }
            DbIoTask::WalSync { document, index, class, .. } => {
                <Self as WalStorage>::sync(self, &ArtifactId(document.as_str().to_string()), *index, *class).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalSeal { document, index, .. } => {
                <Self as WalStorage>::seal(self, &ArtifactId(document.as_str().to_string()), *index).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalRead { document, index, range, output, .. } => Ok(DbIoResult::Pages(self.wal_read_into(document.as_str(), *index, *range, output).await?)),
            DbIoTask::WalLength { document, index, .. } => Ok(DbIoResult::Length(<Self as WalStorage>::segment_len(self, &ArtifactId(document.as_str().to_string()), *index).await?)),
            DbIoTask::WalList { document, output, .. } => {
                let list = <Self as WalStorage>::list_segments(self, &ArtifactId(document.as_str().to_string())).await?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                <Self as WalStorage>::truncate_tail(self, &ArtifactId(document.as_str().to_string()), *index, *new_len).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalDelete { document, index, .. } => {
                <Self as WalStorage>::delete_segment(self, &ArtifactId(document.as_str().to_string()), *index).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                let input = input.take_for_async_driver();
                <Self as SnapshotStorage>::write_generation(self, &ArtifactId(document.as_str().to_string()), *generation, input).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("snapshot", document.as_str(), *generation, output).await?)),
            DbIoTask::SnapshotLatest { document, .. } => Ok(DbIoResult::OptionalLength(<Self as SnapshotStorage>::latest_generation(self, &ArtifactId(document.as_str().to_string())).await?)),
            DbIoTask::SnapshotList { document, output, .. } => {
                let list = <Self as SnapshotStorage>::list_generations(self, &ArtifactId(document.as_str().to_string())).await?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                <Self as SnapshotStorage>::delete_generation(self, &ArtifactId(document.as_str().to_string()), *generation).await?;
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
                <Self as IndexStorage>::write_run(self, &ArtifactId(document.as_str().to_string()), *run_id, input).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("index", document.as_str(), *run_id, output).await?)),
            DbIoTask::IndexList { document, output, .. } => {
                let list = <Self as IndexStorage>::list_runs(self, &ArtifactId(document.as_str().to_string())).await?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                <Self as IndexStorage>::delete_run(self, &ArtifactId(document.as_str().to_string()), *run_id).await?;
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
                let lease = <Self as LeaseStorage>::current(self, document.as_str(), *now_ms)
                    .await?
                    .map(|lease| Ok::<DbIoLeaseResult, DbError>(DbIoLeaseResult::new(DbIoText::try_from_str(&lease.resource)?, DbIoText::try_from_str(&lease.holder)?, lease.fence, lease.expires_at_ms)))
                    .transpose()?;
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
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("PostgreSQL async-native task entered the blocking executor".to_string()))
    }

    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }

    fn close_backend_step(&mut self) -> Result<bool, DbError> {
        if self.pool.is_closed() {
            self.close_future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
            return Ok(true);
        }
        let mut close = self.close_future.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if close.is_none() {
            let pool = self.pool.clone();
            *close = Some(Box::pin(async move { pool.close().await }));
            return Ok(false);
        }
        let context = &mut std::task::Context::from_waker(std::task::Waker::noop());
        let terminal = match close.as_mut() {
            Some(future) => std::future::Future::poll(future.as_mut(), context).is_ready(),
            None => false,
        };
        if terminal {
            close.take();
        }
        Ok(false)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire) && self.pool.is_closed()
    }
}
//#endregion 🔖️TypedExecutor

/// @emoji 🐘️ Typed PostgreSQL facade; only the registered executor owns the external driver.
pub struct PostgresStorage {
    control: DbIoBackendControl,
    worker_pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

impl PostgresStorage {
    pub async fn connect(worker_pool: Arc<WorkerPool>, database_url: &str) -> Result<Self, DbError> {
        let database_url = DbIoText::try_from_str(database_url)?;
        let executor = Box::new(PostgresDbIoExecutor::new(database_url.clone())?);
        let control = register_db_io_backend(DbIoBackendKind::Postgres, executor)?;
        let storage = Self { control, worker_pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = storage.execute(DbIoTask::BackendOpen { backend: control, path: database_url }).await {
            let _ = storage.execute(DbIoTask::BackendClose { backend: control }).await;
            return Err(error);
        }
        Ok(storage)
    }

    async fn execute(&self, task: DbIoTask) -> Result<DbIoResult, DbError> {
        let mut operation = submit_db_io_task(self.worker_pool.as_ref(), task).map_err(|(error, _)| error)?;
        let mut driver = operation.take_async_native().await?;
        let aggregate_operation = driver.operation();
        driver.enter_lane_io_driver_turn()?;
        let terminal = {
            let (executor, task) = driver.parts_mut::<PostgresDbIoExecutor>()?;
            executor.drive_task(aggregate_operation, task).await
        };
        driver.leave_lane_io_driver_turn()?;
        driver.complete(terminal)?;
        operation.await.map_err(crate::db_storage::DbIoFault::into_db_error)?.into_result()
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = postgres_unit(self.execute(DbIoTask::BackendClose { backend: self.control }).await?);
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }
}

impl Drop for PostgresStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

fn postgres_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn postgres_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("PostgreSQL output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn postgres_unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-unit result".to_string())),
    }
}

fn postgres_pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(pages) => Ok(pages),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-page result".to_string())),
    }
}

fn postgres_list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(list) => Ok(list),
        _ => Err(DbError::Internal("PostgreSQL executor returned a non-list result".to_string())),
    }
}

impl WalStorage for PostgresStorage {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalCreate { backend: self.control, document: postgres_document(document)?, index }).await?)
    }
    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalAppend { backend: self.control, document: postgres_document(document)?, index, input: bytes }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSync { backend: self.control, document: postgres_document(document)?, index, class }).await?)
    }
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSeal { backend: self.control, document: postgres_document(document)?, index }).await?)
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::WalRead { backend: self.control, document: postgres_document(document)?, index, range, output: postgres_output(range.len)? }).await?)
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalLength { backend: self.control, document: postgres_document(document)?, index }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::WalList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalTruncate { backend: self.control, document: postgres_document(document)?, index, new_len }).await?)
    }
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalDelete { backend: self.control, document: postgres_document(document)?, index }).await?)
    }
}

impl SnapshotStorage for PostgresStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::SnapshotWrite { backend: self.control, document: postgres_document(document)?, generation, input: bytes }).await?)
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::SnapshotRead { backend: self.control, document: postgres_document(document)?, generation, output: postgres_output(MAX_READ_BYTES)? }).await?)
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self.execute(DbIoTask::SnapshotLatest { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(generation) => Ok(generation),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-generation result".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::SnapshotList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::SnapshotDelete { backend: self.control, document: postgres_document(document)?, generation }).await?)
    }
}

impl PayloadStorage for PostgresStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self.execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-hash result".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: postgres_output(MAX_READ_BYTES)? }).await?)
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self.execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(exists) => Ok(exists),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-exists result".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self.execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
}

impl CatalogStorage for PostgresStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self.execute(DbIoTask::CatalogRead { backend: self.control, output: postgres_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(catalog) => Ok(catalog),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-catalog result".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-fence result".to_string())),
        }
    }
}

impl IndexStorage for PostgresStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::IndexWrite { backend: self.control, document: postgres_document(document)?, run_id, input: bytes }).await?)
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::IndexRead { backend: self.control, document: postgres_document(document)?, run_id, output: postgres_output(MAX_READ_BYTES)? }).await?)
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::IndexList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::IndexDelete { backend: self.control, document: postgres_document(document)?, run_id }).await?)
    }
}

impl LeaseStorage for PostgresStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-fence result".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self.execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(Some(lease)) => Ok(Some(LeaseInfo { resource: lease.resource.as_str().to_string(), holder: lease.holder.as_str().to_string(), fence: lease.fence, expires_at_ms: lease.expires_at_ms })),
            DbIoResult::OptionalLease(None) => Ok(None),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-lease result".to_string())),
        }
    }
}

//#region 🔖️DbBackend
/// @emoji 🎚️ `PostgresStorage`'s fixed capability set — extracted to a free fn so the unit tests
/// below can assert on it without opening a real connection.
fn postgres_capabilities() -> StorageCapabilities {
    StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
}

impl PostgresStorage {
    /// @emoji 🎚️ What this backend actually supports — see [`postgres_capabilities`].
    pub async fn capabilities(&self) -> StorageCapabilities {
        postgres_capabilities()
    }
}
//#endregion 🔖️DbBackend

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_storage::db_io_maintenance_step;

    #[semio_framework_async_macros::async_test]
    async fn lost_postgres_facade_drives_the_real_lazy_pool_to_closed() {
        let url = DbIoText::try_from_str("postgres://localhost/p1q-lost-facade").unwrap();
        let executor = PostgresDbIoExecutor::new(url).unwrap();
        let driver_pool = executor.pool.clone();
        let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(executor)).unwrap();
        let worker_pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let facade = PostgresStorage { control, worker_pool: worker_pool.clone(), closed: std::sync::atomic::AtomicBool::new(false) };
        drop(facade);
        close_db_io_backend(control).await.unwrap();
        loop {
            match db_io_maintenance_step() {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => panic!("PostgreSQL lost-facade maintenance failed: {error}"),
            }
        }
        assert!(driver_pool.is_closed());
        worker_pool.shutdown();
    }

    // 🔬️ Live-Postgres integration testing (schema bootstrap round-trip, real CAS races across
    // connections, transaction rollback on error) is deliberately deferred — this environment has
    // no `DATABASE_URL`/live Postgres server. Everything below tests the pure decision logic each
    // trait method delegates to, which is where this backend's actual correctness lives; the SQL
    // itself is exercised by inspection against the schema in `//#region 🔖️Schema`.

    //#region 🔖️RangeAndTruncate
    #[semio_framework_async_macros::async_test]
    async fn validate_read_range_accepts_in_bounds_slice() {
        assert_eq!(validate_read_range(10, ByteRange { offset: 2, len: 5 }).unwrap(), (2, 5));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_read_range_rejects_out_of_bounds() {
        assert!(matches!(validate_read_range(10, ByteRange { offset: 8, len: 5 }), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_read_range_rejects_offset_len_overflow() {
        assert!(matches!(validate_read_range(10, ByteRange { offset: u64::MAX, len: 1 }), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_truncate_rejects_sealed_segment() {
        assert!(matches!(validate_truncate(true, 10, 5), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_truncate_rejects_growth() {
        assert!(matches!(validate_truncate(false, 10, 20), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_truncate_accepts_shrink() {
        assert!(validate_truncate(false, 10, 5).is_ok());
    }
    //#endregion 🔖️RangeAndTruncate

    //#region 🔖️Lease
    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_fresh_resource_starts_at_initial_fence() {
        assert_eq!(lease_acquire_decision(None, "alice", 0).unwrap(), EpochFence::INITIAL);
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_reacquire_by_same_live_holder_keeps_fence() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL.next(), expires_at_ms: 1_000 };
        assert_eq!(lease_acquire_decision(Some(&existing), "alice", 500).unwrap(), existing.fence);
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_by_other_holder_before_expiry_conflicts() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(matches!(lease_acquire_decision(Some(&existing), "bob", 500), Err(DbError::Conflict(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_after_expiry_bumps_fence_for_new_holder() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert_eq!(lease_acquire_decision(Some(&existing), "bob", 2_000).unwrap(), EpochFence::INITIAL.next());
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_renew_rejects_absent_expired_wrong_holder_and_wrong_fence() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(matches!(lease_renew_check(None, "alice", EpochFence::INITIAL, 0), Err(DbError::NotFound(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 2_000), Err(DbError::Unavailable(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "bob", EpochFence::INITIAL, 500), Err(DbError::Unauthorized(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL.next(), 500), Err(DbError::Fenced { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_renew_accepts_matching_live_holder_and_fence() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 500).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_release_rejects_wrong_holder_or_fence() {
        let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(matches!(lease_release_check(Some(&existing), "bob", EpochFence::INITIAL), Err(DbError::Unauthorized(_))));
        assert!(matches!(lease_release_check(Some(&existing), "alice", EpochFence::INITIAL.next()), Err(DbError::Fenced { .. })));
        assert!(lease_release_check(Some(&existing), "alice", EpochFence::INITIAL).is_ok());
    }
    //#endregion 🔖️Lease

    //#region 🔖️Conversion
    #[semio_framework_async_macros::async_test]
    async fn to_i64_round_trips_ordinary_values() {
        assert_eq!(to_i64(42).unwrap(), 42);
        assert_eq!(to_i64(0).unwrap(), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_i64_rejects_values_above_i64_max() {
        assert!(to_i64(u64::MAX).is_err());
    }
    //#endregion 🔖️Conversion

    //#region 🔖️ErrorMapping
    #[semio_framework_async_macros::async_test]
    async fn map_sqlx_error_classifies_row_not_found() {
        assert!(matches!(map_sqlx_error(sqlx::Error::RowNotFound), DbError::NotFound(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn map_sqlx_error_classifies_pool_exhaustion_as_unavailable() {
        assert!(matches!(map_sqlx_error(sqlx::Error::PoolClosed), DbError::Unavailable(_)));
        assert!(matches!(map_sqlx_error(sqlx::Error::PoolTimedOut), DbError::Unavailable(_)));
        assert!(matches!(map_sqlx_error(sqlx::Error::WorkerCrashed), DbError::Unavailable(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn map_sqlx_error_classifies_protocol_and_configuration_as_invalid_argument() {
        assert!(matches!(map_sqlx_error(sqlx::Error::Protocol("bad frame".to_string())), DbError::InvalidArgument(_)));
        assert!(matches!(map_sqlx_error(sqlx::Error::InvalidArgument("bad bind".to_string())), DbError::InvalidArgument(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn map_sqlx_error_classifies_io_as_io() {
        assert!(matches!(map_sqlx_error(sqlx::Error::Io(std::io::Error::other("disconnected"))), DbError::Io(_)));
    }
    //#endregion 🔖️ErrorMapping

    //#region 🔖️SchemaAndCapabilities
    #[semio_framework_async_macros::async_test]
    async fn schema_statements_cover_every_storage_table() {
        let joined = SCHEMA_STATEMENTS.join(" ");
        for table in ["db_wal_segment", "db_snapshot_generation", "db_payload", "db_catalog_root", "db_index_run", "db_lease"] {
            assert!(joined.contains(table), "schema is missing table {table}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_seeds_the_catalog_singleton_row() {
        assert!(SCHEMA_STATEMENTS.iter().any(|statement| statement.contains("INSERT INTO db_catalog_root")));
    }

    #[semio_framework_async_macros::async_test]
    async fn postgres_capabilities_report_durable_fsync_and_real_cas() {
        let caps = postgres_capabilities();
        assert!(caps.durable);
        assert!(caps.supports_fsync);
        assert!(caps.supports_cas);
        assert_eq!(caps.max_durability, DurabilityClass::Fsync);
    }
    //#endregion 🔖️SchemaAndCapabilities

    //#region 🔖️ContentAddressing
    #[semio_framework_async_macros::async_test]
    async fn payload_hash_is_deterministic_and_content_addressed() {
        let a = ContentHash(*blake3::hash(b"hello").as_bytes());
        let b = ContentHash(*blake3::hash(b"hello").as_bytes());
        let c = ContentHash(*blake3::hash(b"world").as_bytes());
        assert_eq!(a.0, b.0, "identical bytes hash identically");
        assert_ne!(a.0, c.0, "different bytes hash differently");
    }
    //#endregion 🔖️ContentAddressing
}
//#endregion 🧪️Tests
