
use super::*;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;

//#region 🛑️CancelTokenTests
#[semio_framework_async_macros::async_test]
async fn cancel_token_root_starts_live() {
    let token = CancelToken::root().await;
    assert!(token.is_live().await);
    assert!(!token.is_cancelled().await);
    assert!(!token.is_parked().await);
}

#[semio_framework_async_macros::async_test]
async fn cancel_token_park_then_unpark_returns_to_live() {
    let token = CancelToken::root().await;
    token.park().await;
    assert!(token.is_parked().await);
    token.unpark().await;
    assert!(token.is_live().await);
}

#[semio_framework_async_macros::async_test]
async fn cancel_token_cancel_is_terminal_over_park() {
    let token = CancelToken::root().await;
    token.park().await;
    token.cancel().await;
    token.unpark().await;
    assert!(token.is_cancelled().await, "cancel must never be undone by a later park/unpark");
}

#[semio_framework_async_macros::async_test]
async fn cancelling_parent_transitively_cancels_child_and_grandchild() {
    let root = CancelToken::root().await;
    let child = root.child().await;
    let grandchild = child.child().await;
    assert!(root.is_live().await && child.is_live().await && grandchild.is_live().await);
    root.cancel().await;
    assert!(root.is_cancelled().await);
    assert!(child.is_cancelled().await, "child must observe parent cancellation");
    assert!(grandchild.is_cancelled().await, "grandchild must observe ancestor cancellation transitively");
}

#[semio_framework_async_macros::async_test]
async fn parking_parent_does_not_downgrade_an_already_live_reading_below_park() {
    let root = CancelToken::root().await;
    let child = root.child().await;
    root.park().await;
    assert_eq!(child.state().await, CancelState::Park, "child inherits parent's park via max-severity fold");
    assert_eq!(child.0.local.load(Ordering::SeqCst), CancelState::Live.to_u8().await, "child's OWN local state is untouched by the parent's park");
}

#[semio_framework_async_macros::async_test]
async fn child_cancel_never_propagates_upward_to_parent() {
    let root = CancelToken::root().await;
    let child = root.child().await;
    child.cancel().await;
    assert!(child.is_cancelled().await);
    assert!(root.is_live().await, "cancelling a child must never cancel its parent");
}
//#endregion 🛑️CancelTokenTests

//#region 🌳️ScopeTests
#[semio_framework_async_macros::async_test]
async fn scope_handle_child_scope_shares_cancellation_lineage() {
    use testkit::ManualRuntime;
    let runtime = ManualRuntime::new(0).await;
    let parent = runtime.open_scope(ScopeOwner::Service("test-parent"), None).await;
    let child = runtime.open_scope(ScopeOwner::Service("test-child"), Some(&parent)).await;
    parent.cancel.cancel().await;
    assert!(child.cancel.is_cancelled().await);
}
//#endregion 🌳️ScopeTests

//#region 🧪️ManualRuntimeTests
#[semio_framework_async_macros::async_test]
async fn manual_runtime_spawn_scoped_runs_a_ready_future_on_drive() {
    use testkit::ManualRuntime;
    let runtime = ManualRuntime::new(0).await;
    let scope = runtime.open_scope(ScopeOwner::Actor(1), None).await;
    let ctx = OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) })).await;
    assert_eq!(runtime.drive().await, 1);
    assert!(ran.load(Ordering::SeqCst));
    assert_eq!(runtime.pending_task_count().await, 0);
}

#[semio_framework_async_macros::async_test]
async fn manual_runtime_sleep_until_resolves_only_after_injected_time_advances() {
    use testkit::ManualRuntime;
    let runtime = ManualRuntime::new(0).await;
    let scope = runtime.open_scope(ScopeOwner::Service("timer"), None).await;
    let ctx = OperationContext { actor: 0, generation: 0, trace: TraceId(2), lane: 0, deadline_ms: Some(100), cancel: scope.cancel.clone(), capability: None };
    let woke = Arc::new(AtomicBool::new(false));
    let woke_clone = woke.clone();
    let runtime_for_future = runtime.clone();
    runtime
        .spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                runtime_for_future.sleep_until(100).await;
                woke_clone.store(true, Ordering::SeqCst);
            }),
        )
        .await;
    runtime.drive().await;
    assert!(!woke.load(Ordering::SeqCst), "must not resolve before the injected clock reaches the deadline");
    runtime.set_now_ms(100).await;
    runtime.drive().await;
    assert!(woke.load(Ordering::SeqCst), "must resolve once the injected clock reaches the deadline");
}

