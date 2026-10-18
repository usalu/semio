/// 🚨️ Registration, cancellation, drop and cleanup laws: the cancellation owners, the panic and shutdown close, the registered backend pool, the registration saturation, the forged backend kind and the cleanup fault waiters.

#[semio_framework_async_macros::async_test]
async fn db_io_backend_registration_saturation_returns_exact_executor_before_pool_use() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_backend_registration_saturation_returns_exact_executor_before_pool_use") {
        return;
    }
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "all-retirement-tiers-full-return-exact-executor").unwrap();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let lost_owner_sentinels = LostOwnerPressureLawSlots::reserve();
    let sentinels = RejectedBackendPressureLawSlots::reserve();
    let rejected = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap_err();
    assert_eq!(rejected.has_retained_executor(), row["expected"] == "exact-owner");
    drop(sentinels);
    drop(lost_owner_sentinels);
    let control = rejected.retry().unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        pool.try_submit(Lane::Io, Box::new(|| {})).ok().expect("exact prepared rollback refusal capacity");
    }
    let rollback = DbIoBackendRollbackReservation::try_reserve().unwrap();
    let pool_use = pool.acquire_use().unwrap();
    let previous_generation = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous = registry.next_generation;
        registry.next_generation = u64::MAX;
        previous
    };
    let rejected = register_db_io_backend_prepared_with_use(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone(), pool_use, rollback).unwrap_err();
    db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation = previous_generation;
    let mut rejected = DbStorageOpenRejected::Registration(rejected);
    assert!(rejected.has_retained_backend());
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    release_tx.send(()).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let cause = loop {
        match rejected.retry_close().await {
            Ok(cause) => break cause,
            Err(retained) => rejected = retained,
        }
        assert!(std::time::Instant::now() < deadline, "prepared registration rejection lost its terminal close owner");
    };
    assert!(matches!(cause, DbError::Unavailable(_)));
    assert_eq!(pool.shutdown(), Ok(()));
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_task_uses_registered_backend_pool_not_caller_pool() {
    let _owner = fixture_owner();
    let registered = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let unrelated = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), registered.clone()).unwrap();
    assert_eq!(unrelated.shutdown(), Ok(()));
    let result =
        submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://registered-pool").unwrap() }).unwrap_or_else(|(error, _)| panic!("registered-pool task admission failed: {error}")).finish().await.unwrap();
    assert!(matches!(result, DbIoResult::Unit));
    assert_eq!(registered.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(registered.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_backend_drop_retains_pool_until_deferred_close_terminal() {
    let _owner = fixture_owner();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let allow_close = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(DeferredBackendCloseLawExecutor { allow_close: allow_close.clone(), terminal: false }), pool.clone()).unwrap();
    drop(DropRegisteredBackend(control));
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    allow_close.store(true, std::sync::atomic::Ordering::Release);
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_forged_backend_kind_is_rejected_before_task_page_admission() {
    let _owner = fixture_owner();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (slot, generation) = db_io_backend_parts(control);
    let forged = DbIoBackendControl::Filesystem { slot, generation };
    let input = pages(b"must-remain-owned");
    let (error, task) = match submit_db_io_task(DbIoTask::PayloadPut { backend: forged, input }) {
        Err(rejected) => rejected,
        Ok(_) => panic!("forged kind published a task"),
    };
    assert!(matches!(error, DbError::StaleGeneration { .. }));
    let DbIoTask::PayloadPut { input, .. } = task else { panic!("forged task returned the wrong exact owner") };
    assert!(input.pages.iter().flatten().all(|page| page.phase().unwrap() == DbIoPagePhase::CheckedOutInput));
    drain_pages(input);
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

/// 🧯️ An operation-cleanup fault stays with its backend: the close ring records it on the failing backend and moves on, so a
/// healthy backend's operation finishes, drains and closes `Ok` beside it, while the failing backend's own waiter and close
/// report the fault (its kind preserved) until its cleanup succeeds.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let fail = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let faulty = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(CleanupFaultLawExecutor { fail: fail.clone(), terminal: false }), pool.clone()).unwrap();
    let healthy = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let injected = DbError::Io("injected operation cleanup fault".to_string());
    let faulted = submit_db_io_task(DbIoTask::BackendOpen { backend: faulty, path: DbIoText::try_from_str("fixture://cleanup-fault").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert_eq!(faulted.finish().await.err(), Some(injected.clone()), "the failing backend's own waiter reports its cleanup fault");
    let neighbour = submit_db_io_task(DbIoTask::BackendOpen { backend: healthy, path: DbIoText::try_from_str("fixture://cleanup-fault-neighbour").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert!(matches!(neighbour.finish().await, Ok(DbIoResult::Unit)), "a neighbour's operation never inherits another backend's cleanup fault");
    drain_control_tasks(healthy).await;
    close_db_io_backend(healthy).await.unwrap();
    assert_eq!(close_db_io_backend(faulty).await, Err(injected), "the failing backend's close reports its own cleanup fault");
    fail.store(false, std::sync::atomic::Ordering::Release);
    drain_control_tasks(faulty).await;
    close_db_io_backend(faulty).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
    assert_eq!(ledger_witness(), before);
}
