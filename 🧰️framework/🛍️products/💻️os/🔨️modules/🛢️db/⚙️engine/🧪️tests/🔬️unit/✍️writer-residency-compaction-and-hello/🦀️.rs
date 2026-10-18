// ✍️ Writer residency, compaction and hello: the join helpers, the writer slot rounds, the filesystem documents beyond the slots, the compaction passes, the shutdown retry owners and the hello sessions.

//#region 🔖️Deferred extension seams
//#endregion 🔖️Deferred extension seams
//#region 🔖️WriterResidency
/// 🤝️ Polls every future of `futures` on the current task until all finished; outputs in order.
async fn join_every<F: Future>(futures: Vec<F>) -> Vec<F::Output> {
    let mut futures: Vec<std::pin::Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
    let mut outputs: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();
    std::future::poll_fn(|context| {
        for (future, output) in futures.iter_mut().zip(outputs.iter_mut()) {
            if output.is_none() {
                if let std::task::Poll::Ready(value) = future.as_mut().poll(context) {
                    *output = Some(value);
                }
            }
        }
        if outputs.iter().all(Option::is_some) {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    })
    .await;
    outputs.into_iter().map(|output| output.expect("every joined future finished")).collect()
}

/// ⏱️ `future`'s output, or `None` once `deadline_ms` passed on `pool`'s clock first.
async fn within<F: Future + Unpin>(pool: &WorkerPool, deadline_ms: u64, mut future: F) -> Option<F::Output> {
    let mut deadline = std::pin::pin!(pool.timer().sleep_until(deadline_ms));
    std::future::poll_fn(|context| {
        if let std::task::Poll::Ready(output) = std::pin::Pin::new(&mut future).poll(context) {
            return std::task::Poll::Ready(Some(output));
        }
        deadline.as_mut().poll(context).map(|_| None)
    })
    .await
}

/// 🗂️ Opens `documents` documents and keeps every handle while `clients` clients commit to `documents / clients` of them
/// each, `in_flight` commits at a time, for `rounds` rounds; every commit must be acknowledged at its exact head within the
/// round's deadline. Like a hub client, a commit refused as transient (`Unavailable`) is resent after a short backoff on
/// the pool's timer; a round that misses its deadline fails with every transient refusal seen, the stuck submits' progress
/// and `census` (the backend's writer admission line).
async fn commit_rounds_through_writer_slots(pool: &WorkerPool, database: &Database, prefix: &str, documents: usize, clients: usize, in_flight: usize, rounds: usize, census: &dyn Fn() -> String) -> Vec<(protocol::ArtifactId, ArtifactHandle)> {
    const ROUND_DEADLINE_MS: u64 = 180_000;
    let mut handles = Vec::with_capacity(documents);
    for ordinal in 0..documents {
        let document = protocol::ArtifactId(format!("{prefix}-{ordinal:04}"));
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap_or_else(|rejected| panic!("document {ordinal} opens beside the others: {rejected:?}"));
        handles.push((document, handle));
    }
    for round in 1..=rounds {
        let refusals = std::sync::Mutex::new(std::collections::BTreeMap::<String, usize>::new());
        let deadline_ms = pool.now_ms() + ROUND_DEADLINE_MS;
        let mut outcomes = Vec::with_capacity(documents);
        for (wave, chunk) in handles.chunks(in_flight).enumerate() {
            let commits = chunk
                .iter()
                .enumerate()
                .map(|(offset, (document, handle))| {
                    let ordinal = wave * in_flight + offset;
                    let refusals = &refusals;
                    async move {
                        let edit = format!("{}-round-{round}", document.0);
                        let previous = format!("{}-round-{}", document.0, round - 1);
                        let dependencies: Vec<&str> = if round == 1 { Vec::new() } else { vec![previous.as_str()] };
                        loop {
                            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&edit, &dependencies, &format!("client-{}", ordinal % clients), document, &[("round", serde_json::json!(round))]).await]).await.unwrap();
                            let mut submit = handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() });
                            let Some(outcome) = within(pool, deadline_ms, &mut submit).await else { return Err(format!("stuck in submit progress {:?}", submit.progress())) };
                            match outcome.and_then(std::convert::identity) {
                                Err(DbError::Unavailable(reason)) => {
                                    *refusals.lock().unwrap().entry(reason).or_default() += 1;
                                    if within(pool, deadline_ms, pool.timer().sleep_until(pool.now_ms() + 2)).await.is_none() {
                                        return Err("resending past the deadline".to_string());
                                    }
                                }
                                outcome => break outcome.map_err(|error| format!("refused: {error}")),
                            }
                        }
                    }
                })
                .collect();
            outcomes.extend(join_every(commits).await);
        }
        let failed: Vec<String> = handles.iter().zip(&outcomes).filter_map(|((document, _), outcome)| outcome.as_ref().err().map(|error| format!("{}: {error}", document.0))).collect();
        assert!(failed.is_empty(), "round {round}: {} of {documents} commits failed (first {:?}); transient refusals {:?}; writer admission {}", failed.len(), &failed[..failed.len().min(8)], refusals.lock().unwrap(), census());
        for ((document, _), outcome) in handles.iter().zip(outcomes) {
            let receipt = outcome.expect("checked commit");
            assert_eq!(receipt.frontier.head_seq, round as u64, "{} round {round} landed at its exact head (transient refusals {:?})", document.0, refusals.lock().unwrap());
        }
    }
    handles
}

