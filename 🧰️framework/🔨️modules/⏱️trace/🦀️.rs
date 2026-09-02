//! ⏱️ Interactivity tracing/observability primitives for the Semio interactive job runtime: latency
//! instrumentation ([`StepTimer`]/[`CallbackTimer`] RAII guards, a hard [`INTERACTIVE_STEP_CEILING_US`]
//! plus per-family soft targets, a [`Watchdog`] that reports [`ContractViolation`]s on overrun, and a
//! [`PercentileRing`] per labelled site), cheap process-wide thread-role identity
//! ([`ThreadRole::Ui`]/[`ThreadRole::Worker`]/[`ThreadRole::IoBoundary`]), lock-free active-worker/permit/queue counters
//! ([`WorkerCounters`]/[`PermitLedger`]/[`QueueCounter`]), and a bounded operation-[`TraceEvent`] ring
//! (started/stage-changed/preview-published/checkpoint/commit/cancelled/failed) with preview- and
//! cancellation-latency queries. Every later packet of INTERACTIVE-JOB-RUNTIME-REFACTOR (design ticket
//! `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, this is packet P0a) depends on this module to answer
//! "did this stay interactive" instead of instrumenting its own ad hoc timers.
//!
//! 🧬 **Zero dependencies**: pure `std`, no `serde`/`thiserror`/`owned schema exporter`/anything else — this is a leaf
//! instrumentation primitive every other crate may end up depending on, so it must never widen anyone
//! else's dependency graph.
//!
//! 🕰️ **Clock**: `std::time::Instant` doesn't exist (usably) on `wasm32-unknown-unknown` without WASI
//! p2. The embedding host installs one actual monotonic source shared by tracing and job deadlines.
//! Missing or backward readings produce an exact callback fault, never synthetic time. Telemetry is
//! optional and nonblocking; [`CallbackVerdict`] is the caller-owned quarantine authority.
//!
//! 🚫️async: this crate deliberately writes NO `async fn` anywhere, breaking with the rest of the
//! repo's universal-async convention (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
//! MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📌️important.md`'s R9/E1/E5 exception classes). Every public
//! fn here is either reached from `Drop::drop` (a fixed sync external-trait signature — the same E1
//! class as `CancelToken`'s `Debug` impl and `ActorMetrics::wall_us_p95` in `⏳️async`/`🎭️actor`'s
//! `🦀️.rs`) or is itself a cheap hot-path identity/atomic primitive meant to be callable from
//! ANY context, including one with no executor running yet (the same R9 "pure, zero-I/O, sync-only
//! consumer" class, generalized: here the consumer is "arbitrary caller", not one specific closure).
//! Wrapping either shape in `async fn` would force every caller to own an executor just to check "am I
//! the UI thread" or drop a timer, defeating this module's entire purpose as a zero-overhead,
//! always-available probe.

use std::cell::Cell;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};

//#region 🕰️Clock
/// 🕰️ Host-installable monotonic-microseconds clock override, checked by [`now_us`] before falling
/// back to [`default_clock_us`]. Set at most meaningfully once, at host startup — mirrors
/// `HostAsyncRuntime::now_ms`'s "the implementation supplies the clock" seam, just via a free fn
/// instead of a trait method (this crate has no runtime/host object to hang a trait on).
static CLOCK_OVERRIDE: OnceLock<fn() -> Option<u64>> = OnceLock::new();

/// 🕰️ Installs one real monotonic-microseconds source shared by deadlines and watchdogs.
pub fn install_clock(clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    install_exact_clock(&CLOCK_OVERRIDE, clock)
}

fn install_exact_clock(authority: &OnceLock<fn() -> Option<u64>>, clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    let existing = authority.get_or_init(|| clock);
    if std::ptr::fn_addr_eq(*existing, clock) { Ok(()) } else { Err(clock) }
}

/// 🌐️ Converts fractional platform milliseconds into checked unsigned microseconds.
pub fn microseconds_from_milliseconds(milliseconds: f64) -> Option<u64> {
    let microseconds = milliseconds * 1_000.0;
    (microseconds.is_finite() && microseconds >= 0.0 && microseconds < 18_446_744_073_709_551_616.0).then(|| microseconds.floor() as u64)
}

/// 🕰️ Native and WASI p2 clocks preserve their actual monotonic microsecond precision.
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
fn default_clock_us() -> Option<u64> {
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    u64::try_from(START.get_or_init(std::time::Instant::now).elapsed().as_micros()).ok()
}

