use super::*;

use db_io_core::{CatalogStorage as _, DB_IO_PAGE_BYTES, DbIoPageWriter, DbIoPages, IndexStorage as _, LeaseStorage as _, PayloadStorage as _, WalSegmentState, WalStorage as _};
use db_io_driver::{block_on_ready, db_io_test_pool, poll_once};
use db_storage_fs::FsStorage;
use db_storage_memory::MemoryStorage;

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("test storage pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}
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
static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
/// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
/// test helper convention. The test-owned process pool drives the same retained operation path.
async fn fs_scratch(name: &str) -> FsStorage {
    fs_scratch_at(name).await.0
}

async fn fs_scratch_at(name: &str) -> (FsStorage, std::path::PathBuf) {
    let pid = std::process::id();
    let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
    (poll_once(FsStorage::open(db_io_test_pool(), &dir)).await.unwrap(), dir)
}
//#endregion 🔖️Fs
