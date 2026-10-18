//! 🏊️ Pool future, requests and completion slot: the relay job budget, the retained-waker pool future, the relay requests and completions, the drive signal and the completion slot. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
/// ⛽️ Every cold relay turn stays inside the user-visible interactive slice. The guest job owns its
/// persistent state; the host grants one `step-job` call per shared-pool closure.
pub const RELAY_JOB_BUDGET: JobBudget = JobBudget { fuel: semio_framework_job::USER_VISIBLE_LANE_FUEL, deadline_ms: (semio_framework_job::USER_VISIBLE_LANE_WALL_US / 1_000) as u32 };

/// 🔁️ One retained-waker future polled once per finite shared-pool turn.
type GuestRelayFailureHandler = Box<dyn FnOnce(GuestRelayPoolFailure) + Send + 'static>;

pub(super) struct GuestRelayPoolFuture {
    pub(super) pool: WorkerPool,
    lane: Lane,
    pub(super) future: Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>>>,
    failure_handler: Mutex<Option<GuestRelayFailureHandler>>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    pub(super) complete: AtomicBool,
}

#[derive(Clone, Copy)]
pub(super) enum GuestRelayPoolFailure {
    FuturePanicked,
    Admission(semio_framework_async::WorkerSubmitErrorKind),
}

impl GuestRelayPoolFuture {
    pub(super) fn spawn_recoverable(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: impl FnOnce(GuestRelayPoolFailure) + Send + 'static) {
        Self::spawn_inner(pool, lane, future, Box::new(failure_handler));
    }

    fn spawn_inner(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: GuestRelayFailureHandler) {
        let task =
            Arc::new(Self { pool, lane, future: Mutex::new(Some(Box::pin(future))), failure_handler: Mutex::new(Some(failure_handler)), scheduled: AtomicBool::new(false), wake_requested: AtomicBool::new(false), complete: AtomicBool::new(false) });
        task.schedule();
    }

    pub(crate) fn schedule(self: &Arc<Self>) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        if self.pool.is_shutdown() {
            self.fail(GuestRelayPoolFailure::Admission(semio_framework_async::WorkerSubmitErrorKind::Shutdown));
            return;
        }
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if self.scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            let task = Arc::clone(self);
            if let Err(error) = self.pool.try_submit(self.lane, Box::new(move || task.poll_once())) {
                let kind = error.kind();
                let retry = Arc::clone(self);
                drop(error.into_job());
                match kind {
                    semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => {
                        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
                            retry.scheduled.store(false, std::sync::atomic::Ordering::Release);
                            retry.schedule();
                        });
                    }
                    semio_framework_async::WorkerSubmitErrorKind::Shutdown | semio_framework_async::WorkerSubmitErrorKind::Poisoned => self.fail(GuestRelayPoolFailure::Admission(kind)),
                }
            }
        }
    }

    pub(crate) fn fail(&self, failure: GuestRelayPoolFailure) {
        if self.complete.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        self.future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if let Some(handler) = self.failure_handler.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(failure)));
        }
    }

    fn poll_once(self: Arc<Self>) {
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        let waker = std::task::Waker::from(Arc::clone(&self));
        let mut context = std::task::Context::from_waker(&waker);
        let poll = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut future = self.future.lock().expect("guest relay future lock poisoned");
            future.as_mut().map_or(std::task::Poll::Ready(()), |future| std::future::Future::poll(future.as_mut(), &mut context))
        }));
        match poll {
            Ok(std::task::Poll::Pending) => {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    self.schedule();
                }
            }
            Ok(std::task::Poll::Ready(())) => {
                self.complete.store(true, std::sync::atomic::Ordering::Release);
                self.future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            }
            Err(_) => {
                self.fail(GuestRelayPoolFailure::FuturePanicked);
            }
        }
    }
}

impl std::task::Wake for GuestRelayPoolFuture {
    fn wake(self: Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.schedule();
    }
}

pub(super) enum GuestRelayRequest {
    Start { kind: String, input: Vec<u8> },
    Step,
    Cancel,
}

#[derive(Clone, Copy)]
pub(super) enum GuestRelayRequestKind {
    Start,
    Step,
    Cancel,
}

