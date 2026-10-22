//! 🫴️ Complete Pack Records and authored typed fields retain the exact genesis source through every turn.
use semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl;
use semio_framework_dsl_record::{BorrowedDslRecord,FieldValue,receiving::{DslOwnedReceivingRecord,DslOwnedReceivingField,OwnedFieldReceiving,OwnedSlot}};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::size_of;
use protocol::PackLimits;
use pack::record::RetainedRecordReceiving;

/// 🗿️ Native genesis declares a required persistent original receiver independently of cold Pack decoding.
pub trait OriginalArtifactPackReceiving:semio_framework_value::retirement::RetireOwned+Sync+Sized {
 type Receiving:OriginalArtifactSnapshotReceiving<Self>;
 fn original_pack_birth_demand()->RetirementDemand;
 fn admit_original_pack(source:&[u8],limits:PackLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self::Receiving,RetainedCloneProgress)>,ValueError>;
}

/// 🫴️ The original Pack and actual receiving ledger survive until native output and every partial owner close.
pub trait OriginalArtifactSnapshotReceiving<P>:Send {
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>;
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn is_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_into(&mut self,output:&mut Option<P>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,native:&mut dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

/// 📸️ The caller retains the original full Pack bytes until typed output and all receiving metadata close.
pub struct OriginalSnapshotRecordReceiving<T:DslOwnedReceivingRecord+BorrowedDslRecord>{
 source:(usize,usize),control:(u64,Option<u64>),envelope:&'static str,prefix:usize,position:usize,phase:u8,limits:PackLimits,
 record:Option<RetainedRecordReceiving>,generic:OwnedSlot<FieldValue>,binding:<T as DslOwnedReceivingField>::Cursor,output:OwnedSlot<T>,closing:bool,closed:bool,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn metadata<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("original snapshot receiving depth overflow"))?;Ok(demand)}
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> OriginalSnapshotRecordReceiving<T>{
 /// 🎟️ Binds actual immutable source and static schema without constructing a second wire or record.
 pub fn admit(source:&[u8],envelope:&'static str,limits:PackLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=metadata::<Self>();if !funds(grant,demand){return Ok(None)}let prefix=semio_framework_os_semio::declared_envelope_prefix_len(envelope,semio_framework_os_semio::Component::Pack,1)?;
  if source.len()<prefix||source.len()as u64>limits.max_file_len{return Err(invalid("original snapshot Pack is outside its declared source extent"))}
  Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),control:(native.ledger_identity(),native.retirement_recipient_identity()),envelope,prefix,position:0,phase:0,limits,record:None,generic:Default::default(),binding:Default::default(),output:Default::default(),closing:false,closed:false},receipt(demand))))
 }
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original snapshot Pack source identity changed"))}Ok(())}
 fn validate_control(&self,native:&dyn NativeDecodeCloseControl)->Result<(),ValueError>{if self.control!=(native.ledger_identity(),native.retirement_recipient_identity()){return Err(invalid("original snapshot receiving changed its actual receiving owner"))}Ok(())}
 /// 📐️ Quotes one envelope byte, original Pack action, field binding or fixed output transfer.
 pub fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.closing||self.closed{return Ok(Default::default())}match self.phase{
   0=>Ok(metadata::<u8>()),1=>Ok(RetirementDemand{depth:2,..Default::default()}),
   2=>if self.record.as_ref().unwrap().is_ready(){Ok(nested(metadata::<FieldValue>())?)}else{nested(self.record.as_ref().unwrap().demands(source)?)},
   3=>if self.binding.is_ready(){nested(metadata::<T>())}else{nested(self.binding.demands(&self.generic.value,body)?)},
   _=>Ok(Default::default()),
  }
 }
 /// 🪜️ Borrows the real receiving control; a denied frontier preserves every source and partial destination.
 pub fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.validate_control(native)?;let demand=self.demands(source,body)?;if self.phase>=4||self.closing||self.closed||!funds(grant,demand){return Ok(Default::default())}native.checkpoint()?;
  match self.phase{
   0=>{let expected=semio_framework_os_semio::declared_binary_prefix_byte(self.envelope,semio_framework_os_semio::Component::Pack,1,self.position)?;if source.get(self.position).copied()!=expected{return Err(invalid("original snapshot Pack envelope differs from its authored identity"))}self.position+=1;if self.position==self.prefix{self.phase=1;}},
   1=>{let Some((record,progress))=RetainedRecordReceiving::admit_document(source,(self.prefix,source.len()),T::RECORD,self.limits.clone(),child(grant))?else{return Ok(Default::default())};self.record=Some(record);self.phase=2;return Ok(progress)},
   2=>{let record=self.record.as_mut().unwrap();if !record.is_ready(){return record.step(source,native,child(grant))}let mut original=None;let progress=record.take_into(&mut original,child(grant))?;if let Some(record)=original{*self.generic.value=Some(FieldValue::Record(record));self.phase=3;}return Ok(progress)},
   3=>{if !self.binding.is_ready(){return self.binding.step(&mut self.generic.value,native,body,child(grant))}let progress=self.binding.take_into(&mut self.output.value,child(grant))?;if self.output.value.is_some(){self.phase=4;}return Ok(progress)},
   _=>unreachable!(),
  }Ok(receipt(demand))
 }
 /// 👁️ Completion includes the actual authored typed destination, independently of later source cleanup.
 pub fn is_ready(&self)->bool{self.phase==4&&!self.closing&&!self.closed&&self.output.value.is_some()}
 /// 📬️ Moves the same typed owner after a separately funded recipient admission.
 pub fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let demand=metadata::<T>();if output.is_some()||!self.is_ready()||!funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();self.phase=5;Ok(receipt(demand))}
 /// 🧹️ Each retained typed, generic and physical parser owner closes under its own next demand.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}if !self.output.terminal_is_empty(){return nested(self.output.close_demands(body)?)}if !self.binding.terminal_is_empty(){return nested(self.binding.close_demands(body)?)}if !self.generic.terminal_is_empty(){return nested(self.generic.close_demands(body)?)}if let Some(record)=&self.record{if !record.terminal_is_empty(){return nested(record.close_demands(body)?)} }Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ♻️ Every partial value remains resident when admission refuses or cancellation starts closure.
 pub fn close_step(&mut self,native:&mut dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.validate_control(native)?;if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}native.charge(demand.capacity_bytes)?;self.closing=true;
  if !self.output.terminal_is_empty(){return self.output.close_step(child(grant)).map(RetainedCloneStep::Progress)}if !self.binding.terminal_is_empty(){return self.binding.close_step(child(grant)).map(RetainedCloneStep::Progress)}if !self.generic.terminal_is_empty(){return self.generic.close_step(child(grant)).map(RetainedCloneStep::Progress)}if let Some(record)=&mut self.record{if !record.terminal_is_empty(){return record.close_step(child(grant))}self.record=None;return Ok(RetainedCloneStep::Progress(receipt(demand)))}self.closed=true;Ok(RetainedCloneStep::Complete(receipt(demand)))
 }
 /// 🕳️ Terminal means no typed, generic or Pack backing can be released by implicit destruction.
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.record.is_none()&&self.generic.terminal_is_empty()&&self.binding.terminal_is_empty()&&self.output.terminal_is_empty()}
}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> Drop for OriginalSnapshotRecordReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original snapshot receiver abandoned paid source or partial typed owners")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="📜️pair/🦀️.rs"]
mod pair;
pub use pair::{OriginalArtifactHistoryReadLimits,OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactPairCause};
