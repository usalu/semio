//! 🤺️ One set of cross-process WAL writer fence laws (`🧫️fixtures/🌐️remote-guard`) run unchanged
//! against SQLite, PostgreSQL and Neo4j: a second instance and a second process are refused while a
//! writer holds the document, release and process exit admit the next writer, and a holder that lost
//! ownership without knowing it is fenced by the storage before any WAL byte lands. The PostgreSQL
//! and Neo4j lanes start a disposable Docker server each and cross-check the lock through that
//! server's own client (`psql`, `cypher-shell`), so they are `#[ignore]`d outside
//! `wal-writer-fence-live`.
use crate::db_ids::{ArtifactId, DbError};
#[cfg(any(feature = "postgres", feature = "neo4j"))]
use crate::db_storage::docker_server::{docker, free_port, Container};
use crate::db_storage::{db_io_copy_pages, db_io_test_pool, DbIoPages, WalStorage, WalWriterPermit};
use std::process::Command;

const CHILD_MODE: &str = "SEMIO_WAL_WRITER_FENCE_CHILD_MODE";
const CHILD_TARGET: &str = "SEMIO_WAL_WRITER_FENCE_CHILD_TARGET";
const CHILD_SENTINEL: &str = "SEMIO_WAL_WRITER_FENCE_CHILD_SENTINEL";

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🌐️remote-guard/🔣️.json")).unwrap()
}

async fn pages(bytes: &[u8]) -> DbIoPages {
    db_io_copy_pages(bytes).unwrap().await.unwrap()
}

/// 🎯️ Where the laws run; serialised into the child process's environment.
#[derive(Clone)]
enum FenceTarget {
    Sqlite { path: String },
    #[cfg(feature = "postgres")]
    Postgres { url: String, container: String },
    #[cfg(feature = "neo4j")]
    Neo4j { uri: String, container: String },
}

