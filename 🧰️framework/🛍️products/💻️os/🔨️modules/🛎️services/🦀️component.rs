//! 🛎️ Host-side async SERVICES built ON TOP of `semio-framework-async`'s `HostAsyncRuntime` — the
//! ONE crate in this tree allowed to name `tokio`. Every other framework/os/plugin crate reaches
//! this functionality only through `semio-framework-async` vocabulary (`OperationContext`,
//! `ScopeHandle`, `ChannelPolicy`, `ThreadPlan`) or this crate's own domain types — never a tokio
//! type — exactly as `wasmtime` is confined behind `GuestRuntime` in the plugin-host crate. Verify
//! this holds with `grep -nE 'tokio' 🦀️component.rs | grep -v '^\s*[0-9]*://'` against every `pub
//! fn`/`pub struct` signature (see the packet report's `## tokio-containment evidence`).
//!
//! 🚂️ [`TokioHostRuntime`] is the one `HostAsyncRuntime` implementation. Every other public type
//! here ([`TimerWheel`], [`ComputePool`], [`HttpPool`], [`StorageScheduler`], [`EventRouter`]) is a
//! SERVICE built on top of that trait, reaching around it into raw tokio only where explicitly
//! noted (the timer driver task, the storage dispatcher) — never around `semio-framework-async`.
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
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{CancelState, CancelToken, ChannelPolicy, HostAsyncRuntime, HostFuture, OperationContext, ScopeDrainReport, ScopeHandle, ScopeId, ScopeOwner, ThreadBudget, ThreadPlan, ThreadRole};

//#region 🚂️TokioHostRuntime
/// 🌳️ What a scope's spawned task decided about itself before running its real body — used only to
/// classify [`ScopeDrainReport`] buckets truthfully. Never exposed outside this crate.
enum TaskOutcome {
    Finished,
    CancelledEarly,
}

/// 🌳️ One scope's identity/lineage/cancellation plus the `JoinSet` tracking every task
/// [`ScopeTable::spawn_scoped`]/`run_blocking` put into it.
struct ScopeRecord {
    cancel: CancelToken,
    tasks: tokio::task::JoinSet<TaskOutcome>,
}

/// 🐢️ Poll interval a newly-submitted unit of work waits on while its scope is `Park`ed before
/// checking again — see [`await_live_or_cancelled`]'s doc for why this is a poll rather than an
/// event wake.
const PARK_POLL_INTERVAL_MS: u64 = 20;

/// ⏳️ Waits until `cancel`'s effective state is no longer `Park`, returning `true` if it settled on
/// `Live` (the caller should run its real work) or `false` if it settled on `Cancelled` (the caller
/// should skip its real work entirely — this is how "new work is held while parked, and refused once
/// cancelled" is implemented). Polls on [`PARK_POLL_INTERVAL_MS`] rather than waking on an event
/// because `CancelToken` (frozen for this packet, from `semio-framework-async`) exposes only a
/// point-in-time `state()` read, no unpark notification.
async fn await_live_or_cancelled(cancel: &CancelToken) -> bool {
    loop {
        match cancel.state() {
            CancelState::Live => return true,
            CancelState::Cancelled => return false,
            CancelState::Park => tokio::time::sleep(Duration::from_millis(PARK_POLL_INTERVAL_MS)).await,
        }
    }
}

/// 🌳️ Root scope per package, child scope per actor, one `JoinSet` per scope — see the crate doc.
/// Every [`TokioHostRuntime`] scope method delegates here. Deliberately NOT `pub`: this is
/// `TokioHostRuntime`'s own bookkeeping, which is what lets `tokio::runtime::Handle` live in its
/// constructor without that type ever reaching this crate's public API surface. Wraps its state in
/// an `Arc` so [`TokioHostRuntime::cancel_scope`] can hand out a `'static` future without borrowing
/// `&self` — the trait requires a boxed `'static` future, and a plain `&self`-borrowing `async fn`
/// cannot produce one.
#[derive(Clone)]
struct ScopeTable(Arc<ScopeTableInner>);

struct ScopeTableInner {
    handle: tokio::runtime::Handle,
    next_id: AtomicU64,
    records: Mutex<HashMap<ScopeId, ScopeRecord>>,
    owner_index: Mutex<HashMap<ScopeOwner, ScopeId>>,
    children: Mutex<HashMap<ScopeId, Vec<ScopeId>>>,
}

impl ScopeTable {
    fn new(handle: tokio::runtime::Handle) -> ScopeTable {
        ScopeTable(Arc::new(ScopeTableInner { handle, next_id: AtomicU64::new(1), records: Mutex::new(HashMap::new()), owner_index: Mutex::new(HashMap::new()), children: Mutex::new(HashMap::new()) }))
    }

    /// 🔍️ Assumes at most one OPEN scope per `ScopeOwner` at a time (one root scope per package,
    /// one child scope per actor — see the crate doc). Re-opening the same owner replaces the
    /// owner-index entry; the earlier scope's tasks stay tracked by id but become unreachable via
    /// `cancel_scope(owner)` afterward. No caller in this packet re-opens an owner, so this is a
    /// documented limitation rather than an exercised bug.
    fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        let id = ScopeId(self.0.next_id.fetch_add(1, Ordering::SeqCst));
        let cancel = match parent {
            Some(parent_handle) => parent_handle.cancel.child(),
            None => CancelToken::root(),
        };
        let parent_id = parent.map(|handle| handle.id);
        self.0.records.lock().expect("ScopeTable records mutex poisoned").insert(id, ScopeRecord { cancel: cancel.clone(), tasks: tokio::task::JoinSet::new() });
        self.0.owner_index.lock().expect("ScopeTable owner_index mutex poisoned").insert(owner.clone(), id);
        if let Some(parent_id) = parent_id {
            self.0.children.lock().expect("ScopeTable children mutex poisoned").entry(parent_id).or_default().push(id);
        }
        ScopeHandle { id, owner, cancel }
    }

    fn spawn_scoped(&self, scope: &ScopeHandle, ctx: &OperationContext, fut: HostFuture<()>) {
        let cancel = ctx.cancel.clone();
        let wrapped = async move {
            if await_live_or_cancelled(&cancel).await {
                fut.await;
                TaskOutcome::Finished
            } else {
                TaskOutcome::CancelledEarly
            }
        };
        let mut records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
        if let Some(record) = records.get_mut(&scope.id) {
            record.tasks.spawn_on(wrapped, &self.0.handle);
        }
    }

    fn run_blocking(&self, scope: &ScopeHandle, ctx: &OperationContext, work: Box<dyn FnOnce() + Send>) {
        let cancel = ctx.cancel.clone();
        let wrapped = async move {
            if await_live_or_cancelled(&cancel).await {
                let _ = tokio::task::spawn_blocking(work).await;
                TaskOutcome::Finished
            } else {
                TaskOutcome::CancelledEarly
            }
        };
        let mut records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
        if let Some(record) = records.get_mut(&scope.id) {
            record.tasks.spawn_on(wrapped, &self.0.handle);
        }
    }

    fn collect_descendants(&self, root: ScopeId) -> Vec<ScopeId> {
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
    /// link, this just keeps the bookkeeping honest either way. Drains each scope's `JoinSet`
    /// within `grace_ms`; whatever is still in the set once the grace period elapses is honestly
    /// counted `leaked` (never silently folded into `finished`) and THEN force-aborted, so a task
    /// that ignored cancellation does not become a real resource leak in the host process — a
    /// blocking OS thread already underway cannot be preempted, but the tracking task wrapping it
    /// is stopped here.
    async fn cancel_scope(self, owner: ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
        let root_id = match self.0.owner_index.lock().expect("ScopeTable owner_index mutex poisoned").get(&owner).copied() {
            Some(id) => id,
            None => return ScopeDrainReport::default(),
        };
        let scope_ids = self.collect_descendants(root_id);
        {
            let records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
            for id in &scope_ids {
                if let Some(record) = records.get(id) {
                    record.cancel.cancel();
                }
            }
        }
        let grace = Duration::from_millis(grace_ms);
        let mut report = ScopeDrainReport::default();
        for id in &scope_ids {
            let mut tasks = {
                let mut records = self.0.records.lock().expect("ScopeTable records mutex poisoned");
                match records.get_mut(id) {
                    Some(record) => std::mem::replace(&mut record.tasks, tokio::task::JoinSet::new()),
                    None => continue,
                }
            };
            let deadline = tokio::time::Instant::now() + grace;
            loop {
                if tasks.is_empty() {
                    break;
                }
                let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
                match tokio::time::timeout(remaining, tasks.join_next()).await {
                    Ok(Some(Ok(TaskOutcome::Finished))) => report.finished += 1,
                    Ok(Some(Ok(TaskOutcome::CancelledEarly))) => report.cancelled += 1,
                    Ok(Some(Err(_join_error))) => report.cancelled += 1,
                    Ok(None) => break,
                    Err(_timed_out) => break,
                }
            }
            report.leaked += tasks.len() as u32;
            tasks.abort_all();
        }
        report
    }
}

