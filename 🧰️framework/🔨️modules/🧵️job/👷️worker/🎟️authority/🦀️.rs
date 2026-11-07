//! 🎟️ Original native worker authority owns its initialized producer, callback receipts, quarantine, faults and paid retirement.
use semio_framework_job::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,MaybeUninit,size_of,size_of_val};
use std::task::Waker;
/// 🧩️ Original inline worker job whose terminal removal never transfers its carrier by value.
pub struct WorkerJobSource<J>{original:MaybeUninit<J>,present:bool,operation:OperationId,generation:Generation}
impl<J> WorkerJobSource<J>{
 pub fn identity(&self)->(OperationId,Generation){(self.operation,self.generation)}
    #[cfg(test)]
    fn from_job(job:J,operation:OperationId,generation:Generation)->Self{Self{original:MaybeUninit::new(job),present:true,operation,generation}}
    fn original(&self)->Option<&J>{if self.present{Some(unsafe{self.original.assume_init_ref()})}else{None}}
    fn original_mut(&mut self)->Option<&mut J>{if self.present{Some(unsafe{self.original.assume_init_mut()})}else{None}}
    fn is_empty(&self)->bool{!self.present}
    fn terminal_removal_demands(&self)->RetirementDemand{if self.present{RetirementDemand{copy_bytes:0,depth:1,..Default::default()}}else{Default::default()}}
}
impl<J:InteractiveJob> WorkerJobSource<J>{
 /// ♻️ Removes only a terminal source under its unchanged Original caller and posts its actual receipt.
 pub fn retire_terminal_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{if self.identity()!=(cx.operation(),cx.generation()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source retirement requires its original operation identity").into())}if cx.retirement_should_yield(){return Ok(RetainedCloneStep::Progress(Default::default()))}let turn=self.remove_terminal(cx.retained_grant());receive_original_retained_work(cx,turn)}
 fn remove_terminal(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
  let Some(original)=self.original()else{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if !original.terminal_is_empty(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker job removal precedes its terminal witness")),work:0}}
  if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  unsafe{self.original.assume_init_drop();}self.present=false;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
}
impl<J> Drop for WorkerJobSource<J>{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"worker job slot abandoned its original carrier");}}



mod admission_source{pub trait Original{}impl Original for semio_framework_job::StepContext<'_>{}}
/// 🪙️ Carries the original identity, clock and cumulative normal receipt into physical session admission.
pub trait WorkerJobAdmissionControl: admission_source::Original {
    fn admission_identity(&self)->(OperationId,Generation);
    fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>;
    fn admission_is_open(&self)->bool;
    /// 🔓️ Cancellation keeps paid cleanup available within the original fuel and deadline.
    fn retirement_is_open(&self)->bool;
    /// ✖️ Borrows the actual admitting source cancellation without creating an alias.
    fn admission_cancel(&self)->&CancelToken;
    fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
impl WorkerJobAdmissionControl for StepContext<'_>{
    fn admission_identity(&self)->(OperationId,Generation){(self.operation(),self.generation())}
    fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>{self.original_source_parent()?.validate_borrowed_context(self)?;if !self.retained_recipient_is_admitted(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session context recipient exceeds original normal authority"))}Ok(self.retained_grant())}
    fn admission_is_open(&self)->bool{self.original_source_parent().is_ok_and(|source|source.validate_borrowed_context(self).is_ok())&&!self.is_cancelled()&&!self.should_yield()}
    fn retirement_is_open(&self)->bool{self.original_source_parent().is_ok_and(|source|source.validate_borrowed_context(self).is_ok())&&!self.retirement_should_yield()}
    fn admission_cancel(&self)->&CancelToken{self.original_cancel_token()}
    fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.consume_retained(progress)}
}

/// 🏗️ Initializes one original job in its source storage under separately admitted input preparation.
/// 🧷️ Implementors retain every partial child owner until completion or their own paid retirement.
pub unsafe trait WorkerJobInitializer<J> {
    fn initialization_demand(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
    unsafe fn initialize_step(&mut self,target:*mut MaybeUninit<J>,grant:RetainedCloneGrant)->Result<WorkerJobInitializationStep,ValueError>;
    fn initialization_input_is_empty(&self)->bool;
    fn partial_retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
    unsafe fn close_partial_step(&mut self,target:*mut MaybeUninit<J>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn partial_terminal_is_empty(&self)->bool;
}
/// 🧾️ Receipts initialized source writes separately from later destination admission.
pub struct WorkerJobInitializationStep {pub progress:RetainedCloneProgress,pub complete:bool}
impl<J> WorkerJobSource<J> {
    /// 🌱️ Starts original input storage in place without moving an uninitialized job carrier.
    pub fn start_preparation<'source>(target:&'source mut MaybeUninit<Self>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(&'source mut Self,RetainedCloneProgress)>,ValueError>{
        let grant=control.admission_grant()?;let copied_bytes=0;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0||!control.admission_is_open(){return Ok(None)}
        let identity=control.admission_identity();
        unsafe{std::ptr::addr_of_mut!((*target.as_mut_ptr()).present).write(false);std::ptr::addr_of_mut!((*target.as_mut_ptr()).operation).write(identity.0);std::ptr::addr_of_mut!((*target.as_mut_ptr()).generation).write(identity.1);}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()};control.receive_admission(progress)?;
        Ok(Some((unsafe{target.assume_init_mut()},progress)))
    }
    /// 🖋️ Admits declared initialized source fields and their final presence before any writer enters.
    pub fn prepare_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,control:&mut impl WorkerJobAdmissionControl)->Result<WorkerJobInitializationStep,ValueError>{
        if self.present{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job source is already initialized"))}
        if control.admission_identity()!=(self.operation,self.generation){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source preparation received another operation identity"))}
        let grant=control.admission_grant()?;let presence=0;
        if !control.admission_is_open()||grant.maximum_items==0||grant.maximum_copy_bytes<presence||grant.maximum_depth==0{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
        let child=grant;let demand=initializer.initialization_demand(child.maximum_copy_bytes)?;
        if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
        let mut step=match unsafe{initializer.initialize_step(&mut self.original,child)}{Ok(step)=>step,Err(error)=>{semio_framework_value::retained_clone::admit_retained_clone_progress(child,error.retained_progress(),"original failed source preparation")?;control.receive_admission(error.retained_progress())?;return Err(error)}};
        semio_framework_value::retained_clone::admit_retained_clone_progress(child,step.progress,"original job source preparation")?;
        if step.complete{self.present=true;step.progress.copied_bytes+=presence;}
        control.receive_admission(step.progress)?;Ok(step)
    }
    /// 🔎️ Borrows the same initialized source throughout staged destination admission.
    pub fn source(&self)->Option<&J>{self.original()}
}
#[derive(Clone, Copy, Debug)]
pub struct BatchDriveConfig {
    pub retained: RetainedCloneGrant,
    pub site: &'static str,
    pub stage: InteractiveStage,
    pub fuel_per_step: u64,
    pub step_budget_us: u64,
}

#[derive(Clone)]
pub struct BatchJobParams {
    pub operation: OperationId,
    pub generation: Generation,
    pub cancel: CancelToken,
    pub config: BatchDriveConfig,
    pub now_us: fn() -> Option<u64>,
}

struct NativeWorkerAuthorityState<J> {
    job: WorkerJobSource<J>,
    params: Option<BatchJobParams>,
    cancel_retirement:Option<semio_framework_async::CancelTokenRetirement>,
    step_sequence: u64,
    issued_retained:RetainedCloneGrant,
    retained_receipt_pending:bool,
    retained_step_progress:RetainedCloneProgress,
    retained_progress:RetainedCloneProgress,
    payload:OriginalWorkerPayload,
    outcome: JobOutcomeDescriptorSlot,
    callback_verdict: Option<semio_framework_trace::CallbackVerdict>,
    overruns: semio_framework_trace::StepOverrunLedger,
    worker_fault:bool,
    fault_pending:bool,
    step_fault:ManuallyDrop<Option<OriginalTurnFailure>>,
    step_fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    panic_fault:ManuallyDrop<Option<Box<dyn std::any::Any+Send>>>,
    clock: ClockStride,
    last_step_end_us: Option<u64>,
    close_stage: u8,
    initializer: Option<usize>,
    source_initialized: bool,
}

/// 🐝️ Names only the original authority's child-custody frontier; native session scaffolds remain above it.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalWorkerAuthorityPhase{Outcome,OriginalFailure,BeginClose,Payload,Job,JobRelease,Parameters,Empty}

/// 🪪️ Opaque equality witness for the unchanged original native allocation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkerAuthorityIdentity(usize);
/// 🧵️ Owns one original native authority allocation and all of its producer, receipt and retirement custody.
pub struct OriginalWorkerJobAuthority<J>{original:NativeWorkerAuthorityOwner<J>}
impl<J> OriginalWorkerJobAuthority<J>{
 pub fn admission_demand()->RetirementDemand{NativeWorkerAuthorityState::<J>::admission_demand()}
 /// 🧱️ Allocates the fixed native authority only inside its enclosing admitted atomic session birth.
 /// # Safety
 /// The enclosing caller must admit this exact quoted physical extent before entry and receive the complete atomic birth receipt before returning the owner.
 pub unsafe fn admit_pending_prepaid(operation:OperationId,generation:Generation,reservation:OriginalWorkerPageReservation)->Result<Self,ValueError>{let mut storage=Vec::new();storage.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native authority storage reservation refused"))?;Ok(Self{original:NativeWorkerAuthorityOwner::write_pending_storage(storage,operation,generation,reservation)})}
 pub fn is_initialized(&self)->bool{self.original.is_initialized()}
 pub fn admission_page_identity(&self)->Option<*const MaybeUninit<u8>>{self.original.admission_page_identity()}
 pub fn source_identity(&self)->Option<(OperationId,Generation)>{self.original.source_identity()}
 pub fn native_identity(&self)->Option<OriginalWorkerAuthorityIdentity>{(!self.original.0.is_empty()).then(||OriginalWorkerAuthorityIdentity(self.original.0.as_ptr()as usize))}
 pub fn fault_payload_capacity_bytes(&self)->usize{self.original.payload.fault().map_or(0,RetainedJobPayload::allocated_capacity_bytes)}
 pub fn outcome_demands(&self)->Option<RetirementDemand>{if !self.original.outcome.is_empty(){return Some(self.original.outcome.retirement_demands())}self.original.payload.outcome().map(|outcome|RetirementDemand{depth:if outcome.is_acknowledged(){1}else{2},..Default::default()})}
 pub fn can_admit_prepared(&self,control:&impl WorkerJobAdmissionControl,generation:Generation)->bool{let original=&self.original;original.params.as_ref().is_some_and(|params|control.admission_identity()==(params.operation,params.generation)&&generation==params.generation&&control.admission_cancel().is_same_node(&params.cancel)&&!params.cancel.is_cancelled_now())&&original.job.present&&!original.retained_receipt_pending&&original.outcome.is_empty()&&original.payload.outcome().is_none()&&original.payload.continuation().is_none()&&!original.fault_pending&&original.step_fault.is_none()&&original.panic_fault.is_none()}
 pub fn sequence(&self)->u64{self.original.sequence()}
 pub fn last_step_end_us(&self)->Option<u64>{self.original.last_step_end_us()}
 pub fn callback_verdict(&self)->Option<&semio_framework_trace::CallbackVerdict>{self.original.callback_verdict()}
 pub fn overrun_ledger(&self)->&semio_framework_trace::StepOverrunLedger{self.original.overrun_ledger()}
 pub fn issued_grant(&self,requested:RetainedCloneGrant)->RetainedCloneGrant{self.original.issued_grant(requested)}
 pub fn receipt_pending(&self)->bool{self.original.receipt_pending()}
 pub fn retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.original.retained_step_receipt()}
 pub fn take_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.original.take_retained_step_receipt()}
 pub fn retained_step_progress(&self)->RetainedCloneProgress{self.original.retained_step_progress()}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.original.retained_progress()}
 pub fn retained_failure(&self)->Option<&OriginalTurnFailure>{self.original.retained_failure()}
 pub fn original_initialization_failure(&self)->Option<&OriginalTurnFailure>{self.original.original_initialization_failure()}
 pub fn retain_initialization_failure(&mut self,error:ValueError)->ValueError{self.original.retain_initialization_failure(error)}
 pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.original.outcome_descriptor()}
 pub fn continuation(&self)->Option<&OriginalJobContinuation>{if self.original.retained_receipt_pending{None}else{self.original.payload.continuation()}}
 pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.original.acknowledge_outcome(cx)}
 pub fn is_resumable(&self)->bool{self.original.is_resumable()}
 pub fn job(&self)->&J{self.original.job()}
 pub fn job_mut(&mut self)->Result<&mut J,ValueError>{self.original.job_mut()}
 pub fn terminal_is_empty(&self)->bool{self.original.0.is_empty()}
 pub fn close_phase(&self)->OriginalWorkerAuthorityPhase{if self.original.0.is_empty(){OriginalWorkerAuthorityPhase::Empty}else{self.original.close_phase()}}
 pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->bool{self.original.has_original_cancel_alias_witness(witness)}
 pub fn return_original_cancel_alias_step(&mut self,witness:&CancelToken,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<RetainedCloneStep>,semio_framework_async::CancelTokenRetirementError>{self.original.return_original_cancel_alias_step(witness,grant)}
 /// 🪵️ Publishes already-admitted owned sources into this same unpublished native allocation.
 /// # Safety
 /// The enclosing atomic producer must admit the complete source move before entry.
 pub unsafe fn write_owned_source_prepaid(&mut self,job:&mut Option<J>,params:&mut Option<BatchJobParams>){unsafe{self.original.write_owned_source_prepaid(job,params)}}
 /// 🔏️ Publishes the completed initializer after its separately admitted presence handoff.
 /// # Safety
 /// The enclosing producer must admit the presence change and retain the unchanged original parameter owner.
 pub unsafe fn publish_initialized_source_prepaid(&mut self,params:&mut Option<BatchJobParams>){unsafe{self.original.publish_initialized_source_prepaid(params)}}
 pub fn begin_initialized_source_close(&mut self){self.original.begin_initialized_source_close()}
 pub fn initialize_source_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,control:&mut impl WorkerJobAdmissionControl)->Result<WorkerJobInitializationStep,ValueError>{self.original.initialize_source_step(initializer,control)}
 pub fn source_is_initialized(&self)->bool{self.original.source_initialized}
 pub fn finish_partial_source(&mut self,initializer:&impl WorkerJobInitializer<J>)->Result<(),ValueError>{self.original.bind_initializer(initializer)?;if !initializer.partial_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native authority retains its unfinished original initializer"))}self.original.initializer=None;Ok(())}
 pub fn close_partial_source_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let step=self.original.close_partial_source_step(initializer,cx)?;if initializer.partial_terminal_is_empty(){self.original.initializer=None;}Ok(step)}
 pub fn advance_staged_source(&mut self,source:&mut WorkerJobSource<J>,params:&mut Option<BatchJobParams>,copied:&mut usize,control:&mut impl WorkerJobAdmissionControl)->Result<(bool,RetainedCloneProgress),ValueError>{self.original.advance_staged_source(source,params,copied,control)}
}
impl<J:InteractiveJob> OriginalWorkerJobAuthority<J>{
 pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{self.original.outcome()}
 pub fn register_close_wake(&self,waker:&Waker)->bool{self.original.register_close_wake(waker)}
 pub fn validate_original_caller(&self,cx:&StepContext<'_>)->Result<(),ValueError>{let original=self.original.params.as_ref().expect("admitted authority retains original parameters");if original.operation!=cx.operation()||original.generation!=cx.generation()||!original.cancel.is_same_node(cx.original_cancel_token())||!std::ptr::fn_addr_eq(original.now_us,cx.clock_source()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback requires its admitted business identity, cancellation and issuer clock"))}if self.original.retained_receipt_pending{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback receipt still belongs to its prior recipient"))}Ok(())}
 pub fn step_on_original_caller(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.original.step_on_original_caller(cx)}
 pub fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{if self.original.0.is_empty(){return Ok(Default::default())}if self.original.child_custody_is_empty(){self.original.terminal_authority_demands()}else{self.original.retirement_demands(maximum_copy_bytes)}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
  {static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let n=N.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if n<10||n%50_000==0{eprintln!("[DEBUG] aggregator-gate C2 n={} empty={} init_pending={} alias={} child_custody_empty={} yield={}",n,self.original.0.is_empty(),self.original.initializer.is_some()&&!self.original.source_initialized,self.original.has_original_cancel_alias_witness(cx.original_cancel_token()),self.original.child_custody_is_empty(),cx.retirement_should_yield());}}
  if self.original.0.is_empty(){return Ok(WorkerJobCloseStep::Complete{progress:Default::default()})}if self.original.initializer.is_some()&&!self.original.source_initialized{return Ok(WorkerJobCloseStep::Blocked)}
  if self.original.has_original_cancel_alias_witness(cx.original_cancel_token()){
   if cx.retirement_should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
   let turn=self.original.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant());let blocked=matches!(&turn.result,Ok(None)|Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)));let result=match turn.result{Ok(Some(step))=>Ok(step),Ok(None)|Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)};let step=receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;
   return Ok(if blocked{WorkerJobCloseStep::Blocked}else{WorkerJobCloseStep::Pending{progress:step.progress()}})
  }
  if !self.original.child_custody_is_empty(){return self.original.close_step(cx)}if cx.retirement_should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let result=self.original.close_terminal_worker_authority(cx.retained_grant());receive_original_worker_close_result(cx,before,result)
 }
}

impl<J> NativeWorkerAuthorityState<J>{
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn is_initialized(&self)->bool{self.payload.fault().is_some()}
 pub fn admission_page_identity(&self)->Option<*const MaybeUninit<u8>>{self.payload.page_identity()}
 pub fn source_identity(&self)->Option<(OperationId,Generation)>{self.params.as_ref().map(|params|(params.operation,params.generation))}
 pub fn sequence(&self)->u64{self.step_sequence}
 pub fn last_step_end_us(&self)->Option<u64>{self.last_step_end_us}
 pub fn callback_verdict(&self)->Option<&semio_framework_trace::CallbackVerdict>{self.callback_verdict.as_ref()}
 pub fn overrun_ledger(&self)->&semio_framework_trace::StepOverrunLedger{&self.overruns}
 pub fn issued_grant(&self,requested:RetainedCloneGrant)->RetainedCloneGrant{original_worker_turn_grant(self.params.as_ref().expect("original worker parameters remain owned").config.retained,requested)}
 pub fn receipt_pending(&self)->bool{self.retained_receipt_pending}
 pub fn retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.retained_receipt_pending.then_some((&self.issued_retained,&self.retained_step_progress))}
 pub fn take_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{if !self.retained_receipt_pending{return None}self.retained_receipt_pending=false;Some((self.issued_retained,self.retained_step_progress))}
 pub fn retained_step_progress(&self)->RetainedCloneProgress{self.retained_step_progress}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.retained_progress}
 pub fn retained_failure(&self)->Option<&OriginalTurnFailure>{(!self.retained_receipt_pending).then(||self.step_fault.as_ref()).flatten()}
 pub fn original_initialization_failure(&self)->Option<&OriginalTurnFailure>{self.step_fault.as_ref()}
 pub fn retain_initialization_failure(&mut self,error:ValueError)->ValueError{let kind=error.kind;let progress=error.retained_progress();assert!(self.step_fault.is_none());*self.step_fault=Some(error.into());ValueError::literal(kind,"native preparation retains its original failed initializer").with_retained_progress(progress)}
 pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{if self.retained_receipt_pending{None}else if self.worker_fault{self.outcome.original()}else{self.payload.outcome()}}
 pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.retained_receipt_pending{return Ok(false)}
  if !self.outcome.is_empty(){let demand=self.outcome.retirement_demands();close_original_retained_step(cx,demand,|grant|self.outcome.close_step(grant))?;if self.outcome.is_empty(){self.worker_fault=false}return Ok(self.outcome.is_empty())}
  let(_,receiver)=self.payload.context_parts()?;receiver.acknowledge_outcome(cx)
 }
 pub fn is_resumable(&self)->bool{!self.retained_receipt_pending&&self.outcome.is_empty()&&self.payload.outcome().is_none()&&self.payload.continuation().is_none()}
 pub fn job(&self)->&J{self.job.original().expect("original authority retains its producer")}
 pub fn job_mut(&mut self)->Result<&mut J,ValueError>{if self.retained_receipt_pending||!self.outcome.is_empty()||self.payload.outcome().is_some()||self.payload.continuation().is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original semantic result requires acknowledgement before mutable job access"))}Ok(self.job.original_mut().expect("original authority retains its producer"))}
 pub fn child_custody_is_empty(&self)->bool{self.job.is_empty()&&self.params.is_none()&&self.cancel_retirement.is_none()&&self.payload.terminal_is_empty()&&self.outcome.is_empty()&&self.step_fault.is_none()&&self.step_fault_retirement.is_none()&&self.panic_fault.is_none()}
 pub fn close_phase(&self)->OriginalWorkerAuthorityPhase{if !self.outcome.is_empty(){return OriginalWorkerAuthorityPhase::Outcome}if self.step_fault.is_some()||self.step_fault_retirement.is_some()||self.panic_fault.is_some(){return OriginalWorkerAuthorityPhase::OriginalFailure}if self.close_stage==0{return OriginalWorkerAuthorityPhase::BeginClose}if !self.payload.terminal_is_empty(){return OriginalWorkerAuthorityPhase::Payload}match self.close_stage{1=>OriginalWorkerAuthorityPhase::Job,2=>OriginalWorkerAuthorityPhase::JobRelease,3=>OriginalWorkerAuthorityPhase::Parameters,_=>OriginalWorkerAuthorityPhase::Empty}}
 pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->bool{self.close_stage==3&&self.params.is_none()&&self.cancel_retirement.as_ref().is_some_and(|owner|owner.is_original_alias_witness(witness))}
 pub fn return_original_cancel_alias_step(&mut self,witness:&CancelToken,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<RetainedCloneStep>,semio_framework_async::CancelTokenRetirementError>{if self.close_stage!=3||self.params.is_some(){return semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0}}match self.cancel_retirement.as_mut(){Some(original)if !original.terminal_is_empty()=>{let turn=original.return_alias_step(witness,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(Some),work:turn.work}},_=>semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0}}}
 /// 📝️ Installs the original owned sources after their single enclosing native admission.
 /// # Safety
 /// The caller must have admitted the complete source move and must retain exclusive unpublished native storage.
 pub unsafe fn write_owned_source_prepaid(&mut self,job:&mut Option<J>,params:&mut Option<BatchJobParams>){assert!(!self.job.present&&self.params.is_none()&&self.is_initialized());let original=params.as_ref().expect("prepaid original parameters");assert_eq!((original.operation,original.generation),(self.job.operation,self.job.generation));self.job.original.write(job.take().expect("prepaid original producer"));self.job.present=true;self.source_initialized=true;self.params=params.take();self.close_stage=0;}
 /// 🧲️ Publishes only the completed original initializer into the same prepaid native header.
 /// # Safety
 /// The enclosing caller must admit the presence handoff before entry and retain the same parameter owner.
 pub unsafe fn publish_initialized_source_prepaid(&mut self,params:&mut Option<BatchJobParams>){assert!(self.source_initialized&&!self.job.present&&self.params.is_none());let original=params.as_ref().expect("prepaid original parameters");assert_eq!((original.operation,original.generation),(self.job.operation,self.job.generation));self.job.present=true;self.params=params.take();self.close_stage=0;}
 pub fn begin_initialized_source_close(&mut self){assert!(self.source_initialized&&self.params.is_none());self.job.present=true;self.close_stage=0;}
 fn bind_initializer(&mut self,initializer:&impl WorkerJobInitializer<J>)->Result<(),ValueError>{let identity=initializer as*const _ as usize;if self.initializer.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native authority requires its original initializer"))}self.initializer=Some(identity);Ok(())}
 pub fn initialize_source_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,control:&mut impl WorkerJobAdmissionControl)->Result<WorkerJobInitializationStep,ValueError>{
  self.bind_initializer(initializer)?;if self.source_initialized||self.job.present||self.params.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native authority source is already initialized"))}if control.admission_identity()!=(self.job.operation,self.job.generation){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native authority initializer received another original identity"))}let grant=control.admission_grant()?;let demand=initializer.initialization_demand(grant.maximum_copy_bytes)?;if !control.admission_is_open()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
  let step=match unsafe{initializer.initialize_step(&mut self.job.original,grant)}{Ok(step)=>step,Err(error)=>{let progress=error.retained_progress();let refusal=self.retain_initialization_failure(error);semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"native failed initializer")?;control.receive_admission(progress)?;return Err(refusal)}};semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress,"native original initializer")?;if step.complete{self.source_initialized=true;if !initializer.initialization_input_is_empty()||step.progress.copied_items==0{let refusal=self.retain_initialization_failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"native authority initializer completed with retained input").with_retained_progress(step.progress));control.receive_admission(step.progress)?;return Err(refusal)}}control.receive_admission(step.progress)?;Ok(step)
 }
 pub fn close_partial_source_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.bind_initializer(initializer)?;if self.source_initialized||self.job.present{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native partial source close requires its unfinished initializer").into())}let demand=initializer.partial_retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let target=&mut self.job.original as*mut MaybeUninit<J>;close_original_retained_step(cx,demand,|remaining|{let turn=unsafe{initializer.close_partial_step(target,remaining)}.and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(remaining,step,initializer.partial_terminal_is_empty(),"native partial initializer closure"));let work=u64::from(turn.as_ref().is_ok_and(|step|step.progress().copied_items>0));semio_framework_value::RetainedWorkOutcome{result:turn,work}})}
 pub fn advance_staged_source(&mut self,source:&mut WorkerJobSource<J>,params:&mut Option<BatchJobParams>,copied:&mut usize,control:&mut impl WorkerJobAdmissionControl)->Result<(bool,RetainedCloneProgress),ValueError>{
  let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged native source requires original parameters"))?;let identity=(original.operation,original.generation);if identity!=(self.job.operation,self.job.generation)||identity!=(source.operation,source.generation)||identity!=control.admission_identity()||source.source().is_none()||self.job.present||self.params.is_some()||!self.is_initialized()||*copied>size_of::<J>(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged native authority requires its unchanged original source"))}let grant=control.admission_grant()?;if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}let remaining=size_of::<J>()-*copied;if remaining>0{if grant.maximum_copy_bytes==0{return Ok((false,Default::default()))}let bytes=remaining.min(grant.maximum_copy_bytes);let original=source.source().unwrap()as*const J;unsafe{std::ptr::copy_nonoverlapping(original.cast::<MaybeUninit<u8>>().add(*copied),self.job.original.as_mut_ptr().cast::<MaybeUninit<u8>>().add(*copied),bytes);}*copied+=bytes;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};control.receive_admission(progress)?;return Ok((false,progress))}self.params=params.take();source.present=false;self.job.present=true;self.source_initialized=true;self.close_stage=0;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};control.receive_admission(progress)?;Ok((true,progress))
 }
}

