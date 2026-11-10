//! 🧭️ The original sealed owner records a lifetime-bound projection before publishing it.
use super::*;
use semio_framework_value_receiving::span::effect::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};
use crate::{RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand};
use std::{marker::PhantomData,mem::{ManuallyDrop,size_of},ptr::NonNull};

/// 🫴️ Deposits only a reference borrowed from the original resident owner.
pub struct OriginalProjectionReply<'reply,'source,T:?Sized>{inner:OriginalEffectReply<'reply,NonNull<T>,ValueError>,source:PhantomData<&'source T>}
impl<'reply,'source,T:?Sized> OriginalProjectionReply<'reply,'source,T>{
 fn new(inner:OriginalEffectReply<'reply,NonNull<T>,ValueError>,_:&'source T)->Self{Self{inner,source:PhantomData}}
 /// ✅️ Records the actual projected reference or its original owned refusal.
 pub fn complete(self,result:Result<&'source T,ValueError>)->OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>{self.inner.complete(result.map(NonNull::from))}
 /// ⏳️ Retains the original action while its producer has no reply.
 pub fn pending(self)->OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>{self.inner.pending()}
}
/// 🌿️ Supplies and recovers a projection directly from the actual sealed owner.
pub trait OriginalSourceProjection<T:Sync+'static>:RetireOwned+Sync{
 /// ⏱️ Quotes the selected projection producer's actual bounded work.
 fn projection_work_demand(&self)->Result<u64,ValueError>;
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,T>)->Result<OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>,ValueError>;
 fn recover_source_projection<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,T>)->Result<OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>,ValueError>;
}
impl<T:RetireOwned+Sync+'static> OriginalSourceProjection<T> for T{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,T>)->Result<OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"source projection exceeded its original work"))))}Ok(reply.complete(Ok(self)))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,T>)->Result<OriginalEffectReplyComplete<'reply,NonNull<T>,ValueError>,ValueError>{Ok(reply.pending())}
}
/// 🏠️ Retains the actual source and inner answer throughout the common recorded action.
pub struct OriginalProjectionProducer<T:Sync+'static,A:OriginalSourceProjection<T>>{owner:ManuallyDrop<Option<SealedShared<A>>>,answer:ManuallyDrop<Option<Result<NonNull<T>,ValueError>>>}
/// 🪪️ A context created only by its resident projection frame.
pub struct OriginalProjectionContext{private:()}
impl<T:Sync+'static,A:OriginalSourceProjection<T>> OriginalProjectionProducer<T,A>{
 pub(crate) fn new(owner:SealedShared<A>)->Self{Self{owner:ManuallyDrop::new(Some(owner)),answer:ManuallyDrop::new(None)}}
 pub(crate) fn context(&self)->OriginalProjectionContext{OriginalProjectionContext{private:()}}
 pub(crate) fn demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Result<NonNull<T>,ValueError>>>()+size_of::<Option<Result<RetainedCloneStep,ValueError>>>(),depth:1,..Default::default()}}
 pub(crate) fn work_demand(&self)->Result<u64,ValueError>{self.owner.as_ref().ok_or_else(||ValueError::invariant("original projection owner absent"))?.get().projection_work_demand()?.checked_add(1).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original projection work overflow"))}
 pub(crate) fn transfer_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<A>>>()+2*size_of::<Option<Result<NonNull<T>,ValueError>>>(),depth:1,..Default::default()}}
 pub(crate) fn transfer(&mut self,owner:&mut Option<SealedShared<A>>,answer:&mut Option<Result<NonNull<T>,ValueError>>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{if owner.is_some()||answer.is_some()||self.owner.is_none()||self.answer.is_none(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original projection transfer places changed")),work:0}}let demand=Self::transfer_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if remaining<2{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let before=effect.performed_work();let performed=effect.performed(2);let work=effect.performed_work()-before;match performed{Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"source projection transfer exceeded its original work")),work},Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work}}*owner=self.owner.take();*answer=self.answer.take();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work}}
 pub(crate) fn return_owner_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<A>>>(),depth:1,..Default::default()}}
 pub(crate) fn return_owner(&mut self,owner:&mut Option<SealedShared<A>>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{if owner.is_some()||self.owner.is_none()||self.answer.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original unanswered projection owner places changed")),work:0}}let demand=Self::return_owner_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if remaining<1{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let before=effect.performed_work();let performed=effect.performed(1);let work=effect.performed_work()-before;match performed{Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"source projection owner return exceeded its original work")),work},Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work}}*owner=self.owner.take();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work}}
 fn record<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{if self.answer.is_none(){return Ok(reply.pending())}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"source projection reply exceeded its original work"))))}Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:Self::demand().copy_bytes,..Default::default()}))))}
}
impl<T:Sync+'static,A:OriginalSourceProjection<T>> OriginalEffectProducer for OriginalProjectionProducer<T,A>{
 type Output=RetainedCloneStep;type Failure=ValueError;type Context<'turn,'source>=OriginalProjectionContext where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if self.owner.is_some(){Ok(())}else{Err(ValueError::invariant("original projection owner absent"))}}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{if self.answer.is_some(){return Err(ValueError::invariant("original projection answer already recorded"))}let source=self.owner.as_ref().ok_or_else(||ValueError::invariant("original projection owner absent"))?.get();let inner=OriginalEffectReply::borrow(&mut self.answer)?;let token=OriginalProjectionReply{inner,source:PhantomData};drop(source.project_source(effect,token)?);self.record(effect,reply)}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{if self.answer.is_none(){let source=self.owner.as_ref().ok_or_else(||ValueError::invariant("original projection owner absent"))?.get();let inner=OriginalEffectReply::borrow(&mut self.answer)?;let token=OriginalProjectionReply{inner,source:PhantomData};drop(source.recover_source_projection(effect,token)?)}self.record(effect,reply)}
}

