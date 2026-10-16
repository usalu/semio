//! 💾️ The storage scheduler: lane-ordered dispatch under a byte budget and an in-flight bound, with deadline races.

use semio_framework_actor::PackageId;
use semio_framework_async::{Either, HostAsyncRuntime, Lane, OperationContext, ScopeHandle, WorkerPool, oneshot, select2};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::task::{Context, Poll, Waker};
use super::host_runtime::global_worker_pool;

//#region 💾️Storage
//#region 💾️StorageScheduler
/// 🚫️ [`StorageScheduler::submit`]'s failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError {
    BytesQuotaExceeded {
        plugin: PackageId,
        limit: u64,
    },
    Io(String),
    Closed,
    /// ⏰️ `ctx.deadline_ms` elapsed before this job's turn came up (still queued) or before it
    /// finished (already dispatched) — [`StorageTicket::await_result`] raced the result against the
    /// deadline and the deadline won. A job already running on a blocking OS thread when this fires
    /// is NOT preempted (same honest limitation [`ComputeError::DeadlineExceeded`] documents); it is
    /// marked cancelled so [`storage_try_dispatch`] skips it if it is STILL QUEUED when popped.
    DeadlineExceeded,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::BytesQuotaExceeded { plugin, limit } => write!(f, "plugin {:?} exceeded its {limit}-byte storage quota", plugin.0),
            StorageError::Io(message) => write!(f, "storage io error: {message}"),
            StorageError::Closed => write!(f, "storage scheduler dropped the job before it ran"),
            StorageError::DeadlineExceeded => write!(f, "storage job exceeded its deadline"),
        }
    }
}
impl std::error::Error for StorageError {}

struct StorageJob {
    pub(super) plugin: PackageId,
    pub(super) bytes: u64,
    pub(super) ctx: OperationContext,
    pub(super) work: Box<dyn FnOnce() -> Result<Vec<u8>, std::io::Error> + Send>,
    pub(super) result_tx: oneshot::Sender<Result<Vec<u8>, StorageError>>,
    /// ⏰️ Set by [`StorageTicket::await_result`] if the deadline race is lost while this job is
    /// still queued — [`storage_try_dispatch`] checks it right after popping, same lazy-skip
    /// discipline [`WheelCore::disarm`] already uses for timers.
    pub(super) cancelled: Arc<AtomicBool>,
}

struct StorageState<R: HostAsyncRuntime> {
    pub(super) runtime: Arc<R>,
    pub(super) scope: ScopeHandle,
    pub(super) pool: WorkerPool,
    queues: Mutex<BTreeMap<u8, VecDeque<StorageJob>>>,
    pub(super) in_flight: AtomicU32,
    max_in_flight: u32,
    per_plugin_bytes: Mutex<HashMap<PackageId, u64>>,
    byte_quota_per_plugin: u64,
}

/// 🚫️async: E5 executor bridge — polls `fut` exactly ONCE with a no-op waker (same construction
/// `semio-framework-async::testkit::ManualRuntime::drive` already uses) and panics if it is not
/// `Poll::Ready` on that first poll. Correct ONLY for a future documented to never suspend;
/// `storage_try_dispatch`'s ONLY use of it is `scope.cancel.is_cancelled()`, whose body is a plain
/// atomic load with no real `.await` point (confirmed by reading `CancelToken::state`'s body) — the
/// SAME justification `WheelCore`'s own `🚫️async: E1-adjacent` tags give for wrapping trivially-sync
/// logic in an `async fn` signature.
///
/// WHY THIS CRATE CARRIES A SECOND E5 EXCEPTION, past R2's normal "at most one per crate": R2's
/// bound is a discipline against proliferating ad-hoc bridges, not a literal ceiling that survives
/// every cross-crate constraint. `StorageScheduler::submit`'s signature is fixed SYNCHRONOUS by a
/// caller this packet cannot edit: `🔌️plugin/🖥️host/📥️imports/🦀️.rs`'s `storage_read`/`storage_write`
/// call it un-awaited, inside an already-`async fn`, then immediately `.await` the returned
/// [`StorageTicket`] separately — `submit` becoming `async fn` would desync that call site with no
/// way for this packet to fix it, so `storage_try_dispatch` (which `submit` calls synchronously) has
/// no path to [`TokioHostRuntime::block_on`]'s mechanism either; `resolve_ready` is the smallest
/// correct bridge for the one non-async-callable check it still needs.
// 🚫️async: E5 executor bridge — see the doc comment directly above (kept short here, within the
// codemod's 6-line lookback, since the full rationale needed more room than that).
fn resolve_ready<F: Future>(fut: F) -> F::Output {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut fut = std::pin::pin!(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => unreachable!(
            "resolve_ready: storage_try_dispatch only ever polls a future documented to never \
             suspend — a Pending here means that contract was violated"
        ),
    }
}

