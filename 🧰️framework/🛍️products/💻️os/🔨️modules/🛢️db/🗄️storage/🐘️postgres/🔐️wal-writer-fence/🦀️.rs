//! 🔐️ Postgres wal writer fence: the advisory lock namespace and keys, the writer session, the unlock and the writer guard. See [`super`].

use super::*;

//#region 🔖️WriterFence
/// 🔒️ Namespace hashed with the document into the session advisory-lock key — contract
/// `🔐️writer/🧫️fixtures/🌐️remote-guard` (`postgres.lockNamespace`).
pub(super) const WAL_WRITER_LOCK_NAMESPACE: &str = "semio/db/wal-writer/v1";

/// 🏷️ `application_name` of every writer session, so an operator (or the conformance law's
/// independent `psql`) can name the exact session that holds a document.
pub(super) const WAL_WRITER_APPLICATION_NAME: &str = "semio-wal-writer";

/// 💓 Server-side TCP keepalive of a writer session: a vanished host's lock is released after
/// `idle + interval × count` seconds instead of the kernel's two-hour default.
pub(super) const WAL_WRITER_KEEPALIVE: [(&str, &str); 3] = [("tcp_keepalives_idle", "10"), ("tcp_keepalives_interval", "5"), ("tcp_keepalives_count", "3")];

/// 🔑 The 64-bit advisory-lock key of one document: the first eight bytes (big-endian) of
/// `sha256(namespace ‖ 0x00 ‖ document)`.
pub(super) fn wal_writer_lock_key(document: &str) -> i64 {
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(WAL_WRITER_LOCK_NAMESPACE.as_bytes());
    hash.update(&[0]);
    hash.update(document.as_bytes());
    let digest = hash.finalize();
    i64::from_be_bytes(digest[..8].try_into().expect("sha256 digest has eight leading bytes"))
}

/// 🧵 A dedicated connection whose session holds the document's advisory lock; every WAL
/// mutation of the permit runs on this session, so a terminated session can never write.
pub(super) struct PostgresWalWriterSession {
    pub(super) connection: PgConnection,
    pub(super) key: i64,
}

impl PostgresWalWriterSession {
    pub(super) async fn open(pool: &PgPool, document: &DbIoText) -> Result<Self, DbError> {
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

/// 🚨️ Classifies an error raised on the writer session: transport loss and the server's
/// operator-intervention / connection-exception classes (`57P*`, `08*`) end the session, and with it
/// the advisory lock, so they answer `Fenced` — the caller then drops the session instead of lending it again.
pub(super) fn map_session_error(err: sqlx::Error) -> DbError {
    let ended = match err.as_database_error() {
        Some(db_err) => db_err.code().is_some_and(|code| code.starts_with("57P") || code.starts_with("08")),
        None => matches!(err, sqlx::Error::Io(_) | sqlx::Error::Tls(_) | sqlx::Error::Protocol(_) | sqlx::Error::WorkerCrashed | sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut),
    };
    if ended {
        return DbError::Fenced { expected: 0, actual: 0 };
    }
    map_sqlx_error(err)
}

/// 🔔 Completion witness of a detached unlock; wakes the release controller and a parked backend close.
struct PostgresWalWriterUnlock {
    done: std::sync::atomic::AtomicBool,
    pub(super) waker: std::sync::Mutex<Option<std::task::Waker>>,
}

/// 🔐️ One document's cross-process writer fence: the lock session, lent to one pinned
/// operation at a time, then an in-flight unlock, then terminal.
pub(super) enum PostgresWalWriterGuard {
    Held { session: Option<PostgresWalWriterSession>, backend: DbIoBackendControl },
    Unlocking(Arc<PostgresWalWriterUnlock>),
    Terminal,
}

impl PostgresWalWriterGuard {
    pub(super) fn lend(&mut self, generation: u64) -> Result<PostgresWalWriterSession, DbError> {
        match self {
            Self::Held { session, .. } => session.take().ok_or(DbError::Fenced { expected: 0, actual: generation }),
            _ => Err(DbError::Closed),
        }
    }

    pub(super) fn restore(&mut self, lent: PostgresWalWriterSession) {
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
                drop(db_storage_driver_runtime::detach_unit(Box::pin(async move {
                    let _ = sqlx::query("SELECT pg_advisory_unlock($1)").bind(lent.key).execute(&mut lent.connection).await;
                    let _ = lent.connection.close().await;
                    signal.done.store(true, std::sync::atomic::Ordering::Release);
                    if let Some(waker) = signal.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        waker.wake();
                    }
                    db_io_core::request_controller(backend);
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
