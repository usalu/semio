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
use std::{mem::size_of, sync::Arc};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};

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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        if self.complete.is_none() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < size_of::<FrameDirectives>() {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_depth == 0 {
            return InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress: RetainedCloneProgress::default() };
        }
        self.complete = None;
        InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<FrameDirectives>(), ..RetainedCloneProgress::default() } }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(if self.complete.is_some() { size_of::<FrameDirectives>() } else { 0 })
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(usize::from(self.complete.is_some()))
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

/// ⏱️ One frame-build step's grant: the interactive lane's fuel and wall budget, charged per retained
/// Worker turn by [`ActiveFrameBuild::step`].
fn batch_params(operation: OperationId, generation: Generation, cancel: CancelToken, retained: RetainedCloneGrant) -> BatchJobParams {
    BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { retained, site: "os_renderer_frame_build", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US }, now_us }
}
//#endregion ⏱️Clock

//#region 📮️FrameBuildHandle
/// 📮️ The non-blocking poll/resubmit contract item 5 of the packet brief asks for: never waits on the
/// worker. `poll_and_resubmit` always returns immediately — either this tick's freshly-completed
/// directives, or (if the in-flight job hasn't finished, or none was in flight yet) the last
/// successfully computed ones. One job is in flight at a time; a still-running job is left alone (not
/// cancelled) and re-checked next call rather than submitting a second overlapping one.
pub(crate) struct FrameBuildHandle {
    retained: RetainedCloneGrant,
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
    retained: RetainedCloneGrant,
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

fn run_frame_owner_turn(cx: &mut StepContext<'_>, advance: impl FnOnce(&mut StepContext<'_>) -> ActiveFrameStep) -> Option<ActiveFrameStep> {
    if cx.should_yield() {
        return None;
    }
    let step = advance(cx);
    cx.consume_fuel(1);
    Some(step)
}

fn worker_close_step(step: semio_framework_job::WorkerJobCloseStep, grant: RetainedCloneGrant, terminal: bool) -> InteractiveJobCloseStep {
    use semio_framework_job::WorkerJobCloseStep;
    match step {
        WorkerJobCloseStep::Pending { progress } => InteractiveJobCloseStep::Pending { progress },
        WorkerJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Complete { progress },
        WorkerJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
        WorkerJobCloseStep::Refused { kind, progress } => InteractiveJobCloseStep::Refused { kind, progress },
    }.admit(grant, terminal)
}

