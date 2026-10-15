//! 👷️ Canonical retained worker-session ownership and its private nominal custody.
use semio_framework_job::*;
use semio_framework_job_worker_authority::{OriginalWorkerJobAuthority,OriginalWorkerAuthorityPhase,WorkerJobSource,WorkerJobAdmissionControl,WorkerJobAdmissionContext,WorkerJobInitializer,WorkerJobInitializationStep,BatchDriveConfig,BatchJobParams,WorkerJobCloseStep,receive_original_worker_close_effects,receive_original_worker_close_result};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool,AtomicPtr,AtomicU8,AtomicU32,AtomicU64,AtomicUsize,Ordering};
use std::sync::Arc;
use std::task::Waker;
#[cfg(test)]
macro_rules! admit_original_fixture_owner {
 ($caller:ident,$owner:ident,$job:expr,$params:expr $(,)?)=>{{
  let mut original_job=Some($job);let mut original_params=Some($params);let grant=$caller.remaining_grant();
   $caller.turn(grant,|cx|{let before=cx.retained_progress();match $owner::try_admit_owned(&mut original_job,&mut original_params,cx)?{Some((owner,progress))=>{assert!(original_job.is_none()&&original_params.is_none());assert_eq!(before.checked_add(progress).unwrap(),cx.retained_progress());Ok(owner)},None=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture source retains its unchanged admission sources").into())}})
 }};
}
#[cfg(test)]
const WORKER_TEST_STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:20000,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:128};

fn turn(demand:RetirementDemand,grant:RetainedCloneGrant,owner:impl FnOnce(RetainedCloneGrant)->Result<(RetainedCloneStep,bool),ValueError>)->Result<RetainedCloneStep,ValueError>{semio_framework_value::advance_retirement_turn(demand,grant,owner).map_err(|error|match error{semio_framework_value::RetirementTurnError::Owner(error)|semio_framework_value::RetirementTurnError::Receipt(error)=>error})}

#[path="../♻️session-return/🦀️.rs"]
mod session_return;

#[path="🚪️admission/🦀️.rs"]
mod original_worker_admission;
pub use original_worker_admission::{WorkerJobAdmission,WorkerJobSessionPreparation};
#[path="🔀️erasure/🦀️.rs"]
mod original_session_erasure;
pub use original_session_erasure::{OriginalBatchSessionErasure,ErasedBatchJobSession,OriginalPreviewSession};


#[path="../♻️retirement/🔔️wake/🦀️.rs"]
mod worker_wake_retirement;
pub use worker_wake_retirement::{RetainedWorkerWake,OriginalWorkerWakeIssuer,admit_original_worker_wake,original_worker_wake_admission_demands};
#[cfg(not(target_arch="wasm32"))]
pub use worker_wake_retirement::admit_original_thread_worker_wake;
//#region 🏭️RetainedSessions

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerJobTicket {
    pub generation: Generation,
    pub step_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobContention {
    Submitted(WorkerJobTicket),
    Outcome(WorkerJobTicket),
    Terminal(WorkerJobTicket),
    Rejected(Generation),
    CheckedOut(Generation),
    Closing(Generation),
    WakeExhausted(Generation),
    TerminalEmpty,
}

/// 🧭️ Keeps contention separate from a refused original ownership demand.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum WorkerJobDemandError {Contention(WorkerJobContention),Refused(ValueError)}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobSubmitFault {
    Contention(WorkerJobContention),
    Pool(semio_framework_async::WorkerSubmitErrorKind),
    SequenceExhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobPoll {
    Idle,
    Submitted,
    Outcome,
    Terminal,
    Rejected,
    CheckedOut,
    Closing,
    TerminalEmpty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobTakeFault {
    Pending,
    Stale,
    WrongPhase,
}

/// 🔭️ Which named phase a worker session's bounded close cursor is parked in, read without
/// spending a turn.
///
/// `close_step` reports HOW MUCH a turn released, never WHERE the cursor is, so a close that
/// stops advancing is indistinguishable from one that is merely slow: both answer
/// `Pending { 0, 0 }`. A host draining a session on a short grant, and every law that asserts a
/// session reaches terminal, needs the phase name to tell "the job is still releasing its own
/// owners" from "the cursor cannot leave the pre-admitted fault page" — the two that looked
/// identical while the mounted presence capture law spun 4096 times (ticket 26/09/18).
///
/// The phases are the exact ladder [`WorkerJobSession::close_step`] walks, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobClosePhase {
    OriginalFailure,
    /// 🚦️ Not closing: the session is idle, submitted, or holding an outcome nobody closed.
    Open,
    /// 🤝️ The authority is checked out, so the cursor has not been handed the session yet.
    CheckedOut,
    /// ☣️ Releasing the quarantined outcome's retained payload pages.
    QuarantinedOutcome,
    /// 📦️ Releasing the checked-out step outcome's retained payload pages.
    Outcome,
    /// 🔔️ The turn that hands `begin_close` to the job itself.
    BeginClose,
    /// 🧨️ Releasing the session's pre-admitted terminal-fault page.
    PreadmittedFault,
    /// 🧩️ The job is releasing its own owners through `InteractiveJob::close_step`.
    Job,
    /// 🗑️ Dropping the released job.
    JobRelease,
    /// 🎛️ Dropping the batch parameters.
    ParamsRelease,
    /// 📒️ Waiting on the payload ledger's outstanding stream credits.
    PayloadLedger,
    /// 🧺️ Releasing the original worker authority backing and its separately owned ledger.
    AuthorityRelease,
    /// 🎟️ Returning the pre-admitted retirement slot.
    RetirementSlot,
    SessionArc,
    /// 🕳️ Terminal-empty: every owner is released and the slot is returned.
    Empty,
}

pub struct BatchJobSession<J: InteractiveJob + 'static> {
    session: WorkerJobSession<J>,
    ticket: Option<WorkerJobTicket>,
    checked_out: Option<WorkerJobOutcome<J>>,
}

pub enum MountedWorkerJobPumpFault {
    Submit(WorkerJobSubmitFault),
    Take(WorkerJobTakeFault),
    MissingTicket,
    CheckedOut,
}

pub struct MountedWorkerJobSession<J: InteractiveJob + 'static> {
    session: WorkerJobSession<J>,
    ticket: Option<WorkerJobTicket>,
    checked_out: Option<WorkerJobOutcome<J>>,
}

impl<J: InteractiveJob + 'static> MountedWorkerJobSession<J> {
    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    pub fn generation(&self) -> Generation {
        self.session.generation()
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// 🧵️ `J: Send` for the same reason as [`WorkerJobSession::try_submit_step`] — this is the
    /// pool-submitting half of the mounted session, not the single-threaded drive path.
    pub fn pump_one(&mut self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault>
    where
        J: Send,
    {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        match self.session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                if lane == Lane::Interactive {
                    let (ticket, poll) = self
                        .session
                        .try_step_on_caller(retained)
                        .map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
                    self.ticket = Some(ticket);
                    return match poll {
                        WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => self.pump_one(pool, lane,retained),
                        other => Ok(other),
                    };
                }
                let ticket = self.session.try_submit_step(pool, lane,retained).map_err(MountedWorkerJobPumpFault::Submit)?;
                self.ticket = Some(ticket);
                Ok(WorkerJobPoll::Submitted)
            }
            WorkerJobPoll::Submitted => Ok(WorkerJobPoll::Submitted),
            WorkerJobPoll::Outcome => {
                let ticket = self.ticket.take().ok_or(MountedWorkerJobPumpFault::MissingTicket)?;
                let owner = self.session.take_outcome(ticket).map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Outcome)
            }
            WorkerJobPoll::Terminal => {
                let owner = self.session.take_terminal().map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Terminal)
            }
            WorkerJobPoll::Rejected => {
                let owner = self.session.take_rejected().map_err(MountedWorkerJobPumpFault::Take)?;
                owner.resume();
                Ok(WorkerJobPoll::Rejected)
            }
            poll => Ok(poll),
        }
    }

    /// 🧵️ Runs the session's next step on the calling thread and checks its outcome out. For a caller
    /// that already owns a bounded slice of its own — a guest `semio.infer` `step-job` crossing —
    /// where handing the step to a pool and waiting for it only adds a round trip per step.
    pub fn step_on_caller(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained).map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
        if poll==WorkerJobPoll::Idle{return Ok(poll)}
        let owner = if poll == WorkerJobPoll::Terminal { self.session.take_terminal() } else { self.session.take_outcome(ticket) };
        self.checked_out = Some(owner.map_err(MountedWorkerJobPumpFault::Take)?);
        Ok(poll)
    }

    /// 🕰️ See [`WorkerJobSession::last_step_end_us`].
    pub fn last_step_end_us(&self) -> Option<u64> {
        self.session.last_step_end_us()
    }

    /// 🏃️ Executes the admitted business producer on the same original caller without renewing any budget.
    pub fn pump_one_for_interactive_turn(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobPoll,OriginalTurnFailure>{
        if self.checked_out.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker outcome must be returned before another callback").into())}
        let Some((ticket,poll))=self.session.try_step_on_original_caller(cx)?else{return Ok(WorkerJobPoll::Idle)};
        if poll==WorkerJobPoll::Idle{return Ok(poll)}
        let owner=if poll==WorkerJobPoll::Terminal{self.session.take_terminal()}else{self.session.take_outcome(ticket)};
        self.checked_out=Some(owner.map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback lost its exact checked-out producer"))?);
        Ok(poll)
    }

    /// 📥️ Returns the original effective grant and actual physical receipt once before semantic custody advances.
    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    /// 🧾️ Borrows the actual same-owner worker receipt before transferring its outcome.
    pub fn checked_out_retained_step_progress(&self)->Option<RetainedCloneProgress>{self.checked_out.as_ref().map(WorkerJobOutcome::retained_step_progress)}

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    /// 🎫️ Observes the same paid descriptor through its separate acknowledgement and removal phases.
    pub fn checked_out_outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.checked_out.as_ref().and_then(WorkerJobOutcome::outcome_descriptor)}

    /// 🛑️ Keeps the worker's original refusal inside its checked-out authority.
    pub fn checked_out_failure(&self)->Option<&OriginalTurnFailure>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_failure)}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn acknowledge_checked_out_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Complete(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        self.checked_out.as_mut().and_then(|original|original.job_mut().ok())
    }
    pub fn checked_out_job(&self)->Option<&J>{self.checked_out.as_ref().map(WorkerJobOutcome::job)}

    /// 🧮️ Inspect the exact retained owner without advancing or enlarging its close grant.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>authority.retirement_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if let Some(mut owner)=self.checked_out.take(){
            if let Some(demand)=owner.authority.as_ref().and_then(OriginalWorkerJobAuthority::outcome_demands){
                let result=close_original_retained_step(cx,demand,|grant|Ok(owner.acknowledge_outcome(grant)));self.checked_out=Some(owner);return result.map(|step|match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked})
            }
            let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if let Err(error)=cx.consume_retained(progress){self.checked_out=Some(owner);return Err(error.into())}cx.consume_fuel(1);owner.begin_close();return Ok(WorkerJobCloseStep::Pending{progress})
        }
        self.session.close_step(cx)
    }

    /// 🪢️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(false);}
        self.session.has_original_cancel_alias_witness(witness)
    }

    /// 🪢️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(None);}
        self.session.return_original_cancel_alias_step(witness,grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 🔭️ The named phase this mounted session's close cursor is parked in — see
    /// [`WorkerJobClosePhase`]. A retained checked-out outcome is the mounted half's own phase and
    /// outranks whatever the inner session reports.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.checked_out.is_some() { WorkerJobClosePhase::CheckedOut } else { self.session.close_phase() }
    }
}