impl GuestRelayRequest {
    pub(crate) fn kind(&self) -> GuestRelayRequestKind {
        match self {
            Self::Start { .. } => GuestRelayRequestKind::Start,
            Self::Step => GuestRelayRequestKind::Step,
            Self::Cancel => GuestRelayRequestKind::Cancel,
        }
    }
}

pub(crate) enum GuestRelayCompletion {
    Started(Result<(), TurnFault>),
    Stepped(Result<GuestRelayStepCompletion, TurnFault>),
    Cancelled,
    Rejected(GuestRelayOwnedBytes),
    Fault(GuestRelayOwnedBytes),
    TerminalFault(GuestRelayOwnedBytes),
}

pub(super) enum GuestRelayStepCompletion {
    Running { progress: Option<GuestRelayOwnedBytes> },
    Done { output: GuestRelayOwnedBytes },
    Failed { error: GuestRelayOwnedBytes },
}

impl GuestRelayStepCompletion {
    pub(super) fn from_guest(step: JobStep) -> Self {
        match step {
            JobStep::Running { progress } => Self::Running { progress: progress.map(GuestRelayOwnedBytes::new) },
            JobStep::Done { output } => Self::Done { output: GuestRelayOwnedBytes::new(output) },
            JobStep::Failed { error } => Self::Failed { error: GuestRelayOwnedBytes::new(error) },
        }
    }
}

pub(super) struct GuestRelayOwnedBytes {
    pub(super) source: Vec<u8>,
}

impl GuestRelayOwnedBytes {
    pub(crate) fn new(source: Vec<u8>) -> Self {
        Self { source }
    }

    pub(super) fn into_source(self) -> Vec<u8> {
        self.source
    }
}

#[derive(Clone)]
pub(super) struct GuestRelayCompletionSender {
    pub(super) slot: Arc<GuestRelayCompletionSlot>,
}

impl GuestRelayCompletionSender {
    pub(crate) fn new(slot: Arc<GuestRelayCompletionSlot>) -> Self {
        Self { slot }
    }

    pub(crate) fn send(&self, completion: GuestRelayCompletion) {
        let mut value = self.slot.value.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if value.is_none() {
            *value = Some(completion);
            self.slot.wake.store(true, std::sync::atomic::Ordering::Release);
            drop(value);
            self.slot.signal.wake_owner();
        }
    }
}

/// 🏃️ One relay request. Its first poll runs on the thread that owns the relay — the caller's own
/// poll of the mounted future — so a crossing that completes inside that poll (every `step-job`
/// that does not suspend) pays no pool queue and no hop back to the caller. A request that suspends
/// (the instance gate is held, the guest yields, the step waits on the caller's cancel) is resumed by
/// the pool exactly as before, so an abandoned or detached relay still finishes and releases the
/// instance on its own.
pub(super) struct GuestRelayInlineRequest {
    pub(super) future: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>,
    pub(super) failure: Option<GuestRelayFailureHandler>,
}

enum GuestRelayDriveState {
    Idle,
    Owed(GuestRelayInlineRequest),
    Polling { woken: bool },
    Suspended(GuestRelayInlineRequest),
    Handed,
}

struct GuestRelayDriveSignal {
    pub(super) state: Mutex<GuestRelayDriveState>,
    pub(super) owner: Mutex<Option<std::task::Waker>>,
    pub(super) pool: WorkerPool,
}

impl GuestRelayDriveSignal {
    fn wake_owner(&self) {
        if let Some(waker) = self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    fn is_owed(&self) -> bool {
        matches!(&*self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayDriveState::Owed(_))
    }

    fn hand_to_pool(&self, request: GuestRelayInlineRequest) {
        let GuestRelayInlineRequest { future, failure } = request;
        GuestRelayPoolFuture::spawn_inner(self.pool.clone(), Lane::UserVisible, future, failure.unwrap_or_else(|| Box::new(|_| {})));
    }

    pub(crate) fn resume(&self) {
        let handed = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match std::mem::replace(&mut *state, GuestRelayDriveState::Idle) {
                GuestRelayDriveState::Suspended(request) => {
                    *state = GuestRelayDriveState::Handed;
                    Some(request)
                }
                GuestRelayDriveState::Polling { .. } => {
                    *state = GuestRelayDriveState::Polling { woken: true };
                    None
                }
                other => {
                    *state = other;
                    None
                }
            }
        };
        if let Some(request) = handed {
            self.hand_to_pool(request);
        }
    }
}

