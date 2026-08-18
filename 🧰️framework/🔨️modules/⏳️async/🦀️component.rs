//! ⏳️ Async-runtime interface layer — domain-neutral, PURE: no `tokio`, no `std::thread`, no I/O,
//! no clock (every timestamp is `now_ms: u64` handed in by the caller) anywhere in this file. This
//! is what keeps mobile and wasm targets open: every other framework/os/plugin crate names
//! [`HostAsyncRuntime`] and the vocabulary types below, never `tokio` directly, so swapping the
//! concrete executor (tokio today, something else on a constrained target tomorrow) never touches
//! a call site outside the one crate that implements this trait.
//!
//! 🪡 **Where tokio actually lives**: nowhere in this crate. The concrete `tokio`-backed
//! [`HostAsyncRuntime`] implementation is a SIBLING crate (design ticket
//! `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet R2) that depends on this one, not the
//! other way round. [`ManualRuntime`] in this crate (behind the `testkit` feature) exists so
//! downstream crates can unit-test against [`HostAsyncRuntime`] without ever linking tokio.
//!
//! 🧭 **Scope discipline**: [`HostAsyncRuntime::spawn_scoped`] takes a `&ScopeHandle`, not an
//! ambient context — there is no detached-spawn entry point on this trait. Every unit of async work
//! belongs to a [`Scope`] that can be found, waited on and drained; "who owns this task" is always
//! answerable.
//!
//! 🚫 **Not on this trait**: I/O primitives. Timers, HTTP pools, storage schedulers etc. are owned
//! by services built ON TOP of a [`HostAsyncRuntime`] (packet R2 and later); this trait only ever
//! parks/wakes/spawns/cancels, it never touches a socket or a file.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

//#region 🪪️OperationContext
/// 🔖️ Opaque per-operation trace correlation id — carried end to end through logs/metrics, never
/// interpreted by this crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct TraceId(pub u64);

/// 🔑️ Opaque handle to a capability grant held elsewhere (the concrete grant type is an
/// application-layer concern this framework-tier crate must not depend on — same seam discipline
/// `🎭️actor`'s `CapabilityGrant` doc records). Revoking the grant this id names is how a capability
/// revocation propagates into an in-flight [`OperationContext`]: the holder checks the id is still
/// live, it does not itself carry rights.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct CapabilityTokenId(pub u64);

/// 🪪️ Carried by every async host operation so cancellation, deadlines, tracing, scheduling
/// priority (`lane`, mirroring `🎭️actor::Lane`'s discriminant order — kept as a bare `u8` here so
/// this crate never depends on the actor crate) and capability revocation all propagate through the
/// whole operation with one value. Deliberately NOT `Serialize`/`Deserialize`: [`CancelToken`] is a
/// live in-process handle (an `Arc`), not wire data — a context is passed by value within one host
/// process, never encoded to bytes.
#[derive(Clone)]
pub struct OperationContext {
    pub actor: u64,
    pub generation: u16,
    pub trace: TraceId,
    pub lane: u8,
    pub deadline_ms: Option<u64>,
    pub cancel: CancelToken,
    pub capability: Option<CapabilityTokenId>,
}

impl std::fmt::Debug for OperationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperationContext")
            .field("actor", &self.actor)
            .field("generation", &self.generation)
            .field("trace", &self.trace)
            .field("lane", &self.lane)
            .field("deadline_ms", &self.deadline_ms)
            .field("cancel", &self.cancel)
            .field("capability", &self.capability)
            .finish()
    }
}
//#endregion 🪪️OperationContext

//#region 🛑️CancelToken
/// 🛑️ Tri-state a [`CancelToken`] can be in. `Park` is the suspend state — in-flight operations
/// finish, new work is held — distinct from the terminal `Cancelled`. Ordered by severity
/// (`Live < Park < Cancelled`) so a parent's state can be folded into a child's with a plain `max`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum CancelState {
    Live,
    Park,
    Cancelled,
}

impl CancelState {
    fn from_u8(tag: u8) -> CancelState {
        match tag {
            0 => CancelState::Live,
            1 => CancelState::Park,
            _ => CancelState::Cancelled,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            CancelState::Live => 0,
            CancelState::Park => 1,
            CancelState::Cancelled => 2,
        }
    }
}

struct CancelNode {
    local: AtomicU8,
    parent: Option<CancelToken>,
}

