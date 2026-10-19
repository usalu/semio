//! 🧵️ P3/P5 mounted frame coordinator. [`FrameBuildJob`] incrementally derives deadline candidates,
//! and [`FrameBuildHandle::poll_runtime_on_original_caller`] couples that protocol to the complete
//! `AppRuntime::frame` transaction under the initiating caller's own paid authority: the admitted
//! session, its receipts and every close step are funded by that caller's context. Frame
//! construction owns shell traversal, layout/tessellation, engine-scene directives, and
//! prepared-packet creation; `AppPresenter` retains platform/GPU presentation authority.
//!
//! The scalar wheel deadline candidate remains non-authoritative: `AppRuntime::frame` revalidates it
//! against live state before applying it. The wasm implementation drives the job only after the
//! renderer has booted inside its dedicated Worker isolate; calls from a browser UI isolate fail
//! closed and never execute the transaction inline.

use semio_framework_job::{CancelToken,InteractiveJob,StepContext};
use semio_framework_job_worker::BatchJobSession;use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::mem::size_of;
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_frame_actions::{FrameBuildInputs, FrameDirectives};

//#region 📥️FrameBuildInputs
//#endregion 📥️FrameBuildInputs

//#region 📤️FrameDirectives
//#endregion 📤️FrameDirectives

//#region 🧩️FrameBuildJob
/// 🧩️ The deadline-sweep computation as a real [`InteractiveJob`] — one call to `step` always reaches
/// [`JobOutcomeBorrow::Complete`] (the work is O(open deadlines), never large enough to need more than one
/// step; `cx.consume_fuel` is still called so a future, larger stage added to this same job under
/// Phase 5 inherits real yield-on-overrun behaviour instead of it being bolted on later).
pub struct FrameBuildJob {
    wheel_zoom_deadline_cleared: bool,
    complete: Option<FrameDirectives>,
    closing: bool,
}

impl FrameBuildJob {
    pub fn new(inputs: FrameBuildInputs) -> Self {
        Self { wheel_zoom_deadline_cleared: inputs.wheel_zoom_deadline_ms > 0.0 && inputs.now_ms >= inputs.wheel_zoom_deadline_ms, complete: None, closing: false }
    }

    pub fn take_directives(&mut self) -> Option<FrameDirectives> {
        self.complete.take()
    }


}

