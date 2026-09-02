//! 🛎️ Host-side async SERVICES built ON TOP of `semio-framework-async`'s `HostAsyncRuntime` — the
//! ONE crate in this tree allowed to name `tokio`. Every other framework/os/plugin crate reaches
//! this functionality only through `semio-framework-async` vocabulary (`OperationContext`,
//! `ScopeHandle`, `ChannelPolicy`, `ThreadPlan`) or this crate's own domain types — never a tokio
//! type — exactly as `wasmtime` is confined behind `GuestRuntime` in the plugin-host crate. Verify
//! this holds with `grep -nE 'tokio' 🦀️.rs | grep -v '^\s*[0-9]*://'` against every `pub
//! fn`/`pub struct` signature (see the packet report's `## tokio-containment evidence`).
//!
//! 🚂️ [`TokioHostRuntime`] is the one `HostAsyncRuntime` implementation. Every other public type
//! here ([`TimerWheel`], [`ComputePool`], [`HttpPool`], [`StorageScheduler`], [`EventRouter`]) is a
//! SERVICE built on top of that trait, reaching around it into raw tokio only where explicitly
//! noted (the timer driver task, the HTTP bucket refill driver, the storage dispatcher) — never
//! around `semio-framework-async`.
//!
//! 🧾️ Re-entry into the kernel from any of these services happens ONLY through [`CompletionSink`].
//! No type in this crate holds or calls a `Kernel` directly.
//!
//! 🏷️ Naming: per-plugin quota accounting ([`TimerWheel`]'s armed-timer count,
//! [`StorageScheduler`]'s byte budget, [`HttpPool`]'s byte bucket) and per-actor accounting
//! ([`HttpPool`]'s outstanding cap, [`EventRouter`]'s subscribers) use `semio_framework_actor`'s
//! own [`PackageId`]/[`ActorId`] directly rather than a parallel local newtype for the same
//! concept — unlike `semio-framework-async` (which stays domain-neutral and keeps
//! `OperationContext.actor` a bare `u64` on purpose), this crate IS the OS-tier host serving this
//! kernel, and the actor crate is pure (no tokio, no threads, builds for `wasm32-unknown-unknown`),
//! so depending on it costs no platform coupling. `🔌️plugin/🖥️host` already depends on it the same
//! way. `CompletionSink::complete` and [`TimerFired`]'s `actor`/`generation` fields stay bare
//! `u64`/`u16`, unrelated to this: they mirror `OperationContext`'s own untyped re-entry shape, and
//! `OperationContext.generation` is a different concept from `ActorId`'s packed 14-bit
//! restart-generation bits — converting one into the other would be a category error, not a typing
//! improvement.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{
    block_on, oneshot, select2, CancelState, CancelToken, ChannelPolicy, Either, HostAsyncRuntime, HostFuture, Lane, OperationContext, OwnedPermit, ProcessKind, ScopeDrainReport, ScopeHandle, ScopeId, ScopeOwner, Semaphore, WorkerPool,
    WorkerPoolConfig,
};
use semio_framework_job::{
    default_now_us, Generation as JobGeneration, InteractiveJob, InteractiveStage, OperationId, StepOutcome, BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US,
    MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US,
};

//#region 🧵️GlobalWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every service subsystem.
fn global_worker_pool() -> WorkerPool {
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
    pool: WorkerPool,
    lane: Lane,
    future: Mutex<Option<HostFuture<()>>>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    complete: AtomicBool,
}

impl PoolFutureTask {
    fn spawn(pool: WorkerPool, lane: Lane, future: HostFuture<()>) {
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
    cancel: CancelToken,
    tasks: Vec<oneshot::Receiver<TaskOutcome>>,
}

/// 🐢️ Poll interval a newly-submitted unit of work waits on while its scope is `Park`ed before
/// checking again — see [`await_live_or_cancelled`]'s doc for why this is a poll rather than an
/// event wake.
const PARK_POLL_INTERVAL_MS: u64 = 20;

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
    pool: WorkerPool,
    next_id: AtomicU64,
    records: Mutex<HashMap<ScopeId, ScopeRecord>>,
    owner_index: Mutex<HashMap<ScopeOwner, ScopeId>>,
    children: Mutex<HashMap<ScopeId, Vec<ScopeId>>>,
}

impl ScopeTable {
    // 🚫️async: E1-adjacent pure constructor — no suspension point exists (`Arc::new`/
    // `AtomicU64::new`/`Mutex::new`/`WorkerPool::clone` are in-memory only). See R9.
    fn new(pool: WorkerPool) -> ScopeTable {
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

    async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
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
    fn spawn_scoped(&self, scope: &ScopeHandle, ctx: &OperationContext, fut: HostFuture<()>) {
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
    async fn cancel_scope(self, owner: ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
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
    pool: WorkerPool,
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

//#region ⏲️TimerWheel
/// 🔖️ Identity of one armed timer, minted by [`WheelCore::arm`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimerId(u64);

/// 🚫️ [`WheelCore::arm`]'s only failure mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimerError {
    QuotaExceeded { plugin: PackageId, limit: u32 },
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::QuotaExceeded { plugin, limit } => write!(f, "plugin {:?} already has {limit} timers armed", plugin.0),
        }
    }
}
impl std::error::Error for TimerError {}

/// 🔔️ What fired: enough to route a [`CompletionSink::complete`] call to the right actor without
/// `WheelCore` needing to know anything about actors beyond these four bare fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimerFired {
    pub id: TimerId,
    pub plugin: PackageId,
    pub actor: u64,
    pub generation: u16,
    pub lane: u8,
}

struct TimerEntry {
    plugin: PackageId,
    actor: u64,
    generation: u16,
    lane: u8,
    expiry_ms: u64,
    repeat_ms: Option<u64>,
    cancelled: bool,
}

/// ⏲️ Pure timer-wheel arithmetic: no tokio, no clock of its own — every `now_ms` is caller-supplied
/// — and unit-testable directly with no runtime at all. Exactly ONE of these exists per host process
/// (owned by [`TimerWheel`]); plugins declare timers through it and never own a timer of their own.
pub struct WheelCore {
    next_id: u64,
    next_seq: u64,
    entries: HashMap<TimerId, TimerEntry>,
    order: BinaryHeap<Reverse<(u64, u64, TimerId)>>,
    per_plugin_counts: HashMap<PackageId, u32>,
    quota_per_plugin: u32,
}

impl WheelCore {
    pub async fn new(quota_per_plugin: u32) -> WheelCore {
        WheelCore { next_id: 1, next_seq: 0, entries: HashMap::new(), order: BinaryHeap::new(), per_plugin_counts: HashMap::new(), quota_per_plugin }
    }

    /// ⏲️ Arms a timer for `plugin`/`actor` firing at `at_ms`, optionally repeating every
    /// `repeat_ms`. The per-plugin quota is checked BEFORE any insertion, so a rejected call leaves
    /// the wheel completely untouched.
    // 🚫️async: E1-adjacent — identical reasoning to `pop_expired`/`next_expiry_ms` below (R9): pure,
    // in-memory only, with no suspension point of its own, and reached through a
    // `std::sync::Mutex<WheelCore>` guard held by an async caller. `async` here forced that
    // `MutexGuard` (not `Send`) to live across the caller's await point, making
    // `TimerWheel::arm`'s future non-`Send` and breaking the `HostFuture<()>: Send` bound R3 requires.
    #[allow(clippy::too_many_arguments)]
    pub fn arm(&mut self, plugin: PackageId, actor: u64, generation: u16, lane: u8, at_ms: u64, repeat_ms: Option<u64>) -> Result<TimerId, TimerError> {
        let count = self.per_plugin_counts.get(&plugin).copied().unwrap_or(0);
        if count >= self.quota_per_plugin {
            return Err(TimerError::QuotaExceeded { plugin, limit: self.quota_per_plugin });
        }
        let id = TimerId(self.next_id);
        self.next_id += 1;
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.insert(id, TimerEntry { plugin: plugin.clone(), actor, generation, lane, expiry_ms: at_ms, repeat_ms, cancelled: false });
        self.order.push(Reverse((at_ms, seq, id)));
        *self.per_plugin_counts.entry(plugin).or_insert(0) += 1;
        Ok(id)
    }

    /// 🔕️ Disarms `id`, returning whether it was actually armed. Lazy: the heap entry is left in
    /// place and skipped when eventually popped by [`WheelCore::pop_expired`], since a `BinaryHeap`
    /// cannot remove an arbitrary interior element faster than O(n).
    // 🚫️async: E1-adjacent — same reasoning as `arm` above (R9).
    pub fn disarm(&mut self, id: TimerId) -> bool {
        match self.entries.get_mut(&id) {
            Some(entry) if !entry.cancelled => {
                entry.cancelled = true;
                if let Some(count) = self.per_plugin_counts.get_mut(&entry.plugin) {
                    *count = count.saturating_sub(1);
                }
                true
            }
            _ => false,
        }
    }

    /// 🔔️ Pops every timer whose expiry is `<= now_ms`, rearming repeaters at `expiry + repeat_ms`
    /// and catching up (rather than drifting) if more than one repeat period has actually elapsed.
    /// Cancelled entries are dropped here rather than rearmed — this is where lazy
    /// [`WheelCore::disarm`] deletion actually reclaims heap/map space.
    // 🚫️async: E1-adjacent pure computation (in-memory heap only, no suspension point) whose sole
    // caller holds it behind a `std::sync::Mutex<WheelCore>` inside the finite timer-driver turn.
    pub fn pop_expired(&mut self, now_ms: u64) -> Vec<TimerFired> {
        self.pop_expired_batch(now_ms, usize::MAX)
    }

