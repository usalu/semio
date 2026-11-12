//! 🎯️ Each authored action selects its actual leaf receiver while retaining the original argument source.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value::{DslValue,NativeDecodeControl,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retained_clone::{RetainedCloneSource,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnWitness},retirement::{RetirementCursor,RetirementStep}};
use semio_framework_value_native_owned::{NativeReceive,native_decoding::{OriginalReceivingPrimary,NativeReceivingFailure}};
use std::{marker::PhantomData,mem::size_of};
/// 🏭️ Static authored leaf authority quotes and constructs only its own concrete receiving frame.
pub struct NativeActionFactory<M>{pub birth:fn()->RetirementDemand,pub admit:fn(RetainedCloneGrant)->Result<(Box<dyn NativeActionReceiving<M>>,RetainedCloneProgress),ValueError>}
impl<M> Copy for NativeActionFactory<M>{}
impl<M> Clone for NativeActionFactory<M>{fn clone(&self)->Self{*self}}
/// 🧬️ Typed action custody preserves the same source until receiving and every initialized child close; every receiving and close action runs inside its caller's original effect.
pub trait NativeActionReceiving<M>:Send{
 fn capture_demands(&self)->Result<RetirementDemand,ValueError>;
 fn capture(&mut self,source:&mut Option<RetainedCloneSource<DslValue>>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn receiver_birth_demands(&self)->Result<RetirementDemand,ValueError>;
 fn begin_receiver(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn advance_work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>;
 fn ready(&self)->bool;
 fn take_demands(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take(&mut self,output:&mut Option<M>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>;
 fn is_closing(&self)->bool;
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn begin_close_work_demand(&self)->Result<u64,ValueError>;
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn requires_original_control(&self)->bool;
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll;
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>;
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>;
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>;
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn release_demands(&self)->RetirementDemand;
}
struct Leaf<L:NativeReceive<DslValue>,M>{inner:Option<NativeReceivingOriginal<DslValue,L>>,staged:Option<L>,marker:PhantomData<fn()->M>}
fn invalid()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native action leaf ownership phase changed")}
impl<M:'static> NativeActionFactory<M>{
 /// 🌱️ A concrete leaf's receiver remains unborn until its separately funded original-source capture.
 pub const fn of<L:NativeReceive<DslValue>+Into<M>+'static>()->Self{Self{birth:birth::<L,M>,admit:admit::<L,M>}}
}
fn birth<L:NativeReceive<DslValue>,M>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Leaf<L,M>>(),capacity_bytes:size_of::<Leaf<L,M>>(),depth:1,..Default::default()}}
fn admit<L:NativeReceive<DslValue>+Into<M>+'static,M:'static>(g:RetainedCloneGrant)->Result<(Box<dyn NativeActionReceiving<M>>,RetainedCloneProgress),ValueError>{let d=birth::<L,M>();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Err(invalid())}Ok((Box::new(Leaf::<L,M>{inner:None,staged:None,marker:PhantomData}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
impl<L:NativeReceive<DslValue>+Into<M>+'static,M:'static> NativeActionReceiving<M> for Leaf<L,M>{
 fn capture_demands(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<NativeReceivingOriginal<DslValue,L>>(),depth:1,..Default::default()})}
 fn capture(&mut self,source:&mut Option<RetainedCloneSource<DslValue>>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.capture_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}let original=source.take().ok_or_else(invalid)?;self.inner=Some(NativeReceivingOriginal::new(original));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
 fn receiver_birth_demands(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn begin_receiver(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{Ok(Default::default())}
 fn demands(&self,body:usize,m:u64)->Result<RetirementDemand,ValueError>{self.inner.as_ref().ok_or_else(invalid)?.demands(body,m)}
 fn advance_work_demand(&self,body:usize,m:u64)->Result<u64,ValueError>{self.inner.as_ref().ok_or_else(invalid)?.advance_work_demand(body,m)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,m:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{match self.inner.as_mut(){Some(inner)=>inner.advance(control,witness,body,m,g),None=>RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn ready(&self)->bool{self.inner.as_ref().is_some_and(|inner|inner.output_ready())}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{let mut d=self.inner.as_ref().ok_or_else(invalid)?.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<M>()).ok_or_else(invalid)?;Ok(d)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.inner.as_ref().ok_or_else(invalid)?.take_work_demand()}
 fn take(&mut self,output:&mut Option<M>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let Some(inner)=self.inner.as_mut()else{return RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(invalid().into()),work:0})};
  let poll=inner.take(&mut self.staged,effect);
  if let Some(value)=self.staged.take(){*output=Some(value.into());}
  poll
 }
 fn is_closing(&self)->bool{self.inner.as_ref().is_some_and(|inner|inner.is_closing())}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{match self.inner.as_ref(){Some(inner)=>inner.begin_close_demand(),None=>Ok(Default::default())}}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{match self.inner.as_ref(){Some(inner)=>inner.begin_close_work_demand(),None=>Ok(0)}}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{match self.inner.as_mut(){Some(inner)=>inner.begin_close_into(effect,reply),None=>Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.inner.as_ref(){Some(inner)=>inner.close_demands(body),None=>Ok(Default::default())}}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{match self.inner.as_ref(){Some(inner)=>inner.close_work_demand(body),None=>Ok(0)}}
 fn requires_original_birth(&self)->bool{self.inner.as_ref().is_some_and(|inner|inner.requires_original_birth())}
 fn requires_original_control(&self)->bool{self.inner.as_ref().is_some_and(|inner|inner.requires_original_control())}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{match self.inner.as_mut(){Some(inner)=>inner.close_original_birth_step(original,grant),None=>RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{match self.inner.as_mut(){Some(inner)=>RetirementCursor::close_into(inner,body,effect,reply),None=>Ok(reply.complete(Ok(RetirementStep::Complete(Default::default()))))}}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{match self.inner.as_mut(){Some(inner)=>RetirementCursor::recover_close_into(inner,body,effect,reply),None=>Err(invalid())}}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{match self.inner.as_mut(){Some(inner)=>RetirementCursor::close_original_into(inner,original,body,effect,reply),None=>Ok(reply.complete(Ok(RetirementStep::Complete(Default::default()))))}}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{match self.inner.as_mut(){Some(inner)=>RetirementCursor::recover_close_original_into(inner,original,body,effect,reply),None=>Err(invalid())}}
 fn terminal_is_empty(&self)->bool{self.staged.is_none()&&self.inner.as_ref().is_none_or(|inner|inner.terminal_is_empty())}
 fn release_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<L:NativeReceive<DslValue>,M> Drop for Leaf<L,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.inner.as_ref().is_none_or(|inner|inner.terminal_is_empty()),"native action leaf requires original funded closure")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
