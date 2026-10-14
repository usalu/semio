//! 📥️ Canonical ingress is born in the same resident Entry before any incoming byte donation.
use super::*;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct OriginalPollInputHeader{operation:u64,generation:u64,extent:usize}
pub(super) struct OriginalActorPollInput{header:Option<OriginalPollInputHeader>,bytes:ManuallyDrop<Option<Vec<u8>>>,retirement:ManuallyDrop<Option<ControlledRetirement<Vec<u8>>>>,closing:bool}
impl OriginalActorPollInput{
 pub(super) fn empty()->Self{Self{header:None,bytes:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),closing:false}}
 pub(super) fn terminal_is_empty(&self)->bool{self.header.is_none()&&self.bytes.is_none()&&self.retirement.is_none()}
 fn matches(&self,input:RetainedTurnInput)->bool{self.header.is_none_or(|header|(header.operation,header.generation)==(input.operation,input.generation))}
}
impl OriginalActorGuestEntry{
 pub fn has_original_poll_input(&self)->bool{!self.poll_input.terminal_is_empty()}
 /// 🔎️ The initialized original extent remains borrowed inside the resident source callback.
 pub fn with_original_poll_input<R>(&self,read:impl FnOnce(&[u8])->R)->Option<R>{self.poll_input.bytes.as_ref().map(|bytes|read(bytes))}
 fn poll_input_result<T>(&mut self,call:OriginalActorCall<T,OriginalActorCallError<OriginalTurnFailure>>)->OriginalActorCall<T,OriginalActorEntryFailure>{let result=match call.result{Ok(value)=>Ok(value),Err(OriginalActorCallError::Authority(error))=>Err(OriginalActorEntryFailure::Authority(error)),Err(OriginalActorCallError::Semantic(error))=>{assert!(self.value_fault.is_none(),"original poll input retains its whole first cause");*self.value_fault=Some(error);Err(OriginalActorEntryFailure::Semantic)}};OriginalActorCall{input:call.input,receipt_spent:call.receipt_spent,work_units_spent:call.work_units_spent,result}}
 fn poll_input_refusal<T>(&self,input:RetainedTurnInput,recipient:&RetainedCloneProgress)->OriginalActorCall<T,OriginalActorEntryFailure>{OriginalActorCall{input,receipt_spent:*recipient,work_units_spent:0,result:Err(OriginalActorEntryFailure::Authority(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original poll input source or failure remains in resident custody")))}}
 /// 🏛️ Header birth and exact original backing reservation are separate funded frontiers.
 pub fn reserve_poll_input_step(&mut self,extent:usize,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>)->OriginalActorCall<bool,OriginalActorEntryFailure>{
  if !self.poll_input.matches(input)||self.poll_input.closing||self.has_close_failure()||self.value_fault.is_some()||self.fault.is_some(){return self.poll_input_refusal(input,recipient)}
  if self.poll_input.header.is_some_and(|header|header.extent!=extent){return self.poll_input_refusal(input,recipient)}
  let source=&mut self.poll_input;let call=self.context.dispatch_step(input,budget,recipient,clock,|cx|->Result<bool,OriginalTurnFailure>{
   let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
   if source.header.is_none(){cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;source.header=Some(OriginalPollInputHeader{operation:input.operation,generation:input.generation,extent});return Ok(false)}
   if source.bytes.is_some(){return Ok(true)}if grant.maximum_capacity_bytes<extent{return Ok(false)}cx.consume_fuel(1);let mut bytes=Vec::new();bytes.try_reserve_exact(extent).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original poll input backing reservation refused"))?;let capacity=bytes.capacity();*source.bytes=Some(bytes);cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;Ok(true)
  });self.poll_input_result(call)
 }
 /// 📄️ One caller-owned fixed window initializes only the same pre-reserved backing.
 pub fn write_poll_input_step(&mut self,offset:usize,source:&[u8],input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>)->OriginalActorCall<bool,OriginalActorEntryFailure>{self.write_poll_input_body_step(offset,source.len(),input,budget,recipient,clock,|bytes|bytes.extend_from_slice(source))}
 /// 🔣️ Scalar canonical words are decoded only inside the actual funded initialization frontier.
 pub fn write_poll_input_words_step(&mut self,offset:usize,length:usize,words:[u64;8],input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>)->OriginalActorCall<bool,OriginalActorEntryFailure>{self.write_poll_input_body_step(offset,length,input,budget,recipient,clock,|bytes|{let mut remaining=length;for word in words{if remaining==0{break}let count=remaining.min(8);bytes.extend_from_slice(&word.to_le_bytes()[..count]);remaining-=count}})}
 fn write_poll_input_body_step(&mut self,offset:usize,length:usize,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>,initialize:impl FnOnce(&mut Vec<u8>))->OriginalActorCall<bool,OriginalActorEntryFailure>{
  if !self.poll_input.matches(input)||self.poll_input.closing||self.value_fault.is_some()||self.fault.is_some()||self.has_close_failure(){return self.poll_input_refusal(input,recipient)}
  let held=&mut self.poll_input;let call=self.context.dispatch_step(input,budget,recipient,clock,|cx|->Result<bool,OriginalTurnFailure>{
   let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<length{return Ok(false)}
   let header=held.header.ok_or_else(||refusal("original poll input header has not been admitted"))?;let bytes=held.bytes.as_mut().ok_or_else(||refusal("original poll input backing has not been reserved"))?;
   if length>64||length==0||offset!=bytes.len()||offset.checked_add(length).is_none_or(|end|end>header.extent){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original poll input window differs from its resident frontier").into())}
   cx.consume_fuel(1);initialize(bytes);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:length,..Default::default()})?;Ok(true)
  });self.poll_input_result(call)
 }
 /// ♻️ Partial or completed ingress retires its exact backing before its inline source header.
 pub fn close_poll_input_step(&mut self,input:RetainedTurnInput,budget:StepBudget,recipient:&mut RetainedCloneProgress,clock:fn()->Option<u64>)->OriginalActorCall<bool,OriginalActorEntryFailure>{
  if !self.poll_input.matches(input){return self.poll_input_refusal(input,recipient)}
  let source=&mut self.poll_input;let call=self.context.dispatch_step(input,budget,recipient,clock,|cx|->Result<bool,OriginalTurnFailure>{
   if source.terminal_is_empty(){return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
   if source.bytes.is_some(){cx.consume_fuel(1);let bytes=source.bytes.take().unwrap();match ControlledRetirement::new(bytes){Ok(owner)=>{*source.retirement=Some(owner);source.closing=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(false)},Err((error,bytes))=>{*source.bytes=Some(bytes);Err(error.into())}}}
   else if let Some(owner)=source.retirement.as_mut(){if !owner.terminal_is_empty(){let demand=RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?};if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(false)}cx.consume_fuel(1);let step=owner.step(RetainedCloneGrant{maximum_items:1,..grant}).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;cx.consume_retained(step.progress())?;return Ok(false)}cx.consume_fuel(1);drop(source.retirement.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(false)}
   else{cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;source.header=None;source.closing=false;Ok(true)}
  });self.poll_input_result(call)
 }
}