/// 🔒️ Bare Wasm must install its embedding host's real clock before instrumentation or work.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn default_clock_us() -> Option<u64> { None }

/// 🕰️ Current monotonic microsecond reading: the host's [`install_clock`] override if one was set,
/// else [`default_clock_us`] for this target.
pub fn try_now_us() -> Option<u64> {
    match CLOCK_OVERRIDE.get() {
        Some(clock) => clock(),
        None => default_clock_us(),
    }
}

/// 🕰️ Instrumentation requires the same installed real clock as interactive deadlines.
pub fn now_us() -> u64 {
    try_now_us().expect("real monotonic microsecond clock must be installed before instrumentation")
}
//#endregion 🕰️Clock

//#region 🔖️Constants
/// 🚨️ Hard ceiling, in microseconds, for a single interactive step — [`Watchdog`] reports a
/// [`ContractViolation`] once elapsed time crosses this, regardless of which [`InteractiveStage`] the
/// site belongs to.
pub const INTERACTIVE_STEP_CEILING_US: u64 = 8_000;

/// 🚨️ Interactive callbacks must finish strictly below the shared eight-millisecond ceiling.
pub fn interactive_step_contract_violated(elapsed_us: u64) -> bool {
    elapsed_us >= INTERACTIVE_STEP_CEILING_US
}

/// 🎯️ Soft target for a UI event handler (input → dispatch), in microseconds.
pub const UI_EVENT_SOFT_TARGET_US: u64 = 1_000;
/// 🎯️ Soft target for a UI present/paint, in microseconds.
pub const UI_PRESENT_SOFT_TARGET_US: u64 = 2_000;
/// 🎯️ Soft target for one interactive-lane job step, in microseconds.
pub const INTERACTIVE_STEP_SOFT_TARGET_US: u64 = 1_000;
/// 🎯️ Soft target for a user-visible simulation step, in microseconds.
pub const USER_VISIBLE_SIM_STEP_SOFT_TARGET_US: u64 = 2_000;
/// 🎯️ Soft target for a background-lane step, in microseconds.
pub const BACKGROUND_STEP_SOFT_TARGET_US: u64 = 4_000;

/// 🎭️ Which family of interactive contract a [`Watchdog`]-wrapped site belongs to — each maps to its
/// own [`InteractiveStage::soft_target_us`] below; every kind still shares the one hard
/// [`INTERACTIVE_STEP_CEILING_US`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractiveStage {
    UiEvent,
    UiPresent,
    InteractiveStep,
    UserVisibleSimStep,
    BackgroundStep,
}

impl InteractiveStage {
    /// 🎯️ This stage's own soft target, in microseconds — see the per-kind constants above.
    pub const fn soft_target_us(self) -> u64 {
        match self {
            InteractiveStage::UiEvent => UI_EVENT_SOFT_TARGET_US,
            InteractiveStage::UiPresent => UI_PRESENT_SOFT_TARGET_US,
            InteractiveStage::InteractiveStep => INTERACTIVE_STEP_SOFT_TARGET_US,
            InteractiveStage::UserVisibleSimStep => USER_VISIBLE_SIM_STEP_SOFT_TARGET_US,
            InteractiveStage::BackgroundStep => BACKGROUND_STEP_SOFT_TARGET_US,
        }
    }
}
//#endregion 🔖️Constants

//#region 🔄️BoundedRing
/// 🔄️ Generic fixed-capacity overwrite-oldest ring, the same shape as
/// `ActorMetrics::wall_us_ring`/[`PercentileRing`] generalized over `T: Copy` so it isn't duplicated
/// per event type — shared by [`Watchdog`]'s violation store and the [`TraceEvent`] store below.
struct BoundedRing<T: Copy, const N: usize> {
    entries: [Option<T>; N],
    pos: usize,
    len: usize,
    total: u64,
}

impl<T: Copy, const N: usize> BoundedRing<T, N> {
    const fn new() -> BoundedRing<T, N> {
        BoundedRing { entries: [None; N], pos: 0, len: 0, total: 0 }
    }

    fn push(&mut self, value: T) {
        self.entries[self.pos] = Some(value);
        self.pos = (self.pos + 1) % N;
        self.len = (self.len + 1).min(N);
        self.total = self.total.saturating_add(1);
    }

