
use super::*;
use semio_framework_async::TraceId;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

async fn test_ctx(actor: u64, cancel: CancelToken) -> OperationContext {
    OperationContext { actor, generation: 0, trace: TraceId(actor), lane: 0, deadline_ms: None, cancel, capability: None }
}

/// 🧵️ A small, deterministically-sized [`WorkerPool`] for a test that wants its OWN pool rather
/// than [`global_worker_pool`]'s process-wide singleton (which every test-binary-wide
/// [`ComputePool`]/[`HttpPool`]/[`StorageScheduler`]/[`TimerWheel`] construction resolves,
/// because their constructors are frozen — see that fn's doc). `HeadlessBatch` so `workers` is
/// exactly what it says (no `-1` interactive-core reservation eating into a small test size).
fn test_pool(workers: usize) -> WorkerPool {
    WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, workers))
}

/// 🐢️ Sleeps `ms` against `runtime`'s own clock (`runtime.now_ms()` + `runtime.sleep_until`) —
/// this crate no longer builds a `tokio::runtime::Runtime`, so `tokio::time::sleep` is gone; every
/// test that used to reach for it now reaches for this instead.
async fn sleep_ms<R: HostAsyncRuntime>(runtime: &R, ms: u64) {
    let now = runtime.now_ms().await;
    runtime.sleep_until(now + ms).await;
}

//#region 🚂️TokioHostRuntimeTests
/// 🚂️ `TokioHostRuntime::with_pool` must not resize or replace the [`WorkerPool`] it is handed —
/// the whole point of Phase 1 packet P1b is that this type owns no thread pool of its own any
/// more.
#[test]
fn tokio_host_runtime_with_pool_never_resizes_the_injected_pool() {
    let pool = test_pool(3);
    let _runtime = TokioHostRuntime::with_pool(pool.clone());
    assert_eq!(pool.worker_count(), 3, "TokioHostRuntime must not resize the pool it was handed");
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn tokio_host_runtime_now_ms_advances_monotonically() {
    let runtime = TokioHostRuntime::with_pool(test_pool(2));
    let first = runtime.now_ms().await;
    sleep_ms(&runtime, 5).await;
    assert!(runtime.now_ms().await >= first, "now_ms must never go backward");
}
//#endregion 🚂️TokioHostRuntimeTests

//#region 🌳️ScopeTableTests
#[semio_framework_async_macros::async_test]
async fn cancel_scope_cancels_child_scopes_transitively() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let package = runtime.open_scope(ScopeOwner::Package("pkg-a".to_string()), None).await;
    let actor = runtime.open_scope(ScopeOwner::Actor(1), Some(&package)).await;
    let _ = runtime.cancel_scope(&package.owner, 50).await;
    assert!(actor.cancel.is_cancelled().await, "child scope must observe the package scope's cancellation");
}

/// 🚨️ The spawned task sleeps far past the grace period and never checks its cancel token, so
/// `cancel_scope` must report it `leaked` rather than `finished`. The 20ms sleep before
/// cancelling gives the task a chance to actually start (pass the initial live/cancelled gate)
/// first.
#[semio_framework_async_macros::async_test]
async fn cancel_scope_reports_leaked_task_that_ignores_cancellation_not_finished() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let scope = runtime.open_scope(ScopeOwner::Actor(2), None).await;
    let ctx = test_ctx(2, scope.cancel.clone()).await;
    runtime
        .spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                loop {
                    std::thread::sleep(Duration::from_secs(3600));
                }
            }),
        )
        .await;
    sleep_ms(&runtime, 20).await;
    let report = runtime.cancel_scope(&scope.owner, 60).await;
    assert_eq!(report.leaked, 1, "a task that ignores cancellation must be reported leaked, never finished");
    assert_eq!(report.finished, 0);
}

#[semio_framework_async_macros::async_test]
async fn cancel_scope_counts_a_cooperative_task_as_finished() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let scope = runtime.open_scope(ScopeOwner::Actor(3), None).await;
    let ctx = test_ctx(3, scope.cancel.clone()).await;
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) })).await;
    sleep_ms(&runtime, 20).await;
    let report = runtime.cancel_scope(&scope.owner, 60).await;
    assert!(ran.load(Ordering::SeqCst));
    assert_eq!(report.finished, 1);
    assert_eq!(report.leaked, 0);
}

#[semio_framework_async_macros::async_test]
async fn park_holds_new_work_until_unparked() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let scope = runtime.open_scope(ScopeOwner::Service("park-test"), None).await;
    scope.cancel.park().await;
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) })).await;
    sleep_ms(&runtime, 2 * PARK_POLL_INTERVAL_MS).await;
    assert!(!ran.load(Ordering::SeqCst), "parked scope must hold new work rather than running it");
    scope.cancel.unpark().await;
    sleep_ms(&runtime, 4 * PARK_POLL_INTERVAL_MS).await;
    assert!(ran.load(Ordering::SeqCst), "unparked scope must eventually run the held work");
}

#[semio_framework_async_macros::async_test]
async fn pending_scoped_future_releases_the_only_worker_between_polls() {
    let pool = test_pool(1);
    let runtime = TokioHostRuntime::with_pool(pool.clone());
    let scope = runtime.open_scope(ScopeOwner::Service("finite-future-turn"), None).await;
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    let wait_pool = pool.clone();
    let completed = Arc::new(AtomicBool::new(false));
    let completed_after_wait = Arc::clone(&completed);
    let deadline = pool.now_ms() + 80;
    runtime
        .spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                wait_pool.timer().sleep_until(deadline).await;
                completed_after_wait.store(true, Ordering::SeqCst);
            }),
        )
        .await;
    sleep_ms(&runtime, 5).await;
    let (signal_tx, signal_rx) = std::sync::mpsc::channel();
    pool.submit(Lane::Interactive, Box::new(move || signal_tx.send(()).expect("signal receiver alive")));
    signal_rx.recv_timeout(Duration::from_millis(40)).expect("pending future must not pin the only worker");
    assert!(!completed.load(Ordering::SeqCst), "delayed future completed before its deadline");
    sleep_ms(&runtime, 100).await;
    assert!(completed.load(Ordering::SeqCst), "timer wake must schedule the future's next finite turn");
    pool.shutdown();
}
//#endregion 🌳️ScopeTableTests

