//! 🖥️ Native work-stealing `WorkerPool`: OS worker threads, lane queues, timers and parking.

use std::sync::atomic::AtomicU32;

use super::*;
use std::sync::atomic::AtomicUsize;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// 🧺️ One worker's lane queues plus each queue's length as of its last unlock, so a scan (the
/// owner's or a thief's) skips an empty lane without taking its lock.
#[doc(hidden)]
pub struct WorkerLocal {
    pub queues: [Mutex<VecDeque<Job>>; LANE_COUNT],
    queued: [AtomicUsize; LANE_COUNT],
}

thread_local! {
    /// ⏰️ Set while this thread fires the pool's timer wheel, so a callback that re-arms a
    /// deadline does not wake a second worker to keep a wheel this one is about to re-read.
    static FIRING_TIMERS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

impl WorkerLocal {
    fn new() -> WorkerLocal {
        WorkerLocal { queues: std::array::from_fn(|_| Mutex::new(admitted_job_queue())), queued: std::array::from_fn(|_| AtomicUsize::new(0)) }
    }

    fn publish(&self, lane: Lane, queue: &VecDeque<Job>) {
        self.queued[lane.index()].store(queue.len(), Ordering::Release);
    }

    fn queued(&self, lane: Lane) -> bool {
        self.queued[lane.index()].load(Ordering::Acquire) != 0
    }
}

#[doc(hidden)] pub struct PoolInner {
    pub workers: Vec<WorkerLocal>,
    pub maintenance: WorkerMaintenanceRegistry,
    deferred_wakes: WorkerDeferredWakeRegistry,
    lifecycle: Mutex<PoolLifecycleState>,
    shutdown: std::sync::atomic::AtomicBool,
    next_submit: AtomicUsize,
    io_source: AtomicUsize,
    pub parking: Arc<worker_parking::WorkerParking>,
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
    fn reserve_low_priority(&self) -> Option<LowPriorityPermit<'_>> {
        if !self.interactive_reserve || self.workers.len() < 2 {
            return Some(LowPriorityPermit { inner: self, claimed: false });
        }
        let ceiling = self.workers.len() as u32 - 1;
        let mut current = self.low_priority_active.load(Ordering::SeqCst);
        loop {
            if current >= ceiling {
                return None;
            }
            match self.low_priority_active.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return Some(LowPriorityPermit { inner: self, claimed: true }),
                Err(observed) => current = observed,
            }
        }
    }

    fn now_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    fn timer_due(&self) -> Option<Duration> {
        let now = self.now_ms();
        self.wheel.next_deadline_ms().map(|deadline| Duration::from_millis(deadline.saturating_sub(now)))
    }

    fn notify_idle(&self) {
        self.parking.signal_work();
    }
}

#[doc(hidden)]
pub struct LowPriorityPermit<'a> {
    pub inner: &'a PoolInner,
    claimed: bool,
}

impl Drop for LowPriorityPermit<'_> {
    fn drop(&mut self) {
        if self.claimed {
            self.inner.low_priority_active.fetch_sub(1, Ordering::SeqCst);
            self.inner.notify_idle();
        }
    }
}

/// ⚖️ Deficit-round-robin selection over one worker's OWN lane queues (never a sibling's — see
/// [`steal`] for that). `cursor`/`deficits` are private, un-synchronized state owned entirely by
/// the calling worker thread — no atomics needed, since no other thread ever touches them.
/// `UNIT_COST` is [`Lane::Interactive`]'s own weight (the maximum), so the highest-weight lane
/// is serviced every single scan while lower-weight lanes accrue deficit proportionally slower
/// and are serviced roughly every `UNIT_COST / weight` scans. One selection completes at most
/// `UNIT_COST` rounds before returning idle, so eligible queued work never parks merely to accrue deficit.
fn select_and_pop<'a>(inner: &'a PoolInner, my_index: usize, cursor: &mut usize, deficits: &mut [i64; LANE_COUNT]) -> Option<(Lane, PoolWork, Option<LowPriorityPermit<'a>>)> {
    const UNIT_COST: i64 = Lane::Interactive.weight() as i64;
    if inner.shutdown.load(Ordering::SeqCst) {
        return inner.deferred_wakes.select().map(|wake| (Lane::Io, PoolWork::DeferredWake(wake), None));
    }
    let my = &inner.workers[my_index];
    for _ in 0..LANE_COUNT * UNIT_COST as usize {
        let lane = Lane::ALL[*cursor];
        *cursor = (*cursor + 1) % LANE_COUNT;
        if !my.queued(lane) && !inner.maintenance.has_pending(Some(lane)) && (lane != Lane::Io || !inner.deferred_wakes.has_pending()) {
            deficits[lane.index()] = 0;
            continue;
        }
        let mut queue = my.queues[lane.index()].lock().unwrap_or_else(PoisonError::into_inner);
        deficits[lane.index()] += lane.weight() as i64;
        if deficits[lane.index()] >= UNIT_COST {
            let low_priority_permit = if is_low_priority(lane) {
                match inner.reserve_low_priority() {
                    Some(permit) => Some(permit),
                    None => continue,
                }
            } else {
                None
            };
            let work = if lane == Lane::Io {
                let first = inner.io_source.fetch_add(1, Ordering::SeqCst) % 3;
                (0..3).find_map(|offset| match (first + offset) % 3 {
                    0 => queue.pop_front().map(PoolWork::Job),
                    1 => inner.maintenance.select_hook(lane),
                    _ => inner.deferred_wakes.select().map(PoolWork::DeferredWake),
                })
            } else {
                inner.maintenance.select(lane, &mut queue)
            };
            my.publish(lane, &queue);
            if let Some(work) = work {
                deficits[lane.index()] -= UNIT_COST;
                return Some((lane, work, low_priority_permit));
            }
        }
    }
    None
}