    /// 📸️ Every retained entry, oldest first.
    fn snapshot(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len);
        if self.len < N {
            out.extend(self.entries[..self.len].iter().flatten().copied());
        } else {
            out.extend(self.entries[self.pos..].iter().flatten().copied());
            out.extend(self.entries[..self.pos].iter().flatten().copied());
        }
        out
    }

    /// 🔢️ Total ever pushed — unlike [`BoundedRing::snapshot`], never shrunk by eviction.
    fn total(&self) -> u64 {
        self.total
    }

    fn clear(&mut self) {
        *self = BoundedRing::new();
    }
}
//#endregion 🔄️BoundedRing

//#region 📈️PercentileRing
/// 📈️ Ring capacity for one site's [`PercentileRing`] — chosen to match
/// `ActorMetrics::WALL_US_RING_CAPACITY` (`🧰️framework/🔨️modules/🎭️actor/🦀️.rs`), the
/// precedent this type mirrors.
const SAMPLE_RING_CAPACITY: usize = 64;

/// 📈️ Fixed-capacity ring of the last [`SAMPLE_RING_CAPACITY`] microsecond samples for one labelled
/// site, with p50/p95/p99 accessors. Mirrors `ActorMetrics::wall_us_ring`/`wall_us_p95`
/// (`🧰️framework/🔨️modules/🎭️actor/🦀️.rs`) — array-of-`u32` plus sort-on-read, no
/// dependency on that crate — rather than inventing a different percentile strategy.
#[derive(Clone, Debug)]
pub struct PercentileRing {
    samples: [u32; SAMPLE_RING_CAPACITY],
    len: u8,
    pos: u8,
}

impl PercentileRing {
    pub const fn new() -> PercentileRing {
        PercentileRing { samples: [0; SAMPLE_RING_CAPACITY], len: 0, pos: 0 }
    }

    pub fn record(&mut self, value_us: u64) {
        self.samples[self.pos as usize] = value_us.min(u32::MAX as u64) as u32;
        self.pos = (self.pos + 1) % SAMPLE_RING_CAPACITY as u8;
        self.len = (self.len + 1).min(SAMPLE_RING_CAPACITY as u8);
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // 🚫️async: E1 pure computation over already-recorded in-memory samples — same class as
    // `ActorMetrics::wall_us_p95`, which this mirrors.
    fn percentile(&self, p: f32) -> u32 {
        if self.len == 0 {
            return 0;
        }
        let n = self.len as usize;
        let mut sorted = self.samples;
        let slice = &mut sorted[..n];
        slice.sort_unstable();
        let idx = ((n as f32) * p).floor() as usize;
        slice[idx.min(n - 1)]
    }

    pub fn p50(&self) -> u32 {
        self.percentile(0.50)
    }

    pub fn p95(&self) -> u32 {
        self.percentile(0.95)
    }

    pub fn p99(&self) -> u32 {
        self.percentile(0.99)
    }
}

impl Default for PercentileRing {
    fn default() -> PercentileRing {
        PercentileRing::new()
    }
}

const SITE_CAPACITY: usize = 256;
type SiteRegistry = [Option<(&'static str, PercentileRing)>; SITE_CAPACITY];
static OMITTED_SITE_SAMPLES: AtomicU64 = AtomicU64::new(0);
static OMITTED_EVENTS: AtomicU64 = AtomicU64::new(0);
static OMITTED_VIOLATIONS: AtomicU64 = AtomicU64::new(0);

/// 📇️ Fixed static backing admits optional site samples without a lazy allocation.
fn site_registry() -> &'static Mutex<SiteRegistry> {
    static REGISTRY: Mutex<SiteRegistry> = Mutex::new([const { None }; SITE_CAPACITY]);
    &REGISTRY
}

fn record_site_sample(site: &'static str, elapsed_us: u64) {
    let Ok(mut registry) = site_registry().try_lock() else { OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed); return; };
    let mut vacant = None;
    for (index, slot) in registry.iter_mut().enumerate() {
        match slot {
            Some((label, ring)) if std::ptr::eq(*label, site) => { ring.record(elapsed_us); return; }
            None if vacant.is_none() => vacant = Some(index),
            _ => {}
        }
    }
    if let Some(index) = vacant {
        let mut ring = PercentileRing::new();
        ring.record(elapsed_us);
        registry[index] = Some((site, ring));
    } else { OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed); }
}