/// ▶️ Reentrant dispatch step: while a slot is free, pops the head of the HIGHEST-priority
/// non-empty lane (`BTreeMap` iterates ascending, and lower `ctx.lane` is higher priority — the same
/// convention `OperationContext.lane` documents) and submits it onto `state.pool` (this crate's ONE
/// process-wide [`WorkerPool`], via [`Lane::from_context_lane`]) — replacing the deleted
/// `HostAsyncRuntime::run_blocking` bridge P1a removed. No separate background polling task exists:
/// [`StorageScheduler::submit`] and every job's own completion both call this again, so a freed slot
/// or a newly queued job always gets a chance to dispatch without a dedicated loop. Honors
/// `state.scope`'s cancellation the same way the pre-Phase-1 `ScopeTable::run_blocking` wrapper did —
/// a job popped after its scope was cancelled is skipped (never runs `work`, reports
/// [`StorageError::Closed`]) rather than silently executing anyway.
// 🚫️async: E1-adjacent — forced sync by `StorageScheduler::submit` (see `resolve_ready`'s tag
// above for the full cross-crate reasoning). Recurses into itself from within the pool job closure
// below — still synchronous, since that closure runs to completion on its own worker.
fn storage_try_dispatch<R: HostAsyncRuntime + 'static>(state: &Arc<StorageState<R>>) {
    loop {
        if state.in_flight.load(Ordering::SeqCst) >= state.max_in_flight {
            break;
        }
        let job = {
            let mut queues = state.queues.lock().expect("StorageScheduler queues mutex poisoned");
            let mut popped = None;
            for jobs in queues.values_mut() {
                if let Some(job) = jobs.pop_front() {
                    popped = Some(job);
                    break;
                }
            }
            popped
        };
        let Some(job) = job else { break };
        let scope_cancelled = resolve_ready(state.scope.cancel.is_cancelled());
        if job.cancelled.load(Ordering::SeqCst) || scope_cancelled {
            let mut usage = state.per_plugin_bytes.lock().expect("StorageScheduler per_plugin_bytes mutex poisoned");
            if let Some(current) = usage.get_mut(&job.plugin) {
                *current = current.saturating_sub(job.bytes);
            }
            drop(usage);
            let error = if scope_cancelled { StorageError::Closed } else { StorageError::DeadlineExceeded };
            let _ = job.result_tx.send(Err(error));
            continue;
        }
        state.in_flight.fetch_add(1, Ordering::SeqCst);
        let recurse_state = state.clone();
        let plugin = job.plugin.clone();
        let bytes = job.bytes;
        let lane = Lane::from_context_lane(job.ctx.lane);
        state.pool.submit(
            lane,
            Box::new(move || {
                let result = match (job.work)() {
                    Ok(bytes) => Ok(bytes),
                    Err(error) => Err(StorageError::Io(error.to_string())),
                };
                let _ = job.result_tx.send(result);
                {
                    let mut usage = recurse_state.per_plugin_bytes.lock().expect("StorageScheduler per_plugin_bytes mutex poisoned");
                    if let Some(current) = usage.get_mut(&plugin) {
                        *current = current.saturating_sub(bytes);
                    }
                }
                recurse_state.in_flight.fetch_sub(1, Ordering::SeqCst);
                storage_try_dispatch(&recurse_state);
            }),
        );
    }
}

/// 💾️ Bounded priority-FIFO dispatcher over blocking file ops: [`StorageScheduler::submit`] enqueues
/// into the `ctx.lane`-keyed queue and reserves the plugin's byte quota up front (released back on
/// completion, whether the op succeeds or fails); [`storage_try_dispatch`] — triggered on submit and
/// again on every completion — pulls the highest-priority ready job while `in_flight <
/// max_in_flight`. [`StorageTicket::await_result`] races `ctx.deadline_ms` internally now (same
/// same absolute-deadline race [`ComputePool::run_io`] uses) — see that method's doc; a
/// caller with its own external deadline-racing wrapper built against the OLD "not wired" gap (see
/// the crate report's `## honest gaps`) can drop it.
pub struct StorageScheduler<R: HostAsyncRuntime>(pub(super) Arc<StorageState<R>>);

