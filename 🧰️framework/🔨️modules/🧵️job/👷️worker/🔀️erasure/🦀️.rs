//! 🔀️ Funded metadata erasure preserves the original native Batch authority and every borrowed outcome.
use super::*;
use std::alloc::{alloc,Layout};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"session erasure requires its same idle original Batch and receiving caller")}
fn metadata(grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&grant.maximum_depth>0}
fn transferred()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}

pub(crate) mod preview_session_sealed{pub trait Sealed{}}
impl<J:InteractiveJob+'static> preview_session_sealed::Sealed for BatchJobSession<J>{}
/// 🪢️ Only first-party original Batch owners can implement this borrowed preview operation table.
pub trait OriginalPreviewSession:preview_session_sealed::Sealed+JobThreadTransfer{
 fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>;
 fn step(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobPoll,WorkerJobContention>;
 fn poll(&self)->WorkerJobPoll;
 fn checkout_outcome(&mut self)->bool;
 fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>;
 fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>;
 fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep;
 fn resume(&mut self)->Result<(),WorkerJobContention>;
 fn begin_close(&mut self);
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>;
 fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,WorkerJobDemandError>;
 fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>;
 fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>;
 fn terminal_is_empty(&self)->bool;
}
impl<J:InteractiveJob+'static> OriginalPreviewSession for BatchJobSession<J>{
 fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{BatchJobSession::original_identity(self)}
 fn step(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobPoll,WorkerJobContention>{BatchJobSession::step(self,grant)}
 fn poll(&self)->WorkerJobPoll{BatchJobSession::poll(self)}
 fn checkout_outcome(&mut self)->bool{BatchJobSession::checkout_outcome(self)}
 fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.take_checked_out_retained_step_receipt()}
 fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{self.checked_out_outcome()}
 fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.acknowledge_outcome(grant)}
 fn resume(&mut self)->Result<(),WorkerJobContention>{BatchJobSession::resume(self)}
 fn begin_close(&mut self){BatchJobSession::begin_close(self)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{BatchJobSession::close_step(self,cx)}
 fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,WorkerJobDemandError>{BatchJobSession::retirement_demands(self,maximum_copy_bytes)}
 fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{self.has_original_cancel_alias_witness(witness)}
 fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{self.return_original_cancel_alias_step(witness,grant)}
 fn terminal_is_empty(&self)->bool{BatchJobSession::terminal_is_empty(self)}
}

