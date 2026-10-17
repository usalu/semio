//! 🧵️ Thread roles and counters of the trace primitives: the process-wide thread role registry and the worker, permit and queue counters. See [`super`].

use super::*;

//#region 🧵️ThreadRole
/// 🧵️ Which role the calling OS thread was registered under — [`ThreadRole::Unknown`] until
/// [`register_ui_thread`]/[`register_worker_thread`]/[`register_io_boundary_thread`] is called on
/// that same thread. [`ThreadRole::IoBoundary`] (P1f) is a THIRD, deliberately distinct role from
/// [`ThreadRole::Worker`]: a thread census must be able to tell "a `WorkerPool` worker, indexed and
/// counted by that pool" apart from "a platform I/O boundary thread this pool cannot absorb (a
/// genuinely blocking OS pipe read with no non-blocking alternative), counted separately and bounded
/// per its own site" — collapsing the two into one `Worker(_)` bucket would hide the real pool size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThreadRole {
    Ui,
    Worker(u32),
    IoBoundary(&'static str),
    Unknown,
}

thread_local! {
    static THREAD_ROLE: Cell<ThreadRole> = const { Cell::new(ThreadRole::Unknown) };
}

/// 🖱️ Marks the CALLING thread as the UI thread. Call once, from the UI thread itself, at startup.
pub fn register_ui_thread() {
    THREAD_ROLE.with(|role| role.set(ThreadRole::Ui));
}

/// 🧵️ Marks the calling thread as worker `index`. Call once, from that worker thread itself, at
/// startup.
pub fn register_worker_thread(index: u32) {
    THREAD_ROLE.with(|role| role.set(ThreadRole::Worker(index)));
}

/// 🚪️ Marks the calling thread as a bounded, documented platform I/O boundary (`site`, e.g.
/// `"process-shard-reader"`) — never a `WorkerPool` worker and never running domain logic, just a
/// blocking OS read/write the pool cannot absorb. Call once, from that thread itself, at startup.
pub fn register_io_boundary_thread(site: &'static str) {
    THREAD_ROLE.with(|role| role.set(ThreadRole::IoBoundary(site)));
}

/// 🔍️ The calling thread's registered role — a cheap thread-local read, available in every build.
pub fn current_role() -> ThreadRole {
    THREAD_ROLE.with(Cell::get)
}

/// 🔍️ Non-panicking query: is the calling thread the registered UI thread?
pub fn is_ui_thread() -> bool {
    matches!(current_role(), ThreadRole::Ui)
}

/// 🔍️ Non-panicking query: is the calling thread a registered worker (any index)?
pub fn is_worker_thread() -> bool {
    matches!(current_role(), ThreadRole::Worker(_))
}

/// 🔍️ Non-panicking query: is the calling thread a registered platform I/O boundary (any site)?
pub fn is_io_boundary_thread() -> bool {
    matches!(current_role(), ThreadRole::IoBoundary(_))
}

/// 🚨️ Debug-only tripwire (`debug_assert!`, compiled to nothing in release): panics if the calling
/// thread is not the registered UI thread.
pub fn assert_ui_thread() {
    debug_assert!(is_ui_thread(), "assert_ui_thread: called from {:?}, not the UI thread", current_role());
}

/// 🚨️ Debug-only tripwire (`debug_assert!`, compiled to nothing in release): panics if the calling
/// thread is not a registered worker thread.
pub fn assert_worker_thread() {
    debug_assert!(is_worker_thread(), "assert_worker_thread: called from {:?}, not a worker thread", current_role());
}
//#endregion 🧵️ThreadRole

//#region 📊️Counters
/// 📊️ Lock-free active-worker count. `worker_started`/`worker_finished` return the count AFTER the
/// update, so callers get an atomic read-modify-write without a separate `.active()` call.
#[derive(Debug, Default)]
pub struct WorkerCounters {
    pub(super) active: AtomicU32,
}

impl WorkerCounters {
    pub const fn new() -> WorkerCounters {
        WorkerCounters { active: AtomicU32::new(0) }
    }

    pub fn worker_started(&self) -> u32 {
        self.active.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// ⚖️ Mirrors `ThreadBudget::checkout`'s overdraw tripwire
    /// (`🧰️framework/🔨️modules/⏳️async/🦀️.rs`): a release build never panics on an
    /// unbalanced `worker_finished`, it just lets the counter wrap — a development-time tripwire, not a
    /// runtime enforcement mechanism.
    pub fn worker_finished(&self) -> u32 {
        let previous = self.active.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous >= 1, "WorkerCounters::worker_finished: no active worker to finish");
        previous.wrapping_sub(1)
    }

    pub fn active(&self) -> u32 {
        self.active.load(Ordering::SeqCst)
    }
}

/// 📊️ Lock-free permit-ledger occupancy — same shape as [`WorkerCounters`], kept as a separate type so
/// a snapshot never conflates "how many workers are running" with "how many permits are checked out".
#[derive(Debug, Default)]
pub struct PermitLedger {
    occupied: AtomicU32,
}

impl PermitLedger {
    pub const fn new() -> PermitLedger {
        PermitLedger { occupied: AtomicU32::new(0) }
    }

    pub fn acquire(&self) -> u32 {
        self.occupied.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// ⚖️ Same overdraw tripwire as [`WorkerCounters::worker_finished`].
    pub fn release(&self) -> u32 {
        let previous = self.occupied.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous >= 1, "PermitLedger::release: no permit to release");
        previous.wrapping_sub(1)
    }

    pub fn occupancy(&self) -> u32 {
        self.occupied.load(Ordering::SeqCst)
    }
}

/// 📈️ `(items, bytes)` read of a [`QueueCounter`] at one instant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueCounterSnapshot {
    pub items: u64,
    pub bytes: u64,
}

/// 📊️ Lock-free per-queue item/byte counters. One instance per named queue — a caller owns a
/// `static QUEUE: QueueCounter = QueueCounter::new();` per queue site, the same instantiate-per-site
/// shape [`PercentileRing`]'s call sites use logically, except here the caller holds the instance
/// directly rather than this crate keeping a name-keyed registry — a registry would need a lock, and
/// this section's requirement is lock-free atomics throughout.
#[derive(Debug, Default)]
pub struct QueueCounter {
    pub(super) items: AtomicU64,
    pub(super) bytes: AtomicU64,
}

impl QueueCounter {
    pub const fn new() -> QueueCounter {
        QueueCounter { items: AtomicU64::new(0), bytes: AtomicU64::new(0) }
    }

    pub fn enqueued(&self, bytes: u64) -> QueueCounterSnapshot {
        self.items.fetch_add(1, Ordering::SeqCst);
        self.bytes.fetch_add(bytes, Ordering::SeqCst);
        self.snapshot()
    }

    /// ⚖️ Same overdraw tripwire as [`WorkerCounters::worker_finished`] for `items`; `bytes` is left to
    /// wrap on its own since a byte count has no single-unit "overdraw" notion.
    pub fn dequeued(&self, bytes: u64) -> QueueCounterSnapshot {
        let previous_items = self.items.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous_items >= 1, "QueueCounter::dequeued: no item to dequeue");
        self.bytes.fetch_sub(bytes, Ordering::SeqCst);
        self.snapshot()
    }

    pub fn snapshot(&self) -> QueueCounterSnapshot {
        QueueCounterSnapshot { items: self.items.load(Ordering::SeqCst), bytes: self.bytes.load(Ordering::SeqCst) }
    }
}
//#endregion 📊️Counters
