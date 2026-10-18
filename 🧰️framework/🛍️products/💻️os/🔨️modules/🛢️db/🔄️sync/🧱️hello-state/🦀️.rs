//! 🧱️ Hello state: the driver authority, the shared hello state with its core, the live counters, the retirement witness, the registry and wake and the state operations. See [`super`].

use super::*;

//#region 🔖️Hello
//#region 👋️RetainedHello
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DatabaseSyncHelloDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

pub(super) struct DatabaseSyncHelloCore {
    pub(super) future: Option<DatabaseSyncHelloExecutionFuture>,
    pub execution: Option<DatabaseSyncHelloExecution>,
    pub frame: Option<Result<Option<protocol::ServerFrame>, DbError>>,
    pub(super) frame_close: Option<DatabaseSyncHelloFrameClose>,
    pub returned_frame: Option<DatabaseSyncHelloReturnedFrameLease>,
    pub(super) returned_fallback: Option<DatabaseSyncHelloReturnedFrameLease>,
    pub(super) quarantined: Option<DatabaseSyncHelloQuarantineClose>,
}

pub(super) struct DatabaseSyncHelloState {
    pub(super) pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub(super) _pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>,
    pub slot: usize,
    pub(super) generation: u64,
    pub admission: std::sync::Mutex<Option<DatabaseSyncHelloAdmission>>,
    pub core: std::sync::Mutex<DatabaseSyncHelloCore>,
    pub driver: std::sync::atomic::AtomicU8,
    pub(super) retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub(super) progress_deadline_ms: std::sync::atomic::AtomicU64,
    pub(super) deadline_armed_ms: std::sync::atomic::AtomicU64,
    pub(super) progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
    pub(super) wake_requested: std::sync::atomic::AtomicBool,
    pub demand: std::sync::atomic::AtomicBool,
    pub(super) abandoned: std::sync::atomic::AtomicBool,
    pub(super) close_armed: std::sync::atomic::AtomicBool,
    pub(super) close_requested: std::sync::atomic::AtomicBool,
    pub(super) waker: std::sync::Mutex<Option<std::task::Waker>>,
    pub(super) returned_generation: std::sync::atomic::AtomicU64,
}

/// 🔬️ How many retained sync-hello slots still hold their `WorkerPoolUse` clone. A database
/// shutdown that never leaves its `PoolUse` phase is blocked by one of these registry families.
pub fn database_sync_hello_live_slots() -> usize {
    database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count()
}

#[cfg(any(test, feature = "testkit"))]
impl DatabaseSyncHelloState {
    fn debug_witness(&self) -> String {
        use std::sync::atomic::Ordering::Acquire;
        let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        format!(
            "driver={} retry_job={} cancelled={} expired={} abandoned={} close_armed={} close_requested={} wake={} demand={} future={} execution={} frame={} frame_close={} returned={} fallback={} quarantined={} progress={}",
            self.driver.load(Acquire),
            self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            self.cancelled.load(Acquire),
            self.expired.load(Acquire),
            self.abandoned.load(Acquire),
            self.close_armed.load(Acquire),
            self.close_requested.load(Acquire),
            self.wake_requested.load(Acquire),
            self.demand.load(Acquire),
            core.future.is_some(),
            core.execution.is_some(),
            core.frame.is_some(),
            core.frame_close.is_some(),
            core.returned_frame.is_some(),
            core.returned_fallback.is_some(),
            core.quarantined.is_some(),
            self.progress.load(Acquire),
        )
    }
}

/// 🔬️ Per-owner retirement witness of one hello session: the state it owns, observed without
/// keeping it alive, so concurrent sessions of other owners never enter the observation.
#[cfg(any(test, feature = "testkit"))]
pub struct DatabaseSyncHelloRetirementWitness(pub(super) std::sync::Weak<DatabaseSyncHelloState>);

#[cfg(any(test, feature = "testkit"))]
impl DatabaseSyncHelloRetirementWitness {
    pub fn retired(&self) -> bool {
        self.0.strong_count() == 0
    }

    pub fn describe(&self) -> String {
        self.0.upgrade().map_or_else(|| String::from("retired"), |state| state.debug_witness())
    }
}

