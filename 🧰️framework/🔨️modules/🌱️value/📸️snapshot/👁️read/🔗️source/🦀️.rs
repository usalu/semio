//! 🔗️ Captured snapshot sources keep the original Store read and its installed typed retirement issuer.
use super::{SnapshotRead,SnapshotReadReturn,SnapshotReadLeaseRefusal,SnapshotReadRegistryAliasRetirement,try_return_original_snapshot_read,snapshot_registry_alias_demands,snapshot_registry_alias_work_demand,snapshot_registry_alias_close_step,advance_returned_snapshot_read,artifact_retirement_box_demands,artifact_retirement_box_close_step,};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retirement::{erased_cursor_turn,record_retirement_work,RetireOwned,RetirementCursor,RetirementStep},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

/// 🧳️ The projected snapshot needs no intrinsic retirement when its original Store issuer owns that capability.
pub struct CapturedSnapshotSource<T:Send+Sync+'static>{
 read:ManuallyDrop<Option<SnapshotRead<T>>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
}
#[doc(hidden)]
pub struct CapturedSnapshotRetirement<T:Send+Sync+'static>{
 read:ManuallyDrop<Option<SnapshotRead<T>>>,
 registry:ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
 active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
 factory_close:Option<FactoryAuthority>,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("captured snapshot source depth overflow"))?;Ok(d)}