    /// ⏱️ Pops at most `max_items` due timers so one driver turn has a hard item bound.
    pub fn pop_expired_batch(&mut self, now_ms: u64, max_items: usize) -> Vec<TimerFired> {
        let mut fired = Vec::new();
        let mut processed_items = 0usize;
        while processed_items < max_items {
            let Some(&Reverse((expiry, _seq, id))) = self.order.peek() else { break };
            if expiry > now_ms {
                break;
            }
            self.order.pop();
            processed_items += 1;
            let Some(entry) = self.entries.get_mut(&id) else { continue };
            if entry.cancelled {
                self.entries.remove(&id);
                continue;
            }
            fired.push(TimerFired { id, plugin: entry.plugin.clone(), actor: entry.actor, generation: entry.generation, lane: entry.lane });
            match entry.repeat_ms {
                Some(repeat) if repeat > 0 => {
                    let mut next_expiry = entry.expiry_ms + repeat;
                    while next_expiry <= now_ms {
                        next_expiry += repeat;
                    }
                    entry.expiry_ms = next_expiry;
                    let seq = self.next_seq;
                    self.next_seq += 1;
                    self.order.push(Reverse((next_expiry, seq, id)));
                }
                _ => {
                    let plugin = entry.plugin.clone();
                    self.entries.remove(&id);
                    if let Some(count) = self.per_plugin_counts.get_mut(&plugin) {
                        *count = count.saturating_sub(1);
                    }
                }
            }
        }
        fired
    }

    /// ⏰️ The earliest heaped expiry. A cancelled head may cause one harmless early driver turn;
    /// bounded expiry processing discards it without an unbounded scan here.
    // 🚫️async: E1-adjacent — same reasoning as `pop_expired` above (R9): pure, in-memory only, and
    // held behind a `std::sync::Mutex` across an async caller.
    pub fn next_expiry_ms(&self) -> Option<u64> {
        self.order.peek().map(|Reverse((expiry, _, _))| *expiry)
    }

    // 🚫️async: E1-adjacent — same reasoning as `arm` above (R9).
    pub fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.per_plugin_counts.get(plugin).copied().unwrap_or(0)
    }
}

/// 🐌️ Maximum delay between finite timer-driver turns while the wheel is empty.
const TIMER_DRIVER_MAX_IDLE_MS: u64 = 4;
const TIMER_DRIVER_BATCH_ITEMS: usize = 8;

/// ⏲️ The ONE host timer wheel for every plugin's timers — see the crate doc. Owns a [`WheelCore`]
/// behind a `Mutex` plus the finite driver chain ([`TimerWheel::spawn_driver`]) that posts
/// firings to a [`CompletionSink`]. Plugins arm/disarm through here; they must never spin up a timer
/// of their own.
pub struct TimerWheel {
    core: Arc<Mutex<WheelCore>>,
    driver_started: AtomicBool,
}

impl TimerWheel {
    pub async fn new(quota_per_plugin: u32) -> TimerWheel {
        TimerWheel { core: Arc::new(Mutex::new(WheelCore::new(quota_per_plugin).await)), driver_started: AtomicBool::new(false) }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn arm(&self, plugin: PackageId, actor: u64, generation: u16, lane: u8, at_ms: u64, repeat_ms: Option<u64>) -> Result<TimerId, TimerError> {
        let id = self.core.lock().expect("TimerWheel core mutex poisoned").arm(plugin, actor, generation, lane, at_ms, repeat_ms)?;
        Ok(id)
    }

    pub async fn disarm(&self, id: TimerId) -> bool {
        self.core.lock().expect("TimerWheel core mutex poisoned").disarm(id)
    }

    pub async fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.core.lock().expect("TimerWheel core mutex poisoned").armed_count(plugin)
    }

    /// ▶️ Starts a chain of finite timer-lane turns. Each turn handles at most
    /// [`TIMER_DRIVER_BATCH_ITEMS`] firings and schedules its successor without occupying a worker
    /// while it waits. The four-millisecond idle cadence bounds newly-armed-timer latency without a
    /// permanent polling task.
    // 🚫️async: E1-adjacent — no suspension point of its own (only SUBMITS the job; never drives it
    // here). See R9.
    pub fn spawn_driver(&self, pool: &WorkerPool, sink: Arc<dyn CompletionSink>) {
        if self.driver_started.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            schedule_timer_driver_turn(pool, self.core.clone(), sink);
        }
    }
}

fn schedule_timer_driver_turn(pool: &WorkerPool, core: Arc<Mutex<WheelCore>>, sink: Arc<dyn CompletionSink>) {
    let now_ms = pool.now_ms();
    let deadline_ms = core.lock().expect("TimerWheel core mutex poisoned").next_expiry_ms().unwrap_or(now_ms + TIMER_DRIVER_MAX_IDLE_MS).min(now_ms + TIMER_DRIVER_MAX_IDLE_MS);
    let turn_pool = pool.clone();
    pool.submit_at(
        deadline_ms,
        Lane::Timer,
        Box::new(move || {
            let now_ms = turn_pool.now_ms();
            let fired = core.lock().expect("TimerWheel core mutex poisoned").pop_expired_batch(now_ms, TIMER_DRIVER_BATCH_ITEMS);
            for timer in fired {
                sink.complete(timer.actor, timer.generation, timer.id.0.to_le_bytes().to_vec(), timer.lane);
            }
            schedule_timer_driver_turn(&turn_pool, core, sink);
        }),
    );
}
//#endregion ⏲️TimerWheel

//#region 🧮️ComputePool
/// 🚫️ [`ComputePool`]'s dispatch failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComputeError {
    /// ⏰️ `ctx.deadline_ms` elapsed before admission or terminal completion. Interactive jobs
    /// receive cancellation immediately and observe it at their next bounded step; platform I/O
    /// remains non-preemptible once its operating-system call has started.
    DeadlineExceeded,
    /// 💥️ The result channel closed before a value arrived (the worker thread panicked, or the
    /// runtime is shutting down).
    WorkerLost,
}

impl std::fmt::Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeError::DeadlineExceeded => write!(f, "compute deadline exceeded"),
            ComputeError::WorkerLost => write!(f, "compute worker lost before returning a result"),
        }
    }
}
impl std::error::Error for ComputeError {}

/// 🧮️ Bounds every interactive compute job and blocking platform-I/O call admitted to
/// `capacity`, independent of
/// [`WorkerPool`]'s own (larger, process-wide) `worker_count` — the real dispatch substrate is now
/// [`global_worker_pool`] (this type's constructor is frozen by external call sites this packet's
/// boundary does not own — see that fn's doc — so it cannot accept an injected pool), but a
/// [`Semaphore`] sized to `capacity` still gates LOGICAL admission on top of it, and is also what
/// lets `ctx.deadline_ms` be enforced honestly (a bounded semaphore has somewhere to race a timeout
/// against; submitting straight to the pool with no local admission gate would not).
pub struct ComputePool {
    admission: Arc<Semaphore>,
    pool: WorkerPool,
}

impl ComputePool {
    pub async fn new(capacity: u32) -> ComputePool {
        Self::with_pool(capacity, global_worker_pool())
    }

    pub fn with_pool(capacity: u32, pool: WorkerPool) -> ComputePool {
        ComputePool { admission: Arc::new(Semaphore::new(capacity.max(1) as usize)), pool }
    }

    /// 🧮️ Drives `job` to a terminal outcome on [`global_worker_pool`] through one retained mounted
    /// session. Every worker closure pumps at most one bounded session transition; resumable outcomes
    /// enqueue a fresh closure on the context lane. The admission permit spans the whole job. Cancellation is checked
    /// before admission and inside every step, while an absolute deadline cancels the job and returns
    /// [`ComputeError::DeadlineExceeded`].
    #[cfg(test)]
    pub async fn run_job<J: InteractiveJob + 'static, R: HostAsyncRuntime>(&self, runtime: &R, _scope: &ScopeHandle, ctx: OperationContext, job: J) -> Result<StepOutcome, ComputeError> {
        let lane = Lane::from_context_lane(ctx.lane);
        let Some(permit) = self.acquire_job_permit(runtime, &ctx).await? else { return Ok(StepOutcome::Cancelled) };
        if ctx.cancel.is_cancelled().await {
            return Ok(StepOutcome::Cancelled);
        }
        let (stage, fuel, wall_us) = compute_job_budget(lane);
        let params = semio_framework_job::BatchJobParams {
            operation: OperationId(ctx.trace.0),
            generation: JobGeneration(u64::from(ctx.generation)),
            cancel: ctx.cancel.clone(),
            config: semio_framework_job::BatchDriveConfig { site: "os-services.compute-job", stage, fuel_per_step: fuel, step_budget_us: wall_us },
            now_us: default_now_us,
        };
        let session = match semio_framework_job::MountedWorkerJobSession::try_new(job, params) {
            Ok(session) => session,
            Err(mut rejected) => {
                rejected.begin_close();
                while !rejected.terminal_is_empty() {
                    let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                return Err(ComputeError::WorkerLost);
            }
        };
        let (result_tx, result_rx) = oneshot::channel();
        let state = Arc::new(Mutex::new(ComputeJobDriveState { session, lane, retained_outcome: None, sender: Some(result_tx), _permit: permit }));
        schedule_compute_job_step(&self.pool, state);
        match ctx.deadline_ms {
            Some(deadline_ms) => match select2(result_rx, runtime.sleep_until(deadline_ms)).await {
                Either::Left(result) => result.map_err(|_| ComputeError::WorkerLost),
                Either::Right(()) => {
                    ctx.cancel.cancel().await;
                    Err(ComputeError::DeadlineExceeded)
                }
            },
            None => result_rx.await.map_err(|_| ComputeError::WorkerLost),
        }
    }

