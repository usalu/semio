use super::*;

use db_io_core::{DB_IO_LEDGER_OWNER, DB_IO_PAGE_BYTES, DB_IO_TASK_SLOTS, DbIoBackendControl, DbIoCredit, DbIoLedgerOwner, DbIoPageWriter, DbIoPages, PayloadStorage as _, db_io_backend_parts, db_io_ledger_census};
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, db_io_maintenance_step, db_io_test_pool, process_isolated_law};
use db_io_registry::{db_io_backend_control, db_io_backend_registry, db_io_page_maintenance_step};
use db_storage_fs::FsStorage;
use db_storage_memory::lock;

fn fixture_owner() -> DbIoLedgerOwner {
    DbIoLedgerOwner::enter()
}

#[derive(serde::Deserialize)]
struct PageLifecycleFixture {
    pub pattern_modulo: usize,
    pub pattern_addend: usize,
    pub lengths: Vec<usize>,
    pub fault_conversion_retires_result_lease: bool,
    pub fault_categories: Vec<FaultCategoryFixture>,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(tag = "category", rename_all = "snake_case")]
enum FaultCategoryFixture {
    Io,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    Conflict,
    Fenced { expected: u64, actual: u64 },
    StaleGeneration { expected: u64, actual: u64 },
    LimitExceeded,
    Unavailable,
    Timeout,
    Corrupt,
    Closed,
    Unauthorized,
    Unimplemented,
    Internal,
}

