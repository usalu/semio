/// 🛣️ Concurrent routes and poisoned slot: the concurrent route refusals while a cleanup is pending, the poisoned instance slot recovery and the relay drive policy.

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_start_failure_cleanup_pending_then_reuses_only_after_cleanup_success() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_013);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_start_failure_release();
    mock.fail_next_start();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    assert_eq!(mock.cancel_admissions(), 0, "the barrier must precede cleanup scheduling");
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must reject promptly");
    assert!(second_error.to_string().contains("cleanup pending after start-job failure"));
    assert_eq!(mock.start_admissions(), 1, "cleanup-pending rejection must not re-enter start-job");

    release.release();
    let first_error = first.await.expect("failed producer result").expect_err("ordinary start fault");
    assert!(first_error.to_string().contains("scripted start-job failure"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    mock.script_job_step(actor, JobStep::Done { output: b"clean-reuse".to_vec() }).await;
    assert_eq!(handle.infer(b"after-cleanup", &semio_framework_job::root_cancel_token()).await.expect("cleanup success restores mounted availability"), b"clean-reuse");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_step_failure_cleanup_pending_then_quarantine_is_stable() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_014);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_step_failure_release();
    mock.script_fault(actor, "scripted concurrent step-job failure").await;
    mock.fail_next_cancel();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must not enter the guest");
    assert!(second_error.to_string().contains("cleanup pending after step-job failure"));
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1);

    release.release();
    let first_error = first.await.expect("failed producer result").expect_err("ordinary step fault");
    assert!(first_error.to_string().contains("scripted concurrent step-job failure"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let third_error = handle.infer(b"after-failed-cleanup", &semio_framework_job::root_cancel_token()).await.expect_err("cancel failure must leave a stable quarantine");
    assert!(third_error.to_string().contains("quarantined after cancel-job failure"));
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_retained_start_panic_cleanup_pending_before_recovery() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_015);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_start_failure_release();
    mock.panic_next_start();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("panic cleanup-pending route must reject promptly");
    assert!(second_error.to_string().contains("cleanup pending after guest relay panic"));
    assert_eq!(mock.start_admissions(), 1, "panic cleanup-pending rejection must not re-enter start-job");

    release.release();
    let first_error = first.await.expect("panicking producer result").expect_err("retained start panic must fault");
    assert!(first_error.to_string().contains("plugin guest relay panicked"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    mock.script_job_step(actor, JobStep::Done { output: b"panic-clean-reuse".to_vec() }).await;
    assert_eq!(handle.infer(b"after-panic-cleanup", &semio_framework_job::root_cancel_token()).await.expect("panic cleanup success restores availability"), b"panic-clean-reuse");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn poisoned_instance_slot_recovers_without_losing_the_mounted_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_007);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"poison-recovered".to_vec() }).await;
    let instance = Arc::clone(&handle.instance);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = instance.lock().expect("unpoisoned fixture slot");
        panic!("scripted instance-slot poison");
    }));
    assert!(handle.instance.is_poisoned());
    assert_eq!(handle.infer(b"request", &semio_framework_job::root_cancel_token()).await.expect("poison recovery must preserve the resident instance"), b"poison-recovered");
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
}

fn test_relay_drive_policy()->semio_framework_value::RetainedCloneGrant{
 serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../../../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["driveGrant"].clone()).unwrap()
}
