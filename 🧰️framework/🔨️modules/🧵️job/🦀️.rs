//! 🧵️ Resumable interactive job protocol and its original caller authority.
//!
//! StepContext retains the initiating clock, absolute deadline, cancellation, recipient and funded stock.
//! Results borrow their paid payload source through descriptors and close through exact physical receipts.
//! Core owns the private payload/context/retirement knot; worker, scope, registry and progress have distinct owners.

use std::future::Future;
use std::mem::{ManuallyDrop, MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::RetirementDemand;

#[path="📥️snapshot/🧬️schema/🦀️.rs"]
mod original_snapshot_intake;
pub use original_snapshot_intake::{OriginalSnapshotIntake,OriginalSnapshotIntakeProducer};

use semio_framework_value::{ValueError, ValueRefusalKind};
#[path="⚠️failure/🦀️.rs"]
mod original_turn_failure;
pub use original_turn_failure::{OriginalTurnFailure,OriginalTurnPrimary,OriginalReceivingPrimary,OriginalClosureFailure,OriginalDomainFailure,OriginalDomainPrimary,OriginalDomainProjection,OriginalDomainView,admit_original_domain_failure,original_domain_failure_demand,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault};
#[path="♻️retirement/🎟️context/🦀️.rs"]
mod original_close_context;
pub use original_close_context::{close_original_granted_step,close_original_nested_step,close_original_retained_step,close_with_self_funded_context,SelfFundedCloseCaller,close_original_retained_turn};

#[cfg(test)]
use std::time::Instant;

use semio_framework_async::ChannelPolicy;
use semio_framework_trace::{TraceEvent, Watchdog};
#[cfg(test)] use semio_framework_trace::{record_operation_started};

pub use semio_framework_async::CancelToken;
pub use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
pub use semio_framework_trace::{Generation, InteractiveStage, OperationId, allocate_operation_id, allocate_operation_id_in_slot, interactive_step_contract_violated, runtime_diagnostics_enabled, set_runtime_diagnostics, INTERACTIVE_STEP_CEILING_US, RUNTIME_DIAGNOSTICS_ENV, SUSTAINED_OVERRUN_QUARANTINE_STEPS};

pub use semio_framework_job_payload::{JobPayloadStep, JobPayloadAuthority, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadStream, JobPayloadAdmissionFault, JobPayloadRejectedPage, RetainedJobPayload, RetainedJobPayloadReader, RetainedJobPayloadWriter, RetainedPayloadBuilder, OriginalPayloadSourceIdentity, OriginalPayloadBuilderIdentity, JobPayloadPageGrant, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_PROCESS_BYTES};

#[path = "🕰️clock-identity-and-budget/🦀️.rs"]
mod clock_identity_and_budget;
use clock_identity_and_budget::poll_ready_now;
pub use clock_identity_and_budget::{BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US, CommitValidation, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US, JobSequenceExhausted, MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US, Operation, RevisionId, StepBudget, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US, child_cancel_token, default_now_ms, default_now_us, install_microsecond_clock, logical_now_us, microseconds_from_milliseconds, root_cancel_token, validate_commit};

#[path = "🚦️clock-stride-and-job-protocol/🦀️.rs"]
mod clock_stride_and_job_protocol;
pub use clock_stride_and_job_protocol::{Checkpoint, ClockStride, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, StepOutcome};
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use clock_stride_and_job_protocol::JobThreadTransfer;
#[cfg(not(all(target_arch="wasm32",not(target_env="p2"))))]
pub use clock_stride_and_job_protocol::JobThreadTransfer;

//#region 🕰️Clock

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_async::install_browser_monotonic_clock;

//#endregion 🕰️Clock

//#region 📄️RetainedPayload

#[path="♻️retirement/📄️payload/🦀️.rs"]
mod payload_retirement;
pub use payload_retirement::{close_step_outcome_slot,step_outcome_slot_retirement_demands,JobOutcomeSlot,JobPayloadSlot};

#[path="🏃️work/🎟️grant/🦀️.rs"]
pub mod retained_work;

#[path="📬️outcome/🤝️loan/🦀️.rs"]
mod outcome_loan;
pub use outcome_loan::{JobOutcomeAdmission,JobOutcomeBorrow,drive_step};

#[path="📬️outcome/🎟️descriptor/🦀️.rs"]
mod original_outcome_descriptor;
pub use original_outcome_descriptor::{JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView};
pub use original_outcome_descriptor::JobOutcomeDescriptorSlot;
#[path="📬️outcome/📮️return/🦀️.rs"]
mod original_payload_return;
#[path="📬️outcome/📦️builder/📮️return/🦀️.rs"]
mod builder_supplied_page_return;
pub use builder_supplied_page_return::RetainedPayloadBuilderSupply;

pub use original_payload_return::{OriginalJobPageReturnRegistry,OriginalJobPageReservation,OriginalSuppliedJobPage,OriginalContiguousJobBytes,OriginalJobBytesReturnRegistry,OriginalJobBytesReservation,OriginalSuppliedJobBytes};
#[path="📬️outcome/👷️worker/🦀️.rs"]
mod original_worker_payload;
pub use original_worker_payload::{OriginalWorkerPayload,OriginalWorkerPageReservation};

enum JobPayloadLedgerReference<'a>{Owned(Arc<JobPayloadOperationLedger>),Borrowed(&'a JobPayloadAuthority),OriginalWorker(&'a Arc<JobPayloadOperationLedger>)}
impl std::ops::Deref for JobPayloadLedgerReference<'_>{type Target=Arc<JobPayloadOperationLedger>;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::OriginalWorker(original)=>original,Self::Borrowed(authority)=>authority.original().expect("original context outlived its payload authority")}}}
enum StepCancelReference<'a>{Owned(CancelToken),Borrowed(&'a CancelToken),Observed(&'a CancelToken)}
impl std::ops::Deref for StepCancelReference<'_>{type Target=CancelToken;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::Borrowed(original)|Self::Observed(original)=>original}}}
#[path="⏱️context/📸️capture/🦀️.rs"]
mod original_caller_capture;
pub use original_caller_capture::{OriginalCallerCapture,CapturedOriginalCaller,OriginalCallerIssuer,OriginalCallerTurn,OriginalCallerCloseReturn,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalCallerChildSource,OriginalChildTurnReturn};
#[path="🛬️receiving/🦀️.rs"]
mod original_receiving;
pub use original_receiving::OriginalJobDecodeOwner;
//#endregion 📄️RetainedPayload

#[path="⏱️context/📦️return/🪶️weak/🦀️.rs"]
mod original_native_weak_return;

#[path="⏱️context/⏸️park/🦀️.rs"]
mod original_owned_caller_turn;
pub use original_owned_caller_turn::{OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome};

//#region 🧭️StepContext
/// 🧭️ Everything one [`InteractiveJob::step`] call needs: identity ([`OperationId`]/[`Generation`]),
/// the two-bound budget, cancellation, the clock, and the running preview-sequence cursor. Fields are
/// private with accessor methods (a deliberate narrowing from the design doc's Decision 1 sketch,
/// which exposed `pub fuel: &mut u64`/`pub cancel: CancelToken` directly) so [`StepContext::is_cancelled`]
/// can own the [`poll_ready_now`] seam in exactly one place instead of every job reimplementing it.
pub struct StepContext<'a> {
    original_decoder:Option<&'a mut OriginalJobDecodeOwner>,
    receiving_spent:RetainedCloneProgress,receiving_issued:Option<RetainedCloneGrant>,
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
    payload_page_granted: bool,
}

