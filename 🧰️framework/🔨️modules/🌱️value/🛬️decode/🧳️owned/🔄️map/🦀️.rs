//! 🔄️ Authored wrappers retain the genuine child handoff before a separately funded ownership mapping.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,OriginalReceivingPrimary,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use super::{NativeReceive,NativeReceiver,NativeAdvanceSelection,NativeDecodeControl,RetainedCloneRef,tree::{fixed,receipt,invalid,NativeValueSource}};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete,owned::RetainedCursorRelease};
use std::{marker::PhantomData,mem::ManuallyDrop};
pub trait NativeReceiveMap<T>:RetireOwned{fn from_native(value:T)->Self;}
pub struct NativeMappedReceiver<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>{inner:crate::owned::NativeReceiveSlot<I,T>,binding:Option<crate::retained_clone::RetainedCloneBinding>,pending:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<U>>,retirement:RetainedCloneClose,closing:bool,marker:PhantomData<fn(&I)->U>}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"native mapped receiving depth overflow"))?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>> NativeMappedReceiver<I,T,U>{pub fn new()->Self{Self{inner:Default::default(),binding:None,pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,marker:PhantomData}} pub fn original_record_subset()->Option<Self>{Some(Self{inner:{T::record_subset_receiver()?;crate::owned::NativeReceiveSlot::original_record_subset()},binding:None,pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,marker:PhantomData})}
 #[cfg(any(test,feature="testkit"))]pub fn child_is_closing(&self)->bool{self.inner.is_closing()}
}
semio_framework_value_native_receiver_protocol::native_mapped_receiver_engine!();
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>Drop for NativeMappedReceiver<I,T,U>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native mapped receiving requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.output);}}}}
#[path="📦️box/🦀️.rs"]mod boxed;pub use boxed::NativeBoxReceiver;

