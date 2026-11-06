//! 🪟️ Bounded UI text receives original UTF8 bytes without a temporary native string.
use super::UiText;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value::retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits};
use semio_framework_value::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,owned::{NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{NativeValueSource,fixed,receipt,invalid}}};
use std::mem::ManuallyDrop;

pub(crate) struct TextRetirement(pub(crate) Option<UiText>);
impl RetirementCursor for TextRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(fixed(size_of::<UiText>()))}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{if !retained_clone_grant_admits(g,fixed(size_of::<UiText>())){return RetirementStep::BudgetExhausted}let work=u64::from(self.0.take().is_some());RetirementStep::Complete(work)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl RetireOwned for UiText{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(TextRetirement(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TextRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

pub struct UiTextNativeReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<UiText>>,offset:usize,ready:bool,closing:bool,retirement:RetainedCloneClose,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiText{
 type Receiver=UiTextNativeReceiver<I>;
 fn receiver()->Self::Receiver{UiTextNativeReceiver{binding:None,output:ManuallyDrop::new(None),offset:0,ready:false,closing:false,retirement:Default::default(),input:std::marker::PhantomData}}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,UiText> for UiTextNativeReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.ready{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  if self.output.is_none(){return Ok(fixed(size_of::<UiText>()))}
  Ok(fixed(input.get().text()?.len().saturating_sub(self.offset).min(body)))
 }
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  let mut work=0;let result=(||{
   let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;
   let bound=input.bind(&mut self.binding,g);work=bound.work;if let Some(p)=bound.result?{return Ok(RetainedCloneStep::Progress(p))}
   work=1;let text=input.get().text()?;if text.len()>semio_ui_contract_budget::UI_TEXT_MAX_BYTES{return Err(invalid().into())}
   if self.output.is_none(){*self.output=Some(UiText::default());work=1;return Ok(RetainedCloneStep::Progress(receipt(d)))}
   if self.offset<text.len(){if d.copy_bytes==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let end=self.offset+d.copy_bytes;self.output.as_mut().unwrap().bytes[self.offset..end].copy_from_slice(&text.as_bytes()[self.offset..end]);self.offset=end;work=1;return Ok(RetainedCloneStep::Progress(receipt(d)))}
   self.output.as_mut().unwrap().len=text.len()as u16;self.ready=true;work=1;Ok(RetainedCloneStep::Complete(receipt(d)))
  })();RetainedWorkOutcome{result,work}
 }
 fn output_ready(&self)->bool{self.ready&&!self.closing}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.output_ready(){Ok(fixed(size_of::<UiText>()))}else{Err(invalid())}}
 fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(UiText,RetainedCloneProgress)>>{let d=fixed(size_of::<UiText>());if !self.output_ready()||!retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}self.ready=false;self.closing=true;RetainedWorkOutcome{result:Ok(self.output.take().map(|value|(value,receipt(d)))),work:1}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty()||self.output.is_some(){self.retirement.next_owner_demands::<UiText>(self.output.is_some(),&None,body)}else{RetainedCloneBinding::close_demand(&self.binding,body)}}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,g);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for UiTextNativeReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"UI text receiving reached Drop before funded closure")}}
impl NativeDefaultValue for UiText{
 fn default_demand()->RetirementDemand{fixed(size_of::<UiText>())}
 fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,fixed(size_of::<UiText>())){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::default());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(fixed(size_of::<UiText>())))),work:1}}
}
impl NativeDefault for UiText{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
