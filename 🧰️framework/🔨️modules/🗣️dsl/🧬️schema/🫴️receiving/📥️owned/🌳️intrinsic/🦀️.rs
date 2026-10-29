//! 🌳️ A declared intrinsic Record transfers its actual Value into the original native receiver continuation.
use super::*;
use semio_framework_value::{DslValue,retained_clone::{RetainedCloneSource}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::NativeReceivingOriginal};
use std::mem::size_of;
/// 🧬️ The record owner explicitly declares the one intrinsic field passed to its native typed receiver.
pub trait NativeIntrinsicRecord:NativeReceive<DslValue>+Sync{const INTRINSIC_FIELD_ID:u16;}
/// 🧳️ Record, native source, receiver and output remain original owned slots through every denied turn.
pub struct NativeIntrinsicReceiving<T:NativeIntrinsicRecord>{record:OwnedSlot<FieldValue>,original:OwnedSlot<DslValue>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:Option<Box<NativeReceivingOriginal<DslValue,T>>>,output:OwnedSlot<T>,shape:DeclaredRecordShape,phase:u8,closing:bool}
impl<T:NativeIntrinsicRecord> Default for NativeIntrinsicReceiving<T>{fn default()->Self{Self{record:Default::default(),original:Default::default(),source:ManuallyDrop::new(None),receiver:None,output:Default::default(),shape:Default::default(),phase:0,closing:false}}}
fn extra<T>()->usize{2*size_of::<Option<T>>()+1}
fn add(mut d:RetirementDemand,copy:usize,capacity:usize,release:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invariant)?;d.capacity_bytes=d.capacity_bytes.checked_add(capacity).ok_or_else(invariant)?;d.release_bytes=d.release_bytes.checked_add(release).ok_or_else(invariant)?;Ok(d)}
fn progress(mut p:RetainedCloneProgress,copy:usize,capacity:usize,release:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(copy).ok_or_else(invariant)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(capacity).ok_or_else(invariant)?;p.released_bytes=p.released_bytes.checked_add(release).ok_or_else(invariant)?;Ok(p)}
fn remaining(g:RetainedCloneGrant,copy:usize,capacity:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_capacity_bytes:g.maximum_capacity_bytes-capacity,..g}}
impl<T:NativeIntrinsicRecord> OwnedFieldReceiving<T> for NativeIntrinsicReceiving<T>{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invariant());}match self.phase{
  0=>{let Some(FieldValue::Record(_))=source.as_ref()else{return Err(invalid());};if !self.shape.is_ready(){Ok(metadata::<()>())}else{Ok(metadata::<Option<FieldValue>>())}},
  1=>Ok(metadata::<FieldValue>()),
  2=>{let d=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();Ok(RetirementDemand{copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes()+1,capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})},
  3=>add(NativeReceivingOriginal::<DslValue,T>::shell_demand(),extra::<Box<NativeReceivingOriginal<DslValue,T>>>(),size_of::<NativeReceivingOriginal<DslValue,T>>(),0),
  4=>add(self.receiver.as_ref().ok_or_else(invariant)?.original_capture_demand()?,1,0,0),
  5=>add(self.receiver.as_ref().ok_or_else(invariant)?.receiver_birth_demand()?,1,0,0),
  6=>add(self.receiver.as_ref().ok_or_else(invariant)?.demands(body.saturating_sub(1))?,1,0,0),
  7=>add(self.receiver.as_ref().ok_or_else(invariant)?.take_demand()?,1,0,0),
  8=>if self.receiver.as_ref().ok_or_else(invariant)?.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:extra::<Box<NativeReceivingOriginal<DslValue,T>>>(),release_bytes:size_of::<NativeReceivingOriginal<DslValue,T>>(),depth:1,..Default::default()})}else{add(self.receiver.as_ref().ok_or_else(invariant)?.close_demands(body.saturating_sub(1))?,1,0,0)},
  9=>if self.record.terminal_is_empty(){Ok(metadata::<u8>())}else{add(self.record.close_demands(body.saturating_sub(1))?,1,0,0)},
  10=>Ok(Default::default()),_=>Err(invariant()),
 }}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.demands(source,body)?;if !funds(g,d){return Ok(Default::default());}native.checkpoint()?;if self.phase!=6{if d.capacity_bytes!=0{native.charge(d.capacity_bytes)?;}}let mut p=receipt(d);match self.phase{
  0=>{let Some(FieldValue::Record(record))=source.as_ref()else{return Err(invalid());};if !self.shape.is_ready(){return self.shape.step(record,&[(T::INTRINSIC_FIELD_ID,true)],native,g);}*self.record.value=source.take();self.phase=1;},
  1=>{let Some(FieldValue::Record(record))=self.record.value.as_mut()else{return Err(invariant());};let field=record.fields.get_mut(&T::INTRINSIC_FIELD_ID).ok_or_else(invalid)?;if !matches!(field,FieldValue::Value(_)){return Err(invalid());}let FieldValue::Value(value)=std::mem::replace(field,FieldValue::Absent)else{unreachable!()};*self.original.value=Some(value);self.phase=2;},
  2=>{let value=self.original.value.take().ok_or_else(invariant)?;match RetainedCloneSource::admit_owned(value,(),remaining(g,1,0)){Ok((value,owned))=>{*self.source=Some(value);p=progress(owned,1,0,0)?;self.phase=3;},Err((e,value,()))=>{*self.original.value=Some(value);return Err(e);}}},
  3=>{let copy=extra::<Box<NativeReceivingOriginal<DslValue,T>>>();let capacity=size_of::<NativeReceivingOriginal<DslValue,T>>();let(value,owned)=NativeReceivingOriginal::begin_shell(remaining(g,copy,capacity))?;self.receiver=Some(Box::new(value));p=progress(owned,copy,capacity,0)?;self.phase=4;},
  4=>{p=progress(self.receiver.as_mut().ok_or_else(invariant)?.admit_original(&mut self.source,remaining(g,1,0))?,1,0,0)?;self.phase=5;},
  5=>{p=progress(self.receiver.as_mut().ok_or_else(invariant)?.begin_receiver(remaining(g,1,0))?,1,0,0)?;self.phase=6;},
  6=>{let receiver=self.receiver.as_mut().ok_or_else(invariant)?;p=progress(receiver.advance(native,body.saturating_sub(1),remaining(g,1,0))?.progress(),1,0,0)?;if receiver.output_ready(){self.phase=7;}},
  7=>{let receiver=self.receiver.as_mut().ok_or_else(invariant)?;let(value,owned)=receiver.take(remaining(g,1,0))?.ok_or_else(invariant)?;*self.output.value=Some(value);receiver.begin_close();self.phase=8;p=progress(owned,1,0,0)?;},
  8=>{if self.receiver.as_ref().ok_or_else(invariant)?.terminal_is_empty(){drop(self.receiver.take());self.phase=9;}else{p=progress(self.receiver.as_mut().ok_or_else(invariant)?.close_step(remaining(g,1,0))?.progress(),1,0,0)?;}},
  9=>{if self.record.terminal_is_empty(){self.phase=10;}else{p=progress(self.record.close_step(remaining(g,1,0))?,1,0,0)?;}},
  10=>return Ok(Default::default()),_=>return Err(invariant()),
 }semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original native intrinsic Record receiving")?;Ok(p)}
 fn is_ready(&self)->bool{!self.closing&&self.phase==10&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.is_ready(){return Err(invariant());}let d=metadata::<Option<T>>();if !funds(g,d){return Ok(Default::default());}*output=self.output.value.take();self.phase=11;Ok(receipt(d))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing&&!self.terminal_is_empty(){return Ok(metadata::<u8>());}if let Some(receiver)=self.receiver.as_ref(){if receiver.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:extra::<Box<NativeReceivingOriginal<DslValue,T>>>(),release_bytes:size_of::<NativeReceivingOriginal<DslValue,T>>(),depth:1,..Default::default()});}return receiver.close_demands(body);}if let Some(source)=self.source.as_ref(){if source.terminal_is_empty(){return Ok(metadata::<Option<RetainedCloneSource<DslValue>>>());}return source.next_close_demand(body);}if !self.original.terminal_is_empty(){return self.original.close_demands(body);}if !self.output.terminal_is_empty(){return self.output.close_demands(body);}self.record.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if self.terminal_is_empty()||!funds(g,d){return Ok(Default::default());}if !self.closing{self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close();}return Ok(receipt(d));}if let Some(receiver)=self.receiver.as_mut(){if receiver.terminal_is_empty(){drop(self.receiver.take());return Ok(receipt(d));}receiver.begin_close();return receiver.close_step(g).map(|s|s.progress());}if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){drop(self.source.take());return Ok(receipt(d));}return source.close_step(g).map(|s|s.progress());}if !self.original.terminal_is_empty(){return self.original.close_step(g);}if !self.output.terminal_is_empty(){return self.output.close_step(g);}self.record.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.record.terminal_is_empty()&&self.original.terminal_is_empty()&&self.source.is_none()&&self.receiver.is_none()&&self.output.terminal_is_empty()}
}
impl<T:NativeIntrinsicRecord> Drop for NativeIntrinsicReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native intrinsic Record requires funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

impl<T:NativeIntrinsicRecord> NativeIntrinsicReceiving<T>{
 /// 🎯️ Loans this original close frontier with its unchanged body policy and complete physical quote.
 pub fn select_close(&mut self,body:usize)->Result<semio_framework_value::RetirementSelection<'_,Self>,ValueError>{semio_framework_value::select_retirement_cursor(self,body)}
}
impl<T:NativeIntrinsicRecord> semio_framework_value::ErasedSnapshotRetirement for NativeIntrinsicReceiving<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{OwnedFieldReceiving::close_demands(self,body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.depth)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{OwnedFieldReceiving::close_step(self,g).map(RetainedCloneStep::Progress)}
 fn terminal_is_empty(&self)->bool{OwnedFieldReceiving::terminal_is_empty(self)}
}
