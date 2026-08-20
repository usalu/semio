//! ⏳️ Async-runtime interface layer AND the process-wide CPU substrate: `OperationContext`/
//! `CancelToken`/`Scope`/`ChannelPolicy` vocabulary, the [`HostAsyncRuntime`] trait that hides the
//! concrete future-polling executor from every other framework crate, and — as of the INTERACTIVE-
//! JOB-RUNTIME-REFACTOR Phase 1 packet P1a — [`WorkerPool`], the ONE process-wide work-stealing
//! thread pool every subsystem now schedules CPU-bound and blocking work onto. No `tokio` in this
//! crate; the future-*polling* executor (tokio today, packet R2's sibling crate) still owns turning
//! a `Future` into progress, but the OS threads it runs ON, and every other subsystem's OS threads
//! (shard executors, DB actors, epoch tickers, HTTP fetch threads — see Phase 0's thread census),
//! collapse into this one pool. `now_ms` stays caller/implementation-supplied everywhere in the
//! vocabulary types below (`OperationContext`, `HostAsyncRuntime::now_ms`); [`WorkerPool`]'s OWN
//! internal idle-park/timer-wheel bookkeeping is the one exception — parking a real OS thread for a
//! bounded interval is a mechanism, not a caller-facing clock reading, exactly the same way
//! `semio_framework_trace::now_us` owns a clock for its own instrumentation without becoming
//! `HostAsyncRuntime`'s clock.
//!
//! 🪡 **Where tokio actually lives**: the future-polling reactor only, in a sibling crate (design
//! ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet R2; re-hosted onto
//! [`WorkerPool`] by Phase 1 packet P1b, `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/`).
//! [`ManualRuntime`] in this crate (behind the `testkit` feature) exists so downstream crates can
//! unit-test against [`HostAsyncRuntime`] without ever linking tokio.
//!
//! 🧭 **Scope discipline**: [`HostAsyncRuntime::spawn_scoped`] takes a `&ScopeHandle`, not an
//! ambient context — there is no detached-spawn entry point on this trait. Every unit of async work
//! belongs to a [`Scope`] that can be found, waited on and drained; "who owns this task" is always
//! answerable.
//!
//! 🚫 **Not on this trait**: raw I/O primitives (sockets, files). `run_blocking` was removed from
//! this trait in Phase 1 packet P1a — a caller that needs to run a blocking closure now submits it
//! to [`WorkerPool`] directly (on [`Lane::Io`] or [`Lane::Background`] as appropriate), the same
//! substrate every other lane uses, instead of a dedicated trait method.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`
//! and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📋️master.md` (Phase 1,
//! packet P1a) for the worker-pool redesign this file implements.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};

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
/// this crate never depends on the actor crate; [`Lane::from_context_lane`] converts it into this
/// crate's own [`Lane`] for [`WorkerPool::submit`]) and capability revocation all propagate through
/// the whole operation with one value. Deliberately NOT `Serialize`/`Deserialize`: [`CancelToken`] is
/// a live in-process handle (an `Arc`), not wire data — a context is passed by value within one host
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

/// 🌳️ A structured-concurrency scope: every [`HostAsyncRuntime::spawn_scoped`] task belongs to
/// exactly one of these, found by [`ScopeHandle::id`]. There is no detached-spawn entry point on
/// [`HostAsyncRuntime`] — a handle is mandatory. Not `Serialize`: it carries a live [`CancelToken`],
/// same reasoning as [`OperationContext`].
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
/// 🚰️ Declared backpressure vocabulary every channel/service/mailbox/stream in the async layer
/// must state up front — there is no implicit "unbounded" option. `LatestWins`/`Coalesced` drop
/// older queued values under load; `LosslessBounded`/`ByteCredit` reject or stall producers instead
/// of dropping. Phase 1 packet P1a requirement: EVERY variant bounds both item count and bytes —
/// `LatestWins` holds at most one slot so its item bound is implicitly `1` (no field needed);
/// every other variant carries an explicit `max_items` alongside `max_bytes`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChannelPolicy {
    LatestWins { max_bytes: u64 },
    Coalesced { key: String, max_items: u32, max_bytes: u64 },
    LosslessBounded { max_items: u32, max_bytes: u64 },
    ByteCredit { max_items: u32, max_bytes: u64 },
}
//#endregion 🚰️ChannelPolicy

//#region 🎛️HostAsyncRuntime
/// ⏳️ A boxed, `Send`, `'static` future — the one shape every async host operation crosses this
/// trait's boundary as. No `futures` crate, no runtime-specific future type ever appears here.
pub type HostFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// 🎛️ Hides the concrete future-polling executor (tokio today; packet R2) from every other crate.
/// `run_blocking` was REMOVED from this trait in Phase 1 packet P1a — a caller that needs to run a
/// blocking closure off the async task graph now submits it directly to [`WorkerPool`] (own a
/// [`Lane`], not a trait method); every remaining method here is either pure bookkeeping
/// (`open_scope`) or a scheduling primitive (`spawn_scoped`/`sleep_until`/`cancel_scope`/`now_ms`) —
/// no I/O primitive belongs here; a service that needs a socket, a file or an HTTP pool is built ON
/// TOP of an implementation of this trait plus [`WorkerPool`], in a later packet.
pub trait HostAsyncRuntime: Send + Sync {
    /// 🌳️ Opens a new [`Scope`] under `owner`, optionally nested under `parent` (so its
    /// [`CancelToken`] is a [`CancelToken::child`] of the parent's — cancelling the parent
    /// transitively cancels this scope and everything spawned into it).
    ///
    /// 🚫️async: R15 — declared as RPITIT with an explicit `Send` bound (not a literal `async fn`
    /// signature) so a generic `R: HostAsyncRuntime` caller can box this future into a
    /// `HostFuture<()>` (R1's sanctioned erased spawn channel). Every `impl` still provides this
    /// with a literal `async fn` body; only the trait declaration's shape changed. See R15 in this
    /// ticket's `📌️important.md` for the full ruling.
    fn open_scope<'a>(&'a self, owner: ScopeOwner, parent: Option<&'a ScopeHandle>) -> impl Future<Output = ScopeHandle> + Send + 'a;

    /// ▶️ Spawns `fut` into `scope` — the ONLY way to start async work on this trait. There is no
    /// detached-spawn entry point: a [`ScopeHandle`] is mandatory, so every task is always
    /// findable, cancellable and drainable through its scope.
    ///
    /// 🚫️async: R15, see [`HostAsyncRuntime::open_scope`]'s doc.
    fn spawn_scoped<'a>(&'a self, scope: &'a ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) -> impl Future<Output = ()> + Send + 'a;

    /// ⏰️ Resolves once the implementation's own clock reaches `deadline_ms`. This trait never reads
    /// a clock itself; `deadline_ms` is caller-supplied, same as everywhere else in this crate. A
    /// concrete implementation may delegate to [`WorkerPool::timer`]'s [`TimerWheel::sleep_until`].
    ///
    /// 🚫️async: R15, see [`HostAsyncRuntime::open_scope`]'s doc. `HostFuture<T>` still survives in
    /// this trait ONLY as `spawn_scoped`'s argument type, where the caller builds the box at a
    /// concrete type — this RPITIT declaration is not that shape, it has no body to box.
    fn sleep_until(&self, deadline_ms: u64) -> impl Future<Output = ()> + Send + '_;