/// 🕵️ Work-stealing: scans every sibling worker (starting just after `my_index`, wrapping) for
/// the first non-empty lane, in [`Lane::ALL`] order, respecting the same admission-control gate
/// as [`select_and_pop`]. Stealing always pops from the FRONT (same end the owner pops from) so
/// a stolen job is still the oldest one queued for that lane on that worker.
fn steal(inner: &PoolInner, my_index: usize) -> Option<(Lane, PoolWork, Option<LowPriorityPermit<'_>>)> {
    let worker_count = inner.workers.len();
    for offset in 1..worker_count {
        let victim = (my_index + offset) % worker_count;
        for lane in Lane::ALL {
            if !inner.workers[victim].queued(lane) {
                continue;
            }
            let mut queue = inner.workers[victim].queues[lane.index()].lock().unwrap_or_else(PoisonError::into_inner);
            if queue.is_empty() {
                continue;
            }
            let low_priority_permit = if is_low_priority(lane) {
                match inner.reserve_low_priority() {
                    Some(permit) => Some(permit),
                    None => continue,
                }
            } else {
                None
            };
            if let Some(job) = queue.pop_front() {
                inner.workers[victim].publish(lane, &queue);
                return Some((lane, PoolWork::Job(job), low_priority_permit));
            }
        }
    }
    None
}

fn worker_loop(inner: &Arc<PoolInner>, index: u32) {
    semio_framework_trace::register_worker_thread(index);
    let mut cursor = 0usize;
    let mut deficits = [0i64; LANE_COUNT];
    while !inner.shutdown.load(Ordering::SeqCst) || inner.deferred_wakes.has_pending() {
        let observed = inner.parking.observe();
        FIRING_TIMERS.with(|firing| firing.set(true));
        inner.wheel.fire_due_batch(inner.now_ms(), TIMER_ACTIONS_PER_POOL_TURN);
        FIRING_TIMERS.with(|firing| firing.set(false));
        let picked = select_and_pop(inner, index as usize, &mut cursor, &mut deficits);
        let picked = if inner.shutdown.load(Ordering::SeqCst) { picked } else { picked.or_else(|| steal(inner, index as usize)) };
        match picked {
            Some((_lane, job, low_priority_permit)) => {
                if inner.wheel.next_deadline_ms().is_some() {
                    inner.parking.hand_off_timers();
                }
                inner.trace_workers.worker_started();
                let permit = inner.ledger.checkout(1).expect("WorkerPool: internal permit invariant violated — checked out more than worker_count concurrently");
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job.run(&inner.maintenance)));
                drop(permit);
                inner.trace_workers.worker_finished();
                drop(low_priority_permit);
            }
            None => {
                let _ = inner.parking.park(observed, inner.timer_due());
            }
        }
    }
}

