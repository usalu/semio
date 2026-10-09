//! 🌱️ Original bounded initializer cancellation carries its envelope, catalog and actor through granted frontiers.
use super::*;
use semio_framework_value::{ToValue,FromValue,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of,size_of_val};
struct BoundedStoreInitializationAuthority<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+RetireOwned+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+'static{
 operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,schema:&'static str,
 envelope:ManuallyDrop<Option<store::ArtifactEnvelope<P,M>>>,owners:ManuallyDrop<Option<store::DocumentStoreOwners<P,M>>>,retirement:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
 actor:ManuallyDrop<Option<protocol::ActorId>>,actor_close:ManuallyDrop<Option<ControlledRetirement<protocol::ActorId>>>,closing:bool,terminal:bool,
}
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"initializer retained child depth overflow"))?;Ok(demand)}
impl<P,M> BoundedStoreInitializationAuthority<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+RetireOwned+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+'static{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>(),release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};}
  if self.envelope.is_some(){
   let Some(owners)=self.owners.as_ref()else{return Ok(RetirementDemand{copy_bytes:size_of::<Option<store::DocumentStoreOwners<P,M>>>(),capacity_bytes:store::bounded_artifact_store_owners_birth_bytes::<P,M>(),depth:1,..Default::default()});};
   if !owners.constructor_is_complete(){return nested(owners.constructor_demands()?);}
   let mut demand=nested(owners.uninstalled_envelope_retirement_demands(self.envelope.as_ref().unwrap()))?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"initializer original frame copy overflow"))?;return Ok(demand);
  }
  if let Some(owners)=self.owners.as_ref(){return if owners.uninstalled_owners_terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<store::DocumentStoreOwners<P,M>>>(),depth:1,..Default::default()})}else{nested(owners.uninstalled_owners_demands(body)?)};}
  if let Some(actor)=self.actor_close.as_ref(){return if actor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<protocol::ActorId>>>(),depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:actor.next_copy_byte_demand()?,capacity_bytes:actor.next_capacity_byte_demand(body)?,release_bytes:actor.next_release_byte_demand()?,depth:actor.next_depth_demand()?})};}
  if self.actor.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<protocol::ActorId>>()+size_of::<Option<ControlledRetirement<protocol::ActorId>>>(),depth:1,..Default::default()});}
  Ok(Default::default())
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}let demand=self.demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
  if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){drop(self.retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));}let step=owner.close_step(child)?;let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original bounded initializer envelope")?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.envelope.is_some(){
   if self.owners.is_none(){let placement=demand.copy_bytes;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};return match store::bounded_artifact_store_owners::<P,M>(child){Ok((owners,mut receipt))=>{*self.owners=Some(owners);receipt.copied_bytes+=placement;Ok(RetainedCloneStep::Progress(receipt))},Err(refused)=>{*self.owners=refused.owners;Err(refused.error.with_retained_progress(refused.progress))}};}
   let owners=self.owners.as_mut().unwrap();if !owners.constructor_is_complete(){return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error,receipt)|error.with_retained_progress(receipt));}
   let placement=size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();let child=RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-placement,..child};let owners=self.owners.take().unwrap();let envelope=self.envelope.take().unwrap();return match owners.retire_envelope_uninstalled(envelope,child){Ok((owner,mut receipt))=>{*self.retirement=Some(owner);receipt.copied_bytes+=placement;Ok(RetainedCloneStep::Progress(receipt))},Err((error,owners,envelope))=>{*self.owners=Some(owners);*self.envelope=Some(envelope);Err(error)}};
  }
  if let Some(owners)=self.owners.as_mut(){if owners.uninstalled_owners_terminal_is_empty(){drop(self.owners.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let step=owners.close_uninstalled_owners_step(child)?;return admit_retained_clone_close(child,step,owners.uninstalled_owners_terminal_is_empty(),"initializer original catalog").map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(actor)=self.actor_close.as_mut(){if actor.terminal_is_empty(){drop(self.actor_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let step=actor.step(child)?;return admit_retained_clone_close(child,step,actor.terminal_is_empty(),"initializer original actor").map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(actor)=self.actor.take(){return match ControlledRetirement::new(actor){Ok(owner)=>{*self.actor_close=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))},Err((error,actor))=>{*self.actor=Some(actor);Err(error)}};}
  self.terminal=true;Ok(RetainedCloneStep::Complete(empty))
 }
}
impl<P,M> ArtifactStoreInitializationAuthority<P,M> for BoundedStoreInitializationAuthority<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+RetireOwned+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+'static{
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.closing=true;self.close_original(grant)}
 fn begin_close(&mut self){self.closing=true;}
 fn request_cancel(&mut self){self.closing=true;}
 fn take_candidate(&mut self)->Option<ArtifactStore<P,M>>{None}
 fn terminal_is_empty(&self)->bool{self.terminal&&self.envelope.is_none()&&self.owners.is_none()&&self.retirement.is_none()&&self.actor.is_none()&&self.actor_close.is_none()}
 fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->semio_framework_job::StepOutcome{
  if cx.operation()!=self.operation||cx.generation()!=self.generation{self.closing=true;}
  if self.closing{let step=self.close_original(cx.retained_grant());match step{Ok(step)=>{if cx.consume_retained(step.progress()).is_err(){return semio_framework_job::StepOutcome::Yield;}cx.consume_fuel(u64::from(step.progress().copied_items>0));if self.terminal_is_empty(){semio_framework_job::StepOutcome::Cancelled}else{semio_framework_job::StepOutcome::Yield}},Err(error)=>{let _=cx.consume_retained(error.retained_progress());semio_framework_job::StepOutcome::Yield}}}
  else{let valid=self.envelope.as_ref().is_some_and(|envelope|envelope.schema==self.schema&&!envelope.id.is_empty());let message:&[u8]=if valid{b"bounded-store.original-incremental-semantic-initializer-required"}else{b"bounded-store.original-envelope-invalid"};match cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault,message){Ok(detail)=>semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault{detail}),Err(_)=>semio_framework_job::StepOutcome::Yield}}
 }
}
impl<P,M> Drop for BoundedStoreInitializationAuthority<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+RetireOwned+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+'static{
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"bounded initializer retains original envelope/catalog/actor until granted closure");}
}
/// 🌱️ Captures the original completed envelope for genuine cancellation and an explicit semantic ingress frontier.
pub fn bounded_document_store_initialization_job<P,M>(envelope:store::ArtifactEnvelope<P,M>,schema:&'static str,operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,actor:protocol::ActorId)->ArtifactStoreInitializationJob<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+RetireOwned+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+'static{
 ArtifactStoreInitializationJob::new(Box::new(BoundedStoreInitializationAuthority{operation,generation,schema,envelope:ManuallyDrop::new(Some(envelope)),owners:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),actor:ManuallyDrop::new(Some(actor)),actor_close:ManuallyDrop::new(None),closing:false,terminal:false}))
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
