//! 🗂️ Native object receiving owns each actual immutable map update through publication or cancellation.
use super::{NativeReceive,NativeReceiver,NativeDefault,tree::{NativeValueSource,NativeReceiveSlot,fixed,invalid,receipt,nested,child_grant,add_receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,SharedOwner,UpdateCursor};
use std::mem::{ManuallyDrop,size_of};

type Residual<V>=(Box<UpdateCursor<V>>,OrderedMap<V>,Option<SharedOwner<V>>);
/// 🧺️ One original record supplies keys and child values without an intermediate object conversion.
pub struct NativeMapReceiver<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync>{binding:Option<RetainedCloneBinding>,map:ManuallyDrop<Option<OrderedMap<V>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<V>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<V>>>,cursor:ManuallyDrop<Option<Box<UpdateCursor<V>>>>,residual:ManuallyDrop<Option<Residual<V>>>,child:NativeReceiveSlot<dyn NativeValueSource,V>,retirement:RetainedCloneClose,index:usize,offset:usize,phase:u8,closing:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceive<I> for OrderedMap<V>{
 type Receiver=NativeMapReceiver<I,V>;
 fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>())}
 fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}Ok((NativeMapReceiver{binding:None,map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),child:Default::default(),retirement:Default::default(),index:0,offset:0,phase:0,closing:false,input:std::marker::PhantomData},receipt(d)))}
}
impl<V:RetireOwned+Sync> NativeDefault for OrderedMap<V>{
 fn native_default_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 fn native_default(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::native_default_demand();if !retained_clone_grant_admits(g,d){return Err(invalid())}Ok((Self::new(),receipt(d)))}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeMapReceiver<I,V>{
 fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{input.get().record_child(self.index)?;let index=self.index;Ok(input.project::<dyn NativeValueSource,_>(index,move|v|v.record_child(index).unwrap()))}
 fn fragment(&self,input:&I)->Result<usize,ValueError>{let key=input.record_name(self.index)?;let mut end=(self.offset+64).min(key.len());while !key.is_char_boundary(end){end-=1}Ok(end-self.offset)}
 fn residual_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<Residual<V>>(self.residual.is_some(),&None,body)}
 fn residual_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.retirement.is_empty(){self.retirement.step_granted(g)}else{Ok(self.retirement.begin_granted(&mut self.residual,g)?.unwrap_or(RetainedCloneStep::Complete(Default::default())))}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceiver<I,OrderedMap<V>> for NativeMapReceiver<I,V>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=13{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}let scalar=size_of::<usize>()+size_of::<u8>();Ok(match self.phase{
 0=>fixed(size_of::<u8>()),
 1=>if self.index==input.get().record_len()?{fixed(size_of::<u8>())}else{RetirementDemand{capacity_bytes:input.get().record_name(self.index)?.len(),..fixed(size_of::<Option<String>>()+scalar)}},
 2=>fixed(self.fragment(input.get())?+scalar),
 3=>nested(self.child.demands(self.source(input)?,body.saturating_sub(size_of::<u8>()))?,size_of::<u8>())?,
 4=>fixed(2*size_of::<Option<V>>()+size_of::<u8>()),
 5=>RetirementDemand{capacity_bytes:SharedOwner::<String>::allocation_bytes(),..fixed(2*size_of::<String>()+size_of::<Option<SharedOwner<String>>>()+size_of::<u8>())},
 6=>RetirementDemand{capacity_bytes:SharedOwner::<V>::allocation_bytes(),..fixed(2*size_of::<V>()+size_of::<Option<SharedOwner<V>>>()+size_of::<u8>())},
 7=>RetirementDemand{capacity_bytes:size_of::<UpdateCursor<V>>(),..fixed(size_of::<UpdateCursor<V>>()+size_of::<Option<SharedOwner<String>>>()+size_of::<Option<SharedOwner<V>>>()+size_of::<Option<Box<UpdateCursor<V>>>>()+size_of::<u8>())},
 8=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;RetirementDemand{copy_bytes:c.next_copy_byte_demand()+size_of::<u8>(),capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}},
 9=>fixed(3*size_of::<OrderedMap<V>>()+2*size_of::<Option<Box<UpdateCursor<V>>>>()+size_of::<Residual<V>>()+size_of::<u8>()),
 10=>nested(self.child.close_demands(body.saturating_sub(size_of::<u8>()))?,size_of::<u8>())?,
 11=>fixed(2*size_of::<NativeReceiveSlot<dyn NativeValueSource,V>>()+size_of::<u8>()),
 12=>nested(self.residual_demands(body.saturating_sub(scalar))?,scalar)?,_=>return Err(invalid())})}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;if let Some(p)=input.bind(&mut self.binding,g)?{return Ok(RetainedCloneStep::Progress(p))}match self.phase{
 0=>{input.get().record_len()?;self.phase=1},
 1=>{if self.index==input.get().record_len()?{self.phase=13}else{control.charge(d.capacity_bytes)?;*self.key=Some(String::with_capacity(d.capacity_bytes));self.offset=0;self.phase=2}},
 2=>{let n=self.fragment(input.get())?;self.key.as_mut().ok_or_else(invalid)?.push_str(&input.get().record_name(self.index)?[self.offset..self.offset+n]);self.offset+=n;if self.offset==input.get().record_name(self.index)?.len(){self.phase=3}},
 3=>{let step=self.child.advance(self.source(input)?,control,body.saturating_sub(size_of::<u8>()),child_grant(g,size_of::<u8>()))?;if self.child.ready(){self.phase=4}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),size_of::<u8>())?))},
 4=>{*self.value=Some(self.child.take().ok_or_else(invalid)?);self.phase=5},
 5=>{control.charge(d.capacity_bytes)?;let key=self.key.take().ok_or_else(invalid)?;let(shared,_)=SharedOwner::admit(key,g).unwrap_or_else(|_|unreachable!());*self.shared_key=Some(shared);self.phase=6},
 6=>{control.charge(d.capacity_bytes)?;let value=self.value.take().ok_or_else(invalid)?;let(shared,_)=SharedOwner::admit(value,g).unwrap_or_else(|_|unreachable!());*self.shared_value=Some(shared);self.phase=7},
 7=>{control.charge(d.capacity_bytes)?;*self.cursor=Some(Box::new(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?)));self.phase=8},
 8=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let mut p=c.advance_insert_controlled(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<u8>(),..g},control)?.progress();if c.is_complete(){self.phase=9}p.copied_bytes+=size_of::<u8>();p.copied_items=p.copied_items.max(1);return Ok(RetainedCloneStep::Progress(p))},
 9=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.child.begin_close();self.phase=10},
 10=>{control.charge(d.capacity_bytes)?;let step=self.child.close_step(child_grant(g,size_of::<u8>()))?;if self.child.terminal_is_empty(){self.phase=11}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),size_of::<u8>())?))},
 11=>{self.child=Default::default();self.phase=12},
 12=>{control.charge(d.capacity_bytes)?;let extra=size_of::<usize>()+size_of::<u8>();let step=self.residual_step(child_grant(g,extra))?;if self.residual.is_none()&&self.retirement.is_empty(){self.index+=1;self.phase=1}return Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),extra)?))},_=>return Err(invalid())}Ok(if self.phase==13{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})}
 fn output_ready(&self)->bool{!self.closing&&self.phase==13&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<OrderedMap<V>>>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(OrderedMap<V>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}Ok(Some((self.map.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}macro_rules! next{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}}}next!(residual,Residual<V>);next!(cursor,Box<UpdateCursor<V>>);next!(shared_value,SharedOwner<V>);next!(shared_key,SharedOwner<String>);next!(value,V);next!(key,String);next!(map,OrderedMap<V>);RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.child.terminal_is_empty(){return self.child.close_step(g)}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}macro_rules! close{($field:ident)=>{if self.$field.is_some(){return Ok(self.retirement.begin_granted(&mut self.$field,g)?.ok_or_else(invalid)?)}}}close!(residual);close!(cursor);close!(shared_value);close!(shared_key);close!(value);close!(key);close!(map);RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> Drop for NativeMapReceiver<I,V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native map receiving requires funded closure")}}
#[cfg(test)]#[path="tests.rs"]mod tests;
