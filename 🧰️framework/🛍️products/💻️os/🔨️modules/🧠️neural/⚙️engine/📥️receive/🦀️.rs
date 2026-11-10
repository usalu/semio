//! 📥️ Native receiving of the untagged neural vocabulary: an object is a dictionary, any other scalar is an atom, with no intermediate wire owner.
use super::*;
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,retained_clone::{RetainedCloneBinding,RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceive,NativeReceiver,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary,NativeReceivingFailure,owned::{OriginalSourceTurnWitness,native_receiver_close_start,NativeAdvanceSelection,NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,NativeMappedReceiver,NativeReceiveMap,tree::{NativeReceiveSlot,NativeScalarReceiver,NativeScalarValue,NativeValueSource,add_receipt,child_grant,fixed,invalid,nested,receipt}}};
use std::mem::size_of;

/// ⚛️ A scalar source selects its atom kind by its actual original type; integers beyond `i64` are refused.
impl<I:NativeValueSource+?Sized> NativeScalarValue<I> for Atom{
 fn read(input:&I)->Result<Self,ValueError>{
  if input.is_null(){return Ok(Self::Null)}
  if let Ok(value)=input.boolean(){return Ok(Self::Boolean(value))}
  if let Ok(number)=input.number(){return match number{Number::Int(value)=>Ok(Self::Integer(value)),Number::UInt(value)=>i64::try_from(value).map(Self::Integer).map_err(|_|invalid()),Number::Float(value)=>Ok(Self::Decimal(value))}}
  input.text().map(|text|Self::String(text.clone()))
 }
 fn read_copy_bytes(input:&I)->Result<usize,ValueError>{Ok(size_of::<Self>()+input.text().map_or(0,|text|text.len()))}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Atom{
 type Receiver=NativeScalarReceiver<I,Self>;
 fn receiver()->Self::Receiver{NativeScalarReceiver::new()}
}

/// 📚️ A dictionary adopts its completed ordered map of values without rebuilding it.
impl NativeReceiveMap<OrderedMap<Value>> for Dictionary{fn from_native(pairs:OrderedMap<Value>)->Self{Self{pairs}}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Dictionary{
 type Receiver=NativeMappedReceiver<I,OrderedMap<Value>,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
impl NativeDefaultValue for Dictionary{
 fn default_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if output.is_some(){return Ok(reply.complete(Err(invalid().into())))}
  let demand=Self::default_demand();if !retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"neural dictionary default exceeded its work authority").into())))}
  *output=Some(Self::new());Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(demand)))))
 }
 fn recover_default<'reply>(_:&mut Option<Self>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
}
impl NativeDefault for Dictionary{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}


