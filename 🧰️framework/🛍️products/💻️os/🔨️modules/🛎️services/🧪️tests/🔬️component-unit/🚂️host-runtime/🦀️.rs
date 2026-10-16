//! 🚂️ Tests of `TokioHostRuntime` over an injected worker pool and of its scope table: transitive cancellation, leaked tasks, parking and worker release between polls.

use semio_framework_async::{HostAsyncRuntime, Lane, ScopeOwner};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use super::super::host_runtime::{PARK_POLL_INTERVAL_MS, TokioHostRuntime};
use super::support::{sleep_ms, test_ctx, test_pool};

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
