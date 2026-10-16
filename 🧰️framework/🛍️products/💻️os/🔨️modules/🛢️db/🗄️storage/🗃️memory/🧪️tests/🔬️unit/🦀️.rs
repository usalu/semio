use super::*;

use db_io_core::{CatalogStorage, DB_IO_BACKEND_CONTROLS, DB_IO_PAGE_BYTES, DbIoPageWriter, DbIoPages, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, WalSegmentState, WalStorage};
use db_io_driver::{DB_IO_ADMISSION_WAIT_MS, WalWriterGuard as _, block_on_ready, db_io_backend_admission_saturated, db_io_test_pool, process_isolated_law};

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("test storage pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}
//#region 🔖️WalStorage
async fn exercise_wal_storage(storage: &impl WalStorage) {
    let document: ArtifactId = "doc-wal".into();
    let writer = block_on_ready(storage.acquire_writer(&document)).await.unwrap();

    block_on_ready(storage.create_segment(&writer, 0)).await.unwrap();
    assert!(matches!(block_on_ready(storage.create_segment(&writer, 0)).await, Err(DbError::AlreadyExists(_))));

    let len_after_first = block_on_ready(storage.append(&writer, 0, pages(b"hello "))).await.unwrap();
    assert_eq!(len_after_first, 6);
    let len_after_second = block_on_ready(storage.append(&writer, 0, pages(b"world"))).await.unwrap();
    assert_eq!(len_after_second, 11);
    assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);
    assert_eq!(block_on_ready(storage.segment_state(&document, 0)).await.unwrap(), WalSegmentState::Active);
    assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);
    assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 11 })).await.unwrap(), b"hello world");
    assert!(matches!(block_on_ready(storage.segment_state(&document, 99)).await, Err(DbError::NotFound(_))));

    let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).await.unwrap();
    assert_eq!(read_back, b"world");
    assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })).await, Err(DbError::InvalidArgument(_))));

    block_on_ready(storage.sync(&writer, 0, DurabilityClass::Fsync)).await.unwrap();

    block_on_ready(storage.truncate_tail(&writer, 0, 6)).await.unwrap();
    assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
    assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");

    block_on_ready(storage.create_segment(&writer, 1)).await.unwrap();
    assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0, 1]);

    block_on_ready(storage.seal(&writer, 0)).await.unwrap();
    assert_eq!(block_on_ready(storage.segment_state(&document, 0)).await.unwrap(), WalSegmentState::Sealed);
    assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
    assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");
    assert!(matches!(block_on_ready(storage.append(&writer, 0, pages(b"!"))).await, Err(DbError::InvalidArgument(_))));
    assert!(matches!(block_on_ready(storage.truncate_tail(&writer, 0, 0)).await, Err(DbError::InvalidArgument(_))));

    block_on_ready(storage.delete_segment(&writer, 1)).await.unwrap();
    assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0]);

    assert!(matches!(block_on_ready(storage.append(&writer, 99, pages(b"x"))).await, Err(DbError::NotFound(_))));
    writer.release().await.unwrap();
}

