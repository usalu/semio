// 🔌️ Mount cleanup fault, round trip and deferred seams: the cleanup fault that consumes a racing resume, the unknown document, the submit and query round trip, the history replay, the reopen, the exact consistency and the deferred seams.

//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-racing-cleanup-resume"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (fault_tx, fault_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let hook_counted = hook_count.clone();
    *database.mount_cleanup_fault_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || {
        hook_counted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        fault_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut mount = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(mount.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    fault_rx.recv().unwrap();
    let owner = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cleanup owner was not mounted") };
        owner.clone()
    };
    owner.drive();
    assert!(owner.resume_requested.load(std::sync::atomic::Ordering::Acquire));
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let rejected = match mount.await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted mount was admitted")
        }
    };
    assert!(!rejected.has_retained_writer());
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    assert_eq!(hook_count.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(owner.terminal.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseDocumentMountDriver::Terminal as u8);
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_of_an_unknown_id_errs_not_found() {
    let root = tempdir("unknown-doc").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let never_created = protocol::ArtifactId("never-created".to_string());
    let result = database.document(&never_created);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("unknown document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::NotFound(_)));
}
//#endregion 🔖️Database open/catalog
//#region 🔖️Round trip
#[semio_framework_async_macros::async_test]
async fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
    if !db_io_driver::process_isolated_law("tests::full_submit_durable_query_round_trip_over_a_real_document_authority") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("round-trip").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.document, document);
    assert_eq!(receipt.frontier.head_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!("hello"));

    let frontier = handle.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);
    let checkpoint_snapshot = handle.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(checkpoint_snapshot.authority_generation, GenerationId::INITIAL.0, "a never-restarted document authority publishes the initial supervision generation");
    assert_eq!(checkpoint_snapshot.frontier, frontier);
    assert_eq!(checkpoint_snapshot.head_edit_id, Some(protocol::MutationId("op-1".to_string())));

    let mut at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).await.unwrap();
    assert_eq!(at_least.len(), 1);
    while at_least.close_step().unwrap() {}
    assert!(at_least.terminal_is_empty());

    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    assert!(history.operation_id_eq(0, 0, "op-1"));
    while history.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_empty_and_two_batch_replay_are_deterministic() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_empty_and_two_batch_replay_are_deterministic") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("history-order").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-doc".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut empty = handle.history().await.unwrap();
    assert!(empty.entries().is_empty());
    while empty.close_step() {}
    for id in ["history-1", "history-2"] {
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(id, &[], "alice", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    let mut first = handle.history().await.unwrap();
    let mut second = handle.history().await.unwrap();
    assert_eq!(first.entries(), second.entries());
    assert!(first.operation_id_eq(0, 0, "history-1"));
    assert!(first.operation_id_eq(1, 0, "history-2"));
    assert!(second.operation_id_eq(0, 0, "history-1"));
    assert!(second.operation_id_eq(1, 0, "history-2"));
    while first.close_step() {}
    while second.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
    let root = tempdir("reopen").await;
    let document = protocol::ArtifactId("doc-1".to_string());
    {
        let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
    }

    let reopened = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert_eq!(reopened.catalog().await.artifacts.len(), 1, "the catalog root must have survived the reopen");

    let handle = reopened.document(&document).await.unwrap();
    let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
    assert_eq!(handle.frontier().await.unwrap().head_seq, 1);
    assert_eq!(handle.checkpoint_publication_snapshot().await.unwrap().head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
    let root = tempdir("exact-consistency").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let stale = handle.frontier().await.unwrap();

    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
    assert!(matches!(result.await, Err(DbError::Unavailable(_))));
}
//#endregion 🔖️Round trip
//#region 🔖️Deferred extension seams
#[semio_framework_async_macros::async_test]
async fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
    let root = tempdir("deferred").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();

    assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(handle.preview(handle.frontier().await.unwrap()).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full).await), Ok(Err(DbError::Unimplemented(_)))));
}
//#endregion 🔖️Deferred extension seams