/// 🪜️ The stack every pool worker owns, stated rather than inherited.
///
/// A worker runs whole subsystem closures — shard turns, DB actor turns, artifact-engine mounts
/// — so its budget is a product decision, not `std::thread`'s default. Left unstated it was
/// **two** numbers at once, which is the whole defect: a cargo-launched process got
/// `.cargo/config.toml`'s repo-wide `RUST_MIN_STACK = 67108864`, while the shipped `os-hub`,
/// started by a holder rather than by cargo, got `std::thread`'s 2 MiB. Every gate therefore ran
/// these workers with 64 MiB and stayed green while the binary aborted on its first document
/// socket with `thread 'semio-pool-worker-N' has overflowed its stack` (ticket 26/09/18 slice
/// HS1, `📓️hs1-hub-pool-worker-stack-overflow.md`).
///
/// The number is the one the gates have in fact been proving all along, now stated where the
/// binary reads it too. It is not a guess: HS1 priced the artifact-engine turn chain out of the
/// crash reports' own AArch64 prologues — 21,520,944 B before the `db_artifact` boxing of §4.1
/// and 9,060,912 B after it, on a path that still exhausted a 16 MiB stack — so 16 MiB is
/// measurably too small for a debug build and 64 MiB is the budget under which this suite has
/// always passed. It costs address space, not memory: thread stacks are lazily committed, so an
/// untouched page never becomes resident.
///
/// `Builder::stack_size` **overrides** `RUST_MIN_STACK` in both directions, which is why this
/// constant must never be lowered casually: dropping it to 16 MiB took three green hub laws to
/// `SIGABRT` in seconds. The law that pins it is
/// `native_pool_workers_own_the_stated_worker_stack`.
pub const WORKER_STACK_BYTES: usize = 64 * 1024 * 1024;

/// 🧵️ The native, multi-OS-thread work-stealing pool. Per-worker per-lane deques (own-thread
/// DRR pop, cross-thread steal) implemented with plain `std::sync::Mutex<VecDeque<_>>` — no
/// lock-free Chase-Lev deque, no external crate: a `Mutex` per (worker, lane) pair is simple,
/// correct, and cheap enough at this crate's job granularity (whole closures, not fine-grained
/// tasks). See the module doc for why timers live here too instead of a dedicated OS thread.
#[derive(Clone)]
pub struct WorkerPool {
    #[doc(hidden)] pub inner: Arc<PoolInner>,
}

pub struct WorkerPoolUse {
    #[doc(hidden)] pub inner: std::sync::Weak<PoolInner>,
}

impl Drop for WorkerPoolUse {
    fn drop(&mut self) {
        let Some(inner) = self.inner.upgrade() else { return };
        let mut lifecycle = inner.lifecycle.lock().unwrap_or_else(PoisonError::into_inner);
        if let PoolLifecycleState::Open { retained_uses } = &mut *lifecycle {
            *retained_uses = retained_uses.checked_sub(1).expect("WorkerPoolUse: retained-use underflow");
        }
    }
}

impl WorkerPool {
    pub fn new(config: WorkerPoolConfig) -> WorkerPool {
        let worker_count = worker_count_for(config.process_kind, config.cores).max(1);
        let inner = Arc::new(PoolInner {
            workers: (0..worker_count).map(|_| WorkerLocal::new()).collect(),
            maintenance: WorkerMaintenanceRegistry::new(),
            deferred_wakes: WorkerDeferredWakeRegistry::new(),
            lifecycle: Mutex::new(PoolLifecycleState::Open { retained_uses: 0 }),
            shutdown: std::sync::atomic::AtomicBool::new(false),
            next_submit: AtomicUsize::new(0),
            io_source: AtomicUsize::new(0),
            parking: Arc::new(worker_parking::WorkerParking::new()),
            wheel: TimerWheel::new(),
            low_priority_active: AtomicU32::new(0),
            interactive_reserve: config.interactive_reserve,
            ledger: PermitLedger::new(worker_count as u32),
            trace_workers: semio_framework_trace::WorkerCounters::new(),
            start: Instant::now(),
            handles: Mutex::new(Vec::with_capacity(worker_count)),
        });
        let parking = Arc::clone(&inner.parking);
        inner.wheel.install_earlier_deadline_hook(Box::new(move || {
            if FIRING_TIMERS.with(std::cell::Cell::get) {
                parking.signal_timer_from_firing_worker();
            } else {
                parking.signal_timer();
            }
        }));
        let mut handles = Vec::with_capacity(worker_count);
        for index in 0..worker_count {
            let worker_inner = Arc::clone(&inner);
            let handle = thread::Builder::new().name(format!("semio-pool-worker-{index}")).stack_size(WORKER_STACK_BYTES).spawn(move || worker_loop(&worker_inner, index as u32)).expect("WorkerPool: failed to spawn worker thread");
            handles.push(handle);
        }
        *inner.handles.lock().unwrap_or_else(PoisonError::into_inner) = handles;
        WorkerPool { inner }
    }