    /// 🛑️ Cancels every task in `owner`'s scope (and its descendants), waits up to `grace_ms` for
    /// in-flight work to finish, then reports what happened via [`ScopeDrainReport`]. See
    /// [`HostAsyncRuntime::sleep_until`]'s doc for why this returns `ScopeDrainReport` directly
    /// rather than `HostFuture<ScopeDrainReport>`.
    ///
    /// 🚫️async: R15, see [`HostAsyncRuntime::open_scope`]'s doc.
    fn cancel_scope<'a>(&'a self, owner: &'a ScopeOwner, grace_ms: u64) -> impl Future<Output = ScopeDrainReport> + Send + 'a;

    /// 🕐️ The implementation's own notion of the current time, in milliseconds. The only place in
    /// the whole `HostAsyncRuntime` contract permitted to read a real clock — everything else on
    /// this trait takes `now_ms`/`deadline_ms` as a parameter instead.
    ///
    /// 🚫️async: R15, see [`HostAsyncRuntime::open_scope`]'s doc.
    fn now_ms(&self) -> impl Future<Output = u64> + Send + '_;
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
/// 🚪️ Phase 1 packet P1a: gated behind the `entrypoint` cargo feature (always available under
/// `cfg(test)` for this crate's own suite, same shape as `testkit`/[`ManualRuntime`]) — the ONLY
/// approved callers are process entry points (a CLI `main`) and test/testkit consumers that
/// explicitly opt in with `features = ["entrypoint"]`. Interactive-reachable code must never call
/// this: driving a future to completion on the calling thread is exactly the run-to-completion
/// shape the whole INTERACTIVE-JOB-RUNTIME-REFACTOR forbids on a thread that must stay responsive.
/// `semio_framework_async_macros::async_test`'s generated `#[test]` harness does NOT call this fn —
/// it carries its own self-contained, zero-dependency inline copy (that macro crate must not link
/// this one, a runtime dependency, from 65+ crates' dev-dependency-only edge) — so gating this fn
/// behind a feature never affects `#[async_test]` expansion.
///
/// On every target except `wasm32-unknown-unknown` this parks the calling [`std::thread::Thread`]
/// and re-polls only once its [`std::task::Wake`] waker fires. `wasm32-unknown-unknown` has no
/// `Thread::unpark` to wake (single-threaded, no real OS thread to park), so that target instead
/// spins on [`std::task::Waker::noop`] — busy-polling is the correct fallback there since there is
/// no other thread that could ever wake a parked one.
#[cfg(any(test, feature = "entrypoint"))]
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
        let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
        let mut cx = Context::from_waker(&waker);
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::park(),
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

//#region 🏭️ProcessKind
/// 🏭️ Which sizing rule [`worker_count_for`] applies. Made explicit at construction — never
/// inferred by guessing whether a UI is present — because a headless batch process (CLI conversion,
/// CI worker) wants every core for throughput, while an interactive native process must always
/// leave the OS/UI thread its own core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub enum ProcessKind {
    InteractiveNative,
    HeadlessBatch,
}

/// 🔢️ `max(1, cores-1)` for [`ProcessKind::InteractiveNative`] (the OS/UI thread keeps the
/// remaining logical core), `cores` for [`ProcessKind::HeadlessBatch`]. Never zero — a single-core
/// interactive host still gets one worker (see [`WorkerPool`]'s doc for how that worker stays
/// cooperative rather than starving the OS/UI thread).
pub fn worker_count_for(process_kind: ProcessKind, cores: usize) -> usize {
    let cores = cores.max(1);
    match process_kind {
        ProcessKind::InteractiveNative => cores.saturating_sub(1).max(1),
        ProcessKind::HeadlessBatch => cores,
    }
}
//#endregion 🏭️ProcessKind

//#region 🛣️Lane
/// 🛣️ How many [`Lane`] variants exist — the fixed width of every per-worker queue array.
const LANE_COUNT: usize = 6;

/// 🛣️ Logical scheduling lane on [`WorkerPool`], replacing the old per-role OS threads
/// (`ThreadPlan`'s `kernel`/`shards`/`io_workers`/`compute`/`epoch_ticker`, deleted in this packet).
/// `Interactive`/`UserVisible`/`Background`/`Maintenance` mirror `🎭️actor::Lane`'s discriminant
/// order and [`Lane::weight`] verbatim (this crate must not depend on the actor crate — see
/// [`OperationContext::lane`]'s doc — so the mirroring is a deliberate, documented duplication, not
/// a shared type); `Io` and `Timer` are new lanes for the OS threads Phase 0's census found with no
/// actor-crate analogue (HTTP fetch threads, DB storage blocking I/O, the epoch-ticker replacement).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub enum Lane {
    Interactive,
    UserVisible,
    Background,
    Maintenance,
    Io,
    Timer,
}

impl Lane {
    /// 🛣️ Every lane, in a fixed order used both for per-worker queue storage indexing (`as usize`)
    /// and as the deficit-round-robin scan order — DRR fairness does not depend on this order being
    /// priority-sorted, only on it being STABLE across scans.
    pub const ALL: [Lane; LANE_COUNT] = [Lane::Interactive, Lane::UserVisible, Lane::Background, Lane::Maintenance, Lane::Io, Lane::Timer];

    /// ⚖️ Deficit-round-robin service weight — verbatim `🎭️actor::Lane::weight` for the four
    /// mirrored variants (8/4/2/1); `Io`=4 (as latency-sensitive as `UserVisible` — a stalled I/O
    /// lane stalls whatever's awaiting it) and `Timer`=3 (between `UserVisible` and `Background` —
    /// prompt but not as urgent as directly-observed interactive/user-visible work). A lane with
    /// pending work but insufficient accrued deficit is skipped THIS round, not starved forever —
    /// see [`WorkerPool`]'s scheduler doc for the accrual rule.
    pub const fn weight(self) -> u32 {
        match self {
            Lane::Interactive => 8,
            Lane::UserVisible => 4,
            Lane::Io => 4,
            Lane::Timer => 3,
            Lane::Background => 2,
            Lane::Maintenance => 1,
        }
    }

    /// 🔢️ Storage index into a `[T; LANE_COUNT]` per-worker queue array.
    const fn index(self) -> usize {
        self as usize
    }

    /// 🔄️ Maps `OperationContext.lane`'s bare `u8` (mirroring `🎭️actor::Lane`'s 0..3 discriminant
    /// order) onto this crate's [`Lane`]. Values outside `0..=3` (this crate's `Io`/`Timer`, which
    /// have no `🎭️actor::Lane` analogue) fall back to [`Lane::Background`] — a caller that wants
    /// `Io`/`Timer` submits to [`WorkerPool::submit`] with that [`Lane`] directly rather than via an
    /// `OperationContext`.
    pub const fn from_context_lane(value: u8) -> Lane {
        match value {
            0 => Lane::Interactive,
            1 => Lane::UserVisible,
            2 => Lane::Background,
            3 => Lane::Maintenance,
            _ => Lane::Background,
        }
    }
}

