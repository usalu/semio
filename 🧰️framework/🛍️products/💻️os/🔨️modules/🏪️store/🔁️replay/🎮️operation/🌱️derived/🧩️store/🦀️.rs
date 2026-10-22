//! 🧩️ Store coordinator admissions bind original semantic producers to the component frontier.
use super::*;
use semio_framework_value::{RetirementDemand,ValueRefusalKind};
use std::mem::ManuallyDrop;
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool { grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth }
fn header<T>()->RetirementDemand { RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()} }
fn refusal(message:&'static str)->ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated,message) }
#[cfg(test)]
#[path="../🧪️tests/🦀️.rs"]
mod tests;
impl<P,M> ArtifactStore<P,M>
where P:Clone+ToValue+FromValue+ArtifactPack+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+semio_framework_os_spr::ColdMutation<P>+OpBinary+OpText+Send+'static{
    /// 🎟️ Quotes the installed original producer before any projection alias transfers.
    pub fn derived_operation_birth_demand(&self,source:&ArtifactDerivedSnapshot<P>)->Result<RetirementDemand,ValueError> {
        self.admit_derived_source(&source.registry,source.generation,source.revision).map_err(|_|refusal("derived operation requires this Store's current original source"))?;
        self.owned_mutation_retirement_factory()?;
        if self.owners.initial_snapshot_retirement_factory.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"derived operation requires its installed snapshot cleanup issuer"));}
        let factory=self.owners.replay_preparation_factory.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"derived operation requires its installed semantic factory"))?;
        let mut demand=factory.preparation_birth_demand();
        demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<ArtifactDerivedOperation<P,M>>()+SnapshotRead::<P>::capture_copy_byte_demand()+4*size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>()).ok_or_else(||refusal("derived operation capture overflow"))?;
        Ok(demand)
    }
    /// 🌱️ Moves a genuine retained input and original Store read into its installed semantic producer.
    pub fn begin_derived_operation(&self,slot:&mut Option<ArtifactDerivedOperation<P,M>>,source:&ArtifactDerivedSnapshot<P>,input:&mut Option<semio_framework_value::retained_clone::RetainedOwnedProjection<M>>,operation_index:u32,mode:ArtifactDerivedOperationMode,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError> {
        if slot.is_some(){return Err(refusal("derived operation slot retains its original producer"));}
        let demand=self.derived_operation_birth_demand(source)?;if !funded(grant,demand){return Ok(None);}
        if input.is_none(){return Err(refusal("derived operation requires a genuine original input projection"));}
        let context=ArtifactReplayPreparationContext{mode:ArtifactReplayPreparationMode::Prefix,operation_index,generation:source.generation,base_revision:source.revision};
        let alias=Arc::clone(&source.owner);let lease=match self.owners.snapshot_read_leases.try_issue(Arc::clone(&alias)){Ok(lease)=>lease,Err(alias)=>{drop(alias);return Ok(None);}};
        let request=ArtifactReplayPreparationRequest{base:SnapshotRead::new(alias,lease),input:input.take().unwrap(),context,snapshots:Arc::clone(self.owners.initial_snapshot_retirement_factory.as_ref().unwrap()),mutations:Arc::clone(self.owners.mutation_retirement_factory.as_ref().unwrap())};
        let factory=self.owners.replay_preparation_factory.as_ref().unwrap();let birth=factory.preparation_birth_demand();let parent_copy=demand.copy_bytes.checked_sub(birth.copy_bytes).ok_or_else(||refusal("original constructor demand moved backwards"))?;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,..grant};
        let mut owner=ArtifactDerivedOperation{registry:Some(source.registry.clone()),context,failed_read:ManuallyDrop::new(None),failed_input:ManuallyDrop::new(None),registry_close:ManuallyDrop::new(None),mode,policy:self.merge_policy(),task:ManuallyDrop::new(None),task_close:ManuallyDrop::new(None),prepared:ManuallyDrop::new(None),mutation:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(self.owners.initial_snapshot_retirement_factory.as_ref().unwrap()))),mutations:ManuallyDrop::new(Some(Arc::clone(self.owners.mutation_retirement_factory.as_ref().unwrap()))),factory_close:std::array::from_fn(|_|None),message:0,accepted:true,ready:false,finished:false,closing:false,progress:Default::default()};
        let result=factory.begin(request,child);
        let receipt=match result{Ok((cursor,p))=>{*owner.task=Some(ArtifactReplayPreparationTask::new(context,cursor));p},Err((error,request))=>{*owner.failed_read=Some(request.base.into_erased());*owner.failed_input=Some(request.input);owner.factory_close[2]=Some(semio_framework_value::FactoryAuthority::new(request.snapshots));owner.factory_close[3]=Some(semio_framework_value::FactoryAuthority::new(request.mutations));owner.cancel();let mut progress=error.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(parent_copy).ok_or_else(||refusal("original constructor refusal receipt overflow"))?;owner.record(progress);*slot=Some(owner);return Err(error.with_retained_progress(progress));}};
        let progress=RetainedCloneProgress{copied_items:receipt.copied_items.max(1),copied_bytes:receipt.copied_bytes.checked_add(parent_copy).ok_or_else(||refusal("original constructor success receipt overflow"))?,..receipt};owner.record(progress);
        *slot=Some(owner);if !receipt.fits(child)||receipt.retained_capacity_bytes!=demand.capacity_bytes{slot.as_mut().unwrap().cancel();return Err(refusal("derived operation factory changed its admitted birth receipt").with_retained_progress(progress));}Ok(Some(progress))
    }
    /// 📏️ Requires this Store's original issuer before exposing the next retained work frontier.
    pub fn derived_operation_demands(&self,owner:&ArtifactDerivedOperation<P,M>,body:usize)->Result<RetirementDemand,ValueError>{
        if !owner.registry.as_ref().is_none_or(|registry|registry.ptr_eq(&self.owners.snapshot_read_leases)){return Err(refusal("foreign derived operation issuer"));}owner.demands(self.generation(),self.content_revision(),body)
    }
    /// 🧳️ Keeps a refused replay request's original read, projection and exact issuers for paid closure.
    pub(crate) fn retain_refused_replay_request(&self,request:ArtifactReplayPreparationRequest<P,M>)->ArtifactDerivedOperation<P,M>{
        ArtifactDerivedOperation{registry:Some(self.owners.snapshot_read_leases.clone()),context:request.context,failed_read:ManuallyDrop::new(Some(request.base.into_erased())),failed_input:ManuallyDrop::new(Some(request.input)),registry_close:ManuallyDrop::new(None),mode:ArtifactDerivedOperationMode::HistoryPrefix,policy:self.merge_policy(),task:ManuallyDrop::new(None),task_close:ManuallyDrop::new(None),prepared:ManuallyDrop::new(None),mutation:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(request.snapshots)),mutations:ManuallyDrop::new(Some(request.mutations)),factory_close:std::array::from_fn(|_|None),message:0,accepted:false,ready:false,finished:false,closing:true,progress:Default::default()}
    }
    /// ⏯️ Supplies the current frontier to the original retained operation; closing needs no source reads.
    pub fn step_derived_operation(&self,owner:&mut ArtifactDerivedOperation<P,M>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<RetainedCloneStep,ValueError>{
        if !owner.registry.as_ref().is_none_or(|registry|registry.ptr_eq(&self.owners.snapshot_read_leases)){return Err(refusal("foreign derived operation issuer"));}owner.step(self.generation(),self.content_revision(),body,grant,receiving,deadline)
    }
    /// 📤️ Transfers only a completely settled result from the same unchanged Store authority.
    pub fn take_derived_operation(&self,owner:&mut ArtifactDerivedOperation<P,M>,grant:RetainedCloneGrant)->Result<Option<((Option<ArtifactDerivedSnapshot<P>>,Vec<protocol::MutationMessage>),RetainedCloneProgress)>,ValueError> {
        if !owner.is_finished(){return Ok(None);}if !owner.registry.as_ref().is_some_and(|registry|registry.ptr_eq(&self.owners.snapshot_read_leases)){return Err(refusal("derived operation handoff requires its original Store issuer"));}if owner.context.generation!=self.generation()||owner.context.base_revision!=self.content_revision(){return Err(refusal("derived operation result moved from its original Store frontier"));}
        let demand=header::<ArtifactReplayPrepared<P,M>>();if !funded(grant,demand){return Ok(None);}let mut prepared=owner.prepared.take().unwrap();owner.finished=false;owner.closing=true;let result=(prepared.next.take().map(|next|self.seal_derived_snapshot(next)),std::mem::take(&mut prepared.messages));let p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};owner.record(p);Ok(Some((result,p)))
    }
}

