//! 🧳️ Original encoded member operations become one typed batch under their installed decoder and retirement issuers.
use super::MemberStoreOwnedBatch;
use semio_framework_os_store_one_item_publication::{lifted_prepared_operation_wire::{ArtifactOperationWire, ArtifactOriginalOperationReceiving},lifted_member_owned_batch::{BatchRetirementIssuer,BatchRetirement}};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection,RetirementDemand,ValueError,ValueRefusalKind,OriginalRetirementControl};
use semio_framework_value::retained_clone::{RetainedCloneSourceTake,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::{mem::size_of,sync::Arc};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original member batch requires its same decoder, recipient, source and output slots")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 📬️ Erases only the vocabulary; original wire, decoder and typed batch custody remain explicit.
pub trait MemberOriginalBatchReceiving:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn settling(&self)->bool;
 fn settle(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_demands(&self)->RetirementDemand;
 fn take(&mut self,batch:&mut Option<MemberStoreOwnedBatch>,wires:&mut Option<Vec<Vec<u8>>>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
/// 🪡️ Holds each exact wire vector through typed receiving and returns it before admitting the next operation.
pub struct OriginalMemberBatchReceiving<M:Send+Sync+'static>{
 codec:ArtifactOperationWire<M>,issuer:BatchRetirementIssuer<M>,factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,factory_close:Option<FactoryAuthority>,
 wires:Option<Vec<Vec<u8>>>,values:Option<Vec<M>>,batch:Option<MemberStoreOwnedBatch>,source:Option<RetainedCloneSource<Vec<u8>>>,wire:Option<RetainedOwnedProjection<Vec<u8>>>,receiver:Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,input:Option<M>,wire_close:Option<ControlledRetirement<Vec<Vec<u8>>>>,
 index:usize,body:usize,ledger:u64,recipient:u64,closing:bool,
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
  let owner=Self{codec,issuer,factory:Some(Arc::clone(factory)),factory_close:None,wires:wires.take(),values:None,batch:None,source:None,wire:None,receiver:None,input:None,wire_close:None,index:0,body:grant.maximum_copy_bytes,ledger:control.ledger_identity(),recipient,closing:false};
  Ok(Some((Box::new(owner),receipt(d))))
 }
 fn validate(&self,ledger:u64,recipient:Option<u64>)->Result<(),ValueError>{if ledger!=self.ledger||recipient!=Some(self.recipient){return Err(invalid())}Ok(())}
 fn active(&self)->bool{self.receiver.is_some()||self.wire.is_some()||self.source.is_some()}
 fn active_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(RetirementDemand{release_bytes:receiver.allocation_bytes(),..header::<Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>>()})}else{receiver.close_demands(body)}}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<Vec<u8>>>>())}else{source.next_take_demand(body)}}
  Ok(Default::default())
 }
 fn close_active(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.active_demands(self.body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..grant};
  if let Some(receiver)=self.receiver.as_mut(){if !receiver.terminal_is_empty(){receiver.begin_close();return receiver.close_step(control,grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.receiver=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if let Some(wire)=self.wire.as_mut(){if wire.terminal_is_empty(){self.wire=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return wire.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return match source.take_authority(grant)?{RetainedCloneSourceTake::Pending(p)=>Ok(RetainedCloneStep::Progress(p)),RetainedCloneSourceTake::Ready(bytes,p)=>{let slot=&mut self.wires.as_mut().ok_or_else(invalid)?[self.index];if !slot.is_empty()||slot.capacity()!=0{return Err(invalid())}*slot=bytes;Ok(RetainedCloneStep::Progress(p))}}}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<M:Send+Sync+'static> MemberOriginalBatchReceiving for OriginalMemberBatchReceiving<M>{
 fn demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(self.body)}
  if self.values.is_none()&&self.batch.is_none(){let count=self.wires.as_ref().ok_or_else(invalid)?.len();return Ok(RetirementDemand{capacity_bytes:count.checked_mul(size_of::<M>()).ok_or_else(invalid)?,..header::<Option<Vec<M>>>()})}
  if self.input.is_some(){if self.active(){return self.active_demands(self.body)}return Ok(header::<(M,usize)>())}
  if self.index==self.wires.as_ref().ok_or_else(invalid)?.len(){return Ok(RetirementDemand{capacity_bytes:MemberStoreOwnedBatch::issuer_scaffold_byte_demand::<M>(),depth:1,..Default::default()})}
  if self.source.is_none(){return Ok(RetirementDemand{capacity_bytes:RetainedCloneSource::<Vec<u8>>::owned_constructor_capacity_bytes::<()>(),..header::<Vec<u8>>()})}
  if self.wire.is_none()&&self.receiver.is_none(){return Ok(RetainedOwnedProjection::<Vec<u8>>::constructor_demand())}
  if self.receiver.is_none(){return Ok((self.codec.original_receiving.as_ref().ok_or_else(invalid)?.demands)())}
  let receiver=self.receiver.as_ref().unwrap();if receiver.ready()||receiver.owned_handoff_pending(){receiver.take_owned_demands(self.body)}else{receiver.demands(self.body)}
 }
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;if self.closing{return Err(invalid())}if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.demands(self.body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.values.is_none(){control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;let mut values=Vec::new();values.try_reserve_exact(self.wires.as_ref().unwrap().len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original member typed vector reservation failed"))?;self.values=Some(values);return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if self.input.is_some(){if self.active(){return Err(invalid())}self.values.as_mut().unwrap().push(self.input.take().unwrap());self.index+=1;return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if self.index==self.wires.as_ref().unwrap().len(){control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;let Some((batch,p))=MemberStoreOwnedBatch::admit_with_issuer(&mut self.values,self.issuer,grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.batch=Some(batch);return Ok(RetainedCloneStep::Complete(p))}
  if self.source.is_none(){control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;let bytes=std::mem::take(&mut self.wires.as_mut().unwrap()[self.index]);match RetainedCloneSource::admit_owned(bytes,(),grant){Ok((source,_))=>self.source=Some(source),Err((error,bytes,()))=>{self.wires.as_mut().unwrap()[self.index]=bytes;return Err(error)}}return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if self.wire.is_none()&&self.receiver.is_none(){let(wire,p)=self.source.as_ref().unwrap().retain_original(grant)?;self.wire=Some(wire);return Ok(RetainedCloneStep::Progress(p))}
  if self.receiver.is_none(){return (self.codec.original_receiving.as_ref().unwrap().admit)(&mut self.wire,&mut self.receiver,self.factory.as_ref().unwrap(),control,grant).map(RetainedCloneStep::Progress)}
  let receiver=self.receiver.as_mut().unwrap();if receiver.ready()||receiver.owned_handoff_pending(){let result=receiver.take_owned_into(&mut self.input,control,self.body,grant);return result.map(|step|RetainedCloneStep::Progress(step.progress()))}receiver.advance(control,self.body,grant).map(|step|RetainedCloneStep::Progress(step.progress()))
 }
 fn settling(&self)->bool{!self.closing&&self.input.is_some()&&self.active()}
 fn settle(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;if !self.settling(){return Err(invalid())}self.close_active(control,grant)}
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
 fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.validate(control.ledger_identity(),control.retirement_recipient_identity())?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(self.body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let grant=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..grant};
  if self.active(){return self.close_active(control,grant)}
  if self.input.is_some(){self.values.as_mut().ok_or_else(invalid)?.push(self.input.take().unwrap());return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if self.values.is_some(){control.charge(d.capacity_bytes)?;if let Some((batch,p))=MemberStoreOwnedBatch::admit_with_issuer(&mut self.values,self.issuer,grant)?{self.batch=Some(batch);return Ok(RetainedCloneStep::Progress(p))}return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(batch)=self.batch.as_mut(){if batch.terminal_is_empty(){self.batch=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return batch.close_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if self.wires.is_some(){control.charge(d.capacity_bytes)?;let values=self.wires.take().unwrap();match ControlledRetirement::new(values){Ok(close)=>{self.wire_close=Some(close);return Ok(RetainedCloneStep::Progress(receipt(d)))},Err((error,values))=>{self.wires=Some(values);return Err(error)}}}
  if let Some(close)=self.wire_close.as_mut(){if close.terminal_is_empty(){self.wire_close=None;return Ok(RetainedCloneStep::Progress(receipt(d)))}control.charge(d.capacity_bytes)?;return close.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(receipt(d)))}
  if let Some(close)=self.factory_close.as_mut(){if close.terminal_is_empty(){self.factory_close=None;return Ok(RetainedCloneStep::Complete(receipt(d)))}control.charge(d.capacity_bytes)?;return close.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.wires.is_none()&&self.values.is_none()&&self.batch.is_none()&&self.source.is_none()&&self.wire.is_none()&&self.receiver.is_none()&&self.input.is_none()&&self.wire_close.is_none()&&self.factory.is_none()&&self.factory_close.is_none()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
impl<M:Send+Sync+'static> Drop for OriginalMemberBatchReceiving<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original member batch receiving must close every original source and typed owner before Drop")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
