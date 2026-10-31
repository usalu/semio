//! 🛬️ Graph original trees retain sorted unique property bodies under the same funded native receiver.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{Number,NativeDecodeControl,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,owned::{NativeDefault,tree::{NativeValueSource,NativeTreeKind,bulk::{NativeTreeReceiver,NativeTreeValue}}}};
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
pub struct PropertyBagReceiver<I:NativeValueSource+?Sized>{child:NativeTreeReceiver<I,PropertyValue>,validated:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PropertyBag{type Receiver=PropertyBagReceiver<I>;fn receiver()->Self::Receiver{PropertyBagReceiver{child:NativeTreeReceiver::new(),validated:false}}}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,PropertyBag> for PropertyBagReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if !self.validated{return Ok(RetirementDemand{depth:1,..Default::default()})}self.child.demands(input,body)}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  if self.validated{return self.child.advance(input,control,body,g)}let d=RetirementDemand{depth:1,..Default::default()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if let Err(error)=control.checkpoint(){return RetainedWorkOutcome{result:Err(error.into()),work:0}}let p=RetainedCloneProgress{copied_items:1,..Default::default()};if input.get().tree_kind()!=Some(NativeTreeKind::Object){return RetainedWorkOutcome{result:Err(refusal("graph property object required").with_retained_progress(p).into()),work:1}}self.validated=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work:1}
 }
 fn output_ready(&self)->bool{self.validated&&self.child.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.child.take_demand()}
 fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(PropertyBag,RetainedCloneProgress)>>{let turn=self.child.take(g);RetainedWorkOutcome{result:turn.result.map(|value|value.map(|(value,p)|{let PropertyValue::Object(bag)=value else{unreachable!("qualified original object retains its native object kind")};(bag,p)})),work:turn.work}}
 fn begin_close(&mut self){self.child.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.child.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.child.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()}
}
impl semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue for PropertyBag{
 fn default_demand()->semio_framework_value::RetirementDemand{semio_framework_value_native_owned::native_decoding::owned::tree::fixed(0)}
 fn admit_default(output:&mut Option<Self>,g:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{if output.is_some(){return semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default())),work:0}}let d=<Self as semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue>::default_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::new());semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value_native_owned::native_decoding::owned::tree::receipt(d))),work:1}}
}
impl semio_framework_value_native_owned::native_decoding::owned::NativeDefault for PropertyBag{type Cursor=semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor::new()}}


#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod work_tests;
