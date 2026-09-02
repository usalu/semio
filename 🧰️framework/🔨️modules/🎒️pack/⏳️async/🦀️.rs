//! 📦️ `pack_async` — runtime-neutral async access layer for `pack` files: the `AsyncPackSource`
//! trait (implementable over network/browser/worker sources without pulling in a concrete
//! runtime), a cooperative `CancellationToken`, and a `ReadScheduler` that coalesces overlapping
//! reads, dedups identical in-flight requests, and orders contention through a priority-aware
//! `BoundedDemand` backpressure primitive. No `unsafe`, no hard `tokio` dependency — concurrency
//! is built on `std::sync` primitives plus the repository-owned async combinators.

//#region 🔖️AsyncSource
use std::cmp::Ordering as CmpOrdering;
use std::collections::BinaryHeap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use crate::{ByteRange, PackError};

/// 📥️ A random-access read source reachable only through `async`, e.g. a network range-fetcher
/// (see `pack_http`) or a browser `fetch`/worker bridge. Deliberately runtime-neutral: nothing
/// here requires `tokio`, so wasm/browser callers and native callers share one trait.
// 🚪️ R8: plain AFIT, no `#[async_trait]` — the trait mixes one E1-sync fn (`len`) with one
// genuinely-`async fn` (`read_at`); native `async fn` in trait already supports that mix, and
// there is zero `dyn AsyncPackSource` anywhere in the repo (verified: `🎒️pack/**` plus
// `🧰️framework`, `🛍️products`, `✏️s` repo-wide), so no dyn-erasure/O1 concern applies here.
pub trait AsyncPackSource: Send + Sync {
    // 🚫️async: no suspension point — every implementor answers from an already-known length
    // (see `pack_http::SharedState::len`'s doc comment); deliberately synchronous by contract.
    fn len(&self) -> u64;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError>;
}

/// 🗄️ Cancellation flag plus the wakers parked on it, behind one mutex so a `cancel()` racing a
/// `CancelWatch::poll`'s check-then-register can never produce a lost wakeup.
struct CancellationInner {
    cancelled: bool,
    wakers: Vec<Waker>,
}

/// 🛑️ A clone-cheap, thread-safe flag a caller can flip to short-circuit an in-flight
/// `ReadScheduler::read`. `cancel()` wakes every `CancelWatch` parked on it directly — no polling,
/// no sleeping; see `CancelWatch::poll` below.
#[derive(Clone)]
pub struct CancellationToken(Arc<Mutex<CancellationInner>>);

impl CancellationToken {
    // 🚫️async: no suspension point; also called from `Default::default` below, an E1 impl of
    // the externally-declared `Default` trait whose fixed sync signature cannot `.await`.
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(CancellationInner { cancelled: false, wakers: Vec::new() })))
    }

    // 🚫️async: no suspension point, and load-bearing — the cancellation test drives this from a
    // plain `std::thread::spawn` closure with no executor, simulating a foreign-thread cancel
    // signal. Wakes every waker parked by `CancelWatch::poll` under the same lock `cancel` takes.
    pub fn cancel(&self) {
        let mut inner = self.0.lock().unwrap();
        inner.cancelled = true;
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }
    }

    // 🚫️async: no suspension point; a cheap synchronous check kept sync so existing sync callers
    // (including `CancelWatch::poll`'s own crate-private `poll_cancelled` below) never need an
    // executor just to ask.
    pub fn is_cancelled(&self) -> bool {
        self.0.lock().unwrap().cancelled
    }

    /// 👀️ Checks cancellation and, if not yet cancelled, registers `waker` to be woken by the
    /// next `cancel()` — both under one lock, so this can never race a concurrent `cancel()` into
    /// missing the registration.
    // 🚫️async: E1 — the only caller is `CancelWatch::poll`, an impl of the externally-declared
    // `Future` trait, whose fixed sync signature cannot `.await`.
    fn poll_cancelled(&self, waker: &Waker) -> bool {
        let mut inner = self.0.lock().unwrap();
        if inner.cancelled {
            return true;
        }
        if !inner.wakers.iter().any(|parked| parked.will_wake(waker)) {
            inner.wakers.push(waker.clone());
        }
        false
    }
}