impl std::task::Wake for GuestRelayDriveSignal {
    fn wake(self: Arc<Self>) {
        self.resume();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.resume();
    }
}

pub(super) struct GuestRelayCompletionSlot {
    pub(super) value: Mutex<Option<GuestRelayCompletion>>,
    pub(super) wake: AtomicBool,
    signal: Arc<GuestRelayDriveSignal>,
}

impl GuestRelayCompletionSlot {
    pub(crate) fn new(pool: WorkerPool) -> Self {
        Self { value: Mutex::new(None), wake: AtomicBool::new(false), signal: Arc::new(GuestRelayDriveSignal { state: Mutex::new(GuestRelayDriveState::Idle), owner: Mutex::new(None), pool }) }
    }

    /// 🎟️ Retains the request this slot answers; it owes its first poll to the owner.
    pub(crate) fn admit(&self, request: GuestRelayInlineRequest) {
        *self.signal.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayDriveState::Owed(request);
    }

    /// 🏃️ Gives an owed request its first poll on the calling thread. A panic reaches the request's
    /// own recovery exactly as the pool-hosted future's did, so the instance is restored or
    /// quarantined and the owner still receives one completion.
    pub(crate) fn drive(&self) {
        let mut request = {
            let mut state = self.signal.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match std::mem::replace(&mut *state, GuestRelayDriveState::Idle) {
                GuestRelayDriveState::Owed(request) => {
                    *state = GuestRelayDriveState::Polling { woken: false };
                    request
                }
                other => {
                    *state = other;
                    return;
                }
            }
        };
        let waker = std::task::Waker::from(Arc::clone(&self.signal));
        let mut context = std::task::Context::from_waker(&waker);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| request.future.as_mut().poll(&mut context)));
        let mut state = self.signal.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let woken = matches!(*state, GuestRelayDriveState::Polling { woken: true });
        match polled {
            Ok(std::task::Poll::Ready(())) => *state = GuestRelayDriveState::Idle,
            Ok(std::task::Poll::Pending) if woken => {
                *state = GuestRelayDriveState::Handed;
                drop(state);
                self.signal.hand_to_pool(request);
            }
            Ok(std::task::Poll::Pending) => *state = GuestRelayDriveState::Suspended(request),
            Err(_) => {
                *state = GuestRelayDriveState::Idle;
                drop(state);
                if let Some(failure) = request.failure.take() {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| failure(GuestRelayPoolFailure::FuturePanicked)));
                }
            }
        }
    }

    /// 🧺️ An owner that goes away before the first poll hands the request to the pool, so nothing
    /// it would have admitted is lost.
    pub(super) fn abandon(&self) {
        let owed = {
            let mut state = self.signal.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match std::mem::replace(&mut *state, GuestRelayDriveState::Idle) {
                GuestRelayDriveState::Owed(request) => {
                    *state = GuestRelayDriveState::Handed;
                    Some(request)
                }
                other => {
                    *state = other;
                    None
                }
            }
        };
        if let Some(request) = owed {
            self.signal.hand_to_pool(request);
        }
    }

    pub(super) fn try_take(&self) -> Option<GuestRelayCompletion> {
        self.drive();
        let completion = self.value.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if completion.is_some() {
            self.wake.store(false, std::sync::atomic::Ordering::Release);
        }
        completion
    }

    fn register_wake(&self, waker: &std::task::Waker) {
        if self.wake.load(std::sync::atomic::Ordering::Acquire) || self.signal.is_owed() {
            waker.wake_by_ref();
            return;
        }
        *self.signal.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(waker.clone());
        if self.wake.load(std::sync::atomic::Ordering::Acquire) {
            self.signal.wake_owner();
        }
    }
}
//#endregion 🔀️PostTurnRelay
