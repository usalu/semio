//! 🔢️ Fixed arrays retain initialized rows, source aliases and actual construction receipts until funded retirement.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::*;
use crate::{OriginalEffectReply,OriginalEffectReplyComplete};
use std::mem::MaybeUninit;
pub struct NativeArrayReceiver<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize>where[T;N]:RetireOwned{
 binding:Option<RetainedCloneBinding>,source:ManuallyDrop<Option<crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>>>,child:NativeReceiveSlot<dyn NativeValueSource,T>,rows:ManuallyDrop<MaybeUninit<[T;N]>>,pending:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<[T;N]>>,retirement:RetainedCloneClose,index:usize,live:usize,phase:u8,closing:bool,failed:bool,input:std::marker::PhantomData<fn(&I)>
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize>NativeArrayReceiver<I,T,N>where[T;N]:RetireOwned{
 pub fn new()->Self{Self{binding:None,source:ManuallyDrop::new(None),child:Default::default(),rows:ManuallyDrop::new(MaybeUninit::uninit()),pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),index:0,live:0,phase:0,closing:false,failed:false,input:std::marker::PhantomData}}
 fn projection_demand()->Result<RetirementDemand,ValueError>{let mut d=crate::retained_clone::RetainedOwnedProjection::<dyn NativeValueSource>::constructor_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>>>()+size_of::<u8>()).ok_or_else(invalid)?;Ok(d)}
 fn place_demand()->RetirementDemand{fixed(size_of::<T>()+size_of::<Option<T>>()+2*size_of::<usize>()+size_of::<u8>()+size_of::<bool>())}
 fn child_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.child.is_closing(){self.child.close_demands(body)}else{self.child.begin_close_demand()}}
 fn child_close_work(&self,body:usize)->Result<u64,ValueError>{if self.child.is_closing(){self.child.close_work_demand(body)}else{self.child.begin_close_work_demand()}}
 fn close_dispatch<'reply>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let d=self.close_demands(body)?;if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.child.terminal_is_empty(){if !self.child.is_closing(){return if recover{self.child.recover_begin_close_into(effect,reply)}else{self.child.begin_close_into(effect,reply)}}return match(original,recover){(Some(loan),false)=>self.child.close_original_into(loan,body,effect,reply),(Some(loan),true)=>self.child.recover_close_original_into(loan,body,effect,reply),(None,false)=>self.child.close_into(body,effect,reply),(None,true)=>self.child.recover_close_into(body,effect,reply)}}
  if !self.retirement.is_empty(){return match(original,recover){(Some(loan),false)=>self.retirement.step_original_into(loan,body,effect,reply),(Some(loan),true)=>self.retirement.recover_step_original_into(loan,body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply)}}
  if recover{return Err(invalid())}
  if self.pending.is_some(){let turn=self.retirement.begin_granted(&mut self.pending,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.map_or_else(Default::default,|step|step.progress()))).map_err(Into::into)))}
  if self.live>0{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native array close exceeded its work authority").into())))}self.live-=1;*self.pending=Some(unsafe{self.rows.as_ptr().cast::<T>().add(self.live).read()});return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.map_or_else(Default::default,|step|step.progress()))).map_err(Into::into)))}
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){crate::RetainedWorkPoll::Pending{..}=>reply.pending(),crate::RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map_err(Into::into))})
 }
 fn alias_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(invalid)?;if source.terminal_is_empty(){Ok(fixed(size_of::<Option<crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>>>()+size_of::<u8>()))}else{source.close_demand(body)}}
 fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{input.get().sequence_child(self.index)?;let index=self.index;Ok(input.project(index,move|v|v.sequence_child(index).expect("validated original array child")))}

}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize>NativeReceive<I>for[T;N]where[T;N]:RetireOwned{type Receiver=NativeArrayReceiver<I,T,N>;fn receiver()->Self::Receiver{NativeArrayReceiver::new()}}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize>NativeReceiver<I,[T;N]>for NativeArrayReceiver<I,T,N>where[T;N]:RetireOwned{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}if self.failed{return Ok(Default::default())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}match self.phase{0|1|2|7|8=>Ok(fixed(0)),4=>self.child.take_demand(),3=>if self.child.ready(){Ok(fixed(0))}else{nested(self.child.demands(self.source(input)?,body,maximum_work_units)?,0)},5=>Ok(Self::place_demand()),6=>if self.child.terminal_is_empty(){Ok(fixed(0))}else{self.child_close_demand(body)},_=>Err(invalid())}}
fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.failed{return Err(invalid())}
  let work=if !input.check_binding(&self.binding)?{input.binding_work_demand(&self.binding)?}else{match self.phase{0|1|2|7|8=>1,3=>if self.child.ready(){1}else{return Ok(NativeAdvanceSelection::Nested)},4=>self.child.take_work_demand()?,5=>6,6=>if self.child.terminal_is_empty(){1}else{self.child_close_work(body)?},_=>return Err(invalid())}};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native array action exceeds its original work ceiling"))}Ok(NativeAdvanceSelection::Local{work})
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{
  if self.closing||self.failed||self.phase!=3||self.child.ready()||self.binding.is_none(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  let source=match self.source(input){Ok(source)=>source,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  self.child.advance(source,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let g=effect.grant();
  macro_rules! perform{($n:expr)=>{if !effect.performed($n)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native array action exceeded its original work").into())}}}
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(g,d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   match self.phase{
    4=>{return match self.child.take(&mut *self.pending,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.pending.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.phase=5;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}},
    5=>{let placed=Self::place_demand();if !retained_clone_grant_admits(g,placed){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}if self.pending.is_none()||self.index!=self.live||self.index>=N{return Err(invalid().into())}perform!(6);let value=self.pending.take().ok_or_else(invalid)?;unsafe{self.rows.as_mut_ptr().cast::<T>().add(self.index).write(value)}self.live+=1;self.index+=1;self.phase=6;return Ok(Some(RetainedCloneStep::Progress(receipt(placed))))},
    6 if !self.child.terminal_is_empty()=>{let turn=if !self.child.is_closing(){semio_framework_value_native_receiver_protocol::native_close_settle(effect,|effect,reply|self.child.begin_close_into(effect,reply))}else{semio_framework_value_native_receiver_protocol::native_close_settle(effect,|effect,reply|self.child.close_into(body,effect,reply))};return match turn{crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)}},
    _=>{}}
   perform!(1);
   match self.phase{0=>self.phase=1,1=>{let length=input.get().sequence_len()?;if length!=N{self.failed=true;return Err(invalid().with_retained_progress(receipt(fixed(0))).into())}self.phase=if N==0{8}else{2};},2=>{input.get().sequence_child(self.index)?;self.phase=3;},3=>{if !self.child.ready(){return Err(invalid().into())}self.phase=4;},6=>self.phase=7,7=>{self.child=Default::default();self.phase=if self.index==N{8}else{2};},8=>{if self.live!=N{return Err(invalid().into())}*self.output=Some(unsafe{self.rows.as_ptr().read()});self.live=0;self.phase=9;},_=>return Err(invalid().into())}
   Ok(Some(if self.output_ready(){RetainedCloneStep::Complete(receipt(fixed(0)))}else{RetainedCloneStep::Progress(receipt(fixed(0)))}))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==9&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output!([T;N],invalid);
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child_close_demand(body)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.live>0{return Ok(fixed(std::mem::size_of::<T>()+std::mem::size_of::<Option<T>>()+std::mem::size_of::<usize>()))}if self.output.is_some(){return self.retirement.next_owner_demands::<[T;N]>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child_close_work(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.live>0{return Ok(1)}if self.output.is_some(){return self.retirement.next_owner_work::<[T;N]>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,true)}
 fn advance_pending(&self)->bool{false}
 fn requires_original_birth(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,true)}

 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.live==0&&self.pending.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize>Drop for NativeArrayReceiver<I,T,N>where[T;N]:RetireOwned{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native array requires funded closure")}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
