
use super::*;
use db_storage::{MemoryStorage, PayloadStorage as _, WalStorage as _};
use db_wal::{WalPayloadRef, WalRecord};
use {DurabilityClass, Frontier};

fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test compaction writer admitted");
    for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

async fn doc(id: &str) -> ArtifactId {
    ArtifactId::from(id)
}

async fn frontier(document: &ArtifactId, head_seq: u64) -> Frontier {
    Frontier { document: document.clone(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }
}

async fn sample_body(head_seq: u64) -> db_snapshot::SnapshotBody {
    db_snapshot::SnapshotBody { head_seq, commit_seq: head_seq, epoch: 0, chain_hash: [0u8; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: vec![], created_at_ms: head_seq * 1_000 }
}

async fn wal_bytes(source: &[u8]) -> db_wal::WalBytes {
    let mut control = db_wal::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    db_wal::WalBytes::try_admit(source.to_vec(), 1024 * 1024, &mut control).await.unwrap()
}

async fn submit_record(storage: &MemoryStorage, wal: &mut db_wal::ArtifactWal, record: WalRecord, now_ms: u64) {
    let mut records = db_wal::WalRecordBatch::new();
    assert!(records.push(record).is_ok());
    wal.submit(storage, &records, DurabilityClass::Fsync, now_ms).await.unwrap();
    while records.close_step().unwrap() {}
}

fn committed_compaction_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-effects/🔣️.json")).unwrap()
}

async fn append_fixture_record(writer: &mut protocol::SprWriter<Vec<u8>>, mut record: WalRecord) {
    let (kind, critical, payload) = record.encode().await;
    let mut frame = writer.begin_identity_record(kind, critical, payload.len()).await.unwrap();
    frame.write_fragment(&payload).await.unwrap();
    frame.finish().await.unwrap();
    while record.close_step().unwrap() {}
}

async fn append_committed_compaction_segment(storage: &MemoryStorage, document: &ArtifactId, row: &serde_json::Value, previous: Option<[u8; 32]>, payloads: &[(&str, ContentHash)]) -> [u8; 32] {
    let index = row["index"].as_u64().unwrap();
    let options = protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 };
    let mut writer = protocol::SprWriter::begin(Vec::new(), &options).await.unwrap();
    append_fixture_record(&mut writer, WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash: previous }).await;
    writer.commit().await.unwrap();
    for transaction in row["transactions"].as_array().unwrap() {
        let tx_id = transaction["id"].as_u64().unwrap();
        append_fixture_record(&mut writer, WalRecord::TxBegin { tx_id }).await;
        for record in transaction["records"].as_array().unwrap() {
            let record = match record["kind"].as_str().unwrap() {
                "frontier" => WalRecord::Frontier(frontier(document, record["headSeq"].as_u64().unwrap()).await),
                "snapshot" => WalRecord::SnapshotPub { generation: 1, frontier: frontier(document, record["headSeq"].as_u64().unwrap()).await },
                "payload" => {
                    let name = record["payload"].as_str().unwrap();
                    let hash = payloads.iter().find_map(|(candidate, hash)| (*candidate == name).then_some(*hash)).unwrap_or_else(|| panic!("unknown committed compaction payload {name}"));
                    WalRecord::Payload(WalPayloadRef::CasRef(hash))
                }
                other => panic!("unknown committed compaction record {other}"),
            };
            append_fixture_record(&mut writer, record).await;
        }
        let record_count = transaction["records"].as_array().unwrap().len() as u32;
        let terminal = match transaction["outcome"].as_str().unwrap() {
            "commit" => WalRecord::TxCommit { tx_id, record_count },
            "abort" => WalRecord::TxAbort { tx_id },
            other => panic!("unknown committed compaction outcome {other}"),
        };
        append_fixture_record(&mut writer, terminal).await;
        writer.commit().await.unwrap();
    }
    let bytes = writer.into_sink().await;
    let mut verification = protocol::format::retained::RetainedSprVerification::new(bytes.len() as u64, protocol::format::retained::RetainedSprLimits::default()).unwrap();
    let mut fuel = bytes.len();
    assert_eq!(verification.push(&bytes, &mut fuel).unwrap(), bytes.len());
    let span = verification.finish().unwrap();
    assert_eq!(span.tail(), 0);
    let chain = *span.chain();
    let writer_permit = storage.acquire_writer(document).await.unwrap();
    storage.create_segment(&writer_permit, index).await.unwrap();
    assert_eq!(storage.append(&writer_permit, index, pages(&bytes)).await.unwrap(), bytes.len() as u64);
    if row["state"] == "sealed" {
        storage.seal(&writer_permit, index).await.unwrap();
    }
    writer_permit.release().await.unwrap();
    chain
}

async fn index_put(handle: &db_index::IndexHandle<'_, MemoryStorage>, key: &[u8], value: &[u8]) {
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = db_index::IndexCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let key = db_index::IndexBytes::try_admit(key.to_vec(), 1024 * 1024, &mut control).await.unwrap();
    let value = db_index::IndexBytes::try_admit(value.to_vec(), 1024 * 1024, &mut control).await.unwrap();
    handle.put(key, value, &mut control).await.unwrap();
}

async fn state_page(source: &[u8]) -> db_state::Page {
    db_state::Page::try_from_pages(pages(source)).await.unwrap()
}