//#region 🧮️ComputePoolTests
struct CountingComputeJob {
    current: Option<Arc<AtomicU32>>,
    observed_max: Option<Arc<AtomicU32>>,
    remaining_steps: u8,
    entered: bool,
    closing: bool,
}

impl InteractiveJob for CountingComputeJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            if self.entered {
                self.current.as_ref().expect("compute current counter").fetch_sub(1, Ordering::SeqCst);
                self.entered = false;
            }
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        if !self.entered {
            let now = self.current.as_ref().expect("compute current counter").fetch_add(1, Ordering::SeqCst) + 1;
            self.observed_max.as_ref().expect("compute observed counter").fetch_max(now, Ordering::SeqCst);
            self.entered = true;
        }
        cx.consume_fuel(1);
        if self.remaining_steps > 0 {
            self.remaining_steps -= 1;
            return StepOutcome::Yield;
        }
        self.current.as_ref().expect("compute current counter").fetch_sub(1, Ordering::SeqCst);
        self.entered = false;
        StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.entered {
            self.current.as_ref().expect("compute current counter").fetch_sub(1, Ordering::SeqCst);
            self.entered = false;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.current.take().is_some() || self.observed_max.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && !self.entered && self.current.is_none() && self.observed_max.is_none()
    }
}

struct NeverCompleteComputeJob {
    closing: bool,
}

impl InteractiveJob for NeverCompleteComputeJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        cx.consume_fuel(1);
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[semio_framework_async_macros::async_test]
async fn interactive_jobs_never_exceed_the_compute_bound_under_a_burst() {
    const COMPUTE_CAPACITY: u32 = 3;
    let runtime = TokioHostRuntime::with_pool(test_pool(2));
    let pool = ComputePool::new(COMPUTE_CAPACITY).await;
    let scope = runtime.open_scope(ScopeOwner::Service("compute-burst"), None).await;
    let current = Arc::new(AtomicU32::new(0));
    let observed_max = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for i in 0..12u32 {
        let pool = &pool;
        let runtime = &runtime;
        let scope = &scope;
        let current = current.clone();
        let observed_max = observed_max.clone();
        let ctx = test_ctx(i as u64, scope.cancel.clone()).await;
        handles.push(async move {
            let job = CountingComputeJob { current: Some(current), observed_max: Some(observed_max), remaining_steps: 8, entered: false, closing: false };
            pool.run_job(runtime, scope, ctx, job).await.expect("interactive job without a deadline must not fail");
        });
    }
    futures_join_all(handles).await;
    assert!(observed_max.load(Ordering::SeqCst) <= COMPUTE_CAPACITY, "observed concurrency {} exceeded the compute bound {}", observed_max.load(Ordering::SeqCst), COMPUTE_CAPACITY);
    assert!(observed_max.load(Ordering::SeqCst) >= 2, "burst should have produced measurable overlap; observed {}", observed_max.load(Ordering::SeqCst));
}

#[semio_framework_async_macros::async_test]
async fn interactive_job_deadline_cancels_the_resumable_job() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let pool = ComputePool::new(4).await;
    let scope = runtime.open_scope(ScopeOwner::Service("compute-deadline"), None).await;
    let now = runtime.now_ms().await;
    let mut ctx = test_ctx(0, scope.cancel.clone()).await;
    ctx.deadline_ms = Some(now + 40);
    let cancel = ctx.cancel.clone();
    let outcome = pool.run_job(&runtime, &scope, ctx, NeverCompleteComputeJob { closing: false }).await;
    assert_eq!(outcome, Err(ComputeError::DeadlineExceeded), "a non-terminal job must stop at its absolute deadline");
    assert!(cancel.is_cancelled().await, "deadline propagation must cancel the running job");
}

#[semio_framework_async_macros::async_test]
async fn stopped_compute_pool_returns_worker_lost_and_releases_the_job_owner() {
    let runtime = TokioHostRuntime::with_pool(test_pool(1));
    let workers = test_pool(1);
    workers.shutdown();
    let pool = ComputePool::with_pool(1, workers);
    let scope = runtime.open_scope(ScopeOwner::Service("compute-stopped"), None).await;
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    let outcome = pool.run_job(&runtime, &scope, ctx, NeverCompleteComputeJob { closing: false }).await;
    assert_eq!(outcome, Err(ComputeError::WorkerLost));
}

