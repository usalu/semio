//! 🧺️ Native membership receiving reads the wire string array through funded sequence custody, then admits every member insert through the ordered owner's paid update cursor.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,tree::{NativeReceiveSlot,NativeValueSource,child_grant,fixed,invalid,nested,receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,OrderedSet,SharedOwner,UpdateCursor};
use std::mem::ManuallyDrop;
use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,OriginalEffectReply,OriginalEffectReplyComplete,native_relay,native_close_begun,ordered_original_close_bridge,ordered_close_demands,ordered_close_step};

type Residual=(UpdateCursor<()>,OrderedMap<()>,Option<SharedOwner<()>>);
type Members=Vec<String>;
const READ:u8=0;
const TAKE:u8=1;
const READ_CLOSE:u8=2;
const READ_RESET:u8=3;
const PICK:u8=4;
const SHARE_KEY:u8=5;
const SHARE_VALUE:u8=6;
const CURSOR:u8=7;
const INSERT:u8=8;
const SWAP:u8=9;
const RESIDUAL:u8=10;
const RELEASE:u8=11;
const READY:u8=12;
pub struct NativeSetReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,child:NativeReceiveSlot<I,Members>,members:ManuallyDrop<Option<Members>>,map:ManuallyDrop<Option<OrderedMap<()>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<()>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<()>>>,cursor:ManuallyDrop<Option<UpdateCursor<()>>>,residual:ManuallyDrop<Option<Residual>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for OrderedSet{
 type Receiver=NativeSetReceiver<I>;
 