    /// 🌐️ Runs a blocking platform-I/O boundary on the pool's dedicated fair I/O lane.
    pub async fn run_io<T: Send + 'static, R: HostAsyncRuntime>(&self, runtime: &R, _scope: &ScopeHandle, ctx: OperationContext, work: impl FnOnce() -> T + Send + 'static) -> Result<T, ComputeError> {
        self.run_in_lane(runtime, ctx, Lane::Io, work).await
    }

    async fn acquire_job_permit<R: HostAsyncRuntime>(&self, runtime: &R, ctx: &OperationContext) -> Result<Option<OwnedPermit>, ComputeError> {
        loop {
            if ctx.cancel.is_cancelled().await {
                return Ok(None);
            }
            let now_ms = runtime.now_ms().await;
            if ctx.deadline_ms.is_some_and(|deadline| now_ms >= deadline) {
                return Err(ComputeError::DeadlineExceeded);
            }
            let poll_deadline = ctx.deadline_ms.map_or(now_ms.saturating_add(1), |deadline| deadline.min(now_ms.saturating_add(1)));
            match select2(self.admission.acquire_owned(), runtime.sleep_until(poll_deadline)).await {
                Either::Left(permit) => return Ok(Some(permit)),
                Either::Right(()) => {}
            }
        }
    }

    async fn run_in_lane<T: Send + 'static, R: HostAsyncRuntime>(&self, runtime: &R, ctx: OperationContext, lane: Lane, work: impl FnOnce() -> T + Send + 'static) -> Result<T, ComputeError> {
        let admission = self.admission.clone();
        let permit = match ctx.deadline_ms {
            Some(deadline_ms) => match select2(admission.acquire_owned(), runtime.sleep_until(deadline_ms)).await {
                Either::Left(permit) => permit,
                Either::Right(()) => return Err(ComputeError::DeadlineExceeded),
            },
            None => admission.acquire_owned().await,
        };
        let (result_tx, result_rx) = oneshot::channel::<T>();
        self.pool.submit(
            lane,
            Box::new(move || {
                let _permit = permit;
                let result = work();
                let _ = result_tx.send(result);
            }),
        );
        match ctx.deadline_ms {
            Some(deadline_ms) => match select2(result_rx, runtime.sleep_until(deadline_ms)).await {
                Either::Left(result) => result.map_err(|_| ComputeError::WorkerLost),
                Either::Right(()) => Err(ComputeError::DeadlineExceeded),
            },
            None => result_rx.await.map_err(|_| ComputeError::WorkerLost),
        }
    }
}

#[cfg(test)]
struct ComputeJobDriveState<J> {
    session: semio_framework_job::MountedWorkerJobSession<J>,
    lane: Lane,
    retained_outcome: Option<StepOutcome>,
    sender: Option<oneshot::Sender<StepOutcome>>,
    _permit: OwnedPermit,
}

#[cfg(test)]
fn compute_job_budget(lane: Lane) -> (InteractiveStage, u64, u64) {
    match lane {
        Lane::Interactive => (InteractiveStage::InteractiveStep, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US),
        Lane::UserVisible => (InteractiveStage::UserVisibleSimStep, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US),
        Lane::Background | Lane::Io | Lane::Timer => (InteractiveStage::BackgroundStep, BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US),
        Lane::Maintenance => (InteractiveStage::BackgroundStep, MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US),
    }
}

#[cfg(test)]
fn schedule_compute_job_step<J: InteractiveJob + 'static>(pool: &WorkerPool, state: Arc<Mutex<ComputeJobDriveState<J>>>) {
    let next_pool = pool.clone();
    let lane = state.lock().expect("ComputeJobDriveState mutex poisoned").lane;
    pool.submit(
        lane,
        Box::new(move || {
            let terminal = {
                let mut state = state.lock().expect("ComputeJobDriveState mutex poisoned");
                if let Some(outcome) = state.retained_outcome.as_mut() {
                    let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    if outcome.terminal_is_empty() {
                        state.retained_outcome = None;
                        let _ = state.session.resume();
                    }
                    None
                } else {
                    let lane = state.lane;
                    match state.session.pump_one(&next_pool, lane) {
                        Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {
                            let outcome = state.session.take_checked_out_outcome().expect("mounted compute session checked out one exact outcome");
                            if outcome.is_terminal() {
                                Some((state.sender.take().expect("terminal compute job has a result sender"), outcome))
                            } else {
                                state.retained_outcome = Some(outcome);
                                None
                            }
                        }
                        Ok(_) | Err(_) => None,
                    }
                }
            };
            if let Some((sender, outcome)) = terminal {
                let _ = sender.send(outcome);
            } else {
                schedule_compute_job_step(&next_pool, state);
            }
        }),
    );
}
//#endregion 🧮️ComputePool

//#region 🌐️HttpPool
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// 🧾️ Response status/headers only — the body is [`HttpBody`], pulled separately, so a caller can
/// see the head (and reject on status) before committing to draining a possibly-huge body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponseHead {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

/// 🚫️ [`HttpPool::request`]'s failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpPoolError {
    OutstandingCapReached { actor: ActorId, limit: u32 },
    ByteBudgetExhausted { package: PackageId },
    Transport(String),
    Compute(ComputeError),
}

impl std::fmt::Display for HttpPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpPoolError::OutstandingCapReached { actor, limit } => write!(f, "actor {actor:?} already has {limit} outstanding HTTP requests"),
            HttpPoolError::ByteBudgetExhausted { package } => write!(f, "package {:?} exhausted its network_bytes_per_min budget", package.0),
            HttpPoolError::Transport(message) => write!(f, "http transport error: {message}"),
            HttpPoolError::Compute(error) => write!(f, "http compute error: {error}"),
        }
    }
}
impl std::error::Error for HttpPoolError {}

/// 🌊️ One streamed HTTP response body, pulled chunk by chunk. `next_chunk` returns `Ok(None)` at
/// EOF. Implementations reach `&mut self` synchronously (to extract whatever owned state the next
/// read needs) and return a `'static` [`HostFuture`] that owns that state — never a future borrowing
/// `self` — the same shape [`AsyncHttpTransport::start`] itself uses. This is the ONE body type fed
/// to [`HttpPool::fetch`]'s [`HttpPoolBody`] wrapper; a later packet reuses it verbatim for the WASI
/// `stream<u8>` writer and the poll world's chunked events — see the crate report's `## seam design`.
// 🔀️ dedyn-fw-os-misc: DELIBERATELY left `dyn` — a reasoned exception, not an oversight.
// `next_chunk` is plain sync (returns a `HostFuture`, doesn't take the `async` keyword), so `dyn
// HttpBody` is not an E0038 violation and stays R1-legal. Neither de-dyn mechanism fits: (a) closed
// set — the real production set is exactly ONE impl (`BufferedHttpBody`, R11 case 3 would apply),
// but `AsyncHttpTransport::start` (a sibling trait, out of this packet's family list) returns
// `Box<dyn HttpBody>` from its OWN trait-level signature, and a `#[cfg(test)]`-only second
// implementor (`LocalSocketBody`, this file's own tests) implements that SAME trait method with a
// genuinely different concrete body type — collapsing to one concrete type would break the test
// impl, and fixing that means giving `AsyncHttpTransport` an associated type, out of scope here. (b)
// `dyn_enum_close!` — its variant DSL has no per-variant `#[cfg]` (confirmed empirically for
// `VersionGraph`'s own `#[cfg(feature = "vcs")]` case, same file family), so a `#[cfg(test)]`-only
// variant cannot be expressed in one enum declaration. Revisit alongside `AsyncHttpTransport` if
// that trait ever gains an associated `Body` type.
// 🚫️async: E6 dyn-compat — machine-readable form of the `dedyn-fw-os-misc` reasoning above, added
// so `asyncify-universal.py` stops re-breaking this trait (it does not yet recognise that tag).
pub trait HttpBody: Send {
    fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>>;
}

/// 🌐️ The seam a real HTTP client plugs into: `start` returns the head as soon as it is known plus
/// a [`HttpBody`] the caller streams at its own pace — no whole-body buffering happens below this
/// trait. [`BlockingHttpTransport`] is the ONLY implementation this packet ships (today's
/// synchronous-`HttpTransport`-on-`ComputePool` behaviour, unchanged); a sibling packet adds a real
/// async client behind this same trait, adding no new dependency to THIS crate — see the crate
/// report's `## honest gaps`.
// 🚫️async: E6 dyn-compat — same class as `HttpTransport` above. `start` already returns a
// `HostFuture` (R1-legal argument/return erasure per `dyn_enum_close!`'s sibling exceptions); an
// `async fn` wrapping THAT would be the literal double-future shape R1 bans, on top of breaking
// `Arc<dyn AsyncHttpTransport>`'s object safety (E0038). `asyncify-universal.py` doesn't yet
// recognise this class — see the `HttpTransport` tag above for the coordinator note.
/// 🌐️ [`AsyncHttpTransport::start`]'s result — factored into its own alias to keep that trait
/// method's signature under clippy's `type_complexity` threshold; not otherwise meaningful on its
/// own.
type StartedTransport = Result<(HttpResponseHead, Box<dyn HttpBody>), HttpPoolError>;

