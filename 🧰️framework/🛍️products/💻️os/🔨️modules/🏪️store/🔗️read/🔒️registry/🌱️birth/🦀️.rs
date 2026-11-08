//! 🌱️ Original registry custody admits its frame and every queue, slot and generation page separately.
use super::{SnapshotReadRegistryHandle,SnapshotReadLeaseRegistry};
use crate::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands};
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{NativeAllocationIdentity, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::{mem::{size_of,ManuallyDrop},sync::Arc};

/// 🧳️ A partial registry cannot issue reads and keeps its original physical pages through refusal and cancellation.
pub struct SnapshotReadRegistryBirth {registry:ManuallyDrop<Option<SnapshotReadRegistryHandle>>,retirement:ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,ledger:Option<NativeAllocationIdentity>,closing:bool,taken:bool,last_work:u128}
impl SnapshotReadRegistryBirth {
 pub fn new()->Self{Self{registry:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),ledger:None,closing:false,taken:false,last_work:0}}
 /// ⛽️ Reports the selected bounded birth or close action independently of physical receipt fields.
 pub fn last_turn_work(&self)->u128{self.last_work}
 pub fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.taken{return Ok(Default::default());}
  if let Some(registry)=self.registry.as_ref(){return registry.birth_page_demands();}
  Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadLeaseRegistry>(),capacity_bytes:crate::snapshot_registry_frame_bytes()?,depth:1,..Default::default()})
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant,control:&mut NativeDecodeControl<'_>)->Result<RetainedCloneStep,ValueError>{
  self.last_work=0;let demand=self.demands()?;if self.closing||self.taken||demand.capacity_bytes==0||!retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.ledger.is_some_and(|ledger|ledger!=control.ledger_identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registry birth requires original decoder ledger"));}
  if self.registry.as_ref().is_some_and(|registry|Arc::strong_count(&registry.inner)!=1||Arc::weak_count(&registry.inner)!=0){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"unpublished registry acquired a foreign alias"))}
  control.checkpoint()?;control.admit_turn_capacity(grant.maximum_capacity_bytes)?;if !control.prepare_work(1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Err(error)=control.charge(demand.capacity_bytes){control.commit_work(0)?;return Err(error)}self.ledger=Some(control.ledger_identity());
  if let Some(registry)=self.registry.as_mut(){Arc::get_mut(&mut registry.inner).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"unpublished registry acquired a foreign alias"))?.admit_birth_page()?;}
  else{*self.registry=Some(SnapshotReadRegistryHandle{inner:Arc::new(SnapshotReadLeaseRegistry::vacant())});}
  self.last_work=1;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()};control.commit_work(self.last_work).map_err(|error|error.with_retained_progress(progress))?;Ok(if self.ready().map_err(|error|error.with_retained_progress(progress))?{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn ready(&self)->Result<bool,ValueError>{Ok(!self.closing&&!self.taken&&self.registry.as_ref().is_some_and(|registry|registry.birth_page_demands().is_ok_and(|demand|demand.capacity_bytes==0)))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(SnapshotReadRegistryHandle,RetainedCloneProgress)>,ValueError>{let demand=RetirementDemand{copy_bytes:size_of::<SnapshotReadRegistryHandle>(),depth:1,..Default::default()};if !self.ready()?||!retained_clone_grant_admits(grant,demand){return Ok(None);}self.taken=true;Ok(Some((self.registry.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if self.registry.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadRegistryHandle>()+size_of::<SnapshotReadRegistryAliasRetirement>(),depth:1,..Default::default()});}snapshot_registry_alias_demands(&self.retirement)}
 /// 🔒️ The caller names the decoder ledger that owns this close effect; a registry born under another ledger refuses before any release.
 pub fn close_step(&mut self,ledger:NativeAllocationIdentity,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();self.last_work=0;let demand=self.close_demands()?;if !self.closing||effect.maximum_work_units()==0||!retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.ledger.is_some_and(|born|born!=ledger){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registry close requires original decoder ledger"));}
  if self.registry.is_some(){crate::pay_pure_work(effect,&mut self.last_work)?;}
  if let Some(registry)=self.registry.take(){*self.retirement=Some(SnapshotReadRegistryAliasRetirement::new(registry));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}let Some(owner)=self.retirement.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};let result=owner.step(effect);self.last_work=owner.last_turn_work();if owner.closed(){self.retirement.take();}result
 }
 pub fn terminal_is_empty(&self)->bool{(self.closing||self.taken)&&self.registry.is_none()&&self.retirement.is_none()}
}
impl Drop for SnapshotReadRegistryBirth {fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"partial registry requires original funded handoff or retirement");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