#[semio_framework_async_macros::async_test]
async fn manual_runtime_drive_observes_a_real_cross_thread_wake() {
    use testkit::ManualRuntime;
    let runtime = ManualRuntime::new(0).await;
    let scope = runtime.open_scope(ScopeOwner::Service("cross-thread-wake"), None).await;
    let ctx = OperationContext { actor: 0, generation: 0, trace: TraceId(5), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
    let (tx, rx) = oneshot::channel::<u32>();
    let observed = Arc::new(AtomicU32::new(0));
    let observed_for_task = observed.clone();
    runtime
        .spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                observed_for_task.store(rx.await.expect("sender must complete"), Ordering::SeqCst);
            }),
        )
        .await;
    let sender = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(20));
        tx.send(42).expect("receiver must remain alive");
    });
    assert_eq!(runtime.drive().await, 1);
    sender.join().expect("sender thread must not panic");
    assert_eq!(observed.load(Ordering::SeqCst), 42);
    assert_eq!(runtime.pending_task_count().await, 0);
}

#[semio_framework_async_macros::async_test]
async fn manual_runtime_cancel_scope_reports_finished_and_cancelled() {
    use testkit::ManualRuntime;
    let runtime = ManualRuntime::new(0).await;
    let scope = runtime.open_scope(ScopeOwner::Actor(7), None).await;
    let ctx = OperationContext { actor: 7, generation: 0, trace: TraceId(3), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
    runtime.spawn_scoped(&scope, ctx.clone(), Box::pin(async move {})).await;
    runtime.drive().await;
    let ctx2 = OperationContext { actor: 7, generation: 0, trace: TraceId(4), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
    let runtime_for_future = runtime.clone();
    runtime.spawn_scoped(&scope, ctx2, Box::pin(async move { runtime_for_future.sleep_until(u64::MAX).await })).await;
    let report = runtime.cancel_scope(&scope.owner, 0).await;
    assert_eq!(report.finished, 1);
    assert_eq!(report.cancelled, 1);
}
//#endregion 🧪️ManualRuntimeTests

//#region 🌉️BlockOnTests
/// 🐢️ A plain, non-`async` `#[test]` (not `#[async_test]`) proving [`block_on`] genuinely drives
/// a future through several `Poll::Pending`s before it completes, rather than only working on a
/// future that happens to be ready on the first poll.
#[test]
fn block_on_drives_a_future_through_several_pending_polls_before_completing() {
    use std::sync::atomic::AtomicU32;
    use std::task::{Context, Poll};

    struct Countdown(AtomicU32);
    impl Future for Countdown {
        type Output = u32;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
            let polls_left = self.0.fetch_sub(1, Ordering::SeqCst);
            if polls_left == 0 {
                Poll::Ready(42)
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let value = block_on(Countdown(AtomicU32::new(4)));
    assert_eq!(value, 42);
}
//#endregion 🌉️BlockOnTests

//#region 🏭️ProcessKindTests
#[test]
fn worker_count_interactive_native_reserves_one_core() {
    assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 10), 9);
    assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 2), 1);
    assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 1), 1, "a single-core interactive host must still get one worker, never zero");
}

#[test]
fn worker_count_headless_batch_uses_every_core() {
    assert_eq!(worker_count_for(ProcessKind::HeadlessBatch, 10), 10);
    assert_eq!(worker_count_for(ProcessKind::HeadlessBatch, 1), 1);
}
//#endregion 🏭️ProcessKindTests

//#region ⚖️PermitLedgerTests
#[test]
fn permit_ledger_checkout_debits_and_release_credits() {
    let ledger = PermitLedger::new(4);
    let guard = ledger.checkout(3).expect("3 of 4 permits must be available");
    assert_eq!(ledger.remaining(), 1);
    assert_eq!(ledger.occupancy(), 3);
    drop(guard);
    assert_eq!(ledger.remaining(), 4);
    assert_eq!(ledger.occupancy(), 0);
}

