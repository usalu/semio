//! 🧵️ Resumable interactive job protocol and its original caller authority.
//!
//! See `README.md`.

pub use semio_framework_job_receiving::OriginalNativeReceivingOutcome;
pub use semio_framework_job_replacement::{ReplacementIdentity,ReplacementStaleness};
use semio_framework_job_receiving::{OriginalJobWorkRecipient,retained_delta,retained_remaining,retained_min,retained_is_subset};
use std::future::Future;
use std::mem::{ManuallyDrop, MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
pub use semio_framework_value_retained_clone::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value_retained_clone::RetirementDemand;
pub use semio_framework_value_control_contract::{OriginalSourceTurn,OriginalSourceTurnIssuer,OriginalSourceTurnIdentity,OriginalSourceTurnWitness};

use semio_framework_value_retained_clone::{ValueError, ValueRefusalKind};
#[path="../📬️outcome/⏸️continuation/🦀️.rs"]
pub(crate) mod original_job_continuation;
pub use original_job_continuation::{OriginalJobReturnOrigin,OriginalJobReturnReceiver,OriginalJobReturnClaim,InteractiveJobStep,OriginalJobReturn,OriginalJobContinuation,OriginalJobContinuationView,OriginalJobOutcomeReturn,OriginalJobOutcome};
pub use semio_framework_job_failure::{OriginalFailureContext,OriginalWorkFailure,OriginalTurnFailure,OriginalTurnPrimary,OriginalReceivingPrimary,OriginalRetirementReceivingPrimary,OriginalClosureFailure,OriginalDomainFailure,OriginalDomainPrimary,OriginalDomainReservation,OriginalDomainProjection,OriginalDomainView,admit_original_domain_failure,original_domain_failure_demand,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault,project_original_retirement_failure,key_original_retirement_failure,project_original_retirement_receiving,key_original_retirement_receiving,project_original_receiving_primary,key_original_receiving_primary};
#[path="../♻️retirement/🎟️context/🦀️.rs"]
mod original_close_context;
pub use original_close_context::{close_original_quoted_retained_step,receive_original_retained_work,receive_original_receiving_work,close_original_granted_step,close_original_nested_step,close_original_retained_step,close_original_retained_turn};

#[cfg(test)]
use std::time::Instant;

use semio_framework_async::ChannelPolicy;
use semio_framework_trace::{TraceEvent, Watchdog};
#[cfg(test)] use semio_framework_trace::{record_operation_started};

pub use semio_framework_async::CancelToken;
pub use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
pub use semio_framework_trace::{Generation, InteractiveStage, OperationId, allocate_operation_id, allocate_operation_id_in_slot, interactive_step_contract_violated, runtime_diagnostics_enabled, set_runtime_diagnostics, INTERACTIVE_STEP_CEILING_US, RUNTIME_DIAGNOSTICS_ENV, SUSTAINED_OVERRUN_QUARANTINE_STEPS};

pub use semio_framework_job_payload::{JobPayloadStep, JobPayloadAuthority, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadStream, JobPayloadAdmissionFault, JobPayloadRejectedPage, RetainedJobPayload, RetainedJobPayloadReader, RetainedPayloadBuilder, OriginalPayloadSourceIdentity, OriginalPayloadBuilderIdentity, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_PROCESS_BYTES};

pub use semio_framework_job_clock::{BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US, CommitValidation, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US, JobSequenceExhausted, MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US, Operation, RevisionId, StepBudget, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US, child_cancel_token, default_now_ms, default_now_us, install_microsecond_clock, logical_now_us, microseconds_from_milliseconds, root_cancel_token, validate_commit};

#[path = "../🚦️clock-stride-and-job-protocol/🦀️.rs"]
mod clock_stride_and_job_protocol;
pub use clock_stride_and_job_protocol::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, StepOutcome};
pub use semio_framework_job_clock::ClockStride;
pub use semio_framework_job_clock::{OriginalClockAction,OriginalClockAuthority,OriginalClockDeadline,OriginalClockObservation,OriginalClockDecisionFrame,OriginalClockDecisionStatus,OriginalClockProducer,OriginalClockReply,OriginalClockReplyComplete,OriginalSystemClockContext};
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
pub use semio_framework_job_clock::OriginalSystemClock;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use clock_stride_and_job_protocol::JobThreadTransfer;
#[cfg(not(all(target_arch="wasm32",not(target_env="p2"))))]
pub use clock_stride_and_job_protocol::JobThreadTransfer;

//#region 🕰️Clock

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_async::install_browser_monotonic_clock;

//#endregion 🕰️Clock

//#region 📄️RetainedPayload