//#region 🔖️Budget
#[semio_framework_async_macros::async_test]
async fn compaction_budget_default_is_finite_and_unlimited_is_boundless() {
    let default = CompactionBudget::default();
    assert!(default.max_wal_segments > 0 && default.max_wal_segments < u64::MAX);
    assert!(default.max_snapshot_generations > 0 && default.max_snapshot_generations < u64::MAX);
    assert!(default.max_payloads > 0 && default.max_payloads < u64::MAX);

    let unlimited = CompactionBudget::unlimited();
    assert_eq!(unlimited.max_wal_segments, u64::MAX);
    assert_eq!(unlimited.max_snapshot_generations, u64::MAX);
    assert_eq!(unlimited.max_payloads, u64::MAX);
}

fn held_compaction_worker_pool() -> (Arc<WorkerPool>, Arc<std::sync::atomic::AtomicBool>) {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let entered = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let held = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let worker_entered = entered.clone();
    let worker_held = held.clone();
    pool.try_submit(
        Lane::Maintenance,
        Box::new(move || {
            worker_entered.store(true, std::sync::atomic::Ordering::Release);
            while worker_held.load(std::sync::atomic::Ordering::Acquire) {
                std::thread::yield_now();
            }
        }),
    )
    .ok()
    .expect("compaction blocker admission");
    while !entered.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }
    (pool, held)
}