/// 🗂️ Open documents are neither writers nor maintenance hooks (declared `residency`), on the filesystem backend where a
/// writer is a cross-process sidecar lock: 72 documents stay open at once — more than twice the writer slots of their
/// backend and more than the maintenance hooks of their pool — while 8 clients commit to 9 of them each, 8 commits in flight
/// at a time, for 2 rounds, so nearly every commit is lent a writer another document parked. Every commit is acknowledged at
/// its exact head, every lent lock re-verified against its segment, and a reopened database replays every document at its
/// last head. (The process's resident state — one DB I/O operation per state value — bounds how many documents and commits
/// may be in flight at once; that ceiling is set C's.)
#[semio_framework_async_macros::async_test]
async fn open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay() {
    if !db_io_driver::process_isolated_law("tests::open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay") {
        return;
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🗄️storage/🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let residency = &fixture["residency"];
    let count = |key: &str| residency[key].as_u64().unwrap() as usize;
    let (documents, clients, in_flight, rounds) = (count("documents"), count("clients"), count("inFlight"), count("rounds"));
    assert!(documents > 2 * db_io_core::WAL_WRITER_CAPACITY && documents > semio_framework_async::WORKER_MAINTENANCE_CAPACITY && documents % clients == 0);
    let root = tempdir("writer-residency-fs").await;
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Fs(db_storage_fs::FsStorage::open(pool.clone(), &root).await.unwrap()));
    let db_storage::DbBackend::Fs(fs) = storage.as_ref() else { unreachable!("filesystem residency backend") };
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let handles = commit_rounds_through_writer_slots(&pool, &database, "residency-fs", documents, clients, in_flight, rounds, &|| format!("{:?}", fs.writer_admission_census())).await;
    drop(handles);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(database);
    storage.close().await.unwrap();
    drop(storage);
    let reopened_storage = Arc::new(db_storage::DbBackend::Fs(db_storage_fs::FsStorage::open(pool.clone(), &root).await.unwrap()));
    let mut reopened = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), reopened_storage.clone()).await.unwrap();
    for ordinal in 0..documents {
        let document = protocol::ArtifactId(format!("residency-fs-{ordinal:04}"));
        let handle = reopened.ensure_document(&document).await.unwrap_or_else(|rejected| panic!("{} reopens: {rejected:?}", document.0));
        assert_eq!(handle.frontier().await.unwrap().head_seq, rounds as u64, "{} replays every round", document.0);
    }
    reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(reopened);
    reopened_storage.close().await.unwrap();
    drop(reopened_storage);
    pool.shutdown().unwrap();
    let _ = std::fs::remove_dir_all(root);
}
//#endregion 🔖️WriterResidency
//#region 🔖️Compact + Sync
#[semio_framework_async_macros::async_test]
async fn compact_document_runs_a_real_compaction_pass_without_error() {
    let root = tempdir("compact").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let report = database.compact_document(&document, "holder-1", false).await.unwrap();
    assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
}

