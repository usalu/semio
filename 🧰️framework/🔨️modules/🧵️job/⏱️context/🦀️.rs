//! 🧵️ Resumable interactive job protocol and its original caller authority.
//!
//! See `README.md`.

pub use semio_framework_job_receiving::OriginalNativeReceivingOutcome;
pub use semio_framework_job_replacement::{ReplacementIdentity,ReplacementStaleness};
use semio_framework_job_receiving::{OriginalJobWorkRecipient,retained_delta,retained_remaining,retained_min,retained_is_subset};
use std::future::Future;
use std::mem::ManuallyDrop;
use std::sync::Arc;
pub use semio_framework_value_retained_clone::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value_retained_clone::RetirementDemand;
pub use semio_framework_value_control_contract::{OriginalSourceTurn,OriginalSourceTurnIssuer,OriginalSourceTurnIdentity,OriginalSourceTurnWitness};

use semio_framework_value_retained_clone::{ValueError, ValueRefusalKind};
#[path="../📬️outcome/⏸️continuation/🦀️.rs"]
pub(crate) mod original_job_continuation;
#[doc(hidden)]
pub use original_job_continuation::source as original_return_source;
pub use semio_framework_job_failure::{OriginalSourceCloseFailure,project_original_source_close_failure,key_original_source_close_failure,OriginalFailureContext,OriginalWorkFailure,OriginalTurnFailure,OriginalTurnPrimary,OriginalReceivingPrimary,OriginalRetirementReceivingPrimary,OriginalClosureFailure,OriginalDomainFailure,OriginalDomainPrimary,OriginalDomainReservation,OriginalDomainProjection,OriginalDomainView,admit_original_domain_failure,original_domain_failure_demand,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault,project_original_retirement_failure,key_original_retirement_failure,project_original_retirement_receiving,key_original_retirement_receiving,project_original_receiving_primary,key_original_receiving_primary};
#[path="../♻️retirement/🎟️context/🦀️.rs"]
mod original_close_context;
pub use original_close_context::{close_original_source_effect_step,close_original_quoted_retained_step,receive_original_retained_work,receive_original_receiving_work,close_original_granted_step,close_original_nested_step,close_original_retained_step,close_original_retained_turn,close_original_effect_step,close_original_retained_effect_step};

#[cfg(test)]
use std::time::Instant;

use semio_framework_trace::TraceEvent;
#[cfg(test)] use semio_framework_trace::{record_operation_started};

pub use semio_framework_async::{CancelToken,CancelTokenRetirement,CancelTokenRetirementError,OriginalCancelRootAdmission,OriginalCancelRootIdentity};
pub use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
pub use semio_framework_trace::{Generation, InteractiveStage, OperationId, allocate_operation_id, allocate_operation_id_in_slot, interactive_step_contract_violated, runtime_diagnostics_enabled, set_runtime_diagnostics, INTERACTIVE_STEP_CEILING_US, RUNTIME_DIAGNOSTICS_ENV, SUSTAINED_OVERRUN_QUARANTINE_STEPS};

pub use semio_framework_job_payload::{JobPayloadStep, OriginalJobPayloadAdmission, JobPayloadAuthority, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadStream, JobPayloadAdmissionFault, JobPayloadRejectedPage, RetainedJobPayload, RetainedJobPayloadReader, RetainedPayloadBuilder, OriginalPayloadSourceIdentity, OriginalPayloadBuilderIdentity, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_PROCESS_BYTES};

pub use semio_framework_job_clock::{BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US, CommitPolicy, CommitValidation, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US, JobSequenceExhausted, MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US, Operation, RevisionId, StepBudget, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US, child_cancel_token, default_now_ms, default_now_us, install_microsecond_clock, logical_now_us, microseconds_from_milliseconds, root_cancel_token, validate_commit, validate_commit_with};

pub use semio_framework_job_outcome::{Checkpoint,CommitCandidate,InteractiveJobCloseStep,JobFault,StepOutcome};
pub use semio_framework_job_clock::ClockStride;
pub use semio_framework_job_clock::{OriginalClockAction,OriginalClockAuthority,OriginalClockDeadline,OriginalClockObservation,OriginalClockDecisionFrame,OriginalClockDecisionStatus,OriginalClockProducer,OriginalClockReply,OriginalClockReplyComplete,OriginalSystemClockContext};
pub use semio_framework_job_clock::OriginalSystemClock;

//#region 🕰️Clock

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_async::install_browser_monotonic_clock;

//#endregion 🕰️Clock

//#region 📄️RetainedPayload

pub use semio_framework_job_outcome::{close_step_outcome_slot,step_outcome_slot_retirement_demands,step_outcome_slot_retirement_work,JobOutcomeSlot,JobPayloadSlot};

pub use semio_framework_job_work_grant as retained_work;

pub use semio_framework_job_outcome_borrow::{JobOutcomeAdmission,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView,JobOutcomeDescriptorSlot};
impl semio_framework_job_outcome_borrow::JobOutcomeContext for StepContext<'_>{
 fn retained_grant(&self)->RetainedCloneGrant{self.retained_grant()}fn should_yield(&self)->bool{self.should_yield()}fn retirement_should_yield(&self)->bool{self.retirement_should_yield()}fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.consume_retained(progress)}fn consume_fuel(&mut self,work:u64){self.consume_fuel(work)}fn operation(&self)->OperationId{self.operation()}fn generation(&self)->Generation{self.generation()}fn original_retirement_active(&self)->bool{self.original_retirement_active()}
}
#[path="🪪️binding/🦀️.rs"]
mod original_source_binding;
pub use original_source_binding::OriginalSourceBinding;