#[test]
fn permit_ledger_over_allocation_returns_err_never_wraps() {
    let ledger = PermitLedger::new(2);
    let _guard = ledger.checkout(2).expect("exactly the full ledger must be checkoutable");
    let error = ledger.checkout(1).expect_err("over-allocation must return Err, never wrap the counter");
    assert_eq!(error, PermitError { requested: 1, remaining: 0 });
    assert_eq!(ledger.remaining(), 0, "a failed checkout must never mutate the remaining count");
}

/// 🚫️ The Phase 0 gate-report defect this type exists to close: `ThreadBudget::checkout` used
/// `fetch_sub` + `debug_assert!`, so a release build silently wrapped on over-draw. This test
/// runs under BOTH `cargo test` and `cargo test --release` (no `#[cfg(debug_assertions)]` guard
/// anywhere on this test or on `PermitLedger::checkout`) — proving the checked behavior holds in
/// release, not just in debug.
#[test]
fn permit_ledger_checked_in_release_too() {
    let ledger = PermitLedger::new(1);
    assert!(ledger.checkout(5).is_err());
    assert_eq!(ledger.remaining(), 1, "release build must never wrap: remaining stays exactly what it started at");
}
//#endregion ⚖️PermitLedgerTests

//#region 🛣️LaneTests
#[test]
fn lane_from_context_lane_mirrors_actor_lane_discriminant_order() {
    assert_eq!(Lane::from_context_lane(0), Lane::Interactive);
    assert_eq!(Lane::from_context_lane(1), Lane::UserVisible);
    assert_eq!(Lane::from_context_lane(2), Lane::Background);
    assert_eq!(Lane::from_context_lane(3), Lane::Maintenance);
}

#[test]
fn lane_weights_never_starve_the_lowest_lane() {
    assert!(Lane::Maintenance.weight() >= 1, "every lane must accrue SOME deficit every scan, or it would never run");
    assert!(Lane::Interactive.weight() > Lane::Maintenance.weight());
}
//#endregion 🛣️LaneTests

//#region ⏰️TimerWheelTests
#[test]
fn timer_wheel_next_deadline_ms_reflects_earliest_pending() {
    let wheel = TimerWheel::new();
    assert_eq!(wheel.next_deadline_ms(), None);
    let waker = Waker::noop();
    wheel.register(200, waker.clone());
    wheel.register(50, waker.clone());
    wheel.register(300, waker.clone());
    assert_eq!(wheel.next_deadline_ms(), Some(50));
}

#[test]
fn timer_wheel_fires_deadlines_in_order() {
    let wheel = Arc::new(TimerWheel::new());
    let order = Arc::new(Mutex::new(Vec::new()));

    struct RecordingWaker {
        label: u32,
        order: Arc<Mutex<Vec<u32>>>,
    }
    impl std::task::Wake for RecordingWaker {
        fn wake(self: Arc<Self>) {
            self.order.lock().unwrap().push(self.label);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.order.lock().unwrap().push(self.label);
        }
    }

    let waker_a = Waker::from(Arc::new(RecordingWaker { label: 1, order: order.clone() }));
    let waker_b = Waker::from(Arc::new(RecordingWaker { label: 2, order: order.clone() }));
    let waker_c = Waker::from(Arc::new(RecordingWaker { label: 3, order: order.clone() }));
    wheel.register(300, waker_c);
    wheel.register(100, waker_a);
    wheel.register(200, waker_b);

    assert_eq!(wheel.fire_due(150), 1);
    assert_eq!(*order.lock().unwrap(), vec![1]);
    assert_eq!(wheel.fire_due(250), 1);
    assert_eq!(*order.lock().unwrap(), vec![1, 2]);
    assert_eq!(wheel.fire_due(1000), 1);
    assert_eq!(*order.lock().unwrap(), vec![1, 2, 3]);
}

