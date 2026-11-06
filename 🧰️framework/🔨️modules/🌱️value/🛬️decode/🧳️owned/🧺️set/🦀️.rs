//! 🧺️ Native membership receiving reads the wire string array through funded sequence custody, then admits every member insert through the ordered owner's paid update cursor.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeDefault,NativeReceive,NativeReceiver,tree::{NativeReceiveSlot,NativeValueSource,add_receipt,child_grant,fixed,invalid,nested,receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,OrderedSet,SharedOwner,UpdateCursor};
use std::mem::{ManuallyDrop,size_of};

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
pub struct NativeSetReceiver<I:NativeValueSource+?Sized>{child:NativeReceiveSlot<I,Members>,members:ManuallyDrop<Option<Members>>,map:ManuallyDrop<Option<OrderedMap<()>>>,key:ManuallyDrop<Option<String>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<()>>>,cursor:ManuallyDrop<Option<UpdateCursor<()>>>,residual:ManuallyDrop<Option<Residual>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for OrderedSet{
 type Receiver=NativeSetReceiver<I>;
 
 fn receiver()->Self::Receiver{NativeSetReceiver{child:Default::default(),members:ManuallyDrop::new(None),map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),retirement:Default::default(),phase:READ,closing:false}}
}
impl<I:NativeValueSource+?Sized> NativeSetReceiver<I>{
 fn owner_demands<T:RetireOwned>(&self,present:bool,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<T>(present,&None,body)}
 fn owner_step<T:RetireOwned>(retirement:&mut RetainedCloneClose,owner:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome{if !retirement.is_empty(){retirement.step_granted(g)}else{let turn=retirement.begin_granted(owner,g);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}

}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,OrderedSet> for NativeSetReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=READY{return Err(invalid())}Ok(match self.phase{
 READ=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(input,body)?,0)?},
 TAKE|READ_RESET|PICK|CURSOR|SWAP=>fixed(0),
 READ_CLOSE=>if self.child.terminal_is_empty(){fixed(0)}else{self.child.close_demands(body)?},
 SHARE_KEY=>if self.shared_key.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:SharedOwner::<String>::allocation_bytes(),..fixed(0)}},
 SHARE_VALUE=>if self.shared_value.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:SharedOwner::<()>::allocation_bytes(),..fixed(0)}},
 INSERT=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{RetirementDemand{copy_bytes:c.next_copy_byte_demand(),capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}}},
 RESIDUAL=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Residual>(self.residual.is_some(),body)?},
 RELEASE=>if self.members.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.owner_demands::<Members>(self.members.is_some(),body)?},_=>return Err(invalid())})}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut work=0;let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;match self.phase{
 READ=>{if self.child.ready(){work=1;self.phase=TAKE}else{let turn=self.child.advance(input,control,body,child_grant(g,0));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 TAKE=>{let turn=self.child.take(g);work=turn.work;let Some((members,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.members=Some(members);self.child.begin_close();self.phase=READ_CLOSE;return Ok(RetainedCloneStep::Progress(p))},
 READ_CLOSE=>{if self.child.terminal_is_empty(){work=1;self.phase=READ_RESET}else{let turn=self.child.close_step(g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 READ_RESET=>{self.child=Default::default();work=1;self.phase=PICK},
 PICK=>{let next=self.members.as_mut().ok_or_else(invalid)?.pop();work=1;match next{Some(key)=>{*self.key=Some(key);self.phase=SHARE_KEY},None=>self.phase=RELEASE}},
 SHARE_KEY=>{if self.shared_key.is_some(){work=1;self.phase=SHARE_VALUE}else{let key=self.key.take().ok_or_else(invalid)?;let turn=SharedOwner::admit(key,g);let result=match turn.result{Ok((value,p))=>{*self.shared_key=Some(value);Ok(RetainedCloneStep::Progress(p))},Err((error,value))=>{*self.key=Some(value);Err(error)}};let turn=control.receive_retained_work(g,RetainedWorkOutcome{result,work:turn.work});work=turn.work;return turn.result}},
 SHARE_VALUE=>{if self.shared_value.is_some(){work=1;self.phase=CURSOR}else{let turn=SharedOwner::admit((),g);let result=match turn.result{Ok((value,p))=>{*self.shared_value=Some(value);Ok(RetainedCloneStep::Progress(p))},Err((error,()))=>Err(error)};let turn=control.receive_retained_work(g,RetainedWorkOutcome{result,work:turn.work});work=turn.work;return turn.result}},
 CURSOR=>{*self.cursor=Some(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?));work=1;self.phase=INSERT},
 INSERT=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;if c.is_complete(){work=1;self.phase=SWAP}else{let turn=c.advance_insert_controlled(g,control);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}},
 SWAP=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));work=1;self.phase=RESIDUAL},
 RESIDUAL=>{if self.residual.is_none()&&self.retirement.is_empty(){work=1;self.phase=PICK}else{let turn=Self::owner_step(&mut self.retirement,&mut self.residual,g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 RELEASE=>{if self.members.is_none()&&self.retirement.is_empty(){work=1;self.phase=READY}else{let turn=Self::owner_step(&mut self.retirement,&mut self.members,g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},_=>return Err(invalid().into())}Ok(if self.phase==READY{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})})();RetainedWorkOutcome{result,work}}
 fn output_ready(&self)->bool{!self.closing&&self.phase==READY&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(0))}
 fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(OrderedSet,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}work=1;Ok(Some((OrderedSet::from_map(self.map.take().ok_or_else(invalid)?),receipt(d))))})();RetainedWorkOutcome{result,work}}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.owner_demands::<()>(false,body)}macro_rules! next{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.owner_demands::<$ty>(true,body)}}}next!(residual,Residual);next!(cursor,UpdateCursor<()>);next!(shared_value,SharedOwner<()>);next!(shared_key,SharedOwner<String>);next!(key,String);next!(members,Members);next!(map,OrderedMap<()>);Ok(Default::default())}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.child.terminal_is_empty(){let turn=self.child.close_step(g);work=turn.work;return turn.result}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);work=turn.work;return turn.result}macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,g);work=turn.work;return turn.result.and_then(|step|step.ok_or_else(invalid))}}}close!(residual);close!(cursor);close!(shared_value);close!(shared_key);close!(key);close!(members);close!(map);Ok(RetainedCloneStep::Complete(Default::default()))})();RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.map.is_none()&&self.members.is_none()&&self.key.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeSetReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native set receiving requires funded closure")}}