pub use semio_framework_job_outcome::{close_step_outcome_slot,step_outcome_slot_retirement_demands,JobOutcomeSlot,JobPayloadSlot};

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
#[path="📸️capture/🦀️.rs"]
mod original_caller_capture;
pub use original_caller_capture::{OriginalCallerCapture,CapturedOriginalCaller,OriginalCallerIssuer,OriginalCallerTurn,OriginalCallerCloseReturn,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalCallerChildSource,OriginalChildTurnReturn,ORIGINAL_NATIVE_SOURCE_CAPACITY,OriginalNativeLease,OriginalNativeRetirement,OriginalNativeBorrow,OriginalNativeBorrowed,OriginalNativeRecipientIdentity,OriginalNativeRecipientClosed,OriginalNativeRetiredWitness};
#[path="🛬️owned/🦀️.rs"]
mod original_job_decoder;
pub use original_job_decoder::OriginalJobDecodeOwner;
//#endregion 📄️RetainedPayload

#[path="📦️return/🪶️weak/🦀️.rs"]
mod original_native_weak_return;

#[path="⏸️park/🦀️.rs"]
mod original_owned_caller_turn;
pub use original_owned_caller_turn::{OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome};

//#region 🧭️StepContext
/// 🧭️ Borrows one job step’s identity, budget, cancellation, clock and preview cursor; private fields centralize cancellation observation.
pub struct StepContext<'a> {
    original_decoder:Option<&'a mut OriginalJobDecodeOwner>,
    receiving_spent:RetainedCloneProgress,
    receiving_issued:Option<RetainedCloneGrant>,
    retained:RetainedCloneGrant,
    retained_progress:&'a mut RetainedCloneProgress,
    operation: OperationId,
    generation: Generation,
    fuel_remaining: u64,
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
    reserved_observer:Option<(&'a mut RetainedCloneProgress,&'a mut u128)>,
    native_weak_parent:Option<original_native_weak_return::OriginalNativeWeakParent>,
    source_epoch: Option<u64>,
    original_retirement: bool,
    original_clock_binding: Option<u64>,
    original_total_fuel: u64,
    original_total_work_start: u128,
    original_absolute_deadline: u64,
    original_total_retained: RetainedCloneGrant,
    original_total_retained_start:RetainedCloneProgress,
    retained_policy:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    original_physical_recipient:Option<(&'a mut RetainedCloneProgress,RetainedCloneProgress)>,
    original_work_recipient:Option<&'a mut u128>,
    replacement_current:Option<ReplacementIdentity>,
    payload_page_granted: bool,
}

struct NativeEncoderJobWork<'loan,'turn>{source:&'loan mut StepContext<'turn>}
impl semio_framework_value::native_encoding::NativeEncodeWorkRecipient for NativeEncoderJobWork<'_,'_>{
 fn admits(&mut self,_observed:u64,additional:u64)->Result<bool,ValueError>{Ok(!self.source.is_cancelled()&&!self.source.should_yield()&&additional<=self.source.fuel_remaining())}
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.source.consume_fuel(actual);Ok(())}
}

impl Drop for StepContext<'_>{fn drop(&mut self){if let Some((recipient,start))=self.original_physical_recipient.as_mut(){let delta=retained_delta(*self.retained_progress,*start);**recipient=recipient.checked_add(delta).expect("original physical receipt exceeds usize");}if self.reserved_observer.is_some(){*self.retained_progress=semio_framework_job_return_budget::source_fresh_progress(*self.retained_progress,self.reserved_return_progress).expect("actual reserved effects exceed their aggregate observer");}if let Some(recipient)=self.original_work_recipient.as_mut(){**recipient=recipient.checked_add(self.work_units_spent).expect("original source work counter exceeds u128");}}}

