//! 🗂️ Native object receiving owns each actual immutable map update through publication or cancellation.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,tree::{NativeValueSource,NativeReceiveSlot,fixed,invalid,receipt,nested,child_grant}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,SharedOwner,UpdateCursor};
use std::mem::ManuallyDrop;
use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,OriginalEffectReply,OriginalEffectReplyComplete,native_relay,native_close_begun,ordered_original_close_bridge,ordered_close_demands,ordered_close_step};

type Residual<V>=(UpdateCursor<V>,OrderedMap<V>,Option<SharedOwner<V>>);
pub struct NativeMapReceiver<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync>{binding:Option<RetainedCloneBinding>,map:ManuallyDrop<Option<OrderedMap<V>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<V>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<V>>>,cursor:ManuallyDrop<Option<UpdateCursor<V>>>,residual:ManuallyDrop<Option<Residual<V>>>,child:NativeReceiveSlot<dyn NativeValueSource,V>,retirement:RetainedCloneClose,key_frame:crate::native_decoding::NativeReceivingEffect,index:usize,offset:usize,phase:u8,closing:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceive<I> for OrderedMap<V>{
 type Receiver=NativeMapReceiver<I,V>;
 
 fn receiver()->Self::Receiver{NativeMapReceiver{binding:None,map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),child:Default::default(),retirement:Default::default(),key_frame:Default::default(),index:0,offset:0,phase:0,closing:false,input:std::marker::PhantomData}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeMapReceiver<I,V>{
 fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{input.get().record_child(self.index)?;let index=self.index;Ok(input.project::<dyn NativeValueSource,_>(index,move|v|v.record_child(index).unwrap()))}
 fn fragment(&self,input:&I)->Result<usize,ValueError>{let key=input.record_name(self.index)?;let mut end=(self.offset+64).min(key.len());while !key.is_char_boundary(end){end-=1}Ok(end-self.offset)}
 pub fn fragment_probe(&self,input:&I)->Result<usize,ValueError>{self.fragment(input)}
 fn relay_close<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if self.phase==10{let child=&mut self.child;return semio_framework_value_native_receiver_protocol::native_relay(reply.progress(),|inner|if !child.is_closing(){if recover{child.recover_begin_close_into(effect,inner)}else{child.begin_close_into(effect,inner)}}else if recover{child.recover_close_into(body,effect,inner)}else{child.close_into(body,effect,inner)})}
  let(retirement,residual)=(&mut self.retirement,&mut self.residual);semio_framework_value_native_receiver_protocol::native_relay(reply.progress(),|inner|Self::residual_step(retirement,residual,body,effect,inner,recover))
 }
 fn residual_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<Residual<V>>(self.residual.is_some(),&None,body)}
 fn residual_step<'r>(retirement:&mut RetainedCloneClose,residual:&mut Option<Residual<V>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !retirement.is_empty(){return if recover{retirement.recover_step_into(body,effect,reply)}else{retirement.step_into(body,effect,reply)}}if recover{return Err(invalid())}semio_framework_value_native_receiver_protocol::native_close_begun(retirement.begin_granted(residual,effect),reply)}

}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceiver<I,OrderedMap<V>> for NativeMapReceiver<I,V>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=13&&self.phase!=14{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}Ok(match self.phase{
 0|1|9|11=>fixed(0),
 4=>self.child.take_demand()?,
 14=>if self.key.is_some(){fixed(0)}else{RetirementDemand{copy_bytes:std::mem::size_of::<String>(),capacity_bytes:input.get().record_name(self.index)?.len(),depth:1,..Default::default()}},
 2=>fixed(self.fragment(input.get())?),
 3=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(self.source(input)?,body,maximum_work_units)?,0)?},
 5=>if self.shared_key.is_some(){fixed(0)}else{SharedOwner::<String>::source_admission_demand()?},
 6=>if self.shared_value.is_some(){fixed(0)}else{SharedOwner::<V>::source_admission_demand()?},
 7=>if self.cursor.is_some(){fixed(0)}else{self.map.as_ref().ok_or_else(invalid)?.update_constructor_demand()},
 8=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{c.native_demands(body)?}},
 10=>if self.child.terminal_is_empty(){fixed(0)}else if self.child.is_closing(){self.child.close_demands(body)?}else{self.child.begin_close_demand()?},
 12=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.residual_demands(body)?},_=>return Err(invalid())})}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.phase>=13&&self.phase!=14{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  let local=|work:u64|Ok(NativeAdvanceSelection::Local{work});
  match self.phase{
   0|1|2|9|11=>local(1),14=>if self.key.is_some(){local(1)}else{Ok(NativeAdvanceSelection::Nested)},3=>if self.child.ready(){local(1)}else{Ok(NativeAdvanceSelection::Nested)},4=>local(self.child.take_work_demand()?),
   5=>local(if self.shared_key.is_some(){1}else{SharedOwner::<String>::source_admission_work_demand()}),6=>local(if self.shared_value.is_some(){1}else{SharedOwner::<V>::source_admission_work_demand()}),
   7=>local(if self.cursor.is_some(){1}else{self.map.as_ref().ok_or_else(invalid)?.update_constructor_work()}),
   8=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;local(if c.is_complete(){1}else{c.native_work_demand(body)?.max(1)})},
   10=>local(if self.child.terminal_is_empty(){1}else if self.child.is_closing(){self.child.close_work_demand(body)?}else{self.child.begin_close_work_demand()?}),
   12=>local(if self.residual.is_none()&&self.retirement.is_empty(){1}else{self.retirement.next_owner_work::<Residual<V>>(self.residual.is_some(),&None,body)?}),_=>Err(invalid())}
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{
  use crate::RetainedWorkPoll::Complete;
  let result:Result<crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>,OriginalReceivingPrimary>=(||{
   if maximum_work_units==0||self.closing||self.binding.is_none(){return Err(invalid().into())}
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}))}
   control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;
   Ok(match self.phase{
    14=>{let turn=control.admit_string(&mut self.key,input.get().record_name(self.index)?.len(),g,&mut self.key_frame);Complete(RetainedWorkOutcome{work:turn.work,result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child;receiving.work_refusal=failure.work_refusal;receiving.into()})})},
    3=>self.child.advance(self.source(input)?,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress())),
    _=>return Err(invalid().into())})
  })();
  match result{Ok(poll)=>poll,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let pre:Result<Option<Option<RetainedCloneStep>>,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(Some(RetainedCloneStep::Progress(Default::default()))))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(Some(RetainedCloneStep::Progress(p.unwrap_or_default())))).map_err(Into::into)}
   let progress=|turn:RetainedWorkOutcome|turn.result.map(|s|Some(Some(RetainedCloneStep::Progress(s.progress())))).map_err(Into::into);
   match self.phase{
    4=>return match self.child.take(&mut *self.value,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(Some(None)),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.value.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.phase=5;Ok(Some(Some(RetainedCloneStep::Progress(step.progress()))))}},
    5 if self.shared_key.is_none()=>return progress(SharedOwner::<String>::admit_from(&mut self.key,&mut self.shared_key,effect)),
    6 if self.shared_value.is_none()=>return progress(SharedOwner::<V>::admit_from(&mut self.value,&mut self.shared_value,effect)),
    7 if self.cursor.is_none()=>{let map=self.map.as_ref().ok_or_else(invalid)?;return progress(map.admit_update_cursor(&mut self.shared_key,&mut self.shared_value,&mut self.cursor,effect))}
    8 if !self.cursor.as_ref().ok_or_else(invalid)?.is_complete()=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;return progress(c.advance_native(body,effect))}
    10 if !self.child.terminal_is_empty()=>return Ok(None),
    12 if self.residual.is_some()||!self.retirement.is_empty()=>return Ok(None),
    _=>{}}
   if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native map action exceeded its original work").into())}
   match self.phase{
    0=>{input.get().record_len()?;self.phase=1},
    1=>self.phase=if self.index==input.get().record_len()?{13}else{14},
    14=>{if self.key.is_none(){return Err(invalid().into())}self.offset=0;self.phase=2},
    2=>{let n=self.fragment(input.get())?;let source=&input.get().record_name(self.index)?[self.offset..self.offset+n];let key=self.key.as_mut().ok_or_else(invalid)?;if key.capacity().saturating_sub(key.len())<n{return Err(invalid().into())}key.push_str(source);self.offset+=n;if self.offset==input.get().record_name(self.index)?.len(){self.phase=3}},
    3=>{if !self.child.ready(){return Err(invalid().into())}self.phase=4},
    5=>self.phase=6,6=>self.phase=7,
    7=>self.phase=8,
    8=>self.phase=9,
    9=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.phase=10},
    10=>self.phase=11,
    11=>{self.child=Default::default();self.phase=12},
    12=>{self.index+=1;self.phase=1},
    _=>return Err(invalid().into())}
   Ok(Some(Some(if self.phase==13{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})))
  })();
  match pre{Ok(Some(Some(step)))=>Ok(reply.complete(Ok(step))),Ok(Some(None))=>Ok(reply.pending()),Err(error)=>Ok(reply.complete(Err(error))),Ok(None)=>self.relay_close(body,effect,reply,false)}
 }
 fn recover_advance_local<'reply>(&mut self,_input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if matches!(self.phase,10|12){return self.relay_close(body,effect,reply,true)}Ok(reply.pending())}
 fn advance_pending(&self)->bool{self.child.advance_pending()}
 fn output_ready(&self)->bool{!self.closing&&self.phase==13&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}semio_framework_value_native_receiver_protocol::native_output_demand::<OrderedMap<V>>(0)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<OrderedMap<V>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{if !self.output_ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}semio_framework_value_native_receiver_protocol::native_output_take(&mut self.map,output,0,effect)})();semio_framework_value_native_receiver_protocol::native_record_pure_take(turn,reply)}
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 ordered_close_demands!(residual:Residual<V>,cursor:UpdateCursor<V>,shared_value:SharedOwner<V>,shared_key:SharedOwner<String>,value:V,key:String,map:OrderedMap<V>);
 semio_framework_value_native_receiver_protocol::map_close_work_quote!();
 ordered_original_close_bridge!();
 ordered_close_step!(residual,cursor,shared_value,shared_key,value,key,map);
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
  fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()&&self.key_frame.phase()==crate::native_decoding::construction::NativeReceivingEffectPhase::Empty}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> Drop for NativeMapReceiver<I,V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native map receiving requires funded closure")}}

