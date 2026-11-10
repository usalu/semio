//! 🪴️ Recursive UI values receive exact original descendants into the registered bounded arena without a cold wire conversion.
use super::*;
use semio_framework_value::{NativeDecodeControl,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,record_retirement_work};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value::native_decoding::{NativeReceive,NativeReceiver,OriginalReceivingPrimary,NativeReceivingFailure,owned::{OriginalSourceTurnWitness,NativeAdvanceSelection,native_receiver_close_start,native_output_demand,native_output_take,native_record_pure_take,NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{NativeValueSource,NativeTreeKind,NativeReceiveSlot,fixed,receipt,invalid,nested,child_grant}}};
use std::mem::ManuallyDrop;
#[path="🏟️collections/🦀️.rs"]mod collections;
/// 🧵️ One exact UI value and its final-alias descendants retire through a paid arena step.
#[doc(hidden)]
pub struct UiNativeRetirement{owner:UiValueRetirement}
const RETIRE_METADATA:usize=size_of::<UiNativeRetirement>()+size_of::<collection_arena::UiPageSlot>()+size_of::<collection_arena::UiCollectionSlot>();
impl RetirementCursor for UiNativeRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(!self.owner.terminal_is_empty()))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(fixed(RETIRE_METADATA+body.min(UI_TEXT_MAX_BYTES)))}
 fn close_into<'reply>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if self.owner.terminal_is_empty(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
  let g=effect.grant();
  if g.maximum_items==0||g.maximum_depth==0||g.maximum_copy_bytes<=RETIRE_METADATA||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
  let step=self.owner.close_step(1,(g.maximum_copy_bytes-RETIRE_METADATA).min(UI_TEXT_MAX_BYTES));
  Ok(reply.complete(Ok(match step{
   Ok(step) if step.progressed=>match record_retirement_work(effect,1){Ok(work)=>RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:RETIRE_METADATA+step.released_bytes,..Default::default()},work),Err((error,work))=>RetirementStep::Failure(error.into(),work)},
   Ok(_)=>RetirementStep::BudgetExhausted,
   Err(error)=>match record_retirement_work(effect,1){Ok(work)|Err((_,work))=>RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,error).into(),work)},
  })))
 }
 fn recover_close_into<'reply>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