 fn receiver()->Self::Receiver{NativeSetReceiver{binding:None,child:Default::default(),members:ManuallyDrop::new(None),map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),retirement:Default::default(),phase:READ,closing:false}}
}
impl<I:NativeValueSource+?Sized> NativeSetReceiver<I>{
 fn owner_demands<T:RetireOwned>(&self,present:bool,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<T>(present,&None,body)}
 fn owner_close<'r,T:RetireOwned>(retirement:&mut RetainedCloneClose,owner:&mut Option<T>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !retirement.is_empty(){return if recover{retirement.recover_step_into(body,effect,reply)}else{retirement.step_into(body,effect,reply)}}if recover{return Err(invalid())}semio_framework_value_native_receiver_protocol::native_close_begun(retirement.begin_granted(owner,effect),reply)}
 fn relay_close<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  match self.phase{
   READ_CLOSE=>{let child=&mut self.child;semio_framework_value_native_receiver_protocol::native_relay(reply.progress(),|inner|if !child.is_closing(){if recover{child.recover_begin_close_into(effect,inner)}else{child.begin_close_into(effect,inner)}}else if recover{child.recover_close_into(body,effect,inner)}else{child.close_into(body,effect,inner)})},
   RESIDUAL=>{let(retirement,residual)=(&mut self.retirement,&mut self.residual);semio_framework_value_native_receiver_protocol::native_relay(reply.progress(),|inner|Self::owner_close(retirement,residual,body,effect,inner,recover))},
   _=>{let(retirement,members)=(&mut self.retirement,&mut self.members);semio_framework_value_native_receiver_protocol::native_relay(reply.progress(),|inner|Self::owner_close(retirement,members,body,effect,inner,recover))}}
 }

}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,OrderedSet> for NativeSetReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=READY{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}Ok(match self.phase{
 READ=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(input,body,maximum_work_units)?,0)?},
 TAKE=>self.child.take_demand()?,
 READ_RESET|PICK|SWAP=>fixed(0),
 READ_CLOSE=>if self.child.terminal_is_empty(){fixed(0)}else if self.child.is_closing(){self.child.close_demands(body)?}else{self.child.begin_close_demand()?},
 SHARE_KEY=>if self.shared_key.is_some(){fixed(0)}else{SharedOwner::<String>::source_admission_demand()?},
 SHARE_VALUE=>if self.shared_value.is_some(){fixed(0)}else{SharedOwner::<()>::source_admission_demand()?},
 CURSOR=>if self.cursor.is_some(){fixed(0)}else{self.map.as_ref().ok_or_else(invalid)?.update_constructor_demand()},
 INSERT=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{c.native_demands(body)?}},
 RESIDUAL=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Residual>(self.residual.is_some(),body)?},
 RELEASE=>if self.members.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Members>(self.members.is_some(),body)?},_=>return Err(invalid())})}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.phase>=READY{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  let local=|work:u64|Ok(NativeAdvanceSelection::Local{work});
  match self.phase{
   READ=>if self.child.ready(){local(1)}else{Ok(NativeAdvanceSelection::Nested)},TAKE=>local(self.child.take_work_demand()?),
   READ_CLOSE=>local(if self.child.terminal_is_empty(){1}else if self.child.is_closing(){self.child.close_work_demand(body)?}else{self.child.begin_close_work_demand()?}),
   READ_RESET|PICK|SWAP=>local(1),SHARE_KEY=>local(if self.shared_key.is_some(){1}else{SharedOwner::<String>::source_admission_work_demand()}),SHARE_VALUE=>local(if self.shared_value.is_some(){1}else{SharedOwner::<()>::source_admission_work_demand()}),
   CURSOR=>local(if self.cursor.is_some(){1}else{self.map.as_ref().ok_or_else(invalid)?.update_constructor_work()}),
   INSERT=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;local(if c.is_complete(){1}else{c.native_work_demand(body)?.max(1)})},
   RESIDUAL=>local(if self.residual.is_none()&&self.retirement.is_empty(){1}else{self.retirement.next_owner_work::<Residual>(self.residual.is_some(),&None,body)?}),
   RELEASE=>local(if self.members.is_none()&&self.retirement.is_empty(){1}else{self.retirement.next_owner_work::<Members>(self.members.is_some(),&None,body)?}),_=>Err(invalid())}
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{
  use crate::RetainedWorkPoll::Complete;
  if maximum_work_units==0||self.closing||self.binding.is_none()||self.phase!=READ||self.child.ready(){return Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  let admitted=self.demands(input,body,maximum_work_units).map(|d|retained_clone_grant_admits(g,d));match admitted{Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0}),Ok(false)=>return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}),Ok(true)=>{}}
  self.child.advance(input,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let pre:Result<Option<Option<RetainedCloneStep>>,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(Some(RetainedCloneStep::Progress(Default::default()))))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(Some(RetainedCloneStep::Progress(p.unwrap_or_default())))).map_err(Into::into)}
   let progress=|turn:RetainedWorkOutcome|turn.result.map(|s|Some(Some(RetainedCloneStep::Progress(s.progress())))).map_err(Into::into);
   match self.phase{
    TAKE=>return match self.child.take(&mut *self.members,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(Some(None)),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.members.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.phase=READ_CLOSE;Ok(Some(Some(RetainedCloneStep::Progress(step.progress()))))}},
    READ_CLOSE if !self.child.terminal_is_empty()=>return Ok(None),
    SHARE_KEY if self.shared_key.is_none()=>return progress(SharedOwner::<String>::admit_from(&mut self.key,&mut self.shared_key,effect)),
    SHARE_VALUE if self.shared_value.is_none()=>return progress(SharedOwner::<()>::admit_from(&mut self.value,&mut self.shared_value,effect)),
    CURSOR if self.cursor.is_none()=>{let map=self.map.as_ref().ok_or_else(invalid)?;return progress(map.admit_update_cursor(&mut self.shared_key,&mut self.shared_value,&mut self.cursor,effect))}
    INSERT if !self.cursor.as_ref().ok_or_else(invalid)?.is_complete()=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;return progress(c.advance_native(body,effect))}
    RESIDUAL if self.residual.is_some()||!self.retirement.is_empty()=>return Ok(None),
    RELEASE if self.members.is_some()||!self.retirement.is_empty()=>return Ok(None),
    _=>{}}
   if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native set action exceeded its original work").into())}
   match self.phase{
    READ=>{if !self.child.ready(){return Err(invalid().into())}self.phase=TAKE},
    READ_CLOSE=>self.phase=READ_RESET,
    READ_RESET=>{self.child=Default::default();self.phase=PICK},
    PICK=>{let next=self.members.as_mut().ok_or_else(invalid)?.pop();match next{Some(key)=>{*self.key=Some(key);*self.value=Some(());self.phase=SHARE_KEY},None=>self.phase=RELEASE}},
    SHARE_KEY=>self.phase=SHARE_VALUE,SHARE_VALUE=>self.phase=CURSOR,
    CURSOR=>self.phase=INSERT,
    INSERT=>self.phase=SWAP,
    SWAP=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.phase=RESIDUAL},
    RESIDUAL=>self.phase=PICK,RELEASE=>self.phase=READY,_=>return Err(invalid().into())}
   Ok(Some(Some(if self.phase==READY{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})))
  })();
  match pre{Ok(Some(Some(step)))=>Ok(reply.complete(Ok(step))),Ok(Some(None))=>Ok(reply.pending()),Err(error)=>Ok(reply.complete(Err(error))),Ok(None)=>self.relay_close(body,effect,reply,false)}
 }
 fn recover_advance_local<'reply>(&mut self,_input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if matches!(self.phase,READ_CLOSE|RESIDUAL|RELEASE){return self.relay_close(body,effect,reply,true)}Ok(reply.pending())}
 fn advance_pending(&self)->bool{self.child.advance_pending()}
 fn output_ready(&self)->bool{!self.closing&&self.phase==READY&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}let bytes=std::mem::size_of::<Option<OrderedMap<()>>>().checked_add(std::mem::size_of::<Option<OrderedSet>>()).ok_or_else(invalid)?;Ok(fixed(bytes))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<OrderedSet>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let demand=match self.take_demand(){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}if !retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=match self.map.take(){Some(value)=>value,None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})};*output=Some(OrderedSet::from_map(value));crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(demand))),work:1})
 })();semio_framework_value_native_receiver_protocol::native_record_pure_take(turn,reply)}
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 ordered_close_demands!(residual:Residual,cursor:UpdateCursor<()>,shared_value:SharedOwner<()>,shared_key:SharedOwner<String>,value:(),key:String,members:Members,map:OrderedMap<()>);
 semio_framework_value_native_receiver_protocol::set_close_work_quote!();
 ordered_original_close_bridge!();
 ordered_close_step!(residual,cursor,shared_value,shared_key,value,key,members,map);
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
  fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.members.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeSetReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native set receiving requires funded closure")}}