/// 🌀️ A self-contained cooperative yield with no tokio `rt`-feature dependency (this crate no
/// longer enables `rt`/`rt-multi-thread`, so `tokio::task::yield_now` is unavailable) — wakes
/// itself immediately, giving whatever executor is driving this future one chance to poll a
/// sibling before resuming.
struct Yield(bool);
impl Future for Yield {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 🧵️ Minimal `join_all` so this crate's tests do not pull in the `futures` crate for one call
/// site — polls every future in a simple round-robin until all are ready.
async fn futures_join_all<F: Future<Output = ()>>(futures: Vec<F>) {
    let mut pending: Vec<Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
    while !pending.is_empty() {
        let mut still_pending = Vec::new();
        for mut fut in pending {
            if Future::poll(fut.as_mut(), &mut Context::from_waker(Waker::noop())) == Poll::Pending {
                still_pending.push(fut);
            }
        }
        pending = still_pending;
        Yield(false).await;
    }
}
//#endregion 🧮️ComputePoolTests

//#region ⏲️WheelCoreTests
#[semio_framework_async_macros::async_test]
async fn wheel_core_pop_expired_fires_in_expiry_order_not_arm_order() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let late = wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
    let early = wheel.arm(plugin, 2, 0, 0, 100, None).unwrap();
    let fired: Vec<TimerId> = wheel.pop_expired(1_000).into_iter().map(|f| f.id).collect();
    assert_eq!(fired, vec![early, late], "expiry order must win over arm order");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_pop_expired_respects_now_ms_boundary() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    wheel.arm(plugin, 1, 0, 0, 500, None).unwrap();
    assert!(wheel.pop_expired(400).is_empty(), "must not fire before its expiry");
    assert_eq!(wheel.pop_expired(500).len(), 1, "must fire once now_ms reaches the expiry");
}

/// ⏲️ Jumps far past several repeat periods: a naive `expiry += repeat` applied once would
/// still land before `now_ms` and fire again immediately on the next call; the catch-up loop
/// in `pop_expired` must land beyond `now_ms` instead.
#[semio_framework_async_macros::async_test]
async fn wheel_core_repeat_rearms_and_catches_up_without_drift_accumulation() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin, 1, 0, 0, 100, Some(100)).unwrap();
    let first = wheel.pop_expired(100);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].id, id);
    let second = wheel.pop_expired(450);
    assert_eq!(second.len(), 1, "a repeating timer must fire exactly once per pop_expired call even after a large time jump");
    assert!(wheel.next_expiry_ms().unwrap() > 450);
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_disarm_prevents_a_future_fire() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin, 1, 0, 0, 100, None).unwrap();
    assert!(wheel.disarm(id));
    assert!(wheel.pop_expired(1_000).is_empty());
    assert!(!wheel.disarm(id), "disarming twice must report false the second time");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_rejects_arm_past_the_per_plugin_quota_with_a_typed_error() {
    let mut wheel = WheelCore::new(2).await;
    let plugin = PackageId("p".to_string());
    wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
    wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
    let result = wheel.arm(plugin.clone(), 1, 0, 0, 300, None);
    assert_eq!(result, Err(TimerError::QuotaExceeded { plugin: plugin.clone(), limit: 2 }));
    assert_eq!(wheel.armed_count(&plugin), 2, "a rejected arm must leave the wheel untouched");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_disarm_frees_quota_for_a_new_arm() {
    let mut wheel = WheelCore::new(1).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
    assert!(wheel.arm(plugin.clone(), 1, 0, 0, 200, None).is_err());
    wheel.disarm(id);
    assert!(wheel.arm(plugin, 1, 0, 0, 200, None).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_expiry_batch_preserves_due_remainder() {
    let mut wheel = WheelCore::new(32).await;
    let plugin = PackageId("batch".to_string());
    for actor in 0..20 {
        wheel.arm(plugin.clone(), actor, 0, 0, 100, None).expect("timer arm");
    }
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 8);
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 8);
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 4);
    assert!(wheel.pop_expired_batch(100, 8).is_empty());
}
//#endregion ⏲️WheelCoreTests

