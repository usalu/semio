// 🪢️ Retained sync hello laws: the held pool helpers and every retained sync hello handoff, refusal, stream, snapshot cursor, deadline, cancellation, quarantine and census law.

//#region 🔖️Hello
fn held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let entered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let held = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let worker_entered = entered.clone();
    let worker_held = held.clone();
    pool.submit(
        semio_framework_async::Lane::Maintenance,
        Box::new(move || {
            worker_entered.store(true, std::sync::atomic::Ordering::Release);
            while worker_held.load(std::sync::atomic::Ordering::Acquire) {
                std::thread::yield_now();
            }
        }),
    );
    while !entered.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }
    (pool, held)
}

fn replenishing_sync_hello_io_job(pool: std::sync::Arc<semio_framework_async::WorkerPool>, active: std::sync::Arc<std::sync::atomic::AtomicBool>) -> semio_framework_async::Job {
    Box::new(move || {
        if active.load(std::sync::atomic::Ordering::Acquire) {
            let next = replenishing_sync_hello_io_job(pool.clone(), active.clone());
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, next) {
                drop(error.into_job());
            }
        }
    })
}

fn replenishing_held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let (pool, held) = held_sync_hello_pool();
    let active = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    loop {
        let job = replenishing_sync_hello_io_job(pool.clone(), active.clone());
        if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, job) {
            drop(error.into_job());
            break;
        }
    }
    (pool, held, active)
}