/// 📊️ `(p50, p95, p99)` microseconds recorded for `site`, or `None` if nothing has landed there yet.
pub fn site_percentiles(site: &str) -> Option<(u32, u32, u32)> {
    let registry = site_registry().lock().unwrap_or_else(PoisonError::into_inner);
    registry.iter().flatten().find(|(label, _)| *label == site).map(|(_, ring)| (ring.p50(), ring.p95(), ring.p99()))
}

/// 📉️ Optional telemetry omissions never replace an exact callback verdict.
pub fn telemetry_omitted_counts() -> (u64, u64, u64) {
    (OMITTED_SITE_SAMPLES.load(Ordering::Relaxed), OMITTED_EVENTS.load(Ordering::Relaxed), OMITTED_VIOLATIONS.load(Ordering::Relaxed))
}
//#endregion 📈️PercentileRing

//#region ⏱️Timers
/// ⏱️ RAII guard for a plain latency sample: records elapsed microseconds into `site`'s
/// [`PercentileRing`] when dropped. No operation context and no overrun reporting — that's
/// [`Watchdog`]'s job, layered on the same per-site ring. Use this for a step with no single owning
/// operation (a per-frame simulation tick, a render pass).
pub struct StepTimer {
    site: &'static str,
    start_us: Option<u64>,
}

impl StepTimer {
    /// ▶️ Starts timing `site` from [`now_us`].
    pub fn start(site: &'static str) -> StepTimer {
        StepTimer { site, start_us: try_now_us() }
    }
}

impl Drop for StepTimer {
    // 🚫️async: E1 external-trait impl — `Drop::drop`'s signature is fixed by std, so this can never
    // `.await`; same reasoning as `CancelToken`'s `Debug::fmt` impl in `⏳️async/🦀️.rs`.
    fn drop(&mut self) {
        if let Some(elapsed) = self.start_us.zip(try_now_us()).and_then(|(start, end)| end.checked_sub(start)) { record_site_sample(self.site, elapsed); }
        else { OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed); }
    }
}

/// ⏱️ Same shape as [`StepTimer`], named separately for callback/event-handler call sites (a UI event
/// dispatch, a promise resolution) so a latency report reads by call-site intent instead of forcing
/// every caller to squint at one generic timer type.
// 🔕 dead_code: this field is never read directly — it exists purely for its `Drop` side effect,
// same as any RAII guard whose value nobody inspects.
#[allow(dead_code)]
pub struct CallbackTimer(StepTimer);

impl CallbackTimer {
    pub fn start(site: &'static str) -> CallbackTimer {
        CallbackTimer(StepTimer::start(site))
    }
}
//#endregion ⏱️Timers

//#region 🐕️Watchdog
/// 🚨️ What [`Watchdog::violations`] reports once a wrapped site's elapsed time crosses
/// [`INTERACTIVE_STEP_CEILING_US`] — enough to name the offending site, correlate it back to the
/// operation/generation in flight, and quarantine it afterward.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractViolation {
    pub site: &'static str,
    pub operation: OperationId,
    pub generation: Generation,
    pub stage: InteractiveStage,
    pub elapsed_us: u64,
}

/// 🕰️ A missing or backward monotonic reading denies callback publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallbackClockFault { Missing, Backward }

/// 🔒️ Immutable verdict minted only by its exact callback guard, independent of telemetry storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackVerdict {
    site: &'static str,
    operation: OperationId,
    generation: Generation,
    stage: InteractiveStage,
    elapsed: Result<u64, CallbackClockFault>,
}

impl CallbackVerdict {
    pub fn is_fault(&self) -> bool { self.elapsed.is_err() || self.elapsed.is_ok_and(interactive_step_contract_violated) }
    pub fn elapsed_us(&self) -> Option<u64> { self.elapsed.ok() }
    pub fn clock_fault(&self) -> Option<CallbackClockFault> { self.elapsed.err() }
    pub fn operation(&self) -> OperationId { self.operation }
    pub fn generation(&self) -> Generation { self.generation }
    pub fn violation(&self) -> Option<ContractViolation> {
        self.elapsed.ok().filter(|elapsed| interactive_step_contract_violated(*elapsed)).map(|elapsed_us| ContractViolation { site: self.site, operation: self.operation, generation: self.generation, stage: self.stage, elapsed_us })
    }
}

/// 🚨️ Ring capacity for the process-wide violation store — bounded so a runaway-overrunning site
/// cannot grow this unboundedly; oldest violations are evicted first (see [`Watchdog::violation_count`]
/// for the never-shrinking total).
const VIOLATION_RING_CAPACITY: usize = 128;

