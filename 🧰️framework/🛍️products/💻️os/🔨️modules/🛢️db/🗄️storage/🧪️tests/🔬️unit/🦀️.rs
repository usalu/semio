
use super::*;

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

#[semio_framework_async_macros::async_test]
async fn memory_storage_satisfies_wal_storage_laws() {
    exercise_wal_storage(&MemoryStorage::new(db_io_test_pool()).await.unwrap()).await;
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_wal_storage_laws() {
    exercise_wal_storage(&fs_scratch("wal_laws").await).await;
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

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_snapshot_storage_laws() {
    exercise_snapshot_storage(&fs_scratch("snapshot_laws").await).await;
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

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_payload_storage_laws() {
    exercise_payload_storage(&fs_scratch("payload_laws").await).await;
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

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_catalog_storage_laws() {
    exercise_catalog_storage(&fs_scratch("catalog_laws").await).await;
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

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_index_storage_laws() {
    exercise_index_storage(&fs_scratch("index_laws").await).await;
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

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_satisfies_lease_storage_laws() {
    exercise_lease_storage(&fs_scratch("lease_laws").await).await;
}
//#endregion 🔖️LeaseStorage

//#region 🔖️DbBackend
#[semio_framework_async_macros::async_test]
async fn memory_storage_db_backend_accessors_and_capabilities() {
    let storage: DbBackend = DbBackend::Memory(MemoryStorage::new(db_io_test_pool()).await.unwrap());
    let document: ArtifactId = "doc-umbrella".into();
    let writer = block_on_ready(poll_once(storage.wal()).await.acquire_writer(&document)).await.unwrap();
    block_on_ready(poll_once(storage.wal()).await.create_segment(&writer, 0)).await.unwrap();
    assert_eq!(block_on_ready(poll_once(storage.wal()).await.segment_state(&document, 0)).await.unwrap(), WalSegmentState::Active);
    block_on_ready(poll_once(storage.wal()).await.seal(&writer, 0)).await.unwrap();
    assert_eq!(block_on_ready(poll_once(storage.wal()).await.segment_state(&document, 0)).await.unwrap(), WalSegmentState::Sealed);
    writer.release().await.unwrap();
    block_on_ready(poll_once(storage.catalog()).await.cas_root(EpochFence::INITIAL, pages(b"root"))).await.unwrap();

    let capabilities = poll_once(storage.capabilities()).await;
    assert!(!capabilities.durable);
    assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
    assert!(capabilities.supports_cas);
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_db_backend_accessors_and_capabilities() {
    let storage: DbBackend = DbBackend::Fs(fs_scratch("umbrella").await);
    let document: ArtifactId = "doc-umbrella".into();
    block_on_ready(poll_once(storage.index()).await.write_run(&document, 0, pages(b"run"))).await.unwrap();
    assert_eq!(block_on_ready(poll_once(storage.index()).await.read_run(&document, 0)).await.unwrap(), b"run");

    let capabilities = poll_once(storage.capabilities()).await;
    assert!(capabilities.durable);
    assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
    assert!(capabilities.supports_fsync);
}
//#endregion 🔖️DbBackend

//#region 🔖️Fs
#[cfg(feature = "fs")]
static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// @emoji 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
/// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
/// test helper convention. The test-owned process pool drives the same retained operation path.
#[cfg(feature = "fs")]
async fn fs_scratch(name: &str) -> FsStorage {
    fs_scratch_at(name).await.0
}

#[cfg(feature = "fs")]
async fn fs_scratch_at(name: &str) -> (FsStorage, std::path::PathBuf) {
    let pid = std::process::id();
    let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
    (poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap(), dir)
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_stale_seal_marker_does_not_resurrect_missing_segment() {
    let (storage, root) = fs_scratch_at("stale_wal_marker").await;
    let document: ArtifactId = "stale-marker".into();
    let writer = block_on_ready(storage.acquire_writer(&document)).await.unwrap();
    block_on_ready(storage.create_segment(&writer, 0)).await.unwrap();
    block_on_ready(storage.seal(&writer, 0)).await.unwrap();
    std::fs::remove_file(root.join("wal/stale-marker/segment-00000000000000000000.bin")).unwrap();
    assert!(matches!(block_on_ready(storage.segment_state(&document, 0)).await, Err(DbError::NotFound(_))));
    writer.release().await.unwrap();
    storage.close().await.unwrap();
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_canonical_alias_writer_fences_all_six_mutations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["mutations"], serde_json::json!(["create", "append", "sync", "seal", "truncate", "delete"]));
    let pid = std::process::id();
    let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!("db_storage_test_writer_alias_{pid}_{counter}"));
    let alias = root.join(".");
    let storage = poll_once(FsStorage::open(db_io_test_pool(), &root)).await.unwrap();
    let contender = poll_once(FsStorage::open(db_io_test_pool(), &alias)).await.unwrap();
    let document: ArtifactId = "canonical-alias".into();
    let writer = block_on_ready(storage.acquire_writer(&document)).await.unwrap();
    assert!(matches!(block_on_ready(contender.acquire_writer(&document)).await, Err(DbError::Conflict(_))));

    let foreign_document: ArtifactId = "foreign-backend".into();
    let foreign = block_on_ready(contender.acquire_writer(&foreign_document)).await.unwrap();
    assert!(matches!(block_on_ready(storage.create_segment(&foreign, 9)).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.append(&foreign, 9, pages(b"x"))).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.sync(&foreign, 9, DurabilityClass::Fsync)).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.seal(&foreign, 9)).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.truncate_tail(&foreign, 9, 0)).await, Err(DbError::Fenced { .. })));
    assert!(matches!(block_on_ready(storage.delete_segment(&foreign, 9)).await, Err(DbError::Fenced { .. })));
    assert!(block_on_ready(storage.list_segments(&foreign_document)).await.unwrap().is_empty());
    foreign.release().await.unwrap();

    block_on_ready(storage.create_segment(&writer, 0)).await.unwrap();
    let bytes = vec![0x5a; DB_IO_PAGE_BYTES + 1];
    assert_eq!(block_on_ready(storage.append(&writer, 0, pages(&bytes))).await.unwrap(), bytes.len() as u64);
    block_on_ready(storage.sync(&writer, 0, DurabilityClass::Fsync)).await.unwrap();
    block_on_ready(storage.truncate_tail(&writer, 0, 1)).await.unwrap();
    block_on_ready(storage.seal(&writer, 0)).await.unwrap();
    block_on_ready(storage.delete_segment(&writer, 0)).await.unwrap();
    writer.release().await.unwrap();
    let next = block_on_ready(contender.acquire_writer(&document)).await.unwrap();
    next.release().await.unwrap();
    let sidecars = std::fs::read_dir(root.join(".semio-wal-writer")).unwrap().count();
    assert_eq!(sidecars, 2);
    contender.close().await.unwrap();
    storage.close().await.unwrap();
    assert_eq!(std::fs::read_dir(root.join(".semio-wal-writer")).unwrap().count(), sidecars);
    eprintln!("[DEBUG] canonical filesystem aliases shared stable sidecars, fenced all six foreign-backend mutations before effect, and retained lock inodes after terminal release");
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_rejects_unsafe_path_components() {
    let storage = fs_scratch("path_safety").await;
    let traversal_document: ArtifactId = "../escape".into();
    let traversal = block_on_ready(storage.acquire_writer(&traversal_document)).await.unwrap();
    assert!(matches!(block_on_ready(storage.create_segment(&traversal, 0)).await, Err(DbError::InvalidArgument(_))));
    traversal.release().await.unwrap();

    let separator_document: ArtifactId = "sub/dir".into();
    let separator = block_on_ready(storage.acquire_writer(&separator_document)).await.unwrap();
    assert!(matches!(block_on_ready(storage.create_segment(&separator, 0)).await, Err(DbError::InvalidArgument(_))));
    separator.release().await.unwrap();

    let empty_document: ArtifactId = "".into();
    assert!(matches!(block_on_ready(storage.acquire_writer(&empty_document)).await, Err(DbError::InvalidArgument(_))));
    storage.close().await.unwrap();
}

#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn fs_storage_write_atomic_survives_reopen_across_instances() {
    let pid = std::process::id();
    let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("db_storage_test_reopen_{pid}_{counter}"));

    {
        let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
        let document: ArtifactId = "doc-reopen".into();
        block_on_ready(storage.write_generation(&document, 0, pages(b"persisted across reopen"))).await.unwrap();
    }
    {
        let storage = poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap();
        let document: ArtifactId = "doc-reopen".into();
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"persisted across reopen");
    }
}
//#endregion 🔖️Fs
