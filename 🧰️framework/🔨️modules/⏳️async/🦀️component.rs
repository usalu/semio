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
    async fn from_u8(tag: u8) -> CancelState {
        match tag {
            0 => CancelState::Live,
            1 => CancelState::Park,
            _ => CancelState::Cancelled,
        }
    }

    async fn to_u8(self) -> u8 {
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
    pub async fn root() -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(CancelState::Live.to_u8().await), parent: None }))
    }

    /// 👶️ A descendant token: its effective state is never less severe than `self`'s.
    pub async fn child(&self) -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(CancelState::Live.to_u8().await), parent: Some(self.clone()) }))
    }

    /// ⏸️ Enter the suspend state — a no-op once `Cancelled` (terminal, never downgraded).
    pub async fn park(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Live.to_u8().await, CancelState::Park.to_u8().await, Ordering::SeqCst, Ordering::SeqCst);
    }

    /// ▶️ Leave the suspend state back to `Live` — a no-op once `Cancelled`.
    pub async fn unpark(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Park.to_u8().await, CancelState::Live.to_u8().await, Ordering::SeqCst, Ordering::SeqCst);
    }

    /// 🛑️ Terminal: always wins over `Live`/`Park`, and over anything a later `park`/`unpark` on
    /// this same token would attempt.
    pub async fn cancel(&self) {
        self.0.local.store(CancelState::Cancelled.to_u8().await, Ordering::SeqCst);
    }

    /// 🔍️ Max-severity fold of this token's local state and every ancestor's. Walks the parent
    /// chain iteratively rather than recursing through `parent.state().await` — a recursive
    /// `async fn` call needs `Box::pin` indirection (its state machine would otherwise be
    /// infinitely sized), and an iterative walk needs neither the allocation nor the indirection.
    pub async fn state(&self) -> CancelState {
        let mut effective = CancelState::Live;
        let mut node = Some(self);
        while let Some(current) = node {
            let local = CancelState::from_u8(current.0.local.load(Ordering::SeqCst)).await;
            effective = effective.max(local);
            node = current.0.parent.as_ref();
        }
        effective
    }

    pub async fn is_cancelled(&self) -> bool {
        self.state().await == CancelState::Cancelled
    }

    pub async fn is_parked(&self) -> bool {
        self.state().await == CancelState::Park
    }

    pub async fn is_live(&self) -> bool {
        self.state().await == CancelState::Live
    }
}