impl<J: InteractiveJob + 'static> BatchJobSession<J> {
    /// 🪪️ Borrows the original parameters under the same native custody gate used by close quotes.
    pub fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{
        if let Some(owner)=self.checked_out.as_ref(){return Ok(owner.authority.as_ref().and_then(OriginalWorkerJobAuthority::source_identity))}
        if self.session.inner.0.is_none(){return Ok(None)}
        let phase=self.session.inner.phase();
        if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT|SESSION_EMPTY)||self.session.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.session.contention())}
        let authority=unsafe{self.session.inner.take_authority()};
        let identity=authority.source_identity();
        unsafe{self.session.inner.put_authority_quiet(authority,phase)};Ok(identity)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    /// 🤝️ Hands the same prepared native authority to Batch on a separate paid metadata turn.
    pub fn admit_prepared(original:&mut Option<WorkerJobSession<J>>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"prepared Batch admission requires the same idle original authority");
        let session=original.as_ref().ok_or_else(invalid)?;
        if session.inner.0.is_none(){return Err(invalid())}
        let grant=control.admission_grant()?;
        if !control.admission_is_open()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if session.inner.phase.compare_exchange(SESSION_IDLE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(invalid())}
        let authority=unsafe{session.inner.take_authority()};
        let admitted=authority.can_admit_prepared(control,session.generation)&&!session.inner.close_requested.load(Ordering::Acquire);
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        let received=if admitted{control.receive_admission(progress)}else{Err(invalid())};
        unsafe{session.inner.put_authority_quiet(authority,SESSION_IDLE)};
        received?;
        let session=original.take().ok_or_else(invalid)?;
        Ok(Some((Self{session,ticket:None,checked_out:None},progress)))
    }

    /// 🎟️ Keeps the exact original callback ticket while the caller separately receives semantic custody.
    pub fn step_on_original_caller(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobPoll,OriginalTurnFailure>{
        if self.checked_out.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original batch outcome remains checked out").into())}
        let Some((ticket,poll))=self.session.try_step_on_original_caller(cx)?else{return Ok(WorkerJobPoll::Idle)};
        if poll!=WorkerJobPoll::Idle{self.ticket=Some(ticket)}
        Ok(poll)
    }

    pub fn step(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, WorkerJobContention> {
        if self.checked_out.is_some() {
            return Err(WorkerJobContention::CheckedOut(self.session.generation()));
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained)?;
        if poll!=WorkerJobPoll::Idle{self.ticket = Some(ticket);}
        Ok(poll)
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Progress(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checkout_outcome(&mut self) -> bool {
        if self.checked_out.is_some() {
            return true;
        }
        let owner = match self.session.poll() {
            WorkerJobPoll::Outcome => match self.ticket.take().and_then(|ticket| self.session.take_outcome(ticket).ok()) {
                Some(owner) => owner,
                None => return false,
            },
            WorkerJobPoll::Terminal => match self.session.take_terminal().ok() {
                Some(owner) => owner,
                None => return false,
            },
            _ => return false,
        };
        self.checked_out = Some(owner);
        true
    }

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        WorkerJobOutcome::job_mut(self.checked_out.as_mut()?).ok()
    }

    /// 🧭️ Forward the exact owner demand while preserving checkout and worker ownership.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>authority.retirement_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if let Some(owner)=self.checked_out.take(){let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if let Err(error)=cx.consume_retained(progress){self.checked_out=Some(owner);return Err(error.into())}cx.consume_fuel(1);owner.begin_close();return Ok(WorkerJobCloseStep::Pending{progress})}
        self.session.close_step(cx)
    }

    /// 🪢️ Preserves checkout custody before inspecting the original borrowed cancellation witness.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(false)}self.session.has_original_cancel_alias_witness(witness)}
    /// 🪢️ Returns the original alias through its sole session after checkout custody has returned.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(None)}self.session.return_original_cancel_alias_step(witness,grant)}

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 🔭️ Observe the actual batch checkout or original worker retirement phase without effects.
    pub fn close_phase(&self)->WorkerJobClosePhase{if self.checked_out.is_some(){WorkerJobClosePhase::CheckedOut}else{self.session.close_phase()}}
}

const SESSION_TRANSITION: u8 = 0;
const SESSION_IDLE: u8 = 1;
const SESSION_SUBMITTED: u8 = 2;
const SESSION_OUTCOME: u8 = 3;
const SESSION_TERMINAL: u8 = 4;
const SESSION_REJECTED: u8 = 5;
const SESSION_CHECKED_OUT: u8 = 6;
const SESSION_CLOSE: u8 = 7;
const SESSION_EMPTY: u8 = 8;

pub const WORKER_JOB_SESSION_SLOTS: usize = 256;

#[repr(C)]
struct WorkerJobRetirementHeader {
    slot: usize,
    phase: unsafe fn(*mut WorkerJobRetirementHeader) -> WorkerJobClosePhase,
    pump: unsafe fn(*mut WorkerJobRetirementHeader, &mut StepContext<'_>) -> Result<WorkerJobCloseStep,OriginalTurnFailure>,
    destroy: unsafe fn(*mut WorkerJobRetirementHeader),
}

const WORKER_JOB_RETIREMENT_RESERVED: *mut WorkerJobRetirementHeader = std::ptr::without_provenance_mut(1);
static WORKER_JOB_RETIREMENT_SLOTS: [AtomicPtr<WorkerJobRetirementHeader>; WORKER_JOB_SESSION_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; WORKER_JOB_SESSION_SLOTS];
static WORKER_JOB_RETIREMENT_WAKE: AtomicBool = AtomicBool::new(false);

fn reserve_worker_job_retirement_slot() -> Option<usize> {
    WORKER_JOB_RETIREMENT_SLOTS.iter().enumerate().find_map(|(index, slot)| slot.compare_exchange(std::ptr::null_mut(), WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| index))
}

/// 🧹️ Whether any dropped worker-job session still parks a node in the process-wide retirement array.
/// A host reads this to decide that it is NOT idle: the parked node holds one of the
/// [`WORKER_JOB_SESSION_SLOTS`] admissions every later session competes for, and only
/// [`pump_worker_job_retirements`] gives it back.
pub fn worker_job_retirements_are_parked() -> bool {
    WORKER_JOB_RETIREMENT_SLOTS.iter().any(|slot| {
        let pointer = slot.load(Ordering::Acquire);
        !pointer.is_null() && pointer != WORKER_JOB_RETIREMENT_RESERVED
    })
}

pub fn take_worker_job_retirement_wake() -> bool {
    WORKER_JOB_RETIREMENT_WAKE.swap(false, Ordering::AcqRel)
}

/// 🔭️ Observes one parked original owner under exclusive slot admission without funding it.
pub fn worker_job_retirement_phase(maximum_sessions:usize)->Option<WorkerJobClosePhase>{
    let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}
        let pointer=slot.load(Ordering::Acquire);
        if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}
        inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let phase=unsafe{((*pointer).phase)(pointer)};
        slot.store(pointer,Ordering::Release);
        return Some(phase);
    }
    worker_job_retirements_are_parked().then_some(WorkerJobClosePhase::CheckedOut)
}