async fn retained_sync_hello_storage() -> std::sync::Arc<db_storage::DbBackend> {
    std::sync::Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_handoff_first_poll_cancel_preserves_exact_owner_and_io_lane() {
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let storage_identity = std::sync::Arc::as_ptr(&storage);
    let document = ArtifactId(String::from("p1z-handoff-cancel"));
    let document_identity = document.0.as_ptr();
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-session"), protocol::ActorId(String::from("p1z-origin").into()), 4096).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseSyncHelloDriverAuthority::Queued as u8);
    future.cancel();
    held.store(false, std::sync::atomic::Ordering::Release);
    let result = future.await.unwrap();
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owners = core.execution.as_ref().unwrap().owners.as_ref().unwrap();
    assert_eq!(std::sync::Arc::as_ptr(owners.storage.as_ref().unwrap()), storage_identity);
    assert_eq!(owners.document.0.as_ptr(), document_identity);
    assert!(matches!(core.execution.as_ref().unwrap().prepared, Err(DbError::Closed)));
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_max_plus_one_refusal_keeps_storage_document_frontier_session_origin_identity() {
    if !db_io_driver::process_isolated_law("tests::retained_sync_hello_max_plus_one_refusal_keeps_storage_document_frontier_session_origin_identity") {
        return;
    }
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let mut futures = Vec::new();
    for index in 0..DATABASE_SYNC_HELLO_SLOTS {
        futures.push(DatabaseSyncHelloFuture::try_submit(pool.clone(), storage.clone(), ArtifactId(format!("p1z-max-{index}")), None, format!("session-{index}"), protocol::ActorId(format!("origin-{index}").into()), 4096).unwrap());
    }
    let document = ArtifactId(String::from("p1z-max-plus-one"));
    let identity = document.0.as_ptr();
    let rejected = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("retained-session"), protocol::ActorId(String::from("retained-origin").into()), 4096).unwrap_err();
    let owners = rejected.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!(owners.as_ref().unwrap().document.0.as_ptr(), identity);
    assert_eq!(owners.as_ref().unwrap().session_id, "retained-session");
    assert_eq!(owners.as_ref().unwrap().origin.0.as_str(), "retained-origin");
    drop(owners);
    held.store(false, std::sync::atomic::Ordering::Release);
    for future in futures {
        future.cancel();
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_tail_stream_publishes_welcome_then_one_backpressured_frame() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-tail-stream"));
    seed_wal(&memory, &document, 3).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-tail"), protocol::ActorId(String::from("p1z-origin").into()), 4096).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Tail, .. }));
    welcome.acknowledge().unwrap();
    let frame = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::Commands { envelopes, .. } if envelopes.len() == 3));
    frame.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_snapshot_cursor_copies_at_most_one_page_fragment_per_driver_opportunity() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-snapshot-stream"));
    seed_wal(&memory, &document, 4).await;
    let floor = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 9, floor).await;
    let pages = db_io_registry::db_io_copy_pages(b"0123456789").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 9, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-snapshot"), protocol::ActorId(String::from("p1z-origin").into()), 4).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Snapshot { inline: None, .. }, .. }));
    welcome.acknowledge().unwrap();
    for (seq, bytes) in [(0, b"0123".as_slice()), (1, b"4567".as_slice()), (2, b"89".as_slice())] {
        let frame = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes: actual_bytes } if *actual == seq && actual_bytes.as_slice() == bytes));
        frame.acknowledge().unwrap();
    }
    let done = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    done.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_returned_snapshot_credit_waits_for_exact_generation_ack() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-returned-credit"));
    seed_wal(&memory, &document, 2).await;
    let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [2; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 7, floor).await;
    let pages = db_io_registry::db_io_copy_pages(b"abcdefgh").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 7, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-returned-credit-session"), protocol::ActorId(String::from("p1z-returned-credit-origin").into()), 4).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    let first = session.next_frame().await.unwrap().unwrap();
    let state = session.state.as_ref().unwrap().clone();
    let (generation, items, bytes, ledger_before) = {
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let lease = core.returned_frame.as_ref().unwrap();
        let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
        (lease.generation, lease.items, lease.bytes, (ledger.items, ledger.bytes))
    };
    assert!(generation != 0 && items == 1 && bytes >= 4);
    let mut next = Box::pin(session.next_frame());
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    assert!(matches!(std::future::Future::poll(next.as_mut(), &mut context), std::task::Poll::Pending));
    {
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let lease = core.returned_frame.as_ref().unwrap();
        let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
        assert_eq!(lease.generation, generation);
        assert_eq!((ledger.items, ledger.bytes), ledger_before);
        assert!(core.frame.is_none());
    }
    first.acknowledge().unwrap();
    let second = next.await.unwrap().unwrap();
    assert!(matches!(second.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: 1, .. }));
    second.acknowledge().unwrap();
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_maximum_snapshot_request_stays_page_unit_bounded() {
    let mut page_ledger = DatabaseSyncHelloBackingLedger::default();
    let page_reservation = database_sync_hello_reserve_snapshot_pages(&mut page_ledger).unwrap();
    assert_eq!((page_ledger.items, page_ledger.bytes), (DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES));
    let mut page_owner = db_io_registry::db_io_copy_pages(b"fixed-page-owner").unwrap().await.unwrap();
    let (page_items, page_bytes) = page_reservation.observed(&page_owner).unwrap();
    assert_eq!((page_items, page_bytes), (1, db_io_core::DB_IO_PAGE_BYTES));
    page_reservation.settle(&mut page_ledger, page_items, page_bytes).unwrap();
    while page_owner.close_step().unwrap().is_some() {}
    page_ledger.release(page_items, page_bytes).unwrap();
    assert!(page_owner.terminal_is_empty() && page_ledger.terminal_is_empty());

    let mut fixed_ledger = DatabaseSyncHelloBackingLedger::default();
    let fixed_reservation = database_sync_hello_reserve_snapshot_chunk(&mut fixed_ledger, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES).unwrap();
    assert_eq!((fixed_ledger.items, fixed_ledger.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(fixed_reservation.bytes, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    let fixed = fixed_reservation.allocate(&mut fixed_ledger).unwrap();
    assert_eq!(fixed.backing_bytes(), DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    let fixed_frame = protocol::ServerFrame::SnapshotChunk { seq: 0, bytes: fixed };
    let (fixed_items, fixed_bytes) = database_sync_hello_returned_frame_credit(&fixed_frame).unwrap();
    let mut fixed_lease = DatabaseSyncHelloReturnedFrameLease { generation: 1, items: fixed_items, bytes: fixed_bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(fixed_frame), envelope: None }) };
    assert_eq!((fixed_lease.items, fixed_lease.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert!(fixed_lease.close.as_mut().unwrap().close_one());
    assert!(fixed_lease.close.as_mut().unwrap().close_one());
    assert!(fixed_lease.close.as_ref().unwrap().terminal_is_empty());
    fixed_ledger.release(fixed_lease.items, fixed_lease.bytes).unwrap();
    assert!(fixed_ledger.terminal_is_empty());

    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-maximum-frame-unit"));
    seed_wal(&memory, &document, 2).await;
    let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [3; 32], epoch: 0 };
    publish_snapshot_marker(&memory, &document, 11, floor).await;
    let source = vec![b'x'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES * 2 + 1];
    let pages = db_io_registry::db_io_copy_pages(&source).unwrap().await.unwrap();
    SnapshotStorage::write_generation(&memory, &document, 11, pages).await.unwrap();
    let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-maximum-frame-session"), protocol::ActorId(String::from("p1z-maximum-frame-origin").into()), DATABASE_SYNC_HELLO_MAX_BYTES).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    for (seq, expected) in [(0, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (2, 1)] {
        let frame = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes } if *actual == seq && bytes.len() == expected && bytes.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        frame.acknowledge().unwrap();
    }
    let done = session.next_frame().await.unwrap().unwrap();
    assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    done.acknowledge().unwrap();
    assert!(session.next_frame().await.unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_grant_deadline_between_allocation_copy_and_publication_retains_credit() {
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    let expired = std::sync::atomic::AtomicBool::new(false);
    let document = ArtifactId(String::from("p1z-grant-deadline"));
    let source = vec![b'g'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES];
    let pages = db_io_registry::db_io_copy_pages(&source).unwrap().await.unwrap();
    let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut before_copy = DatabaseSyncHelloGrant::expiring_at(3);
    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_copy, &mut std::task::Context::from_waker(std::task::Waker::noop())), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));
    assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset: 0, seq: 0, .. } if chunk.is_empty() && chunk.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(ledger.items, 1);
    assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    while follow_up.close_one().unwrap() {}
    while ledger.close_one_credit() {}
    assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

    let pages = db_io_registry::db_io_copy_pages(&source).unwrap().await.unwrap();
    let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut before_publication = DatabaseSyncHelloGrant::expiring_at(5);
    assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_publication, &mut std::task::Context::from_waker(std::task::Waker::noop())), Err(DbError::Timeout(ref message)) if message == "database sync hello 8 ms grant"));
    assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset, seq: 0, .. } if chunk.len() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES && *offset == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
    assert_eq!(ledger.items, 1);
    assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
    while follow_up.close_one().unwrap() {}
    while ledger.close_one_credit() {}
    assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

    let independently_expired = std::sync::atomic::AtomicBool::new(true);
    assert!(matches!(database_sync_hello_control(&cancelled, &independently_expired), Err(DbError::Timeout(ref message)) if message == "database sync hello deadline"));
    assert_eq!(document.0, "p1z-grant-deadline");
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_cancel_before_stream_demand_publishes_no_new_frame() {
    let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let memory = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = ArtifactId(String::from("p1z-cancel-before-stream"));
    seed_wal(&memory, &document, 2).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
    let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-cancel-stream-session"), protocol::ActorId(String::from("p1z-cancel-stream-origin").into()), 4_096).unwrap().await.unwrap();
    let mut session = result.close_and_take_session().unwrap();
    session.take_welcome().unwrap().acknowledge().unwrap();
    let state = session.state.as_ref().unwrap().clone();
    session.cancel();
    assert!(matches!(session.next_frame().await, Err(DbError::Closed)));
    assert!(!state.demand.load(std::sync::atomic::Ordering::Acquire));
    let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(!matches!(core.frame.as_ref(), Some(Ok(Some(_)))));
}