async fn retained_compaction_storage() -> Arc<db_storage::DbBackend> {
    Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms() {
    let (pool, held) = held_compaction_worker_pool();
    let storage = retained_compaction_storage().await;
    let storage_identity = Arc::as_ptr(&storage) as usize;
    let document = ArtifactId(String::from("p1y-handoff-cancel"));
    let document_identity = document.0.as_ptr();
    let holder = db_storage::DbIoText::try_from_str("p1y-holder").unwrap();
    let started = std::time::Instant::now();
    let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, document, holder, false, CompactionBudget::default(), 0).unwrap();
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    let generation = future.generation();
    let state = future.state.as_ref().unwrap().clone();
    assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseCompactionDriverAuthority::Queued as u8);
    future.cancel();
    held.store(false, std::sync::atomic::Ordering::Release);
    let result = future.await.unwrap();
    let (storage, document, mut holder, report) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
    assert_eq!(document.0.as_ptr(), document_identity);
    assert_eq!(holder.as_str(), "p1y-holder");
    assert_eq!(report, Err(DbError::Closed));
    assert!(holder.close_step());
    assert_eq!(state.progress(), DatabaseCompactionProgress::Cancelled);
    assert!(!database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().flatten().any(|owner| owner.generation == generation));
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once() {
    let (pool, held) = held_compaction_worker_pool();
    let storage = retained_compaction_storage().await;
    let storage_identity = Arc::as_ptr(&storage) as usize;
    let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-deadline")), db_storage::DbIoText::try_from_str("deadline-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    state.deadline_ms.store(0, std::sync::atomic::Ordering::Release);
    state.deadline_callback();
    std::task::Wake::wake_by_ref(&state);
    held.store(false, std::sync::atomic::Ordering::Release);
    let result = future.await.unwrap();
    let (storage, document, mut holder, report) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
    assert_eq!(document.0, "p1y-deadline");
    assert_eq!(report, Err(DbError::Timeout("database compaction deadline".to_string())));
    assert!(holder.close_step());
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseCompactionDriverAuthority::Idle as u8);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority() {
    let (pool, held) = held_compaction_worker_pool();
    let mut admitted = Vec::with_capacity(DATABASE_COMPACTION_SLOTS);
    for index in 0..DATABASE_COMPACTION_SLOTS {
        admitted.push(
            DatabaseCompactionFuture::try_submit(pool.clone(), retained_compaction_storage().await, ArtifactId(format!("p1y-max-{index}")), db_storage::DbIoText::try_from_str("max-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap(),
        );
    }
    let slot_storage = retained_compaction_storage().await;
    let slot_storage_identity = Arc::as_ptr(&slot_storage) as usize;
    let slot_document = ArtifactId(String::from("p1y-slot-max-plus-one"));
    let slot_document_identity = slot_document.0.as_ptr();
    let slot_rejected = DatabaseCompactionFuture::try_submit(pool.clone(), slot_storage, slot_document, db_storage::DbIoText::try_from_str("slot-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap_err();
    let (slot_error, slot_storage, slot_document, mut slot_holder) = slot_rejected.into_parts().unwrap();
    assert_eq!(slot_error, DbError::LimitExceeded("database compaction admission slots"));
    assert_eq!(Arc::as_ptr(&slot_storage) as usize, slot_storage_identity);
    assert_eq!(slot_document.0.as_ptr(), slot_document_identity);
    assert!(slot_holder.close_step());
    let storage = retained_compaction_storage().await;
    let storage_identity = Arc::as_ptr(&storage) as usize;
    let mut external = String::with_capacity(db_storage::DbIoText::maximum_capacity() + 1);
    external.push_str("p1y-max-plus-one");
    let document_identity = external.as_ptr();
    let rejected = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(external), db_storage::DbIoText::try_from_str("exact-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap_err();
    let (error, storage, document, mut holder) = rejected.into_parts().unwrap();
    assert_eq!(error, DbError::LimitExceeded("database compaction document backing"));
    assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
    assert_eq!(document.0.as_ptr(), document_identity);
    assert_eq!(holder.as_str(), "exact-holder");
    assert!(holder.close_step());
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    let mut hashes = DatabaseCompactionHashOwners { slots: [Some(ContentHash([7; 32])); DATABASE_COMPACTION_MAX_HASHES], len: DATABASE_COMPACTION_MAX_HASHES as u16 };
    assert_eq!(hashes.insert(ContentHash([9; 32]), &cancelled).await, Err(DbError::LimitExceeded("database compaction payload hash owners")));
    let descriptor = db_snapshot::SnapshotDescriptor {
        document: ArtifactId(String::from("p1y-observed-backing")),
        generation: 1,
        parent_generation: None,
        head_seq: 1,
        commit_seq: 1,
        epoch: 1,
        chain_hash: [0; 32],
        protocol_version: 1,
        vcs_head: None,
        base_pack_hash: None,
        roots: Vec::with_capacity(DATABASE_COMPACTION_OPERATION_ITEMS as usize),
        new_pages: Vec::new(),
        created_at_ms: 1,
    };
    let mut descriptor_ledger = DatabaseCompactionBackingLedger::default();
    assert_eq!(database_compaction_admit_descriptor(descriptor, &mut descriptor_ledger).await.unwrap_err(), DbError::LimitExceeded("database compaction snapshot backing"));
    for future in &admitted {
        future.cancel();
    }
    drop(admitted);
    held.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_compaction_future_acquires_pool_use_before_admission() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    assert_eq!(pool.shutdown(), Ok(()));
    let before = {
        let admission = DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (admission.items, admission.bytes, admission.slots.iter().filter(|slot| slot.occupied).count())
    };
    let storage = retained_compaction_storage().await;
    let storage_identity = Arc::as_ptr(&storage) as usize;
    let document = ArtifactId(String::from("pool-use-precedes-compaction-admission"));
    let document_identity = document.0.as_ptr();
    let holder = db_storage::DbIoText::try_from_str("pool-use-holder").unwrap();
    let rejected = DatabaseCompactionFuture::try_submit(pool, storage, document, holder, false, CompactionBudget::default(), 0).unwrap_err();
    let (error, storage, document, mut holder) = rejected.into_parts().unwrap();
    assert!(matches!(error, DbError::Unavailable(detail) if detail.contains("WorkerPool use rejected")));
    assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
    assert_eq!(document.0.as_ptr(), document_identity);
    assert_eq!(holder.as_str(), "pool-use-holder");
    assert!(holder.close_step());
    let after = {
        let admission = DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (admission.items, admission.bytes, admission.slots.iter().filter(|slot| slot.occupied).count())
    };
    assert_eq!(after, before);
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity() {
    let (pool, held) = held_compaction_worker_pool();
    let future =
        DatabaseCompactionFuture::try_submit(pool.clone(), retained_compaction_storage().await, ArtifactId(String::from("p1y-stale")), db_storage::DbIoText::try_from_str("stale-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    let replacement = state.generation.checked_add(1).unwrap();
    DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = replacement;
    assert_eq!(future.await.unwrap_err(), DbError::StaleGeneration { expected: GenerationId(state.generation), actual: GenerationId(replacement) });
    assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
    DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = state.generation;
    held.store(false, std::sync::atomic::Ordering::Release);
    while state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
        std::thread::yield_now();
    }
    let mut reports = CompactionIndexReports::default();
    for kind in db_index::IndexKind::ALL {
        reports.push(IndexKindReport { kind, stats: db_index::IndexStats { run_count: 0, entry_count: 0, total_bytes: 0 } }).unwrap();
    }
    let mut owners = DatabaseCompactionTerminalOwners {
        storage: Some(retained_compaction_storage().await),
        document: Some(ArtifactId(String::from("p1y-close"))),
        holder: Some(db_storage::DbIoText::try_from_str("close-holder").unwrap()),
        result: Some(Ok(CompactionReport { index_reports: reports, ..CompactionReport::default() })),
    };
    let before = owners.result.as_ref().and_then(|result| result.as_ref().ok()).map(|report| report.index_reports.len()).unwrap();
    assert!(owners.close_one());
    let after = owners.result.as_ref().and_then(|result| result.as_ref().ok()).map(|report| report.index_reports.len()).unwrap();
    assert_eq!(before - after, 1);
    while owners.close_one() {}
    assert!(owners.terminal_is_empty());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_index_child_uses_exact_parent_cancel_and_eight_ms_control() {
    let storage = retained_compaction_storage().await;
    let index_storage = storage.index().await;
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let handle = db_index::IndexHandle::new(&index_storage, ArtifactId(String::from("p1y-index-cancel")), db_index::IndexKind::Command).await;
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS);
    let mut control = handle.retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL).unwrap();
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert_eq!(handle.compact(&mut control).await, Err(DbError::Unavailable("index cursor cancelled".to_string())));
    assert_eq!(DATABASE_COMPACTION_TURN_MS, 8);
    assert_ne!(DATABASE_COMPACTION_INDEX_FUEL, 65_536);
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_expected_snapshot_publication_never_persists_stale_baseline() {
    let storage = retained_compaction_storage().await;
    let snapshot = storage.snapshot().await;
    let manager = db_snapshot::SnapshotManager::new(&snapshot).await;
    let document = ArtifactId(String::from("p1y-publication-cas"));
    assert_eq!(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(1).await).await.unwrap(), 0);
    let expected = snapshot.latest_generation(&document).await.unwrap().unwrap();
    assert_eq!(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(2).await).await.unwrap(), 1);
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_millis(8), DATABASE_COMPACTION_INDEX_FUEL).unwrap();
    let rejected = manager.publish_retained_expected(&document, expected, &[], 0, sample_body(1).await, &mut control).await.unwrap_err();
    let (error, body) = rejected.into_parts();
    assert_eq!(error, DbError::StaleGeneration { expected: GenerationId(0), actual: GenerationId(1) });
    retire_compaction_snapshot_body(body).await;
    let mut generations = snapshot.list_generations(&document).await.unwrap();
    assert_eq!(generations.as_slice(), &[0, 1]);
    close_compaction_owner(|| Ok(generations.close_step())).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_panic_after_lease_acquire_releases_once_before_public_fault_and_registry_drain() {
    let (pool, held) = held_compaction_worker_pool();
    let storage = retained_compaction_storage().await;
    let lease_storage = storage.clone();
    let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-panic-release")), db_storage::DbIoText::try_from_str("panic-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
    state.lease_recovery.install(fence);
    let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y injected post-lease panic") });
    let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
    drop(original);
    held.store(false, std::sync::atomic::Ordering::Release);
    assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
    assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_none());
    assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.panic_retired.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_release_error_retries_through_real_worker_loop_until_success_before_public_fault() {
    let (pool, held) = held_compaction_worker_pool();
    let storage = retained_compaction_storage().await;
    let lease_storage = storage.clone();
    let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-release-error-success")), db_storage::DbIoText::try_from_str("release-error-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
    state.lease_recovery.install(fence);
    state.lease_recovery.fail_release_attempts(1);
    let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y release error then success") });
    let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
    drop(original);
    held.store(false, std::sync::atomic::Ordering::Release);
    assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
    assert!(state.lease_recovery.release_attempts.load(std::sync::atomic::Ordering::Acquire) >= 2);
    assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.lease_recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_none());
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(core.future.is_none() && core.release_fault.is_none() && core.release_retry_fault.is_none());
    drop(core);
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_perpetual_release_error_keeps_fence_fault_admission_and_registry_discoverable() {
    let (pool, held) = held_compaction_worker_pool();
    let storage = retained_compaction_storage().await;
    let lease_storage = storage.clone();
    let future =
        DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-perpetual-release-error")), db_storage::DbIoText::try_from_str("perpetual-release-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
    state.lease_recovery.install(fence);
    state.lease_recovery.fail_release_attempts(usize::MAX);
    let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y perpetual release error") });
    let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
    drop(original);
    held.store(false, std::sync::atomic::Ordering::Release);
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert!(state.lease_recovery.release_attempts.load(std::sync::atomic::Ordering::Acquire) >= 2);
    assert!(!state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(*state.lease_recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(fence));
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(core.future.is_none());
    assert!(core.release_fault.is_some());
    assert!(core.panic_release.is_some() || state.release_retry_armed.load(std::sync::atomic::Ordering::Acquire));
    drop(core);
    assert!(!state.panic_retired.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
    assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_some());
    state.lease_recovery.fail_release_attempts(0);
    assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
    assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn retained_compaction_cumulative_observed_backing_rejects_individually_valid_combined_max_plus_one() {
    let mut ledger = DatabaseCompactionBackingLedger::default();
    let first = Vec::<u8>::with_capacity(DATABASE_COMPACTION_OPERATION_BYTES as usize / 2);
    let second = Vec::<u8>::with_capacity(DATABASE_COMPACTION_OPERATION_BYTES as usize / 2 + 1);
    let first_identity = first.as_ptr();
    let second_identity = second.as_ptr();
    database_compaction_observe_backing(&mut ledger, 1, first.capacity(), "p1y cumulative first").unwrap();
    assert_eq!(database_compaction_observe_backing(&mut ledger, 1, second.capacity(), "p1y cumulative max plus one"), Err(DbError::LimitExceeded("p1y cumulative max plus one")));
    assert_eq!(first.as_ptr(), first_identity);
    assert_eq!(second.as_ptr(), second_identity);
    ledger.release(1, first.capacity()).unwrap();
    database_compaction_observe_backing(&mut ledger, 1, second.capacity(), "p1y cumulative recovered").unwrap();
    ledger.release(1, second.capacity()).unwrap();
    assert_eq!((ledger.items, ledger.bytes), (0, 0));
    semio_framework_async::yield_once().await;
    drop(first);
    semio_framework_async::yield_once().await;
    drop(second);
}

#[semio_framework_async_macros::async_test]
async fn compaction_fixed_pages_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_return_exact_credit() {
    while compaction_page_maintenance_step().unwrap() {}
    let mut retained = CompactionRetainedPages::new();
    for index in 0..COMPACTION_RETAINED_PAGE_OWNERS {
        assert!(retained.try_push(state_page(&[index as u8]).await).is_ok());
    }
    let rejected = retained.try_push(state_page(b"max-plus-one").await).unwrap_err();
    assert_eq!(retained.len(), COMPACTION_RETAINED_PAGE_OWNERS);
    let exit = MountedCompactionPageClose::new(&mut retained).await.unwrap();
    assert_eq!(exit, CompactionCloseExit::Closed);
    assert!(retained.terminal_is_empty());
    {
        let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(CompactionRetainedPages::new());
        }
    }
    {
        let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(CompactionRetainedPages::new());
        }
    }
    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    let mut refusal = CompactionRetainedPages::new();
    assert!(refusal.try_push(rejected).is_ok());
    let exact_operation = refusal.slots()[0].as_ref().expect("exact refused compaction page").operation();
    let mut second_refusal = CompactionRetainedPages::new();
    assert!(second_refusal.try_push(state_page(b"max-plus-two").await).is_ok());
    let second_operation = second_refusal.slots()[0].as_ref().expect("second exact refused compaction page").operation();
    assert_eq!(refusal.retirement.map(|reservation| reservation.tier), Some(2));
    assert_eq!(second_refusal.retirement.map(|reservation| reservation.tier), Some(2));
    assert!(retire_compaction_pages(refusal).is_ok());
    assert!(retire_compaction_pages(second_refusal).is_ok());
    assert!(COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation)), Some(exact_operation));
        assert!(quarantine.iter().flatten().any(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation) == Some(second_operation)));
    }
    {
        let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..COMPACTION_RETIREMENT_SLOTS * 2 {
        assert!(compaction_page_maintenance_step().unwrap());
    }
    assert!(compaction_page_maintenance_step().unwrap());
    {
        let retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation)), Some(exact_operation));
    }
    while compaction_page_maintenance_step().unwrap() {}

    for tier in [&COMPACTION_PAGE_RETIREMENT, &COMPACTION_PAGE_RETIREMENT_OVERFLOW, &COMPACTION_PAGE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(CompactionRetainedPages::new());
        }
    }
    let exact_refusal = state_page(b"exact-all-tier-compaction-refusal").await;
    let exact_refusal_operation = exact_refusal.operation();
    let mut refused = CompactionRetainedPages::new();
    let exact_refusal = refused.try_push(exact_refusal).unwrap_err();
    assert_eq!(exact_refusal.operation(), exact_refusal_operation);
    for tier in [&COMPACTION_PAGE_RETIREMENT, &COMPACTION_PAGE_RETIREMENT_OVERFLOW, &COMPACTION_PAGE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = None;
        }
    }
    let mut recovered = CompactionRetainedPages::new();
    assert!(recovered.try_push(exact_refusal).is_ok());
    assert_eq!(MountedCompactionPageClose::new(&mut recovered).await.unwrap(), CompactionCloseExit::Closed);
    assert!(refused.terminal_is_empty());
}
//#endregion 🔖️Budget

