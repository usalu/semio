// 🏊️ Worker pool use and mount owner laws: the worker pool use during shutdown and before the first probe, the published opening join, the cancelled ensure waiter and the mount owner that emits before ready.

//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    let document = protocol::ArtifactId(String::from("pool-use-mounted-authority"));
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    drop(handle);
    let unrelated = pool.acquire_use().unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("terminal database admitted another mount")
        }
    };
    assert_eq!(rejected_document_open_error(rejected).await, DbError::Closed);
    assert!(matches!(database.create_document_catalog_retained(protocol::ArtifactId(String::from("post-terminal-catalog"))), Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))));
    assert!(matches!(
        database.hello_retained(to_core_document_id(&document).await, None, String::from("post-terminal-session"), protocol::ActorId(String::from("post-terminal-actor")), 4096,),
        Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))
    ));
    let (ran_tx, ran_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), Lane::UserVisible, Box::new(move || ran_tx.send(()).unwrap()));
    ran_rx.recv().unwrap();
    drop(unrelated);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_is_admitted_before_the_first_storage_probe() {
    let pool = test_worker_pool();
    assert_eq!(pool.shutdown(), Ok(()));
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let error = match Database::open(pool, DbConfig::for_profile(Profile::Test), storage).await {
        Err(error) => error,
        Ok(mut database) => {
            let _ = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await;
            panic!("database opened on a stopped WorkerPool")
        }
    };
    assert!(matches!(error, DbError::Unavailable(detail) if detail.contains("WorkerPool use rejected")));
}

#[semio_framework_async_macros::async_test]
async fn database_published_opening_joins_without_actor_overwrite() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-published"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |document| {
        published_tx.send(document.clone()).unwrap();
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
    assert_eq!(published_rx.recv().unwrap(), document);
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("published mount was not retained as Opening") };
        assert_eq!(waiters.iter().flatten().count(), 2);
    }
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let first = first.await.unwrap();
    let second = second.await.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(first);
    drop(second);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_cancelled_ensure_waiter_does_not_cancel_mount_owner() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-cancel"));
    let mut cancelled = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    drop(cancelled);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cancelled waiter removed retained owner") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        assert!(!owner.terminal.load(std::sync::atomic::Ordering::Acquire));
        eprintln!(
            "cancelled-waiter stage=owner-retained driver={} wake={} resume={}",
            owner.driver.load(std::sync::atomic::Ordering::Acquire),
            owner.wake_requested.load(std::sync::atomic::Ordering::Acquire),
            owner.resume_requested.load(std::sync::atomic::Ordering::Acquire)
        );
    }
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    drop(handle);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "cancelled-waiter shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after cancelled waiter failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-emission"));
    let mut elected = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(elected.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    let mut survivor = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(survivor.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    assert_eq!(emit.document_event_count(), 0);
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 0);
    drop(elected);
    emit.release_document_event();
    let survivor = survivor.await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    let later = database.ensure_document(&document).await.unwrap();
    assert!(Arc::ptr_eq(&survivor.authority, &later.authority));
    assert_eq!(emit.document_event_count(), 1);
    drop(survivor);
    drop(later);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "emission mount shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled mount emission failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-capacity"));
    let mut waiters = Vec::new();
    for _ in 0..DATABASE_DOCUMENT_MOUNT_WAITERS {
        let mut waiter = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(waiter.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        waiters.push(waiter);
    }
    emit.wait_until_document_event();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("document mount admitted waiter 33")
        }
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Unavailable(_)));
    drop(waiters.remove(0));
    let mut replacement = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(replacement.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    waiters.push(replacement);
    emit.release_document_event();
    let mut handles = Vec::new();
    for waiter in waiters {
        handles.push(waiter.await.unwrap());
    }
    assert_eq!(handles.len(), DATABASE_DOCUMENT_MOUNT_WAITERS);
    assert!(handles.iter().all(|handle| Arc::ptr_eq(&handle.authority, &handles[0].authority)));
    assert_eq!(emit.document_event_count(), 1);
    drop(handles);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}
//#endregion 🔖️Database open/catalog