/// 🚂️ The ONE `tokio::runtime::Runtime` this crate builds. `worker_threads` is sized from
/// `plan.io_workers`, `max_blocking_threads` from `plan.compute` — this is what turns tokio's
/// default of up to 512 blocking threads into the bounded budget the design calls for. Threads are
/// checked out from the shared [`ThreadBudget`] at construction; this type never calls
/// `std::thread::available_parallelism` or sizes itself from a core count — the whole point of
/// [`ThreadPlan`] is that exactly one place in the process reads that number.
pub struct TokioHostRuntime {
    runtime: tokio::runtime::Runtime,
    scopes: ScopeTable,
    epoch: tokio::time::Instant,
}

/// 🚫️ [`TokioHostRuntime::new`]'s only failure mode: the underlying builder rejected the plan.
#[derive(Debug)]
pub struct RuntimeBuildError(String);

impl std::fmt::Display for RuntimeBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokioHostRuntime failed to build: {}", self.0)
    }
}
impl std::error::Error for RuntimeBuildError {}

impl TokioHostRuntime {
    /// 🚂️ Builds the one host runtime from `plan`, checking out `plan.io_workers` and
    /// `plan.compute` threads from `budget` — the caller owns `budget` and must pass the SAME
    /// `ThreadPlan` used to size every other role in the process. Reads no clock and no core count
    /// of its own: `epoch` below is read once, from INSIDE the runtime's own context (via a brief
    /// `block_on`), so it and every later `now_ms`/`sleep_until` call are anchored to the same
    /// clock the runtime actually drives.
    pub fn new(plan: ThreadPlan, budget: &ThreadBudget) -> Result<TokioHostRuntime, RuntimeBuildError> {
        budget.checkout(ThreadRole::IoWorker, plan.io_workers);
        budget.checkout(ThreadRole::Compute, plan.compute);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(plan.io_workers.max(1) as usize)
            .max_blocking_threads(plan.compute.max(1) as usize)
            .thread_name("semio-os-services")
            .enable_all()
            .build()
            .map_err(|error| RuntimeBuildError(error.to_string()))?;
        let handle = runtime.handle().clone();
        let epoch = runtime.block_on(async { tokio::time::Instant::now() });
        Ok(TokioHostRuntime { runtime, scopes: ScopeTable::new(handle), epoch })
    }

    /// 🧵️ Drives `f` to completion on this runtime — the entry point a process bootstrap uses for
    /// its own top-level async setup before handing scopes to services.
    pub fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        self.runtime.block_on(f)
    }
}