pub fn pump_worker_job_retirements(maximum_sessions:usize,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
    if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}let pointer=slot.load(Ordering::Acquire);if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let step=match unsafe{((*pointer).pump)(pointer,cx)}{Ok(step)=>step,Err(failure)=>{slot.store(pointer,Ordering::Release);WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);return Err(failure)}};
        if matches!(step,WorkerJobCloseStep::Complete{..}){slot.store(std::ptr::null_mut(),Ordering::Release);unsafe{((*pointer).destroy)(pointer)}}else{slot.store(pointer,Ordering::Release)}
        WORKER_JOB_RETIREMENT_WAKE.store(worker_job_retirements_are_parked(),Ordering::Release);return receive_original_worker_close_effects(cx,before,match step{WorkerJobCloseStep::Complete{progress}if worker_job_retirements_are_parked()=>WorkerJobCloseStep::Pending{progress},step=>step})
    }
    Ok(if worker_job_retirements_are_parked(){WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);WorkerJobCloseStep::Blocked}else{WorkerJobCloseStep::Complete{progress:Default::default()}})
}

pub(crate) struct WorkerJobSessionInner<J> {
    generation: Generation,
    phase: AtomicU8,
    authority: ManuallyDrop<std::cell::UnsafeCell<Option<OriginalWorkerJobAuthority<J>>>>,
    rejection_kind: AtomicU8,
    close_requested: AtomicBool,
    terminal_intent: AtomicU8,
    wake_pending: AtomicBool,
    wake_sequence: AtomicU64,
    wake_exhausted: AtomicBool,
    wake_guard: AtomicBool,
    waker: ManuallyDrop<std::cell::UnsafeCell<Option<Waker>>>,
    retained_wake:ManuallyDrop<std::cell::UnsafeCell<Option<Box<dyn RetainedWorkerWake>>>>,
    last_step_end_us: AtomicU64,
}

unsafe impl<J: Send> Send for WorkerJobSessionInner<J> {}
unsafe impl<J: Send> Sync for WorkerJobSessionInner<J> {}

impl<J> WorkerJobSessionInner<J> {
    fn phase(&self) -> u8 {
        self.phase.load(Ordering::Acquire)
    }

    unsafe fn take_authority(&self) -> OriginalWorkerJobAuthority<J> {
        unsafe { (&mut *self.authority.get()).take().expect("session phase owns exact authority") }
    }

    unsafe fn put_authority(&self, authority: OriginalWorkerJobAuthority<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
        self.raise_wake();
    }

    unsafe fn put_authority_quiet(&self, authority: OriginalWorkerJobAuthority<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
    }