pub(super) static DATABASE_SYNC_HELLO_STATE_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ How many LIVE `DatabaseSyncHelloState` owners still hold the `WorkerPoolUse` clone their
/// admission was given. The registry slot is cleared the moment the hello stops being pending, but
/// the `Arc<DatabaseSyncHelloState>` outlives that clearing inside its future, its session and its
/// deadline callback — so `database_sync_hello_live_slots() == 0` does NOT exclude a live carrier.
/// This counter is incremented where the state is built and decremented in its `Drop`.
pub fn database_sync_hello_live_states() -> usize {
    DATABASE_SYNC_HELLO_STATE_LIVE.load(std::sync::atomic::Ordering::Acquire)
}

impl Drop for DatabaseSyncHelloState {
    fn drop(&mut self) {
        DATABASE_SYNC_HELLO_STATE_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

pub fn database_sync_hello_registry() -> &'static std::sync::Mutex<[Option<std::sync::Arc<DatabaseSyncHelloState>>; DATABASE_SYNC_HELLO_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<std::sync::Arc<DatabaseSyncHelloState>>; DATABASE_SYNC_HELLO_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseSyncHelloState {
    fn wake(self: std::sync::Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::SeqCst);
        self.schedule();
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::SeqCst);
        self.schedule();
    }
}

