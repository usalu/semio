//! 📦️ Original unpublished candidates retire their real edit, root, seal and metadata through captured issuers.
use crate::ArtifactStoreOneItemPreparedParts;
use std::sync::Arc;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError};
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use crate::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_value::{FactoryAuthority,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand}};
use std::mem::ManuallyDrop;
/// 🧳️ An owning cancellation cursor retains every original prepared allocation and declared issuer.
pub struct ArtifactStorePreparedRetirement<P,M>{pub fold_digest:[u8;32],pub fold_clock:HybridLogicalTimestamp,pub foreign_step_presence:Option<bool>,pub edit:ManuallyDrop<Option<Box<Edit<M>>>>,unboxed:ManuallyDrop<Option<Edit<M>>>,pub post:ManuallyDrop<Option<Arc<P>>>,pub authority:ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,pub strings:ManuallyDrop<[Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>;3]>,pub mutation_factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,pub snapshot_factory:ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,pub active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub factories:ManuallyDrop<[Option<FactoryAuthority>;2]>}
impl<P,M> ArtifactStoreOneItemPrepared<P,M>{
 /// 📏️ The exact cursor frame is priced before any original owner moves.
 pub fn retirement_birth_demand()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:size_of::<ArtifactStorePreparedRetirement<P,M>>(),depth:1}}
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStoreOneItemPrepared<P,M>{
 /// 🎟️ Refusal returns the actual candidate and both already captured issuers unchanged.
 pub fn admit_retirement(self,mutation_factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshot_factory:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Self,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)>{self.admit_fold_retirement(mutation_factory,snapshot_factory,grant).map(|(owner,receipt)|(owner as Box<dyn ErasedSnapshotRetirement>,receipt))}
  pub fn admit_fold_retirement(self,mutation_factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshot_factory:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<ArtifactStorePreparedRetirement<P,M>>,RetainedCloneProgress),(ValueError,Self,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)>{
  let receipt=match Self::retirement_birth_demand().admit(grant){Ok(receipt)=>receipt,Err(error)=>return Err((error,self,mutation_factory,snapshot_factory))};let ArtifactStoreOneItemPreparedParts{outcome_messages,edit,post_snapshot,local_actor,applied_edit_id,tail_edit_id,authority,edit_digest,next_clock,foreign_step_presence}=self.into_parts();let owner=ArtifactStorePreparedRetirement{outcome_messages:ManuallyDrop::new(outcome_messages),fold_digest:edit_digest,fold_clock:next_clock,foreign_step_presence,edit:ManuallyDrop::new(Some(edit)),unboxed:ManuallyDrop::new(None),post:ManuallyDrop::new(Some(post_snapshot)),authority:ManuallyDrop::new(Some(authority)),strings:ManuallyDrop::new([local_actor,Some(applied_edit_id),Some(tail_edit_id)]),mutation_factory:ManuallyDrop::new(Some(mutation_factory)),snapshot_factory:ManuallyDrop::new(Some(snapshot_factory)),active:ManuallyDrop::new(None),factories:ManuallyDrop::new(Default::default())};Ok((Box::new(owner),receipt))
 }
}
#[cfg(feature = "durable-group-testing")]
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStorePreparedRetirement<P,M> {
 /// 🫴️ Admits the actual returned fixture edit and post with their original issuers before retaining its cursor frame.
 pub fn admit_returned(edit:Edit<M>,post:Arc<P>,mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,snapshots:Arc<dyn SnapshotRetirementFactory<P>>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Edit<M>,Arc<P>,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<P>>)> {
  let receipt=match ArtifactStoreOneItemPrepared::<P,M>::retirement_birth_demand().admit(grant){Ok(receipt)=>receipt,Err(error)=>return Err((error,edit,post,mutations,snapshots))};
  Ok((Box::new(Self{outcome_messages:ManuallyDrop::new(None),fold_digest:[0;32],fold_clock:HybridLogicalTimestamp{physical_ms:0,logical:0,actor:0},foreign_step_presence:None,edit:ManuallyDrop::new(None),unboxed:ManuallyDrop::new(Some(edit)),post:ManuallyDrop::new(Some(post)),authority:ManuallyDrop::new(None),strings:ManuallyDrop::new(Default::default()),mutation_factory:ManuallyDrop::new(Some(mutations)),snapshot_factory:ManuallyDrop::new(Some(snapshots)),active:ManuallyDrop::new(None),factories:ManuallyDrop::new(Default::default())}),receipt))
 }
}
impl<P,M> ArtifactStorePreparedRetirement<P,M>{
 pub fn empty(&self)->bool{self.outcome_messages.is_none()&&self.edit.is_none()&&self.unboxed.is_none()&&self.post.is_none()&&self.authority.is_none()&&self.strings.iter().all(Option::is_none)&&self.mutation_factory.is_none()&&self.snapshot_factory.is_none()&&self.active.is_none()&&self.factories.iter().all(Option::is_none)}
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactStorePreparedRetirement<P,M>{
 /// 📐️ Every physical child extent is admitted independently from logical copy work.
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared retirement depth overflow"))?;Ok(demand)};
  if let Some(messages)=self.outcome_messages.as_ref(){return nested(messages.close_demands()?);}
  if let Some(active)=self.active.as_ref(){return nested(artifact_retirement_box_demands(active,body)?);}
  if self.edit.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Edit<M>>(),release_bytes:size_of::<Edit<M>>(),depth:1,..Default::default()});}
  if self.unboxed.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<ArtifactStoreDecodedEditRetirement<M>>(),depth:2,..Default::default()});}
  if let Some(post)=self.post.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.snapshot_factory.as_ref().expect("prepared original root issuer").retirement_birth_bytes(post),depth:2,..Default::default()});}
  if self.strings.iter().any(Option::is_some){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>(),depth:2,..Default::default()});}
  if let Some(authority)=self.authority.as_ref(){let birth=authority.retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
  if self.mutation_factory.is_some()||self.snapshot_factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
  self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|factory|nested(factory.demands(body)?))
 }
}
impl<P:Send+Sync+'static,M:Send+'static> ErasedSnapshotRetirement for ArtifactStorePreparedRetirement<P,M>{
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared retirement exceeds admitted depth"));}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(messages)=self.outcome_messages.as_mut(){messages.begin_close();let step=messages.close_step(child)?;if messages.terminal_is_empty(){*self.outcome_messages=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.active.is_some(){return artifact_retirement_box_close_step(&mut self.active,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(edit)=self.edit.take(){*self.unboxed=Some(*edit);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
  if let Some(edit)=self.unboxed.take(){let factory=self.mutation_factory.as_ref().expect("prepared original mutation issuer");return match admit_artifact_retirement(edit,child,|original|ArtifactStoreDecodedEditRetirement::new(original,Arc::clone(factory))){Ok((active,receipt))=>{*self.active=Some(active);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*self.unboxed=Some(original);Err(error)}};}
  if let Some(post)=self.post.take(){return match self.snapshot_factory.as_ref().expect("prepared original root issuer").retire(post,child){Ok((active,receipt))=>{*self.active=Some(active);semio_framework_value::retained_clone::admit_retained_clone_progress(child,receipt,"prepared root issuer birth")?;if receipt.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"prepared root issuer changed admitted frame extent"));}Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*self.post=Some(original);Err(error)}};}
  if let Some(slot)=self.strings.iter_mut().find(|slot|slot.is_some()){let value=slot.take().unwrap();return match semio_framework_value::retirement::admit_owned_retirement(value,child){Ok((owner,receipt))=>{*self.active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*slot=Some(original);Err(error)}};}
  if let Some(authority)=self.authority.take(){return match authority.retire(child){Ok((active,receipt))=>{*self.active=Some(active);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*self.authority=Some(original);Err(error)}};}
  if let Some(factory)=self.mutation_factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.factories[0]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(factory)=self.snapshot_factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.factories[1]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  let slot=self.factories.iter_mut().find(|slot|slot.is_some()).expect("prepared original factory close");let factory=slot.as_mut().unwrap();let step=factory.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"prepared original factory")?;if factory.terminal_is_empty(){*slot=None;}Ok(if self.empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<P,M> Drop for ArtifactStorePreparedRetirement<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"prepared retirement retains original owners until funded terminal closure");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.outcome_messages);ManuallyDrop::drop(&mut self.edit);ManuallyDrop::drop(&mut self.unboxed);ManuallyDrop::drop(&mut self.post);ManuallyDrop::drop(&mut self.authority);ManuallyDrop::drop(&mut self.strings);ManuallyDrop::drop(&mut self.mutation_factory);ManuallyDrop::drop(&mut self.snapshot_factory);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factories);}}}}

