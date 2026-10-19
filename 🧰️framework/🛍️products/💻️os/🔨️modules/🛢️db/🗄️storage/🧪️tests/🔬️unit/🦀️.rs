use super::*;

#[semio_framework_async_macros::async_test]
async fn db_io_fault_returns_original_typed_pack_error_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🆔️ids/🧪️tests/🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    let row = &fixture["cases"][0];
    let error = semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, row["message"].as_str().unwrap());
    let pointer = error.message.as_ptr();
    let error = DbError::from(pack::PackRefusal::ValueRefusal(error));
    let fault = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, error);
    assert_eq!(fault.cause, DbIoFaultCause::Pack(pack::PackCauseKind::Refusal(semio_framework_value::ValueRefusalKind::InvalidValue)));
    let DbError::Pack(pack::PackError::Refusal(pack::PackRefusal::ValueRefusal(error))) = fault.into_db_error() else { panic!("original typed Pack error") };
    assert_eq!(error.message.as_ptr(), pointer);
    assert_eq!(serde_json::json!({"message":error.message}), serde_json::json!({"message":row["message"]}));
    eprintln!("[DEBUG] DB I/O terminal returns the actual Pack refusal with its original owned message allocation");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_retirement_releases_one_original_semantic_allocation_at_a_time() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🆔️ids/🧪️tests/🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    let mut message = String::with_capacity(256);
    message.push_str(fixture["cases"][0]["message"].as_str().unwrap());
    let capacity = message.capacity();
    let error = semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message);
    let mut owner = Some(DbError::from(pack::PackRefusal::ValueRefusal(error)));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(progressed);
    assert!(owner.is_some());
    assert_eq!((born, released), (0, capacity));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(progressed);
    assert!(owner.is_none());
    assert_eq!((born, released), (0, 0));
    let mut message = String::with_capacity(DB_IO_STRUCTURED_FAULT_CLOSE_GRANT.maximum_release_bytes + fixture["retirement"]["deniedExtraBytes"].as_u64().unwrap() as usize);
    message.push_str(fixture["cases"][0]["message"].as_str().unwrap());
    let pointer = message.as_ptr();
    let capacity = message.capacity();
    let mut owner = Some(DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(!progressed);
    assert_eq!(born, fixture["retirement"]["deniedNewBytes"].as_u64().unwrap() as usize);
    assert_eq!(released, fixture["retirement"]["deniedReleasedBytes"].as_u64().unwrap() as usize);
    let Some(DbError::Protocol(protocol::ProtocolError::Malformed { what, offset, detail })) = owner else { panic!("original oversized cause") };
    assert_eq!((what, offset, detail.as_ptr(), detail.capacity()), ("fixture", 42, pointer, capacity));
    assert_eq!(serde_json::json!({"message":detail}), serde_json::json!({"message":fixture["cases"][0]["message"]}));
    eprintln!("[DEBUG] DB structured fault obeys independent finite parent release authority and preserves one-over original causes with heap0");
}

struct FaultCustodyTransportPolicy;
impl pack::PackTransportPolicy for FaultCustodyTransportPolicy {
    fn checkpoint(&self, _: pack::PackTransportProgress) -> Result<(), semio_framework_value::ValueError> { Ok(()) }
    fn progress(&self, _: pack::PackTransportProgress) {}
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_maintenance_serves_every_tier_while_retaining_unsupported_original() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(4096, 0), FaultCustodyTransportPolicy).ok().unwrap();
    let original = context.reserve::<std::io::Error>(pack::PackTransportCategory::NativeIo, pack::PackRetryDisposition::Never).unwrap().publish(std::io::Error::from(std::io::ErrorKind::NotFound));
    let pointer = original.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error;
    let mut refused = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbError::from(pack::PackError::TransportFailure(original)));
    while refused.detail.close_step() {}
    let fault = || {
        let mut message = String::with_capacity(fixture["fairness"]["releaseCapacity"].as_u64().unwrap() as usize);
        message.push_str("original fixture cause");
        let mut fault = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
        while fault.detail.close_step() {}
        fault
    };
    {
        let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(refused));
        owners[1] = Some(DbIoLostOwner::Fault(fault()));
    }
    {
        let mut owners = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(fault()));
    }
    {
        let mut owners = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(fault()));
    }
    *DB_IO_LOST_OWNER_CURSOR.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = 0;
    for key in ["firstCycleReleasedBytes", "secondCycleReleasedBytes"] {
        let mut actual = Vec::new();
        for expected in fixture["fairness"][key].as_array().unwrap() {
            let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
            assert!(progressed.unwrap());
            assert_eq!(born, fixture["fairness"]["newBytes"].as_u64().unwrap() as usize);
            assert_eq!(released, expected.as_u64().unwrap() as usize);
            actual.push(released);
        }
        assert_eq!(serde_json::json!(actual), fixture["fairness"][key]);
    }
    let Some(DbIoLostOwner::Fault(fault)) = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[0].take() else { panic!("retained original fault") };
    let DbError::Pack(pack::PackError::TransportFailure(original)) = fault.into_db_error() else { panic!("original transport handback") };
    assert_eq!(original.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error, pointer);
    assert!(!db_io_lost_owner_close_step().unwrap());
    eprintln!("[DEBUG] DB fault round-robin serves primary, overflow and quarantine with heap0 while preserving unsupported original source identity");
}

