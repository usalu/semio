//! 🌳️ Host-runtime vocabulary: scopes, channel policy and the `HostAsyncRuntime` trait.

use std::future::Future;
use std::pin::Pin;
use super::{CancelToken, OperationContext};

//#region 🌳️Scope
/// 🌳️ Who a [`Scope`] belongs to. `Service` is a small closed set of well-known host subsystem
/// names (e.g. `"timer"`, `"http_pool"`) rather than owned `String` data — an in-process identity,
/// never wire data, so it carries no serialization derive at all.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeOwner {
    Actor(u64),
    Package(String),
    /// 🏷️ A `&'static str` naming a host subsystem (never plugin/user data), e.g. `"timer"`.
    Service(&'static str),
}

/// 🔖️ Identity of one [`ScopeHandle`] — monotonically assigned by whatever
/// [`HostAsyncRuntime::open_scope`] implementation mints it. No `Serialize`/`Deserialize`: repo-wide
/// grep found zero wire-serialization call sites (see [`CancelState`]'s doc for the same finding).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
/// `leaked` is always worth surfacing to an operator; it is never silently swallowed. No
/// `Serialize`/`Deserialize`: same repo-wide zero-call-site finding as [`CancelState`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScopeDrainReport {
    pub finished: u32,
    pub cancelled: u32,
    pub leaked: u32,
}
//#endregion 🌳️Scope

//#region 🚰️ChannelPolicy
/// 🚰️ Declared backpressure vocabulary every channel/service/mailbox/stream in the async layer
/// must state up front — there is no implicit "unbounded" option. `LatestWins`/`Coalesced`/`Ring`
/// drop older queued values under load; `LosslessBounded`/`ByteCredit` reject or stall producers instead
/// of dropping. Phase 1 packet P1a requirement: EVERY variant bounds both item count and bytes —
/// `LatestWins` holds at most one slot so its item bound is implicitly `1` (no field needed);
/// every other variant carries an explicit `max_items` alongside `max_bytes`. No
/// `Serialize`/`Deserialize`: same repo-wide zero-call-site finding as [`CancelState`] — every
/// caller (`job::channel_policy_for`, `os::services::EventRouter`) builds/matches variants
/// in-process, never over a wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChannelPolicy {
    LatestWins { max_bytes: u64 },
    Coalesced { key: String, max_items: u32, max_bytes: u64 },
    Ring { max_items: u32, max_bytes: u64 },
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
