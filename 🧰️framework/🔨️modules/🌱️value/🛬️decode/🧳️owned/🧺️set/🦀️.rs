//! 🧺️ Native membership receiving reads the wire string array through funded sequence custody, then admits every member insert through the ordered owner's paid update cursor.
use super::{NativeDefault,NativeReceive,NativeReceiver,tree::{NativeReceiveSlot,NativeValueSource,add_receipt,child_grant,fixed,invalid,nested,receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,OrderedSet,SharedOwner,UpdateCursor};
use std::mem::{ManuallyDrop,size_of};

type Residual=(Box<UpdateCursor<()>>,OrderedMap<()>,Option<SharedOwner<()>>);
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
/// 🧺️ The completed member array is consumed one admitted persistent insert at a time; duplicates collapse exactly as the wire decoder's collection does.
pub struct NativeSetReceiver<I:NativeValueSource+?Sized>{child:NativeReceiveSlot<I,Members>,members:ManuallyDrop<Option<Members>>,map:ManuallyDrop<Option<OrderedMap<()>>>,key:ManuallyDrop<Option<String>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<()>>>,cursor:ManuallyDrop<Option<Box<UpdateCursor<()>>>>,residual:ManuallyDrop<Option<Residual>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for OrderedSet{
 type Receiver=NativeSetReceiver<I>;
 fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>())}
 fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}Ok((NativeSetReceiver{child:Default::default(),members:ManuallyDrop::new(None),map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),retirement:Default::default(),phase:READ,closing:false},receipt(d)))}
}
impl NativeDefault for OrderedSet{
 fn native_default_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let demand=Self::native_default_demand();if !retained_clone_grant_admits(grant,demand){return Err(invalid())}Ok((Self::new(),receipt(demand)))}
}
impl<I:NativeValueSource+?Sized> NativeSetReceiver<I>{
 fn owner_demands<T:RetireOwned>(&self,present:bool,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<T>(present,&None,body)}
 fn owner_step<T:RetireOwned>(retirement:&mut RetainedCloneClose,owner:&mut Option<T>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !retirement.is_empty(){retirement.step_granted(g)}else{Ok(retirement.begin_granted(owner,g)?.unwrap_or(RetainedCloneStep::Complete(Default::default())))}}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,OrderedSet> for NativeSetReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=READY{return Err(invalid())}let scalar=size_of::<usize>()+size_of::<u8>();let tag=size_of::<u8>();Ok(match self.phase{
 READ=>nested(self.child.demands(input,body.saturating_sub(tag))?,tag)?,
 TAKE=>fixed(2*size_of::<Option<Members>>()+tag),
 READ_CLOSE=>nested(self.child.close_demands(body.saturating_sub(tag))?,tag)?,
 READ_RESET=>fixed(2*size_of::<NativeReceiveSlot<I,Members>>()+tag),
 PICK=>fixed(2*size_of::<Option<String>>()+tag),
 SHARE_KEY=>RetirementDemand{capacity_bytes:SharedOwner::<String>::allocation_bytes(),..fixed(2*size_of::<String>()+size_of::<Option<SharedOwner<String>>>()+tag)},
 SHARE_VALUE=>RetirementDemand{capacity_bytes:SharedOwner::<()>::allocation_bytes(),..fixed(size_of::<Option<SharedOwner<()>>>()+tag)},
 CURSOR=>RetirementDemand{capacity_bytes:size_of::<UpdateCursor<()>>(),..fixed(size_of::<UpdateCursor<()>>()+size_of::<Option<SharedOwner<String>>>()+size_of::<Option<SharedOwner<()>>>()+size_of::<Option<Box<UpdateCursor<()>>>>()+tag)},
 INSERT=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;RetirementDemand{copy_bytes:c.next_copy_byte_demand()+tag,capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}},
 SWAP=>fixed(3*size_of::<OrderedMap<()>>()+2*size_of::<Option<Box<UpdateCursor<()>>>>()+size_of::<Residual>()+tag),
 RESIDUAL=>nested(self.owner_demands::<Residual>(self.residual.is_some(),body.saturating_sub(scalar))?,scalar)?,
 RELEASE=>nested(self.owner_demands::<Members>(self.members.is_some(),body.saturating_sub(scalar))?,scalar)?,
 _=>return Err(invalid())})}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;let tag=size_of::<u8>();let scalar=size_of::<usize>()+tag;match self.phase{
 READ=>{let step=self.child.advance(input,control,body.saturating_sub(tag),child_grant(g,tag))?;if self.child.ready(){self.phase=TAKE}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),tag)?))},
 TAKE=>{*self.members=Some(self.child.take().ok_or_else(invalid)?);self.child.begin_close();self.phase=READ_CLOSE},
 READ_CLOSE=>{control.charge(d.capacity_bytes)?;let step=self.child.close_step(child_grant(g,tag))?;if self.child.terminal_is_empty(){self.phase=READ_RESET}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),tag)?))},
 READ_RESET=>{self.child=Default::default();self.phase=PICK},
 PICK=>match self.members.as_mut().ok_or_else(invalid)?.pop(){Some(key)=>{*self.key=Some(key);self.phase=SHARE_KEY},None=>self.phase=RELEASE},
 SHARE_KEY=>{control.charge(d.capacity_bytes)?;let key=self.key.take().ok_or_else(invalid)?;let(shared,_)=SharedOwner::admit(key,g).unwrap_or_else(|_|unreachable!());*self.shared_key=Some(shared);self.phase=SHARE_VALUE},
 SHARE_VALUE=>{control.charge(d.capacity_bytes)?;let(shared,_)=SharedOwner::admit((),g).unwrap_or_else(|_|unreachable!());*self.shared_value=Some(shared);self.phase=CURSOR},
 CURSOR=>{control.charge(d.capacity_bytes)?;*self.cursor=Some(Box::new(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?)));self.phase=INSERT},
 INSERT=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let mut p=c.advance_insert_controlled(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-tag,..g},control)?.progress();if c.is_complete(){self.phase=SWAP}p.copied_bytes+=tag;p.copied_items=p.copied_items.max(1);return Ok(RetainedCloneStep::Progress(p))},
 SWAP=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.phase=RESIDUAL},
 RESIDUAL=>{control.charge(d.capacity_bytes)?;let step=Self::owner_step(&mut self.retirement,&mut self.residual,child_grant(g,scalar))?;if self.residual.is_none()&&self.retirement.is_empty(){self.phase=PICK}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),scalar)?))},
 RELEASE=>{control.charge(d.capacity_bytes)?;let step=Self::owner_step(&mut self.retirement,&mut self.members,child_grant(g,scalar))?;if self.members.is_none()&&self.retirement.is_empty(){self.phase=READY}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),scalar)?))},
 _=>return Err(invalid())}Ok(if self.phase==READY{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})}
 fn output_ready(&self)->bool{!self.closing&&self.phase==READY&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<OrderedSet>>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(OrderedSet,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}Ok(Some((OrderedSet::from_map(self.map.take().ok_or_else(invalid)?),receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.owner_demands::<()>(false,body)}macro_rules! next{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.owner_demands::<$ty>(true,body)}}}next!(residual,Residual);next!(cursor,Box<UpdateCursor<()>>);next!(shared_value,SharedOwner<()>);next!(shared_key,SharedOwner<String>);next!(key,String);next!(members,Members);next!(map,OrderedMap<()>);Ok(Default::default())}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.child.terminal_is_empty(){return self.child.close_step(g)}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}macro_rules! close{($field:ident)=>{if self.$field.is_some(){return Ok(self.retirement.begin_granted(&mut self.$field,g)?.ok_or_else(invalid)?)}}}close!(residual);close!(cursor);close!(shared_value);close!(shared_key);close!(key);close!(members);close!(map);Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{self.map.is_none()&&self.members.is_none()&&self.key.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeSetReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native set receiving requires funded closure")}}