/// 🏗️ Retains actual header storage before its original receipt is consumed, including refused births.
pub struct OriginalBatchSessionErasure<J:InteractiveJob+'static>{
 storage:ManuallyDrop<Option<Box<MaybeUninit<BatchJobSession<J>>>>>,
 binding:(usize,OperationId,Generation),initialized:bool,receipt_refused:bool,closing:bool,
}
impl<J:InteractiveJob+'static> OriginalBatchSessionErasure<J>{
 pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:Layout::new::<BatchJobSession<J>>().size(),depth:1,..Default::default()}}
 fn original(source:&Option<BatchJobSession<J>>,control:&impl WorkerJobAdmissionControl)->Result<(OperationId,Generation),ValueError>{
  let session=source.as_ref().ok_or_else(invalid)?;
  let identity=session.original_identity().map_err(|_|invalid())?.ok_or_else(invalid)?;
  if identity!=control.admission_identity()||session.poll()!=WorkerJobPoll::Idle{return Err(invalid())}
  if control.admission_cancel().is_some_and(|token|session.has_original_cancel_alias_witness(token).is_ok_and(|has|!has)){return Err(invalid())}
  Ok(identity)
 }
 /// 🧾️ Stores a fallible physical birth before handing its exact receipt to the original caller.
 pub fn try_begin(target:&mut Option<Self>,source:&mut Option<BatchJobSession<J>>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<RetainedCloneProgress>,ValueError>{
  if target.is_some(){return Err(invalid())}let identity=Self::original(source,control)?;let grant=control.admission_grant()?;let demand=Self::admission_demand();
  if !control.admission_is_open()||!metadata(grant)||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(None)}
  let layout=Layout::new::<BatchJobSession<J>>();let pointer=unsafe{alloc(layout)}.cast::<MaybeUninit<BatchJobSession<J>>>();if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original Batch header allocation refused"))}
  let storage=unsafe{Box::from_raw(pointer)};let progress=RetainedCloneProgress{retained_capacity_bytes:layout.size(),..transferred()};
  *target=Some(Self{storage:ManuallyDrop::new(Some(storage)),binding:(source as*const _ as usize,identity.0,identity.1),initialized:false,receipt_refused:true,closing:false});
  control.receive_admission(progress)?;target.as_mut().expect("actual Batch storage remains retained").receipt_refused=false;Ok(Some(progress))
 }
 fn check(&self,source:&Option<BatchJobSession<J>>,control:&impl WorkerJobAdmissionControl)->Result<(),ValueError>{if self.binding.0!=source as*const _ as usize||(self.binding.1,self.binding.2)!=control.admission_identity(){return Err(invalid())}Ok(())}
 /// 🎟️ Transfers the original handle, then erases the same paid Box on a separate metadata turn.
 pub fn advance(&mut self,source:&mut Option<BatchJobSession<J>>,control:&mut impl WorkerJobAdmissionControl)->Result<(Option<ErasedBatchJobSession>,RetainedCloneProgress),ValueError>{
  self.check(source,control)?;if self.storage.is_none()||self.receipt_refused||self.closing{return Err(invalid())}let grant=control.admission_grant()?;
  if !control.admission_is_open()||!metadata(grant){return Ok((None,Default::default()))}
  if !self.initialized{Self::original(source,control)?;control.receive_admission(transferred())?;self.storage.as_mut().expect("paid original header storage").write(source.take().ok_or_else(invalid)?);self.initialized=true;return Ok((None,transferred()))}
  if source.is_some(){return Err(invalid())}control.receive_admission(transferred())?;
  let storage=self.storage.take().ok_or_else(invalid)?;let session:Box<dyn OriginalPreviewSession>=unsafe{storage.assume_init()};self.initialized=false;
  Ok((Some(ErasedBatchJobSession{session:ManuallyDrop::new(Some(session)),allocated_bytes:Self::admission_demand().capacity_bytes}),transferred()))
 }
 pub fn begin_close(&mut self){self.closing=true;}
 /// ↩️ Returns initialized metadata to its original source before separately releasing empty backing.
 pub fn close_step(&mut self,source:&mut Option<BatchJobSession<J>>,control:&mut impl WorkerJobAdmissionControl)->Result<RetainedCloneStep,ValueError>{
  self.check(source,control)?;if !self.closing{return Err(invalid())}let grant=control.admission_grant()?;
  if self.storage.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !control.retirement_is_open()||!metadata(grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.initialized{if source.is_some(){return Err(invalid())}control.receive_admission(transferred())?;let session=unsafe{self.storage.as_mut().expect("original initialized header").assume_init_read()};*source=Some(session);self.initialized=false;return Ok(RetainedCloneStep::Progress(transferred()))}
  let released_bytes=Self::admission_demand().capacity_bytes;if grant.maximum_release_bytes<released_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  drop(self.storage.take().expect("empty header backing remains retained"));let progress=RetainedCloneProgress{released_bytes,..transferred()};control.receive_admission(progress)?;Ok(RetainedCloneStep::Complete(progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.storage.is_none()&&!self.initialized}
}
impl<J:InteractiveJob+'static> Drop for OriginalBatchSessionErasure<J>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Batch erasure reached Drop before paid return and header retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.storage)}}}}

/// 🧵️ A sealed operation table dispatches directly to one original concrete Batch allocation.
pub struct ErasedBatchJobSession{session:ManuallyDrop<Option<Box<dyn OriginalPreviewSession>>>,allocated_bytes:usize}
impl ErasedBatchJobSession{
 pub fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{self.session.as_ref().map_or(Ok(None),|session|session.original_identity())}
 pub fn storage_identity(&self)->Option<*const ()>{self.session.as_ref().map(|session|&**session as*const dyn OriginalPreviewSession as*const ())}
 pub fn poll(&self)->WorkerJobPoll{self.session.as_ref().map_or(WorkerJobPoll::Terminal,|session|session.poll())}
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobPoll,WorkerJobContention>{self.session.as_mut().expect("original erased session remains owned").step(grant)}
 pub fn checkout_outcome(&mut self)->bool{self.session.as_mut().is_some_and(|session|session.checkout_outcome())}
 pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.session.as_mut().and_then(|session|session.take_checked_out_retained_step_receipt())}
 pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{self.session.as_ref().map_or(Ok(None),|session|session.checked_out_outcome())}
 pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.session.as_mut().map_or(RetainedCloneStep::Complete(Default::default()),|session|session.acknowledge_outcome(grant))}
 pub fn resume(&mut self)->Result<(),WorkerJobContention>{self.session.as_mut().expect("original erased session remains owned").resume()}
 pub fn begin_close(&mut self){if let Some(session)=self.session.as_mut(){session.begin_close()}}
 pub fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,WorkerJobDemandError>{match self.session.as_ref(){Some(session)if !session.terminal_is_empty()=>session.retirement_demands(maximum_copy_bytes),Some(_)=>Ok(RetirementDemand{release_bytes:self.allocated_bytes,depth:1,..Default::default()}),None=>Ok(Default::default())}}
 pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{self.session.as_ref().map_or(Ok(false),|session|session.has_original_cancel_alias_witness(witness))}
 pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{self.session.as_ref().map_or(Ok(None),|session|session.return_original_cancel_alias_step(witness,grant))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
  let Some(session)=self.session.as_mut()else{return Ok(WorkerJobCloseStep::Complete{progress:Default::default()})};
  if !session.terminal_is_empty(){return Ok(match session.close_step(cx)?{WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step})}
  let demand=RetirementDemand{release_bytes:self.allocated_bytes,depth:1,..Default::default()};
  let step=close_original_retained_step(cx,demand,|_|{drop(self.session.take().expect("empty original erased header remains retained"));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{released_bytes:demand.release_bytes,..transferred()}))})?;
  Ok(match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress}}else{WorkerJobCloseStep::Pending{progress}},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked})
 }
 pub fn terminal_is_empty(&self)->bool{self.session.is_none()}
}
impl Drop for ErasedBatchJobSession{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"erased original Batch reached Drop before paid physical retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.session)}}}}

impl preview_session_sealed::Sealed for ErasedBatchJobSession{}
impl OriginalPreviewSession for ErasedBatchJobSession{
 fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{ErasedBatchJobSession::original_identity(self)}
 fn step(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobPoll,WorkerJobContention>{ErasedBatchJobSession::step(self,grant)}
 fn poll(&self)->WorkerJobPoll{ErasedBatchJobSession::poll(self)}
 fn checkout_outcome(&mut self)->bool{ErasedBatchJobSession::checkout_outcome(self)}
 fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{ErasedBatchJobSession::take_checked_out_retained_step_receipt(self)}
 fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{ErasedBatchJobSession::checked_out_outcome(self)}
 fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{ErasedBatchJobSession::acknowledge_outcome(self,grant)}
 fn resume(&mut self)->Result<(),WorkerJobContention>{ErasedBatchJobSession::resume(self)}
 fn begin_close(&mut self){ErasedBatchJobSession::begin_close(self)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{ErasedBatchJobSession::close_step(self,cx)}
 fn retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,WorkerJobDemandError>{ErasedBatchJobSession::retirement_demands(self,maximum_copy_bytes)}
 fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{ErasedBatchJobSession::has_original_cancel_alias_witness(self,witness)}
 fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{ErasedBatchJobSession::return_original_cancel_alias_step(self,witness,grant)}
 fn terminal_is_empty(&self)->bool{ErasedBatchJobSession::terminal_is_empty(self)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