/// 🧭️ Selects only the actual retained projection producer state.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalProjectionFramePhase{Drive,TransferProducer,AcceptAnswer,ReturnOwner,Finish}
/// 🧳️ Keeps the producer in its original action authority until paid transfer after settlement.
pub struct OriginalProjectionFrame<T:Sync+'static,A:OriginalSourceProjection<T>>{id:u64,authority:Option<OriginalEffectAuthority<OriginalProjectionProducer<T,A>>>,producer:Option<OriginalProjectionProducer<T,A>>}
unsafe impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> Send for OriginalProjectionFrame<T,A>{}
/// 🧷️ Shared frame queries inspect frozen action metadata, typed immutable answers and the sealed Sync authority; all mutation requires an exclusive frame loan.
unsafe impl<T:Sync+'static,A:OriginalSourceProjection<T>> Sync for OriginalProjectionFrame<T,A>{}
impl<T:Sync+'static,A:OriginalSourceProjection<T>> OriginalProjectionFrame<T,A>{
 pub(crate) fn new(id:u64,owner:SealedShared<A>)->Self{Self{id,authority:Some(OriginalEffectAuthority::new(OriginalProjectionProducer::new(owner))),producer:None}}
 /// 📐️ Quotes the real Source projection and recorded reply.
 pub fn projection_demand(&self)->RetirementDemand{OriginalProjectionProducer::<T,A>::demand()}
 /// ⏱️ Quotes the exact selected Source producer work.
 pub fn projection_work_demand(&self)->Result<u64,ValueError>{self.authority.as_ref().ok_or_else(||ValueError::invariant("original projection authority absent"))?.producer().work_demand()}
 /// 🧭️ Observes the actual action and retained result without admitting a turn.
 pub fn phase(&self)->Result<OriginalProjectionFramePhase,ValueError>{if let Some(authority)=self.authority.as_ref(){if authority.has_pending_action()||authority.producer().answer.is_none(){return Ok(OriginalProjectionFramePhase::Drive)}return Ok(OriginalProjectionFramePhase::TransferProducer)}if let Some(producer)=self.producer.as_ref(){if producer.owner.is_none()&&producer.answer.is_none(){return Ok(OriginalProjectionFramePhase::Finish)}if producer.owner.is_some()&&producer.answer.is_some(){return Ok(OriginalProjectionFramePhase::AcceptAnswer)}if producer.owner.is_some()&&producer.answer.is_none(){return Ok(OriginalProjectionFramePhase::ReturnOwner)}}Err(ValueError::invariant("original projection frame state changed"))}
 /// ⏳️ Observes the real unresolved producer action.
 pub fn has_pending_action(&self)->bool{self.authority.as_ref().is_some_and(|authority|authority.has_pending_action())}
 /// 🪪️ Creates the private context for this exact resident producer.
 pub fn context(&self)->Result<OriginalProjectionContext,ValueError>{self.authority.as_ref().map(|authority|authority.producer().context()).ok_or_else(||ValueError::invariant("original projection authority absent"))}
 /// 🎟️ Lends the original action authority to the real Source/Work settlement driver.
 /// 👓️ Borrows the same original authority while the actual projection producer retains it.
 pub fn original_authority(&self)->Option<&A>{self.authority.as_ref().map(|authority|authority.producer()).or(self.producer.as_ref()).and_then(|producer|producer.owner.as_ref().map(SealedShared::get))}
 pub fn authority(&mut self)->Result<&mut OriginalEffectAuthority<OriginalProjectionProducer<T,A>>,ValueError>{self.authority.as_mut().ok_or_else(||ValueError::invariant("original projection authority absent"))}
 /// 📐️ Quotes the selected direct resident producer transfer after settlement.
 pub fn producer_transfer_demand(&self)->Result<RetirementDemand,ValueError>{OriginalEffectAuthority::<OriginalProjectionProducer<T,A>>::producer_transfer_demand()}
 /// 🫴️ Moves the same idle producer directly into this frame under the actual effect.
 pub fn transfer_producer(&mut self,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let context=match self.context(){Ok(context)=>context,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};OriginalEffectAuthority::transfer_producer(&mut self.authority,&context,&mut self.producer,effect)}
 pub(crate) fn matches(&self,id:u64)->bool{self.id==id}
 pub(crate) fn terminal_is_empty(&self)->bool{self.authority.is_none()&&self.producer.as_ref().is_some_and(|producer|producer.owner.is_none()&&producer.answer.is_none())}
 pub(crate) fn return_owner_demand(&self)->RetirementDemand{OriginalProjectionProducer::<T,A>::return_owner_demand()}
 pub(crate) fn return_unanswered_owner(&mut self,owner:&mut Option<SealedShared<A>>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{match self.producer.as_mut(){Some(producer)=>producer.return_owner(owner,effect),None=>crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original projection producer not transferred")),work:0}}}
 pub(crate) fn answer_demand(&self)->RetirementDemand{OriginalProjectionProducer::<T,A>::transfer_demand()}
 pub(crate) fn transfer_answer(&mut self,owner:&mut Option<SealedShared<A>>,answer:&mut Option<Result<NonNull<T>,ValueError>>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{match self.producer.as_mut(){Some(producer)=>producer.transfer(owner,answer,effect),None=>crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original projection producer not transferred")),work:0}}}
}
impl<T:Sync+'static,A:OriginalSourceProjection<T>> Drop for OriginalProjectionProducer<T,A>{fn drop(&mut self){assert!(std::thread::panicking()||(self.owner.is_none()&&self.answer.is_none()),"original projection requires paid owner and answer transfer")}}

impl<T:Sync+'static,A:OriginalSourceProjection<T>+RetireOwned+Sync,B:RetireOwned> RetainedCloneSourceAdmission<T,A,B>{
 /// 🧭️ Selects the next staged projection stage: 1 begin, 2 drive, 3 transfer producer, 4 accept answer, 5 finish; `None` when no projection is selected.
 pub fn projection_stage(&self,frame:&Option<OriginalProjectionFrame<T,A>>)->Result<Option<u8>,ValueError>{
  match frame{Some(frame)=>Ok(Some(match frame.phase()?{OriginalProjectionFramePhase::Drive=>2,OriginalProjectionFramePhase::TransferProducer=>3,OriginalProjectionFramePhase::AcceptAnswer=>4,OriginalProjectionFramePhase::Finish=>5,_=>return Err(ValueError::invariant("original projection requires its owner return"))})),None=>Ok((self.phase()=="projectionStart").then_some(1))}
 }
 /// 📐️ Quotes the physical demand and Work units of one staged projection stage.
 pub fn projection_stage_quote(&self,frame:&Option<OriginalProjectionFrame<T,A>>,stage:u8)->Result<(RetirementDemand,u64),ValueError>{
  match(stage,frame.as_ref()){(1,_)=>Ok((self.begin_projection_demand(),1)),(2,Some(f))=>Ok((f.projection_demand(),f.projection_work_demand()?)),(3,Some(f))=>Ok((f.producer_transfer_demand()?,1)),(4,Some(f))=>Ok((self.projection_answer_demand(f)?,2)),(5,_)=>Ok((self.finish_projection_demand(),1)),_=>Err(ValueError::invariant("original projection stage changed"))}
 }
 /// 🧭️ Runs one staged projection stage; a driven producer left unsettled reports Pending with its performed Work.
 pub fn project_staged(&mut self,frame:&mut Option<OriginalProjectionFrame<T,A>>,action:&mut Option<crate::retained_clone::OriginalEffectAction<crate::RetainedCloneStep,ValueError>>,stage:u8,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<crate::RetainedCloneStep,ValueError>{
  use crate::{RetainedCloneStep,RetainedWorkPoll::{Complete,Pending}};
  let step=|turn:RetainedWorkOutcome|Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work});
  let gone=||Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original projection frame absent")),work:0});
  match stage{
   1=>step(self.begin_projection(frame,effect)),
   2=>{let before=effect.performed_work();let driven=(||->Result<Option<Result<RetainedCloneStep,ValueError>>,ValueError>{let selected=frame.as_mut().ok_or_else(||ValueError::invariant("original projection frame absent"))?;let mut context=selected.context()?;if !selected.authority()?.drive(&mut context,effect,action)?{return Ok(None)}let mut recorded=None;action.as_mut().ok_or_else(||ValueError::invariant("original projection action absent"))?.transfer_reply(&mut recorded)?;selected.authority()?.finish(action)?;Ok(Some(recorded.take().ok_or_else(||ValueError::invariant("original projection reply absent"))?))})();let work=effect.performed_work().saturating_sub(before);
    match driven{Ok(None)=>Pending{work},Ok(Some(answer))=>Complete(RetainedWorkOutcome{result:answer.map(|step|RetainedCloneStep::Progress(step.progress())),work}),Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work})}},
   3=>match frame.as_mut(){Some(selected)=>{let turn=selected.transfer_producer(effect);Complete(RetainedWorkOutcome{result:turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())),work:turn.work})},None=>gone()},
   4=>match frame.as_mut(){Some(selected)=>step(self.accept_projection(selected,effect)),None=>gone()},
   _=>step(self.finish_projection(frame,effect)),
  }
 }
}