    fn raise_wake(&self) {
        if self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if self.wake_sequence.try_update(Ordering::AcqRel, Ordering::Acquire, |sequence| sequence.checked_add(1)).is_err() {
            self.wake_exhausted.store(true, Ordering::Release);
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            if let Some(original)=unsafe{(&*self.retained_wake.get()).as_ref()}{original.wake_by_ref();}
            let waker = unsafe { (&mut *self.waker.get()).take() };
            self.wake_guard.store(false, Ordering::Release);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    fn register_waker(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.wake_exhausted.load(Ordering::Acquire) {
            return Err(WorkerJobContention::WakeExhausted(self.generation));
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobContention::CheckedOut(self.generation));
        }
        if unsafe{(&*self.retained_wake.get()).is_some()}{self.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        let wake_now = unsafe {
            *self.waker.get() = Some(waker.clone());
            if self.wake_pending.load(Ordering::Acquire) { (&mut *self.waker.get()).take() } else { None }
        };
        self.wake_guard.store(false, Ordering::Release);
        if let Some(waker) = wake_now {
            waker.wake();
        }
        Ok(())
    }
}

impl<J> Drop for WorkerJobSessionInner<J> {
    fn drop(&mut self) {
        if self.phase.load(Ordering::Acquire) == SESSION_EMPTY {
            unsafe {
                ManuallyDrop::drop(&mut self.authority);
                ManuallyDrop::drop(&mut self.waker);
                ManuallyDrop::drop(&mut self.retained_wake);
            }
        }
    }
}

struct WorkerJobSessionArc<J>(Option<session_return::SessionHandle<J>>);
impl<J> std::ops::Deref for WorkerJobSessionArc<J>{type Target=session_return::SessionHandle<J>;fn deref(&self)->&Self::Target{self.0.as_ref().expect("live session retains its original Arc")}}

pub struct WorkerJobSession<J: InteractiveJob + 'static> {
    inner: WorkerJobSessionArc<J>,
    generation: Generation,
    terminal_step_end_us: Option<u64>,
    retirement: std::cell::UnsafeCell<Option<Box<WorkerJobRetirementNode<J>>>>,
    retirement_state: AtomicU8,
}

unsafe impl<J: InteractiveJob + 'static> Send for WorkerJobSession<J> {}
unsafe impl<J: InteractiveJob + 'static> Sync for WorkerJobSession<J> {}

#[repr(C)]
struct WorkerJobRetirementNode<J> {
    header: WorkerJobRetirementHeader,
    inner: Option<session_return::SessionHandle<J>>,
}

fn worker_authority_phase(phase:OriginalWorkerAuthorityPhase)->WorkerJobClosePhase{match phase{OriginalWorkerAuthorityPhase::QuarantinedOutcome=>WorkerJobClosePhase::QuarantinedOutcome,OriginalWorkerAuthorityPhase::Outcome=>WorkerJobClosePhase::Outcome,OriginalWorkerAuthorityPhase::OriginalFailure=>WorkerJobClosePhase::OriginalFailure,OriginalWorkerAuthorityPhase::AdmissionPage|OriginalWorkerAuthorityPhase::Fault=>WorkerJobClosePhase::PreadmittedFault,OriginalWorkerAuthorityPhase::BeginClose=>WorkerJobClosePhase::BeginClose,OriginalWorkerAuthorityPhase::Job=>WorkerJobClosePhase::Job,OriginalWorkerAuthorityPhase::JobRelease=>WorkerJobClosePhase::JobRelease,OriginalWorkerAuthorityPhase::Parameters=>WorkerJobClosePhase::ParamsRelease,OriginalWorkerAuthorityPhase::Ledger=>WorkerJobClosePhase::PayloadLedger,OriginalWorkerAuthorityPhase::Empty=>WorkerJobClosePhase::AuthorityRelease}}

unsafe fn worker_job_retirement_node_phase<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader)->WorkerJobClosePhase{
    let node=unsafe{&*pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{return WorkerJobClosePhase::RetirementSlot};
    let phase=inner.phase();
    if phase==SESSION_EMPTY{return WorkerJobClosePhase::SessionArc}
    if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return WorkerJobClosePhase::CheckedOut}
    let authority=unsafe{inner.take_authority()};
    let result=if phase!=SESSION_CLOSE{WorkerJobClosePhase::Open}else{worker_authority_phase(authority.close_phase())};
    unsafe{inner.put_authority(authority,phase)};
    result
}

unsafe fn pump_worker_job_retirement_node<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
    let before=cx.retained_progress();let grant=cx.retained_grant();let node=unsafe{&mut *pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{let released_bytes=size_of::<WorkerJobRetirementNode<J>>();return Ok(if grant.maximum_items>0&&grant.maximum_release_bytes>=released_bytes&&grant.maximum_depth>0{WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}}else{WorkerJobCloseStep::Pending{progress:Default::default()}})};
    if matches!(worker_job_begin_close(inner),WorkerJobCloseStep::Blocked){return Ok(WorkerJobCloseStep::Blocked)}
    let step=match worker_job_close_step(inner,cx)?{
        WorkerJobCloseStep::Complete{progress}if progress!=Default::default()=>WorkerJobCloseStep::Pending{progress},
        WorkerJobCloseStep::Complete{..}=>{
            let grant=cx.retained_grant();if !inner.is_unique()||unsafe{(&*inner.waker.get()).is_some()}{return Ok(WorkerJobCloseStep::Blocked)}
            match worker_wake_retirement::close_session_retained_wake(inner,grant){Ok(Some(step))=>return receive_original_worker_close_effects(cx,before,step),Ok(None)=>{},Err(error)=>return receive_original_worker_close_result(cx,before,Err(error))}
            let released_bytes=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<WorkerJobSessionInner<J>>()).expect("worker original Arc layout").0.pad_to_align().size();
            if grant.maximum_items==0||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            match node.inner.take().unwrap().try_return(){Ok(inner)=>{drop(inner);WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}},Err(inner)=>{node.inner=Some(inner);WorkerJobCloseStep::Blocked}}
        },step=>step
    };receive_original_worker_close_effects(cx,before,step)
}

unsafe fn destroy_worker_job_retirement_node<J>(pointer: *mut WorkerJobRetirementHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<WorkerJobRetirementNode<J>>()) });
}

pub struct WorkerJobSessionAdmissionRejected<J> {
    job: ManuallyDrop<Option<J>>,
    params: ManuallyDrop<Option<BatchJobParams>>,
    fault_source: ManuallyDrop<Option<JobPayloadPageSource>>,
    closing: bool,
    close_stage: u8,
}

impl<J> WorkerJobSessionAdmissionRejected<J> {
    pub fn job(&self) -> &J {
        self.job.as_ref().expect("rejected worker session admission owns exact job")
    }

    pub fn fault_backing_identity(&self) -> Option<*const MaybeUninit<u8>> {
        self.fault_source.as_ref().map(JobPayloadPageSource::backing_identity)
    }
}