impl DatabaseSyncHelloState {
    pub(super) fn current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseSyncHelloAdmission::is_current)
    }

    /// 🔔️ The signal half of the driver handoff. Every request publishes `wake_requested` before it
    /// tries to queue a turn, so a caller that finds the driver busy never re-submits and never loses
    /// its request either: `drive_one` clears the signal when its turn starts and re-reads it after
    /// its release. That covers every reason a turn is requested — a waker, an acknowledged frame
    /// mounting its close, a close request, a demand — not only the ones that also carry their own
    /// flag. Both halves are `SeqCst` so the store and the CAS here and the release store and the
    /// loads there share one total order: whenever this CAS reads a busy driver, the release is
    /// guaranteed to read the signal, and whenever the release reads no signal, this CAS is
    /// guaranteed to read `Idle` and queue the turn itself. A lost request parks the hello with
    /// nothing scheduled; its registry slot and `WorkerPoolUse` then outlive every session, and the
    /// only recovery was the 30 s `DATABASE_SYNC_HELLO_DEADLINE_MS` callback.
    pub(super) fn schedule(self: &std::sync::Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::SeqCst);
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Idle as u8, DatabaseSyncHelloDriverAuthority::Queued as u8, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()), 0);
    }

    fn submit_exact(self: &std::sync::Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next = attempt.saturating_add(1).min(DATABASE_SYNC_HELLO_RETRY_LIMIT);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Queued as u8, DatabaseSyncHelloDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    pub(super) fn retry(self: std::sync::Arc<Self>) {
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire) || attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Retry as u8, DatabaseSyncHelloDriverAuthority::Queued as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    /// 🚚️ One driver turn, and the release that must never drop a signal raised during it. The
    /// turn clears `wake_requested` before it does any work, so every request raised while it runs
    /// is still set when the driver goes `Idle` — the reason the signals are re-read AFTER the
    /// release. A turn spent retiring a returned frame while a close is requested stays pending, so
    /// the close itself gets the next turn. A dropped signal parks the whole hello with nothing scheduled, and its only recovery
    /// is the 30 s deadline callback, which every caller of `hello()`/`next_frame()` experiences as a
    /// bootstrap frame that simply never arrives and a stopping `Database` as a hello that never
    /// retires.
    pub(super) fn drive_one(self: std::sync::Arc<Self>) {
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Queued as u8, DatabaseSyncHelloDriverAuthority::Driving as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        self.wake_requested.store(false, std::sync::atomic::Ordering::SeqCst);
        let returned = self.close_returned_frame_one();
        let closing = self.close_requested.load(std::sync::atomic::Ordering::Acquire) && self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.is_none();
        let pending = match returned {
            Some(pending) => pending || closing,
            None if closing => self.close_one_claimed(),
            None => self.poll_one(),
        };
        self.driver.store(DatabaseSyncHelloDriverAuthority::Idle as u8, std::sync::atomic::Ordering::SeqCst);
        let signalled = self.wake_requested.load(std::sync::atomic::Ordering::SeqCst) || self.demand.load(std::sync::atomic::Ordering::SeqCst);
        if pending || signalled {
            self.schedule();
        }
    }

    fn close_returned_frame_one(&self) -> Option<bool> {
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let fallback = core.returned_fallback.is_some();
        let lease = if fallback { core.returned_fallback.as_mut()? } else { core.returned_frame.as_mut()? };
        let close = lease.close.as_mut()?;
        let pending = close.close_one();
        if !close.terminal_is_empty() {
            return Some(pending);
        }
        let items = lease.items;
        let bytes = lease.bytes;
        let generation = lease.generation;
        let Some(execution) = core.execution.as_mut() else {
            return Some(true);
        };
        let Some(ledger) = execution.ledger.as_mut() else {
            return Some(true);
        };
        if ledger.release(items, bytes).is_err() {
            return Some(true);
        }
        if fallback {
            if core.returned_fallback.as_ref().is_some_and(|owner| owner.generation == generation) {
                core.returned_fallback.take();
            }
        } else if core.returned_frame.as_ref().is_some_and(|owner| owner.generation == generation) {
            core.returned_frame.take();
        }
        drop(core);
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        Some(false)
    }

    fn poll_one(self: &std::sync::Arc<Self>) -> bool {
        if !self.current() {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        let future = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.take();
        if let Some(mut future) = future {
            let waker = std::task::Waker::from(self.clone());
            let mut context = std::task::Context::from_waker(&waker);
            let turn_ends = std::time::Instant::now() + std::time::Duration::from_micros(DATABASE_SYNC_HELLO_POLL_TURN_MICROS);
            let polled = loop {
                let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
                let repoll = matches!(polled, Ok(std::task::Poll::Pending))
                    && std::time::Instant::now() < turn_ends
                    && !self.cancelled.load(std::sync::atomic::Ordering::Acquire)
                    && self.wake_requested.swap(false, std::sync::atomic::Ordering::SeqCst);
                if !repoll {
                    break polled;
                }
            };
            return match polled {
                Ok(std::task::Poll::Pending) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    core.future = Some(future);
                    true
                }
                Ok(std::task::Poll::Ready(execution)) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if self.current() {
                        core.execution = Some(execution);
                    } else {
                        core.execution = Some(DatabaseSyncHelloExecution { owners: execution.owners, prepared: Err(DbError::Closed), ledger: execution.ledger, close_fault: execution.close_fault });
                    }
                    drop(core);
                    if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
                        self.arm_close();
                    }
                    if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        waker.wake();
                    }
                    false
                }
                Err(_) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let bytes = size_of_val(&*future);
                    core.quarantined = Some(DatabaseSyncHelloQuarantineClose { future: Some(future), items: 1, bytes });
                    core.frame = Some(Err(DbError::Internal("database sync hello worker panic".to_string())));
                    self.progress.store(DatabaseSyncHelloProgress::Fault as u8, std::sync::atomic::Ordering::Release);
                    drop(core);
                    self.arm_close();
                    false
                }
            };
        }
        if !self.demand.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return false;
        }
        if let Err(error) = database_sync_hello_control(&self.cancelled, &self.expired) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            core.frame = Some(Err(error));
            drop(core);
            self.arm_close();
            if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
            return false;
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.frame.is_some() || core.returned_frame.is_some() {
            return false;
        }
        let Some(execution) = core.execution.as_mut() else {
            core.frame = Some(Err(DbError::Closed));
            return false;
        };
        let waker = std::task::Waker::from(self.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let step = match (&mut execution.prepared, &mut execution.ledger) {
            (Ok(prepared), Some(ledger)) => prepared.follow_up.drive_one(ledger, &self.cancelled, &self.expired, &mut context),
            (Ok(_), None) => Err(DbError::Internal("database sync hello backing ledger missing".to_string())),
            (Err(_), _) => Ok(DatabaseSyncHelloFollowUpStep::Done),
        };
        let waiting = matches!(step, Ok(DatabaseSyncHelloFollowUpStep::Waiting));
        match step {
            Ok(DatabaseSyncHelloFollowUpStep::Frame(frame)) => {
                self.progress_deadline_ms.store(self.pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS), std::sync::atomic::Ordering::Release);
                core.frame = Some(Ok(Some(frame)));
            }
            Ok(DatabaseSyncHelloFollowUpStep::Done) => {
                core.frame = Some(Ok(None));
                self.progress.store(DatabaseSyncHelloProgress::Completed as u8, std::sync::atomic::Ordering::Release);
            }
            Ok(DatabaseSyncHelloFollowUpStep::Progress | DatabaseSyncHelloFollowUpStep::Waiting) => self.demand.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => core.frame = Some(Err(error)),
        }
        drop(core);
        if waiting {
            return false;
        }
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        self.demand.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(super) fn arm_close(self: &std::sync::Arc<Self>) {
        if self.close_armed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        self.progress.store(DatabaseSyncHelloProgress::Closing as u8, std::sync::atomic::Ordering::Release);
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.close_armed.store(false, std::sync::atomic::Ordering::Release);
            state.close_requested.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        });
    }

    fn close_one_claimed(&self) -> bool {
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_some() {
            return false;
        }
        let mut pending = false;
        if let Some(cursor) = core.frame_close.as_mut() {
            pending = cursor.close_one();
            if cursor.terminal_is_empty() {
                core.frame_close = None;
            }
        } else if let Some(frame) = core.frame.take() {
            if let Ok(Some(frame)) = frame {
                core.frame_close = Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None });
            }
            pending = true;
        }
        if let Some(execution) = core.execution.as_mut() {
            if let Ok(prepared) = execution.prepared.as_mut() {
                if pending {
                } else if let Some(cursor) = prepared.welcome_close.as_mut() {
                    pending = cursor.close_one();
                    if cursor.terminal_is_empty() {
                        prepared.welcome_close = None;
                    }
                } else if let Some(welcome) = prepared.welcome.take() {
                    prepared.welcome_close = Some(DatabaseSyncHelloFrameClose { owner: Some(welcome), envelope: None });
                    pending = true;
                } else if let Some(fault) = execution.close_fault.as_mut() {
                    pending = fault.close_one();
                    if fault.terminal_is_empty() {
                        execution.close_fault = None;
                    }
                } else {
                    let result = prepared.follow_up.close_one();
                    pending = database_sync_hello_apply_follow_up_close_result(&mut execution.close_fault, prepared.follow_up.terminal_is_empty(), result);
                }
            }
            if !pending {
                if let Some(owners) = execution.owners.as_mut() {
                    pending = owners.close_one();
                    if owners.terminal_is_empty() {
                        execution.owners = None;
                    }
                }
            }
            if !pending {
                if let Some(ledger) = execution.ledger.as_mut() {
                    pending = ledger.close_one_credit();
                    if ledger.terminal_is_empty() {
                        execution.ledger = None;
                    }
                }
            }
            if !pending {
                let prepared_terminal = execution.prepared.as_ref().is_err() || execution.prepared.as_ref().is_ok_and(|prepared| prepared.welcome.is_none() && prepared.welcome_close.is_none() && prepared.follow_up.terminal_is_empty());
                pending = !prepared_terminal || execution.owners.is_some() || execution.ledger.is_some() || execution.close_fault.is_some();
            }
        }
        if !pending {
            if let Some(quarantine) = core.quarantined.as_mut() {
                pending = quarantine.close_one();
                if quarantine.terminal_is_empty() {
                    let _ = core.quarantined.take();
                }
            }
        }
        if !pending {
            core.execution = None;
            core.frame = None;
            core.frame_close = None;
        }
        drop(core);
        if !pending {
            let mut registry = database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            drop(registry);
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.close_requested.store(false, std::sync::atomic::Ordering::Release);
        }
        pending
    }

    /// ⏳️ Expires one hello that made no progress for `DATABASE_SYNC_HELLO_DEADLINE_MS`: every frame it produced renewed its
    /// deadline, so a long streamed tail that keeps moving is never cut and the callback re-arms at the renewed one. The
    /// `WorkerPool` timer holds the
    /// state WEAKLY: `callback_at` cannot be cancelled, so an owning clone would pin the state — and
    /// with it the `Database`'s `WorkerPoolUse` — for the full `DATABASE_SYNC_HELLO_DEADLINE_MS`
    /// after the hello finished, long past the registry slot being cleared. A database that
    /// completed a sync could then not shut down inside its own deadline. A hello whose owners are
    /// all gone has nothing left to expire, so the upgrade failing is the terminal case.
    pub fn deadline_callback(self: &std::sync::Arc<Self>) {
        let renewed = self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if renewed > self.deadline_armed_ms.swap(renewed, std::sync::atomic::Ordering::AcqRel) {
            let deadline = std::sync::Arc::downgrade(self);
            self.pool.callback_at(renewed, move || {
                if let Some(state) = deadline.upgrade() {
                    state.deadline_callback();
                }
            });
            return;
        }
        if self.current() && !matches!(self.progress(), DatabaseSyncHelloProgress::Completed | DatabaseSyncHelloProgress::Cancelled | DatabaseSyncHelloProgress::Fault) {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.schedule();
        }
    }

    pub(super) fn progress(&self) -> DatabaseSyncHelloProgress {
        DatabaseSyncHelloProgress::from_u8(self.progress.load(std::sync::atomic::Ordering::Acquire))
    }
}
//#endregion 👋️RetainedHello
//#endregion 🔖️Hello
