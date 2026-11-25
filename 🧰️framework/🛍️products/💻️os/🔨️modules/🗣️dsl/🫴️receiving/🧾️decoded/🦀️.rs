//! 🧾️ An operation whose authored binary codec is its only wire decoder keeps one paid decode and its typed output across original native turns.
use super::*;
use protocol::OpBinary;

/// 🧾️ Receives one stored operation by running its authored [`OpBinary::decode_op`] once its bytes are paid for, then hands the typed output to its original recipient.
pub struct OriginalDecodedOperationReceiving<M:OpBinary+RetireOwned>{
 source:(usize,usize),range:(usize,usize),output:OwnedSlot<M>,paid:usize,phase:u8,closing:u8,closed:bool,failed:bool,
}
impl<M:OpBinary+RetireOwned> OriginalDecodedOperationReceiving<M>{
 /// 🎟️ Preserves the full original source and the exact operation range without parsing or allocating.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let mut work=0;
  let result=(||{
   if range.0>range.1||range.1>source.len(){return Err(invalid("original decoded operation escaped its full source"))}
   if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
   work=1;
   Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,output:Default::default(),paid:0,phase:0,closing:0,closed:false,failed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
  })();
  RetainedWorkOutcome{result,work}
 }
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original decoded operation source identity changed"))}Ok(())}
 fn quote(&self)->usize{(self.range.1-self.range.0).saturating_add(size_of::<Option<M>>())}
 fn demands_inner(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  if self.closing!=0{return self.close_demand(body)}
  let mut demand=if self.phase==0{let remaining=self.quote().saturating_sub(self.paid);RetirementDemand{copy_bytes:remaining.min(body.max(1)),depth:1,..Default::default()}}else{metadata::<M>()};
  demand.depth+=1;
  Ok(demand)
 }
 fn step_action(&mut self,source:&[u8],body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  let demand=self.demands_inner(source,body)?;
  if self.failed||self.phase!=0||self.closing!=0||!funded(effect.grant(),demand){return Ok(Default::default())}
  perform(effect,1)?;
  if self.paid.saturating_add(demand.copy_bytes)<self.quote(){self.paid+=demand.copy_bytes;return Ok(receipt(demand))}
  let decoded=M::decode_op(&source[self.range.0..self.range.1]).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,format!("{error:?}")).with_retained_progress(receipt(demand)))?;
  *self.output.value=Some(decoded);self.phase=1;
  Ok(receipt(demand))
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}
  if self.closing==0{return Ok(metadata::<()>())}
  if !self.output.terminal_is_empty(){return self.output.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn close_work(&self,body:usize)->Result<u64,ValueError>{
  if self.closed{return Ok(0)}
  if self.closing==0||self.output.terminal_is_empty(){return Ok(1)}
  self.output.close_work_demand(body)
 }
 fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let child=grant_child(grant);
  let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  let demand=match self.close_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return settle(effect,Err(error))};
  if !funded(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  match self.close_work(grant.maximum_copy_bytes).and_then(|work|effect.remaining_work_units().map(|remaining|remaining>=work)){
   Ok(true)=>{}
   Ok(false)=>return settle(effect,Ok(RetainedCloneStep::Progress(Default::default()))),
   Err(error)=>return settle(effect,Err(error)),
  }
  if self.closing==0{if let Err(error)=perform(effect,1){return settle(effect,Err(error))}self.closing=1;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if !self.output.terminal_is_empty(){return stepped(self.output.close_step(&mut effect.nested(child)))}
  if let Err(error)=perform(effect,1){return settle(effect,Err(error))}
  self.closed=true;settle(effect,Ok(RetainedCloneStep::Complete(receipt(demand))))
 }
}
/// ♻️ Retires the decoded receiving's remaining output through its paid close.
#[doc(hidden)]
pub struct DecodedOperationRetirement<M:OpBinary+RetireOwned>(ManuallyDrop<OriginalDecodedOperationReceiving<M>>);
impl<M:OpBinary+RetireOwned> RetireOwned for OriginalDecodedOperationReceiving<M>{
 type RetirementCursor=DecodedOperationRetirement<M>;
 fn retirement_seed(self)->Self::RetirementCursor{DecodedOperationRetirement(ManuallyDrop::new(self))}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:OpBinary+RetireOwned> DecodedOperationRetirement<M>{
 fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{erased_cursor_turn(self.0.close(effect))}
}
impl<M:OpBinary+RetireOwned> RetirementCursor for DecodedOperationRetirement<M>{
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.close_work(body)}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:OpBinary+RetireOwned> Drop for DecodedOperationRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original decoded operation retirement lost its output");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}
impl<M:OpBinaryReceiving<DomainFailure=ValueError,Receiving=OriginalDecodedOperationReceiving<M>>> OpBinaryReceivingCursor<M> for OriginalDecodedOperationReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands_inner(source,body)}
 fn step_in(&mut self,source:&[u8],_native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let result=self.step_action(source,body,effect);if result.is_err(){self.failed=true}RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn domain_failure(&self)->Option<&ValueError>{None}
 fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn domain_failure_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
 fn take_domain_failure_into(&mut self,recipient:&mut Option<ValueError>,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{RetainedWorkOutcome{result:if recipient.is_some(){Err(invalid("decoded operation domain failure requires an empty original recipient"))}else{Ok(Default::default())},work:0}}
 fn is_ready(&self)->bool{self.phase==1&&self.closing==0&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  let demand=metadata::<M>();
  if self.failed||self.phase!=1||self.closing!=0||recipient.is_some()||self.output.value.is_none()||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  *recipient=self.output.value.take();RetainedWorkOutcome{result:Ok(receipt(demand)),work:1}
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_work(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close(effect)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.output.terminal_is_empty()}
}
/// 🧾️ Implements an operation receiving whose authored `OpBinary` codec is its only wire decoder.
#[macro_export]
macro_rules! original_decoded_operation_receiving{
 ($operation:ty)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type DomainFailure=$crate::original_receiving::ValueError;
  type Receiving=$crate::original_receiving::OriginalDecodedOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{<Self::Receiving>::admit(source,range,grant)}
 }};
}