// 🚫️async: E1 impl of the externally-declared `Default` trait; `default`'s signature is fixed by
// `std::default::Default` and must stay sync.
impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// 👀️ Resolves with a cancellation error the moment `token` is cancelled, driven purely by
/// `CancellationToken::cancel`'s waker registration — never polls in a loop or sleeps. Raced via
/// `semio_framework_async::race2` against the real read/wait future so cancellation always wins as
/// soon as it is observed.
struct CancelWatch<'a> {
    token: &'a CancellationToken,
}

impl Future for CancelWatch<'_> {
    type Output = Result<Arc<Vec<u8>>, PackError>;

    // 🚫️async: E1 impl of the externally-declared `Future` trait; `poll`'s signature is fixed by
    // `std::future::Future` and must stay sync. The wait for cancellation is satisfied entirely
    // by `CancellationToken::poll_cancelled` registering this poll's waker — `cancel()` on
    // another thread calls `Waker::wake` directly, so this never spins or sleeps.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.token.poll_cancelled(cx.waker()) {
            Poll::Ready(Err(PackError::Io("read cancelled".to_string())))
        } else {
            Poll::Pending
        }
    }
}
//#endregion 🔖️AsyncSource

//#region 🔖️Scheduler
/// 🚦️ Relative urgency of a read, highest (`Critical`) to lowest (`Background`); consulted by
/// `BoundedDemand` to decide who gets the next free slot under contention.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LoadPriority {
    Critical,
    Visible,
    Requested,
    Prefetch,
    Background,
}

impl LoadPriority {
    /// 🏅️ Lower rank = served first.
    // 🚫️async: E1 — the only caller is `AcquireFuture::poll` below, an impl of the
    // externally-declared `Future` trait, whose fixed sync signature cannot `.await`.
    fn rank(self) -> u8 {
        match self {
            LoadPriority::Critical => 0,
            LoadPriority::Visible => 1,
            LoadPriority::Requested => 2,
            LoadPriority::Prefetch => 3,
            LoadPriority::Background => 4,
        }
    }
}

/// 📨️ One caller's request for a byte range, tagged with the priority `BoundedDemand` should
/// weigh it by when reads are contended.
#[derive(Clone, Copy, Debug)]
pub struct ReadRequest {
    pub range: ByteRange,
    pub priority: LoadPriority,
}

/// 🤝️ True iff `a` and `b` overlap or sit back-to-back (touching), so a single physical read of
/// their union serves both.
// 🚫️async: no suspension point — called by `join_or_create_group` while holding two
// `std::sync::MutexGuard`s across a loop; those guards are not `Send` and awaiting here would
// force them into the enclosing future's state, which R3 forbids (host-side `Send` must come
// structurally, never from a bound). Pure arithmetic, never needs to suspend.
fn ranges_touch(a: ByteRange, b: ByteRange) -> bool {
    let a_end = a.offset.saturating_add(a.len);
    let b_end = b.offset.saturating_add(b.len);
    a.offset <= b_end && b.offset <= a_end
}

/// ➕️ The smallest `ByteRange` spanning both `a` and `b`.
// 🚫️async: no suspension point — same lock-held-across-the-call constraint as `ranges_touch`.
fn ranges_union(a: ByteRange, b: ByteRange) -> ByteRange {
    let a_end = a.offset.saturating_add(a.len);
    let b_end = b.offset.saturating_add(b.len);
    let offset = a.offset.min(b.offset);
    ByteRange { offset, len: a_end.max(b_end) - offset }
}