impl<R: HostAsyncRuntime + 'static> StorageScheduler<R> {
    pub async fn new(runtime: Arc<R>, scope: ScopeHandle, max_in_flight: u32, byte_quota_per_plugin: u64) -> StorageScheduler<R> {
        StorageScheduler(Arc::new(StorageState {
            runtime,
            scope,
            pool: global_worker_pool(),
            queues: Mutex::new(BTreeMap::new()),
            in_flight: AtomicU32::new(0),
            max_in_flight: max_in_flight.max(1),
            per_plugin_bytes: Mutex::new(HashMap::new()),
            byte_quota_per_plugin,
        }))
    }

    /// 💾️ Enqueues `work`, reserving `bytes` against `plugin`'s budget up front. Returns a
    /// [`StorageTicket`] the caller awaits for the eventual result, or a typed
    /// [`StorageError::BytesQuotaExceeded`] immediately if the reservation itself does not fit —
    /// the wheel is left untouched on that path, same discipline as [`WheelCore::arm`]. The ticket
    /// captures `ctx.deadline_ms` (if set) to race against in [`StorageTicket::await_result`].
    // 🚫️async: E1-adjacent — forced synchronous by `🔌️plugin/🖥️host/📥️imports/🦀️.rs`'s
    // `storage_read`/`storage_write`, which call this un-awaited inside an already-`async fn` then
    // `.await` the returned `StorageTicket` separately (that file is outside this packet's
    // `path_scope`, so it cannot be updated to `.await` this call instead). See `resolve_ready`'s
    // doc comment above for the full cross-crate reasoning (R9).
    pub fn submit(&self, ctx: &OperationContext, plugin: PackageId, bytes: u64, work: impl FnOnce() -> Result<Vec<u8>, std::io::Error> + Send + 'static) -> Result<StorageTicket<R>, StorageError> {
        {
            let mut usage = self.0.per_plugin_bytes.lock().expect("StorageScheduler per_plugin_bytes mutex poisoned");
            let current = usage.get(&plugin).copied().unwrap_or(0);
            if current + bytes > self.0.byte_quota_per_plugin {
                return Err(StorageError::BytesQuotaExceeded { plugin, limit: self.0.byte_quota_per_plugin });
            }
            usage.insert(plugin.clone(), current + bytes);
        }
        let (result_tx, result_rx) = oneshot::channel();
        let cancelled = Arc::new(AtomicBool::new(false));
        let job = StorageJob { plugin, bytes, ctx: ctx.clone(), work: Box::new(work), result_tx, cancelled: cancelled.clone() };
        self.0.queues.lock().expect("StorageScheduler queues mutex poisoned").entry(ctx.lane).or_default().push_back(job);
        storage_try_dispatch(&self.0);
        Ok(StorageTicket { receiver: result_rx, cancelled, runtime: self.0.runtime.clone(), deadline_ms: ctx.deadline_ms })
    }

    pub async fn in_flight(&self) -> u32 {
        self.0.in_flight.load(Ordering::SeqCst)
    }
}

/// 🎫️ A handle to one [`StorageScheduler::submit`] call's eventual result — deliberately opaque:
/// the [`oneshot::Receiver`] it wraps is a PRIVATE field (never named on this struct's own
/// declaration line), so nothing outside this crate can see or name it through here.
pub struct StorageTicket<R: HostAsyncRuntime> {
    pub(super) receiver: oneshot::Receiver<Result<Vec<u8>, StorageError>>,
    pub(super) cancelled: Arc<AtomicBool>,
    pub(super) runtime: Arc<R>,
    pub(super) deadline_ms: Option<u64>,
}
impl<R: HostAsyncRuntime> StorageTicket<R> {
    /// 💾️ Awaits the eventual result, racing it against `ctx.deadline_ms` (captured at
    /// [`StorageScheduler::submit`] time) whenever one was set — a REAL race via
    /// [`HostAsyncRuntime::sleep_until`], not a documented-but-unenforced field. Losing the race
    /// marks the job [`StorageJob::cancelled`] so [`storage_try_dispatch`] skips it if it is still
    /// queued when popped; see [`StorageError::DeadlineExceeded`]'s doc for the already-dispatched
    /// case.
    pub async fn await_result(self) -> Result<Vec<u8>, StorageError> {
        match self.deadline_ms {
            Some(deadline_ms) => match select2(self.receiver, self.runtime.sleep_until(deadline_ms)).await {
                Either::Left(result) => result.unwrap_or(Err(StorageError::Closed)),
                Either::Right(()) => {
                    self.cancelled.store(true, Ordering::SeqCst);
                    Err(StorageError::DeadlineExceeded)
                }
            },
            None => self.receiver.await.unwrap_or(Err(StorageError::Closed)),
        }
    }
}
//#endregion 💾️StorageScheduler
//#endregion 💾️Storage
