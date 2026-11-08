//! 📦️ Original mutation values retain their installed issuer through independently admitted cleanup.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::{mem::ManuallyDrop,sync::Arc};

/// 🧳️ A send-only original remains owned when its exact payload issuer refuses admission.
pub struct FactoryOwnedRetirement<T:Send+'static> {
    original:ManuallyDrop<Option<T>>,
    unique:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
    production:ManuallyDrop<Option<semio_framework_value_receiving::span::effect::OriginalEffectAuthority<FactoryOwnedProducer<T>>>>,
    production_action:Option<semio_framework_value_receiving::span::effect::OriginalEffectAction<RetainedCloneProgress,crate::OriginalRetirementFailure>>,
    production_reply:Option<Result<RetainedCloneProgress,crate::OriginalRetirementFailure>>,
}
struct FactoryOwnedProducer<T:Send+'static>{original:Option<T>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,output:Option<Box<dyn ErasedSnapshotRetirement>>,reply:Option<crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalRetirementFailure>>,attempted:Option<u128>,demand:RetirementDemand,maximum_work:u64}
impl<T:Send+'static> semio_framework_value_receiving::span::effect::OriginalEffectProducer for FactoryOwnedProducer<T>{
 type Output=RetainedCloneProgress;
 type Failure=crate::OriginalRetirementFailure;
 type Context<'turn,'source>=Option<crate::OriginalSourceTurnIdentity> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{*context}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_receiving::span::effect::OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<semio_framework_value_receiving::span::effect::OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError>where 'source:'turn{self.factory.retire_owned(&mut self.original,effect,crate::FactoryProducerReply::borrow(&mut self.output,&mut self.reply,&mut self.attempted)?);Ok(match self.reply.take(){Some(turn)=>reply.complete(turn.result),None=>reply.pending()})}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_receiving::span::effect::OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<semio_framework_value_receiving::span::effect::OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError>where 'source:'turn{if self.reply.is_none(){self.factory.recover_retire_owned(&mut self.original,effect,crate::FactoryProducerReply::borrow(&mut self.output,&mut self.reply,&mut self.attempted)?);}Ok(match self.reply.take(){Some(turn)=>reply.complete(turn.result),None=>reply.pending()})}
}
impl<T:Send+'static> FactoryOwnedRetirement<T> {
    pub(crate) fn from_captured(original:Option<T>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{original:ManuallyDrop::new(original),unique:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(factory)),closing_factory:ManuallyDrop::new(None),production:ManuallyDrop::new(None),production_action:None,production_reply:None}}
    pub(crate) fn original(&self)->Option<&T>{self.production.as_ref().map_or_else(||self.original.as_ref(),|source|source.producer().original.as_ref())}
    pub(crate) fn receive_original(&mut self,original:Option<T>){assert!(self.original.is_none()&&self.production.is_none()&&self.unique.is_none(),"factory original destination is occupied");*self.original=original}
    pub const fn constructor_capacity_bytes()->usize {size_of::<Self>()}
    /// 🎟️ Grants the complete frame before taking its original or cloning its installed issuer.
    /// ⛽️ Quotes the allocation, original transfer and factory alias before the physical grant.
    pub fn admit_original_source_work_demand(original:&Option<T>)->u64{u64::from(original.is_some())}
    pub fn original_admission_demands(original:&Option<T>)->RetirementDemand{if original.is_none(){return Default::default()}RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>()+size_of::<Self>()+size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
    pub fn admit_original(original:&mut Option<T>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
     if output.is_some(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("factory original output occupied")),work:0})}if original.is_none(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}if factory.is_none(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("factory original issuer absent")),work:0})}let demand=Self::original_admission_demands(original);if effect.maximum_work_units()==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if effect.actual().is_some(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("factory original allocation marker occupied")),work:0})}let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error),work})};*effect.actual()=Some(0);let layout=std::alloc::Layout::new::<Self>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>())else{return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"factory original frame allocation failed")),work})};*effect.actual()=Some(layout.size());let owner=unsafe{pointer.as_ptr().write(Self::from_captured(original.take(),factory.take().expect("admitted exact original issuer")));Box::from_raw(pointer.as_ptr())};*output=Some(owner);crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()})),work})
    }
    fn produce(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
     let boundary=effect.performed_boundary();if self.production.is_none(){let demand=match self.original.as_ref().zip(self.factory.as_ref()).map(|(value,factory)|factory.retire_owned_demands(value)){Some(Ok(demand))=>demand,Some(Err(error))=>return done(Err(error.into()),0),None=>return done(Err(ValueError::invariant("factory producer source absent").into()),0)};let maximum_work=match self.original.as_ref().zip(self.factory.as_ref()).map(|(value,factory)|factory.retire_owned_work_demand(value)){Some(Ok(work))=>work,Some(Err(error))=>return done(Err(error.into()),0),None=>return done(Err(ValueError::invariant("factory producer source absent").into()),0)};let Some(original)=self.original.take()else{return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("factory producer lost its original").into()),work:0})};let Some(factory)=self.factory.take()else{*self.original=Some(original);return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("factory producer lost its issuer").into()),work:0})};*self.production=Some(semio_framework_value_receiving::span::effect::OriginalEffectAuthority::new(FactoryOwnedProducer{original:Some(original),factory,output:None,reply:None,attempted:None,demand,maximum_work}))}
     let mut context=effect.work_source_turn();let authority=self.production.as_mut().expect("resident original producer");let drive=authority.drive(&mut context,effect,&mut self.production_action);let work=effect.performed_since(boundary).expect("factory producer retains its actual receipt");match drive{Err(error)=>return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work}),Ok(false)=>return crate::RetainedWorkPoll::Pending{work},Ok(true)=>{}}
     let action=self.production_action.as_mut().expect("resident original producer action");if let Err(error)=action.transfer_reply(&mut self.production_reply){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work})}if let Err(error)=authority.finish(&mut self.production_action){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work})}let authority=self.production.take().expect("finished producer remains resident");let source=authority.into_producer().ok().expect("finished producer returns its original");*self.original=source.original;*self.factory=Some(source.factory);*self.unique=source.output;crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:self.production_reply.take().expect("recorded original producer reply").map(RetainedCloneStep::Progress),work})
    }
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
     let mut demand=if let Some(source)=self.production.as_ref(){source.inspect(|source|source.demand)}else if let Some(owner)=self.unique.as_ref(){crate::factory_ticket_demands(owner,body)?}else if let Some(value)=self.original.as_ref(){self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?.retire_owned_demands(value)?}else if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>()+size_of::<Option<FactoryAuthority>>(),depth:1,..Default::default()})}else if let Some(owner)=self.closing_factory.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryAuthority>>(),depth:1,..Default::default()})}owner.demands(body)?}else{return Ok(Default::default())};demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::invariant("factory child depth overflow"))?;Ok(demand)
    }

}
fn done(result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>,work:u128)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result,work})}
impl<T:Send+'static> ErasedSnapshotRetirement for FactoryOwnedRetirement<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(source)=self.production.as_ref(){return Ok(source.inspect(|source|source.maximum_work))}if let Some(owner)=self.unique.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}if let Some(value)=self.original.as_ref(){return self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?.retire_owned_work_demand(value)}if self.factory.is_some(){return Ok(1)}self.closing_factory.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
 fn requires_original_birth(&self)->bool{self.production.is_none()&&(self.unique.as_ref().is_some_and(|owner|owner.requires_original_birth())||(self.unique.is_none()&&self.original.is_none()&&self.factory.is_none()&&self.closing_factory.as_ref().is_some_and(|owner|owner.requires_original_birth())))}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{let Some(depth)=grant.maximum_depth.checked_sub(1)else{return done(Ok(RetainedCloneStep::Progress(Default::default())),0)};let child=RetainedCloneGrant{maximum_depth:depth,..grant};if let Some(owner)=self.unique.as_mut(){return owner.close_original_birth_step(original,child)}if let Some(owner)=self.closing_factory.as_mut(){return owner.close_original_birth_step(original,child)}done(Err(ValueError::invariant("factory owner has no selected original birth").into()),0)}
 fn requires_original_control(&self)->bool{self.production.is_none()&&(self.unique.as_ref().is_some_and(|owner|owner.requires_original_control())||(self.unique.is_none()&&self.original.is_none()&&self.factory.is_none()&&self.closing_factory.as_ref().is_some_and(|owner|owner.requires_original_control())))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let grant=effect.grant();let demand=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error.into()),0)};if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let mut effect=effect.nested(child);if self.unique.is_some(){return semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.unique,&mut effect,control).map(|step|RetainedCloneStep::Progress(step.progress()))}match self.closing_factory.as_mut(){Some(owner)=>owner.step_controlled(&mut effect,control).map(|step|RetainedCloneStep::Progress(step.progress())),None=>done(Err(ValueError::invariant("factory owner has no selected original control").into()),0)}}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();let demand=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error.into()),0)};if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let mut effect=effect.nested(child);if self.unique.is_some(){return semio_framework_value_retirement::close_factory_ticket_original(&mut self.unique,original,&mut effect).map(|step|RetainedCloneStep::Progress(step.progress()))}match self.closing_factory.as_mut(){Some(owner)=>owner.step_original(original,&mut effect).map(|step|RetainedCloneStep::Progress(step.progress())),None=>done(Err(ValueError::invariant("factory owner has no selected original control").into()),0)}}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}let grant=effect.grant();let demand=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error.into()),0)};let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return done(Err(error.into()),0)};if effect.maximum_work_units()<work||!semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if self.production.is_some()||self.unique.is_none()&&self.original.is_some(){return self.produce(effect)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if self.unique.is_some(){return crate::close_factory_ticket(&mut self.unique,&mut effect.nested(child)).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if self.factory.is_some(){let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};let factory:Arc<dyn FactoryRetirement>=self.factory.take().expect("resident original factory");*self.closing_factory=Some(FactoryAuthority::new(factory));return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work)}
  let Some(owner)=self.closing_factory.as_mut()else{return done(Err(ValueError::invariant("factory closing owner absent").into()),0)};if owner.terminal_is_empty(){let work=match crate::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};self.closing_factory.take();return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work)}owner.step(&mut effect.nested(child)).map(|step|RetainedCloneStep::Progress(step.progress()))
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.unique.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()&&self.production.is_none()&&self.production_action.is_none()&&self.production_reply.is_none()}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<T:Send+'static> Drop for FactoryOwnedRetirement<T> {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned factory retirement abandoned original payload or issuer custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.unique);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}}}
}

