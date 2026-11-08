//! 🛬️ Graph original trees retain sorted unique property bodies under the same funded native receiver.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{Number,NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,owned::{NativeAdvanceSelection,NativeDefault,tree::{NativeValueSource,NativeTreeKind,bulk::{NativeTreeReceiver,NativeTreeValue}}}};
use semio_framework_value_native_owned::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll};
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
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{self.child.advance_nested(input,control,body,maximum_work_units,g)}
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
 fn begin_close(&mut self){self.child.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.child.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.child.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.child.close_step(effect)}
 fn requires_original_control(&self)->bool{self.child.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.child.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.child.close_original_birth_step(original,grant)}
 fn close_original_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.child.close_original_step(original,effect)}
 fn terminal_is_empty(&self)->bool{self.taken.is_none()&&self.child.terminal_is_empty()}
}
impl<I:NativeValueSource+?Sized> PropertyBagReceiver<I>{fn publish(&mut self,output:&mut Option<PropertyBag>)->Result<(),ValueError>{match self.taken.take(){None=>Ok(()),Some(PropertyValue::Object(bag))=>{*output=Some(bag);Ok(())},Some(_)=>Err(ValueError::invariant("qualified original object lost its native object kind"))}}}
impl semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue for PropertyBag{
 fn default_demand()->semio_framework_value::RetirementDemand{semio_framework_value_native_owned::native_decoding::owned::tree::fixed(0)}
 fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn default_allocates()->bool{false}
 fn admit_default(output:&mut Option<Self>,g:semio_framework_value::RetainedCloneGrant,_actual:Option<&mut Option<usize>>)->semio_framework_value::RetainedWorkOutcome{if output.is_some(){return semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default())),work:0}}let d=<Self as semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue>::default_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::new());semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value_native_owned::native_decoding::owned::tree::receipt(d))),work:1}}
}
impl semio_framework_value_native_owned::native_decoding::owned::NativeDefault for PropertyBag{type Cursor=semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor::new()}}


#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod work_tests;