impl HostAsyncRuntime for TokioHostRuntime {
    fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        self.scopes.open_scope(owner, parent)
    }

    fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) {
        self.scopes.spawn_scoped(scope, &ctx, fut);
    }

    fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>) {
        self.scopes.run_blocking(scope, &ctx, work);
    }

    fn sleep_until(&self, deadline_ms: u64) -> HostFuture<()> {
        let target = self.epoch + Duration::from_millis(deadline_ms);
        Box::pin(async move { tokio::time::sleep_until(target).await })
    }

    fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> HostFuture<ScopeDrainReport> {
        let scopes = self.scopes.clone();
        let owner = owner.clone();
        Box::pin(async move { scopes.cancel_scope(owner, grace_ms).await })
    }

    fn now_ms(&self) -> u64 {
        (tokio::time::Instant::now() - self.epoch).as_millis() as u64
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
    pub fn new(quota_per_plugin: u32) -> WheelCore {
        WheelCore { next_id: 1, next_seq: 0, entries: HashMap::new(), order: BinaryHeap::new(), per_plugin_counts: HashMap::new(), quota_per_plugin }
    }

    /// ⏲️ Arms a timer for `plugin`/`actor` firing at `at_ms`, optionally repeating every
    /// `repeat_ms`. The per-plugin quota is checked BEFORE any insertion, so a rejected call leaves
    /// the wheel completely untouched.
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
    pub fn pop_expired(&mut self, now_ms: u64) -> Vec<TimerFired> {
        let mut fired = Vec::new();
        while let Some(&Reverse((expiry, _seq, id))) = self.order.peek() {
            if expiry > now_ms {
                break;
            }
            self.order.pop();
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

    /// ⏰️ The earliest still-armed (non-cancelled) expiry, if any — lets a driver compute how long
    /// to sleep. O(n) in the number of currently-heaped entries (an acceptable scan given per-host
    /// timer counts are bounded by `quota_per_plugin` times the plugin count; a later packet can
    /// swap in a cancellation-aware heap if that ever matters).
    pub fn next_expiry_ms(&self) -> Option<u64> {
        self.order.iter().filter(|Reverse((_, _, id))| self.entries.get(id).is_some_and(|entry| !entry.cancelled)).map(|Reverse((expiry, _, _))| *expiry).min()
    }

    pub fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.per_plugin_counts.get(plugin).copied().unwrap_or(0)
    }
}

/// 🐌️ How long the driver sleeps when the wheel is empty, before checking again — bounded so the
/// loop still wakes promptly once [`TimerWheel::arm`] posts to `wake` even if that notification is
/// somehow missed.
const TIMER_DRIVER_IDLE_POLL_MS: u64 = 250;

/// ⏲️ The ONE host timer wheel for every plugin's timers — see the crate doc. Owns a [`WheelCore`]
/// behind a `Mutex` plus the thin tokio-backed driver task ([`TimerWheel::spawn_driver`]) that wakes
/// it and posts firings to a [`CompletionSink`]. Plugins arm/disarm through here; they must never
/// spin up a timer of their own.
pub struct TimerWheel {
    core: Arc<Mutex<WheelCore>>,
    wake: Arc<tokio::sync::Notify>,
}

impl TimerWheel {
    pub fn new(quota_per_plugin: u32) -> TimerWheel {
        TimerWheel { core: Arc::new(Mutex::new(WheelCore::new(quota_per_plugin))), wake: Arc::new(tokio::sync::Notify::new()) }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn arm(&self, plugin: PackageId, actor: u64, generation: u16, lane: u8, at_ms: u64, repeat_ms: Option<u64>) -> Result<TimerId, TimerError> {
        let id = self.core.lock().expect("TimerWheel core mutex poisoned").arm(plugin, actor, generation, lane, at_ms, repeat_ms)?;
        self.wake.notify_one();
        Ok(id)
    }

    pub fn disarm(&self, id: TimerId) -> bool {
        self.core.lock().expect("TimerWheel core mutex poisoned").disarm(id)
    }

    pub fn armed_count(&self, plugin: &PackageId) -> u32 {
        self.core.lock().expect("TimerWheel core mutex poisoned").armed_count(plugin)
    }

    /// ▶️ Spawns the driver loop into `scope` on `runtime`: sleeps until the wheel's next expiry (or
    /// wakes early on [`TimerWheel::arm`]), pops everything due, and hands each firing to `sink` —
    /// the ONLY re-entry path, per [`CompletionSink`]'s own doc. `runtime` is `Arc`-owned because
    /// the loop is a detached, indefinitely-recurring task; `ctx` identifies the DRIVER task itself
    /// (its own cancellation/lane), independent of the per-firing `actor`/`generation`/`lane` each
    /// [`TimerFired`] already carries.
    pub fn spawn_driver(&self, runtime: &Arc<dyn HostAsyncRuntime>, scope: &ScopeHandle, ctx: OperationContext, sink: Arc<dyn CompletionSink>) {
        let core = self.core.clone();
        let wake = self.wake.clone();
        let runtime_for_loop = runtime.clone();
        let fut: HostFuture<()> = Box::pin(async move {
            loop {
                let now_ms = runtime_for_loop.now_ms();
                let next = core.lock().expect("TimerWheel core mutex poisoned").next_expiry_ms();
                let sleep_fut: HostFuture<()> = match next {
                    Some(expiry) if expiry > now_ms => runtime_for_loop.sleep_until(expiry),
                    Some(_) => Box::pin(std::future::ready(())),
                    None => runtime_for_loop.sleep_until(now_ms + TIMER_DRIVER_IDLE_POLL_MS),
                };
                tokio::select! {
                    _ = sleep_fut => {}
                    _ = wake.notified() => {}
                }
                let fired = core.lock().expect("TimerWheel core mutex poisoned").pop_expired(runtime_for_loop.now_ms());
                for timer in fired {
                    sink.complete(timer.actor, timer.generation, timer.id.0.to_le_bytes().to_vec(), timer.lane);
                }
            }
        });
        runtime.spawn_scoped(scope, ctx, fut);
    }
}
//#endregion ⏲️TimerWheel

//#region 🧮️ComputePool
/// 🚫️ [`ComputePool::run_blocking`]'s failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComputeError {
    /// ⏰️ `ctx.deadline_ms` elapsed before a pool permit was available, or before the blocking work
    /// finished. Either way the caller's async wait loses the race; the underlying OS thread running
    /// `work` (if it had already started) is NOT forcibly killed — blocking OS threads are not
    /// preemptible, so this is an honest limitation, not a claim of hard cancellation.
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

/// 🧮️ Bounds every blocking-CPU host operation to `plan.compute` — tokio's own default
/// `max_blocking_threads` of 512 is precisely the unbounded-per-CPU failure this type exists to
/// prevent. A semaphore sized to the plan gates logical admission on top of
/// [`TokioHostRuntime::new`]'s OWN `max_blocking_threads` bound, and is also what lets
/// `ctx.deadline_ms` be enforced honestly (a bounded semaphore has somewhere to race a timeout
/// against; an unbounded `spawn_blocking` call does not).
pub struct ComputePool {
    admission: Arc<tokio::sync::Semaphore>,
}

impl ComputePool {
    pub fn new(capacity: u32) -> ComputePool {
        ComputePool { admission: Arc::new(tokio::sync::Semaphore::new(capacity.max(1) as usize)) }
    }

    /// 🧮️ Runs `work` off-executor via [`HostAsyncRuntime::run_blocking`], admitted only once a
    /// permit is free, racing BOTH the admission wait and the result wait against
    /// `runtime.sleep_until(deadline)` whenever `ctx.deadline_ms` is set — either race losing
    /// returns [`ComputeError::DeadlineExceeded`] rather than letting a caller wait past its own
    /// stated deadline.
    pub async fn run_blocking<T: Send + 'static>(&self, runtime: &dyn HostAsyncRuntime, scope: &ScopeHandle, ctx: OperationContext, work: impl FnOnce() -> T + Send + 'static) -> Result<T, ComputeError> {
        let admission = self.admission.clone();
        let permit = match ctx.deadline_ms {
            Some(deadline_ms) => {
                tokio::select! {
                    permit = admission.acquire_owned() => permit.map_err(|_| ComputeError::WorkerLost)?,
                    _ = runtime.sleep_until(deadline_ms) => return Err(ComputeError::DeadlineExceeded),
                }
            }
            None => admission.acquire_owned().await.map_err(|_| ComputeError::WorkerLost)?,
        };
        let (result_tx, result_rx) = tokio::sync::oneshot::channel::<T>();
        let ctx_for_run = ctx.clone();
        runtime.run_blocking(
            scope,
            ctx_for_run,
            Box::new(move || {
                let _permit = permit;
                let result = work();
                let _ = result_tx.send(result);
            }),
        );
        match ctx.deadline_ms {
            Some(deadline_ms) => {
                tokio::select! {
                    result = result_rx => result.map_err(|_| ComputeError::WorkerLost),
                    _ = runtime.sleep_until(deadline_ms) => Err(ComputeError::DeadlineExceeded),
                }
            }
            None => result_rx.await.map_err(|_| ComputeError::WorkerLost),
        }
    }
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

/// 🌐️ Blocking HTTP transport this pool drives through [`ComputePool`] — the same "one blocking
/// call on a dedicated thread" technique `📇️directory/🔌️client` already uses for `ureq`. NO
/// implementation ships in this packet: [`HttpPool::new`] takes any `Arc<dyn HttpTransport>`, and a
/// real transport (a `ureq`-backed one, or a real connection-pooling client) is later-packet wiring
/// — see the crate report's `## honest gaps`. Kept as a trait rather than a concrete client so this
/// crate adds no new external HTTP dependency of its own.
pub trait HttpTransport: Send + Sync {
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error>;
}

/// 🚧️ The default [`HttpTransport`] until a later packet wires a real one — every call fails
/// loudly rather than silently succeeding with fake data.
pub struct UnwiredHttpTransport;
impl HttpTransport for UnwiredHttpTransport {
    fn call(&self, _request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        Err(std::io::Error::other("HttpPool: no HttpTransport wired yet (see the packet report's honest gaps)"))
    }
}

struct TokenBucket {
    remaining_bytes: u64,
    capacity_bytes: u64,
}

impl TokenBucket {
    fn new(capacity_bytes: u64) -> TokenBucket {
        TokenBucket { remaining_bytes: capacity_bytes, capacity_bytes }
    }

    fn try_consume(&mut self, bytes: u64) -> bool {
        if bytes > self.remaining_bytes {
            false
        } else {
            self.remaining_bytes -= bytes;
            true
        }
    }

    /// ♻️ Refills back toward capacity. The per-minute replenishment SCHEDULE (a driver ticking this
    /// once a minute) is not wired in this packet — see the crate report's `## honest gaps`; this
    /// method exists so that wiring is a one-line addition later, and so tests can exercise refill
    /// deterministically today.
    fn refill(&mut self, bytes: u64) {
        self.remaining_bytes = (self.remaining_bytes + bytes).min(self.capacity_bytes);
    }
}

/// 🌐️ Shared connection-pool boundary: a per-package `network_bytes_per_min` token bucket and a
/// per-actor `outstanding_requests` cap, gating an [`HttpTransport`] call run through
/// [`ComputePool`]. See [`UnwiredHttpTransport`]'s doc for what is and is not wired in this packet.
pub struct HttpPool {
    transport: Arc<dyn HttpTransport>,
    compute: Arc<ComputePool>,
    buckets: Mutex<HashMap<PackageId, TokenBucket>>,
    bytes_per_minute_cap: u64,
    outstanding: Mutex<HashMap<ActorId, u32>>,
    outstanding_cap: u32,
}

impl HttpPool {
    pub fn new(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        HttpPool { transport, compute, buckets: Mutex::new(HashMap::new()), bytes_per_minute_cap, outstanding: Mutex::new(HashMap::new()), outstanding_cap: outstanding_cap.max(1) }
    }

    /// ♻️ Test/operator hook for the per-minute refill this packet does not yet drive on a timer —
    /// see [`TokenBucket::refill`]'s doc.
    pub fn refill_package_budget(&self, package: &PackageId, bytes: u64) {
        self.buckets.lock().expect("HttpPool buckets mutex poisoned").entry(package.clone()).or_insert_with(|| TokenBucket::new(self.bytes_per_minute_cap)).refill(bytes);
    }

    pub async fn request(&self, runtime: &dyn HostAsyncRuntime, scope: &ScopeHandle, ctx: OperationContext, package: PackageId, actor: ActorId, request: HttpRequest) -> Result<HttpResponse, HttpPoolError> {
        {
            let mut outstanding = self.outstanding.lock().expect("HttpPool outstanding mutex poisoned");
            let count = outstanding.entry(actor).or_insert(0);
            if *count >= self.outstanding_cap {
                return Err(HttpPoolError::OutstandingCapReached { actor, limit: self.outstanding_cap });
            }
            *count += 1;
        }
        let estimated_bytes = (request.body.len() + request.url.len()) as u64;
        let admitted = {
            let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
            let bucket = buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new(self.bytes_per_minute_cap));
            bucket.try_consume(estimated_bytes)
        };
        if !admitted {
            let mut outstanding = self.outstanding.lock().expect("HttpPool outstanding mutex poisoned");
            if let Some(count) = outstanding.get_mut(&actor) {
                *count = count.saturating_sub(1);
            }
            return Err(HttpPoolError::ByteBudgetExhausted { package });
        }
        let transport = self.transport.clone();
        let result = self.compute.run_blocking(runtime, scope, ctx, move || transport.call(request)).await;
        {
            let mut outstanding = self.outstanding.lock().expect("HttpPool outstanding mutex poisoned");
            if let Some(count) = outstanding.get_mut(&actor) {
                *count = count.saturating_sub(1);
            }
        }
        match result {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(io_error)) => Err(HttpPoolError::Transport(io_error.to_string())),
            Err(compute_error) => Err(HttpPoolError::Compute(compute_error)),
        }
    }
}
//#endregion 🌐️HttpPool

