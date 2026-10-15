//! 🔀️ The post-turn relay of the plugin host: guest relay pool, instance slots and leases, cold relay jobs, the mounted registry and the plugin instance handle.

use super::*;

//#region 🔀️PostTurnRelay
/// ⛽️ Every cold relay turn stays inside the user-visible interactive slice. The guest job owns its
/// persistent state; the host grants one `step-job` call per shared-pool closure.
pub(super) const RELAY_JOB_BUDGET: JobBudget = JobBudget { fuel: semio_framework_job::USER_VISIBLE_LANE_FUEL, deadline_ms: (semio_framework_job::USER_VISIBLE_LANE_WALL_US / 1_000) as u32 };

/// 🔁️ One retained-waker future polled once per finite shared-pool turn.
type GuestRelayFailureHandler = Box<dyn FnOnce(GuestRelayPoolFailure) + Send + 'static>;

struct GuestRelayPoolFuture {
    pool: WorkerPool,
    lane: Lane,
    future: Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>>>,
    failure_handler: Mutex<Option<GuestRelayFailureHandler>>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    complete: AtomicBool,
}

#[derive(Clone, Copy)]
enum GuestRelayPoolFailure {
    FuturePanicked,
    Admission(semio_framework_async::WorkerSubmitErrorKind),
}

impl GuestRelayPoolFuture {
    fn spawn_recoverable(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: impl FnOnce(GuestRelayPoolFailure) + Send + 'static) {
        Self::spawn_inner(pool, lane, future, Box::new(failure_handler));
    }

    fn spawn_inner(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: GuestRelayFailureHandler) {
        let task =
            Arc::new(Self { pool, lane, future: Mutex::new(Some(Box::pin(future))), failure_handler: Mutex::new(Some(failure_handler)), scheduled: AtomicBool::new(false), wake_requested: AtomicBool::new(false), complete: AtomicBool::new(false) });
        task.schedule();
    }