/// 🚦️ Whether `lane` is subject to [`WorkerPool`]'s interactive-reserve admission control —
/// `Background`/`Maintenance` are the two lanes that may never occupy every worker while the reserve
/// is active. `Io`/`Timer` are deliberately excluded: a stalled I/O or timer lane can itself block
/// interactive work waiting on it, so throttling them would be self-defeating. Native-only: the
/// wasm cooperative pool has exactly one logical worker, where admission control is a documented
/// no-op (see [`WorkerPool::pump`]'s doc), so this fn would otherwise be dead code on that target.
#[cfg(not(target_arch = "wasm32"))]
const fn is_low_priority(lane: Lane) -> bool {
    matches!(lane, Lane::Background | Lane::Maintenance)
}
//#endregion 🛣️Lane

//#region ⚖️PermitLedger
/// 🚫️ What [`PermitLedger::checkout`] returns on over-allocation — CHECKED in every build
/// (including release): the counter never wraps. Replaces `ThreadBudget::checkout`'s release-mode
/// `debug_assert!`-only tripwire (Phase 0 gate report finding: `previous.wrapping_sub(n)` silently
/// wraps to a huge value in release), this packet's explicit exit-gate item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PermitError {
    pub requested: u32,
    pub remaining: u32,
}

impl std::fmt::Display for PermitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PermitLedger over-allocated: requested {} permits but only {} remained", self.requested, self.remaining)
    }
}

impl std::error::Error for PermitError {}

/// ⚖️ Checked permit accounting, replacing `ThreadBudget`. Unlike the old per-role `ThreadBudget`
/// (five separate counters, one per OS-thread role), a [`PermitLedger`] tracks ONE pool of permits —
/// [`WorkerPool`] owns one sized to its worker count, and any other subsystem that wants a bounded
/// concurrent-admission gate (a future I/O connection pool, a plugin blocking-call limiter) can size
/// its own independently. [`PermitLedger::checkout`] uses a checked compare-exchange loop — never a
/// bare `fetch_sub` — so over-allocation is always observable as `Err`, in every build profile.
/// [`PermitLedger::occupancy`] feeds `semio_framework_trace::PermitLedger` so the trace module can
/// read live occupancy without this crate depending on trace's internals beyond that one type.
#[derive(Debug)]
pub struct PermitLedger {
    remaining: AtomicU32,
    trace_permits: semio_framework_trace::PermitLedger,
}

impl PermitLedger {
    pub fn new(total: u32) -> PermitLedger {
        PermitLedger { remaining: AtomicU32::new(total), trace_permits: semio_framework_trace::PermitLedger::new() }
    }

    /// ⚖️ Draws `n` permits, returning a [`PermitGuard`] that returns them on drop, or `Err` if
    /// fewer than `n` remain. A checked compare-exchange loop: a release build gets the exact same
    /// `Err` a debug build would, never a silently wrapped counter.
    pub fn checkout(&self, n: u32) -> Result<PermitGuard<'_>, PermitError> {
        loop {
            let current = self.remaining.load(Ordering::SeqCst);
            if n > current {
                return Err(PermitError { requested: n, remaining: current });
            }
            let next = current - n;
            if self.remaining.compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                for _ in 0..n {
                    self.trace_permits.acquire();
                }
                return Ok(PermitGuard { ledger: self, n });
            }
        }
    }

    pub fn remaining(&self) -> u32 {
        self.remaining.load(Ordering::SeqCst)
    }

    /// 📊️ Live occupancy — how many permits are currently checked out. Backed by
    /// `semio_framework_trace::PermitLedger`, so anything already reading that trace type sees the
    /// same number.
    pub fn occupancy(&self) -> u32 {
        self.trace_permits.occupancy()
    }
}

/// ⚖️ RAII guard returned by [`PermitLedger::checkout`] — returns its `n` permits to the ledger
/// (and releases the matching trace occupancy) on drop, whether that drop is an ordinary scope exit
/// or unwinding past a panic.
#[derive(Debug)]
pub struct PermitGuard<'a> {
    ledger: &'a PermitLedger,
    n: u32,
}

impl Drop for PermitGuard<'_> {
    fn drop(&mut self) {
        self.ledger.remaining.fetch_add(self.n, Ordering::SeqCst);
        for _ in 0..self.n {
            self.ledger.trace_permits.release();
        }
    }
}
//#endregion ⚖️PermitLedger

//#region ⏰️TimerWheel
/// ⏰️ One registered [`TimerWheel::sleep_until`] wait: its logical deadline, the [`Waker`] to fire
/// once due, and whether it has already fired (checked by [`TimerSleep::poll`] on a spurious
/// re-poll before the waker's task actually runs again).
struct TimerRegistration {
    // 🔕 dead_code: kept for future debugging/introspection (e.g. listing pending deadlines); the
    // heap already carries `deadline_ms` for ordering, so this field is never read today.
    #[allow(dead_code)]
    deadline_ms: u64,
    waker: Option<Waker>,
    fired: bool,
}

struct TimerWheelState {
    heap: BinaryHeap<Reverse<(u64, u64)>>,
    entries: HashMap<u64, TimerRegistration>,
    next_id: u64,
    last_now_ms: u64,
}

/// ⏰️ Min-heap-of-deadlines timer primitive shared by [`WorkerPool`] (native: an idle-parked worker
/// calls [`TimerWheel::fire_due`] with its own elapsed-wall-clock reading each time it wakes; wasm:
/// [`WorkerPool::pump`] calls it with the host-supplied `now_ms`) — the epoch-ticker OS thread Phase
/// 0's census found (`🧵️shard/…host.rs` `"semio-epoch-ticker"`, 1 ms poll loop) disappears; deadline
/// tracking becomes this reactive structure plus [`Lane::Timer`] instead of a dedicated thread.
/// Deliberately does NOT read a clock itself (this crate's "caller supplies `now_ms`" discipline,
/// same as [`HostAsyncRuntime::now_ms`]) — [`TimerWheel::fire_due`] is always told the current time
/// by whichever driver called it.
pub struct TimerWheel {
    state: Mutex<TimerWheelState>,
}

impl TimerWheel {
    pub fn new() -> TimerWheel {
        TimerWheel { state: Mutex::new(TimerWheelState { heap: BinaryHeap::new(), entries: HashMap::new(), next_id: 1, last_now_ms: 0 }) }
    }

