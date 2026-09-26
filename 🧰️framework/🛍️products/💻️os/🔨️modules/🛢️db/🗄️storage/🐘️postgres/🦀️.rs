//! 🗄️ `db_storage_postgres` — a `db_storage::DbStorage` backend over PostgreSQL (via `sqlx`),
//! informed by the deleted `os-semio_hub-storage-postgres` crate's connection/schema-bootstrap shape but
//! implementing the generic, semio_hub-agnostic `db_storage` trait family rather than semio_hub-specific
//! tables. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! every `db_storage` sub-trait method is a plain `async fn` — `sqlx`'s Postgres driver
//! is async-only (Postgres has no blocking C client to wrap, unlike sqlite), and this backend used
//! to bridge that onto the family's then-synchronous trait signatures by owning a dedicated
//! multi-thread `tokio::runtime::Runtime` and `block_on`-ing every call. The `block_on` bridge is
//! GONE and every method body here is the SAME already-async `sqlx` code, handed straight back as
//! `Box::pin(async move { .. })`, so this backend parks no thread per call.
//!
//! 🧵 **Who polls it.** Not the caller's own executor — that claim was wrong and fatal. A
//! `PostgresStorage` call becomes a `DbIoTask` submitted to the process `WorkerPool`, and an
//! async-native backend's future is driven from `Lane::Io`, whose workers are plain `std::thread`s
//! with no `tokio` context at all. `sqlx` needs that context at **poll** time, so the hub aborted
//! with `this functionality requires a Tokio context` on the first Postgres document call (ticket
//! `26/09/18`, D4). This backend therefore names the bounded runtime `db_storage_driver_runtime`
//! owns through [`DbIoTaskExecutor::driver_runtime`], and `Lane::Io` only ever polls the repo-owned
//! rendezvous that runtime hands back. `sqlx`'s `runtime-tokio` feature selects ITS internal
//! executor binding at ITS compile time; the runtime this backend is polled on is the one named
//! here, and there is exactly one per process.
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
    close_db_io_backend, db_io_close_platform, db_io_copy_observed_text, db_io_hash_pages, db_io_prepare_platform, db_io_transfer_list, db_io_write_observed_bytes, register_db_io_backend, register_db_io_backend_prepared_with_use,
    retire_db_io_backend, submit_db_io_task, CatalogStorage, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoBackendRollbackReservation, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoLeaseResult,
    DbIoAsyncDriverRuntime, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbStorageOpenRejected, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities,
    WalSegmentState, WalStorage, DB_IO_LIST_ITEMS, DB_IO_PAGE_BYTES,
};

macro_rules! with_admitted_artifact {
    ($operation:expr, $document:expr, $artifact:ident, $call:expr) => {{
        let mut owner = DbIoArtifactId::try_from_text($operation, $document)?;
        let $artifact = owner.as_artifact()?;
        let terminal = $call.await;
        while owner.close_step()? {
            semio_framework_async::yield_once().await;
        }
        terminal
    }};
}
use crate::db_storage::writer::{release, WalWriterGuard, WalWriterTable};
use crate::db_storage::DbIoWriterReleaseStep;
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;

use crate::db_storage::DB_IO_MAX_READ_BYTES;
const POSTGRES_WAL_STATE_QUERY: &str = "SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2";

fn postgres_wal_segment_state(sealed: bool) -> WalSegmentState {
    if sealed {
        WalSegmentState::Sealed
    } else {
        WalSegmentState::Active
    }
}

/// @emoji 🐘️ A `db_storage::DbStorage` backend over PostgreSQL — `pool` is the connection pool
/// every trait method below runs its query against directly (no runtime of its own to bridge
/// through anymore — see module doc).
struct PostgresDbIoExecutor {
    pool: PgPool,
    database_url: DbIoText,
    writers: std::sync::Mutex<Option<Box<WalWriterTable<PostgresWalWriterGuard>>>>,
    backend_terminal: std::sync::atomic::AtomicBool,
    active_operation: u64,
    close_future: std::sync::Mutex<Option<semio_framework_async::oneshot::Receiver<()>>>,
}

