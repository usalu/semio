//! 📸️ Derived snapshot aliases close through their own original registry and snapshot issuer.
use super::*;
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value::{RetirementDemand,FactoryAuthority,OriginalRetirementControl};
struct SnapshotAliasRetirement<P>{alias:ManuallyDrop<Option<Arc<P>>>,registry:ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,snapshots:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,factory:Option<FactoryAuthority>}
/// 📏️ Quotes only the original derived snapshot alias and its defining cleanup authority.
pub fn original_snapshot_alias_retirement_demand<P>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ArtifactDerivedSnapshot<P>>()+size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),capacity_bytes:size_of::<SnapshotAliasRetirement<P>>(),depth:1,..Default::default()}}
/// 🎟️ Captures the same original snapshot and registry after their complete shell admission.
pub fn admit_original_snapshot_alias<P:Send+Sync+'static>(original:&mut Option<ArtifactDerivedSnapshot<P>>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
 let d=original_snapshot_alias_retirement_demand::<P>();if original.is_none()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-d.copy_bytes,..grant};
 let(owner,mut p)=admit_artifact_retirement((),child,|()|{let ArtifactDerivedSnapshot{owner,registry,..}=original.take().unwrap();SnapshotAliasRetirement{alias:ManuallyDrop::new(Some(owner)),registry:ManuallyDrop::new(Some(SnapshotReadRegistryAliasRetirement::new(registry))),active:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(snapshots))),factory:None}}).map_err(|(error,())|error)?;p.copied_bytes+=d.copy_bytes;Ok(Some((owner,p)))
}
enum AliasCloseControl<'a,'loan>{Uncontrolled,Controlled(&'a mut dyn OriginalRetirementControl),Original(&'a mut semio_framework_value::OriginalRetirementCloseLoan<'loan>)}
impl<P:Send+Sync+'static> SnapshotAliasRetirement<P>{
 fn step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:AliasCloseControl<'_,'_>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
  let g=effect.grant();
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.next_demand(g.maximum_copy_bytes)?;
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.active.is_some(){let poll=match control{AliasCloseControl::Uncontrolled=>semio_framework_value::close_factory_ticket(&mut self.active,effect),AliasCloseControl::Controlled(control)=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,effect,control),AliasCloseControl::Original(original)=>semio_framework_value::close_factory_ticket_original(&mut self.active,original,effect)};return semio_framework_value::RetirementPollExit::resume(poll)}
  if self.registry.is_none()&&self.alias.is_none()&&self.snapshots.is_none(){
   if let Some(factory)=self.factory.as_mut().filter(|factory|!factory.terminal_is_empty()&&factory.requires_original_control()){
    let poll=match control{AliasCloseControl::Uncontrolled=>factory.step(effect),AliasCloseControl::Controlled(control)=>factory.step_controlled(effect,control),AliasCloseControl::Original(original)=>factory.step_original(original,effect)};return semio_framework_value::RetirementPollExit::resume(poll)
   }
  }
  self.local_step(effect)
 }
 fn local_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
  if self.registry.is_some(){return semio_framework_value::RetirementPollExit::resume(snapshot_registry_alias_close_step(&mut self.registry,effect))}
  if self.alias.is_some(){let mut factory=self.snapshots.as_ref().cloned();if factory.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"derived snapshot alias lost its issuer").into())}return semio_framework_value::RetirementPollExit::resume(ReturnedSnapshotReadRetirement::admit_original(&mut self.alias,&mut factory,&mut self.active,effect).map_err(Into::into))}
  if let Some(snapshots)=self.snapshots.take(){self.factory=Some(FactoryAuthority::new(snapshots));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),..Default::default()}))}
  if let Some(factory)=self.factory.as_mut(){
   if factory.terminal_is_empty(){self.factory=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<FactoryAuthority>>(),..Default::default()}))}
   return semio_framework_value::RetirementPollExit::resume(factory.step(effect))
  }
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<P:Send+Sync+'static> ErasedSnapshotRetirement for SnapshotAliasRetirement<P>{
 fn requires_original_control(&self)->bool{if let Some(active)=self.active.as_ref(){return !active.terminal_is_empty()&&active.requires_original_control()}if self.registry.is_some()||self.alias.is_some()||self.snapshots.is_some(){return false}self.factory.as_ref().is_some_and(|factory|!factory.terminal_is_empty()&&factory.requires_original_control())}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}
  if self.registry.is_some(){return snapshot_registry_alias_work_demand(&self.registry)}
  if self.alias.is_some(){return Ok(ReturnedSnapshotReadRetirement::<P>::admit_original_source_work_demand(&self.alias))}
  if self.snapshots.is_some(){return Ok(1)}
  self.factory.as_ref().map_or(Ok(0),|factory|if factory.terminal_is_empty(){Ok(1)}else{factory.next_computational_work(body)})
 }
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(active)=self.active.as_ref(){return semio_framework_value::factory_ticket_demands(active,body)}if self.registry.is_some(){return snapshot_registry_alias_demands(&self.registry)}if self.alias.is_some(){return Ok(RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()})}if self.snapshots.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()})}self.factory.as_ref().map_or(Ok(Default::default()),|factory|if factory.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryAuthority>>(),depth:1,..Default::default()})}else{factory.demands(body)})}
 fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{let result=self.step(effect,AliasCloseControl::Uncontrolled);semio_framework_value::RetirementPollExit::settle(effect,result)}
 fn close_step_controlled(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,c:&mut dyn OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll{let result=self.step(effect,AliasCloseControl::Controlled(c));semio_framework_value::RetirementPollExit::settle(effect,result)}
 fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{let result=self.step(effect,AliasCloseControl::Original(original));semio_framework_value::RetirementPollExit::settle(effect,result)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{Ok(self.next_demand(b)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.depth)}
 fn terminal_is_empty(&self)->bool{self.alias.is_none()&&self.registry.is_none()&&self.active.is_none()&&self.snapshots.is_none()&&self.factory.is_none()}
}
impl<P> Drop for SnapshotAliasRetirement<P>{fn drop(&mut self){assert!(std::thread::panicking()||(self.alias.is_none()&&self.registry.is_none()&&self.active.is_none()&&self.snapshots.is_none()&&self.factory.is_none()),"derived snapshot alias abandoned original cleanup custody");}}