//#region ⏲️TimerWheelDriverTests
/// ⏱️ Polls `sink.recorded()` on a short real-time tick until it is non-empty or `timeout`
/// elapses while finite timer turns use the pool's real monotonic clock.
async fn wait_for_completions(sink: &MockCompletionSink, timeout: Duration) -> Vec<CompletionRecord> {
    let start = std::time::Instant::now();
    loop {
        let recorded = sink.recorded().await;
        if !recorded.is_empty() || start.elapsed() >= timeout {
            return recorded;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[semio_framework_async_macros::async_test]
async fn timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink() {
    let pool = test_pool(2);
    let wheel = TimerWheel::new(10).await;
    let sink = Arc::new(MockCompletionSink::new().await);
    wheel.spawn_driver(&pool, sink.clone());
    let now = pool.now_ms();
    wheel.arm(PackageId("plugin-a".to_string()), 42, 3, 1, now + 30, None).await.expect("arm should succeed");
    assert!(sink.recorded().await.is_empty(), "must not fire before its deadline");
    let recorded = wait_for_completions(&sink, Duration::from_secs(5)).await;
    assert_eq!(recorded.len(), 1, "the driver must post exactly one completion for the fired timer");
    assert_eq!(recorded[0].actor, 42);
    assert_eq!(recorded[0].generation, 3);
    assert_eq!(recorded[0].lane, 1);
    pool.shutdown();
}
//#endregion ⏲️TimerWheelDriverTests

//#region 📄️FixedFilePageTests
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page() {
    let directory = std::env::temp_dir().join(format!("semio-fixed-file-page-{}-{:?}", std::process::id(), std::thread::current().id()));
    let path = directory.join("page.bin");
    let exact = vec![0x5au8; STORAGE_FIXED_FILE_PAGE_BYTES];
    storage_worker_write_fixed_file_page(&path, &exact, STORAGE_FIXED_FILE_PAGE_BYTES).expect("exact page must be admitted");
    let retained = storage_worker_read_fixed_file_page(&path, STORAGE_FIXED_FILE_PAGE_BYTES).expect("exact page must be readable");
    let oracle = std::fs::read(&path).expect("system oracle must read the same published page");
    assert_eq!(retained, oracle);
    assert_eq!(retained, exact);

    let plus_one = vec![0xa5u8; STORAGE_FIXED_FILE_PAGE_BYTES + 1];
    assert_eq!(storage_worker_write_fixed_file_page(&path, &plus_one, STORAGE_FIXED_FILE_PAGE_BYTES).expect_err("max plus one must fail before truncation").kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(std::fs::read(&path).expect("rejected write must preserve the last valid page"), exact);

    std::fs::write(&path, &plus_one).expect("system oracle must publish hostile oversized input");
    assert_eq!(storage_worker_read_fixed_file_page(&path, STORAGE_FIXED_FILE_PAGE_BYTES).expect_err("oversized hostile page must fail closed").kind(), std::io::ErrorKind::InvalidData);
    let _ = std::fs::remove_dir_all(&directory);
}
//#endregion 📄️FixedFilePageTests

//#region 💾️StorageSchedulerTests
/// 💾️ `ManualRuntime`'s dispatch would run synchronously, in-line, so nothing can ever be
/// genuinely QUEUED behind it — a real `TokioHostRuntime` is required here to make the priority
/// ordering observable at all: one job occupies the single in-flight slot on its own worker
/// while the two real submissions below queue behind it.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_dispatches_highest_priority_lane_first_despite_submit_order() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-priority-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 1, 1_000_000).await;
    let order = Arc::new(Mutex::new(Vec::<&'static str>::new()));

    let (occupy_tx, occupy_rx) = std::sync::mpsc::channel::<()>();
    let occupy_rx = Mutex::new(occupy_rx);
    let occupy_ctx = test_ctx(0, scope.cancel.clone()).await;
    let occupy_ticket = scheduler
        .submit(&occupy_ctx, PackageId("occupy".to_string()), 1, move || {
            occupy_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();

    let mut low_ctx = test_ctx(0, scope.cancel.clone()).await;
    low_ctx.lane = 200;
    let order_low = order.clone();
    let low_ticket = scheduler
        .submit(&low_ctx, PackageId("p".to_string()), 1, move || {
            order_low.lock().unwrap().push("low");
            Ok(Vec::new())
        })
        .unwrap();

    let mut high_ctx = test_ctx(0, scope.cancel).await;
    high_ctx.lane = 1;
    let order_high = order.clone();
    let high_ticket = scheduler
        .submit(&high_ctx, PackageId("p".to_string()), 1, move || {
            order_high.lock().unwrap().push("high");
            Ok(Vec::new())
        })
        .unwrap();

    runtime.block_on(async {
        occupy_tx.send(()).expect("occupying job must still be waiting to receive");
        let _ = occupy_ticket.await_result().await;
        let _ = low_ticket.await_result().await;
        let _ = high_ticket.await_result().await;
    });
    assert_eq!(*order.lock().unwrap(), vec!["high", "low"], "the higher-priority lane (lower ctx.lane) must dispatch before the lower-priority one despite submitting second");
}

#[semio_framework_async_macros::async_test]
async fn storage_scheduler_never_exceeds_max_in_flight() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-cap-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 2, 1_000_000).await;
    let mut tickets = Vec::new();
    for i in 0..8u32 {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let ticket = scheduler
            .submit(&ctx, PackageId(format!("p{i}")), 1, move || {
                std::thread::sleep(Duration::from_millis(15));
                Ok(Vec::new())
            })
            .unwrap();
        tickets.push(ticket);
    }
    let cap_violated = Arc::new(AtomicBool::new(false));
    runtime.block_on(async {
        let scheduler_ref = &scheduler;
        let cap_violated_ref = &cap_violated;
        let sampler = async {
            loop {
                if scheduler_ref.in_flight().await > 2 {
                    cap_violated_ref.store(true, Ordering::SeqCst);
                }
                Yield(false).await;
            }
        };
        let drain = async {
            for ticket in tickets {
                let _ = ticket.await_result().await;
            }
        };
        select2(sampler, drain).await;
    });
    assert!(!cap_violated.load(Ordering::SeqCst), "in-flight count must never exceed max_in_flight");
}

/// 💾️ A real `TokioHostRuntime` is required here too (same reasoning as the priority test
/// above): the first job must still be genuinely IN FLIGHT — holding its 60-byte reservation —
/// when the second `submit` is checked, so it is blocked on a channel rather than left to
/// `ManualRuntime`'s synchronous, immediately-releasing execution.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_rejects_over_budget_submit_with_a_typed_error_and_untouched_usage() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-budget-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 4, 100).await;
    let ctx = test_ctx(0, scope.cancel).await;
    let plugin = PackageId("p".to_string());
    let (hold_tx, hold_rx) = std::sync::mpsc::channel::<()>();
    let hold_rx = Mutex::new(hold_rx);
    let first_ticket = scheduler
        .submit(&ctx, plugin.clone(), 60, move || {
            hold_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();
    let result = scheduler.submit(&ctx, plugin.clone(), 60, || Ok(Vec::new()));
    let error = result.err().expect("submitting past the byte quota must be rejected while the first job still holds its reservation");
    assert_eq!(error, StorageError::BytesQuotaExceeded { plugin, limit: 100 });
    let _ = hold_tx.send(());
    runtime.block_on(async {
        let _ = first_ticket.await_result().await;
    });
}

/// ⏰️ Occupies the scheduler's ONE in-flight slot with a job blocked on a channel, then submits
/// a second job with a short `ctx.deadline_ms` — since the slot never frees during that window,
/// the second job stays queued, so `await_result` must lose the race against its own deadline
/// (never actually run `ran`), and once the occupier finally completes, `storage_try_dispatch`
/// must skip the now-cancelled job and release ITS byte reservation too (proved by a follow-up
/// submit that would otherwise not fit the tight per-plugin quota) — the 50ms sleep after the
/// occupier's own result gives that skip a chance to happen, since it runs on the completion
/// closure's own thread, which may still be unwinding when this task's waker fires.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_races_a_queued_job_against_its_deadline_and_frees_its_reservation_when_lost() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-deadline-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 1, 50).await;

    let (occupy_tx, occupy_rx) = std::sync::mpsc::channel::<()>();
    let occupy_rx = Mutex::new(occupy_rx);
    let occupy_ctx = test_ctx(0, scope.cancel.clone()).await;
    let occupy_ticket = scheduler
        .submit(&occupy_ctx, PackageId("occupy".to_string()), 1, move || {
            occupy_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();

    let plugin = PackageId("p".to_string());
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    let outcome = runtime.block_on(async {
        let now = runtime.now_ms().await;
        let mut deadline_ctx = test_ctx(0, scope.cancel.clone()).await;
        deadline_ctx.deadline_ms = Some(now + 30);
        let deadline_ticket = scheduler.submit(&deadline_ctx, plugin.clone(), 42, move || {
            ran_clone.store(true, Ordering::SeqCst);
            Ok(Vec::new())
        });
        deadline_ticket.expect("submit itself must succeed; only the eventual run races the deadline").await_result().await
    });
    assert_eq!(outcome, Err(StorageError::DeadlineExceeded), "a job stuck behind a full in-flight slot must lose the race against its own deadline");
    assert!(!ran.load(Ordering::SeqCst), "the queued job must never have actually run once its deadline had already fired");

    let _ = occupy_tx.send(());
    runtime.block_on(async {
        let _ = occupy_ticket.await_result().await;
        sleep_ms(runtime.as_ref(), 50).await;
    });
    let verify_ticket = scheduler.submit(&test_ctx(0, scope.cancel).await, plugin.clone(), 45, || Ok(Vec::new()));
    let verify_ticket = verify_ticket.expect("the deadline-lost job's 42-byte reservation must have been released, leaving room for 45 more under the 50-byte quota");
    runtime.block_on(async {
        let _ = verify_ticket.await_result().await;
    });
}
//#endregion 💾️StorageSchedulerTests

//#region 📮️EventRouterTests
#[semio_framework_async_macros::async_test]
async fn event_router_latest_wins_collapses_older_pending_value() {
    let router = EventRouter::new();
    let topic = Topic("scene.updates".to_string());
    let actor = ActorId(1);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 1_000_000 }).await;
    let first = router.publish(&topic, None, b"v1").await;
    let second = router.publish(&topic, None, b"v2").await;
    assert_eq!(first, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(second, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&topic, actor).await, vec![b"v2".to_vec()], "only the LATEST value must survive the collapse");
}

#[semio_framework_async_macros::async_test]
async fn event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth() {
    let router = EventRouter::new();
    let topic = Topic("jobs.updates".to_string());
    let actor = ActorId(2);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LosslessBounded { max_items: 2, max_bytes: 1_000_000 }).await;
    assert_eq!(router.publish(&topic, None, b"a").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"b").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"c").await, vec![(actor, PublishOutcome::RejectedFull { cap: 2 })], "must reject rather than grow past cap");
    assert_eq!(router.drain(&topic, actor).await, vec![b"a".to_vec(), b"b".to_vec()], "the rejected message must never have been queued");
}

#[semio_framework_async_macros::async_test]
async fn event_router_coalesced_collapses_same_key_but_queues_distinct_keys() {
    let router = EventRouter::new();
    let topic = Topic("cursor.updates".to_string());
    let actor = ActorId(3);
    router.subscribe(topic.clone(), actor, ChannelPolicy::Coalesced { key: "cursor".to_string(), max_items: 100, max_bytes: 1_000_000 }).await;
    router.publish(&topic, Some("peer-1"), b"pos-1").await;
    let outcome = router.publish(&topic, Some("peer-1"), b"pos-2").await;
    router.publish(&topic, Some("peer-2"), b"pos-a").await;
    assert_eq!(outcome, vec![(actor, PublishOutcome::Collapsed)]);
    let drained = router.drain(&topic, actor).await;
    assert_eq!(drained.len(), 2, "distinct coalesce keys must not collapse into each other");
    assert!(drained.contains(&b"pos-2".to_vec()));
    assert!(drained.contains(&b"pos-a".to_vec()));
}

#[semio_framework_async_macros::async_test]
async fn event_router_ring_overwrites_oldest_by_item_and_byte_bounds() {
    let router = EventRouter::new();
    let topic = Topic("diagnostics".to_string());
    let actor = ActorId(6);
    router.subscribe(topic.clone(), actor, ChannelPolicy::Ring { max_items: 3, max_bytes: 4 }).await;
    assert_eq!(router.publish(&topic, None, b"aa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"bb").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"cc").await, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&topic, actor).await, vec![b"bb".to_vec(), b"cc".to_vec()]);
}

#[semio_framework_async_macros::async_test]
async fn event_router_payload_bytes_are_enforced_for_every_queueing_policy() {
    let router = EventRouter::new();
    let latest = Topic("bounded.latest".to_string());
    let coalesced = Topic("bounded.coalesced".to_string());
    let lossless = Topic("bounded.lossless".to_string());
    let actor = ActorId(7);
    router.subscribe(latest.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 2 }).await;
    router.subscribe(coalesced.clone(), actor, ChannelPolicy::Coalesced { key: "entity".to_string(), max_items: 2, max_bytes: 4 }).await;
    router.subscribe(lossless.clone(), actor, ChannelPolicy::LosslessBounded { max_items: 4, max_bytes: 3 }).await;
    assert_eq!(router.publish(&latest, None, b"xxx").await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)]);
    assert_eq!(router.publish(&coalesced, Some("a"), b"aaa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&coalesced, Some("b"), b"bb").await, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&coalesced, actor).await, vec![b"bb".to_vec()]);
    assert_eq!(router.publish(&lossless, None, b"aa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&lossless, None, b"bb").await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)]);
}

#[semio_framework_async_macros::async_test]
async fn event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket() {
    let router = EventRouter::new();
    let topic = Topic("stream.frames".to_string());
    let actor = ActorId(4);
    router.subscribe(topic.clone(), actor, ChannelPolicy::ByteCredit { max_items: 100, max_bytes: 4 }).await;
    assert_eq!(router.publish(&topic, None, &[0u8; 3]).await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, &[0u8; 3]).await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)], "must reject once the remaining credit is insufficient");
}

#[semio_framework_async_macros::async_test]
async fn event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber() {
    let router = EventRouter::new();
    let topic = Topic("scene.updates".to_string());
    let actor = ActorId(5);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 1_000_000 }).await;
    router.unsubscribe(&topic, actor).await;
    assert_eq!(router.publish(&topic, None, b"x").await, Vec::new(), "no subscribers left means no outcomes at all");
    assert_eq!(router.send_message(&topic, actor, b"x".to_vec()).await, PublishOutcome::NoSuchSubscriber);
}
//#endregion 📮️EventRouterTests

