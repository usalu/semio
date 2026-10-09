//! 🧾️ Returned snapshot witnesses preserve original registry custody through funded closure.
use super::{SnapshotReadReturn,SnapshotReadLeaseRegistry,snapshot_registry_frame_bytes};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{mem::ManuallyDrop,sync::Arc};

struct SnapshotReadReturnRetirement {
 registry:ManuallyDrop<Option<crate::os_store::SnapshotReadRegistryHandle>>,
 last_registry:ManuallyDrop<Option<SnapshotReadLeaseRegistry>>,
 index:u16,
 generation:u64,
}
impl SnapshotReadReturnRetirement {
 fn new(value:SnapshotReadReturn)->Self {Self{registry:ManuallyDrop::new(Some(value.registry)),last_registry:ManuallyDrop::new(None),index:value.index,generation:value.generation}}
 fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.registry.is_some(){return Ok(RetirementDemand{release_bytes:snapshot_registry_frame_bytes()?,depth:1,..Default::default()});}
  if let Some(owner)=self.last_registry.as_ref(){let mut demand=owner.empty_backing_demands()?;demand.depth=demand.depth.max(1);return Ok(demand);}
  Ok(Default::default())
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.demands()?;
  if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"returned read witness exceeds admitted depth"));}
  if grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(registry)=self.registry.as_ref(){
   if registry.contains(self.index,self.generation){return Ok(RetainedCloneStep::Progress(Default::default()));}
   let last=self.registry.take().expect("original returned read registry").into_inner();
   let released=if let Some(registry)=last{*self.last_registry=Some(registry);demand.release_bytes}else{0};
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));
  }
  let owner=self.last_registry.as_ref().expect("last original registry custody");
  if !owner.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"last returned read registry still requires its store-owned typed retirement pump"));}
  if demand.release_bytes!=0{return owner.close_empty_backing_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  self.last_registry.take();
  Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl RetireOwned for SnapshotReadReturn {
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SnapshotReadReturnRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SnapshotReadReturnRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SnapshotReadReturnRetirement {
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
 fn terminal_is_empty(&self)->bool{self.registry.is_none()&&self.last_registry.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for SnapshotReadReturnRetirement {
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"returned read witness abandoned original registry custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.last_registry);}}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