//#region 🔖️Lease
#[semio_framework_async_macros::async_test]
async fn compaction_lease_round_trips_and_is_scoped_distinctly_from_the_snapshot_lease() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;

    let fence = db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-a", 1_000, 0)).unwrap();
    assert!(db_actor::block_on(CompactionLease::current(&storage, &document, 0)).unwrap().is_some());

    db_actor::block_on(CompactionLease::renew(&storage, &document, "holder-a", fence, 1_000, 500)).unwrap();
    assert!(matches!(db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-b", 1_000, 500)), Err(DbError::Conflict(_))));

    db_actor::block_on(CompactionLease::release(&storage, &document, "holder-a", fence)).unwrap();
    assert!(db_actor::block_on(CompactionLease::current(&storage, &document, 500)).unwrap().is_none());

    assert_ne!(CompactionLease::resource(&document), db_snapshot::SnapshotLease::resource(&document));
}
//#endregion 🔖️Lease

//#region 🔖️WalRetention
#[semio_framework_async_macros::async_test]
async fn segment_horizons_tracks_the_max_head_seq_seen_within_each_segment_span() {
    let document = doc("doc-1").await;
    let mut records = db_wal::WalRecordBatch::new();
    for record in [
        WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
        WalRecord::Command(wal_bytes(b"a").await),
        WalRecord::Frontier(frontier(&document, 3).await),
        WalRecord::Frontier(frontier(&document, 7).await),
        WalRecord::SegmentHeader { document: document.clone(), segment_index: 1, prev_chain_hash: Some([1u8; 32]) },
        WalRecord::Command(wal_bytes(b"b").await),
    ] {
        assert!(records.push(record).is_ok());
    }
    let horizons = segment_horizons(records.iter()).await;
    assert_eq!(horizons, vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(7) }, SegmentHorizon { segment_index: 1, max_head_seq: None },]);
    while records.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn plan_wal_retention_never_selects_the_highest_segment_index_even_if_it_qualifies() {
    let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(5) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(5) }];
    let selected = plan_wal_retention(&horizons, 100, &CompactionBudget::default());
    assert_eq!(selected, vec![0]);
}