enum FenceStorage {
    Sqlite(crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(crate::db_storage_neo4j::Neo4jStorage),
}

impl FenceStorage {
    async fn acquire(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
        match self {
            Self::Sqlite(storage) => storage.acquire_writer(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.acquire_writer(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.acquire_writer(document).await,
        }
    }

    async fn create(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Sqlite(storage) => storage.create_segment(writer, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.create_segment(writer, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.create_segment(writer, index).await,
        }
    }

    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
        let bytes = pages(bytes).await;
        match self {
            Self::Sqlite(storage) => storage.append(writer, index, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.append(writer, index, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.append(writer, index, bytes).await,
        }
    }

    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Sqlite(storage) => storage.seal(writer, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.seal(writer, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.seal(writer, index).await,
        }
    }

    async fn delete(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Sqlite(storage) => storage.delete_segment(writer, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.delete_segment(writer, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.delete_segment(writer, index).await,
        }
    }

    async fn length(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self {
            Self::Sqlite(storage) => storage.segment_len(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.segment_len(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.segment_len(document, index).await,
        }
    }

    async fn close(self) {
        match self {
            Self::Sqlite(storage) => storage.close().await.unwrap(),
            #[cfg(feature = "postgres")]
            Self::Postgres(storage) => storage.close().await.unwrap(),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(storage) => storage.close().await.unwrap(),
        }
    }
}

impl FenceTarget {
    fn backend(&self) -> &'static str {
        match self {
            Self::Sqlite { .. } => "sqlite",
            #[cfg(feature = "postgres")]
            Self::Postgres { .. } => "postgres",
            #[cfg(feature = "neo4j")]
            Self::Neo4j { .. } => "neo4j",
        }
    }

    fn encode(&self) -> String {
        match self {
            Self::Sqlite { path } => format!("sqlite\n{path}"),
            #[cfg(feature = "postgres")]
            Self::Postgres { url, container } => format!("postgres\n{url}\n{container}"),
            #[cfg(feature = "neo4j")]
            Self::Neo4j { uri, container } => format!("neo4j\n{uri}\n{container}"),
        }
    }

    fn decode(encoded: &str) -> Self {
        let parts: Vec<&str> = encoded.split('\n').collect();
        match parts[0] {
            "sqlite" => Self::Sqlite { path: parts[1].to_string() },
            #[cfg(feature = "postgres")]
            "postgres" => Self::Postgres { url: parts[1].to_string(), container: parts[2].to_string() },
            #[cfg(feature = "neo4j")]
            "neo4j" => Self::Neo4j { uri: parts[1].to_string(), container: parts[2].to_string() },
            other => panic!("unknown fence target {other}"),
        }
    }

    async fn open(&self) -> FenceStorage {
        let pool = db_io_test_pool();
        match self {
            Self::Sqlite { path } => FenceStorage::Sqlite(crate::db_storage_sqlite::SqliteStorage::open(pool, std::path::Path::new(path)).await.unwrap()),
            #[cfg(feature = "postgres")]
            Self::Postgres { url, .. } => FenceStorage::Postgres(crate::db_storage_postgres::PostgresStorage::connect(pool, url).await.unwrap()),
            #[cfg(feature = "neo4j")]
            Self::Neo4j { uri, .. } => FenceStorage::Neo4j(crate::db_storage_neo4j::Neo4jStorage::connect(pool, uri, "neo4j", "semio-test").await.unwrap()),
        }
    }

    /// 🔎️ How many live writer locks the server's own client sees — `None` where the lock is a
    /// sidecar file lock with no server to ask.
    fn independent_holders(&self, document: &str) -> Option<u64> {
        match self {
            Self::Sqlite { .. } => None,
            #[cfg(feature = "postgres")]
            Self::Postgres { container, .. } => {
                let sql = "SELECT count(*) FROM pg_locks l JOIN pg_stat_activity a ON a.pid = l.pid WHERE l.locktype = 'advisory' AND l.granted AND a.application_name = 'semio-wal-writer'";
                let _ = document;
                Some(docker(&["exec", container, "psql", "-U", "postgres", "-tAc", sql]).parse().unwrap())
            }
            #[cfg(feature = "neo4j")]
            Self::Neo4j { container, .. } => {
                let cypher = format!("MATCH (w:WalWriter {{document: '{document}'}}) WHERE w.holder IS NOT NULL AND w.expiresAtMs > timestamp() RETURN count(w) AS live");
                let out = docker(&["exec", container, "cypher-shell", "-u", "neo4j", "-p", "semio-test", "--format", "plain", &cypher]);
                Some(out.lines().last().unwrap().trim().parse().unwrap())
            }
        }
    }

    /// 🪓️ Takes ownership away from a live holder behind its back, through the server's own client.
    fn lose_ownership(&self, document: &str) {
        match self {
            Self::Sqlite { .. } => panic!("a SQLite sidecar lock is only lost with its process"),
            #[cfg(feature = "postgres")]
            Self::Postgres { container, .. } => {
                let sql = "SELECT count(*) FROM (SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE application_name = 'semio-wal-writer') AS terminated";
                assert_eq!(docker(&["exec", container, "psql", "-U", "postgres", "-tAc", sql]), "1");
            }
            #[cfg(feature = "neo4j")]
            Self::Neo4j { container, .. } => {
                let cypher = format!("MATCH (w:WalWriter {{document: '{document}'}}) SET w.expiresAtMs = 0 RETURN w.token AS token");
                docker(&["exec", container, "cypher-shell", "-u", "neo4j", "-p", "semio-test", "--format", "plain", &cypher]);
            }
        }
    }

    fn token(&self, document: &str) -> Option<u64> {
        match self {
            #[cfg(feature = "neo4j")]
            Self::Neo4j { container, .. } => {
                let cypher = format!("MATCH (w:WalWriter {{document: '{document}'}}) RETURN w.token AS token");
                let out = docker(&["exec", container, "cypher-shell", "-u", "neo4j", "-p", "semio-test", "--format", "plain", &cypher]);
                Some(out.lines().last().unwrap().trim().parse().unwrap())
            }
            _ => {
                let _ = document;
                None
            }
        }
    }

    fn crash_release_ms(&self) -> u64 {
        let fixture = fixture();
        let row = fixture["backends"].as_array().unwrap().iter().find(|row| row["backend"] == self.backend()).unwrap().clone();
        match row["crashRelease"].as_str().unwrap() {
            "immediate" => 0,
            "after-lease-ttl" => fixture["neo4j"]["leaseTtlMs"].as_u64().unwrap(),
            other => panic!("unknown crash release {other}"),
        }
    }
}

/// 👶️ The re-executed test binary: one contender process against the parent's target.
async fn fence_child() -> bool {
    let (Ok(mode), Ok(target), Some(sentinel)) = (std::env::var(CHILD_MODE), std::env::var(CHILD_TARGET), std::env::var_os(CHILD_SENTINEL)) else { return false };
    let target = FenceTarget::decode(&target);
    let storage = target.open().await;
    let document: ArtifactId = fixture()["document"].as_str().unwrap().into();
    match mode.as_str() {
        "conflict" => {
            let refused = storage.acquire(&document).await;
            assert!(matches!(refused, Err(DbError::Conflict(_))), "a second process must be refused while the writer is held: {:?}", refused.map(|_| ()));
            std::fs::write(sentinel, format!("{}:conflict", std::process::id())).unwrap();
            storage.close().await;
        }
        "crash" => {
            let _writer = storage.acquire(&document).await.unwrap();
            std::fs::write(sentinel, format!("{}:acquired", std::process::id())).unwrap();
            std::process::exit(0);
        }
        other => panic!("unknown fence child mode {other}"),
    }
    true
}

fn run_child(test: &str, target: &FenceTarget, mode: &str, sentinel: &std::path::Path) {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([test, "--exact", "--include-ignored", "--test-threads=1", "--nocapture"])
        .env(CHILD_MODE, mode)
        .env(CHILD_TARGET, target.encode())
        .env(CHILD_SENTINEL, sentinel)
        .output()
        .unwrap();
    assert!(output.status.success(), "{} fence child `{mode}` failed:\n{}\n{}", target.backend(), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
}

fn scratch(backend: &str) -> std::path::PathBuf {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = base.join(format!("wal-writer-fence-{backend}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    root
}

/// ⚖️ Every shared law of the contract that names `target`'s backend, in contract order.
async fn run_fence_laws(target: FenceTarget, test: &str) {
    let fixture = fixture();
    let backend = target.backend();
    let laws: Vec<String> = fixture["laws"].as_array().unwrap().iter().filter(|law| law["backends"].as_array().unwrap().iter().any(|name| name == backend)).map(|law| law["name"].as_str().unwrap().to_string()).collect();
    let document: ArtifactId = fixture["document"].as_str().unwrap().into();
    let scratch = scratch(backend);
    let holder = target.open().await;
    let contender = target.open().await;
    let mut proven = Vec::new();
    for law in &laws {
        match law.as_str() {
            "second-instance-conflicts" => {
                let writer = holder.acquire(&document).await.unwrap();
                assert!(matches!(contender.acquire(&document).await, Err(DbError::Conflict(_))), "{backend}: a second instance must be refused");
                if let Some(live) = target.independent_holders(document.0.as_str()) {
                    assert_eq!(live, 1, "{backend}: the server's own client must see exactly one live writer lock");
                }
                writer.release().await.unwrap();
            }
            "other-process-conflicts" => {
                let writer = holder.acquire(&document).await.unwrap();
                let sentinel = scratch.join("conflict.txt");
                run_child(test, &target, "conflict", &sentinel);
                assert!(std::fs::read_to_string(&sentinel).unwrap().ends_with(":conflict"));
                writer.release().await.unwrap();
            }
            "release-admits-contender" => {
                let writer = holder.acquire(&document).await.unwrap();
                writer.release().await.unwrap();
                if let Some(live) = target.independent_holders(document.0.as_str()) {
                    assert_eq!(live, 0, "{backend}: a released writer must leave no live lock on the server");
                }
                let next = contender.acquire(&document).await.unwrap();
                next.release().await.unwrap();
            }
            "process-exit-releases" => {
                let sentinel = scratch.join("crash.txt");
                run_child(test, &target, "crash", &sentinel);
                assert!(std::fs::read_to_string(&sentinel).unwrap().ends_with(":acquired"));
                let exited = std::time::Instant::now();
                let wait = target.crash_release_ms();
                let writer = loop {
                    match holder.acquire(&document).await {
                        Ok(writer) => break writer,
                        Err(DbError::Conflict(_)) if wait != 0 && exited.elapsed().as_millis() < u128::from(wait + fixture["neo4j"]["renewEveryMs"].as_u64().unwrap()) => std::thread::sleep(std::time::Duration::from_millis(250)),
                        Err(error) => panic!("{backend}: an exited holder must release the writer (waited {:?}): {error}", exited.elapsed()),
                    }
                };
                eprintln!("{backend}: exited holder released after {:?} (contract {wait} ms)", exited.elapsed());
                writer.release().await.unwrap();
            }
            "lost-owner-is-fenced" => {
                let stale = holder.acquire(&document).await.unwrap();
                holder.create(&stale, 0).await.unwrap();
                assert_eq!(holder.append(&stale, 0, b"held").await.unwrap(), 4);
                target.lose_ownership(document.0.as_str());
                let successor = contender.acquire(&document).await.unwrap();
                let before = target.token(document.0.as_str());
                assert!(matches!(holder.append(&stale, 0, b"stale").await, Err(DbError::Fenced { .. })), "{backend}: a holder that lost ownership must be fenced");
                assert!(matches!(holder.seal(&stale, 0).await, Err(DbError::Fenced { .. })), "{backend}: fencing is sticky");
                assert_eq!(contender.length(&document, 0).await.unwrap(), 4, "{backend}: a fenced write must leave no WAL effect");
                assert_eq!(target.token(document.0.as_str()), before, "{backend}: a fenced write must not advance the fencing token");
                assert_eq!(contender.append(&successor, 0, b"next").await.unwrap(), 8);
                contender.delete(&successor, 0).await.unwrap();
                successor.release().await.unwrap();
                stale.release().await.unwrap();
            }
            other => panic!("unknown fence law {other}"),
        }
        proven.push(law.clone());
    }
    holder.close().await;
    contender.close().await;
    let _ = std::fs::remove_dir_all(&scratch);
    assert!(proven.len() >= 4, "{backend}: too few shared laws ran");
    eprintln!("wal-writer-fence {backend}: {}", proven.join(", "));
}

#[semio_framework_async_macros::async_test]
async fn sqlite_wal_writer_fence_holds_every_shared_law() {
    if fence_child().await {
        return;
    }
    let root = scratch("sqlite-db");
    let path = root.join("fence.sqlite3").to_string_lossy().into_owned();
    run_fence_laws(FenceTarget::Sqlite { path }, "db_storage::writer::fence_conformance::sqlite_wal_writer_fence_holds_every_shared_law").await;
    let _ = std::fs::remove_dir_all(root);
}

#[cfg(feature = "postgres")]
#[semio_framework_async_macros::async_test]
#[ignore = "starts a Docker postgres:17-alpine; run through `wal-writer-fence-live`"]
async fn postgres_wal_writer_fence_holds_every_shared_law() {
    if fence_child().await {
        return;
    }
    let port = free_port();
    let name = format!("semio-db-fence-postgres-{}-{port}", std::process::id());
    docker(&["run", "--detach", "--rm", "--name", &name, "--env", "POSTGRES_PASSWORD=postgres", "--publish", &format!("127.0.0.1:{port}:5432"), "postgres:17-alpine"]);
    let container = Container { name: name.clone() };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while Command::new("docker").args(["exec", &name, "psql", "-U", "postgres", "-h", "127.0.0.1", "-tAc", "SELECT 1"]).output().map_or(true, |out| !out.status.success()) {
        assert!(std::time::Instant::now() < deadline, "postgres fixture never answered");
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    run_fence_laws(FenceTarget::Postgres { url, container: name }, "db_storage::writer::fence_conformance::postgres_wal_writer_fence_holds_every_shared_law").await;
    drop(container);
}

#[cfg(feature = "neo4j")]
#[semio_framework_async_macros::async_test]
#[ignore = "starts a Docker neo4j:5-community; run through `wal-writer-fence-live`"]
async fn neo4j_wal_writer_fence_holds_every_shared_law() {
    if fence_child().await {
        return;
    }
    let port = free_port();
    let name = format!("semio-db-fence-neo4j-{}-{port}", std::process::id());
    docker(&["run", "--detach", "--rm", "--name", &name, "--env", "NEO4J_AUTH=neo4j/semio-test", "--publish", &format!("127.0.0.1:{port}:7687"), "neo4j:5-community"]);
    let container = Container { name: name.clone() };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
    while Command::new("docker").args(["exec", &name, "cypher-shell", "-u", "neo4j", "-p", "semio-test", "RETURN 1"]).output().map_or(true, |out| !out.status.success()) {
        assert!(std::time::Instant::now() < deadline, "neo4j fixture never answered");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    let uri = format!("127.0.0.1:{port}");
    run_fence_laws(FenceTarget::Neo4j { uri, container: name }, "db_storage::writer::fence_conformance::neo4j_wal_writer_fence_holds_every_shared_law").await;
    drop(container);
}