impl<T:Send+Sync+'static> CapturedSnapshotSource<T>{
 /// 📏️ Quotes the actual read and issuer transfer before either original owner changes.
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<SnapshotRead<T>>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(),depth:1,..Default::default()}}
 /// ⛽️ Quotes the original read transfer and issuer alias before either effect.
 pub fn constructor_work_demand(read:&Option<SnapshotRead<T>>)->u64{if read.is_some(){2}else{0}}
 /// 🎟️ Captures the exact issued read and factory alias after all original currencies are admitted.
 pub fn admit(read:&mut Option<SnapshotRead<T>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let d=Self::constructor_demand();if effect.maximum_work_units()<Self::constructor_work_demand(read)||!retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}if read.is_none(){return RetainedWorkOutcome{result:Err(invalid("captured snapshot source requires its original Store read")),work:0}}
  let work=match record_retirement_work(effect,2){Ok(work)=>work,Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}};let Some(original)=read.take()else{return RetainedWorkOutcome{result:Err(invalid("captured snapshot source lost its original Store read")),work}};
  RetainedWorkOutcome{result:Ok(Some((Self{read:ManuallyDrop::new(Some(original)),factory:ManuallyDrop::new(Some(Arc::clone(factory)))},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work}
 }
 /// 👁️ Borrows the immutable original root while its exact registry lease remains captured.
 pub fn get(&self)->&T{self.read.as_ref().expect("captured snapshot source has started original return").get()}
}
impl<T:Send+Sync+'static> CapturedSnapshotRetirement<T>{
 fn new(mut source:CapturedSnapshotSource<T>)->Self{Self{read:ManuallyDrop::new(source.read.take()),registry:ManuallyDrop::new(None),active:ManuallyDrop::new(None),factory:ManuallyDrop::new(source.factory.take()),factory_close:None}}
 /// 📪️ Quotes exact read return, last-registry typed pumping, and installed issuer closure.
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.read.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadReturn>()+size_of::<SnapshotRead<T>>()+size_of::<u64>()+size_of::<usize>(),depth:2,..Default::default()});}
  if let Some(active)=self.active.as_ref(){return nested(artifact_retirement_box_demands(active,body)?);}
  if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){return nested(registry.returned_admission_demands::<T>(|_|RetirementDemand{capacity_bytes:semio_framework_value_factory::shared::FactorySharedRetirement::<T>::constructor_capacity_bytes(),depth:1,..Default::default()}).map_err(SnapshotReadLeaseRefusal::into_value_error)?);}return snapshot_registry_alias_demands(&self.registry);}
  if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<FactoryAuthority>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(),depth:1,..Default::default()});}
  self.factory_close.as_ref().map_or(Ok(Default::default()),|factory|if factory.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryAuthority>>(),depth:1,..Default::default()})}else{factory.demands(body)})
 }
 /// ↩️ Returns this original read before releasing registry custody or retiring any original payload.
 fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();
  let done=|turn:OriginalRetirementOutcome|RetainedWorkPoll::Complete(turn);
  let refuse=|error:ValueError,work:u128|done(RetainedWorkOutcome{result:Err(error.into()),work});
  let denied=||done(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0});
  if self.terminal_is_empty(){return done(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0});}let d=match self.close_demand(body){Ok(d)=>d,Err(error)=>return refuse(error,0)};if effect.maximum_work_units()==0||!retained_clone_grant_admits(grant,d){return denied();}
  let mut work=0;
  let moved=|work:u128|done(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work});
  if self.read.is_some(){
   if !matches!(effect.remaining_work_units(),Ok(1..)){return denied();}
   let witness=match self.read.as_mut(){Some(read)=>match try_return_original_snapshot_read(&mut read.owner,&mut read.lease){Ok(witness)=>witness,Err(())=>return denied()},None=>return denied()};
   *self.registry=Some(SnapshotReadRegistryAliasRetirement::new(witness.registry));self.read.take();
   if let Err(error)=crate::pay_pure_work(effect,&mut work){return refuse(error,work);}
   return moved(work);
  }
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
  if self.active.is_some(){return artifact_retirement_box_close_step(&mut self.active,body,&mut effect.nested(child));}
  if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){let Some(factory)=self.factory.as_ref()else{return refuse(invalid("captured snapshot source lost its installed issuer"),0)};return advance_returned_snapshot_read(registry,&mut self.active,factory,&mut effect.nested(child));}return snapshot_registry_alias_close_step(&mut self.registry,effect);}
  if self.factory.is_some(){if let Err(error)=crate::pay_pure_work(effect,&mut work){return refuse(error,work);}if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));}return moved(work);}
  let Some(factory)=self.factory_close.as_mut()else{return refuse(invalid("captured snapshot source lost its final issuer cursor"),0)};
  if factory.terminal_is_empty(){if let Err(error)=crate::pay_pure_work(effect,&mut work){return refuse(error,work);}self.factory_close=None;return moved(work);}
  factory.step(body,effect).map(|step|RetainedCloneStep::Progress(step.progress()))
 }
 /// 🫙️ Every original read, registry backing, typed payload ticket and issuer has returned.
 fn empty(&self)->bool{self.read.is_none()&&self.registry.is_none()&&self.active.is_none()&&self.factory.is_none()&&self.factory_close.is_none()}
}
impl<T:Send+Sync+'static> Drop for CapturedSnapshotSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.read.is_none()&&self.factory.is_none(),"captured snapshot source requires ownership transfer into its granted retirement cursor");}}
impl<T:Send+Sync+'static> Drop for CapturedSnapshotRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"captured snapshot cursor requires original registry return and granted issuer closure");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factory);}}}}
/// 🌿️ The captured read projects its immutable original root as the typed Source.
impl<T:Send+Sync+'static> semio_framework_value::retained_clone::OriginalSourceProjection<T> for CapturedSnapshotSource<T>{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalProjectionReply<'reply,'source,T>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<T>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"captured snapshot projection exceeded its original work"))))}Ok(reply.complete(Ok(self.get())))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalProjectionReply<'reply,'source,T>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<T>,ValueError>,ValueError>{Ok(reply.pending())}
}
impl<T:Send+Sync+'static> RetireOwned for CapturedSnapshotSource<T>{
 type RetirementCursor=CapturedSnapshotRetirement<T>;
 fn retirement_seed(self)->Self::RetirementCursor{CapturedSnapshotRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CapturedSnapshotRetirement<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:Send+Sync+'static> RetirementCursor for CapturedSnapshotRetirement<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.read.is_some(){return Ok(1)}if let Some(owner)=self.active.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){return Ok(1)}return snapshot_registry_alias_work_demand(&self.registry)}if self.factory.is_some(){return Ok(1)}match self.factory_close.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
 fn requires_original_control(&self)->bool{self.read.is_none()&&(self.active.is_some()||(self.registry.is_none()&&self.factory.is_none()&&self.factory_close.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())))}
 fn close_step_controlled(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();use RetirementStep;if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured source has no selected original child").into(),0)}let d=match self.close_demand(body){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}let turn=if self.active.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};semio_framework_value::close_factory_ticket_controlled(&mut self.active,body,&mut effect.nested(child),control)}else{match self.factory_close.as_mut(){Some(owner)=>owner.step_controlled(body,effect,control),None=>return RetirementStep::Failure(invalid("captured source lost original factory").into(),0)}};erased_cursor_turn(turn)}
 fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();use RetirementStep;if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured source has no selected original child").into(),0)}let d=match self.close_demand(body){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}let turn=if self.active.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};semio_framework_value::close_factory_ticket_original(&mut self.active,original,body,&mut effect.nested(child))}else{match self.factory_close.as_mut(){Some(owner)=>owner.step_original(original,body,effect),None=>return RetirementStep::Failure(invalid("captured source lost original factory").into(),0)}};erased_cursor_turn(turn)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{Ok(reply.complete(Ok(erased_cursor_turn(self.step(body,effect)))))}
 fn recover_close_into<'r>(&mut self,_body:usize,_:&mut RetainedAllocationEffect<'_>,_:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,ValueError>{Err(invalid("synchronous captured snapshot close has no unrecorded callback"))}
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.close_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.close_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