impl<J: InteractiveJob> WorkerJobSessionAdmissionRejected<J> {
    /// 🪙️ Publish the physical demand of the next retained rejection owner.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,ValueError> {
        if self.close_stage==0{
            if let Some(job)=self.job.as_ref(){if !job.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?})}}
        }
        Ok(RetirementDemand{release_bytes:if self.close_stage>1&&self.params.is_none(){self.fault_source.as_ref().map_or(0,JobPayloadPageSource::allocated_capacity_bytes)}else{0},depth:1,..RetirementDemand::default()})
    }

    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let grant=cx.retained_grant();self.begin_close();
        if self.close_stage==0{if let Some(job)=self.job.as_mut(){let step=job.close_step(cx)?;let step=step.admit(grant,job.terminal_is_empty())?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>{self.close_stage=1;InteractiveJobCloseStep::Pending{progress}},step=>step})}self.close_stage=2;return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let progress=if self.close_stage==1{drop(self.job.take());self.close_stage=2;RetainedCloneProgress{copied_items:1,..Default::default()}}else if self.params.is_some(){drop(self.params.take());self.close_stage=3;RetainedCloneProgress{copied_items:1,..Default::default()}}else if let Some(source)=self.fault_source.as_ref(){let extent=source.allocated_capacity_bytes();if grant.maximum_release_bytes<extent{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}drop(self.fault_source.take());self.close_stage=4;RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()}}else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};cx.consume_fuel(1);cx.consume_retained(progress)?;Ok(InteractiveJobCloseStep::Pending{progress})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.fault_source.is_none()
    }
}

impl<J> Drop for WorkerJobSessionAdmissionRejected<J> {
    fn drop(&mut self) {
        debug_assert!(self.job.is_none() && self.params.is_none() && self.fault_source.is_none(), "rejected worker session admission requires exact incremental close");
    }
}

struct WorkerJobSubmission<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<OriginalWorkerJobAuthority<J>>,
    ran: bool,
}

impl<J: InteractiveJob + 'static> WorkerJobSubmission<J> {
    fn run(mut self) {
        self.ran = true;
        let mut authority = self.authority.take().expect("submitted worker closure owns exact job authority");
        let terminal = authority.run_staged_callback();
        self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
    }
}

impl<J> Drop for WorkerJobSubmission<J> {
    fn drop(&mut self) {
        let Some(authority) = self.authority.take() else { return };
        let rejected = self.inner.rejection_kind.load(Ordering::Acquire) != u8::MAX;
        unsafe { self.inner.put_authority(authority, if rejected { SESSION_REJECTED } else { SESSION_CLOSE }) };
    }
}