    pub(super) fn schedule(self: &Arc<Self>) {
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

    pub(super) fn fail(&self, failure: GuestRelayPoolFailure) {
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

enum GuestRelayRequest {
    Start { kind: String, input: Vec<u8> },
    Step,
    Cancel,
}

#[derive(Clone, Copy)]
enum GuestRelayRequestKind {
    Start,
    Step,
    Cancel,
}

impl GuestRelayRequest {
    pub(super) fn kind(&self) -> GuestRelayRequestKind {
        match self {
            Self::Start { .. } => GuestRelayRequestKind::Start,
            Self::Step => GuestRelayRequestKind::Step,
            Self::Cancel => GuestRelayRequestKind::Cancel,
        }
    }
}

pub(super) enum GuestRelayCompletion {
    Started(Result<(), TurnFault>),
    Stepped(Result<GuestRelayStepCompletion, TurnFault>),
    Cancelled,
    Rejected(GuestRelayOwnedBytes),
    Fault(GuestRelayOwnedBytes),
    TerminalFault(GuestRelayOwnedBytes),
}

enum GuestRelayStepCompletion {
    Running { progress: Option<GuestRelayOwnedBytes> },
    Done { output: GuestRelayOwnedBytes },
    Failed { error: GuestRelayOwnedBytes },
}

impl GuestRelayStepCompletion {
    fn from_guest(step: JobStep) -> Self {
        match step {
            JobStep::Running { progress } => Self::Running { progress: progress.map(GuestRelayOwnedBytes::new) },
            JobStep::Done { output } => Self::Done { output: GuestRelayOwnedBytes::new(output) },
            JobStep::Failed { error } => Self::Failed { error: GuestRelayOwnedBytes::new(error) },
        }
    }
}

struct GuestRelayOwnedBytes {
    source: Vec<u8>,
}

impl GuestRelayOwnedBytes {
    pub(super) fn new(source: Vec<u8>) -> Self {
        Self { source }
    }

    fn into_source(self) -> Vec<u8> {
        self.source
    }
}

#[derive(Clone)]
struct GuestRelayCompletionSender {
    slot: Arc<GuestRelayCompletionSlot>,
}

impl GuestRelayCompletionSender {
    pub(super) fn new(slot: Arc<GuestRelayCompletionSlot>) -> Self {
        Self { slot }
    }

    pub(super) fn send(&self, completion: GuestRelayCompletion) {
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
struct GuestRelayInlineRequest {
    future: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>,
    failure: Option<GuestRelayFailureHandler>,
}

enum GuestRelayDriveState {
    Idle,
    Owed(GuestRelayInlineRequest),
    Polling { woken: bool },
    Suspended(GuestRelayInlineRequest),
    Handed,
}

struct GuestRelayDriveSignal {
    state: Mutex<GuestRelayDriveState>,
    owner: Mutex<Option<std::task::Waker>>,
    pool: WorkerPool,
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

    pub(super) fn resume(&self) {
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

struct GuestRelayCompletionSlot {
    value: Mutex<Option<GuestRelayCompletion>>,
    wake: AtomicBool,
    signal: Arc<GuestRelayDriveSignal>,
}

impl GuestRelayCompletionSlot {
    pub(super) fn new(pool: WorkerPool) -> Self {
        Self { value: Mutex::new(None), wake: AtomicBool::new(false), signal: Arc::new(GuestRelayDriveSignal { state: Mutex::new(GuestRelayDriveState::Idle), owner: Mutex::new(None), pool }) }
    }

    /// 🎟️ Retains the request this slot answers; it owes its first poll to the owner.
    pub(super) fn admit(&self, request: GuestRelayInlineRequest) {
        *self.signal.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayDriveState::Owed(request);
    }

    /// 🏃️ Gives an owed request its first poll on the calling thread. A panic reaches the request's
    /// own recovery exactly as the pool-hosted future's did, so the instance is restored or
    /// quarantined and the owner still receives one completion.
    pub(super) fn drive(&self) {
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
    fn abandon(&self) {
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

    fn try_take(&self) -> Option<GuestRelayCompletion> {
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

enum GuestInstanceSlot {
    Available(GuestInstance),
    Leased,
    CleanupPending { instance: Option<GuestInstance>, detail: Vec<u8> },
    Quarantined { instance: GuestInstance, detail: Vec<u8> },
}

impl GuestInstanceSlot {
    #[cfg(test)]
    fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }

    #[cfg(test)]
    fn is_quarantined(&self) -> bool {
        matches!(self, Self::Quarantined { .. })
    }

    #[cfg(test)]
    fn is_cleanup_pending(&self) -> bool {
        matches!(self, Self::CleanupPending { .. })
    }

    fn admission_fault(&self) -> Option<Vec<u8>> {
        match self {
            Self::CleanupPending { detail, .. } | Self::Quarantined { detail, .. } => Some(detail.clone()),
            Self::Leased => Some(b"plugin instance is busy retiring a prior relay".to_vec()),
            Self::Available(_) => None,
        }
    }
}

enum GuestInstanceLeaseOrigin {
    Mounted,
    Cleanup,
}

enum GuestInstanceLeaseDisposition {
    Available,
    CleanupResolved,
    CleanupPending(Vec<u8>),
    Quarantined(Vec<u8>),
}

struct GuestInstanceLease {
    slot: Arc<Mutex<GuestInstanceSlot>>,
    guest: Option<GuestInstance>,
    origin: GuestInstanceLeaseOrigin,
    disposition: GuestInstanceLeaseDisposition,
}

impl GuestInstanceLease {
    fn acquire(slot: Arc<Mutex<GuestInstanceSlot>>, unwind_detail: Vec<u8>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::Available(guest) => Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Mounted, disposition: GuestInstanceLeaseDisposition::CleanupPending(unwind_detail) }),
            GuestInstanceSlot::Leased => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Leased;
                Err(b"plugin instance missing behind an acquired relay permit".to_vec())
            }
            pending @ GuestInstanceSlot::CleanupPending { .. } => {
                let error = pending.admission_fault().expect("cleanup-pending slot owns its detail");
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = pending;
                Err(error)
            }
            GuestInstanceSlot::Quarantined { instance, detail } => {
                let error = detail.clone();
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Quarantined { instance, detail };
                Err(error)
            }
        }
    }

    fn acquire_cleanup(slot: Arc<Mutex<GuestInstanceSlot>>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::CleanupPending { instance: Some(guest), detail } => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::CleanupPending { instance: None, detail };
                Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Cleanup, disposition: GuestInstanceLeaseDisposition::Quarantined(b"plugin instance quarantined after cleanup cancel-job panic".to_vec()) })
            }
            state => {
                let error = state.admission_fault().unwrap_or_else(|| b"plugin instance was not cleanup-pending behind an acquired cleanup permit".to_vec());
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = state;
                Err(error)
            }
        }
    }

    pub(super) fn guest(&mut self) -> &mut GuestInstance {
        self.guest.as_mut().expect("guest instance lease owns its instance")
    }

    pub(super) fn quarantine(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::Quarantined(detail);
    }

    fn cleanup_pending(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupPending(detail);
    }

    pub(super) fn available(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::Available;
    }

    fn cleanup_resolved(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupResolved;
    }
}

impl Drop for GuestInstanceLease {
    fn drop(&mut self) {
        let Some(guest) = self.guest.take() else {
            return;
        };
        let mut slot = self.slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let external_cleanup_detail = match (&self.origin, &*slot) {
            (GuestInstanceLeaseOrigin::Mounted, GuestInstanceSlot::CleanupPending { instance: None, detail }) => Some(detail.clone()),
            _ => None,
        };
        let expected = match self.origin {
            GuestInstanceLeaseOrigin::Mounted => matches!(&*slot, GuestInstanceSlot::Leased | GuestInstanceSlot::CleanupPending { instance: None, .. }),
            GuestInstanceLeaseOrigin::Cleanup => matches!(&*slot, GuestInstanceSlot::CleanupPending { instance: None, .. }),
        };
        if expected {
            *slot = match std::mem::replace(&mut self.disposition, GuestInstanceLeaseDisposition::Quarantined(b"plugin instance lease lost its disposition".to_vec())) {
                GuestInstanceLeaseDisposition::Available => match external_cleanup_detail {
                    Some(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                    None => GuestInstanceSlot::Available(guest),
                },
                GuestInstanceLeaseDisposition::CleanupResolved => GuestInstanceSlot::Available(guest),
                GuestInstanceLeaseDisposition::CleanupPending(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                GuestInstanceLeaseDisposition::Quarantined(detail) => GuestInstanceSlot::Quarantined { instance: guest, detail },
            };
            return;
        }
        *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
            GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during cleanup recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: None, .. } | GuestInstanceSlot::Leased => GuestInstanceSlot::Quarantined { instance: guest, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
        };
    }
}

fn quarantine_guest_instance(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail },
        GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail },
        pending @ GuestInstanceSlot::CleanupPending { instance: None, .. } => pending,
        GuestInstanceSlot::Leased => GuestInstanceSlot::Leased,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

fn mark_guest_cleanup_pending(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::CleanupPending { instance: Some(instance), detail },
        GuestInstanceSlot::Leased => GuestInstanceSlot::CleanupPending { instance: None, detail },
        pending @ GuestInstanceSlot::CleanupPending { .. } => pending,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

async fn cancel_guest_job_once(runtime: &GuestRuntimes, instance: &mut GuestInstance, job: u64, cancel_admitted: &AtomicBool) -> Result<bool, TurnFault> {
    if cancel_admitted.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
        runtime.cancel_job(instance, job).await?;
        return Ok(true);
    }
    Ok(false)
}

fn cancel_guest_job_failure_detail(error: &TurnFault) -> Vec<u8> {
    format!("plugin instance quarantined after cancel-job failure: {error}").into_bytes()
}

fn guest_relay_cleanup_pending_detail(stage: &str, error: &TurnFault) -> Vec<u8> {
    format!("plugin instance cleanup pending after {stage} failure: {error}").into_bytes()
}

fn foreground_cancel_completion(result: Result<bool, TurnFault>, guest: &mut GuestInstanceLease) -> GuestRelayCompletion {
    match result {
        Ok(true) => {
            guest.cleanup_resolved();
            GuestRelayCompletion::Cancelled
        }
        Ok(false) => {
            let detail = b"plugin instance quarantined because cancel-job admission was already consumed".to_vec();
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
        Err(error) => {
            let detail = cancel_guest_job_failure_detail(&error);
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
    }
}

#[derive(Clone)]
struct GuestRelayAttempt {
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    pool: WorkerPool,
    cancel_scheduled: Arc<AtomicBool>,
    cancel_admitted: Arc<AtomicBool>,
    job: u64,
    sender: GuestRelayCompletionSender,
}

async fn run_guest_relay_request(attempt: GuestRelayAttempt, cancel: semio_framework_async::CancelToken, request: GuestRelayRequest) {
    let GuestRelayAttempt { runtime, instance, instance_gate, pool: _pool, cancel_scheduled: _cancel_scheduled, cancel_admitted, job, sender } = attempt;
    let permit = if matches!(&request, GuestRelayRequest::Cancel) {
        instance_gate.acquire_owned().await
    } else {
        match semio_framework_async::select2(instance_gate.acquire_owned(), cancel.cancelled()).await {
            semio_framework_async::Either::Left(permit) => permit,
            semio_framework_async::Either::Right(()) => {
                sender.send(GuestRelayCompletion::Cancelled);
                return;
            }
        }
    };
    let unwind_detail = match &request {
        GuestRelayRequest::Cancel => b"plugin instance quarantined after cancel-job panic".to_vec(),
        GuestRelayRequest::Start { .. } | GuestRelayRequest::Step => b"plugin instance cleanup pending after guest relay panic".to_vec(),
    };
    let mut guest = match if matches!(&request, GuestRelayRequest::Cancel) { GuestInstanceLease::acquire_cleanup(Arc::clone(&instance)) } else { GuestInstanceLease::acquire(Arc::clone(&instance), unwind_detail) } {
        Ok(guest) => guest,
        Err(error) => {
            sender.send(GuestRelayCompletion::Rejected(GuestRelayOwnedBytes::new(error)));
            return;
        }
    };
    let mut cleanup_pending = false;
    let completion = match request {
        GuestRelayRequest::Start { kind, input } => match semio_framework_async::select2(runtime.start_job(guest.guest(), job, &kind, input), cancel.cancelled()).await {
            semio_framework_async::Either::Left(Ok(())) => {
                guest.available();
                GuestRelayCompletion::Started(Ok(()))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("start-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Started(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Step => match semio_framework_async::select2(runtime.step_job(guest.guest(), job, RELAY_JOB_BUDGET), cancel.cancelled()).await {
            semio_framework_async::Either::Left(Ok(step)) => {
                guest.available();
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::from_guest(step)))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("step-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Stepped(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Cancel => {
            let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
            foreground_cancel_completion(result, &mut guest)
        }
    };
    drop(guest);
    drop(permit);
    #[cfg(test)]
    wait_for_scripted_guest_relay_release(&runtime, &completion).await;
    let _ = cleanup_pending;
    sender.send(completion);
}

fn recover_guest_relay_failure(attempt: GuestRelayAttempt, request: GuestRelayRequestKind, failure: GuestRelayPoolFailure) {
    let GuestRelayAttempt { runtime, instance, instance_gate, pool, cancel_scheduled, cancel_admitted, job, sender } = attempt;
    if let GuestRelayPoolFailure::Admission(kind) = failure {
        let detail = match kind {
            semio_framework_async::WorkerSubmitErrorKind::Shutdown => b"plugin instance quarantined after relay scheduler shutdown".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Poisoned => b"plugin instance quarantined after relay scheduler poison".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => b"plugin instance quarantined after non-retryable relay admission failure".to_vec(),
        };
        quarantine_guest_instance(&instance, detail.clone());
        sender.send(GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail)));
        return;
    }
    #[cfg(test)]
    if let GuestRuntimes::Mock(mock) = runtime.as_ref() {
        let barrier = match request {
            GuestRelayRequestKind::Start => mock.take_relay_start_failure_release(),
            GuestRelayRequestKind::Step => mock.take_relay_step_failure_release(),
            GuestRelayRequestKind::Cancel => None,
        };
        if let Some(barrier) = barrier {
            let panic_instance = Arc::clone(&instance);
            GuestRelayPoolFuture::spawn_recoverable(
                pool.clone(),
                Lane::UserVisible,
                async move {
                    barrier.wait().await;
                    recover_guest_relay_failure(GuestRelayAttempt { runtime, instance, instance_gate, pool, cancel_scheduled, cancel_admitted, job, sender }, request, GuestRelayPoolFailure::FuturePanicked);
                },
                move |_| quarantine_guest_instance(&panic_instance, b"plugin instance quarantined after relay panic recovery panic".to_vec()),
            );
            return;
        }
    }
    let detail = if matches!(request, GuestRelayRequestKind::Cancel) || cancel_admitted.load(std::sync::atomic::Ordering::Acquire) {
        let detail = b"plugin instance quarantined after cancel-job panic";
        quarantine_guest_instance(&instance, detail.to_vec());
        detail.to_vec()
    } else {
        let _ = (runtime, instance_gate, pool, job, cancel_scheduled, cancel_admitted);
        mark_guest_cleanup_pending(&instance, b"plugin instance cleanup pending after guest relay panic".to_vec());
        b"plugin guest relay panicked after acquiring its instance".to_vec()
    };
    sender.send(if matches!(request, GuestRelayRequestKind::Cancel) { GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail)) } else { GuestRelayCompletion::Fault(GuestRelayOwnedBytes::new(detail)) });
}

#[derive(Clone, Copy)]
enum GuestRelayPublicationKind {
    Preview,
    Commit,
    Fault,
}

struct GuestRelayPublication {
    kind: GuestRelayPublicationKind,
    source: std::mem::ManuallyDrop<Vec<u8>>,
    cursor: usize,
    copied: bool,
    retired: bool,
    oversized: bool,
    delivered: bool,
    writer: Option<semio_framework_job::RetainedPayloadBuilder>,
}

impl GuestRelayPublication {
    pub(super) fn new(kind: GuestRelayPublicationKind, source: Vec<u8>) -> Self {
        let oversized = source.len() > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES;
        let kind = if oversized { GuestRelayPublicationKind::Fault } else { kind };
        let stream = match kind {
            GuestRelayPublicationKind::Preview => semio_framework_job::JobPayloadStream::Preview,
            GuestRelayPublicationKind::Commit => semio_framework_job::JobPayloadStream::CommitOutput,
            GuestRelayPublicationKind::Fault => semio_framework_job::JobPayloadStream::Fault,
        };
        Self { kind, source:std::mem::ManuallyDrop::new(source), cursor: 0, copied: false, retired: false, oversized, delivered: false, writer: Some(semio_framework_job::RetainedPayloadBuilder::new(stream)) }
    }

    /// 🏁️ A commit or fault ends the relay; only a preview is retired inside the running job.
    fn is_terminal(&self) -> bool {
        !matches!(self.kind, GuestRelayPublicationKind::Preview)
    }

    pub(super) fn invalid(detail: &'static str) -> semio_framework_value::ValueError {
        semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, detail)
    }

    /// ✍️ Pays one frontier of the publication (bind, append, retire the source, seal) and lends the sealed pages once.
    pub(super) fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.delivered {
            return Ok(None);
        }
        if !self.writer.as_ref().ok_or_else(|| Self::invalid("guest relay publication lost its payload builder"))?.is_initialized() {
            self.writer.as_mut().expect("guest relay publication owns builder").advance_initialization(context)?;
            return Ok(None);
        }
        if !self.copied {
            let bytes = if self.oversized { &b"plugin-guest-relay-output-limit"[..] } else { self.source.as_slice() };
            let writer = self.writer.as_mut().expect("guest relay publication owns builder");
            if writer.append_original(context, bytes, &mut self.cursor)? {
                self.copied = true;
            }
            context.consume_fuel(1);
            return Ok(None);
        }
        if !self.retired {
            if self.source.is_empty() && self.source.capacity() == 0 {
                let grant = context.retained_grant();
                let bytes = size_of::<bool>();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth < 1 {
                    return Ok(None);
                }
                self.retired = true;
                context.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                context.consume_fuel(1);
                return Ok(None);
            }
            let step = self.close_original_source(context.retained_grant()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))?;
            context.consume_retained(step.progress())?;
            context.consume_fuel(1);
            return Ok(None);
        }
        let writer = self.writer.as_mut().expect("guest relay publication owns builder");
        if writer.published().is_none() {
            writer.seal(context)?;
            return Ok(None);
        }
        let payload = writer.published().ok_or_else(|| Self::invalid("guest relay publication lost its sealed payload"))?;
        let outcome = match self.kind {
            GuestRelayPublicationKind::Preview => semio_framework_job::JobOutcomeBorrow::admit_preview(context, payload)?,
            GuestRelayPublicationKind::Commit => semio_framework_job::JobOutcomeBorrow::admit_complete(context, None, Some(payload))?,
            GuestRelayPublicationKind::Fault => semio_framework_job::JobOutcomeBorrow::admit_fault(context, payload)?,
        };
        self.delivered |= outcome.is_some();
        Ok(outcome)
    }

    /// 🤝️ Resolves a descriptor against the same sealed pages this publication lent.
    pub(super) fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        let payload = self.writer.as_ref().and_then(semio_framework_job::RetainedPayloadBuilder::published).filter(|_| self.delivered).ok_or_else(|| Self::invalid("guest relay descriptor requires its delivered payload"))?;
        match self.kind {
            GuestRelayPublicationKind::Preview => descriptor.preview(payload),
            GuestRelayPublicationKind::Commit => descriptor.complete(None, Some(payload)),
            GuestRelayPublicationKind::Fault => descriptor.fault(payload),
        }
    }

    /// ♻️ Retires a delivered non-terminal publication's pages inside the next running step; true once nothing is left.
    pub(super) fn retire(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if let Some(writer) = self.writer.as_mut() {
            if !writer.terminal_is_empty() {
                let step = writer.close_step_granted(context.retained_grant()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))?;
                context.consume_retained(step.progress())?;
                return Ok(false);
            }
        }
        self.writer = None;
        Ok(self.terminal_is_empty())
    }

    pub(super) fn begin_close(&mut self) {}

    fn original_source_demand(&self)->semio_framework_value::RetirementDemand{
        if !self.source.is_empty(){semio_framework_value::RetirementDemand{copy_bytes:1+size_of::<usize>(),depth:1,..Default::default()}}
        else if self.source.capacity()!=0{semio_framework_value::RetirementDemand{copy_bytes:size_of::<Vec<u8>>(),release_bytes:self.source.capacity(),depth:1,..Default::default()}}
        else{Default::default()}
    }

    fn close_original_source(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementTurnError};
        if self.source.is_empty()&&self.source.capacity()==0{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.original_source_demand();
        semio_framework_value::advance_retirement_turn(demand,grant,|grant|{
            if !self.source.is_empty(){let count=self.source.len().min(grant.maximum_copy_bytes-size_of::<usize>());let keep=self.source.len()-count;self.source.truncate(keep);return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:count+size_of::<usize>(),..Default::default()}),false));}
            drop(std::mem::take(&mut*self.source));
            Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),self.source.capacity()==0))
        }).map_err(|error|match error{RetirementTurnError::Owner(error)|RetirementTurnError::Receipt(error)=>error})
    }

    pub(super) fn retirement_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(writer)=self.writer.as_ref(){return if writer.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::RetainedPayloadBuilder>>(),depth:1,..Default::default()})}else{let mut demand=writer.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"Host publication writer depth overflow"))?;Ok(demand)};}
        Ok(self.original_source_demand())
    }

    pub(super) fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let demand=self.retirement_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(S::Pending{progress:Default::default()});}
        if let Some(writer)=self.writer.as_mut(){
            if writer.terminal_is_empty(){drop(self.writer.take());return Ok(S::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}});}
            return writer.close_step_granted(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|S::Pending{progress:step.progress()});
        }
        self.close_original_source(grant).map(|step|match step{semio_framework_value::RetainedCloneStep::Progress(progress)=>S::Pending{progress},semio_framework_value::RetainedCloneStep::Complete(progress)=>S::Complete{progress}})
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.source.is_empty() && self.source.capacity() == 0
    }
}