//#region 💾️StorageScheduler
/// 🚫️ [`StorageScheduler::submit`]'s failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError {
    BytesQuotaExceeded { plugin: PackageId, limit: u64 },
    Io(String),
    Closed,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::BytesQuotaExceeded { plugin, limit } => write!(f, "plugin {:?} exceeded its {limit}-byte storage quota", plugin.0),
            StorageError::Io(message) => write!(f, "storage io error: {message}"),
            StorageError::Closed => write!(f, "storage scheduler dropped the job before it ran"),
        }
    }
}
impl std::error::Error for StorageError {}

struct StorageJob {
    plugin: PackageId,
    bytes: u64,
    ctx: OperationContext,
    work: Box<dyn FnOnce() -> Result<Vec<u8>, std::io::Error> + Send>,
    result_tx: tokio::sync::oneshot::Sender<Result<Vec<u8>, StorageError>>,
}

struct StorageState {
    runtime: Arc<dyn HostAsyncRuntime>,
    scope: ScopeHandle,
    queues: Mutex<BTreeMap<u8, VecDeque<StorageJob>>>,
    in_flight: AtomicU32,
    max_in_flight: u32,
    per_plugin_bytes: Mutex<HashMap<PackageId, u64>>,
    byte_quota_per_plugin: u64,
}

