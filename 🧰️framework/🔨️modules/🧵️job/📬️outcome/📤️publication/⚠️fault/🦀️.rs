//! 📤️ Canonical faults stream from their original typed owner into separately funded publication pages.
use semio_framework_diagnostic::{Fault,FaultWireCursor};
use crate::{Generation,OperationId,StepContext,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,JobPayloadStream,RetainedJobPayload,RetainedPayloadBuilder};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::mem::MaybeUninit;

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
  Ok(RetirementDemand{depth:2,..Default::default()})
 }
 /// ✍️ Preserves the same typed source while actual bytes cross one original window and page at a time.
 pub fn advance_from_fault<'a>(&'a mut self,fault:&Fault,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
  let demand=self.advance_demands(fault,cx)?;if self.delivered||!Self::permits(cx.retained_grant(),demand){return Ok(None)}
  if self.binding.is_none(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.binding=Some(Self::original(fault,cx));return Ok(None)}
  if !self.builder.is_initialized(){self.builder.advance_initialization(cx)?;return Ok(None)}
  let buffered=self.wire.buffered_bytes(&self.buffer)?.len();
  if buffered==self.buffer.len()||self.wire.is_complete()&&buffered>0{
   if self.flush_cursor<buffered{self.builder.append_original(cx,self.wire.buffered_bytes(&self.buffer)?,&mut self.flush_cursor)?;return Ok(None)}
   let step=self.wire.drain_buffer(&self.buffer,RetainedCloneGrant{maximum_depth:cx.retained_grant().maximum_depth-1,..cx.retained_grant()})?;cx.consume_retained(step.progress())?;if step.progress().copied_items>0{self.flush_cursor=0}return Ok(None)
  }
  if !self.wire.is_complete(){let step=self.wire.advance_one(fault,&mut self.buffer,RetainedCloneGrant{maximum_depth:cx.retained_grant().maximum_depth-1,..cx.retained_grant()})?;cx.consume_retained(step.progress())?;return Ok(None)}
  if self.builder.published().is_none(){self.builder.seal(cx)?;return Ok(None)}
  let original=JobOutcomeBorrow::admit_fault(cx,self.builder.published().ok_or_else(Self::invalid)?)?;if original.is_some(){self.delivered=true}Ok(original)
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
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.retirement_demands()?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if !self.wire.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.wire.close_step(child)?.progress()))}
  if !self.builder.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.builder.close_step_granted(child)?.progress()))}
  self.binding=None;self.flush_cursor=0;self.delivered=false;self.closing=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl Default for RetainedFaultPublication{fn default()->Self{Self::new()}}
impl Drop for RetainedFaultPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"fault publication reached Drop before its original physical retirement")}}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
