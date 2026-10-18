/// 🪂️ Dropped futures, wake fallback and cancellation: the dropped pending mounted future, the coalesced wake fallback, the pending guest step, the cancellation race, the dropped live relay, the caller cancel and the finished infer.

#[semio_framework_async_macros::async_test]
async fn dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll() {
    let fixture = relay_lifecycle_fixture();
    let expected_cancels = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-abandoned-blocked-wake").and_then(|trace| trace["expected"]["cancelAdmissions"].as_u64()).expect("cancel trace");
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_016);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, gate, instance) = relay_session(Arc::clone(&mock), actor, pool.clone(), cancel.clone(), JobStep::Done { output: b"abandoned".to_vec() }).await;
    let cancel_gate = mock.script_pending_cancel_job();
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),test_relay_wake_authority()));
    let (index, generation) = registry.reserve().expect("mounted relay slot");
    registry.mount(index, generation, GuestRelayMountedOwner::Session(session));
    let mut future = Box::pin(GuestRelayMountedFuture { registry: Arc::clone(&registry), index, generation, complete: false });
    let mut admitted = false;
    for _ in 0..64 {
        let poll = std::future::poll_fn(|context| std::task::Poll::Ready(std::future::Future::poll(future.as_mut(), context))).await;
        assert!(poll.is_pending(), "gated mounted relay remains pending");
        if mock.step_admissions() == 1 && gate.is_waiting() {
            admitted = true;
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(admitted, "the pending guest step must be owned before detachment");
    drop(future);
    assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap));
    for _ in 0..64 {
        if registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst) == 1 {
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert_eq!(registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst), 1, "the detached session parks once on its relay-completion waker");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "a wake-capable detached session needs no timer fallback");
    let blocked_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst);
    for _ in 0..8 {
        pool_timer_barrier(&pool).await;
    }
    assert_eq!(registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst), blocked_polls, "an unwoken detached session performs zero close rechecks");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "an unwoken detached session creates zero timers");
    cancel_gate.release();
    let mut reaped = false;
    for _ in 0..256 {
        if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) {
            reaped = true;
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(reaped, "the registry-owned reaper must release the detached slot without foreground pump");
    let post_wake_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst) - blocked_polls;
    assert!((1..=64).contains(&post_wake_polls), "one actual wake permits only a finite close sequence, got {post_wake_polls}");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-driven close remains timer-free through terminal release");
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &instance).await;
    assert!(cancel.is_cancelled_now());
    assert_eq!(mock.cancel_admissions(), expected_cancels as usize);
    assert_eq!(mock.step_admissions(), 1);
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn wake_incapable_close_uses_one_coalesced_bounded_fallback() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),test_relay_wake_authority()));
    registry.park_blocked_reaper(std::task::Waker::noop(), true);
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-capable owners never enter fallback timing");
    registry.park_blocked_reaper(std::task::Waker::noop(), false);
    registry.park_blocked_reaper(std::task::Waker::noop(), false);
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "a wake-incapable owner receives one coalesced fallback");
    assert!(registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire));
    for _ in 0..64 {
        if !registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire) {
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(!registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire), "the bounded fallback relinquishes its pending ownership");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "the fallback never duplicates itself while coalesced");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn pending_guest_releases_the_only_worker_and_admits_no_duplicate_step() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_001), pool.clone(), cancel, JobStep::Done { output: b"done".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;

    for _ in 0..8 {
        assert_eq!(test_relay_step(&mut session, &pool).await, TestRelayOutcome::Yield);
    }
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1, "pending caller turns must only try-receive the admitted guest future");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(b"competitor-progress".to_vec());
        }),
    );
    assert_eq!(receiver.await.expect("competing user-visible job"), b"competitor-progress", "a genuinely pending guest must not park the single worker");

    gate.release();
    let terminal = loop {
        let outcome = test_relay_step(&mut session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Complete(b"done".to_vec()));
    assert_eq!(test_relay_step(&mut session, &pool).await, TestRelayOutcome::Yield);
    assert_eq!(mock.step_admissions(), 1, "neither pending polls nor terminal replay may duplicate guest admission");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_002), pool.clone(), cancel.clone(), JobStep::Done { output: b"raced".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;

    cancel.cancel_now();
    let terminal = loop {
        let outcome = test_relay_step(&mut session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Cancelled);
    assert_eq!(test_relay_step(&mut session, &pool).await, TestRelayOutcome::Yield, "the terminal cancellation must not be delivered twice");
    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1, "the completion/cancellation race must admit guest cancellation exactly once");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn dropping_a_live_nonterminal_relay_cancels_the_guest_exactly_once() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (mut session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_003), pool.clone(), cancel.clone(), JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&mut session, &pool, &mock).await;

    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    assert!(cancel.is_cancelled_now(), "dropping an admitted nonterminal relay must cancel its owned scope before scheduling guest cleanup");
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1, "the pending-step and Drop cleanup paths share one cancellation admission bit");
    pool.shutdown().expect("worker shutdown");
}

/// 🛑️ `inference_run`'s job cancel: the caller's token reaches a mounted `semio.infer` route whose
/// guest step is still pending, the route answers a cancellation instead of the solve, and the guest
/// sees exactly one cancel admission.
#[semio_framework_async_macros::async_test]
async fn a_callers_cancel_stops_a_mounted_infer_blocked_in_a_pending_guest_step() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_020);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let _gate = mock.script_pending_job_step(actor, JobStep::Done { output: b"never-delivered".to_vec() }).await;
    let pool = plugin_host_worker_pool();
    let caller = semio_framework_async::CancelToken::root_now();
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    let (route_handle, route_cancel) = (Arc::clone(&handle), caller.clone());
    GuestRelayPoolFuture::spawn_recoverable(
        pool.clone(),
        Lane::UserVisible,
        async move {
            let _ = sender.send(route_handle.infer(b"blocked", &route_cancel).await);
        },
        |_| {},
    );
    for _ in 0..256 {
        if mock.step_admissions() == 1 {
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert_eq!(mock.step_admissions(), 1, "the guest step must be admitted and pending before the cancel");
    caller.cancel_now();
    let error = receiver.await.expect("the mounted route answers").expect_err("a cancelled route must not answer the solve");
    assert!(error.to_string().contains("cancelled"), "{error}");
    wait_for_cancel_admission(&pool, &mock).await;
    assert_eq!(mock.cancel_admissions(), 1);
}

/// 🧬️ The relay runs under a CHILD of the caller's token, so closing a finished route never cancels
/// the caller — a dependency chain reuses one token across several routes.
#[semio_framework_async_macros::async_test]
async fn a_finished_mounted_infer_leaves_the_callers_token_live() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_021);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"first".to_vec() }).await;
    let caller = semio_framework_async::CancelToken::root_now();
    assert_eq!(handle.infer(b"one", &caller).await.expect("first route"), b"first");
    assert!(!caller.is_cancelled_now(), "closing the relay must not cancel the caller's token");
    mock.script_job_step(actor, JobStep::Done { output: b"second".to_vec() }).await;
    assert_eq!(handle.infer(b"two", &caller).await.expect("second route under the same token"), b"second");
}