/// 🛑️ Cooperative cancellation handle: an `Arc`-shared tri-state ([`CancelState`]) plus an optional
/// parent link. [`CancelToken::child`] derives a descendant whose effective [`CancelToken::state`]
/// is the max-severity fold of its own local state and every ancestor's — cancelling a parent scope
/// therefore transitively cancels every descendant without walking a child registry, and a child's
/// own `park`/`cancel` never affects its parent.
#[derive(Clone)]
pub struct CancelToken(Arc<CancelNode>);

impl CancelToken {
    /// 🌱️ A fresh root token with no parent, starting `Live`.
    pub fn root() -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(CancelState::Live.to_u8()), parent: None }))
    }

    /// 👶️ A descendant token: its effective state is never less severe than `self`'s.
    pub fn child(&self) -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(CancelState::Live.to_u8()), parent: Some(self.clone()) }))
    }

    /// ⏸️ Enter the suspend state — a no-op once `Cancelled` (terminal, never downgraded).
    pub fn park(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Live.to_u8(), CancelState::Park.to_u8(), Ordering::SeqCst, Ordering::SeqCst);
    }

    /// ▶️ Leave the suspend state back to `Live` — a no-op once `Cancelled`.
    pub fn unpark(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Park.to_u8(), CancelState::Live.to_u8(), Ordering::SeqCst, Ordering::SeqCst);
    }

    /// 🛑️ Terminal: always wins over `Live`/`Park`, and over anything a later `park`/`unpark` on
    /// this same token would attempt.
    pub fn cancel(&self) {
        self.0.local.store(CancelState::Cancelled.to_u8(), Ordering::SeqCst);
    }

    /// 🔍️ Max-severity fold of this token's local state and every ancestor's.
    pub fn state(&self) -> CancelState {
        let local = CancelState::from_u8(self.0.local.load(Ordering::SeqCst));
        match &self.0.parent {
            Some(parent) => local.max(parent.state()),
            None => local,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.state() == CancelState::Cancelled
    }

    pub fn is_parked(&self) -> bool {
        self.state() == CancelState::Park
    }

    pub fn is_live(&self) -> bool {
        self.state() == CancelState::Live
    }
}

impl std::fmt::Debug for CancelToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CancelToken({:?})", self.state())
    }
}
//#endregion 🛑️CancelToken

//#region 🌳️Scope
/// 🌳️ Who a [`Scope`] belongs to. `Service` is a small closed set of well-known host subsystem
/// names (e.g. `"timer"`, `"http_pool"`) rather than owned `String` data, so it deliberately has no
/// `Deserialize`/typegen derive here — see the field-level doc.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeOwner {
    Actor(u64),
    Package(String),
    /// 🏷️ A `&'static str` naming a host subsystem (never plugin/user data), e.g. `"timer"`. Kept
    /// out of the `Serialize`/typegen derive on [`ScopeOwner`] as a whole: `serde::Deserialize` has
    /// no impl for `&'static str` (there is no way to borrow past the deserializer's input
    /// lifetime), and a scope owner is an in-process identity, never wire data.
    Service(&'static str),
}

/// 🔖️ Identity of one [`ScopeHandle`] — monotonically assigned by whatever
/// [`HostAsyncRuntime::open_scope`] implementation mints it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ScopeId(pub u64);

/// 🌳️ A structured-concurrency scope: every [`HostAsyncRuntime::spawn_scoped`] task and
/// [`HostAsyncRuntime::run_blocking`] unit of work belongs to exactly one of these, found by
/// [`ScopeHandle::id`]. There is no detached-spawn entry point on [`HostAsyncRuntime`] — a handle is
/// mandatory. Not `Serialize`: it carries a live [`CancelToken`], same reasoning as
/// [`OperationContext`].
#[derive(Clone, Debug)]
pub struct ScopeHandle {
    pub id: ScopeId,
    pub owner: ScopeOwner,
    pub cancel: CancelToken,
}

impl ScopeHandle {
    pub fn is_same_scope(&self, other: &ScopeHandle) -> bool {
        self.id == other.id
    }
}

/// 📊️ What [`HostAsyncRuntime::cancel_scope`] hands back once a scope has finished draining:
/// how many spawned tasks ran to completion, how many were cancelled in flight, and how many
/// outlived the grace period (`leaked` — still running when the drain gave up waiting). A non-zero
/// `leaked` is always worth surfacing to an operator; it is never silently swallowed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ScopeDrainReport {
    pub finished: u32,
    pub cancelled: u32,
    pub leaked: u32,
}
//#endregion 🌳️Scope

