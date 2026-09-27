//! 🐘️ A PostgreSQL list is ONE statement, however many rows it returns. Each list used to cost a
//! round trip per row, and the index lists its runs several times per edit, so on a remote server
//! every edit paid hundreds of round trips and outgrew the hub's 30 s frame deadline under load
//! (ticket 26/09/23 H9 session 12). The oracle is PostgreSQL's own table statistics
//! (`pg_stat_user_tables`: one scan of the listed table per statement, a statement per row before), read through the
//! claimed server's own `psql` on the claimed run database, which only the list law writes: every other law here works in
//! a database of its own (`FreshPostgresDatabase`). The laws run on the ONE shared development server claimed by
//! `os-hub-ts backend run postgres -- …`, so they are `#[ignore]`d outside that run. The same server proves the backend's
//! one async-native operation slot queues concurrent writers instead of refusing them, and that concurrent openers of a
//! fresh database all find its schema.
use super::PostgresStorage;
use crate::db_ids::ArtifactId;
use crate::db_storage::claimed_backend;
use crate::db_storage::{db_io_copy_pages, db_io_test_pool, IndexStorage, WalStorage};

const RUNS: u64 = 40;
const SEGMENTS: u64 = 12;
/// ⏳️ A backend flushes its pending statistics at most 10 s after it goes idle (`PGSTAT_IDLE_INTERVAL`); a count that
/// held still longer than that has every scan of the finished statements in it.
const STATISTICS_STILL: std::time::Duration = std::time::Duration::from_secs(12);

/// 🔢️ How many scans the claimed run database has made of `table`, once its statistics are still.
fn settled_scans(table: &str) -> u64 {
    let query = format!("SELECT coalesce(sum(seq_scan + coalesce(idx_scan, 0)), 0) FROM pg_stat_user_tables WHERE relname = '{table}'");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(90);
    let mut last = claimed_backend::client(&query).parse::<u64>().expect("scan count");
    let mut still_since = std::time::Instant::now();
    while still_since.elapsed() < STATISTICS_STILL {
        assert!(std::time::Instant::now() < deadline, "{table} statistics never settled");
        std::thread::sleep(std::time::Duration::from_millis(500));
        let now = claimed_backend::client(&query).parse::<u64>().expect("scan count");
        if now != last {
            last = now;
            still_since = std::time::Instant::now();
        }
    }
    last
}

#[semio_framework_async_macros::async_test]
#[ignore = "needs the claimed shared postgres: `os-hub-ts backend run postgres -- … --include-ignored db_storage_postgres::round_trips`"]
async fn a_postgres_list_is_one_statement_however_many_rows_it_returns() {
    let storage = PostgresStorage::connect(db_io_test_pool(), &claimed_backend::env("OS_HUB_DATABASE_URL")).await.unwrap();
    let document: ArtifactId = "round-trips".into();

    for run in 0..RUNS {
        storage.write_run(&document, run, db_io_copy_pages(format!("run-{run}").as_bytes()).unwrap().await.unwrap()).await.unwrap();
    }
    let before = settled_scans("db_index_run");
    let runs = storage.list_runs(&document).await.unwrap();
    assert_eq!(runs.as_slice(), (0..RUNS).collect::<Vec<_>>().as_slice(), "every run, ascending");
    assert_eq!(settled_scans("db_index_run") - before, 1, "{RUNS} index runs are listed by one statement");

    let writer = storage.acquire_writer(&document).await.unwrap();
    for index in 0..SEGMENTS {
        storage.create_segment(&writer, index).await.unwrap();
        storage.seal(&writer, index).await.unwrap();
    }
    let before = settled_scans("db_wal_segment");
    let segments = storage.list_segments(&document).await.unwrap();
    assert_eq!(segments.as_slice(), (0..SEGMENTS).collect::<Vec<_>>().as_slice(), "every segment, ascending");
    assert_eq!(settled_scans("db_wal_segment") - before, 1, "{SEGMENTS} WAL segments are listed by one statement");

    writer.release().await.unwrap();
    storage.close().await.unwrap();
}

/// 🚦️ Two dozen writers on one PostgreSQL backend at once are all admitted: the backend runs one async-native
/// operation at a time, and a second concurrent operation used to be refused (`async-native backend operation
/// capacity exhausted`, growth e2e g18 on postgres: 24 documents edited together) instead of waiting for the slot.
#[semio_framework_async_macros::async_test]
#[ignore = "needs the claimed shared postgres: `os-hub-ts backend run postgres -- … --include-ignored db_storage_postgres::round_trips`"]
async fn concurrent_postgres_writers_wait_for_the_backends_one_operation_slot() {
    const WRITERS: usize = 24;
    let database = claimed_backend::FreshPostgresDatabase::create("writers");
    let storage = PostgresStorage::connect(db_io_test_pool(), &database.url).await.unwrap();
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
}

/// 🔒️ Openers of one fresh database at once all find its schema: PostgreSQL's `CREATE TABLE IF NOT EXISTS` races a
/// concurrent creator of the same table (`duplicate key value violates unique constraint "pg_type_typname_nsp_index"`),
/// which refused two of three storages opening one claimed run database together; the bootstrap now runs in one
/// transaction under an advisory lock. The law opens sixteen storages concurrently on a database it creates fresh
/// through the claimed server's own client, and drops it afterwards.
#[semio_framework_async_macros::async_test]
#[ignore = "needs the claimed shared postgres: `os-hub-ts backend run postgres -- … --include-ignored db_storage_postgres::round_trips`"]
async fn concurrent_openers_of_a_fresh_database_all_find_its_schema() {
    const OPENERS: usize = 16;
    let database = claimed_backend::FreshPostgresDatabase::create("bootstrap");
    let outcomes: Vec<Result<(), String>> = std::thread::scope(|scope| {
        let openers: Vec<_> = (0..OPENERS)
            .map(|_| {
                let url = &database.url;
                scope.spawn(move || {
                    crate::db_actor::block_on(async move {
                        let storage = PostgresStorage::connect(db_io_test_pool(), url).await.map_err(|refused| format!("{refused:?}"))?;
                        storage.close().await.map_err(|error| error.to_string())
                    })
                })
            })
            .collect();
        openers.into_iter().map(|opener| opener.join().unwrap()).collect()
    });
    let refused: Vec<&String> = outcomes.iter().filter_map(|outcome| outcome.as_ref().err()).collect();
    assert!(refused.is_empty(), "{} of {OPENERS} concurrent openers of a fresh database were refused: {refused:?}", refused.len());
}