pub trait AsyncHttpTransport: Send + Sync {
    fn start(&self, ctx: &OperationContext, request: HttpRequest) -> HostFuture<StartedTransport>;
}

/// 🌐️ Blocking HTTP transport [`BlockingHttpTransport`] drives through [`ComputePool`] — the same
/// "one blocking call on a dedicated thread" technique `📇️directory/🔌️client` already uses for
/// `ureq`. NO implementation ships in this packet: [`HttpPool::new`] takes any `Arc<dyn
/// HttpTransport>`, and a real transport (a `ureq`-backed one, or a real connection-pooling client)
/// is later-packet wiring — see the crate report's `## honest gaps`. Kept as a trait rather than a
/// concrete client so this crate adds no new external HTTP dependency of its own.
// 🔀️ dedyn-fw-os-misc: DELIBERATELY left `dyn` — a reasoned exception, not an oversight. `call` is
// plain sync (no `async fn`), so `dyn HttpTransport` is not an E0038 violation and stays R1-legal.
// Closing it fully would need EITHER (a) `dyn_enum_close!`, impossible here: implementors span two
// crates in a one-directional dependency (`semio-framework-os-kernel`'s `📇️directory::
// UreqHttpTransport` implements a trait `semio-framework-os-services` defines, so `os-services`
// cannot name `UreqHttpTransport` without a reverse dependency) — OR (b) generics, which would force
// the widely-referenced, non-generic `HttpPool`/`HttpPoolTransport` (60 references across 5 files,
// including `📺️renderer/…/Shell/🎯️targets/🧊️wgpu/🦀️.rs` — flagged in `📌️important.md` as live/shared with
// concurrent tickets right now) to become generic too, for a trait that costs nothing left as `dyn`.
// Revisit if `HttpTransport` ever needs to become `async fn` (R2/universal-async) — an async trait
// method genuinely cannot stay `dyn`, unlike this sync one.
//
// 🚫️async: E6 dyn-compat (gate-services, new class — not in the R2 E1–E5 list; flagged to the
// coordinator to fold in) — `async fn` in a trait breaks object safety (E0038: "for a trait to be
// dyn compatible it needs to allow building a vtable"), and `Arc<dyn HttpTransport>` is load-bearing
// per the block comment above (implementors span two crates with no valid `dyn_enum_close!` or
// generics fix). `asyncify-universal.py` does not yet recognise this class of exemption — re-scan
// after adding it, or repair by hand again, the way this packet did.
pub trait HttpTransport: Send + Sync {
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error>;
}

/// 🚧️ The default [`HttpTransport`] until a later packet wires a real one — every call fails
/// loudly rather than silently succeeding with fake data.
pub struct UnwiredHttpTransport;
impl HttpTransport for UnwiredHttpTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag above.
    fn call(&self, _request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        Err(std::io::Error::other("HttpPool: no HttpTransport wired yet (see the packet report's honest gaps)"))
    }
}

/// 🐌️ One whole response, buffered by a [`HttpTransport::call`], replayed as a SINGLE chunk —
/// exactly what `HttpPool::request` did before this packet, now expressed as the one degenerate case
/// of [`HttpBody`] rather than a second code path. See [`BlockingHttpTransport`].
struct BufferedHttpBody {
    remaining: Option<Vec<u8>>,
}
impl HttpBody for BufferedHttpBody {
    // 🚫️async: E6 dyn-compat — see the `HttpBody` trait's `dedyn-fw-os-misc` tag above.
    fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let chunk = self.remaining.take();
        Box::pin(async move { Ok(chunk) })
    }
}

/// 🌐️ Wraps a legacy [`HttpTransport`] (today's ONLY shipped transport) as an [`AsyncHttpTransport`]
/// by running the whole blocking call through [`ComputePool::run_io`] and replaying the
/// buffered result as one [`BufferedHttpBody`] chunk. `runtime`/`scope` are captured at
/// CONSTRUCTION time (unlike `HttpPool::fetch`'s own `runtime`/`scope` parameters) because
/// [`AsyncHttpTransport::start`] itself takes neither — a transport that needs to reach
/// `ComputePool::run_io` must own that context itself.
// 🔀️ dedyn-fw-os-misc: `TokioHostRuntime`, not `<R: HostAsyncRuntime>` — this is the ONE production
// spawn site for HTTP transport work, and R3 requires Send-ness be obtained STRUCTURALLY (a known
// concrete future type at the spawn site), never by bounding a generic. `TokioHostRuntime` is this
// crate's sole real runtime; `ManualRuntime` (the other `HostAsyncRuntime` impl, testkit-only) never
// constructs a `BlockingHttpTransport` — nothing in this file's tests does. Generalising over `R`
// bought no actual caller and cost exactly the unprovable-Send error this comment replaces.
pub struct BlockingHttpTransport {
    transport: Arc<dyn HttpTransport>,
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
}

impl BlockingHttpTransport {
    pub async fn new(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> BlockingHttpTransport {
        BlockingHttpTransport { transport, compute, runtime, scope }
    }
}

impl AsyncHttpTransport for BlockingHttpTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn start(&self, ctx: &OperationContext, request: HttpRequest) -> HostFuture<StartedTransport> {
        let transport = self.transport.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = ctx.clone();
        Box::pin(async move {
            let result = compute.run_io(runtime.as_ref(), &scope, ctx, move || transport.call(request)).await;
            match result {
                Ok(Ok(response)) => {
                    let head = HttpResponseHead { status: response.status, headers: response.headers };
                    let body: Box<dyn HttpBody> = Box::new(BufferedHttpBody { remaining: Some(response.body) });
                    Ok((head, body))
                }
                Ok(Err(io_error)) => Err(HttpPoolError::Transport(io_error.to_string())),
                Err(compute_error) => Err(HttpPoolError::Compute(compute_error)),
            }
        })
    }
}

const SOCKET_HTTP_URL_BYTES: usize = 2_048;
const SOCKET_HTTP_HOST_BYTES: usize = 256;
const SOCKET_HTTP_HEADER_BYTES: usize = 16 * 1024;
const SOCKET_HTTP_HEADER_ITEMS: usize = 64;
const SOCKET_HTTP_BODY_PAGE_BYTES: usize = 16 * 1024;

#[derive(Clone, Copy)]
enum SocketHttpFraming {
    Length(u64),
    Chunked { remaining: usize },
    UntilEof,
    Terminal,
}

struct SocketHttpBodyState {
    reader: std::io::BufReader<std::net::TcpStream>,
    framing: SocketHttpFraming,
}

pub struct SocketHttpBody {
    state: Arc<Mutex<Option<SocketHttpBodyState>>>,
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
    ctx: OperationContext,
}

impl HttpBody for SocketHttpBody {
    fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let state = self.state.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = self.ctx.clone();
        Box::pin(async move {
            compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    let mut slot = state.lock().map_err(|_| HttpPoolError::Transport("socket HTTP body lock poisoned".into()))?;
                    let body = slot.as_mut().ok_or_else(|| HttpPoolError::Transport("socket HTTP body reached terminal ownership".into()))?;
                    socket_http_read_page(body)
                })
                .await
                .map_err(HttpPoolError::Compute)?
        })
    }
}

pub struct SocketHttpTransport {
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
}

impl SocketHttpTransport {
    pub fn new(compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> Self {
        Self { compute, runtime, scope }
    }
}

impl AsyncHttpTransport for SocketHttpTransport {
    fn start(&self, ctx: &OperationContext, request: HttpRequest) -> HostFuture<StartedTransport> {
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let connect_compute = compute.clone();
        let connect_runtime = runtime.clone();
        let connect_scope = scope.clone();
        let connect_ctx = ctx.clone();
        let body_ctx = ctx.clone();
        Box::pin(async move {
            let connected = connect_compute.run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || socket_http_connect(request)).await.map_err(HttpPoolError::Compute)??;
            let (head, state) = connected;
            let body: Box<dyn HttpBody> = Box::new(SocketHttpBody { state: Arc::new(Mutex::new(Some(state))), compute, runtime, scope, ctx: body_ctx });
            Ok((head, body))
        })
    }
}

