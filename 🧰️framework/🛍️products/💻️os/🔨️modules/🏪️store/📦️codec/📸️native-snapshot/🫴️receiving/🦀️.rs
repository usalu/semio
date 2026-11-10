//! 🫴️ Complete Pack Records and authored typed fields retain the exact genesis source through every turn.
use semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl;
use semio_framework_dsl_record::{BorrowedDslRecord,FieldValue,receiving::{DslOwnedReceivingRecord,DslOwnedReceivingField,OwnedFieldReceiving,OwnedSlot}};
use semio_framework_value::retained_clone::NativeAllocationIdentity;
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep, erased_cursor_turn}};
use semio_framework_dsl_record::receiving::owned::OwnedAdvanceSelection;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};
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
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>;
 fn is_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_into(&mut self,output:&mut Option<P>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}

/// 📸️ The caller retains the original full Pack bytes until typed output and all receiving metadata close.
pub struct OriginalSnapshotRecordReceiving<T:DslOwnedReceivingRecord+BorrowedDslRecord>{
 source:(usize,usize),control:(NativeAllocationIdentity,Option<u64>),envelope:&'static str,prefix:usize,position:usize,phase:u8,limits:PackLimits,
 record:Option<RetainedRecordReceiving>,generic:OwnedSlot<FieldValue>,binding:<T as DslOwnedReceivingField>::Cursor,output:OwnedSlot<T>,closing:bool,closed:bool,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn metadata<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("original snapshot receiving depth overflow"))?;Ok(demand)}