//#region 🌐️HttpPoolTests
struct RecordingTransport {
    calls: Arc<Mutex<u32>>,
}
impl HttpTransport for RecordingTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        *self.calls.lock().unwrap() += 1;
        Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
    }
}

async fn sample_request() -> HttpRequest {
    HttpRequest { method: "GET".to_string(), url: "https://example.invalid/x".to_string(), headers: Vec::new(), body: Vec::new() }
}

struct BlockingTransport(Mutex<std::sync::mpsc::Receiver<()>>);
impl HttpTransport for BlockingTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        self.0.lock().unwrap().recv().ok();
        Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
    }
}

/// 🌐️ Runs a first request on a background OS thread (this crate no longer builds a tokio
/// `Runtime`, so `tokio::spawn` is gone — a plain `std::thread::spawn` driving its own
/// `runtime.block_on` call is the direct replacement), blocked on `unblock_rx` so it stays
/// genuinely outstanding, then waits 40ms (well past the background thread's own startup) before
/// issuing a second request that must be rejected while the first still holds the actor's one
/// slot.
#[semio_framework_async_macros::async_test]
async fn http_pool_rejects_past_the_per_actor_outstanding_cap() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-test"), None).await;
    let (unblock_tx, unblock_rx) = std::sync::mpsc::channel::<()>();
    let compute = Arc::new(ComputePool::new(4).await);
    let pool = Arc::new(HttpPool::new(Arc::new(BlockingTransport(Mutex::new(unblock_rx))), compute, 1_000_000, 1).await);
    let actor = ActorId(9);

    let pool_bg = pool.clone();
    let runtime_bg = runtime.clone();
    let scope_bg = scope.clone();
    let ctx_bg = test_ctx(0, scope.cancel.clone()).await;
    let request_bg = sample_request().await;
    let handle = std::thread::spawn(move || {
        let fut = pool_bg.request(runtime_bg.as_ref(), &scope_bg, ctx_bg, PackageId("pkg".to_string()), actor, request_bg);
        runtime_bg.block_on(fut)
    });
    sleep_ms(runtime.as_ref(), 40).await;
    let ctx2 = test_ctx(0, scope.cancel.clone()).await;
    let second = pool.request(runtime.as_ref(), &scope, ctx2, PackageId("pkg".to_string()), actor, sample_request().await).await;
    assert_eq!(second, Err(HttpPoolError::OutstandingCapReached { actor, limit: 1 }));
    let _ = unblock_tx.send(());
    let first_result = handle.join().expect("background request thread must not panic");
    assert!(first_result.is_ok(), "the first request must still complete once unblocked");
}

