/// 💥️ Panics and cleanup cancel failures: the start, step and cancel panics, the background and drop cleanup cancel failures and the start and step cleanup failures stored for the next route.

#[semio_framework_async_macros::async_test]
async fn mounted_start_panic_restores_the_instance_and_the_next_route_progresses() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_004);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-start-panic".to_vec() }).await;
    mock.panic_next_start();

    let error = handle.infer(b"first", &semio_framework_job::root_cancel_token()).await.expect_err("start panic must surface as a typed host fault");
    assert!(error.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available(), "the unwind lease must restore the taken instance");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("the process worker pool must survive the retained-future panic");
    assert_eq!(handle.infer(b"second", &semio_framework_job::root_cancel_token()).await.expect("the next mounted route must acquire the restored instance"), b"mounted-after-start-panic");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn mounted_step_panic_restores_the_instance_and_terminalizes_once() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_005);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-step-panic".to_vec() }).await;
    mock.panic_next_step();

    let error = handle.infer(b"first", &semio_framework_job::root_cancel_token()).await.expect_err("step panic must surface as a typed host fault");
    assert!(error.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
    assert_eq!(handle.infer(b"second", &semio_framework_job::root_cancel_token()).await.expect("a mounted route after step panic must not deadlock"), b"mounted-after-step-panic");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_006)).await;
    let relay_cancel = semio_framework_async::CancelToken::root_now();
    let session_cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;
    mock.panic_next_cancel();
    relay_cancel.cancel_now();

    let terminal = loop {
        let outcome = test_relay_step(&mut session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Fault(b"plugin instance quarantined after cancel-job panic".to_vec()));
    assert_eq!(test_relay_step(&mut session, &pool).await, TestRelayOutcome::Yield);
    wait_for_cancel_admission(&pool, &mock).await;
    assert_eq!(mock.cancel_admissions(), 1, "a panicking cancel-job admission must never be retried");
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_quarantined(), "cancel unwind must retain but quarantine the uncertain instance before fault delivery");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("the sole worker and semaphore permit must survive cancel panic");
    pool.shutdown().expect("worker shutdown");
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("a mounted route must reject rather than reuse a quarantined guest");
    assert!(next.to_string().contains("quarantined after cancel-job panic"));
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_008)).await;
    mock.panic_next_start();
    mock.panic_next_cancel();

    let first = handle.infer(b"first", &semio_framework_job::root_cancel_token()).await.expect_err("start panic must fault");
    assert!(first.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("the mounted route must reject background-cancel quarantine without deadlock");
    assert!(next.to_string().contains("quarantined after cleanup cancel-job panic"));
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn context_cancellation_failure_faults_once_quarantines_and_releases_one_worker() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_009)).await;
    let relay_cancel = semio_framework_async::CancelToken::root_now();
    let session_cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;
    mock.fail_next_cancel();
    relay_cancel.cancel_now();

    let terminal = loop {
        let outcome = test_relay_step(&mut session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert!(matches!(
        terminal,
        TestRelayOutcome::Fault(fault)
            if fault == b"plugin instance quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"
    ));
    assert_eq!(test_relay_step(&mut session, &pool).await, TestRelayOutcome::Yield);
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    assert_eq!(mock.cancel_admissions(), 1, "ordinary cancel failure must consume the sole admission without retry");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(b"worker-survived".to_vec());
        }),
    );
    assert_eq!(receiver.await.expect("one-worker competitor after cancel failure"), b"worker-survived");
    pool.shutdown().expect("worker shutdown");

    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("quarantined mounted route must reject without entering the guest");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions, "quarantine rejection must not enter start-job");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn drop_cleanup_cancel_failure_quarantines_before_the_next_mounted_route() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_010)).await;
    let cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), cancel.clone(), cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;
    mock.fail_next_cancel();

    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    assert_eq!(mock.cancel_admissions(), 1, "Drop cleanup must not retry an ordinary cancel failure");
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("one-worker permit and worker survive Drop cleanup failure");
    pool.shutdown().expect("worker shutdown");

    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("stored Drop cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn start_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_011)).await;
    mock.fail_next_start();
    mock.fail_next_cancel();

    let first = handle.infer(b"first", &semio_framework_job::root_cancel_token()).await.expect_err("ordinary start failure must fault");
    assert!(first.to_string().contains("guest trapped: scripted start-job failure"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("start-failure cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn step_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_012);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_fault(actor, "scripted step-job failure").await;
    mock.fail_next_cancel();

    let first = handle.infer(b"first", &semio_framework_job::root_cancel_token()).await.expect_err("ordinary step failure must fault");
    assert!(first.to_string().contains("guest trapped: scripted step-job failure"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route", &semio_framework_job::root_cancel_token()).await.expect_err("step-failure cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1);
}