#[semio_framework_async_macros::async_test]
async fn plan_wal_retention_never_selects_a_segment_with_no_known_horizon() {
    let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: None }, SegmentHorizon { segment_index: 1, max_head_seq: Some(999) }];
    let selected = plan_wal_retention(&horizons, 10, &CompactionBudget::default());
    assert!(selected.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn plan_wal_retention_only_selects_segments_at_or_below_the_floor() {
    let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(5) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(15) }, SegmentHorizon { segment_index: 2, max_head_seq: Some(20) }];
    let selected = plan_wal_retention(&horizons, 10, &CompactionBudget::default());
    assert_eq!(selected, vec![0]);
}

#[semio_framework_async_macros::async_test]
async fn plan_wal_retention_respects_the_budget_cap() {
    let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(1) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(1) }, SegmentHorizon { segment_index: 2, max_head_seq: Some(1) }];
    let budget = CompactionBudget { max_wal_segments: 1, ..CompactionBudget::default() };
    let selected = plan_wal_retention(&horizons, 100, &budget);
    assert_eq!(selected.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects() {
    let fixture = committed_compaction_fixture();
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("committed-compaction-effects").await;
    let aborted = storage.put(pages(b"aborted-payload")).await.unwrap();
    let committed = storage.put(pages(b"committed-payload")).await.unwrap();
    let mut previous = None;
    for row in fixture["segments"].as_array().unwrap() {
        previous = Some(append_committed_compaction_segment(&storage, &document, row, previous, &[("aborted", aborted), ("committed", committed)]).await);
    }
    let backend = db_storage::DbBackend::Memory(storage);
    let report = Compactor::new(&backend).await.run(&document, "committed-compaction-holder", fixture["floorHeadSeq"].as_u64().unwrap(), false, &CompactionBudget::default(), 0).await.unwrap();
    assert_eq!(report.wal_segments_deleted, fixture["expected"]["deletedSegments"].as_u64().unwrap());
    assert_eq!(report.payloads_deleted, fixture["expected"]["deletedPayloads"].as_u64().unwrap());
    let wal = backend.wal().await;
    let remaining_segments: Vec<u64> = fixture["expected"]["remainingSegments"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect();
    let mut remaining = wal.list_segments(&document).await.unwrap();
    assert_eq!(remaining.as_slice(), remaining_segments.as_slice(), "header-only highest segment remains the active horizon");
    while remaining.close_step() {}
    assert!(remaining.terminal_is_empty());
    let payload = backend.payload().await;
    let retained_payloads = fixture["expected"]["retainedPayloads"].as_array().unwrap();
    assert_eq!(payload.contains(&aborted).await.unwrap(), retained_payloads.iter().any(|value| value == "aborted"), "aborted CAS reference never becomes a deletion candidate");
    assert_eq!(payload.contains(&committed).await.unwrap(), retained_payloads.iter().any(|value| value == "committed"), "document-scoped compaction must retain committed CAS bytes without global reference authority");
}

#[semio_framework_async_macros::async_test]
async fn document_compaction_retains_shared_and_private_cas_without_global_reference_authority() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document_a = doc("payload-owner-a").await;
    let document_b = doc("payload-owner-b").await;
    let shared = storage.put(pages(b"shared-across-documents")).await.unwrap();
    let private_a = storage.put(pages(b"private-to-document-a")).await.unwrap();
    let sealed_a = serde_json::json!({
        "index": 0,
        "state": "sealed",
        "transactions": [{
            "id": 1,
            "outcome": "commit",
            "records": [
                { "kind": "frontier", "headSeq": 1 },
                { "kind": "payload", "payload": "shared" },
                { "kind": "payload", "payload": "private-a" }
            ]
        }]
    });
    let active_a = serde_json::json!({ "index": 1, "state": "active", "transactions": [] });
    let active_b = serde_json::json!({
        "index": 0,
        "state": "active",
        "transactions": [{
            "id": 1,
            "outcome": "commit",
            "records": [{ "kind": "payload", "payload": "shared" }]
        }]
    });
    let payloads = [("shared", shared), ("private-a", private_a)];
    let chain = append_committed_compaction_segment(&storage, &document_a, &sealed_a, None, &payloads).await;
    append_committed_compaction_segment(&storage, &document_a, &active_a, Some(chain), &payloads).await;
    append_committed_compaction_segment(&storage, &document_b, &active_b, None, &payloads).await;

    let backend = db_storage::DbBackend::Memory(storage);
    let report = Compactor::new(&backend).await.run(&document_a, "document-a-holder", 1, false, &CompactionBudget::default(), 0).await.unwrap();
    assert_eq!(report.wal_segments_deleted, 1);
    assert_eq!(report.payloads_deleted, 0);
    let payload = backend.payload().await;
    assert!(payload.contains(&shared).await.unwrap(), "document B's live CAS reference must survive document A compaction");
    assert!(payload.contains(&private_a).await.unwrap(), "even apparently private CAS bytes require storage-global reference authority before deletion");
}

#[semio_framework_async_macros::async_test]
async fn apply_wal_retention_deletes_selected_segments_and_is_idempotent() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    db_actor::block_on(storage.create_segment(&writer, 0)).unwrap();
    db_actor::block_on(storage.create_segment(&writer, 1)).unwrap();
    db_actor::block_on(storage.create_segment(&writer, 2)).unwrap();
    writer.release().await.unwrap();

    let deleted = db_actor::block_on(apply_wal_retention(&storage, &document, &[0, 1])).unwrap();
    assert_eq!(deleted, 2);
    assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![2]);

    db_actor::block_on(apply_wal_retention(&storage, &document, &[0, 1])).unwrap();
    assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![2]);
}
//#endregion 🔖️WalRetention