#[semio_framework_async_macros::async_test]
async fn http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let scope = runtime.open_scope(ScopeOwner::Service("http-budget-test"), None).await;
    let calls = Arc::new(Mutex::new(0));
    let compute = Arc::new(ComputePool::new(4).await);
    let pool = HttpPool::new(Arc::new(RecordingTransport { calls: calls.clone() }), compute, 4, 4).await;
    let actor = ActorId(10);
    let package = PackageId("pkg".to_string());
    let mut big_request = sample_request().await;
    big_request.body = vec![0u8; 10];
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    let result = runtime.block_on(pool.request(&runtime, &scope, ctx, package.clone(), actor, big_request));
    assert_eq!(result, Err(HttpPoolError::ByteBudgetExhausted { package }));
    assert_eq!(*calls.lock().unwrap(), 0, "the transport must never be called once the budget rejects the request");
}

/// ♻️ Directly seeds a package's bucket down to a known remainder (module-private field access
/// from this `tests` submodule — no new production API needed for it), then proves
/// `spawn_refill_driver`'s job actually RUNS on a tick: no refill before the interval elapses, a
/// full top-up once it does. Injects a SHORT `interval_ms` (real wall-clock milliseconds, not
/// [`HTTP_BUCKET_REFILL_INTERVAL_MS`]'s real 60 seconds — `spawn_refill_driver` now drives real
/// [`WorkerPool`] worker threads on the real clock, no injected `ManualRuntime` clock any more).
#[semio_framework_async_macros::async_test]
async fn http_pool_refill_driver_actually_refills_a_consumed_bucket_on_its_tick() {
    const REFILL_INTERVAL_MS: u64 = 40;
    let pool_workers = test_pool(2);
    let compute = Arc::new(ComputePool::new(2).await);
    let pool = HttpPool::new(Arc::new(UnwiredHttpTransport), compute, 100, 4).await;
    let package = PackageId("pkg-refill".to_string());
    {
        let mut buckets = pool.buckets.lock().unwrap();
        buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(100, 0)).try_consume(70);
    }
    assert_eq!(pool.remaining_package_budget(&package).await, 30);

    pool.spawn_refill_driver(&pool_workers, REFILL_INTERVAL_MS);
    assert_eq!(pool.remaining_package_budget(&package).await, 30, "must not refill before the tick interval elapses");
    let start = std::time::Instant::now();
    loop {
        if pool.remaining_package_budget(&package).await == 100 {
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(5), "the refill driver must actually run its loop and top the bucket back up on its own tick");
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[semio_framework_async_macros::async_test]
async fn finite_timer_and_refill_drivers_do_not_starve_a_single_worker() {
    let workers = test_pool(1);
    let wheel = TimerWheel::new(10).await;
    wheel.spawn_driver(&workers, Arc::new(MockCompletionSink::new().await));
    let compute = Arc::new(ComputePool::with_pool(1, workers.clone()));
    let http = HttpPool::new(Arc::new(UnwiredHttpTransport), compute, 100, 1).await;
    http.spawn_refill_driver(&workers, 10);
    let (tx, rx) = std::sync::mpsc::channel();
    workers.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("interactive signal")));
    rx.recv_timeout(Duration::from_secs(1)).expect("finite service drivers must leave the only worker available");
    workers.shutdown();
}