fn socket_http_connect(request: HttpRequest) -> Result<(HttpResponseHead, SocketHttpBodyState), HttpPoolError> {
    use std::io::Write;
    if request.method != "GET" || !request.body.is_empty() {
        return Err(HttpPoolError::Transport("socket HTTP transport admits GET with an empty body only".into()));
    }
    if request.url.len() > SOCKET_HTTP_URL_BYTES || request.headers.len() > SOCKET_HTTP_HEADER_ITEMS {
        return Err(HttpPoolError::Transport("socket HTTP request exceeded fixed credits".into()));
    }
    let (host, port, path) = socket_http_url(&request.url)?;
    let header_bytes = request.headers.iter().try_fold(0usize, |total, (name, value)| {
        if name.bytes().any(|byte| byte <= b' ' || byte == b':') || value.bytes().any(|byte| byte == b'\r' || byte == b'\n') {
            return Err(HttpPoolError::Transport("socket HTTP request contained invalid header bytes".into()));
        }
        total
            .checked_add(name.len())
            .and_then(|total| total.checked_add(value.len()))
            .and_then(|total| total.checked_add(4))
            .filter(|total| *total <= SOCKET_HTTP_HEADER_BYTES)
            .ok_or_else(|| HttpPoolError::Transport("socket HTTP request headers exceeded fixed credits".into()))
    })?;
    let _ = header_bytes;
    let mut stream = std::net::TcpStream::connect((host.as_str(), port)).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    stream.set_nodelay(true).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    let mut head = Vec::with_capacity(request.url.len().min(SOCKET_HTTP_URL_BYTES));
    head.extend_from_slice(b"GET ");
    head.extend_from_slice(path.as_bytes());
    head.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    head.extend_from_slice(host.as_bytes());
    head.extend_from_slice(b"\r\nConnection: close\r\nAccept-Encoding: identity\r\n");
    for (name, value) in request.headers {
        head.extend_from_slice(name.as_bytes());
        head.extend_from_slice(b": ");
        head.extend_from_slice(value.as_bytes());
        head.extend_from_slice(b"\r\n");
    }
    head.extend_from_slice(b"\r\n");
    if head.len() > SOCKET_HTTP_HEADER_BYTES {
        return Err(HttpPoolError::Transport("socket HTTP serialized request exceeded fixed credits".into()));
    }
    stream.write_all(&head).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    let mut reader = std::io::BufReader::with_capacity(SOCKET_HTTP_BODY_PAGE_BYTES, stream);
    let status_line = socket_http_read_line(&mut reader, SOCKET_HTTP_HEADER_BYTES)?;
    let mut status_parts = status_line.split_whitespace();
    if !status_parts.next().is_some_and(|version| version == "HTTP/1.1" || version == "HTTP/1.0") {
        return Err(HttpPoolError::Transport("socket HTTP response used an unsupported protocol".into()));
    }
    let status = status_parts.next().and_then(|status| status.parse::<u16>().ok()).ok_or_else(|| HttpPoolError::Transport("socket HTTP response omitted a valid status".into()))?;
    let mut headers = Vec::with_capacity(SOCKET_HTTP_HEADER_ITEMS);
    let mut header_total = status_line.len().saturating_add(2);
    let mut content_length = None;
    let mut chunked = false;
    loop {
        let line = socket_http_read_line(&mut reader, SOCKET_HTTP_HEADER_BYTES.saturating_sub(header_total))?;
        header_total =
            header_total.checked_add(line.len()).and_then(|total| total.checked_add(2)).filter(|total| *total <= SOCKET_HTTP_HEADER_BYTES).ok_or_else(|| HttpPoolError::Transport("socket HTTP response headers exceeded fixed credits".into()))?;
        if line.is_empty() {
            break;
        }
        if headers.len() == SOCKET_HTTP_HEADER_ITEMS {
            return Err(HttpPoolError::Transport("socket HTTP response header count exceeded fixed credits".into()));
        }
        let (name, value) = line.split_once(':').ok_or_else(|| HttpPoolError::Transport("socket HTTP response header was malformed".into()))?;
        let value = value.trim().to_string();
        if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(value.parse::<u64>().map_err(|_| HttpPoolError::Transport("socket HTTP Content-Length was invalid".into()))?);
        }
        if name.eq_ignore_ascii_case("transfer-encoding") {
            chunked = value.split(',').any(|part| part.trim().eq_ignore_ascii_case("chunked"));
        }
        headers.push((name.to_string(), value));
    }
    let framing = if chunked {
        SocketHttpFraming::Chunked { remaining: 0 }
    } else if let Some(length) = content_length {
        SocketHttpFraming::Length(length)
    } else {
        SocketHttpFraming::UntilEof
    };
    Ok((HttpResponseHead { status, headers }, SocketHttpBodyState { reader, framing }))
}

fn socket_http_url(url: &str) -> Result<(String, u16, String), HttpPoolError> {
    let rest = url.strip_prefix("http://").ok_or_else(|| HttpPoolError::Transport("socket HTTP transport requires an http:// URL; HTTPS has no owned TLS stream yet".into()))?;
    let (authority, path) = rest.split_once('/').map_or((rest, "/".to_string()), |(authority, path)| (authority, format!("/{path}")));
    if authority.is_empty() || authority.len() > SOCKET_HTTP_HOST_BYTES || path.len() > SOCKET_HTTP_URL_BYTES {
        return Err(HttpPoolError::Transport("socket HTTP authority/path exceeded fixed credits".into()));
    }
    let (host, port) =
        authority.rsplit_once(':').map_or_else(|| Ok((authority.to_string(), 80)), |(host, port)| port.parse::<u16>().map(|port| (host.to_string(), port)).map_err(|_| HttpPoolError::Transport("socket HTTP port was invalid".into())))?;
    if host.is_empty() {
        return Err(HttpPoolError::Transport("socket HTTP host was empty".into()));
    }
    Ok((host, port, path))
}

fn socket_http_read_line(reader: &mut impl std::io::Read, remaining: usize) -> Result<String, HttpPoolError> {
    if remaining == 0 {
        return Err(HttpPoolError::Transport("socket HTTP line exceeded fixed credits".into()));
    }
    let mut bytes = Vec::with_capacity(remaining.min(256));
    for _ in 0..remaining {
        let mut byte = [0];
        reader.read_exact(&mut byte).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
        bytes.push(byte[0]);
        if byte[0] == b'\n' {
            break;
        }
    }
    if !bytes.ends_with(b"\n") {
        return Err(HttpPoolError::Transport("socket HTTP response line exceeded fixed credits".into()));
    }
    if bytes.ends_with(b"\n") {
        bytes.pop();
    }
    if bytes.ends_with(b"\r") {
        bytes.pop();
    }
    String::from_utf8(bytes).map_err(|_| HttpPoolError::Transport("socket HTTP response header was not UTF-8".into()))
}

