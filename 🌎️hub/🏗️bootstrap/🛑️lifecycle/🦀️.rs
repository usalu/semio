//! 🛑️ `lifecycle` — worker pool, storage connection, socket drain and termination signals.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::ws::{CloseFrame, Message};
use futures::SinkExt;
use semio_framework_trace::record::{TraceOutcome, TraceRecord, Tracer};
use semio_hub::artifact_authority::chunk_cas::{Neo4jArtifactChunkCasStorage, PostgresArtifactChunkCasStorage, SqliteArtifactChunkCasStorage, ArtifactChunkCasStores, FsArtifactChunkCasStorage};
use semio_hub::directory::sqlite::SqliteDirectory;
use semio_hub::directory::{HubDirectories, LocalBootstrapTransport};
use std::sync::Arc;
use crate::startup::HubError;

//#region 🔖️Main
/// 🧬️ Resolves and connects `db::Database`'s storage substrate, selected by
/// `OS_HUB_STORAGE_BACKEND` (`fs` default, zero-touch, rooted at `{data_dir}/db`; `sqlite`,
/// `postgres` — requires `OS_HUB_DATABASE_URL` — or `neo4j` — requires `OS_HUB_NEO4J_URI` —
/// otherwise, each match arm compiled only when this crate's same-named feature enables `db`'s own
/// matching storage feature). Independent of `connect_directory`'s own backend choice (the
/// contract's "storage swappability" requirement applies to `db`'s substrate and the directory's
/// substrate separately, even though both now share the same three feature names).
/// 🧵️ Hub's ONE process-wide `WorkerPool` — Phase 1
/// (`26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`) replaces `HubDbRuntime` (a `HostAsyncRuntime` bridge
/// whose entire reason to exist was `run_blocking`, now deleted from that trait — see
/// `db_storage`'s module doc) with this: `db::storage_sqlite::SqliteStorage::open`'s blocking body
/// now dispatches onto `Lane::Io` here directly, and every `db::Database` receives the same pool
/// during construction so `ArtifactHandle::submit` shares it — no more per-submit
/// `"db-engine-submit-bridge"` OS thread, no more sqlite-storage
/// calls stalling their caller's own tokio worker thread. Hub is a headless server
/// (`ProcessKind::HeadlessBatch`: no UI thread to reserve a core for), sized to the process's visible
/// core count.
///
/// 🧪️ Under `cfg(test)` each hub gets its OWN pool instead of the process singleton. `process_worker_pool`
/// is a `OnceLock` — one pool per process entry point — and that is right for a hub, which is one process.
/// The bin test binary is not one hub: `cargo test` hosts every law of `🔬️bin-unit` in a single process, so
/// ~180 independent `HubState`s were sharing ten workers and queueing each other's blocking storage work
/// behind laws that hold a worker for their whole body. That is why `an_authenticated_session_read…` and
/// `the_observability_route_answers…` never answered in-process while passing under nextest (one process per
/// law) and on a live hub. Per-state pools restore the isolation nextest gets for free.
pub(crate) fn hub_worker_pool() -> Arc<semio_framework_async::WorkerPool> {
    #[cfg(test)]
    {
        return Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    }
    #[cfg(not(test))]
    {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let config = semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, cores);
        Arc::new(semio_framework_async::process_worker_pool(config))
    }
}

pub(crate) async fn connect_db(data_dir: &std::path::Path) -> Result<db::Database, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let profile = db::Profile::Prod;
    let pool = hub_worker_pool();
    match backend.as_str() {
        "fs" | "" => {
            let root = data_dir.join("db");
            Ok(db::Database::open_at(pool, &root, profile).await?)
        }
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            let storage = db::storage_sqlite::SqliteStorage::open(pool.clone(), std::path::Path::new(&path)).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Sqlite(storage))).await?)
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            let storage = db::storage_postgres::PostgresStorage::connect(pool.clone(), &database_url).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Postgres(storage))).await?)
        }
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            let storage = db::storage_neo4j::Neo4jStorage::connect(pool.clone(), &uri, &user, &password).await?;
            Ok(db::Database::open(pool, db::DbConfig::for_profile(profile), Arc::new(db::storage::DbBackend::Neo4j(storage))).await?)
        }
        other => Err(HubError::UnknownStorageBackend(other.to_string())),
    }
}