/// 🌱️ Lifecycle of one coalesced physical read.
enum GroupState {
    /// Still open: newly arriving requests whose range touches `range` are folded in.
    Gathering,
    /// Closed for merging; a physical read for `range` is (or is about to be) in flight.
    Dispatched,
    /// Finished; every waiter gets a clone of this shared result.
    Done(Result<Arc<Vec<u8>>, PackError>),
}

/// 🗂️ One coalesced physical read and everyone waiting on it.
struct Group {
    range: ByteRange,
    state: GroupState,
    wakers: Vec<Waker>,
}

/// ⏳️ Non-leader wait: parks on a `Group` until it reaches `Done`, then hands back a clone of
/// the shared result (sliced to the caller's own sub-range by the caller).
struct WaitForGroup {
    group: Arc<Mutex<Group>>,
}

impl Future for WaitForGroup {
    type Output = Result<Arc<Vec<u8>>, PackError>;

    // 🚫️async: E1 impl of the externally-declared `Future` trait; signature fixed by std. Already
    // waker-correct (registers `cx.waker()` and returns `Pending`, woken by `finalize_group`'s
    // `waker.wake()`) — no sleep here, nothing to fix.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.group.lock().unwrap();
        match &inner.state {
            GroupState::Done(result) => Poll::Ready(result.clone()),
            _ => {
                if !inner.wakers.iter().any(|w| w.will_wake(cx.waker())) {
                    inner.wakers.push(cx.waker().clone());
                }
                Poll::Pending
            }
        }
    }
}

/// 🧵️ Coalesces adjacent/overlapping `AsyncPackSource` reads into a single physical fetch,
/// dedups callers asking for the identical range, and gates concurrency through a priority-aware
/// `BoundedDemand`. One instance owns one `AsyncPackSource`.
pub struct ReadScheduler<S: AsyncPackSource> {
    source: S,
    groups: Mutex<Vec<Arc<Mutex<Group>>>>,
    demand: BoundedDemand,
}

impl<S: AsyncPackSource> ReadScheduler<S> {
    /// 🆕️ A scheduler with a sensible default concurrency cap. Use `with_capacity` to tune it.
    // 🚫️async: no suspension point — construction only; kept sync so existing plain-sync test
    // call sites (`let scheduler = ReadScheduler::new(source);`, unawaited) keep compiling.
    pub fn new(source: S) -> Self {
        Self::with_capacity(source, 4)
    }

    /// 🆕️ A scheduler that admits at most `max_concurrent_reads` physical reads at once.
    // 🚫️async: no suspension point — same constructor reasoning as `new` above.
    pub fn with_capacity(source: S, max_concurrent_reads: usize) -> Self {
        Self { source, groups: Mutex::new(Vec::new()), demand: BoundedDemand::new(max_concurrent_reads) }
    }

    /// 📖️ Reads `request.range`, coalescing with any compatible in-flight read and honoring
    /// `cancel`. Returns exactly `request.range.len` bytes on success.
    pub async fn read(&self, request: ReadRequest, cancel: &CancellationToken) -> Result<Vec<u8>, PackError> {
        if cancel.is_cancelled() {
            return Err(PackError::Io("read cancelled".to_string()));
        }
        let (group, is_leader) = self.join_or_create_group(request.range);
        if is_leader {
            semio_framework_async::yield_once().await;
        }
        let outcome: Result<Arc<Vec<u8>>, PackError> = if is_leader {
            semio_framework_async::race2(self.dispatch_leader(&group, request.priority), CancelWatch { token: cancel }).await
        } else {
            semio_framework_async::race2(WaitForGroup { group: group.clone() }, CancelWatch { token: cancel }).await
        };
        if is_leader {
            self.finalize_group(&group, outcome.clone());
        }
        let data = outcome?;
        slice_group_result(&data, &group, request.range)
    }