fn perform(effect:&mut RetainedAllocationEffect<'_>,units:u128)->Result<(),ValueError>{let units=u64::try_from(units).map_err(|_|ValueError::invariant("original snapshot receiving work exceeds a turn"))?;if units==0||effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original snapshot receiving work is not funded"))}}
fn unpoll<E>(poll:RetainedWorkPoll<RetainedCloneProgress,E>)->RetainedWorkOutcome<RetainedCloneProgress,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(Default::default()),work}}}
fn stepped(poll:OriginalRetirementPoll<RetainedCloneProgress>)->OriginalRetirementPoll{match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})}}
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> OriginalSnapshotRecordReceiving<T>{
 /// 🎟️ Binds actual immutable source and static schema without constructing a second wire or record.
 pub fn admit(source:&[u8],envelope:&'static str,limits:PackLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=metadata::<Self>();if !funds(grant,demand){return Ok(None)}let prefix=semio_os_semio::declared_envelope_prefix_len(envelope,semio_os_semio::Component::Pack,1)?;
  if source.len()<prefix||source.len()as u64>limits.max_file_len{return Err(invalid("original snapshot Pack is outside its declared source extent"))}
  Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),control:(native.ledger_identity(),native.retirement_recipient_identity()),envelope,prefix,position:0,phase:0,limits,record:None,generic:Default::default(),binding:T::receiving_cursor(),output:Default::default(),closing:false,closed:false},receipt(demand))))
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
 /// 🪜️ Runs inside the caller's one original effect; a denied frontier preserves every source and partial destination.
 pub fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let result=self.step_action(source,native,body,effect);RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn step_action(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  self.validate_control(native)?;let grant=effect.grant();let demand=self.demands(source,body)?;if self.phase>=4||self.closing||self.closed||!funds(grant,demand){return Ok(Default::default())}
  let child=child(grant);
  match self.phase{
   0=>{native.checkpoint()?;perform(effect,1)?;let expected=semio_os_semio::declared_binary_prefix_byte(self.envelope,semio_os_semio::Component::Pack,1,self.position)?;if source.get(self.position).copied()!=expected{return Err(invalid("original snapshot Pack envelope differs from its authored identity").into())}self.position+=1;if self.position==self.prefix{self.phase=1;}Ok(receipt(demand))},
   1=>{let turn=RetainedRecordReceiving::admit_document(source,(self.prefix,source.len()),T::RECORD,self.limits.clone(),child);perform(effect,turn.work)?;let Some((record,progress))=turn.result?else{return Ok(Default::default())};self.record=Some(record);self.phase=2;Ok(progress)},
   2=>{let record=self.record.as_mut().ok_or_else(||invalid("native snapshot record is absent"))?;if !record.is_ready(){return record.step(source,native,&mut effect.nested(child)).result}let mut original=None;let turn=record.take_into(&mut original,child);perform(effect,turn.work)?;let progress=turn.result?;if let Some(record)=original{*self.generic.value=Some(FieldValue::Record(record));self.phase=3;}Ok(progress)},
   3=>{
    if !self.binding.is_ready(){return match self.binding.select_advance(&self.generic.value,body,effect.maximum_work_units())?{
     OwnedAdvanceSelection::Local{..}=>unpoll(self.binding.advance_local(&mut self.generic.value,body,&mut effect.nested(child))).result,
     OwnedAdvanceSelection::Nested=>{let witness=native.source_witness()?;unpoll(self.binding.advance_nested(&mut self.generic.value,native,witness,body,effect.maximum_work_units(),child)).result}
    }}
    let turn=unpoll(self.binding.take_into(&mut self.output.value,&mut effect.nested(child)));if self.output.value.is_some(){self.phase=4;}turn.result.map_err(OriginalReceivingPrimary::from)
   },
   _=>unreachable!(),
  }
 }
 /// 📑️ The schema-checked original Record may be projected in bounded domain rows before whole-owner binding.
 pub fn record_is_ready(&self)->bool{self.phase==3&&!self.closing&&!self.closed&&matches!(self.generic.value.as_ref(),Some(FieldValue::Record(_)))}
 /// 📬️ Transfers that exact generic Record once; its recipient owns all domain projection and retirement.
 pub fn take_record_into(&mut self,output:&mut Option<semio_framework_dsl_record::RecordValue>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let demand=metadata::<semio_framework_dsl_record::RecordValue>();if output.is_some()||!self.record_is_ready()||!funds(grant,demand){return Ok(Default::default())}
  let Some(FieldValue::Record(record))=self.generic.value.take()else{unreachable!()};*output=Some(record);self.phase=5;Ok(receipt(demand))
 }
 /// 👁️ Completion includes the actual authored typed destination, independently of later source cleanup.
 pub fn is_ready(&self)->bool{self.phase==4&&!self.closing&&!self.closed&&self.output.value.is_some()}
 /// 📬️ Moves the same typed owner after a separately funded recipient admission.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(metadata::<T>())}
 pub fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let demand=metadata::<T>();if output.is_some()||!self.is_ready()||!funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();self.phase=5;Ok(receipt(demand))}
 /// 🧹️ Each retained typed, generic and physical parser owner closes under its own next demand.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}if !self.closing{return Ok(metadata::<bool>())}if !self.output.terminal_is_empty(){return nested(self.output.close_demands(body)?)}if !self.binding.terminal_is_empty(){return nested(self.binding.close_demands(body)?)}if !self.generic.terminal_is_empty(){return nested(self.generic.close_demands(body)?)}if let Some(record)=&self.record{if !record.terminal_is_empty(){return nested(record.close_demands(body)?)}return Ok(metadata::<Option<RetainedRecordReceiving>>())}Ok(metadata::<bool>())
 }
 /// ♻️ Every partial value remains resident when admission refuses or cancellation starts closure.
 /// ♻️ Settles the close poll as the erased cursor step.
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{erased_cursor_turn(self.close_poll(body,effect))}
 pub fn close_poll(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let child=child(grant);
  let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  let demand=match self.close_demands(body){Ok(demand)=>demand,Err(error)=>return settle(effect,Err(error))};if !funds(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  if !self.closing{if let Err(error)=perform(effect,1){return settle(effect,Err(error))}self.closing=true;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if !self.output.terminal_is_empty(){return stepped(self.output.close_step(&mut effect.nested(child)))}
  if !self.binding.terminal_is_empty(){return stepped(self.binding.close_step(&mut effect.nested(child)))}
  if !self.generic.terminal_is_empty(){return stepped(self.generic.close_step(&mut effect.nested(child)))}
  if let Some(record)=&mut self.record{if !record.terminal_is_empty(){return match record.close_step(body,&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}if let Err(error)=perform(effect,1){return settle(effect,Err(error))}self.record=None;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if let Err(error)=perform(effect,1){return settle(effect,Err(error))}self.closed=true;settle(effect,Ok(RetainedCloneStep::Complete(receipt(demand))))
 }
 /// 🕳️ Terminal means no typed, generic or Pack backing can be released by implicit destruction.
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.record.is_none()&&self.generic.terminal_is_empty()&&self.binding.terminal_is_empty()&&self.output.terminal_is_empty()}
}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> Drop for OriginalSnapshotRecordReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original snapshot receiver abandoned paid source or partial typed owners")}}

impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> RetireOwned for OriginalSnapshotRecordReceiving<T>{
 type RetirementCursor=Self;
 fn retirement_seed(mut self)->Self{self.closing=true;self}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> RetirementCursor for OriginalSnapshotRecordReceiving<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 semio_framework_value::native_retirement_reply!();
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<T:DslOwnedReceivingRecord+BorrowedDslRecord> OriginalArtifactSnapshotReceiving<T> for OriginalSnapshotRecordReceiving<T>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands(source,body)}
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{OriginalSnapshotRecordReceiving::step(self,source,native,body,effect)}
 fn is_ready(&self)->bool{self.is_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.take_demand()}
 fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.take_into(output,grant)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let body=effect.grant().maximum_copy_bytes;OriginalSnapshotRecordReceiving::close_poll(self,body,effect)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="📜️pair/🦀️.rs"]
mod pair;
pub use pair::{OriginalArtifactHistoryReadLimits,OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactPairCause};
