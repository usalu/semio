//! 🔗️ Captured snapshot sources keep the original Store read and its installed typed retirement issuer.
use super::{SnapshotRead,SnapshotReadReturn,SnapshotReadLeaseRefusal,SnapshotReadRegistryAliasRetirement,try_return_original_snapshot_read,snapshot_registry_alias_demands,snapshot_registry_alias_work_demand,snapshot_registry_alias_close_step,advance_returned_snapshot_read,artifact_retirement_box_demands,artifact_retirement_box_close_step,original_outcome_retirement_step};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

/// 🧳️ The projected snapshot needs no intrinsic retirement when its original Store issuer owns that capability.
pub struct CapturedSnapshotSource<T:Send+Sync+'static>{
 read:ManuallyDrop<Option<SnapshotRead<T>>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
}
struct CapturedSnapshotRetirement<T:Send+Sync+'static>{
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
 pub fn admit(read:&mut Option<SnapshotRead<T>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(None),work:0}}let Some(original)=read.take()else{return RetainedWorkOutcome{result:Err(invalid("captured snapshot source requires its original Store read")),work:0}};
  RetainedWorkOutcome{result:Ok(Some((Self{read:ManuallyDrop::new(Some(original)),factory:ManuallyDrop::new(Some(Arc::clone(factory)))},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work:2}
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
 fn step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
  let refuse=|error:ValueError|RetainedWorkOutcome{result:Err(error.into()),work:0};
  let moved=|progress:RetainedCloneProgress|RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work:1};
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}let d=match self.close_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return refuse(error)};if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  if let Some(read)=self.read.as_mut(){let witness=match try_return_original_snapshot_read(&mut read.owner,&mut read.lease){Ok(witness)=>witness,Err(())=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}};*self.registry=Some(SnapshotReadRegistryAliasRetirement::new(witness.registry));self.read.take();return moved(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
  if self.active.is_some(){return artifact_retirement_box_close_step(&mut self.active,child);}
  if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){let Some(factory)=self.factory.as_ref()else{return refuse(invalid("captured snapshot source lost its installed issuer"))};return advance_returned_snapshot_read(registry,&mut self.active,factory,child);}return snapshot_registry_alias_close_step(&mut self.registry,grant);}
  if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));return moved(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});}
  let Some(factory)=self.factory_close.as_mut()else{return refuse(invalid("captured snapshot source lost its final issuer cursor"))};if factory.terminal_is_empty(){self.factory_close=None;return moved(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}let turn=factory.step(grant);
  RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}
 }
 /// 🫙️ Every original read, registry backing, typed payload ticket and issuer has returned.
 fn empty(&self)->bool{self.read.is_none()&&self.registry.is_none()&&self.active.is_none()&&self.factory.is_none()&&self.factory_close.is_none()}
}
impl<T:Send+Sync+'static> Drop for CapturedSnapshotSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.read.is_none()&&self.factory.is_none(),"captured snapshot source requires ownership transfer into its granted retirement cursor");}}
impl<T:Send+Sync+'static> Drop for CapturedSnapshotRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"captured snapshot cursor requires original registry return and granted issuer closure");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factory);}}}}
impl<T:Send+Sync+'static> RetireOwned for CapturedSnapshotSource<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(CapturedSnapshotRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CapturedSnapshotRetirement<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:Send+Sync+'static> RetirementCursor for CapturedSnapshotRetirement<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.read.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured read return lacks a native work quote"))}if let Some(owner)=self.active.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"returned read admission lacks a native work quote"))}return snapshot_registry_alias_work_demand(&self.registry)}if self.factory.is_some(){return Ok(1)}match self.factory_close.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
 fn requires_original_control(&self)->bool{self.read.is_none()&&(self.active.is_some()||(self.registry.is_none()&&self.factory.is_none()&&self.factory_close.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::retirement::RetirementStep{let grant=effect.grant();use semio_framework_value::retirement::RetirementStep;if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured source has no selected original child").into(),0)}let d=match self.close_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}let turn=if self.active.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};semio_framework_value::close_factory_ticket_controlled(&mut self.active,child,control)}else{match self.factory_close.as_mut(){Some(owner)=>owner.step_controlled(grant,control),None=>return RetirementStep::Failure(invalid("captured source lost original factory").into(),0)}};original_outcome_retirement_step(turn)}
 fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{let grant=effect.grant();use semio_framework_value::retirement::RetirementStep;if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured source has no selected original child").into(),0)}let d=match self.close_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}let turn=if self.active.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};semio_framework_value::close_factory_ticket_original(&mut self.active,original,child)}else{match self.factory_close.as_mut(){Some(owner)=>owner.step_original(original,grant),None=>return RetirementStep::Failure(invalid("captured source lost original factory").into(),0)}};original_outcome_retirement_step(turn)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{let grant=effect.grant();original_outcome_retirement_step(self.step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect")))}
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