impl InteractiveJob for FrameBuildJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if cx.is_cancelled() || self.closing {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from);
        }
        if self.complete.is_none() {
            let grant = cx.retained_grant();
            let copy_bytes = size_of::<Option<FrameDirectives>>();
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < copy_bytes || grant.maximum_depth == 0 { return Ok(None); }
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..RetainedCloneProgress::default() })?;
            self.complete = Some(FrameDirectives { wheel_zoom_deadline_cleared: self.wheel_zoom_deadline_cleared });
            cx.consume_fuel(1);
            return Ok(None);
        }
        JobOutcomeBorrow::admit_complete(cx, None, None).map_err(OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, original: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match original.kind() {
            JobOutcomeKind::Cancelled => original.cancelled(),
            _ => original.complete(None, None),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.complete.is_none() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let grant = cx.retained_grant();
        let copy_bytes = size_of::<FrameDirectives>();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_copy_bytes < copy_bytes {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        if grant.maximum_depth == 0 {
            return Err(OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original frame directive close requires admitted depth"), cx));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..RetainedCloneProgress::default() };
        cx.consume_retained(progress)?;
        self.complete = None;
        cx.consume_fuel(1);
        Ok(InteractiveJobCloseStep::Pending { progress })
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

//#region 📮️FrameBuildHandle
/// 📮️ The non-blocking poll contract: never waits on the worker. One frame build is admitted at a time
/// under the initiating caller; a still-running build is left alone (not cancelled) and re-checked next
/// call rather than admitting a second overlapping one.
pub struct FrameBuildHandle {
    session: Option<semio_framework_job_worker::WorkerJobSession<ActiveFrameBuild>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    original_job: Option<ActiveFrameBuild>,
    original_params: Option<BatchJobParams>,
    original_params_cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    original_outcome: Option<semio_framework_job_worker::WorkerJobOutcome<ActiveFrameBuild>>,
    original_outcome_acknowledged: bool,
    original_outcome_kind: Option<JobOutcomeKind>,
    original_refused_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    original_presentation_generation: Option<Generation>,
    closing: bool,
}

enum ActiveFramePhase {
    DeadlineAdmission { job: Option<FrameBuildJob>, params: Option<BatchJobParams>, cancel_retirement: Option<semio_framework_async::CancelTokenRetirement> },
    Deadlines(BatchJobSession<FrameBuildJob>),
    ApplyPending(FrameDirectives),
    Build(semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction),
    Prepare(semio_framework_os_renderer_wgpu_app_runtime::AppFramePreparation),
    Terminal,
}

struct ActiveFrameBuild {
    retained: RetainedCloneGrant,
    runtime: semio_framework_os_renderer_wgpu_app_runtime::AppMailbox,
    handle: semio_framework_os_renderer_wgpu_app_runtime::AppHandle,
    operation: OperationId,
    generation: Generation,
    cancel: std::mem::ManuallyDrop<Option<CancelToken>>,
    cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    presentation_generation: Generation,
    preview_sequence: u64,
    phase: ActiveFramePhase,
    retiring_phase: Option<ActiveFramePhase>,
    deadline_receipt_received: bool,
    deadline_outcome_pending: Option<bool>,
    deadline_acknowledged: bool,
    deadline_refused_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    overruns: semio_framework_trace::StepOverrunLedger,
    completed: Option<semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation>,
    closing: bool,
}

enum ActiveFrameStep {
    Pending,
    Fault(semio_framework_value::ValueError),
    CloseFailure(OriginalTurnFailure),
    Complete(Option<semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation>),
}

fn run_frame_owner_turn(cx: &mut StepContext<'_>, advance: impl FnOnce(&mut StepContext<'_>) -> ActiveFrameStep) -> Option<ActiveFrameStep> {
    if cx.should_yield() {
        return None;
    }
    let step = advance(cx);
    cx.consume_fuel(1);
    Some(step)
}

fn retire_active_phase(phase: &mut ActiveFramePhase, cx: &mut StepContext<'_>, witness: &CancelToken) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let grant=cx.retained_grant();
    if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
    let empty = match phase {
        ActiveFramePhase::DeadlineAdmission { job, params, cancel_retirement } => {
            if params.is_some() {
                if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
                let original=params.take().expect("original deadline params remain held before paid transfer");
                *cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(original.cancel));
                return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx);
            }
            if let Some(original)=cancel_retirement.as_mut() {
                if !original.terminal_is_empty() {
                    return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(match original.return_alias_step(witness,grant) {
                        Ok(step)=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),
                        Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(InteractiveJobCloseStep::Blocked),
                        Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error),
                    },cx);
                }
                if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
                *cancel_retirement=None;
                return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx);
            }
            if let Some(original)=job.as_mut() {
                original.begin_close();
                if !original.terminal_is_empty(){return semio_framework_os_renderer_wgpu_app_runtime::frame_received_child(original.close_step(cx),grant,original.terminal_is_empty())}
                if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
                *job=None;
                return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx);
            }
            true
        }
        ActiveFramePhase::Deadlines(session) => {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            match session.has_original_cancel_alias_witness(witness) {
                Ok(true)=>return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(match session.return_original_cancel_alias_step(witness,grant) {
                    Ok(Some(step))=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),
                    Ok(None)|Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>Ok(InteractiveJobCloseStep::Blocked),
                    Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>Err(error),
                },cx),
                Ok(false)=>{},
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(InteractiveJobCloseStep::Blocked),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),
            }
            if !session.terminal_is_empty() { let step=session.close_step(cx)?;return semio_framework_os_renderer_wgpu_app_runtime::frame_worker_child_step(step,grant,session.terminal_is_empty()).map_err(OriginalTurnFailure::from); }
            true
        }
        ActiveFramePhase::ApplyPending(_) => true,
        ActiveFramePhase::Build(transaction) => {
            if !transaction.terminal_is_empty() { return semio_framework_os_renderer_wgpu_app_runtime::frame_received_child(transaction.close_step(cx),grant,transaction.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::Prepare(preparation) => {
            if !preparation.terminal_is_empty() { return semio_framework_os_renderer_wgpu_app_runtime::frame_received_child(preparation.close_step(cx,Some(witness)), grant, preparation.terminal_is_empty()); }
            true
        }
        ActiveFramePhase::Terminal => return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }),
    };
    if empty {
        let bytes = size_of::<ActiveFramePhase>();
        if let Some(step) = semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant, bytes).map_err(|error|OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
        *phase = ActiveFramePhase::Terminal;
        return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(bytes),cx);
    }
    Ok(InteractiveJobCloseStep::Blocked)
}