#[derive(Clone,Copy)]
struct OriginalRetainedLoanIdentity{turn:Option<OriginalSourceTurnIdentity>,parent:Option<usize>,recipient:usize,cursor:usize,ledger:usize,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,deadline:u64}
impl OriginalRetainedLoanIdentity{
 fn capture(cx:&StepContext<'_>)->Self{Self{turn:cx.original_source_turn_identity(),parent:cx.source_parent.map(|parent|parent as*const _ as usize),recipient:cx.retained_progress as*const _ as usize,cursor:cx.preview_sequence as*const _ as usize,ledger:Arc::as_ptr(&cx.payload_ledger)as usize,operation:cx.operation,generation:cx.generation,clock:cx.now_us,deadline:cx.deadline_us}}
 fn matches(&self,cx:&StepContext<'_>)->bool{self.turn==cx.original_source_turn_identity()&&self.parent==cx.source_parent.map(|parent|parent as*const _ as usize)&&self.recipient==cx.retained_progress as*const _ as usize&&self.cursor==cx.preview_sequence as*const _ as usize&&self.ledger==Arc::as_ptr(&cx.payload_ledger)as usize&&self.operation==cx.operation&&self.generation==cx.generation&&std::ptr::fn_addr_eq(self.clock,cx.now_us)&&self.deadline==cx.deadline_us}
}
struct OriginalRetainedLoan<'a>{target:*mut StepContext<'a>,original:RetainedCloneGrant,identity:OriginalRetainedLoanIdentity}
struct OriginalRetirementLoan<'a>{target:*mut StepContext<'a>,original:bool,identity:OriginalRetainedLoanIdentity}
impl Drop for OriginalRetirementLoan<'_>{fn drop(&mut self){unsafe{if self.identity.matches(&*self.target){(*self.target).original_retirement=self.original}}}}
impl OriginalRetainedLoan<'_>{fn matches_original(&self)->bool{unsafe{self.identity.matches(&*self.target)}}}
impl Drop for OriginalRetainedLoan<'_>{fn drop(&mut self){if self.matches_original(){unsafe{(*self.target).retained=self.original}}}}
struct OriginalWorkLoan<'a>{target:*mut StepContext<'a>,fuel:u64,deadline:u64,issued:u64,spent:u128,reserved:u64,cancel:usize,identity:OriginalRetainedLoanIdentity}
impl OriginalWorkLoan<'_>{fn matches_source(&self)->bool{let cx=unsafe{&*self.target};self.identity.matches(cx)&&self.cancel==cx.original_cancel_token()as*const CancelToken as usize&&cx.work_units_spent>=self.spent&&cx.reserved_return_work>=self.reserved}fn matches_original(&self)->bool{if !self.matches_source(){return false}let cx=unsafe{&*self.target};let physical=cx.work_units_spent-self.spent;let reserved=u128::from(cx.reserved_return_work-self.reserved);physical.checked_sub(reserved).is_some_and(|work|work<=u128::from(self.issued))&&cx.fuel_remaining<=self.issued}}
impl Drop for OriginalWorkLoan<'_>{fn drop(&mut self){if self.matches_source(){let cx=unsafe{&mut*self.target};let physical=cx.work_units_spent-self.spent;let reserved=cx.reserved_return_work.saturating_sub(self.reserved);let spent=physical.saturating_sub(u128::from(reserved));cx.fuel_remaining=self.fuel.saturating_sub(u64::try_from(spent).unwrap_or(u64::MAX));cx.deadline_us=self.deadline;}}}

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
    pub fn admit_original_core_wire_delivery(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_core_wire_delivery(self)}
    pub fn admit_original_return_table(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_return_table(self)}
    pub fn admit_original_escrow_publisher(&mut self)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::admit_original_escrow_publisher(self)}
    pub fn publish_original_escrow(&mut self,header:usize,expose:impl for<'loan> FnOnce(&'loan [u8])->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{original_job_continuation::source::publish_original_escrow(self,header,expose)}
    pub fn native_receiving_identity(&self)->Option<(u64,u64)>{self.original_decoder.as_deref().and_then(|owner|owner.receiving_identity())}
    pub fn with_original_decoder(mut self,original:&'a mut OriginalJobDecodeOwner)->Self{self.original_decoder=Some(original);self}
    /// 🫴️ Loans original cancellation, clock, independent remaining credits and the exact shared construction receipt.
    pub fn with_native_receiving(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        if self.receiving_spent!=RetainedCloneProgress::default(){return Err(OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving scope still owes its enclosing semantic receipt")))}
        self.receiving_issued=Some(self.retained_grant());
        let mut operation=Some(operation);self.original_receiving_outcome(out,semio_framework_job_receiving::OriginalReceivingOperation::Receive(&mut |native,grant|operation.take().unwrap()(native,grant)))
    }
    /// 🛫️ Loans and commits this original Job caller’s work to the supplied encoder without replacing its physical owner.
    pub fn with_native_encoder_work<T,E:From<ValueError>>(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let mut work=NativeEncoderJobWork{source:self};native.with_work_recipient(&mut work,operation)
    }
    /// ♻️ Loans retirement-only authority after cancellation while retaining the original deadline, fuel and physical credits.
    pub fn with_native_retirement<T>(&mut self,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>)->Result<T,ValueError>)->Result<T,ValueError>{
        if self.fuel_remaining==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original retirement turn has no fuel"));}
        let grant=self.retained_grant();let mut observed=0;let Self{original_decoder,deadline_us,now_us,original_retirement,fuel_remaining,..}=self;let owner=original_decoder.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native retirement requires the original Job decoder issuer"))?;let retirement=&*original_retirement;let original_deadline=&*deadline_us;let now=*now_us;let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|*retirement||now().is_some_and(|value|value<*original_deadline);let mut deadline=||*retirement||now().is_some_and(|value|value<*original_deadline);let mut ready=||*retirement||now().is_some_and(|value|value<*original_deadline);let mut work=OriginalJobWorkRecipient::new(fuel_remaining,&mut ready);semio_framework_job_receiving::native_call_result(owner.retire(grant,&mut callback,&mut deadline,&mut work,&mut observed,operation))
    }
    /// 🌉️ Runs one non-suspending unit step and retains every original receiving refusal.
    pub fn with_native_receiving_ready(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,step:impl AsyncFnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
        self.with_native_receiving(out,|control,grant|{let mut task=std::task::Context::from_waker(std::task::Waker::noop());let mut future=std::pin::pin!(step(control,grant));match future.as_mut().poll(&mut task){std::task::Poll::Ready(result)=>result,std::task::Poll::Pending=>Err(ValueError::invariant("original receiving scope cannot span a suspension"))}})
    }
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        StepContext::with_payload_ledger(budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)),retained_progress)
    }

    fn with_payload_ledger(budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, clock: ClockStride, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        let (operation, generation) = (payload_ledger.operation, payload_ledger.generation);
        assert!(retained_progress.fits(budget.retained),"original job recipient exceeds its incoming retained grant");
        StepContext { original_decoder:None,receiving_spent:Default::default(),receiving_issued:None,retained:budget.retained, retained_progress, operation, generation, fuel_remaining: budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,
            work_units_spent:0, deadline_us: budget.deadline_us, now_us, clock: std::cell::Cell::new(clock), cancel:StepCancelReference::Owned(cancel), stage: "initial", preview_sequence, payload_ledger:JobPayloadLedgerReference::Owned(payload_ledger), source_parent:None,source_turn:None,return_budget:None,reserved_return_progress:Default::default(),reserved_return_work:0,reserved_observer:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,replacement_current:None, payload_page_granted: false }
    }

    pub fn with_payload_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,authority:&'a JobPayloadAuthority)->Result<Self,ValueError>{
        let Some(original)=authority.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority has already closed"))};
        if original.operation!=operation||original.generation!=generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority identity does not match caller turn"))}
        if !retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job recipient exceeds incoming retained grant"))}
        Ok(Self{original_decoder:None,receiving_spent:Default::default(),receiving_issued:None,retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,work_units_spent:0,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(ClockStride::new()),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::Borrowed(authority),source_parent:None,source_turn:None,return_budget:None,reserved_return_progress:Default::default(),reserved_return_work:0,reserved_observer:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,replacement_current:None,payload_page_granted:false})
    }

    /// 🎟️ Preserves the original caller depth and independently unspent physical credits.
    /// 🆕️ True while this context has consumed no retained currency since its turn began, so the remaining grant IS the whole grant of the turn.
    pub fn retained_turn_untouched(&self)->bool{retained_delta(*self.retained_progress,self.reserved_return_progress)==self.original_total_retained_start}
    pub fn retained_grant(&self)->RetainedCloneGrant{let current=retained_delta(*self.retained_progress,self.reserved_return_progress);let remaining=retained_remaining(self.retained,current);let total=retained_remaining(self.original_total_retained,retained_delta(current,self.original_total_retained_start));let mut remaining=retained_min(remaining,total);if let Some(budget)=self.return_budget.as_ref().filter(|budget|budget.get().is_bound()){remaining=retained_min(remaining,budget.get().remaining().map_or(RetainedCloneGrant::default(),|remaining|remaining.0));}match self.retained_policy{Some((policy,start))=>retained_min(remaining,retained_remaining(policy,retained_delta(current,start))),None=>remaining}}
    pub fn with_retained_loan<R:Copy>(&mut self,requested:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let original=self.retained;let posted=retained_delta(self.retained_progress(),self.reserved_return_progress);
        if !posted.fits(original){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan recipient already exceeds its ceiling").into())}
        let remaining=self.retained_grant();let ceiling=|paid:usize,wanted:usize,available:usize|paid.checked_add(wanted.min(available)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained loan ceiling overflow"));
        let narrowed=RetainedCloneGrant{maximum_items:ceiling(posted.copied_items,requested.maximum_items,remaining.maximum_items)?,maximum_copy_bytes:ceiling(posted.copied_bytes,requested.maximum_copy_bytes,remaining.maximum_copy_bytes)?,maximum_capacity_bytes:ceiling(posted.retained_capacity_bytes,requested.maximum_capacity_bytes,remaining.maximum_capacity_bytes)?,maximum_release_bytes:ceiling(posted.released_bytes,requested.maximum_release_bytes,remaining.maximum_release_bytes)?,maximum_depth:requested.maximum_depth.min(remaining.maximum_depth)};
        let restore=OriginalRetainedLoan{target:std::ptr::addr_of_mut!(*self),original,identity:OriginalRetainedLoanIdentity::capture(self)};self.retained=narrowed;let result=run(self);
        if restore.matches_original(){return result}
        let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan callback replaced its initiating context");
        Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
    }
    pub fn with_original_work_loan<R:Copy>(&mut self,requested_fuel:u64,absolute_deadline_us:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.with_retained_loan(self.retained_grant(),|cx|{
            let fuel=cx.fuel_remaining;let deadline=cx.deadline_us;let issued=requested_fuel.min(cx.fuel_remaining());cx.fuel_remaining=issued;cx.deadline_us=absolute_deadline_us.min(deadline);
            let restore=OriginalWorkLoan{target:std::ptr::addr_of_mut!(*cx),fuel,deadline,issued,spent:cx.work_units_spent,reserved:cx.reserved_return_work,cancel:cx.original_cancel_token()as*const CancelToken as usize,identity:OriginalRetainedLoanIdentity::capture(cx)};let result=run(cx);
            if restore.matches_original(){return result}
            let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original work loan callback replaced its initiating context or refilled its work");
            Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
        })
    }
    pub fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
        let admitted=self.retained_recipient_is_admitted()&&progress.fits(self.retained_grant());
        *self.retained_progress=self.retained_progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;
        let source=self.record_original_source_retained(progress);
        if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"job child receipt exceeds original remaining retained grant").with_retained_progress(progress));}source?;Ok(())
    }
    pub(crate) fn record_original_source_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().record_retained(progress)?;}Ok(())}
    pub fn retained_progress(&self)->RetainedCloneProgress{*self.retained_progress}
    pub fn retained_work_can_enter(&self,demand:RetirementDemand)->bool{retained_work::RetainedWorkBudget::new(self.retained_grant()).can_enter(demand)}
    /// 🚧️ Admits one work step against this turn's grant: `Ok(false)` retries on a fresh turn when earlier work already used this one, while a demand that cannot fit even an untouched turn is refused typed, never idled.
    /// 🧾️ Returns the grant issued for the open original receiving scope.
    pub fn receiving_issued_grant(&self)->Option<RetainedCloneGrant>{self.receiving_issued}
    /// 📥️ Returns physical spend already accepted inside the open receiving scope.
    pub fn receiving_child_progress(&self)->RetainedCloneProgress{self.receiving_spent}
    /// 📩️ Closes a receiving scope whose semantic receipt equals its accepted physical spend.
    pub fn settle_receiving_scope(&mut self)->Result<(),ValueError>{match self.receiving_issued{Some(issued)=>{let spent=self.receiving_spent;self.consume_receiving_retained(issued,spent)}None=>Ok(())}}
    /// 🧾 Debits the enclosing semantic receipt after its original construction child accepted physical spend.
    pub fn consume_receiving_retained(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{let original=self.receiving_issued.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt lacks its original receiving scope"))?;if grant.maximum_items>original.maximum_items||grant.maximum_copy_bytes>original.maximum_copy_bytes||grant.maximum_capacity_bytes>original.maximum_capacity_bytes||grant.maximum_release_bytes>original.maximum_release_bytes||grant.maximum_depth>original.maximum_depth||!progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt differs from original receiving grant").with_retained_progress(progress))}let child=std::mem::take(&mut self.receiving_spent);let mismatch=||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt omitted accepted original construction spend").with_retained_progress(progress);let delta=RetainedCloneProgress{copied_items:progress.copied_items.checked_sub(child.copied_items).ok_or_else(mismatch)?,copied_bytes:progress.copied_bytes.checked_sub(child.copied_bytes).ok_or_else(mismatch)?,retained_capacity_bytes:progress.retained_capacity_bytes.checked_sub(child.retained_capacity_bytes).ok_or_else(mismatch)?,released_bytes:progress.released_bytes.checked_sub(child.released_bytes).ok_or_else(mismatch)?};self.consume_retained(delta)}
    /// 🎟 Captures the replacement identity installed for this turn.
    pub fn replacement_capture(&self)->Option<ReplacementIdentity>{self.replacement_current}
    /// 🕰️ Reports whether a stored replacement identity is stale against the current one.
    pub fn replacement_is_stale(&self,stored:Option<ReplacementIdentity>)->bool{ReplacementIdentity::is_stale(stored,self.replacement_current)}
    /// 🔄 Installs a replacement identity for the duration of `run`, then restores the previous one.
    pub fn with_replacement_current<R>(&mut self,current:ReplacementIdentity,run:impl FnOnce(&mut Self)->R)->R{let previous=self.replacement_current.replace(current);let result=run(self);self.replacement_current=previous;result}
    pub fn admit_work_step(&self,demand:RetirementDemand)->Result<bool,ValueError>{
        if self.retained_work_can_enter(demand){return Ok(true)}
        if self.retained_turn_untouched(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"step demand exceeds the whole grant of its turn and can never fit"))}
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

    pub fn original_total_retained_grant(&self)->RetainedCloneGrant{let current=retained_delta(*self.retained_progress,self.reserved_return_progress);let remaining=retained_remaining(self.original_total_retained,retained_delta(current,self.original_total_retained_start));self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(remaining,|budget|retained_min(remaining,budget.get().remaining().map_or(RetainedCloneGrant::default(),|remaining|remaining.0)))}

    pub fn bind_original_clock_binding(&mut self,binding:u64)->Result<(),ValueError>{self.original_source_parent()?.validate_borrowed_context(self).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original clock binding requires its genuine source context"))?;if binding==0||self.original_clock_binding.is_some_and(|original|original!=binding){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source clock authority cannot be absent or rebound"))}self.original_clock_binding=Some(binding);Ok(())}

    pub fn bind_original_source_epoch(&mut self,epoch:u64)->Result<(),ValueError>{
        self.original_source_parent()?;
        if epoch==0||self.source_epoch.is_some_and(|original|original!=epoch){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source epoch cannot be absent or rebound"))}
        self.source_epoch=Some(epoch);Ok(())
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    /// 🔖️ Borrows the actual Source-issued dispatch token retained by this caller.
    pub fn original_source_turn(&self)->Result<&OriginalSourceTurn,ValueError>{self.source_turn.as_ref().map(SourceTurnReference::token).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller context has no original Source dispatch"))}
    /// 🧵️ Lends the original dispatch to a shorter native continuation turn.
    pub fn original_source_turn_witness(&self)->Result<OriginalSourceTurnWitness<'_>,ValueError>{Ok(self.original_source_turn()?.witness())}
    /// 🪪️ Observes the opaque dispatch identity without creating another token.
    pub fn original_source_turn_identity(&self)->Option<OriginalSourceTurnIdentity>{self.source_turn.as_ref().map(|turn|turn.token().identity())}
    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn original_cancel_token(&self)->&CancelToken{&self.cancel}

    pub fn borrowed_original_cancel_token(&self)->Result<&'a CancelToken,ValueError>{match &self.cancel{StepCancelReference::Borrowed(original)|StepCancelReference::Observed(original)=>Ok(*original),StepCancelReference::Owned(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native job controls require an original borrowed cancellation witness"))}}

    pub fn original_source_parent(&self)->Result<&'a OriginalCallerSourceParent,ValueError>{self.source_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original caller has no parent source supplier"))}

    fn borrowed_original_caller_source(&self)->Result<original_caller_capture::OriginalCallerSource<'a>,ValueError>{let ledger=match &self.payload_ledger{JobPayloadLedgerReference::Borrowed(authority)=>authority.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original caller ledger has already closed"))?,JobPayloadLedgerReference::Owned(_)=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller capture requires its admitted borrowed original ledger"))};Ok(original_caller_capture::OriginalCallerSource{ledger,cancel:self.borrowed_original_cancel_token()?,operation:self.operation,generation:self.generation,clock:self.now_us,deadline:self.deadline_us})}

    pub fn admit_original_cancel_alias(&mut self,output:&mut Option<CancelToken>)->Result<bool,OriginalTurnFailure>{
        self.original_source_parent()?.validate_borrowed_context(self)?;if let Some(alias)=output{if !alias.is_same_node(self.original_cancel_token()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation alias slot belongs to another source").into())}return Ok(true)}let grant=self.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||self.is_cancelled()||self.should_yield(){return Ok(false)}let turn=self.original_cancel_token().admit_alias(grant);let result=match turn.result{Ok(Some((alias,progress)))=>{*output=Some(alias);Ok(RetainedCloneStep::Progress(progress))},Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};
        #[cfg(any(test,feature="testkit"))]if output.is_some(){crate::testkit::narrow_cancel_receipt(self);}
        receive_original_retained_work(self,semio_framework_value_retained_clone::RetainedWorkOutcome{result,work:turn.work})?;Ok(output.is_some())
    }

    /// first one).
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

    pub fn narrow_original_work_budget(&mut self,budget:StepBudget)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||!retained_is_subset(budget.retained,self.retained_grant())||budget.fuel>self.fuel_remaining||budget.deadline_us>self.deadline_us{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work view cannot change source currencies or increase its work/deadline"))}self.retained_policy=Some((budget.retained,retained_delta(*self.retained_progress,self.reserved_return_progress)));self.fuel_remaining=budget.fuel;self.deadline_us=budget.deadline_us;Ok(())}
    pub fn bind_original_retained_total(&mut self,total:RetainedCloneGrant)->Result<(),ValueError>{self.original_source_parent()?;if !retained_is_subset(total,self.retained_grant()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original total exceeds its persistent source recipient"))}self.original_total_retained=total;self.original_total_retained_start=retained_delta(*self.retained_progress,self.reserved_return_progress);Ok(())}
    pub fn bind_original_physical_recipient(&mut self,recipient:&'a mut RetainedCloneProgress)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original physical delta already has its external recipient"))}self.original_physical_recipient=Some((recipient,*self.retained_progress));Ok(())}
    pub fn bind_original_work_recipient(&mut self,recipient:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||self.original_work_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work return requires its one source-bound clock and external recipient"))}self.original_work_recipient=Some(recipient);Ok(())}
    pub fn original_work_units_spent(&self)->u128{self.work_units_spent}
    pub fn reserved_return_progress(&self)->RetainedCloneProgress{self.reserved_return_progress}
    pub fn reserved_return_work(&self)->u64{self.reserved_return_work}
    pub fn original_source_fuel_remaining(&self)->u64{self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(self.fuel_remaining,|budget|budget.get().raw_remaining().map_or(0,|remaining|remaining.1))}
    /// 🧾️ Observes authentic redemptions in the persistent actor without narrowing their aggregate wire receipt.
    pub fn bind_original_reserved_recipient(&mut self,progress:&'a mut RetainedCloneProgress,work:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_none()||self.original_work_recipient.is_none()||self.reserved_observer.is_some()||self.reserved_return_progress!=Default::default()||self.reserved_return_work!=0{return Err(ValueError::invariant("reserved observer must bind before any redemption"))}self.reserved_observer=Some((progress,work));Ok(())}
    fn preflight_original_reserved_return(&self,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError>{if let Some((recipient,start))=&self.original_physical_recipient{let delta=semio_framework_job_return_budget::source_fresh_progress(*self.retained_progress,*start).ok_or_else(||ValueError::invariant("original aggregate observer baseline changed"))?;recipient.checked_add(delta)?.checked_add(progress)?;}if let Some(recipient)=&self.original_work_recipient{recipient.checked_add(self.work_units_spent).and_then(|n|n.checked_add(u128::from(work))).ok_or_else(||ValueError::invariant("original aggregate work observer overflow"))?;}if let Some((p,w))=&self.reserved_observer{p.checked_add(progress)?;w.checked_add(u128::from(work)).ok_or_else(||ValueError::invariant("reserved observer work overflow"))?;}self.retained_progress.checked_add(progress)?;self.reserved_return_progress.checked_add(progress)?;self.reserved_return_work.checked_add(work).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original reserved return CPU observer overflow"))?;self.work_units_spent.checked_add(u128::from(work)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return actual CPU counter overflow"))?;Ok(())}
    fn receive_original_reserved_return(&mut self,progress:RetainedCloneProgress,work:u64)->Result<(),ValueError>{
        self.preflight_original_reserved_return(progress,work)?;
        let recipient=self.retained_progress.checked_add(progress)?;let reserved=self.reserved_return_progress.checked_add(progress)?;let reserved_work=self.reserved_return_work.checked_add(work).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original reserved return CPU observer overflow"))?;let total_work=self.work_units_spent.checked_add(u128::from(work)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original return actual CPU counter overflow"))?;
        if let Some((p,w))=self.reserved_observer.as_mut(){**p=p.checked_add(progress)?;**w=w.checked_add(u128::from(work)).ok_or_else(||ValueError::invariant("reserved observer work overflow"))?;}*self.retained_progress=recipient;self.reserved_return_progress=reserved;self.reserved_return_work=reserved_work;self.work_units_spent=total_work;Ok(())
    }

    pub fn fuel_remaining(&self) -> u64 {
        self.return_budget.as_ref().filter(|budget|budget.get().is_bound()).map_or(self.fuel_remaining,|budget|self.fuel_remaining.min(budget.get().remaining().map_or(0,|remaining|remaining.1)))
    }

    pub fn consume_fuel(&mut self, units: u64) {
        self.work_units_spent+=u128::from(units);self.fuel_remaining = self.fuel_remaining.saturating_sub(units);if let Some(budget)=self.return_budget.as_mut(){budget.get_mut().record_work(units);}
    }

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

    pub fn observe_original_cancellation(&mut self)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation observation requires its bound source clock"))}let original=self.borrowed_original_cancel_token()?;self.cancel=StepCancelReference::Observed(original);Ok(())}

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

    /// [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
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
pub mod testkit {
    use super::*;
    thread_local!{static NEXT_LOAN_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};static NEXT_CANCEL_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}
    pub use semio_framework_job_receiving::testkit::{with_native_decode_progress,observe_native_decode_progress};
    pub fn set_loan_receipt_refusal(value:bool){NEXT_LOAN_REFUSAL.with(|flag|flag.set(value))}
    pub fn set_cancel_receipt_refusal(value:bool){NEXT_CANCEL_REFUSAL.with(|flag|flag.set(value))}
    pub(crate) fn narrow_loan_receipt(cx:&mut StepContext<'_>){if NEXT_LOAN_REFUSAL.with(|flag|flag.replace(false)){cx.retained.maximum_items=cx.retained_progress().copied_items}}
    pub(crate) fn narrow_cancel_receipt(cx:&mut StepContext<'_>){if NEXT_CANCEL_REFUSAL.with(|flag|flag.replace(false)){cx.retained_policy=Some((RetainedCloneGrant::default(),cx.retained_progress()))}}
    pub fn renew_payload_page_grant(cx:&mut StepContext<'_>){cx.payload_page_granted=false}
    pub fn renew_turn(cx:&mut StepContext<'_>,fuel:u64,grant:RetainedCloneGrant)->Result<(),ValueError>{
        let current=retained_delta(*cx.retained_progress,cx.reserved_return_progress);
        if let Some(budget)=cx.return_budget.as_mut().filter(|budget|budget.get().is_bound()){budget.get_mut().renew_turn()?}
        let refill=fuel.saturating_sub(cx.fuel_remaining);cx.fuel_remaining=fuel;cx.payload_page_granted=false;
        cx.retained=RetainedCloneGrant{maximum_items:current.copied_items.saturating_add(grant.maximum_items),maximum_copy_bytes:current.copied_bytes.saturating_add(grant.maximum_copy_bytes),maximum_capacity_bytes:current.retained_capacity_bytes.saturating_add(grant.maximum_capacity_bytes),maximum_release_bytes:current.released_bytes.saturating_add(grant.maximum_release_bytes),maximum_depth:grant.maximum_depth};
        cx.original_total_retained_start=current;
        match cx.return_budget.as_ref().filter(|budget|budget.get().is_bound()){
            Some(budget)=>{let(retained,total)=budget.get().remaining()?;cx.original_total_retained=retained;cx.original_total_fuel=total;cx.original_total_work_start=cx.work_units_spent.saturating_sub(u128::from(cx.reserved_return_work));}
            None=>{cx.original_total_fuel=cx.original_total_fuel.saturating_add(refill);}
        }
        Ok(())
    }
    pub fn child_source_status(child:&OriginalCallerChildSource)->(bool,bool,usize,OriginalSourceBinding){child.test_status()}
    pub fn source_return_budget_is_bound(parent:&OriginalCallerSourceParent)->Result<bool,ValueError>{Ok(parent.return_source()?.try_borrow().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"actual source budget is borrowed"))?.get().is_bound())}
    pub fn return_budget_is_inherited(cx:&StepContext<'_>)->bool{matches!(cx.return_budget.as_ref(),Some(original_return_source::OriginalReturnBudgetReference::Borrowed(_)))}

    pub use super::original_job_continuation::source as original_return_source;
    pub fn operation_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut OperationId{&mut cx.operation}
    pub fn retained_grant_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut RetainedCloneGrant{&mut cx.retained}
    pub fn retained_progress_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut RetainedCloneProgress{cx.retained_progress}
    pub fn return_budget_snapshot(cx:&StepContext<'_>)->original_return_source::OriginalReturnBudgetSnapshot{cx.return_budget.as_ref().expect("actual original source budget").get().snapshot()}
    pub fn return_budget_is_bound(cx:&StepContext<'_>)->bool{cx.return_budget.as_ref().is_some_and(|budget|budget.get().is_bound())}
    pub fn original_total_retained_start(cx:&StepContext<'_>)->RetainedCloneProgress{cx.original_total_retained_start}
    pub fn return_source_is_borrowed(parent:&OriginalCallerSourceParent)->Result<bool,ValueError>{Ok(parent.return_source()?.try_borrow().is_none())}
    pub fn admit_return_origin(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.original_source_parent()?.admit_return_origin(cx)}
    pub fn qualify_return_claim(claim:&OriginalJobReturnClaim<'_>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{claim.qualify(cx)}

    pub fn with_original_work_refusals<T>(cx:&mut StepContext<'_>,operation:Option<OperationId>,capacity:Option<usize>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{
        struct Restore<'a>{context:*mut StepContext<'a>,operation:OperationId,retained:RetainedCloneGrant}
        impl Drop for Restore<'_>{fn drop(&mut self){unsafe{(*self.context).operation=self.operation;(*self.context).retained=self.retained}}}
        let restore=Restore{context:std::ptr::addr_of_mut!(*cx),operation:cx.operation,retained:cx.retained};
        if let Some(operation)=operation{cx.operation=operation}if let Some(capacity)=capacity{cx.retained.maximum_capacity_bytes=capacity}
        let value=run(cx);drop(restore);value
    }

    pub fn with_retained_receipt_policy<T>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{let original=cx.retained_policy;let result=run(cx);cx.retained_policy=original;result}
    pub fn refuse_current_retained_receipt(cx:&mut StepContext<'_>){cx.retained_policy=Some((RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},cx.retained_progress()));}

    pub fn payload_ledger(operation:OperationId,generation:Generation)->Arc<JobPayloadOperationLedger>{Arc::new(JobPayloadOperationLedger::new(operation,generation))}
    pub fn payload_ledger_terminal_is_empty(ledger:&JobPayloadOperationLedger)->bool{ledger.terminal_is_empty()}
    pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{semio_framework_job_payload::ledger_demands(ledger)}
    pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,grant:RetainedCloneGrant)->semio_framework_value_retained_clone::RetainedWorkOutcome{semio_framework_job_payload::close_ledger(ledger,grant)}
}
//#endregion 🧪️Tests

#[path="📦️return/🎟️place/🦀️.rs"]
mod original_return_place;
pub use original_return_place::{OriginalCallerReturnPlace,OriginalSizedClaimReserve};
pub use original_job_continuation::source::{finish_original_header,begin_original_header,write_original_header,with_original_header_outcome,with_original_frozen_header,accept_original_header,close_original_header_failure,abandon_original_unexposed_header,cancel_original_unexposed_header_loan,OriginalHeaderWriteStatus};

#[path="⚠️failure/🦀️.rs"]
mod original_failure_context;

#[path="📦️owner/🦀️.rs"]
mod step_context_owner;
pub use step_context_owner::StepContextOwner;



pub use original_job_continuation::source::{prepare_original_header_guard,qualify_original_header_guard,observe_original_header_guard,release_original_header_guard,OriginalReturnGuardPermit};

pub use original_job_continuation::source::{OriginalHeaderExposurePermit,prepare_original_header_exposure,observe_original_header_exposure,cancel_original_header_exposure};

pub use semio_framework_job_return_escrow_publication::OriginalEscrowHeaderLoanCell;
