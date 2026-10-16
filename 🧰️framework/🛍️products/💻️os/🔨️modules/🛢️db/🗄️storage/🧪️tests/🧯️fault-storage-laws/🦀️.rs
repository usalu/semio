use super::*;

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).expect("testkit writer admitted");
    for fragment in bytes.chunks(db_io_core::DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}
//#region 🔖️FaultStorage
#[semio_framework_async_macros::async_test]
async fn fault_storage_passes_through_untouched_when_no_fault_is_scripted() {
    let inner = Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let faulted = FaultStorage::new(inner).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.append(&writer, 0, pages(b"hello"))).unwrap(), 5);
    assert_eq!(db_actor::block_on(faulted.read(&document, 0, pack::ByteRange { offset: 0, len: 5 })).unwrap(), b"hello");
    assert_eq!(faulted.append_calls().await, 1);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fail_nth_write_fails_exactly_once_at_the_scripted_call() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { fail_nth_write: Some(2), ..FaultScript::default() }).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"first"))).is_ok(), "call #1 must succeed");
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"second"))).is_err(), "call #2 must be the injected failure");
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"third"))).is_ok(), "call #3 must succeed again — the script fires exactly once");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_torn_write_forwards_only_the_kept_prefix() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { torn_write_at: Some((1, 3)), ..FaultScript::default() }).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    let new_len = db_actor::block_on(faulted.append(&writer, 0, pages(b"hello world"))).unwrap();
    assert_eq!(new_len, 3, "a torn write must report only the bytes that actually landed");
    assert_eq!(db_actor::block_on(faulted.read(&document, 0, pack::ByteRange { offset: 0, len: 3 })).unwrap(), b"hel");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fsync_lies_never_delegates_to_the_inner_backend() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok());
    assert_eq!(faulted.sync_delegated_calls().await, 1, "an unfaulted sync must delegate");

    faulted.set_script(FaultScript { fsync_lies: true, ..FaultScript::default() }).await;
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok(), "a lying fsync must still report success");
    assert_eq!(faulted.sync_delegated_calls().await, 1, "a lying fsync must never actually delegate");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fail_nth_sync_fails_once_after_the_preceding_append() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-sync-fault".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.append(&writer, 0, pages(b"committed-before-sync"))).unwrap(), 21);
    faulted.set_script(FaultScript { fail_nth_sync: Some(1), ..FaultScript::default() }).await;
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_err());
    assert_eq!(faulted.sync_calls().await, 1);
    assert_eq!(faulted.sync_delegated_calls().await, 0);
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok());
    assert_eq!(faulted.sync_calls().await, 2);
    assert_eq!(faulted.sync_delegated_calls().await, 1);
    assert_eq!(db_actor::block_on(faulted.segment_len(&document, 0)).unwrap(), 21);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_segment_state_is_observational_and_counter_neutral() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-state".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.segment_state(&document, 0)).unwrap(), WalSegmentState::Active);
    db_actor::block_on(faulted.seal(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.segment_state(&document, 0)).unwrap(), WalSegmentState::Sealed);
    assert_eq!(faulted.append_calls().await, 0);
    assert_eq!(faulted.sync_delegated_calls().await, 0);
    assert_eq!(faulted.cas_calls().await, 0);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_cas_conflict_injection_rejects_without_touching_the_inner_root() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { cas_conflict_nth: Some(1), ..FaultScript::default() }).await;
    let result = db_actor::block_on(faulted.cas_root(EpochFence::INITIAL, pages(b"attempt")));
    assert!(matches!(result, Err(DbError::Fenced { .. })), "the scripted call must be rejected as fenced");
    assert!(db_actor::block_on(faulted.read_root()).unwrap().is_none(), "the injected conflict must never have reached the inner backend's root");
}
//#endregion 🔖️FaultStorage
