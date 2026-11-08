//! ♻️ Closing aliases atomically retain the original registry after funded header release.
use super::{SnapshotReadRegistryHandle,SnapshotReadLeaseRegistry,snapshot_registry_frame_bytes};
use semio_framework_value::{ErasedSnapshotRetirement,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

pub struct SnapshotReadRegistryAliasRetirement {
 registry:ManuallyDrop<Option<SnapshotReadRegistryHandle>>,
 last_registry:ManuallyDrop<Option<SnapshotReadLeaseRegistry>>,
 identity:usize,
 last_work:u128,
}
impl SnapshotReadRegistryAliasRetirement {
 pub fn new(registry:SnapshotReadRegistryHandle)->Self{let identity=registry.identity();Self{registry:ManuallyDrop::new(Some(registry)),last_registry:ManuallyDrop::new(None),identity,last_work:0}}
 pub fn last_turn_work(&self)->u128{self.last_work}
 pub fn selected_work_demand(&self)->Result<u64,ValueError>{if self.closed(){return Ok(0)}if self.registry.is_some(){return Ok(1)}let registry=self.last_registry.as_ref().expect("original last registry remains in inline custody");if !registry.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"last registry requires its original typed store retirement pump"))}if self.demands()?.release_bytes!=0{registry.owned_backing_work_demand()}else{Ok(1)}}
 pub fn identity(&self)->usize{self.identity}
 pub fn ptr_eq(&self,other:&SnapshotReadRegistryHandle)->bool{self.registry.as_ref().is_some_and(|registry|registry.ptr_eq(other))}
 pub fn strong_count(&self)->usize{self.registry.as_ref().map_or(usize::from(self.last_registry.is_some()),SnapshotReadRegistryHandle::strong_count)}
 pub fn alias_handle(&self)->&SnapshotReadRegistryHandle{self.registry.as_ref().expect("live source registry alias is unavailable after header retirement")}
 pub fn frame_byte_demand(&self)->Result<usize,ValueError>{if self.registry.is_some(){snapshot_registry_frame_bytes()}else{Ok(0)}}
 pub fn custody_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(|demand|demand.release_bytes)}
 pub(crate) fn closed(&self)->bool{self.registry.is_none()&&self.last_registry.is_none()}
 fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.registry.is_some(){return Ok(RetirementDemand{release_bytes:snapshot_registry_frame_bytes()?,depth:1,..Default::default()});}
  if let Some(registry)=self.last_registry.as_ref(){let mut demand=registry.owned_backing_demands()?;demand.depth=demand.depth.max(1);return Ok(demand);}
  Ok(Default::default())
 }
 pub(crate) fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();self.last_work=0;
  if self.closed(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0||effect.maximum_work_units()==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.demands()?;
  if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry alias exceeds admitted depth"));}
  if grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.registry.is_some(){
   crate::pay_pure_work(effect,&mut self.last_work)?;
   let Some(registry)=self.registry.take()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registry alias lost its handle"))};
   let last=registry.into_inner();let released=if let Some(registry)=last{*self.last_registry=Some(registry);demand.release_bytes}else{0};
   let progress=RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()};
   if self.last_registry.as_ref().is_some_and(|registry|registry.terminal_is_empty()&&registry.owned_backing_demands().is_ok_and(|demand|demand.release_bytes==0)){self.last_registry.take();}
   return Ok(if self.closed(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)});
  }
  let Some(registry)=self.last_registry.as_mut()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original last registry remains in inline custody"))};
  if !registry.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"last registry requires its original typed store retirement pump"));}
  if demand.release_bytes!=0{let turn=registry.close_owned_backing_step(effect);self.last_work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
  crate::pay_pure_work(effect,&mut self.last_work)?;self.last_registry.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl std::ops::Deref for SnapshotReadRegistryAliasRetirement {
 type Target=SnapshotReadLeaseRegistry;
 fn deref(&self)->&Self::Target{self.registry.as_ref().map(|registry|&**registry).or_else(||self.last_registry.as_ref()).expect("registry retirement has reached terminal custody")}
}
impl Drop for SnapshotReadRegistryAliasRetirement {
 fn drop(&mut self){assert!(std::thread::panicking()||self.closed(),"original registry alias retirement dropped before funded terminal custody");}
}
pub fn snapshot_registry_alias_demands(owner:&Option<SnapshotReadRegistryAliasRetirement>)->Result<RetirementDemand,ValueError>{owner.as_ref().map_or(Ok(Default::default()),|owner|if owner.closed(){Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadRegistryAliasRetirement>(),depth:1,..Default::default()})}else{owner.demands()})}
pub fn snapshot_registry_alias_work_demand(owner:&Option<SnapshotReadRegistryAliasRetirement>)->Result<u64,ValueError>{owner.as_ref().map_or(Ok(0),|owner|if owner.closed(){Ok(1)}else{owner.selected_work_demand()})}
pub fn snapshot_registry_alias_close_step(slot:&mut Option<SnapshotReadRegistryAliasRetirement>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let done=|turn:OriginalRetirementOutcome|RetainedWorkPoll::Complete(turn);
 let grant=effect.grant();
 let Some(owner)=slot.as_mut()else{return done(OriginalRetirementOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})};
 if owner.closed(){let d=RetirementDemand{copy_bytes:size_of::<SnapshotReadRegistryAliasRetirement>(),depth:1,..Default::default()};if effect.maximum_work_units()==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return done(OriginalRetirementOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let mut work=0;if let Err(error)=crate::pay_pure_work(effect,&mut work){return done(OriginalRetirementOutcome{result:Err(error.into()),work})}slot.take();return done(OriginalRetirementOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work})}
 let result=owner.step(effect).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);done(OriginalRetirementOutcome{result,work:owner.last_turn_work()})
}

impl ErasedSnapshotRetirement for SnapshotReadRegistryAliasRetirement {
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{self.demands()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
 fn next_capacity_byte_demand(&self,_body:usize)->Result<usize,ValueError>{Ok(self.demands()?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.step(effect).map_err(Into::into);RetainedWorkPoll::Complete(OriginalRetirementOutcome{result,work:self.last_work})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{self.selected_work_demand()}
 fn terminal_is_empty(&self)->bool{self.closed()}
}