#[semio_framework_async_macros::async_test]
async fn compact_document_uses_live_actor_writer_and_restores_submits() {
    let root = tempdir("compact-live-writer").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("compact-live-writer".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let first = db_artifact_contract::CommandBatch::new(vec![envelope("compact-before", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(first, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    let fill_ids: Vec<String> = (0..12).map(|index| format!("compact-fill-{index}")).collect();
    for (index, mutation_id) in fill_ids.iter().enumerate() {
        let dependency = if index == 0 { "compact-before" } else { fill_ids[index - 1].as_str() };
        let value = format!("{index}:{}", "x".repeat(50_000));
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(mutation_id, &[dependency], "alice", &document, &[("x", serde_json::json!(value))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    handle.authority.snapshot_now(100).await.unwrap();

    let core_document = to_core_document_id(&document).await;
    let storage = database.storage().await;
    let mut before = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(before.len() >= 2, "bounded public submissions must leave a sealed predecessor and active successor");
    let active = *before.last().unwrap();
    while before.close_step() {}
    assert!(before.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "the live actor must retain its original writer before compaction");
    let compacted = database.compact_document(&document, "actor-holder", false).await;
    let report = compacted.unwrap();
    assert!(report.wal_segments_deleted >= 1);
    assert_eq!(report.payloads_deleted, 0, "document-scoped maintenance must not reclaim global CAS bytes");
    let mut after = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(after.as_slice().contains(&active), "the actor WAL's authoritative active segment must survive compaction");
    while after.close_step() {}
    assert!(after.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "compaction must return the same retained writer to the actor");

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(matches!(handle.compact("cancelled-holder", false, cancelled).await, Err(DbError::Closed)));
    let second = db_artifact_contract::CommandBatch::new(vec![envelope("compact-after", &["compact-fill-11"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(second, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, 14);
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "cancelled maintenance must restore the engine without releasing its writer");
    drop(storage);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_cancellation_preserves_exact_retry_owners() {
    let root = tempdir("shutdown-retry-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-retry-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("shutdown-seed", &[], "owner", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();
    drop(handle);
    let closing = |database: &Database| matches!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(&document.0), Some(DatabaseDocumentMountSlot::Closing { .. }));
    assert!(closing(&database), "the last handle unmounts its document into a closing slot");

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(5), cancelled.clone());
    assert_eq!(database.shutdown(&control).await, Err(DbError::Closed));
    assert!(closing(&database), "cancelled shutdown retains the closing slot");
    assert!(database.closing_authority.is_none(), "an unmounted document retires through its closing slot");
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let drain_deadline = std::time::Instant::now() + std::time::Duration::from_secs(4);
    while std::time::Instant::now() < drain_deadline {
        database.shutdown_step(&control).await.unwrap();
        if database.closing_authority.is_none() && database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert!(database.closing_authority.is_none());
    database.shutdown(&control).await.expect("the retried shutdown completes");
    assert!(database.shutdown_complete);
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_shared_authority_blocks_without_closing_live_handle() {
    let root = tempdir("shutdown-shared-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-shared-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let control = DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(1)));
    assert_eq!(handle.frontier().await.unwrap().head_seq, 0);
    drop(handle);
    database.shutdown(&control).await.expect("released shared authority permits terminal shutdown");
}

#[semio_framework_async_macros::async_test]
async fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
    let root = tempdir("hello").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let mut session = database.hello(document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 4096).await.unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { .. }));
    welcome.acknowledge().unwrap();
}

/// 🚪️ The hub's document socket drives exactly this: welcome acknowledged, every bootstrap frame
/// acknowledged until the session ends, the session dropped with the socket — or dropped right
/// after the welcome when the socket dies early, so the close request races the returned frame's
/// retirement on the shared I/O lane. Every hello must retire: a retained one keeps its
/// `WorkerPoolUse`, and with it a stopping hub's `Database` and WAL writers, past every deadline.
#[semio_framework_async_macros::async_test]
async fn hello_sessions_retire_when_drained_and_when_close_races_the_returned_frame() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId("doc-hello-retirement".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    handle.submit(batch, db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
    for round in 0..8 {
        let mut session = database.hello(document.clone(), None, format!("session-{round}"), protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        if round % 2 == 0 {
            while let Some(frame) = session.next_frame().await.unwrap() {
                frame.acknowledge().unwrap();
            }
        }
        let witness = session.retirement_witness();
        drop(session);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !witness.retired() && std::time::Instant::now() < deadline {
            semio_framework_async::yield_once().await;
        }
        assert!(witness.retired(), "hello session {round} retired: {}", witness.describe());
    }
    drop(handle);
}

/// 🎟️ More hellos than the process has admission slots are all welcomed: the retained submission
/// still refuses the one beyond capacity (its owners handed back), but [`Database::hello`] waits for
/// a slot and is welcomed as soon as one is released — two dozen document sockets reopening
/// together after a hub restart used to see `database sync hello admission saturated` (ticket
/// 26/09/23 H9 session 12, growth e2e g15).
#[semio_framework_async_macros::async_test]
async fn hellos_beyond_the_admission_slots_wait_for_a_slot_instead_of_refusing() {
    if !db_io_driver::process_isolated_law("tests::hellos_beyond_the_admission_slots_wait_for_a_slot_instead_of_refusing") {
        return;
    }
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId("doc-hello-admission".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    handle.submit(batch, db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
    let origin = protocol::ActorId("semio_hub".to_string());
    let mut held = Vec::new();
    for slot in 0..8 {
        let mut session = database.hello(document.clone(), None, format!("held-{slot}"), origin.clone(), 64 * 1024).await.unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        held.push(session);
    }
    let core = to_core_document_id(&document).await;
    match database.hello_retained(core, None, "retained-ninth".to_string(), origin.clone(), 64 * 1024) {
        Err(DatabaseRetainedActivityRejected::Retained(rejected)) => {
            assert!(rejected.admission_saturated(), "the retained ninth hello is refused for capacity alone");
            drop(rejected.close_and_take_error());
        }
        Err(DatabaseRetainedActivityRejected::Closed(error)) => panic!("database closed: {error:?}"),
        Ok(_) => panic!("the retained ninth hello was admitted beyond the declared slots"),
    }
    let mut ninth = std::pin::pin!(database.hello(document.clone(), None, "waiting-ninth".to_string(), origin.clone(), 64 * 1024));
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..1_000 {
        assert!(ninth.as_mut().poll(&mut context).is_pending(), "the ninth hello waits while every slot is held");
        if db_sync::database_sync_hello_admission_waiters() == 1 {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert_eq!(db_sync::database_sync_hello_admission_waiters(), 1, "the ninth hello is registered as a waiter");
    drop(held.pop());
    let mut session = ninth.await.expect("the ninth hello is welcomed once a slot is released");
    session.take_welcome().unwrap().acknowledge().unwrap();
    assert_eq!(db_sync::database_sync_hello_admission_waiters(), 0, "a welcomed hello leaves the waiter table");
    drop(session);
    drop(held);
    drop(handle);
}

// 🔬️ `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
// caller below the document-actor boundary (os-semio_hub's blob routes) can round-trip a payload
// through it directly, independent of any document actor.
#[semio_framework_async_macros::async_test]
async fn storage_accessor_reaches_the_same_backend_payload_store() {
    let root = tempdir("storage-accessor").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let hash = db_actor::block_on(async {
        let pages = db_io_registry::db_io_copy_pages(b"hello storage accessor").unwrap().await.unwrap();
        database.storage().await.payload().await.put(pages).await
    })
    .unwrap();
    assert_eq!(db_actor::block_on(async { database.storage().await.payload().await.get(&hash).await }).unwrap(), b"hello storage accessor");
}
//#endregion 🔖️Compact + Sync