fn violation_ring() -> &'static Mutex<BoundedRing<ContractViolation, VIOLATION_RING_CAPACITY>> {
    static RING: Mutex<BoundedRing<ContractViolation, VIOLATION_RING_CAPACITY>> = Mutex::new(BoundedRing::new());
    &RING
}

/// 🐕️ RAII guard: wraps one [`InteractiveStage`] site for a specific operation/generation, records the
/// elapsed sample into the same per-site [`PercentileRing`] [`StepTimer`] uses, and reports a
/// [`ContractViolation`] at the strict ceiling. [`Watchdog::finish`] transfers the exact verdict;
/// global snapshots are optional diagnostics and must never authorize publication or quarantine.
pub struct Watchdog {
    site: &'static str,
    operation: OperationId,
    generation: Generation,
    stage: InteractiveStage,
    start_us: Option<u64>,
    finished: bool,
}

impl Watchdog {
    pub fn start(site: &'static str, operation: OperationId, generation: Generation, stage: InteractiveStage) -> Watchdog {
        Watchdog { site, operation, generation, stage, start_us: try_now_us(), finished: false }
    }

    pub fn finish(mut self) -> CallbackVerdict {
        self.finished = true;
        self.report(try_now_us())
    }

    pub fn is_admitted(&self) -> bool { self.start_us.is_some() }

    fn verdict_at(&self, end_us: Option<u64>) -> CallbackVerdict {
        let elapsed = match (self.start_us, end_us) {
            (Some(start), Some(end)) => end.checked_sub(start).ok_or(CallbackClockFault::Backward),
            _ => Err(CallbackClockFault::Missing),
        };
        CallbackVerdict { site: self.site, operation: self.operation, generation: self.generation, stage: self.stage, elapsed }
    }

    fn report(&self, end_us: Option<u64>) -> CallbackVerdict {
        let verdict = self.verdict_at(end_us);
        if let Some(elapsed) = verdict.elapsed_us() { record_site_sample(self.site, elapsed); }
        if let Some(violation) = verdict.violation() {
            if let Ok(mut ring) = violation_ring().try_lock() { ring.push(violation); }
            else { OMITTED_VIOLATIONS.fetch_add(1, Ordering::Relaxed); }
        }
        verdict
    }

    /// 📸️ Every violation recorded so far, oldest first, capped at the last [`VIOLATION_RING_CAPACITY`].
    pub fn violations() -> Vec<ContractViolation> {
        violation_ring().lock().unwrap_or_else(PoisonError::into_inner).snapshot()
    }

    /// 🔢️ Total violations ever recorded — unlike [`Watchdog::violations`], never shrunk by ring
    /// eviction, so a long-running host can still see "how bad has it been" after the ring wraps.
    pub fn violation_count() -> u64 {
        violation_ring().lock().unwrap_or_else(PoisonError::into_inner).total()
    }

    /// 🧹️ Resets the violation store — for test isolation and for a host that has finished quarantining
    /// every reported offender and wants a clean slate.
    pub fn clear() {
        violation_ring().lock().unwrap_or_else(PoisonError::into_inner).clear();
    }
}

impl Drop for Watchdog {
    // 🚫️async: E1 external-trait impl, same reasoning as `StepTimer::drop` above.
    fn drop(&mut self) {
        if !self.finished { self.report(try_now_us()); }
    }
}

#[path = "⏱️clock/🏁️tail/🦀️.rs"]
mod watchdog_tail;
pub use watchdog_tail::WatchdogAdmission;
//#endregion 🐕️Watchdog

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
    active: AtomicU32,
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
    items: AtomicU64,
    bytes: AtomicU64,
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

//#region 🪪️Operation
/// 🔖️ Opaque per-interactive-operation id — a local, pack-free stand-in distinct from
/// `semio_framework_async::TraceId`/`OperationContext` (same seam discipline: this crate must not
/// depend on the async crate — see that crate's `CapabilityTokenId` doc for the precedent). Allocated
/// via [`allocate_operation_id`], or constructed directly by a caller/test that already owns an id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationId(pub u64);

/// 🔖️ Which retry/replay attempt of an [`OperationId`] a [`TraceEvent`] belongs to — bumped every time
/// the same logical operation restarts (a cancelled-then-retried interactive drag, a checkpoint replay).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(pub u64);

