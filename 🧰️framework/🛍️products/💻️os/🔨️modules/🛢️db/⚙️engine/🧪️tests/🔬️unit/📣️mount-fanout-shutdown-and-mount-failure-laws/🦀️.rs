// 📣️ Mount fanout, shutdown and mount failure laws: the fanout wake after the registry unlock, the shutdown interrupt, the catalog open and create laws and the mount failure terminalization.

//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-reentrant-wake"));
    let mut mount = Box::pin(database.ensure_document(&document));
    let probe = Arc::new(MountFanoutLockProbe { registry: database.open_artifacts.clone(), called: std::sync::atomic::AtomicBool::new(false), unlocked: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    let mut context = std::task::Context::from_waker(&waker);
    assert!(mount.as_mut().poll(&mut context).is_pending());
    emit.wait_until_document_event();
    emit.release_document_event();
    while !probe.called.load(std::sync::atomic::Ordering::Acquire) {
        semio_framework_async::yield_once().await;
    }
    assert!(probe.unlocked.load(std::sync::atomic::Ordering::Acquire));
    let handle = mount.await.unwrap();
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-shutdown"));
    let mut cancelled_waiter = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled_waiter.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    drop(cancelled_waiter);
    let (generation, owner) = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, waiters } = registry.slots.get(&document.0).unwrap() else { panic!("mount owner escaped before emission") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        (*generation, Arc::as_ptr(owner))
    };
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation: retained_generation, owner: retained_owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost mount owner") };
        assert_eq!((*retained_generation, Arc::as_ptr(retained_owner)), (generation, owner));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    emit.release_document_event();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count() != 1 {
        assert!(std::time::Instant::now() < deadline, "resumed mount did not publish its ready authority");
        semio_framework_async::yield_once().await;
    }
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
    let root = tempdir("open-at-fresh").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert!(database.catalog().await.artifacts.is_empty());
    assert_eq!(database.health().await.open_artifacts, 0);
    assert!(matches!(database.health().await.report.overall, db_observe::HealthState::Healthy));
}

#[semio_framework_async_macros::async_test]
async fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
    let root = tempdir("create-and-find").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let catalog = database.catalog().await;
    assert_eq!(catalog.artifacts.len(), 1);
    assert_eq!(catalog.artifacts[0].document, document);

    let handle = database.document(&document).await.unwrap();
    assert_eq!(handle.document_id().await, &document);
}

#[semio_framework_async_macros::async_test]
async fn create_document_twice_errs_already_exists() {
    let root = tempdir("create-twice").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let result = database.create_document(ArtifactSpec::new(document).await);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("duplicate document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::AlreadyExists(_)));
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout() {
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(test_worker_pool()).await.unwrap()));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_write: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId("database-retained-open-rejection".to_string());
    let rejected = match database.create_document(ArtifactSpec::new(document.clone()).await).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted database document builder was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(!rejected.has_retained_writer());
    let core_document = to_core_document_id(&document).await;
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation() {
    let pool = test_worker_pool();
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-retained-failure"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    for result in [first_result, second_result] {
        let rejected = match result {
            Err(rejected) => rejected,
            Ok(handle) => {
                drop(handle);
                panic!("faulted mount was admitted")
            }
        };
        assert!(!rejected.has_retained_writer());
        assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    }
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_storage::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-unlock-fault"));
    let (parked_tx, parked_rx) = std::sync::mpsc::sync_channel(1);
    *database.mount_parked_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || parked_tx.send(()).unwrap()));
    let mut first = Box::pin(database.ensure_document(&document));
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    parked_rx.recv().unwrap();
    let exact = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).expect("temporary unlock fault retains the mounted owner") else { panic!("temporary unlock fault replaced its opening slot") };
        let work = owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountWork::Parked { rejected, .. } = &*work else { panic!("temporary unlock fault did not park its retained owner") };
        assert!(rejected.has_retained_writer());
        assert!(matches!(rejected.cleanup_error(), Some(DbError::Unavailable(detail)) if detail.contains("injected WAL writer unlock failure")));
        (*generation, Arc::as_ptr(owner))
    };
    drop(first);
    drop(second);
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost parked cleanup") };
        assert_eq!((*generation, Arc::as_ptr(owner)), exact);
        assert!(matches!(&*owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner), DatabaseDocumentMountWork::Parked { rejected, .. } if rejected.has_retained_writer()));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
        assert!(std::time::Instant::now() < deadline, "resumed parked mount cleanup did not retire its opening slot");
        semio_framework_async::yield_once().await;
    }
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_storage::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    drop(handle);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        eprintln!(
            "unlock-retry shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled unlock retry failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_coalesces_join_drives_without_shared_pool_starvation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-poller-coalescing"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |_| {
        published_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut first = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    published_rx.recv().unwrap();
    let mut joins = Vec::new();
    for _ in 0..8 {
        let mut join = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(join.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        joins.push(join);
    }
    let release_gate = gate.clone();
    pool.submit_at(
        pool.now_ms(),
        Lane::UserVisible,
        Box::new(move || {
            let (lock, ready) = &*release_gate;
            *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            ready.notify_one();
        }),
    );
    let first = first.await.unwrap();
    for join in joins {
        let joined = join.await.unwrap();
        assert!(Arc::ptr_eq(&first.authority, &joined.authority));
        drop(joined);
    }
    drop(first);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}
//#endregion 🔖️Database open/catalog
