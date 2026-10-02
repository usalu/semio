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
//! insert-only tables; a delivered outbox row is a second fact, not an updated flag. Snapshots,
//! leases and the outbox cursor (the id up to which every row is delivered) are replaceable
//! accelerators, projections are derived and rebuildable, and sessions are deliberately not
//! event-sourced (the storage contract requires a revoked session to vanish).
//!
//! **One flush per batch, and only for facts.** A batch of turns — events, outbox rows, receipts,
//! snapshots — is one transaction ([`AuthorityStore::commit`]), so under load the disk is flushed
//! once per batch of commands rather than twice per command, and a fact never lands without its
//! receipt. What the proctor makes again when it is lost — read models, their checkpoints, outbox
//! delivery marks — commits without waiting for the disk (`Database::remakeable`) and rides along
//! with the next flush of facts.
//!
//! **Two sanctioned removals.** An erasure on request ([`Database::erase`]) and a pruning of
//! registrations nobody played under ([`Database::remove`], batch by batch, then
//! [`Database::scrub`]) remove whole streams — never single facts — of a stopped proctor together
//! with their receipts, outbox rows, snapshots and leases, drop every read model and rewrite the
//! file so the bytes are gone. What is left is a log in which those actors never existed; the next
//! start folds it like any other.
//!
//! **Operator copies.** [`Database::snapshot`] copies the database while it is served and
//! [`Database::adopt`] puts a copy in place of a stopped proctor's database; both verify what they
//! hold ([`Database::inspect`]).
//!
//! **Synchronous driver, async ports.** `rusqlite` (bundled) is the workspace's only SQLite binding
//! (a workspace may link one native `sqlite3`); every port method is `async` as the framework
//! requires, while its body is a short synchronous statement under a mutex that is never held
//! across an `.await`. On the serving (multi-threaded) runtime every such statement runs with its
//! worker's other tasks handed to another thread ([`off_worker`]), so a commit waiting for the disk
//! stalls no socket and no other request.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs — the four contracts
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs — the laws they are held to
//! @see <https://www.sqlite.org/wal.html>

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use protocol::codec::ids::ContentHash;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use semio_framework_async::CancelToken;
use server::contract::{ActorKey, CommandId, CommandReceipt, EventRecord, HybridLogicalClock, IdempotencyKey, Principal, Revision, SessionId, TenantId};
use server::policy::principal_key;
use server::storage::{AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError, TurnCommit};

//#region 🔖️Database
/// 📄️ The file name of the proctor's database inside `PROCTOR_DATA`.
pub const DATABASE_FILE: &str = "proctor.sqlite";

/// 🏷️ The format this binary writes and accepts.
pub const FORMAT_SCHEMA: &str = "semio.teaching.proctor.sqlite";

/// 🔢️ The format version this binary writes and accepts. Version 2 has one stream per handle key
/// instead of a roster stream and no `learner-recalled` events; a file of version 1 is refused.
pub const FORMAT_VERSION: i64 = 2;

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

/// ⌛️ How long an operator verb waits for a database another process still holds before it refuses.
pub const VACANCY_WAIT: Duration = Duration::from_millis(250);

/// ⏲️ How often a running [`Database::snapshot`] reports its progress and looks for a cancellation.
pub const SNAPSHOT_POLL: Duration = Duration::from_millis(100);

/// 🧪️ What [`Database::inspect`] found in a whole proctor database: how many events it holds and
/// the position of the newest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inspection {
    pub events: u64,
    pub head: u64,
}

/// 🗑️ What an erasure removes for one actor: its events, receipts, outbox rows, snapshots and leases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErasedActor {
    pub actor: ActorKey,
    pub events: u64,
    pub receipts: u64,
    pub outbox: u64,
    pub snapshots: u64,
    pub leases: u64,
}

/// 🧻️ What an erasure removes: everything of every named actor, and every read model (they are
/// derived, and the next start rebuilds them from what is left).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Erasure {
    pub actors: Vec<ErasedActor>,
    pub projections: u64,
}

/// 🧺️ The rows a removal took, table by table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Removed {
    pub events: u64,
    pub receipts: u64,
    pub outbox: u64,
    pub snapshots: u64,
    pub leases: u64,
    pub projections: u64,
}