/// 🧩️ Opens the hub-owned artifact CAS in a namespace independent from generic DB payloads.
pub(crate) async fn connect_artifact_cas(data_dir: &std::path::Path) -> Result<Arc<ArtifactChunkCasStores>, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "fs".into());
    let storage = match backend.as_str() {
        "fs" | "" => ArtifactChunkCasStores::Filesystem(FsArtifactChunkCasStorage::open(&data_dir.join("artifact-cas/v1")).await?),
        "sqlite" => {
            let path = std::env::var("OS_HUB_DB_SQLITE").unwrap_or_else(|_| data_dir.join("db.sqlite3").to_string_lossy().into_owned());
            ArtifactChunkCasStores::Sqlite(SqliteArtifactChunkCasStorage::open(std::path::Path::new(&path)).await?)
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL").map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            ArtifactChunkCasStores::Postgres(PostgresArtifactChunkCasStorage::connect(&database_url).await?)
        }
        "neo4j" => {
            let uri = std::env::var("OS_HUB_NEO4J_URI").map_err(|_| HubError::UnknownStorageBackend("neo4j requires OS_HUB_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_NEO4J_PASSWORD").unwrap_or_default();
            ArtifactChunkCasStores::Neo4j(Neo4jArtifactChunkCasStorage::connect(&uri, &user, &password).await?)
        }
        other => return Err(HubError::UnknownStorageBackend(other.to_string())),
    };
    Ok(Arc::new(storage))
}

