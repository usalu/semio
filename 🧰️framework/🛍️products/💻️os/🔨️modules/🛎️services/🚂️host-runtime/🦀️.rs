//! 🚂️ The process-wide worker pool handle and `TokioHostRuntime`, the one `HostAsyncRuntime` implementation, with its scope table and pool-hosted future tasks.

use semio_framework_async::{
    CancelState,
    CancelToken,
    HostAsyncRuntime,
    HostFuture,
    Lane,
    OperationContext,
    ProcessKind,
    ScopeDrainReport,
    ScopeHandle,
    ScopeId,
    ScopeOwner,
    WorkerPool,
    WorkerPoolConfig,
    block_on,
    oneshot,
};
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::task::{Context, Poll, Wake, Waker};

//#region 🧵️GlobalWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every service subsystem.
pub(super) fn global_worker_pool() -> WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores))
}
//#endregion 🧵️GlobalWorkerPool

//#region 🚂️TokioHostRuntime
/// 🌳️ What a scope's spawned task decided about itself before running its real body — used only to
/// classify [`ScopeDrainReport`] buckets truthfully. Never exposed outside this crate.
enum TaskOutcome {
    Finished,
    CancelledEarly,
}

//#region 🔁️PoolFutureTask
/// 🔁️ One pool-hosted future polled exactly once per finite worker turn. A pending future keeps no
/// worker occupied: its waker schedules the next turn only after the current poll has returned.
struct PoolFutureTask {
    pub(super) pool: WorkerPool,
    pub(super) lane: Lane,
    pub(super) future: Mutex<Option<HostFuture<()>>>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    pub(super) complete: AtomicBool,
}

impl PoolFutureTask {
    pub(super) fn spawn(pool: WorkerPool, lane: Lane, future: HostFuture<()>) {
        let task = Arc::new(PoolFutureTask { pool, lane, future: Mutex::new(Some(future)), scheduled: AtomicBool::new(false), wake_requested: AtomicBool::new(false), complete: AtomicBool::new(false) });
        task.schedule();
    }

    fn schedule(self: &Arc<Self>) {
        if self.complete.load(Ordering::Acquire) || self.pool.is_shutdown() {
            return;
        }
        if self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let task = Arc::clone(self);
            self.pool.submit(self.lane, Box::new(move || task.poll_once()));
        }
    }

    fn poll_once(self: Arc<Self>) {
        if self.complete.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            return;
        }
        self.wake_requested.store(false, Ordering::Release);
        let waker = Waker::from(Arc::clone(&self));
        let mut context = Context::from_waker(&waker);
        let poll = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut future = self.future.lock().expect("PoolFutureTask future mutex poisoned");
            future.as_mut().map_or(Poll::Ready(()), |future| future.as_mut().poll(&mut context))
        }));
        match poll {
            Ok(Poll::Pending) => {
                self.scheduled.store(false, Ordering::Release);
                if self.wake_requested.swap(false, Ordering::AcqRel) {
                    self.schedule();
                }
            }
            Ok(Poll::Ready(())) | Err(_) => {
                self.complete.store(true, Ordering::Release);
                self.future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                self.scheduled.store(false, Ordering::Release);
            }
        }
    }

    fn request_wake(self: &Arc<Self>) {
        if self.complete.load(Ordering::Acquire) {
            return;
        }
        self.wake_requested.store(true, Ordering::Release);
        if !self.scheduled.load(Ordering::Acquire) {
            self.schedule();
        }
    }
}

impl Wake for PoolFutureTask {
    fn wake(self: Arc<Self>) {
        self.request_wake();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.request_wake();
    }
}
//#endregion 🔁️PoolFutureTask

/// 🌳️ One scope's identity/lineage/cancellation plus the outcome receivers tracking every task
/// [`ScopeTable::spawn_scoped`] put onto the shared [`WorkerPool`] — replaces the pre-Phase-1
/// `tokio::task::JoinSet` one-for-one: an [`oneshot::Receiver`] per spawned task, completed by its
/// resumable [`PoolFutureTask`]. Like the `JoinSet` it replaces, entries are only ever reclaimed by
/// [`ScopeTable::cancel_scope`] draining the whole
/// scope — a long-lived scope that spawns many short tasks and is never cancelled accumulates dead
/// receivers exactly as the old `JoinSet` accumulated dead `AbortHandle`s until `join_next` was
/// called; not a new limitation.
struct ScopeRecord {
    pub(super) cancel: CancelToken,
    tasks: Vec<oneshot::Receiver<TaskOutcome>>,
}