fn worker_job_begin_close<J>(inner: &WorkerJobSessionInner<J>) -> WorkerJobCloseStep {
    inner.close_requested.store(true, Ordering::Release);
    loop {
        let phase = inner.phase();
        if phase == SESSION_SUBMITTED || phase == SESSION_TRANSITION || phase == SESSION_CHECKED_OUT {
            inner.terminal_intent.store(1, Ordering::Release);
            inner.raise_wake();
            return WorkerJobCloseStep::Blocked;
        }
        if phase == SESSION_CLOSE {
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        if phase == SESSION_EMPTY {
            return WorkerJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let authority = unsafe { inner.take_authority() };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
    }
}

fn worker_job_close_step<J:InteractiveJob>(inner:&WorkerJobSessionInner<J>,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}if inner.phase.compare_exchange(SESSION_CLOSE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Ok(if inner.phase()==SESSION_EMPTY{WorkerJobCloseStep::Complete{progress:Default::default()}}else{WorkerJobCloseStep::Blocked})}let mut authority=unsafe{inner.take_authority()};let result=authority.close_step(cx);if matches!(result,Ok(WorkerJobCloseStep::Complete{..})){assert!(authority.terminal_is_empty());drop(authority);inner.phase.store(SESSION_EMPTY,Ordering::Release);inner.raise_wake()}else{unsafe{inner.put_authority(authority,SESSION_CLOSE)}}result}

impl<J: InteractiveJob + 'static> WorkerJobSession<J> {
    /// 📏️ Quotes the actual fixed storage births and initialized source/control writes before custody changes.
    pub fn admission_demand()->RetirementDemand{
        let mut demand=OriginalWorkerJobAuthority::<J>::admission_demand();
        demand.capacity_bytes+=semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>()+size_of::<WorkerJobRetirementNode<J>>();
        demand
    }
    /// 🚪️ Retains both original sources until their existing normal control admits every physical birth.
    /// 🎟️ Quotes complete atomic initialization and the original job/parameter moves before storage birth.
    pub fn owned_admission_demand()->Result<RetirementDemand,ValueError>{
        let mut demand=Self::admission_demand();let initialized=OriginalWorkerPayload::initialization_copy_bytes()+OriginalWorkerJobAuthority::<J>::admission_page_publication_copy_bytes()+size_of::<J>()+2*size_of::<Option<J>>()+2*size_of::<Option<BatchJobParams>>()+2*size_of::<bool>()+size_of::<u8>();demand.copy_bytes=demand.copy_bytes.checked_add(initialized).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original atomic session initialization extent overflow"))?;Ok(demand)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires original parameters"))?;
        if job.is_none()||control.admission_identity()!=(original.operation,original.generation)||!control.admission_cancel().is_none_or(|token|token.is_same_node(&original.cancel)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires the same original job and identity"))}
        let demand=Self::owned_admission_demand()?;let grant=control.admission_grant()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth||original.cancel.is_cancelled_now()||!control.admission_is_open(){return Ok(None)}
        let Some(session)=Self::birth_storage(original.operation,original.generation,control)?else{return Ok(None)};
        let inner=session.inner.0.as_ref().expect("admitted original session Arc");let authority=unsafe{(&mut *inner.authority.get()).as_mut().expect("admitted original authority")};
        while !authority.is_initialized(){let progress=authority.advance_initialization(grant);assert!(progress.copied_items>0,"admitted original atomic initialization must advance");}
        unsafe{authority.write_owned_source_prepaid(job,params);}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:0};
        control.receive_admission(progress).expect("same exclusive caller admitted original session births");
        Ok(Some((session,progress)))
    }

    fn birth_storage(operation:OperationId,generation:Generation,control:&mut impl WorkerJobAdmissionControl)->Result<Option<Self>,ValueError>{if !OriginalWorkerPageReservation::available(){return Ok(None)}let Some(slot)=reserve_worker_job_retirement_slot()else{return Ok(None)};let Some(reservation)=OriginalWorkerPageReservation::try_reserve()else{WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(None)};let authority=match unsafe{OriginalWorkerJobAuthority::admit_pending_prepaid(operation,generation,reservation)}{Ok(authority)=>authority,Err(error)=>{WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Err(error)}};let inner=session_return::SessionHandle::new(WorkerJobSessionInner{generation,phase:AtomicU8::new(SESSION_IDLE),authority:ManuallyDrop::new(std::cell::UnsafeCell::new(Some(authority))),rejection_kind:AtomicU8::new(u8::MAX),close_requested:AtomicBool::new(false),terminal_intent:AtomicU8::new(0),wake_pending:AtomicBool::new(false),wake_sequence:AtomicU64::new(0),wake_exhausted:AtomicBool::new(false),wake_guard:AtomicBool::new(false),waker:ManuallyDrop::new(std::cell::UnsafeCell::new(None)),retained_wake:ManuallyDrop::new(std::cell::UnsafeCell::new(None)),last_step_end_us:AtomicU64::new(u64::MAX)});let retirement=Box::new(WorkerJobRetirementNode{header:WorkerJobRetirementHeader{slot,phase:worker_job_retirement_node_phase::<J>,pump:pump_worker_job_retirement_node::<J>,destroy:destroy_worker_job_retirement_node::<J>},inner:None});Ok(Some(Self{inner:WorkerJobSessionArc(Some(inner)),generation,terminal_step_end_us:None,retirement:std::cell::UnsafeCell::new(Some(retirement)),retirement_state:AtomicU8::new(0)}))}

    /// 📥️ Returns an outstanding original physical receipt even when cancellation already requested close.
    pub fn take_retained_step_receipt(&self)->Result<Option<(RetainedCloneGrant,RetainedCloneProgress)>,WorkerJobContention>{if self.inner.0.is_none(){return Ok(None)}let phase=self.inner.phase();if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.contention())}let mut authority=unsafe{self.inner.take_authority()};let original=authority.take_retained_step_receipt();unsafe{self.inner.put_authority_quiet(authority,phase)};Ok(original)}

    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobPoll::TerminalEmpty}else{WorkerJobPoll::Closing}}
        match self.inner.phase() {
            SESSION_IDLE => WorkerJobPoll::Idle,
            SESSION_SUBMITTED => WorkerJobPoll::Submitted,
            SESSION_OUTCOME => WorkerJobPoll::Outcome,
            SESSION_TERMINAL => WorkerJobPoll::Terminal,
            SESSION_REJECTED => WorkerJobPoll::Rejected,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobPoll::CheckedOut,
            SESSION_CLOSE => WorkerJobPoll::Closing,
            SESSION_EMPTY => WorkerJobPoll::TerminalEmpty,
            _ => WorkerJobPoll::Closing,
        }
    }

    fn try_step_inline(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self.contention());
        }
        let mut authority = unsafe { self.inner.take_authority() };
        let issued=authority.issued_grant(retained);
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.sequence() };
        if issued.maximum_items==0||issued.maximum_depth==0{unsafe{self.inner.put_authority_quiet(authority,SESSION_IDLE)};return Ok((ticket,WorkerJobPoll::Idle))}
        authority.stage_callback(issued);
        let terminal = authority.run_staged_callback();
        self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
        Ok((ticket, if terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome }))
    }

    /// 🎟️ Borrows the initiating context for one exact original callback and retains job custody on every failure.
    pub fn try_step_on_original_caller(&self,cx:&mut StepContext<'_>)->Result<Option<(WorkerJobTicket,WorkerJobPoll)>,OriginalTurnFailure>{if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}if self.inner.0.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback session is terminal empty").into())}if self.inner.phase.compare_exchange(SESSION_IDLE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Ok(None)}let mut authority=unsafe{self.inner.take_authority()};if let Err(error)=authority.validate_original_caller(cx){unsafe{self.inner.put_authority_quiet(authority,SESSION_IDLE)};return Err(error.into())}let ticket=WorkerJobTicket{generation:self.inner.generation,step_sequence:authority.sequence()};let terminal=match authority.step_on_original_caller(cx){Ok(terminal)=>terminal,Err(failure)=>{unsafe{self.inner.put_authority(authority,SESSION_CLOSE)};return Err(failure)}};self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX),Ordering::Release);unsafe{self.inner.put_authority(authority,if terminal{SESSION_TERMINAL}else{SESSION_OUTCOME})};Ok(Some((ticket,if terminal{WorkerJobPoll::Terminal}else{WorkerJobPoll::Outcome})))}

    pub fn try_step_on_caller(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 🕰️ The clock reading the last step of this session ended at — its driver's exit reading, so a
    /// caller deciding whether to run another step needs no read of its own.
    pub fn last_step_end_us(&self) -> Option<u64> {
        if self.inner.0.is_none(){return self.terminal_step_end_us}
        Some(self.inner.last_step_end_us.load(Ordering::Acquire)).filter(|end_us| *end_us != u64::MAX)
    }

    /// 🧵️ Executes one exact owner turn from a retained scheduler already running inside its worker.
    pub fn try_step_on_worker(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 👁️ Borrows the original typed registration state under its existing wake gate.
    pub fn has_retained_wake(&self)->Result<bool,WorkerJobContention>{
        if self.inner.0.is_none(){return Ok(false)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let retained=unsafe{(&*self.inner.retained_wake.get()).is_some()};self.inner.wake_guard.store(false,Ordering::Release);Ok(retained)
    }

    /// 🔔️ Keeps the original admitted typed wake frame until its own paid close turns finish.
    pub fn register_retained_wake(&self,original:&mut Option<Box<dyn RetainedWorkerWake>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,WorkerJobContention>{
        if original.is_none(){return Ok(None)}
        if self.inner.0.is_none()||self.inner.phase()==SESSION_EMPTY{return Err(WorkerJobContention::TerminalEmpty)}
        let copy=size_of::<Option<Box<dyn RetainedWorkerWake>>>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let slot=unsafe{&mut *self.inner.retained_wake.get()};
        if slot.is_some()||unsafe{(&*self.inner.waker.get()).is_some()}{self.inner.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        *slot=original.take();
        if self.inner.wake_pending.load(Ordering::Acquire){slot.as_ref().unwrap().wake_by_ref();}
        self.inner.wake_guard.store(false,Ordering::Release);
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }

    pub fn register_wake(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        self.inner.register_waker(waker)
    }

    /// 🔔️ Replaces a stale session wake with the exact job-owned close wake when available.
    pub fn register_close_wake(&self, waker: &Waker) -> Result<bool, WorkerJobContention> {
        if self.inner.0.is_none(){return Ok(false)}
        loop {
            let phase = self.inner.phase();
            if phase != SESSION_CLOSE {
                self.take_wake();
                self.register_wake(waker)?;
                return Ok(true);
            }
            if self.inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
                continue;
            }
            self.inner.wake_pending.store(false, Ordering::Release);
            let authority = unsafe { self.inner.take_authority() };
            let registered = authority.register_close_wake(waker);
            unsafe { self.inner.put_authority_quiet(authority, SESSION_CLOSE) };
            return Ok(registered);
        }
    }

    pub fn take_wake(&self) -> bool {
        if self.inner.0.is_none(){return false}
        self.inner.wake_pending.swap(false, Ordering::AcqRel)
    }

    /// 🧵️ `J: Send` is stated here rather than on [`InteractiveJob`]: this is the one call that
    /// actually hands the job to another thread, so the requirement belongs to it. Targets with no
    /// second thread (the browser wasm build) never reach this and keep their `Rc`-owning jobs.
    pub fn try_submit_step(&self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobTicket, WorkerJobSubmitFault>
    where
        J: Send,
    {
        if self.inner.0.is_none(){return Err(WorkerJobSubmitFault::Contention(WorkerJobContention::TerminalEmpty))}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobSubmitFault::Contention(self.contention()));
        }
        let mut authority = unsafe { self.inner.take_authority() };
        authority.stage_callback(retained);
        if authority.sequence() == u64::MAX {
            unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
            return Err(WorkerJobSubmitFault::SequenceExhausted);
        }
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.sequence() };
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        self.inner.phase.store(SESSION_SUBMITTED, Ordering::Release);
        let submission = WorkerJobSubmission { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), ran: false };
        let closure: semio_framework_async::Job = Box::new(move || submission.run());
        match pool.try_submit(lane, closure) {
            Ok(()) => Ok(ticket),
            Err(error) => {
                let kind = error.kind();
                self.inner.rejection_kind.store(worker_rejection_code(kind), Ordering::Release);
                drop(error.into_job());
                Err(WorkerJobSubmitFault::Pool(kind))
            }
        }
    }

    pub fn take_outcome(&self, ticket: WorkerJobTicket) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(if ticket.generation!=self.generation{WorkerJobTakeFault::Stale}else{WorkerJobTakeFault::WrongPhase})}
        if ticket.generation != self.inner.generation {
            return Err(WorkerJobTakeFault::Stale);
        }
        if self.inner.phase.compare_exchange(SESSION_OUTCOME, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        if authority.sequence() != ticket.step_sequence.saturating_add(1) {
            unsafe { self.inner.put_authority(authority, SESSION_OUTCOME) };
            return Err(WorkerJobTakeFault::Stale);
        }
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_OUTCOME })
    }

    pub fn take_terminal(&self) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_TERMINAL, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_TERMINAL })
    }

    pub fn take_rejected(&self) -> Result<WorkerJobRejected<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_REJECTED, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobTakeFault::WrongPhase);
        }
        let authority = unsafe { self.inner.take_authority() };
        let kind = worker_rejection_kind(self.inner.rejection_kind.load(Ordering::Acquire));
        Ok(WorkerJobRejected { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), kind })
    }

    pub fn begin_close(&self) -> WorkerJobCloseStep {
        if self.inner.0.is_none(){return if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:Default::default()}}else{WorkerJobCloseStep::Pending{progress:Default::default()}}}
        worker_job_begin_close(&self.inner)
    }

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{if self.inner.0.is_none(){return Ok(false)}let phase=self.inner.phase();if phase==SESSION_EMPTY{return Ok(false)}if phase!=SESSION_CLOSE||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(self.contention()))}let authority=unsafe{self.inner.take_authority()};let original=authority.has_original_cancel_alias_witness(witness);unsafe{self.inner.put_authority_quiet(authority,phase)};Ok(original)}

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{if self.inner.0.is_none(){return Ok(None)}let phase=self.inner.phase();if phase==SESSION_EMPTY{return Ok(None)}if phase!=SESSION_CLOSE||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(self.contention()))}let mut authority=unsafe{self.inner.take_authority()};let result=authority.return_original_cancel_alias_step(witness,grant).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>WorkerJobDemandError::Refused(error),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation))});unsafe{self.inner.put_authority_quiet(authority,phase)};result}
    /// 🔐️ Inspect under the same exclusive phase admission used by the close cursor.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        if self.inner.0.is_none(){return Ok(if self.retirement_state.load(Ordering::Acquire)==3{Default::default()}else{RetirementDemand{copy_bytes:0,release_bytes:size_of::<WorkerJobRetirementNode<J>>(),depth:1,..Default::default()}})}
        let phase = self.inner.phase();
        if phase == SESSION_EMPTY { return self.original_session_arc_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused); }
        if matches!(phase, SESSION_SUBMITTED | SESSION_TRANSITION | SESSION_CHECKED_OUT) || self.inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation)));
        }
        let authority = unsafe { self.inner.take_authority() };
        let demand = authority.retirement_demands(maximum_copy_bytes);
        unsafe { self.inner.put_authority_quiet(authority, phase) };
        demand.map_err(WorkerJobDemandError::Refused)
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let grant=cx.retained_grant();if self.inner.0.is_none(){let step=self.release_retirement_slot(grant);return receive_original_worker_close_effects(cx,before,step)}
        let step=match worker_job_close_step(&self.inner,cx)?{WorkerJobCloseStep::Complete{progress}if progress!=Default::default()=>WorkerJobCloseStep::Pending{progress},WorkerJobCloseStep::Complete{..}=>{let grant=cx.retained_grant();let result=self.close_original_session_arc(grant).map(|step|match step{WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step});return receive_original_worker_close_result(cx,before,result)},step=>step};receive_original_worker_close_effects(cx,before,step)
    }

    fn original_session_arc_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        if self.inner.0.is_none(){return Ok(Default::default())}
        if let Some(demand)=worker_wake_retirement::session_retained_wake_demands(&self.inner,maximum_copy_bytes)?{return Ok(demand)}
        Ok(RetirementDemand{copy_bytes:0,release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>(),depth:1,..Default::default()})
    }

    fn close_original_session_arc(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobCloseStep,ValueError>{
        let demand=self.original_session_arc_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        let original=self.inner.0.as_ref().expect("terminal session retains original Arc");
        if !original.is_unique()||unsafe{(&*original.waker.get()).is_some()}{return Ok(WorkerJobCloseStep::Blocked)}
        if let Some(step)=worker_wake_retirement::close_session_retained_wake(original,grant)?{return Ok(step)}
        self.terminal_step_end_us=self.last_step_end_us();Ok(match self.inner.0.take().unwrap().try_return(){Ok(original)=>{drop(original);WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}},Err(original)=>{self.inner.0=Some(original);WorkerJobCloseStep::Blocked}})
    }

    pub fn terminal_is_empty(&self) -> bool {self.inner.0.is_none()&&self.retirement_state.load(Ordering::Acquire)==3}

    /// 🔭️ The named phase this session's close cursor is parked in — see [`WorkerJobClosePhase`].
    /// Reads the same fields the next `close_step` would walk, in the same order, without taking
    /// the authority or spending a turn.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobClosePhase::Empty}else{WorkerJobClosePhase::RetirementSlot}}
        match self.inner.phase() {
            SESSION_EMPTY => return WorkerJobClosePhase::SessionArc,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => return WorkerJobClosePhase::CheckedOut,
            SESSION_CLOSE => {}
            _ => return WorkerJobClosePhase::Open,
        }
        let Some(authority) = (unsafe { (&*self.inner.authority.get()).as_ref() }) else { return WorkerJobClosePhase::CheckedOut };
        worker_authority_phase(authority.close_phase())
    }

    fn contention(&self) -> WorkerJobContention {
        if self.inner.0.is_none(){return WorkerJobContention::TerminalEmpty}
        let generation = self.inner.generation;
        if self.inner.wake_exhausted.load(Ordering::Acquire) {
            return WorkerJobContention::WakeExhausted(generation);
        }
        let sequence = unsafe { (&*self.inner.authority.get()).as_ref().map_or(0, |authority| authority.sequence()) };
        let ticket = WorkerJobTicket { generation, step_sequence: sequence };
        match self.inner.phase() {
            SESSION_SUBMITTED => WorkerJobContention::Submitted(ticket),
            SESSION_OUTCOME => WorkerJobContention::Outcome(ticket),
            SESSION_TERMINAL => WorkerJobContention::Terminal(ticket),
            SESSION_REJECTED => WorkerJobContention::Rejected(generation),
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobContention::CheckedOut(generation),
            SESSION_CLOSE => WorkerJobContention::Closing(generation),
            SESSION_EMPTY => WorkerJobContention::TerminalEmpty,
            _ => WorkerJobContention::CheckedOut(generation),
        }
    }

    fn release_retirement_slot(&self, grant:RetainedCloneGrant) -> WorkerJobCloseStep {
        if self.retirement_state.load(Ordering::Acquire) == 3 {
            return WorkerJobCloseStep::Complete{progress:RetainedCloneProgress::default()};
        }
        let released_bytes=size_of::<WorkerJobRetirementNode<J>>();
        let copied_bytes=0;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0||self.retirement_state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress::default()};
        }
        let retirement = unsafe { (&mut *self.retirement.get()).take().expect("live worker session owns pre-admitted retirement node") };
        WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(), Ordering::Release);
        drop(retirement);
        self.retirement_state.store(3, Ordering::Release);
        WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copied_bytes,released_bytes,..RetainedCloneProgress::default()}}
    }
}

