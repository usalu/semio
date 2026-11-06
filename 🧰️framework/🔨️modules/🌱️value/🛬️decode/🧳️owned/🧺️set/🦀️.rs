//! 🧺️ Native membership receiving reads the wire string array through funded sequence custody, then admits every member insert through the ordered owner's paid update cursor.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,tree::{NativeReceiveSlot,NativeValueSource,child_grant,fixed,invalid,nested,receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,OrderedSet,SharedOwner,UpdateCursor};
use std::mem::ManuallyDrop;
use semio_framework_value_native_receiver_protocol::{NativeReceivingChild,ordered_original_close_bridge,ordered_shared_admission,ordered_failure_accessors,ordered_cleanup_guard,ordered_close_demands,ordered_close_step};

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
pub struct NativeSetReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,child:NativeReceiveSlot<I,Members>,members:ManuallyDrop<Option<Members>>,map:ManuallyDrop<Option<OrderedMap<()>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<()>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<()>>>,cursor:ManuallyDrop<Option<UpdateCursor<()>>>,residual:ManuallyDrop<Option<Residual>>,retirement:RetainedCloneClose,ordered:NativeReceivingChild,cleanup:NativeReceivingChild<crate::OriginalRetirementFailure>,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for OrderedSet{
 type Receiver=NativeSetReceiver<I>;
 
