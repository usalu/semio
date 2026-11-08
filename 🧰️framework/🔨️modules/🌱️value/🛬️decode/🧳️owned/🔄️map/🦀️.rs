//! 🔄️ Authored wrappers retain the genuine child handoff before a separately funded ownership mapping.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,OriginalReceivingPrimary,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use super::{NativeReceive,NativeReceiver,NativeAdvanceSelection,NativeDecodeControl,RetainedCloneRef,tree::{fixed,receipt,invalid}};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete};
use std::{marker::PhantomData,mem::ManuallyDrop};
pub trait NativeReceiveMap<T>:RetireOwned{fn from_native(value:T)->Self;}
pub struct NativeMappedReceiver<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>{inner:crate::owned::NativeReceiveSlot<I,T>,binding:Option<crate::retained_clone::RetainedCloneBinding>,pending:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<U>>,retirement:RetainedCloneClose,closing:bool,marker:PhantomData<fn(&I)->U>}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"native mapped receiving depth overflow"))?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>> NativeMappedReceiver<I,T,U>{pub fn new()->Self{Self{inner:Default::default(),binding:None,pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,marker:PhantomData}} pub fn original_record_subset()->Option<Self>{Some(Self{inner:{T::record_subset_receiver()?;crate::owned::NativeReceiveSlot::original_record_subset()},binding:None,pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,marker:PhantomData})}}
semio_framework_value_native_receiver_protocol::native_mapped_receiver_engine!();
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>Drop for NativeMappedReceiver<I,T,U>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native mapped receiving requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.output);}}}}
#[path="📦️box/🦀️.rs"]mod boxed;pub use boxed::NativeBoxReceiver;

/// 🧪️ Validates resident original fields without allocating before their funded transfer.
pub trait NativeReceiveTryMap<T>:RetireOwned{
 fn validate_native(value:&T)->Result<(),ValueError>;
 fn from_valid_native(value:T)->Self;
}
/// 🧳️ Holds the real child, validation refusal and published output across independent turns.
pub struct NativeTryMappedReceiver<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>>{
 inner:crate::owned::NativeReceiveSlot<I,T>,binding:Option<crate::retained_clone::RetainedCloneBinding>,pending:ManuallyDrop<Option<T>>,error:ManuallyDrop<Option<ValueError>>,output:ManuallyDrop<Option<U>>,retirement:RetainedCloneClose,validated:bool,failed:bool,closing:bool,marker:PhantomData<fn(&I)->U>
}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> NativeTryMappedReceiver<I,T,U>{
 pub fn new()->Self{Self{inner:Default::default(),binding:None,pending:ManuallyDrop::new(None),error:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),validated:false,failed:false,closing:false,marker:PhantomData}}
 #[cfg(test)]pub(crate) fn validation_refusal(&self)->Option<&ValueError>{self.error.as_ref()}
 #[cfg(test)]pub(crate) fn retained_input(&self)->Option<&T>{self.pending.as_ref()}
}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> NativeReceiver<I,U> for NativeTryMappedReceiver<I,T,U>{
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
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  if self.closing||self.output.is_some()||self.failed||self.binding.is_none()||self.error.is_some()||self.pending.is_some()||self.inner.ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  self.inner.advance(input,control,body,maximum_work_units,child(g)).map(|s|RetainedCloneStep::Progress(s.progress()))
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
 fn begin_close(&mut self){self.closing=true;self.inner.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return nested(self.inner.close_demands(body)?)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.error.is_some(){return self.retirement.next_owner_demands::<ValueError>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<U>(true,&None,body)}if self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.error.is_some(){return self.retirement.next_owner_work::<ValueError>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<U>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;if self.requires_original_control(){return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped child requires its original close port").into()),work:0})}if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty(){return self.inner.close_step(&mut effect.nested(child(g))).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|step|RetainedCloneStep::Progress(step.progress()))}if self.pending.is_none()&&self.error.is_none()&&self.output.is_none()&&self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_one(&mut self.binding,effect)}let turn=if self.pending.is_some(){self.retirement.begin_granted(&mut self.pending,effect)}else if self.error.is_some(){self.retirement.begin_granted(&mut self.error,effect)}else{self.retirement.begin_granted(&mut self.output,effect)};Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}

 fn requires_original_birth(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty()&&self.inner.requires_original_birth(){return self.inner.close_original_birth_step(original,child(grant))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped owner has no original birth to settle").into()),work:0})}
 fn requires_original_control(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into()),work:0})}let g=effect.grant();if !self.inner.terminal_is_empty()&&self.inner.requires_original_control(){let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}return self.inner.close_original_step(original,&mut effect.nested(child(g)))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()&&self.pending.is_none()&&self.error.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveTryMap<T>> Drop for NativeTryMappedReceiver<I,T,U>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native validated receiving requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.error);ManuallyDrop::drop(&mut self.output);}}}}
