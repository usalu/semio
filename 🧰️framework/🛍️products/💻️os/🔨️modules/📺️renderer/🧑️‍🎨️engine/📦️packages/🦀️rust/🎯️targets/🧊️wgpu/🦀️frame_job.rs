//! 🧵️ P3/P5 mounted frame coordinator. [`FrameBuildJob`] incrementally derives deadline candidates,
//! and [`FrameBuildHandle::poll_runtime_and_resubmit`] couples that protocol to the complete
//! `AppRuntime::frame` transaction. Native submits one generation at a time to the process worker
//! pool, publishes only matching completions, and never waits while polling. Frame construction owns
//! shell traversal, layout/tessellation, engine-scene directives, and prepared-packet creation;
//! `AppPresenter` retains platform/GPU presentation authority.
//!
//! The scalar wheel deadline candidate remains non-authoritative: `AppRuntime::frame` revalidates it
//! against live state before applying it. The wasm implementation drives the job only after the
//! renderer has booted inside its dedicated Worker isolate; calls from a browser UI isolate fail
//! closed and never execute the transaction inline.

use semio_framework_job::{
    root_cancel_token, BatchDriveConfig, BatchJobParams, BatchJobSession, CancelToken, CommitCandidate, InteractiveJob, StepContext, StepOutcome, WorkerJobSessionAdmissionRejected, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_MS,
};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;

//#region 📥️FrameBuildInputs
/// 📥️ The fixed scalar `Send`-safe slice of `AppRuntime` this job needs.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FrameBuildInputs {
    pub wheel_zoom_deadline_ms: f64,
    pub now_ms: f64,
}
//#endregion 📥️FrameBuildInputs

//#region 📤️FrameDirectives
/// 📤️ The worker's fixed scalar candidate, revalidated against live state before use.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FrameDirectives {
    pub wheel_zoom_deadline_cleared: bool,
}
//#endregion 📤️FrameDirectives

//#region 🧩️FrameBuildJob
/// 🧩️ The deadline-sweep computation as a real [`InteractiveJob`] — one call to `step` always reaches
/// [`StepOutcome::Complete`] (the work is O(open deadlines), never large enough to need more than one
/// step; `cx.consume_fuel` is still called so a future, larger stage added to this same job under
/// Phase 5 inherits real yield-on-overrun behaviour instead of it being bolted on later).
pub(crate) struct FrameBuildJob {
    wheel_zoom_deadline_cleared: bool,
    complete: Option<FrameDirectives>,
    closing: bool,
}

impl FrameBuildJob {
    pub(crate) fn new(inputs: FrameBuildInputs) -> Self {
        Self { wheel_zoom_deadline_cleared: inputs.wheel_zoom_deadline_ms > 0.0 && inputs.now_ms >= inputs.wheel_zoom_deadline_ms, complete: None, closing: false }
    }

    pub(crate) fn take_directives(&mut self) -> Option<FrameDirectives> {
        self.complete.take()
    }

    fn close_step(&mut self) -> bool {
        if let Some(directives) = self.complete.as_mut() {
            if !directives.close_step() {
                return false;
            }
            self.complete = None;
            return false;
        }
        true
    }
}

impl FrameDirectives {
    fn close_step(&mut self) -> bool {
        true
    }
}

impl InteractiveJob for FrameBuildJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        self.complete = Some(FrameDirectives { wheel_zoom_deadline_cleared: self.wheel_zoom_deadline_cleared });
        cx.consume_fuel(1);
        StepOutcome::Complete(CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 && self.complete.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if FrameBuildJob::close_step(self) {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.complete.is_none()
    }
}
//#endregion 🧩️FrameBuildJob

//#region ⏱️Clock
/// ⏱️ `BatchJobParams::now_ms` is a plain `fn() -> u64` pointer (not a closure) — `crate::app_now_ms`
/// returns `f64` wall-clock milliseconds, so this wraps it to the integer form the job protocol wants
/// without changing `app_now_ms`'s own signature (used elsewhere for sub-millisecond arithmetic).
fn now_ms_u64() -> u64 {
    crate::app_now_ms() as u64
}