/// 🚪️ Opening a backend while every backend control of the process is taken waits for a release instead of refusing,
/// and the wait is bounded: the in-process `--lib` gate failed unrelated laws with "db I/O backend control capacity
/// exhausted" whenever the laws running beside them held or were still retiring all 64 controls. The capacity is filled
/// with single open attempts (each refused at once when full, the pre-fix behaviour of every constructor); an admitted
/// open then stays pending instead of answering that refusal, is served as soon as one held backend closes, and with
/// the capacity held past [`DB_IO_ADMISSION_WAIT_MS`] it answers the capacity refusal instead of waiting forever.
#[semio_framework_async_macros::async_test]
async fn a_backend_opened_while_every_backend_control_is_taken_waits_for_a_release_instead_of_refusing() {
    if !db_io_driver::process_isolated_law("tests::a_backend_opened_while_every_backend_control_is_taken_waits_for_a_release_instead_of_refusing") {
        return;
    }
    let pool = db_io_test_pool();
    let mut held = Vec::new();
    let refusal = loop {
        match MemoryStorage::open_once(pool.clone()).await {
            Ok(storage) => held.push(storage),
            Err(rejected) => break rejected,
        }
        assert!(held.len() <= DB_IO_BACKEND_CONTROLS, "no more than the declared {DB_IO_BACKEND_CONTROLS} backends are admitted");
    };
    assert!(db_io_backend_admission_saturated(refusal.error()), "a full process refuses a single attempt with its capacity refusal: {:?}", refusal.error());
    drop(refusal);
    assert!(!held.is_empty(), "the law holds at least one backend");
    let mut waiting = Box::pin(MemoryStorage::new(pool.clone()));
    let pending = std::future::poll_fn(|context| std::task::Poll::Ready(waiting.as_mut().poll(context).is_pending())).await;
    assert!(pending, "an admitted open waits while every control is taken instead of answering the refusal");
    held.pop().expect("a held backend").close().await.expect("closing one held backend releases its control");
    let opened = waiting.await.expect("the waiting open is served once a control is released");
    held.push(opened);
    let started = std::time::Instant::now();
    let exhausted = MemoryStorage::new(pool.clone()).await.err().expect("a capacity that never frees ends in its refusal");
    let waited = started.elapsed();
    assert!(db_io_backend_admission_saturated(exhausted.error()), "the bounded wait answers the capacity refusal: {:?}", exhausted.error());
    assert!(waited >= std::time::Duration::from_millis(DB_IO_ADMISSION_WAIT_MS / 2), "the refusal came only after the admission wait: {waited:?}");
    assert!(waited < std::time::Duration::from_millis(DB_IO_ADMISSION_WAIT_MS * 3), "the wait is bounded by the pool clock, not by the next release: {waited:?}");
    drop(exhausted);
    for storage in held {
        storage.close().await.expect("every held backend closes");
    }
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_wal_storage_laws() {
    exercise_wal_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️WalStorage
//#region 🔖️SnapshotStorage
async fn exercise_snapshot_storage(storage: &impl SnapshotStorage) {
    let document: ArtifactId = "doc-snap".into();
    assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), None);

    block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-bytes"))).await.unwrap();
    block_on_ready(storage.write_generation(&document, 1, pages(b"gen-one-bytes"))).await.unwrap();
    assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![0, 1]);
    assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), Some(1));
    assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-bytes");

    block_on_ready(storage.write_generation(&document, 0, pages(b"gen-zero-overwritten"))).await.unwrap();
    assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-overwritten");

    block_on_ready(storage.delete_generation(&document, 0)).await.unwrap();
    assert!(matches!(block_on_ready(storage.read_generation(&document, 0)).await, Err(DbError::NotFound(_))));
    assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![1]);
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_snapshot_storage_laws() {
    exercise_snapshot_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️SnapshotStorage
//#region 🔖️PayloadStorage
async fn exercise_payload_storage(storage: &impl PayloadStorage) {
    let bytes = b"a payload blob that gets content-addressed";
    let hash_a = block_on_ready(storage.put(pages(bytes))).await.unwrap();
    let hash_b = block_on_ready(storage.put(pages(bytes))).await.unwrap();
    assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
    assert_eq!(hash_a, ContentHash(*semio_framework_hash::hash(bytes).as_bytes()));

    assert!(block_on_ready(storage.contains(&hash_a)).await.unwrap());
    assert_eq!(block_on_ready(storage.get(&hash_a)).await.unwrap(), bytes);
    assert_eq!(block_on_ready(storage.len(&hash_a)).await.unwrap(), bytes.len() as u64);

    let other_hash = ContentHash([0xAB; 32]);
    assert!(!block_on_ready(storage.contains(&other_hash)).await.unwrap());
    assert!(matches!(block_on_ready(storage.get(&other_hash)).await, Err(DbError::NotFound(_))));

    block_on_ready(storage.delete(&hash_a)).await.unwrap();
    assert!(!block_on_ready(storage.contains(&hash_a)).await.unwrap());
    assert!(matches!(block_on_ready(storage.get(&hash_a)).await, Err(DbError::NotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_payload_storage_laws() {
    exercise_payload_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️PayloadStorage
//#region 🔖️CatalogStorage
async fn exercise_catalog_storage(storage: &impl CatalogStorage) {
    assert_eq!(block_on_ready(storage.read_root()).await.unwrap(), None);

    let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-v1"))).await.unwrap();
    assert_eq!(epoch_1, EpochFence::INITIAL.next());
    let (bytes, fence) = block_on_ready(storage.read_root()).await.unwrap().unwrap();
    assert_eq!(bytes, b"root-v1");
    assert_eq!(fence, epoch_1);

    // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
    assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, pages(b"root-stale"))).await, Err(DbError::Fenced { .. })));

    let epoch_2 = block_on_ready(storage.cas_root(epoch_1, pages(b"root-v2"))).await.unwrap();
    assert_eq!(epoch_2, epoch_1.next());
    assert_eq!(block_on_ready(storage.read_root()).await.unwrap().unwrap().0, b"root-v2");
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_catalog_storage_laws() {
    exercise_catalog_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️CatalogStorage
//#region 🔖️IndexStorage
async fn exercise_index_storage(storage: &impl IndexStorage) {
    let document: ArtifactId = "doc-index".into();
    block_on_ready(storage.write_run(&document, 0, pages(b"run-zero"))).await.unwrap();
    block_on_ready(storage.write_run(&document, 1, pages(b"run-one"))).await.unwrap();
    assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![0, 1]);
    assert_eq!(block_on_ready(storage.read_run(&document, 1)).await.unwrap(), b"run-one");

    block_on_ready(storage.delete_run(&document, 0)).await.unwrap();
    assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![1]);
    assert!(matches!(block_on_ready(storage.read_run(&document, 0)).await, Err(DbError::NotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_index_storage_laws() {
    exercise_index_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️IndexStorage
//#region 🔖️LeaseStorage
async fn exercise_lease_storage(storage: &impl LeaseStorage) {
    let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).await.unwrap();
    assert_eq!(fence_1, EpochFence::INITIAL);

    // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
    let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).await.unwrap();
    assert_eq!(fence_reacquire, fence_1);

    // A different holder cannot acquire an unexpired lease.
    assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)).await, Err(DbError::Conflict(_))));

    block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).await.unwrap();
    assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)).await, Err(DbError::Unauthorized(_))));

    let mut current = block_on_ready(storage.current("shard-1", 600)).await.unwrap().unwrap();
    assert_eq!(current.holder.as_str(), "node-a");
    assert_eq!(current.fence, fence_1);
    current.close_step();

    // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
    // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
    assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).await.unwrap(), None);
    let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).await.unwrap();
    assert_eq!(fence_2, fence_1.next());

    // The old holder's stale fence is now rejected.
    assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)).await, Err(DbError::Unauthorized(_))));

    block_on_ready(storage.release("shard-1", "node-b", fence_2)).await.unwrap();
    assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).await.unwrap(), None);
    assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)).await, Err(DbError::NotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_lease_storage_laws() {
    exercise_lease_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}
//#endregion 🔖️LeaseStorage