#[derive(Debug)]
struct FaultCustodySource { message: String, drops: Arc<std::sync::atomic::AtomicUsize> }
impl std::fmt::Display for FaultCustodySource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) }
}
impl std::error::Error for FaultCustodySource {}
impl Drop for FaultCustodySource {
    fn drop(&mut self) { self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_prebirth_custody_preserves_full_denied_cancelled_and_stale_originals() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let expected = &fixture["reservation"];
    let cancellation = db_io_fault_custody_reserve().unwrap();
    let cancelled_slot = cancellation.slot;
    let cancelled_generation = cancellation.generation;
    let (_, born, released) = crate::test_allocation::observe_backing(|| drop(cancellation));
    assert_eq!((born, released), (0, 0));
    let custody = db_io_fault_custody_reserve().unwrap();
    assert_eq!(custody.slot, cancelled_slot);
    assert!(custody.generation > cancelled_generation);
    let (slot, generation) = (custody.slot, custody.generation);
    let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(4096, 0), FaultCustodyTransportPolicy).ok().unwrap();
    let mut message = String::with_capacity(256);
    message.push_str("original retained source");
    let message_pointer = message.as_ptr();
    let message_capacity = message.capacity();
    let original = context.reserve::<FaultCustodySource>(pack::PackTransportCategory::NativeIo, pack::PackRetryDisposition::Transient).unwrap().publish(FaultCustodySource { message, drops: drops.clone() });
    let pointer = original.source_error().downcast_ref::<FaultCustodySource>().unwrap() as *const FaultCustodySource;
    let fault = db_io_task_fault(custody, DbIoFaultKind::Backend, DbError::from(pack::PackError::TransportFailure(original)));
    let sentinel = || DbIoLostOwner::Fault(db_io_literal_fault(db_io_fault_custody_reserve().unwrap(), DbIoFaultKind::Backend, DbIoFaultCause::Internal, ""));
    for cell in DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    for cell in DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    for cell in DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    let mut held = Vec::new();
    for _ in 0..DB_IO_FAULT_CUSTODY_SLOTS { match db_io_fault_custody_reserve() { Ok(custody) => held.push(custody), Err(DbError::LimitExceeded("db_io fault custody")) => break, Err(error) => panic!("unexpected admission refusal: {error:?}") } }
    let mut producer_calls = 0;
    let (denied, born, released) = crate::test_allocation::observe_backing(|| db_io_fault_custody_reserve().map(|custody| { producer_calls += 1; custody }));
    assert!(matches!(denied, Err(DbError::LimitExceeded("db_io fault custody"))));
    assert_eq!((born, released, producer_calls), (expected["deniedNewBytes"].as_u64().unwrap() as usize, expected["deniedReleasedBytes"].as_u64().unwrap() as usize, expected["producerCallsAtRefusal"].as_u64().unwrap() as usize));
    let (_, born, released) = crate::test_allocation::observe_backing(|| drop(fault));
    assert_eq!((born, released), (0, 0));
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), expected["sourceDropsBeforeHandoff"].as_u64().unwrap() as usize);
    let (stale, born, released) = crate::test_allocation::observe_backing(|| { let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner); db_io_fault_custody_take(&mut arena, slot, generation + 1) });
    assert!(matches!(stale, Err(DbError::StaleGeneration { .. })));
    assert_eq!((born, released), (0, 0));
    assert!(matches!(db_io_fault_custody_release(slot, generation + 1), Err(DbError::StaleGeneration { .. })));
    assert!(matches!(db_io_fault_custody_release(slot, generation), Err(DbError::LimitExceeded("db_io fault custody remains occupied"))));
    let fault = { let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner); let fault = db_io_fault_custody_take(&mut arena, slot, generation).unwrap().unwrap(); assert!(db_io_fault_custody_take(&mut arena, slot, generation).unwrap().is_none()); fault };
    let (error, born, released) = crate::test_allocation::observe_backing(|| fault.into_db_error());
    assert_eq!((born, released), (0, 0));
    let DbError::Pack(pack::PackError::TransportFailure(original)) = error else { panic!("exact original source handoff") };
    let source = original.source_error().downcast_ref::<FaultCustodySource>().unwrap();
    assert_eq!((source as *const FaultCustodySource, source.message.as_ptr(), source.message.capacity()), (pointer, message_pointer, message_capacity));
    assert_eq!(original.retry(), pack::PackRetryDisposition::Transient);
    assert_eq!(serde_json::json!({"message":source.message}), serde_json::json!({"message":"original retained source"}));
    assert!(matches!(db_io_fault_custody_release(slot, generation), Err(DbError::StaleGeneration { .. })));
    let reused = db_io_fault_custody_reserve().unwrap();
    assert_eq!(reused.slot, slot);
    assert!(reused.generation > generation);
    drop(reused);
    drop(held);
    for _ in 0..2 * (DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS) { if !db_io_lost_owner_close_step().unwrap() { break; } }
    assert!(!db_io_lost_owner_close_step().unwrap());
    let semantic_custody = db_io_fault_custody_reserve().unwrap();
    let semantic_slot = semantic_custody.slot;
    let semantic_generation = semantic_custody.generation;
    let mut semantic_message = String::with_capacity(256);
    semantic_message.push_str("original funded semantic cause");
    let semantic = db_io_task_fault(semantic_custody, DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: semantic_message }));
    drop(semantic);
    *DB_IO_LOST_OWNER_CURSOR.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS + semantic_slot;
    let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
    assert!(progressed.unwrap());
    assert_eq!((born, released), (0, 256));
    let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
    let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
    assert!(progressed.unwrap());
    assert_eq!((born, released), (0, 0));
    assert_eq!(DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire), epoch + 1);
    {
        let arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(arena.cells[semantic_slot].generation, semantic_generation);
        assert!(!arena.cells[semantic_slot].reserved && arena.cells[semantic_slot].owner.is_none());
    }
    drop(original);
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), expected["sourceDropsAfterExplicitDrop"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] DB full custody admission refuses before producer with heap0; cancellation, stale access and exactly-once original handoff preserve source pointer, capacity and retry");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_refused_task_handback_retains_original_custody_after_cause_close_or_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    for transfer in [false, true] {
        let custody = db_io_fault_custody_reserve().unwrap();
        let (slot, generation) = (custody.slot, custody.generation);
        let credit = DbIoCredit { pages: 0, bytes: size_of::<DbIoFault>() as u64 + 256, items: 1, controls: 1 };
        let operation = db_io_operation_reserve(credit).unwrap();
        db_io_operation_add_result_lease(operation).unwrap();
        let index = db_io_operation_owner_slot(operation).unwrap();
        db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[index].result_leases = 0;
        let mut message = String::with_capacity(256);
        message.push_str("original refused handback cause");
        let pointer = message.as_ptr();
        let capacity = message.capacity();
        let mut fault = db_io_task_fault(custody, DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
        fault.result_handback = Some(DbIoResultHandback { handle: DbIoTaskHandle { slot: 0, generation: 0, operation }, retained_credit: credit });
        let original = if transfer {
            let original = fault.into_db_error();
            let DbError::Protocol(protocol::ProtocolError::Malformed { what, offset, detail }) = &original else { panic!("exact transferred cause") };
            assert_eq!((*what, *offset, detail.as_ptr(), detail.capacity()), ("fixture", 42, pointer, capacity));
            Some(original)
        } else {
            assert!(fault.close_step());
            assert!(fault.close_step());
            assert!(!fault.close_step());
            let retained = fault.custody.as_ref().unwrap();
            assert_eq!((retained.slot, retained.generation), (slot, generation));
            assert!(fault.structured.is_none());
            drop(fault);
            None
        };
        let mut fault = {
            let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = arena.cells[slot].owner.as_ref().unwrap();
            assert!(owner.result_handback.is_some());
            assert_eq!(owner.custody.as_ref().unwrap().generation, generation);
            db_io_fault_custody_take(&mut arena, slot, generation).unwrap().unwrap()
        };
        assert_eq!(fixture["reservation"][if transfer { "explicitCauseHandbackRetainsRefusedTaskCustody" } else { "failedTaskHandbackRetainsCustody" }], serde_json::json!(fault.custody.is_some()));
        db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[index].result_leases = 1;
        assert!(fault.close_step());
        assert!(fault.close_step());
        assert!(!fault.close_step());
        assert!(fault.custody.is_none());
        assert!(!DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).cells[slot].reserved);
        drop(original);
    }
    eprintln!("[DEBUG] DB refused aggregate handback retains exact original reservation after semantic closure and explicit cause transfer; repaired handback releases once outside custody guard");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_full_custody_refuses_actual_task_admission_before_backend_effects() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let expected = &fixture["task"];
    let storage = MemoryStorage::new(db_io_test_pool()).await.unwrap();
    let document = DbIoText::try_from_str(expected["document"].as_str().unwrap()).unwrap();
    let payload: Vec<u8> = expected["payload"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
    let input = pages(&payload);
    let identity = input.pages[0].as_ref().map(|page| (page.slot, page.generation, page.operation, page.used));
    let pointer = input.page(0).unwrap().as_ptr();
    let operation = input.operation;
    let task = DbIoTask::SnapshotWrite { backend: storage.control, document, generation: expected["generation"].as_u64().unwrap(), input };
    let mut held = Vec::new();
    for _ in 0..DB_IO_FAULT_CUSTODY_SLOTS { match db_io_fault_custody_reserve() { Ok(custody) => held.push(custody), Err(DbError::LimitExceeded("db_io fault custody")) => break, Err(error) => panic!("unexpected admission refusal: {error:?}") } }
    let before = db_io_census();
    let task_arena = { let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner); (arena.free_read, arena.free_len, arena.next_generation) };
    let (refused, born, released) = crate::test_allocation::observe_backing(|| db_io_allocate_task(task));
    assert_eq!((born, released), (expected["newBytesOnRefusal"].as_u64().unwrap() as usize, expected["releasedBytesOnRefusal"].as_u64().unwrap() as usize));
    let (error, mut task) = refused.err().expect("actual task refused before admission effects");
    assert!(matches!(error, DbError::LimitExceeded("db_io fault custody")));
    let DbIoTask::SnapshotWrite { backend, document, generation, input } = &task else { panic!("exact original task") };
    assert_eq!(*backend, storage.control);
    assert_eq!(*generation, expected["generation"].as_u64().unwrap());
    assert_eq!(document.as_str(), expected["document"].as_str().unwrap());
    assert_eq!((input.operation, input.page(0).unwrap().as_ptr(), input.pages[0].as_ref().map(|page| (page.slot, page.generation, page.operation, page.used))), (operation, pointer, identity));
    assert_eq!(serde_json::json!(input.page(0).unwrap()), expected["payload"]);
    let after = db_io_census();
    assert_eq!((after.tasks, after.steps, after.turns), (before.tasks, before.steps, before.turns));
    assert_eq!({ let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner); (arena.free_read, arena.free_len, arena.next_generation) }, task_arena);
    drop(held);
    while task.close_step().unwrap().is_some() {}
    assert_eq!(storage.latest_generation(&ArtifactId::from(expected["document"].as_str().unwrap())).await.unwrap(), None);
    storage.close().await.unwrap();
    eprintln!("[DEBUG] DB full custody refuses actual SnapshotWrite admission with heap0, unchanged original page pointer/generation/operation/backend, unchanged task arena and no producer steps");
}

use db_io_core::{CatalogStorage as _, DB_IO_PAGE_BYTES, DbIoPageWriter, DbIoPages, IndexStorage as _, LeaseStorage as _, PayloadStorage as _, WalSegmentState, WalStorage as _};
use db_io_driver::{block_on_ready, db_io_test_pool, poll_once};
use db_storage_fs::FsStorage;
use db_storage_memory::MemoryStorage;
use db_io_core::DbIoFaultKind;
use db_io_core::DbIoLostOwner;
use db_io_core::DbIoFaultCause;
use db_io_core::DbIoText;
use db_io_core::DbIoTask;
use db_io_core::DbIoFault;

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