//#region 🔖️PayloadGc
#[semio_framework_async_macros::async_test]
async fn sweep_payloads_deletes_orphaned_candidates_but_keeps_hashes_still_referenced_elsewhere() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let orphan_hash = db_actor::block_on(storage.put(pages(b"orphan-payload"))).unwrap();
    let shared_hash = db_actor::block_on(storage.put(pages(b"shared-payload"))).unwrap();
    let document = doc("doc-1").await;

    let mut records = db_wal::WalRecordBatch::new();
    for record in [
        WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
        WalRecord::Payload(WalPayloadRef::CasRef(orphan_hash)),
        WalRecord::Payload(WalPayloadRef::CasRef(shared_hash)),
        WalRecord::SegmentHeader { document, segment_index: 1, prev_chain_hash: Some([0u8; 32]) },
        WalRecord::Payload(WalPayloadRef::CasRef(shared_hash)),
    ] {
        assert!(records.push(record).is_ok());
    }

    let report = db_actor::block_on(sweep_payloads(&storage, records.iter(), &[0], &CompactionBudget::default())).unwrap();
    assert_eq!(report.candidates_checked, 2);
    assert_eq!(report.deleted, 1);
    assert!(!db_actor::block_on(storage.contains(&orphan_hash)).unwrap());
    assert!(db_actor::block_on(storage.contains(&shared_hash)).unwrap());
    while records.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn sweep_payloads_respects_the_budget_cap() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let hash_a = db_actor::block_on(storage.put(pages(b"a"))).unwrap();
    let hash_b = db_actor::block_on(storage.put(pages(b"b"))).unwrap();
    let document = doc("doc-1").await;
    let mut records = db_wal::WalRecordBatch::new();
    assert!(records.push(WalRecord::SegmentHeader { document, segment_index: 0, prev_chain_hash: None }).is_ok());
    assert!(records.push(WalRecord::Payload(WalPayloadRef::CasRef(hash_a))).is_ok());
    assert!(records.push(WalRecord::Payload(WalPayloadRef::CasRef(hash_b))).is_ok());
    let budget = CompactionBudget { max_payloads: 1, ..CompactionBudget::default() };
    let report = db_actor::block_on(sweep_payloads(&storage, records.iter(), &[0], &budget)).unwrap();
    assert_eq!(report.deleted, 1);
    while records.close_step().unwrap() {}
}
//#endregion 🔖️PayloadGc

