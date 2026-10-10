//! 🌉️ One admitted guest entry retains its original semantic fault through publication and physical closure.
use super::*;
use semio_framework_diagnostic::Fault;
use semio_framework_job::{JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,RetainedFaultPublication};
use semio_framework_value::{retirement::controlled::ControlledRetirement,RetainedCloneGrant};
#[path="📤️receipt/🦀️.rs"]
mod fixed_receipt;
pub use fixed_receipt::{OriginalActorFixedCall,OriginalActorFixedKind,OriginalActorFixedReceipt};

#[derive(Debug)]
pub enum OriginalActorEntryFailure{Authority(ValueError),Semantic}
/// 🏛️ The original enclosing context binding admits these inline publication and failure slots together.
pub struct OriginalActorGuestEntry{context:OriginalActorContextSlot,fault:ManuallyDrop<Option<Fault>>,publication:RetainedFaultPublication,retirement:ManuallyDrop<Option<ControlledRetirement<Fault>>>}
impl OriginalActorGuestEntry{
 pub fn new()->Self{Self{context:OriginalActorContextSlot::empty(),fault:ManuallyDrop::new(None),publication:RetainedFaultPublication::new(),retirement:ManuallyDrop::new(None)}}
 pub fn admit_step(&mut self,input:RetainedTurnInput)->OriginalActorCall<OriginalActorAdmission,ValueError>{if self.fault.is_some()||self.retirement.is_some(){return OriginalActorCall{input,receipt_spent:Default::default(),result:Err(refusal("original guest failure remains in paid custody"))}}self.context.admit_step(input)}
 /// 🤝️ Returns raw effects before semantic classification and captures the same original fault once.
 pub fn dispatch_step<T>(&mut self,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,Fault>)->OriginalActorCall<T,OriginalActorEntryFailure>{
  if self.fault.is_some()||self.retirement.is_some(){return OriginalActorCall{input,receipt_spent:*recipient,result:Err(OriginalActorEntryFailure::Authority(refusal("original guest failure fences a second semantic producer")))}}
  let original=self.context.dispatch_step(input,budget,recipient,clock,produce);
  let result=match original.result{Ok(value)=>Ok(value),Err(OriginalActorCallError::Authority(error))=>Err(OriginalActorEntryFailure::Authority(error)),Err(OriginalActorCallError::Semantic(fault))=>{*self.fault=Some(fault);Err(OriginalActorEntryFailure::Semantic)}};
  OriginalActorCall{input:original.input,receipt_spent:original.receipt_spent,result}
 }
 pub fn original_fault(&self)->Option<&Fault>{self.fault.as_ref()}
 /// 📤️ Lends the same original typed fault to the shared paid window and immutable descriptor.
 pub fn publish_failure_step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{let original=self.fault.as_ref().ok_or_else(||refusal("original guest has no retained semantic fault"))?;self.publication.advance_from_fault(original,cx)}
 /// 🎟️ Executes one original issued failure-publication turn through the same paid context and recipient.
 pub fn publish_failure_call(&mut self,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>)->OriginalActorCall<Option<JobOutcomeDescriptor>,OriginalActorCallError<ValueError>>{let Self{context,fault,publication,..}=self;context.dispatch_step(input,budget,recipient,clock,|cx|{let original=fault.as_ref().ok_or_else(||refusal("original guest has no retained semantic fault"))?;publication.advance_from_fault(original,cx).map(|loan|loan.map(JobOutcomeBorrow::into_descriptor))})}
 pub fn borrow_failure_outcome<'a>(&'a self,original:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{self.publication.borrow_outcome(original)}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||refusal("original guest failure depth overflow"))?;Ok(demand)}
 fn close_failure_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.publication.terminal_is_empty(){let demand=Self::nested(self.publication.retirement_demands()?)?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}return self.publication.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if self.fault.is_some(){if !Self::permits(grant,RetirementDemand{depth:1,..Default::default()}){return Ok(RetainedCloneStep::Progress(Default::default()))}let original=self.fault.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>{*self.retirement=Some(owner);return Ok(metadata(false))},Err((error,original))=>{*self.fault=Some(original);return Err(error)}}}
  if let Some(original)=self.retirement.as_mut(){if !original.terminal_is_empty(){let demand=Self::nested(RetirementDemand{copy_bytes:original.next_copy_byte_demand()?,capacity_bytes:original.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:original.next_release_byte_demand()?,depth:original.next_depth_demand()?})?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}return original.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}if !Self::permits(grant,RetirementDemand{depth:1,..Default::default()}){return Ok(RetainedCloneStep::Progress(Default::default()))}drop(self.retirement.take());return Ok(metadata(false))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// ♻️ Publisher and original fault finish before the same cancellation and ledger context closes.
 pub fn close_step(&mut self,input:RetainedTurnInput)->OriginalActorCall<RetainedCloneStep,OriginalActorContextError>{
  if self.fault.is_none()&&self.retirement.is_none()&&self.publication.terminal_is_empty(){return self.context.close_step(input)}
  let result=(||{self.context.validate_binding(input)?;self.context.binding=Some((input.operation,input.generation,input.epoch));self.close_failure_step(input.grant)})();let receipt_spent=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};OriginalActorCall{input,receipt_spent,result:result.map_err(OriginalActorContextError::Refused)}
 }
 pub fn terminal_is_empty(&self)->bool{self.context.terminal_is_empty()&&self.fault.is_none()&&self.retirement.is_none()&&self.publication.terminal_is_empty()}
}
impl Drop for OriginalActorGuestEntry{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original guest entry reached Drop before its paid fault and context closure")}}

impl Default for OriginalActorGuestEntry{fn default()->Self{Self::new()}}