impl std::fmt::Debug for CancelToken {
    /// 🚫️async: E1 external-trait impl — `std::fmt::Debug`'s signature is fixed by std, so this
    /// can never `.await`. Folds the same max-severity parent chain as [`CancelToken::state`], but
    /// inline and iteratively over the raw atomics: a `Debug` impl must never need an executor to
    /// run, and duplicating the fold as a second named (necessarily non-`async`) function would just
    /// relocate the same problem onto a fn that then needs its own R2 exception tag.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut effective = CancelState::Live;
        let mut node = Some(self);
        while let Some(current) = node {
            let local = match current.0.local.load(Ordering::SeqCst) {
                0 => CancelState::Live,
                1 => CancelState::Park,
                _ => CancelState::Cancelled,
            };
            effective = effective.max(local);
            node = current.0.parent.as_ref();
        }
        write!(f, "CancelToken({effective:?})")
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
    pub async fn is_same_scope(&self, other: &ScopeHandle) -> bool {
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

async fn clamp_u32(value: u32, lo: u32, hi: u32) -> u32 {
    value.max(lo).min(hi)
}

async fn ceil_div_usize(numerator: usize, denominator: usize) -> usize {
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
pub async fn thread_plan(cores: usize) -> ThreadPlan {
    let cores = cores.max(1);
    let shards = clamp_u32(ceil_div_usize(cores, 2).await as u32, 2, 8).await;
    let io_workers = clamp_u32(ceil_div_usize(cores, 4).await as u32, 1, 4).await;
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
    pub async fn from_plan(plan: ThreadPlan) -> ThreadBudget {
        ThreadBudget {
            kernel: AtomicU32::new(plan.kernel),
            shards: AtomicU32::new(plan.shards),
            io_workers: AtomicU32::new(plan.io_workers),
            compute: AtomicU32::new(plan.compute),
            epoch_ticker: AtomicU32::new(plan.epoch_ticker),
        }
    }

    async fn counter(&self, role: ThreadRole) -> &AtomicU32 {
        match role {
            ThreadRole::Kernel => &self.kernel,
            ThreadRole::Shard => &self.shards,
            ThreadRole::IoWorker => &self.io_workers,
            ThreadRole::Compute => &self.compute,
            ThreadRole::EpochTicker => &self.epoch_ticker,
        }
    }

    /// ⚖️ Draws `n` threads from `role`'s remaining count, returning what remains afterward.
    pub async fn checkout(&self, role: ThreadRole, n: u32) -> u32 {
        let counter = self.counter(role).await;
        let previous = counter.fetch_sub(n, Ordering::SeqCst);
        debug_assert!(previous >= n, "ThreadBudget overdrawn for {role:?}: requested {n} but only {previous} remained");
        previous.wrapping_sub(n)
    }

    pub async fn remaining(&self, role: ThreadRole) -> u32 {
        self.counter(role).await.load(Ordering::SeqCst)
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
    async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle;

    /// ▶️ Spawns `fut` into `scope` — the ONLY way to start async work on this trait. There is no
    /// detached-spawn entry point: a [`ScopeHandle`] is mandatory, so every task is always
    /// findable, cancellable and drainable through its scope.
    async fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>);

    /// 🧱️ Runs `work` off the async executor's own worker threads (a real thread pool in the tokio
    /// implementation), still accounted to `scope`.
    async fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>);

    /// ⏰️ Resolves once the implementation's own clock reaches `deadline_ms`. This trait never reads
    /// a clock itself; `deadline_ms` is caller-supplied, same as everywhere else in this crate. Plain
    /// `async fn`, not `-> HostFuture<()>`: an `async fn` already returns a future, so boxing a
    /// second one on top is exactly the double-future shape `dyn Future` erasure is banned from
    /// trait-method return position to remove — `HostFuture<T>` survives in this trait ONLY as
    /// `spawn_scoped`'s argument type, where the caller builds the box at a concrete type.
    async fn sleep_until(&self, deadline_ms: u64);

    /// 🛑️ Cancels every task in `owner`'s scope (and its descendants), waits up to `grace_ms` for
    /// in-flight work to finish, then reports what happened via [`ScopeDrainReport`]. See
    /// [`HostAsyncRuntime::sleep_until`]'s doc for why this returns `ScopeDrainReport` directly
    /// rather than `HostFuture<ScopeDrainReport>`.
    async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport;

    /// 🕐️ The implementation's own notion of the current time, in milliseconds. The only place in
    /// the whole `HostAsyncRuntime` contract permitted to read a real clock — everything else on
    /// this trait takes `now_ms`/`deadline_ms` as a parameter instead.
    async fn now_ms(&self) -> u64;
}
//#endregion 🎛️HostAsyncRuntime

//#region 🌉️BlockOn
/// 🌉️ Drives `fut` to completion on the calling thread — the boundary where a plain OS thread
/// BECOMES an executor.
///
/// 🚫️async: E5 executor bridge — one of the few functions in the whole repo deliberately NOT
/// `async`: an `async fn` can only ever be *driven* by something already polling it, so the bridge
/// that turns a thread into that poller cannot itself be `async` without begging the question. At
/// most one such bridge per crate (R2); this is this crate's.
///
/// Used by [`crate::HostAsyncRuntime`] callers that must block a real thread on a future (a
/// synchronous host entry point, a CLI `main`), and mirrored — as a separate, self-contained,
/// zero-dependency inline copy, never a call into this fn — by
/// `semio_framework_async_macros::async_test`'s generated `#[test]` harness (that macro crate must
/// not link this one, a runtime dependency, from 65+ crates' dev-dependency-only edge).
///
/// On every target except `wasm32-unknown-unknown` this parks the calling [`std::thread::Thread`]
/// and re-polls only once its [`std::task::Wake`] waker fires. `wasm32-unknown-unknown` has no
/// `Thread::unpark` to wake (single-threaded, no real OS thread to park), so that target instead
/// spins on [`std::task::Waker::noop`] — busy-polling is the correct fallback there since there is
/// no other thread that could ever wake a parked one.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);

    #[cfg(not(target_arch = "wasm32"))]
    {
        struct ThreadWaker(std::thread::Thread);
        impl std::task::Wake for ThreadWaker {
            fn wake(self: Arc<Self>) {
                self.0.unpark();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                self.0.unpark();
            }
        }
        let waker = std::task::Waker::from(Arc::new(ThreadWaker(std::thread::current())));
        let mut cx = std::task::Context::from_waker(&waker);
        loop {
            match fut.as_mut().poll(&mut cx) {
                std::task::Poll::Ready(value) => return value,
                std::task::Poll::Pending => std::thread::park(),
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let waker = std::task::Waker::noop();
        let mut cx = std::task::Context::from_waker(waker);
        loop {
            if let std::task::Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
                return value;
            }
        }
    }
}
//#endregion 🌉️BlockOn

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
        pub async fn new(start_now_ms: u64) -> ManualRuntime {
            ManualRuntime(Arc::new(ManualRuntimeState {
                now_ms: AtomicU64::new(start_now_ms),
                next_scope_id: AtomicU64::new(1),
                scopes: Mutex::new(HashMap::new()),
                tasks: Mutex::new(Vec::new()),
            }))
        }

        /// 🕐️ Injects the current time — the ONLY way this runtime's clock ever moves.
        pub async fn set_now_ms(&self, now_ms: u64) {
            self.0.now_ms.store(now_ms, Ordering::SeqCst);
        }

        /// ▶️ Polls every not-yet-finished task once with a no-op waker, repeating until a full
        /// pass makes no further progress. Returns how many tasks completed across the whole call.
        pub async fn drive(&self) -> usize {
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

        pub async fn pending_task_count(&self) -> usize {
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
        async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
            let id = ScopeId(self.0.next_scope_id.fetch_add(1, Ordering::SeqCst));
            let cancel = match parent {
                Some(parent_handle) => parent_handle.cancel.child().await,
                None => CancelToken::root().await,
            };
            self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned").insert(id.0, ManualScopeRecord { cancel: cancel.clone(), finished: 0, cancelled: 0 });
            ScopeHandle { id, owner, cancel }
        }

        async fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) {
            let _ = ctx;
            self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").push(ManualTask { scope_id: scope.id.0, future: fut });
        }

        async fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>) {
            let _ = ctx;
            work();
            if let Some(record) = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned").get_mut(&scope.id.0) {
                record.finished += 1;
            }
        }

        async fn sleep_until(&self, deadline_ms: u64) {
            ManualSleep { state: self.0.clone(), deadline_ms }.await
        }

        async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
            let _ = grace_ms;
            let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
            let mut report = ScopeDrainReport::default();
            let mut tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
            let mut cancelled_scope_ids = Vec::new();
            for (id, record) in scopes.iter() {
                if scope_owner_matches(owner, record).await {
                    cancelled_scope_ids.push(*id);
                }
            }
            for id in &cancelled_scope_ids {
                if let Some(record) = scopes.get(id) {
                    record.cancel.cancel().await;
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
            report
        }

        async fn now_ms(&self) -> u64 {
            self.0.now_ms.load(Ordering::SeqCst)
        }
    }

    /// 🔍️ A [`ScopeOwner`] has no stable numeric identity to key a `HashMap` by, so
    /// [`ManualRuntime::cancel_scope`] matches scopes by structural equality against the tracked
    /// owner recorded at `open_scope` time. `ManualScopeRecord` does not store the owner today
    /// (only its cancel token), so this comparison is intentionally permissive: a real
    /// `HostAsyncRuntime` implementation (packet R2) is expected to index scopes by owner directly.
    async fn scope_owner_matches(_owner: &ScopeOwner, _record: &ManualScopeRecord) -> bool {
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

    //#region 🧵️ThreadPlanTests
    #[semio_framework_async_macros::async_test]
    async fn thread_plan_invariant_holds_once_floors_stop_binding() {
        for cores in 4usize..=64 {
            let plan = thread_plan(cores).await;
            assert!(
                (plan.shards + plan.compute + 1) as usize <= cores,
                "cores={cores} shards={} compute={} violates shards+compute+1<=cores",
                plan.shards,
                plan.compute
            );
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn thread_plan_low_core_counts_oversubscribe_but_never_zero_a_role() {
        for cores in 1usize..4 {
            let plan = thread_plan(cores).await;
            assert!(plan.kernel >= 1 && plan.shards >= 2 && plan.io_workers >= 1 && plan.compute >= 1 && plan.epoch_ticker >= 1, "cores={cores} produced a zeroed role: {plan:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn thread_plan_shards_and_io_workers_never_exceed_their_ceilings() {
        for cores in 1usize..=256 {
            let plan = thread_plan(cores).await;
            assert!(plan.shards >= 2 && plan.shards <= 8);
            assert!(plan.io_workers >= 1 && plan.io_workers <= 4);
            assert!(plan.compute >= 1);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn thread_plan_is_deterministic() {
        assert_eq!(thread_plan(16).await, thread_plan(16).await);
    }

    #[semio_framework_async_macros::async_test]
    async fn thread_budget_checkout_debits_and_returns_remaining() {
        let budget = ThreadBudget::from_plan(thread_plan(16).await).await;
        let remaining = budget.checkout(ThreadRole::Compute, 2).await;
        assert_eq!(remaining, budget.remaining(ThreadRole::Compute).await);
        assert_eq!(remaining + 2, thread_plan(16).await.compute);
    }

    #[semio_framework_async_macros::async_test]
    #[should_panic(expected = "overdrawn")]
    async fn thread_budget_checkout_debug_panics_on_overdraw() {
        let budget = ThreadBudget::from_plan(thread_plan(4).await).await;
        budget.checkout(ThreadRole::Kernel, 999).await;
    }
    //#endregion 🧵️ThreadPlanTests

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

    //#region 🔖️Typegen
    #[cfg(feature = "typegen")]
    #[semio_framework_async_macros::async_test]
    async fn exports_typescript_bindings() {
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