//#region 🔖️IndexCompaction
#[semio_framework_async_macros::async_test]
async fn compact_all_indexes_reports_every_kind_and_merges_multiple_runs_into_one() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let handle = db_index::IndexHandle::new(&storage, document.clone(), db_index::IndexKind::Command).await;
    index_put(&handle, b"a", b"1").await;
    index_put(&handle, b"b", b"2").await;
    let mut control = db_index::IndexCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    assert!(handle.stats(&mut control).await.unwrap().run_count >= 2, "two separate put calls must land in separate runs below the auto-merge threshold");

    let reports = db_actor::block_on(compact_all_indexes(&storage, &document)).unwrap();
    assert_eq!(reports.len(), db_index::IndexKind::ALL.len());

    let command_report = reports.iter().find(|report| report.kind == db_index::IndexKind::Command).unwrap();
    assert_eq!(command_report.stats.run_count, 1);
    assert_eq!(command_report.stats.entry_count, 2);
}
//#endregion 🔖️IndexCompaction

//#region 🔖️SnapshotConsolidation
#[semio_framework_async_macros::async_test]
async fn consolidate_produces_a_self_sufficient_full_baseline_covering_the_whole_chain() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;

    let gen0_pages = vec![state_page(b"base-a").await, state_page(b"base-b").await];
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &gen0_pages, sample_body(0).await)).unwrap();

    let gen1_pages = vec![state_page(b"delta-a").await];
    let mut body1 = sample_body(5).await;
    body1.roots = vec![gen1_pages[0].hash, gen0_pages[1].hash];
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &gen1_pages, body1.clone())).unwrap();

    let consolidator = SnapshotConsolidator::new(&storage).await;
    let new_generation = db_actor::block_on(consolidator.consolidate(&document, 1, &CompactionBudget::default())).unwrap();
    assert_eq!(new_generation, 2);

    let mut bytes = db_actor::block_on(storage.read_generation(&document, new_generation)).unwrap();
    let mut prepared = db_storage::db_io_prepare_platform(&bytes).unwrap().await.unwrap();
    let handle = db_snapshot::open_latest(prepared.as_slice()).await.unwrap();
    assert!(handle.parent_footer_offset().await.is_none(), "a consolidated generation must be a self-sufficient full baseline");
    assert_eq!(handle.descriptor.roots, body1.roots);
    while prepared.close_step().unwrap() {}
    while bytes.close_step().unwrap().is_some() {}

    let control = db_snapshot::SnapshotCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut cursor = manager.chain_cursor(&document, new_generation, control);
    for page in gen0_pages.iter().chain(gen1_pages.iter()) {
        let mut read_back = cursor.read_page(page.hash).await.unwrap();
        assert_eq!(read_back, *page.pages());
        while read_back.close_step().unwrap().is_some() {}
    }
    while cursor.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn retain_from_after_consolidate_prunes_every_generation_below_the_new_baseline() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(0).await)).unwrap();
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(1).await)).unwrap();

    let consolidator = SnapshotConsolidator::new(&storage).await;
    let new_generation = db_actor::block_on(consolidator.consolidate(&document, 1, &CompactionBudget::default())).unwrap();
    db_actor::block_on(consolidator.retain_from(&document, new_generation)).unwrap();

    assert_eq!(db_actor::block_on(storage.list_generations(&document)).unwrap(), vec![new_generation]);
}

#[semio_framework_async_macros::async_test]
async fn consolidate_respects_the_snapshot_chain_depth_budget() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(0).await)).unwrap();
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(1).await)).unwrap();
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(2).await)).unwrap();

    let consolidator = SnapshotConsolidator::new(&storage).await;
    let tight_budget = CompactionBudget { max_snapshot_generations: 1, ..CompactionBudget::default() };
    assert!(matches!(db_actor::block_on(consolidator.consolidate(&document, 2, &tight_budget)), Err(DbError::LimitExceeded(_))));
}
//#endregion 🔖️SnapshotConsolidation