macro_rules! retire_owned{($t:ty,$owner:ident=>$value:expr)=>{impl RetireOwned for $t{type RetirementCursor=UiNativeRetirement;fn retirement_seed(self)->UiNativeRetirement{let $owner=self;UiNativeRetirement{owner:$value}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<UiNativeRetirement>())}fn controlled_retirement_supported()->bool{true}}}}
retire_owned!(UiValue,value=>UiValueRetirement::new(value));
retire_owned!(UiValueRetirement,owner=>owner);
retire_owned!(UiList,list=>UiValueRetirement::new(UiValue::List(list)));
retire_owned!(UiMap,map=>UiValueRetirement::new(UiValue::Map(map)));
retire_owned!(UiListBuilder,builder=>UiValueRetirement::new(UiValue::List(builder.finish_initialized())));
retire_owned!(UiMapBuilder,builder=>UiValueRetirement::new(UiValue::Map(builder.finish_initialized())));
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Kind,Scalar,Child,Wrap,Ready,Spent}
/// 🌳️ The kind, the one selected child receiver, its resident output and the final value stay owned until paid publication or closure.
pub struct UiValueNativeReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,text:NativeReceiveSlot<I,UiText>,list:NativeReceiveSlot<I,UiList>,map:NativeReceiveSlot<I,UiMap>,text_out:ManuallyDrop<Option<UiText>>,list_out:ManuallyDrop<Option<UiList>>,map_out:ManuallyDrop<Option<UiMap>>,output:ManuallyDrop<Option<UiValue>>,retirement:RetainedCloneClose,kind:Option<NativeTreeKind>,phase:Phase,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiValue{type Receiver=UiValueNativeReceiver<I>;fn receiver()->Self::Receiver{UiValueNativeReceiver{binding:None,text:Default::default(),list:Default::default(),map:Default::default(),text_out:ManuallyDrop::new(None),list_out:ManuallyDrop::new(None),map_out:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),kind:None,phase:Phase::Kind,closing:false}}}
macro_rules! open_slot{($self:ident,$slot:ident=>$body:expr,$otherwise:expr)=>{if !$self.text.terminal_is_empty(){let $slot=&mut $self.text;$body}else if !$self.list.terminal_is_empty(){let $slot=&mut $self.list;$body}else if !$self.map.terminal_is_empty(){let $slot=&mut $self.map;$body}else{$otherwise}}}
macro_rules! open_slot_ref{($self:ident,$slot:ident=>$body:expr,$otherwise:expr)=>{if !$self.text.terminal_is_empty(){let $slot=&$self.text;$body}else if !$self.list.terminal_is_empty(){let $slot=&$self.list;$body}else if !$self.map.terminal_is_empty(){let $slot=&$self.map;$body}else{$otherwise}}}
impl<I:NativeValueSource+?Sized> UiValueNativeReceiver<I>{
 fn child_ready(&self)->bool{match self.kind{Some(NativeTreeKind::Text)=>self.text.ready(),Some(NativeTreeKind::Array)=>self.list.ready(),Some(NativeTreeKind::Object)=>self.map.ready(),_=>false}}
 fn selected_take(&self)->Result<(RetirementDemand,u64),ValueError>{match self.kind{Some(NativeTreeKind::Text)=>Ok((self.text.take_demand()?,self.text.take_work_demand()?)),Some(NativeTreeKind::Array)=>Ok((self.list.take_demand()?,self.list.take_work_demand()?)),Some(NativeTreeKind::Object)=>Ok((self.map.take_demand()?,self.map.take_work_demand()?)),_=>Err(invalid())}}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,UiValue> for UiValueNativeReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  match self.phase{
   Phase::Kind=>Ok(fixed(size_of::<NativeTreeKind>()+size_of::<Option<NativeTreeKind>>())),
   Phase::Scalar|Phase::Wrap=>Ok(fixed(size_of::<UiValue>()+size_of::<Option<UiValue>>())),
   Phase::Child=>if self.child_ready(){nested(self.selected_take()?.0,0)}else{match self.kind{Some(NativeTreeKind::Text)=>nested(self.text.demands(input,body,maximum_work_units)?,0),Some(NativeTreeKind::Array)=>nested(self.list.demands(input,body,maximum_work_units)?,0),Some(NativeTreeKind::Object)=>nested(self.map.demands(input,body,maximum_work_units)?,0),_=>Err(invalid())}},
   Phase::Ready|Phase::Spent=>Err(invalid()),
  }
 }
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  let work=match self.phase{
   Phase::Kind|Phase::Scalar|Phase::Wrap=>1,
   Phase::Child=>if self.child_ready(){self.selected_take()?.1}else{return Ok(NativeAdvanceSelection::Nested)},
   Phase::Ready|Phase::Spent=>return Err(invalid()),
  };
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native UI value action exceeds its original work ceiling"))}
  Ok(NativeAdvanceSelection::Local{work})
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  let refuse=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
  if self.closing||self.binding.is_none()||self.phase!=Phase::Child||self.child_ready(){return refuse()}
  match self.kind{
   Some(NativeTreeKind::Text)=>self.text.advance(input,control,witness,body,maximum_work_units,child_grant(g,0)).map(|step|RetainedCloneStep::Progress(step.progress())),
   Some(NativeTreeKind::Array)=>self.list.advance(input,control,witness,body,maximum_work_units,child_grant(g,0)).map(|step|RetainedCloneStep::Progress(step.progress())),
   Some(NativeTreeKind::Object)=>self.map.advance(input,control,witness,body,maximum_work_units,child_grant(g,0)).map(|step|RetainedCloneStep::Progress(step.progress())),
   _=>refuse(),
  }
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  macro_rules! perform{()=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native UI value action exceeded its original work").into())}}}
  macro_rules! publish{($slot:ident,$out:ident)=>{match self.$slot.take(&mut *self.$out,effect){RetainedWorkPoll::Pending{..}=>return Ok(None),RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.$out.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.phase=Phase::Wrap;return Ok(Some(RetainedCloneStep::Progress(step.progress())))}}}}
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   match self.phase{
    Phase::Kind=>{perform!();let kind=input.get().tree_kind().ok_or_else(invalid)?;self.kind=Some(kind);self.phase=match kind{NativeTreeKind::Null|NativeTreeKind::Bool|NativeTreeKind::Number=>Phase::Scalar,_=>Phase::Child};Ok(Some(RetainedCloneStep::Progress(receipt(d))))},
    Phase::Scalar=>{perform!();let value=match self.kind{Some(NativeTreeKind::Null)=>{if !input.get().is_null(){return Err(invalid().into())}UiValue::Null},Some(NativeTreeKind::Bool)=>UiValue::Bool(input.get().boolean()?),Some(NativeTreeKind::Number)=>{let value=input.get().number()?.as_f64();if !value.is_finite(){return Err(invalid().into())}UiValue::Number(value)},_=>return Err(invalid().into())};*self.output=Some(value);self.phase=Phase::Ready;Ok(Some(RetainedCloneStep::Complete(receipt(d))))},
    Phase::Child=>match self.kind{Some(NativeTreeKind::Text)=>publish!(text,text_out),Some(NativeTreeKind::Array)=>publish!(list,list_out),Some(NativeTreeKind::Object)=>publish!(map,map_out),_=>Err(invalid().into())},
    Phase::Wrap=>{perform!();let value=match self.kind{Some(NativeTreeKind::Text)=>UiValue::Text(self.text_out.take().ok_or_else(invalid)?),Some(NativeTreeKind::Array)=>UiValue::List(self.list_out.take().ok_or_else(invalid)?),Some(NativeTreeKind::Object)=>UiValue::Map(self.map_out.take().ok_or_else(invalid)?),_=>return Err(invalid().into())};*self.output=Some(value);self.phase=Phase::Ready;Ok(Some(RetainedCloneStep::Complete(receipt(d))))},
    Phase::Ready|Phase::Spent=>Err(invalid().into()),
   }
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}native_output_demand::<UiValue>(size_of_val(&self.phase))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<UiValue>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if !self.output_ready(){return Ok(reply.complete(Err(invalid().into())))}
  let turn=native_output_take(&mut self.output,output,size_of_val(&self.phase),effect);
  if matches!(&turn,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(_)),..})){self.phase=Phase::Spent}
  native_record_pure_take(turn,reply)
 }
 fn advance_pending(&self)->bool{self.text.advance_pending()||self.list.advance_pending()||self.map.advance_pending()}
 native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  open_slot_ref!(self,slot=>return if slot.is_closing(){nested(slot.close_demands(body)?,0)}else{slot.begin_close_demand()},());
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<UiValue>(false,&None,body)}
  if self.text_out.is_some(){return self.retirement.next_owner_demands::<UiText>(true,&None,body)}
  if self.list_out.is_some(){return self.retirement.next_owner_demands::<UiList>(true,&None,body)}
  if self.map_out.is_some(){return self.retirement.next_owner_demands::<UiMap>(true,&None,body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<UiValue>(true,&None,body)}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(invalid())}
  open_slot_ref!(self,slot=>return if slot.is_closing(){slot.close_work_demand(body)}else{slot.begin_close_work_demand()},());
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<UiValue>(false,&None,body)}
  if self.text_out.is_some(){return self.retirement.next_owner_work::<UiText>(true,&None,body)}
  if self.list_out.is_some(){return self.retirement.next_owner_work::<UiList>(true,&None,body)}
  if self.map_out.is_some(){return self.retirement.next_owner_work::<UiMap>(true,&None,body)}
  if self.output.is_some(){return self.retirement.next_owner_work::<UiValue>(true,&None,body)}
  RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let g=effect.grant();let d=self.close_demands(body)?;
  if !retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if self.$slot.is_closing(){self.$slot.close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.begin_close_into(effect,reply.progress())}}}}
  relay!(text);relay!(list);relay!(map);
  if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply)}
  macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}}}
  close!(text_out);close!(list_out);close!(map_out);close!(output);
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
 }
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if self.$slot.is_closing(){self.$slot.recover_close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.recover_begin_close_into(effect,reply.progress())}}}}
  relay!(text);relay!(list);relay!(map);
  if !self.retirement.is_empty(){return self.retirement.recover_step_into(body,effect,reply)}
  if self.text_out.is_some()||self.list_out.is_some()||self.map_out.is_some()||self.output.is_some(){return Ok(reply.pending())}
  self.close_step(body,effect,reply)
 }
 fn requires_original_birth(&self)->bool{open_slot_ref!(self,slot=>slot.requires_original_birth(),self.retirement.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{
  use RetainedWorkPoll::Complete;
  if grant.maximum_items==0||grant.maximum_depth==0{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  open_slot!(self,slot=>return slot.close_original_birth_step(original,child_grant(grant,0)),());
  if self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}
  Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native UI value has no original birth to settle").into()),work:0})
 }
 fn requires_original_control(&self)->bool{open_slot_ref!(self,slot=>slot.requires_original_control(),self.retirement.requires_original_control())}
 fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if self.requires_original_birth(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect"))}
  let g=effect.grant();
  if !self.text.terminal_is_empty()||!self.list.terminal_is_empty()||!self.map.terminal_is_empty(){
   let d=self.close_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if !self.$slot.is_closing(){self.$slot.begin_close_into(effect,reply.progress())}else if self.$slot.requires_original_control(){self.$slot.close_original_into(original,body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}}}}
   relay!(text);relay!(list);relay!(map);
  }
  if self.retirement.requires_original_control(){return self.retirement.step_original_into(original,body,effect,reply)}
  self.close_step(body,effect,reply)
 }
 fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if !self.$slot.is_closing(){self.$slot.recover_begin_close_into(effect,reply.progress())}else if self.$slot.requires_original_control(){self.$slot.recover_close_original_into(original,body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.recover_close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}}}}
  relay!(text);relay!(list);relay!(map);
  if self.retirement.requires_original_control(){return self.retirement.recover_step_original_into(original,body,effect,reply)}
  self.recover_close_step(body,effect,reply)
 }
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.text.terminal_is_empty()&&self.list.terminal_is_empty()&&self.map.terminal_is_empty()&&self.text_out.is_none()&&self.list_out.is_none()&&self.map_out.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized> Drop for UiValueNativeReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native recursive UI receiving retains every original descendant until funded closure")}}
macro_rules! empty_default{($t:ty,$value:expr)=>{
 impl NativeDefaultValue for $t{
  fn default_demand()->RetirementDemand{fixed(size_of::<Option<Self>>())}
  fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
  fn admit_default<'reply>(out:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if out.is_some(){return Ok(reply.complete(Err(invalid().into())))}let d=Self::default_demand();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"UI default exceeded its work authority").into())))}*out=Some($value);Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
  fn recover_default<'reply>(_:&mut Option<Self>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 }
 impl NativeDefault for $t{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
}}
empty_default!(UiValue,UiValue::Null);empty_default!(UiList,UiList::default());empty_default!(UiMap,UiMap::default());