impl std::ops::AddAssign for Removed {
    fn add_assign(&mut self, more: Self) {
        *self = Self { events: self.events + more.events, receipts: self.receipts + more.receipts, outbox: self.outbox + more.outbox, snapshots: self.snapshots + more.snapshots, leases: self.leases + more.leases, projections: self.projections + more.projections };
    }
}

/// 🪜️ One stream at a glance: its id, when its first event was committed (milliseconds since the
/// epoch), how many events it holds and how many of them are of the kind asked about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamSpan {
    pub id: String,
    pub since: u64,
    pub events: u64,
    pub marked: u64,
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS proctor_format (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), schema TEXT NOT NULL, version INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_receipt (key TEXT PRIMARY KEY, command_id TEXT NOT NULL, tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, revision INTEGER NOT NULL, millis INTEGER NOT NULL, counter INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS proctor_event (position INTEGER PRIMARY KEY AUTOINCREMENT, tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, seq INTEGER NOT NULL, millis INTEGER NOT NULL, counter INTEGER NOT NULL, event_kind TEXT NOT NULL, payload BLOB NOT NULL, UNIQUE (tenant, kind, id, seq));
CREATE TABLE IF NOT EXISTS proctor_snapshot (tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, revision INTEGER NOT NULL, bytes BLOB NOT NULL, PRIMARY KEY (tenant, kind, id));
CREATE TABLE IF NOT EXISTS proctor_outbox (id INTEGER PRIMARY KEY AUTOINCREMENT, tenant TEXT NOT NULL, kind TEXT NOT NULL, actor TEXT NOT NULL, entry_kind TEXT NOT NULL, payload BLOB NOT NULL, event_tenant TEXT, event_actor_kind TEXT, event_actor TEXT, event_seq INTEGER, event_millis INTEGER, event_counter INTEGER, event_kind TEXT, event_payload BLOB);
CREATE TABLE IF NOT EXISTS proctor_outbox_delivery (id INTEGER PRIMARY KEY REFERENCES proctor_outbox (id));
CREATE TABLE IF NOT EXISTS proctor_outbox_cursor (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), position INTEGER NOT NULL);
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

    /// 📸️ Write a whole, consistent copy of `directory/proctor.sqlite` to `target` while a proctor
    /// serves the directory: one read transaction of a read-only handle (`VACUUM INTO`), so the
    /// serving process keeps committing and the copy needs no WAL beside it.
    ///
    /// `target` is never overwritten: it is reserved first (refused when it exists), the copy is
    /// written beside it, verified ([`Database::inspect`], and its event count must lie between the
    /// counts of the source before and after the copy) and only then moved onto the reservation, so
    /// the name never shows a partial backup. `progress` hears the bytes written so far and the size
    /// of the source every [`SNAPSHOT_POLL`]; once `cancel` fires the copy is interrupted, removed
    /// and `None` answered.
    pub fn snapshot(directory: &Path, target: &Path, cancel: &CancelToken, mut progress: impl FnMut(u64, u64)) -> Result<Option<Inspection>, StorageError> {
        let source = Connection::open_with_flags(directory.join(DATABASE_FILE), OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX).map_err(backend)?;
        source.busy_timeout(BUSY_TIMEOUT).map_err(backend)?;
        verify_format(&source, &directory.join(DATABASE_FILE))?;
        let events = |connection: &Connection| connection.query_row("SELECT COUNT(*) FROM proctor_event", [], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend);
        let size: u64 = source.query_row("SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()", [], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend)?;
        let before = events(&source)?;
        std::fs::OpenOptions::new().write(true).create_new(true).open(target).map_err(|error| backend(format!("cannot reserve {}: {error}; a backup is never overwritten", target.display())))?;
        let mut partial = target.as_os_str().to_owned();
        partial.push(format!(".partial-{}", std::process::id()));
        let partial = PathBuf::from(partial);
        let discard = |error: StorageError| {
            let _ = std::fs::remove_file(&partial);
            let _ = std::fs::remove_file(target);
            error
        };
        let interrupt = source.get_interrupt_handle();
        let destination = partial.to_string_lossy().into_owned();
        let copy = std::thread::spawn(move || {
            let outcome = source.execute("VACUUM INTO ?1", params![destination]).map(|_| ());
            (source, outcome)
        });
        while !copy.is_finished() {
            if cancel.is_cancelled_now() {
                interrupt.interrupt();
            }
            progress(std::fs::metadata(&partial).map_or(0, |written| written.len()), size);
            std::thread::sleep(SNAPSHOT_POLL);
        }
        let (source, copied) = copy.join().map_err(|_| discard(backend("the copying thread panicked")))?;
        if cancel.is_cancelled_now() {
            discard(backend("cancelled"));
            return Ok(None);
        }
        copied.map_err(|error| discard(backend(error)))?;
        let inspection = Self::inspect(&partial).map_err(discard)?;
        let after = events(&source).map_err(discard)?;
        if inspection.events < before || inspection.events > after {
            return Err(discard(backend(format!("the copy holds {} events, the source held {before} before and {after} after it", inspection.events))));
        }
        progress(size, size);
        std::fs::rename(&partial, target).map_err(|error| discard(backend(format!("cannot name the backup {}: {error}", target.display()))))?;
        Ok(Some(inspection))
    }

    /// 🔎️ Verify that `file` is a whole proctor database of this format (format row, SQLite's own
    /// integrity check) and answer how many events it holds and the position of the newest.
    pub fn inspect(file: &Path) -> Result<Inspection, StorageError> {
        let connection = Connection::open_with_flags(file, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX).map_err(backend)?;
        verify_format(&connection, file).map_err(|error| StorageError::Backend(format!("{} is not a proctor database: {error}", file.display())))?;
        let verdict: String = connection.query_row("PRAGMA integrity_check", [], |row| row.get(0)).map_err(backend)?;
        if verdict != "ok" {
            return Err(StorageError::Backend(format!("{} fails the SQLite integrity check: {verdict}", file.display())));
        }
        connection.query_row("SELECT COUNT(*), COALESCE(MAX(position), 0) FROM proctor_event", [], |row| Ok(Inspection { events: unsigned(row.get(0)?), head: unsigned(row.get(1)?) })).map_err(backend)
    }

    /// 🔧️ Open `directory/proctor.sqlite` for an operator's own work on a stopped proctor: refused
    /// while another process holds the file open, and for a directory without a database.
    pub fn open_offline(directory: &Path) -> Result<Self, StorageError> {
        let path = directory.join(DATABASE_FILE);
        if !path.exists() {
            return Err(backend(format!("{} does not exist", path.display())));
        }
        vacant(&path)?;
        Self::open(directory)
    }

    /// 🧮️ What erasing `actors` would remove, without removing anything.
    pub fn erasure(&self, actors: &[ActorKey]) -> Result<Erasure, StorageError> {
        self.with(|connection| erasure(connection, actors))
    }

    /// 🧨️ Remove every trace of `actors` and every read model ([`Database::remove`]), then rewrite
    /// the file so the removed bytes are no longer in it ([`Database::scrub`]); what was removed,
    /// actor by actor. For a stopped proctor only ([`Database::open_offline`]); the next start
    /// rebuilds the read models from the events that are left.
    pub fn erase(&self, actors: &[ActorKey]) -> Result<Erasure, StorageError> {
        let erased = self.erasure(actors)?;
        self.remove(actors, &[])?;
        self.scrub()?;
        Ok(erased)
    }

    /// 🧹️ Remove every trace of `actors` — events, receipts, outbox rows and their deliveries,
    /// snapshots, leases —, the receipts stored under `receipts` (what an actor that stays holds
    /// on behalf of one that goes) and every read model, in one transaction: all of it or nothing.
    /// The actors are matched as one set, so removing thousands costs a pass over each table, not
    /// one per actor. The removed bytes stay in free pages (overwritten: `secure_delete`) until
    /// [`Database::scrub`] rewrites the file.
    pub fn remove(&self, actors: &[ActorKey], receipts: &[IdempotencyKey]) -> Result<Removed, StorageError> {
        self.with(|connection| {
            connection.pragma_update(None, "secure_delete", "ON").map_err(backend)?;
            let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(backend)?;
            transaction.execute_batch("CREATE TEMP TABLE IF NOT EXISTS proctor_doomed (tenant TEXT NOT NULL, kind TEXT NOT NULL, id TEXT NOT NULL, PRIMARY KEY (tenant, kind, id)) WITHOUT ROWID; DELETE FROM temp.proctor_doomed;").map_err(backend)?;
            {
                let mut doom = transaction.prepare_cached("INSERT OR IGNORE INTO temp.proctor_doomed (tenant, kind, id) VALUES (?1, ?2, ?3)").map_err(backend)?;
                for actor in actors {
                    doom.execute(params![actor.tenant.0, actor.kind, actor.id]).map_err(backend)?;
                }
            }
            let mut relayed = 0;
            {
                let mut receipt = transaction.prepare_cached("DELETE FROM proctor_receipt WHERE key = ?1").map_err(backend)?;
                for key in receipts {
                    relayed += receipt.execute(params![key.0]).map_err(backend)? as u64;
                }
            }
            const DOOMED: &str = "IN (SELECT tenant, kind, id FROM temp.proctor_doomed)";
            let gone = |statement: String| transaction.execute(&statement, []).map(|rows| rows as u64).map_err(backend);
            gone(format!("DELETE FROM proctor_outbox_delivery WHERE id IN (SELECT id FROM proctor_outbox WHERE (tenant, kind, actor) {DOOMED})"))?;
            let removed = Removed {
                outbox: gone(format!("DELETE FROM proctor_outbox WHERE (tenant, kind, actor) {DOOMED}"))?,
                events: gone(format!("DELETE FROM proctor_event WHERE (tenant, kind, id) {DOOMED}"))?,
                receipts: relayed + gone(format!("DELETE FROM proctor_receipt WHERE (tenant, kind, id) {DOOMED}"))?,
                snapshots: gone(format!("DELETE FROM proctor_snapshot WHERE (tenant, kind, id) {DOOMED}"))?,
                leases: gone(format!("DELETE FROM proctor_lease WHERE (tenant, kind, id) {DOOMED}"))?,
                projections: gone("DELETE FROM proctor_projection".to_string())?,
            };
            gone("DELETE FROM proctor_checkpoint".to_string())?;
            gone("DELETE FROM temp.proctor_doomed".to_string())?;
            transaction.commit().map_err(backend)?;
            Ok(removed)
        })
    }

    /// 🧽️ Fold the write-ahead log into the file and rewrite the file without its free pages
    /// (`VACUUM`), so nothing that was removed is a byte of any file of the database any more.
    pub fn scrub(&self) -> Result<(), StorageError> {
        self.with(|connection| {
            connection.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).map_err(backend)?;
            connection.execute("VACUUM", []).map_err(backend)?;
            connection.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).map_err(backend)
        })
    }

    /// ♻️ Make the database file `staged` (in `directory`) the content of `directory`: refused unless
    /// it is a whole proctor database of this format and no process holds `directory/proctor.sqlite`
    /// open. The previous file and its WAL siblings are replaced. Answers the position of the newest
    /// restored event.
    pub fn adopt(directory: &Path, staged: &Path) -> Result<u64, StorageError> {
        let position = Self::inspect(staged)?.head;
        let path = directory.join(DATABASE_FILE);
        if path.exists() {
            vacant(&path)?;
        }
        for file in [&path, staged] {
            for sibling in ["-wal", "-shm"] {
                let mut name = file.as_os_str().to_owned();
                name.push(sibling);
                match std::fs::remove_file(&name) {
                    Err(error) if error.kind() != std::io::ErrorKind::NotFound => return Err(backend(format!("cannot remove {}: {error}", Path::new(&name).display()))),
                    _ => {}
                }
            }
        }
        std::fs::rename(staged, &path).map_err(|error| backend(format!("cannot replace {}: {error}", path.display())))?;
        Ok(position)
    }

    /// 📍️ The database file, or `None` for an in-memory database.
    pub fn location(&self) -> Option<&Path> {
        self.location.as_deref()
    }

    /// 🔐️ Run `work` against the connection under the handle's mutex.
    fn with<T>(&self, work: impl FnOnce(&mut Connection) -> Result<T, StorageError>) -> Result<T, StorageError> {
        off_worker(|| {
            let mut connection = self.connection.lock().unwrap_or_else(PoisonError::into_inner);
            work(&mut connection)
        })
    }

    /// 🧾️ Run `work` inside one immediate transaction, committed only when it succeeds.
    fn transaction<T>(&self, work: impl FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StorageError>) -> Result<T, StorageError> {
        self.with(|connection| transact(connection, work))
    }

    /// 🪶️ Run `work` inside one immediate transaction whose commit does not wait for the disk: for
    /// rows the proctor makes again when they are lost with the power — read models and their
    /// checkpoints (folded again from the log) and outbox delivery marks (the row is handed out
    /// again and its follow-up is deduplicated by its idempotency key). The next facts committed
    /// flush the write-ahead log, these rows included, and a crash of the process alone loses
    /// nothing; what the power takes is always a suffix of the commits, never a hole.
    ///
    /// @see <https://www.sqlite.org/pragma.html#pragma_synchronous> — `NORMAL` in WAL mode
    fn remakeable<T>(&self, work: impl FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StorageError>) -> Result<T, StorageError> {
        self.with(|connection| {
            connection.pragma_update(None, "synchronous", "NORMAL").map_err(backend)?;
            let outcome = transact(connection, work);
            connection.pragma_update(None, "synchronous", "FULL").map_err(backend)?;
            outcome
        })
    }
}

