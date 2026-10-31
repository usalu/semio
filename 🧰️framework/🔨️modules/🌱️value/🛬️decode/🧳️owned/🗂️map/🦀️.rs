//! 🗂️ Native object receiving owns each actual immutable map update through publication or cancellation.
use super::{NativeReceive,NativeReceiver,NativeDefault,tree::{NativeValueSource,NativeReceiveSlot,fixed,invalid,receipt,nested,child_grant,add_receipt}};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_ordered::{OrderedMap,SharedOwner,UpdateCursor};
use std::mem::{ManuallyDrop,size_of};

type Residual<V>=(UpdateCursor<V>,OrderedMap<V>,Option<SharedOwner<V>>);
/// 🧺️ One original record supplies keys and child values without an intermediate object conversion.
pub struct NativeMapReceiver<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync>{binding:Option<RetainedCloneBinding>,map:ManuallyDrop<Option<OrderedMap<V>>>,key:ManuallyDrop<Option<String>>,value:ManuallyDrop<Option<V>>,shared_key:ManuallyDrop<Option<SharedOwner<String>>>,shared_value:ManuallyDrop<Option<SharedOwner<V>>>,cursor:ManuallyDrop<Option<UpdateCursor<V>>>,residual:ManuallyDrop<Option<Residual<V>>>,child:NativeReceiveSlot<dyn NativeValueSource,V>,retirement:RetainedCloneClose,index:usize,offset:usize,phase:u8,closing:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceive<I> for OrderedMap<V>{
 type Receiver=NativeMapReceiver<I,V>;
 