/// 🔖️ Opaque cross-cutting trace correlation id, orthogonal to [`OperationId`] — meant to carry a whole
/// input→frame→operation→preview→commit chain that may span several operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceId(pub u64);

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);

/// 🌱️ A fresh, process-unique [`OperationId`] (never `0`, leaving `OperationId(0)` free for a caller's
/// own "no operation" sentinel).
pub fn allocate_operation_id() -> OperationId {
    OperationId(NEXT_OPERATION_ID.fetch_add(1, Ordering::SeqCst))
}
//#endregion 🪪️Operation

//#region 🛰️Trace
/// 🛰️ What stage of an operation's lifecycle a [`TraceEvent`] marks. [`TraceStage::StageChanged`]
/// carries a `&'static str` label rather than growing a new variant per stage name —
/// [`CANCEL_REQUESTED_STAGE_LABEL`] is the one label this crate itself gives meaning to (for
/// [`cancellation_latency_us`]); every other label is caller-defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceStage {
    Started,
    StageChanged { label: &'static str },
    PreviewPublished,
    Checkpoint,
    Committed,
    Cancelled,
    Failed,
}

/// 🏷️ The [`TraceStage::StageChanged`] label [`record_cancel_requested`] uses, so
/// [`cancellation_latency_us`] can find "cancel requested" without a dedicated [`TraceStage`] variant.
pub const CANCEL_REQUESTED_STAGE_LABEL: &str = "cancel_requested";

/// 🛰️ One entry in the process-wide operation-trace ring: an [`OperationId`]/[`Generation`] pair, a
/// monotonic `sequence` (for total order even when two events share a microsecond), the [`TraceStage`],
/// and the [`now_us`] timestamp it was recorded at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceEvent {
    pub operation: OperationId,
    pub generation: Generation,
    pub sequence: u64,
    pub stage: TraceStage,
    pub at_us: u64,
}

/// 🛰️ Ring capacity for the process-wide trace-event store.
const TRACE_RING_CAPACITY: usize = 4096;

static NEXT_SEQUENCE: AtomicU64 = AtomicU64::new(1);

fn trace_ring() -> &'static Mutex<BoundedRing<TraceEvent, TRACE_RING_CAPACITY>> {
    static RING: Mutex<BoundedRing<TraceEvent, TRACE_RING_CAPACITY>> = Mutex::new(BoundedRing::new());
    &RING
}

fn push_trace_event(operation: OperationId, generation: Generation, stage: TraceStage) -> Option<TraceEvent> {
    let Some(at_us) = try_now_us() else { OMITTED_EVENTS.fetch_add(1, Ordering::Relaxed); return None; };
    let event = TraceEvent { operation, generation, sequence: NEXT_SEQUENCE.fetch_add(1, Ordering::SeqCst), stage, at_us };
    if let Ok(mut ring) = trace_ring().try_lock() { ring.push(event); }
    else { OMITTED_EVENTS.fetch_add(1, Ordering::Relaxed); }
    Some(event)
}

pub fn record_operation_started(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Started)
}

pub fn record_stage_changed(operation: OperationId, generation: Generation, label: &'static str) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::StageChanged { label })
}

/// 🏷️ Convenience over [`record_stage_changed`] with [`CANCEL_REQUESTED_STAGE_LABEL`] — the start point
/// [`cancellation_latency_us`] measures from.
pub fn record_cancel_requested(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    record_stage_changed(operation, generation, CANCEL_REQUESTED_STAGE_LABEL)
}

pub fn record_preview_published(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::PreviewPublished)
}

pub fn record_checkpoint(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Checkpoint)
}

pub fn record_committed(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Committed)
}

/// 🏁️ The terminal "cancel observed" event — the end point [`cancellation_latency_us`] measures to.
pub fn record_cancelled(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Cancelled)
}

pub fn record_failed(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Failed)
}

/// 📸️ Every trace event recorded so far, oldest first, capped at the last [`TRACE_RING_CAPACITY`].
pub fn trace_snapshot() -> Vec<TraceEvent> {
    trace_ring().lock().unwrap_or_else(PoisonError::into_inner).snapshot()
}

/// 📸️ [`trace_snapshot`] filtered to one operation, in recorded order — the end-to-end
/// input/frame/operation/preview/commit trail later phases follow by id.
pub fn trace_snapshot_for(operation: OperationId) -> Vec<TraceEvent> {
    trace_snapshot().into_iter().filter(|event| event.operation == operation).collect()
}