/// 🧬️ Resolves and connects the identity/tenancy directory backend, selected by
/// `OS_HUB_DIRECTORY_BACKEND` (`sqlite` default, zero-touch, `{data_dir}/directory.db`; `postgres`
/// — requires `OS_HUB_DIRECTORY_DATABASE_URL` — or `neo4j` — requires
/// `OS_HUB_DIRECTORY_NEO4J_URI` — otherwise, each match arm compiled only when this crate's
/// same-named feature is enabled).
// 🌵️ `data_dir` is only read by the `sqlite` arm below (`postgres`/`neo4j` connect via env-provided
// URIs/connection strings instead, never a local path) — whenever the `sqlite` feature is off it
// goes genuinely unused (whether or not another backend feature is on), so the allow is scoped to
// exactly that condition rather than blanket-silencing the lint for every feature set.
pub(crate) async fn connect_directory(data_dir: &std::path::Path) -> Result<Arc<HubDirectories>, HubError> {
    let backend = std::env::var("OS_HUB_DIRECTORY_BACKEND").unwrap_or_else(|_| "sqlite".into());
    match backend.as_str() {
        "sqlite" | "" => {
            let path = data_dir.join("directory.db");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let directory = SqliteDirectory::connect(&path.to_string_lossy()).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DIRECTORY_DATABASE_URL").map_err(|_| HubError::UnknownDirectoryBackend("postgres requires OS_HUB_DIRECTORY_DATABASE_URL".into()))?;
            let directory = semio_hub::directory::postgres::PostgresDirectory::connect(&database_url).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        "neo4j" => {
            let uri = std::env::var("OS_HUB_DIRECTORY_NEO4J_URI").map_err(|_| HubError::UnknownDirectoryBackend("neo4j requires OS_HUB_DIRECTORY_NEO4J_URI".into()))?;
            let user = std::env::var("OS_HUB_DIRECTORY_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
            let password = std::env::var("OS_HUB_DIRECTORY_NEO4J_PASSWORD").unwrap_or_default();
            let directory = semio_hub::directory::neo4j::Neo4jDirectory::connect(&uri, &user, &password).await?;
            directory.seed().await?;
            Ok(Arc::new(directory.into()))
        }
        other => Err(HubError::UnknownDirectoryBackend(other.to_string())),
    }
}

/// ⏳️ How long `main` waits for upgraded sockets to finish their own close path (directory
/// session close, presence leave, color release) after it told them to close.
pub(crate) const SOCKET_DRAIN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(5);

/// ⏳️ How long `main` gives the `Database` to retire every document authority before it
/// closes the storage anyway. Closing the storage is what releases the cross-process WAL writer
/// fence (Postgres advisory lock, Neo4j lease), so it runs even when this deadline elapsed.
pub(crate) const DATABASE_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

/// 🚪️ The one owner of every upgraded socket's lifetime. `axum`'s graceful shutdown stops
/// accepting and waits for HTTP responses, but an upgraded connection has left `hyper`'s connection
/// tracking, so without this owner a document socket keeps its `ArtifactHandle` (and with it the
/// document's WAL writer) until the process dies.
pub(crate) struct HubSocketDrainV1 {
    pub(crate) closing: tokio::sync::watch::Sender<bool>,
    pub(crate) live: std::sync::atomic::AtomicUsize,
    idle: tokio::sync::Notify,
}

impl Default for HubSocketDrainV1 {
    fn default() -> Self {
        Self { closing: tokio::sync::watch::Sender::new(false), live: std::sync::atomic::AtomicUsize::new(0), idle: tokio::sync::Notify::new() }
    }
}

impl HubSocketDrainV1 {
    /// 🎟️ Admits one freshly upgraded socket; `None` once shutdown has begun.
    pub(crate) fn admit(self: &Arc<Self>) -> Option<HubSocketDrainAdmissionV1> {
        self.live.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let admission = HubSocketDrainAdmissionV1 { drain: self.clone(), closing: self.closing.subscribe() };
        if *admission.closing.borrow() {
            return None;
        }
        Some(admission)
    }

    /// 📣️ Tells every admitted socket to close and refuses every later one. Idempotent.
    pub(crate) fn begin(&self) {
        self.closing.send_replace(true);
    }

    /// ⏳️ Waits until the last admission dropped or `deadline` elapsed; answers how many
    /// sockets are still live.
    pub(crate) async fn drained(&self, deadline: std::time::Duration) -> usize {
        let wait = async {
            loop {
                let notified = self.idle.notified();
                tokio::pin!(notified);
                notified.as_mut().enable();
                if self.live.load(std::sync::atomic::Ordering::Acquire) == 0 {
                    return;
                }
                notified.await;
            }
        };
        let _ = tokio::time::timeout(deadline, wait).await;
        self.live.load(std::sync::atomic::Ordering::Acquire)
    }
}

/// 🎫️ One live socket's hold on [`HubSocketDrainV1`]; dropping it is that socket's exit.
pub(crate) struct HubSocketDrainAdmissionV1 {
    pub(crate) drain: Arc<HubSocketDrainV1>,
    pub(crate) closing: tokio::sync::watch::Receiver<bool>,
}

impl HubSocketDrainAdmissionV1 {
    /// 📣️ Resolves once the hub began shutting down.
    pub(crate) async fn closing(&mut self) {
        let _ = self.closing.wait_for(|closing| *closing).await;
    }
}

impl Drop for HubSocketDrainAdmissionV1 {
    fn drop(&mut self) {
        if self.drain.live.fetch_sub(1, std::sync::atomic::Ordering::AcqRel) == 1 {
            self.drain.idle.notify_waiters();
        }
    }
}

/// 🔌️ The close code a socket receives when the hub shuts down: 1012 "service restart",
/// which every client treats as transient and redials (only 4401 is terminal). Pinned by
/// `🧫️fixtures/🤝️two-client-document-v1` `shutdown.closeCode`.
pub(crate) const HUB_SHUTDOWN_CLOSE_CODE: u16 = 1012;

/// 🔌️ The close frame carrying [`HUB_SHUTDOWN_CLOSE_CODE`].
pub(crate) fn hub_shutdown_close_frame() -> Message {
    Message::Close(Some(CloseFrame { code: HUB_SHUTDOWN_CLOSE_CODE, reason: "hub-shutdown".into() }))
}

/// 🔚️ Retires the hub's `Database` and closes its storage. The `Database` is shut down once
/// `main` is its last owner (every socket and task that held a clone has drained by then); the
/// storage close that follows releases every WAL writer on the backend itself — the Postgres
/// advisory lock's session ends and the Neo4j lease is marked released — and resolves only after
/// the backend confirmed it, so a restart right after this process exits reopens the same documents
/// immediately. Contract: `🛢️db/🗄️storage/🔐️writer/🧬️schema/🔣️.json#/$defs/WalWriterFenceV1`.
pub(crate) async fn close_hub_database(db: Arc<db::Database>, deadline: std::time::Duration) -> Result<(), HubError> {
    let started = std::time::Instant::now();
    let control = db::DatabaseShutdownControl::for_timeout(deadline);
    let storage = db.storage().await;
    let mut owner = db;
    let shutdown = loop {
        match Arc::try_unwrap(owner) {
            Ok(mut database) => break database.shutdown(&control).await,
            Err(retained) if started.elapsed() < deadline => {
                owner = retained;
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
            Err(retained) => break Err(db::DbError::Conflict(format!("hub database still shared by {} owners at shutdown", Arc::strong_count(&retained) - 1))),
        }
    };
    let close = storage.close().await;
    shutdown.and(close).map_err(HubError::Db)
}

/// 🛑️ Which termination signal an orchestrator sent. `docker stop`, `systemctl stop` and a
/// Kubernetes pod eviction all send `SIGTERM`; a terminal sends `SIGINT`. Both mean the same thing
/// to this process — stop accepting, then run the drains `main` already runs after `axum::serve`
/// returns (`AdminOperationTaskOwner::shutdown`, `ArtifactCreationHttpTaskOwnerV1::shutdown` →
/// `shutdown_with_deadline`, the inference runtime close and the artifact CAS maintenance stop).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TerminationSignalV1 {
    Interrupt,
    Terminate,
}

impl TerminationSignalV1 {
    /// 🏷️ The POSIX name, as the shutdown line reports it.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Interrupt => "SIGINT",
            Self::Terminate => "SIGTERM",
        }
    }
}

