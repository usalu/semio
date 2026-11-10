//! 📋️ Vector backing, one original field, one child and one new retained row advance on separate funded turns.
use super::*;
pub struct VectorReceiving<T:DslOwnedReceivingField>{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,child_output:OwnedSlot<T>,output:OwnedSlot<Vec<T>>,child:OwnedReceivingSlot<T>,index:usize,length:usize,phase:u8,closing:bool,failed:bool}
impl<T:DslOwnedReceivingField>Default for VectorReceiving<T>{fn default()->Self{Self{source:Default::default(),current:Default::default(),child_output:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),index:0,length:0,phase:0,closing:false,failed:false}}}
impl<T:DslOwnedReceivingField>DslOwnedReceivingField for Vec<T>{type Cursor=VectorReceiving<T>;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
impl<T:DslOwnedReceivingField>VectorReceiving<T>{
 fn selected(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.failed||self.closing{return done(effect,Err(invariant().into()))}if self.phase==8{return idle(effect)}
  let d=match self.demands(source,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let g=effect.grant();
  match self.phase{
   1 if self.output.value.is_none()=>{
    let result=perform(effect).and_then(|()|{let vector=allocate_empty_vec::<T>(self.length,effect.actual())?;let capacity=vector.capacity()*std::mem::size_of::<T>();*self.output.value=Some(vector);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Vec<T>>(),retained_capacity_bytes:capacity,..Default::default()})}).map_err(|error|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>()))));
    return done(effect,result)
   },
   3 if !self.child.is_ready()=>{let poll=self.child.advance_local(&mut self.current.value,body,&mut effect.nested(child_grant(g)));return settle(effect,poll)},
   4 if self.child_output.value.is_none()=>{let poll=self.child.take_into(&mut self.child_output.value,&mut effect.nested(child_grant(g)));return settle(effect,normal(poll))},
   5 if !self.child.terminal_is_empty()=>{let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,receiving(poll))},
   7 if !self.source.terminal_is_empty()=>{let poll=self.source.close_step(effect);return settle(effect,receiving(poll))},
   _=>{}
  }
  local(effect,d,||{match self.phase{
   0=>{let value=source.as_ref().ok_or_else(invalid)?;self.length=match value{FieldValue::List(v)|FieldValue::Tuple(v)=>v.len(),FieldValue::Bytes64(v)=>v.len(),_=>return Err(invalid())};*self.source.value=source.take();self.phase=1;},
   1=>self.phase=2,
   2=>{if self.index==self.length{self.phase=7;}else{if !self.current.terminal_is_empty(){return Err(invariant())}let value=match self.source.value.as_mut().ok_or_else(invariant)?{FieldValue::List(v)|FieldValue::Tuple(v)=>std::mem::replace(v.get_mut(self.index).ok_or_else(invariant)?,FieldValue::Absent),FieldValue::Bytes64(v)=>FieldValue::UInt(u64::from(*v.retained_bytes().get(self.index).ok_or_else(invariant)?)),_=>return Err(invariant())};*self.current.value=Some(value);self.phase=3;}},
   3=>self.phase=4,
   4=>self.phase=5,
   5=>self.phase=6,
   6=>{let output=self.output.value.as_mut().ok_or_else(invariant)?;if output.len()==output.capacity()||self.child_output.value.is_none(){return Err(invariant())}output.push(self.child_output.value.take().ok_or_else(||ValueError::invariant("value absent"))?);self.index+=1;self.child=OwnedReceivingSlot::new();self.phase=2;},
   7=>self.phase=8,
   _=>return Err(invariant())}Ok(())})
 }
}
impl<T:DslOwnedReceivingField>OwnedFieldReceiving<Vec<T>>for VectorReceiving<T>{
 fn demands(&self,_:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{0=>Ok(metadata::<()>()),1 if self.output.value.is_none()=>Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Vec<T>>(),capacity_bytes:self.length.checked_mul(std::mem::size_of::<T>()).ok_or_else(invariant)?,depth:1,..Default::default()}),2 if self.index<self.length&&matches!(self.source.value.as_ref(),Some(FieldValue::Bytes64(_)))=>Ok(payload::<u64>()),3 if !self.child.is_ready()=>self.child.demands(&self.current.value,body).and_then(nested),4 if self.child_output.value.is_none()=>nested(nested(metadata::<T>())?),5 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),6=>Ok(payload::<T>()),7 if !self.source.terminal_is_empty()=>self.source.close_demands(body),8=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,_:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.failed||self.closing{return Err(invariant())}let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});match self.phase{3 if !self.child.is_ready()=>self.child.select_advance(&self.current.value,body,maximum_work_units),4 if self.child_output.value.is_none()=>local(self.child.take_work_demand()?),5 if !self.child.terminal_is_empty()=>local(self.child.close_work_demand(body)?),7 if !self.source.terminal_is_empty()=>local(self.source.close_work_demand(body)?),8=>local(0),_=>local(1)}}
 fn advance_nested(&mut self,_:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{if self.failed||self.closing||self.phase!=3||self.child.is_ready(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}let poll=self.child.advance_nested(&mut self.current.value,native,witness,body,maximum_work_units,child_grant(g));poison(&mut self.failed,&poll);poll}
 fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(source,body,effect);poison(&mut self.failed,&poll);poll}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==8&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<Vec<T>>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested)}if !self.current.terminal_is_empty(){return self.current.close_demands(body)}if !self.child_output.terminal_is_empty(){return self.child_output.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.closing{return Ok(1)}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.current.terminal_is_empty(){return self.current.close_work_demand(body)}if !self.child_output.terminal_is_empty(){return self.child_output.close_work_demand(body)}if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}self.source.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{if self.terminal_is_empty(){return idle(effect)}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())})}if !self.child.terminal_is_empty(){let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,poll)}if !self.current.terminal_is_empty(){return self.current.close_step(effect)}if !self.child_output.terminal_is_empty(){return self.child_output.close_step(effect)}if !self.output.terminal_is_empty(){return self.output.close_step(effect)}self.source.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.current.terminal_is_empty()&&self.child_output.terminal_is_empty()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