/// 🐢️ Poll interval a newly-submitted unit of work waits on while its scope is `Park`ed before
/// checking again — see [`await_live_or_cancelled`]'s doc for why this is a poll rather than an
/// event wake.
#[doc(hidden)] pub const PARK_POLL_INTERVAL_MS: u64 = 20;

/// 🐢️ Poll interval [`ScopeTable::cancel_scope`] re-checks its still-pending task receivers on while
/// draining a scope, bounded by the caller's own `grace_ms` — see that method's doc.
const CANCEL_DRAIN_POLL_MS: u64 = 5;

/// ⏳️ Waits until `cancel`'s effective state is no longer `Park`, returning `true` if it settled on
/// `Live` (the caller should run its real work) or `false` if it settled on `Cancelled` (the caller
/// should skip its real work entirely — this is how "new work is held while parked, and refused once
/// cancelled" is implemented). Polls on [`PARK_POLL_INTERVAL_MS`] rather than waking on an event
/// because `CancelToken` (frozen for this packet, from `semio-framework-async`) exposes only a
/// point-in-time `state()` read, no unpark notification. Sleeps through `pool`'s own [`TimerWheel`]
/// (`semio_framework_async::WorkerPool::timer`) rather than `tokio::time::sleep` — this crate no
/// longer builds a `tokio::runtime::Runtime`, so no tokio timer driver exists to service that call.
async fn await_live_or_cancelled(pool: &WorkerPool, cancel: &CancelToken) -> bool {
    loop {
        match cancel.state().await {
            CancelState::Live => return true,
            CancelState::Cancelled => return false,
            CancelState::Park => pool.timer().sleep_until(pool.now_ms() + PARK_POLL_INTERVAL_MS).await,
        }
    }
}

/// 🌳️ Root scope per package, child scope per actor, one outcome-receiver list per scope — see the
/// crate doc. Every [`TokioHostRuntime`] scope method delegates here. Deliberately NOT `pub`: this is
/// `TokioHostRuntime`'s own bookkeeping. Wraps its state in an `Arc` so
/// [`TokioHostRuntime::cancel_scope`] can clone [`ScopeTable`] and `.await` the drain without
/// borrowing `&self` across the async body.
#[derive(Clone)]
struct ScopeTable(Arc<ScopeTableInner>);

struct ScopeTableInner {
    pub(super) pool: WorkerPool,
    pub(super) next_id: AtomicU64,
    records: Mutex<HashMap<ScopeId, ScopeRecord>>,
    owner_index: Mutex<HashMap<ScopeOwner, ScopeId>>,
    children: Mutex<HashMap<ScopeId, Vec<ScopeId>>>,
}

impl ScopeTable {
    // 🚫️async: E1-adjacent pure constructor — no suspension point exists (`Arc::new`/
    // `AtomicU64::new`/`Mutex::new`/`WorkerPool::clone` are in-memory only). See R9.
    pub(super) fn new(pool: WorkerPool) -> ScopeTable {
        ScopeTable(Arc::new(ScopeTableInner { pool, next_id: AtomicU64::new(1), records: Mutex::new(HashMap::new()), owner_index: Mutex::new(HashMap::new()), children: Mutex::new(HashMap::new()) }))
    }

    /// 🔍️ Assumes at most one OPEN scope per `ScopeOwner` at a time (one root scope per package,
    /// one child scope per actor — see the crate doc). Re-opening the same owner replaces the
    /// owner-index entry; the earlier scope's tasks stay tracked by id but become unreachable via
    /// `cancel_scope(owner)` afterward. No caller in this packet re-opens an owner, so this is a
    /// documented limitation rather than an exercised bug.
    fn open_scope_now(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        let id = ScopeId(self.0.next_id.fetch_add(1, Ordering::SeqCst));
        let cancel = match parent {
            Some(parent_handle) => parent_handle.cancel.child_now(),
            None => CancelToken::root_now(),
        };
        let parent_id = parent.map(|handle| handle.id);
        self.0.records.lock().expect("ScopeTable records mutex poisoned").insert(id, ScopeRecord { cancel: cancel.clone(), tasks: Vec::new() });
        self.0.owner_index.lock().expect("ScopeTable owner_index mutex poisoned").insert(owner.clone(), id);
        if let Some(parent_id) = parent_id {
            self.0.children.lock().expect("ScopeTable children mutex poisoned").entry(parent_id).or_default().push(id);
        }
        ScopeHandle { id, owner, cancel }
    }