enum SourceTurnReference<'a>{Owned(OriginalSourceTurn),Borrowed(&'a OriginalSourceTurn)}
impl SourceTurnReference<'_>{fn token(&self)->&OriginalSourceTurn{match self{Self::Owned(turn)=>turn,Self::Borrowed(turn)=>turn}}}
enum JobPayloadLedgerReference<'a>{Owned(Arc<JobPayloadOperationLedger>),Borrowed(&'a JobPayloadAuthority)}
impl std::ops::Deref for JobPayloadLedgerReference<'_>{type Target=Arc<JobPayloadOperationLedger>;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::Borrowed(authority)=>authority.original().expect("original context outlived its payload authority")}}}
enum StepCancelReference<'a>{Owned(CancelToken),Borrowed(&'a CancelToken),Observed(&'a CancelToken)}
impl std::ops::Deref for StepCancelReference<'_>{type Target=CancelToken;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::Borrowed(original)|Self::Observed(original)=>original}}}
#[path="🎟️source/🦀️.rs"]
mod owned_source;
/// 🧿️ Names the actual borrowed roots of one live caller context without lending or renewing them.
pub struct OriginalCallerSource<'source>{pub ledger:&'source Arc<JobPayloadOperationLedger>,pub cancel:&'source CancelToken,pub operation:OperationId,pub generation:Generation,pub clock:fn()->Option<u64>,pub deadline:u64}
pub use owned_source::{OriginalCallerCloseReturn,OriginalCallerSourceParent,OriginalCallerSourceLoan,ORIGINAL_NATIVE_SOURCE_CAPACITY,OriginalNativeLease,OriginalNativeRetirement,OriginalNativeBorrow,OriginalNativeBorrowed,OriginalNativeRecipientIdentity,OriginalNativeRecipientClosed,OriginalNativeRetiredWitness};
#[path="🛬️owned/🦀️.rs"]
mod original_job_decoder;
pub use original_job_decoder::{OriginalJobDecodeOwner,OriginalCreditedOutput,original_receiving_scope_headroom,original_receiving_service_scope_headroom,unquoted_receiving_work};
//#endregion 📄️RetainedPayload

#[path="📦️return/🪶️weak/🦀️.rs"]
mod original_native_weak_return;


//#region 🧭️StepContext
/// 🧭️ Borrows one job step’s identity, budget, cancellation, clock and preview cursor; private fields centralize cancellation observation.
pub struct StepContext<'a> {
    original_decoder:Option<&'a mut OriginalJobDecodeOwner>,
    receiving:semio_framework_job_receiving::OriginalReceivingScope,
    retained:RetainedCloneGrant,
    retained_progress:&'a mut RetainedCloneProgress,
    operation: OperationId,
    generation: Generation,
    fuel_remaining: u64,
    semantic_step_limit: Option<u64>,
    work_units_spent:u128,
    deadline_us: u64,
    now_us: fn() -> Option<u64>,
    clock: std::cell::Cell<ClockStride>,
    cancel: StepCancelReference<'a>,
    stage: &'static str,
    preview_sequence: &'a mut u64,
    payload_ledger: JobPayloadLedgerReference<'a>,
    source_parent: Option<&'a OriginalCallerSourceParent>,
    source_turn:Option<SourceTurnReference<'a>>,
    return_budget:Option<original_job_continuation::OriginalReturnBudgetReference<'a>>,
    reserved_return_progress:RetainedCloneProgress,
    reserved_return_work:u64,
    birth_work:u128,
    reserved_observer:Option<(&'a mut RetainedCloneProgress,&'a mut u128)>,
    native_weak_parent:Option<original_native_weak_return::OriginalNativeWeakParent>,
    source_epoch: Option<u64>,
    original_retirement: bool,
    loan_deadline_us: Option<u64>,
    original_clock_binding: Option<u64>,
    original_total_fuel: u64,
    issued_ceiling:(RetainedCloneGrant,u64),
    issued_start:RetainedCloneProgress,
    original_total_work_start: u128,
    original_absolute_deadline: u64,
    original_total_retained: RetainedCloneGrant,
    original_total_retained_start:RetainedCloneProgress,
    original_total_retained_bound:bool,
    retained_policy:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    original_physical_recipient:Option<(&'a mut RetainedCloneProgress,RetainedCloneProgress)>,
    original_work_recipient:Option<&'a mut u128>,
    replacement_current:Option<ReplacementIdentity>,
    payload_page_granted: bool,
}

impl Drop for StepContext<'_>{fn drop(&mut self){if let Some((recipient,start))=self.original_physical_recipient.as_mut(){let delta=retained_delta(*self.retained_progress,*start);**recipient=recipient.checked_add(delta).expect("original physical receipt exceeds usize");}if self.reserved_observer.is_some(){*self.retained_progress=semio_framework_job_return_budget::source_fresh_progress(*self.retained_progress,self.reserved_return_progress).expect("actual reserved effects exceed their aggregate observer");}let business=self.original_work_units_spent();if let Some(recipient)=self.original_work_recipient.as_mut(){**recipient=recipient.checked_add(business).expect("original source work counter exceeds u128");}}}

fn replaced_context<R>(result:Result<R,OriginalTurnFailure>,message:std::borrow::Cow<'static,str>)->Result<R,OriginalTurnFailure>{let refusal=||{let mut refusal=ValueError::invariant("");refusal.message=message.clone();refusal};Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})}
#[derive(Clone,Copy)]
/// 🪪️ Pins the exact turn context a loan or parked wallet was lent from.
pub struct OriginalRetainedLoanIdentity{turn:Option<OriginalSourceTurnIdentity>,parent:Option<usize>,recipient:usize,cursor:usize,ledger:usize,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,deadline:u64}
impl OriginalRetainedLoanIdentity{
 pub fn capture(cx:&StepContext<'_>)->Self{Self{turn:cx.original_source_turn_identity(),parent:cx.source_parent.map(|parent|parent as*const _ as usize),recipient:cx.retained_progress as*const _ as usize,cursor:cx.preview_sequence as*const _ as usize,ledger:Arc::as_ptr(&cx.payload_ledger)as usize,operation:cx.operation,generation:cx.generation,clock:cx.now_us,deadline:cx.deadline_us}}
 pub fn matches(&self,cx:&StepContext<'_>)->bool{self.turn==cx.original_source_turn_identity()&&self.parent==cx.source_parent.map(|parent|parent as*const _ as usize)&&self.recipient==cx.retained_progress as*const _ as usize&&self.cursor==cx.preview_sequence as*const _ as usize&&self.ledger==Arc::as_ptr(&cx.payload_ledger)as usize&&self.operation==cx.operation&&self.generation==cx.generation&&std::ptr::fn_addr_eq(self.clock,cx.now_us)&&self.deadline==cx.deadline_us}
}
struct OriginalRetainedLoan<'a>{target:*mut StepContext<'a>,original:RetainedCloneGrant,identity:OriginalRetainedLoanIdentity}
struct OriginalRetirementLoan<'a>{target:*mut StepContext<'a>,original:bool,identity:OriginalRetainedLoanIdentity}
impl Drop for OriginalRetirementLoan<'_>{fn drop(&mut self){unsafe{if self.identity.matches(&*self.target){(*self.target).original_retirement=self.original}}}}
impl OriginalRetainedLoan<'_>{fn matches_original(&self)->bool{unsafe{self.identity.matches(&*self.target)}}}
impl Drop for OriginalRetainedLoan<'_>{fn drop(&mut self){if self.matches_original(){unsafe{(*self.target).retained=self.original}}}}
pub type OriginalBirthBaseline=(u128,RetainedCloneProgress,u64,RetainedCloneProgress);
struct OriginalWorkLoan<'a>{target:*mut StepContext<'a>,fuel:u64,semantic_step_limit:Option<u64>,deadline:u64,loaned:Option<u64>,issued:u64,spent:u128,reserved:u64,cancel:usize,identity:OriginalRetainedLoanIdentity}
impl OriginalWorkLoan<'_>{fn matches_source(&self)->bool{let cx=unsafe{&*self.target};self.identity.matches(cx)&&self.cancel==cx.original_cancel_token()as*const CancelToken as usize&&cx.work_units_spent>=self.spent&&cx.reserved_return_work>=self.reserved}fn matches_original(&self)->bool{if !self.matches_source(){return false}let cx=unsafe{&*self.target};let physical=cx.work_units_spent-self.spent;let reserved=u128::from(cx.reserved_return_work-self.reserved);physical.checked_sub(reserved).is_some_and(|work|work<=u128::from(self.issued))&&cx.fuel_remaining<=self.issued}}
/// 🗣️ Says whether the loan lost its context or which stage overspent its issued work.
impl OriginalWorkLoan<'_>{fn refusal_message(&self)->std::borrow::Cow<'static,str>{if !self.matches_source(){return "original work loan callback replaced its initiating context".into()}let cx=unsafe{&*self.target};format!("original work loan callback spent {} of {} issued work units with {} reserved and {} left (stage {})",cx.work_units_spent-self.spent,self.issued,cx.reserved_return_work-self.reserved,cx.fuel_remaining,cx.stage).into()}}
impl Drop for OriginalWorkLoan<'_>{fn drop(&mut self){if self.matches_source(){let cx=unsafe{&mut*self.target};let physical=cx.work_units_spent-self.spent;let reserved=cx.reserved_return_work.saturating_sub(self.reserved);let spent=physical.saturating_sub(u128::from(reserved));cx.fuel_remaining=self.fuel.saturating_sub(u64::try_from(spent).unwrap_or(u64::MAX));cx.semantic_step_limit=self.semantic_step_limit;cx.deadline_us=self.deadline;cx.loan_deadline_us=self.loaned;}}}

/// 🔗️ The clock, payload page and stage a parked wallet carries between contexts; only the context can read or install them.
#[derive(Clone,Copy)]
pub struct ParkedState{clock:ClockStride,payload_page_granted:bool,stage:&'static str}
impl ParkedState{
    /// 🕰️ Returns the clock observation the wallet parked with.
    pub fn clock(&self)->ClockStride{self.clock}
}
impl StepContext<'_>{
    /// 🔗️ Snapshots the state a parked wallet would carry.
    pub fn park_state(&self)->ParkedState{ParkedState{clock:self.clock.get(),payload_page_granted:self.payload_page_granted,stage:self.stage}}
    /// 🔗️ Parks this turn: hands out its preview sequence and state, then leaves it no fuel and only the grant its posted spend already holds.
    pub fn park_take(&mut self,posted:RetainedCloneProgress)->(u64,ParkedState){let state=self.park_state();let sequence=std::mem::take(self.preview_sequence);self.fuel_remaining=0;self.retained=RetainedCloneGrant{maximum_items:posted.copied_items,maximum_copy_bytes:posted.copied_bytes,maximum_capacity_bytes:posted.retained_capacity_bytes,maximum_release_bytes:posted.released_bytes,maximum_depth:0};(sequence,state)}
    /// 🔗️ Installs a parked wallet's own state into the context it issues.
    pub fn park_restore(&mut self,state:ParkedState){self.clock.set(state.clock);self.payload_page_granted=state.payload_page_granted;self.stage=state.stage}
    /// 🔗️ Returns a parked wallet's preview sequence, and its clock observation when it is later than this context's.
    pub fn park_return(&mut self,sequence:u64,clock:Option<ClockStride>){*self.preview_sequence=(*self.preview_sequence).max(sequence);if let Some(clock)=clock{self.clock.set(clock)}}
}