fn socket_http_read_page(body: &mut SocketHttpBodyState) -> Result<Option<Vec<u8>>, HttpPoolError> {
    use std::io::Read;
    match body.framing {
        SocketHttpFraming::Length(0) | SocketHttpFraming::Terminal => {
            body.framing = SocketHttpFraming::Terminal;
            Ok(None)
        }
        SocketHttpFraming::Length(remaining) => {
            let mut bytes = vec![0; usize::try_from(remaining).unwrap_or(usize::MAX).min(SOCKET_HTTP_BODY_PAGE_BYTES)];
            let count = body.reader.read(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            if count == 0 {
                return Err(HttpPoolError::Transport("socket HTTP body ended before Content-Length".into()));
            }
            bytes.truncate(count);
            body.framing = SocketHttpFraming::Length(remaining - count as u64);
            Ok(Some(bytes))
        }
        SocketHttpFraming::UntilEof => {
            let mut bytes = vec![0; SOCKET_HTTP_BODY_PAGE_BYTES];
            let count = body.reader.read(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            if count == 0 {
                body.framing = SocketHttpFraming::Terminal;
                return Ok(None);
            }
            bytes.truncate(count);
            Ok(Some(bytes))
        }
        SocketHttpFraming::Chunked { mut remaining } => {
            if remaining == 0 {
                let line = socket_http_read_line(&mut body.reader, 128)?;
                let length = line.split(';').next().and_then(|value| usize::from_str_radix(value.trim(), 16).ok()).ok_or_else(|| HttpPoolError::Transport("socket HTTP chunk length was invalid".into()))?;
                if length == 0 {
                    let mut trailer_bytes = 0usize;
                    for _ in 0..SOCKET_HTTP_HEADER_ITEMS {
                        let line = socket_http_read_line(&mut body.reader, SOCKET_HTTP_HEADER_BYTES.saturating_sub(trailer_bytes))?;
                        trailer_bytes = trailer_bytes
                            .checked_add(line.len())
                            .and_then(|bytes| bytes.checked_add(2))
                            .filter(|bytes| *bytes <= SOCKET_HTTP_HEADER_BYTES)
                            .ok_or_else(|| HttpPoolError::Transport("socket HTTP trailers exceeded fixed credits".into()))?;
                        if line.is_empty() {
                            body.framing = SocketHttpFraming::Terminal;
                            return Ok(None);
                        }
                    }
                    return Err(HttpPoolError::Transport("socket HTTP trailer count exceeded fixed credits".into()));
                }
                remaining = length;
            }
            let mut bytes = vec![0; remaining.min(SOCKET_HTTP_BODY_PAGE_BYTES)];
            body.reader.read_exact(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            remaining -= bytes.len();
            if remaining == 0 {
                let mut suffix = [0; 2];
                body.reader.read_exact(&mut suffix).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
                if suffix != *b"\r\n" {
                    return Err(HttpPoolError::Transport("socket HTTP chunk omitted its terminator".into()));
                }
            }
            body.framing = SocketHttpFraming::Chunked { remaining };
            Ok(Some(bytes))
        }
    }
}

struct TokenBucket {
    remaining_bytes: u64,
    capacity_bytes: u64,
    refill_epoch: u64,
}

impl TokenBucket {
    fn new_at(capacity_bytes: u64, refill_epoch: u64) -> TokenBucket {
        TokenBucket { remaining_bytes: capacity_bytes, capacity_bytes, refill_epoch }
    }

    fn observe_refill_epoch(&mut self, refill_epoch: u64) {
        if self.refill_epoch < refill_epoch {
            self.remaining_bytes = self.capacity_bytes;
            self.refill_epoch = refill_epoch;
        }
    }

    // 🚫️async: E1-adjacent — same reasoning as `new` above (R9).
    fn try_consume(&mut self, bytes: u64) -> bool {
        if bytes > self.remaining_bytes {
            false
        } else {
            self.remaining_bytes -= bytes;
            true
        }
    }

    /// ♻️ Refills back toward capacity. Driven for real, once a minute, by
    /// [`HttpPool::spawn_refill_driver`] — see that method's doc; this method itself stays the pure
    /// arithmetic step so tests can also exercise refill deterministically without a driver.
    // 🚫️async: E1-adjacent — same reasoning as `new` above (R9).
    fn refill(&mut self, bytes: u64) {
        self.remaining_bytes = (self.remaining_bytes + bytes).min(self.capacity_bytes);
    }
}

/// 🐌️ How often [`HttpPool::spawn_refill_driver`] tops every tracked package's bucket back toward
/// its `network_bytes_per_min` cap — the PRODUCTION `interval_ms` argument that fn's real caller (a
/// process bootstrap, out of this packet's boundary — see the packet report's `## honest gaps`)
/// passes; this crate's own tests pass a short interval instead, so nothing in THIS crate reads this
/// const today.
#[allow(dead_code)]
const HTTP_BUCKET_REFILL_INTERVAL_MS: u64 = 60_000;

/// 🔓️ Releases one outstanding-request slot for `actor`, shared between [`HttpPool::fetch`]'s
/// own early-return paths (no [`HttpPoolBody`] was ever created to own the release) and
/// [`HttpPoolBody::finish`] (the body's own EOF/drop path) — ONE decrement implementation either way.
// 🚫️async: E1 pure in-memory decrement whose consumer chain reaches `Drop::drop` (external trait,
// cannot be async) via `HttpPoolBody::finish` — see R9.
fn release_outstanding_slot(outstanding: &Mutex<HashMap<ActorId, u32>>, actor: ActorId) {
    let mut outstanding = outstanding.lock().expect("HttpPool outstanding mutex poisoned");
    if let Some(count) = outstanding.get_mut(&actor) {
        *count = count.saturating_sub(1);
    }
}

enum HttpPoolTransport {
    Blocking { transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool> },
    Async(Arc<dyn AsyncHttpTransport>),
}

/// 🌐️ Shared connection-pool boundary: a per-package `network_bytes_per_min` token bucket and a
/// per-actor `outstanding_requests` cap, gating an [`AsyncHttpTransport`]. [`HttpPool::fetch`]
/// charges the bucket per REAL response chunk (via [`HttpPoolBody`]) rather than a pre-request
/// estimate, and releases the outstanding slot on EOF or on the caller dropping the body early —
/// see the crate report's `## one-implementation argument`.
pub struct HttpPool {
    transport: HttpPoolTransport,
    buckets: Arc<Mutex<HashMap<PackageId, TokenBucket>>>,
    bytes_per_minute_cap: u64,
    refill_epoch: Arc<AtomicU64>,
    refill_driver_started: Arc<AtomicBool>,
    outstanding: Arc<Mutex<HashMap<ActorId, u32>>>,
    outstanding_cap: u32,
}

impl HttpPool {
    /// 🌐️ Today's only shipped shape: stores `transport`/`compute` directly, dispatched inline by
    /// [`HttpPool::fetch`] with the runtime/scope IT receives per call — the same dispatch
    /// [`BlockingHttpTransport::start`] performs, just not routed through that type here, because
    /// `fetch`/`request` keep their runtime/scope as borrowed PER-CALL parameters (so existing
    /// callers built against today's `HttpPool::new`/`request` keep compiling unchanged) while
    /// [`AsyncHttpTransport::start`] needs a transport that OWNS them — see the crate report's
    /// `## honest gaps` for this one acknowledged duplication.
    pub async fn new(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        Self::new_now(transport, compute, bytes_per_minute_cap, outstanding_cap)
    }

    pub fn new_now(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        HttpPool {
            transport: HttpPoolTransport::Blocking { transport, compute },
            buckets: Arc::new(Mutex::new(HashMap::new())),
            bytes_per_minute_cap,
            refill_epoch: Arc::new(AtomicU64::new(0)),
            refill_driver_started: Arc::new(AtomicBool::new(false)),
            outstanding: Arc::new(Mutex::new(HashMap::new())),
            outstanding_cap: outstanding_cap.max(1),
        }
    }

    /// 🌐️ For a real [`AsyncHttpTransport`] (a sibling packet's real async client) that needs no
    /// [`ComputePool`] of its own — the transport already does real async I/O.
    pub async fn new_with_async_transport(transport: Arc<dyn AsyncHttpTransport>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        Self::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap)
    }

    pub fn new_with_async_transport_now(transport: Arc<dyn AsyncHttpTransport>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        HttpPool {
            transport: HttpPoolTransport::Async(transport),
            buckets: Arc::new(Mutex::new(HashMap::new())),
            bytes_per_minute_cap,
            refill_epoch: Arc::new(AtomicU64::new(0)),
            refill_driver_started: Arc::new(AtomicBool::new(false)),
            outstanding: Arc::new(Mutex::new(HashMap::new())),
            outstanding_cap: outstanding_cap.max(1),
        }
    }

    /// ♻️ Test/operator hook for a manual top-up outside the once-a-minute driver — see
    /// [`TokenBucket::refill`]'s doc.
    pub async fn refill_package_budget(&self, package: &PackageId, bytes: u64) {
        let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
        let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
        let bucket = buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
        bucket.observe_refill_epoch(refill_epoch);
        bucket.refill(bytes);
    }

    /// 🔍️ `package`'s remaining bytes this minute — untracked packages read as a full bucket
    /// (nothing has been charged against them yet).
    pub async fn remaining_package_budget(&self, package: &PackageId) -> u64 {
        let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
        let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
        let Some(bucket) = buckets.get_mut(package) else { return self.bytes_per_minute_cap };
        bucket.observe_refill_epoch(refill_epoch);
        bucket.remaining_bytes
    }

    /// ▶️ Starts one finite maintenance turn per refill interval. A turn advances a shared epoch
    /// in O(1); each package observes that epoch and refills lazily on its next access, so no turn
    /// scans an unbounded package map or occupies a worker while waiting.
    // 🚫️async: E1-adjacent — no suspension point of its own (only SUBMITS the job). See R9.
    pub fn spawn_refill_driver(&self, pool: &WorkerPool, interval_ms: u64) {
        if self.refill_driver_started.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            schedule_http_refill_turn(pool, self.refill_epoch.clone(), interval_ms.max(1));
        }
    }

    /// 🌊️ Starts `request` and returns as soon as the head is known, plus a [`HttpPoolBody`] the
    /// caller streams at its own pace. Charges the per-actor outstanding cap up front (released on
    /// the body's EOF or drop — see [`HttpPoolBody`]) and the per-package byte bucket for the
    /// EXACT, known outbound bytes (`request.url`/`request.body`, not header framing — see the
    /// crate report's `## honest gaps`) up front; RESPONSE bytes are charged separately, for real,
    /// per chunk, as [`HttpPoolBody::next_chunk`] pulls them — this is the fix for the estimate-only
    /// accounting this packet was measured against.
    pub async fn fetch<R: HostAsyncRuntime>(&self, runtime: &R, scope: &ScopeHandle, ctx: OperationContext, package: PackageId, actor: ActorId, request: HttpRequest) -> Result<(HttpResponseHead, HttpPoolBody), HttpPoolError> {
        {
            let mut outstanding = self.outstanding.lock().expect("HttpPool outstanding mutex poisoned");
            let count = outstanding.entry(actor).or_insert(0);
            if *count >= self.outstanding_cap {
                return Err(HttpPoolError::OutstandingCapReached { actor, limit: self.outstanding_cap });
            }
            *count += 1;
        }
        let outbound_bytes = (request.body.len() + request.url.len()) as u64;
        let admitted = {
            let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
            let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
            let bucket = buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
            bucket.observe_refill_epoch(refill_epoch);
            bucket.try_consume(outbound_bytes)
        };
        if !admitted {
            release_outstanding_slot(&self.outstanding, actor);
            return Err(HttpPoolError::ByteBudgetExhausted { package });
        }
        let start_result = match &self.transport {
            HttpPoolTransport::Blocking { transport, compute } => {
                let transport = transport.clone();
                let compute = compute.clone();
                let ctx_for_run = ctx.clone();
                let result = compute.run_io(runtime, scope, ctx_for_run, move || transport.call(request)).await;
                match result {
                    Ok(Ok(response)) => {
                        let head = HttpResponseHead { status: response.status, headers: response.headers };
                        let body: Box<dyn HttpBody> = Box::new(BufferedHttpBody { remaining: Some(response.body) });
                        Ok((head, body))
                    }
                    Ok(Err(io_error)) => Err(HttpPoolError::Transport(io_error.to_string())),
                    Err(compute_error) => Err(HttpPoolError::Compute(compute_error)),
                }
            }
            HttpPoolTransport::Async(async_transport) => async_transport.start(&ctx, request).await,
        };
        match start_result {
            Ok((head, body)) => {
                Ok((head, HttpPoolBody { inner: body, package, actor, buckets: self.buckets.clone(), bytes_per_minute_cap: self.bytes_per_minute_cap, refill_epoch: self.refill_epoch.clone(), outstanding: self.outstanding.clone(), finished: false }))
            }
            Err(error) => {
                release_outstanding_slot(&self.outstanding, actor);
                Err(error)
            }
        }
    }

    /// 🌐️ The pre-streaming buffered shape: built ENTIRELY on [`HttpPool::fetch`] — collects every
    /// chunk into one `Vec<u8>` — so there is exactly ONE request/response code path in this crate;
    /// see the crate report's `## one-implementation argument`.
    pub async fn request<R: HostAsyncRuntime>(&self, runtime: &R, scope: &ScopeHandle, ctx: OperationContext, package: PackageId, actor: ActorId, request: HttpRequest) -> Result<HttpResponse, HttpPoolError> {
        let (head, mut body) = self.fetch(runtime, scope, ctx, package, actor, request).await?;
        let mut collected = Vec::new();
        while let Some(chunk) = body.next_chunk().await? {
            collected.extend_from_slice(&chunk);
        }
        Ok(HttpResponse { status: head.status, headers: head.headers, body: collected })
    }
}

fn schedule_http_refill_turn(pool: &WorkerPool, refill_epoch: Arc<AtomicU64>, interval_ms: u64) {
    let deadline_ms = pool.now_ms().saturating_add(interval_ms);
    let turn_pool = pool.clone();
    pool.submit_at(
        deadline_ms,
        Lane::Maintenance,
        Box::new(move || {
            refill_epoch.fetch_add(1, Ordering::SeqCst);
            schedule_http_refill_turn(&turn_pool, refill_epoch, interval_ms);
        }),
    );
}

/// 🌊️ A [`HttpPool::fetch`]'d body: wraps the transport's own [`HttpBody`], charging the
/// per-package byte bucket for the REAL length of every chunk actually pulled (never an estimate),
/// and releasing the actor's outstanding slot exactly once — on EOF, on a mid-body budget abort, or
/// on the caller dropping this value early (`Drop` calls the SAME [`HttpPoolBody::finish`] the
/// success paths do, guarded by `finished` so a drop after EOF never double-releases). Dropping this
/// value also drops `inner`, so whatever connection the transport's [`HttpBody`] owns closes with
/// it — an aborted or cancelled stream is not left dangling.
pub struct HttpPoolBody {
    inner: Box<dyn HttpBody>,
    package: PackageId,
    actor: ActorId,
    buckets: Arc<Mutex<HashMap<PackageId, TokenBucket>>>,
    bytes_per_minute_cap: u64,
    refill_epoch: Arc<AtomicU64>,
    outstanding: Arc<Mutex<HashMap<ActorId, u32>>>,
    finished: bool,
}

impl HttpPoolBody {
    // 🚫️async: E1 pure bookkeeping consumed by `Drop::drop` below (external trait, cannot be
    // async) — see R9.
    fn finish(&mut self) {
        if !self.finished {
            self.finished = true;
            release_outstanding_slot(&self.outstanding, self.actor);
        }
    }

    /// 🌊️ Pulls the next real chunk, charging the per-package bucket for its EXACT length before
    /// handing it back. Once the bucket cannot afford a chunk that has ALREADY arrived, this returns
    /// [`HttpPoolError::ByteBudgetExhausted`] and releases the outstanding slot — the caller is
    /// expected to drop this value on error, which closes the underlying connection (see this
    /// type's own doc).
    pub async fn next_chunk(&mut self) -> Result<Option<Vec<u8>>, HttpPoolError> {
        if self.finished {
            return Ok(None);
        }
        match self.inner.next_chunk().await {
            Ok(Some(chunk)) => {
                let admitted = {
                    let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
                    let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
                    let bucket = buckets.entry(self.package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
                    bucket.observe_refill_epoch(refill_epoch);
                    bucket.try_consume(chunk.len() as u64)
                };
                if !admitted {
                    self.finish();
                    return Err(HttpPoolError::ByteBudgetExhausted { package: self.package.clone() });
                }
                Ok(Some(chunk))
            }
            Ok(None) => {
                self.finish();
                Ok(None)
            }
            Err(error) => {
                self.finish();
                Err(error)
            }
        }
    }
}

impl Drop for HttpPoolBody {
    fn drop(&mut self) {
        self.finish();
    }
}
//#endregion 🌐️HttpPool

//#region 💾️Storage
//#region 📄️FixedFilePage
/// 📄️ Maximum logical payload admitted by one host-storage worker turn.
pub const STORAGE_FIXED_FILE_PAGE_BYTES: usize = 16 * 1024;

/// 📖️ Reads exactly one bounded file page from a caller-owned path. This synchronous
/// primitive is deliberately owned by the host-storage service and must be invoked only by an
/// already-admitted [`Lane::Io`] worker job; presentation code may retain paths and results but
/// never performs the platform operation itself.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_worker_read_fixed_file_page(path: &std::path::Path, maximum_bytes: usize) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    if maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page limit exceeds host-storage authority"));
    }
    let mut file = std::fs::File::open(path)?;
    let logical_bytes = usize::try_from(file.metadata()?.len()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page length is not representable"))?;
    if logical_bytes > maximum_bytes {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page exceeds admitted bytes"));
    }
    let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES];
    file.read_exact(&mut page[..logical_bytes])?;
    let mut trailing = [0u8; 1];
    if file.read(&mut trailing)? != 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page grew beyond admitted bytes"));
    }
    Ok(page[..logical_bytes].to_vec())
}

