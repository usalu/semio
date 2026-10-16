//! 🕸️ Cooperative single-thread `WorkerPool` for wasm targets and tests: deficit-round-robin lane selection over a manual pump.

use super::*;

/// 📊️ Fixed-size diagnostic copy; no queued closure or scheduling authority escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CooperativePoolSnapshot {
    pub pump_calls: u64,
    pub selections: u64,
    pub no_selection: u64,
    pub selected_by_lane: [u64; LANE_COUNT],
    pub queued_by_lane: [usize; LANE_COUNT],
    pub deficits: [i64; LANE_COUNT],
    pub cursor: usize,
}

struct SchedulerState {
    pub queues: [VecDeque<Job>; LANE_COUNT],
    cursor: usize,
    io_source: usize,
    deficits: [i64; LANE_COUNT],
    selections: u64,
    no_selection: u64,
    pub selected_by_lane: [u64; LANE_COUNT],
}

impl SchedulerState {
    fn new() -> SchedulerState {
        SchedulerState { queues: std::array::from_fn(|_| admitted_job_queue()), cursor: 0, io_source: 0, deficits: [0; LANE_COUNT], selections: 0, no_selection: 0, selected_by_lane: [0; LANE_COUNT] }
    }

    fn select_and_pop(&mut self, maintenance: &WorkerMaintenanceRegistry, deferred_wakes: &WorkerDeferredWakeRegistry, shutdown: bool) -> Option<(Lane, PoolWork)> {
        if shutdown {
            return deferred_wakes.select().map(|wake| (Lane::Io, PoolWork::DeferredWake(wake)));
        }
        const UNIT_COST: i64 = Lane::Interactive.weight() as i64;
        for _ in 0..LANE_COUNT {
            let lane = Lane::ALL[self.cursor];
            self.cursor = (self.cursor + 1) % LANE_COUNT;
            if self.queues[lane.index()].is_empty() && !maintenance.has_pending(Some(lane)) && (lane != Lane::Io || !deferred_wakes.has_pending()) {
                self.deficits[lane.index()] = 0;
                continue;
            }
            self.deficits[lane.index()] += lane.weight() as i64;
            if self.deficits[lane.index()] >= UNIT_COST {
                let work = if lane == Lane::Io {
                    let first = self.io_source;
                    self.io_source = (self.io_source + 1) % 3;
                    (0..3).find_map(|offset| match (first + offset) % 3 {
                        0 => self.queues[lane.index()].pop_front().map(PoolWork::Job),
                        1 => maintenance.select_hook(lane),
                        _ => deferred_wakes.select().map(PoolWork::DeferredWake),
                    })
                } else {
                    maintenance.select(lane, &mut self.queues[lane.index()])
                };
                if let Some(work) = work {
                    self.deficits[lane.index()] -= UNIT_COST;
                    self.selections = self.selections.saturating_add(1);
                    self.selected_by_lane[lane.index()] = self.selected_by_lane[lane.index()].saturating_add(1);
                    return Some((lane, work));
                }
            }
        }
        self.no_selection = self.no_selection.saturating_add(1);
        None
    }

    fn has_pending(&self) -> bool {
        self.queues.iter().any(|queue| !queue.is_empty())
    }
}

struct PoolInner {
    pub state: Mutex<SchedulerState>,
    pub maintenance: WorkerMaintenanceRegistry,
    deferred_wakes: WorkerDeferredWakeRegistry,
    lifecycle: Mutex<PoolLifecycleState>,
    wheel: TimerWheel,
    now_ms: std::sync::atomic::AtomicU64,
    pump_calls: std::sync::atomic::AtomicU64,
    shutdown: std::sync::atomic::AtomicBool,
    ledger: PermitLedger,
    trace_workers: semio_framework_trace::WorkerCounters,
}