impl ActiveFrameBuild {
    fn new(runtime: semio_framework_os_renderer_wgpu_app_runtime::AppMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, cancel: CancelToken, retained: RetainedCloneGrant) -> Self {
        let handle = runtime.downgrade();
        let phase = ActiveFramePhase::DeadlineAdmission { job:Some(FrameBuildJob::new(inputs)),params:None,cancel_retirement:None };
        Self { retained, runtime, handle, operation, generation, cancel:std::mem::ManuallyDrop::new(Some(cancel)), cancel_retirement:None, presentation_generation:generation, preview_sequence: 0, phase, retiring_phase:None, deadline_receipt_received:false, deadline_outcome_pending:None, deadline_acknowledged:false, deadline_refused_receipt:None, overruns: semio_framework_trace::StepOverrunLedger::new(), completed: None, closing: false }
    }

    fn cancel(&self) {
        if let Some(cancel)=self.cancel.as_ref(){cancel.cancel_now();}
    }

    fn retire_cancelled_phase(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let grant=cx.retained_grant();
        if let Some(phase)=self.retiring_phase.as_mut(){
            let step=retire_active_phase(phase,cx,self.cancel.as_ref().unwrap())?;
            if !matches!(step,InteractiveJobCloseStep::Complete{progress} if progress==RetainedCloneProgress::default()){return semio_framework_os_renderer_wgpu_app_runtime::frame_received_child(Ok(step),grant,matches!(phase,ActiveFramePhase::Terminal))}
            if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(cx.retained_grant(),0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.retiring_phase=None;
            return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx);
        }
        if matches!(self.phase,ActiveFramePhase::Terminal) && (self.deadline_receipt_received || self.deadline_outcome_pending.is_some() || self.deadline_acknowledged || self.deadline_refused_receipt.is_some()){
            if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.deadline_receipt_received=false;self.deadline_outcome_pending=None;self.deadline_acknowledged=false;self.deadline_refused_receipt=None;
            return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx);
        }
        let candidate_returned = match &mut self.phase {
            ActiveFramePhase::Build(transaction) => transaction.discard_presented_input_candidate(&self.runtime),
            ActiveFramePhase::Prepare(preparation) => preparation.discard_presented_input_candidate(&self.runtime),
            _ => true,
        };
        if !candidate_returned {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        retire_active_phase(&mut self.phase, cx, self.cancel.as_ref().unwrap())
    }

    /// 🛑️ Terminates the frame for an overrun its own `StepOverrunLedger` attributed to the step —
    /// an unusable clock reading, or `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive over-ceiling
    /// phases. One over-ceiling WALL reading on a browser worker sharing a core is recorded by the
    /// watchdog and never reaches here.
    fn quarantine_overrun(&self, site: &'static str) {
        self.runtime.record_frame_fault(site);
        self.cancel.as_ref().unwrap().cancel_now();
    }

    /// 📮️ One runtime-mailbox opportunity, priced by the same interactive ceiling every frame phase is.
    ///
    /// ⚖️ Returns whether a completion was applied, and quarantines on its own overrun exactly as the
    /// phase that used to own this call did.
    fn pump_runtime_mailbox_step(&mut self,cx:&mut StepContext<'_>) -> Result<bool,OriginalTurnFailure> {
        let watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.apply_pending", self.operation, self.generation, InteractiveStage::InteractiveStep);
        if !watchdog.is_admitted() {
            self.quarantine_overrun("os_renderer.frame.apply_pending has no monotonic clock");
            return Ok(true);
        }
        let applied = self.runtime.apply_pending_step(cx)?;
        if self.overruns.admit(&watchdog.finish()).is_terminal() {
            self.quarantine_overrun("os_renderer.frame.apply_pending overran the interactive ceiling");
            return Ok(true);
        }
        Ok(applied)
    }