impl<J: InteractiveJob + 'static> Drop for WorkerJobSession<J> {
    fn drop(&mut self) {
        if self.terminal_is_empty(){return}
        if let Some(original)=self.inner.0.as_ref(){original.close_requested.store(true,Ordering::Release);original.terminal_intent.store(1,Ordering::Release);original.raise_wake();}
        if self.retirement_state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){assert!(std::thread::panicking(),"original session Arc requires its admitted close turn");return}
        let mut retirement=unsafe{(&mut *self.retirement.get()).take().expect("live worker session owns original retirement node")};
        retirement.inner=self.inner.0.take();
        let slot=retirement.header.slot;
        let pointer=Box::into_raw(retirement).cast::<WorkerJobRetirementHeader>();
        WORKER_JOB_RETIREMENT_SLOTS[slot].store(pointer,Ordering::Release);
        self.retirement_state.store(2,Ordering::Release);
        WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);
    }
}

pub struct WorkerJobOutcome<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<OriginalWorkerJobAuthority<J>>,
    restore_phase: u8,
}

impl<J> WorkerJobOutcome<J> {
    /// 📥️ The receiving caller consumes the same actual turn receipt exactly once before semantic access.
    /// 🧾️ The original checked-out authority retains both borrowed fields until its unique recipient accepts them.
    pub fn retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::retained_step_receipt)}

    pub fn take_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.authority.as_mut().and_then(OriginalWorkerJobAuthority::take_retained_step_receipt)}

    /// 🧾️ Returns the exact latest actual worker turn receipt without changing its outcome owner.
    pub fn retained_step_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original outcome retains its authority").retained_step_progress()}
    /// 📒️ Returns the actual physical receipt accumulated by this original worker authority.
    pub fn retained_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original outcome retains its authority").retained_progress()}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.authority.as_ref().and_then(OriginalWorkerJobAuthority::callback_verdict)
    }

    /// 📒️ This session's own overrun ledger — how many of its steps breached the interactive ceiling,
    /// the longest consecutive run, and the worst reading, for a host that wants the counters without
    /// waiting for a quarantine.
    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.authority.as_ref().map(OriginalWorkerJobAuthority::overrun_ledger)
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().expect("checked-out worker outcome owns exact job").job()
    }

    pub fn job_mut(&mut self)->Result<&mut J,ValueError>{self.authority.as_mut().expect("original outcome retains its authority").job_mut()}
    pub fn retained_failure(&self)->Option<&OriginalTurnFailure>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::retained_failure)}
    /// 🪪️ Borrows original semantic metadata while its payload remains in this worker authority.
    pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::outcome_descriptor)}
    pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>where J:InteractiveJob{self.authority.as_ref().expect("original outcome retains its authority").outcome()}
    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.authority.as_mut().expect("original outcome retains its authority").acknowledge_outcome(grant)}

    #[expect(clippy::result_large_err, reason = "Refused resumption returns the checked-out job authority and retained outcome without allocation or ownership loss.")]
    pub fn resume(mut self) -> Result<(), Self> {
        if self.restore_phase == SESSION_TERMINAL {
            return Err(self);
        }
        let authority = self.authority.as_ref().expect("checked-out worker outcome owns authority");
        if !authority.is_resumable() {
            return Err(self);
        }
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
        Ok(())
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobOutcome<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, self.restore_phase) };
        }
    }
}