    /// 🔎️ Finds a `Gathering` group whose range touches `range` and folds `range` into it
    /// (returning `is_leader = false`), or opens a fresh group (returning `is_leader = true`).
    // 🚫️async: no suspension point — the whole body runs under `self.groups`'s lock (plus, per
    // candidate, the group's own lock); never yields, so it stays a fast synchronous critical
    // section rather than holding a non-`Send` `MutexGuard` across an `.await`.
    fn join_or_create_group(&self, range: ByteRange) -> (Arc<Mutex<Group>>, bool) {
        let mut groups = self.groups.lock().unwrap();
        for group in groups.iter() {
            let mut inner = group.lock().unwrap();
            if matches!(inner.state, GroupState::Gathering) && ranges_touch(inner.range, range) {
                inner.range = ranges_union(inner.range, range);
                drop(inner);
                return (group.clone(), false);
            }
        }
        let group = Arc::new(Mutex::new(Group { range, state: GroupState::Gathering, wakers: Vec::new() }));
        groups.push(group.clone());
        (group, true)
    }

    /// 🚀️ Closes `group` for merging, waits its turn under `demand`, and performs the physical
    /// read for the group's (possibly merged) final range.
    async fn dispatch_leader(&self, group: &Arc<Mutex<Group>>, priority: LoadPriority) -> Result<Arc<Vec<u8>>, PackError> {
        let range = {
            let mut inner = group.lock().unwrap();
            inner.state = GroupState::Dispatched;
            inner.range
        };
        let permit = self.demand.acquire(priority).await;
        let physical = self.source.read_at(range.offset, range.len as usize).await;
        drop(permit);
        physical.map(Arc::new)
    }

    /// 🏁️ Records `result` as the group's outcome, wakes every waiter, and drops the group from
    /// the lookup table so nothing else can merge into it.
    // 🚫️async: no suspension point — runs entirely under `group`'s lock, then `self.groups`'s;
    // never yields (same lock-across-await avoidance as `join_or_create_group`).
    fn finalize_group(&self, group: &Arc<Mutex<Group>>, result: Result<Arc<Vec<u8>>, PackError>) {
        let wakers = {
            let mut inner = group.lock().unwrap();
            inner.state = GroupState::Done(result);
            std::mem::take(&mut inner.wakers)
        };
        for waker in wakers {
            waker.wake();
        }
        self.groups.lock().unwrap().retain(|candidate| !Arc::ptr_eq(candidate, group));
    }
}

/// ✂️ Slices `caller_range` out of `data`, which covers `group`'s (possibly wider, merged)
/// final range starting at its own offset.
// 🚫️async: no suspension point — pure byte-range arithmetic and a `Vec` copy, no I/O.
fn slice_group_result(data: &Arc<Vec<u8>>, group: &Arc<Mutex<Group>>, caller_range: ByteRange) -> Result<Vec<u8>, PackError> {
    let group_range = group.lock().unwrap().range;
    let start = caller_range.offset.checked_sub(group_range.offset).ok_or_else(|| PackError::Malformed { what: "async_read_slice", offset: caller_range.offset, detail: "requested range precedes the coalesced group range".to_string() })? as usize;
    let end = start.checked_add(caller_range.len as usize).ok_or(PackError::LimitExceeded("async_read_slice length overflow"))?;
    if end > data.len() {
        return Err(PackError::Truncated(caller_range.offset + caller_range.len));
    }
    Ok(data[start..end].to_vec())
}
//#endregion 🔖️Scheduler

//#region 🔖️Backpressure
/// 🎟️ One waiter parked on `BoundedDemand`, ordered by priority rank then arrival order.
struct DemandWaiter {
    rank: u8,
    seq: u64,
    waker: Mutex<Option<Waker>>,
    granted: AtomicBool,
}

// 🚫️async: E1 impls of the externally-declared `PartialEq`/`Eq`/`PartialOrd`/`Ord` traits;
// signatures fixed by `std::cmp`, required (sync) by `BinaryHeap<Arc<DemandWaiter>>`.
impl PartialEq for DemandWaiter {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.seq == other.seq
    }
}

