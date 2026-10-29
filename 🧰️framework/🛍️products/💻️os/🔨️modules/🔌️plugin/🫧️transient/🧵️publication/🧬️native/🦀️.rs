//! 🧬️ Ephemeral publication uses the same original retained semantics and central diff producer as document publication.
use super::*;
use std::mem::{size_of,ManuallyDrop};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,ErasedSnapshotRetirement,OriginalRetirementControl,RetainedCloneSource,RetainedOwnedProjection,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use semio_framework_value_snapshot_read::{SnapshotRead,CapturedSnapshotSource};
use semio_framework_os_store_replay_operation::{ArtifactDerivedOperation,ArtifactDerivedOperationMode,ArtifactReplayPreparationFactory,ArtifactReplayPreparationRequest,ArtifactReplayPreparationTask,ArtifactReplayPreparationContext,ArtifactReplayPreparationMode};

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Factory<P:'static,M:Sync+'static>{#[factory_child] snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,#[factory_child] mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,#[factory_child] semantics:Arc<dyn ArtifactReplayPreparationFactory<P,M>>}

struct Preparation<P:Send+Sync+'static,M:Send+Sync+'static>{
 base:ManuallyDrop<Option<SnapshotRead<P>>>,mutation:ManuallyDrop<Option<M>>,
 authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,input:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 native:ManuallyDrop<Option<ArtifactDerivedOperation<P,M>>>,prepared:ManuallyDrop<Option<store::ArtifactEphemeralOneItemPrepared<P>>>,
 captured:ManuallyDrop<Option<CapturedSnapshotSource<P>>>,retirement:RetainedCloneClose,active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
 snapshots:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,mutations:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,semantics:ManuallyDrop<Option<Arc<dyn ArtifactReplayPreparationFactory<P,M>>>>,factories:[Option<FactoryAuthority>;5],
 context:ArtifactReplayPreparationContext,checkpoint:store::os_store::one_item::ArtifactStoreOneItemCheckpoint,original:Option<(u64,Option<u64>)>,phase:u8,closing:bool,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<P:Send+Sync+'static,M:Send+Sync+'static> Factory<P,M>{
 fn construct(&self,request:store::ArtifactEphemeralOneItemPreparationRequest<P,M>)->Result<Preparation<P,M>,store::ArtifactEphemeralOneItemPreparationRequest<P,M>>{
  if matches!(request.base.0,store::ArtifactEphemeralBaseOwner::Transient(_)){return Err(request);}
  let base=match request.base.0{store::ArtifactEphemeralBaseOwner::Presence(read)|store::ArtifactEphemeralBaseOwner::TransientRead(read)=>read,store::ArtifactEphemeralBaseOwner::Transient(_)=>unreachable!()};
  Ok(Preparation{base:ManuallyDrop::new(Some(base)),mutation:ManuallyDrop::new(Some(request.mutation)),authority:ManuallyDrop::new(None),source:ManuallyDrop::new(None),input:ManuallyDrop::new(None),native:ManuallyDrop::new(None),prepared:ManuallyDrop::new(None),captured:ManuallyDrop::new(None),retirement:Default::default(),active:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(&self.snapshots))),mutations:ManuallyDrop::new(Some(Arc::clone(&self.mutations))),semantics:ManuallyDrop::new(Some(Arc::clone(&self.semantics))),factories:Default::default(),context:ArtifactReplayPreparationContext{mode:ArtifactReplayPreparationMode::Prefix,operation_index:0,generation:request.generation.0,base_revision:[0;32]},checkpoint:Default::default(),original:None,phase:0,closing:false})
 }
}
impl<P:Send+Sync+'static,M: ::semio_framework_os_kernel::StoreMutationFor<P>> store::ArtifactEphemeralOneItemPreparationFactory<P,M> for Factory<P,M>{
 fn preflight(&self,_:&M)->Result<store::os_store::one_item::ArtifactStoreOneItemFootprint,String>{Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<M>()))}
 fn begin(&self,request:store::ArtifactEphemeralOneItemPreparationRequest<P,M>)->Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<P,M>>,store::ArtifactEphemeralOneItemPreparationRequest<P,M>>{self.construct(request).map(|owner|Box::new(owner)as Box<dyn store::ArtifactEphemeralOneItemPreparation<P,M>>)}
}
impl<P:Send+Sync+'static,M:Send+Sync+'static> Preparation<P,M>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  Ok(match self.phase{
   0=>header::<FactoryOwnedSourceAuthority<M>>(),
   1=>{let birth=RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<M>>();RetirementDemand{copy_bytes:RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::borrowed_constructor_copy_bytes::<FactoryOwnedSourceAuthority<M>>(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()}},
   2=>RetainedOwnedProjection::<M>::constructor_demand(),
   3=>{let mut d=self.semantics.as_ref().ok_or_else(||invalid("ephemeral original semantic issuer absent"))?.preparation_birth_demand();d.copy_bytes+=size_of::<ArtifactDerivedOperation<P,M>>()+size_of::<ArtifactReplayPreparationRequest<P,M>>();Ok::<_,ValueError>(d)?},
   4=>self.native.as_ref().ok_or_else(||invalid("ephemeral original semantic owner absent"))?.demands(self.context.generation,self.context.base_revision,body)?,
   5=>header::<store::ArtifactEphemeralOneItemPrepared<P>>(),
   _=>Default::default(),
  })
 }
 fn empty(&self)->bool{self.base.is_none()&&self.mutation.is_none()&&self.authority.is_none()&&self.source.is_none()&&self.input.is_none()&&self.native.is_none()&&self.prepared.is_none()&&self.captured.is_none()&&self.retirement.is_empty()&&self.active.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.semantics.is_none()&&self.factories.iter().all(Option::is_none)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.native.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<ArtifactDerivedOperation<P,M>>>())}else{owner.demands(self.context.generation,self.context.base_revision,body)};}
  if let Some(owner)=self.active.as_ref(){return semio_framework_value::factory_ticket_demands(owner,body);}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<CapturedSnapshotSource<P>>(false,&None,body);}
  if let Some(owner)=self.input.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{owner.close_demand(body)};}
  if let Some(owner)=self.source.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{owner.next_close_demand(body)};}
  if self.authority.is_some(){return self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(true,&None,body);}
  if self.base.is_some(){return Ok(CapturedSnapshotSource::<P>::constructor_demand());}
  if self.captured.is_some(){return self.retirement.next_owner_demands::<CapturedSnapshotSource<P>>(true,&None,body);}
  if self.mutation.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  if self.prepared.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<store::ArtifactEphemeralOneItemPrepared<P>>(),capacity_bytes:semio_framework_value::factory::shared::FactorySharedRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  if self.snapshots.is_some()||self.mutations.is_some()||self.semantics.is_some(){return Ok(header::<FactoryAuthority>());}
  self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|owner|owner.demands(body))
 }
 fn close(&mut self,grant:RetainedCloneGrant,mut control:Option<&mut dyn OriginalRetirementControl>)->Result<RetainedCloneStep,ValueError>{
  if self.empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.original.is_some()&&control.as_ref().is_none_or(|control|self.original!=Some((control.ledger_identity(),control.retirement_recipient_identity()))){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"ephemeral retirement requires its original decoder and recipient"));}
  if let Some(owner)=self.native.as_mut(){if owner.terminal_is_empty(){self.native.take();return Ok(RetainedCloneStep::Progress(progress(demand)));}return owner.close_step_controlled(grant,&mut **control.as_mut().ok_or_else(||invalid("ephemeral native retirement lacks original control"))?);}
  if self.active.is_some(){return match control{Some(control)=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,grant,control),None=>semio_framework_value::close_factory_ticket(&mut self.active,grant)};}
  if let Some(control)=control.as_mut(){control.charge(demand.capacity_bytes)?;}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}
  if let Some(owner)=self.input.as_mut(){if owner.terminal_is_empty(){self.input.take();return Ok(RetainedCloneStep::Progress(progress(demand)));}return owner.close_step(grant);}
  if let Some(owner)=self.source.as_mut(){if owner.terminal_is_empty(){self.source.take();return Ok(RetainedCloneStep::Progress(progress(demand)));}return owner.close_step(grant);}
  if let Some(step)=self.retirement.begin_granted(&mut self.authority,grant)?{return Ok(step);}
  if self.base.is_some(){let Some((owner,receipt))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.captured=Some(owner);return Ok(RetainedCloneStep::Progress(receipt));}
  if let Some(step)=self.retirement.begin_granted(&mut self.captured,grant)?{return Ok(step);}
  if self.mutation.is_some(){let Some((owner,receipt))=semio_framework_value::FactoryOwnedRetirement::admit_original(&mut self.mutation,self.mutations.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.active=Some(owner);return Ok(RetainedCloneStep::Progress(receipt));}
  if let Some(prepared)=self.prepared.take(){let mut original=Some(prepared.next_root);match semio_framework_value::factory::shared::FactorySharedRetirement::admit_original(&mut original,self.snapshots.as_ref().unwrap(),grant){Ok(Some((owner,mut receipt)))=>{*self.active=Some(owner);receipt.copied_bytes+=demand.copy_bytes;return Ok(RetainedCloneStep::Progress(receipt));},Ok(None)=>{*self.prepared=Some(store::ArtifactEphemeralOneItemPrepared{next_root:original.unwrap()});return Ok(RetainedCloneStep::Progress(Default::default()));},Err(error)=>{*self.prepared=Some(store::ArtifactEphemeralOneItemPrepared{next_root:original.unwrap()});return Err(error);}}}
  if let Some(owner)=self.snapshots.take(){self.factories[0]=Some(FactoryAuthority::new(owner));return Ok(RetainedCloneStep::Progress(progress(demand)));}
  if let Some(owner)=self.mutations.take(){self.factories[1]=Some(FactoryAuthority::new(owner));return Ok(RetainedCloneStep::Progress(progress(demand)));}
  if let Some(owner)=self.semantics.take(){self.factories[2]=Some(FactoryAuthority::new(owner));return Ok(RetainedCloneStep::Progress(progress(demand)));}
  if let Some(slot)=self.factories.iter_mut().find(|slot|slot.is_some()){let owner=slot.as_mut().unwrap();let step=owner.step(grant)?;if owner.terminal_is_empty(){*slot=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<P:Send+Sync+'static,M:Send+Sync+'static> store::ArtifactEphemeralOneItemPreparation<P,M> for Preparation<P,M>{
 fn advance(&mut self,grant:store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep,ValueError>{
  use store::os_store::one_item::ArtifactStoreOneItemPreparationStep as Step;
  if self.closing||!grant.permits_one(){return Ok(Step::Blocked);}
  if self.prepared.is_some(){return Ok(Step::Prepared(self.checkpoint,Default::default()));}
  let grant=grant.retained_grant();let demand=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(Step::Blocked);}
  let identity=(receiving.ledger_identity(),receiving.retirement_recipient_identity());if self.original.is_some_and(|prior|prior!=identity){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"ephemeral preparation requires its original decoder and recipient"));}self.original=Some(identity);receiving.checkpoint()?;
  if self.phase!=4{receiving.admit_turn_capacity(grant.maximum_capacity_bytes)?;receiving.charge(demand.capacity_bytes)?;}
  let receipt=match self.phase{
   0=>{*self.authority=Some(FactoryOwnedSourceAuthority::new(self.mutation.take().unwrap(),Arc::clone(self.mutations.as_ref().unwrap())));self.phase=1;progress(demand)},
   1=>{let authority=self.authority.take().unwrap();match RetainedCloneSource::admit_borrowed(authority,FactoryOwnedSourceAuthority::get,grant){Ok((source,receipt))=>{*self.source=Some(source);self.phase=2;receipt},Err((error,authority))=>{*self.authority=Some(authority);return Err(error);}}},
   2=>{let(input,receipt)=self.source.as_ref().unwrap().retain_original(grant)?;*self.input=Some(input);self.phase=3;receipt},
   3=>{

    let factory=self.semantics.as_ref().unwrap();let parent=demand.copy_bytes-factory.preparation_birth_demand().copy_bytes;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,..grant};
    let request=ArtifactReplayPreparationRequest{base:self.base.take().unwrap(),input:self.input.take().unwrap(),context:self.context,snapshots:Arc::clone(self.snapshots.as_ref().unwrap()),mutations:Arc::clone(self.mutations.as_ref().unwrap())};
    match factory.begin(request,child){Ok((cursor,mut receipt))=>{let mut owner=ArtifactDerivedOperation{base:ManuallyDrop::new(None),producer:ManuallyDrop::new(None),registry:None,context:self.context,failed_read:ManuallyDrop::new(None),failed_input:ManuallyDrop::new(None),registry_close:ManuallyDrop::new(None),mode:ArtifactDerivedOperationMode::Provisional,policy:protocol::MergePolicy::default(),task:ManuallyDrop::new(None),task_close:ManuallyDrop::new(None),prepared:ManuallyDrop::new(None),mutation:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(self.snapshots.as_ref().unwrap()))),mutations:ManuallyDrop::new(Some(Arc::clone(self.mutations.as_ref().unwrap()))),factory_close:Default::default(),message:0,accepted:true,ready:false,finished:false,closing:false,progress:Default::default()};*owner.task=Some(ArtifactReplayPreparationTask::new(self.context,cursor));*self.native=Some(owner);self.phase=4;receipt.copied_bytes+=parent;receipt},Err((error,request))=>{*self.base=Some(request.base);*self.input=Some(request.input);self.factories[3]=Some(FactoryAuthority::new(request.snapshots));self.factories[4]=Some(FactoryAuthority::new(request.mutations));return Err(error);}}
   },
   4=>{let owner=self.native.as_mut().unwrap();let step=owner.step(self.context.generation,self.context.base_revision,grant.maximum_copy_bytes,grant,receiving,&mut||false)?;if owner.is_finished(){self.phase=5;}step.progress()},
   5=>{let owner=self.native.as_mut().unwrap();let next=owner.prepared.as_mut().and_then(|prepared|prepared.next.take());owner.cancel();if let Some(next_root)=next{*self.prepared=Some(store::ArtifactEphemeralOneItemPrepared{next_root});self.phase=6;}else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"ephemeral original retained semantics refused its captured base"));}progress(demand)},
   _=>return Err(invalid("ephemeral native preparation has an invalid frontier")),
  };
  if !receipt.fits(grant){return Err(invalid("ephemeral native producer exceeded original grant").with_retained_progress(receipt));}
  self.checkpoint.completed_items=self.checkpoint.completed_items.saturating_add(receipt.copied_items as u32);self.checkpoint.completed_bytes=self.checkpoint.completed_bytes.saturating_add(receipt.copied_bytes as u64);
  Ok(if self.prepared.is_some(){Step::Prepared(self.checkpoint,receipt)}else{Step::Progress(self.checkpoint,receipt)})
 }
 fn checkpoint(&self)->store::os_store::one_item::ArtifactStoreOneItemCheckpoint{self.checkpoint}
 fn prepared(&self)->Option<&store::ArtifactEphemeralOneItemPrepared<P>>{self.prepared.as_ref()}
 fn take_prepared(&mut self)->Option<store::ArtifactEphemeralOneItemPrepared<P>>{self.prepared.take()}
 fn cancel(&mut self){self.begin_close();}
 fn begin_close(&mut self){self.closing=true;if let Some(owner)=self.native.as_mut(){owner.cancel();}}
 fn close_step(&mut self,grant:store::os_store::one_item::ArtifactStoreOneItemGrant)->Result<RetainedCloneStep,ValueError>{self.close(grant.retained_grant(),None)}
 fn close_step_controlled(&mut self,grant:store::os_store::one_item::ArtifactStoreOneItemGrant,control:&mut dyn OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{self.close(grant.retained_grant(),Some(control))}
 fn requires_original_control(&self)->bool{self.original.is_some()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.empty()}
}
impl<P:Send+Sync+'static,M:Send+Sync+'static> Drop for Preparation<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"ephemeral native publication abandoned original custody");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.base);ManuallyDrop::drop(&mut self.mutation);ManuallyDrop::drop(&mut self.authority);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.input);ManuallyDrop::drop(&mut self.native);ManuallyDrop::drop(&mut self.prepared);ManuallyDrop::drop(&mut self.captured);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.snapshots);ManuallyDrop::drop(&mut self.mutations);ManuallyDrop::drop(&mut self.semantics);}}}}
/// 🏭️ Uses the mutation family's existing original retained semantics and intrinsic snapshot issuer.
pub fn bounded_transient_preparation_factory<P,M>()->Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P,M>>
where P:semio_framework_value_snapshot_projection::SnapshotProjection<Output=P>+::semio_framework_os_kernel::StoreState,M:protocol::RetainedMutation<P>+store::ArtifactCanonicalJsonTree+::semio_framework_os_kernel::StoreMutationFor<P>{
 Arc::new(Factory{snapshots:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<P>::default()),mutations:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<M>::default()),semantics:Arc::new(store::PagedOneItemPreparationFactory::<P,M,store::MutationApplyEdit<P,M>>::default())})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