/// ▶️ Reentrant dispatch step: while a slot is free, pops the head of the HIGHEST-priority
/// non-empty lane (`BTreeMap` iterates ascending, and lower `ctx.lane` is higher priority — the same
/// convention `OperationContext.lane` documents) and runs it through [`HostAsyncRuntime::run_blocking`].
/// No separate background polling task exists: [`StorageScheduler::submit`] and every job's own
/// completion both call this again, so a freed slot or a newly queued job always gets a chance to
/// dispatch without a dedicated loop.
fn storage_try_dispatch(state: &Arc<StorageState>) {
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
        state.in_flight.fetch_add(1, Ordering::SeqCst);
        let recurse_state = state.clone();
        let plugin = job.plugin.clone();
        let bytes = job.bytes;
        let ctx = job.ctx.clone();
        state.runtime.run_blocking(
            &state.scope,
            ctx,
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
/// max_in_flight`. Deadline racing (unlike [`ComputePool::run_blocking`]) is not wired for storage
/// ops in this packet — see the crate report's `## honest gaps`.
pub struct StorageScheduler(Arc<StorageState>);

impl StorageScheduler {
    pub fn new(runtime: Arc<dyn HostAsyncRuntime>, scope: ScopeHandle, max_in_flight: u32, byte_quota_per_plugin: u64) -> StorageScheduler {
        StorageScheduler(Arc::new(StorageState { runtime, scope, queues: Mutex::new(BTreeMap::new()), in_flight: AtomicU32::new(0), max_in_flight: max_in_flight.max(1), per_plugin_bytes: Mutex::new(HashMap::new()), byte_quota_per_plugin }))
    }

    /// 💾️ Enqueues `work`, reserving `bytes` against `plugin`'s budget up front. Returns a
    /// [`StorageTicket`] the caller awaits for the eventual result, or a typed
    /// [`StorageError::BytesQuotaExceeded`] immediately if the reservation itself does not fit —
    /// the wheel is left untouched on that path, same discipline as [`WheelCore::arm`].
    pub fn submit(&self, ctx: &OperationContext, plugin: PackageId, bytes: u64, work: impl FnOnce() -> Result<Vec<u8>, std::io::Error> + Send + 'static) -> Result<StorageTicket, StorageError> {
        {
            let mut usage = self.0.per_plugin_bytes.lock().expect("StorageScheduler per_plugin_bytes mutex poisoned");
            let current = usage.get(&plugin).copied().unwrap_or(0);
            if current + bytes > self.0.byte_quota_per_plugin {
                return Err(StorageError::BytesQuotaExceeded { plugin, limit: self.0.byte_quota_per_plugin });
            }
            usage.insert(plugin.clone(), current + bytes);
        }
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();
        let job = StorageJob { plugin, bytes, ctx: ctx.clone(), work: Box::new(work), result_tx };
        self.0.queues.lock().expect("StorageScheduler queues mutex poisoned").entry(ctx.lane).or_default().push_back(job);
        storage_try_dispatch(&self.0);
        Ok(StorageTicket { receiver: result_rx })
    }

    pub fn in_flight(&self) -> u32 {
        self.0.in_flight.load(Ordering::SeqCst)
    }
}

/// 🎫️ A handle to one [`StorageScheduler::submit`] call's eventual result — deliberately opaque:
/// the tokio receiver it wraps is a PRIVATE field (never named on this struct's own declaration
/// line), so nothing outside this crate can see or name a tokio type through it.
pub struct StorageTicket {
    receiver: tokio::sync::oneshot::Receiver<Result<Vec<u8>, StorageError>>,
}
impl StorageTicket {
    pub async fn await_result(self) -> Result<Vec<u8>, StorageError> {
        self.receiver.await.unwrap_or(Err(StorageError::Closed))
    }
}
//#endregion 💾️StorageScheduler

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
    LatestWins(Option<Vec<u8>>),
    Coalesced { pending: HashMap<String, Vec<u8>>, order: VecDeque<String> },
    LosslessBounded { cap: u32, pending: VecDeque<Vec<u8>> },
    ByteCredit { remaining: u64 },
}

impl Mailbox {
    fn new(policy: &ChannelPolicy) -> Mailbox {
        match policy {
            ChannelPolicy::LatestWins => Mailbox::LatestWins(None),
            ChannelPolicy::Coalesced { .. } => Mailbox::Coalesced { pending: HashMap::new(), order: VecDeque::new() },
            ChannelPolicy::LosslessBounded { cap } => Mailbox::LosslessBounded { cap: *cap, pending: VecDeque::new() },
            ChannelPolicy::ByteCredit { bytes } => Mailbox::ByteCredit { remaining: *bytes },
        }
    }

    /// 📮️ `coalesce_key` is only meaningful for a [`ChannelPolicy::Coalesced`] mailbox — an
    /// incoming message under a key already pending REPLACES it (collapse); a new key queues
    /// alongside the others.
    fn publish(&mut self, coalesce_key: Option<&str>, payload: Vec<u8>) -> PublishOutcome {
        match self {
            Mailbox::LatestWins(slot) => {
                let collapsed = slot.is_some();
                *slot = Some(payload);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::Coalesced { pending, order } => {
                let key = coalesce_key.unwrap_or_default().to_string();
                let collapsed = pending.insert(key.clone(), payload).is_some();
                if !collapsed {
                    order.push_back(key);
                }
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::LosslessBounded { cap, pending } => {
                if pending.len() as u32 >= *cap {
                    PublishOutcome::RejectedFull { cap: *cap }
                } else {
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
    fn drain(&mut self) -> Vec<Vec<u8>> {
        match self {
            Mailbox::LatestWins(slot) => slot.take().into_iter().collect(),
            Mailbox::Coalesced { pending, order } => {
                let mut out = Vec::new();
                while let Some(key) = order.pop_front() {
                    if let Some(value) = pending.remove(&key) {
                        out.push(value);
                    }
                }
                out
            }
            Mailbox::LosslessBounded { pending, .. } => pending.drain(..).collect(),
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
/// `LatestWins`/`Coalesced` collapse, `LosslessBounded` rejects rather than growing past `cap`,
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
    pub fn new() -> EventRouter {
        EventRouter { subscribers: Mutex::new(HashMap::new()), mailboxes: Mutex::new(HashMap::new()) }
    }

    pub fn subscribe(&self, topic: Topic, actor: ActorId, policy: ChannelPolicy) {
        let mailbox = Mailbox::new(&policy);
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").insert((topic.clone(), actor), mailbox);
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").entry(topic).or_default().push(Subscriber { actor, policy });
    }

    /// 🔍️ The `ChannelPolicy` `actor` declared for `topic` at [`EventRouter::subscribe`] time, if
    /// still subscribed — lets a caller (e.g. a diagnostics surface) inspect backpressure
    /// vocabulary without reaching into a `Mailbox`, which is private.
    pub fn declared_policy(&self, topic: &Topic, actor: ActorId) -> Option<ChannelPolicy> {
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get(topic)?.iter().find(|subscriber| subscriber.actor == actor).map(|subscriber| subscriber.policy.clone())
    }

    pub fn unsubscribe(&self, topic: &Topic, actor: ActorId) {
        if let Some(subscribers) = self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get_mut(topic) {
            subscribers.retain(|subscriber| subscriber.actor != actor);
        }
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").remove(&(topic.clone(), actor));
    }

    /// 📮️ Delivers `payload` to every subscriber of `topic`, honouring each one's OWN policy
    /// independently — one bounded subscriber rejecting never affects another's delivery.
    pub fn publish(&self, topic: &Topic, coalesce_key: Option<&str>, payload: &[u8]) -> Vec<(ActorId, PublishOutcome)> {
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
    pub fn send_message(&self, topic: &Topic, actor: ActorId, payload: Vec<u8>) -> PublishOutcome {
        match self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)) {
            Some(mailbox) => mailbox.publish(None, payload),
            None => PublishOutcome::NoSuchSubscriber,
        }
    }

    pub fn drain(&self, topic: &Topic, actor: ActorId) -> Vec<Vec<u8>> {
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)).map(|mailbox| mailbox.drain()).unwrap_or_default()
    }
}
//#endregion 📮️EventRouter

//#region 🧾️CompletionSink
/// 🧾 The ONLY way any service in this crate re-enters the kernel: hand a completed operation's
/// result back as raw bytes tagged with the actor/generation/lane it belongs to. No type in this
/// crate holds or calls a `Kernel` directly — every completion flows out through this trait instead,
/// the same seam discipline `HostAsyncRuntime` itself uses to hide tokio.
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
    pub fn new() -> MockCompletionSink {
        MockCompletionSink::default()
    }

    pub fn recorded(&self) -> Vec<CompletionRecord> {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").clone()
    }
}

impl CompletionSink for MockCompletionSink {
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8) {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").push(CompletionRecord { actor, generation, event_bytes, lane });
    }
}
//#endregion 🧾️CompletionSink

//#region 🧬️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_async::{testkit::ManualRuntime, thread_plan, TraceId};
    use std::sync::atomic::AtomicBool;

    fn test_ctx(actor: u64, cancel: CancelToken) -> OperationContext {
        OperationContext { actor, generation: 0, trace: TraceId(actor), lane: 0, deadline_ms: None, cancel, capability: None }
    }

    //#region 🚂️TokioHostRuntimeTests
    #[test]
    fn tokio_host_runtime_checks_out_io_and_compute_threads_from_the_budget() {
        let plan = thread_plan(8);
        let budget = ThreadBudget::from_plan(plan);
        let _runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        assert_eq!(budget.remaining(ThreadRole::IoWorker), 0);
        assert_eq!(budget.remaining(ThreadRole::Compute), 0);
        assert_eq!(budget.remaining(ThreadRole::Shard), plan.shards, "TokioHostRuntime must not touch roles it does not own");
    }

    #[test]
    fn tokio_host_runtime_now_ms_advances_monotonically() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let first = runtime.now_ms();
        runtime.block_on(async { tokio::time::sleep(Duration::from_millis(5)).await });
        assert!(runtime.now_ms() >= first, "now_ms must never go backward");
    }
    //#endregion 🚂️TokioHostRuntimeTests

    //#region 🌳️ScopeTableTests
    #[test]
    fn cancel_scope_cancels_child_scopes_transitively() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let package = runtime.open_scope(ScopeOwner::Package("pkg-a".to_string()), None);
        let actor = runtime.open_scope(ScopeOwner::Actor(1), Some(&package));
        runtime.block_on(async {
            let _ = runtime.cancel_scope(&package.owner, 50).await;
        });
        assert!(actor.cancel.is_cancelled(), "child scope must observe the package scope's cancellation");
    }

    /// 🚨️ The spawned task sleeps far past the grace period and never checks its cancel token, so
    /// `cancel_scope` must report it `leaked` rather than `finished`. The 20ms sleep before
    /// cancelling gives the task a chance to actually start (pass the initial live/cancelled gate)
    /// first.
    #[test]
    fn cancel_scope_reports_leaked_task_that_ignores_cancellation_not_finished() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let scope = runtime.open_scope(ScopeOwner::Actor(2), None);
        let ctx = test_ctx(2, scope.cancel.clone());
        runtime.spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                }
            }),
        );
        let report = runtime.block_on(async {
            tokio::time::sleep(Duration::from_millis(20)).await;
            runtime.cancel_scope(&scope.owner, 60).await
        });
        assert_eq!(report.leaked, 1, "a task that ignores cancellation must be reported leaked, never finished");
        assert_eq!(report.finished, 0);
    }

    #[test]
    fn cancel_scope_counts_a_cooperative_task_as_finished() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let scope = runtime.open_scope(ScopeOwner::Actor(3), None);
        let ctx = test_ctx(3, scope.cancel.clone());
        let ran = Arc::new(AtomicBool::new(false));
        let ran_clone = ran.clone();
        runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) }));
        let report = runtime.block_on(async {
            tokio::time::sleep(Duration::from_millis(20)).await;
            runtime.cancel_scope(&scope.owner, 60).await
        });
        assert!(ran.load(Ordering::SeqCst));
        assert_eq!(report.finished, 1);
        assert_eq!(report.leaked, 0);
    }

    #[test]
    fn park_holds_new_work_until_unparked() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let scope = runtime.open_scope(ScopeOwner::Service("park-test"), None);
        scope.cancel.park();
        let ran = Arc::new(AtomicBool::new(false));
        let ran_clone = ran.clone();
        let ctx = test_ctx(0, scope.cancel.clone());
        runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) }));
        runtime.block_on(async { tokio::time::sleep(Duration::from_millis(2 * PARK_POLL_INTERVAL_MS)).await });
        assert!(!ran.load(Ordering::SeqCst), "parked scope must hold new work rather than running it");
        scope.cancel.unpark();
        runtime.block_on(async { tokio::time::sleep(Duration::from_millis(4 * PARK_POLL_INTERVAL_MS)).await });
        assert!(ran.load(Ordering::SeqCst), "unparked scope must eventually run the held work");
    }
    //#endregion 🌳️ScopeTableTests

    //#region 🧮️ComputePoolTests
    #[test]
    fn run_blocking_never_exceeds_the_compute_bound_under_a_burst() {
        let plan = ThreadPlan { kernel: 1, shards: 2, io_workers: 1, compute: 3, epoch_ticker: 1 };
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let pool = ComputePool::new(plan.compute);
        let scope = runtime.open_scope(ScopeOwner::Service("compute-burst"), None);
        let current = Arc::new(AtomicU32::new(0));
        let observed_max = Arc::new(AtomicU32::new(0));
        runtime.block_on(async {
            let mut handles = Vec::new();
            for i in 0..12u32 {
                let pool = &pool;
                let runtime = &runtime;
                let scope = &scope;
                let current = current.clone();
                let observed_max = observed_max.clone();
                let ctx = test_ctx(i as u64, scope.cancel.clone());
                handles.push(async move {
                    pool.run_blocking(runtime, scope, ctx, move || {
                        let now = current.fetch_add(1, Ordering::SeqCst) + 1;
                        observed_max.fetch_max(now, Ordering::SeqCst);
                        std::thread::sleep(Duration::from_millis(25));
                        current.fetch_sub(1, Ordering::SeqCst);
                    })
                    .await
                    .expect("run_blocking without a deadline must not fail");
                });
            }
            futures_join_all(handles).await;
        });
        assert!(observed_max.load(Ordering::SeqCst) <= plan.compute, "observed concurrency {} exceeded the compute bound {}", observed_max.load(Ordering::SeqCst), plan.compute);
        assert!(observed_max.load(Ordering::SeqCst) >= 2, "burst should have produced measurable overlap; observed {}", observed_max.load(Ordering::SeqCst));
    }

    #[test]
    fn run_blocking_deadline_actually_fires_and_the_late_result_is_not_awaited() {
        let plan = thread_plan(8);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let pool = ComputePool::new(plan.compute);
        let scope = runtime.open_scope(ScopeOwner::Service("compute-deadline"), None);
        let (unblock_tx, unblock_rx) = std::sync::mpsc::channel::<()>();
        let outcome = runtime.block_on(async {
            let now = runtime.now_ms();
            let mut ctx = test_ctx(0, scope.cancel.clone());
            ctx.deadline_ms = Some(now + 40);
            pool.run_blocking(&runtime, &scope, ctx, move || {
                unblock_rx.recv().expect("test should unblock this thread");
                7
            })
            .await
        });
        assert_eq!(outcome, Err(ComputeError::DeadlineExceeded), "a deadline shorter than the blocking work must lose the race");
        let _ = unblock_tx.send(());
    }

    /// 🧵️ Minimal `join_all` so this crate's tests do not pull in the `futures` crate for one call
    /// site — polls every future in a simple round-robin until all are ready.
    async fn futures_join_all<F: std::future::Future<Output = ()>>(futures: Vec<F>) {
        use std::pin::Pin;
        let mut pending: Vec<Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
        while !pending.is_empty() {
            let mut still_pending = Vec::new();
            for mut fut in pending {
                if std::future::Future::poll(fut.as_mut(), &mut std::task::Context::from_waker(std::task::Waker::noop())) == std::task::Poll::Pending {
                    still_pending.push(fut);
                }
            }
            pending = still_pending;
            tokio::task::yield_now().await;
        }
    }
    //#endregion 🧮️ComputePoolTests

    //#region ⏲️WheelCoreTests
    #[test]
    fn wheel_core_pop_expired_fires_in_expiry_order_not_arm_order() {
        let mut wheel = WheelCore::new(10);
        let plugin = PackageId("p".to_string());
        let late = wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
        let early = wheel.arm(plugin, 2, 0, 0, 100, None).unwrap();
        let fired: Vec<TimerId> = wheel.pop_expired(1_000).into_iter().map(|f| f.id).collect();
        assert_eq!(fired, vec![early, late], "expiry order must win over arm order");
    }

    #[test]
    fn wheel_core_pop_expired_respects_now_ms_boundary() {
        let mut wheel = WheelCore::new(10);
        let plugin = PackageId("p".to_string());
        wheel.arm(plugin, 1, 0, 0, 500, None).unwrap();
        assert!(wheel.pop_expired(400).is_empty(), "must not fire before its expiry");
        assert_eq!(wheel.pop_expired(500).len(), 1, "must fire once now_ms reaches the expiry");
    }

    /// ⏲️ Jumps far past several repeat periods: a naive `expiry += repeat` applied once would
    /// still land before `now_ms` and fire again immediately on the next call; the catch-up loop
    /// in `pop_expired` must land beyond `now_ms` instead.
    #[test]
    fn wheel_core_repeat_rearms_and_catches_up_without_drift_accumulation() {
        let mut wheel = WheelCore::new(10);
        let plugin = PackageId("p".to_string());
        let id = wheel.arm(plugin, 1, 0, 0, 100, Some(100)).unwrap();
        let first = wheel.pop_expired(100);
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].id, id);
        let second = wheel.pop_expired(450);
        assert_eq!(second.len(), 1, "a repeating timer must fire exactly once per pop_expired call even after a large time jump");
        assert!(wheel.next_expiry_ms().unwrap() > 450);
    }

    #[test]
    fn wheel_core_disarm_prevents_a_future_fire() {
        let mut wheel = WheelCore::new(10);
        let plugin = PackageId("p".to_string());
        let id = wheel.arm(plugin, 1, 0, 0, 100, None).unwrap();
        assert!(wheel.disarm(id));
        assert!(wheel.pop_expired(1_000).is_empty());
        assert!(!wheel.disarm(id), "disarming twice must report false the second time");
    }

    #[test]
    fn wheel_core_rejects_arm_past_the_per_plugin_quota_with_a_typed_error() {
        let mut wheel = WheelCore::new(2);
        let plugin = PackageId("p".to_string());
        wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
        wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
        let result = wheel.arm(plugin.clone(), 1, 0, 0, 300, None);
        assert_eq!(result, Err(TimerError::QuotaExceeded { plugin: plugin.clone(), limit: 2 }));
        assert_eq!(wheel.armed_count(&plugin), 2, "a rejected arm must leave the wheel untouched");
    }

    #[test]
    fn wheel_core_disarm_frees_quota_for_a_new_arm() {
        let mut wheel = WheelCore::new(1);
        let plugin = PackageId("p".to_string());
        let id = wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
        assert!(wheel.arm(plugin.clone(), 1, 0, 0, 200, None).is_err());
        wheel.disarm(id);
        assert!(wheel.arm(plugin, 1, 0, 0, 200, None).is_ok());
    }
    //#endregion ⏲️WheelCoreTests

    //#region ⏲️TimerWheelDriverTests
    #[test]
    fn timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink() {
        let manual = ManualRuntime::new(0);
        let runtime: Arc<dyn HostAsyncRuntime> = Arc::new(manual.clone());
        let scope = runtime.open_scope(ScopeOwner::Service("timer-driver-test"), None);
        let wheel = TimerWheel::new(10);
        let sink = Arc::new(MockCompletionSink::new());
        let ctx = test_ctx(0, scope.cancel.clone());
        wheel.spawn_driver(&runtime, &scope, ctx, sink.clone());
        wheel.arm(PackageId("plugin-a".to_string()), 42, 3, 1, 100, None).expect("arm should succeed");
        manual.drive();
        assert!(sink.recorded().is_empty(), "must not fire before the injected clock reaches the deadline");
        manual.set_now_ms(100);
        manual.drive();
        let recorded = sink.recorded();
        assert_eq!(recorded.len(), 1, "the driver must post exactly one completion for the fired timer");
        assert_eq!(recorded[0].actor, 42);
        assert_eq!(recorded[0].generation, 3);
        assert_eq!(recorded[0].lane, 1);
    }
    //#endregion ⏲️TimerWheelDriverTests

    //#region 💾️StorageSchedulerTests
    /// 💾️ `ManualRuntime::run_blocking` runs its closure synchronously, in-line, so nothing can
    /// ever be genuinely QUEUED behind it — a real `TokioHostRuntime` is required here to make the
    /// priority ordering observable at all: one job occupies the single in-flight slot on its own
    /// blocking thread while the two real submissions below queue behind it.
    #[test]
    fn storage_scheduler_dispatches_highest_priority_lane_first_despite_submit_order() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = Arc::new(TokioHostRuntime::new(plan, &budget).expect("runtime should build"));
        let scope = runtime.open_scope(ScopeOwner::Service("storage-priority-test"), None);
        let scheduler = StorageScheduler::new(runtime.clone() as Arc<dyn HostAsyncRuntime>, scope.clone(), 1, 1_000_000);
        let order = Arc::new(Mutex::new(Vec::<&'static str>::new()));

        let (occupy_tx, occupy_rx) = std::sync::mpsc::channel::<()>();
        let occupy_rx = Mutex::new(occupy_rx);
        let occupy_ctx = test_ctx(0, scope.cancel.clone());
        let occupy_ticket = scheduler
            .submit(&occupy_ctx, PackageId("occupy".to_string()), 1, move || {
                occupy_rx.lock().unwrap().recv().ok();
                Ok(Vec::new())
            })
            .unwrap();

        let mut low_ctx = test_ctx(0, scope.cancel.clone());
        low_ctx.lane = 200;
        let order_low = order.clone();
        let low_ticket = scheduler
            .submit(&low_ctx, PackageId("p".to_string()), 1, move || {
                order_low.lock().unwrap().push("low");
                Ok(Vec::new())
            })
            .unwrap();

        let mut high_ctx = test_ctx(0, scope.cancel);
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

    #[test]
    fn storage_scheduler_never_exceeds_max_in_flight() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = Arc::new(TokioHostRuntime::new(plan, &budget).expect("runtime should build"));
        let scope = runtime.open_scope(ScopeOwner::Service("storage-cap-test"), None);
        let scheduler = StorageScheduler::new(runtime.clone() as Arc<dyn HostAsyncRuntime>, scope.clone(), 2, 1_000_000);
        let mut tickets = Vec::new();
        for i in 0..8u32 {
            let ctx = test_ctx(0, scope.cancel.clone());
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
                    if scheduler_ref.in_flight() > 2 {
                        cap_violated_ref.store(true, Ordering::SeqCst);
                    }
                    tokio::task::yield_now().await;
                }
            };
            let drain = async {
                for ticket in tickets {
                    let _ = ticket.await_result().await;
                }
            };
            tokio::select! {
                _ = sampler => {}
                _ = drain => {}
            }
        });
        assert!(!cap_violated.load(Ordering::SeqCst), "in-flight count must never exceed max_in_flight");
    }

    /// 💾️ A real `TokioHostRuntime` is required here too (same reasoning as the priority test
    /// above): the first job must still be genuinely IN FLIGHT — holding its 60-byte reservation —
    /// when the second `submit` is checked, so it is blocked on a channel rather than left to
    /// `ManualRuntime`'s synchronous, immediately-releasing execution.
    #[test]
    fn storage_scheduler_rejects_over_budget_submit_with_a_typed_error_and_untouched_usage() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = Arc::new(TokioHostRuntime::new(plan, &budget).expect("runtime should build"));
        let scope = runtime.open_scope(ScopeOwner::Service("storage-budget-test"), None);
        let scheduler = StorageScheduler::new(runtime.clone() as Arc<dyn HostAsyncRuntime>, scope.clone(), 4, 100);
        let ctx = test_ctx(0, scope.cancel);
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
    //#endregion 💾️StorageSchedulerTests

    //#region 📮️EventRouterTests
    #[test]
    fn event_router_latest_wins_collapses_older_pending_value() {
        let router = EventRouter::new();
        let topic = Topic("scene.updates".to_string());
        let actor = ActorId(1);
        router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins);
        let first = router.publish(&topic, None, b"v1");
        let second = router.publish(&topic, None, b"v2");
        assert_eq!(first, vec![(actor, PublishOutcome::Delivered)]);
        assert_eq!(second, vec![(actor, PublishOutcome::Collapsed)]);
        assert_eq!(router.drain(&topic, actor), vec![b"v2".to_vec()], "only the LATEST value must survive the collapse");
    }

    #[test]
    fn event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth() {
        let router = EventRouter::new();
        let topic = Topic("jobs.updates".to_string());
        let actor = ActorId(2);
        router.subscribe(topic.clone(), actor, ChannelPolicy::LosslessBounded { cap: 2 });
        assert_eq!(router.publish(&topic, None, b"a"), vec![(actor, PublishOutcome::Delivered)]);
        assert_eq!(router.publish(&topic, None, b"b"), vec![(actor, PublishOutcome::Delivered)]);
        assert_eq!(router.publish(&topic, None, b"c"), vec![(actor, PublishOutcome::RejectedFull { cap: 2 })], "must reject rather than grow past cap");
        assert_eq!(router.drain(&topic, actor), vec![b"a".to_vec(), b"b".to_vec()], "the rejected message must never have been queued");
    }

    #[test]
    fn event_router_coalesced_collapses_same_key_but_queues_distinct_keys() {
        let router = EventRouter::new();
        let topic = Topic("cursor.updates".to_string());
        let actor = ActorId(3);
        router.subscribe(topic.clone(), actor, ChannelPolicy::Coalesced { key: "cursor".to_string() });
        router.publish(&topic, Some("peer-1"), b"pos-1");
        let outcome = router.publish(&topic, Some("peer-1"), b"pos-2");
        router.publish(&topic, Some("peer-2"), b"pos-a");
        assert_eq!(outcome, vec![(actor, PublishOutcome::Collapsed)]);
        let drained = router.drain(&topic, actor);
        assert_eq!(drained.len(), 2, "distinct coalesce keys must not collapse into each other");
        assert!(drained.contains(&b"pos-2".to_vec()));
        assert!(drained.contains(&b"pos-a".to_vec()));
    }

    #[test]
    fn event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket() {
        let router = EventRouter::new();
        let topic = Topic("stream.frames".to_string());
        let actor = ActorId(4);
        router.subscribe(topic.clone(), actor, ChannelPolicy::ByteCredit { bytes: 4 });
        assert_eq!(router.publish(&topic, None, &[0u8; 3]), vec![(actor, PublishOutcome::Delivered)]);
        assert_eq!(router.publish(&topic, None, &[0u8; 3]), vec![(actor, PublishOutcome::RejectedInsufficientCredit)], "must reject once the remaining credit is insufficient");
    }

    #[test]
    fn event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber() {
        let router = EventRouter::new();
        let topic = Topic("scene.updates".to_string());
        let actor = ActorId(5);
        router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins);
        router.unsubscribe(&topic, actor);
        assert_eq!(router.publish(&topic, None, b"x"), Vec::new(), "no subscribers left means no outcomes at all");
        assert_eq!(router.send_message(&topic, actor, b"x".to_vec()), PublishOutcome::NoSuchSubscriber);
    }
    //#endregion 📮️EventRouterTests

    //#region 🌐️HttpPoolTests
    struct RecordingTransport {
        calls: Arc<Mutex<u32>>,
    }
    impl HttpTransport for RecordingTransport {
        fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
            *self.calls.lock().unwrap() += 1;
            Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
        }
    }

    fn sample_request() -> HttpRequest {
        HttpRequest { method: "GET".to_string(), url: "https://example.invalid/x".to_string(), headers: Vec::new(), body: Vec::new() }
    }

    struct BlockingTransport(Mutex<std::sync::mpsc::Receiver<()>>);
    impl HttpTransport for BlockingTransport {
        fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
            self.0.lock().unwrap().recv().ok();
            Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
        }
    }

    /// 🌐️ Runs a first request in the background, blocked on `unblock_rx` so it stays genuinely
    /// outstanding, then waits 40ms (well past the background task's own startup) before issuing a
    /// second request that must be rejected while the first still holds the actor's one slot.
    #[test]
    fn http_pool_rejects_past_the_per_actor_outstanding_cap() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = Arc::new(TokioHostRuntime::new(plan, &budget).expect("runtime should build"));
        let scope = runtime.open_scope(ScopeOwner::Service("http-test"), None);
        let (unblock_tx, unblock_rx) = std::sync::mpsc::channel::<()>();
        let compute = Arc::new(ComputePool::new(plan.compute));
        let pool = Arc::new(HttpPool::new(Arc::new(BlockingTransport(Mutex::new(unblock_rx))), compute, 1_000_000, 1));
        let actor = ActorId(9);

        runtime.block_on(async {
            let pool_bg = pool.clone();
            let runtime_bg = runtime.clone();
            let scope_bg = scope.clone();
            let ctx_bg = test_ctx(0, scope.cancel.clone());
            let handle = tokio::spawn(async move { pool_bg.request(runtime_bg.as_ref(), &scope_bg, ctx_bg, PackageId("pkg".to_string()), actor, sample_request()).await });
            tokio::time::sleep(Duration::from_millis(40)).await;
            let ctx2 = test_ctx(0, scope.cancel.clone());
            let second = pool.request(runtime.as_ref(), &scope, ctx2, PackageId("pkg".to_string()), actor, sample_request()).await;
            assert_eq!(second, Err(HttpPoolError::OutstandingCapReached { actor, limit: 1 }));
            let _ = unblock_tx.send(());
            let first_result = handle.await.expect("background request task must not panic");
            assert!(first_result.is_ok(), "the first request must still complete once unblocked");
        });
    }

    #[test]
    fn http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called() {
        let plan = thread_plan(4);
        let budget = ThreadBudget::from_plan(plan);
        let runtime = TokioHostRuntime::new(plan, &budget).expect("runtime should build");
        let scope = runtime.open_scope(ScopeOwner::Service("http-budget-test"), None);
        let calls = Arc::new(Mutex::new(0));
        let compute = Arc::new(ComputePool::new(plan.compute));
        let pool = HttpPool::new(Arc::new(RecordingTransport { calls: calls.clone() }), compute, 4, 4);
        let actor = ActorId(10);
        let package = PackageId("pkg".to_string());
        let mut big_request = sample_request();
        big_request.body = vec![0u8; 10];
        let ctx = test_ctx(0, scope.cancel.clone());
        let result = runtime.block_on(pool.request(&runtime, &scope, ctx, package.clone(), actor, big_request));
        assert_eq!(result, Err(HttpPoolError::ByteBudgetExhausted { package }));
        assert_eq!(*calls.lock().unwrap(), 0, "the transport must never be called once the budget rejects the request");
    }
    //#endregion 🌐️HttpPoolTests

    //#region 🧾️CompletionSinkTests
    #[test]
    fn mock_completion_sink_records_calls_in_order() {
        let sink = MockCompletionSink::new();
        sink.complete(1, 0, vec![1], 0);
        sink.complete(2, 1, vec![2], 1);
        let recorded = sink.recorded();
        assert_eq!(recorded.len(), 2);
        assert_eq!(recorded[0].actor, 1);
        assert_eq!(recorded[1].actor, 2);
    }
    //#endregion 🧾️CompletionSinkTests
}
//#endregion 🧬️Tests