fn retire_active_phase(phase: &mut ActiveFramePhase, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
    let empty = match phase {
        ActiveFramePhase::Deadlines(session) => {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
            }
            if !session.terminal_is_empty() { return crate::frame_child_step(worker_close_step(session.close_step(grant), grant, session.terminal_is_empty()), grant, session.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::DeadlineAdmissionRejected(rejected) => {
            if !rejected.terminal_is_empty() { return crate::frame_child_step(rejected.close_step(grant), grant, rejected.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::ApplyPending(_) => true,
        ActiveFramePhase::Build(transaction) => {
            if !transaction.terminal_is_empty() { return crate::frame_child_step(transaction.close_step(grant), grant, transaction.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::Prepare(preparation) => {
            if !preparation.terminal_is_empty() { return crate::frame_child_step(preparation.close_step(grant), grant, preparation.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::Terminal => return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() },
    };
    if empty {
        let bytes = size_of::<ActiveFramePhase>();
        if let Some(step) = crate::frame_inline_gate(grant, bytes) { return step; }
        *phase = ActiveFramePhase::Terminal;
        return crate::frame_inline_progress(bytes);
    }
    InteractiveJobCloseStep::Blocked
}

impl ActiveFrameBuild {
    fn new(runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, cancel: CancelToken, retained: RetainedCloneGrant) -> Self {
        let handle = runtime.downgrade();
        let phase = match BatchJobSession::try_new(FrameBuildJob::new(inputs), batch_params(operation, generation, cancel.clone(), retained)) {
            Ok(session) => ActiveFramePhase::Deadlines(session),
            Err(mut rejected) => {
                rejected.begin_close();
                ActiveFramePhase::DeadlineAdmissionRejected(rejected)
            }
        };
        Self { retained, runtime, handle, operation, generation, cancel, preview_sequence: 0, phase, overruns: semio_framework_trace::StepOverrunLedger::new(), completed: None, closing: false }
    }

    fn cancel(&self) {
        self.cancel.cancel_now();
    }

    fn retire_cancelled_phase(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let candidate_returned = match &mut self.phase {
            ActiveFramePhase::Build(transaction) => transaction.discard_presented_input_candidate(&self.runtime),
            ActiveFramePhase::Prepare(preparation) => preparation.discard_presented_input_candidate(&self.runtime),
            _ => true,
        };
        if !candidate_returned {
            return InteractiveJobCloseStep::Blocked;
        }
        retire_active_phase(&mut self.phase, grant)
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

    /// 📮️ The mailbox is pumped at EVERY advance, not only in `ApplyPending`.
    ///
    /// 🩸️ It used to be one phase, visited once per build: the build drained the queue, then walked
    /// on. A transaction that parked on `!interaction_available()` therefore held the one live
    /// session forever — no new build could be admitted (`FrameBuildHandle::poll_runtime_and_resubmit`
    /// admits only when `session.is_none()`), so the `ResumeDispatch` carrying the interaction state
    /// home was never applied, and `DispatchEvents` behind it never reached
    /// `winit_app::dispatch_normalized_event` at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-server-input-present-2026-09-13.md` §5.2). Draining from every advance is what makes
    /// "applied within the next frame" true for `DispatchEvents` and `Resize` alike.
    ///
    /// 🌀️ Superseded inputs end THIS build with no frame and no fault; the caller's next
    /// opportunity admits a fresh one against the current witness. The transaction is
    /// DROPPED here rather than closed after its exact external input witness is
    /// returned. Every action its authorities take belongs to `AppRuntime::frame_actions`
    /// (`🧊️renderer/🦀️.rs`), which outlives every candidate.
    ///
    /// 🩺️ A refused preparation is the ONE terminal this build cannot explain by itself.
    fn advance(&mut self, cx: &mut StepContext<'_>) -> ActiveFrameStep {
        if self.cancel.is_cancelled_now() {
            let grant = cx.retained_grant();
            let step = self.retire_cancelled_phase(grant);
            if cx.consume_retained(step.progress()).is_err() { return ActiveFrameStep::Pending; }
            if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
                self.phase = ActiveFramePhase::Terminal;
                return ActiveFrameStep::Complete(None);
            }
            return ActiveFrameStep::Pending;
        }
        if self.pump_runtime_mailbox_step() {
            return ActiveFrameStep::Pending;
        }
        if matches!(self.phase, ActiveFramePhase::ApplyPending(_)) {
            let directives = match &mut self.phase {
                ActiveFramePhase::ApplyPending(directives) => std::mem::take(directives),
                _ => return ActiveFrameStep::Pending,
            };
            self.phase = ActiveFramePhase::Build(crate::FrameTransaction::new(directives, self.operation, self.generation, self.retained));
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
                let grant = cx.retained_grant();
                let step = rejected.close_step(grant).admit(grant, rejected.terminal_is_empty());
                if cx.consume_retained(step.progress()).is_err() { return ActiveFrameStep::Pending; }
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
                let transaction_step = transaction.step(&self.runtime, &self.handle, cx);
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
                    crate::AppFrameTransactionStep::Superseded => {
                        if !transaction.discard_presented_input_candidate(&self.runtime) {
                            return ActiveFrameStep::Pending;
                        }
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
                let outcome = preparation.drive_step(self.operation, self.generation, self.cancel.clone(), self.retained, &mut self.preview_sequence);
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
                        crate::log_debug_once_per_transition("frame-prepare-refused", true, &format!("os_host frame preparation refused: {}", preparation.fault().unwrap_or("unnamed")));
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
    /// ⏱️ One retained Worker turn advances exactly one frame phase and charges that attempt after it
    /// runs, including a terminal attempt. Native resubmits the retained owner through the process
    /// pool; browser wasm reschedules it inside the dedicated frame Worker.
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            self.cancel();
        }
        match run_frame_owner_turn(cx, |cx| self.advance(cx)) {
            None | Some(ActiveFrameStep::Pending) => StepOutcome::Yield,
            Some(ActiveFrameStep::Complete(frame)) => {
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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        if let Some(frame) = self.completed.as_mut() {
            if !frame.discard_presented_input_candidate(&self.runtime) { return InteractiveJobCloseStep::Blocked; }
            if !frame.terminal_is_empty() { return crate::frame_child_step(frame.close_step(grant), grant, frame.terminal_is_empty()); }
            let bytes = size_of::<Option<crate::AppFramePresentation>>();
            if let Some(step) = crate::frame_inline_gate(grant, bytes) { return step; }
            self.completed = None;
            return crate::frame_inline_progress(bytes);
        }
        self.retire_cancelled_phase(grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.completed.is_none() && matches!(self.phase, ActiveFramePhase::Terminal)
    }
}

fn generation_is_fresh(requested: Generation, completed: Generation) -> bool {
    requested == completed
}

impl FrameBuildHandle {
    pub(crate) fn new(retained: RetainedCloneGrant) -> Self {
        Self {
            retained,
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
        let params = batch_params(active.operation, active.generation, active.cancel.clone(), active.retained);
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
            let _ = rejected.close_step(self.retained);
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
                    let _ = session.close_step(self.retained);
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
                        let frame = generation_is_fresh(generation, frame_generation).then(|| owner.job_mut().completed.take()).flatten();
                        owner.begin_close();
                        return frame;
                    }
                }
                semio_framework_job::WorkerJobPoll::Rejected => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                semio_framework_job::WorkerJobPoll::Closing => {
                    let _ = session.close_step(self.retained);
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
            self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone(), self.retained));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    /// 🌀️ A superseded build is RETIRED HERE, in this Worker turn — never parked.
    ///
    /// 🩸️ This used to cancel the session and `return None`, which drove the close ladder by one
    /// step per CALL. On the browser the caller is an event-driven tick, so a shell that is
    /// settled between inputs ticks about once a second: one superseded build then held
    /// `self.session` for tens of seconds, no replacement build could be admitted
    /// (`self.session.is_none()` is the admission gate), and therefore NOTHING pumped the runtime
    /// mailbox — measured on 6118 as `frame build cancelled: session generation Generation(3) !=
    /// requested Generation(4)` followed by 74 seconds with no further `frame build admitted`,
    /// no `render begin`, and 75 undelivered `DispatchEvents`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
    /// Falling through spends one bounded Worker turn retiring it, so a later callback can admit
    /// a fresh build without borrowing work from the page or running an uncharged close loop.
    ///
    /// 🌐️ The dedicated `semio-frame-worker` scheduler owns this opportunity. Wasm has no
    /// second thread for the `Send`-gated pool path, so `try_step_on_worker` executes one exact
    /// retained owner turn in this isolate. Checkout and resume are ownership bookkeeping for
    /// that SAME turn; they do not run another job unit. A later scheduler callback owns every
    /// subsequent unit or close step.
    ///
    /// 🔁️ No live session means the previous frame finished (or never started), and the caller
    /// already refuses to produce while a presentation is pending — so the next frame build starts
    /// HERE, every opportunity.
    ///
    /// 🩸️ This used to admit only when `generation` differed from the last admitted one. The browser
    /// frame generation advances on host EVENTS, not on redraws, so an idle shell admitted exactly
    /// one build for its whole life: the first frame completed with its retained window bodies still
    /// mid-ingress, and no second frame was ever built to finish them — a permanently blank canvas
    /// that still asked for frames (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-blank-paint-2026-09-12.md`). One-build-at-a-time is enforced by `self.session`, which
    /// is the authority that gate was standing in for.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> Option<crate::AppFramePresentation> {
        if self.closing || web_sys::window().is_some() {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(self.retained);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return None;
        }
        if let Some(session) = self.session.as_ref() {
            if session.generation() != generation {
                crate::log_debug_once_per_transition("frame-session-generation", true, &format!("[TRACE] frame build superseded: session generation {:?} != requested {generation:?}", session.generation()));
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                }
            } else {
                crate::log_debug_once_per_transition("frame-session-generation", false, "[TRACE] frame build session generation matches the requested one again");
            }
            let poll = match session.poll() {
                semio_framework_job::WorkerJobPoll::Idle => match session.try_step_on_worker() {
                    Ok((ticket, poll)) => {
                        self.ticket = Some(ticket);
                        poll
                    }
                    Err(_) => return None,
                },
                poll => poll,
            };
            let mut presentation = None;
            let mut retire_session = false;
            match poll {
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
                        let frame = generation_is_fresh(generation, frame_generation).then(|| owner.job_mut().completed.take()).flatten();
                        owner.begin_close();
                        presentation = frame;
                    }
                }
                semio_framework_job::WorkerJobPoll::Closing => {
                    let _ = session.close_step(self.retained);
                    retire_session = session.terminal_is_empty();
                }
                semio_framework_job::WorkerJobPoll::TerminalEmpty => retire_session = true,
                _ => {}
            }
            if retire_session {
                self.session = None;
            }
            return presentation;
        }
        crate::log_debug_diagnostic(&format!("[TRACE] frame build admitted generation={generation:?}"));
        self.cancel = root_cancel_token();
        self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone(), self.retained));
        self.last_submitted_generation = Some(generation);
        None
    }

    /// 🧵️ Whether a frame build is admitted right now — the one owner that must keep its own frames
    /// coming and must not have its inputs renumbered underneath it.
    pub(crate) fn has_live_session(&self) -> bool {
        self.session.is_some() || self.rejected.is_some()
    }

    /// 🪢 Retires one frame owner unit so a component-close bridge created by that frame can run.
    /// Unlike [`Self::close_step`], this keeps the handle reusable and preserves its installed wake.
    pub(crate) fn retire_for_component_surface_close_step(&mut self) -> bool {
        self.cancel.cancel_now();
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(self.retained);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
        } else if let Some(session) = self.session.as_ref() {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                return false;
            }
            let _ = session.close_step(self.retained);
            if session.terminal_is_empty() {
                self.session = None;
                self.ticket = None;
            }
        }
        if self.has_live_session() {
            return false;
        }
        self.last_submitted_generation = None;
        true
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if !self.closing {
            self.closing = true;
            self.cancel.cancel_now();
            return false;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(self.retained);
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
            let _ = session.close_step(self.retained);
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