    /// ⏰️ A future that resolves once this wheel has been [`TimerWheel::fire_due`]'d with a
    /// `now_ms >= deadline_ms` (or was already, before this fn was even called).
    pub fn sleep_until(&self, deadline_ms: u64) -> TimerSleep<'_> {
        TimerSleep { wheel: self, deadline_ms, id: None }
    }

    fn register(&self, deadline_ms: u64, waker: Waker) -> u64 {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let id = state.next_id;
        state.next_id += 1;
        state.entries.insert(id, TimerRegistration { deadline_ms, waker: Some(waker), fired: false });
        state.heap.push(Reverse((deadline_ms, id)));
        id
    }

    fn update_waker(&self, id: u64, waker: Waker) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(entry) = state.entries.get_mut(&id) {
            entry.waker = Some(waker);
        }
    }

    fn is_fired(&self, id: u64) -> bool {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.entries.get(&id).is_none_or(|entry| entry.fired)
    }

    fn forget(&self, id: u64) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.entries.remove(&id);
    }

    fn last_now_ms(&self) -> u64 {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).last_now_ms
    }

    /// ⏰️ The earliest still-pending deadline, if any — used by [`WorkerPool`]'s idle-park loop to
    /// bound how long a worker sleeps before it must re-check timers.
    pub fn next_deadline_ms(&self) -> Option<u64> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.heap.peek().map(|Reverse((deadline, _))| *deadline)
    }

    /// ⏰️ Wakes every registration whose deadline is `<= now_ms`, returning how many fired. Stale
    /// heap entries for already-[`TimerWheel::forget`]-ten ids are skipped lazily (never
    /// compacted eagerly — the heap only ever grows by live-registration count, bounded by however
    /// many `sleep_until` calls are outstanding at once).
    pub fn fire_due(&self, now_ms: u64) -> u32 {
        let mut woken = Vec::new();
        {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.last_now_ms = state.last_now_ms.max(now_ms);
            while let Some(Reverse((deadline, id))) = state.heap.peek().copied() {
                if deadline > now_ms {
                    break;
                }
                state.heap.pop();
                if let Some(entry) = state.entries.get_mut(&id) {
                    if !entry.fired {
                        entry.fired = true;
                        if let Some(waker) = entry.waker.take() {
                            woken.push(waker);
                        }
                    }
                }
            }
        }
        let count = woken.len() as u32;
        for waker in woken {
            waker.wake();
        }
        count
    }
}

impl Default for TimerWheel {
    fn default() -> TimerWheel {
        TimerWheel::new()
    }
}

/// ⏰️ Future returned by [`TimerWheel::sleep_until`]. Registers itself into the wheel on first
/// poll (or resolves immediately if the wheel's last known `now_ms` already reached the deadline),
/// then relies entirely on [`TimerWheel::fire_due`] waking it — no busy-polling.
pub struct TimerSleep<'a> {
    wheel: &'a TimerWheel,
    deadline_ms: u64,
    id: Option<u64>,
}

