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
use crate::db_storage::{CatalogStorage, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
use pack::{ByteRange, ContentHash};
use sqlx::postgres::{PgPool, PgPoolOptions};

/// @emoji 🛡️ Mirrors `db_storage`'s own read-size ceiling (its `MAX_READ_BYTES` is private to that
/// crate) — validated via `check_len` before a `Vec<u8>` sized by an untrusted on-disk
/// length is allocated, per the repo's "validate before allocating" invariant.
const MAX_READ_BYTES: u64 = 1024 * 1024 * 1024;

/// @emoji 🐘️ A `db_storage::DbStorage` backend over PostgreSQL — `pool` is the connection pool
/// every trait method below runs its query against directly (no runtime of its own to bridge
/// through anymore — see module doc).
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    /// @emoji 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration
    /// framework — matches the deleted `os-semio_hub-storage-postgres` precedent), returning a ready
    /// `PostgresStorage`. `async` because connecting a pool and running DDL are themselves I/O — the
    /// caller (ultimately the hub's `#[tokio::main]`) already awaits this on a real runtime.
    pub async fn connect(database_url: &str) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new().max_connections(16).connect(database_url).await.map_err(map_sqlx_error)?;
        bootstrap_schema(&pool).await?;
        Ok(Self { pool })
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
impl WalStorage for PostgresStorage {
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

    async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let row: Option<(bool,)> = sqlx::query_as("SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(doc).bind(idx).fetch_optional(&mut *tx).await.map_err(map_sqlx_error)?;
        let sealed = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0;
        if sealed {
            return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
        }
        let (new_len,): (i64,) =
            sqlx::query_as("UPDATE db_wal_segment SET bytes = bytes || $1 WHERE document_id = $2 AND segment_index = $3 RETURNING octet_length(bytes)").bind(bytes).bind(doc).bind(idx).fetch_one(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(new_len as u64)
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

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(doc).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current_len = len_row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0 as u64;
        let (offset, len) = validate_read_range(current_len, range)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1 FOR $2) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(doc).bind(idx).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(bytes)
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let rows: Vec<(i64,)> = sqlx::query_as("SELECT segment_index FROM db_wal_segment WHERE document_id = $1 ORDER BY segment_index ASC").bind(document.0.as_str()).fetch_all(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(rows.into_iter().map(|(index,)| index as u64).collect())
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
impl SnapshotStorage for PostgresStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: &[u8]) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        sqlx::query(
            "INSERT INTO db_snapshot_generation (document_id, generation, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, generation) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(gen)
        .bind(bytes)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError> {
        let gen = to_i64(generation)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(bytes)
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row: (Option<i64>,) = sqlx::query_as("SELECT MAX(generation) FROM db_snapshot_generation WHERE document_id = $1").bind(document.0.as_str()).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0.map(|generation| generation as u64))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let rows: Vec<(i64,)> = sqlx::query_as("SELECT generation FROM db_snapshot_generation WHERE document_id = $1 ORDER BY generation ASC").bind(document.0.as_str()).fetch_all(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(rows.into_iter().map(|(generation,)| generation as u64).collect())
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        sqlx::query("DELETE FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(document.0.as_str()).bind(gen).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
impl PayloadStorage for PostgresStorage {
    async fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
        let hash = ContentHash(*blake3::hash(bytes).as_bytes());
        sqlx::query("INSERT INTO db_payload (hash, bytes) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING").bind(&hash.0[..]).bind(bytes).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "payload_storage::get")?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(bytes)
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
impl CatalogStorage for PostgresStorage {
    async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
        let (epoch, bytes): (i64, Option<Vec<u8>>) = sqlx::query_as("SELECT epoch, bytes FROM db_catalog_root WHERE id = 1").fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(bytes.map(|bytes| (bytes, EpochFence { epoch: epoch as u64 })))
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        // 🎯️ The bootstrap-seeded singleton row (`id = 1`) always exists, so `SELECT ... FOR
        // UPDATE` here is a real, always-present row lock — unlike `FsStorage::cas_root`'s
        // documented in-process-only mutex, this fences concurrent writers across connections
        // and processes for free via Postgres's own lock manager.
        let (current_epoch,): (i64,) = sqlx::query_as("SELECT epoch FROM db_catalog_root WHERE id = 1 FOR UPDATE").fetch_one(&mut *tx).await.map_err(map_sqlx_error)?;
        let current_fence = EpochFence { epoch: current_epoch as u64 };
        expected.check(current_fence)?;
        let new_fence = expected.next();
        sqlx::query("UPDATE db_catalog_root SET epoch = $1, bytes = $2 WHERE id = 1").bind(to_i64(new_fence.epoch)?).bind(new_bytes).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(new_fence)
    }
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
impl IndexStorage for PostgresStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: &[u8]) -> Result<(), DbError> {
        let run = to_i64(run_id)?;
        sqlx::query(
            "INSERT INTO db_index_run (document_id, run_id, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, run_id) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(run)
        .bind(bytes)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError> {
        let run = to_i64(run_id)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes) FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?.0;
        check_len(len as u64, MAX_READ_BYTES, "index_storage::read_run")?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(bytes)
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let rows: Vec<(i64,)> = sqlx::query_as("SELECT run_id FROM db_index_run WHERE document_id = $1 ORDER BY run_id ASC").bind(document.0.as_str()).fetch_all(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(rows.into_iter().map(|(run_id,)| run_id as u64).collect())
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
    holder: String,
    fence: EpochFence,
    expires_at_ms: u64,
}

/// @emoji 🤝️ Pure decision for `LeaseStorage::acquire` — identical state machine to
/// `db_storage::{MemoryStorage, FsStorage}::acquire`: re-acquire by the same still-live holder keeps
/// the fence, a genuine hand-off (absent or expired) bumps it, a live foreign holder conflicts.
fn lease_acquire_decision(existing: Option<&ExistingLease>, holder: &str, now_ms: u64) -> Result<EpochFence, DbError> {
    match existing {
        Some(info) if now_ms < info.expires_at_ms => {
            if info.holder != holder {
                return Err(DbError::Conflict(format!("resource is leased by another holder ({})", info.holder)));
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
    if info.holder != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// @emoji 🕊️ Pure decision for `LeaseStorage::release` — same holder/fence checks as `renew`, minus
/// the expiry check (a holder may release its own already-expired-but-not-yet-reclaimed lease).
fn lease_release_check(existing: Option<&ExistingLease>, holder: &str, fence: EpochFence) -> Result<(), DbError> {
    let info = existing.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
    if info.holder != holder {
        return Err(DbError::Unauthorized(format!("lease is not held by {holder}")));
    }
    fence.check(info.fence)
}

/// @emoji 🔒️ Reads `resource`'s current lease row through `executor`, taking a `FOR UPDATE` row
/// lock when `executor` is a transaction (the row-lock variant every `acquire`/`renew`/`release`
/// call uses so the read-decide-write sequence is atomic across concurrent connections) —
/// `LeaseStorage::current` instead calls this with the bare pool for a non-locking snapshot read.
async fn read_existing_lease_for_update<'e, E>(executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1 FOR UPDATE").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    Ok(row.map(|(holder, epoch, expires_at_ms)| ExistingLease { holder, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

async fn read_existing_lease<'e, E>(executor: E, resource: &str) -> Result<Option<ExistingLease>, DbError>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let row: Option<(String, i64, i64)> = sqlx::query_as("SELECT holder, epoch, expires_at_ms FROM db_lease WHERE resource = $1").bind(resource).fetch_optional(executor).await.map_err(map_sqlx_error)?;
    Ok(row.map(|(holder, epoch, expires_at_ms)| ExistingLease { holder, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
}

impl LeaseStorage for PostgresStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(&mut *tx, resource).await?;
        let fence = lease_acquire_decision(existing.as_ref(), holder, now_ms)?;
        let expires_at = to_i64(now_ms.saturating_add(ttl_ms))?;
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
        let existing = read_existing_lease_for_update(&mut *tx, resource).await?;
        lease_renew_check(existing.as_ref(), holder, fence, now_ms)?;
        sqlx::query("UPDATE db_lease SET expires_at_ms = $1 WHERE resource = $2").bind(to_i64(now_ms.saturating_add(ttl_ms))?).bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let existing = read_existing_lease_for_update(&mut *tx, resource).await?;
        lease_release_check(existing.as_ref(), holder, fence)?;
        sqlx::query("DELETE FROM db_lease WHERE resource = $1").bind(resource).execute(&mut *tx).await.map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let existing = read_existing_lease(&self.pool, resource).await?;
        Ok(existing.and_then(|info| if now_ms < info.expires_at_ms { Some(LeaseInfo { resource: resource.to_string(), holder: info.holder, fence: info.fence, expires_at_ms: info.expires_at_ms }) } else { None }))
    }
}
//#endregion 🔖️LeaseStorage

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
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL.next(), expires_at_ms: 1_000 };
        assert_eq!(lease_acquire_decision(Some(&existing), "alice", 500).unwrap(), existing.fence);
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_by_other_holder_before_expiry_conflicts() {
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(matches!(lease_acquire_decision(Some(&existing), "bob", 500), Err(DbError::Conflict(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_acquire_after_expiry_bumps_fence_for_new_holder() {
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert_eq!(lease_acquire_decision(Some(&existing), "bob", 2_000).unwrap(), EpochFence::INITIAL.next());
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_renew_rejects_absent_expired_wrong_holder_and_wrong_fence() {
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(matches!(lease_renew_check(None, "alice", EpochFence::INITIAL, 0), Err(DbError::NotFound(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 2_000), Err(DbError::Unavailable(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "bob", EpochFence::INITIAL, 500), Err(DbError::Unauthorized(_))));
        assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL.next(), 500), Err(DbError::Fenced { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_renew_accepts_matching_live_holder_and_fence() {
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
        assert!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 500).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn lease_release_rejects_wrong_holder_or_fence() {
        let existing = ExistingLease { holder: "alice".to_string(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
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