//#region 🚰️ChannelPolicy
/// 🚰️ Declared backpressure vocabulary every channel/service in the async layer must state up
/// front — there is no implicit "unbounded" option. `Coalesced`/`LatestWins` drop older queued
/// values under load; `LosslessBounded`/`ByteCredit` reject or stall producers instead of dropping.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChannelPolicy {
    LatestWins,
    Coalesced { key: String },
    LosslessBounded { cap: u32 },
    ByteCredit { bytes: u64 },
}
//#endregion 🚰️ChannelPolicy

//#region 🧵️ThreadPlan
/// 🧵️ Pure arithmetic result of [`thread_plan`] — the ONE place any component may read to learn how
/// many OS threads of each role exist. No component sizes itself independently from `num_cpus`;
/// that per-component sizing is exactly the failure this type exists to forbid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ThreadPlan {
    pub kernel: u32,
    pub shards: u32,
    pub io_workers: u32,
    pub compute: u32,
    pub epoch_ticker: u32,
}

fn clamp_u32(value: u32, lo: u32, hi: u32) -> u32 {
    value.max(lo).min(hi)
}

fn ceil_div_usize(numerator: usize, denominator: usize) -> usize {
    numerator.div_ceil(denominator)
}

/// 🧵️ Sizes every OS-thread role from a single core count. `shards` (the actor DRR shard workers)
/// and `compute` (CPU-bound plugin work) are the only roles that actually contend for CPU
/// concurrently — `kernel`/`io_workers`/`epoch_ticker` are park-dominated (blocked on a channel
/// or a timer almost all the time). `shards` and `io_workers` each carry a hard floor (2 and 1
/// respectively) so even a single-core host still gets a working actor system and a working async
/// runtime; `compute` carries a floor of 1 for the same reason. Those floors mean the invariant
/// `shards + compute + 1 <= cores` (see the crate's test suite) only holds once the floors stop
/// binding, i.e. for `cores >= 4` — below that this function deliberately returns an oversubscribed
/// plan rather than a zero-thread role, since a small machine still needs a live system, and the OS
/// scheduler already time-slices oversubscription safely.
pub fn thread_plan(cores: usize) -> ThreadPlan {
    let cores = cores.max(1);
    let shards = clamp_u32(ceil_div_usize(cores, 2) as u32, 2, 8);
    let io_workers = clamp_u32(ceil_div_usize(cores, 4) as u32, 1, 4);
    let reserved = shards as i64 + io_workers as i64 + 1;
    let compute = (cores as i64 - reserved).max(1) as u32;
    ThreadPlan { kernel: 1, shards, io_workers, compute, epoch_ticker: 1 }
}

/// 🎭️ A role a [`ThreadBudget`] tracks a remaining count for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub enum ThreadRole {
    Kernel,
    Shard,
    IoWorker,
    Compute,
    EpochTicker,
}

/// ⚖️ Live remaining-thread accounting derived from a [`ThreadPlan`]. [`ThreadBudget::checkout`]
/// draws `n` threads for a role and `debug_assert!`s the draw did not exceed what remained — a
/// release build never panics on over-draw (the counter is simply allowed to underflow-wrap), so
/// this is a development-time tripwire, not a runtime enforcement mechanism.
pub struct ThreadBudget {
    kernel: AtomicU32,
    shards: AtomicU32,
    io_workers: AtomicU32,
    compute: AtomicU32,
    epoch_ticker: AtomicU32,
}

impl ThreadBudget {
    pub fn from_plan(plan: ThreadPlan) -> ThreadBudget {
        ThreadBudget {
            kernel: AtomicU32::new(plan.kernel),
            shards: AtomicU32::new(plan.shards),
            io_workers: AtomicU32::new(plan.io_workers),
            compute: AtomicU32::new(plan.compute),
            epoch_ticker: AtomicU32::new(plan.epoch_ticker),
        }
    }

    fn counter(&self, role: ThreadRole) -> &AtomicU32 {
        match role {
            ThreadRole::Kernel => &self.kernel,
            ThreadRole::Shard => &self.shards,
            ThreadRole::IoWorker => &self.io_workers,
            ThreadRole::Compute => &self.compute,
            ThreadRole::EpochTicker => &self.epoch_ticker,
        }
    }