impl<J:InteractiveJob> NativeWorkerAuthorityState<J>{
 pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{if self.retained_receipt_pending{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker turn receipt awaits its receiving caller"))}let Some(original)=(if self.worker_fault{self.outcome.original()}else{self.payload.outcome()})else{return Ok(None)};let view=if self.worker_fault{original.fault(self.payload.fault().expect("original paid terminal fault remains owned"))?}else{self.job().borrow_outcome(original)?};Ok(Some(view))}
 pub fn register_close_wake(&self,waker:&Waker)->bool{self.job.original().is_some_and(|job|job.register_close_wake(waker))}
 pub fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{worker_job_authority_close_demands(self,maximum_copy_bytes)}
 pub fn step_on_original_caller(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let original=self.params.as_ref().expect("admitted authority retains original parameters");if original.operation!=cx.operation()||original.generation!=cx.generation()||!original.cancel.is_same_node(cx.original_cancel_token())||!std::ptr::fn_addr_eq(original.now_us,cx.clock_source()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback requires its admitted business identity, cancellation and issuer clock").into())}let config=original.config;if self.retained_receipt_pending{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback receipt still belongs to its prior recipient").into())}self.issued_retained=cx.retained_grant();let before=cx.retained_progress();let held=cx.reserved_return_progress();let result=self.drive_original_caller(cx,config.site,config.stage);let after=cx.retained_progress();let progress=original_worker_current_progress(before,after,held,cx.reserved_return_progress())?;self.retained_step_progress=progress;let cumulative=self.retained_progress.checked_add(progress);self.step_sequence=self.step_sequence.saturating_add(1);self.last_step_end_us=cx.latest_us();self.clock=cx.clock_stride();let terminal=match result{Ok(terminal)=>terminal,Err(failure)=>{match cumulative{Ok(total)=>self.retained_progress=total,Err(error)=>{assert!(self.step_fault.is_none());*self.step_fault=Some(error.with_retained_progress(progress).into());}}self.retained_receipt_pending=false;return Err(failure)}};self.retained_progress=match cumulative{Ok(total)=>total,Err(error)=>{self.retained_receipt_pending=false;return Err(error.with_retained_progress(progress).into())}};self.retained_receipt_pending=true;Ok(terminal)
 }
 fn publish_original_fault(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let detail=self.payload.fault().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker fault requires its prepared payload"))?;
  let Some(outcome)=JobOutcomeBorrow::admit_fault(cx,detail)?else{return Ok(false)};
  if self.outcome.retain(outcome.into_descriptor()).is_err(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker fault requires an empty outcome place").into())}
  self.worker_fault=true;self.fault_pending=false;Ok(true)
 }
 fn drive_original_caller(&mut self,cx:&mut StepContext<'_>,site:&'static str,stage:InteractiveStage)->Result<bool,OriginalTurnFailure>{
  if self.step_sequence==u64::MAX{self.fault_pending=true}
  if self.fault_pending{return self.publish_original_fault(cx)}
  let job=self.job.original_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback requires its exact producer"))?;
  let terminal=self.payload.drive_original(job,cx,site,stage,&mut self.callback_verdict)?;
  if !terminal&&self.callback_verdict.as_ref().is_some_and(|verdict|self.overruns.admit(verdict).is_terminal()){self.fault_pending=true}
  Ok(terminal)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
  {static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let n=N.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if n<40||n%50_000==0{eprintln!("[DEBUG] aggregator-gate C3 n={} stage={} receipt_pending={} outcome_empty={} payload_empty={} job_empty={} params={} cancel_ret={} grant_items={} depth={}",n,self.close_stage,self.retained_receipt_pending,self.outcome.is_empty(),self.payload.terminal_is_empty(),self.job.is_empty(),self.params.is_some(),self.cancel_retirement.is_some(),cx.retained_grant().maximum_items,cx.retained_grant().maximum_depth);}}
  if cx.retirement_should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let grant=cx.retained_grant();if self.retained_receipt_pending{return Ok(WorkerJobCloseStep::Blocked)}let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
  if !self.outcome.is_empty(){let step=worker_work_step(cx,self.outcome.close_step(grant))?;return receive_original_worker_close_effects(cx,before,step)}
  match self.close_original_worker_failure(grant){Ok(Some(step))=>return receive_original_worker_close_effects(cx,before,step),Ok(None)=>{},Err(error)=>{cx.consume_fuel(1);return Err(OriginalTurnFailure::debit_original(error,cx))}}
  if self.close_stage!=0&&!self.payload.terminal_is_empty(){let step=match self.payload.close_step(cx)?{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked};return receive_original_worker_close_effects(cx,before,step)}
  if self.close_stage==0{cx.consume_fuel(1);self.job.original_mut().expect("original authority owns closing producer").begin_close();self.close_stage=1;return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
  if self.close_stage==1{let step=self.job.original_mut().expect("original authority owns closing producer").close_step(cx)?;let step=step.admit(grant,self.job().terminal_is_empty())?;let step=match step{InteractiveJobCloseStep::Pending{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked,InteractiveJobCloseStep::Complete{progress}=>{self.close_stage=2;WorkerJobCloseStep::Pending{progress}}};return receive_original_worker_close_effects(cx,before,step)}
  if self.close_stage==2{let step=worker_work_step(cx,self.job.remove_terminal(grant))?;if self.job.is_empty(){self.close_stage=3;}return receive_original_worker_close_effects(cx,before,step)}
  if self.close_stage==3{let result=self.close_original_worker_params(grant);if matches!(result,Ok(WorkerJobCloseStep::Complete{..})){self.close_stage=4}return receive_original_worker_close_result(cx,before,result.map(|step|match step{WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step}))}
  assert!(self.child_custody_is_empty());Ok(WorkerJobCloseStep::Complete{progress:Default::default()})
 }
}

impl<J> NativeWorkerAuthorityState<J> {
    fn original_worker_failure_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if let Some(original)=self.panic_fault.as_ref(){if let Some(text)=original.downcast_ref::<String>(){return Ok(Some(RetirementDemand{release_bytes:if text.capacity()>0{text.capacity()}else{size_of_val(original.as_ref())},depth:1,..Default::default()}))}if original.is::<&'static str>(){return Ok(Some(RetirementDemand{release_bytes:size_of_val(original.as_ref()),depth:1,..Default::default()}))}return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original arbitrary panic payload awaits its native retirement issuer"))}
        if let Some(original)=self.step_fault_retirement.as_ref(){return Ok(Some(if original.terminal_is_empty(){RetirementDemand{release_bytes:size_of_val(original.as_ref()),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:original.next_copy_byte_demand()?,capacity_bytes:original.next_capacity_byte_demand(body)?,release_bytes:original.next_release_byte_demand()?,depth:original.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original worker error parent depth overflow"))?}}))}
        Ok(self.step_fault.as_ref().map(|_|RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<OriginalTurnFailure>(),depth:2,..Default::default()}))
    }
    fn close_original_worker_failure(&mut self,grant:RetainedCloneGrant)->Result<Option<WorkerJobCloseStep>,ValueError>{
        let Some(demand)=self.original_worker_failure_demands(grant.maximum_copy_bytes)?else{return Ok(None)};if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(WorkerJobCloseStep::Pending{progress:Default::default()}))}
        if let Some(original)=self.panic_fault.as_mut(){if let Some(text)=original.downcast_mut::<String>(){if text.capacity()>0{let released_bytes=text.capacity();drop(std::mem::take(text));return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}))}}drop(self.panic_fault.take());return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}))}
        if let Some(original)=self.step_fault_retirement.as_mut(){if original.terminal_is_empty(){drop(self.step_fault_retirement.take());return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=original.close_step(child).result?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,original.terminal_is_empty(),"original worker failed callback")?;return Ok(Some(WorkerJobCloseStep::Pending{progress:step.progress()}))}
        let original=self.step_fault.take().expect("quoted original worker fault");let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};match semio_framework_value::retirement::admit_owned_retirement(original,child).result{Ok((owner,progress))=>{self.step_fault_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original worker error birth")?;Ok(Some(WorkerJobCloseStep::Pending{progress}))},Err((error,original))=>{*self.step_fault=Some(original);Err(error)}}
    }
    fn original_params_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.params.is_some(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
        if let Some(cancel)=self.cancel_retirement.as_ref(){return if cancel.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}else{cancel.retirement_demands()};}
        Ok(Default::default())
    }
    /// 🛑️ Transfers original params, closes their cancellation ancestors, then removes the empty cursor separately.
    fn close_original_worker_params(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobCloseStep,ValueError>{
        let demand=self.original_params_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if let Some(params)=self.params.take(){self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));return Ok(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
        if let Some(cancel)=self.cancel_retirement.as_mut(){
            if cancel.terminal_is_empty(){drop(self.cancel_retirement.take());return Ok(WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
            return match cancel.close_step(grant).result{Ok(step)=>Ok(WorkerJobCloseStep::Pending{progress:step.progress()}),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(WorkerJobCloseStep::Blocked),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)}
        }
        Ok(WorkerJobCloseStep::Complete{progress:Default::default()})
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
}

impl WorkerJobCloseStep {
    /// 📊️ The actual currencies admitted by this close turn.
    pub fn progress(self) -> RetainedCloneProgress {
        match self {
            Self::Pending { progress } | Self::Complete { progress } => progress,
            Self::Blocked => RetainedCloneProgress::default(),
        }
    }
}

fn original_worker_turn_grant(policy:RetainedCloneGrant,issued:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(issued.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(issued.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(issued.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(issued.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(issued.maximum_depth)}}

fn worker_job_authority_close_demands<J:InteractiveJob>(authority:&NativeWorkerAuthorityState<J>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
    if !authority.outcome.is_empty(){return Ok(authority.outcome.retirement_demands());}
    if let Some(demand)=authority.original_worker_failure_demands(maximum_copy_bytes)?{return Ok(demand)}
    if authority.close_stage==0{return Ok(RetirementDemand{depth:1,..Default::default()});}
    if !authority.payload.terminal_is_empty(){return authority.payload.retirement_demands(maximum_copy_bytes)}
    if authority.close_stage==1{let job=authority.job.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"closing worker lacks its original job"))?;return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?});}
    if authority.close_stage==2{return Ok(authority.job.terminal_removal_demands());}
    if authority.close_stage==3{return authority.original_params_demands();}
    Ok(Default::default())
}

/// 🧾️ Separates the genuine Source-held return redemption from this currently issued worker receipt.
pub fn original_worker_current_progress(before:RetainedCloneProgress,after:RetainedCloneProgress,held_before:RetainedCloneProgress,held_after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
 let current=|a:usize,b:usize,h:usize,j:usize|b.checked_sub(a).and_then(|all|j.checked_sub(h).and_then(|held|all.checked_sub(held))).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"worker current receipt must preserve its original held return span"));
 Ok(RetainedCloneProgress{copied_items:current(before.copied_items,after.copied_items,held_before.copied_items,held_after.copied_items)?,copied_bytes:current(before.copied_bytes,after.copied_bytes,held_before.copied_bytes,held_after.copied_bytes)?,retained_capacity_bytes:current(before.retained_capacity_bytes,after.retained_capacity_bytes,held_before.retained_capacity_bytes,held_after.retained_capacity_bytes)?,released_bytes:current(before.released_bytes,after.released_bytes,held_before.released_bytes,held_after.released_bytes)?})
}