/// 🛑️ Whichever of the two arrives first, with `SIGTERM` preferred when both are already
/// ready — the orchestrator's verdict outranks a console interrupt. Taking the two futures as
/// arguments is what makes the choice testable without raising a real signal at a shared test
/// binary.
pub(crate) async fn first_termination_signal(interrupt: impl std::future::Future<Output = ()>, terminate: impl std::future::Future<Output = ()>) -> TerminationSignalV1 {
    tokio::pin!(interrupt);
    tokio::pin!(terminate);
    tokio::select! {
        biased;
        () = &mut terminate => TerminationSignalV1::Terminate,
        () = &mut interrupt => TerminationSignalV1::Interrupt,
    }
}

/// 🛑️ The real process-wide handler. A platform that refuses the `SIGTERM` registration
/// leaves that arm pending for ever rather than failing the boot: a hub that cannot hear `SIGTERM`
/// is the behaviour this process had before, and it still hears `SIGINT`.
#[cfg(unix)]
pub(crate) async fn termination_signal(tracer: &Tracer) -> TerminationSignalV1 {
    let interrupt = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(error) => {
                let mut record = TraceRecord::new("server.boot", TraceOutcome::Failed);
                record.detail = Some(format!("sigterm-handler-unavailable:{error}"));
                tracer.emit(record);
                std::future::pending::<()>().await;
            }
        }
    };
    first_termination_signal(interrupt, terminate).await
}

/// 🛑️ See the unix twin. Windows has no `SIGTERM`; `ctrl_c` covers both console interrupt
/// and the shutdown event a service manager raises.
#[cfg(not(unix))]
pub(crate) async fn termination_signal(_tracer: &Tracer) -> TerminationSignalV1 {
    let _ = tokio::signal::ctrl_c().await;
    TerminationSignalV1::Interrupt
}

/// 🧵️ Stack budget of every hub runtime thread, workers and blocking pool alike. Durable-store turns
/// (fixed-capacity owners that decode, inflate and verify a pack) run inline under the whole HTTP
/// middleware stack, and in an unoptimized build that chain outgrows tokio's 2 MiB default: measured
/// as a SIGABRT stack overflow in `inflate` under `post_inference_gis_map_job_approval`. 8 MiB is the
/// budget a process main thread gets on every supported platform.
pub(crate) const HUB_RUNTIME_THREAD_STACK_BYTES: usize = 8 * 1024 * 1024;
//#endregion 🔖️Main