fn batch_params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams {
        operation,
        generation,
        cancel,
        config: BatchDriveConfig { site: "os_renderer_frame_build", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_ms: INTERACTIVE_LANE_WALL_MS },
        now_ms: now_ms_u64,
    }
}
//#endregion ⏱️Clock

//#region 📮️FrameBuildHandle
/// 📮️ The non-blocking poll/resubmit contract item 5 of the packet brief asks for: never waits on the
/// worker. `poll_and_resubmit` always returns immediately — either this tick's freshly-completed
/// directives, or (if the in-flight job hasn't finished, or none was in flight yet) the last
/// successfully computed ones. One job is in flight at a time; a still-running job is left alone (not
/// cancelled) and re-checked next call rather than submitting a second overlapping one.
pub(crate) struct FrameBuildHandle {
    session: Option<semio_framework_job::WorkerJobSession<ActiveFrameBuild>>,
    rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>,
    ticket: Option<semio_framework_job::WorkerJobTicket>,
    #[cfg(not(target_arch = "wasm32"))]
    completion_waker: Option<Arc<dyn Fn() + Send + Sync>>,
    latest_requested_generation: Generation,
    last_submitted_generation: Option<Generation>,
    cancel: CancelToken,
    closing: bool,
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameCompletionWake(Arc<dyn Fn() + Send + Sync>);

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for FrameCompletionWake {
    fn wake(self: Arc<Self>) {
        (self.0)();
    }
}

enum ActiveFramePhase {
    Deadlines(BatchJobSession<FrameBuildJob>),
    DeadlineAdmissionRejected(WorkerJobSessionAdmissionRejected<FrameBuildJob>),
    ApplyPending(FrameDirectives),
    Build(crate::AppFrameTransaction),
    Prepare(crate::AppFramePreparation),
    Terminal,
}

struct ActiveFrameBuild {
    runtime: crate::RuntimeMailbox,
    handle: crate::AppHandle,
    operation: OperationId,
    generation: Generation,
    dpr: f32,
    cancel: CancelToken,
    preview_sequence: u64,
    phase: ActiveFramePhase,
    completed: Option<crate::AppFramePresentation>,
    closing: bool,
}

enum ActiveFrameStep {
    Pending,
    Complete(Option<crate::AppFramePresentation>),
}

fn retire_active_phase(phase: &mut ActiveFramePhase) -> bool {
    match phase {
        ActiveFramePhase::Deadlines(session) => {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return false;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            session.terminal_is_empty()
        }
        ActiveFramePhase::DeadlineAdmissionRejected(rejected) => {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            rejected.terminal_is_empty()
        }
        ActiveFramePhase::ApplyPending(directives) => directives.close_step(),
        ActiveFramePhase::Build(transaction) => transaction.close_step(),
        ActiveFramePhase::Prepare(preparation) => preparation.close_step() && preparation.terminal_is_empty(),
        ActiveFramePhase::Terminal => true,
    }
}

fn frame_phase_overran(before: u64, site: &'static str, operation: OperationId, generation: Generation) -> bool {
    semio_framework_trace::Watchdog::violation_count() > before && semio_framework_trace::Watchdog::violations().into_iter().rev().any(|violation| violation.site == site && violation.operation == operation && violation.generation == generation)
}

impl ActiveFrameBuild {
    fn new(runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32, cancel: CancelToken) -> Self {
        let handle = runtime.downgrade();
        let phase = match BatchJobSession::try_new(FrameBuildJob::new(inputs), batch_params(operation, generation, cancel.clone())) {
            Ok(session) => ActiveFramePhase::Deadlines(session),
            Err(mut rejected) => {
                rejected.begin_close();
                ActiveFramePhase::DeadlineAdmissionRejected(rejected)
            }
        };
        Self { runtime, handle, operation, generation, dpr, cancel, preview_sequence: 0, phase, completed: None, closing: false }
    }

    fn cancel(&self) {
        self.cancel.cancel_now();
    }

    fn retire_cancelled_phase(&mut self) -> bool {
        retire_active_phase(&mut self.phase)
    }

    fn quarantine_overrun(&self, site: &'static str) {
        self.runtime.record_frame_fault(site);
        self.cancel.cancel_now();
    }