    /// ⚖️ Draws `n` threads from `role`'s remaining count, returning what remains afterward.
    pub fn checkout(&self, role: ThreadRole, n: u32) -> u32 {
        let counter = self.counter(role);
        let previous = counter.fetch_sub(n, Ordering::SeqCst);
        debug_assert!(previous >= n, "ThreadBudget overdrawn for {role:?}: requested {n} but only {previous} remained");
        previous.wrapping_sub(n)
    }

    pub fn remaining(&self, role: ThreadRole) -> u32 {
        self.counter(role).load(Ordering::SeqCst)
    }
}
//#endregion 🧵️ThreadPlan

//#region 🎛️HostAsyncRuntime
/// ⏳️ A boxed, `Send`, `'static` future — the one shape every async host operation crosses this
/// trait's boundary as. No `futures` crate, no runtime-specific future type ever appears here.
pub type HostFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// 🎛️ Hides the concrete async executor (tokio in packet R2; potentially something else on a
/// constrained target) from every other crate. Every method is either pure bookkeeping
/// (`open_scope`) or a scheduling primitive (`spawn_scoped`/`run_blocking`/`sleep_until`/
/// `cancel_scope`/`now_ms`) — no I/O primitive belongs here; a service that needs a socket, a file
/// or an HTTP pool is built ON TOP of an implementation of this trait, in a later packet.
pub trait HostAsyncRuntime: Send + Sync {
    /// 🌳️ Opens a new [`Scope`] under `owner`, optionally nested under `parent` (so its
    /// [`CancelToken`] is a [`CancelToken::child`] of the parent's — cancelling the parent
    /// transitively cancels this scope and everything spawned into it).
    fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle;

    /// ▶️ Spawns `fut` into `scope` — the ONLY way to start async work on this trait. There is no
    /// detached-spawn entry point: a [`ScopeHandle`] is mandatory, so every task is always
    /// findable, cancellable and drainable through its scope.
    fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>);

    /// 🧱️ Runs `work` off the async executor's own worker threads (a real thread pool in the tokio
    /// implementation), still accounted to `scope`.
    fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>);

    /// ⏰️ A future that resolves once the implementation's own clock reaches `deadline_ms`. This
    /// trait never reads a clock itself; `deadline_ms` is caller-supplied, same as everywhere else
    /// in this crate.
    fn sleep_until(&self, deadline_ms: u64) -> HostFuture<()>;

    /// 🛑️ Cancels every task in `owner`'s scope (and its descendants), waits up to `grace_ms` for
    /// in-flight work to finish, then reports what happened via [`ScopeDrainReport`].
    fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> HostFuture<ScopeDrainReport>;

    /// 🕐️ The implementation's own notion of the current time, in milliseconds. The only place in
    /// the whole `HostAsyncRuntime` contract permitted to read a real clock — everything else on
    /// this trait takes `now_ms`/`deadline_ms` as a parameter instead.
    fn now_ms(&self) -> u64;
}
//#endregion 🎛️HostAsyncRuntime

