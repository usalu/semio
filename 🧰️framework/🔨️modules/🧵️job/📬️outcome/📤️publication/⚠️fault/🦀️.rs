//! 📤️ Canonical faults stream from their original typed owner into separately funded publication pages.
use semio_framework_diagnostic::{Fault,FaultWireCursor};
use semio_framework_job::{Generation, OperationId, StepContext, OriginalTurnFailure, receive_original_retained_work, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView};
use semio_framework_job_payload::{JobPayloadStream, RetainedJobPayload, RetainedPayloadBuilder,JobPayloadStep,JobPayloadOperationLedger};
use std::sync::Arc;
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind,RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::mem::MaybeUninit;

/// 🧯️ A fault publisher borrows only the authenticated, bounded original retirement loan.
pub struct OriginalFaultPayloadStep<'a,'b>(&'a mut StepContext<'b>,std::marker::PhantomData<&'a ()>);
impl<'a,'b> OriginalFaultPayloadStep<'a,'b>{fn bind(cx:&'a mut StepContext<'b>)->Result<Self,ValueError>{if !cx.original_retirement_active(){return Err(ValueError::invariant("fault publication requires original retirement authority"))}Ok(Self(cx,std::marker::PhantomData))}pub fn borrow_fault(cx:&'a mut StepContext<'b>,_:&'a Fault)->Result<Self,ValueError>{Self::bind(cx)}pub fn borrow_failure(cx:&'a mut StepContext<'b>,_:&'a OriginalTurnFailure)->Result<Self,ValueError>{Self::bind(cx)}}
impl JobPayloadStep for OriginalFaultPayloadStep<'_,'_>{fn payload_ledger(&self)->&Arc<JobPayloadOperationLedger>{JobPayloadStep::payload_ledger(self.0)}fn payload_page_granted(&self)->bool{JobPayloadStep::payload_page_granted(self.0)}fn grant_payload_page(&mut self){JobPayloadStep::grant_payload_page(self.0)}fn is_cancelled(&self)->bool{false}fn should_yield(&self)->bool{self.0.retirement_should_yield()}fn retained_grant(&self)->RetainedCloneGrant{self.0.retained_grant()}fn consume_retained(&mut self,p:RetainedCloneProgress)->Result<(),ValueError>{self.0.consume_retained(p)}}

#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalFaultSource{address:usize,operation:OperationId,generation:Generation}
/// 🧳️ Enclosing admission owns this inline header and fixed window; the original Fault remains borrowed.
pub struct RetainedFaultPublication{builder:RetainedPayloadBuilder,wire:FaultWireCursor,buffer:[MaybeUninit<u8>;17],binding:Option<OriginalFaultSource>,flush_cursor:usize,delivered:bool,closing:bool}
impl RetainedFaultPublication{
 pub fn new()->Self{Self{builder:RetainedPayloadBuilder::new(JobPayloadStream::Fault),wire:FaultWireCursor::new(),buffer:[MaybeUninit::uninit();17],binding:None,flush_cursor:0,delivered:false,closing:false}}
 fn original(fault:&Fault,cx:&StepContext<'_>)->OriginalFaultSource{OriginalFaultSource{address:fault as*const Fault as usize,operation:cx.operation(),generation:cx.generation()}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"fault publication requires its original immutable Fault and operation through funded closure")}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.wire.terminal_is_empty()&&self.builder.terminal_is_empty()&&!self.closing}
 pub fn published(&self)->Option<&RetainedJobPayload>{self.builder.published()}
 /// 📏️ Quotes encoding, window publication, metadata drainage and semantic borrowing as separate frontiers.
 pub fn advance_demands(&self,fault:&Fault,cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.binding.is_some_and(|original|original!=Self::original(fault,cx)){return Err(Self::invalid())}
  if self.delivered{return Ok(Default::default())}
  if self.binding.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.builder.is_initialized(){return Self::nested(self.builder.initialization_demand())}
  let buffered=self.wire.buffered_bytes(&self.buffer)?.len();
  if buffered==self.buffer.len()||self.wire.is_complete()&&buffered>0{return Self::nested(if self.flush_cursor<buffered{self.builder.append_demand(cx.retained_grant())?}else{RetirementDemand{depth:1,..Default::default()}})}
  if !self.wire.is_complete(){return Self::nested(self.wire.advance_demands(fault,&self.buffer)?)}
  if self.builder.published().is_none(){return Self::nested(RetainedPayloadBuilder::seal_demand())}
  Ok(RetirementDemand{depth:2,..Default::default()})
 }
 /// ✍️ Preserves the same typed source while actual bytes cross one original window and page at a time.
 pub fn advance_from_fault<'a>(&'a mut self,fault:&Fault,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{self.advance_from_fault_with_cursor(fault,cx,0)}
 /// 📤️ Funds the caller's exact descriptor destination when the sealed fault is admitted.
 pub fn advance_from_fault_with_cursor<'a>(&'a mut self,fault:&Fault,cx:&mut StepContext<'_>,cursor:usize)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;let mut demand=self.advance_demands(fault,cx)?;if self.wire.is_complete()&&self.wire.buffered_bytes(&self.buffer)?.is_empty()&&self.builder.published().is_some(){demand.copy_bytes=demand.copy_bytes.checked_add(cursor).ok_or_else(Self::invalid)?;}if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()})||self.delivered||!Self::permits(cx.retained_grant(),demand){return Ok(None)}
  if self.binding.is_none(){self.binding=Some(Self::original(fault,cx));receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;return Ok(None)}
  if !self.builder.is_initialized(){let turn=if cx.original_retirement_active(){self.builder.advance_initialization(&OriginalFaultPayloadStep::borrow_fault(cx,fault)?)}else{self.builder.advance_initialization(cx)};receive_original_retained_work(cx,turn)?;return Ok(None)}
  let buffered=self.wire.buffered_bytes(&self.buffer)?.len();
  if buffered==self.buffer.len()||self.wire.is_complete()&&buffered>0{
   if self.flush_cursor<buffered{let turn=if cx.original_retirement_active(){self.builder.append_original(&mut OriginalFaultPayloadStep::borrow_fault(cx,fault)?,self.wire.buffered_bytes(&self.buffer)?,&mut self.flush_cursor)}else{self.builder.append_original(cx,self.wire.buffered_bytes(&self.buffer)?,&mut self.flush_cursor)};receive_original_retained_work(cx,turn)?;return Ok(None)}
   let turn=self.wire.drain_buffer(&self.buffer,RetainedCloneGrant{maximum_depth:cx.retained_grant().maximum_depth-1,..cx.retained_grant()});let step=receive_original_retained_work(cx,turn)?;if step.progress().copied_items>0{self.flush_cursor=0}return Ok(None)
  }
  if !self.wire.is_complete(){let turn=self.wire.advance_one(fault,&mut self.buffer,RetainedCloneGrant{maximum_depth:cx.retained_grant().maximum_depth-1,..cx.retained_grant()});receive_original_retained_work(cx,turn)?;return Ok(None)}
  if self.builder.published().is_none(){let turn=if cx.original_retirement_active(){self.builder.seal(&OriginalFaultPayloadStep::borrow_fault(cx,fault)?)}else{self.builder.seal(cx)};receive_original_retained_work(cx,turn)?;return Ok(None)}
  let original=if cx.original_retirement_active(){JobOutcomeBorrow::admit_fault_with_cursor_retiring(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?}else{JobOutcomeBorrow::admit_fault_with_cursor(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?};if original.is_some(){self.delivered=true}Ok(original)
 }
 /// 🤝️ Resolves only the same original sealed fault header against its exclusive descriptor.
 pub fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{if !self.delivered||self.closing{return Err(Self::invalid())}descriptor.fault(self.builder.published().ok_or_else(Self::invalid)?)}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.wire.terminal_is_empty(){return Self::nested(self.wire.retirement_demands())}
  if !self.builder.terminal_is_empty(){return Self::nested(self.builder.retirement_demands()?)}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ♻️ Ends encoding metadata and physical pages before removing the original source binding.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !Self::permits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if !self.wire.terminal_is_empty(){let turn=self.wire.close_step(child);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  if !self.builder.terminal_is_empty(){let turn=self.builder.close_step_granted(child);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  self.binding=None;self.flush_cursor=0;self.delivered=false;self.closing=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
}
impl Default for RetainedFaultPublication{fn default()->Self{Self::new()}}
impl Drop for RetainedFaultPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"fault publication reached Drop before its original physical retirement")}}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[global_allocator]
static TEST_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