impl Future for TimerSleep<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        if this.wheel.last_now_ms() >= this.deadline_ms {
            return Poll::Ready(());
        }
        match this.id {
            Some(id) if this.wheel.is_fired(id) => Poll::Ready(()),
            Some(id) => {
                this.wheel.update_waker(id, cx.waker().clone());
                Poll::Pending
            }
            None => {
                this.id = Some(this.wheel.register(this.deadline_ms, cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

impl Drop for TimerSleep<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            self.wheel.forget(id);
        }
    }
}
//#endregion ⏰️TimerWheel

//#region 🧵️WorkerPool
/// 🧵️ One submitted unit of pool work — a plain closure, never a `Future`. `WorkerPool` is the CPU
/// substrate every subsystem's OS-thread work collapses onto (Phase 0 census: shard executors, shard
/// outcome forwarders, DB actor threads, the epoch ticker, HTTP fetch threads); polling `Future`s to
/// completion remains the future-polling executor's job (packet R2/P1b), built ON TOP of this pool.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// 🧵️ Construction parameters for [`WorkerPool::new`]. `process_kind` is never inferred — see
/// [`ProcessKind`]'s doc — and `interactive_reserve` defaults to "on" for
/// [`ProcessKind::InteractiveNative`] (admission control active, see [`WorkerPool`]'s doc) and "off"
/// for [`ProcessKind::HeadlessBatch`] (a headless process has no UI thread to protect, so reserving
/// a slot for it would only cost throughput for nothing).
#[derive(Clone, Copy, Debug)]
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

/// 🧵️ Bound on how long an idle native worker parks before it must wake and re-check timers/
/// shutdown even with no new work signalled — keeps a single-worker pool making progress (a parked
/// thread that only wakes on `notify` could otherwise sit past a due timer indefinitely if the
/// notify race is lost) and keeps every worker's idle-to-active latency low enough that the 8 ms
/// interactive ceiling is never blown by scheduling latency alone. Native-only: the wasm cooperative
/// pool has no idle-park loop (the host drives it via [`WorkerPool::pump`] instead).
#[cfg(not(target_arch = "wasm32"))]
const MAX_IDLE_PARK_MS: u64 = 4;

//#region 🧵️WorkerPoolNative
#[cfg(not(target_arch = "wasm32"))]
mod native_pool {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Condvar;
    use std::thread::{self, JoinHandle};
    use std::time::{Duration, Instant};

    struct WorkerLocal {
        queues: [Mutex<VecDeque<Job>>; LANE_COUNT],
    }

    impl WorkerLocal {
        fn new() -> WorkerLocal {
            WorkerLocal { queues: std::array::from_fn(|_| Mutex::new(VecDeque::new())) }
        }
    }

    struct PoolInner {
        workers: Vec<WorkerLocal>,
        shutdown: std::sync::atomic::AtomicBool,
        next_submit: AtomicUsize,
        idle: (Mutex<()>, Condvar),
        wheel: TimerWheel,
        low_priority_active: AtomicU32,
        interactive_reserve: bool,
        ledger: PermitLedger,
        trace_workers: semio_framework_trace::WorkerCounters,
        start: Instant,
        handles: Mutex<Vec<JoinHandle<()>>>,
    }

    impl PoolInner {
        /// 🚦️ Whether a `Background`/`Maintenance` job may be admitted right now — false only when
        /// the interactive reserve is active, there are 2+ workers, and every non-reserved worker
        /// slot is already occupied by low-priority work. Real runtime enforcement, not a comment:
        /// checked on the hot scheduling path before a low-priority lane is even considered.
        fn admit_low_priority(&self) -> bool {
            if !self.interactive_reserve || self.workers.len() < 2 {
                return true;
            }
            let ceiling = self.workers.len() as u32 - 1;
            self.low_priority_active.load(Ordering::SeqCst) < ceiling
        }

        fn now_ms(&self) -> u64 {
            self.start.elapsed().as_millis() as u64
        }

        fn idle_timeout(&self) -> Duration {
            let now = self.now_ms();
            let wait_ms = self.wheel.next_deadline_ms().map_or(MAX_IDLE_PARK_MS, |deadline| deadline.saturating_sub(now)).clamp(1, MAX_IDLE_PARK_MS);
            Duration::from_millis(wait_ms)
        }

        fn notify_idle(&self) {
            let (lock, cvar) = &self.idle;
            let _guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
            cvar.notify_all();
        }
    }

    /// ⚖️ Deficit-round-robin selection over one worker's OWN lane queues (never a sibling's — see
    /// [`steal`] for that). `cursor`/`deficits` are private, un-synchronized state owned entirely by
    /// the calling worker thread — no atomics needed, since no other thread ever touches them.
    /// `UNIT_COST` is [`Lane::Interactive`]'s own weight (the maximum), so the highest-weight lane
    /// is serviced every single scan while lower-weight lanes accrue deficit proportionally slower
    /// and are serviced roughly every `UNIT_COST / weight` scans — bounded, never starved.
    fn select_and_pop(inner: &PoolInner, my_index: usize, cursor: &mut usize, deficits: &mut [i64; LANE_COUNT]) -> Option<(Lane, Job)> {
        const UNIT_COST: i64 = Lane::Interactive.weight() as i64;
        let my = &inner.workers[my_index];
        for _ in 0..LANE_COUNT {
            let lane = Lane::ALL[*cursor];
            *cursor = (*cursor + 1) % LANE_COUNT;
            if is_low_priority(lane) && !inner.admit_low_priority() {
                continue;
            }
            let mut queue = my.queues[lane.index()].lock().unwrap_or_else(PoisonError::into_inner);
            if queue.is_empty() {
                deficits[lane.index()] = 0;
                continue;
            }
            deficits[lane.index()] += lane.weight() as i64;
            if deficits[lane.index()] >= UNIT_COST {
                deficits[lane.index()] -= UNIT_COST;
                let job = queue.pop_front().expect("WorkerPool: queue observed non-empty then empty under its own lock");
                return Some((lane, job));
            }
        }
        None
    }

    /// 🕵️ Work-stealing: scans every sibling worker (starting just after `my_index`, wrapping) for
    /// the first non-empty lane, in [`Lane::ALL`] order, respecting the same admission-control gate
    /// as [`select_and_pop`]. Stealing always pops from the FRONT (same end the owner pops from) so
    /// a stolen job is still the oldest one queued for that lane on that worker.
    fn steal(inner: &PoolInner, my_index: usize) -> Option<(Lane, Job)> {
        let worker_count = inner.workers.len();
        for offset in 1..worker_count {
            let victim = (my_index + offset) % worker_count;
            for lane in Lane::ALL {
                if is_low_priority(lane) && !inner.admit_low_priority() {
                    continue;
                }
                let mut queue = inner.workers[victim].queues[lane.index()].lock().unwrap_or_else(PoisonError::into_inner);
                if let Some(job) = queue.pop_front() {
                    return Some((lane, job));
                }
            }
        }
        None
    }

    fn worker_loop(inner: &Arc<PoolInner>, index: u32) {
        semio_framework_trace::register_worker_thread(index);
        let mut cursor = 0usize;
        let mut deficits = [0i64; LANE_COUNT];
        while !inner.shutdown.load(Ordering::SeqCst) {
            inner.wheel.fire_due(inner.now_ms());
            let picked = select_and_pop(inner, index as usize, &mut cursor, &mut deficits).or_else(|| steal(inner, index as usize));
            match picked {
                Some((lane, job)) => {
                    let low_priority = is_low_priority(lane);
                    if low_priority {
                        inner.low_priority_active.fetch_add(1, Ordering::SeqCst);
                    }
                    inner.trace_workers.worker_started();
                    let permit = inner.ledger.checkout(1).expect("WorkerPool: internal permit invariant violated — checked out more than worker_count concurrently");
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(job));
                    drop(permit);
                    inner.trace_workers.worker_finished();
                    if low_priority {
                        inner.low_priority_active.fetch_sub(1, Ordering::SeqCst);
                    }
                }
                None => {
                    let (lock, cvar) = &inner.idle;
                    let guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
                    let timeout = inner.idle_timeout();
                    let _ = cvar.wait_timeout(guard, timeout);
                }
            }
        }
    }

    /// 🧵️ The native, multi-OS-thread work-stealing pool. Per-worker per-lane deques (own-thread
    /// DRR pop, cross-thread steal) implemented with plain `std::sync::Mutex<VecDeque<_>>` — no
    /// lock-free Chase-Lev deque, no external crate: a `Mutex` per (worker, lane) pair is simple,
    /// correct, and cheap enough at this crate's job granularity (whole closures, not fine-grained
    /// tasks). See the module doc for why timers live here too instead of a dedicated OS thread.
    #[derive(Clone)]
    pub struct WorkerPool {
        inner: Arc<PoolInner>,
    }

    impl WorkerPool {
        pub fn new(config: WorkerPoolConfig) -> WorkerPool {
            let worker_count = worker_count_for(config.process_kind, config.cores).max(1);
            let inner = Arc::new(PoolInner {
                workers: (0..worker_count).map(|_| WorkerLocal::new()).collect(),
                shutdown: std::sync::atomic::AtomicBool::new(false),
                next_submit: AtomicUsize::new(0),
                idle: (Mutex::new(()), Condvar::new()),
                wheel: TimerWheel::new(),
                low_priority_active: AtomicU32::new(0),
                interactive_reserve: config.interactive_reserve,
                ledger: PermitLedger::new(worker_count as u32),
                trace_workers: semio_framework_trace::WorkerCounters::new(),
                start: Instant::now(),
                handles: Mutex::new(Vec::with_capacity(worker_count)),
            });
            let mut handles = Vec::with_capacity(worker_count);
            for index in 0..worker_count {
                let worker_inner = Arc::clone(&inner);
                let handle = thread::Builder::new().name(format!("semio-pool-worker-{index}")).spawn(move || worker_loop(&worker_inner, index as u32)).expect("WorkerPool: failed to spawn worker thread");
                handles.push(handle);
            }
            *inner.handles.lock().unwrap_or_else(PoisonError::into_inner) = handles;
            WorkerPool { inner }
        }

        /// ▶️ Enqueues `job` onto `lane`, targeting one worker round-robin (siblings pick it up via
        /// [`steal`] if that worker is busy) and waking any idle-parked worker.
        pub fn submit(&self, lane: Lane, job: Job) {
            let index = self.inner.next_submit.fetch_add(1, Ordering::SeqCst) % self.inner.workers.len();
            self.inner.workers[index].queues[lane.index()].lock().unwrap_or_else(PoisonError::into_inner).push_back(job);
            self.inner.notify_idle();
        }

        pub fn worker_count(&self) -> usize {
            self.inner.workers.len()
        }

        /// 📊️ Workers currently executing a job (not merely alive) — see the module doc's
        /// distinction from [`WorkerPool::occupancy`].
        pub fn active_workers(&self) -> u32 {
            self.inner.trace_workers.active()
        }

        /// 📊️ Permits currently checked out of this pool's own [`PermitLedger`] (sized to
        /// `worker_count`) — numerically the same as [`WorkerPool::active_workers`] for THIS pool's
        /// internal bookkeeping today, exposed separately because [`PermitLedger`] is also a
        /// standalone, independently reusable type.
        pub fn occupancy(&self) -> u32 {
            self.inner.ledger.occupancy()
        }

        pub fn permits(&self) -> &PermitLedger {
            &self.inner.ledger
        }

        pub fn timer(&self) -> &TimerWheel {
            &self.inner.wheel
        }

        /// 🕐️ This pool's own monotonic milliseconds-since-construction reading — usable by a
        /// concrete [`HostAsyncRuntime`] implementation that wants to delegate `now_ms` to the pool
        /// it already schedules onto, rather than owning a second clock.
        pub fn now_ms(&self) -> u64 {
            self.inner.now_ms()
        }

        /// 🛑️ Signals shutdown, wakes every idle-parked worker, and joins every worker thread —
        /// blocks until all in-flight jobs finish. Not called automatically on drop (a cloned
        /// handle dropping must never tear down siblings' pool).
        pub fn shutdown(&self) {
            self.inner.shutdown.store(true, Ordering::SeqCst);
            self.inner.notify_idle();
            let mut handles = self.inner.handles.lock().unwrap_or_else(PoisonError::into_inner);
            for handle in handles.drain(..) {
                let _ = handle.join();
            }
        }
    }
}
//#endregion 🧵️WorkerPoolNative