impl Drop for GuestRelayPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Host publication abandoned original writer or source backing");if self.source.is_empty()&&self.source.capacity()==0{unsafe{std::mem::ManuallyDrop::drop(&mut self.source);}}}}

fn receive_relay_close(result:Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    let step=result.map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
    cx.consume_retained(step.progress())?;
    if step.progress().copied_items!=0{cx.consume_fuel(1)}
    Ok(step)
}
#[cfg(test)]
#[path="../♻️publication/🧪️tests/🦀️.rs"]
mod original_publication_source_tests;

struct GuestColdRelayJob {
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    pool: WorkerPool,
    cancel: semio_framework_async::CancelToken,
    cancel_scheduled: Arc<AtomicBool>,
    cancel_admitted: Arc<AtomicBool>,
    job: u64,
    start: Option<(String, Vec<u8>)>,
    pending: Option<Arc<GuestRelayCompletionSlot>>,
    publication: Option<GuestRelayPublication>,
    closing: bool,
    start_submitted: bool,
    started: bool,
    cleanup_required: bool,
    terminal_delivered: bool,
}

impl GuestColdRelayJob {
    fn original_close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let Some(publication)=self.publication.as_ref()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host relay runtime, Pool, completion and cancellation owners require native retirement"));};
        if publication.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<GuestRelayPublication>>(),depth:1,..Default::default()});}
        let mut demand=publication.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Host relay publication depth overflow"))?;Ok(demand)
    }

    pub(super) fn submit(&mut self, request: GuestRelayRequest) {
        let request_kind = request.kind();
        let slot = Arc::new(GuestRelayCompletionSlot::new(self.pool.clone()));
        let attempt = GuestRelayAttempt {
            runtime: Arc::clone(&self.runtime),
            instance: Arc::clone(&self.instance),
            instance_gate: Arc::clone(&self.instance_gate),
            pool: self.pool.clone(),
            cancel_scheduled: Arc::clone(&self.cancel_scheduled),
            cancel_admitted: Arc::clone(&self.cancel_admitted),
            job: self.job,
            sender: GuestRelayCompletionSender::new(Arc::clone(&slot)),
        };
        let recovery = attempt.clone();
        slot.admit(GuestRelayInlineRequest { future: Box::pin(run_guest_relay_request(attempt, self.cancel.clone(), request)), failure: Some(Box::new(move |failure| recover_guest_relay_failure(recovery, request_kind, failure))) });
        self.pending = Some(slot);
    }

    fn submit_cleanup(&mut self) {
        self.cancel.cancel_now();
        if self.start_submitted && self.pending.is_none() && self.cancel_scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            self.submit(GuestRelayRequest::Cancel);
        }
    }

}

impl semio_framework_job::InteractiveJob for GuestColdRelayJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.delivered) {
            if self.publication.as_ref().is_some_and(GuestRelayPublication::is_terminal) {
                return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let retired = self.publication.as_mut().expect("delivered guest relay publication").retire(context)?;
            if retired {
                let bytes = size_of::<Option<GuestRelayPublication>>();
                let grant = context.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth < 1 {
                    return Ok(None);
                }
                context.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                self.publication = None;
            }
            return Ok(None);
        }
        if self.publication.is_some() {
            let terminal = self.publication.as_ref().is_some_and(GuestRelayPublication::is_terminal);
            let outcome = self.publication.as_mut().expect("guest relay publication was just observed").step(context)?;
            if outcome.is_some() && terminal {
                self.terminal_delivered = true;
            }
            return Ok(outcome);
        }
        if self.terminal_delivered {
            return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if context.is_cancelled() || self.cancel.is_cancelled_now() {
            self.cancel.cancel_now();
            if self.pending.is_none() && self.cleanup_required {
                self.submit_cleanup();
                return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if self.pending.is_none() {
                self.terminal_delivered = true;
                return JobOutcomeBorrow::admit_cancelled(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        if let Some(slot) = &self.pending {
            let Some(completion) = slot.try_take() else { return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from) };
            self.pending = None;
            match completion {
                GuestRelayCompletion::Started(Ok(())) => {
                    self.started = true;
                }
                GuestRelayCompletion::Started(Err(error)) | GuestRelayCompletion::Stepped(Err(error)) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.to_string().into_bytes()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: Some(progress) })) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Preview, progress.into_source()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: None })) => {}
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Done { output })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output.into_source()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Failed { error })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Rejected(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Fault(error) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::TerminalFault(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Cancelled => {
                    self.cleanup_required = false;
                    self.terminal_delivered = true;
                    return JobOutcomeBorrow::admit_cancelled(context).map_err(semio_framework_job::OriginalTurnFailure::from);
                }
            }
            return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if let Some((kind, input)) = self.start.take() {
            self.start_submitted = true;
            self.cleanup_required = true;
            self.submit(GuestRelayRequest::Start { kind, input });
        } else if self.started {
            self.submit(GuestRelayRequest::Step);
        } else {
            self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, b"plugin guest relay lost its start state".to_vec()));
        }
        JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.publication.as_ref().ok_or_else(|| GuestRelayPublication::invalid("guest relay descriptor requires its publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.cancel.cancel_now();
        if let Some(publication) = self.publication.as_mut() {
            publication.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        if !self.closing{self.begin_close();}
        if let Some(publication)=self.publication.as_mut(){
            if publication.terminal_is_empty(){
                let bytes=size_of::<Option<GuestRelayPublication>>();
                if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
                let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};cx.consume_retained(progress)?;
                drop(self.publication.take());cx.consume_fuel(1);return Ok(S::Pending{progress});
            }
            if grant.maximum_depth==0{return Ok(S::Pending{progress:Default::default()});}
            return receive_relay_close(publication.close_step(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|match step{S::Complete{progress}=>S::Pending{progress},step=>step}),cx);
        }
        Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host relay native retirement is undeclared"),cx))
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        false
    }
}

impl Drop for GuestColdRelayJob {
    fn drop(&mut self) {
        if self.cleanup_required || self.start.is_some() || self.pending.is_some() || self.publication.as_ref().is_some_and(|publication| !publication.terminal_is_empty()) {
            self.cancel.cancel_now();
            self.closing = true;
        }
        if let Some(slot) = self.pending.take() {
            slot.abandon();
        }
    }
}

impl GuestColdRelayJob {
    pub(super) fn new(runtime: Arc<GuestRuntimes>, instance: Arc<Mutex<GuestInstanceSlot>>, instance_gate: Arc<semio_framework_async::Semaphore>, pool: WorkerPool, cancel: semio_framework_async::CancelToken, job: u64, start: (String, Vec<u8>)) -> Self {
        Self {
            runtime,
            instance,
            instance_gate,
            pool,
            cancel,
            cancel_scheduled: Arc::new(AtomicBool::new(false)),
            cancel_admitted: Arc::new(AtomicBool::new(false)),
            job,
            start: Some(start),
            pending: None,
            publication: None,
            closing: false,
            start_submitted: false,
            started: false,
            cleanup_required: false,
            terminal_delivered: false,
        }
    }
}

const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;
const GUEST_RELAY_REAPER_FALLBACK_MS: u64 = 8;

struct GuestRelayLifecycleProbeControl {
    awake: AtomicBool,
    release_permits: std::sync::atomic::AtomicUsize,
    waker: Mutex<Option<std::task::Waker>>,
    close_admissions: std::sync::atomic::AtomicUsize,
    releases: std::sync::atomic::AtomicUsize,
}

impl GuestRelayLifecycleProbeControl {
    pub(super) fn new() -> Self {
        Self { awake: AtomicBool::new(false), release_permits: std::sync::atomic::AtomicUsize::new(0), waker: Mutex::new(None), close_admissions: std::sync::atomic::AtomicUsize::new(0), releases: std::sync::atomic::AtomicUsize::new(0) }
    }

    pub(super) fn wake(&self) {
        self.awake.store(true, std::sync::atomic::Ordering::Release);
        self.wake_registered();
    }

    fn release_one(&self) {
        self.release_permits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.wake_registered();
    }

    fn wake_registered(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }
}

struct GuestRelayLifecycleProbeJob {
    control: Arc<GuestRelayLifecycleProbeControl>,
    remaining: usize,
    terminal_output: Option<Vec<u8>>,
    publication: Option<GuestRelayPublication>,
    closing: bool,
}

impl GuestRelayLifecycleProbeJob {
    fn publication_demand(&self)->Option<Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>>{
        let publication=self.publication.as_ref()?;
        if publication.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<GuestRelayPublication>>(),depth:1,..Default::default()}));}
        Some(publication.retirement_demands().and_then(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"lifecycle probe publication depth overflow"))?;Ok(demand)}))
    }
}

