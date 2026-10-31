//! 👷️ Worker fault and receiving custody are prepared under the actual admitting source before native ownership.
use super::*;
use semio_framework_job_payload::RetainedPayloadBuilder;
const ORIGINAL_WORKER_FAULT:&[u8]=b"job-session.terminal-fault";

/// 🎟️ The actual admitting source owns this canonical fault Builder and one genuine receiving origin.
pub struct OriginalWorkerPageReservation{origin:OriginalJobReturnOrigin,fault:Option<Box<RetainedPayloadBuilder>>,binding:Option<OriginalSourceBinding>,returned:Option<OriginalJobOutcome>,pending_continuation:Option<OriginalJobContinuation>,closing:bool}
impl OriginalWorkerPageReservation{
 pub const fn new()->Self{Self{origin:OriginalJobReturnOrigin::empty(),fault:None,binding:None,returned:None,pending_continuation:None,closing:false}}
 pub fn available()->bool{JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire)<=JOB_PAYLOAD_PROCESS_BYTES-JOB_PAYLOAD_PAGE_BYTES}
 pub fn ready(&self)->bool{!self.closing&&!self.has_pending_return()&&self.binding.is_some()&&self.origin.body_ready()&&self.fault.as_ref().is_some_and(|builder|builder.published().is_some_and(|payload|payload.len()==ORIGINAL_WORKER_FAULT.len()))}
 pub fn terminal_is_empty(&self)->bool{!self.has_pending_return()&&self.fault.is_none()&&self.origin.terminal_is_empty()&&self.binding.is_none()&&!self.closing}
 fn has_pending_return(&self)->bool{self.returned.is_some()||self.pending_continuation.is_some()}
 fn receive_pending(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.returned.is_some(){return self.origin.receive_outcome(cx,&mut self.returned)}if self.pending_continuation.is_some(){return self.origin.receive_continuation(cx,&mut self.pending_continuation)}Ok(true)
 }
 fn drive_original<J:InteractiveJob+?Sized>(&mut self,job:&mut J,cx:&mut StepContext<'_>,site:&'static str,stage:InteractiveStage,verdict:&mut Option<semio_framework_trace::CallbackVerdict>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing worker cannot enter another semantic callback").into())}self.validate(cx)?;
  if !self.has_pending_return(){let(_,receiver)=self.origin.context_parts()?;match drive_step(job,cx,receiver,site,stage,verdict)?{OriginalJobReturn::Preparing|OriginalJobReturn::Pending=>return Ok(false),OriginalJobReturn::Outcome(reply)=>self.returned=Some(reply.into_descriptor()),OriginalJobReturn::Continuation(value)=>self.pending_continuation=Some(value)}}
  if !self.receive_pending(cx)?{return Ok(false)}Ok(self.origin.continuation().is_some()||self.origin.outcome().is_some_and(JobOutcomeDescriptor::is_terminal))
 }
 pub fn identity(&self)->Option<(OperationId,Generation)>{self.binding.map(|binding|(binding.operation(),binding.generation()))}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker preparation requires its actual admitting source").into())}Ok(())}
 /// 🌱️ Every admitted turn performs at most one real child action under the same current and lifetime currencies.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker preparation is already closing").into())}if self.has_pending_return(){return Ok(false)}
  let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}self.validate(cx)?;
  if !self.origin.body_ready(){self.origin.admit(cx)?;return Ok(false)}
  if !self.origin.admit(cx)?{return Ok(false)}
  if self.fault.is_none(){let binding=OriginalSourceBinding::borrow(cx);let turn=RetainedPayloadBuilder::admit_boxed(JobPayloadStream::Fault,&mut self.fault,cx);if self.fault.is_some(){self.binding=Some(binding);}receive_original_retained_work(cx,turn)?;return Ok(false)}
  let builder=self.fault.as_mut().unwrap();
  let turn=if !builder.is_initialized(){builder.advance_initialization(cx)}else if builder.original().unwrap().page_count()==0{
   match builder.staged_page_len(){
    None=>builder.begin_staged_page_funded(cx),
    Some(written)if written<ORIGINAL_WORKER_FAULT.len()=>{let count=(ORIGINAL_WORKER_FAULT.len()-written).min(grant.maximum_copy_bytes);if count==0{return Ok(false)}builder.write_staged_source_funded(cx,&ORIGINAL_WORKER_FAULT[written..written+count])},
    Some(written)if written==ORIGINAL_WORKER_FAULT.len()=>builder.commit_staged_page_funded(cx),
    _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker fault page exceeded its actual source").into()),
   }
  }else if builder.published().is_none(){builder.seal(cx)}else{return Ok(true)};
  receive_original_retained_work(cx,turn)?;Ok(false)
 }
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(builder)=self.fault.as_ref(){return if builder.terminal_is_empty(){Ok(RetainedPayloadBuilder::boxed_release_demands())}else{builder.retirement_demands()}}
  if !self.origin.terminal_is_empty(){return self.origin.retirement_demands(copy)}Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ♻️ Fault backing and its stable header close before the actual source loan and final local controls.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
  self.validate(cx)?;self.origin.validate_close_context(cx)?;
  if self.has_pending_return(){let before=cx.retained_progress();self.receive_pending(cx)?;let after=cx.retained_progress();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}

  if !self.closing{return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.closing=true;semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})}
  if let Some(builder)=self.fault.as_mut(){if !builder.terminal_is_empty(){let demand=builder.retirement_demands()?;return close_original_retained_step(cx,demand,|grant|builder.close_step_granted(grant))}
   let demand=RetainedPayloadBuilder::boxed_release_demands();return close_original_retained_step(cx,demand,|_|{drop(self.fault.take());semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}})
  }
  if !self.origin.terminal_is_empty(){return self.origin.close_step(cx)}
  close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.binding=None;self.closing=false;semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
 }
}
impl Drop for OriginalWorkerPageReservation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"worker preparation retains its actual source and canonical fault custody");}}

