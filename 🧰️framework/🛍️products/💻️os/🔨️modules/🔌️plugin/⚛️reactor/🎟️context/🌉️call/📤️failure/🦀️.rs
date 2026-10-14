//! 📤️ Entry borrows its whole original failure while bounded Pack windows become paid fault pages.
use semio_framework_job::OriginalDomainProjection;
use semio_framework_job::{OriginalTurnFailure,StepContext,RetainedPayloadBuilder,JobPayloadStream,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView};
use semio_framework_os_kernel::record::BorrowedProjectedPackCursor;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

/// 🏛️ The enclosing Entry admission owns this fixed window and metadata before the same cause is lent.
pub(super) struct RetainedOriginalFailurePublication{builder:RetainedPayloadBuilder,cursor:BorrowedProjectedPackCursor,buffer:[u8;64],binding:Option<(usize,u64,u64)>,length:usize,offset:usize,delivered:bool,closing:bool}
impl RetainedOriginalFailurePublication{
 pub(super) fn new()->Self{Self{builder:RetainedPayloadBuilder::new(JobPayloadStream::Fault),cursor:Default::default(),buffer:[0;64],binding:None,length:0,offset:0,delivered:false,closing:false}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Entry failure publication requires its same immutable cause and caller")}
 fn identity(original:&OriginalTurnFailure,cx:&StepContext<'_>)->(usize,u64,u64){(original.projection_identity(),cx.operation().0,cx.generation().0)}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub(super) fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.builder.terminal_is_empty()&&!self.closing}
 pub(super) fn advance_demands(&self,original:&OriginalTurnFailure,cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.binding.is_some_and(|bound|bound!=Self::identity(original,cx)){return Err(Self::invalid())}
  if self.delivered{return Ok(Default::default())}if self.binding.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.builder.is_initialized(){return Self::nested(self.builder.initialization_demand())}
  if self.offset<self.length{return Self::nested(self.builder.append_demand(cx.retained_grant())?)}
  if self.length>0{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.cursor.is_complete(){return Self::nested(RetirementDemand{copy_bytes:self.cursor.next_minimum_copy_bytes(),capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:self.cursor.next_advance_depth_demand()?,..Default::default()})}
  Ok(RetirementDemand{depth:2,..Default::default()})
 }
 /// 🪙️ Every encoding, copy, page, seal and descriptor action consumes the same original context once.
 pub(super) fn advance<'a>(&'a mut self,original:&OriginalTurnFailure,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
  let demand=self.advance_demands(original,cx)?;if self.delivered||cx.should_yield()||!Self::permits(cx.retained_grant(),demand){return Ok(None)}let grant=cx.retained_grant();cx.consume_fuel(1);
  if self.binding.is_none(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.binding=Some(Self::identity(original,cx));return Ok(None)}
  if !self.builder.is_initialized(){self.builder.advance_initialization(cx)?;return Ok(None)}
  if self.offset<self.length{self.builder.append_original(cx,&self.buffer[..self.length],&mut self.offset)?;return Ok(None)}
  if self.length>0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.length=0;self.offset=0;return Ok(None)}
  if !self.cursor.is_complete(){let result=self.cursor.advance_intrinsic(original,1,&mut self.buffer,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});match result{Ok(step)=>{self.length=step.written_bytes;cx.consume_retained(step.progress)?},Err(error)=>{self.length=error.written_bytes;let _=cx.consume_retained(error.reason.retained_progress());return Err(error.reason)}}return Ok(None)}
  if self.builder.published().is_none(){self.builder.seal(cx)?;return Ok(None)}let outcome=JobOutcomeBorrow::admit_fault(cx,self.builder.published().ok_or_else(Self::invalid)?)?;if outcome.is_some(){self.delivered=true}Ok(outcome)
 }
 pub(super) fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{if !self.delivered||self.closing{return Err(Self::invalid())}descriptor.fault(self.builder.published().ok_or_else(Self::invalid)?)}
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}if !self.cursor.terminal_is_empty(){return Self::nested(self.cursor.retirement_demands()?)}if !self.builder.terminal_is_empty(){return Self::nested(self.builder.retirement_demands()?)}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// ♻️ Symbols and original publication pages finish before Entry may move the whole cause into retirement.
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if !self.cursor.terminal_is_empty(){return self.cursor.close(child).map(|step|RetainedCloneStep::Progress(step.progress)).map_err(|error|error.reason)}if !self.builder.terminal_is_empty(){return self.builder.close_step_granted(child).map(|step|RetainedCloneStep::Progress(step.progress()))}
  self.binding=None;self.length=0;self.offset=0;self.delivered=false;self.closing=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl Drop for RetainedOriginalFailurePublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Entry failure publication requires funded terminal closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