impl semio_framework_job::InteractiveJob for GuestRelayLifecycleProbeJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.publication.is_none() {
            let Some(output) = self.terminal_output.take() else { return semio_framework_job::JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from) };
            self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output));
            return Ok(None);
        }
        let publication = self.publication.as_mut().expect("lifecycle probe publication was just observed");
        if publication.delivered {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        publication.step(context)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.publication.as_ref().ok_or_else(|| GuestRelayPublication::invalid("lifecycle probe descriptor requires its publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            self.control.close_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        if let Some(publication)=self.publication.as_mut(){
            if publication.terminal_is_empty(){
                let bytes=size_of::<Option<GuestRelayPublication>>();
                if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
                let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};cx.consume_retained(progress)?;
                drop(self.publication.take());cx.consume_fuel(1);return Ok(S::Pending{progress});
            }
            if grant.maximum_depth==0{return Ok(S::Pending{progress:Default::default()});}
            return receive_relay_close(publication.close_step(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|match step{S::Complete{progress}=>S::Pending{progress},step=>step}),cx);
        }
        if self.remaining==0{return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host lifecycle control retirement is undeclared"),cx))}
        if !self.control.awake.load(std::sync::atomic::Ordering::Acquire){return Ok(S::Blocked);}
        let bytes=size_of::<usize>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
        if self.control.release_permits.try_update(std::sync::atomic::Ordering::AcqRel,std::sync::atomic::Ordering::Acquire,|permits|permits.checked_sub(1)).is_err(){return Ok(S::Blocked);}
        self.remaining-=1;self.control.releases.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
        receive_relay_close(Ok(S::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}),cx)
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.copy_bytes);}if self.remaining==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original lifecycle control Arc, wake and output owners require native retirement"));}Ok(size_of::<usize>())}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.capacity_bytes);}self.next_close_copy_byte_demand().map(|_|0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.release_bytes);}self.next_close_copy_byte_demand().map(|_|0)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{if let Some(demand)=self.publication_demand(){return Ok(demand?.depth);}self.next_close_copy_byte_demand().map(|_|1)}

    fn terminal_is_empty(&self)->bool{false}
}

/// 🧾️ The original job and parameters a refused worker admission leaves with the registry until their paid close.
struct GuestRelayRefusedSource<J: semio_framework_job::InteractiveJob> {
    job: Option<J>,
    params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    closing: bool,
}

impl<J: semio_framework_job::InteractiveJob> GuestRelayRefusedSource<J> {
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none()
    }

    pub(super) fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(job) = self.job.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()? });
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.params.is_some()), ..Default::default() })
    }

    pub(super) fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            if let Some(job) = self.job.as_mut() {
                job.begin_close();
            }
        }
    }

    pub(super) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        self.begin_close();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        let item = semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if let Some(job) = self.job.as_mut() {
            if job.terminal_is_empty() {
                if grant.maximum_items == 0 {
                    return Ok(S::Pending { progress: Default::default() });
                }
                cx.consume_retained(item)?;drop(self.job.take());cx.consume_fuel(1);
                return Ok(S::Pending { progress: item });
            }
            let step=job.close_step(cx)?.admit(grant,job.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            return Ok(match step {
                S::Complete { progress } => S::Pending { progress },
                step => step,
            });
        }
        if self.params.is_some() {
            if grant.maximum_items == 0 {
                return Ok(S::Pending { progress: Default::default() });
            }
            cx.consume_retained(item)?;drop(self.params.take());cx.consume_fuel(1);
            return Ok(S::Pending { progress: item });
        }
        Ok(S::Complete { progress: Default::default() })
    }
}

/// 🚪️ Admits one session from the registry's own drive grant; a refusal keeps the original job and parameters.
fn admit_guest_relay_session<J: semio_framework_job::InteractiveJob + 'static>(job: J, params: semio_framework_job_worker_authority::BatchJobParams, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_job_worker::WorkerJobSession<J>, GuestRelayRefusedSource<J>> {
    let mut job = Some(job);
    let mut params = Some(params);
    let original = params.as_ref().expect("original session parameters");
    let (operation, generation, now_us) = (original.operation, original.generation, original.now_us);
    let mut recipient = semio_framework_value::RetainedCloneProgress::default();
    match semio_framework_job_worker_authority::WorkerJobAdmissionContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, grant), now_us, &mut recipient) {
        Ok(mut control) => match semio_framework_job_worker::WorkerJobSession::try_admit_owned(&mut job, &mut params, &mut control) {
            Ok(Some((session, _admission))) => Ok(session),
            Ok(None) | Err(_) => Err(GuestRelayRefusedSource { job, params, closing: false }),
        },
        Err(_) => Err(GuestRelayRefusedSource { job, params, closing: false }),
    }
}

#[expect(clippy::large_enum_variant, reason = "Failed worker admission must retain the exact job and credits for bounded retirement without allocating on rejection.")]
enum GuestRelayMountedOwner {
    Session(semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>),
    Rejected(GuestRelayRefusedSource<GuestColdRelayJob>),
    LifecycleProbe(semio_framework_job_worker::WorkerJobSession<GuestRelayLifecycleProbeJob>),
    Empty,
}

/// 🧾️ Preserves the exact original embedding policy and native wake effect before another child runs.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestRelayWakeReceipt{pub epoch:u64,pub slot:usize,pub generation:u64,pub grant:semio_framework_value::retained_clone::RetainedCloneGrant,pub progress:semio_framework_value::retained_clone::RetainedCloneProgress}
/// 📥️ The original embedding debits or retains a borrowed full receipt before admitting its successor.
pub trait GuestRelayWakeReceiptReceiver:Send+Sync{fn checkout(&self,slot:usize,generation:u64)->Result<Option<GuestRelayWakeTurnGrant>,semio_framework_value::ValueError>;fn collect(&self,receipt:&GuestRelayWakeReceipt)->Result<bool,semio_framework_value::ValueError>;}
/// 🚦️ Required first-party issuer and receipt receiver belong to the actual Host embedding.
#[derive(Clone)]
pub struct GuestRelayWakeAuthority{pub drive_policy:semio_framework_value::retained_clone::RetainedCloneGrant,pub policy:semio_framework_value::retained_clone::RetainedCloneGrant,pub issuer:semio_framework_job_worker::OriginalWorkerWakeIssuer,pub receiver:Arc<GuestRelayWakeDriver>}
#[path="../🔔️wake/🦀️.rs"]
mod original_wake_receipts;
pub use original_wake_receipts::{GuestRelayOriginalRoot,GuestRelayRootPolicy,GuestRelayDriverPolicy,GuestRelayDriverTreasury,GuestRelayDriverTurnGrant,GuestRelayWakeDriver,GuestRelayWakeContextTurn,GuestRelayWakeReceiptLedger,GuestRelayWakeTurnGrant,GuestRelayWakeTurnReceipt};
#[cfg(test)]
#[global_allocator]
static HOST_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
struct GuestRelayWakeCustody{original:std::mem::ManuallyDrop<Option<Box<dyn semio_framework_job_worker::RetainedWorkerWake>>>,receipt:Option<GuestRelayWakeReceipt>,registered:bool,refusal:Option<semio_framework_value::ValueRefusalKind>}
impl GuestRelayWakeCustody{fn new()->Self{Self{original:std::mem::ManuallyDrop::new(None),receipt:None,registered:false,refusal:None}}}
impl Drop for GuestRelayWakeCustody{fn drop(&mut self){assert!(std::thread::panicking()||(self.original.is_none()&&self.receipt.is_none()),"mounted relay abandoned original wake frame or receipt");if self.original.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.original);}}}}

struct GuestRelayMountedSession {
    wake:GuestRelayWakeCustody,
    generation: u64,
    owner: GuestRelayMountedOwner,
    checked_out: Option<semio_framework_job_worker::WorkerJobOutcome<GuestColdRelayJob>>,
    lifecycle_probe_checked_out: Option<semio_framework_job_worker::WorkerJobOutcome<GuestRelayLifecycleProbeJob>>,
    outcome_stage: GuestRelayOutcomeStage,
    outcome_kind: GuestRelayOutcomeKind,
    outcome_page: usize,
    output: GuestRelayMountedOutput,
    terminal: Option<GuestRelayMountedTerminal>,
    lifecycle: GuestRelayMountedLifecycle,
}

/// 🚦️ Where the checked-out semantic outcome of a mounted relay is: its pages are read once, then acknowledged, then handed back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestRelayOutcomeStage {
    Idle,
    Reading,
    Acknowledging,
}

/// 🏁️ What the lent outcome means for the relay: it goes on, or it is the terminal result the caller drains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestRelayOutcomeKind {
    Continues,
    Complete,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestRelayMountedLifecycle {
    Running,
    DrainingForCaller,
    DetachedForReap,
}

struct GuestRelayMountedOutput {
    storage: Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>,
    length: usize,
}

impl GuestRelayMountedOutput {
    pub(super) fn new() -> Self {
        let storage = Box::<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>::new_uninit();
        Self { storage: unsafe { storage.assume_init() }, length: 0 }
    }

    fn write_page(&mut self, page: &[u8]) -> Result<(), PluginHostError> {
        let end = self.length.checked_add(page.len()).filter(|end| *end <= semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES).ok_or_else(|| PluginHostError::Plugin("plugin cold relay mounted output exceeds its admitted authority".into()))?;
        for (target, byte) in self.storage[self.length..end].iter_mut().zip(page.iter().copied()) {
            target.write(byte);
        }
        self.length = end;
        Ok(())
    }