fn ledger_witness() -> (DbIoCredit, usize) {
    db_io_ledger_census(DB_IO_LEDGER_OWNER.with(std::cell::Cell::get))
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

fn drain_pages(mut pages: DbIoPages) {
    while pages.close_step().unwrap().is_some() {}
    while db_io_page_maintenance_step().unwrap().is_some() {}
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn sqlite_payload_roundtrip_obeys_the_neutral_page_lifecycle_fixture() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    let storage = db_storage_sqlite::SqliteStorage::open_in_memory(db_io_test_pool()).await.unwrap();

    for length in fixture.lengths {
        let bytes: Vec<u8> = (0..length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect();
        let expected_hash = ContentHash(*semio_framework_hash::hash(&bytes).as_bytes());
        let hash = storage.put(pages(&bytes)).await.unwrap();
        assert_eq!(hash, expected_hash);
        assert!(storage.contains(&hash).await.unwrap());
        assert_eq!(storage.len(&hash).await.unwrap(), length as u64);
        let fetched = storage.get(&hash).await.unwrap();
        assert_eq!(fetched, bytes);
        drain_pages(fetched);
        storage.delete(&hash).await.unwrap();
        assert!(!storage.contains(&hash).await.unwrap());
    }

    let missing = ContentHash([0xff; 32]);
    assert!(!storage.contains(&missing).await.unwrap());
    assert!(matches!(storage.get(&missing).await, Err(DbError::NotFound(_))));
    storage.close().await.unwrap();
    while db_io_maintenance_step().unwrap() {}
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
async fn open_real_storage_fixture(backend: &str, pool: Arc<WorkerPool>, path: &std::path::Path) -> Result<(), DbStorageOpenRejected> {
    match backend {
        "fs" => drop(FsStorage::open(pool, path).await?),
        "sqlite" => drop(db_storage_sqlite::SqliteStorage::open(pool, path).await?),
        _ => panic!("unknown physical storage fixture"),
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn opening_fixture_control(pool: &Arc<WorkerPool>) -> DbIoBackendControl {
    let registry = lock(db_io_backend_registry());
    let (index, slot) = registry.slots.iter().enumerate().find(|(_, slot)| slot.pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool))).expect("opening registered its exact pool");
    db_io_backend_control(slot.kind, index as u16, slot.generation)
}

#[cfg(not(target_arch = "wasm32"))]
fn opening_fixture_close_requested(control: DbIoBackendControl) -> bool {
    let registry = lock(db_io_backend_registry());
    let (index, generation) = db_io_backend_parts(control);
    let slot = &registry.slots[index as usize];
    slot.generation != generation || slot.close_requested
}

#[cfg(not(target_arch = "wasm32"))]
async fn drain_opening_fixture_pool(pool: &Arc<WorkerPool>) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let retained = lock(db_io_backend_registry()).slots.iter().any(|slot| slot.pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool)));
        let tasks_retained = DB_IO_TASK_SLOTS.iter().any(|task| lock(task).pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool)));
        if !retained && !tasks_retained {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "opening Drop failed to retire the exact registered pool use");
        db_io_maintenance_step().expect("opening terminal maintenance");
        semio_framework_async::yield_once().await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen() {
    if !db_io_driver::process_isolated_law("db_io_retained_fixtures::db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen") {
        return;
    }
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    for row in fixture["opening"].as_array().unwrap().iter().filter(|row| row["cause"] == "queued-drop") {
        let before = ledger_witness();
        let slots_before = lock(db_io_backend_registry()).free_len;
        let backend = row["backend"].as_str().unwrap();
        let root = std::env::temp_dir().join(format!("storage-open-queued-{backend}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.submit(
            Lane::Io,
            Box::new(move || {
                let _ = started_tx.send(());
                let _ = release_rx.recv();
            }),
        );
        started_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("opening lane blocker starts");
        let mut opening = Box::pin(open_real_storage_fixture(backend, pool.clone(), &root));
        let pending = std::future::Future::poll(opening.as_mut(), &mut std::task::Context::from_waker(std::task::Waker::noop())).is_pending();
        let control = opening_fixture_control(&pool);
        assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
        drop(opening);
        let close_requested = opening_fixture_close_requested(control);
        release_tx.send(()).expect("opening lane blocker release");
        assert!(pending, "actual BackendOpen remained queued before cancellation");
        assert_eq!(close_requested, row["closeRequestedBeforeRetry"].as_bool().unwrap(), "cancelled {backend} opening must request close before any caller retry");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(ledger_witness() == before && lock(db_io_backend_registry()).free_len == slots_before, row["ledgerBaseline"].as_bool().unwrap(), "cancelled {backend} registration returns exact credit and slot");
        assert_eq!(open_real_storage_fixture(backend, pool.clone(), &root).await.is_ok(), row["reopens"].as_bool().unwrap(), "fresh {backend} storage opens after cancellation");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(pool.shutdown().is_ok(), row["poolShutdownAfterDrain"].as_bool().unwrap());
        assert_eq!(ledger_witness(), before);
        assert_eq!(lock(db_io_backend_registry()).free_len, slots_before);
        let _ = std::fs::remove_dir_all(&root);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry() {
    if !db_io_driver::process_isolated_law("db_io_retained_fixtures::db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry") {
        return;
    }
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    for row in fixture["opening"].as_array().unwrap().iter().filter(|row| row["cause"] == "path-type-conflict") {
        let before = ledger_witness();
        let slots_before = lock(db_io_backend_registry()).free_len;
        let backend = row["backend"].as_str().unwrap();
        let root = std::env::temp_dir().join(format!("storage-open-fault-{backend}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let bad = root.join("wrong-physical-type");
        if backend == "fs" {
            std::fs::write(&bad, b"not a directory").unwrap();
        } else {
            std::fs::create_dir_all(&bad).unwrap();
            assert!(rusqlite::Connection::open(&bad).is_err(), "independent SQLite rejects the same physical directory path");
        }
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let rejected = open_real_storage_fixture(backend, pool.clone(), &bad).await.expect_err("physical BackendOpen must reject the wrong filesystem type");
        let control = match &rejected {
            DbStorageOpenRejected::Registered { control, .. } => *control,
            _ => panic!("physical opening fault lost its registered backend"),
        };
        let close_requested = opening_fixture_close_requested(control);
        drop(rejected);
        assert_eq!(close_requested, row["closeRequestedBeforeRetry"].as_bool().unwrap(), "failed {backend} opening must already own deferred close");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(ledger_witness() == before && lock(db_io_backend_registry()).free_len == slots_before, row["ledgerBaseline"].as_bool().unwrap());
        assert_eq!(open_real_storage_fixture(backend, pool.clone(), &root.join("recovered")).await.is_ok(), row["reopens"].as_bool().unwrap());
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(pool.shutdown().is_ok(), row["poolShutdownAfterDrain"].as_bool().unwrap());
        assert_eq!(ledger_witness(), before);
        assert_eq!(lock(db_io_backend_registry()).free_len, slots_before);
        let _ = std::fs::remove_dir_all(&root);
    }
}
