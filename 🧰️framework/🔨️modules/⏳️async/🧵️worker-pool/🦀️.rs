//! 🧵️ Worker-pool vocabulary: `Job`, admission errors, `WorkerPoolConfig` and the one process-wide pool accessor.

use std::collections::VecDeque;
use std::sync::OnceLock;
use super::{ProcessKind, WorkerPool};

//#region 🧵️WorkerPool
/// 🧵️ One submitted unit of pool work — a plain closure, never a `Future`. `WorkerPool` is the CPU
/// substrate every subsystem's OS-thread work collapses onto (Phase 0 census: shard executors, shard
/// outcome forwarders, DB actor threads, the epoch ticker, HTTP fetch threads); polling `Future`s to
/// completion remains the future-polling executor's job (packet R2/P1b), built ON TOP of this pool.
pub type Job = Box<dyn FnOnce() + Send + 'static>;


/// 🚦️ Exact finite admission bound for every worker/lane queue. Queue storage is reserved once
/// when the process pool is constructed; a submitted step never grows or relocates the deque.
pub const WORKER_JOBS_PER_LANE: usize = 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerSubmitErrorKind {
    Shutdown,
    Contended,
    Poisoned,
    Saturated,
}

pub struct WorkerSubmitError {
    pub(super) kind: WorkerSubmitErrorKind,
    pub(super) job: Job,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PoolLifecycleState {
    Open { retained_uses: usize },
    Closing,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerPoolUseError {
    Closing,
    Stopped,
    Exhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerPoolShutdownError {
    Busy { retained_uses: usize },
    Closing,
}

impl WorkerSubmitError {
    pub fn kind(&self) -> WorkerSubmitErrorKind {
        self.kind
    }

    pub fn into_job(self) -> Job {
        self.job
    }
}

pub(super) fn admitted_job_queue() -> VecDeque<Job> {
    let mut queue = VecDeque::new();
    queue.try_reserve_exact(WORKER_JOBS_PER_LANE).expect("WorkerPool: failed to pre-admit its fixed per-lane queue");
    queue
}

/// 🧵️ Construction parameters for [`WorkerPool::new`]. `process_kind` is never inferred — see
/// [`ProcessKind`]'s doc — and `interactive_reserve` defaults to "on" for
/// [`ProcessKind::InteractiveNative`] (admission control active, see [`WorkerPool`]'s doc) and "off"
/// for [`ProcessKind::HeadlessBatch`] (a headless process has no UI thread to protect, so reserving
/// a slot for it would only cost throughput for nothing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerPoolConfig {
    pub process_kind: ProcessKind,
    pub cores: usize,
    pub interactive_reserve: bool,
}

impl WorkerPoolConfig {
    pub fn new(process_kind: ProcessKind, cores: usize) -> WorkerPoolConfig {
        WorkerPoolConfig { process_kind, cores, interactive_reserve: matches!(process_kind, ProcessKind::InteractiveNative) }
    }
}

//#region 🌐️ProcessWorkerPool
static PROCESS_WORKER_POOL_CONFIG: OnceLock<WorkerPoolConfig> = OnceLock::new();
static PROCESS_WORKER_POOL: OnceLock<WorkerPool> = OnceLock::new();

/// 🌐️ Returns the one worker pool owned by this process. The first process entry point to call this
/// function fixes the process kind and sizing; every later subsystem must request that same process
/// contract and receives a clone of the same underlying pool instead of constructing a private
/// scheduler. A mismatched later request fails in release too so a headless subsystem cannot
/// silently steal an interactive process's reserved worker.
pub fn process_worker_pool(config: WorkerPoolConfig) -> WorkerPool {
    let established = *PROCESS_WORKER_POOL_CONFIG.get_or_init(|| config);
    assert_eq!(config, established, "process worker pool configuration mismatch: process entry point established {established:?}, later subsystem requested {config:?}");
    PROCESS_WORKER_POOL.get_or_init(|| WorkerPool::new(established)).clone()
}
//#endregion 🌐️ProcessWorkerPool
//#endregion 🧵️WorkerPool
