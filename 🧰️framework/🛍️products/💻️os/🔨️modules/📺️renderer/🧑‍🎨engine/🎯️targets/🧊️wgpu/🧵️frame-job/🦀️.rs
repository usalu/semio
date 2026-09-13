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

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
use semio_framework_job::{
    root_cancel_token, BatchDriveConfig, BatchJobParams, BatchJobSession, CancelToken, CommitCandidate, InteractiveJob, StepContext, StepOutcome, WorkerJobSessionAdmissionRejected, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US,
};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::sync::Arc;

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
    /// ♻️ Incremental retirement of the directive owner — driven by `🧊️renderer/🦀️.rs`'s own frame
    /// close ladder, so it is crate-visible rather than module-private.
    pub(crate) fn close_step(&mut self) -> bool {
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
/// ⏱️ Reads the shared real microsecond clock used by job deadlines and watchdogs.
fn now_us() -> Option<u64> {
    semio_framework_job::default_now_us()
}

/// ⏱️ The share of one interactive step the browser frame caller may spend driving the frame build
/// before it must hand the rest of its tick to the present half. Half of the ratified
/// [`INTERACTIVE_STEP_CEILING_US`] — the other half belongs to `present_snapshot`, whose own prepared
/// GPU opportunities are priced separately. Derived rather than chosen so it cannot drift away from
/// the one ceiling every interactive site is measured against.
#[cfg(target_arch = "wasm32")]
const BROWSER_FRAME_BUILD_DRIVE_US: u64 = semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2;

fn batch_params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams {
        operation,
        generation,
        cancel,
        config: BatchDriveConfig { site: "os_renderer_frame_build", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us,
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
    rejected: Option<WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>,
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
    Build(crate::FrameTransaction),
    Prepare(crate::AppFramePreparation),
    Terminal,
}

struct ActiveFrameBuild {
    runtime: crate::RuntimeMailbox,
    handle: crate::AppHandle,
    operation: OperationId,
    generation: Generation,
    cancel: CancelToken,
    preview_sequence: u64,
    phase: ActiveFramePhase,
    overruns: semio_framework_trace::StepOverrunLedger,
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
        ActiveFramePhase::Build(transaction) => transaction.close_step() && transaction.terminal_is_empty(),
        ActiveFramePhase::Prepare(preparation) => preparation.close_step() && preparation.terminal_is_empty(),
        ActiveFramePhase::Terminal => true,
    }
}

impl ActiveFrameBuild {
    fn new(runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, cancel: CancelToken) -> Self {
        let handle = runtime.downgrade();
        let phase = match BatchJobSession::try_new(FrameBuildJob::new(inputs), batch_params(operation, generation, cancel.clone())) {
            Ok(session) => ActiveFramePhase::Deadlines(session),
            Err(mut rejected) => {
                rejected.begin_close();
                ActiveFramePhase::DeadlineAdmissionRejected(rejected)
            }
        };
        Self { runtime, handle, operation, generation, cancel, preview_sequence: 0, phase, overruns: semio_framework_trace::StepOverrunLedger::new(), completed: None, closing: false }
    }

    fn cancel(&self) {
        self.cancel.cancel_now();
    }

    fn retire_cancelled_phase(&mut self) -> bool {
        retire_active_phase(&mut self.phase)
    }

    /// 🛑️ Terminates the frame for an overrun its own `StepOverrunLedger` attributed to the step —
    /// an unusable clock reading, or `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive over-ceiling
    /// phases. One over-ceiling WALL reading on a browser worker sharing a core is recorded by the
    /// watchdog and never reaches here.
    fn quarantine_overrun(&self, site: &'static str) {
        self.runtime.record_frame_fault(site);
        self.cancel.cancel_now();
    }

    /// 📮️ One runtime-mailbox opportunity, priced by the same interactive ceiling every frame phase is.
    ///
    /// ⚖️ Returns whether a completion was applied, and quarantines on its own overrun exactly as the
    /// phase that used to own this call did.
    fn pump_runtime_mailbox_step(&mut self) -> bool {
        let watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.apply_pending", self.operation, self.generation, InteractiveStage::InteractiveStep);
        if !watchdog.is_admitted() {
            self.quarantine_overrun("os_renderer.frame.apply_pending has no monotonic clock");
            return true;
        }
        let applied = self.runtime.apply_pending_step();
        if self.overruns.admit(&watchdog.finish()).is_terminal() {
            self.quarantine_overrun("os_renderer.frame.apply_pending overran the interactive ceiling");
            return true;
        }
        applied
    }

    fn advance(&mut self) -> ActiveFrameStep {
        if self.cancel.is_cancelled_now() {
            if self.retire_cancelled_phase() {
                self.phase = ActiveFramePhase::Terminal;
                return ActiveFrameStep::Complete(None);
            }
            return ActiveFrameStep::Pending;
        }
        // 📮️ The mailbox is pumped at EVERY advance, not only in `ApplyPending`.
        //
        // 🩸️ It used to be one phase, visited once per build: the build drained the queue, then walked
        // on. A transaction that parked on `!interaction_available()` therefore held the one live
        // session forever — no new build could be admitted (`FrameBuildHandle::poll_runtime_and_resubmit`
        // admits only when `session.is_none()`), so the `ResumeDispatch` carrying the interaction state
        // home was never applied, and `DispatchEvents` behind it never reached
        // `winit_app::dispatch_normalized_event` at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
        // `📓️wgpu-server-input-present-2026-09-13.md` §5.2). Draining from every advance is what makes
        // "applied within the next frame" true for `DispatchEvents` and `Resize` alike.
        if self.pump_runtime_mailbox_step() {
            return ActiveFrameStep::Pending;
        }
        if matches!(self.phase, ActiveFramePhase::ApplyPending(_)) {
            let directives = match &mut self.phase {
                ActiveFramePhase::ApplyPending(directives) => std::mem::take(directives),
                _ => return ActiveFrameStep::Pending,
            };
            self.phase = ActiveFramePhase::Build(crate::FrameTransaction::new(directives, self.operation, self.generation));
            return ActiveFrameStep::Pending;
        }
        match &mut self.phase {
            ActiveFramePhase::Deadlines(session) => {
                let poll = session.step();
                if !matches!(poll, Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) || !session.checkout_outcome() {
                    return ActiveFrameStep::Pending;
                }
                let deadline_verdict = session.callback_verdict().copied();
                if deadline_verdict.is_some_and(|verdict| self.overruns.admit(&verdict).is_terminal()) {
                    self.quarantine_overrun("os_renderer.frame.deadlines exceeded its exact clock authority");
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
            ActiveFramePhase::ApplyPending(_) => ActiveFrameStep::Pending,
            ActiveFramePhase::Build(transaction) => {
                let watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.transaction", self.operation, self.generation, InteractiveStage::InteractiveStep);
                if !watchdog.is_admitted() {
                    self.quarantine_overrun("os_renderer.frame.transaction has no monotonic clock");
                    return ActiveFrameStep::Pending;
                }
                let transaction_step = {
                    let now = now_us();
                    let mut context = StepContext::new(self.operation, self.generation, now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)), self.cancel.clone(), now_us, &mut self.preview_sequence);
                    transaction.step(&self.runtime, &self.handle, &mut context)
                };
                if self.overruns.admit(&watchdog.finish()).is_terminal() {
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
                    // 🌀️ Superseded inputs end THIS build with no frame and no fault; the caller's next
                    // opportunity admits a fresh one against the current witness.
                    crate::AppFrameTransactionStep::Superseded => {
                        self.phase = ActiveFramePhase::Terminal;
                        ActiveFrameStep::Complete(None)
                    }
                    crate::AppFrameTransactionStep::Fault => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Prepare(preparation) => {
                let outcome = preparation.drive_step(self.operation, self.generation, self.cancel.clone(), &mut self.preview_sequence);
                let prepare_verdict = preparation.callback_verdict().copied();
                if prepare_verdict.is_some_and(|verdict| self.overruns.admit(&verdict).is_terminal()) {
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
    /// ⏱️ One step spends the WHOLE grant its [`StepContext`] was minted with
    /// ([`batch_params`]: [`INTERACTIVE_LANE_FUEL`] fuel, [`INTERACTIVE_LANE_WALL_US`] wall), not one
    /// [`Self::advance`].
    ///
    /// 🩸️ It used to yield after a single `advance`, and the browser session driver spends a whole
    /// caller frame per step plus a second one consuming the outcome — so the chrome walk advanced ONE
    /// child per ~32 ms at 62 fps. A retained window body ingests one document page per advance, and a
    /// boot chrome pass is hundreds of them across seven surfaces, so the first frame could not
    /// converge at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-blank-paint-2026-09-12.md`). Every `advance` still carries its own
    /// `os_renderer.frame.*` watchdog and quarantine admission, so the loop cannot hide an
    /// over-ceiling phase; the budget is what ends the step.
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        loop {
            if cx.is_cancelled() {
                self.cancel();
            }
            if cx.should_yield() {
                return StepOutcome::Yield;
            }
            cx.consume_fuel(1);
            match self.advance() {
                ActiveFrameStep::Pending => {}
                ActiveFrameStep::Complete(frame) => {
                    self.completed = frame;
                    return StepOutcome::Complete(CommitCandidate {
                        state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                        output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                    });
                }
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
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> Option<crate::AppFramePresentation> {
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
            self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone()));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> Option<crate::AppFramePresentation> {
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
            // 🌀️ A superseded build is RETIRED HERE, in this caller's own share — never parked.
            //
            // 🩸️ This used to cancel the session and `return None`, which drove the close ladder by one
            // step per CALL. On the browser the caller is an event-driven tick, so a shell that is
            // settled between inputs ticks about once a second: one superseded build then held
            // `self.session` for tens of seconds, no replacement build could be admitted
            // (`self.session.is_none()` is the admission gate), and therefore NOTHING pumped the runtime
            // mailbox — measured on 6118 as `frame build cancelled: session generation Generation(3) !=
            // requested Generation(4)` followed by 74 seconds with no further `frame build admitted`,
            // no `render begin`, and 75 undelivered `DispatchEvents`
            // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
            // Falling through to the drive loop spends the SAME interactive share retiring it that a
            // live build would have spent stepping, so the next opportunity admits a fresh one.
            if session.generation() != generation {
                crate::log_debug_once_per_transition("frame-session-generation", true, &format!("[DEBUG] frame build superseded: session generation {:?} != requested {generation:?}", session.generation()));
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                }
            } else {
                crate::log_debug_once_per_transition("frame-session-generation", false, "[DEBUG] frame build session generation matches the requested one again");
            }
            // 🌐️ The browser drives the step ON THE CALLER, never through a pool: this function
            // only runs inside the dedicated `semio-frame-worker` isolate (the `web_sys::window()`
            // guard above fails closed everywhere else), that isolate IS the frame thread, and a
            // `wasm32-unknown-unknown` build has no second thread to submit to at all. The job
            // legitimately owns `Rc<JsValue>` plugin handles and an `Rc` waker, which
            // `try_submit_step`'s `J: Send` — the one call that really hands a job to another
            // thread — rules out by construction. Contention is transient: the next
            // `poll_runtime_and_resubmit` retries the same generation, exactly as a saturated
            // pool submission used to.
            //
            // ⏱️ Step / take-outcome / resume are THREE session phases, so a driver that ran one
            // phase per call spent two caller frames per job step and the frame build advanced at
            // 31 steps a second — a boot chrome pass never converged (see [`ActiveFrameBuild::step`]).
            // The loop runs the same phases back to back until the frame build finishes or this
            // caller's own share of the interactive ceiling is gone.
            let drive_deadline_us = now_us().map(|now| now.saturating_add(BROWSER_FRAME_BUILD_DRIVE_US));
            let mut presentation = None;
            let mut retire_session = false;
            loop {
                match session.poll() {
                    semio_framework_job::WorkerJobPoll::Idle => match session.try_step_on_caller() {
                        Ok((ticket, _)) => self.ticket = Some(ticket),
                        Err(_) => break,
                    },
                    semio_framework_job::WorkerJobPoll::Outcome => {
                        let Some(ticket) = self.ticket.take() else { break };
                        let Ok(mut owner) = session.take_outcome(ticket) else { break };
                        if !matches!(owner.outcome(), StepOutcome::Yield) {
                            owner.begin_close();
                            break;
                        }
                        let _ = owner.take_outcome();
                        let _ = owner.resume();
                    }
                    semio_framework_job::WorkerJobPoll::Terminal => {
                        let Ok(mut owner) = session.take_terminal() else { break };
                        let frame_generation = owner.job().generation;
                        let frame = owner.job_mut().completed.take();
                        owner.begin_close();
                        presentation = generation_is_fresh(generation, frame_generation).then_some(frame).flatten();
                        break;
                    }
                    semio_framework_job::WorkerJobPoll::Closing => {
                        let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                        if session.terminal_is_empty() {
                            retire_session = true;
                            break;
                        }
                    }
                    semio_framework_job::WorkerJobPoll::TerminalEmpty => {
                        retire_session = true;
                        break;
                    }
                    _ => break,
                }
                if drive_deadline_us.is_none_or(|deadline| now_us().is_none_or(|now| now >= deadline)) {
                    break;
                }
            }
            if retire_session {
                self.session = None;
            }
            return presentation;
        }
        // 🔁️ No live session means the previous frame finished (or never started), and the caller
        // already refuses to produce while a presentation is pending — so the next frame build starts
        // HERE, every opportunity.
        //
        // 🩸️ This used to admit only when `generation` differed from the last admitted one. The browser
        // frame generation advances on host EVENTS, not on redraws, so an idle shell admitted exactly
        // one build for its whole life: the first frame completed with its retained window bodies still
        // mid-ingress, and no second frame was ever built to finish them — a permanently blank canvas
        // that still asked for frames (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
        // `📓️wgpu-blank-paint-2026-09-12.md`). One-build-at-a-time is enforced by `self.session`, which
        // is the authority that gate was standing in for.
        crate::log_debug(&format!("[DEBUG] frame build admitted generation={generation:?}"));
        self.cancel = root_cancel_token();
        self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone()));
        self.last_submitted_generation = Some(generation);
        None
    }

    /// 🧵️ Whether a frame build is admitted right now — the one owner that must keep its own frames
    /// coming and must not have its inputs renumbered underneath it.
    pub(crate) fn has_live_session(&self) -> bool {
        self.session.is_some() || self.rejected.is_some()
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
#[path = "../../../🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs"]
mod tests;