    pub(super) fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast::<u8>(), self.length) }
    }

    fn into_vec(self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

enum GuestRelayMountedTerminal {
    Complete,
    Cancelled,
    Fault,
    HostFault(PluginHostError),
}

enum GuestRelayMountedClose {
    Complete,
    Pending,
    Blocked { wake_registered: bool },
}

#[expect(clippy::large_enum_variant, reason = "The fixed registry allocates all slot storage before admission, including retained rejection and terminal owners.")]
enum GuestRelayMountedSlot {
    Empty,
    Reserved { generation: u64, output: GuestRelayMountedOutput },
    Mounted(GuestRelayMountedSession),
}

struct GuestRelayMountedRegistry {
    wake_authority:GuestRelayWakeAuthority,
    slots: Box<[Mutex<GuestRelayMountedSlot>]>,
    next_generation: std::sync::atomic::AtomicU64,
    pool: WorkerPool,
    reaper_cursor: std::sync::atomic::AtomicUsize,
    reaper_epoch: std::sync::atomic::AtomicU64,
    reaper_active: AtomicBool,
    reaper_failed: AtomicBool,
    reaper_timer_pending: AtomicBool,
    #[cfg(test)]
    reaper_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    reaper_timers: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    reaper_wake_waits: std::sync::atomic::AtomicUsize,
}

impl GuestRelayMountedRegistry {
    pub(super) fn new(wake_authority:GuestRelayWakeAuthority) -> Self {
        Self::with_pool(plugin_host_worker_pool(),wake_authority)
    }

    pub(super) fn with_pool(pool: WorkerPool,wake_authority:GuestRelayWakeAuthority) -> Self {
        Self {
            wake_authority,
            slots: std::iter::repeat_with(|| Mutex::new(GuestRelayMountedSlot::Empty)).take(GUEST_RELAY_MOUNTED_SLOTS).collect(),
            next_generation: std::sync::atomic::AtomicU64::new(1),
            pool,
            reaper_cursor: std::sync::atomic::AtomicUsize::new(0),
            reaper_epoch: std::sync::atomic::AtomicU64::new(0),
            reaper_active: AtomicBool::new(false),
            reaper_failed: AtomicBool::new(false),
            reaper_timer_pending: AtomicBool::new(false),
            #[cfg(test)]
            reaper_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            reaper_timers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            reaper_wake_waits: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn checkout_original_wake(&self,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64)->Option<GuestRelayWakeTurnGrant>{
        let mut context=match self.wake_authority.receiver.checkout(index,generation){Ok(context)=>context?,Err(error)=>{wake.refusal=Some(error.kind);return None}};let policy=self.wake_authority.policy;
        context.grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:context.grant.maximum_items.min(policy.maximum_items),maximum_copy_bytes:context.grant.maximum_copy_bytes.min(policy.maximum_copy_bytes),maximum_capacity_bytes:context.grant.maximum_capacity_bytes.min(policy.maximum_capacity_bytes),maximum_release_bytes:context.grant.maximum_release_bytes.min(policy.maximum_release_bytes),maximum_depth:context.grant.maximum_depth.min(policy.maximum_depth)};Some(context)
    }

    fn collect_original_wake_receipt(&self,wake:&mut GuestRelayWakeCustody)->bool{
        let Some(receipt)=wake.receipt.as_ref()else{return true};
        match self.wake_authority.receiver.collect(receipt){Ok(true)=>{wake.receipt=None;true},Ok(false)=>false,Err(error)=>{wake.refusal=Some(error.kind);false}}
    }

    fn admit_original_session_wake<J:semio_framework_job::InteractiveJob+'static>(&self,owner:&semio_framework_job_worker::WorkerJobSession<J>,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64,waker:&std::task::Waker)->bool{
        if !self.collect_original_wake_receipt(wake){return false}
        if wake.registered{return true}
        let Some(context)=self.checkout_original_wake(wake,index,generation)else{return false};let grant=context.grant;let epoch=context.epoch;
        if wake.original.is_some(){
            match owner.register_retained_wake(&mut wake.original,grant){
                Ok(Some(progress))=>{assert!(progress.fits(grant),"original mounted wake registration exceeded supplied authority");wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});wake.registered=true;waker.wake_by_ref();},
                Ok(None)|Err(_)=>{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});}
            }
            self.collect_original_wake_receipt(wake);return false;
        }
        match (self.wake_authority.issuer)(waker,grant){
            Ok(Some((original,progress)))=>{assert!(progress.fits(grant),"original mounted wake issuer exceeded supplied authority");*wake.original=Some(original);wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});wake.refusal=None;wake.original.as_ref().unwrap().wake_by_ref();},
            Ok(None)=>{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});},
            Err(error)=>{wake.refusal=Some(error.kind);let progress=error.retained_progress();wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});}
        }
        self.collect_original_wake_receipt(wake);false
    }

    fn close_original_wake(&self,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64)->GuestRelayMountedReap{
        if !self.collect_original_wake_receipt(wake){return GuestRelayMountedReap::Blocked{wake_registered:false}}
        if wake.original.is_none(){return GuestRelayMountedReap::Idle};let Some(context)=self.checkout_original_wake(wake,index,generation)else{return GuestRelayMountedReap::Blocked{wake_registered:false}};let grant=context.grant;let epoch=context.epoch;let original=wake.original.as_mut().unwrap();
        let terminal=original.terminal_is_empty();let demand=if terminal{semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<Box<dyn semio_framework_job_worker::RetainedWorkerWake>>>(),release_bytes:original.frame_release_bytes(),depth:1,..Default::default()}}else{match original.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>{wake.refusal=Some(error.kind);wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:error.retained_progress()});self.collect_original_wake_receipt(wake);return GuestRelayMountedReap::Blocked{wake_registered:false}}}};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});self.collect_original_wake_receipt(wake);return GuestRelayMountedReap::Blocked{wake_registered:false}}
        let progress=if terminal{drop(wake.original.take());semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}else{match original.close_step(grant){Ok(step)=>step.progress(),Err(error)=>{wake.refusal=Some(error.kind);error.retained_progress()}}};
        assert!(progress.fits(grant),"original mounted wake close exceeded supplied authority");wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});let collected=self.collect_original_wake_receipt(wake);if !collected||progress==Default::default(){GuestRelayMountedReap::Blocked{wake_registered:false}}else{GuestRelayMountedReap::Progress}
    }

    pub(super) fn reserve(&self) -> Option<(usize, u64)> {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        for (index, slot) in self.slots.iter().enumerate() {
            let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
                return None;
            }
            if matches!(*slot, GuestRelayMountedSlot::Empty) {
                let generation = self
                    .next_generation
                    .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| match generation {
                        0 => None,
                        u64::MAX => Some(0),
                        generation => Some(generation + 1),
                    })
                    .ok()?;
                *slot = GuestRelayMountedSlot::Reserved { generation, output: GuestRelayMountedOutput::new() };
                return Some((index, generation));
            }
        }
        None
    }

    fn mount(&self, index: usize, generation: u64, mut owner: GuestRelayMountedOwner) {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let reserved = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
        let GuestRelayMountedSlot::Reserved { generation: reserved_generation, output } = reserved else {
            debug_assert!(false, "mounted relay requires its exact reserved output owner");
            return;
        };
        if reserved_generation != generation {
            debug_assert!(false, "mounted relay generation must match its reserved output owner");
            *slot = GuestRelayMountedSlot::Reserved { generation: reserved_generation, output };
            return;
        }
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire){
            if let GuestRelayMountedOwner::Rejected(rejected)=&mut owner{Self::close_rejected(rejected);}
        }
        *slot =
            GuestRelayMountedSlot::Mounted(GuestRelayMountedSession { wake:GuestRelayWakeCustody::new(), generation, owner, checked_out: None, lifecycle_probe_checked_out: None, outcome_stage: GuestRelayOutcomeStage::Idle, outcome_kind: GuestRelayOutcomeKind::Continues, outcome_page: 0, output, terminal: None, lifecycle: GuestRelayMountedLifecycle::Running });
    }

    pub(super) fn detach(self: &Arc<Self>, index: usize, generation: u64) {
        let detached = {
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { return };
            if session.generation != generation {
                return;
            }
            if let Some(owner) = session.checked_out.take() {
                owner.begin_close();
            } else if let Some(owner) = session.lifecycle_probe_checked_out.take() {
                owner.begin_close();
            } else {
                match &session.owner {
                    GuestRelayMountedOwner::Session(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::LifecycleProbe(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::Rejected(_) | GuestRelayMountedOwner::Empty => {}
                }
            }
            session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
            true
        };
        if detached {
            self.reaper_epoch.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.ensure_reaper();
        }
    }

    fn ensure_reaper(self: &Arc<Self>) {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            self.fail_detached();
            return;
        }
        if self.reaper_active.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let registry = Arc::clone(self);
        let recovery = Arc::clone(self);
        GuestRelayPoolFuture::spawn_recoverable(self.pool.clone(), Lane::Maintenance, GuestRelayMountedReaper { registry }, move |failure| {
            recovery.reaper_active.store(false, std::sync::atomic::Ordering::Release);
            match failure {
                GuestRelayPoolFailure::FuturePanicked => recovery.ensure_reaper(),
                GuestRelayPoolFailure::Admission(_) => {
                    recovery.reaper_failed.store(true, std::sync::atomic::Ordering::Release);
                    recovery.fail_detached();
                }
            }
        });
    }

    fn fail_detached(&self) {
        self.reaper_failed.store(true,std::sync::atomic::Ordering::Release);
    }

    fn close_rejected(owner:&mut GuestRelayRefusedSource<GuestColdRelayJob>){
        owner.begin_close();
    }

    fn reap_one(&self, waker: &std::task::Waker) -> GuestRelayMountedReap {
        let cursor = self.reaper_cursor.load(std::sync::atomic::Ordering::Acquire) % GUEST_RELAY_MOUNTED_SLOTS;
        for offset in 0..GUEST_RELAY_MOUNTED_SLOTS {
            let index = (cursor + offset) % GUEST_RELAY_MOUNTED_SLOTS;
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { continue };
            if !matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
                continue;
            }
            self.reaper_cursor.store((index + 1) % GUEST_RELAY_MOUNTED_SLOTS, std::sync::atomic::Ordering::Release);
            match self.close_original_wake(&mut session.wake,index,session.generation){GuestRelayMountedReap::Idle=>{},step=>return step}
            return match self.pump_close(session,index,Some(waker)) {
                Ok(GuestRelayMountedClose::Complete) => {
                    *slot = GuestRelayMountedSlot::Empty;
                    GuestRelayMountedReap::Progress
                }
                Ok(GuestRelayMountedClose::Pending) => GuestRelayMountedReap::Progress,
                Ok(GuestRelayMountedClose::Blocked { wake_registered }) => GuestRelayMountedReap::Blocked { wake_registered },
                Err(failure)=>{session.terminal=Some(GuestRelayMountedTerminal::HostFault(PluginHostError::OriginalTurn(failure)));GuestRelayMountedReap::Blocked{wake_registered:session.wake.registered}},
            };
        }
        GuestRelayMountedReap::Idle
    }

    fn schedule_reaper_timer(self: &Arc<Self>, waker: &std::task::Waker) {
        if self.reaper_timer_pending.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        #[cfg(test)]
        self.reaper_timers.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let registry = Arc::clone(self);
        let wake = waker.clone();
        self.pool.submit_at(
            self.pool.now_ms().saturating_add(GUEST_RELAY_REAPER_FALLBACK_MS),
            Lane::Maintenance,
            Box::new(move || {
                registry.reaper_timer_pending.store(false, std::sync::atomic::Ordering::Release);
                wake.wake();
            }),
        );
    }

    fn park_blocked_reaper(self: &Arc<Self>, waker: &std::task::Waker, wake_registered: bool) {
        if wake_registered {
            #[cfg(test)]
            self.reaper_wake_waits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            self.schedule_reaper_timer(waker);
        }
    }

    fn has_detached(&self) -> bool {
        self.slots.iter().any(|slot| {
            let slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap))
        })
    }

    fn pump_close(&self,session:&mut GuestRelayMountedSession,index:usize,_waker:Option<&std::task::Waker>)->Result<GuestRelayMountedClose,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
        if !self.collect_original_wake_receipt(&mut session.wake){return Ok(GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered})}
        let result=self.wake_authority.receiver.dispatch_cleanup_step(index,session.generation,|cx|{
            let grant=cx.retained_grant();let bytes=if session.checked_out.is_some(){size_of::<Option<semio_framework_job_worker::WorkerJobOutcome<GuestColdRelayJob>>>()}else if session.lifecycle_probe_checked_out.is_some(){size_of::<Option<semio_framework_job_worker::WorkerJobOutcome<GuestRelayLifecycleProbeJob>>>()}else{size_of::<GuestRelayMountedOwner>()};
            let terminal=match &session.owner{GuestRelayMountedOwner::Session(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::LifecycleProbe(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::Rejected(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::Empty=>true};
            if session.checked_out.is_some()||session.lifecycle_probe_checked_out.is_some()||terminal{
                if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(GuestRelayMountedClose::Pending)}
                cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
                if let Some(owner)=session.checked_out.take(){owner.begin_close();session.outcome_stage=GuestRelayOutcomeStage::Idle;return Ok(GuestRelayMountedClose::Pending)}
                if let Some(owner)=session.lifecycle_probe_checked_out.take(){owner.begin_close();session.outcome_stage=GuestRelayOutcomeStage::Idle;return Ok(GuestRelayMountedClose::Pending)}
                session.owner=GuestRelayMountedOwner::Empty;return Ok(GuestRelayMountedClose::Complete)
            }
            let step=match &mut session.owner{
                GuestRelayMountedOwner::Session(owner)=>owner.close_step(cx)?,
                GuestRelayMountedOwner::LifecycleProbe(owner)=>owner.close_step(cx)?,
                GuestRelayMountedOwner::Rejected(owner)=>match owner.close_step(cx)?{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress},semio_framework_job::InteractiveJobCloseStep::Blocked=>semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked},
                GuestRelayMountedOwner::Empty=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original mounted close lost its live owner").into()),
            };
            Ok(match step{semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{..}|semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{..}=>GuestRelayMountedClose::Pending,semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered}})
        });
        if result.as_ref().is_ok_and(|value|value.is_none()){return Ok(GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered})}
        let returned=self.wake_authority.receiver.finish_turn(index,session.generation);
        match(result,returned){(Ok(Some(step)),Ok(_))=>Ok(step),(Err(mut failure),Err(receipt))=>{failure.context_refusal=Some(receipt);Err(failure)},(Err(failure),Ok(_))=>Err(failure),(Ok(_),Err(receipt))=>Err(receipt.into()),(Ok(None),Ok(_))=>unreachable!()}
    }

    pub(super) fn pump(self: &Arc<Self>, index: usize, generation: u64, waker: &std::task::Waker) -> std::task::Poll<Result<Vec<u8>, PluginHostError>> {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Mounted(session) = &mut *slot else {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay slot is not mounted".into())));
        };
        if session.generation != generation {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay generation is stale".into())));
        }
        if matches!(session.lifecycle,GuestRelayMountedLifecycle::Running){
            let admitted=match &session.owner{GuestRelayMountedOwner::Session(owner)=>self.admit_original_session_wake(owner,&mut session.wake,index,generation,waker),GuestRelayMountedOwner::LifecycleProbe(owner)=>self.admit_original_session_wake(owner,&mut session.wake,index,generation,waker),GuestRelayMountedOwner::Rejected(_)|GuestRelayMountedOwner::Empty=>true};
            if !admitted{return std::task::Poll::Pending}
        }else if !self.collect_original_wake_receipt(&mut session.wake){return std::task::Poll::Pending}
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay ownership is detached".into())));
        }
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DrainingForCaller) {
            match self.pump_close(session,index,Some(waker)) {
                Ok(GuestRelayMountedClose::Complete) => {
                    let mounted = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
                    let GuestRelayMountedSlot::Mounted(mut session) = mounted else { unreachable!("mounted relay slot remains mounted through close") };
                    let result = match session.terminal.take() {
                        Some(GuestRelayMountedTerminal::Complete) => Ok(session.output.into_vec()),
                        Some(GuestRelayMountedTerminal::Cancelled) => Err(PluginHostError::Plugin("plugin cold relay cancelled".into())),
                        Some(GuestRelayMountedTerminal::Fault) => Err(PluginHostError::Plugin(String::from_utf8_lossy(session.output.bytes()).into_owned())),
                        Some(GuestRelayMountedTerminal::HostFault(error)) => Err(error),
                        None => Err(PluginHostError::Plugin("plugin cold relay closed".into())),
                    };
                    return std::task::Poll::Ready(result);
                }
                Ok(GuestRelayMountedClose::Pending) => waker.wake_by_ref(),
                Ok(GuestRelayMountedClose::Blocked { wake_registered: false }) => self.schedule_reaper_timer(waker),
                Ok(GuestRelayMountedClose::Blocked { wake_registered: true }) => {},
                Err(failure)=>return std::task::Poll::Ready(Err(PluginHostError::OriginalTurn(failure)))
            }
            return std::task::Poll::Pending;
        }
        if session.checked_out.is_some() || session.lifecycle_probe_checked_out.is_some() {
            return self.pump_outcome(session, index, generation, waker);
        }
        let Some(context) = self.checkout_original_wake(&mut session.wake, index, generation) else {
            return std::task::Poll::Pending;
        };
        let (grant, epoch) = (context.grant, context.epoch);
        let stepped = match &session.owner {
            GuestRelayMountedOwner::LifecycleProbe(owner) => match step_guest_relay_owner(owner, grant) {
                GuestRelayStep::Checked(checked, receipt) => {
                    session.lifecycle_probe_checked_out = Some(checked);
                    receipt
                }
                GuestRelayStep::Idle(receipt) => receipt,
                GuestRelayStep::Unavailable(detail) => return std::task::Poll::Ready(Err(PluginHostError::Plugin(detail.into()))),
            },
            GuestRelayMountedOwner::Session(owner) => match step_guest_relay_owner(owner, grant) {
                GuestRelayStep::Checked(checked, receipt) => {
                    session.checked_out = Some(checked);
                    receipt
                }
                GuestRelayStep::Idle(receipt) => receipt,
                GuestRelayStep::Unavailable(detail) => return std::task::Poll::Ready(Err(PluginHostError::Plugin(detail.into()))),
            },
            GuestRelayMountedOwner::Rejected(_) | GuestRelayMountedOwner::Empty => {
                session.wake.receipt = Some(GuestRelayWakeReceipt { epoch, slot: index, generation, grant, progress: Default::default() });
                self.collect_original_wake_receipt(&mut session.wake);
                if matches!(session.owner, GuestRelayMountedOwner::Rejected(_)) {
                    let _ = self.pump_close(session, index, Some(waker));
                }
                return std::task::Poll::Pending;
            }
        };
        let progress = stepped.map_or(Default::default(), |(_, progress)| progress);
        if !progress.fits(grant) {
            session.wake.refusal = Some(semio_framework_value::ValueRefusalKind::InvariantViolated);
        }
        session.wake.receipt = Some(GuestRelayWakeReceipt { epoch, slot: index, generation, grant, progress });
        self.collect_original_wake_receipt(&mut session.wake);
        if session.checked_out.is_some() || session.lifecycle_probe_checked_out.is_some() {
            session.outcome_stage = GuestRelayOutcomeStage::Reading;
            session.outcome_kind = GuestRelayOutcomeKind::Continues;
            session.outcome_page = 0;
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }

    /// 📖️ One paid turn of the checked-out outcome: read its pages once, acknowledge the descriptor, then resume or drain the terminal.
    fn pump_outcome(&self, session: &mut GuestRelayMountedSession, index: usize, generation: u64, waker: &std::task::Waker) -> std::task::Poll<Result<Vec<u8>, PluginHostError>> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        let Some(context) = self.checkout_original_wake(&mut session.wake, index, generation) else {
            return std::task::Poll::Pending;
        };
        let grant = context.grant;
        let mut progress = RetainedCloneProgress::default();
        match session.outcome_stage {
            GuestRelayOutcomeStage::Idle | GuestRelayOutcomeStage::Reading => {
                let read = match (session.checked_out.as_ref(), session.lifecycle_probe_checked_out.as_ref()) {
                    (Some(checked), _) => read_guest_relay_outcome_page(checked, session.outcome_page),
                    (None, Some(checked)) => read_guest_relay_outcome_page(checked, session.outcome_page),
                    (None, None) => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted relay lost its checked-out outcome")),
                };
                match read {
                    Err(error) => {
                        session.wake.refusal = Some(error.kind);
                        progress = error.retained_progress();
                    }
                    Ok((Some(page), kind)) => {
                        let bytes = page.len() + 2 * size_of::<usize>();
                        if grant.maximum_items > 0 && grant.maximum_copy_bytes >= bytes && grant.maximum_depth > 0 && session.output.length.checked_add(page.len()).is_some_and(|length| length <= semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES) {
                            if matches!(kind, GuestRelayOutcomeKind::Complete | GuestRelayOutcomeKind::Fault) {
                                session.output.write_page(page).expect("original output capacity was admitted before page effects");
                            }
                            session.outcome_page += 1;
                            progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
                        }
                    }
                    Ok((None, kind)) => {
                        if grant.maximum_items > 0 && grant.maximum_depth > 0 {
                            session.outcome_kind = kind;
                            session.outcome_stage = GuestRelayOutcomeStage::Acknowledging;
                            progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
                        }
                    }
                }
            }
            GuestRelayOutcomeStage::Acknowledging => {
                let step = match (session.checked_out.as_mut(), session.lifecycle_probe_checked_out.as_mut()) {
                    (Some(checked), _) => checked.acknowledge_outcome(grant),
                    (None, Some(checked)) => checked.acknowledge_outcome(grant),
                    (None, None) => RetainedCloneStep::Progress(Default::default()),
                };
                progress = step.progress();
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.finish_guest_relay_outcome(session);
                }
            }
        }
        if !progress.fits(grant) {
            session.wake.refusal = Some(semio_framework_value::ValueRefusalKind::InvariantViolated);
        }
        session.wake.receipt = Some(GuestRelayWakeReceipt { epoch: context.epoch, slot: index, generation, grant, progress });
        self.collect_original_wake_receipt(&mut session.wake);
        if progress.copied_items > 0 {
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }

    /// 🔚️ A continuing outcome resumes its worker; a terminal one becomes the result the caller drains while the session closes.
    fn finish_guest_relay_outcome(&self, session: &mut GuestRelayMountedSession) {
        match session.outcome_kind {
            GuestRelayOutcomeKind::Continues => {
                if let Some(checked) = session.checked_out.take() {
                    if let Err(checked) = checked.resume() {
                        session.checked_out = Some(checked);
                        return;
                    }
                } else if let Some(checked) = session.lifecycle_probe_checked_out.take() {
                    if let Err(checked) = checked.resume() {
                        session.lifecycle_probe_checked_out = Some(checked);
                        return;
                    }
                }
            }
            terminal => {
                session.terminal = Some(match terminal {
                    GuestRelayOutcomeKind::Complete => GuestRelayMountedTerminal::Complete,
                    GuestRelayOutcomeKind::Cancelled => GuestRelayMountedTerminal::Cancelled,
                    _ => GuestRelayMountedTerminal::Fault,
                });
                session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
                if let Some(checked) = session.checked_out.take() {
                    checked.begin_close();
                } else if let Some(checked) = session.lifecycle_probe_checked_out.take() {
                    checked.begin_close();
                }
            }
        }
        session.outcome_stage = GuestRelayOutcomeStage::Idle;
        session.outcome_page = 0;
    }
}

