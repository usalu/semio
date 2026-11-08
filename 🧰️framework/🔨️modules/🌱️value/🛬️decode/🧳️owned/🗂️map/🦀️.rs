//! 🗂️ Native object receiving owns each actual immutable map update through publication or cancellation.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,tree::{NativeValueSource,NativeReceiveSlot,fixed,invalid,receipt,nested,child_grant}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,SharedOwner,UpdateCursor};
use std::mem::ManuallyDrop;
use crate::native_decoding::OriginalEffectAuthority;
use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeLocalAuthority,NativeLocalProducer,OriginalEffectReply,OriginalEffectReplyComplete,native_poll_local,native_grant_step,NativeReceivingChild,ordered_original_close_bridge,ordered_shared_admission,ordered_failure_accessors,ordered_cleanup_guard,ordered_close_demands,ordered_close_step};

type Residual<V>=(UpdateCursor<V>,OrderedMap<V>,Option<SharedOwner<V>>);
pub struct NativeMapReceiver<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync>{binding:Option<RetainedCloneBinding>,map:ManuallyDrop<Option<OrderedMap<V>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<V>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<V>>>,cursor:ManuallyDrop<Option<UpdateCursor<V>>>,residual:ManuallyDrop<Option<Residual<V>>>,child:NativeReceiveSlot<dyn NativeValueSource,V>,retirement:RetainedCloneClose,ordered:NativeReceivingChild,cleanup:NativeReceivingChild<crate::OriginalRetirementFailure>,ordered_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,cleanup_authority:ManuallyDrop<Option<NativeLocalAuthority<crate::OriginalRetirementFailure>>>,key_frame:crate::native_decoding::NativeReceivingEffect,index:usize,offset:usize,phase:u8,closing:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceive<I> for OrderedMap<V>{
 type Receiver=NativeMapReceiver<I,V>;
 
 fn receiver()->Self::Receiver{NativeMapReceiver{binding:None,map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),child:Default::default(),retirement:Default::default(),ordered:Default::default(),cleanup:Default::default(),ordered_authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeLocalProducer::new()))),cleanup_authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeLocalProducer::new()))),key_frame:Default::default(),index:0,offset:0,phase:0,closing:false,input:std::marker::PhantomData}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeMapReceiver<I,V>{
 ordered_failure_accessors!();
 fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{input.get().record_child(self.index)?;let index=self.index;Ok(input.project::<dyn NativeValueSource,_>(index,move|v|v.record_child(index).unwrap()))}
 fn fragment(&self,input:&I)->Result<usize,ValueError>{let key=input.record_name(self.index)?;let mut end=(self.offset+64).min(key.len());while !key.is_char_boundary(end){end-=1}Ok(end-self.offset)}
 pub fn fragment_probe(&self,input:&I)->Result<usize,ValueError>{self.fragment(input)}
 fn residual_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<Residual<V>>(self.residual.is_some(),&None,body)}
 fn residual_step(retirement:&mut RetainedCloneClose,residual:&mut Option<Residual<V>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if !retirement.is_empty(){retirement.step_granted(effect).map(|s|RetainedCloneStep::Progress(s.progress()))}else{let turn=retirement.begin_granted(residual,effect);crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}}

}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceiver<I,OrderedMap<V>> for NativeMapReceiver<I,V>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=13&&self.phase!=14{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}if let Some(d)=self.ordered.demand.or(self.cleanup.demand){return Ok(d)}Ok(match self.phase{
 0|1|4|7|9|11=>fixed(0),
 14=>if self.key.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:input.get().record_name(self.index)?.len(),..fixed(0)}},
 2=>fixed(self.fragment(input.get())?),
 3=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(self.source(input)?,body,maximum_work_units)?,0)?},
 5=>if self.shared_key.is_some(){fixed(0)}else{SharedOwner::<String>::admission_demand(&self.key,&self.shared_key)?},
 6=>if self.shared_value.is_some(){fixed(0)}else{SharedOwner::<V>::admission_demand(&self.value,&self.shared_value)?},
 8=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{RetirementDemand{copy_bytes:c.next_copy_byte_demand(),capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}}},
 10=>if self.child.terminal_is_empty(){fixed(0)}else{self.child.close_demands(body)?},
 12=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.residual_demands(body)?},_=>return Err(invalid())})}
fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.phase>=13&&self.phase!=14{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  if self.ordered.demand.is_some()||self.cleanup.demand.is_some(){return Ok(NativeAdvanceSelection::Nested)}
  let local=|ready:bool|if ready{Ok(NativeAdvanceSelection::Local{work:1})}else{Ok(NativeAdvanceSelection::Nested)};
  match self.phase{
   0|1|2|7|9|11=>local(true),14=>local(self.key.is_some()),3=>local(self.child.ready()),4=>Ok(NativeAdvanceSelection::Local{work:self.child.take_work_demand()?}),
   5=>local(self.shared_key.is_some()),6=>local(self.shared_value.is_some()),8=>local(self.cursor.as_ref().ok_or_else(invalid)?.is_complete()),
   10=>local(self.child.terminal_is_empty()),12=>local(self.residual.is_none()&&self.retirement.is_empty()),_=>Err(invalid())}
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  use crate::RetainedWorkPoll::Complete;
  let result:Result<crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,OriginalReceivingPrimary>=(||{
   if maximum_work_units==0||self.closing||self.binding.is_none(){return Err(invalid().into())}
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}))}
   if self.ordered.demand.is_none()&&self.cleanup.demand.is_none(){control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;}
   Ok(match self.phase{
    14=>{let turn=control.admit_string(&mut self.key,input.get().record_name(self.index)?.len(),g,&mut self.key_frame);Complete(RetainedWorkOutcome{work:turn.work,result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child;receiving.work_refusal=failure.work_refusal;receiving})})},
    3=>self.child.advance(self.source(input)?,control,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress())),
    5=>ordered_shared_admission!(self,control,d,g,maximum_work_units,key,shared_key),
    6=>ordered_shared_admission!(self,control,d,g,maximum_work_units,value,shared_value),
    8=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let authority=self.ordered_authority.as_mut().ok_or_else(invalid)?;native_poll_local(&mut self.ordered,control,d,g,1,maximum_work_units,authority,&mut |effect|native_grant_step(effect,|g,m|c.advance(g,m)))},
    10=>{ordered_cleanup_guard!(self,child);let units=match self.cleanup.work{Some(units)=>units,None=>self.child.close_work_demand(body)?};let authority=self.cleanup_authority.as_mut().ok_or_else(invalid)?;native_poll_local(&mut self.cleanup,control,d,g,units,maximum_work_units,authority,&mut |effect|self.child.close_step(effect))},
    12=>{ordered_cleanup_guard!(self,retirement);let units=match self.cleanup.work{Some(units)=>units,None=>self.retirement.next_owner_work::<Residual<V>>(self.residual.is_some(),&None,body)?};let authority=self.cleanup_authority.as_mut().ok_or_else(invalid)?;native_poll_local(&mut self.cleanup,control,d,g,units,maximum_work_units,authority,&mut |effect|Self::residual_step(&mut self.retirement,&mut self.residual,effect))},
    _=>return Err(invalid().into())})
  })();
  match result{Ok(poll)=>poll,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work:0})}
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   if self.phase==4{return match self.child.take(&mut *self.value,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.value.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.phase=5;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}}
   if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native map action exceeded its original work").into())}
   match self.phase{
    0=>{input.get().record_len()?;self.phase=1},
    1=>self.phase=if self.index==input.get().record_len()?{13}else{14},
    14=>{if self.key.is_none(){return Err(invalid().into())}self.offset=0;self.phase=2},
    2=>{let n=self.fragment(input.get())?;let source=&input.get().record_name(self.index)?[self.offset..self.offset+n];let key=self.key.as_mut().ok_or_else(invalid)?;if key.capacity().saturating_sub(key.len())<n{return Err(invalid().into())}key.push_str(source);self.offset+=n;if self.offset==input.get().record_name(self.index)?.len(){self.phase=3}},
    3=>{if !self.child.ready(){return Err(invalid().into())}self.phase=4},
    5=>self.phase=6,6=>self.phase=7,
    7=>{*self.cursor=Some(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?));self.phase=8},
    8=>self.phase=9,
    9=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.child.begin_close();self.phase=10},
    10=>self.phase=11,
    11=>{self.child=Default::default();self.phase=12},
    12=>{self.index+=1;self.phase=1},
    _=>return Err(invalid().into())}
   Ok(Some(if self.phase==13{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))}))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==13&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}semio_framework_value_native_receiver_protocol::native_output_demand::<OrderedMap<V>>(0)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<OrderedMap<V>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{if !self.output_ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}semio_framework_value_native_receiver_protocol::native_output_take(&mut self.map,output,0,effect)})();semio_framework_value_native_receiver_protocol::native_record_pure_take(turn,reply)}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 ordered_close_demands!(residual:Residual<V>,cursor:UpdateCursor<V>,shared_value:SharedOwner<V>,shared_key:SharedOwner<String>,value:V,key:String,map:OrderedMap<V>,ordered_authority:NativeLocalAuthority<ValueError>,cleanup_authority:NativeLocalAuthority<crate::OriginalRetirementFailure>);
 semio_framework_value_native_receiver_protocol::map_close_work_quote!();
 ordered_original_close_bridge!();
 ordered_close_step!(residual,cursor,shared_value,shared_key,value,key,map,ordered_authority,cleanup_authority);
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()&&self.ordered_authority.is_none()&&self.cleanup_authority.is_none()&&self.key_frame.phase()==crate::native_decoding::construction::NativeReceivingEffectPhase::Empty&&self.ordered.terminal_is_empty()&&self.cleanup.terminal_is_empty()}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> Drop for NativeMapReceiver<I,V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native map receiving requires funded closure")}}