/// 📝️ Writes exactly one bounded file page from an already-admitted [`Lane::Io`] worker
/// job. Byte admission precedes directory creation, file opening, truncation, and publication.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_worker_write_fixed_file_page(path: &std::path::Path, bytes: &[u8], maximum_bytes: usize) -> std::io::Result<()> {
    use std::io::Write;
    if maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES || bytes.len() > maximum_bytes {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page exceeds admitted bytes"));
    }
    let parent = path.parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page has no parent"))?;
    std::fs::create_dir_all(parent)?;
    let mut file = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(path)?;
    file.write_all(bytes)
}
//#endregion 📄️FixedFilePage

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
    plugin: PackageId,
    bytes: u64,
    ctx: OperationContext,
    work: Box<dyn FnOnce() -> Result<Vec<u8>, std::io::Error> + Send>,
    result_tx: oneshot::Sender<Result<Vec<u8>, StorageError>>,
    /// ⏰️ Set by [`StorageTicket::await_result`] if the deadline race is lost while this job is
    /// still queued — [`storage_try_dispatch`] checks it right after popping, same lazy-skip
    /// discipline [`WheelCore::disarm`] already uses for timers.
    cancelled: Arc<AtomicBool>,
}

struct StorageState<R: HostAsyncRuntime> {
    runtime: Arc<R>,
    scope: ScopeHandle,
    pool: WorkerPool,
    queues: Mutex<BTreeMap<u8, VecDeque<StorageJob>>>,
    in_flight: AtomicU32,
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
/// caller this packet cannot edit: `🔌️plugin/🖥️host/⏳️imports/🦀️.rs`'s `storage_read`/`storage_write`
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
pub struct StorageScheduler<R: HostAsyncRuntime>(Arc<StorageState<R>>);

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
    // 🚫️async: E1-adjacent — forced synchronous by `🔌️plugin/🖥️host/⏳️imports/🦀️.rs`'s
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
    receiver: oneshot::Receiver<Result<Vec<u8>, StorageError>>,
    cancelled: Arc<AtomicBool>,
    runtime: Arc<R>,
    deadline_ms: Option<u64>,
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

//#region 📮️EventRouter
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Topic(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishOutcome {
    Delivered,
    Collapsed,
    RejectedFull { cap: u32 },
    RejectedInsufficientCredit,
    NoSuchSubscriber,
}

enum Mailbox {
    LatestWins { max_bytes: u64, pending: Option<Vec<u8>> },
    Coalesced { cap: u32, max_bytes: u64, used_bytes: u64, pending: HashMap<String, Vec<u8>>, order: VecDeque<String> },
    Ring { cap: u32, max_bytes: u64, used_bytes: u64, pending: VecDeque<Vec<u8>> },
    LosslessBounded { cap: u32, max_bytes: u64, used_bytes: u64, pending: VecDeque<Vec<u8>> },
    ByteCredit { remaining: u64 },
}

impl Mailbox {
    // 🚫️async: E1-adjacent — pure, in-memory state-machine construction, consumed (below) through
    // sync `Iterator::map`/`map_or` closures and a `std::sync::Mutex` guard held across the whole
    // call in `EventRouter::publish`/`drain`. See R9 and the same reasoning on `TokenBucket`/
    // `WheelCore` above.
    fn new(policy: &ChannelPolicy) -> Mailbox {
        match policy {
            ChannelPolicy::LatestWins { max_bytes } => Mailbox::LatestWins { max_bytes: *max_bytes, pending: None },
            ChannelPolicy::Coalesced { key: _, max_items, max_bytes } => Mailbox::Coalesced { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: HashMap::new(), order: VecDeque::new() },
            ChannelPolicy::Ring { max_items, max_bytes } => Mailbox::Ring { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: VecDeque::new() },
            ChannelPolicy::LosslessBounded { max_items, max_bytes } => Mailbox::LosslessBounded { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: VecDeque::new() },
            ChannelPolicy::ByteCredit { max_items: _, max_bytes } => Mailbox::ByteCredit { remaining: *max_bytes },
        }
    }

    /// 📮️ `coalesce_key` is only meaningful for a [`ChannelPolicy::Coalesced`] mailbox — an
    /// incoming message under a key already pending REPLACES it (collapse); a new key queues
    /// alongside the others.
    // 🚫️async: E1-adjacent — see `Mailbox::new`'s tag above (R9).
    fn publish(&mut self, coalesce_key: Option<&str>, payload: Vec<u8>) -> PublishOutcome {
        match self {
            Mailbox::LatestWins { max_bytes, pending } => {
                if payload.len() as u64 > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let collapsed = pending.is_some();
                *pending = Some(payload);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::Coalesced { cap, max_bytes, used_bytes, pending, order } => {
                let key = coalesce_key.unwrap_or_default().to_string();
                let cost = payload.len() as u64;
                if *cap == 0 {
                    return PublishOutcome::RejectedFull { cap: *cap };
                }
                if cost > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let mut collapsed = false;
                if let Some(previous) = pending.remove(&key) {
                    *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                    order.retain(|queued| queued != &key);
                    collapsed = true;
                }
                while pending.len() as u32 >= *cap || used_bytes.saturating_add(cost) > *max_bytes {
                    let Some(oldest) = order.pop_front() else { break };
                    if let Some(previous) = pending.remove(&oldest) {
                        *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                        collapsed = true;
                    }
                }
                pending.insert(key.clone(), payload);
                order.push_back(key);
                *used_bytes = used_bytes.saturating_add(cost);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::Ring { cap, max_bytes, used_bytes, pending } => {
                let cost = payload.len() as u64;
                if *cap == 0 {
                    return PublishOutcome::RejectedFull { cap: *cap };
                }
                if cost > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let mut collapsed = false;
                while pending.len() as u32 >= *cap || used_bytes.saturating_add(cost) > *max_bytes {
                    let Some(previous) = pending.pop_front() else { break };
                    *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                    collapsed = true;
                }
                pending.push_back(payload);
                *used_bytes = used_bytes.saturating_add(cost);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::LosslessBounded { cap, max_bytes, used_bytes, pending } => {
                if pending.len() as u32 >= *cap {
                    PublishOutcome::RejectedFull { cap: *cap }
                } else if used_bytes.saturating_add(payload.len() as u64) > *max_bytes {
                    PublishOutcome::RejectedInsufficientCredit
                } else {
                    *used_bytes = used_bytes.saturating_add(payload.len() as u64);
                    pending.push_back(payload);
                    PublishOutcome::Delivered
                }
            }
            Mailbox::ByteCredit { remaining } => {
                let cost = payload.len() as u64;
                if cost > *remaining {
                    PublishOutcome::RejectedInsufficientCredit
                } else {
                    *remaining -= cost;
                    PublishOutcome::Delivered
                }
            }
        }
    }

    /// 💳️ `ByteCredit` tracks a spendable budget only, no queued payload, in this packet — a real
    /// byte-metered channel's actual delivery mechanism is later-packet wiring, so it drains empty.
    // 🚫️async: E1-adjacent — see `Mailbox::new`'s tag above (R9).
    fn drain(&mut self) -> Vec<Vec<u8>> {
        match self {
            Mailbox::LatestWins { pending, .. } => pending.take().into_iter().collect(),
            Mailbox::Coalesced { used_bytes, pending, order, .. } => {
                let mut out = Vec::new();
                while let Some(key) = order.pop_front() {
                    if let Some(value) = pending.remove(&key) {
                        out.push(value);
                    }
                }
                *used_bytes = 0;
                out
            }
            Mailbox::Ring { used_bytes, pending, .. } | Mailbox::LosslessBounded { used_bytes, pending, .. } => {
                *used_bytes = 0;
                pending.drain(..).collect()
            }
            Mailbox::ByteCredit { .. } => Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Subscriber {
    actor: ActorId,
    policy: ChannelPolicy,
}

/// 📮️ Indexed topic router: [`EventRouter::subscribe`]/`unsubscribe` register a
/// `(ActorId, ChannelPolicy)` per topic; [`EventRouter::publish`]/`send_message` deliver into
/// a per-`(topic, actor)` [`Mailbox`] built from the subscriber's OWN declared policy —
/// `LatestWins`/`Coalesced`/`Ring` collapse, `LosslessBounded` rejects rather than growing past `cap`,
/// exactly the backpressure vocabulary `semio_framework_async::ChannelPolicy` declares. Draining a
/// mailbox into a real [`CompletionSink::complete`] call with a real actor generation is a later
/// packet's job — see the crate report's `## honest gaps`; this region's own contract (the
/// routing/backpressure DECISION) is tested directly via [`EventRouter::drain`].
pub struct EventRouter {
    subscribers: Mutex<HashMap<Topic, Vec<Subscriber>>>,
    mailboxes: Mutex<HashMap<(Topic, ActorId), Mailbox>>,
}

impl Default for EventRouter {
    fn default() -> EventRouter {
        EventRouter::new()
    }
}

impl EventRouter {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait, cannot be async)
    // — see R9. No suspension point exists (`Mutex::new`/`HashMap::new` are in-memory only).
    pub fn new() -> EventRouter {
        EventRouter { subscribers: Mutex::new(HashMap::new()), mailboxes: Mutex::new(HashMap::new()) }
    }

    pub async fn subscribe(&self, topic: Topic, actor: ActorId, policy: ChannelPolicy) {
        let mailbox = Mailbox::new(&policy);
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").insert((topic.clone(), actor), mailbox);
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").entry(topic).or_default().push(Subscriber { actor, policy });
    }

    /// 🔍️ The `ChannelPolicy` `actor` declared for `topic` at [`EventRouter::subscribe`] time, if
    /// still subscribed — lets a caller (e.g. a diagnostics surface) inspect backpressure
    /// vocabulary without reaching into a `Mailbox`, which is private.
    pub async fn declared_policy(&self, topic: &Topic, actor: ActorId) -> Option<ChannelPolicy> {
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get(topic)?.iter().find(|subscriber| subscriber.actor == actor).map(|subscriber| subscriber.policy.clone())
    }

    pub async fn unsubscribe(&self, topic: &Topic, actor: ActorId) {
        if let Some(subscribers) = self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get_mut(topic) {
            subscribers.retain(|subscriber| subscriber.actor != actor);
        }
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").remove(&(topic.clone(), actor));
    }

    /// 📮️ Delivers `payload` to every subscriber of `topic`, honouring each one's OWN policy
    /// independently — one bounded subscriber rejecting never affects another's delivery.
    pub async fn publish(&self, topic: &Topic, coalesce_key: Option<&str>, payload: &[u8]) -> Vec<(ActorId, PublishOutcome)> {
        let subscribers = self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get(topic).cloned().unwrap_or_default();
        let mut mailboxes = self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned");
        subscribers
            .into_iter()
            .map(|subscriber| {
                let outcome = mailboxes.get_mut(&(topic.clone(), subscriber.actor)).map_or(PublishOutcome::NoSuchSubscriber, |mailbox| mailbox.publish(coalesce_key, payload.to_vec()));
                (subscriber.actor, outcome)
            })
            .collect()
    }

    /// ✉️ Direct actor-to-actor send, bypassing topic subscription — `actor` must already have a
    /// mailbox for `topic` from a prior [`EventRouter::subscribe`] call.
    pub async fn send_message(&self, topic: &Topic, actor: ActorId, payload: Vec<u8>) -> PublishOutcome {
        match self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)) {
            Some(mailbox) => mailbox.publish(None, payload),
            None => PublishOutcome::NoSuchSubscriber,
        }
    }

    pub async fn drain(&self, topic: &Topic, actor: ActorId) -> Vec<Vec<u8>> {
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)).map(|mailbox| mailbox.drain()).unwrap_or_default()
    }
}
//#endregion 📮️EventRouter

//#region 🧾️CompletionSink
/// 🧾 The ONLY way any service in this crate re-enters the kernel: hand a completed operation's
/// result back as raw bytes tagged with the actor/generation/lane it belongs to. No type in this
/// crate holds or calls a `Kernel` directly — every completion flows out through this trait instead,
/// the same seam discipline `HostAsyncRuntime` itself uses to hide tokio.
// 🚫️async: E6 dyn-compat — `Arc<dyn CompletionSink>` is the load-bearing shape (`spawn_driver`'s
// `sink` parameter, an OPEN extension point implemented outside this crate). `complete` does no
// awaiting of its own (raw-bytes handoff, no I/O in this crate), so unlike `AsyncHttpTransport` it
// needs no `HostFuture` erasure either — plain sync is both correct and dyn-compatible. See the
// `HttpTransport` tag above for the coordinator note on this exemption class.
pub trait CompletionSink: Send + Sync {
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionRecord {
    pub actor: u64,
    pub generation: u16,
    pub event_bytes: Vec<u8>,
    pub lane: u8,
}

/// 🧪️ Test double recording every [`CompletionSink::complete`] call in order, for asserting a
/// service actually reached the ONLY re-entry point rather than swallowing its result.
#[derive(Clone, Default)]
pub struct MockCompletionSink {
    completions: Arc<Mutex<Vec<CompletionRecord>>>,
}

impl MockCompletionSink {
    pub async fn new() -> MockCompletionSink {
        MockCompletionSink::default()
    }

    pub async fn recorded(&self) -> Vec<CompletionRecord> {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").clone()
    }
}

impl CompletionSink for MockCompletionSink {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8) {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").push(CompletionRecord { actor, generation, event_bytes, lane });
    }
}
//#endregion 🧾️CompletionSink

//#region 🧬️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧬️Tests