#[test]
fn timer_wheel_callback_batch_preserves_due_remainder() {
    let wheel = TimerWheel::new();
    let fired = Arc::new(AtomicU32::new(0));
    for _ in 0..40 {
        let fired = fired.clone();
        wheel.schedule_callback(
            10,
            Box::new(move || {
                fired.fetch_add(1, Ordering::SeqCst);
            }),
        );
    }
    assert_eq!(wheel.fire_due_batch(10, 32), 32);
    assert_eq!(fired.load(Ordering::SeqCst), 32);
    assert_eq!(wheel.next_deadline_ms(), Some(10));
    assert_eq!(wheel.fire_due_batch(10, 32), 8);
    assert_eq!(fired.load(Ordering::SeqCst), 40);
}

#[semio_framework_async_macros::async_test]
async fn timer_wheel_sleep_until_resolves_only_after_fire_due_reaches_deadline() {
    let wheel = Arc::new(TimerWheel::new());
    let (tx, rx) = mpsc::channel();
    let wheel_for_task = wheel.clone();
    std::thread::spawn(move || {
        block_on(async move {
            wheel_for_task.sleep_until(50).await;
            tx.send(()).expect("send must succeed");
        });
    });
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert!(rx.try_recv().is_err(), "must not resolve before fire_due reaches the deadline");
    wheel.fire_due(50);
    rx.recv_timeout(std::time::Duration::from_secs(5)).expect("sleep_until must resolve once fire_due reaches the deadline");
}
//#endregion ⏰️TimerWheelTests

//#region 🪢️OneshotTests
#[test]
fn oneshot_send_before_poll_is_observed_by_try_recv() {
    let (tx, mut rx) = oneshot::channel::<u32>();
    tx.send(7).expect("send must succeed while the receiver is alive");
    assert_eq!(rx.try_recv(), Ok(7));
}

#[test]
fn oneshot_try_recv_reports_empty_then_closed() {
    let (tx, mut rx) = oneshot::channel::<u32>();
    assert_eq!(rx.try_recv(), Err(oneshot::TryRecvError::Empty));
    drop(tx);
    assert_eq!(rx.try_recv(), Err(oneshot::TryRecvError::Closed));
}

#[test]
fn oneshot_send_after_receiver_dropped_returns_the_value_back() {
    let (tx, rx) = oneshot::channel::<u32>();
    drop(rx);
    assert_eq!(tx.send(9), Err(9));
}

#[semio_framework_async_macros::async_test]
async fn oneshot_receiver_await_resolves_once_sent_across_a_real_thread() {
    let (tx, rx) = oneshot::channel::<u32>();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(20));
        tx.send(42).expect("send must succeed");
    });
    assert_eq!(rx.await, Ok(42));
}

#[semio_framework_async_macros::async_test]
async fn oneshot_receiver_await_reports_recv_error_when_sender_dropped_without_sending() {
    let (tx, rx) = oneshot::channel::<u32>();
    drop(tx);
    assert_eq!(rx.await, Err(oneshot::RecvError));
}
//#endregion 🪢️OneshotTests

//#region 🔔️NotifyTests
#[test]
fn notify_one_before_notified_stores_a_permit() {
    let notify = Notify::new();
    notify.notify_one();
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut fut = std::pin::pin!(notify.notified());
    assert_eq!(fut.as_mut().poll(&mut cx), Poll::Ready(()));
}

#[semio_framework_async_macros::async_test]
async fn notify_one_wakes_an_already_pending_notified_across_a_real_thread() {
    let notify = Arc::new(Notify::new());
    let notify_for_task = notify.clone();
    let (tx, rx) = mpsc::channel::<()>();
    std::thread::spawn(move || {
        block_on(async move {
            notify_for_task.notified().await;
            tx.send(()).expect("send must succeed");
        });
    });
    std::thread::sleep(std::time::Duration::from_millis(20));
    notify.notify_one();
    rx.recv_timeout(std::time::Duration::from_secs(5)).expect("notify_one must wake an already-pending notified()");
}
//#endregion 🔔️NotifyTests

//#region 🚦️SemaphoreTests
#[semio_framework_async_macros::async_test]
async fn semaphore_sequential_acquire_release_never_exceeds_capacity() {
    let sem = Arc::new(Semaphore::new(1));
    let permit = sem.acquire_owned().await;
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut second = std::pin::pin!(sem.acquire_owned());
    assert!(matches!(second.as_mut().poll(&mut cx), Poll::Pending), "a second acquire must not be admitted while the one permit is held");
    drop(permit);
    assert!(matches!(second.as_mut().poll(&mut cx), Poll::Ready(_)), "releasing the held permit must admit the pending acquire");
}