//#region 🧵️WorkerPoolWasm
#[cfg(target_arch = "wasm32")]
mod wasm_pool {
    use super::*;

    struct SchedulerState {
        queues: [VecDeque<Job>; LANE_COUNT],
        cursor: usize,
        deficits: [i64; LANE_COUNT],
    }

    impl SchedulerState {
        fn new() -> SchedulerState {
            SchedulerState { queues: std::array::from_fn(|_| VecDeque::new()), cursor: 0, deficits: [0; LANE_COUNT] }
        }

        fn select_and_pop(&mut self) -> Option<(Lane, Job)> {
            const UNIT_COST: i64 = Lane::Interactive.weight() as i64;
            for _ in 0..LANE_COUNT {
                let lane = Lane::ALL[self.cursor];
                self.cursor = (self.cursor + 1) % LANE_COUNT;
                let queue = &mut self.queues[lane.index()];
                if queue.is_empty() {
                    self.deficits[lane.index()] = 0;
                    continue;
                }
                self.deficits[lane.index()] += lane.weight() as i64;
                if self.deficits[lane.index()] >= UNIT_COST {
                    self.deficits[lane.index()] -= UNIT_COST;
                    let job = queue.pop_front().expect("WorkerPool: queue observed non-empty then empty");
                    return Some((lane, job));
                }
            }
            None
        }

        fn has_pending(&self) -> bool {
            self.queues.iter().any(|queue| !queue.is_empty())
        }
    }

    struct PoolInner {
        state: Mutex<SchedulerState>,
        wheel: TimerWheel,
        ledger: PermitLedger,
        trace_workers: semio_framework_trace::WorkerCounters,
    }

    /// 🧵️ `wasm32` cooperative, host-driven, single-logical-worker pool — no OS threads exist on
    /// this target. Same public surface as the native pool ([`WorkerPool::new`]/`submit`/
    /// `worker_count`/`active_workers`/`occupancy`/`permits`/`timer`/`shutdown`), PLUS
    /// [`WorkerPool::pump`], which the host (the browser's Web Worker running this WASM module — see
    /// `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`) must call repeatedly to make progress:
    /// each call runs AT MOST one DRR-selected job and fires due timers, then returns whether more
    /// work remains. Running one job per `pump` call (rather than looping internally until a time
    /// budget expires, which this crate cannot measure on plain `wasm32-unknown-unknown` — no clock,
    /// see `semio_framework_trace::now_us`'s same constraint) is what keeps a single long-running
    /// pump call from ever blowing the 8 ms interactive ceiling: the HOST controls the cadence
    /// between calls (a microtask chain, a `requestAnimationFrame` slice), and job AUTHORS (Phase
    /// 2's job protocol) are responsible for keeping one job's own body within the ceiling — the
    /// same caller obligation the native pool documents for its 1-worker case.
    #[derive(Clone)]
    pub struct WorkerPool {
        inner: Arc<PoolInner>,
        interactive_reserve: bool,
    }

    impl WorkerPool {
        pub fn new(config: WorkerPoolConfig) -> WorkerPool {
            let inner = Arc::new(PoolInner { state: Mutex::new(SchedulerState::new()), wheel: TimerWheel::new(), ledger: PermitLedger::new(1), trace_workers: semio_framework_trace::WorkerCounters::new() });
            WorkerPool { inner, interactive_reserve: config.interactive_reserve }
        }

        pub fn submit(&self, lane: Lane, job: Job) {
            let mut state = self.inner.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.queues[lane.index()].push_back(job);
        }

        pub fn worker_count(&self) -> usize {
            1
        }

        pub fn active_workers(&self) -> u32 {
            self.inner.trace_workers.active()
        }

        pub fn occupancy(&self) -> u32 {
            self.inner.ledger.occupancy()
        }

        pub fn permits(&self) -> &PermitLedger {
            &self.inner.ledger
        }

        pub fn timer(&self) -> &TimerWheel {
            &self.inner.wheel
        }

        /// 🛑️ No OS threads to join on `wasm32` — shutdown is a no-op; the host simply stops
        /// calling [`WorkerPool::pump`]. Kept for public-API parity with the native pool.
        pub fn shutdown(&self) {
            let _ = self.interactive_reserve;
        }

        /// ⏱️ Runs at most one DRR-selected job (admission control is a documented no-op here — the
        /// single-worker case is explicitly out of scope for the interactive reserve, same as the
        /// native pool's `worker_count < 2` short-circuit) and fires every timer due at
        /// `now_ms` (host-supplied — this target has no clock of its own). Returns whether more work
        /// is still queued, so the host knows whether to reschedule a pump immediately or wait for
        /// its next natural tick.
        pub fn pump(&self, now_ms: u64) -> bool {
            self.inner.wheel.fire_due(now_ms);
            let picked = {
                let mut state = self.inner.state.lock().unwrap_or_else(PoisonError::into_inner);
                state.select_and_pop()
            };
            if let Some((_lane, job)) = picked {
                self.inner.trace_workers.worker_started();
                let permit = self.inner.ledger.checkout(1).expect("WorkerPool: internal permit invariant violated on wasm pump");
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(job));
                drop(permit);
                self.inner.trace_workers.worker_finished();
            }
            self.has_pending_work()
        }

        pub fn has_pending_work(&self) -> bool {
            self.inner.state.lock().unwrap_or_else(PoisonError::into_inner).has_pending()
        }
    }
}
//#endregion 🧵️WorkerPoolWasm

#[cfg(not(target_arch = "wasm32"))]
pub use native_pool::WorkerPool;
#[cfg(target_arch = "wasm32")]
pub use wasm_pool::WorkerPool;
//#endregion 🧵️WorkerPool