fn stage_changed_label(stage: &TraceStage) -> Option<&'static str> {
    match stage {
        TraceStage::StageChanged { label } => Some(label),
        _ => None,
    }
}

/// ⏱️ Microseconds from `operation`'s [`TraceStage::Started`] event to its first
/// [`TraceStage::PreviewPublished`] event, or `None` if either hasn't happened yet.
pub fn preview_latency_us(operation: OperationId) -> Option<u64> {
    let events = trace_snapshot_for(operation);
    let started = events.iter().find(|event| event.stage == TraceStage::Started)?;
    let preview = events.iter().find(|event| event.stage == TraceStage::PreviewPublished)?;
    Some(preview.at_us.saturating_sub(started.at_us))
}

/// ⏱️ Microseconds from `operation`'s cancel-requested [`TraceStage::StageChanged`] (see
/// [`record_cancel_requested`]) to its terminal [`TraceStage::Cancelled`] event, or `None` if either
/// hasn't happened yet.
pub fn cancellation_latency_us(operation: OperationId) -> Option<u64> {
    let events = trace_snapshot_for(operation);
    let requested = events.iter().find(|event| stage_changed_label(&event.stage) == Some(CANCEL_REQUESTED_STAGE_LABEL))?;
    let observed = events.iter().find(|event| event.stage == TraceStage::Cancelled)?;
    Some(observed.at_us.saturating_sub(requested.at_us))
}
//#endregion 🛰️Trace

//#region 🧪️Tests
#[cfg(test)]
#[path = "⏱️clock/🦀️.rs"]
mod microsecond_clock_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "⏱️clock/🧪️contention/🦀️.rs"]
mod microsecond_telemetry_contention_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "⏱️clock/🏁️tail/🧪️tests/🦀️.rs"]
mod watchdog_tail_tests;

#[cfg(test)]
mod tests {
    use super::*;

    //#region 📈️PercentileRing
    #[test]
    fn percentile_ring_orders_samples_correctly() {
        let mut ring = PercentileRing::new();
        for value in [100u64, 200, 300, 400, 500, 600, 700, 800, 900, 1000] {
            ring.record(value);
        }
        assert_eq!(ring.len(), 10);
        assert!(!ring.is_empty());
        assert_eq!(ring.p50(), 600);
        assert_eq!(ring.p95(), 1000);
        assert_eq!(ring.p99(), 1000);
    }

    #[test]
    fn percentile_ring_wraps_past_capacity_keeping_newest() {
        let mut ring = PercentileRing::new();
        let total = SAMPLE_RING_CAPACITY as u64 * 2;
        for value in 0..total {
            ring.record(value);
        }
        assert_eq!(ring.len(), SAMPLE_RING_CAPACITY);
        let oldest_retained = total - SAMPLE_RING_CAPACITY as u64;
        let n = SAMPLE_RING_CAPACITY as u64;
        assert_eq!(ring.p50() as u64, oldest_retained + n / 2);
        assert_eq!(ring.p99() as u64, oldest_retained + n - 1);
    }

    #[test]
    fn percentile_ring_empty_reads_as_zero() {
        let ring = PercentileRing::new();
        assert!(ring.is_empty());
        assert_eq!(ring.p50(), 0);
        assert_eq!(ring.p95(), 0);
        assert_eq!(ring.p99(), 0);
    }
    //#endregion 📈️PercentileRing

