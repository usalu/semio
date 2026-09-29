//! 🗄️ The proctor's four storage roles over one SQLite file.
//!
//! **One file, one connection.** `<PROCTOR_DATA>/proctor.sqlite` holds the authority (receipts,
//! per-actor event streams, snapshots, the transactional outbox, leases), the rebuildable read
//! models, content-addressed blobs and sessions. All four stores are views over one shared
//! [`Database`] handle, opened in WAL mode with a busy timeout, synchronous commits and a schema
//! format row that refuses a file written by a different format. Like every embedded profile, the
//! directory belongs to one serving process; a second handle (a restart law, an inspection) is a
//! reader or a short-lived writer that SQLite's WAL locking serializes.
//!
//! **Facts are appended, never rewritten.** Events, receipts, outbox rows and outbox deliveries are
//! insert-only tables; a delivered outbox row is a second fact, not an updated flag. Snapshots and
//! leases are replaceable accelerators, projections are derived and rebuildable, and sessions are
//! deliberately not event-sourced (the storage contract requires a revoked session to vanish).
//!
//! **Synchronous driver, async ports.** `rusqlite` (bundled) is the workspace's only SQLite binding
//! (a workspace may link one native `sqlite3`); every port method is `async` as the framework
//! requires, while its body is a short synchronous statement under a mutex that is never held
//! across an `.await`.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs — the four contracts
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs — the laws they are held to
//! @see <https://www.sqlite.org/wal.html>

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use protocol::codec::ids::ContentHash;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use server::contract::{ActorKey, CommandId, CommandReceipt, EventRecord, HybridLogicalClock, IdempotencyKey, Principal, Revision, SessionId, TenantId};
use server::policy::principal_key;
use server::storage::{AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError};

//#region 🔖️Database
/// 📄️ The file name of the proctor's database inside `PROCTOR_DATA`.
pub const DATABASE_FILE: &str = "proctor.sqlite";

/// 🏷️ The format this binary writes and accepts.
pub const FORMAT_SCHEMA: &str = "semio.teaching.proctor.sqlite";

