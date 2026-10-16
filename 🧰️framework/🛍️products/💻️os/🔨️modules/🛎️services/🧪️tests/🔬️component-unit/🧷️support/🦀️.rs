//! 🧷️ Shared test fixtures: the operation context, a small deterministic worker pool and a runtime-clock sleep.

use semio_framework_async::{CancelToken, HostAsyncRuntime, OperationContext, ProcessKind, TraceId, WorkerPool, WorkerPoolConfig};

pub(super) async fn test_ctx(actor: u64, cancel: CancelToken) -> OperationContext {
    OperationContext { actor, generation: 0, trace: TraceId(actor), lane: 0, deadline_ms: None, cancel, capability: None }
}

/// 🧵️ A small, deterministically-sized [`WorkerPool`] for a test that wants its OWN pool rather
/// than [`global_worker_pool`]'s process-wide singleton (which every test-binary-wide
/// [`ComputePool`]/[`HttpPool`]/[`StorageScheduler`]/[`TimerWheel`] construction resolves,
/// because their constructors are frozen — see that fn's doc). `HeadlessBatch` so `workers` is
/// exactly what it says (no `-1` interactive-core reservation eating into a small test size).
pub(super) fn test_pool(workers: usize) -> WorkerPool {
    WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, workers))
}

/// 🐢️ Sleeps `ms` against `runtime`'s own clock (`runtime.now_ms()` + `runtime.sleep_until`) —
/// this crate no longer builds a `tokio::runtime::Runtime`, so `tokio::time::sleep` is gone; every
/// test that used to reach for it now reaches for this instead.
pub(super) async fn sleep_ms<R: HostAsyncRuntime>(runtime: &R, ms: u64) {
    let now = runtime.now_ms().await;
    runtime.sleep_until(now + ms).await;
}