impl Eq for DemandWaiter {}

impl PartialOrd for DemandWaiter {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for DemandWaiter {
    /// `BinaryHeap` is a max-heap; the *highest priority* (lowest rank, earliest seq) waiter
    /// must compare greatest so it pops first.
    fn cmp(&self, other: &Self) -> CmpOrdering {
        other.rank.cmp(&self.rank).then_with(|| other.seq.cmp(&self.seq))
    }
}

struct DemandState {
    in_flight: usize,
    capacity: usize,
    queue: BinaryHeap<Arc<DemandWaiter>>,
    next_seq: u64,
}

/// 🚧️ A semaphore-like backpressure primitive capping concurrent in-flight reads, with priority-
/// ordered release under contention (built entirely on `std::sync`, no runtime dependency).
pub struct BoundedDemand {
    state: Mutex<DemandState>,
}

impl BoundedDemand {
    // 🚫️async: no suspension point — construction only; kept sync so existing plain-sync test
    // call sites (`let demand = BoundedDemand::new(1);`, unawaited) keep compiling.
    pub fn new(capacity: usize) -> Self {
        Self { state: Mutex::new(DemandState { in_flight: 0, capacity: capacity.max(1), queue: BinaryHeap::new(), next_seq: 0 }) }
    }

    // 🚫️async: no suspension point — a cheap synchronous accessor, exercised directly by
    // `bounded_demand_reports_capacity_and_in_flight` as a plain sync call.
    pub fn capacity(&self) -> usize {
        self.state.lock().unwrap().capacity
    }

    // 🚫️async: no suspension point — see `capacity` above.
    pub fn in_flight(&self) -> usize {
        self.state.lock().unwrap().in_flight
    }

    /// 🎫️ Waits for a free slot, weighing contention by `priority`, and returns an RAII permit
    /// that frees the slot (handing it straight to the next-highest-priority waiter, if any) on
    /// drop.
    pub async fn acquire(&self, priority: LoadPriority) -> DemandPermit<'_> {
        AcquireFuture { demand: self, priority, waiter: None }.await
    }

    /// 🔓️ Frees one slot: transfers it to the highest-priority queued waiter if any are waiting,
    /// else simply lowers the in-flight count.
    // 🚫️async: E1 — the only caller is `DemandPermit::drop` below, an impl of the
    // externally-declared `Drop` trait, whose fixed sync signature cannot `.await`.
    fn release(&self) {
        let mut state = self.state.lock().unwrap();
        if let Some(next) = state.queue.pop() {
            next.granted.store(true, Ordering::SeqCst);
            if let Some(waker) = next.waker.lock().unwrap().take() {
                waker.wake();
            }
        } else {
            state.in_flight -= 1;
        }
    }
}

/// 🎟️ An RAII hold on one of `BoundedDemand`'s slots; releasing (via `Drop`) frees it.
pub struct DemandPermit<'a> {
    demand: &'a BoundedDemand,
}

// 🚫️async: E1 impl of the externally-declared `Drop` trait; `drop`'s signature is fixed by
// `std::ops::Drop` and must stay sync.
impl Drop for DemandPermit<'_> {
    fn drop(&mut self) {
        self.demand.release();
    }
}

struct AcquireFuture<'a> {
    demand: &'a BoundedDemand,
    priority: LoadPriority,
    waiter: Option<Arc<DemandWaiter>>,
}

impl<'a> Future for AcquireFuture<'a> {
    type Output = DemandPermit<'a>;