fn retained_delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)}}
fn retained_remaining(grant:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant}}
fn retained_min(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
fn retained_is_subset(a:RetainedCloneGrant,b:RetainedCloneGrant)->bool{retained_min(a,b)==a}

impl Drop for StepContext<'_>{fn drop(&mut self){if let Some((recipient,start))=self.original_physical_recipient.as_mut(){let delta=retained_delta(*self.retained_progress,*start);**recipient=recipient.checked_add(delta).expect("original physical receipt exceeds usize");}if let Some(recipient)=self.original_work_recipient.as_mut(){**recipient=recipient.checked_add(self.work_units_spent).expect("original source work counter exceeds u128");}}}

#[derive(Clone,Copy)]
struct OriginalRetainedLoanIdentity{parent:Option<usize>,recipient:usize,cursor:usize,ledger:usize,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,deadline:u64}
impl OriginalRetainedLoanIdentity{
 fn capture(cx:&StepContext<'_>)->Self{Self{parent:cx.source_parent.map(|parent|parent as*const _ as usize),recipient:cx.retained_progress as*const _ as usize,cursor:cx.preview_sequence as*const _ as usize,ledger:Arc::as_ptr(&cx.payload_ledger)as usize,operation:cx.operation,generation:cx.generation,clock:cx.now_us,deadline:cx.deadline_us}}
 fn matches(&self,cx:&StepContext<'_>)->bool{self.parent==cx.source_parent.map(|parent|parent as*const _ as usize)&&self.recipient==cx.retained_progress as*const _ as usize&&self.cursor==cx.preview_sequence as*const _ as usize&&self.ledger==Arc::as_ptr(&cx.payload_ledger)as usize&&self.operation==cx.operation&&self.generation==cx.generation&&std::ptr::fn_addr_eq(self.clock,cx.now_us)&&self.deadline==cx.deadline_us}
}
struct OriginalRetainedLoan<'a>{target:*mut StepContext<'a>,original:RetainedCloneGrant,identity:OriginalRetainedLoanIdentity}
struct OriginalRetirementLoan<'a>{target:*mut StepContext<'a>,original:bool,identity:OriginalRetainedLoanIdentity}
impl Drop for OriginalRetirementLoan<'_>{fn drop(&mut self){unsafe{if self.identity.matches(&*self.target){(*self.target).original_retirement=self.original}}}}
impl OriginalRetainedLoan<'_>{fn matches_original(&self)->bool{unsafe{self.identity.matches(&*self.target)}}}
impl Drop for OriginalRetainedLoan<'_>{fn drop(&mut self){if self.matches_original(){unsafe{(*self.target).retained=self.original}}}}
struct OriginalWorkLoan<'a>{target:*mut StepContext<'a>,fuel:u64,deadline:u64,issued:u64,spent:u128,cancel:usize,identity:OriginalRetainedLoanIdentity}
impl OriginalWorkLoan<'_>{fn matches_original(&self)->bool{let cx=unsafe{&*self.target};self.identity.matches(cx)&&self.cancel==cx.original_cancel_token()as*const CancelToken as usize&&cx.work_units_spent>=self.spent&&cx.fuel_remaining<=self.issued}}
impl Drop for OriginalWorkLoan<'_>{fn drop(&mut self){if self.matches_original(){let cx=unsafe{&mut*self.target};let spent=cx.work_units_spent-self.spent;cx.fuel_remaining=self.fuel.saturating_sub(u64::try_from(spent).unwrap_or(u64::MAX));cx.deadline_us=self.deadline;}}}

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
    /// 🛬️ Captures the owning operation's original decoder without creating replacement capacity or cancellation authority.
    pub fn with_original_decoder(mut self,original:&'a mut OriginalJobDecodeOwner)->Self{self.original_decoder=Some(original);self}
    /// 🫴️ Loans original cancellation, clock, independent remaining credits and the exact shared construction receipt.
    pub fn with_native_receiving<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
        if self.receiving_spent!=RetainedCloneProgress::default(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving scope still owes its enclosing semantic receipt")))}let grant=self.retained_grant();self.receiving_issued=Some(grant);let Self{original_decoder,retained,retained_progress,receiving_spent,cancel,deadline_us,now_us,..}=self;let owner=original_decoder.as_deref_mut().ok_or_else(||E::from(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires the original Job decoder issuer")))?;let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|!cancel.is_cancelled_now()&&now_us().is_some_and(|now|now<*deadline_us);let mut recipient=OriginalJobConstructionRecipient{original:*retained,spent:&mut **retained_progress,receiving_spent};owner.receive(grant,&mut callback,&mut recipient,operation)
    }
    /// ♻️ Loans retirement-only authority after cancellation while retaining the original deadline, fuel and physical credits.
    pub fn with_native_retirement<T>(&mut self,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>)->Result<T,ValueError>)->Result<T,ValueError>{
        if self.receiving_spent!=RetainedCloneProgress::default()||self.receiving_issued.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving scope still owes its physical receipt"));}if self.fuel_remaining==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original retirement turn has no fuel"));}
        let grant=self.retained_grant();let Self{original_decoder,cancel,deadline_us,now_us,..}=self;let owner=original_decoder.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native retirement requires the original Job decoder issuer"))?;let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|!cancel.is_cancelled_now()&&now_us().is_some_and(|now|now<*deadline_us);let mut deadline=||now_us().is_some_and(|now|now<*deadline_us);owner.retire(grant,&mut callback,&mut deadline,operation)
    }
    /// 🧾️ Debits the enclosing semantic receipt once after its original construction child already accepted physical spend.
    pub fn receiving_issued_grant(&self)->Option<RetainedCloneGrant>{self.receiving_issued}
    pub fn receiving_child_progress(&self)->RetainedCloneProgress{self.receiving_spent}
    /// 🧾️ Closes one receiving scope whose enclosing semantic receipt equals its accepted physical spend: no further debit, no open obligation.
    pub fn settle_receiving_scope(&mut self)->Result<(),ValueError>{match self.receiving_issued{Some(issued)=>{let spent=self.receiving_spent;self.consume_receiving_retained(issued,spent)}None=>Ok(())}}
    /// 🌉️ Runs one non-suspending async step inside one original native receiving scope and settles that scope as its own semantic receipt; a suspended step is refused instead of being dropped silently.
    pub fn with_native_receiving_ready<T>(&mut self,step:impl AsyncFnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->T)->Result<T,ValueError>{
        let output=self.with_native_receiving::<T,ValueError>(|control|{let mut task=std::task::Context::from_waker(std::task::Waker::noop());let mut future=std::pin::pin!(step(control));match future.as_mut().poll(&mut task){std::task::Poll::Ready(output)=>Ok(output),std::task::Poll::Pending=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving scope cannot span a suspension"))}})?;
        self.settle_receiving_scope()?;Ok(output)
    }
    pub fn consume_receiving_retained(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{let original=self.receiving_issued.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt lacks its original receiving scope"))?;if grant.maximum_items>original.maximum_items||grant.maximum_copy_bytes>original.maximum_copy_bytes||grant.maximum_capacity_bytes>original.maximum_capacity_bytes||grant.maximum_release_bytes>original.maximum_release_bytes||grant.maximum_depth>original.maximum_depth||!progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt differs from original receiving grant").with_retained_progress(progress))}let child=std::mem::take(&mut self.receiving_spent);let mismatch=||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic receipt omitted accepted original construction spend").with_retained_progress(progress);let delta=RetainedCloneProgress{copied_items:progress.copied_items.checked_sub(child.copied_items).ok_or_else(mismatch)?,copied_bytes:progress.copied_bytes.checked_sub(child.copied_bytes).ok_or_else(mismatch)?,retained_capacity_bytes:progress.retained_capacity_bytes.checked_sub(child.retained_capacity_bytes).ok_or_else(mismatch)?,released_bytes:progress.released_bytes.checked_sub(child.released_bytes).ok_or_else(mismatch)?};self.consume_retained(delta)}
    fn with_original_worker_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,clock:ClockStride,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,payload_ledger:&'a Arc<JobPayloadOperationLedger>)->Result<Self,ValueError>{if payload_ledger.operation!=operation||payload_ledger.generation!=generation||!retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker context requires its original admitted identity ledger and recipient"))}Ok(Self{original_decoder:None,receiving_spent:Default::default(),receiving_issued:None,retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,work_units_spent:0,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(clock),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::OriginalWorker(payload_ledger),source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,payload_page_granted:false})}
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        StepContext::with_payload_ledger(operation, generation, budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)),retained_progress)
    }

    #[allow(clippy::too_many_arguments)]
    fn with_payload_ledger(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, clock: ClockStride, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        assert_eq!(payload_ledger.operation, operation, "job payload ledger operation must match its step context");
        assert_eq!(payload_ledger.generation, generation, "job payload ledger generation must match its step context");
        assert!(retained_progress.fits(budget.retained),"original job recipient exceeds its incoming retained grant");
        StepContext { original_decoder:None,receiving_spent:Default::default(),receiving_issued:None,retained:budget.retained, retained_progress, operation, generation, fuel_remaining: budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,
            work_units_spent:0, deadline_us: budget.deadline_us, now_us, clock: std::cell::Cell::new(clock), cancel:StepCancelReference::Owned(cancel), stage: "initial", preview_sequence, payload_ledger:JobPayloadLedgerReference::Owned(payload_ledger), source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None, payload_page_granted: false }
    }

    /// 🤝️ Borrows the original paid ledger and cancellation while preserving the caller's exact recipient.
    pub fn with_payload_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,authority:&'a JobPayloadAuthority)->Result<Self,ValueError>{
        let Some(original)=authority.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority has already closed"))};
        if original.operation!=operation||original.generation!=generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority identity does not match caller turn"))}
        if !retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job recipient exceeds incoming retained grant"))}
        Ok(Self{original_decoder:None,receiving_spent:Default::default(),receiving_issued:None,retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,work_units_spent:0,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(ClockStride::new()),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::Borrowed(authority),source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,payload_page_granted:false})
    }

    /// 🎟️ Preserves the original caller depth and independently unspent physical credits.
    pub fn retained_grant(&self)->RetainedCloneGrant{let remaining=retained_remaining(self.retained,*self.retained_progress);let total=retained_remaining(self.original_total_retained,retained_delta(*self.retained_progress,self.original_total_retained_start));let remaining=retained_min(remaining,total);match self.retained_policy{Some((policy,start))=>retained_min(remaining,retained_remaining(policy,retained_delta(*self.retained_progress,start))),None=>remaining}}
    /// 🪙️ Narrows the same original recipient and restores ceilings only while its initiating context remains intact.
    pub fn with_retained_loan<R:Copy>(&mut self,requested:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let original=self.retained;let posted=self.retained_progress();
        if !posted.fits(original){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan recipient already exceeds its ceiling").into())}
        let remaining=self.retained_grant();let ceiling=|paid:usize,wanted:usize,available:usize|paid.checked_add(wanted.min(available)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained loan ceiling overflow"));
        let narrowed=RetainedCloneGrant{maximum_items:ceiling(posted.copied_items,requested.maximum_items,remaining.maximum_items)?,maximum_copy_bytes:ceiling(posted.copied_bytes,requested.maximum_copy_bytes,remaining.maximum_copy_bytes)?,maximum_capacity_bytes:ceiling(posted.retained_capacity_bytes,requested.maximum_capacity_bytes,remaining.maximum_capacity_bytes)?,maximum_release_bytes:ceiling(posted.released_bytes,requested.maximum_release_bytes,remaining.maximum_release_bytes)?,maximum_depth:requested.maximum_depth.min(remaining.maximum_depth)};
        let restore=OriginalRetainedLoan{target:std::ptr::addr_of_mut!(*self),original,identity:OriginalRetainedLoanIdentity::capture(self)};self.retained=narrowed;let result=run(self);
        if restore.matches_original(){return result}
        let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan callback replaced its initiating context");
        Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
    }
    /// ⏳️ Narrows actual CPU and absolute deadline on the same original context, preserving clock observations and charging actual child work even through unwind.
    pub fn with_original_work_loan<R:Copy>(&mut self,requested_fuel:u64,absolute_deadline_us:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.with_retained_loan(self.retained_grant(),|cx|{
            let fuel=cx.fuel_remaining;let deadline=cx.deadline_us;let issued=requested_fuel.min(fuel);cx.fuel_remaining=issued;cx.deadline_us=absolute_deadline_us.min(deadline);
            let restore=OriginalWorkLoan{target:std::ptr::addr_of_mut!(*cx),fuel,deadline,issued,spent:cx.work_units_spent,cancel:cx.original_cancel_token()as*const CancelToken as usize,identity:OriginalRetainedLoanIdentity::capture(cx)};let result=run(cx);
            if restore.matches_original(){return result}
            let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original work loan callback replaced its initiating context or refilled its work");
            Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
        })
    }
    /// 🧾️ Records actual child effects in the external recipient before refusing exceeded authority.
    pub fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
        let admitted=self.retained_progress.fits(self.retained)&&progress.fits(self.retained_grant());
        *self.retained_progress=self.retained_progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;
        if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"job child receipt exceeds original remaining retained grant").with_retained_progress(progress));}Ok(())
    }
    /// 📊️ Exposes actual effects recorded by this same original step context, including failed turns.
    pub fn retained_progress(&self)->RetainedCloneProgress{*self.retained_progress}
    /// 🚪️ Quotes one normal-work demand against the caller's remaining retained grant on every axis, independent of CPU fuel; a denied quote allocates and posts nothing.
    pub fn retained_work_can_enter(&self,demand:RetirementDemand)->bool{retained_work::RetainedWorkBudget::new(self.retained_grant()).can_enter(demand)}
    /// 🎟️ Checks the original cumulative recipient against the issued turn grant.
    pub fn retained_recipient_is_admitted(&self)->bool{self.retained_progress().fits(self.retained)}

    /// 🪪️ Identifies the actual external recipient without creating a replacement or alias.
    pub fn original_recipient_identity(&self)->usize{self.retained_progress as*const RetainedCloneProgress as usize}
    /// 🪪️ Borrows only the equality identity of this turn's live original payload source.
    pub fn payload_authority_identity(&self)->OriginalPayloadSourceIdentity{OriginalPayloadSourceIdentity(Arc::as_ptr(&self.payload_ledger)as usize)}

    /// 🕰️ Borrows the caller's original clock source for a retained child authority.
    pub fn clock_source(&self)->fn()->Option<u64>{self.now_us}
    /// 🕰️ Returns the same monotonic observation state after one worker turn.
    pub fn clock_stride(&self)->ClockStride{self.clock.get()}

    /// 🎟️ Reads the original supplied source turn without deriving it from trace sequences.
    pub fn original_source_epoch(&self)->Option<u64>{self.source_epoch}

    /// 🕰️ Reads the opaque clock authority supplied by this actual retained source.
    pub fn original_clock_binding(&self)->Option<u64>{self.original_clock_binding}

    /// 💼️ Quotes the same source's total remaining CPU while its semantic work loan stays narrowed.
    pub fn original_total_fuel_remaining(&self)->u64{self.original_total_fuel.saturating_sub(u64::try_from(self.work_units_spent.saturating_sub(self.original_total_work_start)).unwrap_or(u64::MAX))}

    /// 🧭️ The supplied source deadline remains immutable while a semantic view may yield sooner.
    pub fn original_absolute_deadline_us(&self)->u64{self.original_absolute_deadline}

    /// 🪙️ Quotes total original remaining currencies without changing the active semantic loan.
    pub fn original_total_retained_grant(&self)->RetainedCloneGrant{retained_remaining(self.original_total_retained,retained_delta(*self.retained_progress,self.original_total_retained_start))}

    /// 🔏️ Binds one opaque source clock label without changing its actual callback or absolute deadline.
    pub fn bind_original_clock_binding(&mut self,binding:u64)->Result<(),ValueError>{self.original_source_parent()?.validate_borrowed_context(self).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original clock binding requires its genuine source context"))?;if binding==0||self.original_clock_binding.is_some_and(|original|original!=binding){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source clock authority cannot be absent or rebound"))}self.original_clock_binding=Some(binding);Ok(())}

    /// 🪪️ The genuine parent issuer binds one input epoch before children borrow this context.
    pub fn bind_original_source_epoch(&mut self,epoch:u64)->Result<(),ValueError>{
        self.original_source_parent()?;
        if epoch==0||self.source_epoch.is_some_and(|original|original!=epoch){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source epoch cannot be absent or rebound"))}
        self.source_epoch=Some(epoch);Ok(())
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// 🤝️ Borrows this caller's original cancellation witness without issuing another alias.
    pub fn original_cancel_token(&self)->&CancelToken{&self.cancel}

    /// 🛑️ Borrows the admitted external cancellation for native controls without an alias or context replacement.
    pub fn borrowed_original_cancel_token(&self)->Result<&'a CancelToken,ValueError>{match &self.cancel{StepCancelReference::Borrowed(original)|StepCancelReference::Observed(original)=>Ok(*original),StepCancelReference::Owned(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native job controls require an original borrowed cancellation witness"))}}

    /// 🏡️ Borrows the genuine supplier parent for its source lifetime without creating another strong loan.
    pub fn original_source_parent(&self)->Result<&'a OriginalCallerSourceParent,ValueError>{self.source_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original caller has no parent source supplier"))}

    /// 📸️ Borrows the genuine external ledger and cancellation roots without creating authority.
    fn borrowed_original_caller_source(&self)->Result<original_caller_capture::OriginalCallerSource<'a>,ValueError>{let ledger=match &self.payload_ledger{JobPayloadLedgerReference::OriginalWorker(original)=>*original,JobPayloadLedgerReference::Borrowed(authority)=>authority.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original caller ledger has already closed"))?,JobPayloadLedgerReference::Owned(_)=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller capture requires its admitted borrowed original ledger"))};Ok(original_caller_capture::OriginalCallerSource{ledger,cancel:self.borrowed_original_cancel_token()?,operation:self.operation,generation:self.generation,clock:self.now_us,deadline:self.deadline_us})}

    /// 🤝️ Issues one alias only after this original context admits its metadata item and depth.
    pub fn admit_original_cancel_alias(&mut self)->Result<Option<CancelToken>,ValueError>{
        let grant=self.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||self.is_cancelled()||self.should_yield(){return Ok(None)}
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.consume_retained(progress)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"original cancellation alias metadata")?;
        Ok(Some(self.original_cancel_token().clone()))
    }

    /// 🏷️ The label passed to the most recent [`StepContext::set_stage`] call (`"initial"` before the
    /// first one).
    pub fn stage(&self) -> &'static str {
        self.stage
    }

    /// ⏱️ The step's clock, read for real only every [`ClockStride`] calls — see there. A job may
    /// call this after every unit of work; inside a Wasm guest each real read is a host call.
    pub fn now_us(&self) -> Option<u64> {
        let mut clock = self.clock.get();
        let now = clock.read(self.now_us);
        self.clock.set(clock);
        now
    }

    /// 🕰️ The latest reading this step holds, without reading the clock.
    pub fn latest_us(&self) -> Option<u64> {
        self.clock.get().latest_us()
    }

    pub fn deadline_us(&self) -> u64 {
        self.deadline_us
    }

    pub fn deadline_exceeded(&self) -> bool {
        self.now_us().is_none_or(|now_us| now_us >= self.deadline_us)
    }

    /// 🧾️ Raw work remains visible even when a returned guest exceeds the caller's finite wallet.
    pub fn narrow_original_work_budget(&mut self,budget:StepBudget)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||!retained_is_subset(budget.retained,self.retained_grant())||budget.fuel>self.fuel_remaining||budget.deadline_us>self.deadline_us{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work view cannot change source currencies or increase its work/deadline"))}self.retained_policy=Some((budget.retained,*self.retained_progress));self.fuel_remaining=budget.fuel;self.deadline_us=budget.deadline_us;Ok(())}
    pub fn bind_original_retained_total(&mut self,total:RetainedCloneGrant)->Result<(),ValueError>{self.original_source_parent()?;if !retained_is_subset(total,self.retained_grant()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original total exceeds its persistent source recipient"))}self.original_total_retained=total;self.original_total_retained_start=*self.retained_progress;Ok(())}
    pub fn bind_original_physical_recipient(&mut self,recipient:&'a mut RetainedCloneProgress)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original physical delta already has its external recipient"))}self.original_physical_recipient=Some((recipient,*self.retained_progress));Ok(())}
    pub fn bind_original_work_recipient(&mut self,recipient:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||self.original_work_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work return requires its one source-bound clock and external recipient"))}self.original_work_recipient=Some(recipient);Ok(())}
    pub fn original_work_units_spent(&self)->u128{self.work_units_spent}
    pub fn fuel_remaining(&self) -> u64 {
        self.fuel_remaining
    }

    /// ⛽️ Decrements the remaining fuel by `units`, saturating at zero — a job calls this after doing
    /// `units` worth of its own work, never before.
    pub fn consume_fuel(&mut self, units: u64) {
        self.work_units_spent+=u128::from(units);self.fuel_remaining = self.fuel_remaining.saturating_sub(units);
    }

    pub fn fuel_exhausted(&self) -> bool {
        self.fuel_remaining == 0
    }

    /// 🚦️ Whether the job must return NOW (before the hard 8 ms ceiling) — either bound crossed.
    pub fn should_yield(&self) -> bool {
        self.fuel_exhausted() || self.deadline_exceeded()
    }

    /// 🧹️ Physical close spends the same finite source even when its semantic deadline expired.
    pub fn retirement_should_yield(&self)->bool{self.fuel_exhausted()||(!self.original_retirement&&self.deadline_exceeded())}

    /// 🛂️ Only a genuine original source can lend close permission; semantic yield remains unchanged.
    pub fn with_original_retirement<T>(&mut self,run:impl FnOnce(&mut Self)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{self.original_source_parent()?.validate_borrowed_context(self)?;let restore=OriginalRetirementLoan{target:std::ptr::addr_of_mut!(*self),original:self.original_retirement,identity:OriginalRetainedLoanIdentity::capture(self)};self.original_retirement=true;let result=run(self);drop(restore);result}

    /// 🛑️ Whether this step's [`CancelToken`] (or an ancestor's) is cancelled — checked via a single
    /// non-blocking [`poll_ready_now`], see the module doc. A job MUST check this on entry and after
    /// every bounded unit of work (design doc Decision 6): return [`StepOutcome::Cancelled`] without
    /// doing further work once true.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.cancel,StepCancelReference::Observed(_))||poll_ready_now(self.cancel.is_cancelled())
    }

    /// 🚩️ Observes the initiating supplier cancellation without spending an outstanding original loan.
    pub fn observe_original_cancellation(&mut self)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation observation requires its bound source clock"))}let original=self.borrowed_original_cancel_token()?;self.cancel=StepCancelReference::Observed(original);Ok(())}

    /// 👶️ A clone of this step's [`CancelToken`] — `Arc`-cheap — for a job that wants to derive a
    /// child scope (see [`JobScope::child_of`]) or hand the token to work it submits elsewhere.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// 🏷️ Records a `semio_framework_trace::StageChanged` event and updates [`StepContext::stage`] —
    /// the job's own instrumentation call for switching between internal lanes/phases (Puzzle 3D's
    /// brush → fill switch is the template). Terminal per-call events (preview/checkpoint/commit/
    /// cancel/fail) are recorded once by [`drive_step`] from the returned [`StepOutcome`] instead —
    /// see the module doc's "trace, not a second instrumentation layer" section.
    ///
    /// 🕰️ The event is stamped with the step's latest reading ([`StepContext::latest_us`]), at most
    /// one stride old, instead of a read of its own.
    pub fn set_stage(&mut self, label: &'static str) -> Option<TraceEvent> {
        self.stage = label;
        let at_us = self.latest_us().or_else(|| self.now_us());
        semio_framework_trace::record_trace_event_at(self.operation, self.generation, semio_framework_trace::TraceStage::StageChanged { label }, at_us)
    }

    /// 🔢️ The next preview-sequence number for this operation, advancing a cursor that survives
    /// across every [`StepContext`] built for the same retained session — one call per
    /// [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
    /// 🔢️ Reads the original caller cursor without admitting another preview.
    pub fn preview_sequence(&self) -> u64 { *self.preview_sequence }

    pub fn next_preview_sequence(&mut self) -> Result<u64, JobSequenceExhausted> {
        let sequence = *self.preview_sequence;
        *self.preview_sequence = (*self.preview_sequence).checked_add(1).ok_or(JobSequenceExhausted::Preview)?;
        Ok(sequence)
    }

    pub fn admit_payload_page<'b>(&mut self, writer: &'b mut RetainedJobPayloadWriter, source: JobPayloadPageSource) -> Result<JobPayloadPageGrant<'b>, JobPayloadRejectedPage> {
        if self.payload_page_granted {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::OpportunityExhausted, source: ManuallyDrop::new(Some(source)) });
        }
        let ledger = Arc::clone(&self.payload_ledger);
        if let Err(fault) = writer.reserve_page(&ledger,source.allocated_capacity_bytes()) {
            return Err(JobPayloadRejectedPage { fault, source: ManuallyDrop::new(Some(source)) });
        }
        self.payload_page_granted = true;
        Ok(writer.begin_page(ledger, source))
    }

    pub fn payload_from_bytes(&mut self, stream: JobPayloadStream, bytes: &[u8]) -> Result<RetainedJobPayload, JobPayloadRejectedPage> {
        let source = JobPayloadPageSource::new();
        if bytes.len() > JOB_PAYLOAD_PAGE_BYTES {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::StreamBytes, source: ManuallyDrop::new(Some(source)) });
        }
        let mut writer = RetainedJobPayloadWriter::new(stream);
        let rejected = match self.admit_payload_page(&mut writer, source) {
            Ok(mut page) => {
                page.write(bytes).expect("single-page payload was length-checked before write");
                page.commit();
                None
            }
            Err(rejected) => Some(rejected),
        };
        let payload = writer.finish().unwrap_or_else(|_| unreachable!("single-page admission leaves no source inside the writer"));
        match rejected {
            Some(rejected) => Err(rejected),
            None => Ok(payload),
        }
    }
}
struct OriginalJobConstructionRecipient<'a>{original:RetainedCloneGrant,spent:&'a mut RetainedCloneProgress,receiving_spent:&'a mut RetainedCloneProgress}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for OriginalJobConstructionRecipient<'_>{
 fn grant(&mut self,demand:semio_framework_value::RetirementDemand)->Result<RetainedCloneGrant,ValueError>{let remaining=RetainedCloneGrant{maximum_items:self.original.maximum_items.saturating_sub(self.spent.copied_items),maximum_copy_bytes:self.original.maximum_copy_bytes.saturating_sub(self.spent.copied_bytes),maximum_capacity_bytes:self.original.maximum_capacity_bytes.saturating_sub(self.spent.retained_capacity_bytes),maximum_release_bytes:self.original.maximum_release_bytes.saturating_sub(self.spent.released_bytes),..self.original};if !semio_framework_value::retained_clone::retained_clone_grant_admits(remaining,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native construction exceeds original remaining Job grant"));}Ok(remaining)}
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=progress.fits(grant)&&grant.maximum_items<=self.original.maximum_items.saturating_sub(self.spent.copied_items)&&grant.maximum_copy_bytes<=self.original.maximum_copy_bytes.saturating_sub(self.spent.copied_bytes)&&grant.maximum_capacity_bytes<=self.original.maximum_capacity_bytes.saturating_sub(self.spent.retained_capacity_bytes)&&grant.maximum_release_bytes<=self.original.maximum_release_bytes.saturating_sub(self.spent.released_bytes)&&grant.maximum_depth<=self.original.maximum_depth;*self.spent=self.spent.checked_add(progress)?;*self.receiving_spent=self.receiving_spent.checked_add(progress)?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction changed its accepted original Job receipt").with_retained_progress(progress));}Ok(())}
}
//#endregion 🧭️StepContext

//#region 🐕️Drive
/// ▶️ Runs exactly one [`InteractiveJob::step`] call under a [`Watchdog`] (so an 8 ms-plus step is
/// ALWAYS caught — never eyeballed, see this ticket's exit gate), pre-checks cancellation so an
/// already-cancelled operation never even enters the job, and is the ONE place a returned
/// [`StepOutcome`] becomes a `semio_framework_trace::record_*` call (module doc). `site` is the
/// `&'static str` label `Watchdog`/the trace ring key on; `stage` is which [`InteractiveStage`]
/// contract family this call belongs to (mirrors the caller's `semio_framework_async::Lane`, kept a
/// separate parameter rather than converted from `Lane` since this crate must not depend on the actor
/// crate's lane-to-stage mapping). `preview_sequence` is threaded across an entire run — see
/// [`StepContext::next_preview_sequence`].
//#endregion 🐕️Drive

#[cfg(test)]
pub(crate) const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

//#region 🧪️Tests

#[path = "🔎️reconcile/🧬️schema/🦀️.rs"]
pub mod reconcile;

#[cfg(test)]
#[global_allocator]
static JOB_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "⏱️context/📦️owner/🦀️.rs"]
mod step_context_owner;
pub use step_context_owner::StepContextOwner;
#[cfg(any(test,feature="testkit"))]
#[path="⏱️context/🧪️testing/🎟️caller/🦀️.rs"]
mod original_fixture_caller;
#[cfg(any(test,feature="testkit"))]
pub use original_fixture_caller::OriginalJobFixtureCaller;

/// 🔬️ Thin accessors for the companion white-box laws; they borrow the private operation ledger, its context constructor and the writer's custody without granting any new authority.
#[cfg(any(test,feature="testkit"))]
pub mod testkit {
    use super::*;
    pub fn payload_ledger(operation:OperationId,generation:Generation)->Arc<JobPayloadOperationLedger>{Arc::new(JobPayloadOperationLedger::new(operation,generation))}
    pub fn payload_ledger_terminal_is_empty(ledger:&JobPayloadOperationLedger)->bool{ledger.terminal_is_empty()}
    #[allow(clippy::too_many_arguments)]
    pub fn step_context_with_payload_ledger<'a>(operation:OperationId,generation:Generation,budget:StepBudget,cancel:CancelToken,now_us:fn()->Option<u64>,clock:ClockStride,preview_sequence:&'a mut u64,payload_ledger:Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress)->StepContext<'a>{StepContext::with_payload_ledger(operation,generation,budget,cancel,now_us,clock,preview_sequence,payload_ledger,retained_progress)}
    pub fn staged_backing_identity(writer:&RetainedJobPayloadWriter)->Option<*const MaybeUninit<u8>>{writer.staged.as_ref().map(|(_,source,_)|source.backing_identity())}
    pub fn rejected_backing_identity(writer:&RetainedJobPayloadWriter)->Option<*const MaybeUninit<u8>>{writer.rejected.as_ref().map(JobPayloadPageSource::backing_identity)}
    pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{payload_retirement::ledger_demands(ledger)}
    pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{payload_retirement::close_ledger(ledger,grant)}
}
//#endregion 🧪️Tests

#[path="🏭️construction/🦀️.rs"]
mod original_construction;
pub use original_construction::{OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction};
