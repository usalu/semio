//! 🛬️ Graph original trees retain sorted unique property bodies under the same funded native receiver.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{Number,NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,NativeReceivingFailure,owned::{OriginalSourceTurnWitness,NativeAdvanceSelection,NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{fixed,receipt,NativeValueSource,NativeTreeKind,bulk::{NativeTreeReceiver,NativeTreeValue}}}};
use semio_framework_value_native_owned::{OriginalRetirementFailure,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll};
use std::mem::size_of;
fn refusal(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
impl NativeTreeValue for PropertyValue{
 fn null()->Self{Self::Null}
 fn boolean(value:bool)->Self{Self::Bool(value)}
 fn number(value:Number)->Result<Self,ValueError>{Ok(Self::Number(value.as_f64()))}
 fn text(value:String)->Self{Self::String(value)}
 fn array(value:Vec<Self>)->Self{Self::Array(value)}
 fn object(value:Vec<(String,Self)>)->Self{Self::Object(PropertyBag::from_admitted(value))}
 fn sorted_unique_objects()->bool{true}
 fn scalar_copy_bytes(kind:NativeTreeKind)->usize{match kind{NativeTreeKind::Bool=>size_of::<bool>(),NativeTreeKind::Number=>size_of::<f64>(),_=>0}}
 fn depth_message()->&'static str{"graph property subtree exceeds depth limit"}
 fn duplicate_message()->&'static str{"graph property object repeats a key"}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PropertyValue{type Receiver=NativeTreeReceiver<I,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
/// 🗂️ Root object qualification precedes the genuine dynamic child without discarding partial custody.
pub struct PropertyBagReceiver<I:NativeValueSource+?Sized>{child:NativeTreeReceiver<I,PropertyValue>,taken:Option<PropertyValue>,validated:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PropertyBag{type Receiver=PropertyBagReceiver<I>;fn receiver()->Self::Receiver{PropertyBagReceiver{child:NativeTreeReceiver::new(),taken:None,validated:false}}}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,PropertyBag> for PropertyBagReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if !self.validated{return Ok(RetirementDemand{depth:1,..Default::default()})}self.child.demands(input,body,maximum_work_units)}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{if self.validated{return self.child.select_advance(input,body,maximum_work_units)}if maximum_work_units<1{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"graph property qualification exceeds its original work ceiling"))}Ok(NativeAdvanceSelection::Local{work:1})}
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{self.child.advance_nested(input,control,witness,body,maximum_work_units,g)}
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if self.validated{return self.child.advance_local(input,body,effect,reply)}
  let d=RetirementDemand{depth:1,..Default::default()};if !retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"graph property qualification exceeded its work authority").into())))}
  let p=RetainedCloneProgress{copied_items:1,..Default::default()};if input.get().tree_kind()!=Some(NativeTreeKind::Object){return Ok(reply.complete(Err(refusal("graph property object required").with_retained_progress(p).into())))}
  self.validated=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(p))))
 }
 fn recover_advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if self.validated{self.child.recover_advance_local(input,body,effect,reply)}else{Ok(reply.pending())}}
 fn output_ready(&self)->bool{self.validated&&self.child.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.child.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.child.take_work_demand()}
 fn take<'reply>(&mut self,output:&mut Option<PropertyBag>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{let complete=self.child.take(&mut self.taken,effect,reply)?;self.publish(output)?;Ok(complete)}
 fn recover_take<'reply>(&mut self,output:&mut Option<PropertyBag>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{let complete=self.child.recover_take(&mut self.taken,effect,reply)?;self.publish(output)?;Ok(complete)}
 fn advance_pending(&self)->bool{self.child.advance_pending()}
 fn is_closing(&self)->bool{self.child.is_closing()}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.child.begin_close_demand()}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.child.begin_close_work_demand()}
 fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.begin_close_into(effect,reply)}
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.recover_begin_close_into(effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.child.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.child.close_work_demand(body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.close_step(body,effect,reply)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.recover_close_step(body,effect,reply)}
 fn requires_original_control(&self)->bool{self.child.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.child.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.child.close_original_birth_step(original,grant)}
 fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.close_original_step(original,body,effect,reply)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.child.recover_close_original_step(original,body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.taken.is_none()&&self.child.terminal_is_empty()}
}
impl<I:NativeValueSource+?Sized> PropertyBagReceiver<I>{fn publish(&mut self,output:&mut Option<PropertyBag>)->Result<(),ValueError>{match self.taken.take(){None=>Ok(()),Some(PropertyValue::Object(bag))=>{*output=Some(bag);Ok(())},Some(_)=>Err(ValueError::invariant("qualified original object lost its native object kind"))}}}
impl NativeDefaultValue for PropertyBag{
 fn default_demand()->RetirementDemand{fixed(size_of::<Option<Self>>())}
 fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if output.is_some(){return Ok(reply.complete(Err(refusal("graph property default output is occupied").into())))}let d=Self::default_demand();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"graph property default exceeded its work authority").into())))}*output=Some(Self::new());Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
 fn recover_default<'reply>(_:&mut Option<Self>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
}
impl NativeDefault for PropertyBag{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}


#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod work_tests;