    /// 📮️ The mailbox is pumped at EVERY advance, not only in `ApplyPending`.
    ///
    /// 🩸️ It used to be one phase, visited once per build: the build drained the queue, then walked
    /// on. A transaction that parked on `!interaction_available()` therefore held the one live
    /// session forever — no new build could be admitted (`FrameBuildHandle::poll_runtime_on_original_caller`
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
        if self.deadline_refused_receipt.is_some(){return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original deadline worker receipt remains refused"))}
        if let Some(phase)=self.retiring_phase.as_mut() {
            let step=match retire_active_phase(phase,cx,self.cancel.as_ref().unwrap()){Ok(step)=>step,Err(error)=>return ActiveFrameStep::CloseFailure(error)};
            if matches!(step,InteractiveJobCloseStep::Complete{..}) {
                let remaining=cx.retained_grant();
                if step.progress()!=RetainedCloneProgress::default() || remaining.maximum_items==0 || remaining.maximum_depth==0 { return ActiveFrameStep::Pending; }
                if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){return ActiveFrameStep::Fault(error)}
                self.retiring_phase=None;
            }
            return ActiveFrameStep::Pending;
        }
        if self.cancel.as_ref().unwrap().is_cancelled_now() {
            let step = match self.retire_cancelled_phase(cx){Ok(step)=>step,Err(error)=>return ActiveFrameStep::CloseFailure(error)};
            if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
                self.phase = ActiveFramePhase::Terminal;
                return ActiveFrameStep::Complete(None);
            }
            return ActiveFrameStep::Pending;
        }
        match self.pump_runtime_mailbox_step(cx){Ok(true)=>return ActiveFrameStep::Pending,Ok(false)=>{},Err(original)=>return ActiveFrameStep::CloseFailure(original)}
        if matches!(self.phase, ActiveFramePhase::ApplyPending(_)) {
            let directives = match &mut self.phase {
                ActiveFramePhase::ApplyPending(directives) => std::mem::take(directives),
                _ => return ActiveFrameStep::Pending,
            };
            self.phase = ActiveFramePhase::Build(semio_framework_os_renderer_wgpu_frame_transaction::FrameTransaction::new(directives, self.operation, self.presentation_generation, self.retained));
            return ActiveFrameStep::Pending;
        }
        match &mut self.phase {
            ActiveFramePhase::DeadlineAdmission { job, params, .. } => {
                if params.is_none() {
                    if self.operation!=cx.operation()||self.generation!=cx.generation()||!self.cancel.as_ref().unwrap().is_same_node(cx.original_cancel_token()){return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame deadline requires its admitted business identity and cancellation source"))}
                    let Some(now)=cx.latest_us()else{return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original frame deadline requires its initiating clock"))};
                    let cancel=match cx.admit_original_cancel_alias(){Ok(Some(cancel))=>cancel,Ok(None)=>return ActiveFrameStep::Pending,Err(error)=>return ActiveFrameStep::Fault(error)};
                    let original=BatchJobParams{operation:self.operation,generation:self.generation,cancel,now_us:cx.clock_source(),config:BatchDriveConfig{retained:cx.retained_grant(),site:"os_renderer_frame_deadlines",stage:InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:cx.deadline_us().saturating_sub(now)}};
                    *params=Some(original);
                    return ActiveFrameStep::Pending;
                }
                match BatchJobSession::try_admit_owned(job,params,cx) {
                    Ok(Some((session,_already_received)))=>self.phase=ActiveFramePhase::Deadlines(session),
                    Ok(None)=>{},
                    Err(error)=>return ActiveFrameStep::Fault(error),
                }
                ActiveFrameStep::Pending
            }
            ActiveFramePhase::Deadlines(session) => {
                if let Some(complete)=self.deadline_outcome_pending {
                    if !self.deadline_acknowledged {
                        let step=session.acknowledge_outcome(cx.retained_grant());
                        if let Err(error)=cx.consume_retained(step.progress()){return ActiveFrameStep::Fault(error)}
                        self.deadline_acknowledged=matches!(step,semio_framework_value::RetainedCloneStep::Complete(_));
                        return ActiveFrameStep::Pending;
                    }
                    if !complete {
                        if session.resume().is_err(){return ActiveFrameStep::Pending}
                        self.deadline_receipt_received=false;self.deadline_outcome_pending=None;self.deadline_acknowledged=false;
                        return ActiveFrameStep::Pending;
                    }
                    let grant=cx.retained_grant();
                    if grant.maximum_items==0 || grant.maximum_depth==0 { return ActiveFrameStep::Pending; }
                    let Some(directives)=session.checked_out_job_mut().and_then(FrameBuildJob::take_directives)else{return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original deadline candidate is absent after paid ACK"))};
                    if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){return ActiveFrameStep::Fault(error)}
                    session.begin_close();
                    self.retiring_phase=Some(std::mem::replace(&mut self.phase,ActiveFramePhase::ApplyPending(directives)));
                    return ActiveFrameStep::Pending;
                }
                if !self.deadline_receipt_received {
                    let expected=cx.retained_grant();let before=cx.retained_progress();
                    let poll=match session.step_on_original_caller(cx){Ok(poll)=>poll,Err(failure)=>return ActiveFrameStep::CloseFailure(failure)};
                    if !matches!(poll,semio_framework_job_worker::WorkerJobPoll::Outcome|semio_framework_job_worker::WorkerJobPoll::Terminal)||!session.checkout_outcome(){return ActiveFrameStep::Pending}
                    let Some((issued,progress))=session.take_checked_out_retained_step_receipt()else{return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original deadline worker omitted its checked-out physical receipt"))};
                    self.deadline_receipt_received=true;
                    let after=cx.retained_progress();
                    let posted=before.checked_add(progress).is_ok_and(|total|total==after);
                    if issued!=expected||!progress.fits(issued)||!posted{self.deadline_refused_receipt=Some((issued,progress));return ActiveFrameStep::Fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original deadline worker tuple differs from its already posted original caller receipt"))}
                    return ActiveFrameStep::Pending;
                }
                match session.checked_out_outcome() {
                    Ok(Some(JobOutcomeView::Complete { .. }))=>self.deadline_outcome_pending=Some(true),
                    Ok(None|Some(JobOutcomeView::Yield { .. }))=>self.deadline_outcome_pending=Some(false),
                    Ok(Some(_))=>self.cancel.as_ref().unwrap().cancel_now(),
                    Err(error)=>return ActiveFrameStep::Fault(error),
                }
                ActiveFrameStep::Pending
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
                    if let semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::Complete(frame) = transaction_step {
                        let preparation = frame.into_preparation();
                        self.phase = ActiveFramePhase::Prepare(preparation);
                    }
                    self.quarantine_overrun("os_renderer.frame.transaction overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match transaction_step {
                    semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::Complete(frame) => {
                        self.phase = ActiveFramePhase::Prepare(frame.into_preparation());
                        ActiveFrameStep::Pending
                    }
                    semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::Pending => ActiveFrameStep::Pending,
                    semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::CloseFailure(failure)=>ActiveFrameStep::CloseFailure(failure),
                    semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::Superseded => {
                        if !transaction.discard_presented_input_candidate(&self.runtime) {
                            return ActiveFrameStep::Pending;
                        }
                        self.phase = ActiveFramePhase::Terminal;
                        ActiveFrameStep::Complete(None)
                    }
                    semio_framework_os_renderer_wgpu_frame_transaction::AppFrameTransactionStep::Fault => {
                        self.cancel.as_ref().unwrap().cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Prepare(preparation) => {
                match preparation.drive_step(cx) {
                    Ok(semio_framework_os_renderer_wgpu_app_runtime::FramePreparationStep::Complete)=>match preparation.take_presentation(cx) {
                        Ok(Some(frame))=>{self.completed=Some(frame);self.retiring_phase=Some(std::mem::replace(&mut self.phase,ActiveFramePhase::Terminal));ActiveFrameStep::Pending},
                        Ok(None)=>ActiveFrameStep::Pending,
                        Err(error)=>ActiveFrameStep::Fault(error),
                    },
                    Ok(semio_framework_os_renderer_wgpu_app_runtime::FramePreparationStep::Pending)=>ActiveFrameStep::Pending,
                    Ok(semio_framework_os_renderer_wgpu_app_runtime::FramePreparationStep::Cancelled|semio_framework_os_renderer_wgpu_app_runtime::FramePreparationStep::Fault)=>{self.cancel.as_ref().unwrap().cancel_now();ActiveFrameStep::Pending},
                    Err(error)=>ActiveFrameStep::Fault(error),
                }
            }
            ActiveFramePhase::Terminal => ActiveFrameStep::Complete(self.completed.take()),
        }
    }
}

impl InteractiveJob for ActiveFrameBuild {
    /// ⏱️ One retained Worker turn advances exactly one frame phase and charges that attempt after it
    /// runs, including a terminal attempt. Native resubmits the retained owner through the process
    /// pool; browser wasm reschedules it inside the dedicated frame Worker.
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if matches!(self.phase, ActiveFramePhase::Terminal) && self.retiring_phase.is_none() {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(OriginalTurnFailure::from);
        }
        if cx.is_cancelled() { self.cancel(); }
        match run_frame_owner_turn(cx, |cx| self.advance(cx)) {
            None | Some(ActiveFrameStep::Pending) => Ok(None),
            Some(ActiveFrameStep::Fault(error)) => Err(OriginalTurnFailure::from(error)),
            Some(ActiveFrameStep::CloseFailure(error)) => Err(error),
            Some(ActiveFrameStep::Complete(frame)) => {
                self.completed = frame;
                Ok(None)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, original: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        original.complete(None, None)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.begin_close();
        let grant=cx.retained_grant();
        if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if let Some(frame) = self.completed.as_mut() {
            if !frame.discard_presented_input_candidate(&self.runtime) { return Ok(InteractiveJobCloseStep::Blocked); }
            if !frame.terminal_is_empty() { return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_child_step(frame.close_step(grant), grant, frame.terminal_is_empty()),cx); }
            let bytes = size_of::<Option<semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation>>();
            if let Some(step) = semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant, bytes).map_err(|error|OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.completed = None;
            return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(bytes),cx);
        }
        if !matches!(self.phase,ActiveFramePhase::Terminal)||self.retiring_phase.is_some()||self.deadline_receipt_received||self.deadline_outcome_pending.is_some()||self.deadline_acknowledged||self.deadline_refused_receipt.is_some(){return self.retire_cancelled_phase(cx)}
        if let Some(original)=self.cancel.take(){
            if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{*self.cancel=Some(original);return Ok(step)}
            self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(original));return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx)
        }
        if let Some(original)=self.cancel_retirement.as_mut(){
            if !original.terminal_is_empty(){return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(match original.return_alias_step(cx.original_cancel_token(),grant){Ok(step)=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(InteractiveJobCloseStep::Blocked),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)},cx)}
            if let Some(step)=semio_framework_os_renderer_wgpu_app_runtime::frame_inline_gate(grant,0).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.cancel_retirement=None;return semio_framework_os_renderer_wgpu_app_runtime::frame_receive_close(semio_framework_os_renderer_wgpu_app_runtime::frame_inline_progress(0),cx)
        }
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.cancel.is_none() && self.cancel_retirement.is_none() && self.completed.is_none() && self.retiring_phase.is_none() && !self.deadline_receipt_received && self.deadline_outcome_pending.is_none() && !self.deadline_acknowledged && self.deadline_refused_receipt.is_none() && matches!(self.phase, ActiveFramePhase::Terminal)
    }
}

fn generation_is_fresh(requested: Generation, completed: Generation) -> bool {
    requested == completed
}

impl FrameBuildHandle {
    /// 🎟️ Admits and advances the existing frame owner with the initiating business authority.
    pub fn poll_runtime_on_original_caller(&mut self,runtime:semio_framework_os_renderer_wgpu_app_runtime::AppMailbox,inputs:FrameBuildInputs,presentation_generation:Generation,cx:&mut StepContext<'_>)->Result<Option<semio_framework_os_renderer_wgpu_app_runtime::AppFramePresentation>,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        if self.original_refused_receipt.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame receiver retains its refused physical tuple").into())}
        if self.original_presentation_generation.is_some_and(|original|original!=presentation_generation){if let Some(session)=self.session.as_ref(){let _=session.begin_close();}else{return Ok(None)}}
        if let Some(owner)=self.original_outcome.as_mut(){
            if !self.original_outcome_acknowledged{let progress=owner.acknowledge_outcome(cx.retained_grant());cx.consume_retained(progress.progress())?;cx.consume_fuel(progress.progress().copied_items as u64);self.original_outcome_acknowledged=matches!(progress,semio_framework_job::RetainedCloneStep::Complete(_));return Ok(None)}
            let kind=self.original_outcome_kind;let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);
            let mut owner=self.original_outcome.take().unwrap();self.original_outcome_acknowledged=false;self.original_outcome_kind=None;
            if kind==Some(JobOutcomeKind::Yield){match owner.resume(){Ok(())=>self.ticket=None,Err(owner)=>self.original_outcome=Some(owner)}return Ok(None)}
            let frame=if kind==Some(JobOutcomeKind::Complete){owner.job_mut().ok().and_then(|job|if generation_is_fresh(presentation_generation,job.presentation_generation){job.completed.take()}else{None})}else{None};owner.begin_close();self.ticket=None;return Ok(frame)
        }
        if let Some(session)=self.session.as_mut(){
            if self.closing||cx.is_cancelled(){let _=session.begin_close();}
            match session.poll(){
                semio_framework_job_worker::WorkerJobPoll::Idle=>{
                    let before=cx.retained_progress();let expected=cx.retained_grant();let Some((ticket,poll))=session.try_step_on_original_caller(cx)?else{return Ok(None)};if poll==semio_framework_job_worker::WorkerJobPoll::Idle{return Ok(None)}
                    let mut owner=if poll==semio_framework_job_worker::WorkerJobPoll::Terminal{session.take_terminal()}else{session.take_outcome(ticket)}.map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame callback lost its retained outcome"))?;
                    let tuple=owner.take_retained_step_receipt();self.original_outcome_kind=owner.outcome_descriptor().map(JobOutcomeDescriptor::kind);self.ticket=Some(ticket);self.original_outcome=Some(owner);
                    let Some((issued,progress))=tuple else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame callback omitted its physical receipt").into())};
                    if issued!=expected||!progress.fits(issued)||!before.checked_add(progress).is_ok_and(|posted|posted==cx.retained_progress()){self.original_refused_receipt=Some((issued,progress));return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame callback receipt differs from its already posted initiating recipient").into())}
                },
                semio_framework_job_worker::WorkerJobPoll::Closing=>{
                    let grant=cx.retained_grant();match session.has_original_cancel_alias_witness(cx.original_cancel_token()){
                        Ok(true)=>match session.return_original_cancel_alias_step(cx.original_cancel_token(),grant){Ok(Some(step))=>{cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64)},Ok(None)|Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>{},Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx))},
                        Ok(false)=>{session.close_step(cx)?;},Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>{},Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),
                    }
                },
                semio_framework_job_worker::WorkerJobPoll::TerminalEmpty=>{let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_depth>0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.session=None;self.original_presentation_generation=None;}},
                _=>{},
            }
            return Ok(None)
        }
        if self.closing||cx.is_cancelled(){return Ok(None)}
        if self.original_job.is_none(){let Some(cancel)=cx.admit_original_cancel_alias()?else{return Ok(None)};let mut job=ActiveFrameBuild::new(runtime,inputs,cx.operation(),cx.generation(),cancel,cx.retained_grant());job.presentation_generation=presentation_generation;self.original_job=Some(job);self.original_presentation_generation=Some(presentation_generation);return Ok(None)}
        if self.original_params.is_none(){let Some(now)=cx.latest_us()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original frame admission requires its initiating clock").into())};let Some(cancel)=cx.admit_original_cancel_alias()?else{return Ok(None)};self.original_params=Some(BatchJobParams{operation:cx.operation(),generation:cx.generation(),cancel,now_us:cx.clock_source(),config:BatchDriveConfig{retained:cx.retained_grant(),site:"os_renderer_frame_build",stage:InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:cx.deadline_us().saturating_sub(now)}});return Ok(None)}
        if let Some((session,_already_posted))=semio_framework_job_worker::WorkerJobSession::try_admit_owned(&mut self.original_job,&mut self.original_params,cx)?{self.session=Some(session)}Ok(None)
    }

    /// ♻️ Returns pending admission and checked-out custody before closing the same admitted session.
    pub fn close_original_frame_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.closing=true;
        if !self.retire_original_frame_step(cx)?{return Ok(false)}
        Ok(true)
    }

    /// 🪢 Retires one frame owner unit under the initiating caller so a component-close bridge created by that frame can run.
    /// Unlike [`Self::close_original_frame_step`], this keeps the handle reusable and preserves its installed wake.
    pub fn retire_for_component_surface_close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        Ok(self.retire_original_frame_step(cx)?&&!self.has_live_session())
    }

    fn retire_original_frame_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        if let Some(owner)=self.original_outcome.as_mut(){let step=owner.acknowledge_outcome(grant);cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);if matches!(step,semio_framework_job::RetainedCloneStep::Complete(_)){self.original_outcome.take().unwrap().begin_close();self.original_outcome_kind=None;self.original_outcome_acknowledged=false;}return Ok(false)}
        if let Some(job)=self.original_job.as_mut(){if !job.terminal_is_empty(){job.close_step(cx)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_job=None;return Ok(false)}
        if let Some(params)=self.original_params.take(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_params_cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));return Ok(false)}
        if let Some(owner)=self.original_params_cancel_retirement.as_mut(){if !owner.terminal_is_empty(){match owner.return_alias_step(cx.original_cancel_token(),grant){Ok(step)=>{cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64)},Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>{},Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx))}return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_params_cancel_retirement=None;return Ok(false)}
        if let Some(session)=self.session.as_mut(){let _=session.begin_close();if !session.terminal_is_empty(){match session.has_original_cancel_alias_witness(cx.original_cancel_token()){
            Ok(true)=>match session.return_original_cancel_alias_step(cx.original_cancel_token(),grant){Ok(Some(step))=>{cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64)},Ok(None)|Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>{},Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx))},
            Ok(false)=>{session.close_step(cx)?;},Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>{},Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),
        }return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.session=None;self.ticket=None;return Ok(false)}
        if self.original_refused_receipt.is_some()||self.original_presentation_generation.is_some(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_refused_receipt=None;self.original_presentation_generation=None;return Ok(false)}
        Ok(true)
    }

    pub fn new() -> Self {
        Self {
            session: None,
            ticket: None,
            original_job:None,
            original_params:None,
            original_params_cancel_retirement:None,
            original_outcome:None,
            original_outcome_acknowledged:false,
            original_outcome_kind:None,
            original_refused_receipt:None,
            original_presentation_generation:None,
            closing: false,
        }
    }



    /// 🧵️ Whether a frame build is admitted right now — the one owner that must keep its own frames
    /// coming and must not have its inputs renumbered underneath it.
    pub fn has_live_session(&self) -> bool {
        self.session.is_some() || self.original_job.is_some() || self.original_params.is_some() || self.original_outcome.is_some()
    }

    pub fn original_event_terminal_is_empty(&self)->bool{!self.has_live_session()&&self.original_params_cancel_retirement.is_none()&&self.original_outcome_kind.is_none()&&self.original_refused_receipt.is_none()&&self.original_presentation_generation.is_none()}

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.original_event_terminal_is_empty()
    }
}

impl Default for FrameBuildHandle {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 📮️FrameBuildHandle

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs"]
mod frame_job_unit_tests;

#[cfg(test)]
#[path = "📬️outcome/🧪️tests/🦀️.rs"]
mod original_outcome_tests;