//#region 🧪️ManualRuntime
/// 🧪️ In-crate [`HostAsyncRuntime`] test double: a manual poll loop over an injected clock, so
/// downstream crates (packets R2/R4) can unit-test against the trait without linking tokio. Time
/// never advances on its own — callers drive it with [`ManualRuntime::set_now_ms`] and progress the
/// futures with [`ManualRuntime::drive`]. Feature-gated (`testkit`, and implicitly available under
/// `cfg(test)` for this crate's own test suite) rather than part of the default build, so the pure
/// crate never ships test-only bookkeeping in a normal dependency. Phase 1 packet P1a: no longer
/// implements `run_blocking` — the trait method was removed; a test that needs blocking-style work
/// now submits directly to a [`WorkerPool`] it constructs itself.
#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::AtomicU64;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Waker};

    #[derive(Clone)]
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
            ManualRuntime(Arc::new(ManualRuntimeState { now_ms: AtomicU64::new(start_now_ms), next_scope_id: AtomicU64::new(1), scopes: Mutex::new(HashMap::new()), tasks: Mutex::new(Vec::new()) }))
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

        async fn sleep_until(&self, deadline_ms: u64) {
            ManualSleep { state: self.0.clone(), deadline_ms }.await;
        }

        async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
            let _ = grace_ms;
            let mut report = ScopeDrainReport::default();
            // 🌉️ R15: `cancel_scope` is now `Send`-bound at the trait (an explicit RPITIT bound,
            // not the implicit auto-trait rustc could not previously prove for a generic `R`), so a
            // `std::sync::MutexGuard` — itself never `Send` — can no longer be held across the
            // `.await`s below. Snapshot both maps into owned, lock-free data first, then await, then
            // take the locks back out only for the plain-synchronous mutation at the end.
            let snapshot: Vec<(u64, ManualScopeRecord)> = {
                let scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
                scopes.iter().map(|(id, record)| (*id, record.clone())).collect()
            };
            let mut cancelled_scope_ids = Vec::new();
            for (id, record) in &snapshot {
                if scope_owner_matches(owner, record).await {
                    cancelled_scope_ids.push(*id);
                }
            }
            for (id, record) in &snapshot {
                if cancelled_scope_ids.contains(id) {
                    record.cancel.cancel().await;
                }
            }
            let cancelled_task_count = {
                let mut tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
                let before = tasks.len();
                tasks.retain(|task| !cancelled_scope_ids.contains(&task.scope_id));
                (before - tasks.len()) as u32
            };
            {
                let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
                for id in &cancelled_scope_ids {
                    if let Some(record) = scopes.get_mut(id) {
                        record.cancelled += cancelled_task_count;
                        report.finished += record.finished;
                        report.cancelled += record.cancelled;
                    }
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
    use std::sync::mpsc;

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

    //#region 🏭️ProcessKindTests
    #[test]
    fn worker_count_interactive_native_reserves_one_core() {
        assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 10), 9);
        assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 2), 1);
        assert_eq!(worker_count_for(ProcessKind::InteractiveNative, 1), 1, "a single-core interactive host must still get one worker, never zero");
    }

    #[test]
    fn worker_count_headless_batch_uses_every_core() {
        assert_eq!(worker_count_for(ProcessKind::HeadlessBatch, 10), 10);
        assert_eq!(worker_count_for(ProcessKind::HeadlessBatch, 1), 1);
    }
    //#endregion 🏭️ProcessKindTests

    //#region ⚖️PermitLedgerTests
    #[test]
    fn permit_ledger_checkout_debits_and_release_credits() {
        let ledger = PermitLedger::new(4);
        let guard = ledger.checkout(3).expect("3 of 4 permits must be available");
        assert_eq!(ledger.remaining(), 1);
        assert_eq!(ledger.occupancy(), 3);
        drop(guard);
        assert_eq!(ledger.remaining(), 4);
        assert_eq!(ledger.occupancy(), 0);
    }

    #[test]
    fn permit_ledger_over_allocation_returns_err_never_wraps() {
        let ledger = PermitLedger::new(2);
        let _guard = ledger.checkout(2).expect("exactly the full ledger must be checkoutable");
        let error = ledger.checkout(1).expect_err("over-allocation must return Err, never wrap the counter");
        assert_eq!(error, PermitError { requested: 1, remaining: 0 });
        assert_eq!(ledger.remaining(), 0, "a failed checkout must never mutate the remaining count");
    }

    /// 🚫️ The Phase 0 gate-report defect this type exists to close: `ThreadBudget::checkout` used
    /// `fetch_sub` + `debug_assert!`, so a release build silently wrapped on over-draw. This test
    /// runs under BOTH `cargo test` and `cargo test --release` (no `#[cfg(debug_assertions)]` guard
    /// anywhere on this test or on `PermitLedger::checkout`) — proving the checked behavior holds in
    /// release, not just in debug.
    #[test]
    fn permit_ledger_checked_in_release_too() {
        let ledger = PermitLedger::new(1);
        assert!(ledger.checkout(5).is_err());
        assert_eq!(ledger.remaining(), 1, "release build must never wrap: remaining stays exactly what it started at");
    }
    //#endregion ⚖️PermitLedgerTests

    //#region 🛣️LaneTests
    #[test]
    fn lane_from_context_lane_mirrors_actor_lane_discriminant_order() {
        assert_eq!(Lane::from_context_lane(0), Lane::Interactive);
        assert_eq!(Lane::from_context_lane(1), Lane::UserVisible);
        assert_eq!(Lane::from_context_lane(2), Lane::Background);
        assert_eq!(Lane::from_context_lane(3), Lane::Maintenance);
    }

    #[test]
    fn lane_weights_never_starve_the_lowest_lane() {
        assert!(Lane::Maintenance.weight() >= 1, "every lane must accrue SOME deficit every scan, or it would never run");
        assert!(Lane::Interactive.weight() > Lane::Maintenance.weight());
    }
    //#endregion 🛣️LaneTests

    //#region ⏰️TimerWheelTests
    #[test]
    fn timer_wheel_next_deadline_ms_reflects_earliest_pending() {
        let wheel = TimerWheel::new();
        assert_eq!(wheel.next_deadline_ms(), None);
        let waker = Waker::noop();
        wheel.register(200, waker.clone());
        wheel.register(50, waker.clone());
        wheel.register(300, waker.clone());
        assert_eq!(wheel.next_deadline_ms(), Some(50));
    }

    #[test]
    fn timer_wheel_fires_deadlines_in_order() {
        let wheel = Arc::new(TimerWheel::new());
        let order = Arc::new(Mutex::new(Vec::new()));

        struct RecordingWaker {
            label: u32,
            order: Arc<Mutex<Vec<u32>>>,
        }
        impl std::task::Wake for RecordingWaker {
            fn wake(self: Arc<Self>) {
                self.order.lock().unwrap().push(self.label);
            }
            fn wake_by_ref(self: &Arc<Self>) {
                self.order.lock().unwrap().push(self.label);
            }
        }

        let waker_a = Waker::from(Arc::new(RecordingWaker { label: 1, order: order.clone() }));
        let waker_b = Waker::from(Arc::new(RecordingWaker { label: 2, order: order.clone() }));
        let waker_c = Waker::from(Arc::new(RecordingWaker { label: 3, order: order.clone() }));
        wheel.register(300, waker_c);
        wheel.register(100, waker_a);
        wheel.register(200, waker_b);

        assert_eq!(wheel.fire_due(150), 1);
        assert_eq!(*order.lock().unwrap(), vec![1]);
        assert_eq!(wheel.fire_due(250), 1);
        assert_eq!(*order.lock().unwrap(), vec![1, 2]);
        assert_eq!(wheel.fire_due(1000), 1);
        assert_eq!(*order.lock().unwrap(), vec![1, 2, 3]);
    }

    #[semio_framework_async_macros::async_test]
    async fn timer_wheel_sleep_until_resolves_only_after_fire_due_reaches_deadline() {
        let wheel = Arc::new(TimerWheel::new());
        let (tx, rx) = mpsc::channel();
        let wheel_for_task = wheel.clone();
        std::thread::spawn(move || {
            block_on(async move {
                wheel_for_task.sleep_until(50).await;
                tx.send(()).expect("send must succeed");
            });
        });
        std::thread::sleep(std::time::Duration::from_millis(20));
        assert!(rx.try_recv().is_err(), "must not resolve before fire_due reaches the deadline");
        wheel.fire_due(50);
        rx.recv_timeout(std::time::Duration::from_secs(5)).expect("sleep_until must resolve once fire_due reaches the deadline");
    }
    //#endregion ⏰️TimerWheelTests

    //#region 🧵️WorkerPoolTests
    #[test]
    fn worker_pool_sizing_multi_core_and_forced_single_core() {
        let multi = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 8));
        assert_eq!(multi.worker_count(), 7);
        multi.shutdown();

        let single = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 1));
        assert_eq!(single.worker_count(), 1, "a forced single-core host must still get exactly one worker");
        single.shutdown();
    }

    #[test]
    fn worker_pool_runs_submitted_jobs_across_all_lanes() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
        let (tx, rx) = mpsc::channel();
        for lane in Lane::ALL {
            let tx = tx.clone();
            pool.submit(lane, Box::new(move || tx.send(lane).expect("send must succeed")));
        }
        let mut seen = Vec::new();
        for _ in 0..LANE_COUNT {
            seen.push(rx.recv_timeout(std::time::Duration::from_secs(5)).expect("every submitted job must eventually run"));
        }
        seen.sort_by_key(|lane| *lane as usize);
        let mut expected = Lane::ALL.to_vec();
        expected.sort_by_key(|lane| *lane as usize);
        assert_eq!(seen, expected);
        pool.shutdown();
    }

    #[test]
    fn worker_pool_work_stealing_moves_work_between_workers() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
        let (start_tx, start_rx) = mpsc::channel::<()>();
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let release_rx = Arc::new(Mutex::new(release_rx));

        pool.submit(
            Lane::Background,
            Box::new(move || {
                start_tx.send(()).expect("send must succeed");
                let _ = release_rx.lock().expect("mutex").recv();
            }),
        );
        start_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("the blocking job must start on some worker");

        let (done_tx, done_rx) = mpsc::channel();
        for _ in 0..3 {
            let done_tx = done_tx.clone();
            pool.submit(Lane::Background, Box::new(move || done_tx.send(()).expect("send must succeed")));
        }
        for _ in 0..3 {
            done_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("other workers must steal and finish this work even while one worker is blocked");
        }
        release_tx.send(()).expect("send must succeed");
        pool.shutdown();
    }

    #[test]
    fn worker_pool_lane_fairness_background_cannot_starve_interactive() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
        let keep_busy = Arc::new(AtomicBool::new(true));
        for _ in 0..2 {
            let keep_busy = keep_busy.clone();
            let pool_for_resubmit = pool.clone();
            pool.submit(Lane::Background, Box::new(move || saturate_background(&pool_for_resubmit, keep_busy)));
        }
        let (tx, rx) = mpsc::channel();
        pool.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("send must succeed")));
        let result = rx.recv_timeout(std::time::Duration::from_secs(10));
        keep_busy.store(false, Ordering::SeqCst);
        pool.shutdown();
        result.expect("a saturated background lane must never starve an interactive job indefinitely");
    }

    fn saturate_background(pool: &WorkerPool, keep_busy: Arc<AtomicBool>) {
        if keep_busy.load(Ordering::SeqCst) {
            let pool_for_resubmit = pool.clone();
            pool.submit(Lane::Background, Box::new(move || saturate_background(&pool_for_resubmit, keep_busy)));
        }
    }

    #[test]
    fn worker_pool_admission_control_keeps_an_interactive_slot_free() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 3));
        assert!(pool.worker_count() >= 2, "this test needs 2+ workers to exercise the reserve");
        let keep_busy = Arc::new(AtomicBool::new(true));
        for _ in 0..(pool.worker_count() * 4) {
            let keep_busy = keep_busy.clone();
            let pool_for_resubmit = pool.clone();
            pool.submit(Lane::Maintenance, Box::new(move || saturate_background(&pool_for_resubmit, keep_busy)));
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
        let (tx, rx) = mpsc::channel();
        pool.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("send must succeed")));
        let result = rx.recv_timeout(std::time::Duration::from_secs(5));
        keep_busy.store(false, Ordering::SeqCst);
        pool.shutdown();
        result.expect("with 2+ workers, at least one slot must stay free for interactive work even under a full low-priority flood");
    }

    #[test]
    fn worker_pool_timer_lane_fires_deadlines_in_order() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
        let order = Arc::new(Mutex::new(Vec::new()));
        let start = pool.now_ms();
        let sleeps: Vec<_> = [30u64, 10, 20]
            .into_iter()
            .map(|delay| {
                let order = order.clone();
                let pool_clone = pool.clone();
                std::thread::spawn(move || {
                    block_on(async move {
                        pool_clone.timer().sleep_until(start + delay).await;
                        order.lock().expect("mutex").push(delay);
                    });
                })
            })
            .collect();
        for handle in sleeps {
            handle.join().expect("timer thread must not panic");
        }
        assert_eq!(*order.lock().expect("mutex"), vec![10, 20, 30], "deadlines must fire in deadline order regardless of registration order");
        pool.shutdown();
    }

    #[test]
    fn worker_pool_within_lane_within_worker_ordering_is_fifo() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let order = Arc::new(Mutex::new(Vec::new()));
        for i in 0..20u32 {
            let order = order.clone();
            pool.submit(Lane::Interactive, Box::new(move || order.lock().expect("mutex").push(i)));
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while order.lock().expect("mutex").len() < 20 && std::time::Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let seen = order.lock().expect("mutex").clone();
        let expected: Vec<u32> = (0..20).collect();
        assert_eq!(seen, expected, "a single worker's own lane must run jobs in submission order — this is the determinism the API promises");
        pool.shutdown();
    }

    #[test]
    fn worker_pool_active_workers_and_occupancy_reflect_running_jobs() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4));
        assert_eq!(pool.active_workers(), 0);
        assert_eq!(pool.occupancy(), 0);
        let (start_tx, start_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let release_rx = Arc::new(Mutex::new(release_rx));
        pool.submit(
            Lane::Background,
            Box::new(move || {
                start_tx.send(()).expect("send must succeed");
                let _ = release_rx.lock().expect("mutex").recv();
            }),
        );
        start_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("job must start");
        assert_eq!(pool.active_workers(), 1);
        assert_eq!(pool.occupancy(), 1);
        release_tx.send(()).expect("send must succeed");
        pool.shutdown();
        assert_eq!(pool.active_workers(), 0);
    }
    //#endregion 🧵️WorkerPoolTests

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
        ProcessKind::export().unwrap();
        Lane::export().unwrap();
    }
    //#endregion 🔖️Typegen
}
//#endregion 🧬️Tests
