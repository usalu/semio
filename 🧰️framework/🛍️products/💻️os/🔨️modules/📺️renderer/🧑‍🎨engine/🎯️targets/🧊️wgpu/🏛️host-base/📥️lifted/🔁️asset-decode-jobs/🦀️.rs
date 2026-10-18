//! 🧵️ The renderer worker pool and its bounded native I/O slots: one `RendererIoHandle` per submitted request, the slot ledger and the pump the winit thread and the kernel workers drive.
//!
//! 🧭️ Lifted out of the asset decode jobs into host-base: the decode jobs are one client of this transport, the kernel and the frame loop are others.
use super::*;

//#region 🧵️RendererWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every renderer subsystem.
#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}

#[cfg(target_arch = "wasm32")]
pub fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 1))
}

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoHandle {
    pub slot: usize,
    pub generation: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoHandle {
    pub fn try_take(&mut self) -> Option<Result<semio_framework_os_services::NativeIoValue, String>> {
        let result = renderer_io_with_node(self.slot, self.generation, |node| {
            let result = node.result.take()?;
            node.result_taken = true;
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            Some(result)
        })
        .flatten();
        self.completed |= result.is_some();
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for RendererIoHandle {
    type Output = Result<semio_framework_os_services::NativeIoValue, String>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return std::task::Poll::Ready(result);
        }
        let registered = renderer_io_with_node(self.slot, self.generation, |node| {
            node.waker = Some(cx.waker().clone());
            if let Some(session) = node.session.as_ref() {
                let _ = session.register_wake(cx.waker());
            }
        })
        .is_some();
        if registered {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            std::task::Poll::Pending
        } else if renderer_io_generation_live(self.slot, self.generation) {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            self.completed = true;
            std::task::Poll::Ready(Err("native I/O mounted session generation is stale".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for RendererIoHandle {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(slot) = RENDERER_IO_SLOTS.get(self.slot) {
            if slot.generation.load(Ordering::Acquire) == self.generation {
                slot.cancel_requested.store(true, Ordering::Release);
            }
        }
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS + 1;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_VACANT: u8 = 0;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_LIVE: u8 = 1;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_CHECKED_OUT: u8 = 2;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_EXHAUSTED: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoSlot {
    generation: AtomicU64,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub state: std::sync::atomic::AtomicU8,
    #[cfg(not(any(test, feature = "testkit")))]
    state: std::sync::atomic::AtomicU8,
    node: std::sync::atomic::AtomicPtr<RendererIoNode>,
    cancel_requested: AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoSlot {
    const fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: std::sync::atomic::AtomicU8::new(RENDERER_IO_VACANT), node: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()), cancel_requested: AtomicBool::new(false) }
    }
}

/// 🪙️ The declared per-turn authority of one mounted native I/O request: admission, one worker step's receipts and their paid acknowledgement.
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_TURN_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 16, maximum_copy_bytes: 4 * 1024 * 1024, maximum_capacity_bytes: 4 * 1024 * 1024, maximum_release_bytes: 16 * 1024 * 1024, maximum_depth: 64 };

/// 🪙️ One page, path or ledger release of a completed or detached result, with the parent depths its payload ladder needs.
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_RESULT_CLOSE_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: 2 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 3 };

/// 🪪️ The original authority one mounted native I/O request runs under: its own identity, root cancellation and paid caller turns, because this transport is pumped without any frame owner.
#[cfg(not(target_arch = "wasm32"))]
struct RendererIoCaller {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    cancel: Option<semio_framework_job::CancelToken>,
    cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    sequence: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoCaller {
    fn new(generation: u64) -> Self {
        Self { operation: semio_framework_job::allocate_operation_id(), generation: semio_framework_job::Generation(generation), cancel: Some(semio_framework_job::root_cancel_token()), cancel_retirement: None, sequence: 0 }
    }

    fn cancel_now(&self) {
        if let Some(cancel) = self.cancel.as_ref() {
            cancel.cancel_now();
        }
    }

    fn turn_under<R>(&mut self, cancel: semio_framework_job::CancelToken, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
        let Some(now) = semio_framework_job::default_now_us() else { return Ok(None) };
        let Some(budget) = semio_framework_job::StepBudget::from_duration(semio_framework_job::MAINTENANCE_LANE_FUEL, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, RENDERER_IO_TURN_GRANT) else { return Ok(None) };
        let mut progress = semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(self.operation, self.generation, budget, cancel, semio_framework_job::default_now_us, &mut self.sequence, &mut progress);
        run(&mut cx).map(Some)
    }

    fn turn<R>(&mut self, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
        let Some(root) = self.cancel.clone() else { return Ok(None) };
        self.turn_under(root, run)
    }

    fn retire_step(&mut self) -> bool {
        if let Some(root) = self.cancel.take() {
            self.cancel_retirement = Some(semio_framework_async::CancelTokenRetirement::from_token(root));
            return false;
        }
        let Some(mut owner) = self.cancel_retirement.take() else { return true };
        if !owner.terminal_is_empty() {
            let _ = self.turn_under(semio_framework_job::root_cancel_token(), |cx| {
                let grant = cx.retained_grant();
                match owner.close_step(grant) {
                    Ok(step) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(step.progress().copied_items as u64);
                    }
                    Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => {}
                    Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
                }
                Ok(())
            });
        }
        if owner.terminal_is_empty() {
            return true;
        }
        self.cancel_retirement = Some(owner);
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.cancel.is_none() && self.cancel_retirement.is_none()
    }
}

/// 📬️ What one paid receipt turn learned from a checked-out worker outcome.
#[cfg(not(target_arch = "wasm32"))]
enum RendererIoReceipt {
    Yield,
    Terminal(Option<Result<semio_framework_os_services::NativeIoValue, String>>),
    Fault(&'static str),
}

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoNode {
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub job: Option<semio_framework_os_services::NativeIoJob>,
    #[cfg(not(any(test, feature = "testkit")))]
    job: Option<semio_framework_os_services::NativeIoJob>,
    params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    params_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    session: Option<semio_framework_job_worker::WorkerJobSession<semio_framework_os_services::NativeIoJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    result: Option<Result<semio_framework_os_services::NativeIoValue, String>>,
    result_taken: bool,
    detached: bool,
    caller: RendererIoCaller,
    waker: Option<std::task::Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoNode {
    fn new(generation: u64, request: semio_framework_os_services::NativeIoRequest) -> Self {
        Self { job: Some(semio_framework_os_services::NativeIoJob::new(request)), params: None, params_retirement: None, session: None, ticket: None, result: None, result_taken: false, detached: false, caller: RendererIoCaller::new(generation), waker: None }
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn admit(&mut self) -> bool {
        let Self { caller, job, params, session, .. } = self;
        if session.is_some() {
            return true;
        }
        let admitted = caller.turn(|cx| -> Result<bool, semio_framework_job::OriginalTurnFailure> {
            if params.is_none() {
                let Some(now) = cx.now_us() else { return Ok(false) };
                let Some(cancel) = cx.admit_original_cancel_alias()? else { return Ok(false) };
                *params = Some(semio_framework_job_worker_authority::BatchJobParams {
                    operation: cx.operation(),
                    generation: cx.generation(),
                    cancel,
                    now_us: cx.clock_source(),
                    config: semio_framework_job_worker_authority::BatchDriveConfig {
                        retained: cx.retained_grant(),
                        site: "os_renderer_native_io",
                        stage: semio_framework_job::InteractiveStage::InteractiveStep,
                        fuel_per_step: semio_framework_job::INTERACTIVE_LANE_FUEL,
                        step_budget_us: cx.deadline_us().saturating_sub(now).min(semio_framework_job::INTERACTIVE_LANE_WALL_US),
                    },
                });
            }
            if let Some((admitted, _posted)) = semio_framework_job_worker::WorkerJobSession::try_admit_owned(job, params, &mut *cx)? {
                *session = Some(admitted);
            }
            Ok(session.is_some())
        });
        matches!(admitted, Ok(Some(true)))
    }

    fn close_detached_result(&mut self) -> bool {
        if !self.detached {
            return false;
        }
        let Some(result) = self.result.as_mut() else { return false };
        match result {
            Ok(value) if !value.terminal_is_empty() => {
                let _ = value.close_step(RENDERER_IO_RESULT_CLOSE_GRANT);
                return true;
            }
            Ok(_) => {}
            Err(error) if !error.is_empty() => {
                if error.len() > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return false;
                }
                error.clear();
                return true;
            }
            Err(_) => {}
        }
        self.result = None;
        self.result_taken = true;
        true
    }

    fn close_unadmitted_step(&mut self) -> bool {
        use semio_framework_job::InteractiveJob as _;
        let Self { caller, job, params, params_retirement, .. } = self;
        if job.is_none() && params.is_none() && params_retirement.is_none() {
            return true;
        }
        let _ = caller.turn(|cx| -> Result<(), semio_framework_job::OriginalTurnFailure> {
            let grant = cx.retained_grant();
            if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(());
            }
            let paid = semio_framework_job::RetainedCloneProgress { copied_items: 1, ..Default::default() };
            if let Some(owner) = job.as_mut() {
                owner.begin_close();
                if !owner.terminal_is_empty() {
                    owner.close_step(cx)?;
                    return Ok(());
                }
                cx.consume_retained(paid)?;
                cx.consume_fuel(1);
                *job = None;
                return Ok(());
            }
            if let Some(original) = params.take() {
                cx.consume_retained(paid)?;
                cx.consume_fuel(1);
                *params_retirement = Some(semio_framework_async::CancelTokenRetirement::from_token(original.cancel));
                return Ok(());
            }
            if let Some(owner) = params_retirement.as_mut() {
                if !owner.terminal_is_empty() {
                    match owner.return_alias_step(cx.original_cancel_token(), grant) {
                        Ok(step) => {
                            cx.consume_retained(step.progress())?;
                            cx.consume_fuel(step.progress().copied_items as u64);
                        }
                        Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => {}
                        Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
                    }
                    return Ok(());
                }
                cx.consume_retained(paid)?;
                cx.consume_fuel(1);
                *params_retirement = None;
            }
            Ok(())
        });
        self.job.is_none() && self.params.is_none() && self.params_retirement.is_none()
    }

    fn close_session_step(&mut self) {
        use semio_framework_job_worker::{WorkerJobDemandError, WorkerJobPoll};
        let Self { caller, session, .. } = self;
        let Some(owner) = session.as_mut() else { return };
        let retired = caller.turn(|cx| -> Result<bool, semio_framework_job::OriginalTurnFailure> {
            let grant = cx.retained_grant();
            if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(false);
            }
            match owner.take_retained_step_receipt() {
                Ok(Some((_issued, progress))) => {
                    cx.consume_retained(progress)?;
                    return Ok(false);
                }
                Ok(None) => {}
                Err(_) => return Ok(false),
            }
            let _ = owner.begin_close();
            if matches!(owner.poll(), WorkerJobPoll::TerminalEmpty) {
                cx.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(true);
            }
            match owner.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                Ok(true) => match owner.return_original_cancel_alias_step(cx.original_cancel_token(), grant) {
                    Ok(Some(step)) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(step.progress().copied_items as u64);
                    }
                    Ok(None) | Err(WorkerJobDemandError::Contention(_)) => {}
                    Err(WorkerJobDemandError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
                },
                Ok(false) => {
                    owner.close_step(cx)?;
                }
                Err(WorkerJobDemandError::Contention(_)) => {}
                Err(WorkerJobDemandError::Refused(error)) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
            }
            Ok(false)
        });
        if matches!(retired, Ok(Some(true))) {
            self.session = None;
            self.ticket = None;
        }
    }

    fn receive_outcome(&mut self, mut owner: semio_framework_job_worker::WorkerJobOutcome<semio_framework_os_services::NativeIoJob>) {
        use semio_framework_job::{JobOutcomeDescriptor, JobOutcomeKind, RetainedCloneStep};
        let received = self.caller.turn(|cx| -> Result<RendererIoReceipt, semio_framework_job::OriginalTurnFailure> {
            let Some((issued, progress)) = owner.take_retained_step_receipt() else { return Ok(RendererIoReceipt::Fault("native I/O step returned no original receipt")) };
            if !progress.fits(issued) || !progress.fits(cx.retained_grant()) {
                return Ok(RendererIoReceipt::Fault("native I/O receipt exceeds its original receiving authority"));
            }
            cx.consume_retained(progress)?;
            let kind = owner.outcome_descriptor().map(JobOutcomeDescriptor::kind);
            let mut acknowledged = false;
            for _ in 0..4 {
                let step = owner.acknowledge_outcome(cx.retained_grant());
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(step.progress().copied_items as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    acknowledged = true;
                    break;
                }
            }
            if !acknowledged {
                return Ok(RendererIoReceipt::Fault("native I/O outcome acknowledgement did not complete"));
            }
            Ok(match kind {
                None | Some(JobOutcomeKind::Yield) => RendererIoReceipt::Yield,
                Some(_) => RendererIoReceipt::Terminal(owner.job_mut().ok().and_then(semio_framework_os_services::NativeIoJob::take_result)),
            })
        });
        match received {
            Ok(Some(RendererIoReceipt::Yield)) => {
                self.ticket = None;
                if let Err(owner) = owner.resume() {
                    self.result = Some(Err("native I/O yield could not resume exact owner".into()));
                    owner.begin_close();
                }
            }
            Ok(Some(RendererIoReceipt::Terminal(result))) => {
                self.result = result.or_else(|| Some(Err("native I/O terminal owner had no structured result".into())));
                self.ticket = None;
                owner.begin_close();
            }
            Ok(Some(RendererIoReceipt::Fault(reason))) => {
                self.result = Some(Err(reason.into()));
                self.ticket = None;
                owner.begin_close();
            }
            Ok(None) | Err(_) => {
                self.result = Some(Err("native I/O original turn was refused".into()));
                self.ticket = None;
                owner.begin_close();
            }
        }
    }

    fn submit_step(&mut self) {
        use semio_framework_job_worker::WorkerJobSubmitFault;
        let Self { caller, session, ticket, result, .. } = self;
        let Some(session) = session.as_ref() else { return };
        let submitted = caller.turn(|cx| -> Result<Option<Result<semio_framework_job_worker::WorkerJobTicket, WorkerJobSubmitFault>>, semio_framework_job::OriginalTurnFailure> {
            let grant = cx.retained_grant();
            if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(None);
            }
            Ok(Some(session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Io, grant)))
        });
        match submitted {
            Ok(Some(Some(Ok(issued)))) => *ticket = Some(issued),
            Ok(Some(Some(Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended))))) => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.resume();
                }
            }
            Ok(Some(Some(Err(error)))) => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.begin_close();
                } else {
                    let _ = session.begin_close();
                }
                *result = Some(Err(format!("native I/O worker submission failed: {error:?}")));
            }
            _ => {}
        }
    }

    fn pump_session_step(&mut self) {
        use semio_framework_job_worker::WorkerJobPoll;
        let Some(poll) = self.session.as_ref().map(semio_framework_job_worker::WorkerJobSession::poll) else { return };
        match poll {
            WorkerJobPoll::Idle => self.submit_step(),
            WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket.take() else {
                    self.result = Some(Err("native I/O outcome lost its exact ticket".into()));
                    if let Some(session) = self.session.as_ref() {
                        let _ = session.begin_close();
                    }
                    return;
                };
                match self.session.as_ref().map(|session| session.take_outcome(ticket)) {
                    Some(Ok(owner)) => self.receive_outcome(owner),
                    _ => self.result = Some(Err("native I/O outcome handback contention".into())),
                }
            }
            WorkerJobPoll::Terminal => match self.session.as_ref().map(semio_framework_job_worker::WorkerJobSession::take_terminal) {
                Some(Ok(owner)) => self.receive_outcome(owner),
                _ => self.result = Some(Err("native I/O terminal handback contention".into())),
            },
            WorkerJobPoll::Rejected => {
                if let Some(Ok(rejected)) = self.session.as_ref().map(semio_framework_job_worker::WorkerJobSession::take_rejected) {
                    rejected.resume();
                }
            }
            WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => self.close_session_step(),
            WorkerJobPoll::Submitted | WorkerJobPoll::CheckedOut => {}
        }
    }

    fn pump_one(&mut self) {
        if self.close_detached_result() {
            self.wake();
            return;
        }
        let Some(session) = self.session.as_ref() else {
            if self.close_unadmitted_step() && !self.caller.terminal_is_empty() {
                let _ = self.caller.retire_step();
            }
            self.wake();
            return;
        };
        let _ = session.take_wake();
        if self.result.is_some() || self.detached {
            self.close_session_step();
        } else {
            self.pump_session_step();
        }
        self.wake();
    }

    fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.job.is_none() && self.params.is_none() && self.params_retirement.is_none() && self.caller.terminal_is_empty() && self.result.is_none() && (self.detached || self.result_taken)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_SLOTS: [RendererIoSlot; RENDERER_IO_SESSION_SLOTS] = [const { RendererIoSlot::vacant() }; RENDERER_IO_SESSION_SLOTS];