#[test]
fn retained_sync_hello_cumulative_actual_backing_rejects_max_plus_one_without_mutation() {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    ledger.observe(DATABASE_SYNC_HELLO_MAX_ITEMS, DATABASE_SYNC_HELLO_MAX_BYTES, "maximum").unwrap();
    let before = (ledger.items, ledger.bytes);
    assert!(ledger.observe(1, 0, "cumulative max plus one").is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
    assert!(ledger.observe(0, 1, "cumulative max plus one").is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
}

#[test]
fn retained_sync_hello_predebits_envelope_clone_and_overallocation_before_owner_construction() {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let mut bytes = database_sync_hello_allocate_vec::<u8>(&mut ledger, 4_096, "database sync hello law backing").unwrap();
    assert_eq!(ledger.items, 1);
    assert_eq!(ledger.bytes, bytes.capacity());
    let capacity = bytes.capacity();
    drop(bytes);
    ledger.release(1, capacity).unwrap();
    let source = String::from("p1z-predebit");
    let owner = database_sync_hello_clone_string(&source, &mut ledger, "database sync hello law clone backing").unwrap();
    assert_ne!(source.as_ptr(), owner.as_ptr());
    assert_eq!(ledger.bytes, owner.capacity());
    let before = (ledger.items, ledger.bytes);
    assert!(database_sync_hello_allocate_vec::<u8>(&mut ledger, DATABASE_SYNC_HELLO_MAX_BYTES, "database sync hello law backing").is_err());
    assert_eq!((ledger.items, ledger.bytes), before);
}

#[test]
fn retained_sync_hello_cancel_between_yield_and_resume_prevents_next_wal_backend_operation() {
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    let expired = std::sync::atomic::AtomicBool::new(false);
    let mut opportunity = std::pin::pin!(database_sync_hello_opportunity(&cancelled, &expired));
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Pending));
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Ready(Err(DbError::Closed))));
    assert!(database_sync_hello_control(&cancelled, &expired).is_err());
}