/// 🪜️ One caller-driven worker step of a mounted relay owner, with its physical receipt taken exactly once.
enum GuestRelayStep<J> {
    Checked(semio_framework_job_worker::WorkerJobOutcome<J>, Option<(semio_framework_value::RetainedCloneGrant, semio_framework_value::RetainedCloneProgress)>),
    Idle(Option<(semio_framework_value::RetainedCloneGrant, semio_framework_value::RetainedCloneProgress)>),
    Unavailable(&'static str),
}

fn step_guest_relay_owner<J: semio_framework_job::InteractiveJob + 'static>(owner: &semio_framework_job_worker::WorkerJobSession<J>, grant: semio_framework_value::RetainedCloneGrant) -> GuestRelayStep<J> {
    let checked = match owner.try_step_on_caller(grant) {
        Ok((ticket, semio_framework_job_worker::WorkerJobPoll::Outcome)) => match owner.take_outcome(ticket) {
            Ok(checked) => checked,
            Err(_) => return GuestRelayStep::Unavailable("plugin cold relay outcome unavailable"),
        },
        Ok((_, semio_framework_job_worker::WorkerJobPoll::Terminal)) => match owner.take_terminal() {
            Ok(checked) => checked,
            Err(_) => return GuestRelayStep::Unavailable("plugin cold relay terminal unavailable"),
        },
        Ok(_) | Err(_) => return GuestRelayStep::Idle(owner.take_retained_step_receipt().ok().flatten()),
    };
    let mut checked = checked;
    let receipt = checked.take_retained_step_receipt();
    GuestRelayStep::Checked(checked, receipt)
}