/// 📦️ Native worker storage retains the already admitted fault and unchanged actual-source receiver.
pub struct OriginalWorkerPayload{prepared:OriginalWorkerPageReservation}
impl OriginalWorkerPayload{
 /// 🖋️ Moves only already admitted custody into the final native state without a private ledger or allocation.
 /// # Safety
 /// The target must be exclusive caller-funded uninitialized storage and the input must be ready under its actual source.
 pub unsafe fn initialize_reserved_at(target:*mut Self,reservation:OriginalWorkerPageReservation){assert!(reservation.ready(),"native worker payload requires its actual prepared source");unsafe{target.write(Self{prepared:reservation});}}
 pub fn fault(&self)->Option<&RetainedJobPayload>{self.prepared.fault.as_ref().and_then(|builder|builder.published())}
 pub fn ledger_identity(&self)->Option<usize>{self.fault().and_then(RetainedJobPayload::payload_authority_identity).map(|identity|identity.0)}
 pub fn page_identity(&self)->Option<*const MaybeUninit<u8>>{self.fault().and_then(|payload|payload.page(0).map(|page|page.as_ptr().cast::<MaybeUninit<u8>>()))}
 pub fn identity(&self)->Option<(OperationId,Generation)>{self.prepared.identity()}
 pub fn original_clock(&self)->Result<(fn()->Option<u64>,u64),ValueError>{self.prepared.origin.original_clock()}
 pub fn context_parts(&mut self)->Result<(&OriginalCallerSourceParent,&mut OriginalJobReturnReceiver),ValueError>{if self.prepared.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing worker cannot enter another semantic callback"))}self.prepared.origin.context_parts()}
 pub fn drive_original<J:InteractiveJob+?Sized>(&mut self,job:&mut J,cx:&mut StepContext<'_>,site:&'static str,stage:InteractiveStage,verdict:&mut Option<semio_framework_trace::CallbackVerdict>)->Result<bool,OriginalTurnFailure>{self.prepared.drive_original(job,cx,site,stage,verdict)}
 pub fn outcome(&self)->Option<&JobOutcomeDescriptor>{self.prepared.origin.outcome()}
 pub fn continuation(&self)->Option<&OriginalJobContinuation>{self.prepared.origin.continuation()}
 pub fn terminal_is_empty(&self)->bool{self.prepared.terminal_is_empty()}
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.prepared.retirement_demands(copy)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.prepared.close_step(cx)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