//#region 🧪️ManualRuntime
/// 🧪️ In-crate [`HostAsyncRuntime`] test double: a manual poll loop over an injected clock, so
/// downstream crates (packets R2/R4) can unit-test against the trait without linking tokio. Time
/// never advances on its own — callers drive it with [`ManualRuntime::set_now_ms`] and progress the
/// futures with [`ManualRuntime::drive`]. Feature-gated (`testkit`, and implicitly available under
/// `cfg(test)` for this crate's own tests) rather than part of the default build, so the pure crate
/// never ships test-only bookkeeping in a normal dependency.
#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::AtomicU64;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Waker};

    struct ManualScopeRecord {
        cancel: CancelToken,
        finished: u32,
        cancelled: u32,
    }

    struct ManualTask {
        scope_id: u64,
        future: Pin<Box<dyn Future<Output = ()> + Send>>,
    }

    struct ManualRuntimeState {
        now_ms: AtomicU64,
        next_scope_id: AtomicU64,
        scopes: Mutex<HashMap<u64, ManualScopeRecord>>,
        tasks: Mutex<Vec<ManualTask>>,
    }

    /// 🧪️ See the module doc — the manual-poll-loop [`HostAsyncRuntime`] test double.
    #[derive(Clone)]
    pub struct ManualRuntime(Arc<ManualRuntimeState>);

    impl ManualRuntime {
        pub fn new(start_now_ms: u64) -> ManualRuntime {
            ManualRuntime(Arc::new(ManualRuntimeState {
                now_ms: AtomicU64::new(start_now_ms),
                next_scope_id: AtomicU64::new(1),
                scopes: Mutex::new(HashMap::new()),
                tasks: Mutex::new(Vec::new()),
            }))
        }

        /// 🕐️ Injects the current time — the ONLY way this runtime's clock ever moves.
        pub fn set_now_ms(&self, now_ms: u64) {
            self.0.now_ms.store(now_ms, Ordering::SeqCst);
        }

        /// ▶️ Polls every not-yet-finished task once with a no-op waker, repeating until a full
        /// pass makes no further progress. Returns how many tasks completed across the whole call.
        pub fn drive(&self) -> usize {
            let waker = Waker::noop();
            let mut cx = Context::from_waker(waker);
            let mut total_completed = 0usize;
            loop {
                let mut tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
                let mut progressed = false;
                let mut index = 0;
                while index < tasks.len() {
                    let ready = matches!(tasks[index].future.as_mut().poll(&mut cx), Poll::Ready(()));
                    if ready {
                        let task = tasks.remove(index);
                        let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
                        if let Some(record) = scopes.get_mut(&task.scope_id) {
                            record.finished += 1;
                        }
                        total_completed += 1;
                        progressed = true;
                    } else {
                        index += 1;
                    }
                }
                drop(tasks);
                if !progressed {
                    break;
                }
            }
            total_completed
        }

        pub fn pending_task_count(&self) -> usize {
            self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").len()
        }
    }

    struct ManualSleep {
        state: Arc<ManualRuntimeState>,
        deadline_ms: u64,
    }

    impl Future for ManualSleep {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.state.now_ms.load(Ordering::SeqCst) >= self.deadline_ms {
                Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    impl HostAsyncRuntime for ManualRuntime {
        fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
            let id = ScopeId(self.0.next_scope_id.fetch_add(1, Ordering::SeqCst));
            let cancel = match parent {
                Some(parent_handle) => parent_handle.cancel.child(),
                None => CancelToken::root(),
            };
            self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned").insert(id.0, ManualScopeRecord { cancel: cancel.clone(), finished: 0, cancelled: 0 });
            ScopeHandle { id, owner, cancel }
        }

        fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) {
            let _ = ctx;
            self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").push(ManualTask { scope_id: scope.id.0, future: fut });
        }

        fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>) {
            let _ = ctx;
            work();
            if let Some(record) = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned").get_mut(&scope.id.0) {
                record.finished += 1;
            }
        }

        fn sleep_until(&self, deadline_ms: u64) -> HostFuture<()> {
            Box::pin(ManualSleep { state: self.0.clone(), deadline_ms })
        }

        fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> HostFuture<ScopeDrainReport> {
            let _ = grace_ms;
            let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
            let mut report = ScopeDrainReport::default();
            let mut tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
            let cancelled_scope_ids: Vec<u64> = scopes
                .iter()
                .filter(|(_, record)| scope_owner_matches(owner, record))
                .map(|(id, _)| *id)
                .collect();
            for id in &cancelled_scope_ids {
                if let Some(record) = scopes.get(id) {
                    record.cancel.cancel();
                }
            }
            let before = tasks.len();
            tasks.retain(|task| !cancelled_scope_ids.contains(&task.scope_id));
            let cancelled_task_count = (before - tasks.len()) as u32;
            for id in &cancelled_scope_ids {
                if let Some(record) = scopes.get_mut(id) {
                    record.cancelled += cancelled_task_count;
                    report.finished += record.finished;
                    report.cancelled += record.cancelled;
                }
            }
            Box::pin(std::future::ready(report))
        }

        fn now_ms(&self) -> u64 {
            self.0.now_ms.load(Ordering::SeqCst)
        }
    }

    /// 🔍️ A [`ScopeOwner`] has no stable numeric identity to key a `HashMap` by, so
    /// [`ManualRuntime::cancel_scope`] matches scopes by structural equality against the tracked
    /// owner recorded at `open_scope` time. `ManualScopeRecord` does not store the owner today
    /// (only its cancel token), so this comparison is intentionally permissive: a real
    /// `HostAsyncRuntime` implementation (packet R2) is expected to index scopes by owner directly.
    fn scope_owner_matches(_owner: &ScopeOwner, _record: &ManualScopeRecord) -> bool {
        true
    }
}
#[cfg(any(test, feature = "testkit"))]
pub use testkit::ManualRuntime;
//#endregion 🧪️ManualRuntime