/// 📄️ Borrows one page of the checked-out outcome's payload and classifies the outcome.
fn read_guest_relay_outcome_page<J: semio_framework_job::InteractiveJob>(checked: &semio_framework_job_worker::WorkerJobOutcome<J>, page: usize) -> Result<(Option<&[u8]>, GuestRelayOutcomeKind), semio_framework_value::ValueError> {
    use semio_framework_job::JobOutcomeView as V;
    let view = checked.outcome()?.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted relay outcome is absent"))?;
    let (payload, kind) = match view {
        V::Yield { .. } => (None, GuestRelayOutcomeKind::Continues),
        V::PreviewReady { payload, .. } => (Some(payload), GuestRelayOutcomeKind::Continues),
        V::CheckpointReady { state, .. } => (Some(state), GuestRelayOutcomeKind::Continues),
        V::Complete { output, .. } => (output, GuestRelayOutcomeKind::Complete),
        V::Cancelled { .. } => (None, GuestRelayOutcomeKind::Cancelled),
        V::Fault { detail, .. } => (Some(detail), GuestRelayOutcomeKind::Fault),
    };
    Ok((payload.and_then(|payload| payload.page(page)), kind))
}

enum GuestRelayMountedReap {
    Idle,
    Progress,
    Blocked { wake_registered: bool },
}

struct GuestRelayMountedReaper {
    registry: Arc<GuestRelayMountedRegistry>,
}

impl std::future::Future for GuestRelayMountedReaper {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        #[cfg(test)]
        {
            self.registry.reaper_polls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        let observed_epoch = self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire);
        match self.registry.reap_one(context.waker()) {
            GuestRelayMountedReap::Progress => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Blocked { wake_registered } => {
                self.registry.park_blocked_reaper(context.waker(), wake_registered);
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Idle => {
                self.registry.reaper_active.store(false, std::sync::atomic::Ordering::Release);
                if self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire) != observed_epoch || self.registry.has_detached() {
                    self.registry.ensure_reaper();
                }
                std::task::Poll::Ready(())
            }
        }
    }
}

struct GuestRelayMountedFuture {
    registry: Arc<GuestRelayMountedRegistry>,
    index: usize,
    generation: u64,
    complete: bool,
}

impl std::future::Future for GuestRelayMountedFuture {
    type Output = Result<Vec<u8>, PluginHostError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.registry.pump(self.index, self.generation, context.waker());
        if poll.is_ready() {
            self.complete = true;
        }
        poll
    }
}

impl Drop for GuestRelayMountedFuture {
    fn drop(&mut self) {
        if !self.complete {
            self.registry.detach(self.index, self.generation);
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleTrace {
    #[serde(rename = "🪪️id")]
    id: String,
    machine: String,
    generation: u64,
    initial: String,
    events: Vec<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleProjection {
    state: String,
    first_reason: Option<String>,
    cancel_admissions: usize,
    release_opportunities: usize,
    caller_output: Option<String>,
}

fn guest_relay_lifecycle_barrier(pool: &WorkerPool) -> Result<(), String> {
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit_at(
        pool.now_ms().saturating_add(1),
        Lane::Maintenance,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    semio_framework_async::block_on(receiver).map_err(|_| "relay lifecycle worker barrier closed".to_string())
}

fn guest_relay_lifecycle_wait(pool: &WorkerPool, mut ready: impl FnMut() -> bool, detail: &'static str) -> Result<(), String> {
    for _ in 0..256 {
        if ready() {
            return Ok(());
        }
        guest_relay_lifecycle_barrier(pool)?;
    }
    Err(detail.to_string())
}

fn guest_relay_lifecycle_probe_session(control: &Arc<GuestRelayLifecycleProbeControl>, generation: u64, terminal_output: Option<Vec<u8>>, remaining: usize, retained:semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_job_worker::WorkerJobSession<GuestRelayLifecycleProbeJob>, String> {
    let job = GuestRelayLifecycleProbeJob { control: Arc::clone(control), remaining, terminal_output, publication: None, closing: false };
    let params = semio_framework_job_worker_authority::BatchJobParams {
        operation: semio_framework_job::OperationId(generation),
        generation: semio_framework_job::Generation(generation),
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job_worker_authority::BatchDriveConfig {
            retained,
            site: "plugin-host.relay-lifecycle",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    admit_guest_relay_session(job, params, retained).map_err(|mut rejected| {
        rejected.begin_close();
        control.wake();
        control.release_one();
        control.release_one();
        "relay lifecycle probe admission refused".to_string()
    })
}

fn exercise_abandoned_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let control = Arc::new(GuestRelayLifecycleProbeControl::new());
        let session = guest_relay_lifecycle_probe_session(&control, generation, None, 2,wake_authority.drive_policy)?;
        registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
        let mut first_reason = None;
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap) {
                    first_reason.get_or_insert_with(|| "detached".to_string());
                }
            } else if event == "blocked" {
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle reaper did not park on the exact close wake")?;
            } else if event == "wake" {
                control.wake();
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle reaper did not park for its first release")?;
            } else if event == "release" {
                let released = control.releases.load(std::sync::atomic::Ordering::Acquire);
                control.release_one();
                guest_relay_lifecycle_wait(&pool, || control.releases.load(std::sync::atomic::Ordering::Acquire) > released, "relay lifecycle reaper did not consume its admitted release")?;
            }
        }
        guest_relay_lifecycle_wait(&pool, || matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty), "relay lifecycle reaper did not reach Empty")?;
        Ok(GuestRelayLifecycleProjection {
            state: "Empty".to_string(),
            first_reason,
            cancel_admissions: control.close_admissions.load(std::sync::atomic::Ordering::Acquire),
            release_opportunities: control.releases.load(std::sync::atomic::Ordering::Acquire),
            caller_output: None,
        })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_live_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let mut first_reason = None;
        let mut releases = 0;
        let mut caller_output = None;
        for event in &trace.events {
            if let Some(output) = event.strip_prefix("terminal:") {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                control.wake();
                let session = guest_relay_lifecycle_probe_session(&control, generation, Some(output.as_bytes().to_vec()), 0,wake_authority.drive_policy)?;
                registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
                for _ in 0..256 {
                    let _ = registry.pump(index, generation, std::task::Waker::noop());
                    let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                        break;
                    }
                }
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle production terminal did not reach caller drain".to_string());
                }
                first_reason.get_or_insert_with(|| "caller".to_string());
            } else if event == "reap-other" {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                let (other_index, other_generation) = registry.reserve().ok_or_else(|| "relay lifecycle competing slot unavailable".to_string())?;
                let session = guest_relay_lifecycle_probe_session(&control, other_generation, None, 2,wake_authority.drive_policy)?;
                registry.mount(other_index, other_generation, GuestRelayMountedOwner::LifecycleProbe(session));
                registry.detach(other_index, other_generation);
                guest_relay_lifecycle_wait(&pool, || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "relay lifecycle competing reaper did not park")?;
                control.wake();
                control.release_one();
                control.release_one();
                guest_relay_lifecycle_wait(&pool, || matches!(&*registry.slots[other_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty), "relay lifecycle competing detached owner was not reclaimed")?;
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle reaper stole the live caller's exact owner".to_string());
                }
            } else if event == "caller-release" {
                for _ in 0..256 {
                    match registry.pump(index, generation, std::task::Waker::noop()) {
                        std::task::Poll::Ready(Ok(bytes)) => {
                            caller_output = Some(String::from_utf8(bytes).map_err(|_| "relay lifecycle caller output is not UTF-8".to_string())?);
                            releases += 1;
                            break;
                        }
                        std::task::Poll::Ready(Err(error)) => return Err(error.to_string()),
                        std::task::Poll::Pending => {}
                    }
                }
                if caller_output.is_none() {
                    return Err("relay lifecycle caller did not receive its exact terminal output".to_string());
                }
            }
        }
        let state = if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) { "Empty" } else { "DrainingForCaller" };
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason, cancel_admissions: 0, release_opportunities: releases, caller_output })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_stale_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
            }
        }
        let state = {
            let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running) {
                "Running"
            } else {
                return Err("relay lifecycle stale generation changed the live owner".to_string());
            }
        };
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
    })();
    let shutdown = pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"));
    result.and_then(|projection| shutdown.map(|()| projection))
}

fn exercise_capacity_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace,wake_authority:GuestRelayWakeAuthority) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = GuestRelayMountedRegistry::with_pool(pool.clone(),wake_authority.clone());
    registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
    let mut reserved = Vec::new();
    let mut state = "Empty";
    for event in &trace.events {
        let Some(requested) = event.strip_prefix("reserve:").and_then(|value| value.parse::<usize>().ok()) else { continue };
        for _ in 0..requested {
            match registry.reserve() {
                Some(owner) => {
                    reserved.push(owner);
                    state = "ReservedFull";
                }
                None => state = "CapacityRefused",
            }
        }
    }
    for (index, _) in reserved {
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    }
    pool.shutdown().map_err(|error| format!("worker shutdown failed: {error:?}"))?;
    Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
}