/// 🚨️ Cancellation proof: an `acquire_owned` future polled once (registering a waker), then
/// DROPPED before it ever resolves — this is exactly what [`select2`] does to the losing branch
/// of a deadline race. A leaked waker entry would neither break correctness nor deadlock this
/// specific test, but it is exactly the kind of stale-registration bug this packet's brief warns
/// about, so this proves the drop path actually removes its own entry: a THIRD acquire, submitted
/// after the cancelled one, must still be admitted promptly once the first permit is released.
#[semio_framework_async_macros::async_test]
async fn semaphore_acquire_dropped_while_pending_does_not_leak_or_block_a_later_acquire() {
    let sem = Arc::new(Semaphore::new(1));
    let permit = sem.acquire_owned().await;
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    {
        let mut cancelled = std::pin::pin!(sem.acquire_owned());
        assert!(matches!(cancelled.as_mut().poll(&mut cx), Poll::Pending));
    }
    drop(permit);
    let third = sem.acquire_owned().await;
    drop(third);
}

/// 🚨️ Stress test: many real OS threads hammering acquire/release in a tight loop must never
/// observe more concurrently-held permits than the semaphore's own capacity, and the process must
/// not deadlock (the test itself times out via `join` never returning if it does).
#[test]
fn semaphore_never_exceeds_capacity_under_concurrent_stress() {
    const CAPACITY: usize = 3;
    const THREADS: usize = 12;
    const ITERATIONS: usize = 400;
    let sem = Arc::new(Semaphore::new(CAPACITY));
    let current = Arc::new(AtomicU32::new(0));
    let observed_max = Arc::new(AtomicU32::new(0));
    let handles: Vec<_> = (0..THREADS)
        .map(|_| {
            let sem = sem.clone();
            let current = current.clone();
            let observed_max = observed_max.clone();
            std::thread::spawn(move || {
                block_on(async move {
                    for _ in 0..ITERATIONS {
                        let permit = sem.acquire_owned().await;
                        let now = current.fetch_add(1, Ordering::SeqCst) + 1;
                        observed_max.fetch_max(now, Ordering::SeqCst);
                        current.fetch_sub(1, Ordering::SeqCst);
                        drop(permit);
                    }
                });
            })
        })
        .collect();
    for handle in handles {
        handle.join().expect("stress thread must not panic");
    }
    assert!(observed_max.load(Ordering::SeqCst) as usize <= CAPACITY, "observed concurrency {} exceeded capacity {}", observed_max.load(Ordering::SeqCst), CAPACITY);
}
//#endregion 🚦️SemaphoreTests

