//! 🗃️ Postgres schema and connection: the schema statements and bootstrap lock, the schema bootstrap, the wal segment state query and the executor with its connection operations. See [`super`].

use super::*;

//#region 🔖️Schema
/// 🧱️ Idempotent DDL bootstrapped by `PostgresStorage::connect` — one statement per table,
/// plus a seed row for the catalog singleton (`db_catalog_root.id = 1`) so `CatalogStorage`'s
/// compare-and-swap can always `SELECT ... FOR UPDATE` a real row instead of racing to insert one.
pub(super) const SCHEMA_STATEMENTS: &[&str] = &[
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

/// 🔒️ The transaction-scoped advisory lock (two-key form, a keyspace apart from the one-key WAL writer locks) that
/// serialises schema bootstraps on one database: PostgreSQL's `CREATE TABLE IF NOT EXISTS` is not safe against a
/// concurrent creator of the same table (`duplicate key value violates unique constraint "pg_type_typname_nsp_index"`), so
/// two storages — two hub processes, a hub and the db CLI, two laws — opening a fresh database at once refused one of
/// them. Keys: `semi` as a big-endian i32, and the schema generation.
const SCHEMA_BOOTSTRAP_LOCK: (i32, i32) = (0x7365_6d69, 1);

/// 🧱️ Runs [`SCHEMA_STATEMENTS`] in one transaction under [`SCHEMA_BOOTSTRAP_LOCK`], so concurrent openers of one
/// database bootstrap it one after the other and every later one finds the tables.
pub(super) async fn bootstrap_schema(pool: &PgPool) -> Result<(), DbError> {
    let mut transaction = pool.begin().await.map_err(map_sqlx_error)?;
    sqlx::query("SELECT pg_advisory_xact_lock($1, $2)").bind(SCHEMA_BOOTSTRAP_LOCK.0).bind(SCHEMA_BOOTSTRAP_LOCK.1).execute(&mut *transaction).await.map_err(map_sqlx_error)?;
    for statement in SCHEMA_STATEMENTS {
        sqlx::query(statement).execute(&mut *transaction).await.map_err(map_sqlx_error)?;
    }
    transaction.commit().await.map_err(map_sqlx_error)
}
//#endregion 🔖️Schema

//#region 🔖️Connection
pub(super) const POSTGRES_WAL_STATE_QUERY: &str = "SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2";

pub(super) fn postgres_wal_segment_state(sealed: bool) -> WalSegmentState {
    if sealed {
        WalSegmentState::Sealed
    } else {
        WalSegmentState::Active
    }
}

/// 🐘️ A `db_storage::DbStorage` backend over PostgreSQL — `pool` is the connection pool
/// every trait method below runs its query against directly (no runtime of its own to bridge
/// through anymore — see module doc).
pub(super) struct PostgresDbIoExecutor {
    pub(super) pool: PgPool,
    pub(super) database_url: DbIoText,
    pub(super) writers: std::sync::Mutex<Option<Box<WalWriterTable<PostgresWalWriterGuard>>>>,
    pub(super) backend_terminal: std::sync::atomic::AtomicBool,
    pub(super) active_operation: u64,
    pub(super) close_future: std::sync::Mutex<Option<semio_framework_async::oneshot::Receiver<()>>>,
}

impl PostgresDbIoExecutor {
    /// 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration
    /// framework — matches the deleted `os-semio_hub-storage-postgres` precedent), returning a ready
    /// `PostgresStorage`. `async` because connecting a pool and running DDL are themselves I/O — the
    /// caller (ultimately the hub's `#[tokio::main]`) already awaits this on a real runtime.
    pub(super) fn new(database_url: DbIoText) -> Result<Self, DbError> {
        let pool = db_storage_driver_runtime::within(|| PgPoolOptions::new().max_connections(16).connect_lazy(database_url.as_str())).map_err(map_sqlx_error)?;
        Ok(Self {
            pool,
            database_url,
            writers: std::sync::Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
            backend_terminal: std::sync::atomic::AtomicBool::new(false),
            active_operation: 0,
            close_future: std::sync::Mutex::new(None),
        })
    }

    pub(super) fn reserve_driver_output(&self, maximum_capacity: u64) -> Result<DbIoDriverReservation, DbError> {
        let maximum_capacity = usize::try_from(maximum_capacity).map_err(|_| DbError::LimitExceeded("PostgreSQL external driver capacity"))?;
        DbIoDriverReservation::try_reserve(self.active_operation, maximum_capacity)
    }
}
//#endregion 🔖️Connection
