//! 🚦️ Original Toy count, label and writer owners advance through distinct granted units.
use super::{OriginalToyOperationSource,ToolRunTickWriter,ToolRunVerdict,ToolRunTraceSubject};
use semio_framework_job::StepContext;
use semio_framework_tool_run::ToolRunTraceOp;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
/// 🧸️ Keeps the actual original codec and every semantic source in place until its handoff closes.
pub(super) struct OriginalToyUnit{phase:u8,operation:Option<OriginalToyOperationSource>,entity:Option<u64>,trace:Option<ToolRunTraceOp>}
impl OriginalToyUnit{
 pub(super) fn new()->Self{Self{phase:0,operation:None,entity:None,trace:None}}
 fn metadata(cx:&mut StepContext<'_>)->Result<bool,ValueError>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)}
 fn receive(cx:&mut StepContext<'_>,result:Result<RetainedCloneStep,ValueError>)->Result<bool,ValueError>{match result{Ok(step)=>{cx.consume_retained(step.progress())?;Ok(matches!(step,RetainedCloneStep::Complete(_)))},Err(original)=>{let _=cx.consume_retained(original.retained_progress());Err(original)}}}
 pub(super) fn advance(&mut self,writer:&mut ToolRunTickWriter,base:i32,unit:u32,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}match self.phase{
   0|4=>{let source=if self.phase==0{OriginalToyOperationSource::admit_count(base.checked_add(i32::try_from(unit).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Toy unit count exceeds i32"))?).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Toy count overflow"))?,cx)?}else{OriginalToyOperationSource::admit_label(unit,cx)?};if let Some(source)=source{self.operation=Some(source);self.phase+=1;}}
   1|5=>{let original=self.operation.as_mut().unwrap();if original.is_ready(){if Self::metadata(cx)?{self.phase+=1;}}else{original.advance(cx)?;}}
   2|6=>{let result=writer.admit_original_op(self.operation.as_mut().unwrap().original_wire_owner()?,grant);if Self::receive(cx,result)?{self.phase+=1;}}
   3|7=>{let original=self.operation.as_mut().unwrap();if original.terminal_is_empty(){if Self::metadata(cx)?{drop(self.operation.take());self.phase+=1;}}else{let result=original.close_step(grant);Self::receive(cx,result)?;}}
   8=>{if Self::metadata(cx)?{self.entity=Some(u64::from(unit));self.phase=9;}}
   9=>{let result=writer.admit_original_entity(&mut self.entity,grant);if Self::receive(cx,result)?{self.phase=10;}}
   10=>{if Self::metadata(cx)?{self.trace=Some(ToolRunTraceOp::Upsert{key:u64::from(unit),verdict:ToolRunVerdict::Success,reason:1,subject:ToolRunTraceSubject::Entity{entity:u64::from(unit)}});self.phase=11;}}
   11=>{let result=writer.admit_original_trace(&mut self.trace,grant);if Self::receive(cx,result)?{self.phase=12;}}
   12=>{if Self::metadata(cx)?{self.phase=0;return Ok(true)}}
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Toy unit phase is absent")),
  }Ok(false)
 }
 pub(super) fn advance_count(&mut self,writer:&mut ToolRunTickWriter,value:i32,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}match self.phase{
   0=>{if let Some(source)=OriginalToyOperationSource::admit_count(value,cx)?{self.operation=Some(source);self.phase=1;}}
   1=>{let original=self.operation.as_mut().unwrap();if original.is_ready(){if Self::metadata(cx)?{self.phase=2;}}else{original.advance(cx)?;}}
   2=>{let result=writer.admit_original_op(self.operation.as_mut().unwrap().original_wire_owner()?,grant);if Self::receive(cx,result)?{self.phase=3;}}
   3=>{let original=self.operation.as_mut().unwrap();if original.terminal_is_empty(){if Self::metadata(cx)?{drop(self.operation.take());self.phase=4;}}else{let result=original.close_step(grant);Self::receive(cx,result)?;}}
   4=>{if Self::metadata(cx)?{self.phase=0;return Ok(true)}}
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Toy count phase is absent")),
  }Ok(false)
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.phase==0&&self.operation.is_none()&&self.entity.is_none()&&self.trace.is_none()}
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{if let Some(original)=self.operation.as_ref(){if !original.terminal_is_empty(){return original.retirement_demands()}}Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(original)=self.operation.as_mut(){if !original.terminal_is_empty(){return original.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}drop(self.operation.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.entity.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.trace.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}self.phase=0;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
}
impl Drop for OriginalToyUnit{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Toy unit abandoned its original codec or writer source");}}