    /// ▶️ Enqueues `job` onto `lane`, targeting one worker round-robin (siblings pick it up via
    /// [`steal`] if that worker is busy) and waking any idle-parked worker.
    pub fn submit(&self, lane: Lane, job: Job) {
        if self.is_shutdown() {
            panic!("WorkerPool: mandatory submission failed closed: {:?}", WorkerSubmitErrorKind::Shutdown);
        }
        let index = self.inner.next_submit.fetch_add(1, Ordering::SeqCst) % self.inner.workers.len();
        let mut queue = self.inner.workers[index].queues[lane.index()].lock().unwrap_or_else(|_| panic!("WorkerPool: mandatory submission failed closed: {:?}", WorkerSubmitErrorKind::Poisoned));
        if queue.len() >= WORKER_JOBS_PER_LANE {
            panic!("WorkerPool: mandatory submission failed closed: {:?}", WorkerSubmitErrorKind::Saturated);
        }
        queue.push_back(job);
        self.inner.workers[index].publish(lane, &queue);
        drop(queue);
        self.inner.notify_idle();
    }

    /// 🔔️ Installs a fixed callback slot before any owner can request retirement.
    pub fn install_maintenance_hook(&self, lane: Lane, callback: WorkerMaintenanceCallback, context: [u64; 2]) -> Result<WorkerMaintenanceTicket, WorkerMaintenanceError> {
        if self.is_shutdown() {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        self.inner.maintenance.install(lane, callback, context)
    }

    /// 📣️ Coalesces an exact wake without a queued closure or task allocation.
    pub fn request_maintenance(&self, ticket: WorkerMaintenanceTicket) -> Result<WorkerMaintenanceRequest, WorkerMaintenanceError> {
        if self.is_shutdown() {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        let result = self.inner.maintenance.request(ticket)?;
        self.inner.notify_idle();
        Ok(result)
    }

    /// 🧹️ Fences future requests and removes only an idle exact callback slot.
    pub fn remove_maintenance_hook(&self, ticket: WorkerMaintenanceTicket) -> Result<bool, WorkerMaintenanceError> {
        self.inner.maintenance.remove(ticket)
    }

    /// 🎫️ Reserves one exact fixed deferred-wake partition.
    pub fn install_deferred_wake_partition(&self) -> Result<WorkerDeferredWakeTicket, WorkerDeferredWakeError> {
        if self.is_shutdown() {
            return Err(WorkerDeferredWakeError::Shutdown);
        }
        self.inner.deferred_wakes.install()
    }

    /// 📨️ Moves one waker into its exact slot without invoking user code.
    pub fn defer_wake(&self, ticket: WorkerDeferredWakeTicket, slot: usize, waker: Waker) -> Result<(), WorkerDeferredWakeRejected> {
        let result = self.inner.deferred_wakes.defer_wake(ticket, slot, waker);
        if result.is_ok() {
            self.inner.notify_idle();
        }
        result
    }

    /// 🔎️ Observes whether an exact slot still retains its deferred owner.
    pub fn deferred_wake_pending(&self, ticket: WorkerDeferredWakeTicket, slot: usize) -> Result<bool, WorkerDeferredWakeError> {
        self.inner.deferred_wakes.pending(ticket, slot)
    }

    /// ➖️ Closes admission and removes a drained exact partition.
    pub fn remove_deferred_wake_partition(&self, ticket: WorkerDeferredWakeTicket) -> Result<bool, WorkerDeferredWakeError> {
        self.inner.deferred_wakes.remove(ticket)
    }

    /// 🔏️ Attempts one hard-bounded admission without waiting for queue ownership. Failure
    /// returns the exact closure so a persistent operation owner can retry without losing state.
    pub fn try_submit(&self, lane: Lane, job: Job) -> Result<(), WorkerSubmitError> {
        if self.is_shutdown() {
            return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Shutdown, job });
        }
        let index = self.inner.next_submit.fetch_add(1, Ordering::SeqCst) % self.inner.workers.len();
        let mut queue = match self.inner.workers[index].queues[lane.index()].try_lock() {
            Ok(queue) => queue,
            Err(std::sync::TryLockError::WouldBlock) => return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Contended, job }),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Poisoned, job }),
        };
        if queue.len() >= WORKER_JOBS_PER_LANE {
            return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Saturated, job });
        }
        queue.push_back(job);
        self.inner.workers[index].publish(lane, &queue);
        drop(queue);
        self.inner.notify_idle();
        Ok(())
    }

    /// 🗓️ Enqueues one finite job when the pool's monotonic clock reaches `deadline_ms`.
    /// Waiting consumes no worker, lane permit, or admission slot. Transient lane contention
    /// retains the exact closure behind one timer-wheel callback until admission succeeds.
    pub fn submit_at(&self, deadline_ms: u64, lane: Lane, job: Job) {
        let pool = self.clone();
        self.inner.wheel.schedule_callback(
            deadline_ms,
            Box::new(move || {
                Self::submit_retained_timer_job(&pool, lane, job);
            }),
        );
    }

    fn submit_retained_timer_job(pool: &WorkerPool, lane: Lane, job: Job) {
        let Err(error) = pool.try_submit(lane, job) else { return };
        let kind = error.kind();
        let job = error.into_job();
        match kind {
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                let retry = pool.clone();
                pool.callback_at(pool.now_ms().saturating_add(1), move || Self::submit_retained_timer_job(&retry, lane, job));
            }
            WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned => drop(job),
        }
    }

    /// 📞️ Registers one bounded owner callback directly with the pool timer wheel. Unlike
    /// [`WorkerPool::submit_at`], firing does not require a second lane admission, so an owner
    /// can use it to retry a previously rejected exact closure without depending on ingress.
    pub fn callback_at(&self, deadline_ms: u64, callback: impl FnOnce() + Send + 'static) {
        self.inner.wheel.schedule_callback(deadline_ms, Box::new(callback));
    }

    pub fn is_shutdown(&self) -> bool {
        self.inner.shutdown.load(Ordering::SeqCst)
    }

    /// 🔐️ Retains one cold lifecycle use until the final clone of the returned cell drops.
    pub fn acquire_use(&self) -> Result<Arc<WorkerPoolUse>, WorkerPoolUseError> {
        let mut lifecycle = self.inner.lifecycle.lock().unwrap_or_else(PoisonError::into_inner);
        match &mut *lifecycle {
            PoolLifecycleState::Open { retained_uses } => {
                *retained_uses = retained_uses.checked_add(1).ok_or(WorkerPoolUseError::Exhausted)?;
                Ok(Arc::new(WorkerPoolUse { inner: Arc::downgrade(&self.inner) }))
            }
            PoolLifecycleState::Closing => Err(WorkerPoolUseError::Closing),
            PoolLifecycleState::Stopped => Err(WorkerPoolUseError::Stopped),
        }
    }

    pub fn worker_count(&self) -> usize {
        self.inner.workers.len()
    }

    /// 🪞️ Whether both handles address the same process worker pool.
    pub fn is_same_pool(&self, other: &WorkerPool) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// 📊️ Workers currently executing a job (not merely alive) — see the module doc's
    /// distinction from [`WorkerPool::occupancy`].
    pub fn active_workers(&self) -> u32 {
        self.inner.trace_workers.active()
    }

    /// 📒️ Permits currently checked out of this pool's own [`PermitLedger`] (sized to
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
    pub fn shutdown(&self) -> Result<(), WorkerPoolShutdownError> {
        {
            let mut lifecycle = self.inner.lifecycle.lock().unwrap_or_else(PoisonError::into_inner);
            match *lifecycle {
                PoolLifecycleState::Open { retained_uses: 0 } => *lifecycle = PoolLifecycleState::Closing,
                PoolLifecycleState::Open { retained_uses } => return Err(WorkerPoolShutdownError::Busy { retained_uses }),
                PoolLifecycleState::Closing => return Err(WorkerPoolShutdownError::Closing),
                PoolLifecycleState::Stopped => return Ok(()),
            }
        }
        self.inner.maintenance.shutdown();
        self.inner.deferred_wakes.shutdown();
        self.inner.shutdown.store(true, Ordering::SeqCst);
        self.inner.wheel.fire_due(u64::MAX);
        self.inner.parking.close();
        let mut handles = self.inner.handles.lock().unwrap_or_else(PoisonError::into_inner);
        for handle in handles.drain(..) {
            let _ = handle.join();
        }
        *self.inner.lifecycle.lock().unwrap_or_else(PoisonError::into_inner) = PoolLifecycleState::Stopped;
        Ok(())
    }
}

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod internals {
    use super::*;
    pub fn select_and_pop<'a>(inner: &'a PoolInner, my_index: usize, cursor: &mut usize, deficits: &mut [i64; LANE_COUNT]) -> Option<(Lane, PoolWork, Option<LowPriorityPermit<'a>>)> {
        super::select_and_pop(inner, my_index, cursor, deficits)
    }
}