//#region 🔀️Select2Tests
#[semio_framework_async_macros::async_test]
async fn select2_returns_the_ready_branch_and_drops_the_other() {
    struct DropFlag(Arc<AtomicBool>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    let loser_dropped = Arc::new(AtomicBool::new(false));
    let flag = DropFlag(loser_dropped.clone());
    let ready = async { 5u32 };
    let never = async move {
        let _flag = flag;
        std::future::pending::<u32>().await
    };
    match select2(ready, never).await {
        Either::Left(value) => assert_eq!(value, 5),
        Either::Right(_) => panic!("the immediately-ready branch must win"),
    }
    assert!(loser_dropped.load(Ordering::SeqCst), "the losing branch must be dropped once the race resolves");
}

#[semio_framework_async_macros::async_test]
async fn select2_right_branch_wins_when_it_resolves_first() {
    let (tx, rx) = oneshot::channel::<u32>();
    drop(tx);
    match select2(std::future::pending::<()>(), rx).await {
        Either::Left(_) => panic!("the pending branch must never win"),
        Either::Right(result) => assert_eq!(result, Err(oneshot::RecvError)),
    }
}
//#endregion 🔀️Select2Tests

//#region 🤝️CooperateTests
#[semio_framework_async_macros::async_test]
async fn join2_preserves_poll_order_and_yield_once_hands_off_one_turn() {
    let order = Arc::new(Mutex::new(Vec::new()));
    let left_order = order.clone();
    let right_order = order.clone();
    let (left, right) = join2(
        async move {
            left_order.lock().expect("order mutex").push("left-start");
            yield_once().await;
            left_order.lock().expect("order mutex").push("left-finish");
            7u8
        },
        async move {
            right_order.lock().expect("order mutex").push("right");
            9u8
        },
    )
    .await;
    assert_eq!((left, right), (7, 9));
    assert_eq!(*order.lock().expect("order mutex"), ["left-start", "right", "left-finish"]);
}
//#endregion 🤝️CooperateTests

//#region 🗺️ShardedMapTests
#[test]
fn sharded_map_get_or_insert_mutate_iterate_and_remove_are_guard_scoped() {
    let map = ShardedMap::<String, usize, 4>::new();
    assert_eq!(map.get_or_insert_with_cloned("alpha".into(), || 1), 1);
    assert_eq!(map.get_or_insert_with_cloned("alpha".into(), || 99), 1);
    map.mutate_or_default("alpha".into(), |value| *value += 2);
    map.insert("beta".into(), 7);
    assert_eq!(map.get_cloned("alpha"), Some(3));
    let mut entries = Vec::new();
    map.for_each(|key, value| entries.push((key.clone(), *value)));
    entries.sort();
    assert_eq!(entries, [("alpha".into(), 3), ("beta".into(), 7)]);
    assert!(!map.remove_if("alpha", |value| *value == 2));
    assert!(map.remove_if("alpha", |value| *value == 3));
    assert_eq!(map.remove("beta"), Some(7));
    assert!(map.is_empty());
}

#[test]
fn sharded_map_accepts_concurrent_disjoint_writers() {
    let map = Arc::new(ShardedMap::<u64, u64, 16>::new());
    let mut writers = Vec::new();
    for worker in 0..8u64 {
        let map = Arc::clone(&map);
        writers.push(std::thread::spawn(move || {
            for offset in 0..128u64 {
                let key = worker * 128 + offset;
                map.insert(key, key * 2);
            }
        }));
    }
    for writer in writers {
        writer.join().expect("sharded map writer");
    }
    assert_eq!(map.len(), 1024);
    assert_eq!(map.get_cloned(&777), Some(1554));
}
//#endregion 🗺️ShardedMapTests

//#region 🧵️WorkerPoolTests
#[test]
fn process_worker_pool_is_singleton_across_subsystem_requests() {
    let config = WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2);
    let first = process_worker_pool(config);
    let second = process_worker_pool(config);
    assert!(first.is_same_pool(&second));
    assert_eq!(first.worker_count(), second.worker_count());
    assert!(std::panic::catch_unwind(|| process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 8))).is_err());
}