fn worker_payload_step(step:RetainedCloneStep)->WorkerJobCloseStep{WorkerJobCloseStep::Pending{progress:step.progress()}}
fn worker_work_step(cx:&mut StepContext<'_>,turn:semio_framework_value::RetainedWorkOutcome)->Result<WorkerJobCloseStep,OriginalTurnFailure>{receive_original_retained_work(cx,turn).map(worker_payload_step)}

/// 📪️ Posts only original wrapper effects not already charged by its nested original caller.
pub fn receive_original_worker_close_effects(cx:&mut StepContext<'_>,before:RetainedCloneProgress,step:WorkerJobCloseStep)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
    let after=cx.retained_progress();let report=step.progress();let subtract=|reported:usize,initial:usize,current:usize|current.checked_sub(initial).and_then(|posted|reported.checked_sub(posted)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker close reported fewer effects than its original recipient"));
    let progress=RetainedCloneProgress{copied_items:subtract(report.copied_items,before.copied_items,after.copied_items)?,copied_bytes:subtract(report.copied_bytes,before.copied_bytes,after.copied_bytes)?,retained_capacity_bytes:subtract(report.retained_capacity_bytes,before.retained_capacity_bytes,after.retained_capacity_bytes)?,released_bytes:subtract(report.released_bytes,before.released_bytes,after.released_bytes)?};cx.consume_fuel(progress.copied_items as u64);cx.consume_retained(progress)?;Ok(step)
}

pub fn receive_original_worker_close_result(cx:&mut StepContext<'_>,before:RetainedCloneProgress,result:Result<WorkerJobCloseStep,ValueError>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{match result{Ok(step)=>receive_original_worker_close_effects(cx,before,step),Err(error)=>{cx.consume_fuel(1);Err(OriginalTurnFailure::debit_original(error,cx))}}}


impl<J> NativeWorkerAuthorityState<J>{
 /// ⏳️ Initializes the actual prepaid native authority in its enclosing owners uninitialized stable slot.
 /// # Safety
 /// The caller must exclusively own this aligned storage and must admit its actual native extent and page reservation before entry.
    pub unsafe fn initialize_pending_at(target:*mut Self,operation:OperationId,generation:Generation,reservation:OriginalWorkerPageReservation){
        let overruns = semio_framework_trace::StepOverrunLedger::new();
        unsafe {
            std::ptr::addr_of_mut!((*target).job.present).write(false);
            std::ptr::addr_of_mut!((*target).job.operation).write(operation);std::ptr::addr_of_mut!((*target).job.generation).write(generation);
            std::ptr::addr_of_mut!((*target).params).write(None);
            std::ptr::addr_of_mut!((*target).cancel_retirement).write(None);
            std::ptr::addr_of_mut!((*target).step_sequence).write(0);
            std::ptr::addr_of_mut!((*target).issued_retained).write(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0});
            std::ptr::addr_of_mut!((*target).retained_receipt_pending).write(false);
            std::ptr::addr_of_mut!((*target).retained_step_progress).write(RetainedCloneProgress::default());
            std::ptr::addr_of_mut!((*target).retained_progress).write(RetainedCloneProgress::default());
            OriginalWorkerPayload::initialize_reserved_at(std::ptr::addr_of_mut!((*target).payload),reservation);
            JobOutcomeDescriptorSlot::initialize_empty(std::ptr::addr_of_mut!((*target).outcome));
            std::ptr::addr_of_mut!((*target).callback_verdict).write(None);
            std::ptr::addr_of_mut!((*target).overruns).write(overruns);
            std::ptr::addr_of_mut!((*target).worker_fault).write(false);
            std::ptr::addr_of_mut!((*target).fault_pending).write(false);
            std::ptr::addr_of_mut!((*target).step_fault).write(ManuallyDrop::new(None));
            std::ptr::addr_of_mut!((*target).step_fault_retirement).write(None);
            std::ptr::addr_of_mut!((*target).panic_fault).write(ManuallyDrop::new(None));
            std::ptr::addr_of_mut!((*target).clock).write(ClockStride::new());
            std::ptr::addr_of_mut!((*target).last_step_end_us).write(None);
            std::ptr::addr_of_mut!((*target).close_stage).write(3);
            std::ptr::addr_of_mut!((*target).initializer).write(None);
            std::ptr::addr_of_mut!((*target).source_initialized).write(false);
        }
    }
}

/// 🧺️ Unique heap owner for one worker authority. Its vector always contains exactly
/// one fully initialized authority, so submission, checkout, rejection, cancellation, and bounded
/// retirement transfer only this owner while the large retained payload carriers keep one address.
struct NativeWorkerAuthorityOwner<J>(Vec<NativeWorkerAuthorityState<J>>);

impl<J> std::ops::Deref for NativeWorkerAuthorityOwner<J> {
    type Target = NativeWorkerAuthorityState<J>;

    fn deref(&self) -> &Self::Target {
        &self.0[0]
    }
}

impl<J> std::ops::DerefMut for NativeWorkerAuthorityOwner<J> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[0]
    }
}