    // 🚫️async: E1 impl of the externally-declared `Future` trait; signature fixed by std.
    // Already waker-correct: parks `cx.waker()` on the `DemandWaiter` and returns `Pending`,
    // woken by `BoundedDemand::release`'s `waker.wake()` — no sleep, no spin.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waiter) = &self.waiter {
            if waiter.granted.load(Ordering::SeqCst) {
                return Poll::Ready(DemandPermit { demand: self.demand });
            }
            *waiter.waker.lock().unwrap() = Some(cx.waker().clone());
            return Poll::Pending;
        }
        let mut state = self.demand.state.lock().unwrap();
        if state.in_flight < state.capacity {
            state.in_flight += 1;
            return Poll::Ready(DemandPermit { demand: self.demand });
        }
        let seq = state.next_seq;
        state.next_seq += 1;
        let waiter = Arc::new(DemandWaiter { rank: self.priority.rank(), seq, waker: Mutex::new(Some(cx.waker().clone())), granted: AtomicBool::new(false) });
        state.queue.push(waiter.clone());
        drop(state);
        self.waiter = Some(waiter);
        Poll::Pending
    }
}
//#endregion 🔖️Backpressure

//#region 🧪️Tests
// 🚫️async: every `semio_framework_async::block_on(...)` in this module drives one `#[test] fn` —
// each is its own synchronous test-harness entry point, the same role `fn main` plays for a
// binary (R4 item 1), so none are converted to `.await` here. See
// `📓️terra-pack-waker-report.md` for the full site-by-site census.
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    //#region 🔖️AsyncSource
    /// 🧪️ An in-memory `AsyncPackSource` test double that counts how many physical reads it
    /// actually served.
    struct RecordingSource {
        data: Vec<u8>,
        read_count: AtomicUsize,
    }

    impl RecordingSource {
        fn new(data: Vec<u8>) -> Self {
            Self { data, read_count: AtomicUsize::new(0) }
        }
    }

    impl AsyncPackSource for RecordingSource {
        fn len(&self) -> u64 {
            self.data.len() as u64
        }

        async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError> {
            self.read_count.fetch_add(1, Ordering::SeqCst);
            let start = offset as usize;
            let end = start.checked_add(len).ok_or(PackError::LimitExceeded("test read overflow"))?;
            if end > self.data.len() {
                return Err(PackError::Truncated(offset + len as u64));
            }
            Ok(self.data[start..end].to_vec())
        }
    }

    /// 🧪️ A test double whose `read_at` never resolves, used to prove `CancellationToken`
    /// actually short-circuits an in-flight read rather than waiting for it to finish.
    struct HangingSource;

    impl AsyncPackSource for HangingSource {
        fn len(&self) -> u64 {
            0
        }

        async fn read_at(&self, _offset: u64, _len: usize) -> Result<Vec<u8>, PackError> {
            std::future::pending::<()>().await;
            unreachable!("HangingSource::read_at never resolves")
        }
    }

    #[test]
    fn cancellation_token_starts_uncancelled_and_latches_once_cancelled() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        let clone = token.clone();
        clone.cancel();
        assert!(token.is_cancelled(), "cancelling a clone must be visible through the original");
    }

    /// 🧪️ Wraps a future and counts how many times it is actually polled — used below to prove
    /// `CancelWatch` is driven by real waker wakeups rather than a hot poll loop.
    struct CountingPoll<F> {
        inner: F,
        polls: Arc<AtomicUsize>,
    }

    impl<F: Future + Unpin> Future for CountingPoll<F> {
        type Output = F::Output;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.polls.fetch_add(1, Ordering::SeqCst);
            Pin::new(&mut self.inner).poll(cx)
        }
    }

    /// 🧪️ Regression test for the poll-sleep defect this packet fixes: `CancelWatch` must stay
    /// `Pending` until a genuinely separate OS thread transitions `CancellationToken`'s state
    /// (calls `cancel()`), then resolve — driven purely by `Waker::wake`, never by sleeping or
    /// busy-repolling. A busy-poll implementation (the original 200µs-sleep version) would rack
    /// up roughly `20ms / 200µs` ≈ 100 polls waiting out the delay below; a correct waker-based
    /// implementation polls a small constant number of times (park, wake, resolve) regardless of
    /// how long the other thread takes.
    #[test]
    fn cancel_watch_resolves_from_another_thread_via_waker_without_spinning_or_sleeping() {
        let token = CancellationToken::new();
        let canceller_token = token.clone();
        let polls = Arc::new(AtomicUsize::new(0));

        let canceller = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(20));
            canceller_token.cancel();
        });

        let counted = CountingPoll { inner: CancelWatch { token: &token }, polls: polls.clone() };
        let result = semio_framework_async::block_on(counted);

        canceller.join().unwrap();
        assert!(result.is_err(), "CancelWatch must resolve once the other thread cancels");
        let observed = polls.load(Ordering::SeqCst);
        assert!(observed <= 4, "CancelWatch::poll must be driven by Waker::wake, not busy-spinning; observed {observed} polls waiting out a 20ms cancellation");
    }
    //#endregion 🔖️AsyncSource

    //#region 🔖️Scheduler
    #[test]
    fn read_scheduler_coalesces_two_overlapping_ranges_into_one_physical_read() {
        let data: Vec<u8> = (0..100u16).map(|value| value as u8).collect();
        let source = RecordingSource::new(data.clone());
        let scheduler = ReadScheduler::new(source);
        let cancel = CancellationToken::new();

        let request_a = ReadRequest { range: ByteRange { offset: 0, len: 50 }, priority: LoadPriority::Requested };
        let request_b = ReadRequest { range: ByteRange { offset: 30, len: 50 }, priority: LoadPriority::Requested };

        let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(request_a, &cancel), scheduler.read(request_b, &cancel)));

        assert_eq!(result_a.unwrap(), data[0..50].to_vec());
        assert_eq!(result_b.unwrap(), data[30..80].to_vec());
        assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 1, "two overlapping ranges must coalesce into a single physical read");
    }

    #[test]
    fn read_scheduler_dedups_identical_in_flight_requests() {
        let data: Vec<u8> = (0..40u16).map(|value| value as u8).collect();
        let source = RecordingSource::new(data.clone());
        let scheduler = ReadScheduler::new(source);
        let cancel = CancellationToken::new();
        let request = ReadRequest { range: ByteRange { offset: 5, len: 10 }, priority: LoadPriority::Visible };

        let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(request, &cancel), scheduler.read(request, &cancel)));

        assert_eq!(result_a.unwrap(), data[5..15].to_vec());
        assert_eq!(result_b.unwrap(), data[5..15].to_vec());
        assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 1, "identical requests must dedup to one physical read");
    }

    #[test]
    fn read_scheduler_non_overlapping_requests_stay_separate_physical_reads() {
        let data: Vec<u8> = (0..40u16).map(|value| value as u8).collect();
        let source = RecordingSource::new(data);
        let scheduler = ReadScheduler::new(source);
        let cancel = CancellationToken::new();

        let far_apart_a = ReadRequest { range: ByteRange { offset: 0, len: 4 }, priority: LoadPriority::Visible };
        let far_apart_b = ReadRequest { range: ByteRange { offset: 30, len: 4 }, priority: LoadPriority::Visible };

        let (result_a, result_b) = semio_framework_async::block_on(semio_framework_async::join2(scheduler.read(far_apart_a, &cancel), scheduler.read(far_apart_b, &cancel)));

        assert!(result_a.is_ok());
        assert!(result_b.is_ok());
        assert_eq!(scheduler.source.read_count.load(Ordering::SeqCst), 2, "disjoint ranges must not be coalesced");
    }

    #[test]
    fn cancellation_short_circuits_an_in_flight_read_instead_of_hanging_forever() {
        let scheduler = ReadScheduler::new(HangingSource);
        let cancel = CancellationToken::new();
        let cancel_from_other_thread = cancel.clone();

        let canceller = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(20));
            cancel_from_other_thread.cancel();
        });

        let request = ReadRequest { range: ByteRange { offset: 0, len: 1 }, priority: LoadPriority::Critical };
        let result = semio_framework_async::block_on(scheduler.read(request, &cancel));

        canceller.join().unwrap();
        assert!(result.is_err(), "a cancelled read must short-circuit rather than hang until the source resolves");
    }

    #[test]
    fn cancellation_already_flagged_before_read_returns_immediately() {
        let scheduler = ReadScheduler::new(HangingSource);
        let cancel = CancellationToken::new();
        cancel.cancel();

        let request = ReadRequest { range: ByteRange { offset: 0, len: 1 }, priority: LoadPriority::Background };
        let result = semio_framework_async::block_on(scheduler.read(request, &cancel));
        assert!(result.is_err());
    }
    //#endregion 🔖️Scheduler

    //#region 🔖️Backpressure
    /// 🧵️ Polls `future` exactly once with a no-op waker, without consuming it if it is not
    /// ready — used to hand-drive several `BoundedDemand::acquire` futures deterministically to
    /// prove priority (not arrival order) decides who is granted next.
    fn poll_once<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        Pin::new(future).poll(&mut cx)
    }

    #[test]
    fn bounded_demand_priority_ordering_under_contention() {
        let demand = BoundedDemand::new(1);

        // Fill the single slot so every subsequent acquire() has to queue.
        let mut held = Box::pin(demand.acquire(LoadPriority::Critical));
        let permit0 = match poll_once(&mut held) {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("first acquire on an empty BoundedDemand must be immediate"),
        };

        // Arrival order deliberately reversed from priority order.
        let mut background = Box::pin(demand.acquire(LoadPriority::Background));
        let mut prefetch = Box::pin(demand.acquire(LoadPriority::Prefetch));
        let mut requested = Box::pin(demand.acquire(LoadPriority::Requested));
        let mut visible = Box::pin(demand.acquire(LoadPriority::Visible));

        assert!(matches!(poll_once(&mut background), Poll::Pending));
        assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
        assert!(matches!(poll_once(&mut requested), Poll::Pending));
        assert!(matches!(poll_once(&mut visible), Poll::Pending));

        drop(permit0);
        assert!(matches!(poll_once(&mut background), Poll::Pending));
        assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
        assert!(matches!(poll_once(&mut requested), Poll::Pending));
        let visible_permit = match poll_once(&mut visible) {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("Visible must be granted next: it is the highest-priority queued waiter"),
        };

        drop(visible_permit);
        assert!(matches!(poll_once(&mut background), Poll::Pending));
        assert!(matches!(poll_once(&mut prefetch), Poll::Pending));
        let requested_permit = match poll_once(&mut requested) {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("Requested must be granted next"),
        };

        drop(requested_permit);
        assert!(matches!(poll_once(&mut background), Poll::Pending));
        let prefetch_permit = match poll_once(&mut prefetch) {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("Prefetch must be granted next"),
        };

        drop(prefetch_permit);
        let background_permit = match poll_once(&mut background) {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("Background must finally be granted once everything else has drained"),
        };
        drop(background_permit);
    }

    #[test]
    fn bounded_demand_reports_capacity_and_in_flight() {
        let demand = BoundedDemand::new(2);
        assert_eq!(demand.capacity(), 2);
        assert_eq!(demand.in_flight(), 0);
        let permit_a = semio_framework_async::block_on(demand.acquire(LoadPriority::Requested));
        assert_eq!(demand.in_flight(), 1);
        let permit_b = semio_framework_async::block_on(demand.acquire(LoadPriority::Requested));
        assert_eq!(demand.in_flight(), 2);
        drop(permit_a);
        assert_eq!(demand.in_flight(), 1);
        drop(permit_b);
        assert_eq!(demand.in_flight(), 0);
    }
    //#endregion 🔖️Backpressure
}
//#endregion 🧪️Tests