#[test]
fn worker_pool_sizing_multi_core_and_forced_single_core() {
    let multi = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 8));
    assert_eq!(multi.worker_count(), 7);
    multi.shutdown();

    let single = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 1));
    assert_eq!(single.worker_count(), 1, "a forced single-core host must still get exactly one worker");
    single.shutdown();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn worker_pool_try_submit_preserves_exact_finite_saturation_authority() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let (started_tx, started_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    pool.submit(
        Lane::Maintenance,
        Box::new(move || {
            started_tx.send(()).expect("blocking worker start");
            release_rx.recv().expect("blocking worker release");
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("the only worker must be occupied");
    for _ in 0..WORKER_JOBS_PER_LANE {
        assert!(pool.try_submit(Lane::Maintenance, Box::new(|| {})).is_ok(), "every pre-admitted lane slot must accept exactly one closure");
    }
    let failure = match pool.try_submit(Lane::Maintenance, Box::new(|| {})) {
        Ok(()) => panic!("the first closure past the fixed cap must be returned"),
        Err(failure) => failure,
    };
    assert_eq!(failure.kind(), WorkerSubmitErrorKind::Saturated);
    drop(failure.into_job());
    release_tx.send(()).expect("release blocking worker");
    pool.shutdown();
}

#[test]
fn worker_pool_runs_submitted_jobs_across_all_lanes() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
    let (tx, rx) = mpsc::channel();
    for lane in Lane::ALL {
        let tx = tx.clone();
        pool.submit(lane, Box::new(move || tx.send(lane).expect("send must succeed")));
    }
    let mut seen = Vec::new();
    for _ in 0..LANE_COUNT {
        seen.push(rx.recv_timeout(std::time::Duration::from_secs(5)).expect("every submitted job must eventually run"));
    }
    seen.sort_by_key(|lane| *lane as usize);
    let mut expected = Lane::ALL.to_vec();
    expected.sort_by_key(|lane| *lane as usize);
    assert_eq!(seen, expected);
    pool.shutdown();
}

#[test]
fn worker_pool_work_stealing_moves_work_between_workers() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
    let (start_tx, start_rx) = mpsc::channel::<()>();
    let (release_tx, release_rx) = mpsc::channel::<()>();
    let release_rx = Arc::new(Mutex::new(release_rx));

    pool.submit(
        Lane::Background,
        Box::new(move || {
            start_tx.send(()).expect("send must succeed");
            let _ = release_rx.lock().expect("mutex").recv();
        }),
    );
    start_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("the blocking job must start on some worker");

    let (done_tx, done_rx) = mpsc::channel();
    for _ in 0..3 {
        let done_tx = done_tx.clone();
        pool.submit(Lane::Background, Box::new(move || done_tx.send(()).expect("send must succeed")));
    }
    for _ in 0..3 {
        done_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("other workers must steal and finish this work even while one worker is blocked");
    }
    release_tx.send(()).expect("send must succeed");
    pool.shutdown();
}

#[test]
fn worker_pool_lane_fairness_background_cannot_starve_interactive() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let keep_busy = Arc::new(AtomicBool::new(true));
    for _ in 0..2 {
        let keep_busy = keep_busy.clone();
        let pool_for_resubmit = pool.clone();
        pool.submit(Lane::Background, Box::new(move || saturate_background(&pool_for_resubmit, keep_busy)));
    }
    let (tx, rx) = mpsc::channel();
    pool.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("send must succeed")));
    let result = rx.recv_timeout(std::time::Duration::from_secs(10));
    keep_busy.store(false, Ordering::SeqCst);
    pool.shutdown();
    result.expect("a saturated background lane must never starve an interactive job indefinitely");
}

fn saturate_background(pool: &WorkerPool, keep_busy: Arc<AtomicBool>) {
    if keep_busy.load(Ordering::SeqCst) {
        let pool_for_resubmit = pool.clone();
        pool.submit(Lane::Background, Box::new(move || saturate_background(&pool_for_resubmit, keep_busy)));
    }
}

#[test]
fn worker_pool_admission_control_keeps_an_interactive_slot_free() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 3));
    assert!(pool.worker_count() >= 2, "this test needs 2+ workers to exercise the reserve");
    let release = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = mpsc::channel();
    let (finished_tx, finished_rx) = mpsc::channel();
    for _ in 0..pool.worker_count() {
        let release = release.clone();
        let started_tx = started_tx.clone();
        let finished_tx = finished_tx.clone();
        pool.submit(
            Lane::Maintenance,
            Box::new(move || {
                started_tx.send(()).expect("start signal must succeed");
                let (lock, changed) = &*release;
                let mut released = lock.lock().expect("release mutex poisoned");
                while !*released {
                    released = changed.wait(released).expect("release mutex poisoned");
                }
                finished_tx.send(()).expect("finish signal must succeed");
            }),
        );
    }
    for _ in 0..(pool.worker_count() - 1) {
        started_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("every non-reserved worker must admit one low-priority job");
    }
    assert!(started_rx.recv_timeout(std::time::Duration::from_millis(50)).is_err(), "low-priority work must not race past the atomic reserve ceiling");
    let (tx, rx) = mpsc::channel();
    pool.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("send must succeed")));
    rx.recv_timeout(std::time::Duration::from_secs(5)).expect("the reserved worker must run interactive work");
    {
        let (lock, changed) = &*release;
        *lock.lock().expect("release mutex poisoned") = true;
        changed.notify_all();
    }
    for _ in 0..pool.worker_count() {
        finished_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("all low-priority jobs must finish after release");
    }
    pool.shutdown();
}