#[test]
fn retained_sync_hello_quarantine_cursor_and_byte_item_ledger_reach_zero_before_release() {
    let future: DatabaseSyncHelloExecutionFuture = Box::pin(std::future::pending());
    let bytes = size_of_val(&*future);
    let mut quarantine = DatabaseSyncHelloQuarantineClose { future: Some(future), items: 1, bytes };
    assert!(quarantine.close_one());
    assert!(quarantine.terminal_is_empty());
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    ledger.observe(2, db_io_core::DB_IO_PAGE_BYTES + 1, "database sync hello quarantine law backing").unwrap();
    assert!(ledger.close_one_credit());
    assert_eq!(ledger.bytes, 1);
    while ledger.close_one_credit() {}
    assert!(ledger.terminal_is_empty());
}

#[test]
fn retained_sync_hello_page_close_error_is_typed_and_blocks_terminal_release() {
    let mut fault = None;
    let pending = database_sync_hello_apply_follow_up_close_result(&mut fault, false, Err(DbError::Internal("p1z retained page-close fault".to_string())));
    assert!(pending);
    let retained = fault.as_ref().unwrap();
    assert_eq!(retained.attempts, 1);
    assert!(matches!(retained.error.as_ref(), Some(DbError::Internal(detail)) if detail == "p1z retained page-close fault"));
    assert!(!retained.terminal_is_empty());
    assert!(fault.as_mut().unwrap().close_one());
    assert!(fault.as_ref().unwrap().terminal_is_empty());
}