/// 🧾️ Run `work` inside one immediate transaction of `connection`, committed only when it succeeds.
fn transact<T>(connection: &mut Connection, work: impl FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StorageError>) -> Result<T, StorageError> {
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(backend)?;
    let value = work(&transaction)?;
    transaction.commit().map_err(backend)?;
    Ok(value)
}

/// 🧵️ Run blocking `work` where it stalls nothing else: on a multi-threaded runtime the worker hands
/// its queue to another thread for the duration; anywhere else — a current-thread runtime, a plain
/// thread — there is nobody to hand it to and the work simply runs.
fn off_worker<T>(work: impl FnOnce() -> T) -> T {
    match tokio::runtime::Handle::try_current() {
        Ok(runtime) if runtime.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread => tokio::task::block_in_place(work),
        _ => work(),
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

/// 🚪️ Refuse unless no other handle holds the database at `path` open: a connection in exclusive
/// locking mode needs the file lock that a WAL connection — a serving proctor — keeps for its whole
/// life. The answer does not get better by waiting, so the probe waits only [`VACANCY_WAIT`] for a
/// process that is just letting go.
fn vacant(path: &Path) -> Result<(), StorageError> {
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX).map_err(backend)?;
    connection.busy_timeout(VACANCY_WAIT).map_err(backend)?;
    connection.pragma_update(None, "locking_mode", "EXCLUSIVE").map_err(backend)?;
    connection.query_row("SELECT COUNT(*) FROM sqlite_schema", [], |row| row.get::<_, i64>(0)).map(|_| ()).map_err(|error| StorageError::Backend(format!("{} is in use ({error}); stop the proctor that serves it first", path.display())))
}

fn erasure(connection: &Connection, actors: &[ActorKey]) -> Result<Erasure, StorageError> {
    let count = |statement: &str, actor: &ActorKey| connection.query_row(statement, params![actor.tenant.0, actor.kind, actor.id], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend);
    let mut erased = Vec::new();
    for actor in actors {
        erased.push(ErasedActor {
            actor: actor.clone(),
            events: count("SELECT COUNT(*) FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND id = ?3", actor)?,
            receipts: count("SELECT COUNT(*) FROM proctor_receipt WHERE tenant = ?1 AND kind = ?2 AND id = ?3", actor)?,
            outbox: count("SELECT COUNT(*) FROM proctor_outbox WHERE tenant = ?1 AND kind = ?2 AND actor = ?3", actor)?,
            snapshots: count("SELECT COUNT(*) FROM proctor_snapshot WHERE tenant = ?1 AND kind = ?2 AND id = ?3", actor)?,
            leases: count("SELECT COUNT(*) FROM proctor_lease WHERE tenant = ?1 AND kind = ?2 AND id = ?3", actor)?,
        });
    }
    let projections = connection.query_row("SELECT COUNT(*) FROM proctor_projection", [], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend)?;
    Ok(Erasure { actors: erased, projections })
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

    /// 🪢️ Every committed event of every stream of one actor kind of a tenant, stream by stream and
    /// in stream order: one range of the stream index, whatever else the log holds.
    pub fn stream_events(&self, tenant: &str, kind: &str) -> Result<Vec<EventRecord>, StorageError> {
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT tenant, kind, id, seq, millis, counter, event_kind, payload FROM proctor_event WHERE tenant = ?1 AND kind = ?2 ORDER BY id, seq").map_err(backend)?;
            let rows = statement.query_map(params![tenant, kind], |row| event_from_columns(row, 0)).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    /// 🗂️ The id of every stream of one actor kind of a tenant, ascending.
    pub fn streams(&self, tenant: &str, kind: &str) -> Result<Vec<String>, StorageError> {
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT DISTINCT id FROM proctor_event WHERE tenant = ?1 AND kind = ?2 ORDER BY id").map_err(backend)?;
            let rows = statement.query_map(params![tenant, kind], |row| row.get(0)).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    /// 🧿️ Every stream of one actor kind of a tenant at a glance, ascending by id: when it began,
    /// how many events it holds and how many of them are of `marked` kind. One pass of the stream
    /// index, whatever the number of streams.
    pub fn stream_spans(&self, tenant: &str, kind: &str, marked: &str) -> Result<Vec<StreamSpan>, StorageError> {
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT id, MIN(millis), COUNT(*), COALESCE(SUM(event_kind = ?3), 0) FROM proctor_event WHERE tenant = ?1 AND kind = ?2 GROUP BY id ORDER BY id").map_err(backend)?;
            let rows = statement.query_map(params![tenant, kind, marked], |row| Ok(StreamSpan { id: row.get(0)?, since: unsigned(row.get(1)?), events: unsigned(row.get(2)?), marked: unsigned(row.get(3)?) })).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    /// 🥇️ The first event of every stream of one actor kind of a tenant, ascending by stream id.
    pub fn first_events(&self, tenant: &str, kind: &str) -> Result<Vec<EventRecord>, StorageError> {
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT tenant, kind, id, seq, millis, counter, event_kind, payload FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND seq = 1 ORDER BY id").map_err(backend)?;
            let rows = statement.query_map(params![tenant, kind], |row| event_from_columns(row, 0)).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    /// 🏘️ Every tenant the log holds events of, ascending.
    pub fn tenants(&self) -> Result<Vec<String>, StorageError> {
        self.database.with(|connection| {
            let mut statement = connection.prepare_cached("SELECT DISTINCT tenant FROM proctor_event ORDER BY tenant").map_err(backend)?;
            let rows = statement.query_map([], |row| row.get(0)).map_err(backend)?;
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

/// 🧾️ Bind `key` to `receipt` inside `transaction`: the identical receipt again is fine, another one
/// a conflict.
fn bind_receipt(transaction: &rusqlite::Transaction<'_>, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
    match read_receipt(transaction, key)? {
        Some(stored) if &stored == receipt => Ok(()),
        Some(stored) => Err(StorageError::Conflict(format!("idempotency key {} is already bound to command {}", key.0, stored.command_id.0))),
        None => transaction
            .prepare_cached("INSERT INTO proctor_receipt (key, command_id, tenant, kind, id, revision, millis, counter) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")
            .map_err(backend)?
            .execute(params![key.0, receipt.command_id.0, receipt.actor.tenant.0, receipt.actor.kind, receipt.actor.id, signed(receipt.revision.0)?, signed(receipt.accepted_at.millis)?, i64::from(receipt.accepted_at.counter)])
            .map(|_| ())
            .map_err(backend),
    }
}

/// ➕️ Append `events` contiguously to `actor`'s stream and queue `outbox` inside `transaction`; the
/// new last sequence.
fn append(transaction: &rusqlite::Transaction<'_>, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
    let head: u64 = transaction.prepare_cached("SELECT COALESCE(MAX(seq), 0) FROM proctor_event WHERE tenant = ?1 AND kind = ?2 AND id = ?3").map_err(backend)?.query_row(params![actor.tenant.0, actor.kind, actor.id], |row| row.get::<_, i64>(0)).map(unsigned).map_err(backend)?;
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
}

/// 📸️ Replace `actor`'s snapshot inside `transaction`; one older than the stored one is a conflict.
fn replace_snapshot(transaction: &rusqlite::Transaction<'_>, actor: &ActorKey, revision: Revision, bytes: &[u8]) -> Result<(), StorageError> {
    let stored: Option<i64> = transaction.query_row("SELECT revision FROM proctor_snapshot WHERE tenant = ?1 AND kind = ?2 AND id = ?3", params![actor.tenant.0, actor.kind, actor.id], |row| row.get(0)).optional().map_err(backend)?;
    if let Some(stored) = stored.map(unsigned) {
        if revision.0 < stored {
            return Err(StorageError::Conflict(format!("snapshot revision {} is older than stored {stored}", revision.0)));
        }
    }
    transaction.execute("INSERT OR REPLACE INTO proctor_snapshot (tenant, kind, id, revision, bytes) VALUES (?1, ?2, ?3, ?4, ?5)", params![actor.tenant.0, actor.kind, actor.id, signed(revision.0)?, bytes]).map(|_| ()).map_err(backend)
}

/// 🚩️ The outbox id up to which every row is delivered: pending rows are looked for after it only,
/// so finding them costs what is pending, not what was ever queued.
fn outbox_cursor(connection: &Connection) -> Result<i64, StorageError> {
    connection.prepare_cached("SELECT COALESCE(MAX(position), 0) FROM proctor_outbox_cursor").map_err(backend)?.query_row([], |row| row.get(0)).map_err(backend)
}

impl AuthorityStore for SqliteAuthorityStore {
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        self.database.with(|connection| read_receipt(connection, key))
    }

    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
        self.database.transaction(|transaction| bind_receipt(transaction, key, receipt))
    }

    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        self.database.transaction(|transaction| append(transaction, actor, events, outbox))
    }

    /// 🧾️ One transaction for the whole batch — one flush however many turns it carries. A snapshot
    /// older than the stored one is skipped; anything else a turn cannot write rolls the batch back,
    /// and every turn answers that error.
    async fn commit(&mut self, turns: &[TurnCommit<'_>]) -> Vec<Result<(), StorageError>> {
        let committed = self.database.transaction(|transaction| {
            for turn in turns {
                append(transaction, turn.actor, turn.events, turn.outbox)?;
                if let Some((key, receipt)) = turn.receipt {
                    bind_receipt(transaction, key, receipt)?;
                }
                match turn.snapshot.map(|(revision, bytes)| replace_snapshot(transaction, turn.actor, revision, bytes)) {
                    Some(Err(StorageError::Conflict(_))) | Some(Ok(())) | None => {}
                    Some(Err(error)) => return Err(error),
                }
            }
            Ok(())
        });
        turns.iter().map(|_| committed.clone()).collect()
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
        self.database.transaction(|transaction| replace_snapshot(transaction, actor, revision, &bytes))
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
            let cursor = outbox_cursor(connection)?;
            let mut statement = connection
                .prepare_cached("SELECT o.id, o.tenant, o.kind, o.actor, o.entry_kind, o.payload, o.event_tenant, o.event_actor_kind, o.event_actor, o.event_seq, o.event_millis, o.event_counter, o.event_kind, o.event_payload FROM proctor_outbox o WHERE o.id > ?1 AND NOT EXISTS (SELECT 1 FROM proctor_outbox_delivery d WHERE d.id = o.id) ORDER BY o.id LIMIT ?2")
                .map_err(backend)?;
            let rows = statement.query_map(params![cursor, limit], outbox_from_row).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)
        })
    }

    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError> {
        if ids.is_empty() {
            return Ok(());
        }
        self.database.remakeable(|transaction| {
            for id in ids {
                let known: bool = transaction.prepare_cached("SELECT EXISTS (SELECT 1 FROM proctor_outbox WHERE id = ?1)").map_err(backend)?.query_row(params![signed(*id)?], |row| row.get(0)).map_err(backend)?;
                if !known {
                    return Err(StorageError::NotFound);
                }
            }
            let mut insert = transaction.prepare_cached("INSERT OR IGNORE INTO proctor_outbox_delivery (id) VALUES (?1)").map_err(backend)?;
            for id in ids {
                insert.execute(params![signed(*id)?]).map_err(backend)?;
            }
            let cursor = outbox_cursor(transaction)?;
            transaction
                .prepare_cached("INSERT OR REPLACE INTO proctor_outbox_cursor (singleton, position) SELECT 1, COALESCE((SELECT MIN(o.id) - 1 FROM proctor_outbox o WHERE o.id > ?1 AND NOT EXISTS (SELECT 1 FROM proctor_outbox_delivery d WHERE d.id = o.id)), (SELECT COALESCE(MAX(id), 0) FROM proctor_outbox), ?1)")
                .map_err(backend)?
                .execute(params![cursor])
                .map(|_| ())
                .map_err(backend)
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
        self.database.remakeable(|transaction| {
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
        self.database.remakeable(|transaction| transaction.execute("INSERT OR REPLACE INTO proctor_projection (projection, key, value) VALUES (?1, ?2, ?3)", params![projection, key, value]).map(|_| ()).map_err(backend))
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
        self.database.remakeable(|transaction| transaction.execute("INSERT OR REPLACE INTO proctor_checkpoint (projection, seq) VALUES (?1, ?2)", params![projection, seq]).map(|_| ()).map_err(backend))
    }

    async fn clear(&mut self, projection: &str) -> Result<(), StorageError> {
        self.database.remakeable(|transaction| {
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
