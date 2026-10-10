//! ♻️ Closing aliases atomically retain the original registry after funded header release.
use super::{SnapshotReadRegistryHandle,SnapshotReadLeaseRegistry,snapshot_registry_frame_bytes};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

pub(crate) struct SnapshotReadRegistryAliasRetirement {
 registry:ManuallyDrop<Option<SnapshotReadRegistryHandle>>,
 last_registry:ManuallyDrop<Option<SnapshotReadLeaseRegistry>>,
 identity:usize,
}
impl SnapshotReadRegistryAliasRetirement {
 pub(crate) fn new(registry:SnapshotReadRegistryHandle)->Self{let identity=registry.identity();Self{registry:ManuallyDrop::new(Some(registry)),last_registry:ManuallyDrop::new(None),identity}}
 pub(super) fn identity(&self)->usize{self.identity}
 pub(super) fn ptr_eq(&self,other:&SnapshotReadRegistryHandle)->bool{self.registry.as_ref().is_some_and(|registry|registry.ptr_eq(other))}
 pub(super) fn strong_count(&self)->usize{self.registry.as_ref().map_or(usize::from(self.last_registry.is_some()),SnapshotReadRegistryHandle::strong_count)}
 pub(super) fn alias_handle(&self)->&SnapshotReadRegistryHandle{self.registry.as_ref().expect("live source registry alias is unavailable after header retirement")}
 pub(super) fn frame_byte_demand(&self)->Result<usize,ValueError>{if self.registry.is_some(){snapshot_registry_frame_bytes()}else{Ok(0)}}
 fn closed(&self)->bool{self.registry.is_none()&&self.last_registry.is_none()}
 fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.registry.is_some(){return Ok(RetirementDemand{release_bytes:snapshot_registry_frame_bytes()?,depth:1,..Default::default()});}
  if let Some(registry)=self.last_registry.as_ref(){let mut demand=registry.empty_backing_demands()?;demand.depth=demand.depth.max(1);return Ok(demand);}
  Ok(Default::default())
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closed(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.demands()?;
  if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry alias exceeds admitted depth"));}
  if grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(registry)=self.registry.take(){
   let last=registry.into_inner();let released=if let Some(registry)=last{*self.last_registry=Some(registry);demand.release_bytes}else{0};
   let progress=RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()};
   if self.last_registry.as_ref().is_some_and(|registry|registry.terminal_is_empty()&&registry.empty_backing_demands().is_ok_and(|demand|demand.release_bytes==0)){self.last_registry.take();}
   return Ok(if self.closed(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)});
  }
  let registry=self.last_registry.as_ref().expect("original last registry remains in inline custody");
  if !registry.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"last registry requires its original typed store retirement pump"));}
  if demand.release_bytes!=0{return registry.close_empty_backing_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  self.last_registry.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl std::ops::Deref for SnapshotReadRegistryAliasRetirement {
 type Target=SnapshotReadLeaseRegistry;
 fn deref(&self)->&Self::Target{self.registry.as_ref().map(|registry|&**registry).or_else(||self.last_registry.as_ref()).expect("registry retirement has reached terminal custody")}
}
impl Drop for SnapshotReadRegistryAliasRetirement {
 fn drop(&mut self){assert!(std::thread::panicking()||self.closed(),"original registry alias retirement dropped before funded terminal custody");}
}
pub(crate) fn snapshot_registry_alias_demands(owner:&Option<SnapshotReadRegistryAliasRetirement>)->Result<RetirementDemand,ValueError>{owner.as_ref().map_or(Ok(Default::default()),SnapshotReadRegistryAliasRetirement::demands)}
pub(crate) fn snapshot_registry_alias_close_step(slot:&mut Option<SnapshotReadRegistryAliasRetirement>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 let Some(owner)=slot.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
 let step=owner.step(grant)?;if owner.closed(){slot.take();}Ok(step)
}
