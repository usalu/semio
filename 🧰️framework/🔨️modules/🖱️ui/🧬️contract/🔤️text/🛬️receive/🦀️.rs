//! 🪟️ Bounded UI text receives original UTF8 bytes without a temporary native string.
use super::UiText;
use semio_framework_value::{NativeDecodeControl,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,record_retirement_work};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,NativeReceivingFailure,owned::{OriginalSourceTurnWitness,NativeAdvanceSelection,native_receiver_close_start,NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,native_output_take,native_record_pure_take,tree::{NativeValueSource,fixed,receipt,invalid}}};
use std::mem::ManuallyDrop;

#[doc(hidden)]
pub struct TextRetirement(pub(crate) Option<UiText>);
impl RetirementCursor for TextRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(fixed(size_of::<UiText>()))}
 fn close_into<'reply>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if self.0.is_none(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
  if effect.maximum_work_units()==0||!retained_clone_grant_admits(effect.grant(),fixed(size_of::<UiText>())){return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
  let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return Ok(reply.complete(Ok(RetirementStep::Failure(error.into(),work))))};
  self.0=None;Ok(reply.complete(Ok(RetirementStep::Complete(work))))
 }
 fn recover_close_into<'reply>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl RetireOwned for UiText{
 type RetirementCursor=TextRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{TextRetirement(Some(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TextRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

pub struct UiTextNativeReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<UiText>>,offset:usize,ready:bool,closing:bool,retirement:RetainedCloneClose,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiText{
 type Receiver=UiTextNativeReceiver<I>;
 fn receiver()->Self::Receiver{UiTextNativeReceiver{binding:None,output:ManuallyDrop::new(None),offset:0,ready:false,closing:false,retirement:Default::default(),input:std::marker::PhantomData}}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,UiText> for UiTextNativeReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.ready{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  if self.output.is_none(){return Ok(fixed(size_of::<UiText>()))}
  Ok(fixed(input.get().text()?.len().saturating_sub(self.offset).min(body)))
 }
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.ready{return Err(invalid())}
  let work=if input.check_binding(&self.binding)?{1}else{input.binding_work_demand(&self.binding)?};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"UI text action exceeds its original work ceiling"))}
  Ok(NativeAdvanceSelection::Local{work})
 }
 fn advance_nested(&mut self,_:RetainedCloneRef<'_,I>,_:&mut NativeDecodeControl<'_>,_:OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)}
   let text=input.get().text()?;if text.len()>semio_ui_contract_budget::UI_TEXT_MAX_BYTES{return Err(invalid().into())}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"UI text action exceeded its original work").into())}
   let Some(output)=self.output.as_mut()else{*self.output=Some(UiText::default());return Ok(RetainedCloneStep::Progress(receipt(d)))};
   if self.offset<text.len(){if d.copy_bytes==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let end=self.offset+d.copy_bytes;output.bytes[self.offset..end].copy_from_slice(&text.as_bytes()[self.offset..end]);self.offset=end;return Ok(RetainedCloneStep::Progress(receipt(d)))}
   output.len=text.len()as u16;self.ready=true;Ok(RetainedCloneStep::Complete(receipt(d)))
  })();
  Ok(reply.complete(result))
 }
 fn output_ready(&self)->bool{self.ready&&!self.closing}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.output_ready(){semio_framework_value::native_decoding::owned::native_output_demand::<UiText>(0)}else{Err(invalid())}}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<UiText>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if !self.output_ready(){return Ok(reply.complete(Err(invalid().into())))}
  let turn=native_output_take(&mut self.output,output,0,effect);
  if matches!(&turn,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(_)),..})){self.ready=false;self.closing=true;}
  native_record_pure_take(turn,reply)
 }
 fn advance_pending(&self)->bool{false}
 native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty()||self.output.is_some(){self.retirement.next_owner_demands::<UiText>(self.output.is_some(),&None,body)}else{RetainedCloneBinding::close_demand(&self.binding,body)}}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty()||self.output.is_some(){self.retirement.next_owner_work::<UiText>(self.output.is_some(),&None,body)}else{RetainedCloneBinding::close_work_demand(&self.binding,body)}}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if !self.closing{return Ok(reply.complete(Err(invalid().into())))}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let demand=self.close_demands(body)?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply)}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
 }
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if !self.retirement.is_empty(){return self.retirement.recover_step_into(body,effect,reply)}if self.output.is_none()&&self.binding.is_some(){return self.close_step(body,effect,reply)}Ok(reply.pending())}
 fn close_original_step<'reply>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_step(body,effect,reply)}
 fn recover_close_original_step<'reply>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.recover_close_step(body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for UiTextNativeReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"UI text receiving reached Drop before funded closure")}}
impl NativeDefaultValue for UiText{
 fn default_demand()->RetirementDemand{fixed(size_of::<UiText>())}
 fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if output.is_some(){return Ok(reply.complete(Err(invalid().into())))}let d=Self::default_demand();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"UI text default exceeded its work authority").into())))}*output=Some(Self::default());Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
 fn recover_default<'reply>(_:&mut Option<Self>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
}
impl NativeDefault for UiText{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