/// 🧪️ Validates resident original fields without allocating before their funded transfer.
pub trait NativeReceiveTryMap<T>:RetireOwned{
 fn validate_native(value:&T)->Result<(),ValueError>;
 fn from_valid_native(value:T)->Self;
}
/// 🧳️ Holds the real child, validation refusal and published output across independent turns.
pub struct NativeTryMappedReceiver<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>>{
 inner:crate::owned::NativeReceiveSlot<I,T>,binding:Option<crate::retained_clone::RetainedCloneBinding>,pending:ManuallyDrop<Option<T>>,error:ManuallyDrop<Option<ValueError>>,output:ManuallyDrop<Option<U>>,retirement:RetainedCloneClose,validated:bool,failed:bool,closing:bool,marker:PhantomData<fn(&I)->U>
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> NativeTryMappedReceiver<I,T,U>{
 pub fn new()->Self{Self{inner:Default::default(),binding:None,pending:ManuallyDrop::new(None),error:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),validated:false,failed:false,closing:false,marker:PhantomData}}
 #[cfg(any(test,feature="testkit"))]pub fn validation_refusal(&self)->Option<&ValueError>{self.error.as_ref()}
 #[cfg(any(test,feature="testkit"))]pub fn retained_input(&self)->Option<&T>{self.pending.as_ref()}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> NativeTryMappedReceiver<I,T,U>{
 fn close_dispatch<'reply>(&mut self,mut original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  if original.is_none()&&self.requires_original_control(){return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped child requires its original close port").into())))}
  if original.is_some()&&self.requires_original_birth(){return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into())))}
  let g=effect.grant();let d=match self.close_demands(body){Ok(d)=>d,Err(error)=>return Ok(reply.complete(Err(error.into())))};if !retained_clone_grant_admits(g,d){return semio_framework_value_native_receiver_protocol::native_close_denied(reply)}
  if !self.inner.terminal_is_empty(){let control=self.inner.requires_original_control();let mut nested=effect.nested(child(g));let reply=reply.progress();return match(original.as_deref_mut().filter(|_|control),recover){(Some(original),false)=>self.inner.close_original_into(original,body,&mut nested,reply),(Some(original),true)=>self.inner.recover_close_original_into(original,body,&mut nested,reply),(None,false)=>self.inner.close_into(body,&mut nested,reply),(None,true)=>self.inner.recover_close_into(body,&mut nested,reply)}}
  if !self.retirement.is_empty(){let control=self.retirement.requires_original_control();let reply=reply.progress();return match(original.as_deref_mut().filter(|_|control),recover){(Some(original),false)=>self.retirement.step_original_into(original,body,effect,reply),(Some(original),true)=>self.retirement.recover_step_original_into(original,body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply)}}
  if self.pending.is_none()&&self.error.is_none()&&self.output.is_none()&&self.binding.is_some(){return if recover{crate::retained_clone::RetainedCloneBinding::recover_close_into(&mut self.binding,body,effect,reply)}else{crate::retained_clone::RetainedCloneBinding::close_into(&mut self.binding,body,effect,reply)}}
  if recover{return Err(invalid())}
  let turn=if self.pending.is_some(){self.retirement.begin_granted(&mut self.pending,effect)}else if self.error.is_some(){self.retirement.begin_granted(&mut self.error,effect)}else{self.retirement.begin_granted(&mut self.output,effect)};
  semio_framework_value_native_receiver_protocol::native_close_begun(turn,reply)
 }
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> NativeReceiver<I,U> for NativeTryMappedReceiver<I,T,U>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.output.is_some()||self.failed{return Err(invalid())}input.check_binding(&self.binding)?;
  if self.binding.is_none(){return Ok(fixed(input.binding_copy_bytes()))}
  if self.error.is_some(){return Ok(fixed(std::mem::size_of::<ValueError>()))}
  if self.pending.is_some(){return Ok(if self.validated{fixed(std::mem::size_of::<T>().saturating_add(std::mem::size_of::<U>()))}else{fixed(0)})}
  if self.inner.ready(){return nested(self.inner.take_demand()?)}nested(self.inner.demands(input,body,maximum_work_units)?)
 }
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.output.is_some()||self.failed{return Err(invalid())}input.check_binding(&self.binding)?;
  if self.binding.is_none(){return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  if self.error.is_some()||self.pending.is_some(){return Ok(NativeAdvanceSelection::Local{work:1})}
  if self.inner.ready(){return Ok(NativeAdvanceSelection::Local{work:self.inner.take_work_demand()?})}Ok(NativeAdvanceSelection::Nested)
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{
  if self.closing||self.output.is_some()||self.failed||self.binding.is_none()||self.error.is_some()||self.pending.is_some()||self.inner.ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  self.inner.advance(input,control,witness,body,maximum_work_units,child(g)).map(|s|RetainedCloneStep::Progress(s.progress()))
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  macro_rules! perform{()=>{if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native validated action exceeded its original work").into())}}}
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   if self.error.is_some(){perform!();let error=self.error.take().ok_or_else(||ValueError::invariant("validation refusal absent"))?;self.failed=true;let progress=error.retained_progress().saturating_add(receipt(d));return Err(error.with_retained_progress(progress).into())}
   if self.pending.is_some(){perform!();if !self.validated{match U::validate_native(self.pending.as_ref().ok_or_else(||ValueError::invariant("validation input absent"))?){Ok(())=>self.validated=true,Err(error)=>*self.error=Some(error)}return Ok(Some(RetainedCloneStep::Progress(receipt(d))))}let value=self.pending.take().ok_or_else(||ValueError::invariant("validated input absent"))?;*self.output=Some(U::from_valid_native(value));return Ok(Some(RetainedCloneStep::Complete(receipt(d))))}
   if self.inner.ready(){return match self.inner.take(&mut *self.pending,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}}
   Err(invalid().into())})();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output!(U,invalid);
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return nested(self.inner.close_demands(body)?)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.error.is_some(){return self.retirement.next_owner_demands::<ValueError>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<U>(true,&None,body)}if self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.error.is_some(){return self.retirement.next_owner_work::<ValueError>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<U>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(&self.binding,body)}

 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,true)}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,true)}
 fn advance_pending(&self)->bool{self.inner.advance_pending()}
 fn requires_original_birth(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty()&&self.inner.requires_original_birth(){return self.inner.close_original_birth_step(original,child(grant))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped owner has no original birth to settle").into()),work:0})}
 fn requires_original_control(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_control()}self.retirement.requires_original_control()}
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
  fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()&&self.pending.is_none()&&self.error.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> Drop for NativeTryMappedReceiver<I,T,U>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native validated receiving requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.error);ManuallyDrop::drop(&mut self.output);}}}}