/// 🔢️ The format version this binary writes and accepts.
pub const FORMAT_VERSION: i64 = 1;

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS proctor_format (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), schema TEXT NOT NULL, version INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_receipt (key TEXT PRIMARY KEY, command_id TEXT NOT NULL, tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, revision INTEGER NOT NULL, millis INTEGER NOT NULL, counter INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_event (position INTEGER PRIMARY KEY AUTOINCREMENT, tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, seq INTEGER NOT NULL, millis INTEGER NOT NULL, counter INTEGER NOT NULL, event_kind TEXT NOT NULL, payload BLOB NOT NULL, UNIQUE (tenant, kind, id, seq));
CREATE TABLE IF NOT EXISTS proctor_snapshot (tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, revision INTEGER NOT NULL, bytes BLOB NOT NULL, PRIMARY KEY (tenant, kind, id));
CREATE TABLE IF NOT EXISTS proctor_outbox (id INTEGER PRIMARY KEY AUTOINCREMENT, tenant TEXT NOT NULL, kind TEXT NOT NULL, actor TEXT NOT NULL, entry_kind TEXT NOT NULL, payload BLOB NOT NULL, event_tenant TEXT, event_actor_kind TEXT, event_actor TEXT, event_seq INTEGER, event_millis INTEGER, event_counter INTEGER, event_kind TEXT, event_payload BLOB);
CREATE TABLE IF NOT EXISTS proctor_outbox_delivery (id INTEGER PRIMARY KEY REFERENCES proctor_outbox (id));
CREATE TABLE IF NOT EXISTS proctor_lease (tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, epoch INTEGER NOT NULL, holder TEXT NOT NULL, PRIMARY KEY (tenant, kind, id));
CREATE TABLE IF NOT EXISTS proctor_projection (projection TEXT NOT NULL, key TEXT NOT NULL, value BLOB NOT NULL, PRIMARY KEY (projection, key)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS proctor_checkpoint (projection TEXT PRIMARY KEY, seq INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_blob (hash BLOB PRIMARY KEY, bytes BLOB NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_session (id TEXT PRIMARY KEY, principal TEXT NOT NULL, record BLOB NOT NULL);
CREATE INDEX IF NOT EXISTS proctor_session_principal ON proctor_session (principal);
";

/// 🔌️ One SQLite database shared by the four stores. Cheap to clone: a handle, not a connection.
#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
    location: Option<PathBuf>,
}

impl Database {
    /// 📂️ Open (creating if needed) `directory/proctor.sqlite` read-write in WAL mode, create the
    /// schema and stamp or verify the format row.
    pub fn open(directory: &Path) -> Result<Self, StorageError> {
        std::fs::create_dir_all(directory).map_err(|error| StorageError::Backend(format!("cannot create {}: {error}", directory.display())))?;
        let path = directory.join(DATABASE_FILE);
        let connection = Connection::open(&path).map_err(backend)?;
        connection.busy_timeout(BUSY_TIMEOUT).map_err(backend)?;
        connection.pragma_update(None, "journal_mode", "WAL").map_err(backend)?;
        connection.pragma_update(None, "synchronous", "FULL").map_err(backend)?;
        connection.pragma_update(None, "foreign_keys", "ON").map_err(backend)?;
        connection.execute_batch(SCHEMA).map_err(backend)?;
        stamp_format(&connection, &path)?;
        Ok(Self { connection: Arc::new(Mutex::new(connection)), location: Some(path) })
    }

    /// 🔒️ Open an existing `directory/proctor.sqlite` without write access: every write any store
    /// attempts answers [`StorageError::Backend`], which is how a failing sink looks to the stores.
    pub fn open_read_only(directory: &Path) -> Result<Self, StorageError> {
        let path = directory.join(DATABASE_FILE);
        let connection = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX).map_err(backend)?;
        connection.busy_timeout(BUSY_TIMEOUT).map_err(backend)?;
        verify_format(&connection, &path)?;
        Ok(Self { connection: Arc::new(Mutex::new(connection)), location: Some(path) })
    }

    /// 🫧️ A private in-memory database: the same schema and statements, nothing survives the handle.
    pub fn memory() -> Result<Self, StorageError> {
        let connection = Connection::open_in_memory().map_err(backend)?;
        connection.pragma_update(None, "foreign_keys", "ON").map_err(backend)?;
        connection.execute_batch(SCHEMA).map_err(backend)?;
        stamp_format(&connection, Path::new(":memory:"))?;
        Ok(Self { connection: Arc::new(Mutex::new(connection)), location: None })
    }

    /// 📍️ The database file, or `None` for an in-memory database.
    pub fn location(&self) -> Option<&Path> {
        self.location.as_deref()
    }

    /// 🔐️ Run `work` against the connection under the handle's mutex.
    fn with<T>(&self, work: impl FnOnce(&mut Connection) -> Result<T, StorageError>) -> Result<T, StorageError> {
        let mut connection = self.connection.lock().unwrap_or_else(PoisonError::into_inner);
        work(&mut connection)
    }

    /// 🧾️ Run `work` inside one immediate transaction, committed only when it succeeds.
    fn transaction<T>(&self, work: impl FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StorageError>) -> Result<T, StorageError> {
        self.with(|connection| {
            let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(backend)?;
            let value = work(&transaction)?;
            transaction.commit().map_err(backend)?;
            Ok(value)
        })
    }
}

fn stamp_format(connection: &Connection, path: &Path) -> Result<(), StorageError> {
    let stored: Option<(String, i64)> = connection.query_row("SELECT schema, version FROM proctor_format WHERE singleton = 1", [], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(backend)?;
    match stored {
        None => connection.execute("INSERT INTO proctor_format (singleton, schema, version) VALUES (1, ?1, ?2)", params![FORMAT_SCHEMA, FORMAT_VERSION]).map(|_| ()).map_err(backend),
        Some(format) => check_format(format, path),
    }
}

fn verify_format(connection: &Connection, path: &Path) -> Result<(), StorageError> {
    let stored: (String, i64) = connection.query_row("SELECT schema, version FROM proctor_format WHERE singleton = 1", [], |row| Ok((row.get(0)?, row.get(1)?))).map_err(backend)?;
    check_format(stored, path)
}

fn check_format((schema, version): (String, i64), path: &Path) -> Result<(), StorageError> {
    if schema == FORMAT_SCHEMA && version == FORMAT_VERSION {
        return Ok(());
    }
    Err(StorageError::Backend(format!("{} holds format {schema} v{version}; this proctor reads {FORMAT_SCHEMA} v{FORMAT_VERSION}", path.display())))
}

fn backend(error: impl std::fmt::Display) -> StorageError {
    StorageError::Backend(error.to_string())
}

fn unsigned(value: i64) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn signed(value: u64) -> Result<i64, StorageError> {
    i64::try_from(value).map_err(|_| StorageError::Backend(format!("{value} exceeds the SQLite integer range")))
}

fn reported<T: Default>(read: Result<T, StorageError>) -> T {
    read.unwrap_or_else(|error| {
        eprintln!("[ERROR] proctor storage read failed: {error}");
        T::default()
    })
}
//#endregion 🔖️Database

//#region 🔖️Authority
/// 🏛️ The authoritative history: receipts, event streams, snapshots, outbox and leases.
#[derive(Clone)]
pub struct SqliteAuthorityStore {
    database: Database,
}

/// 📜️ One committed event with its position in the database-wide commit order.
#[derive(Clone, Debug, PartialEq)]
pub struct LoggedEvent {
    pub position: u64,
    pub event: EventRecord,
}

impl SqliteAuthorityStore {
    /// 🔗️ The authority role over `database`.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// 🔚️ The position of the newest committed event across every stream; `0` when empty.
    pub fn log_head(&self) -> Result<u64, StorageError> {
        self.database.with(|connection| connection.query_row("SELECT COALESCE(MAX(position), 0) FROM proctor_event", [], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend))
    }

    /// 📜️ Up to `limit` committed events after `position`, in commit order across every stream —
    /// the cursor projections are folded along.
    pub fn log_after(&self, position: u64, limit: usize) -> Result<Vec<LoggedEvent>, StorageError> {
        let after = signed(position)?;
        let limit = signed(limit as u64)?;
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT position, tenant, kind, id, seq, millis, counter, event_kind, payload FROM proctor_event WHERE position > ?1 ORDER BY position LIMIT ?2").map_err(backend)?;
            let rows = statement.query_map(params![after, limit], |row| Ok(LoggedEvent { position: unsigned(row.get(0)?), event: event_from_columns(row, 1)? })).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }
}

fn event_from_columns(row: &rusqlite::Row<'_>, from: usize) -> rusqlite::Result<EventRecord> {
    Ok(EventRecord {
        stream: ActorKey { tenant: TenantId(row.get(from)?), kind: row.get(from + 1)?, id: row.get(from + 2)? },
        seq: unsigned(row.get(from + 3)?),
        hlc: HybridLogicalClock { millis: unsigned(row.get(from + 4)?), counter: u32::try_from(row.get::<_, i64>(from + 5)?).unwrap_or(0) },
        kind: row.get(from + 6)?,
        payload: row.get(from + 7)?,
    })
}

fn insert_outbox(transaction: &rusqlite::Transaction<'_>, entry: &OutboxEntry) -> Result<(), StorageError> {
    let event = entry.event.as_ref();
    transaction
        .prepare_cached("INSERT INTO proctor_outbox (tenant, kind, actor, entry_kind, payload, event_tenant, event_actor_kind, event_actor, event_seq, event_millis, event_counter, event_kind, event_payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)")
        .map_err(backend)?
        .execute(params![
            entry.actor.tenant.0,
            entry.actor.kind,
            entry.actor.id,
            entry.kind,
            entry.payload,
            event.map(|event| event.stream.tenant.0.clone()),
            event.map(|event| event.stream.kind.clone()),
            event.map(|event| event.stream.id.clone()),
            event.map(|event| signed(event.seq)).transpose()?,
            event.map(|event| signed(event.hlc.millis)).transpose()?,
            event.map(|event| i64::from(event.hlc.counter)),
            event.map(|event| event.kind.clone()),
            event.map(|event| event.payload.clone()),
        ])
        .map(|_| ())
        .map_err(backend)
}

fn outbox_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<OutboxEntry> {
    let event_seq: Option<i64> = row.get(9)?;
    let event = match event_seq {
        Some(seq) => Some(EventRecord {
            stream: ActorKey { tenant: TenantId(row.get(6)?), kind: row.get(7)?, id: row.get(8)? },
            seq: unsigned(seq),
            hlc: HybridLogicalClock { millis: unsigned(row.get(10)?), counter: u32::try_from(row.get::<_, i64>(11)?).unwrap_or(0) },
            kind: row.get(12)?,
            payload: row.get(13)?,
        }),
        None => None,
    };
    Ok(OutboxEntry { id: unsigned(row.get(0)?), actor: ActorKey { tenant: TenantId(row.get(1)?), kind: row.get(2)?, id: row.get(3)? }, kind: row.get(4)?, payload: row.get(5)?, event, delivered: false })
}

fn read_receipt(connection: &Connection, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
    connection
        .query_row("SELECT command_id, tenant, kind, id, revision, millis, counter FROM proctor_receipt WHERE key = ?1", params![key.0], |row| {
            Ok(CommandReceipt {
                command_id: CommandId(row.get(0)?),
                actor: ActorKey { tenant: TenantId(row.get(1)?), kind: row.get(2)?, id: row.get(3)? },
                revision: Revision(unsigned(row.get(4)?)),
                accepted_at: HybridLogicalClock { millis: unsigned(row.get(5)?), counter: u32::try_from(row.get::<_, i64>(6)?).unwrap_or(0) },
            })
        })
        .optional()
        .map_err(backend)
}

impl AuthorityStore for SqliteAuthorityStore {
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        self.database.with(|connection| read_receipt(connection, key))
    }

    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
        self.database.transaction(|transaction| match read_receipt(transaction, key)? {
            Some(stored) if &stored == receipt => Ok(()),
            Some(stored) => Err(StorageError::Conflict(format!("idempotency key {} is already bound to command {}", key.0, stored.command_id.0))),
            None => transaction
                .execute(
                    "INSERT INTO proctor_receipt (key, command_id, tenant, kind, id, revision, millis, counter) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![key.0, receipt.command_id.0, receipt.actor.tenant.0, receipt.actor.kind, receipt.actor.id, signed(receipt.revision.0)?, signed(receipt.accepted_at.millis)?, i64::from(receipt.accepted_at.counter)],
                )
                .map(|_| ())
                .map_err(backend),
        })
    }

    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        self.database.transaction(|transaction| {
            let head: u64 = transaction.query_row("SELECT COALESCE(MAX(seq), 0) FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend)?;
            let mut expected = head + 1;
            for event in events {
                if &event.stream != actor {
                    return Err(StorageError::Conflict(format!("event at seq {} belongs to stream {}/{}", event.seq, event.stream.kind, event.stream.id)));
                }
                if event.seq != expected {
                    return Err(StorageError::SequenceGap { expected, got: event.seq });
                }
                expected += 1;
            }
            let mut insert = transaction.prepare_cached("INSERT INTO proctor_event (tenant, kind, id, seq, millis, counter, event_kind, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)").map_err(backend)?;
            for event in events {
                insert.execute(params![actor.tenant.0, actor.kind, actor.id, signed(event.seq)?, signed(event.hlc.millis)?, i64::from(event.hlc.counter), event.kind, event.payload]).map_err(backend)?;
            }
            for entry in outbox {
                insert_outbox(transaction, entry)?;
            }
            Ok(expected - 1)
        })
    }

    async fn events_since(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, StorageError> {
        let since = signed(since)?;
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT tenant, kind, id, seq, millis, counter, event_kind, payload FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND id = ?3 AND seq > ?4 ORDER BY seq").map_err(backend)?;
            let rows = statement.query_map(params![actor.tenant.0, actor.kind, actor.id, since], |row| event_from_columns(row, 0)).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    async fn last_seq(&self, actor: &ActorKey) -> Result<u64, StorageError> {
        self.database.with(|connection| connection.query_row("SELECT COALESCE(MAX(seq), 0) FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend))
    }

    async fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> Result<(), StorageError> {
        self.database.transaction(|transaction| {
            let stored: Option<i64> = transaction.query_row("SELECT revision FROM proctor_snapshot WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| row.get(0)).optional().map_err(backend)?;
            if let Some(stored) = stored.map(unsigned) {
                if revision.0 < stored {
                    return Err(StorageError::Conflict(format!("snapshot revision {} is older than stored {stored}", revision.0)));
                }
            }
            transaction.execute("INSERT OR REPLACE INTO proctor_snapshot (tenant, kind, id, revision, bytes) VALUES (?1, ?2, ?3, ?4, ?5)", params![actor.tenant.0, actor.kind, actor.id, signed(revision.0)?, bytes]).map(|_| ()).map_err(backend)
        })
    }

    async fn snapshot(&self, actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError> {
        self.database.with(|connection| connection.query_row("SELECT revision, bytes FROM proctor_snapshot WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| Ok((Revision(unsigned(row.get(0)?)), row.get(1)?))).optional().map_err(backend))
    }

    async fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> Result<(), StorageError> {
        self.database.transaction(|transaction| entries.iter().try_for_each(|entry| insert_outbox(transaction, entry)))
    }

    async fn pending_outbox(&self, limit: usize) -> Result<Vec<OutboxEntry>, StorageError> {
        let limit = signed(limit as u64)?;
        self.database.with(|connection| {
            let mut statement = connection
                .prepare_cached("SELECT o.id, o.tenant, o.kind, o.actor, o.entry_kind, o.payload, o.event_tenant, o.event_actor_kind, o.event_actor, o.event_seq, o.event_millis, o.event_counter, o.event_kind, o.event_payload FROM proctor_outbox o WHERE NOT EXISTS (SELECT 1 FROM proctor_outbox_delivery d WHERE d.id = o.id) ORDER BY o.id LIMIT ?1")
                .map_err(backend)?;
            let rows = statement.query_map(params![limit], outbox_from_row).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError> {
        self.database.transaction(|transaction| {
            for id in ids {
                let known: bool = transaction.query_row("SELECT EXISTS (SELECT 1 FROM proctor_outbox WHERE id = ?1)", params![signed(*id)?], |row| row.get(0)).map_err(backend)?;
                if !known {
                    return Err(StorageError::NotFound);
                }
            }
            let mut insert = transaction.prepare_cached("INSERT OR IGNORE INTO proctor_outbox_delivery (id) VALUES (?1)").map_err(backend)?;
            for id in ids {
                insert.execute(params![signed(*id)?]).map_err(backend)?;
            }
            Ok(())
        })
    }

    async fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> Result<Lease, StorageError> {
        self.database.transaction(|transaction| {
            let current: Option<(i64, String)> = transaction.query_row("SELECT epoch, holder FROM proctor_lease WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(backend)?;
            let lease = match current {
                Some((epoch, current)) if current == holder => Lease { epoch: unsigned(epoch), holder: current },
                Some((epoch, _)) => Lease { epoch: unsigned(epoch) + 1, holder: holder.to_owned() },
                None => Lease { epoch: 1, holder: holder.to_owned() },
            };
            transaction.execute("INSERT OR REPLACE INTO proctor_lease (tenant, kind, id, epoch, holder) VALUES (?1, ?2, ?3, ?4, ?5)", params![actor.tenant.0, actor.kind, actor.id, signed(lease.epoch)?, lease.holder]).map_err(backend)?;
            Ok(lease)
        })
    }

    async fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> bool {
        let current = self.database.with(|connection| connection.query_row("SELECT epoch, holder FROM proctor_lease WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| Ok((unsigned(row.get(0)?), row.get::<_, String>(1)?))).optional().map_err(backend));
        matches!(reported(current), Some((epoch, holder)) if epoch == lease.epoch && holder == lease.holder)
    }
}
//#endregion 🔖️Authority

//#region 🔖️Projection
/// ✍️ One projection entry written by [`SqliteProjectionStore::commit`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionWrite {
    pub projection: String,
    pub key: String,
    pub value: Vec<u8>,
}

/// 🔭️ The rebuildable read models.
#[derive(Clone)]
pub struct SqliteProjectionStore {
    database: Database,
}

impl SqliteProjectionStore {
    /// 🔗️ The projection role over `database`.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// 🧾️ Write every entry and advance `checkpoint` in one transaction, so a fold is visible
    /// entirely or not at all and its checkpoint never runs ahead of (or behind) its entries.
    pub fn commit(&mut self, writes: &[ProjectionWrite], checkpoint: (&str, u64)) -> Result<(), StorageError> {
        self.database.transaction(|transaction| {
            let mut put = transaction.prepare_cached("INSERT OR REPLACE INTO proctor_projection (projection, key, value) VALUES (?1, ?2, ?3)").map_err(backend)?;
            for write in writes {
                put.execute(params![write.projection, write.key, write.value]).map_err(backend)?;
            }
            transaction.execute("INSERT OR REPLACE INTO proctor_checkpoint (projection, seq) VALUES (?1, ?2)", params![checkpoint.0, signed(checkpoint.1)?]).map(|_| ()).map_err(backend)
        })
    }
}

impl ProjectionStore for SqliteProjectionStore {
    async fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        self.database.with(|connection| connection.execute("INSERT OR REPLACE INTO proctor_projection (projection, key, value) VALUES (?1, ?2, ?3)", params![projection, key, value]).map(|_| ()).map_err(backend))
    }

    async fn get(&self, projection: &str, key: &str) -> Option<Vec<u8>> {
        reported(self.database.with(|connection| connection.query_row("SELECT value FROM proctor_projection WHERE projection = ?1 AND key = ?2", params![projection, key], |row| row.get(0)).optional().map_err(backend)))
    }

    async fn list(&self, projection: &str, prefix: &str) -> Vec<(String, Vec<u8>)> {
        reported(self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT key, value FROM proctor_projection WHERE projection = ?1 AND key >= ?2 AND substr(key, 1, length(?2)) = ?2 ORDER BY key").map_err(backend)?;
            let rows = statement.query_map(params![projection, prefix], |row| Ok((row.get(0)?, row.get(1)?))).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        }))
    }

    async fn checkpoint(&self, projection: &str) -> u64 {
        reported(self.database.with(|connection| connection.query_row("SELECT seq FROM proctor_checkpoint WHERE projection = ?1", params![projection], |row| row.get::<_, i64>(0)).optional().map(|seq| seq.map_or(0, unsigned)).map_err(backend)))
    }

    async fn set_checkpoint(&mut self, projection: &str, seq: u64) -> Result<(), StorageError> {
        let seq = signed(seq)?;
        self.database.with(|connection| connection.execute("INSERT OR REPLACE INTO proctor_checkpoint (projection, seq) VALUES (?1, ?2)", params![projection, seq]).map(|_| ()).map_err(backend))
    }

    async fn clear(&mut self, projection: &str) -> Result<(), StorageError> {
        self.database.transaction(|transaction| {
            transaction.execute("DELETE FROM proctor_projection WHERE projection = ?1", params![projection]).map_err(backend)?;
            transaction.execute("DELETE FROM proctor_checkpoint WHERE projection = ?1", params![projection]).map(|_| ()).map_err(backend)
        })
    }
}
//#endregion 🔖️Projection

//#region 🔖️Blob
/// 🧱️ Content-addressed bytes.
#[derive(Clone)]
pub struct SqliteBlobStore {
    database: Database,
}

impl SqliteBlobStore {
    /// 🔗️ The blob role over `database`.
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl BlobStore for SqliteBlobStore {
    async fn put(&mut self, hash: ContentHash, bytes: &[u8]) -> Result<(), StorageError> {
        self.database.transaction(|transaction| {
            let stored: Option<Vec<u8>> = transaction.query_row("SELECT bytes FROM proctor_blob WHERE hash = ?1", params![hash.0.as_slice()], |row| row.get(0)).optional().map_err(backend)?;
            match stored {
                Some(existing) if existing == bytes => Ok(()),
                Some(_) => Err(StorageError::Conflict(format!("content hash {hash} already stores different bytes"))),
                None => transaction.execute("INSERT INTO proctor_blob (hash, bytes) VALUES (?1, ?2)", params![hash.0.as_slice(), bytes]).map(|_| ()).map_err(backend),
            }
        })
    }

    async fn get(&self, hash: &ContentHash) -> Option<Vec<u8>> {
        reported(self.database.with(|connection| connection.query_row("SELECT bytes FROM proctor_blob WHERE hash = ?1", params![hash.0.as_slice()], |row| row.get(0)).optional().map_err(backend)))
    }

    async fn has(&self, hash: &ContentHash) -> bool {
        reported(self.database.with(|connection| connection.query_row("SELECT EXISTS (SELECT 1 FROM proctor_blob WHERE hash = ?1)", params![hash.0.as_slice()], |row| row.get(0)).map_err(backend)))
    }
}
//#endregion 🔖️Blob

//#region 🔖️Session
/// 🎫️ Live authentication state; a removed session leaves no trace.
#[derive(Clone)]
pub struct SqliteSessionStore {
    database: Database,
}

impl SqliteSessionStore {
    /// 🔗️ The session role over `database`.
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl SessionStore for SqliteSessionStore {
    async fn create(&mut self, session: SessionRecord) -> Result<(), StorageError> {
        let record = serde_json::to_vec(&session).map_err(|error| StorageError::Backend(error.to_string()))?;
        self.database.with(|connection| connection.execute("INSERT OR REPLACE INTO proctor_session (id, principal, record) VALUES (?1, ?2, ?3)", params![session.id.0, principal_key(&session.principal), record]).map(|_| ()).map_err(backend))
    }

    async fn get(&self, id: &SessionId) -> Option<SessionRecord> {
        let record: Option<Vec<u8>> = reported(self.database.with(|connection| connection.query_row("SELECT record FROM proctor_session WHERE id = ?1", params![id.0], |row| row.get(0)).optional().map_err(backend)));
        record.and_then(|bytes| serde_json::from_slice(&bytes).ok())
    }

    async fn delete(&mut self, id: &SessionId) -> Result<(), StorageError> {
        self.database.with(|connection| connection.execute("DELETE FROM proctor_session WHERE id = ?1", params![id.0]).map(|_| ()).map_err(backend))
    }

    async fn revoke_principal(&mut self, principal: &Principal) -> Result<usize, StorageError> {
        self.database.with(|connection| connection.execute("DELETE FROM proctor_session WHERE principal = ?1", params![principal_key(principal)]).map_err(backend))
    }
}
//#endregion 🔖️Session

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