/// 🧵️ `wasm32` cooperative, host-driven, single-logical-worker pool — no OS threads exist on
/// this target. Same public surface as the native pool ([`WorkerPool::new`]/`submit`/
/// `worker_count`/`active_workers`/`occupancy`/`permits`/`timer`/`now_ms`/`shutdown`), PLUS
/// [`WorkerPool::pump`], which the host (the browser's Web Worker running this WASM module — see
/// `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`) must call repeatedly to make progress:
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
    #[doc(hidden)] pub inner: Arc<PoolInner>,
    interactive_reserve: bool,
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
        let inner = Arc::new(PoolInner {
            state: Mutex::new(SchedulerState::new()),
            maintenance: WorkerMaintenanceRegistry::new(),
            deferred_wakes: WorkerDeferredWakeRegistry::new(),
            lifecycle: Mutex::new(PoolLifecycleState::Open { retained_uses: 0 }),
            wheel: TimerWheel::new(),
            now_ms: std::sync::atomic::AtomicU64::new(0),
            pump_calls: std::sync::atomic::AtomicU64::new(0),
            shutdown: std::sync::atomic::AtomicBool::new(false),
            ledger: PermitLedger::new(1),
            trace_workers: semio_framework_trace::WorkerCounters::new(),
        });
        WorkerPool { inner, interactive_reserve: config.interactive_reserve }
    }

    /// 🔔️ Installs the same fixed callback authority used by native pool workers.
    pub fn install_maintenance_hook(&self, lane: Lane, callback: WorkerMaintenanceCallback, context: [u64; 2]) -> Result<WorkerMaintenanceTicket, WorkerMaintenanceError> {
        if self.is_shutdown() {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        self.inner.maintenance.install(lane, callback, context)
    }

    /// 📣️ Coalesces exact work for the next host-driven pump without allocating a job.
    pub fn request_maintenance(&self, ticket: WorkerMaintenanceTicket) -> Result<WorkerMaintenanceRequest, WorkerMaintenanceError> {
        if self.is_shutdown() {
            return Err(WorkerMaintenanceError::Shutdown);
        }
        self.inner.maintenance.request(ticket)
    }

    /// 🧹️ Retains the callback until no exact invocation is running.
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

    /// 📨️ Moves one waker into its exact slot for a later host pump.
    pub fn defer_wake(&self, ticket: WorkerDeferredWakeTicket, slot: usize, waker: Waker) -> Result<(), WorkerDeferredWakeRejected> {
        self.inner.deferred_wakes.defer_wake(ticket, slot, waker)
    }

    /// 🔎️ Observes whether an exact slot still retains its deferred owner.
    pub fn deferred_wake_pending(&self, ticket: WorkerDeferredWakeTicket, slot: usize) -> Result<bool, WorkerDeferredWakeError> {
        self.inner.deferred_wakes.pending(ticket, slot)
    }

    /// 🧹️ Closes admission and removes a drained exact partition.
    pub fn remove_deferred_wake_partition(&self, ticket: WorkerDeferredWakeTicket) -> Result<bool, WorkerDeferredWakeError> {
        self.inner.deferred_wakes.remove(ticket)
    }

    pub fn submit(&self, lane: Lane, job: Job) {
        if let Err(error) = self.try_submit(lane, job) {
            panic!("WorkerPool: mandatory submission failed closed: {:?}", error.kind());
        }
    }

    /// 🚦️ Cooperative equivalent of native `try_submit`: never waits, never grows a queue,
    /// and returns the exact closure on contention, poison, shutdown, or finite saturation.
    pub fn try_submit(&self, lane: Lane, job: Job) -> Result<(), WorkerSubmitError> {
        if self.is_shutdown() {
            return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Shutdown, job });
        }
        let mut state = match self.inner.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Contended, job }),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Poisoned, job }),
        };
        let queue = &mut state.queues[lane.index()];
        if queue.len() >= WORKER_JOBS_PER_LANE {
            return Err(WorkerSubmitError { kind: WorkerSubmitErrorKind::Saturated, job });
        }
        queue.push_back(job);
        Ok(())
    }

    /// ⏰️ Enqueues one finite job after a host-driven pump reaches `deadline_ms`.
    /// Waiting consumes no logical worker or permit. Transient cooperative contention retains
    /// the exact closure behind one timer-wheel callback until admission succeeds.
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

    /// 🔔️ Cooperative counterpart of native [`WorkerPool::callback_at`].
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
        1
    }

    /// 🪞️ Whether both handles address the same cooperative process worker pool.
    pub fn is_same_pool(&self, other: &WorkerPool) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
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

    /// 🕐️ The latest monotonic host time supplied through [`WorkerPool::pump`]. Before the
    /// first pump this is zero; regressing host samples never move the pool clock backwards.
    pub fn now_ms(&self) -> u64 {
        self.inner.now_ms.load(Ordering::SeqCst)
    }

    /// 🛑️ Marks the cooperative pool stopped and releases every retained timed callback.
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
        let _ = self.interactive_reserve;
        *self.inner.lifecycle.lock().unwrap_or_else(PoisonError::into_inner) = PoolLifecycleState::Stopped;
        Ok(())
    }

    /// ⏱️ Runs at most one DRR-selected job (admission control is a documented no-op here — the
    /// single-worker case is explicitly out of scope for the interactive reserve, same as the
    /// native pool's `worker_count < 2` short-circuit) and fires every timer due at
    /// `now_ms` (host-supplied — this target has no clock of its own). Returns whether more work
    /// is still queued, so the host knows whether to reschedule a pump immediately or wait for
    /// its next natural tick.
    pub fn pump(&self, now_ms: u64) -> bool {
        self.inner.pump_calls.store(self.inner.pump_calls.load(Ordering::Relaxed).saturating_add(1), Ordering::Relaxed);
        if self.is_shutdown() && !self.inner.deferred_wakes.has_pending() {
            return false;
        }
        if !self.is_shutdown() {
            let monotonic_now_ms = self.inner.now_ms.fetch_max(now_ms, Ordering::SeqCst).max(now_ms);
            self.inner.wheel.fire_due_batch(monotonic_now_ms, TIMER_ACTIONS_PER_POOL_TURN);
        }
        let picked = {
            let mut state = self.inner.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.select_and_pop(&self.inner.maintenance, &self.inner.deferred_wakes, self.is_shutdown())
        };
        if let Some((_lane, job)) = picked {
            self.inner.trace_workers.worker_started();
            let permit = self.inner.ledger.checkout(1).expect("WorkerPool: internal permit invariant violated on wasm pump");
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job.run(&self.inner.maintenance)));
            drop(permit);
            self.inner.trace_workers.worker_finished();
        }
        self.has_pending_work()
    }

    /// 🔎️ Nonblocking six-lane observation; contention retains all scheduler owners untouched.
    pub fn try_cooperative_snapshot(&self) -> Option<CooperativePoolSnapshot> {
        let state = self.inner.state.try_lock().ok()?;
        Some(CooperativePoolSnapshot {
            pump_calls: self.inner.pump_calls.load(Ordering::Relaxed),
            selections: state.selections,
            no_selection: state.no_selection,
            selected_by_lane: state.selected_by_lane,
            queued_by_lane: std::array::from_fn(|index| state.queues[index].len()),
            deficits: state.deficits,
            cursor: state.cursor,
        })
    }

    pub fn has_pending_work(&self) -> bool {
        self.inner.state.lock().unwrap_or_else(PoisonError::into_inner).has_pending() || self.inner.maintenance.has_pending(None) || self.inner.deferred_wakes.has_pending()
    }
}

#[cfg(feature = "wasm-pool-host")]
#[doc(hidden)]
pub mod internals {
    pub use super::{CooperativePoolSnapshot, WorkerPool, WorkerPoolUse};
}