impl<J> NativeWorkerAuthorityOwner<J> {
    /// 🪹️ Initializes the already-paid original authority directly in its stable heap slot.
    fn write_pending_storage(mut storage:Vec<NativeWorkerAuthorityState<J>>,operation:OperationId,generation:Generation,reservation:OriginalWorkerPageReservation)->Self{unsafe{NativeWorkerAuthorityState::initialize_pending_at(storage.as_mut_ptr(),operation,generation,reservation);storage.set_len(1);}Self(storage)}

}


impl<J> NativeWorkerAuthorityOwner<J>{
    fn terminal_authority_demands(&self)->Result<RetirementDemand,ValueError>{
        let release_bytes=self.0.capacity().checked_mul(size_of::<NativeWorkerAuthorityState<J>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"worker authority physical backing overflow"))?;
        Ok(RetirementDemand{copy_bytes:0,release_bytes,depth:1,..Default::default()})
    }
    /// 👷️ Releases the original ledger before dropping its terminal authority in place and its vector backing.
    fn close_terminal_worker_authority(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobCloseStep,ValueError>{
        let authority=self;let demand=authority.terminal_authority_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        assert!(authority.job.is_empty()&&authority.params.is_none()&&authority.cancel_retirement.is_none()&&authority.payload.terminal_is_empty()&&authority.outcome.is_empty()&&authority.step_fault.is_none()&&authority.step_fault_retirement.is_none()&&authority.panic_fault.is_none());
        authority.0.clear();drop(std::mem::take(&mut authority.0));Ok(WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}})
    }

}


#[cfg(test)]
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[cfg(test)]
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
#[cfg(test)]
#[path="🧪️testing/🦀️.rs"]
mod testing;
#[cfg(test)]
#[path="⏱️budget/🧪️tests/🦀️.rs"]
mod budget_tests;
#[cfg(test)]
#[path="🪜️clock/🧪️tests/🦀️.rs"]
mod clock_tests;

#[cfg(test)]
#[path="♻️retirement/🧪️tests/🦀️.rs"]
mod retirement_tests;

#[cfg(test)]
#[path="🚪️admission/🧪️tests/🦀️.rs"]
mod authentic_admission_tests;

#[path="🚪️admission/🦀️.rs"]
mod authentic_admission;
pub use authentic_admission::OriginalWorkerJobAdmission;