pub struct WorkerJobRejected<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<OriginalWorkerJobAuthority<J>>,
    kind: semio_framework_async::WorkerSubmitErrorKind,
}

impl<J> WorkerJobRejected<J> {
    pub fn kind(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.kind
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().expect("checked-out rejected worker owner remains exact").job()
    }

    pub fn resume(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobRejected<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, SESSION_REJECTED) };
        }
    }
}

fn worker_rejection_code(kind: semio_framework_async::WorkerSubmitErrorKind) -> u8 {
    match kind {
        semio_framework_async::WorkerSubmitErrorKind::Shutdown => 0,
        semio_framework_async::WorkerSubmitErrorKind::Contended => 1,
        semio_framework_async::WorkerSubmitErrorKind::Poisoned => 2,
        semio_framework_async::WorkerSubmitErrorKind::Saturated => 3,
    }
}

fn worker_rejection_kind(code: u8) -> semio_framework_async::WorkerSubmitErrorKind {
    match code {
        0 => semio_framework_async::WorkerSubmitErrorKind::Shutdown,
        1 => semio_framework_async::WorkerSubmitErrorKind::Contended,
        2 => semio_framework_async::WorkerSubmitErrorKind::Poisoned,
        _ => semio_framework_async::WorkerSubmitErrorKind::Saturated,
    }
}
//#endregion 🏭️RetainedSessions
/// 🎟️ The [`WORKER_JOB_SESSION_SLOTS`] are process-wide, so one test binary's parallel tests share
/// them. A test that admits sessions holds a shared guard; the one test that owns every slot at
/// once holds the exclusive guard, so neither ever observes the other's admissions.
#[cfg(test)]
static WORKER_SESSION_SLOTS_TEST_AUTHORITY: std::sync::RwLock<()> = std::sync::RwLock::new(());

#[cfg(test)]
fn worker_session_slots_shared() -> std::sync::RwLockReadGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.read().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
fn worker_session_slots_exclusive() -> std::sync::RwLockWriteGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.write().unwrap_or_else(std::sync::PoisonError::into_inner)
}


#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
mod original_worker_fixture;
#[cfg(any(test,feature="testkit"))]
pub use original_worker_fixture::{OriginalFixtureSession,OriginalWorkerFixtureCaller};

#[cfg(test)]
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
#[cfg(test)]
#[global_allocator]
static JOB_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[cfg(test)]
#[path="🧪️tests/🔬️retained-ownership/🦀️.rs"]
mod retained_ownership_tests;

#[cfg(test)]
#[path="⏱️budget/🧪️tests/🦀️.rs"]
mod microsecond_budget_tests;

#[cfg(test)]
#[path="../♻️retirement/👷️authority/🧪️tests/🦀️.rs"]
mod worker_authority_retirement_tests;




#[cfg(test)]
#[path="🧪️tests/📏️close-demand/🦀️.rs"]
mod close_demand_tests;