    pub(super) async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        self.open_scope_now(owner, parent)
    }

    /// ▶️ Submits `fut` as ONE [`WorkerPool`] job on the [`Lane`] `ctx.lane` maps to
    /// ([`Lane::from_context_lane`]) — [`block_on`] (this crate's `entrypoint`-feature import of
    /// [`PoolFutureTask`] polls `fut` once per finite worker turn. Pending I/O or timers retain only
    /// their waker and consume no worker, permit, or admission slot between wakes.
    // 🚫️async: E1-adjacent — no suspension point of its own (only SUBMITS the job; never drives it
    // here). Consumed by `TokioHostRuntime::spawn_scoped`, already `async fn` to match the trait,
    // which calls this plainly — an `async fn` here would add a second, pointless suspension layer.
    // See R9.
    pub(super) fn spawn_scoped(&self, scope: &ScopeHandle, ctx: &OperationContext, fut: HostFuture<()>) {
        let cancel = ctx.cancel.clone();
        let lane = Lane::from_context_lane(ctx.lane);
        let (result_tx, result_rx) = oneshot::channel::<TaskOutcome>();
        let mut records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
        let Some(record) = records.get_mut(&scope.id) else { return };
        record.tasks.push(result_rx);
        drop(records);
        let pool = self.0.pool.clone();
        let wait_pool = pool.clone();
        PoolFutureTask::spawn(
            pool,
            lane,
            Box::pin(async move {
                let outcome = if await_live_or_cancelled(&wait_pool, &cancel).await {
                    fut.await;
                    TaskOutcome::Finished
                } else {
                    TaskOutcome::CancelledEarly
                };
                let _ = result_tx.send(outcome);
            }),
        );
    }

    async fn collect_descendants(&self, root: ScopeId) -> Vec<ScopeId> {
        let children = self.0.children.lock().expect("ScopeTable children mutex poisoned");
        let mut out = vec![root];
        let mut frontier = vec![root];
        while let Some(id) = frontier.pop() {
            if let Some(kids) = children.get(&id) {
                for kid in kids {
                    out.push(*kid);
                    frontier.push(*kid);
                }
            }
        }
        out
    }

    /// 🛑️ Cancels `owner`'s scope and every descendant [`collect_descendants`](ScopeTable::collect_descendants)
    /// can reach, flipping each recorded descendant's OWN token too — defensive, since the
    /// transitive fold in `CancelToken::child` already covers scopes opened with a live parent
    /// link, this just keeps the bookkeeping honest either way. Drains each scope's outcome
    /// receivers within `grace_ms` by polling [`oneshot::Receiver::try_recv`] (a
    /// non-blocking, runtime-context-free call) on a [`CANCEL_DRAIN_POLL_MS`] tick — whatever is
    /// still pending once the grace period elapses is honestly counted `leaked` (never silently
    /// folded into `finished`); a blocking OS thread already underway cannot be preempted, so unlike
    /// the old `JoinSet::abort_all` there is nothing further to force-stop here — the [`WorkerPool`]
    /// worker running it keeps running it to completion regardless, exactly the same honest
    /// limitation [`ComputeError::DeadlineExceeded`] documents.
    pub(super) async fn cancel_scope(self, owner: ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
        let root_id = match self.0.owner_index.lock().expect("ScopeTable owner_index mutex poisoned").get(&owner).copied() {
            Some(id) => id,
            None => return ScopeDrainReport::default(),
        };
        let scope_ids = self.collect_descendants(root_id).await;
        let cancels: Vec<CancelToken> = {
            let records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
            scope_ids.iter().filter_map(|id| records.get(id).map(|record| record.cancel.clone())).collect()
        };
        for cancel in &cancels {
            cancel.cancel().await;
        }
        let deadline = self.0.pool.now_ms() + grace_ms;
        let mut report = ScopeDrainReport::default();
        for id in &scope_ids {
            let tasks = {
                let mut records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
                match records.get_mut(id) {
                    Some(record) => std::mem::take(&mut record.tasks),
                    None => continue,
                }
            };
            let mut pending = tasks;
            loop {
                let mut still_pending = Vec::new();
                for mut receiver in pending {
                    match receiver.try_recv() {
                        Ok(TaskOutcome::Finished) => report.finished += 1,
                        Ok(TaskOutcome::CancelledEarly) => report.cancelled += 1,
                        Err(oneshot::TryRecvError::Closed) => report.cancelled += 1,
                        Err(oneshot::TryRecvError::Empty) => still_pending.push(receiver),
                    }
                }
                pending = still_pending;
                if pending.is_empty() || self.0.pool.now_ms() >= deadline {
                    break;
                }
                let wake_at = (self.0.pool.now_ms() + CANCEL_DRAIN_POLL_MS).min(deadline);
                self.0.pool.timer().sleep_until(wake_at).await;
            }
            report.leaked += pending.len() as u32;
        }
        report
    }
}