impl PostgresDbIoExecutor {
    /// @emoji 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration
    /// framework — matches the deleted `os-semio_hub-storage-postgres` precedent), returning a ready
    /// `PostgresStorage`. `async` because connecting a pool and running DDL are themselves I/O — the
    /// caller (ultimately the hub's `#[tokio::main]`) already awaits this on a real runtime.
    fn new(database_url: DbIoText) -> Result<Self, DbError> {
        let pool = crate::db_storage_driver_runtime::within(|| PgPoolOptions::new().max_connections(16).connect_lazy(database_url.as_str())).map_err(map_sqlx_error)?;
        Ok(Self {
            pool,
            database_url,
            writers: std::sync::Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
            backend_terminal: std::sync::atomic::AtomicBool::new(false),
            active_operation: 0,
            close_future: std::sync::Mutex::new(None),
        })
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

/// @emoji 🆕️ Like `map_session_error`, but a unique-violation becomes `DbError::AlreadyExists(what())`
/// — used by the writer session's `create_segment`.
// 🚫️async: E1 pure accessor called from sync `.map_err(|err| map_create_error(...))` closures — see R9
fn map_create_error(err: sqlx::Error, what: impl FnOnce() -> String) -> DbError {
    if let Some(db_err) = err.as_database_error() {
        if db_err.is_unique_violation() {
            return DbError::AlreadyExists(what());
        }
    }
    map_session_error(err)
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

/// @emoji 📋️ One document's ascending id column in ONE round trip, bounded by the DB I/O list capacity (`sql` binds
/// the document as `$1` and the row bound as `$2`; a list one row over the capacity is refused exactly as pushing it
/// would be). A list used to cost one round trip per row (`… > $2 ORDER BY … LIMIT 1` in a loop): the index lists its
/// runs several times per edit, so every edit on a remote server paid hundreds of round trips and outgrew the hub's
/// 30 s frame deadline under load (ticket 26/09/23 H9 session 12, two-client e2e `tc19`–`tc22`).
async fn postgres_ascending_ids(pool: &PgPool, sql: &'static str, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
    let bound = to_i64(DB_IO_LIST_ITEMS as u64 + 1)?;
    let rows: Vec<(i64,)> = sqlx::query_as(sql).bind(document.0.as_str()).bind(bound).fetch_all(pool).await.map_err(map_sqlx_error)?;
    let mut result = DbIoU64List::new();
    for (id,) in rows {
        result.push(u64::try_from(id).map_err(|_| DbError::InvalidArgument(format!("stored id {id} is negative")))?)?;
    }
    Ok(result)
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

//#region 🔖️WriterFence
/// @emoji 🔒️ Namespace hashed with the document into the session advisory-lock key — contract
/// `🔐️writer/🧫️fixtures/🌐️remote-guard` (`postgres.lockNamespace`).
const WAL_WRITER_LOCK_NAMESPACE: &str = "semio/db/wal-writer/v1";

/// @emoji 🏷️ `application_name` of every writer session, so an operator (or the conformance law's
/// independent `psql`) can name the exact session that holds a document.
const WAL_WRITER_APPLICATION_NAME: &str = "semio-wal-writer";

/// @emoji 💓 Server-side TCP keepalive of a writer session: a vanished host's lock is released after
/// `idle + interval × count` seconds instead of the kernel's two-hour default.
const WAL_WRITER_KEEPALIVE: [(&str, &str); 3] = [("tcp_keepalives_idle", "10"), ("tcp_keepalives_interval", "5"), ("tcp_keepalives_count", "3")];

/// @emoji 🔑 The 64-bit advisory-lock key of one document: the first eight bytes (big-endian) of
/// `sha256(namespace ‖ 0x00 ‖ document)`.
fn wal_writer_lock_key(document: &str) -> i64 {
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(WAL_WRITER_LOCK_NAMESPACE.as_bytes());
    hash.update(&[0]);
    hash.update(document.as_bytes());
    let digest = hash.finalize();
    i64::from_be_bytes(digest[..8].try_into().expect("sha256 digest has eight leading bytes"))
}

/// @emoji 🧵 A dedicated connection whose session holds the document's advisory lock; every WAL
/// mutation of the permit runs on this session, so a terminated session can never write.
struct PostgresWalWriterSession {
    connection: PgConnection,
    key: i64,
}

impl PostgresWalWriterSession {
    async fn open(pool: &PgPool, document: &DbIoText) -> Result<Self, DbError> {
        let options = (*pool.connect_options()).clone().application_name(WAL_WRITER_APPLICATION_NAME).options(WAL_WRITER_KEEPALIVE);
        let mut connection = options.connect().await.map_err(map_sqlx_error)?;
        let key = wal_writer_lock_key(document.as_str());
        let locked = sqlx::query_as::<_, (bool,)>("SELECT pg_try_advisory_lock($1)").bind(key).fetch_one(&mut connection).await;
        match locked {
            Ok((true,)) => Ok(Self { connection, key }),
            Ok((false,)) => {
                let _ = connection.close().await;
                Err(DbError::Conflict("WAL document already has a PostgreSQL writer session".to_string()))
            }
            Err(error) => Err(map_sqlx_error(error)),
        }
    }
}

/// @emoji 🚨️ Classifies an error raised on the writer session: transport loss and the server's
/// operator-intervention / connection-exception classes (`57P*`, `08*`) end the session, and with it
/// the advisory lock, so they answer `Fenced` — the caller then drops the session instead of lending it again.
fn map_session_error(err: sqlx::Error) -> DbError {
    let ended = match err.as_database_error() {
        Some(db_err) => db_err.code().is_some_and(|code| code.starts_with("57P") || code.starts_with("08")),
        None => matches!(err, sqlx::Error::Io(_) | sqlx::Error::Tls(_) | sqlx::Error::Protocol(_) | sqlx::Error::WorkerCrashed | sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut),
    };
    if ended {
        return DbError::Fenced { expected: 0, actual: 0 };
    }
    map_sqlx_error(err)
}

/// @emoji 🔔 Completion witness of a detached unlock; wakes the release controller and a parked backend close.
struct PostgresWalWriterUnlock {
    done: std::sync::atomic::AtomicBool,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
}

/// @emoji 🔐️ One document's cross-process writer fence: the lock session, lent to one pinned
/// operation at a time, then an in-flight unlock, then terminal.
enum PostgresWalWriterGuard {
    Held { session: Option<PostgresWalWriterSession>, backend: DbIoBackendControl },
    Unlocking(Arc<PostgresWalWriterUnlock>),
    Terminal,
}

impl PostgresWalWriterGuard {
    fn lend(&mut self, generation: u64) -> Result<PostgresWalWriterSession, DbError> {
        match self {
            Self::Held { session, .. } => session.take().ok_or(DbError::Fenced { expected: 0, actual: generation }),
            _ => Err(DbError::Closed),
        }
    }

    fn restore(&mut self, lent: PostgresWalWriterSession) {
        if let Self::Held { session, .. } = self {
            *session = Some(lent);
        }
    }
}

impl WalWriterGuard for PostgresWalWriterGuard {
    fn close_step(&mut self) -> Result<bool, DbError> {
        match self {
            Self::Held { session, backend } => {
                let backend = *backend;
                let Some(mut lent) = session.take() else {
                    *self = Self::Terminal;
                    return Ok(true);
                };
                let unlock = Arc::new(PostgresWalWriterUnlock { done: std::sync::atomic::AtomicBool::new(false), waker: std::sync::Mutex::new(None) });
                let signal = unlock.clone();
                drop(crate::db_storage_driver_runtime::detach_unit(Box::pin(async move {
                    let _ = sqlx::query("SELECT pg_advisory_unlock($1)").bind(lent.key).execute(&mut lent.connection).await;
                    let _ = lent.connection.close().await;
                    signal.done.store(true, std::sync::atomic::Ordering::Release);
                    if let Some(waker) = signal.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        waker.wake();
                    }
                    release::request_controller(backend);
                })));
                *self = Self::Unlocking(unlock);
                Ok(true)
            }
            Self::Unlocking(unlock) => {
                if !unlock.done.load(std::sync::atomic::Ordering::Acquire) {
                    return Ok(true);
                }
                *self = Self::Terminal;
                Ok(false)
            }
            Self::Terminal => Ok(false),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self, Self::Terminal)
    }

    fn awaiting_wake(&self) -> bool {
        matches!(self, Self::Unlocking(unlock) if !unlock.done.load(std::sync::atomic::Ordering::Acquire))
    }

    fn register_wake(&self, waker: &std::task::Waker) {
        if let Self::Unlocking(unlock) = self {
            *unlock.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(waker.clone());
            if unlock.done.load(std::sync::atomic::Ordering::Acquire) {
                waker.wake_by_ref();
            }
        }
    }
}
//#endregion 🔖️WriterFence

//#region 🔖️WalStorage
impl PostgresDbIoExecutor {
    async fn create_segment(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("INSERT INTO db_wal_segment (document_id, segment_index) VALUES ($1, $2)")
            .bind(document)
            .bind(idx)
            .execute(&mut *connection)
            .await
            .map_err(|err| map_create_error(err, || format!("wal segment {index} for {document} already exists")))?;
        Ok(())
    }

    async fn append(&self, connection: &mut PgConnection, document: &str, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = async {
            let idx = to_i64(index)?;
            let doc = document;
            let mut tx = connection.begin().await.map_err(map_session_error)?;
            let row: Option<(bool,)> = sqlx::query_as("SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(doc).bind(idx).fetch_optional(&mut *tx).await.map_err(map_session_error)?;
            let sealed = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0;
            if sealed {
                return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
            }
            let (new_len,): (i64,) = sqlx::query_as("UPDATE db_wal_segment SET bytes = bytes || $1 WHERE document_id = $2 AND segment_index = $3 RETURNING octet_length(bytes)::BIGINT")
                .bind(prepared.as_slice())
                .bind(doc)
                .bind(idx)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_session_error)?;
            tx.commit().await.map_err(map_session_error)?;
            Ok(new_len as u64)
        }
        .await;
        db_io_close_platform(prepared).await?;
        result
    }

    /// @emoji 💾️ Every write above already ran as a committed statement/transaction on the writer
    /// session, and Postgres fsyncs its own WAL at COMMIT under the default `synchronous_commit =
    /// on`, so `Fsync` is satisfied when `append`/`truncate_tail` return. `Quorum` (replica
    /// acknowledgement) is `db_cluster`'s concern over Postgres's own replication.
    fn sync(_class: DurabilityClass) -> Result<(), DbError> {
        Ok(())
    }

    async fn seal(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let result: Option<(bool,)> =
            sqlx::query_as("UPDATE db_wal_segment SET sealed = TRUE WHERE document_id = $1 AND segment_index = $2 RETURNING sealed").bind(document).bind(idx).fetch_optional(&mut *connection).await.map_err(map_session_error)?;
        result.map(|_| ()).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read")?;
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(doc).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current_len = len_row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0 as u64;
        let (offset, len) = validate_read_range(current_len, range)?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1::integer FOR $2::integer) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(doc).bind(idx).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(bool,)> = sqlx::query_as(POSTGRES_WAL_STATE_QUERY).bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(sealed,)| postgres_wal_segment_state(sealed)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT segment_index FROM db_wal_segment WHERE document_id = $1 ORDER BY segment_index ASC LIMIT $2", document).await
    }

    async fn truncate_tail(connection: &mut PgConnection, document: &str, index: u64, new_len: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let mut tx = connection.begin().await.map_err(map_session_error)?;
        let row: Option<(bool, i64)> =
            sqlx::query_as("SELECT sealed, octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(document).bind(idx).fetch_optional(&mut *tx).await.map_err(map_session_error)?;
        let (sealed, current_len) = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        validate_truncate(sealed, current_len as u64, new_len)?;
        sqlx::query("UPDATE db_wal_segment SET bytes = substring(bytes FROM 1 FOR $1::integer) WHERE document_id = $2 AND segment_index = $3").bind(to_i64(new_len)?).bind(document).bind(idx).execute(&mut *tx).await.map_err(map_session_error)?;
        tx.commit().await.map_err(map_session_error)?;
        Ok(())
    }

    async fn delete_segment(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("DELETE FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document).bind(idx).execute(&mut *connection).await.map_err(map_session_error)?;
        Ok(())
    }

    fn writer_table(&mut self) -> Result<&mut WalWriterTable<PostgresWalWriterGuard>, DbError> {
        self.writers.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)
    }

    /// @emoji 🔐️ Runs one pinned WAL mutation on its permit's lock session. A session the server
    /// ended is never lent again: the permit is fenced from then on and its release is immediate.
    async fn fenced_wal_mutation(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        let (key, backend, document) = task.writer_stamp().map(|(key, backend, document)| (key, backend, document.clone())).ok_or_else(|| DbError::Internal("PostgreSQL WAL mutation lost its writer stamp".to_string()))?;
        let mut session = self.writer_table()?.pinned_guard_mut(key, backend, &document, operation)?.lend(key.generation())?;
        let connection = &mut session.connection;
        let result = match task {
            DbIoTask::WalCreate { index, .. } => Self::create_segment(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalAppend { index, input, .. } => {
                let input = input.take_for_async_driver();
                self.append(connection, document.as_str(), *index, input).await.map(DbIoResult::Length)
            }
            DbIoTask::WalSync { class, .. } => Self::sync(*class).map(|()| DbIoResult::Unit),
            DbIoTask::WalSeal { index, .. } => Self::seal(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalTruncate { index, new_len, .. } => Self::truncate_tail(connection, document.as_str(), *index, *new_len).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalDelete { index, .. } => Self::delete_segment(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            _ => Err(DbError::Internal("PostgreSQL fenced WAL mutation taxonomy".to_string())),
        };
        if let Err(DbError::Fenced { .. }) = result {
            drop(session);
            return Err(DbError::Fenced { expected: 0, actual: key.generation() });
        }
        self.writer_table()?.pinned_guard_mut(key, backend, &document, operation)?.restore(session);
        result
    }
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
impl SnapshotStorage for PostgresDbIoExecutor {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

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
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row: (Option<i64>,) = sqlx::query_as("SELECT MAX(generation) FROM db_snapshot_generation WHERE document_id = $1").bind(document.0.as_str()).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0.map(|generation| generation as u64))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT generation FROM db_snapshot_generation WHERE document_id = $1 ORDER BY generation ASC LIMIT $2", document).await
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
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "payload_storage::put")?;
        let hash = db_io_hash_pages(&bytes).await;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query("INSERT INTO db_payload (hash, bytes) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING").bind(&hash.0[..]).bind(prepared.as_slice()).execute(&self.pool).await.map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "payload_storage::get")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
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
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
impl CatalogStorage for PostgresDbIoExecutor {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
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
        check_len(new_bytes.len() as u64, DB_IO_MAX_READ_BYTES, "catalog_storage::cas_root")?;
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
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "index_storage::read_run")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT run_id FROM db_index_run WHERE document_id = $1 ORDER BY run_id ASC LIMIT $2", document).await
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
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder).await?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
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
    Ok(Some(ExistingLease { holder: db_io_copy_observed_text(reservation, holder).await?, fence: EpochFence { epoch: epoch as u64 }, expires_at_ms: expires_at_ms as u64 }))
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
        existing.filter(|info| now_ms < info.expires_at_ms).map(|info| Ok(DbIoLeaseResult::new(DbIoText::try_from_str(resource)?, info.holder, info.fence, info.expires_at_ms))).transpose()
    }
}
//#endregion 🔖️LeaseStorage

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
        Some(crate::db_storage_driver_runtime::shared())
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
            *close = Some(crate::db_storage_driver_runtime::detach_unit(Box::pin(async move { pool.close().await })));
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

/// @emoji 🐘️ Typed PostgreSQL facade; only the registered executor owns the external driver.
pub struct PostgresStorage {
    control: DbIoBackendControl,
    worker_pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

impl PostgresStorage {
    pub async fn connect(worker_pool: Arc<WorkerPool>, database_url: &str) -> Result<Self, DbStorageOpenRejected> {
        let database_url = DbIoText::try_from_str(database_url)?;
        let rollback = DbIoBackendRollbackReservation::try_reserve()?;
        let pool_use = worker_pool.acquire_use().map_err(|error| DbError::Unavailable(format!("PostgreSQL DB I/O backend WorkerPool use rejected: {error:?}")))?;
        let executor = Box::new(PostgresDbIoExecutor::new(database_url.clone())?);
        let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Postgres, executor, worker_pool.clone(), pool_use, rollback)?;
        let storage = Self { control, worker_pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = storage.execute(DbIoTask::BackendOpen { backend: control, path: database_url }).await {
            return Err(DbStorageOpenRejected::registered(error, control));
        }
        Ok(storage)
    }

    async fn execute(&self, task: DbIoTask) -> Result<DbIoResult, DbError> {
        let mut operation = submit_db_io_task(task).map_err(|(error, _)| error)?;
        operation.start_async_native_on_lane_io().await?;
        operation.finish().await
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
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<crate::db_storage::WalWriterPermit, DbError> {
        match self.execute(DbIoTask::WalWriterAcquire { backend: self.control, document: postgres_document(document)? }).await? {
            DbIoResult::WalWriter(writer) => Ok(writer),
            _ => Err(DbError::Internal("remote WAL writer result taxonomy".to_string())),
        }
    }
    async fn create_segment(&self, writer: &crate::db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
    async fn append(&self, writer: &crate::db_storage::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-length result".to_string())),
        }
    }
    async fn sync(&self, writer: &crate::db_storage::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
    }
    async fn seal(&self, writer: &crate::db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
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
    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match self.execute(DbIoTask::WalState { backend: self.control, document: postgres_document(document)?, index }).await? {
            DbIoResult::WalSegmentState(state) => Ok(state),
            _ => Err(DbError::Internal("PostgreSQL executor returned a non-WAL-state result".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_list(self.execute(DbIoTask::WalList { backend: self.control, document: postgres_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn truncate_tail(&self, writer: &crate::db_storage::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await?)
    }
    async fn delete_segment(&self, writer: &crate::db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
    }
}

impl SnapshotStorage for PostgresStorage {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        postgres_unit(self.execute(DbIoTask::SnapshotWrite { backend: self.control, document: postgres_document(document)?, generation, input: bytes }).await?)
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        postgres_pages(self.execute(DbIoTask::SnapshotRead { backend: self.control, document: postgres_document(document)?, generation, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
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
        postgres_pages(self.execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
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
        match self.execute(DbIoTask::CatalogRead { backend: self.control, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await? {
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
        postgres_pages(self.execute(DbIoTask::IndexRead { backend: self.control, document: postgres_document(document)?, run_id, output: postgres_output(DB_IO_MAX_READ_BYTES)? }).await?)
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
            DbIoResult::OptionalLease(lease) => Ok(lease),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