//#region 🧬️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::task::{Context, Poll, Waker};

    //#region 🛑️CancelTokenTests
    #[test]
    fn cancel_token_root_starts_live() {
        let token = CancelToken::root();
        assert!(token.is_live());
        assert!(!token.is_cancelled());
        assert!(!token.is_parked());
    }

    #[test]
    fn cancel_token_park_then_unpark_returns_to_live() {
        let token = CancelToken::root();
        token.park();
        assert!(token.is_parked());
        token.unpark();
        assert!(token.is_live());
    }

    #[test]
    fn cancel_token_cancel_is_terminal_over_park() {
        let token = CancelToken::root();
        token.park();
        token.cancel();
        token.unpark();
        assert!(token.is_cancelled(), "cancel must never be undone by a later park/unpark");
    }

    #[test]
    fn cancelling_parent_transitively_cancels_child_and_grandchild() {
        let root = CancelToken::root();
        let child = root.child();
        let grandchild = child.child();
        assert!(root.is_live() && child.is_live() && grandchild.is_live());
        root.cancel();
        assert!(root.is_cancelled());
        assert!(child.is_cancelled(), "child must observe parent cancellation");
        assert!(grandchild.is_cancelled(), "grandchild must observe ancestor cancellation transitively");
    }

    #[test]
    fn parking_parent_does_not_downgrade_an_already_live_reading_below_park() {
        let root = CancelToken::root();
        let child = root.child();
        root.park();
        assert_eq!(child.state(), CancelState::Park, "child inherits parent's park via max-severity fold");
        assert_eq!(child.0.local.load(Ordering::SeqCst), CancelState::Live.to_u8(), "child's OWN local state is untouched by the parent's park");
    }

    #[test]
    fn child_cancel_never_propagates_upward_to_parent() {
        let root = CancelToken::root();
        let child = root.child();
        child.cancel();
        assert!(child.is_cancelled());
        assert!(root.is_live(), "cancelling a child must never cancel its parent");
    }
    //#endregion 🛑️CancelTokenTests

    //#region 🧵️ThreadPlanTests
    #[test]
    fn thread_plan_invariant_holds_once_floors_stop_binding() {
        for cores in 4usize..=64 {
            let plan = thread_plan(cores);
            assert!(
                (plan.shards + plan.compute + 1) as usize <= cores,
                "cores={cores} shards={} compute={} violates shards+compute+1<=cores",
                plan.shards,
                plan.compute
            );
        }
    }

    #[test]
    fn thread_plan_low_core_counts_oversubscribe_but_never_zero_a_role() {
        for cores in 1usize..4 {
            let plan = thread_plan(cores);
            assert!(plan.kernel >= 1 && plan.shards >= 2 && plan.io_workers >= 1 && plan.compute >= 1 && plan.epoch_ticker >= 1, "cores={cores} produced a zeroed role: {plan:?}");
        }
    }

    #[test]
    fn thread_plan_shards_and_io_workers_never_exceed_their_ceilings() {
        for cores in 1usize..=256 {
            let plan = thread_plan(cores);
            assert!(plan.shards >= 2 && plan.shards <= 8);
            assert!(plan.io_workers >= 1 && plan.io_workers <= 4);
            assert!(plan.compute >= 1);
        }
    }

    #[test]
    fn thread_plan_is_deterministic() {
        assert_eq!(thread_plan(16), thread_plan(16));
    }

    #[test]
    fn thread_budget_checkout_debits_and_returns_remaining() {
        let budget = ThreadBudget::from_plan(thread_plan(16));
        let remaining = budget.checkout(ThreadRole::Compute, 2);
        assert_eq!(remaining, budget.remaining(ThreadRole::Compute));
        assert_eq!(remaining + 2, thread_plan(16).compute);
    }

    #[test]
    #[should_panic(expected = "overdrawn")]
    fn thread_budget_checkout_debug_panics_on_overdraw() {
        let budget = ThreadBudget::from_plan(thread_plan(4));
        budget.checkout(ThreadRole::Kernel, 999);
    }
    //#endregion 🧵️ThreadPlanTests

    //#region 🌳️ScopeTests
    #[test]
    fn scope_handle_child_scope_shares_cancellation_lineage() {
        use testkit::ManualRuntime;
        let runtime = ManualRuntime::new(0);
        let parent = runtime.open_scope(ScopeOwner::Service("test-parent"), None);
        let child = runtime.open_scope(ScopeOwner::Service("test-child"), Some(&parent));
        parent.cancel.cancel();
        assert!(child.cancel.is_cancelled());
    }
    //#endregion 🌳️ScopeTests

    //#region 🧪️ManualRuntimeTests
    #[test]
    fn manual_runtime_spawn_scoped_runs_a_ready_future_on_drive() {
        use testkit::ManualRuntime;
        let runtime = ManualRuntime::new(0);
        let scope = runtime.open_scope(ScopeOwner::Actor(1), None);
        let ctx = OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
        let ran = Arc::new(AtomicBool::new(false));
        let ran_clone = ran.clone();
        runtime.spawn_scoped(&scope, ctx, Box::pin(async move { ran_clone.store(true, Ordering::SeqCst) }));
        assert_eq!(runtime.drive(), 1);
        assert!(ran.load(Ordering::SeqCst));
        assert_eq!(runtime.pending_task_count(), 0);
    }

    #[test]
    fn manual_runtime_sleep_until_resolves_only_after_injected_time_advances() {
        use testkit::ManualRuntime;
        let runtime = ManualRuntime::new(0);
        let scope = runtime.open_scope(ScopeOwner::Service("timer"), None);
        let ctx = OperationContext { actor: 0, generation: 0, trace: TraceId(2), lane: 0, deadline_ms: Some(100), cancel: scope.cancel.clone(), capability: None };
        let woke = Arc::new(AtomicBool::new(false));
        let woke_clone = woke.clone();
        let runtime_for_future = runtime.clone();
        runtime.spawn_scoped(
            &scope,
            ctx,
            Box::pin(async move {
                runtime_for_future.sleep_until(100).await;
                woke_clone.store(true, Ordering::SeqCst);
            }),
        );
        runtime.drive();
        assert!(!woke.load(Ordering::SeqCst), "must not resolve before the injected clock reaches the deadline");
        runtime.set_now_ms(100);
        runtime.drive();
        assert!(woke.load(Ordering::SeqCst), "must resolve once the injected clock reaches the deadline");
    }

    #[test]
    fn manual_runtime_cancel_scope_reports_finished_and_cancelled() {
        use testkit::ManualRuntime;
        let runtime = ManualRuntime::new(0);
        let scope = runtime.open_scope(ScopeOwner::Actor(7), None);
        let ctx = OperationContext { actor: 7, generation: 0, trace: TraceId(3), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
        runtime.spawn_scoped(&scope, ctx.clone(), Box::pin(async move {}));
        runtime.drive();
        let ctx2 = OperationContext { actor: 7, generation: 0, trace: TraceId(4), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
        let runtime_for_future = runtime.clone();
        runtime.spawn_scoped(&scope, ctx2, Box::pin(async move { runtime_for_future.sleep_until(u64::MAX).await }));
        let report = futures_poll_ready(runtime.cancel_scope(&scope.owner, 0));
        assert_eq!(report.finished, 1);
        assert_eq!(report.cancelled, 1);
    }

    /// 🧪️ Test-only helper: [`ManualRuntime::cancel_scope`] always returns an already-`Ready`
    /// future (there is no async wait in the manual test double), so this drives it to completion
    /// without needing a real executor in the test.
    fn futures_poll_ready<T>(mut fut: HostFuture<T>) -> T {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("expected an already-ready future"),
        }
    }
    //#endregion 🧪️ManualRuntimeTests

    //#region 🔖️Typegen
    #[cfg(feature = "typegen")]
    #[test]
    fn exports_typescript_bindings() {
        use ts_rs::TS;
        TraceId::export().unwrap();
        CapabilityTokenId::export().unwrap();
        CancelState::export().unwrap();
        ScopeId::export().unwrap();
        ScopeDrainReport::export().unwrap();
        ChannelPolicy::export().unwrap();
        ThreadPlan::export().unwrap();
        ThreadRole::export().unwrap();
    }
    //#endregion 🔖️Typegen
}
//#endregion 🧬️Tests