/// 🚂️ The one [`HostAsyncRuntime`] implementation this crate ships. Owns NO thread pool of its own
/// any more: `pool` is the shared, process-wide [`WorkerPool`] (see [`global_worker_pool`]'s doc for
/// why a lazy static, not full dependency injection, is this packet's honest compromise) — every
/// `HostAsyncRuntime` method below and every [`ScopeTable`] task delegates its real work to it.
pub struct TokioHostRuntime {
    scopes: ScopeTable,
    #[doc(hidden)] pub pool: WorkerPool,
}

impl TokioHostRuntime {
    /// 🚂️ Builds a host runtime backed by [`global_worker_pool`] — the crate-wide default every
    /// frozen-signature constructor here ([`ComputePool::new`], [`HttpPool::new`],
    /// [`StorageScheduler::new`], [`TimerWheel::new`]) also resolves, so a `TokioHostRuntime` built
    /// this way shares its real OS threads with them. Infallible: unlike the deleted
    /// `tokio::runtime::Builder::build()` path, constructing a [`ScopeTable`] over an already-live
    /// [`WorkerPool`] cannot fail.
    pub fn new() -> TokioHostRuntime {
        Self::with_pool(global_worker_pool())
    }

    /// 🚂️ Builds a host runtime over a CALLER-OWNED [`WorkerPool`] — the real dependency-injection
    /// path for a process entry point that wants its own independently-sized pool (a `HeadlessBatch`
    /// CLI, or a test wanting deterministic sizing rather than [`global_worker_pool`]'s
    /// oversubscribed default) instead of the crate-wide singleton [`TokioHostRuntime::new`] resolves.
    pub fn with_pool(pool: WorkerPool) -> TokioHostRuntime {
        TokioHostRuntime { scopes: ScopeTable::new(pool.clone()), pool }
    }

    /// 🧵️ The worker pool this runtime schedules onto, for a service that submits its own maintenance
    /// turns to the same threads (an [`HttpPool`]'s refill driver).
    pub fn worker_pool(&self) -> &WorkerPool {
        &self.pool
    }

    pub fn open_scope_now(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        self.scopes.open_scope_now(owner, parent)
    }

    /// 🧵️ Drives `f` to completion on the calling thread — the entry point a process bootstrap uses
    /// for its own top-level async setup before handing scopes to services. This crate's designated
    /// executor entry point (R4 sanctions `semio-framework-os-services` by name); every `#[test]`
    /// in this file that drives real suspending work goes through THIS bridge, per R4 Clause 5.
    /// Delegates to `semio_framework_async::block_on` (this crate's `entrypoint`-feature import) —
    /// the SAME bridge every [`ScopeTable`]-submitted [`WorkerPool`] job uses internally, just called
    /// here on whatever thread the caller is already on rather than inside a pool job.
    ///
    /// 🚫️async: E5 executor bridge — the thread-becomes-executor boundary; an `async fn` can only be
    /// driven by something already polling it, so this cannot itself be `async` without begging the
    /// question (same reasoning as `semio-framework-async`'s own `block_on`, this crate's sibling
    /// bridge one layer down).
    pub fn block_on<F: Future>(&self, f: F) -> F::Output {
        block_on(f)
    }
}

impl Default for TokioHostRuntime {
    fn default() -> TokioHostRuntime {
        TokioHostRuntime::new()
    }
}

impl HostAsyncRuntime for TokioHostRuntime {
    async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        self.scopes.open_scope(owner, parent).await
    }

    async fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) {
        self.scopes.spawn_scoped(scope, &ctx, fut);
    }

    async fn sleep_until(&self, deadline_ms: u64) {
        self.pool.timer().sleep_until(deadline_ms).await;
    }

    async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
        self.scopes.clone().cancel_scope(owner.clone(), grace_ms).await
    }

    async fn now_ms(&self) -> u64 {
        self.pool.now_ms()
    }
}

//#endregion 🚂️TokioHostRuntime
