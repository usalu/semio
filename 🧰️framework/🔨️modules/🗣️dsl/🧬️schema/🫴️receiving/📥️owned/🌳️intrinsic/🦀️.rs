//! 🌳️ The exact intrinsic source and native receiver remain in real funded storage through publication and cancellation.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use super::*;
use semio_framework_value::{DslValue,ErasedSnapshotRetirement,retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive};
use semio_framework_value_native_storage::NativeOwnedStorage;
/// 🧬️ The declaration identifies the original intrinsic field without allocating a metadata roster.
pub trait NativeIntrinsicRecord:NativeReceive<DslValue>+Sync{const INTRINSIC_FIELD_ID:u16;}
type Receiver<T>=NativeReceivingOriginal<DslValue,T>;
/// 🎒️ Every source, unfinished native child and completed output remains in its authentic owner.
pub struct NativeIntrinsicReceiving<T:NativeIntrinsicRecord>{record:OwnedSlot<FieldValue>,original:OwnedSlot<DslValue>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:NativeOwnedStorage<Receiver<T>>,retired:ManuallyDrop<Option<Box<Receiver<T>>>>,output:OwnedSlot<T>,shape:DeclaredRecordShape,phase:u8,closing:bool,failed:bool}
impl<T:NativeIntrinsicRecord>Default for NativeIntrinsicReceiving<T>{fn default()->Self{Self{record:Default::default(),original:Default::default(),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiver:NativeOwnedStorage::empty(),retired:ManuallyDrop::new(None),output:Default::default(),shape:Default::default(),phase:0,closing:false,failed:false}}}
fn absent<R>(name:&'static str)->Result<R,ValueError>{Err(ValueError::invariant(name))}
impl<T:NativeIntrinsicRecord>NativeIntrinsicReceiving<T>{
 fn receiver_ref(&self)->Result<&Receiver<T>,ValueError>{self.receiver.get().ok_or_else(||ValueError::invariant("get target absent"))}
 fn receiver_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.retired.is_some(){return Ok(storage::header_release_demand::<Receiver<T>>())}if let Some(child)=self.receiver.get(){return if child.terminal_is_empty(){Ok(NativeOwnedStorage::<Receiver<T>>::box_demand())}else{child.close_demands(body).and_then(nested)}}Ok(self.receiver.release_demand())}
 fn receiver_close_work(&self,body:usize)->Result<u64,ValueError>{if self.retired.is_some(){return Ok(1)}if let Some(child)=self.receiver.get(){return if child.terminal_is_empty(){Ok(self.receiver.box_work_demand())}else{child.close_work_demand(body)}}Ok(self.receiver.release_work_demand())}
 fn close_receiver(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  let g=effect.grant();
  if let Some(header)=self.retired.as_ref(){if !header.terminal_is_empty(){return done(effect,Err(invariant().into()))}return unsafe{storage::release_empty_header(&mut self.retired,effect)}}
  if let Some(child)=self.receiver.get_mut(){if !child.terminal_is_empty(){let poll=child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,physical(poll))}let poll=closing(physical(self.receiver.take_box_into(&mut self.retired,effect)));return settle(effect,poll)}
  let poll=closing(physical(RetainedWorkPoll::Complete(self.receiver.release(effect))));settle(effect,poll)
 }
 fn selected(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.closing||self.failed{return done(effect,Err(invariant().into()))}if self.phase==13{return idle(effect)}
  let d=match self.demands(input,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let g=effect.grant();
  match self.phase{
   0 if !self.shape.is_ready()=>{let Some(FieldValue::Record(record))=input.as_ref()else{return done(effect,Err(invalid().into()))};let result=self.shape.step(record,&[(T::INTRINSIC_FIELD_ID,true)],effect).map_err(Into::into);return done(effect,result)},
   3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>{let turn=match self.admission.as_mut(){Some(admission)=>admission.advance(effect),None=>return done(effect,Err(ValueError::invariant("admission absent").into()))};return settle(effect,normal(physical(RetainedWorkPoll::Complete(turn))))},
   4=>{let result=match self.admission.as_mut(){Some(admission)=>{let turn=admission.take_output(&mut self.source,g);let owed=owe(effect,turn.work);turn.result.and_then(|step|owed.map(|()|step.progress()))},None=>absent("admission absent")};return done(effect,result.map_err(Into::into))},
   6=>{let poll=normal(physical(self.receiver.reserve_empty(effect)));return settle(effect,poll)},
   7=>{let source=&mut self.source;let poll=normal(physical(self.receiver.initialize_seed(effect,||Ok(NativeReceivingOriginal::new(source.take().ok_or_else(||ValueError::invariant("source absent"))?)))));return settle(effect,poll)},
   9 if self.output.value.is_none()=>{let poll=match self.receiver.get_mut(){Some(receiver)=>receiver.take(&mut self.output.value,effect).map(|step|step.progress()),None=>return done(effect,Err(ValueError::invariant("get_mut target absent").into()))};return settle(effect,poll)},
   11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>{let poll=self.close_receiver(effect);return settle(effect,receiving(poll))},
   12 if !self.record.terminal_is_empty()=>{let poll=self.record.close_step(effect);return settle(effect,receiving(poll))},
   _=>{}
  }
  local(effect,d,||{match self.phase{
   0=>{*self.record.value=input.take();self.phase=1;},
   1=>{let Some(FieldValue::Record(record))=self.record.value.as_mut()else{return Err(invariant())};let field=record.fields.get_mut(&T::INTRINSIC_FIELD_ID).ok_or_else(invalid)?;if !matches!(field,FieldValue::Value(_)){return Err(invalid())}let FieldValue::Value(value)=std::mem::replace(field,FieldValue::Absent)else{return Err(invariant())};*self.original.value=Some(value);self.phase=2;},
   2=>{*self.admission=Some(RetainedCloneSource::admission_owned(self.original.value.take().ok_or_else(invariant)?,()));self.phase=3;},
   3=>self.phase=4,
   5=>{if !self.admission.as_ref().is_some_and(|a|a.terminal_is_empty()){return Err(invariant())}self.admission.take();self.phase=6;},
   8=>self.phase=9,
   10=>{self.receiver.get_mut().ok_or_else(invariant)?.begin_close();self.phase=11;},
   11=>self.phase=12,
   12=>self.phase=13,
   _=>return Err(invariant())}Ok(())})
 }
 fn advanced(&mut self,poll:OwnedPoll)->OwnedPoll{
  if let RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(_),..})=&poll{match self.phase{4 if self.source.is_some()=>self.phase=5,6 if self.receiver.reserved()=>self.phase=7,7 if self.receiver.initialized()=>self.phase=8,9 if self.output.value.is_some()=>self.phase=10,_=>{}}}
  poison(&mut self.failed,&poll);poll
 }
}
impl<T:NativeIntrinsicRecord>OwnedFieldReceiving<T>for NativeIntrinsicReceiving<T>{
 fn demands(&self,input:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.failed{return Err(invariant())}match self.phase{
  0=>{if !matches!(input,Some(FieldValue::Record(_))){return Err(invalid())}Ok(metadata::<()>())},
  3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>self.admission.as_ref().ok_or_else(||ValueError::invariant("admission absent"))?.next_demand(),
  4 if self.source.is_none()=>Ok(metadata::<()>()),
  6 if !self.receiver.reserved()=>Ok(NativeOwnedStorage::<Receiver<T>>::reserve_demand()),
  7 if !self.receiver.initialized()=>Ok(NativeOwnedStorage::<Receiver<T>>::seed_demand()),
  8 if !self.receiver_ref()?.output_ready()=>self.receiver_ref()?.demands(body,u64::MAX).and_then(nested),
  9 if self.output.value.is_none()=>self.receiver_ref()?.take_demand().and_then(nested),
  11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>self.receiver_close_demand(body),
  12 if !self.record.terminal_is_empty()=>self.record.close_demands(body),13=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,_:&Option<FieldValue>,body:usize,_:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.closing||self.failed{return Err(invariant())}let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});match self.phase{
  8 if !self.receiver_ref()?.output_ready()=>Ok(OwnedAdvanceSelection::Nested),
  3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>local(self.admission.as_ref().ok_or_else(||ValueError::invariant("admission absent"))?.advance_work_demand()?),
  6 if !self.receiver.reserved()=>local(self.receiver.reserve_work_demand()),7 if !self.receiver.initialized()=>local(self.receiver.seed_work_demand()),
  9 if self.output.value.is_none()=>local(self.receiver_ref()?.take_work_demand()?),
  11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>local(self.receiver_close_work(body)?),
  12 if !self.record.terminal_is_empty()=>local(self.record.close_work_demand(body)?),13=>local(0),_=>local(1)}}
 fn advance_nested(&mut self,_:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{
  if self.closing||self.failed||self.phase!=8{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}
  let poll=match self.receiver.get_mut(){Some(receiver)if !receiver.output_ready()=>physical(receiver.advance(native,body,maximum_work_units,child_grant(g))),_=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})};self.advanced(poll)
 }
 fn advance_local(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(input,body,effect);self.advanced(poll)}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==13&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if OwnedFieldReceiving::terminal_is_empty(self){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.receiver_close_demand(body)}if let Some(a)=self.admission.as_ref(){return if a.terminal_is_empty()||a.phase()=="ready"{Ok(metadata::<()>())}else{a.next_return_demand()}}if let Some(s)=self.source.as_ref(){return if s.terminal_is_empty(){Ok(metadata::<()>())}else{s.next_close_demand(body)}}if !self.original.terminal_is_empty(){return self.original.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.record.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if OwnedFieldReceiving::terminal_is_empty(self){return Ok(0)}if !self.closing{return Ok(1)}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.receiver_close_work(body)}if let Some(a)=self.admission.as_ref(){return if a.terminal_is_empty()||a.phase()=="ready"{Ok(1)}else{a.return_work_demand()}}if let Some(s)=self.source.as_ref(){return if s.terminal_is_empty(){Ok(1)}else{s.next_close_work_demand(body)}}if !self.original.terminal_is_empty(){return self.original.close_work_demand(body)}if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}self.record.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  if OwnedFieldReceiving::terminal_is_empty(self){return idle(effect)}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if !self.closing{return local_close(effect,d,||{self.closing=true;if let Some(child)=self.receiver.get_mut(){child.begin_close()}Ok(())})}
  if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.close_receiver(effect)}
  if let Some(a)=self.admission.as_mut(){
   if a.terminal_is_empty(){return local_close(effect,d,||{self.admission.take();Ok(())})}
   if a.phase()=="ready"{let turn=a.take_output(&mut self.source,g);let owed=owe(effect,turn.work);let result=turn.result.and_then(|step|owed.map(|()|step.progress())).map_err(Into::into);return done(effect,result)}
   let mut original=None;let turn=a.return_inputs(&mut original,effect);if let Some((value,()))=original{*self.original.value=Some(value)}return settle(effect,physical(RetainedWorkPoll::Complete(turn).map_err(Into::into)))
  }
  if let Some(s)=self.source.as_mut(){if s.terminal_is_empty(){return local_close(effect,d,||{self.source.take();Ok(())})}let poll=s.close_step(effect);return settle(effect,physical(poll))}
  if !self.original.terminal_is_empty(){return self.original.close_step(effect)}if !self.output.terminal_is_empty(){return self.output.close_step(effect)}self.record.close_step(effect)
 }
 fn terminal_is_empty(&self)->bool{self.record.terminal_is_empty()&&self.original.terminal_is_empty()&&self.admission.is_none()&&self.source.is_none()&&self.receiver.terminal_is_empty()&&self.retired.is_none()&&self.output.terminal_is_empty()}
}
impl<T:NativeIntrinsicRecord>Drop for NativeIntrinsicReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||OwnedFieldReceiving::terminal_is_empty(self),"intrinsic receiving requires funded closure");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
impl<T:NativeIntrinsicRecord>NativeIntrinsicReceiving<T>{
 /// 🎯️ Selects the unchanged actual close frontier for this original owner.
 pub fn select_close(&mut self,body:usize)->Result<semio_framework_value::RetirementSelection<'_,Self>,ValueError>{semio_framework_value::select_retirement_cursor(self,body)}
}
impl<T:NativeIntrinsicRecord>ErasedSnapshotRetirement for NativeIntrinsicReceiving<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{OwnedFieldReceiving::close_demands(self,body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{OwnedFieldReceiving::close_work_demand(self,body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.depth)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{OwnedFieldReceiving::close_step(self,effect).map(RetainedCloneStep::Progress)}
 fn terminal_is_empty(&self)->bool{OwnedFieldReceiving::terminal_is_empty(self)}
}