 fn receiver()->Self::Receiver{NativeMapReceiver{binding:None,map:ManuallyDrop::new(Some(OrderedMap::new())),key:ManuallyDrop::new(None),value:ManuallyDrop::new(None),shared_key:ManuallyDrop::new(None),shared_value:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),residual:ManuallyDrop::new(None),child:Default::default(),retirement:Default::default(),index:0,offset:0,phase:0,closing:false,input:std::marker::PhantomData}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeMapReceiver<I,V>{
 fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{input.get().record_child(self.index)?;let index=self.index;Ok(input.project::<dyn NativeValueSource,_>(index,move|v|v.record_child(index).unwrap()))}
 fn fragment(&self,input:&I)->Result<usize,ValueError>{let key=input.record_name(self.index)?;let mut end=(self.offset+64).min(key.len());while !key.is_char_boundary(end){end-=1}Ok(end-self.offset)}
 fn residual_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<Residual<V>>(self.residual.is_some(),&None,body)}
 fn residual_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{if !self.retirement.is_empty(){self.retirement.step_granted(g)}else{let turn=self.retirement.begin_granted(&mut self.residual,g);crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}

}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> NativeReceiver<I,OrderedMap<V>> for NativeMapReceiver<I,V>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=13&&self.phase!=14{return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}Ok(match self.phase{
 0|1|4|7|9|11=>fixed(0),
 14=>if self.key.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:input.get().record_name(self.index)?.len(),..fixed(0)}},
 2=>fixed(self.fragment(input.get())?),
 3=>if self.child.ready(){fixed(0)}else{nested(self.child.demands(self.source(input)?,body)?,0)?},
 5=>if self.shared_key.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:SharedOwner::<String>::allocation_bytes(),..fixed(0)}},
 6=>if self.shared_value.is_some(){fixed(0)}else{RetirementDemand{capacity_bytes:SharedOwner::<V>::allocation_bytes(),..fixed(0)}},
 8=>{let c=self.cursor.as_ref().ok_or_else(invalid)?;if c.is_complete(){fixed(0)}else{RetirementDemand{copy_bytes:c.next_copy_byte_demand(),capacity_bytes:c.next_capacity_byte_demand()?,depth:c.next_depth_demand().max(1),release_bytes:0}}},
 10=>if self.child.terminal_is_empty(){fixed(0)}else{self.child.close_demands(body)?},
 12=>if self.residual.is_none()&&self.retirement.is_empty(){fixed(0)}else{self.residual_demands(body)?},_=>return Err(invalid())})}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalReceivingPrimary>{let mut work=0;let result:Result<RetainedCloneStep,crate::OriginalReceivingPrimary>=(||{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;if !input.check_binding(&self.binding)?{let turn=input.bind(&mut self.binding,g);let turn=control.receive_retained_work(g,crate::RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work});work=turn.work;return turn.result.map_err(Into::into)}match self.phase{
 0=>{work=1;input.get().record_len()?;self.phase=1},
 1=>{work=1;self.phase=if self.index==input.get().record_len()?{13}else{14}},
 14=>{if self.key.is_none(){let turn=control.admit_string(&mut self.key,input.get().record_name(self.index)?.len(),g);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}work=1;self.offset=0;self.phase=2},
 2=>{let n=self.fragment(input.get())?;let source=&input.get().record_name(self.index)?[self.offset..self.offset+n];let key=self.key.as_mut().ok_or_else(invalid)?;if key.capacity().saturating_sub(key.len())<n{return Err(invalid().into())}key.push_str(source);work=1;self.offset+=n;if self.offset==input.get().record_name(self.index)?.len(){self.phase=3}},
 3=>{if self.child.ready(){work=1;self.phase=4}else{let turn=self.child.advance(self.source(input)?,control,body,child_grant(g,0));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 4=>{let turn=self.child.take(g);work=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.value=Some(value);self.phase=5;return Ok(RetainedCloneStep::Progress(p))},
 5=>{if self.shared_key.is_some(){work=1;self.phase=6}else{let key=self.key.take().ok_or_else(invalid)?;let turn=SharedOwner::admit(key,g);let result=match turn.result{Ok((value,p))=>{*self.shared_key=Some(value);Ok(RetainedCloneStep::Progress(p))},Err((error,value))=>{*self.key=Some(value);Err(error)}};let turn=control.receive_retained_work(g,crate::RetainedWorkOutcome{result,work:turn.work});work=turn.work;return turn.result}},
 6=>{if self.shared_value.is_some(){work=1;self.phase=7}else{let value=self.value.take().ok_or_else(invalid)?;let turn=SharedOwner::admit(value,g);let result=match turn.result{Ok((value,p))=>{*self.shared_value=Some(value);Ok(RetainedCloneStep::Progress(p))},Err((error,value))=>{*self.value=Some(value);Err(error)}};let turn=control.receive_retained_work(g,crate::RetainedWorkOutcome{result,work:turn.work});work=turn.work;return turn.result}},
 7=>{*self.cursor=Some(self.map.as_ref().ok_or_else(invalid)?.begin_set_shared(self.shared_key.take().ok_or_else(invalid)?,self.shared_value.take().ok_or_else(invalid)?));work=1;self.phase=8},
 8=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;if c.is_complete(){work=1;self.phase=9}else{let turn=c.advance_insert_controlled(g,control);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}},
 9=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;let next=c.take_result().ok_or_else(invalid)?;let removed=c.take_removed();c.begin_close();let previous=self.map.replace(next).ok_or_else(invalid)?;*self.residual=Some((self.cursor.take().ok_or_else(invalid)?,previous,removed));self.child.begin_close();work=1;self.phase=10},
 10=>{if self.child.terminal_is_empty(){work=1;self.phase=11}else{let turn=self.child.close_step(g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},
 11=>{self.child=Default::default();work=1;self.phase=12},
 12=>{if self.residual.is_none()&&self.retirement.is_empty(){work=1;self.index+=1;self.phase=1}else{let turn=self.residual_step(g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}},_=>return Err(invalid().into())}Ok(if self.phase==13{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})})();crate::RetainedWorkOutcome{result,work}}
 fn output_ready(&self)->bool{!self.closing&&self.phase==13&&self.map.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(0))}
 fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(OrderedMap<V>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}work=1;Ok(Some((self.map.take().ok_or_else(invalid)?,receipt(d))))})();crate::RetainedWorkOutcome{result,work}}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(c)=self.cursor.as_mut(){c.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}macro_rules! next{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}}}next!(residual,Residual<V>);next!(cursor,UpdateCursor<V>);next!(shared_value,SharedOwner<V>);next!(shared_key,SharedOwner<String>);next!(value,V);next!(key,String);next!(map,OrderedMap<V>);RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.child.terminal_is_empty(){let turn=self.child.close_step(g);work=turn.work;return turn.result}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);work=turn.work;return turn.result}macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,g);work=turn.work;return turn.result.and_then(|step|step.ok_or_else(invalid))}}}close!(residual);close!(cursor);close!(shared_value);close!(shared_key);close!(value);close!(key);close!(map);{let turn=RetainedCloneBinding::close_one(&mut self.binding,g);work=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.map.is_none()&&self.key.is_none()&&self.value.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.cursor.is_none()&&self.residual.is_none()&&self.child.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync> Drop for NativeMapReceiver<I,V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native map receiving requires funded closure")}}


#[cfg(test)]#[path="tests.rs"]mod tests;