/// 💎️ The untagged value chooses its dictionary or atom child from the original source shape before any child work starts.
pub struct NativeValueReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,atom:NativeReceiveSlot<I,Atom>,dictionary:NativeReceiveSlot<I,Dictionary>,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Value{
 type Receiver=NativeValueReceiver<I>;
 fn receiver()->Self::Receiver{NativeValueReceiver{binding:None,atom:Default::default(),dictionary:Default::default(),phase:0,closing:false}}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,Value> for NativeValueReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=5{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  let inner=body.saturating_sub(size_of::<u8>());
  match self.phase{0=>Ok(fixed(size_of::<u8>())),1=>nested(self.atom.demands(input,inner,maximum_work_units)?,size_of::<u8>()),2=>nested(self.dictionary.demands(input,inner,maximum_work_units)?,size_of::<u8>()),_=>Err(invalid())}
 }
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  let bound=input.check_binding(&self.binding)?;if self.closing||self.phase>=3{return Err(invalid())}
  if !bound{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  Ok(if self.phase==0{NativeAdvanceSelection::Local{work:1}}else{NativeAdvanceSelection::Nested})
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  let inner=body.saturating_sub(size_of::<u8>());let child=child_grant(grant,size_of::<u8>());
  let poll=match self.phase{1=>self.atom.advance(input,control,witness,inner,maximum_work_units,child),2=>self.dictionary.advance(input,control,witness,inner,maximum_work_units,child),_=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})};
  match poll{
   RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},
   RetainedWorkPoll::Complete(turn)=>{
    let work=turn.work;
    let result=turn.result.and_then(|step|{
     if self.phase==1&&self.atom.ready(){self.phase=3}else if self.phase==2&&self.dictionary.ready(){self.phase=4}
     Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),size_of::<u8>())?))
    });
    RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})
   }
  }
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{
   let demand=self.demands(input,body,effect.maximum_work_units())?;
   if !retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(Into::into)}
   if !effect.performed(1)?{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native value action exceeded its original work").into())}
   self.phase=if input.get().record_len().is_ok(){2}else{1};
   Ok(RetainedCloneStep::Progress(receipt(demand)))
  })();
  Ok(reply.complete(result))
 }
 fn output_ready(&self)->bool{!self.closing&&matches!(self.phase,3|4)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(2*size_of::<Value>()+size_of::<u8>()))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;let child=if self.phase==3{self.atom.take_work_demand()?}else{self.dictionary.take_work_demand()?};child.checked_add(1).ok_or_else(||ValueError::invariant("native value publication work overflow"))}
 fn take<'reply>(&mut self,output:&mut Option<Value>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let turn=(||->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
   let refuse=|error:ValueError,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work});
   let (demand,quote)=match (self.take_demand(),self.take_work_demand()){(Ok(demand),Ok(quote))=>(demand,quote),(Err(error),_)|(_,Err(error))=>return refuse(error,0)};
   if output.is_some(){return refuse(invalid(),0)}
   if !retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()<quote{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
   let value=if self.phase==3{
    let mut taken=None;
    match self.atom.take(&mut taken,effect){RetainedWorkPoll::Pending{work}=>return RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>if let Err(error)=turn.result{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:turn.work})}}
    taken.map(Value::Atom)
   }else{
    let mut taken=None;
    match self.dictionary.take(&mut taken,effect){RetainedWorkPoll::Pending{work}=>return RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>if let Err(error)=turn.result{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:turn.work})}}
    taken.map(Value::Dictionary)
   };
   let Some(value)=value else{return refuse(invalid(),effect.performed_work())};
   *output=Some(value);self.phase=5;
   match effect.performed(1){Ok(true)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(demand))),work:effect.performed_work()}),Ok(false)=>refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority"),effect.performed_work()),Err(error)=>refuse(error,effect.performed_work())}
  })();
  Ok(match turn{RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
 }
 fn advance_pending(&self)->bool{self.atom.advance_pending()||self.dictionary.advance_pending()}
 native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if !self.atom.terminal_is_empty(){return nested(if self.atom.is_closing(){self.atom.close_demands(body)?}else{self.atom.begin_close_demand()?},0)}
  if !self.dictionary.terminal_is_empty(){return nested(if self.dictionary.is_closing(){self.dictionary.close_demands(body)?}else{self.dictionary.begin_close_demand()?},0)}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.atom.terminal_is_empty(){return if self.atom.is_closing(){self.atom.close_work_demand(body)}else{self.atom.begin_close_work_demand()}}if !self.dictionary.terminal_is_empty(){return if self.dictionary.is_closing(){self.dictionary.close_work_demand(body)}else{self.dictionary.begin_close_work_demand()}}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let demand=self.close_demands(body)?;
  if !retained_clone_grant_admits(g,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if self.$slot.is_closing(){self.$slot.close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.begin_close_into(effect,reply.progress())}}}}
  relay!(atom);relay!(dictionary);
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
 }
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty(){return if self.$slot.is_closing(){self.$slot.recover_close_into(body,&mut effect.nested(child_grant(g,0)),reply.progress())}else{self.$slot.recover_begin_close_into(effect,reply.progress())}}}}
  relay!(atom);relay!(dictionary);
  Ok(reply.pending())
 }
 fn requires_original_birth(&self)->bool{if !self.atom.terminal_is_empty(){return self.atom.requires_original_birth()}!self.dictionary.terminal_is_empty()&&self.dictionary.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{
  if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.atom.terminal_is_empty(){return self.atom.close_original_birth_step(original,child_grant(grant,0))}
  if !self.dictionary.terminal_is_empty(){return self.dictionary.close_original_birth_step(original,child_grant(grant,0))}
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native value receiver has no original birth to settle").into()),work:0})
 }
 fn requires_original_control(&self)->bool{if !self.atom.terminal_is_empty(){return self.atom.requires_original_control()}!self.dictionary.terminal_is_empty()&&self.dictionary.requires_original_control()}
 fn close_original_step<'reply>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  if g.maximum_items==0||g.maximum_depth==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty()&&self.$slot.is_closing()&&self.$slot.requires_original_control(){return self.$slot.close_original_into(original,body,&mut effect.nested(child_grant(g,0)),reply.progress())}}}
  relay!(atom);relay!(dictionary);
  self.close_step(body,effect,reply)
 }
 fn recover_close_original_step<'reply>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{
  let g=effect.grant();
  macro_rules! relay{($slot:ident)=>{if !self.$slot.terminal_is_empty()&&self.$slot.is_closing()&&self.$slot.requires_original_control(){return self.$slot.recover_close_original_into(original,body,&mut effect.nested(child_grant(g,0)),reply.progress())}}}
  relay!(atom);relay!(dictionary);
  self.recover_close_step(body,effect,reply)
 }
 fn terminal_is_empty(&self)->bool{self.atom.terminal_is_empty()&&self.dictionary.terminal_is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeValueReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native value receiving requires funded closure")}}
