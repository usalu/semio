//! 🌳️ The exact intrinsic source and native receiver remain in real funded storage through publication and cancellation.
use super::*;
use semio_framework_value::{DslValue,retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::NativeReceivingOriginal};
use semio_framework_value_native_storage::NativeOwnedStorage;
/// 🧬️ The declaration identifies the original intrinsic field without allocating a metadata roster.
pub trait NativeIntrinsicRecord:NativeReceive<DslValue>+Sync{const INTRINSIC_FIELD_ID:u16;}
/// 🎒️ Every source, unfinished native child and completed output remains in its authentic owner.
pub struct NativeIntrinsicReceiving<T:NativeIntrinsicRecord>{record:OwnedSlot<FieldValue>,original:OwnedSlot<DslValue>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<DslValue,DslValue>>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:NativeOwnedStorage<NativeReceivingOriginal<DslValue,T>>,retired:ManuallyDrop<Option<Box<NativeReceivingOriginal<DslValue,T>>>>,output:OwnedSlot<T>,shape:DeclaredRecordShape,phase:u8,closing:bool,failed:bool}
impl<T:NativeIntrinsicRecord>Default for NativeIntrinsicReceiving<T>{fn default()->Self{Self{record:Default::default(),original:Default::default(),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiver:NativeOwnedStorage::empty(),retired:ManuallyDrop::new(None),output:Default::default(),shape:Default::default(),phase:0,closing:false,failed:false}}}
impl<T:NativeIntrinsicRecord>NativeIntrinsicReceiving<T>{
 fn receiver_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.retired.is_some(){return Ok(storage::header_release_demand::<NativeReceivingOriginal<DslValue,T>>())}if let Some(child)=self.receiver.get(){return if child.terminal_is_empty(){Ok(NativeOwnedStorage::<NativeReceivingOriginal<DslValue,T>>::move_demand())}else{child.close_demands(body).and_then(nested)}}Ok(self.receiver.release_demand())}
 fn close_receiver(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if let Some(header)=self.retired.as_ref(){if !header.terminal_is_empty(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}return unsafe{storage::release_empty_header(&mut self.retired,g)}}if let Some(child)=self.receiver.get_mut(){if !child.terminal_is_empty(){return physical(child.close_step(child_grant(g)))}let turn=self.receiver.take_box(g);let result=turn.result.map(|out|if let Some((header,p))=out{*self.retired=Some(header);p}else{Default::default()});return RetainedWorkOutcome{result,work:turn.work}}physical(self.receiver.release(g))}
 fn selected(&mut self,input:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if self.closing||self.failed{return RetainedWorkOutcome{result:Err(invariant().into()),work:0}}if self.phase==13{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let d=match self.demands(input,body){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  match self.phase{
   0 if !self.shape.is_ready()=>{let Some(FieldValue::Record(record))=input.as_ref()else{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}};let result=self.shape.step(record,&[(T::INTRINSIC_FIELD_ID,true)],native,g);return RetainedWorkOutcome{result:result.map_err(Into::into),work:self.shape.last_turn_work()}},
   3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>{let turn=self.admission.as_mut().unwrap().advance(g);return physical(native.receive_retained_work(g,turn))},
   4=>{let turn=self.admission.as_mut().unwrap().take_output(&mut self.source,g);return physical(native.receive_retained_work(g,turn))},
   6=>{if let Err(e)=storage::capacity(native,d.capacity_bytes){return RetainedWorkOutcome{result:Err(e.into()),work:0}}let turn=self.receiver.reserve(g);return physical(native.receive_retained_work(g,turn))},
   7=>{if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}let mut header=Some(NativeReceivingOriginal::new(self.source.take().unwrap()));return normal(physical(self.receiver.put_from(&mut header,g)))},
   8 if !self.receiver.get().unwrap().output_ready()=>return physical(self.receiver.get_mut().unwrap().advance(native,body,child_grant(g))),
   9=>{let turn=self.receiver.get_mut().unwrap().take(child_grant(g));let result=turn.result.map(|out|if let Some((value,p))=out{*self.output.value=Some(value);p}else{Default::default()});return RetainedWorkOutcome{result:result.map_err(Into::into),work:turn.work}},
   11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>return post_child_close(self.close_receiver(g),native,g),
   12 if !self.record.terminal_is_empty()=>return close_source(&mut self.record,native,g),_=>{}
  }
  if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}
  let result=(||{match self.phase{
   0=>{*self.record.value=input.take();self.phase=1;},
   1=>{let Some(FieldValue::Record(record))=self.record.value.as_mut()else{return Err(invariant())};let field=record.fields.get_mut(&T::INTRINSIC_FIELD_ID).ok_or_else(invalid)?;if !matches!(field,FieldValue::Value(_)){return Err(invalid())}let FieldValue::Value(value)=std::mem::replace(field,FieldValue::Absent)else{unreachable!()};*self.original.value=Some(value);self.phase=2;},
   2=>{*self.admission=Some(RetainedCloneSource::admission_owned(self.original.value.take().ok_or_else(invariant)?,()));self.phase=3;},
   3=>self.phase=4,
   4=>return Err(invariant()),
   5=>{if !self.admission.as_ref().is_some_and(|a|a.terminal_is_empty()){return Err(invariant())}self.admission.take();self.phase=6;},
   6=>return Err(invariant()),
   7=>return Err(invariant()),
   8=>self.phase=9,
   9=>return Err(invariant()),
   10=>{self.receiver.get_mut().ok_or_else(invariant)?.begin_close();self.phase=11;},
   11=>self.phase=12,
   12=>self.phase=13,_=>return Err(invariant())}Ok(receipt(d))})();RetainedWorkOutcome{result:result.map_err(|e|OriginalReceivingPrimary::from(e.with_retained_progress(receipt(metadata::<()>())))),work:1}
 }
}
impl<T:NativeIntrinsicRecord>OwnedFieldReceiving<T>for NativeIntrinsicReceiving<T>{
 fn demands(&self,input:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.failed{return Err(invariant())}match self.phase{
  0=>{if !matches!(input,Some(FieldValue::Record(_))){return Err(invalid())}Ok(metadata::<()>())},
  3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>self.admission.as_ref().unwrap().next_demand(),
  4 if self.source.is_none()=>Ok(metadata::<()>()),
  6 if !self.receiver.reserved()=>Ok(NativeOwnedStorage::<NativeReceivingOriginal<DslValue,T>>::reserve_demand()),
  8 if !self.receiver.get().ok_or_else(invariant)?.output_ready()=>self.receiver.get().unwrap().demands(body).and_then(nested),
  9 if self.output.value.is_none()=>self.receiver.get().ok_or_else(invariant)?.take_demand().and_then(nested),
  11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>self.receiver_close_demand(body),
  12 if !self.record.terminal_is_empty()=>self.record.close_demands(body),13=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn step(&mut self,input:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.selected(input,native,body,g);if turn.result.is_ok(){match self.phase{4 if self.source.is_some()=>self.phase=5,6 if self.receiver.reserved()=>self.phase=7,7 if self.receiver.initialized()=>self.phase=8,9 if self.output.value.is_some()=>self.phase=10,_=>{}}}else if turn.work>0{self.failed=true}turn}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==13&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if !self.is_ready(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}move_into(&mut self.output.value,output,g)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.receiver_close_demand(body)}if let Some(a)=self.admission.as_ref(){return if a.terminal_is_empty()||a.phase()=="ready"{Ok(metadata::<()>())}else{a.next_return_demand()}}if let Some(s)=self.source.as_ref(){return if s.terminal_is_empty(){Ok(metadata::<()>())}else{s.next_close_demand(body)}}if !self.original.terminal_is_empty(){return self.original.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.record.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !self.closing{self.closing=true;if let Some(child)=self.receiver.get_mut(){child.begin_close()}return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.close_receiver(g)}if let Some(a)=self.admission.as_mut(){if a.terminal_is_empty(){self.admission.take();return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}let turn=if a.phase()=="ready"{a.take_output(&mut self.source,g)}else{let mut original=None;let turn=a.return_inputs(&mut original,g);if let Some((value,()))=original{*self.original.value=Some(value)}turn};return physical(turn)}if let Some(s)=self.source.as_mut(){if s.terminal_is_empty(){self.source.take();return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}return physical(s.close_step(g))}if !self.original.terminal_is_empty(){return self.original.close_step(g)}if !self.output.terminal_is_empty(){return self.output.close_step(g)}self.record.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.record.terminal_is_empty()&&self.original.terminal_is_empty()&&self.admission.is_none()&&self.source.is_none()&&self.receiver.terminal_is_empty()&&self.retired.is_none()&&self.output.terminal_is_empty()}
}
impl<T:NativeIntrinsicRecord>Drop for NativeIntrinsicReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"intrinsic receiving requires funded closure");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
impl<T:NativeIntrinsicRecord>NativeIntrinsicReceiving<T>{
 /// 🎯️ Selects the unchanged actual close frontier for this original owner.
 pub fn select_close(&mut self,body:usize)->Result<semio_framework_value::RetirementSelection<'_,Self>,ValueError>{semio_framework_value::select_retirement_cursor(self,body)}
}
impl<T:NativeIntrinsicRecord>semio_framework_value::ErasedSnapshotRetirement for NativeIntrinsicReceiving<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{OwnedFieldReceiving::close_demands(self,body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.depth)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{let turn=OwnedFieldReceiving::close_step(self,g);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
 fn terminal_is_empty(&self)->bool{OwnedFieldReceiving::terminal_is_empty(self)}
}