//#region 🔖️ColdArchive
#[semio_framework_async_macros::async_test]
async fn build_cold_archive_matches_materialize_chain_and_reopens_independently() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    let pages = vec![state_page(b"page-a").await];
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &pages, sample_body(0).await)).unwrap();

    let mut archive = db_actor::block_on(build_cold_archive(&storage, &document, 0)).unwrap();
    let control = db_snapshot::SnapshotCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut cursor = manager.chain_cursor(&document, 0, control);
    let mut expected = db_actor::block_on(cursor.materialize_pages()).unwrap();
    assert_eq!(archive, expected);

    let mut prepared = db_storage::db_io_prepare_platform(&archive).unwrap().await.unwrap();
    let handle = db_snapshot::open_latest(prepared.as_slice()).await.unwrap();
    assert_eq!(handle.generation().await, 0);
    while prepared.close_step().unwrap() {}
    while expected.close_step().unwrap().is_some() {}
    while archive.close_step().unwrap().is_some() {}
    while cursor.close_step().unwrap() {}
}
//#endregion 🔖️ColdArchive

//#region 🔖️Compactor
#[semio_framework_async_macros::async_test]
async fn run_never_deletes_the_sole_or_active_wal_segment() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(&storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    submit_record(&storage, &mut wal, WalRecord::Frontier(frontier(&document, 100).await), 0).await;
    wal.close().await.unwrap();
    let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

    let compactor = Compactor::new(&storage).await;
    let report = db_actor::block_on(compactor.run(&document, "holder-a", 1_000, false, &CompactionBudget::default(), 0)).unwrap();
    assert_eq!(report.wal_segments_deleted, 0);
    assert_eq!(db_actor::block_on(async { storage.wal().await.list_segments(&document).await }).unwrap(), vec![0]);
}

#[semio_framework_async_macros::async_test]
async fn run_end_to_end_compacts_indexes_and_consolidates_snapshots_then_releases_the_lease() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    let gen0_pages = vec![state_page(b"p0").await];
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &gen0_pages, sample_body(0).await)).unwrap();
    let gen1_pages = vec![state_page(b"p1").await];
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &gen1_pages, sample_body(5).await)).unwrap();

    let command_handle = db_index::IndexHandle::new(&storage, document.clone(), db_index::IndexKind::Command).await;
    index_put(&command_handle, b"k1", b"v1").await;
    index_put(&command_handle, b"k2", b"v2").await;
    let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

    let compactor = Compactor::new(&storage).await;
    let report = db_actor::block_on(compactor.run(&document, "holder-a", 0, true, &CompactionBudget::default(), 0)).unwrap();

    assert_eq!(report.snapshot_consolidated_generation, Some(2));
    assert_eq!(report.snapshot_generations_pruned, 2);
    assert_eq!(db_actor::block_on(async { storage.snapshot().await.list_generations(&document).await }).unwrap(), vec![2]);

    assert_eq!(report.index_reports.len(), db_index::IndexKind::ALL.len());
    let command_report = report.index_reports.iter().find(|entry| entry.kind == db_index::IndexKind::Command).unwrap();
    assert_eq!(command_report.stats.run_count, 1);

    assert!(db_actor::block_on(async { CompactionLease::current(&storage.lease().await, &document, 0).await }).unwrap().is_none(), "a successful run must release its lease");
}

#[semio_framework_async_macros::async_test]
async fn run_fails_with_conflict_when_another_holder_already_holds_the_compaction_lease() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let fence = db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-a", 10_000, 0)).unwrap();
    let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

    let compactor = Compactor::new(&storage).await;
    let result = db_actor::block_on(compactor.run(&document, "holder-b", 0, false, &CompactionBudget::default(), 0));
    assert!(matches!(result, Err(DbError::Conflict(_))));

    db_actor::block_on(async { CompactionLease::release(&storage.lease().await, &document, "holder-a", fence).await }).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn run_releases_the_compaction_lease_even_when_a_step_fails() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    db_actor::block_on(storage.create_segment(&writer, 0)).unwrap();
    db_actor::block_on(storage.append(&writer, 0, pages(b"not a valid spr segment at all"))).unwrap();
    db_actor::block_on(storage.seal(&writer, 0)).unwrap();
    writer.release().await.unwrap();
    let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

    let compactor = Compactor::new(&storage).await;
    let budget = CompactionBudget::default();
    let result = db_actor::block_on(compactor.run(&document, "holder-a", 0, false, &budget, 0));
    assert!(result.is_err());

    assert!(db_actor::block_on(async { CompactionLease::current(&storage.lease().await, &document, 0).await }).unwrap().is_none(), "the lease must be freed despite the failure");
}

#[semio_framework_async_macros::async_test]
async fn run_from_latest_snapshot_derives_the_floor_from_the_current_snapshot_head_seq() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(42).await)).unwrap();

    let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(&storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
    submit_record(&storage, &mut wal, WalRecord::Frontier(frontier(&document, 42).await), 0).await;
    wal.close().await.unwrap();
    let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

    let compactor = Compactor::new(&storage).await;
    let report = db_actor::block_on(compactor.run_from_latest_snapshot(&document, "holder-a", false, &CompactionBudget::default(), 0)).unwrap();
    assert_eq!(report.wal_segments_deleted, 0, "the sole segment must still never be touched, even though its horizon is at the floor");
}
//#endregion 🔖️Compactor