impl JobPayloadStep for StepContext<'_> {
    fn payload_ledger(&self) -> &Arc<JobPayloadOperationLedger> { &self.payload_ledger }
    fn payload_page_granted(&self) -> bool { self.payload_page_granted }
    fn grant_payload_page(&mut self) { self.payload_page_granted = true; }
    fn is_cancelled(&self) -> bool { StepContext::is_cancelled(self) }
    fn should_yield(&self) -> bool { StepContext::should_yield(self) }
    fn retained_grant(&self) -> RetainedCloneGrant { StepContext::retained_grant(self) }
    fn consume_retained(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> { StepContext::consume_retained(self, progress) }
}


impl<'a> StepContext<'a> {
    /// 🎟️ Borrows the Source return budget this context holds, if any.
    #[doc(hidden)]
    pub fn source_parent_ref(&self)->Option<&'a OriginalCallerSourceParent>{self.source_parent}
    #[doc(hidden)]
    pub fn return_budget_ref(&self)->Option<&original_job_continuation::OriginalReturnBudgetReference<'a>>{self.return_budget.as_ref()}
    /// 🎟️ Mutably borrows the Source return budget this context holds, if any.
    #[doc(hidden)]
    pub fn return_budget_mut(&mut self)->Option<&mut original_job_continuation::OriginalReturnBudgetReference<'a>>{self.return_budget.as_mut()}
    pub fn admit_original_core_wire_delivery(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_core_wire_delivery(self)}
    /// 🗂️ Requests the return table's birth, which the resident birth service does inside the turn; true only if this very call admitted a table.
    pub fn admit_original_return_table(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_return_table(self)}
    /// 🌱️ Requests the return table's birth and answers whether the table is born and the Source origin current for this turn, the explicit predicate a caller waits on.
    pub fn admit_original_return_origin(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_return_table(self)?;Ok(self.return_budget.as_ref().is_some_and(|budget|budget.get().table().is_some())&&self.original_return_origin_current()?)}
    /// 🔭️ Whether the Source origin is current for this turn.
    pub fn original_return_origin_current(&mut self)->Result<bool,OriginalTurnFailure>{let parent=self.original_source_parent()?;parent.admit_return_origin(self)}
    pub fn admit_original_escrow_publisher(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_escrow_publisher(self)}
    pub fn publish_original_escrow(&mut self,header:usize,expose:impl for<'loan> FnOnce(&'loan [u8])->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::publish_original_escrow(self,header,expose)}
    pub fn native_receiving_identity(&self)->Option<(semio_framework_value_native_owned::NativeAllocationIdentity,u64)>{self.original_decoder.as_deref().and_then(|owner|owner.receiving_identity())}
    pub fn with_original_decoder(mut self,original:&'a mut OriginalJobDecodeOwner)->Self{self.original_decoder=Some(original);self}
    /// 🫴️ Loans original cancellation, clock, remaining credits and the shared construction receipt; with no quote its scope claims at most half of the turn, so it never starves the turn that drives it.
    pub fn with_native_receiving(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
        let mut operation=Some(operation);self.original_receiving_outcome(out,None,semio_framework_job_receiving::OriginalReceivingOperation::Receive(&mut |native,grant|operation.take().unwrap()(native,grant)))
    }
    /// 🤝️ Lends the decoder, grant and work available to its receiving body; the scope's share is raised to cover `floor`, the next indivisible action, never above the whole turn.
    pub fn with_native_receiving_work(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,floor:semio_framework_value_retained_clone::RetirementDemand,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant,u64)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        if self.receiving_floor_exceeds_turn(floor){return Ok(false)}
        self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
        let mut operation=Some(operation);let quote=self.covering_receiving_quote(floor);self.original_receiving_outcome(out,quote,semio_framework_job_receiving::OriginalReceivingOperation::ReceiveWithWork(&mut |native,grant,units|operation.take().unwrap()(native,grant,units)))
    }
    /// 🔪️ Like [`Self::with_native_receiving_work`], but the automatic claim never exceeds `slice` on any axis, so a held scope credit stays a bounded share of Source stock; `floor` raises it to cover the next indivisible action.
    pub fn with_native_receiving_slice(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,floor:semio_framework_value_retained_clone::RetirementDemand,slice:RetainedCloneGrant,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant,u64)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        if self.receiving_floor_exceeds_turn(floor){return Ok(false)}
        self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
        let mut operation=Some(operation);let quote=self.sliced_receiving_quote(floor,slice);self.original_receiving_outcome(out,Some(quote),semio_framework_job_receiving::OriginalReceivingOperation::ReceiveWithWork(&mut |native,grant,units|operation.take().unwrap()(native,grant,units)))
    }
    /// 🎯️ Like [`Self::with_native_receiving`], but its scope claims exactly the quoted grant and work plus its own control.
    pub fn with_native_receiving_quoted(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,quote:(RetainedCloneGrant,u64),operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
        let mut operation=Some(operation);self.original_receiving_outcome(out,Some(quote),semio_framework_job_receiving::OriginalReceivingOperation::Receive(&mut |native,grant|operation.take().unwrap()(native,grant)))
    }
    /// 🎯️ Like [`Self::with_native_receiving_quoted`], and the callback receives the scope body fuel so its effect stays inside that fuel.
    pub fn with_native_receiving_quoted_work(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,quote:(RetainedCloneGrant,u64),operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant,u64)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
        let mut operation=Some(operation);self.original_receiving_outcome(out,Some(quote),semio_framework_job_receiving::OriginalReceivingOperation::ReceiveWithWork(&mut |native,grant,units|operation.take().unwrap()(native,grant,units)))
    }
    /// 🛫️ Loans and commits this original Job caller’s work to the supplied encoder without replacing its physical owner.
    pub fn with_native_encoder_work<T,E:From<ValueError>>(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let fuel=self.fuel_remaining;let deadline=self.deadline_us;let now=self.now_us;let cancel=&self.cancel;let mut ready=move||!cancel.is_cancelled_now()&&now().is_some_and(|value|value<deadline);
        let before=native.actual_work();
        let mut authority=semio_framework_value_control_contract::OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
        let result=authority.with_loan(&mut semio_framework_job_receiving::OriginalJobWorkRecipient::new(&fuel,&mut ready),|work_loan|native.with_work_recipient(work_loan,operation)).map_err(E::from)?;
        let performed=native.actual_work().saturating_sub(before);self.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));result
    }
    /// 🛫️ Snapshots fuel, deadline, clock and cancellation so a later encoder hop without the context still commits under its original Work loan.
    pub fn native_encoder_work(&self)->NativeEncoderWork{NativeEncoderWork{fuel:self.fuel_remaining,deadline:self.deadline_us,now:self.now_us,cancel:self.cancel_token(),performed:0}}
    /// ♻️ Loans retirement-only authority after cancellation while retaining the original deadline, fuel and physical credits.
    pub fn with_native_retirement<T>(&mut self,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>)->Result<T,ValueError>)->Result<T,ValueError>{
        if self.fuel_remaining==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original retirement turn has no fuel"));}
        let grant=self.retained_grant();let mut observed=0;let source=self.original_source_turn_identity();let Self{original_decoder,deadline_us,now_us,original_retirement,fuel_remaining,..}=self;let owner=original_decoder.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native retirement requires the original Job decoder issuer"))?;let retirement=&*original_retirement;let original_deadline=&*deadline_us;let now=*now_us;let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|*retirement||now().is_some_and(|value|value<*original_deadline);let mut deadline=||*retirement||now().is_some_and(|value|value<*original_deadline);let mut ready=||*retirement||now().is_some_and(|value|value<*original_deadline);let mut work=OriginalJobWorkRecipient::new(fuel_remaining,&mut ready).with_source(source);semio_framework_job_receiving::native_call_result(owner.retire(grant,&mut callback,&mut deadline,&mut work,&mut observed,operation))
    }
    /// 🌉️ Runs one non-suspending unit step and retains every original receiving refusal.
    pub fn with_native_receiving_ready(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,step:impl AsyncFnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        self.with_native_receiving(out,|control,grant|{let mut task=std::task::Context::from_waker(std::task::Waker::noop());let mut future=std::pin::pin!(step(control,grant));match future.as_mut().poll(&mut task){std::task::Poll::Ready(result)=>result,std::task::Poll::Pending=>Err(ValueError::invariant("original receiving scope cannot span a suspension"))}})
    }
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        StepContext::with_payload_ledger(budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)),retained_progress)
    }

    #[doc(hidden)]
    pub fn with_payload_ledger(budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, clock: ClockStride, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        let (operation, generation) = (payload_ledger.operation, payload_ledger.generation);
        StepContext::assemble(operation,generation,budget,clock,StepCancelReference::Owned(cancel),JobPayloadLedgerReference::Owned(payload_ledger),now_us,preview_sequence,retained_progress)
    }
    fn assemble(operation: OperationId, generation: Generation, budget: StepBudget, clock: ClockStride, cancel: StepCancelReference<'a>, payload_ledger: JobPayloadLedgerReference<'a>, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64, retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        assert!(retained_progress.fits(budget.retained),"original job recipient exceeds its incoming retained grant");
        let turn_retained_start=*retained_progress;
        StepContext { original_decoder:None,receiving:Default::default(),retained:budget.retained, retained_progress, operation, generation, fuel_remaining: budget.fuel,original_total_fuel:budget.fuel,issued_ceiling:(budget.retained,budget.fuel),issued_start:RetainedCloneProgress::default(),original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,
            work_units_spent:0, semantic_step_limit:None, loan_deadline_us:None, deadline_us: budget.deadline_us, now_us, clock: std::cell::Cell::new(clock), cancel, stage: "initial", preview_sequence, payload_ledger, source_parent:None,source_turn:None,return_budget:None,reserved_return_progress:Default::default(),reserved_return_work:0,birth_work:0,reserved_observer:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:turn_retained_start,original_total_retained_bound:false,retained_policy:None,original_physical_recipient:None,replacement_current:None, payload_page_granted: false }
    }

    pub fn with_payload_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,authority:&'a JobPayloadAuthority)->Result<Self,ValueError>{
        let Some(original)=authority.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority has already closed"))};
        if original.operation!=operation||original.generation!=generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority identity does not match caller turn"))}
        if !retained_progress.fits(budget.retained){return Err(ValueError::invariant("original job recipient exceeds incoming retained grant"))}
        Ok(Self::assemble(operation,generation,budget,ClockStride::new(),StepCancelReference::Borrowed(cancel),JobPayloadLedgerReference::Borrowed(authority),now_us,preview_sequence,retained_progress))
    }

    /// 🎟️ Preserves the original caller depth and independently unspent physical credits.
    /// 🆕️ True while this context has consumed no retained currency since its turn began, so the remaining grant IS the whole grant of the turn.
    /// ⛽️ True while this context has spent no fuel since its turn began, so the remaining fuel IS the whole fuel of the turn.
    /// 🏔️ The retained grant and fuel this turn was issued before any loan narrowed it; a quote above it can never be funded.
    pub fn original_turn_ceiling(&self)->(RetainedCloneGrant,u64){self.issued_ceiling}
    pub fn fuel_turn_untouched(&self)->bool{self.work_units_spent.saturating_sub(u128::from(self.reserved_return_work))<=self.original_total_work_start}
    pub fn retained_turn_untouched(&self)->bool{retained_delta(*self.retained_progress,self.reserved_return_progress)==self.original_total_retained_start}
    /// 🧮️ Returns the spendable grant after applying the turn policy and live Source holds.
    pub fn retained_grant(&self)->RetainedCloneGrant{
        let current=retained_delta(*self.retained_progress,self.reserved_return_progress);
        self.retained_grant_at(current,self.visible_source_remaining().map(|(grant, _)|grant))
    }
    fn retained_grant_at(&self,current:RetainedCloneProgress,source:Option<RetainedCloneGrant>)->RetainedCloneGrant{
        retained_min(self.retained_turn_remaining(current),self.source_total_remaining(current,source))
    }
    fn retained_turn_remaining(&self,current:RetainedCloneProgress)->RetainedCloneGrant{
        let remaining=retained_remaining(self.retained,current);
        self.retained_policy.map_or(remaining,|(policy,start)|retained_min(remaining,retained_remaining(policy,retained_delta(current,start))))
    }
    pub fn with_retained_loan<R:Copy>(&mut self,requested:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let original=self.retained;let posted=retained_delta(self.retained_progress(),self.reserved_return_progress);
        if !posted.fits(original){return Err(ValueError::invariant("original retained loan recipient already exceeds its ceiling").into())}
        let remaining=self.retained_grant();let ceiling=|paid:usize,wanted:usize,available:usize|paid.checked_add(wanted.min(available)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained loan ceiling overflow"));
        let narrowed=RetainedCloneGrant{maximum_items:ceiling(posted.copied_items,requested.maximum_items,remaining.maximum_items)?,maximum_copy_bytes:ceiling(posted.copied_bytes,requested.maximum_copy_bytes,remaining.maximum_copy_bytes)?,maximum_capacity_bytes:ceiling(posted.retained_capacity_bytes,requested.maximum_capacity_bytes,remaining.maximum_capacity_bytes)?,maximum_release_bytes:ceiling(posted.released_bytes,requested.maximum_release_bytes,remaining.maximum_release_bytes)?,maximum_depth:requested.maximum_depth.min(remaining.maximum_depth)};
        let restore=OriginalRetainedLoan{target:std::ptr::addr_of_mut!(*self),original,identity:OriginalRetainedLoanIdentity::capture(self)};self.retained=narrowed;let result=run(self);
        if restore.matches_original(){return result}
        replaced_context(result,"original retained loan callback replaced its initiating context".into())
    }
    pub fn with_original_work_loan<R:Copy>(&mut self,requested_fuel:u64,absolute_deadline_us:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.with_work_loan(requested_fuel, absolute_deadline_us, self.semantic_step_limit, run)
    }

    /// 👣️ Funds original work for one user-requested unit beyond the tool's published progress.
    pub fn with_original_work_loan_for_step<R: Copy>(&mut self, requested_fuel: u64, absolute_deadline_us: u64, completed: u64, run: impl FnOnce(&mut Self) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        let limit = completed.saturating_add(1);
        self.with_work_loan(requested_fuel, absolute_deadline_us, Some(self.semantic_step_limit.map_or(limit, |outer| outer.min(limit))), run)
    }

    /// 🎯️ Absolute completed-unit ceiling for a tool's current single step.
    pub fn semantic_step_limit(&self) -> Option<u64> { self.semantic_step_limit }

    fn with_work_loan<R: Copy>(&mut self, requested_fuel: u64, absolute_deadline_us: u64, semantic_step_limit: Option<u64>, run: impl FnOnce(&mut Self) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        self.with_retained_loan(self.retained_grant(),|cx|{
            let fuel=cx.fuel_remaining;let deadline=cx.deadline_us;let loaned=cx.loan_deadline_us;let issued=requested_fuel.min(cx.fuel_remaining());cx.fuel_remaining=issued;cx.deadline_us=absolute_deadline_us.min(deadline);cx.loan_deadline_us=Some(loaned.map_or(absolute_deadline_us,|held|held.min(absolute_deadline_us)));
            let previous_step_limit = std::mem::replace(&mut cx.semantic_step_limit, semantic_step_limit);
            let restore=OriginalWorkLoan{target:std::ptr::addr_of_mut!(*cx),fuel,semantic_step_limit:previous_step_limit,deadline,loaned,issued,spent:cx.work_units_spent,reserved:cx.reserved_return_work,cancel:cx.original_cancel_token()as*const CancelToken as usize,identity:OriginalRetainedLoanIdentity::capture(cx)};let result=run(cx);
            if restore.matches_original(){return result}
            let message=restore.refusal_message();drop(restore);
            replaced_context(result,message)
        })
    }
    pub fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
        let admitted=self.retained_recipient_is_admitted()&&progress.fits(self.retained_grant());
        self.post_retained(progress,admitted)
    }
    /// ⚖️ Settles a retirement step's declared release against the births this thread recorded; the settled bytes leave this receipt and its Source ledger and return for the step's own receipt.
    pub fn settle_original_birth(&mut self,declared:usize,before:RetainedCloneProgress)->Result<usize,ValueError>{
        let settled=semio_framework_value::birth_ledger::settle_original_birth(declared);
        let refund=settled.min(self.retained_progress.released_bytes.saturating_sub(before.released_bytes));
        if refund!=0{
            self.retained_progress.released_bytes-=refund;
            if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().refund_released(refund)?;}
        }
        Ok(settled)
    }
    /// 💳️ Admits and posts one retained item and one unit of work using the live Source grant and the ordinary deadline.
    ///
    /// An item is one actual retained copy, allocation or release event; its bytes are the bytes really moved, born or freed. Instruction progress is [`Self::charge_work_unit`].
    pub fn charge_retained_item(&mut self,copy:usize,capacity:usize,release:usize)->Result<bool,ValueError>{
        self.charge_item(copy,capacity,release,false)
    }
    /// 📏️ Posts `units` of work under the live Source fuel and the ordinary deadline without any retained effect: guest-internal moves cost work proportional to the bytes moved, never the copy ledger.
    pub fn charge_work_units(&mut self,units:u64)->bool{
        if units==0{return true}
        if self.fuel_remaining()<units||self.deadline_exceeded(){return false}
        self.consume_fuel(units);
        true
    }
    /// ⚙️ Admits and posts one unit of work under the live Source fuel and the ordinary deadline without posting any retained item, copy, capacity or release.
    ///
    /// Scalar instruction progress mutates storage that is already born, so it consumes work (the yield and deadline cadence) and never the retained ledger.
    pub fn charge_work_unit(&mut self)->bool{
        if self.fuel_remaining()==0||self.deadline_exceeded(){return false}
        self.consume_fuel(1);
        true
    }
    /// ♻️ Admits and posts one retained item and one unit of work, honoring an active original retirement loan.
    pub fn charge_retirement_retained_item(&mut self,copy:usize,capacity:usize,release:usize)->Result<bool,ValueError>{
        self.charge_item(copy,capacity,release,self.original_retirement)
    }
    fn charge_item(&mut self,copy:usize,capacity:usize,release:usize,bypass_deadline:bool)->Result<bool,ValueError>{
        let current=retained_delta(*self.retained_progress,self.reserved_return_progress);
        let source=self.visible_source_remaining();
        let grant=self.retained_grant_at(current,source.map(|(grant, _)|grant));
        let fuel=source.map_or(self.fuel_remaining,|(_, fuel)|self.fuel_remaining.min(fuel));
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:release};
        if fuel==0||(!bypass_deadline&&self.deadline_exceeded())||grant.maximum_depth==0||!progress.fits(grant){return Ok(false)}
        self.post_retained(progress,current.fits(self.retained))?;
        self.consume_fuel(1);
        Ok(true)
    }
    fn post_retained(&mut self,progress:RetainedCloneProgress,admitted:bool)->Result<(),ValueError>{
        if progress.copied_bytes!=0&&DBG_POST_ON.load(std::sync::atomic::Ordering::Relaxed){dbg_post(progress.copied_bytes);}
        *self.retained_progress=self.retained_progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;
        let source=self.record_original_source_retained(progress);
        if !admitted{return Err(ValueError::invariant("job child receipt exceeds original remaining retained grant").with_retained_progress(progress));}
        source
    }
    pub(crate) fn record_original_source_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().record_retained(progress)?;}Ok(())}
    pub fn retained_progress(&self)->RetainedCloneProgress{*self.retained_progress}
    /// 🧮️ Measures the fresh retained effects posted since the given receipt and reserved-return observations.
    pub fn fresh_retained_progress_since(&self, before: RetainedCloneProgress, reserved_before: RetainedCloneProgress) -> Result<RetainedCloneProgress, ValueError> {
        let subtract = semio_framework_job_return_budget::source_fresh_progress;
        subtract(self.retained_progress(), before)
            .zip(subtract(self.reserved_return_progress(), reserved_before))
            .and_then(|(total, reserved)| subtract(total, reserved))
            .ok_or_else(|| ValueError::invariant("fresh retained progress must preserve the observed reserved return span"))
    }
    /// 🪙️ Runs one closure inside one original effect settled against this turn's own fuel, up to all of it (a synchronous [`semio_framework_value_control_contract::with_original_invocation`] charges no handoff); `None` means the Work quote was not admitted and nothing ran.
    pub fn with_original_effect<R>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut Self,&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->R)->Result<Option<(R,u128)>,ValueError>{self.with_original_effect_at(None,grant,run)}
    /// 🪙️ Runs one closure inside one original effect whose Work carries this resident Source's own turn.
    pub fn with_original_source_effect<R>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut Self,&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->R)->Result<Option<(R,u128)>,ValueError>{let turn=self.original_source_turn_identity().ok_or_else(||ValueError::invariant("original Source turn is absent"))?;self.with_original_effect_at(Some(turn),grant,run)}
    fn with_original_effect_at<R>(&mut self,source:Option<OriginalSourceTurnIdentity>,grant:RetainedCloneGrant,run:impl FnOnce(&mut Self,&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->R)->Result<Option<(R,u128)>,ValueError>{
        use semio_framework_value_control_contract::{OriginalWork,OriginalWorkAuthority,OriginalWorkSink,RetainedAllocationEffect};
        let fuel=self.fuel_remaining;let maximum=fuel;let retirement=self.original_retirement;let deadline=self.deadline_us;let now=self.now_us;
        let mut ready=move||retirement||now().is_some_and(|value|value<deadline);
        let mut authority=OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
        let result=authority.with_loan(&mut semio_framework_job_receiving::OriginalJobWorkRecipient::new(&fuel,&mut ready).with_source(source),|loan|{
            let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;
            if work.prepare_for_action(maximum,&mut action)?!=Some(true){if action.is_some(){work.cancel_for_action(&mut action)?}return Ok(None)}
            let mut receipt=None;let mut partial=None;
            let mut exchange=||->Result<Option<(R,u128)>,ValueError>{
            let absent=||ValueError::invariant("original effect receipt absent");
            work.issue_performed_for_action(action,&mut receipt)?;let mut actual=None;
            let output={let sink=receipt.as_mut().ok_or_else(absent)?;let mut effect=RetainedAllocationEffect::new(grant,maximum,&mut actual,sink);let mut run=Some(run);let mut output=None;{let mut call=|effect:&mut RetainedAllocationEffect<'_>|{if let Some(run)=run.take(){output=Some(run(&mut *self,effect))}};semio_framework_value_control_contract::with_original_invocation(source,&mut effect,&mut call)?;}output.ok_or_else(||ValueError::invariant("original effect call did not run"))?};
            let performed=OriginalWorkSink::observed_work(receipt.as_ref().ok_or_else(absent)?);work.claim_performed_for_action(action,&mut receipt)?;
            if !work.performed_seen_for_action(action)?{work.cancel_for_action(&mut action)?}else{
                if !work.settle_partial_for_action_into(action,&mut partial)?{return Err(ValueError::invariant("original effect partial receipt is pending"))}
                let partial=partial.take().ok_or_else(||ValueError::invariant("original effect partial outcome absent"))?;if partial.total()!=performed||partial.source_refusal().is_some(){return Err(ValueError::invariant("original effect partial outcome differs from its performed work"))}
                let accepted=work.commit_for_action(performed,action,&mut proof)?;work.finish_for_action(&mut proof,&mut action)?;if !accepted{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect performed more work than its ceiling"))}
            }
                Ok(Some((output,performed)))
            };
            match exchange(){Ok(done)=>Ok(done),Err(error)=>{
                if receipt.is_some(){let _=work.claim_performed_for_action(action,&mut receipt);}
                if action.is_some(){
                    if proof.is_some(){let _=work.finish_for_action(&mut proof,&mut action);}
                    else if work.performed_seen_for_action(action).unwrap_or(false){
                        if work.partial_pending_for_action(action).unwrap_or(false){let mut outcome=None;let _=work.settle_partial_for_action_into(action,&mut outcome);}
                        if let Ok(total)=work.performed_total_for_action(action){let _=work.commit_for_action(total,action,&mut proof);let _=work.finish_for_action(&mut proof,&mut action);}
                    }else{let _=work.cancel_for_action(&mut action);}
                }
                Err(error)
            }}
        })??;
        if authority.has_live_actions(){return Err(ValueError::invariant("original effect left a live work action"))}
        Ok(result)
    }
    pub fn retained_work_can_enter(&self,demand:RetirementDemand)->bool{retained_work::RetainedWorkBudget::new(self.retained_grant()).can_enter(demand)}
    /// 🚧️ Admits one work step against this turn's grant: `Ok(false)` retries on a fresh turn; a demand no untouched turn fits is refused typed, never idled.
    /// 🧾️ Returns the grant issued for the open original receiving scope.
    pub fn receiving_issued_grant(&self)->Option<RetainedCloneGrant>{self.receiving.issued()}
    /// 📥️ Returns physical spend already accepted inside the open receiving scope.
    pub fn receiving_child_progress(&self)->RetainedCloneProgress{self.receiving.spent()}
    /// 📩️ Closes a receiving scope whose semantic receipt equals its accepted physical spend.
    pub fn settle_receiving_scope(&mut self)->Result<(),ValueError>{match self.receiving.issued(){Some(issued)=>{let spent=self.receiving.spent();self.consume_receiving_retained(issued,spent)}None=>Ok(())}}
    /// 🧾 Debits the enclosing semantic receipt after its original construction child accepted physical spend.
    pub fn consume_receiving_retained(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{let delta=self.receiving.settle(grant,progress)?;self.consume_retained(delta)}
    /// 🎟 Captures the replacement identity installed for this turn.
    pub fn replacement_capture(&self)->Option<ReplacementIdentity>{self.replacement_current}
    /// 🕰️ Reports whether a stored replacement identity is stale against the current one.
    pub fn replacement_is_stale(&self,stored:Option<ReplacementIdentity>)->bool{ReplacementIdentity::is_stale(stored,self.replacement_current)}
    /// 🔄 Installs a replacement identity for the duration of `run`, then restores the previous one.
    pub fn with_replacement_current<R>(&mut self,current:ReplacementIdentity,run:impl FnOnce(&mut Self)->R)->R{let previous=self.replacement_current.replace(current);let result=run(self);self.replacement_current=previous;result}
    pub fn admit_work_step(&self,demand:RetirementDemand)->Result<bool,ValueError>{
        if self.retained_work_can_enter(demand){return Ok(true)}
        if !retained_work::RetainedWorkBudget::new(self.original_total_retained_grant()).can_enter(demand){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"step demand exceeds the whole remaining grant of its operation and can never fit"))}
        Ok(false)
    }
    pub fn retained_recipient_is_admitted(&self)->bool{retained_delta(self.retained_progress(),self.reserved_return_progress).fits(self.retained)}

    pub fn original_recipient_identity(&self)->usize{self.retained_progress as*const RetainedCloneProgress as usize}
    pub fn payload_authority_identity(&self)->OriginalPayloadSourceIdentity{OriginalPayloadSourceIdentity(Arc::as_ptr(&self.payload_ledger)as usize)}

    pub fn clock_source(&self)->fn()->Option<u64>{self.now_us}
    pub fn clock_stride(&self)->ClockStride{self.clock.get()}
    pub fn restore_clock_stride(&self,stride:ClockStride){self.clock.set(stride)}
    pub fn observe_step_clock(&self,end:Option<u64>){let mut clock=self.clock.get();clock.begin(end);self.clock.set(clock);}

    pub fn original_source_epoch(&self)->Option<u64>{self.source_epoch}

    pub(crate) fn attach_original_return_budget(&mut self,budget:original_job_continuation::OriginalReturnBudgetReference<'a>)->Result<(),ValueError>{if budget.get().is_bound(){let(retained,fuel)=budget.get().remaining()?;self.original_total_retained=retained;self.original_total_retained_start=retained_delta(*self.retained_progress,self.reserved_return_progress);self.original_total_fuel=fuel;self.original_total_work_start=self.work_units_spent.saturating_sub(u128::from(self.reserved_return_work));}self.return_budget=Some(budget);Ok(())}

    pub fn original_clock_binding(&self)->Option<u64>{self.original_clock_binding}

    pub fn original_total_fuel_remaining(&self)->u64{let work=self.work_units_spent.saturating_sub(u128::from(self.reserved_return_work));let remaining=self.original_total_fuel.saturating_sub(u64::try_from(work.saturating_sub(self.original_total_work_start)).unwrap_or(u64::MAX));self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(remaining,|budget|remaining.min(budget.get().remaining().map_or(0,|remaining|remaining.1)))}

    pub fn original_absolute_deadline_us(&self)->u64{self.original_absolute_deadline}

    pub fn original_total_retained_grant(&self)->RetainedCloneGrant{let current=retained_delta(*self.retained_progress,self.reserved_return_progress);let remaining=retained_remaining(self.original_total_retained,retained_delta(current,self.original_total_retained_start));let turn=retained_min(remaining,retained_remaining(self.issued_ceiling.0,retained_delta(current,self.issued_start)));self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(turn,|budget|retained_min(turn,budget.get().remaining().map_or(RetainedCloneGrant::default(),|remaining|remaining.0)))}
    /// 🏛️ The Source's own remaining total, independent of this turn's issued ceiling, so a narrow turn is never mistaken for a permanent misfit.
    pub fn original_source_total_grant(&self)->RetainedCloneGrant{
        let current=retained_delta(*self.retained_progress,self.reserved_return_progress);
        self.source_total_remaining(current,self.visible_source_remaining().map(|(grant, _)|grant))
    }
    fn visible_source_remaining(&self)->Option<(RetainedCloneGrant,u64)>{
        let source=self.return_budget.as_ref()?.get();
        source.is_bound().then(||source.remaining().unwrap_or_default())
    }
    fn source_total_remaining(&self,current:RetainedCloneProgress,source:Option<RetainedCloneGrant>)->RetainedCloneGrant{
        let remaining=retained_remaining(self.original_total_retained,retained_delta(current,self.original_total_retained_start));
        source.map_or(remaining,|source|retained_min(remaining,source))
    }

    pub fn bind_original_clock_binding(&mut self,binding:u64)->Result<(),ValueError>{self.original_source_parent()?.validate_borrowed_context(self).map_err(|_|ValueError::invariant("original clock binding requires its genuine source context"))?;if binding==0||self.original_clock_binding.is_some_and(|original|original!=binding){return Err(ValueError::invariant("original source clock authority cannot be absent or rebound"))}self.original_clock_binding=Some(binding);Ok(())}

    pub fn bind_original_source_epoch(&mut self,epoch:u64)->Result<(),ValueError>{
        self.original_source_parent()?;
        if epoch==0||self.source_epoch.is_some_and(|original|original!=epoch){return Err(ValueError::invariant("original source epoch cannot be absent or rebound"))}
        self.source_epoch=Some(epoch);Ok(())
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    /// 🔖️ Borrows the actual Source-issued dispatch token retained by this caller.
    pub fn original_return_budget_is_borrowed(&self)->bool{matches!(&self.return_budget,Some(original_job_continuation::source::OriginalReturnBudgetReference::Borrowed(_)))}
    /// 🔖️ Borrows the actual Source-issued dispatch token retained by this caller.
    pub fn original_source_turn(&self)->Result<&OriginalSourceTurn,ValueError>{self.source_turn.as_ref().map(SourceTurnReference::token).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller context has no original Source dispatch"))}
    /// 🎟️ Lends the actual dispatch witness to a native call that also exclusively borrows this context.
    ///
    /// # Safety
    /// The token lives inline in this context and is neither moved nor replaced while the returned witness is used; the caller must drop the witness before this context ends and must not touch the token through any exclusive path.
    pub unsafe fn original_source_turn_witness_detached<'turn>(&self)->Result<OriginalSourceTurnWitness<'turn>,ValueError>{let token=std::ptr::from_ref(self.original_source_turn()?);Ok(unsafe{&*token}.witness())}
    /// 🧵️ Lends the original dispatch to a shorter native continuation turn.
    pub fn original_source_turn_witness(&self)->Result<OriginalSourceTurnWitness<'_>,ValueError>{Ok(self.original_source_turn()?.witness())}
    /// 🪪️ Observes the opaque dispatch identity without creating another token.
    pub fn original_source_turn_identity(&self)->Option<OriginalSourceTurnIdentity>{self.source_turn.as_ref().map(|turn|turn.token().identity())}
    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn original_cancel_token(&self)->&CancelToken{&self.cancel}

    pub fn borrowed_original_cancel_token(&self)->Result<&'a CancelToken,ValueError>{match &self.cancel{StepCancelReference::Borrowed(original)|StepCancelReference::Observed(original)=>Ok(*original),StepCancelReference::Owned(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native job controls require an original borrowed cancellation witness"))}}

    /// 🧾️ Borrows the live return budget loan of this context.
    pub fn original_return_budget(&self)->Option<&original_return_source::OriginalReturnBudgetReference<'a>>{self.return_budget.as_ref()}
    /// 🧾️ Borrows the live return budget loan of this context exclusively.
    pub fn original_return_budget_mut(&mut self)->Option<&mut original_return_source::OriginalReturnBudgetReference<'a>>{self.return_budget.as_mut()}
    pub fn original_source_parent(&self)->Result<&'a OriginalCallerSourceParent,ValueError>{self.source_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original caller has no parent source supplier"))}

    pub fn borrowed_original_caller_source(&self)->Result<OriginalCallerSource<'a>,ValueError>{let ledger=match &self.payload_ledger{JobPayloadLedgerReference::Borrowed(authority)=>authority.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original caller ledger has already closed"))?,JobPayloadLedgerReference::Owned(_)=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller capture requires its admitted borrowed original ledger"))};Ok(OriginalCallerSource{ledger,cancel:self.borrowed_original_cancel_token()?,operation:self.operation,generation:self.generation,clock:self.now_us,deadline:self.deadline_us})}

    pub fn admit_original_cancel_alias(&mut self,output:&mut Option<CancelToken>)->Result<bool,OriginalTurnFailure>{
        self.original_source_parent()?.validate_borrowed_context(self)?;if let Some(alias)=output{if !alias.is_same_node(self.original_cancel_token()){return Err(ValueError::invariant("original cancellation alias slot belongs to another source").into())}return Ok(true)}let grant=self.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||self.is_cancelled()||self.should_yield(){return Ok(false)}let turn=self.original_cancel_token().admit_alias(grant);let result=match turn.result{Ok(Some((alias,progress)))=>{*output=Some(alias);Ok(RetainedCloneStep::Progress(progress))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};
        #[cfg(any(test,feature="testkit"))]if output.is_some(){crate::testkit::narrow_cancel_receipt(self);}
        receive_original_retained_work(self,semio_framework_value_retained_clone::RetainedWorkOutcome{result,work:turn.work})?;Ok(output.is_some())
    }

    /// 🥇️ first one).
    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn now_us(&self) -> Option<u64> {
        let mut clock = self.clock.get();
        let now = clock.read(self.now_us);
        self.clock.set(clock);
        now
    }

    pub fn latest_us(&self) -> Option<u64> {
        self.clock.get().latest_us()
    }

    pub fn deadline_us(&self) -> u64 {
        self.deadline_us
    }

    pub fn deadline_exceeded(&self) -> bool {
        self.now_us().is_none_or(|now_us| now_us >= self.deadline_us)
    }

    pub fn narrow_original_work_budget(&mut self,budget:StepBudget)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||!retained_is_subset(budget.retained,self.retained_grant())||budget.fuel>self.fuel_remaining||budget.deadline_us>self.deadline_us{return Err(ValueError::invariant("original work view cannot change source currencies or increase its work/deadline"))}self.retained_policy=Some((budget.retained,retained_delta(*self.retained_progress,self.reserved_return_progress)));self.fuel_remaining=budget.fuel;self.deadline_us=budget.deadline_us;Ok(())}
    /// 🧷️ Binds the caller's total against the physical ledger and preserves any earlier narrowing.
    pub fn bind_original_retained_total(&mut self,total:RetainedCloneGrant)->Result<(),ValueError>{
        self.original_source_parent()?;
        let current=retained_delta(*self.retained_progress,self.reserved_return_progress);
        let mut ceiling=self.retained_turn_remaining(current);
        if self.original_total_retained_bound {
            let bound=retained_remaining(self.original_total_retained,retained_delta(current,self.original_total_retained_start));
            ceiling=retained_min(ceiling,bound);
        }
        if !retained_is_subset(total,ceiling) {
            return Err(ValueError::invariant("original retained total exceeds its physical source or narrowed authority"));
        }
        self.original_total_retained=total;
        self.original_total_retained_start=current;
        self.original_total_retained_bound=true;
        Ok(())
    }
    pub fn bind_original_physical_recipient(&mut self,recipient:&'a mut RetainedCloneProgress)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_some(){return Err(ValueError::invariant("original physical delta already has its external recipient"))}self.original_physical_recipient=Some((recipient,*self.retained_progress));Ok(())}
    pub fn bind_original_work_recipient(&mut self,recipient:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||self.original_work_recipient.is_some(){return Err(ValueError::invariant("original work return requires its one source-bound clock and external recipient"))}self.original_work_recipient=Some(recipient);Ok(())}
    pub fn original_work_units_spent(&self)->u128{self.work_units_spent.saturating_sub(self.birth_work)}
    pub fn original_birth_baseline(&self)->OriginalBirthBaseline{(self.original_work_units_spent(),*self.retained_progress,self.reserved_return_work,self.reserved_return_progress)}
    /// 🌱️ Takes the birth service's fresh work and retention since the baseline out of the business view; redeemed reservations stay, the reserved observer subtracts them.
    pub fn exclude_original_birth(&mut self,(work,retained,redeemed,reserved):OriginalBirthBaseline){
        self.birth_work+=self.original_work_units_spent().saturating_sub(work+u128::from(self.reserved_return_work-redeemed));
        let spent=retained_delta(retained_delta(*self.retained_progress,retained),retained_delta(self.reserved_return_progress,reserved));
        if let Ok(spent)=spent.checked_add(self.original_total_retained_start){self.original_total_retained_start=spent}
    }
    pub fn reserved_return_progress(&self)->RetainedCloneProgress{self.reserved_return_progress}
    /// 🏠️ Marks this Source's return budget as served by a host that never hands published headers back, so settled return rows are acknowledged locally.
    pub fn acknowledge_return_rows_locally(&mut self){if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().local_acknowledgement=true}}
    /// 🔬️ Temporary: the escrow table summary of this Source's return budget.
    /// 🔬️ Temporary: why the last sized return claim answered false.
    pub fn last_claim_reason(&self)->&'static str{crate::original_job_continuation::source::claim_reason()}
    /// 🏁️ The aggregate physical progress of the context when this call's external recipient was bound, before any birth or refresh paid for the call.
    pub fn original_call_start_progress(&self)->Option<RetainedCloneProgress>{self.original_physical_recipient.as_ref().map(|(_,start)|*start)}
    pub fn reserved_return_work(&self)->u64{self.reserved_return_work}
    /// ⛽️ This turn's own fuel after its loans settled; unlike the Source ledger it never rises when return claims release their holds.
    pub fn original_turn_fuel_remaining(&self)->u64{self.fuel_remaining}
    pub fn original_source_fuel_remaining(&self)->u64{self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(self.fuel_remaining,|budget|budget.get().raw_remaining().map_or(0,|remaining|remaining.1))}
    /// 🧾️ Observes authentic redemptions in the persistent actor without narrowing their aggregate wire receipt.
    pub fn bind_original_reserved_recipient(&mut self,progress:&'a mut RetainedCloneProgress,work:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_none()||self.original_work_recipient.is_none()||self.reserved_observer.is_some()||self.reserved_return_progress!=Default::default()||self.reserved_return_work!=0{return Err(ValueError::invariant("reserved observer must bind before any redemption"))}self.reserved_observer=Some((progress,work));Ok(())}
    pub fn preflight_original_reserved_return(&self,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError>{if let Some((recipient,start))=&self.original_physical_recipient{let delta=semio_framework_job_return_budget::source_fresh_progress(*self.retained_progress,*start).ok_or_else(||ValueError::invariant("original aggregate observer baseline changed"))?;recipient.checked_add(delta)?.checked_add(progress)?;}if let Some(recipient)=&self.original_work_recipient{recipient.checked_add(self.work_units_spent).and_then(|n|n.checked_add(u128::from(work))).ok_or_else(||ValueError::invariant("original aggregate work observer overflow"))?;}if let Some((p,w))=&self.reserved_observer{p.checked_add(progress)?;w.checked_add(u128::from(work)).ok_or_else(||ValueError::invariant("reserved observer work overflow"))?;}self.retained_progress.checked_add(progress)?;self.reserved_return_progress.checked_add(progress)?;self.reserved_return_work.checked_add(work).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original reserved return CPU observer overflow"))?;self.work_units_spent.checked_add(u128::from(work)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return actual CPU counter overflow"))?;Ok(())}
    pub fn receive_original_reserved_return(&mut self,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError>{
        self.preflight_original_reserved_return(progress,work)?;
        let recipient=self.retained_progress.checked_add(progress)?;let reserved=self.reserved_return_progress.checked_add(progress)?;let reserved_work=self.reserved_return_work.checked_add(work).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original reserved return CPU observer overflow"))?;let total_work=self.work_units_spent.checked_add(u128::from(work)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return actual CPU counter overflow"))?;
        if let Some((p,w))=self.reserved_observer.as_mut(){**p=p.checked_add(progress)?;**w=w.checked_add(u128::from(work)).ok_or_else(||ValueError::invariant("reserved observer work overflow"))?;}*self.retained_progress=recipient;self.reserved_return_progress=reserved;self.reserved_return_work=reserved_work;self.work_units_spent=total_work;Ok(())
    }

    pub fn fuel_remaining(&self) -> u64 {
        self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(self.fuel_remaining,|budget|self.fuel_remaining.min(budget.get().remaining().map_or(0,|remaining|remaining.1)))
    }

    /// ⛽️ Debits one settled Work receipt from this turn's fuel, saturating at the scalar fuel width.
    pub fn consume_work(&mut self,work:u128){self.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX))}
    pub fn consume_fuel(&mut self, units: u64) {
        if units > 1 {
            self.clock.get_mut().note_work(units);
        }
        self.work_units_spent+=u128::from(units);self.fuel_remaining = self.fuel_remaining.saturating_sub(units);if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().record_work(units);}
    }

    /// ⏱️ Settles the full original performed-work receipt without narrowing its aggregate.
    pub(crate) fn settle_original_work_exact(&mut self,units:u128)->Result<(),ValueError>{let total=self.work_units_spent.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original Source Work total overflow"))?;self.work_units_spent=total;self.fuel_remaining=if units>=u128::from(self.fuel_remaining){0}else{self.fuel_remaining-u64::try_from(units).expect("bounded by actual Source fuel")};if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().record_work_exact(units)?}Ok(())}

    pub fn fuel_exhausted(&self) -> bool {
        self.fuel_remaining() == 0
    }

    pub fn should_yield(&self) -> bool {
        self.fuel_exhausted() || self.deadline_exceeded()
    }

    pub fn retirement_should_yield(&self)->bool{self.fuel_exhausted()||(!self.original_retirement&&self.deadline_exceeded())}

    pub fn original_retirement_active(&self)->bool{self.original_retirement}

    pub fn with_original_retirement<T>(&mut self,run:impl FnOnce(&mut Self)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{self.original_source_parent()?.validate_borrowed_context(self)?;let restore=OriginalRetirementLoan{target:std::ptr::addr_of_mut!(*self),original:self.original_retirement,identity:OriginalRetainedLoanIdentity::capture(self)};self.original_retirement=true;let result=run(self);drop(restore);result}

    /// 🎨️ Checks this step or ancestor cancellation; jobs check on entry and after every bounded unit.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.cancel,StepCancelReference::Observed(_))||self.cancel.is_cancelled_now()
    }

    pub fn observe_original_cancellation(&mut self)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none(){return Err(ValueError::invariant("original cancellation observation requires its bound source clock"))}let original=self.borrowed_original_cancel_token()?;self.cancel=StepCancelReference::Observed(original);Ok(())}

    /// 👶️ Clones the cheap cancellation token for a child scope or submitted work.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// 👣️ Records a stage change using the latest clock reading; terminal events are recorded by [`drive_step`].
    pub fn set_stage(&mut self, label: &'static str) -> Option<TraceEvent> {
        self.stage = label;
        let at_us = self.latest_us().or_else(|| self.now_us());
        semio_framework_trace::record_trace_event_at(self.operation, self.generation, semio_framework_trace::TraceStage::StageChanged { label }, at_us)
    }

    /// 🖼️ [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
    /// 🔢️ Advances the preview sequence shared across a retained operation’s steps.
    pub fn preview_sequence(&self) -> u64 { *self.preview_sequence }

    pub fn next_preview_sequence(&mut self) -> Result<u64, JobSequenceExhausted> {
        let sequence = *self.preview_sequence;
        *self.preview_sequence = (*self.preview_sequence).checked_add(1).ok_or(JobSequenceExhausted::Preview)?;
        Ok(sequence)
    }

}
//#endregion 🧭️StepContext

//#region 🐕️Drive
/// ▶️ Runs one watched step, prechecks cancellation, records its terminal trace and advances the retained preview sequence.
//#endregion 🐕️Drive

#[cfg(test)]
pub(crate) const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

//#region 🧪️Tests

#[cfg(any(test,feature="testkit"))]
#[path="🧪️testkit/🦀️.rs"]
pub mod testkit;
//#endregion 🧪️Tests


#[path="📦️return/🎟️place/🦀️.rs"]
mod original_return_place;
pub use original_return_place::{OriginalCallerReturnPlace,OriginalSizedClaimReserve};

#[path="⚠️failure/🦀️.rs"]
mod original_failure_context;






pub use semio_framework_job_return_escrow_publication::OriginalEscrowHeaderLoanCell;

/// 🛫️ An owned snapshot of an original Job caller's Work for detached encoder hops; [`NativeEncoderWork::performed`] is charged back with `StepContext::consume_fuel`.
pub struct NativeEncoderWork{fuel:u64,deadline:u64,now:fn()->Option<u64>,cancel:CancelToken,performed:u64}
impl NativeEncoderWork{
    /// 🪢️ Loans the snapshot's Work to the supplied encoder for one hop and records the work the hop performed.
    pub fn lend<T,E:From<ValueError>>(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let fuel=self.fuel.saturating_sub(self.performed);let deadline=self.deadline;let now=self.now;let cancel=&self.cancel;let mut ready=move||!cancel.is_cancelled_now()&&now().is_some_and(|value|value<deadline);
        let before=native.actual_work();
        let mut authority=semio_framework_value_control_contract::OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
        let result=authority.with_loan(&mut semio_framework_job_receiving::OriginalJobWorkRecipient::new(&fuel,&mut ready),|work_loan|native.with_work_recipient(work_loan,operation)).map_err(E::from)?;
        let performed=native.actual_work().saturating_sub(before);self.performed=self.performed.saturating_add(u64::try_from(performed).unwrap_or(u64::MAX));result
    }
    /// 🧾️ The work every lent hop performed so far.
    pub fn performed(&self)->u64{self.performed}
}

/// 🔏️ The private custody surface the facade mounts for the Source children and return places built over the step context.
#[doc(hidden)]
pub mod sealed{pub use super::owned_source::{SourceRoots,refused,delta,metadata,metadata_fits,cancel_refusal};}

pub static DBG_POST_ON:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
static DBG_POST_N:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);
static DBG_POST:std::sync::Mutex<std::collections::BTreeMap<String,(u64,u64)>>=std::sync::Mutex::new(std::collections::BTreeMap::new());
fn dbg_post(bytes:usize){
    let n=DBG_POST_N.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if n%8!=0{return}
    let bt=std::backtrace::Backtrace::force_capture().to_string();
    let mut frames:Vec<String>=Vec::new();
    for line in bt.lines(){let t=line.trim();if let Some((idx,rest))=t.split_once(": "){if idx.chars().all(|c|c.is_ascii_digit()){let name=rest.rsplit_once("::h").map_or(rest,|x|x.0).to_string();if name.contains("dbg_post")||name.contains("post_retained")||name.contains("Backtrace")||name.contains("charge_item")||name.contains("charge_retained")||name.contains("core::")||name.contains("alloc::")||name.contains("std::"){continue}frames.push(name.rsplit("::").take(2).collect::<Vec<_>>().join("<"));if frames.len()>=5{break}}}}
    let key=frames.join(" < ");
    let mut m=DBG_POST.lock().unwrap();let e=m.entry(key).or_insert((0,0));e.0+=8;e.1+=8*bytes as u64;
}
pub fn dbg_post_take()->Vec<String>{let mut m=DBG_POST.lock().unwrap();let mut v:Vec<_>=m.iter().map(|(k,v)|(v.1,v.0,k.clone())).collect();v.sort();v.reverse();m.clear();v.into_iter().take(30).map(|(b,n,k)|format!("[DEBUG] post bytes={b} n={n} {k}")).collect()}