    fn advance(&mut self) -> ActiveFrameStep {
        if self.cancel.is_cancelled_now() {
            if self.retire_cancelled_phase() {
                self.phase = ActiveFramePhase::Terminal;
                return ActiveFrameStep::Complete(None);
            }
            return ActiveFrameStep::Pending;
        }
        match &mut self.phase {
            ActiveFramePhase::Deadlines(session) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let poll = session.step();
                if frame_phase_overran(violations, "os_renderer.frame.deadlines", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.frame.deadlines overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                if !matches!(poll, Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) || !session.checkout_outcome() {
                    return ActiveFrameStep::Pending;
                }
                match session.checked_out_outcome() {
                    Some(StepOutcome::Complete(_)) => {
                        let directives = session.checked_out_job_mut().and_then(FrameBuildJob::take_directives).unwrap_or_default();
                        session.begin_close();
                        self.phase = ActiveFramePhase::ApplyPending(directives);
                        ActiveFrameStep::Pending
                    }
                    Some(StepOutcome::Yield) => {
                        let _ = session.resume();
                        ActiveFrameStep::Pending
                    }
                    Some(StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) | StepOutcome::Cancelled | StepOutcome::Fault(_)) => {
                        session.begin_close();
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                    None => ActiveFrameStep::Pending,
                }
            }
            ActiveFramePhase::DeadlineAdmissionRejected(rejected) => {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if rejected.terminal_is_empty() {
                    self.phase = ActiveFramePhase::Terminal;
                    ActiveFrameStep::Complete(None)
                } else {
                    ActiveFrameStep::Pending
                }
            }
            ActiveFramePhase::ApplyPending(directives) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let applied = {
                    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.apply_pending", self.operation, self.generation, InteractiveStage::InteractiveStep);
                    self.runtime.apply_pending_step()
                };
                if frame_phase_overran(violations, "os_renderer.frame.apply_pending", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.frame.apply_pending overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                if applied {
                    return ActiveFrameStep::Pending;
                }
                self.phase = ActiveFramePhase::Build(crate::AppFrameTransaction::new(std::mem::take(directives), self.generation, self.dpr));
                ActiveFrameStep::Pending
            }
            ActiveFramePhase::Build(transaction) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let transaction_step = {
                    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.transaction", self.operation, self.generation, InteractiveStage::InteractiveStep);
                    let now = now_ms_u64();
                    let mut context = StepContext::new(self.operation, self.generation, semio_framework_job::StepBudget::new(1, now.saturating_add(INTERACTIVE_LANE_WALL_MS)), self.cancel.clone(), now_ms_u64, &mut self.preview_sequence);
                    transaction.step(&self.runtime, &self.handle, &mut context)
                };
                if frame_phase_overran(violations, "os_renderer.frame.transaction", self.operation, self.generation) {
                    if let crate::AppFrameTransactionStep::Complete(frame) = transaction_step {
                        let preparation = frame.into_preparation();
                        self.phase = ActiveFramePhase::Prepare(preparation);
                    }
                    self.quarantine_overrun("os_renderer.frame.transaction overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match transaction_step {
                    crate::AppFrameTransactionStep::Complete(frame) => {
                        self.phase = ActiveFramePhase::Prepare(frame.into_preparation());
                        ActiveFrameStep::Pending
                    }
                    crate::AppFrameTransactionStep::Pending => ActiveFrameStep::Pending,
                    crate::AppFrameTransactionStep::Fault => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Prepare(preparation) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let outcome = preparation.drive_step(self.operation, self.generation, self.cancel.clone(), &mut self.preview_sequence);
                if frame_phase_overran(violations, "os_renderer.prepare.worker", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.prepare.worker overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match outcome {
                    StepOutcome::Complete(_) => {
                        let frame = preparation.take_presentation();
                        self.phase = ActiveFramePhase::Terminal;
                        ActiveFrameStep::Complete(frame)
                    }
                    StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => ActiveFrameStep::Pending,
                    StepOutcome::Cancelled | StepOutcome::Fault(_) => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Terminal => ActiveFrameStep::Complete(None),
        }
    }
}

impl InteractiveJob for ActiveFrameBuild {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            self.cancel();
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        cx.consume_fuel(1);
        match self.advance() {
            ActiveFrameStep::Pending => StepOutcome::Yield,
            ActiveFrameStep::Complete(frame) => {
                self.completed = frame;
                StepOutcome::Complete(CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.cancel();
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return if self.terminal_is_empty() { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        }
        if let Some(frame) = self.completed.as_mut() {
            if !frame.close_step() || !frame.terminal_is_empty() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.completed = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if !retire_active_phase(&mut self.phase) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        self.phase = ActiveFramePhase::Terminal;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.completed.is_none() && matches!(self.phase, ActiveFramePhase::Terminal)
    }
}

fn generation_is_fresh(requested: Generation, completed: Generation) -> bool {
    requested == completed
}

impl FrameBuildHandle {
    pub(crate) fn new() -> Self {
        Self {
            session: None,
            rejected: None,
            ticket: None,
            #[cfg(not(target_arch = "wasm32"))]
            completion_waker: None,
            latest_requested_generation: Generation(0),
            last_submitted_generation: None,
            cancel: root_cancel_token(),
            closing: false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_completion_waker(&mut self, waker: Arc<dyn Fn() + Send + Sync>) {
        if !self.closing {
            self.completion_waker = Some(waker);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_completion_waker(&mut self, _waker: Arc<dyn Fn() + Send + Sync>) {}

    fn admit_active(&mut self, active: ActiveFrameBuild) {
        let params = batch_params(active.operation, active.generation, active.cancel.clone());
        match semio_framework_job::WorkerJobSession::try_new(active, params) {
            Ok(session) => self.session = Some(session),
            Err(mut rejected) => {
                rejected.begin_close();
                self.rejected = Some(rejected);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32) -> Option<crate::AppFramePresentation> {
        if self.closing {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return None;
        }
        if let Some(session) = self.session.as_ref() {
            if session.generation() != generation {
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                } else {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                return None;
            }
            if let Some(callback) = self.completion_waker.as_ref() {
                let waker = std::task::Waker::from(Arc::new(FrameCompletionWake(Arc::clone(callback))));
                let _ = session.register_wake(&waker);
            }
            match session.poll() {
                semio_framework_job::WorkerJobPoll::Idle => match session.try_submit_step(&crate::renderer_worker_pool(), Lane::Interactive) {
                    Ok(ticket) => self.ticket = Some(ticket),
                    Err(semio_framework_job::WorkerJobSubmitFault::Pool(kind)) => {
                        if let Ok(rejected) = session.take_rejected() {
                            if matches!(kind, semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended) {
                                rejected.resume();
                            } else {
                                rejected.begin_close();
                            }
                        }
                    }
                    Err(_) => {
                        let _ = session.begin_close();
                    }
                },
                semio_framework_job::WorkerJobPoll::Outcome => {
                    if let Some(ticket) = self.ticket.take() {
                        if let Ok(mut owner) = session.take_outcome(ticket) {
                            if matches!(owner.outcome(), StepOutcome::Yield) {
                                let _ = owner.take_outcome();
                                let _ = owner.resume();
                            } else {
                                owner.begin_close();
                            }
                        }
                    }
                }
                semio_framework_job::WorkerJobPoll::Terminal => {
                    if let Ok(mut owner) = session.take_terminal() {
                        let frame_generation = owner.job().generation;
                        let frame = owner.job_mut().completed.take();
                        owner.begin_close();
                        return generation_is_fresh(generation, frame_generation).then_some(frame).flatten();
                    }
                }
                semio_framework_job::WorkerJobPoll::Rejected => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                semio_framework_job::WorkerJobPoll::Closing => {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                semio_framework_job::WorkerJobPoll::Submitted | semio_framework_job::WorkerJobPoll::CheckedOut | semio_framework_job::WorkerJobPoll::TerminalEmpty => {}
            }
            return None;
        }
        if self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, dpr, self.cancel.clone()));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32) -> Option<crate::AppFramePresentation> {
        if self.closing || web_sys::window().is_some() {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return None;
        }
        if let Some(session) = self.session.as_ref() {
            if session.generation() != generation {
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                } else {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                return None;
            }
            match session.poll() {
                semio_framework_job::WorkerJobPoll::Idle => {
                    if let Ok((ticket, _)) = session.try_step_on_caller() {
                        self.ticket = Some(ticket);
                    }
                }
                semio_framework_job::WorkerJobPoll::Outcome => {
                    if let Some(ticket) = self.ticket.take() {
                        if let Ok(mut owner) = session.take_outcome(ticket) {
                            let _ = owner.take_outcome();
                            let _ = owner.resume();
                        }
                    }
                }
                semio_framework_job::WorkerJobPoll::Terminal => {
                    if let Ok(mut owner) = session.take_terminal() {
                        let frame_generation = owner.job().generation;
                        let frame = owner.job_mut().completed.take();
                        owner.begin_close();
                        return generation_is_fresh(generation, frame_generation).then_some(frame).flatten();
                    }
                }
                semio_framework_job::WorkerJobPoll::Closing => {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                _ => {}
            }
            return None;
        }
        if self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, dpr, self.cancel.clone()));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if !self.closing {
            self.closing = true;
            self.cancel.cancel_now();
            return false;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return false;
        }
        if let Some(session) = self.session.as_ref() {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                return false;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.completion_waker.take().is_some() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.rejected.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.completion_waker.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}
//#endregion 📮️FrameBuildHandle

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(now_ms: f64) -> FrameBuildInputs {
        FrameBuildInputs { wheel_zoom_deadline_ms: 500.0, now_ms }
    }

    fn compute(inputs: FrameBuildInputs) -> FrameDirectives {
        let params = batch_params(OperationId(1), Generation(1), root_cancel_token());
        let mut session = BatchJobSession::try_new(FrameBuildJob::new(inputs), params).unwrap_or_else(|_| panic!("frame compute session admission"));
        assert!(matches!(session.step(), Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)));
        let directives = session.checked_out_job_mut().and_then(FrameBuildJob::take_directives).unwrap_or_else(|| panic!("completed directives"));
        let mut outcome = session.take_outcome().unwrap_or_else(|| panic!("frame compute retained outcome"));
        assert!(matches!(outcome, StepOutcome::Complete(_)));
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        session.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        directives
    }

    #[test]
    fn not_yet_expired_deadlines_are_kept() {
        // 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
        // "not yet expired" for BOTH needs `now_ms` before the earlier of the two. Caught by the
        // standalone verify crate's real `cargo test` run (`🧪️frame-job-verify`), not by inspection —
        // see `📓️p3b-frame-building.md` §7 for why this file itself cannot be `cargo test`-ed directly.
        let directives = compute(inputs(100.0));
        assert!(!directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn expired_deadline_is_reported() {
        let directives = compute(inputs(1_000.0));
        assert!(directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn stale_runtime_frame_generation_is_rejected() {
        assert!(generation_is_fresh(Generation(7), Generation(7)));
        assert!(!generation_is_fresh(Generation(8), Generation(7)));
    }

    #[test]
    fn cancellation_retires_deadline_apply_and_build_phases_to_terminal_empty() {
        let deadline = BatchJobSession::try_new(FrameBuildJob::new(FrameBuildInputs { wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 }), batch_params(OperationId(20), Generation(20), root_cancel_token()))
            .unwrap_or_else(|_| panic!("frame deadline test session admission"));
        let mut phases = [ActiveFramePhase::Deadlines(deadline), ActiveFramePhase::ApplyPending(FrameDirectives { wheel_zoom_deadline_cleared: false })];
        for phase in &mut phases {
            for _ in 0..2_000 {
                if retire_active_phase(phase) {
                    break;
                }
            }
            assert!(retire_active_phase(phase));
        }
    }

    #[test]
    fn cancellation_retires_empty_preparation_to_terminal_empty() {
        let build = crate::AppFrameBuild {
            input: ui_wgpu::wgpu::PreparedRenderInput::new(1, 1, ui_wgpu::wgpu::DrawList::default(), None, 0.0),
            engine_packets: Vec::new(),
            cursor: ui_wgpu::wgpu::SemioCursor::Default,
            theme_dark: false,
            fullscreen: None,
            cursor_wake: None,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: None,
        };
        let mut phase = ActiveFramePhase::Prepare(build.into_preparation());
        for _ in 0..100 {
            if retire_active_phase(&mut phase) {
                break;
            }
        }
        assert!(retire_active_phase(&mut phase));
    }
}