    //#region 🐕️Watchdog
    #[test]
    fn watchdog_reports_contract_violation_on_overrun() {
        let before = Watchdog::violation_count();
        let operation = allocate_operation_id();
        let generation = Generation(1);
        {
            let _guard = Watchdog::start("test.watchdog.overrun", operation, generation, InteractiveStage::InteractiveStep);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(Watchdog::violation_count() > before, "expected a contract violation to be recorded");
        let violation = Watchdog::violations().into_iter().rev().find(|violation| violation.operation == operation).expect("violation for this operation must be queryable");
        assert_eq!(violation.site, "test.watchdog.overrun");
        assert_eq!(violation.stage, InteractiveStage::InteractiveStep);
        assert!(violation.elapsed_us >= INTERACTIVE_STEP_CEILING_US);
    }

    #[test]
    fn watchdog_stays_silent_under_ceiling() {
        let before = Watchdog::violation_count();
        let operation = allocate_operation_id();
        {
            let _guard = Watchdog::start("test.watchdog.under-ceiling", operation, Generation(1), InteractiveStage::UiEvent);
        }
        assert_eq!(Watchdog::violation_count(), before, "a fast guard must never be reported as a violation");
    }
    //#endregion 🐕️Watchdog

    //#region 🧵️ThreadRole
    #[test]
    fn thread_role_registers_and_asserts() {
        register_worker_thread(7);
        assert_eq!(current_role(), ThreadRole::Worker(7));
        assert!(is_worker_thread());
        assert!(!is_ui_thread());
        assert_worker_thread();

        register_ui_thread();
        assert_eq!(current_role(), ThreadRole::Ui);
        assert!(is_ui_thread());
        assert!(!is_worker_thread());
        assert_ui_thread();
    }

    #[test]
    fn io_boundary_thread_registers_distinct_from_worker_and_ui() {
        register_io_boundary_thread("process-shard-reader");
        assert_eq!(current_role(), ThreadRole::IoBoundary("process-shard-reader"));
        assert!(is_io_boundary_thread());
        assert!(!is_worker_thread());
        assert!(!is_ui_thread());
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "not the UI thread")]
    fn assert_ui_thread_panics_off_ui_thread() {
        register_worker_thread(3);
        assert_ui_thread();
    }
    //#endregion 🧵️ThreadRole

    //#region 📊️Counters
    #[test]
    fn counters_snapshot_reflects_updates() {
        let workers = WorkerCounters::new();
        assert_eq!(workers.worker_started(), 1);
        assert_eq!(workers.worker_started(), 2);
        assert_eq!(workers.worker_finished(), 1);
        assert_eq!(workers.active(), 1);

        let permits = PermitLedger::new();
        assert_eq!(permits.acquire(), 1);
        assert_eq!(permits.release(), 0);
        assert_eq!(permits.occupancy(), 0);

        let queue = QueueCounter::new();
        assert_eq!(queue.enqueued(64), QueueCounterSnapshot { items: 1, bytes: 64 });
        assert_eq!(queue.enqueued(32), QueueCounterSnapshot { items: 2, bytes: 96 });
        assert_eq!(queue.dequeued(64), QueueCounterSnapshot { items: 1, bytes: 32 });
        assert_eq!(queue.snapshot(), QueueCounterSnapshot { items: 1, bytes: 32 });
    }
    //#endregion 📊️Counters

    //#region 🛰️Trace
    #[test]
    fn trace_follows_one_operation_start_to_preview_to_commit() {
        let operation = allocate_operation_id();
        let generation = Generation(1);
        record_operation_started(operation, generation);
        record_stage_changed(operation, generation, "gathering-input");
        record_preview_published(operation, generation);
        record_checkpoint(operation, generation);
        record_committed(operation, generation);

        let events = trace_snapshot_for(operation);
        let stages: Vec<TraceStage> = events.iter().map(|event| event.stage).collect();
        assert_eq!(stages, vec![TraceStage::Started, TraceStage::StageChanged { label: "gathering-input" }, TraceStage::PreviewPublished, TraceStage::Checkpoint, TraceStage::Committed]);
        assert!(events.windows(2).all(|pair| pair[0].sequence < pair[1].sequence));
        assert!(events.iter().all(|event| event.generation == generation));

        assert!(preview_latency_us(operation).is_some());
    }

    #[test]
    fn cancellation_latency_measures_requested_to_observed() {
        let operation = allocate_operation_id();
        let generation = Generation(1);
        record_operation_started(operation, generation);
        record_cancel_requested(operation, generation);
        std::thread::sleep(std::time::Duration::from_millis(1));
        record_cancelled(operation, generation);

        let latency = cancellation_latency_us(operation).expect("cancellation latency must be queryable end to end");
        assert!(latency > 0, "expected the sleep between cancel-requested and cancelled to show up");
    }

    #[test]
    fn latency_helpers_are_none_before_their_events_land() {
        let operation = allocate_operation_id();
        assert_eq!(preview_latency_us(operation), None);
        assert_eq!(cancellation_latency_us(operation), None);
    }
    //#endregion 🛰️Trace

    //#region 🕰️Clock
    #[test]
    fn clock_is_monotonically_non_decreasing() {
        let first = now_us();
        let second = now_us();
        assert!(second >= first);
    }
    //#endregion 🕰️Clock
}
//#endregion 🧪️Tests