 fn receiver()->Self::Receiver{NativeSetReceiver{binding:None,child:Default::default(),members:ManuallyDrop::new(None),map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),retirement:Default::default(),ordered:Default::default(),cleanup:Default::default(),phase:READ,closing:false}}
}
impl<I:NativeValueSource+?Sized> NativeSetReceiver<I>{
 ordered_failure_accessors!();
 fn owner_demands<T:RetireOwned>(&self,present:bool,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<T>(present,&None,body)}
 fn owner_step<T:RetireOwned>(retirement:&mut RetainedCloneClose,owner:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{if !retirement.is_empty(){retirement.step_granted(effect)}else{let turn=retirement.begin_granted(owner,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}}

}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,OrderedSet> for NativeSetReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=READY{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}if let Some(d)=self.ordered.demand.or(self.cleanup.demand){return Ok(d)}Ok(match self.phase{
 READ=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(input,body,maximum_work_units)?,0)?},
 TAKE|READ_RESET|PICK|CURSOR|SWAP=>fixed(0),
 READ_CLOSE=>if self.child.terminal_is_empty(){fixed(0)}else{self.child.close_demands(body)?},
 SHARE_KEY=>if self.shared_key.is_some(){fixed(0)}else{SharedOwner::<String>::admission_demand(&self.key,&self.shared_key)?},
 SHARE_VALUE=>if self.shared_value.is_some(){fixed(0)}else{SharedOwner::<()>::admission_demand(&self.value,&self.shared_value)?},
 INSERT=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{RetirementDemand{copy_bytes:c.next_copy_byte_demand(),capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}}},
 RESIDUAL=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Residual>(self.residual.is_some(),body)?},
 RELEASE=>if self.members.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Members>(self.members.is_some(),body)?},_=>return Err(invalid())})}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut work=0;let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.ordered.demand.is_none()&&self.cleanup.demand.is_none(){control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;}if !input.check_binding(&self.binding)?||self.ordered.tag==1{self.ordered.tag=1;let units=match self.ordered.work{Some(work)=>work,None=>input.binding_work_demand(&self.binding)?};let turn=self.ordered.poll(control,d,g,units,maximum_work_units,|_,effect|{let turn=input.bind(&mut self.binding,effect);RetainedWorkOutcome{result:turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())),work:turn.work}});work=turn.work;return turn.result}match self.phase{
 READ=>{if self.child.ready(){work=1;self.phase=TAKE}else{let turn=self.child.advance(input,control,body,maximum_work_units,child_grant(g,0));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 TAKE=>{let turn=self.child.take(g);work=turn.work;let Some((members,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.members=Some(members);self.child.begin_close();self.phase=READ_CLOSE;return Ok(RetainedCloneStep::Progress(p))},
 READ_CLOSE=>{if self.child.terminal_is_empty()&&self.cleanup.demand.is_none(){work=1;self.phase=READ_RESET}else{ordered_cleanup_guard!(self,child);let units=match self.cleanup.work{Some(units)=>units,None=>self.child.close_work_demand(body)?};let turn=self.cleanup.poll(control,d,g,units,maximum_work_units,|_,effect|self.child.close_step(effect));work=turn.work;return turn.result}},
 READ_RESET=>{self.child=Default::default();work=1;self.phase=PICK},
 PICK=>{let next=self.members.as_mut().ok_or_else(invalid)?.pop();work=1;match next{Some(key)=>{*self.key=Some(key);*self.value=Some(());self.phase=SHARE_KEY},None=>self.phase=RELEASE}},
 SHARE_KEY=>{if self.shared_key.is_some()&&self.ordered.demand.is_none(){work=1;self.phase=SHARE_VALUE}else{ordered_shared_admission!(self,control,d,g,work,maximum_work_units,key,shared_key)}},
 SHARE_VALUE=>{if self.shared_value.is_some()&&self.ordered.demand.is_none(){work=1;self.phase=CURSOR}else{ordered_shared_admission!(self,control,d,g,work,value,shared_value)}},
 CURSOR=>{*self.cursor=Some(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?));work=1;self.phase=INSERT},
 INSERT=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;if c.is_complete()&&self.ordered.demand.is_none(){work=1;self.phase=SWAP}else{let turn=self.ordered.poll(control,d,g,1,maximum_work_units,|_,effect|c.advance(effect));work=turn.work;return turn.result}},
 SWAP=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));work=1;self.phase=RESIDUAL},
 RESIDUAL=>{if self.residual.is_none()&&self.retirement.is_empty()&&self.cleanup.demand.is_none(){work=1;self.phase=PICK}else{ordered_cleanup_guard!(self,retirement);let units=match self.cleanup.work{Some(units)=>units,None=>self.retirement.next_owner_work::<Residual>(self.residual.is_some(),&None,body)?};let turn=self.cleanup.poll(control,d,g,units,maximum_work_units,|_,effect|Self::owner_step(&mut self.retirement,&mut self.residual,effect));work=turn.work;return turn.result}},
 RELEASE=>{if self.members.is_none()&&self.retirement.is_empty()&&self.cleanup.demand.is_none(){work=1;self.phase=READY}else{ordered_cleanup_guard!(self,retirement);let units=match self.cleanup.work{Some(units)=>units,None=>self.retirement.next_owner_work::<Members>(self.members.is_some(),&None,body)?};let turn=self.cleanup.poll(control,d,g,units,maximum_work_units,|_,effect|Self::owner_step(&mut self.retirement,&mut self.members,effect));work=turn.work;return turn.result}},_=>return Err(invalid().into())}Ok(if self.phase==READY{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})})();RetainedWorkOutcome{result,work}}
 fn output_ready(&self)->bool{!self.closing&&self.phase==READY&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}let bytes=std::mem::size_of::<Option<OrderedMap<()>>>().checked_add(std::mem::size_of::<Option<OrderedSet>>()).ok_or_else(invalid)?;Ok(fixed(bytes))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<OrderedSet>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let demand=match self.take_demand(){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}if !retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=match self.map.take(){Some(value)=>value,None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})};*output=Some(OrderedSet::from_map(value));crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(demand))),work:1})
 })();semio_framework_value_native_receiver_protocol::native_record_pure_take(turn,reply)}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 ordered_close_demands!(residual:Residual,cursor:UpdateCursor<()>,shared_value:SharedOwner<()>,shared_key:SharedOwner<String>,value:(),key:String,members:Members,map:OrderedMap<()>);
 semio_framework_value_native_receiver_protocol::set_close_work_quote!();
 ordered_original_close_bridge!();
 ordered_close_step!(residual,cursor,shared_value,shared_key,value,key,members,map);
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.members.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()&&self.ordered.terminal_is_empty()&&self.cleanup.terminal_is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeSetReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native set receiving requires funded closure")}}