#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_WAKE: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_PUMP_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_io_with_node<R>(index: usize, generation: u64, use_node: impl FnOnce(&mut RendererIoNode) -> R) -> Option<R> {
    let slot = RENDERER_IO_SLOTS.get(index)?;
    if slot.generation.load(Ordering::Acquire) != generation || slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    if slot.generation.load(Ordering::Acquire) != generation {
        slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        return None;
    }
    let pointer = slot.node.load(Ordering::Acquire);
    let result = (!pointer.is_null()).then(|| use_node(unsafe { &mut *pointer }));
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_io_generation_live(index: usize, generation: u64) -> bool {
    RENDERER_IO_SLOTS.get(index).is_some_and(|slot| slot.generation.load(Ordering::Acquire) == generation && matches!(slot.state.load(Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT) && !slot.node.load(Ordering::Acquire).is_null())
}

/// 🔭️ Whether any mounted native I/O session is still live or checked out, so a headless entry point knows when to stop pumping.
#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_io_sessions_pending() -> bool {
    RENDERER_IO_SLOTS.iter().any(|slot| matches!(slot.state.load(Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pump_renderer_io_sessions(maximum_sessions: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let start = RENDERER_IO_PUMP_CURSOR.fetch_add(maximum_sessions, Ordering::AcqRel);
    let mut advanced = 0;
    for offset in 0..RENDERER_IO_SESSION_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let index = start.wrapping_add(offset) % RENDERER_IO_SESSION_SLOTS;
        let slot = &RENDERER_IO_SLOTS[index];
        if slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            slot.state.store(RENDERER_IO_VACANT, Ordering::Release);
            continue;
        }
        let node = unsafe { &mut *pointer };
        let cancel_advanced = slot.cancel_requested.swap(false, Ordering::AcqRel);
        if cancel_advanced {
            node.detached = true;
            node.caller.cancel_now();
            if let Some(session) = node.session.as_ref() {
                let _ = session.begin_close();
            }
            node.wake();
        }
        if !cancel_advanced {
            node.pump_one();
        }
        if node.terminal_is_empty() {
            slot.node.store(std::ptr::null_mut(), Ordering::Release);
            slot.cancel_requested.store(false, Ordering::Release);
            drop(unsafe { Box::from_raw(pointer) });
            slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { RENDERER_IO_EXHAUSTED } else { RENDERER_IO_VACANT }, Ordering::Release);
        } else {
            slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        }
        advanced += 1;
    }
    if RENDERER_IO_SLOTS.iter().any(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE) {
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
    advanced
}

#[cfg(not(target_arch = "wasm32"))]
pub fn submit_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<RendererIoHandle, String> {
    let Some((slot_index, generation)) = RENDERER_IO_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        if slot.state.compare_exchange(RENDERER_IO_VACANT, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let Some(generation) = slot.generation.load(Ordering::Acquire).checked_add(1) else {
            slot.state.store(RENDERER_IO_EXHAUSTED, Ordering::Release);
            return None;
        };
        slot.generation.store(generation, Ordering::Release);
        slot.cancel_requested.store(false, Ordering::Release);
        Some((index, generation))
    }) else {
        return Err("native I/O mounted session registry is exhausted".into());
    };
    let mut node = Box::new(RendererIoNode::new(generation, request));
    if !node.admit() {
        node.result = Some(Err("native I/O retained session registry is full".into()));
    }
    RENDERER_IO_SLOTS[slot_index].node.store(Box::into_raw(node), Ordering::Release);
    RENDERER_IO_SLOTS[slot_index].state.store(RENDERER_IO_LIVE, Ordering::Release);
    RENDERER_IO_WAKE.store(true, Ordering::Release);
    Ok(RendererIoHandle { slot: slot_index, generation, completed: false })
}

/// 🧮️ The bounded number of page and ledger frontiers one completed payload may need.
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_PAYLOAD_CLOSE_STEPS: usize = 4096;

/// ♻️ Releases a completed native I/O payload one page or ledger frontier per step until it is terminal empty; false when the bounded walk left an owner behind.
#[cfg(not(target_arch = "wasm32"))]
pub fn close_renderer_io_payload(payload: &mut semio_framework_job::RetainedJobPayload) -> bool {
    for _ in 0..RENDERER_IO_PAYLOAD_CLOSE_STEPS {
        if payload.terminal_is_empty() {
            return true;
        }
        let _ = payload.close_step(RENDERER_IO_RESULT_CLOSE_GRANT);
    }
    payload.terminal_is_empty()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<semio_framework_os_services::NativeIoValue, String> {
    submit_renderer_io(request)?.await
}

//#endregion 🧵️RendererWorkerPool