/// 🧪️ Drives one schema-owned lifecycle trace through the production replay or mounted-relay transition machinery.
#[doc(hidden)]
pub fn exercise_relay_lifecycle_trace(trace_json: &str,wake_authority:GuestRelayWakeAuthority) -> Result<String, String> {
    let trace: GuestRelayLifecycleTrace = serde_json::from_str(trace_json).map_err(|error| format!("invalid relay lifecycle trace: {error}"))?;
    let projection = match (trace.id.as_str(), trace.machine.as_str(), trace.initial.as_str()) {
        ("replay-first-fault-wins", "replay", "CaptureKind") => {
            let actual = shard::exercise_replay_lifecycle_trace(&trace.events)?;
            GuestRelayLifecycleProjection { state: actual.state.to_string(), first_reason: actual.first_reason, cancel_admissions: 0, release_opportunities: actual.release_opportunities, caller_output: None }
        }
        ("relay-abandoned-blocked-wake", "relay", "Running") => exercise_abandoned_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-live-terminal-caller-output", "relay", "Running") => exercise_live_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-stale-generation-refused", "relay", "Running") => exercise_stale_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        ("relay-max-plus-one-refused", "relay", "Empty") => exercise_capacity_relay_lifecycle_trace(&trace,wake_authority.clone())?,
        _ => return Err(format!("unsupported relay lifecycle trace {:?}", trace.id)),
    };
    serde_json::to_string(&projection).map_err(|error| format!("relay lifecycle projection failed: {error}"))
}

/// 🧬️ design-runtime.md §2's post-turn dispatch replacement for the old `Arc<WasmPluginRuntime>`
/// synchronous-call handle. `IoRouter`/`ArtifactInferenceRouter` used to hold a full `WasmPluginRuntime`
/// (component + linker + `Store`) per plugin so a router method could call straight into the guest's
/// own exported functions (`artifact-compose`, `io-run`, `io-sniff`, `artifact-infer`) mid-turn. Those
/// exports do not exist on `world actor` at all — the ONLY guest entry points left are
/// `reactor::poll` and `jobs::{start-job,step-job,cancel-job}` (`📜️.wit`) — so a router now
/// drives a well-known cold job kind (`semio.io-run` / `semio.io-sniff` / `semio.infer`, all three
/// named in `jobs.wit`'s own doc comments) to completion via `GuestRuntime`, strictly AFTER the turn
/// that produced the triggering effect returned (never re-entrantly against that turn's own `Store`).
pub struct PluginInstanceHandle {
    pub actor: RuntimeActorId,
    original_input:semio_framework_actor::RetainedTurnInput,
    pub(super) runtime: Arc<GuestRuntimes>,
    pub(super) instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    relay_registry: Arc<GuestRelayMountedRegistry>,
    next_job_id: std::sync::atomic::AtomicU64,
}

impl PluginInstanceHandle {
    pub async fn new(actor: RuntimeActorId, runtime: Arc<GuestRuntimes>, instance: GuestInstance,original_input:semio_framework_actor::RetainedTurnInput,wake_authority:GuestRelayWakeAuthority) -> Self {
        Self {
            actor,
            original_input,
            runtime,
            instance: Arc::new(Mutex::new(GuestInstanceSlot::Available(instance))),
            instance_gate: Arc::new(semio_framework_async::Semaphore::new(1)),
            relay_registry: Arc::new(GuestRelayMountedRegistry::new(wake_authority)),
            next_job_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// 🧵️ Starts one cold job in the fixed mounted registry. Each host poll admits at most one
    /// relay or close opportunity; the pool never owns an internal run-to-completion chain.
    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>, cancel: semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        if input.len() > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES {
            return Err(PluginHostError::Plugin(format!("{kind} input exceeds the retained operation byte limit")));
        }
        if let Some(detail) = self.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).admission_fault() {
            return Err(PluginHostError::Plugin(format!("{kind} job rejected: {}", String::from_utf8_lossy(&detail))));
        }
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pool = plugin_host_worker_pool();
        let operation = semio_framework_job::OperationId(self.actor.0);
        let generation = semio_framework_job::Generation(job);
        let params = semio_framework_job_worker_authority::BatchJobParams {
            operation,
            generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: semio_framework_job_worker_authority::BatchDriveConfig {
                retained:self.relay_registry.wake_authority.drive_policy,
                site: "plugin-host.cold-relay",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        let relay = GuestColdRelayJob::new(Arc::clone(&self.runtime), Arc::clone(&self.instance), Arc::clone(&self.instance_gate), pool.clone(), cancel, job, (kind.to_string(), input));
        let (index, mounted_generation) = self.relay_registry.reserve().ok_or_else(|| PluginHostError::Plugin(format!("{kind} mounted relay registry is full")))?;
        let owner = match admit_guest_relay_session(relay, params, self.relay_registry.wake_authority.drive_policy) {
            Ok(session) => GuestRelayMountedOwner::Session(session),
            Err(rejected) => GuestRelayMountedOwner::Rejected(rejected),
        };
        self.relay_registry.mount(index, mounted_generation, owner);
        GuestRelayMountedFuture { registry: Arc::clone(&self.relay_registry), index, generation: mounted_generation, complete: false }.await
    }

    /// 🌉️ Executes ONE hop of this plugin's own local io mechanism registry — the absorbed `io-run`
    /// guest export, now `jobs.wit`'s cold job kind `semio.io-run`. `payload` is already JSON
    /// `io_schema::IoPayload` bytes (this method's own caller contract, mirroring the deleted
    /// `WasmPluginRuntime::io_run`'s); `input` bundles what used to be three separate export
    /// parameters into the one JSON `(source, target, IoPayload)` tuple `jobs.wit`'s doc comment
    /// specifies, since a job only carries one opaque `list<u8>`.
    pub async fn io_run(&self, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(&payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        self.run_job_on_worker("semio.io-run", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🔍️ Sniffs this plugin's own `(from, into)` hop — the absorbed `io-sniff` guest export, now
    /// `semio.io-sniff`. Returns the raw `io_schema::Confidence::rank()` byte (`0`=None..`3`=High),
    /// mirroring the deleted `WasmPluginRuntime::io_sniff`'s return shape exactly.
    pub async fn io_sniff(&self, from: &str, into: &str, payload: &[u8]) -> Result<u8, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = semio_framework_pack_json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        let result = self.run_job_on_worker("semio.io-sniff", input, semio_framework_job::root_cancel_token()).await?;
        result.first().copied().ok_or_else(|| PluginHostError::Plugin("semio.io-sniff job returned an empty result".to_string()))
    }

    /// 💡️ Executes one guest inference call — the absorbed `contributor.artifact-infer` export, now
    /// `semio.infer` (`types.wit`'s own doc comment names this job kind explicitly). `request`/the
    /// result are the SAME JSON `io_schema::ArtifactInferenceRequest`/`ArtifactInferenceResult` bytes
    /// the deleted `WasmPluginRuntime::artifact_infer` used — no tuple wrapping needed, since that
    /// call already took exactly one opaque payload. The relay runs under a CHILD of `cancel`: the
    /// caller's cancel stops the in-flight guest job, while the relay's own close never cancels the
    /// caller's token (a dependency chain shares it).
    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.infer", request.to_vec(), cancel.child_now()).await
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest mutation-plan
    /// call — the absorbed `contributor.artifact-mutation-plan` export, now `semio.mutation-plan`.
    /// `request`/the result are the SAME DSL wire-pack bytes `ArtifactMutationRouter::plan` already
    /// builds via `encode_wire_dsl`/decodes via `decode_wire_dsl` (this region's own helpers,
    /// `HostArtifactMutationPlanRequest`/`Result`'s field-for-field guest mirror) — no tuple
    /// wrapping needed, mirroring `infer`'s own doc note above.
    pub async fn mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.mutation-plan", request.to_vec(), semio_framework_job::root_cancel_token()).await
    }

    /// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest versioned
    /// re-encode — the absorbed `migrate-artifact` export, now `semio.migrate`. `from`/`to` are
    /// `io_schema::ArtifactDialect::to_coordinate()` strings; `pack` is the bytes to re-encode.
    /// Bundles all three into one JSON tuple the SAME way `io_run` above bundles its own former
    /// three separate export parameters (`⚛️reactor/💼️jobs/🔀️migrate/🦀️.rs`'s own
    /// `MigrateInput` decodes a struct positionally from this tuple, matching `IoRunInput`'s
    /// established idiom). The caller that resolves WHICH `PluginInstanceHandle` to migrate through
    /// belongs to the pending runtime/db refactor (this method only provides the dispatch, not its
    /// call site — see this packet's report `## lease-requests`).
    pub async fn migrate(&self, from: &str, to: &str, pack: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct MigrateInputWire<'a> {
            from: &'a str,
            to: &'a str,
            pack: Vec<u8>,
        }
        let input = semio_framework_pack_json::to_json_string(&MigrateInputWire { from, to, pack }).into_bytes();
        self.run_job_on_worker("semio.migrate", input, semio_framework_job::root_cancel_token()).await
    }

    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): routes to this plugin's own
    /// `semio.compose` cold job — design-abi.md §2: "`artifact-compose` ... become well-known cold
    /// job kinds ... driven by `start-job` + `step-job`". The guest body is owned by the live
    /// `compose-await` packet (`ComposeStepper`/`ComposeState`, deliberately NOT defined here or
    /// anywhere in this file) — until it registers `"semio.compose"` via `register_bounded_job_kind`, this
    /// fails with the ordinary `job.unknown-kind` fault `step_job` already produces for any
    /// unregistered kind, not a hand-written host refusal. `key_bytes` is the JSON `IoKey`
    /// `IoRouter::compose` already resolved ownership from; `sources_bytes` passes through
    /// byte-for-byte. This wire shape is provisional until `compose-await`'s own guest decode is
    /// defined — coordinate any format change with that packet before altering it here.
    pub async fn compose(&self, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct ComposeInput {
            key: Vec<u8>,
            sources: Vec<u8>,
        }
        let input = semio_framework_pack_json::to_json_string(&ComposeInput { key: key_bytes.to_vec(), sources: sources_bytes.to_vec() }).into_bytes();
        self.run_job_on_worker("semio.compose", input, semio_framework_job::root_cancel_token()).await
    }
}

impl std::fmt::Debug for PluginInstanceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginInstanceHandle").field("actor", &self.actor).finish()
    }
}

#[cfg(test)]
pub(super) fn test_relay_wake_authority()->GuestRelayWakeAuthority{
    GuestRelayWakeAuthority{drive_policy:serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["driveGrant"].clone()).unwrap(),policy:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64},issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:Arc::new(GuestRelayWakeReceiptLedger::new())}
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️guest-cold-relay/🦀️.rs"]
mod guest_cold_relay_tests;
//#endregion 🔀️PostTurnRelay