#[test]
fn retained_sync_hello_refusal_retry_saturation_is_bounded_and_retains_terminal_job_owner() {
    let (pool, held, active) = replenishing_held_sync_hello_pool();
    let rejected = DatabaseSyncHelloRejected::new(
        pool,
        DbError::Unavailable("p1z refusal saturation law".to_string()),
        DatabaseSyncHelloOwners {
            storage: None,
            document: ArtifactId(String::from("p1z-refusal-bounded")),
            hello_frontier: None,
            session_id: String::from("p1z-refusal-session"),
            origin: protocol::ActorId(String::from("p1z-refusal-origin").into()),
            snapshot_chunk_bytes: 4_096,
        },
    );
    let close = rejected.close.clone();
    let generation = rejected.retirement_generation();
    assert!(matches!(rejected.close_and_take_error(), DbError::Unavailable(_)));
    let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
    assert!(witness.owners_retained && witness.job_retained && witness.submissions == 1);
    assert_eq!(close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().unwrap().document.0, "p1z-refusal-bounded");
    assert!(close.submissions.load(std::sync::atomic::Ordering::Acquire) <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
    active.store(false, std::sync::atomic::Ordering::Release);
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[test]
fn retained_sync_hello_forever_stuck_sole_worker_guarantees_discoverable_ownership_only() {
    let (pool, held) = held_sync_hello_pool();
    loop {
        if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, Box::new(|| {})) {
            drop(error.into_job());
            break;
        }
    }
    let rejected = DatabaseSyncHelloRejected::new(
        pool,
        DbError::Unavailable("p1z forever-stuck sole worker".to_string()),
        DatabaseSyncHelloOwners {
            storage: None,
            document: ArtifactId(String::from("p1z-discoverable-only")),
            hello_frontier: None,
            session_id: String::from("p1z-discoverable-session"),
            origin: protocol::ActorId(String::from("p1z-discoverable-origin").into()),
            snapshot_chunk_bytes: 4_096,
        },
    );
    let generation = rejected.retirement_generation();
    rejected.close_and_take_error();
    let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
    assert!(witness.owners_retained && witness.job_retained);
    assert!(witness.submissions <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[semio_framework_async_macros::async_test]
async fn retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service() {
    let (pool, held) = held_sync_hello_pool();
    let storage = retained_sync_hello_storage().await;
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, ArtifactId(String::from("p1z-deadline")), None, String::from("deadline-session"), protocol::ActorId(String::from("deadline-origin").into()), 4096).unwrap();
    let state = future.state.as_ref().unwrap().clone();
    state.deadline_callback();
    drop(future);
    assert!(state.expired.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.cancelled.load(std::sync::atomic::Ordering::Acquire));
    assert!(matches!(database_sync_hello_control(&state.cancelled, &state.expired), Err(DbError::Timeout(ref message)) if message == "database sync hello deadline"));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
    held.store(false, std::sync::atomic::Ordering::Release);
}

#[test]
fn retained_sync_hello_ready_pending_panic_and_repeat_poll_have_typed_terminal_states() {
    let source = fixture_text!("../../../../🦀️.rs");
    assert!(source.contains("Ok(std::task::Poll::Pending)"));
    assert!(source.contains("Ok(std::task::Poll::Ready(execution))"));
    assert!(source.contains("core.quarantined = Some(DatabaseSyncHelloQuarantineClose"));
    assert!(source.contains("if self.completed"));
    assert!(source.contains("std::task::Poll::Ready(Err(DbError::Closed))"));
}

#[test]
fn retained_sync_hello_production_census_has_zero_blocking_waits_and_no_eager_follow_up() {
    let source = fixture_text!("../../../../🦀️.rs");
    assert!(source.contains("#[cfg(test)]\npub async fn handle_hello"));
    assert!(source.contains("DatabaseSyncHelloNextFuture"));
}
//#endregion 🔖️Hello

#[semio_framework_async_macros::async_test]
async fn hello_shared_actor_retirement_releases_actual_final_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📝️actor-retirement/🔣️.json")).unwrap();
    for row in fixture["actors"].as_array().unwrap() {
        let text = row.as_str().unwrap();
        let mut original = String::with_capacity(fixture["reservedTextBytes"].as_u64().unwrap() as usize);
        original.push_str(text);
        let mut actor = semio_framework_value::SharedUtf8::from(original);
        let pointer = actor.as_ptr();
        let allocation_bytes = actor.original_allocation_bytes();
        let mut last = actor.clone();
        assert_eq!(serde_json::to_value(protocol::ActorId(actor.clone())).unwrap(), *row);
        let (closed, born, released) = crate::test_allocation::observe_backing(|| database_sync_hello_retire_shared_text(&mut actor));
        assert!(closed);
        assert!(!actor.has_owner());
        assert_eq!(born, fixture["newAllocationsOnClose"].as_u64().unwrap() as usize);
        assert_eq!(released, fixture["firstLeaseReleasedBytes"].as_u64().unwrap() as usize);
        assert_eq!(last.as_ptr(), pointer);
        assert_eq!(last.as_str(), text);
        let (closed, born, released) = crate::test_allocation::observe_backing(|| database_sync_hello_retire_shared_text(&mut last));
        assert!(closed);
        assert!(!last.has_owner());
        assert_eq!(born, 0);
        assert_eq!(released, allocation_bytes);
        let (closed, born, released) = crate::test_allocation::observe_backing(|| database_sync_hello_retire_shared_text(&mut last));
        assert!(!closed);
        assert_eq!((born, released), (0, 0));
    }
    eprintln!("[DEBUG] Sync hello actor retirement preserves sibling pointers and physically releases original text and lease backing once");
}
