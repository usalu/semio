//! 🐘️ A PostgreSQL list is ONE statement, however many rows it returns. Each list used to cost a
//! round trip per row, and the index lists its runs several times per edit, so on a remote server
//! every edit paid hundreds of round trips and outgrew the hub's 30 s frame deadline under load
//! (ticket 26/09/23 H9 session 12). The oracle is PostgreSQL's own statement log
//! (`log_statement = all`), counted while the list runs; the law starts a disposable
//! `postgres:17-alpine` container, so it is `#[ignore]`d outside a Docker-enabled live run. The same server proves the
//! backend's one async-native operation slot queues concurrent writers instead of refusing them.
use super::PostgresStorage;
use crate::db_ids::ArtifactId;
use crate::db_storage::docker_server::{docker, free_port, Container};
use crate::db_storage::{db_io_copy_pages, db_io_test_pool, IndexStorage, WalStorage};

fn postgres_server() -> (String, Container) {
    let port = free_port();
    let name = format!("semio-db-round-trips-postgres-{}-{port}", std::process::id());
    docker(&["run", "--detach", "--rm", "--name", &name, "--env", "POSTGRES_PASSWORD=postgres", "--publish", &format!("127.0.0.1:{port}:5432"), "postgres:17-alpine", "-c", "log_statement=all"]);
    let container = Container { name: name.clone() };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while std::process::Command::new("docker").args(["exec", &name, "psql", "-U", "postgres", "-h", "127.0.0.1", "-tAc", "SELECT 1"]).output().map_or(true, |out| !out.status.success()) {
        assert!(std::time::Instant::now() < deadline, "postgres fixture never answered");
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    (format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres"), container)
}

const RUNS: u64 = 40;
const SEGMENTS: u64 = 12;

fn logged(container: &str, statement: &str) -> usize {
    let output = std::process::Command::new("docker").args(["logs", container]).output().expect("docker logs");
    let log = [output.stdout, output.stderr].concat();
    String::from_utf8_lossy(&log).lines().filter(|line| line.contains("LOG:") && line.contains(statement)).count()
}

fn settled(container: &str, statement: &str) -> usize {
    let mut last = logged(container, statement);
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(250));
        let now = logged(container, statement);
        if now == last {
            return now;
        }
        last = now;
    }
    last
}

#[semio_framework_async_macros::async_test]
#[ignore = "starts a Docker postgres:17-alpine with statement logging; run with `--include-ignored db_storage_postgres::round_trips`"]
async fn a_postgres_list_is_one_statement_however_many_rows_it_returns() {
    let (url, container) = postgres_server();
    let name = container.name.clone();
    let storage = PostgresStorage::connect(db_io_test_pool(), &url).await.unwrap();
    let document: ArtifactId = "round-trips".into();

    for run in 0..RUNS {
        storage.write_run(&document, run, db_io_copy_pages(format!("run-{run}").as_bytes()).unwrap().await.unwrap()).await.unwrap();
    }
    let runs_statement = "SELECT run_id FROM db_index_run";
    let before = settled(&name, runs_statement);
    let runs = storage.list_runs(&document).await.unwrap();
    assert_eq!(runs.as_slice(), (0..RUNS).collect::<Vec<_>>().as_slice(), "every run, ascending");
    assert_eq!(settled(&name, runs_statement) - before, 1, "{RUNS} index runs are listed by one statement");

    let writer = storage.acquire_writer(&document).await.unwrap();
    for index in 0..SEGMENTS {
        storage.create_segment(&writer, index).await.unwrap();
        storage.seal(&writer, index).await.unwrap();
    }
    let segments_statement = "SELECT segment_index FROM db_wal_segment";
    let before = settled(&name, segments_statement);
    let segments = storage.list_segments(&document).await.unwrap();
    assert_eq!(segments.as_slice(), (0..SEGMENTS).collect::<Vec<_>>().as_slice(), "every segment, ascending");
    assert_eq!(settled(&name, segments_statement) - before, 1, "{SEGMENTS} WAL segments are listed by one statement");

    writer.release().await.unwrap();
    storage.close().await.unwrap();
    drop(container);
}

/// 🚦️ Two dozen writers on one PostgreSQL backend at once are all admitted: the backend runs one async-native
/// operation at a time, and a second concurrent operation used to be refused (`async-native backend operation
/// capacity exhausted`, growth e2e g18 on postgres: 24 documents edited together) instead of waiting for the slot.
#[semio_framework_async_macros::async_test]
#[ignore = "starts a Docker postgres:17-alpine; run with `--include-ignored db_storage_postgres::round_trips`"]
async fn concurrent_postgres_writers_wait_for_the_backends_one_operation_slot() {
    const WRITERS: usize = 24;
    let (url, container) = postgres_server();
    let storage = PostgresStorage::connect(db_io_test_pool(), &url).await.unwrap();
    let outcomes: Vec<Result<(), crate::db_ids::DbError>> = std::thread::scope(|scope| {
        let writers: Vec<_> = (0..WRITERS)
            .map(|writer| {
                let storage = &storage;
                scope.spawn(move || {
                    crate::db_actor::block_on(async move {
                        let document: ArtifactId = format!("concurrent-{writer}").as_str().into();
                        for run in 0..4u64 {
                            storage.write_run(&document, run, db_io_copy_pages(format!("writer-{writer}-run-{run}").as_bytes()).unwrap().await.unwrap()).await?;
                        }
                        Ok(())
                    })
                })
            })
            .collect();
        writers.into_iter().map(|writer| writer.join().unwrap()).collect()
    });
    let refused: Vec<String> = outcomes.iter().filter_map(|outcome| outcome.as_ref().err().map(ToString::to_string)).collect();
    assert!(refused.is_empty(), "{} of {WRITERS} concurrent writers were refused: {refused:?}", refused.len());
    for writer in 0..WRITERS {
        let document: ArtifactId = format!("concurrent-{writer}").as_str().into();
        assert_eq!(storage.list_runs(&document).await.unwrap().as_slice(), &[0, 1, 2, 3], "writer {writer} landed every run");
    }
    storage.close().await.unwrap();
    drop(container);
}
