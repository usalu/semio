/// 🔁️ Retry driver and backend controls: the saturated retry wake, the retry generation maximum, the mock drivers, the shared worker driver, the backend controls and the lost result lease.

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_saturated_task_retry_wakes_parked_caller_without_unrelated_ingress() {
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    assert_eq!(u64::from(DB_IO_RETRY_LIMIT), fixture["retry"]["maximumAttempts"].as_u64().unwrap());
    assert_eq!(DB_IO_RETRY_DELAY_MS, fixture["retry"]["timerDelayMs"].as_u64().unwrap());
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        pool.try_submit(Lane::Io, Box::new(|| {})).ok().expect("exact queue capacity");
    }
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://parked-retry").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert!(DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).retry_attempt.is_some());
    struct WakeSender(std::sync::mpsc::Sender<()>);
    impl std::task::Wake for WakeSender {
        fn wake(self: Arc<Self>) {
            let _ = self.0.send(());
        }
    }
    let (wake_tx, wake_rx) = std::sync::mpsc::channel();
    let waker = std::task::Waker::from(Arc::new(WakeSender(wake_tx)));
    let mut context = std::task::Context::from_waker(&waker);
    DB_IO_RETRY_MAINTENANCE_CURSOR.store((operation.handle.slot as usize + 1) % DB_IO_OPERATION_ITEMS, std::sync::atomic::Ordering::Release);
    assert!(Pin::new(&mut operation).poll(&mut context).is_pending());
    release_tx.send(()).unwrap();
    assert_eq!(wake_rx.recv_timeout(std::time::Duration::from_secs(2)).is_ok(), fixture["retry"]["terminalAfterQueueRelease"].as_bool().unwrap(), "retry must wake the parked caller without another DB request or maintenance poll");
    assert!(matches!(exact_fixture_result(operation.await, "parked retry"), DbIoResult::Unit));
    drain_control_tasks(control).await;
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_retry_generation_max_publishes_a_lossless_terminal_fault() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            panic!("retry max fixture exhausted early: {:?}", error.kind());
        }
    }
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://retry-generation-max").unwrap() }).unwrap_or_else(|(error, _)| panic!("retry max fixture allocation failed: {error}"));
    {
        let mut owner = DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owner.retry_attempt.is_some());
        owner.retry_generation = u64::MAX;
    }
    db_io_retry(operation.handle, u64::MAX);
    let mut fault = match operation.take() {
        Ok(Some(Err(fault))) => fault,
        Ok(Some(Ok(_))) => panic!("retry generation exhaustion published a result"),
        Ok(None) => panic!("retry generation exhaustion did not publish its terminal fault"),
        Err(error) => panic!("retry generation exhaustion take failed: {error}"),
    };
    assert_eq!(fault.kind, DbIoFaultKind::Saturated);
    assert_eq!(fault.detail.as_str(), "DB I/O retry generation exhausted");
    while fault.close_step() {}
    release_tx.send(()).unwrap();
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_postgres_and_neo4j_mock_drivers_use_supplied_writer_and_observed_capacity() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    for kind in [DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
        let control = register_db_io_backend(kind, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
        let output = DbIoPageWriter::try_reserve(1).unwrap();
        let task = DbIoTask::WalRead { backend: control, document: DbIoText::try_from_str("fixture-driver-result").unwrap(), index: 0, range: ByteRange { offset: 0, len: 1 }, output };
        let mut operation = submit_db_io_task(task).unwrap_or_else(|(error, _)| panic!("mock driver task allocation failed: {error}"));
        let mut lease = match operation.take_async_native().await {
            Ok(lease) => lease,
            Err(error) => panic!("mock driver lease failed: {error}"),
        };
        lease.enter_lane_io_driver_turn().unwrap();
        let task_operation = lease.operation();
        assert!(DbIoDriverReservation::try_reserve(task_operation, DB_IO_OPERATION_BYTES as usize).is_err());
        let mut reservation = DbIoDriverReservation::try_reserve(task_operation, DB_IO_PAGE_BYTES).unwrap();
        let mut driver_result = Vec::with_capacity(DB_IO_PAGE_BYTES);
        driver_result.push(0x7c);
        reservation.observe_capacity(driver_result.capacity()).unwrap();
        let result = match lease.task_mut() {
            Ok(DbIoTask::WalRead { output, .. }) => {
                assert_eq!(output.write_fragment(&driver_result).unwrap(), driver_result.len());
                drop(driver_result);
                reservation.close_step().unwrap();
                DbIoResult::Pages(output.finish().unwrap())
            }
            Ok(_) => panic!("mock driver task taxonomy changed"),
            Err(error) => panic!("mock driver lost supplied writer: {error}"),
        };
        lease.leave_lane_io_driver_turn().unwrap();
        lease.complete(Ok(result)).unwrap();
        let mut pages = match exact_fixture_result(operation.await, "mock driver") {
            DbIoResult::Pages(pages) => pages,
            _ => panic!("mock driver returned the wrong typed result"),
        };
        while pages.close_step().unwrap().is_some() {}
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
    }
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let polled_on_worker = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let poll_thread = Arc::new(std::sync::Mutex::new(None));
    let worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let close_thread = Arc::new(std::sync::Mutex::new(None));
    let close_worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let caller = std::thread::current().id();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(
        DbIoBackendKind::Postgres,
        Box::new(AsyncLaneProbeExecutor {
            terminal: false,
            caller,
            polled_on_worker: polled_on_worker.clone(),
            poll_thread: poll_thread.clone(),
            worker_role: worker_role.clone(),
            close_thread: close_thread.clone(),
            close_worker_role: close_worker_role.clone(),
        }),
        pool.clone(),
    )
    .unwrap();
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://actual-lane-io").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    operation.start_async_native_on_lane_io().await.unwrap();
    let terminal = operation.await.unwrap().into_result().unwrap();
    assert!(matches!(terminal, DbIoResult::Unit));
    assert!(polled_on_worker.load(std::sync::atomic::Ordering::Acquire));
    assert!(worker_role.load(std::sync::atomic::Ordering::Acquire));
    assert_ne!(*poll_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert!(close_worker_role.load(std::sync::atomic::Ordering::Acquire));
    assert_ne!(*close_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_all_five_backend_controls_require_explicit_terminal_close_witness() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_all_five_backend_controls_require_explicit_terminal_close_witness") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    for kind in [DbIoBackendKind::Memory, DbIoBackendKind::Filesystem, DbIoBackendKind::Sqlite, DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
        let pool = db_io_test_pool();
        let control = register_db_io_backend(kind, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool).unwrap();
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        let waker = std::task::Waker::noop();
        let context = &mut std::task::Context::from_waker(waker);
        assert!(matches!(db_io_backend_close_step(control, context), Err(DbError::StaleGeneration { .. })));
    }
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_lost_result_lease_retains_every_page_and_final_handback() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let pool = db_io_test_pool();
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingOutputLifecycleLawExecutor { terminal: false, success_steps: counter.clone(), cancel_steps: counter.clone(), abandon_steps: counter }), pool.clone()).unwrap();
    let output = DbIoPageWriter::try_reserve(fixture["resultRetirement"]["pages"].as_u64().unwrap() as usize).unwrap();
    let task = DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x71; 32]), output };
    let operation = submit_db_io_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
    let mut lease = operation.await.unwrap();
    let handle = lease.handle;
    db_io_wait_task_retirement(handle).await.unwrap();
    let handback = DbIoResultHandback { handle, retained_credit: lease.retained_credit };
    let mut owner = DbIoLostOwner::ResultLease { handback, result: lease.result.take() };
    lease.transferred = true;
    drop(lease);
    let mut trace = Vec::new();
    for _ in fixture["resultRetirement"]["terminal"].as_array().unwrap() {
        trace.push(db_io_lost_owner_close_opportunity(&mut owner).unwrap());
    }
    assert!(matches!(owner, DbIoLostOwner::ResultLease { result: None, .. }));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
    assert_eq!(serde_json::to_value(trace).unwrap(), fixture["resultRetirement"]["terminal"]);
}
