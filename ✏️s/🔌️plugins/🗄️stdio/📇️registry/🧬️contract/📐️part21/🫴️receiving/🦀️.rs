//! 🫴️ The exact Part21 tagged wire constructs native value owners through the first-party funded receiver.
use super::{Part21Value,Part21Decimal};
use semio_framework_value::{NativeDecodeControl,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceivingFailure,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::owned::{NativeAdvanceSelection,NativeReceiver,OriginalSourceTurnWitness,native_receiver_close_start,native_output_demand,native_output_take,native_record_pure_take};
use semio_framework_value_native_owned::native_decoding::owned::NativeReceiveSlot;
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Part21NativeInput{
 Ref{value:u64},Str{value:String},Binary{value:String},Enum{value:String},Int{value:i64},Real{value:Part21Decimal},List{values:Vec<Part21Value>},Typed{#[value(rename="typeName")]name:String,values:Vec<Part21Value>},Unset,Derived,
}
impl Part21NativeInput{fn into_value(self)->Part21Value{match self{Self::Ref{value}=>Part21Value::Ref(value),Self::Str{value}=>Part21Value::Str(value),Self::Binary{value}=>Part21Value::Binary(value),Self::Enum{value}=>Part21Value::Enum(value),Self::Int{value}=>Part21Value::Int(value),Self::Real{value}=>Part21Value::Real(value),Self::List{values}=>Part21Value::List(values),Self::Typed{name,values}=>Part21Value::Typed{name,items:values},Self::Unset=>Part21Value::Unset,Self::Derived=>Part21Value::Derived}}}
/// 🫴️ A native Part21 value retains its exact input and all partial child owners until funded take or close.
pub struct Part21ValueReceiver<I:NativeValueSource+?Sized+'static>{inner:NativeReceiveSlot<I,Part21NativeInput>,staged:ManuallyDrop<Option<Part21NativeInput>>,output:ManuallyDrop<Option<Part21Value>>,retirement:RetainedCloneClose,closing:bool,taken:bool,checked:bool,binary_offset:usize,binary_unused:u8}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Part21 native receiving requires its original funded phase")}
fn overhead()->usize{2*size_of::<Option<Part21Value>>()+size_of::<Part21NativeInput>()}
fn receipt(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn progress_poll(turn:RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{turn.map(|step|RetainedCloneStep::Progress(step.progress()))}
impl<I:NativeValueSource+?Sized+'static> Part21ValueReceiver<I>{
 fn local_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:overhead(),depth:1,..Default::default()}}
 fn check_binary(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,OriginalReceivingPrimary>{
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Part21 binary check exceeded its work authority").into())}
  if let Some(Part21Value::Binary(value))=self.output.as_ref(){let bytes=value.as_bytes();if bytes.is_empty(){return Err(invalid().into());}if self.binary_offset==bytes.len(){self.checked=true;}else{let byte=bytes[self.binary_offset];if self.binary_offset==0{if !(b'0'..=b'3').contains(&byte)||(bytes.len()==1&&byte!=b'0'){return Err(invalid().into());}self.binary_unused=byte-b'0';}else{let digit=match byte{b'0'..=b'9'=>byte-b'0',b'A'..=b'F'=>byte-b'A'+10,_=>return Err(invalid().into())};if self.binary_offset==1&&digit>=(1u8<<(4-self.binary_unused)){return Err(invalid().into());}}self.binary_offset+=1;if self.binary_offset==bytes.len(){self.checked=true;}}}else{self.checked=true;}
  let progress=receipt(overhead());Ok(if self.checked{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 fn convert(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,OriginalReceivingPrimary>{
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Part21 conversion exceeded its work authority").into())}
  let staged=self.staged.take().ok_or_else(invalid)?;*self.output=Some(staged.into_value());Ok(RetainedCloneStep::Progress(receipt(overhead())))
 }
}
impl<I:NativeValueSource+?Sized+'static> NativeReceive<I> for Part21Value{
 type Receiver=Part21ValueReceiver<I>;
 fn receiver()->Self::Receiver{Part21ValueReceiver{inner:Default::default(),staged:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false,checked:false,binary_offset:0,binary_unused:0}}
}
impl<I:NativeValueSource+?Sized+'static> NativeReceiver<I,Part21Value> for Part21ValueReceiver<I>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.taken||self.checked{return Err(invalid());}if self.output.is_some()||self.staged.is_some(){return Ok(self.local_demand());}nested(if self.inner.ready(){self.inner.take_demand()?}else{self.inner.demands(original,body,maximum_work_units)?})}
 fn select_advance(&self,_original:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{if self.closing||self.taken||self.checked{return Err(invalid());}if self.output.is_some()||self.staged.is_some(){return Ok(NativeAdvanceSelection::Local{work:1});}if self.inner.ready(){return Ok(NativeAdvanceSelection::Local{work:self.inner.take_work_demand()?});}Ok(NativeAdvanceSelection::Nested)}
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{if self.closing||self.taken||self.checked||self.output.is_some()||self.staged.is_some()||self.inner.ready(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});}progress_poll(self.inner.advance(original,control,witness,body,maximum_work_units,child(grant)))}
 fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
   let demand=self.demands(original,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
   if self.staged.is_some(){return self.convert(effect).map(Some);}
   if self.output.is_some(){return self.check_binary(effect).map(Some);}
   match self.inner.take(&mut *self.staged,effect){RetainedWorkPoll::Pending{..}=>Ok(None),RetainedWorkPoll::Complete(turn)=>Ok(Some(RetainedCloneStep::Progress(turn.result?.progress())))}
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&!self.taken&&self.checked&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}native_output_demand::<Part21Value>(0)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<Part21Value>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let turn=if self.output_ready(){native_output_take(&mut *self.output,output,0,effect)}else{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})};
  if let RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(_)),..})=&turn{self.taken=true;}
  native_record_pure_take(turn,reply)
 }
 fn advance_pending(&self)->bool{self.inner.advance_pending()}
 native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.inner.terminal_is_empty(){return nested(if self.inner.is_closing(){self.inner.close_demands(body)?}else{self.inner.begin_close_demand()?});}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Part21Value>(false,&None,body);}if self.staged.is_some(){return self.retirement.next_owner_demands::<Part21NativeInput>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<Part21Value>(true,&None,body);}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.inner.terminal_is_empty(){return if self.inner.is_closing(){self.inner.close_work_demand(body)}else{self.inner.begin_close_work_demand()};}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Part21Value>(false,&None,body);}if self.staged.is_some(){return self.retirement.next_owner_work::<Part21NativeInput>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_work::<Part21Value>(true,&None,body);}Ok(0)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let grant=effect.grant();
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  if !self.closing{return Ok(reply.complete(Err(invalid().into())))}
  let demand=self.close_demands(body)?;
  if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.inner.terminal_is_empty(){return if self.inner.is_closing(){self.inner.close_into(body,&mut effect.nested(child(grant)),reply.progress())}else{self.inner.begin_close_into(effect,reply.progress())}}
  if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply)}
  if self.staged.is_some(){let turn=self.retirement.begin_granted(&mut self.staged,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  let turn=self.retirement.begin_granted(&mut self.output,effect);Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))
 }
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let grant=effect.grant();
  if !self.inner.terminal_is_empty(){return if self.inner.is_closing(){self.inner.recover_close_into(body,&mut effect.nested(child(grant)),reply.progress())}else{self.inner.recover_begin_close_into(effect,reply.progress())}}
  if !self.retirement.is_empty(){return self.retirement.recover_step_into(body,effect,reply)}
  Ok(reply.pending())
 }
 fn requires_original_birth(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{if !self.inner.terminal_is_empty(){return self.inner.close_original_birth_step(original,child(grant))}self.retirement.step_original_birth(original,grant)}
 fn requires_original_control(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if self.requires_original_birth(){return Err(invalid())}
  let grant=effect.grant();
  if !self.inner.terminal_is_empty()&&self.inner.is_closing()&&self.inner.requires_original_control(){return self.inner.close_original_into(original,body,&mut effect.nested(child(grant)),reply.progress())}
  if self.inner.terminal_is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original_into(original,body,effect,reply)}
  self.close_step(body,effect,reply)
 }
 fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let grant=effect.grant();
  if !self.inner.terminal_is_empty()&&self.inner.is_closing()&&self.inner.requires_original_control(){return self.inner.recover_close_original_into(original,body,&mut effect.nested(child(grant)),reply.progress())}
  if self.inner.terminal_is_empty()&&self.retirement.requires_original_control(){return self.retirement.recover_step_original_into(original,body,effect,reply)}
  self.recover_close_step(body,effect,reply)
 }
 fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.staged.is_none()&&self.retirement.is_empty()&&self.inner.terminal_is_empty()}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
