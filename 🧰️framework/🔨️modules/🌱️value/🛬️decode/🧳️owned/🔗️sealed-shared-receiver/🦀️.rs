//! 🔗️ Original native receiving seals one typed child after its own receiver has transferred and closed.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,NativeAdvanceSelection};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use std::mem::ManuallyDrop;
impl<T:super::NativeRecordFields+Send+Sync+'static> super::NativeRecordFields for SealedShared<T>{
 fn native_schema_count()->usize{T::native_schema_count()}
 fn native_schema_field(index:usize)->Option<super::NativeRecordSchema>{T::native_schema_field(index)}
}

/// 🪆️ Root construction keeps the same original decoder and receiving recipient.
pub trait NativeSharedRoot<T:crate::retirement::RetireOwned+Send+Sync+'static>:crate::retirement::RetireOwned+Send+Sync+'static+Sized{
 fn birth_work_demand(owner:&super::NativeSharedSlots<T,Self>)->Result<u64,ValueError>;
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand;
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>;
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static>NativeSharedRoot<T>for SealedShared<T>{
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand{owner.demand()}
 fn birth_work_demand(owner:&super::NativeSharedSlots<T,Self>)->Result<u64,ValueError>{owner.advance_work_demand()}
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>{owner.advance(control,SharedIssuer::owned())}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static>NativeSharedRoot<T>for std::sync::Arc<T>{
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand{owner.demand()}
 fn birth_work_demand(owner:&super::NativeSharedSlots<T,Self>)->Result<u64,ValueError>{owner.advance_work_demand()}
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>{owner.advance(control)}
}
impl<T:super::NativeRecordFields>super::NativeRecordFields for std::sync::Arc<T>{
 fn native_schema_count()->usize{T::native_schema_count()}
 fn native_schema_field(index:usize)->Option<super::NativeRecordSchema>{T::native_schema_field(index)}
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static>NativeReceive<I>for SealedShared<T>{
 type Receiver=NativeSharedReceiver<I,T>;
 fn receiver()->Self::Receiver{Self::Receiver::new(false)}
 fn record_subset_receiver()->Option<Self::Receiver>{Some(Self::Receiver::new(true))}
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static>NativeReceive<I>for std::sync::Arc<T>{
 type Receiver=NativeSharedReceiver<I,T,Self>;
 fn receiver()->Self::Receiver{Self::Receiver::new(false)}
 fn record_subset_receiver()->Option<Self::Receiver>{Some(Self::Receiver::new(true))}
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Receive,Take,Close,Seal,Ready,Spent}
/// 🧳️ The original input binding, boxed child and unpublished payload remain independently closeable.
pub struct NativeSharedReceiver<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>=SealedShared<T>>{
 binding:Option<RetainedCloneBinding>,child:crate::owned::NativeReceiveSlot<I,T>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<R>>,slots:super::NativeSharedSlots<T,R>,retirement:RetainedCloneClose,phase:Phase,closing:bool,input:std::marker::PhantomData<fn(&I)>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared receiving owner is closed, foreign or already transferred")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_depth:g.maximum_depth-1,..g}}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> NativeSharedReceiver<I,T,R>{
 fn new(subset:bool)->Self{Self{binding:None,child:if subset{crate::owned::NativeReceiveSlot::original_record_subset()}else{Default::default()},raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),slots:Default::default(),retirement:Default::default(),phase:Phase::Bind,closing:false,input:std::marker::PhantomData}}
 pub fn shared_admission(&mut self)->&mut crate::native_decoding::NativeSharedAdmission<T>{self.slots.admission()}
}

impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>>NativeSharedReceiver<I,T,R>{
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> NativeReceiver<I,R> for NativeSharedReceiver<I,T,R>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid())}original.check_binding(&self.binding)?;let metadata=||Ok(fixed(0));match self.phase{
  Phase::Bind=>if original.check_binding(&self.binding)?{metadata()}else{Ok(fixed(original.binding_copy_bytes()))},
  Phase::Receive=>if self.child.ready(){metadata()}else{nested(self.child.demands(original,body,maximum_work_units)?,0)},Phase::Take=>nested(self.child.take_demand()?,0),
  Phase::Close=>if self.child.terminal_is_empty(){metadata()}else{nested(self.child.close_demands(body)?,0)},
  Phase::Seal=>if self.output.is_some(){metadata()}else if self.raw.is_some(){Ok(super::NativeSharedSlots::<T,R>::capture_demand())}else if self.slots.output_ready(){Ok(super::NativeSharedSlots::<T,R>::take_demand())}else{Ok(R::birth_demand(&self.slots))},_=>Err(invalid())}}
 fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid())}
  let work=if !original.check_binding(&self.binding)?{original.binding_work_demand(&self.binding)?}else{match self.phase{
   Phase::Bind=>1,Phase::Receive=>if self.child.ready(){1}else{return Ok(NativeAdvanceSelection::Nested)},Phase::Take=>self.child.take_work_demand()?,
   Phase::Close=>if self.child.terminal_is_empty(){1}else{self.child.close_work_demand(body)?},
   Phase::Seal=>if self.output.is_some(){1}else if self.raw.is_some(){self.slots.capture_work_demand(&self.raw)?}else if self.slots.output_ready(){self.slots.take_work_demand(&self.output)?}else{return Ok(NativeAdvanceSelection::Nested)},_=>return Err(invalid())}};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native shared action exceeds its original work ceiling"))}Ok(NativeAdvanceSelection::Local{work})
 }
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::OriginalReceivingPrimary>{
  use crate::RetainedWorkPoll::Complete;let refuse=||Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
  if self.closing||self.binding.is_none(){return refuse()}
  match self.phase{
   Phase::Receive if !self.child.ready()=>self.child.advance(original,control,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress())),
   Phase::Seal if self.output.is_none()&&self.raw.is_none()&&!self.slots.output_ready()=>{let turn=R::receive(control,&mut self.slots);Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})},
   _=>refuse()}
 }
 fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalReceivingPrimary>,ValueError>{
  let g=effect.grant();
  let result:Result<Option<RetainedCloneStep>,crate::OriginalReceivingPrimary>=(||{
   let d=self.demands(original,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(g,d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=original.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   match self.phase{
    Phase::Take=>{return match self.child.take(&mut *self.raw,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.raw.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.child.begin_close();self.phase=Phase::Close;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}},
    Phase::Close if !self.child.terminal_is_empty()=>{return match self.child.close_step(&mut effect.nested(child_grant(g,0))){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)}},
    Phase::Seal if self.output.is_none()&&self.raw.is_some()=>{let turn=self.slots.capture(&mut self.raw,effect);return turn.result.map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(Into::into)},
    Phase::Seal if self.output.is_none()&&self.slots.output_ready()=>{let turn=self.slots.take(&mut self.output,effect);return turn.result.map(|step|Some(if self.output_ready(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})).map_err(Into::into)},
    _=>{}}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native shared action exceeded its original work").into())}
   match self.phase{Phase::Bind=>self.phase=Phase::Receive,Phase::Receive=>{if !self.child.ready(){return Err(invalid().into())}self.phase=Phase::Take},Phase::Close=>self.phase=Phase::Seal,Phase::Seal=>{if self.output.is_none(){return Err(invalid().into())}self.phase=Phase::Ready},_=>return Err(invalid().into())}
   Ok(Some(if self.output_ready(){RetainedCloneStep::Complete(receipt(fixed(0)))}else{RetainedCloneStep::Progress(receipt(fixed(0)))}))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output_state!(R,invalid,phase,Phase::Spent);
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_demand(body)}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<R>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 semio_framework_value_native_receiver_protocol::shared_close_work_quote!();
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let g=effect.grant();if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.slots.terminal_is_empty(){return self.slots.close_step(&mut effect.nested(g)).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.child.terminal_is_empty(){return self.child.close_step(&mut effect.nested(child_grant(g,0))).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|step|RetainedCloneStep::Progress(step.progress()))}
  macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}}}
  close!(raw);close!(output);
  RetainedCloneBinding::close_one(&mut self.binding,effect)}
 semio_framework_value_native_receiver_protocol::shared_original_close_bridge!();
 fn terminal_is_empty(&self)->bool{self.slots.terminal_is_empty()&&self.child.terminal_is_empty()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}

}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> Drop for NativeSharedReceiver<I,T,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original shared receiving requires funded closure");}}


#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