#[test]
fn worker_pool_timer_lane_fires_deadlines_in_order() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let order = Arc::new(Mutex::new(Vec::new()));
    let start = pool.now_ms();
    let sleeps: Vec<_> = [30u64, 10, 20]
        .into_iter()
        .map(|delay| {
            let order = order.clone();
            let pool_clone = pool.clone();
            std::thread::spawn(move || {
                block_on(async move {
                    pool_clone.timer().sleep_until(start + delay).await;
                    order.lock().expect("mutex").push(delay);
                });
            })
        })
        .collect();
    for handle in sleeps {
        handle.join().expect("timer thread must not panic");
    }
    assert_eq!(*order.lock().expect("mutex"), vec![10, 20, 30], "deadlines must fire in deadline order regardless of registration order");
    pool.shutdown();
}

#[test]
fn worker_pool_submit_at_waits_without_occupying_the_only_worker() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let (immediate_tx, immediate_rx) = mpsc::channel();
    let (delayed_tx, delayed_rx) = mpsc::channel();
    pool.submit_at(pool.now_ms() + 100, Lane::Maintenance, Box::new(move || delayed_tx.send(()).expect("delayed signal")));
    pool.submit(Lane::Interactive, Box::new(move || immediate_tx.send(()).expect("immediate signal")));
    immediate_rx.recv_timeout(std::time::Duration::from_secs(1)).expect("the only worker must remain available while the delayed job waits");
    assert!(delayed_rx.recv_timeout(std::time::Duration::from_millis(20)).is_err(), "the delayed job must not run before its deadline");
    delayed_rx.recv_timeout(std::time::Duration::from_secs(1)).expect("the delayed job must run after its deadline");
    pool.shutdown();
}

#[test]
fn worker_pool_shutdown_wakes_an_in_flight_timer_waiter_before_joining() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let (waiting_tx, waiting_rx) = mpsc::channel();
    let pool_for_waiter = pool.clone();
    pool.submit(
        Lane::Timer,
        Box::new(move || {
            waiting_tx.send(()).expect("waiter start signal must succeed");
            block_on(pool_for_waiter.timer().sleep_until(u64::MAX - 1));
        }),
    );
    waiting_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("timer waiter must start");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while pool.timer().next_deadline_ms().is_none() && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
    assert_eq!(pool.timer().next_deadline_ms(), Some(u64::MAX - 1));
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let pool_for_shutdown = pool;
    let shutdown = std::thread::spawn(move || {
        pool_for_shutdown.shutdown();
        shutdown_tx.send(()).expect("shutdown signal must succeed");
    });
    shutdown_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("shutdown must wake the timer waiter before joining the sole worker");
    shutdown.join().expect("shutdown thread must not panic");
}

#[test]
fn worker_pool_within_lane_within_worker_ordering_is_fifo() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let order = Arc::new(Mutex::new(Vec::new()));
    for i in 0..20u32 {
        let order = order.clone();
        pool.submit(Lane::Interactive, Box::new(move || order.lock().expect("mutex").push(i)));
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while order.lock().expect("mutex").len() < 20 && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let seen = order.lock().expect("mutex").clone();
    let expected: Vec<u32> = (0..20).collect();
    assert_eq!(seen, expected, "a single worker's own lane must run jobs in submission order — this is the determinism the API promises");
    pool.shutdown();
}

#[test]
fn worker_pool_active_workers_and_occupancy_reflect_running_jobs() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
    assert_eq!(pool.active_workers(), 0);
    assert_eq!(pool.occupancy(), 0);
    let (start_tx, start_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel::<()>();
    let release_rx = Arc::new(Mutex::new(release_rx));
    pool.submit(
        Lane::Background,
        Box::new(move || {
            start_tx.send(()).expect("send must succeed");
            let _ = release_rx.lock().expect("mutex").recv();
        }),
    );
    start_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("job must start");
    assert_eq!(pool.active_workers(), 1);
    assert_eq!(pool.occupancy(), 1);
    release_tx.send(()).expect("send must succeed");
    pool.shutdown();
    assert_eq!(pool.active_workers(), 0);
}
//#endregion 🧵️WorkerPoolTests

//#region 🔖️Typegen
#[cfg(feature = "typegen")]
#[semio_framework_async_macros::async_test]
async fn exports_typescript_bindings() {
    schema_metadata::validate().unwrap();
    let rendered = schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, include_str!("../../🤖️generated/⏳️async/🟦️.ts"));
    }
}
//#endregion 🔖️Typegen