/// 🌐️ A test-only `AsyncHttpTransport`/`HttpBody` over a REAL local TCP socket — the harness the
/// packet report's `## honest gaps` asks for if a raw listener inside a unit test is awkward.
/// Every `next_chunk` call does one real blocking `read` through `ComputePool`, so bytes charged
/// against the package bucket are genuinely read off the wire, not buffered/estimated upfront.
struct LocalSocketBody {
    stream: Arc<Mutex<std::net::TcpStream>>,
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
    ctx: OperationContext,
    /// 🔍️ Set when this value drops, so a test can OBSERVE that `HttpPoolBody`'s own `Drop`
    /// really did drop the transport body (and therefore the socket) rather than merely
    /// stopping the caller from polling it further.
    dropped: Arc<AtomicBool>,
}
impl HttpBody for LocalSocketBody {
    // 🚫️async: E6 dyn-compat — see the `HttpBody` trait's tag.
    fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let stream = self.stream.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = self.ctx.clone();
        Box::pin(async move {
            let outcome = compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    use std::io::Read;
                    let mut buf = [0u8; 64];
                    let mut guard = stream.lock().expect("test socket mutex poisoned");
                    match guard.read(&mut buf) {
                        Ok(0) => None,
                        Ok(n) => Some(buf[..n].to_vec()),
                        Err(_) => None,
                    }
                })
                .await;
            outcome.map_err(HttpPoolError::Compute)
        })
    }
}
impl Drop for LocalSocketBody {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
    }
}

struct LocalSocketTransport {
    addr: std::net::SocketAddr,
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
    dropped: Arc<AtomicBool>,
}
impl AsyncHttpTransport for LocalSocketTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn start(&self, ctx: &OperationContext, _request: HttpRequest) -> HostFuture<StartedTransport> {
        let addr = self.addr;
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx_for_connect = ctx.clone();
        let ctx_for_body = ctx.clone();
        let dropped = self.dropped.clone();
        Box::pin(async move {
            let connect_result = compute.run_io(runtime.as_ref(), &scope, ctx_for_connect, move || std::net::TcpStream::connect(addr)).await;
            let stream = match connect_result {
                Ok(Ok(stream)) => stream,
                Ok(Err(io_error)) => return Err(HttpPoolError::Transport(io_error.to_string())),
                Err(compute_error) => return Err(HttpPoolError::Compute(compute_error)),
            };
            let head = HttpResponseHead { status: 200, headers: Vec::new() };
            let body: Box<dyn HttpBody> = Box::new(LocalSocketBody { stream: Arc::new(Mutex::new(stream)), compute, runtime, scope, ctx: ctx_for_body, dropped });
            Ok((head, body))
        })
    }
}

/// 🐌️ Binds an ephemeral local listener and, for every accepted connection, writes `chunks` in
/// order with a small delay between each — a genuine streamed response, not a single buffered
/// write. Accepts indefinitely (background thread lives for the test's duration) so more than
/// one test connection can be served.
async fn spawn_chunk_server(chunks: Vec<Vec<u8>>) -> std::net::SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind local test listener");
    let addr = listener.local_addr().expect("read local test listener addr");
    std::thread::spawn(move || {
        for incoming in listener.incoming() {
            let Ok(mut stream) = incoming else { continue };
            let chunks = chunks.clone();
            std::thread::spawn(move || {
                use std::io::Write;
                for chunk in chunks {
                    if stream.write_all(&chunk).is_err() {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
            });
        }
    });
    addr
}

/// 🌐️ The run-the-real-thing case: a genuine multi-chunk response over a real local TCP socket,
/// asserting the package bucket is charged EXACTLY each chunk's real length as it arrives — not
/// an upfront estimate, not the whole body's length in one shot.
#[semio_framework_async_macros::async_test]
async fn http_pool_fetch_charges_real_bytes_per_chunk_over_a_local_tcp_listener() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-stream-test"), None).await;
    let compute = Arc::new(ComputePool::new(4).await);
    let chunks = vec![vec![1u8; 10], vec![2u8; 15], vec![3u8; 7]];
    let addr = spawn_chunk_server(chunks.clone()).await;
    let dropped = Arc::new(AtomicBool::new(false));
    let transport = Arc::new(LocalSocketTransport { addr, compute, runtime: runtime.clone(), scope: scope.clone(), dropped });
    let pool = HttpPool::new_with_async_transport(transport, 1_000_000, 4).await;
    let package = PackageId("pkg-stream".to_string());
    let actor = ActorId(11);

    runtime.block_on(async {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, package.clone(), actor, sample_request().await).await.expect("fetch should succeed");
        assert_eq!(head.status, 200);
        let mut before = pool.remaining_package_budget(&package).await;
        let mut total = Vec::new();
        while let Some(chunk) = body.next_chunk().await.expect("chunk read should succeed") {
            let after = pool.remaining_package_budget(&package).await;
            assert_eq!(before - after, chunk.len() as u64, "each real chunk must charge exactly its own real byte length, not an estimate");
            before = after;
            total.extend(chunk);
        }
        let expected: Vec<u8> = chunks.into_iter().flatten().collect();
        assert_eq!(total, expected, "streamed bytes must match what the server actually sent");
    });
}

