//! 🧳️ Original encoded member operations become one typed batch under their installed decoder and retirement issuers.
use super::MemberStoreOwnedBatch;
use semio_os_store_one_item_publication::{lifted_prepared_operation_wire::{ArtifactOperationWire, ArtifactOriginalOperationReceiving},lifted_member_owned_batch::{BatchRetirementIssuer,BatchRetirement}};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection,RetirementDemand,ValueError,ValueRefusalKind,OriginalRetirementControl,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit};
use semio_os_store_one_item_publication::native_source::pay;
use semio_framework_value::retained_clone::{RetainedCloneSourceAdmission,RetainedCloneSourceTake,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_native_owned::{NativeAllocationIdentity,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
use std::{mem::size_of,sync::Arc};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original member batch requires its same decoder, recipient, source and output slots")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 📬️ Erases only the vocabulary; original wire, decoder and typed batch custody remain explicit.
pub trait MemberOriginalBatchReceiving:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>;
 fn settling(&self)->bool;
 fn settle(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn ready(&self)->bool;
 fn take_demands(&self)->RetirementDemand;
 fn take(&mut self,batch:&mut Option<MemberStoreOwnedBatch>,wires:&mut Option<Vec<Vec<u8>>>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
/// 🪡️ Holds each exact wire vector through typed receiving and returns it before admitting the next operation.
pub struct OriginalMemberBatchReceiving<M:Send+Sync+'static>{
 codec:ArtifactOperationWire<M>,issuer:BatchRetirementIssuer<M>,factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,factory_close:Option<FactoryAuthority>,
 wires:Option<Vec<Vec<u8>>>,values:Option<Vec<M>>,batch:Option<MemberStoreOwnedBatch>,source:Option<RetainedCloneSource<Vec<u8>>>,source_admission:Option<RetainedCloneSourceAdmission<Vec<u8>,Vec<u8>,()>>,wire:Option<RetainedOwnedProjection<Vec<u8>>>,receiver:Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,input:Option<M>,wire_close:Option<ControlledRetirement<Vec<Vec<u8>>>>,
 index:usize,body:usize,ledger:NativeAllocationIdentity,recipient:u64,closing:bool,
}
impl<M:Send+Sync+'static> OriginalMemberBatchReceiving<M>{
 /// 📐️ Quotes the concrete receiver frame and original vector/factory header transfers.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..header::<Self>()}}
 /// 🎟️ Takes the exact encoded vector only after every constructor axis and original receiving identity is admitted.
 pub fn admit(wires:&mut Option<Vec<Vec<u8>>>,codec:ArtifactOperationWire<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn MemberOriginalBatchReceiving>,RetainedCloneProgress)>,ValueError>{
  let issuer=codec.original_receiving.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"member operation wire has no original receiving authority"))?.batch_retirement;
  let recipient=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"member batch requires its original retirement recipient"))?;
  let d=Self::admission_demands();if wires.is_none()||!retained_clone_grant_admits(grant,d){return Ok(None)}
  control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;
  let owner=Self{codec,issuer,factory:Some(Arc::clone(factory)),factory_close:None,wires:wires.take(),values:None,batch:None,source:None,source_admission:None,wire:None,receiver:None,input:None,wire_close:None,index:0,body:grant.maximum_copy_bytes,ledger:control.ledger_identity(),recipient,closing:false};
  Ok(Some((Box::new(owner),receipt(d))))
 }
 fn validate(&self,ledger:NativeAllocationIdentity,recipient:Option<u64>)->Result<(),ValueError>{if ledger!=self.ledger||recipient!=Some(self.recipient){return Err(invalid())}Ok(())}
 fn active(&self)->bool{self.receiver.is_some()||self.wire.is_some()||self.source.is_some()||self.source_admission.is_some()}
 fn active_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(RetirementDemand{release_bytes:receiver.allocation_bytes(),..header::<Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>>()})}else{receiver.close_demands(body)}}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)}}
  if let Some(admission)=self.source_admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<Vec<u8>>>>())}else{source.next_take_demand(body)}}
  Ok(Default::default())
 }

 fn advance_turn(&mut self,control:&mut NativeDecodeControl<'_>,maximum_work_units:u64,grant:RetainedCloneGrant,work:&mut u128)->Result<RetainedCloneStep,ValueError>{
  self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;if self.closing{return Err(invalid())}if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if self.values.is_some()&&self.input.is_none()&&self.source.is_none()&&self.source_admission.is_none()&&self.wire.is_none()&&self.receiver.is_none()&&self.index<self.wires.as_ref().ok_or_else(invalid)?.len(){let bytes=std::mem::take(&mut self.wires.as_mut().ok_or_else(invalid)?[self.index]);self.source_admission=Some(RetainedCloneSource::admission_owned(bytes,()));}
  let d=self.demands(self.body)?;if maximum_work_units==0||!retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  control.checkpoint()?;
  if self.source.is_some()&&self.wire.is_some()&&self.receiver.is_none(){
   control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;*work=1;
   return (self.codec.original_receiving.as_ref().ok_or_else(invalid)?.admit)(&mut self.wire,&mut self.receiver,self.factory.as_ref().ok_or_else(invalid)?,control,grant).map(RetainedCloneStep::Progress);
  }
  if let Some(receiver)=self.receiver.as_mut(){if !(receiver.ready()||receiver.owned_handoff_pending())&&self.input.is_none(){
   let poll=receiver.advance(control,self.body,maximum_work_units,grant);
   return match poll{RetainedWorkPoll::Pending{work:pending}=>{*work=pending;Ok(RetainedCloneStep::Progress(Default::default()))},RetainedWorkPoll::Complete(turn)=>{*work=turn.work;turn.result.map_err(|failure|{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(invalid).with_retained_progress(progress)})}};
  }}
  let quote=if self.receiver.is_none()&&self.wire.is_none()&&self.source.is_some(){maximum_work_units}else{1};
  let Some((result,spent))=control.with_allocated_effect(d,quote,|control,effect|{
   let result=self.advance_local(control,d,grant,effect);
   let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
   if d.capacity_bytes!=0&&progress.retained_capacity_bytes!=0&&effect.actual().is_none(){*effect.actual()=Some(progress.retained_capacity_bytes);}
   let result=match result{Ok(step) if effect.performed_work()==0&&step.progress()!=RetainedCloneProgress::default()=>pay(effect,1).map(|()|step),other=>other};
   (result,effect.performed_work())
  })? else{return Ok(RetainedCloneStep::Progress(Default::default()))};
  *work=spent;result
 }
 fn advance_local(&mut self,control:&mut NativeDecodeControl<'_>,d:RetirementDemand,grant:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  if self.values.is_none(){let mut values=Vec::new();values.try_reserve_exact(self.wires.as_ref().ok_or_else(invalid)?.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original member typed vector reservation failed"))?;let bytes=values.capacity()*size_of::<M>();self.values=Some(values);if d.capacity_bytes!=0{*effect.actual()=Some(bytes);}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{retained_capacity_bytes:bytes,..receipt(d)}))}
  if self.input.is_some(){if self.active(){return Err(invalid())}self.values.as_mut().ok_or_else(invalid)?.push(self.input.take().ok_or_else(invalid)?);self.index+=1;return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if self.index==self.wires.as_ref().ok_or_else(invalid)?.len(){let Some((batch,p))=MemberStoreOwnedBatch::admit_with_issuer(&mut self.values,self.issuer,grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.batch=Some(batch);return Ok(RetainedCloneStep::Complete(p))}
  if self.source.is_none(){let admission=self.source_admission.as_mut().ok_or_else(invalid)?;let step=if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);pay(effect,turn.work)?;let step=turn.result?;if self.source.is_some(){self.source_admission=None;}step}else{admission.advance(&mut effect.nested(grant)).result?};return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.wire.is_none()&&self.receiver.is_none(){let(wire,p)=self.source.as_ref().ok_or_else(invalid)?.retain_original(&mut effect.nested(grant)).result?;self.wire=Some(wire);return Ok(RetainedCloneStep::Progress(p))}
  let receiver=self.receiver.as_mut().ok_or_else(||ValueError::invariant("receiver absent"))?;
  let step=receiver.take_owned_into(&mut self.input,control,self.body,&mut effect.nested(grant)).result?;
  Ok(RetainedCloneStep::Progress(step.progress()))
 }
 fn close_active(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();
  let d=self.active_demands(self.body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..grant};
  if let Some(receiver)=self.receiver.as_mut(){if !receiver.terminal_is_empty(){receiver.begin_close();return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(receiver.close_step(control,&mut effect.nested(grant)))?.progress()))}self.receiver=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if let Some(wire)=self.wire.as_mut(){if wire.terminal_is_empty(){self.wire=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(wire.close_step(&mut effect.nested(grant)))?.progress()))}
  if let Some(admission)=self.source_admission.as_mut(){if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);pay(effect,turn.work)?;let step=turn.result?;if self.source.is_some(){self.source_admission=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}let mut inputs=None;let step=admission.return_inputs(&mut inputs,&mut effect.nested(grant)).result?;if let Some((bytes,()))=inputs{self.wires.as_mut().ok_or_else(invalid)?[self.index]=bytes;}if admission.terminal_is_empty(){self.source_admission=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return match RetirementPollExit::resume(source.take_authority(&mut effect.nested(grant)))?{RetainedCloneSourceTake::Pending(p)=>Ok(RetainedCloneStep::Progress(p)),RetainedCloneSourceTake::Ready(bytes,p)=>{let slot=&mut self.wires.as_mut().ok_or_else(invalid)?[self.index];if !slot.is_empty()||slot.capacity()!=0{return Err(invalid().into())}*slot=bytes;Ok(RetainedCloneStep::Progress(p))}}}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<M:Send+Sync+'static> MemberOriginalBatchReceiving for OriginalMemberBatchReceiving<M>{
 fn demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(self.body)}
  if self.values.is_none()&&self.batch.is_none(){let count=self.wires.as_ref().ok_or_else(invalid)?.len();return Ok(RetirementDemand{capacity_bytes:count.checked_mul(size_of::<M>()).ok_or_else(invalid)?,..header::<Option<Vec<M>>>()})}
  if self.input.is_some(){if self.active(){return self.active_demands(self.body)}return Ok(header::<(M,usize)>())}
  if self.index==self.wires.as_ref().ok_or_else(invalid)?.len(){return Ok(RetirementDemand{capacity_bytes:MemberStoreOwnedBatch::issuer_scaffold_byte_demand::<M>(),depth:1,..Default::default()})}
  if self.source.is_none(){return self.source_admission.as_ref().ok_or_else(invalid)?.next_demand()}
  if self.wire.is_none()&&self.receiver.is_none(){return Ok(RetainedOwnedProjection::<Vec<u8>>::constructor_demand())}
  if self.receiver.is_none(){return Ok((self.codec.original_receiving.as_ref().ok_or_else(invalid)?.demands)())}
  let receiver=self.receiver.as_ref().unwrap();if receiver.ready()||receiver.owned_handoff_pending(){receiver.take_owned_demands(self.body)}else{receiver.demands(self.body)}
 }
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let mut work=0;let result=self.advance_turn(control,maximum_work_units,grant,&mut work);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work})
 }
 fn settling(&self)->bool{!self.closing&&self.input.is_some()&&self.active()}
 fn settle(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=(||->Result<RetainedCloneStep,RetirementPollExit>{self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;if !self.settling(){return Err(invalid().into())}self.close_active(control,effect)})();RetirementPollExit::settle(effect,result)}
 fn ready(&self)->bool{!self.closing&&self.batch.is_some()}
 fn take_demands(&self)->RetirementDemand{header::<(Option<MemberStoreOwnedBatch>,Option<Vec<Vec<u8>>>)>()}
 fn take(&mut self,batch:&mut Option<MemberStoreOwnedBatch>,wires:&mut Option<Vec<Vec<u8>>>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;if batch.is_some()||wires.is_some(){return Err(invalid())}let d=header::<(Option<MemberStoreOwnedBatch>,Option<Vec<Vec<u8>>>)>();if !self.ready()||!retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}*batch=self.batch.take();*wires=self.wires.take();self.closing=true;Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
 fn close_demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.active(){return self.active_demands(self.body)}
  if self.input.is_some(){return Ok(header::<M>())}
  if self.values.is_some(){return Ok(RetirementDemand{capacity_bytes:MemberStoreOwnedBatch::issuer_scaffold_byte_demand::<M>(),depth:1,..Default::default()})}
  if let Some(batch)=self.batch.as_ref(){return if batch.terminal_is_empty(){Ok(header::<Option<MemberStoreOwnedBatch>>())}else{batch.next_demands(self.body)}}
  if self.wires.is_some(){return Ok(header::<Option<ControlledRetirement<Vec<Vec<u8>>>>>())}
  if let Some(close)=self.wire_close.as_ref(){return if close.terminal_is_empty(){Ok(header::<Option<ControlledRetirement<Vec<Vec<u8>>>>>())}else{close.next_demand(self.body)}}
  if self.factory.is_some(){return Ok(header::<Option<FactoryAuthority>>())}
  self.factory_close.as_ref().map_or(Ok(Default::default()),|close|if close.terminal_is_empty(){Ok(header::<Option<FactoryAuthority>>())}else{close.demands(self.body)})
 }
 fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let grant=effect.grant();
   self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(self.body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..grant};
   if self.active(){return self.close_active(control,effect)}
   if self.input.is_some(){self.values.as_mut().ok_or_else(invalid)?.push(self.input.take().ok_or_else(invalid)?);return Ok(RetainedCloneStep::Progress(receipt(d)))}
   if self.values.is_some(){control.charge(d.capacity_bytes)?;if let Some((batch,p))=MemberStoreOwnedBatch::admit_with_issuer(&mut self.values,self.issuer,grant)?{self.batch=Some(batch);return Ok(RetainedCloneStep::Progress(p))}return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(batch)=self.batch.as_mut(){if batch.terminal_is_empty(){self.batch=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(batch.close_granted(&mut effect.nested(grant))?.progress()))}
   if self.wires.is_some(){control.charge(d.capacity_bytes)?;let values=self.wires.take().ok_or_else(invalid)?;match ControlledRetirement::new(values){Ok(close)=>{self.wire_close=Some(close);return Ok(RetainedCloneStep::Progress(receipt(d)))},Err((error,values))=>{self.wires=Some(values);return Err(error.into())}}}
   if let Some(close)=self.wire_close.as_mut(){if close.terminal_is_empty(){self.wire_close=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(close.step(&mut effect.nested(grant)))?.progress()))}
   if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(receipt(d)))}
   if let Some(close)=self.factory_close.as_mut(){if close.terminal_is_empty(){self.factory_close=None;return Ok(RetainedCloneStep::Complete(receipt(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(close.step(&mut effect.nested(grant)))?.progress()))}
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  RetirementPollExit::settle(effect,result)
 }
 fn terminal_is_empty(&self)->bool{self.wires.is_none()&&self.values.is_none()&&self.batch.is_none()&&self.source.is_none()&&self.source_admission.is_none()&&self.wire.is_none()&&self.receiver.is_none()&&self.input.is_none()&&self.wire_close.is_none()&&self.factory.is_none()&&self.factory_close.is_none()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
impl<M:Send+Sync+'static> Drop for OriginalMemberBatchReceiving<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original member batch receiving must close every original source and typed owner before Drop")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