/// 🔗️ A live immutable source retains its actual value and captured issuer without a send-only retirement child.
pub struct FactoryOwnedSourceAuthority<T:Send+'static> {
    original: ManuallyDrop<Option<T>>,
    factory: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
}
impl<T:Send+'static> FactoryOwnedSourceAuthority<T> {
    /// 🌱️ Moves the original pair into the caller-funded inline source authority.
    pub fn new(original:T,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->Self { Self { original:ManuallyDrop::new(Some(original)),factory:ManuallyDrop::new(Some(factory)) } }
    pub fn get(&self)->&T { self.original.as_ref().expect("live source retains its exact original value") }
    pub fn factory(&self)->&Arc<dyn ArtifactOwnedValueRetirementFactory<T>> { self.factory.as_ref().expect("live source retains its exact original issuer") }
    pub const fn transfer_copy_byte_demand()->usize { size_of::<(T,Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)>() }
    /// 📤️ Refusal preserves the actual original pair until its fixed transfer is funded.
    pub fn take_original(mut self,grant:RetainedCloneGrant)->Result<(T,Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,RetainedCloneProgress),(ValueError,Self)> {
        let refusal=if grant.maximum_items==0 { Some((ValueRefusalKind::WorkLimit,"original source transfer requires one item")) } else if grant.maximum_depth==0 { Some((ValueRefusalKind::DepthLimit,"original source transfer requires admitted depth")) } else if grant.maximum_copy_bytes<Self::transfer_copy_byte_demand() { Some((ValueRefusalKind::OwnershipLimit,"original source pair transfer exceeds admitted copy")) } else { None };
        if let Some((kind,message))=refusal { return Err((ValueError::literal(kind,message),self)); }
        let original=self.original.take().expect("live source original pair");
        let factory=self.factory.take().expect("live source original issuer pair");
        Ok((original,factory,RetainedCloneProgress { copied_items:1,copied_bytes:Self::transfer_copy_byte_demand(),..Default::default() }))
    }
}
#[doc(hidden)]
pub struct OwnedSourceRetirement<T:Send+'static>(FactoryOwnedRetirement<T>);
impl<T:Send+'static> crate::retirement::RetirementCursor for OwnedSourceRetirement<T> {
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{ErasedSnapshotRetirement::next_computational_work(&self.0,body)}
 fn requires_original_control(&self)->bool{ErasedSnapshotRetirement::requires_original_control(&self.0)}
 fn requires_original_birth(&self)->bool{ErasedSnapshotRetirement::requires_original_birth(&self.0)}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::retirement::RetirementStep{crate::authority_cursor_turn(ErasedSnapshotRetirement::close_original_birth_step(&mut self.0,original,grant))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->crate::retirement::RetirementStep{let grant=effect.grant();crate::authority_cursor_turn(ErasedSnapshotRetirement::close_step_controlled(&mut self.0,&mut effect.nested(grant),control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{let grant=effect.grant();crate::authority_cursor_turn(ErasedSnapshotRetirement::close_original_step(&mut self.0,original,&mut effect.nested(grant)))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.demands(body)}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{crate::authority_cursor_turn(ErasedSnapshotRetirement::close_step(&mut self.0,effect))}
    fn terminal_is_empty(&self)->bool { self.0.terminal_is_empty() }
    fn next_work_byte_demand(&self)->Result<usize,ValueError> { self.0.next_copy_byte_demand() }
    fn allows_admitted_narrow_work(&self)->bool { true }
    fn next_birth_bytes(&self,body:usize)->Option<usize> { self.0.next_capacity_byte_demand(body).ok() }
    fn next_close_byte_demand(&self)->Option<usize> { self.0.next_release_byte_demand().ok() }
    fn next_depth_demand(&self)->Result<usize,ValueError> { self.0.next_depth_demand() }
    fn terminal_release_bytes(&self)->Option<usize> { self.0.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl<T:Send+'static> crate::retirement::RetireOwned for FactoryOwnedSourceAuthority<T> {
    type RetirementCursor=OwnedSourceRetirement<T>;
    fn retirement_seed(mut self)->Self::RetirementCursor { OwnedSourceRetirement(FactoryOwnedRetirement { original:ManuallyDrop::new(self.original.take()),factory:ManuallyDrop::new(self.factory.take()),unique:ManuallyDrop::new(None),closing_factory:ManuallyDrop::new(None),production:ManuallyDrop::new(None),production_action:None,production_reply:None }) }
    fn retirement_birth_bytes(&self)->Option<usize> { Some(size_of::<OwnedSourceRetirement<T>>()) }
    fn controlled_retirement_supported()->bool { true }
}
impl<T:Send+'static> Drop for FactoryOwnedSourceAuthority<T> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.original.is_none() && self.factory.is_none()),"live source abandoned its original value or issuer");if self.original.is_none() && self.factory.is_none() { unsafe { ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.factory); } } }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