/// 🌐️ The cancellation case: drop a fetched body after reading only its FIRST chunk (a consumer
/// bailing out mid-stream). This must (a) drop the transport's own `HttpBody` — closing the
/// connection, observed via `dropped` — and (b) free the actor's outstanding slot immediately,
/// proved by a SECOND `fetch` against the same actor succeeding under `outstanding_cap: 1`
/// rather than being rejected.
#[semio_framework_async_macros::async_test]
async fn http_pool_dropping_a_body_mid_stream_frees_the_outstanding_slot_and_drops_the_connection() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-cancel-test"), None).await;
    let compute = Arc::new(ComputePool::new(4).await);
    let chunks: Vec<Vec<u8>> = (0..20).map(|_| vec![9u8; 8]).collect();
    let addr = spawn_chunk_server(chunks).await;
    let dropped = Arc::new(AtomicBool::new(false));
    let transport = Arc::new(LocalSocketTransport { addr, compute, runtime: runtime.clone(), scope: scope.clone(), dropped: dropped.clone() });
    let pool = HttpPool::new_with_async_transport(transport, 1_000_000, 1).await;
    let package = PackageId("pkg-cancel".to_string());
    let actor = ActorId(12);

    runtime.block_on(async {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (_head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, package.clone(), actor, sample_request().await).await.expect("fetch should succeed");
        let first_chunk = body.next_chunk().await.expect("first chunk should read").expect("server must have sent at least one chunk");
        assert_eq!(first_chunk.len(), 8);
        drop(body);
    });
    assert!(dropped.load(Ordering::SeqCst), "dropping HttpPoolBody mid-stream must drop the underlying transport body, closing its connection");

    runtime.block_on(async {
        let ctx2 = test_ctx(0, scope.cancel.clone()).await;
        let second = pool.fetch(runtime.as_ref(), &scope, ctx2, package.clone(), actor, sample_request().await).await;
        assert!(second.is_ok(), "the outstanding slot must have been freed by the drop, not held open until a full response finished");
    });
}

#[semio_framework_async_macros::async_test]
async fn socket_http_transport_streams_chunked_and_content_length_bodies_without_aggregation() {
    fn server(response: &'static [u8]) -> std::net::SocketAddr {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            for page in response.chunks(3) {
                std::io::Write::write_all(&mut stream, page).unwrap();
            }
        });
        addr
    }

    async fn collect(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, addr: std::net::SocketAddr) -> (HttpResponseHead, Vec<Vec<u8>>) {
        let compute = Arc::new(ComputePool::with_pool(2, test_pool(2)));
        let transport = Arc::new(SocketHttpTransport::new(compute, runtime.clone(), scope.clone()));
        let pool = HttpPool::new_with_async_transport_now(transport, 1_000_000, 1);
        let request = HttpRequest { method: "GET".into(), url: format!("http://{addr}/asset"), headers: Vec::new(), body: Vec::new() };
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, PackageId("socket-http".into()), ActorId(17), request).await.unwrap();
        let mut pages = Vec::new();
        while let Some(page) = body.next_chunk().await.unwrap() {
            assert!(!page.is_empty());
            assert!(page.len() <= SOCKET_HTTP_BODY_PAGE_BYTES);
            pages.push(page);
        }
        (head, pages)
    }

    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("socket-http"), None).await;
    let chunked = server(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n3\r\nbye\r\n0\r\n\r\n");
    let (head, pages) = collect(runtime.clone(), scope.clone(), chunked).await;
    assert_eq!(head.status, 200);
    assert_eq!(pages, vec![b"hello".to_vec(), b"bye".to_vec()]);

    let fixed = server(b"HTTP/1.1 206 Partial Content\r\nContent-Length: 8\r\n\r\n12345678");
    let (head, pages) = collect(runtime, scope, fixed).await;
    assert_eq!(head.status, 206);
    assert_eq!(pages.concat(), b"12345678");
}

#[test]
fn socket_http_schema_rejects_tls_and_overlong_lines_before_unbounded_ownership() {
    assert!(socket_http_url("https://example.com/asset").is_err());
    let overlong = format!("http://{}/asset", "x".repeat(SOCKET_HTTP_HOST_BYTES + 1));
    assert!(socket_http_url(&overlong).is_err());
    let mut line = std::io::Cursor::new(vec![b'x'; 129]);
    assert!(socket_http_read_line(&mut line, 128).is_err());
}
//#endregion 🌐️HttpPoolTests

//#region 🧾️CompletionSinkTests
#[semio_framework_async_macros::async_test]
async fn mock_completion_sink_records_calls_in_order() {
    let sink = MockCompletionSink::new().await;
    sink.complete(1, 0, vec![1], 0);
    sink.complete(2, 1, vec![2], 1);
    let recorded = sink.recorded().await;
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].actor, 1);
    assert_eq!(recorded[1].actor, 2);
}
//#endregion 🧾️CompletionSinkTests
