//! 📬️ Paged one-item preparation: any plugin whose documents may exceed the one-item retained cap edits them page by page.
//!
//! Original immutable snapshot roots project through their required typed issuer. Unchanged pages and payloads retain
//! their original authorities while the shared diff applier prepares changed paths. This module owns genuine sources,
//! independently funded projection/transfer turns, the sealer and the complete original-issuer close ladder.

use semio_os_store_one_item_publication::batch_digest::{admit_artifact_batch_digest, ArtifactStoreBatchDigest};
use semio_framework_value_snapshot_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, SnapshotRead};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
pub use semio_os_store_one_item::{PagedOneItemEdit,PagedOneItemEditStep};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{OriginalPostRead,PagedOneItemPostMode};
use semio_os_store_one_item::{ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES};
use semio_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared};
use semio_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemSealer;
use semio_os_spr::{Edit, Mutation};
use semio_os_store_command::HistoryLane;
use semio_os_spr::ActorId;
use semio_framework_value::retained_clone::{ RetainedCloneBirthDemand, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneSourceAdmission, RetainedCloneStep, RetainedCloneRef, RetainedCloneSourceTake};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection, SnapshotProjectionCursor, ProjectionOutput};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use std::{marker::PhantomData, mem::{size_of, ManuallyDrop}, sync::Arc};


#[path = "🧬️mutation-apply/🦀️.rs"]
mod mutation_apply;
pub use mutation_apply::{mutation_apply_preparation_factory, MutationApplyEdit};
#[path = "❄️cold-apply/🦀️.rs"]
mod cold_apply;
pub use cold_apply::{bounded_cold_mutation_apply_preparation_factory, cold_mutation_apply_preparation_factory, ColdMutationApplyFactory, ColdPreparationBounds};
#[cfg(feature="test-support")]
pub use cold_apply::cold_projection_metadata_case;


fn original_mutation_factory<'a, M: Send + Sync + 'static>(
    direct: &'a Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    captured: &'a Option<FactoryOwnedSourceAuthority<M>>,
    source: &'a Option<RetainedCloneSource<M, FactoryOwnedSourceAuthority<M>>>,
) -> Result<&'a dyn ArtifactOwnedValueRetirementFactory<M>, ValueError> {
    if let Some(factory) = direct.as_ref() { return Ok(factory.as_ref()); }
    if let Some(authority) = captured.as_ref() { return Ok(authority.factory().as_ref()); }
    source.as_ref().ok_or_else(|| fault("structural", "mutation-source-issuer"))?.try_authority().map(|authority| authority.factory().as_ref())
}
/// 🔐️ The returned-custody ladder of a failed seal: the original edit, post and exact issuer pair stay held until a funded close retires them.
mod returned_custody {
    use super::*;

    pub fn retain_seal_returns<S,M>(
        authority:&mut Option<Arc<ArtifactStoreOneItemLiveAuthority>>,edit:&mut Option<Edit<M>>,post:&mut Option<Arc<S>>,
        mutations:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,snapshots:&mut Option<Arc<dyn SnapshotRetirementFactory<S>>>,
        returned:(Arc<ArtifactStoreOneItemLiveAuthority>,Edit<M>,Arc<S>,Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,Arc<dyn SnapshotRetirementFactory<S>>),
    ){
        let(original_authority,original_edit,original_post,original_mutations,original_snapshots)=returned;
        *authority=Some(original_authority);*edit=Some(original_edit);*post=Some(original_post);*mutations=Some(original_mutations);*snapshots=Some(original_snapshots);
    }

    pub fn returned_edit_demands<M:Send+'static>(original:&Option<Edit<M>>,close:&Option<semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=close.as_ref(){
            return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};
        }
        Ok(if original.is_some(){RetirementDemand{copy_bytes:size_of::<semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>(),depth:1,..Default::default()}}else{Default::default()})
    }

    pub fn close_returned_edit<M:Send+'static>(original:&mut Option<Edit<M>>,close:&mut Option<semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,semio_framework_value::OriginalRetirementFailure>{
        if original.is_none()&&close.is_none(){return Ok(None);}
        let demand=returned_edit_demands(original,close,grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(owner)=close.as_mut(){
            if owner.terminal_is_empty(){*close=None;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
            let step=owner.close_step(grant).result?;
            let receipt=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"returned structural edit exact issuer")?.progress();
            return Ok(Some(RetainedCloneStep::Progress(receipt)));
        }
        if factory.is_none(){return Err((fault("structural","returned-edit-issuer")).into());}
        let edit=original.take().ok_or_else(||fault("structural","returned-edit"))?;
        let issuer=factory.take().ok_or_else(||fault("structural","returned-edit-issuer"))?;
        *close=Some(semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement::new(edit,issuer));
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }

    pub fn returned_post_demands<S>(original:&Option<Arc<S>>,factory:&Option<Arc<dyn SnapshotRetirementFactory<S>>>)->Result<RetirementDemand,ValueError>{
        let Some(post)=original.as_ref()else{return Ok(Default::default());};
        let issuer=factory.as_ref().ok_or_else(||fault("structural","returned-post-issuer"))?;
        Ok(RetirementDemand{copy_bytes:2*size_of::<Arc<S>>(),capacity_bytes:issuer.retirement_birth_bytes(post),depth:1,..Default::default()})
    }

    pub fn close_returned_post<S>(original:&mut Option<Arc<S>>,factory:&Option<Arc<dyn SnapshotRetirementFactory<S>>>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,semio_framework_value::OriginalRetirementFailure>{
        if original.is_none(){return Ok(None);}
        let demand=returned_post_demands(original,factory)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let issuer=factory.as_ref().ok_or_else(||fault("structural","returned-post-issuer"))?;
        let post=original.take().ok_or_else(||fault("structural","returned-post"))?;
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-demand.copy_bytes,..grant};
        match issuer.retire(post,child).result {
            Ok((owner,mut receipt))=>{
                *active=Some(owner);
                receipt.copied_bytes=receipt.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(||fault("structural","returned-post-copy"))?;
                if receipt.retained_capacity_bytes!=demand.capacity_bytes{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"returned snapshot issuer changed its admitted frame extent").with_retained_progress(receipt)).into());}
                semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"returned structural post exact issuer")?;
                Ok(Some(RetainedCloneStep::Progress(receipt)))
            }
            Err((error,post))=>{
                *original=Some(post);let mut receipt=error.retained_progress();receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(||fault("structural","returned-post-refusal-copy"))?;
                Err(error.with_retained_progress(receipt))
            }
        }
    }
}
use returned_custody::{close_returned_edit, close_returned_post, retain_seal_returns, returned_edit_demands, returned_post_demands};

/// 🧪️ The seams this crate's own `[[test]]` targets drive from outside: the failed-seal custody ladder and the mutation-apply stage. Compiled only with `test-support`, never part of the production API.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub mod test_support {
    pub use super::mutation_apply::{Stage,inverse_rows};
    pub use super::returned_custody::{close_returned_edit, close_returned_post, retain_seal_returns, returned_edit_demands, returned_post_demands};
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PagedOneItemPreparationFactory<S, M, E>(PhantomData<fn() -> (S, M, E)>);

impl<S, M, E> Default for PagedOneItemPreparationFactory<S, M, E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// 🌱️ Prices the concrete structural factory; it owns no original child constructor.
pub fn paged_one_item_factory_birth_bytes<S, M, E>() -> usize
where
    PagedOneItemPreparationFactory<S, M, E>: semio_framework_value::FactoryPayloadRetirement,
{
    semio_framework_value::factory_constructor_birth_bytes::<PagedOneItemPreparationFactory<S, M, E>>(0)
}

fn fault(prefix: &'static str, owner: &'static str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, format!("{prefix}-{owner}-owner"))
}

fn capture_work<T,E>(work:&mut u64,turn:semio_framework_value::RetainedWorkOutcome<T,E>)->Result<T,E>{
    *work=work.saturating_add(turn.work);
    turn.result
}

impl<S, M, E> ArtifactStoreOneItemPreparationFactory<S, M> for PagedOneItemPreparationFactory<S, M, E>
where
    S: SnapshotProjection<Output = S>,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Mutation<S> + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if lane != HistoryLane::Document || !E::recognizes(mutation) {
            return Err(format!("{}-admission", E::PREFIX));
        }
        let retained_bytes = E::preflight(mutation)?;
        if retained_bytes > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(format!("{}-payload", E::PREFIX));
        }
        Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, M>(mutation, retained_bytes.max(1)))
    }

    fn begin_demand(&self, mutation: &M, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        if lane != HistoryLane::Document || !E::recognizes(mutation) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "structural mutation has no original preparation issuer"));
        }
        if !S::controlled_retirement_supported() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "structural clone originals require controlled typed retirement"));
        }
        Ok(RetainedCloneBirthDemand { capacity_bytes: size_of::<PagedOneItemPreparation<S, M, E>>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<S, M, M>>().checked_add(std::mem::size_of::<PagedOneItemPreparation<S, M, E>>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


    fn begin(
        &self,
        request: ArtifactStoreOneItemPreparationRequest<S, M, M>,
        grant: ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<S, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let receipt = match demand.admit(grant.retained_grant()) {
            Ok(receipt) => receipt,
            Err(error) => return Err((error, request)),
        };
        let copied_bytes=self.begin_copy_byte_demand(&request.mutation,request.lane);
        let copied_bytes=match copied_bytes{Ok(bytes)=>bytes,Err(error)=>return Err((error,request))};
        if grant.retained_grant().maximum_copy_bytes<copied_bytes{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"immutable preparation original header capture is not funded"),request));}
        let receipt=RetainedCloneProgress{copied_bytes,..receipt};
        let admitted = request.operation == request.authority.operation() && request.generation == request.authority.generation() && request.base_revision == request.authority.base_revision() && request.authority.actor().len() <= ARTIFACT_STORE_ONE_ITEM_ID_BYTES;
        if !admitted {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "structural request differs from its original authority"), request));
        }
        let ArtifactStoreOneItemPreparationRequest { authority, base, mutation, mutation_retirement, snapshot_retirement, .. } = request;
        Ok((
            Box::new(PagedOneItemPreparation::<S, M, E> {
                base: ManuallyDrop::new(Some(base)),
                source_admission: None,
                mutation_admission: None,
                source: ManuallyDrop::new(None),
                cursor: ManuallyDrop::new(None),
                post: ManuallyDrop::new(None),
                projected: ManuallyDrop::new(None),
                post_read: OriginalPostRead::default(),
                edit: E::default(),
                edit_close_effect: Default::default(),
                pending_inverse: ManuallyDrop::new(None),
                inverse_retirement: ManuallyDrop::new(None),
                outcome_messages: None,
                outcome_publication: None,
                mutation: ManuallyDrop::new(Some(mutation)),
                mutation_source: ManuallyDrop::new(None),
                mutation_authority: ManuallyDrop::new(None),
                returned_edit: ManuallyDrop::new(None),
                returned_edit_retirement: ManuallyDrop::new(None),
                returned_post: ManuallyDrop::new(None),
                authority: ManuallyDrop::new(Some(authority)),
                mutation_retirement: ManuallyDrop::new(Some(mutation_retirement)),
                snapshot_retirement: ManuallyDrop::new(Some(snapshot_retirement)),
                factory_close: ManuallyDrop::new(Default::default()),
                sealer: ManuallyDrop::new(None),
                external_retirement: ManuallyDrop::new(None),
                retirement: RetainedCloneClose::default(),
                checkpoint: Default::default(),
                seal_base_checkpoint: None,
                phase: Phase::MutationAuthority,
                cancelled: false,
                closing: false,
                last_work: 0,
                coordinated: false,
            }),
            receipt,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    MutationAuthority,
    MutationSource,
    Source,
    Cursor,
    Project,
    TakeProjection,
    PlaceProjection,
    CloseProjection,
    Edit,
    CloseEdit,
    TakeMutationSource,
    TakeMutationPair,
    Messages,
    Seal,
    Sealing,
}

struct PagedOneItemPreparation<S: SnapshotProjection<Output = S>, M: Send + Sync + 'static, E: PagedOneItemEdit<S, M>> {
    pub base: ManuallyDrop<Option<SnapshotRead<S>>>,
    source_admission: Option<RetainedCloneSourceAdmission<S, SnapshotRead<S>>>,
    mutation_admission: Option<RetainedCloneSourceAdmission<M, FactoryOwnedSourceAuthority<M>>>,
    pub source: ManuallyDrop<Option<RetainedCloneSource<S, SnapshotRead<S>>>>,
    pub cursor: ManuallyDrop<Option<S::Projection>>,
    pub post: ManuallyDrop<Option<S>>,
    projected: ManuallyDrop<Option<S::Output>>,
    post_read: OriginalPostRead<S>,
    pub edit: E,
    edit_close_effect: semio_framework_value::native_decoding::NativeReceivingEffect<semio_framework_value::OriginalRetirementFailure>,
    pending_inverse: ManuallyDrop<Option<Vec<M>>>,
    inverse_retirement: ManuallyDrop<Option<semio_framework_value_factory::vector::FactoryVectorRetirement<M>>>,
    outcome_messages: Option<protocol::MutationMessageLedgerRetirement>,
    outcome_publication: Option<protocol::MutationMessagePublication>,
    pub mutation: ManuallyDrop<Option<M>>,
    mutation_source: ManuallyDrop<Option<RetainedCloneSource<M, FactoryOwnedSourceAuthority<M>>>>,
    mutation_authority: ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,
    returned_edit: ManuallyDrop<Option<Edit<M>>>,
    returned_edit_retirement: ManuallyDrop<Option<semio_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>>,
    returned_post: ManuallyDrop<Option<Arc<S>>>,
    pub authority: ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,
    pub mutation_retirement: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    pub snapshot_retirement: ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<S>>>>,
    pub factory_close: ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    pub sealer: ManuallyDrop<Option<ArtifactStoreOneItemSealer<S, M>>>,
    external_retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub retirement: RetainedCloneClose,
    pub checkpoint: ArtifactStoreOneItemCheckpoint,
    seal_base_checkpoint: Option<ArtifactStoreOneItemCheckpoint>,
    pub phase: Phase,
    pub cancelled: bool,
    pub closing: bool,
    last_work: u64,
    coordinated: bool,
}

impl<S:SnapshotProjection<Output=S>,M:semio_framework_value::retirement::RetireOwned+Send+Sync+'static,E:PagedOneItemEdit<S,M>> PagedOneItemPreparation<S,M,E>{
    fn placement_demand(&self)->Result<RetirementDemand,ValueError>{
        let mut demand=S::Output::placement_demand();
        demand.copy_bytes=demand.copy_bytes.checked_add(2*size_of::<Option<S::Output>>()+2*size_of::<S>()).ok_or_else(||fault(E::PREFIX,"projection-placement-quote"))?;
        demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(E::PREFIX,"projection-placement-depth"))?;
        Ok(demand)
    }
    fn place_projected(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress>{
        let mut work=0;
        let result=(||{
        let demand=self.placement_demand()?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default());}
        if self.post.is_some(){return Err(fault(E::PREFIX,"projection-placement-target"));}
        let movement=2*size_of::<Option<S::Output>>()+2*size_of::<S>();
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-movement,maximum_depth:grant.maximum_depth-1,..grant};
        let original=self.projected.take();let coordinated=original.is_some();
        let output=original.ok_or_else(||fault(E::PREFIX,"projection-placement-source"))?;
        let mut target=std::mem::MaybeUninit::<S>::uninit();
        let turn=output.place(&mut target,child);work=turn.work.max(u64::from(coordinated));
        let mut progress=match turn.result {Ok(progress)=>progress,Err((error,output))=>{*self.projected=Some(output);return Err(error);}};
        *self.post=Some(unsafe{target.assume_init()});
        progress.copied_items=progress.copied_items.max(1);progress.copied_bytes+=movement;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"structural original projection placement")
        })();
        semio_framework_value::RetainedWorkOutcome{result,work}
    }

}

impl<S, M, E> PagedOneItemPreparation<S, M, E>
where
    S: SnapshotProjection<Output = S>,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    pub fn record(&mut self, progress: RetainedCloneProgress) -> ArtifactStoreOneItemPreparationStep {
        if progress == RetainedCloneProgress::default() {
            return ArtifactStoreOneItemPreparationStep::Blocked;
        }
        self.last_work=self.last_work.checked_add(1).expect("checkpoint work quote prevents overflow");
        let bytes = progress.copied_bytes.saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes);
        self.coordinated=true;
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(u32::try_from(progress.copied_items).unwrap_or(u32::MAX));
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, progress)
    }

    fn advance_sealer(&mut self, grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let prefix = E::PREFIX;
        let sealer = self.sealer.as_mut().ok_or_else(|| fault(prefix, "sealer"))?;
        let turn=sealer.advance(grant,receiving);self.last_work=sealer.last_turn_work();let step=turn?;
        let (checkpoint, progress) = match step {
            ArtifactStoreOneItemPreparationStep::Progress(checkpoint, progress) | ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, progress) => (checkpoint, progress),
            ArtifactStoreOneItemPreparationStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
        };
        let base = self.seal_base_checkpoint.ok_or_else(|| fault(prefix, "seal-checkpoint"))?;
        self.coordinated=true;
        self.checkpoint = ArtifactStoreOneItemCheckpoint {
            cursor: base.cursor.saturating_add(checkpoint.cursor),
            completed_items: base.completed_items.saturating_add(checkpoint.completed_items),
            completed_bytes: base.completed_bytes.saturating_add(checkpoint.completed_bytes),
            digest: checkpoint.digest,
        };
        if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
            if let Some(messages) = self.outcome_publication.take() { sealer.record_prepared_outcome_messages(messages); }
            self.checkpoint.digest = sealer.prepared().ok_or_else(|| fault(prefix, "prepared"))?.edit_digest();
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress));
        }
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, progress))
    }

    fn finish_edit(&mut self, clone_grant: RetainedCloneGrant, progress: RetainedCloneProgress, inverse: Vec<M>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        *self.pending_inverse = Some(inverse);
        self.coordinated=true;
        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, progress, "structural edit")?;
        self.edit.begin_close();
        self.phase = Phase::CloseEdit;
        Ok(self.record(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    fn advance_body(&mut self,body:usize,maximum_work_units:u64,grant:ArtifactStoreOneItemGrant,receiving:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>)->Result<ArtifactStoreOneItemPreparationStep,semio_framework_value::OriginalReceivingPrimary>{
        let prefix = E::PREFIX;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
        match self.phase {
            Phase::MutationAuthority => {
                let copy=size_of::<FactoryOwnedSourceAuthority<M>>();
                if clone_grant.maximum_copy_bytes<copy || clone_grant.maximum_depth==0 {return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                if self.mutation.is_none() || self.mutation_retirement.is_none(){return Err((fault(prefix,"original-mutation-pair")).into());}
                let original=self.mutation.take().ok_or_else(||fault(prefix,"mutation"))?;
                self.coordinated=true;
                let factory=self.mutation_retirement.take().ok_or_else(||fault(prefix,"mutation-issuer"))?;
                *self.mutation_authority=Some(FactoryOwnedSourceAuthority::new(original,factory));
                self.phase=Phase::MutationSource;
                Ok(self.record(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
            }
            Phase::MutationSource => {
                let demand=RetainedCloneSource::<M>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<M>>();
                if demand.admit(clone_grant).is_err() || clone_grant.maximum_copy_bytes<RetainedCloneSource::<M>::borrowed_constructor_copy_bytes::<FactoryOwnedSourceAuthority<M>>(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                if self.mutation_admission.is_none(){
                    let authority=self.mutation_authority.take().ok_or_else(||fault(prefix,"mutation-authority"))?;
                    self.coordinated=true;
                    self.mutation_admission=Some(RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::admission_borrowed(authority,FactoryOwnedSourceAuthority::get));
                    return Ok(self.record(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FactoryOwnedSourceAuthority<M>>(),..Default::default()}));
                }
                let admission=self.mutation_admission.as_mut().ok_or_else(||fault(prefix,"mutation-admission"))?;
                let progress=if admission.phase()=="ready"{
                    let progress=capture_work(&mut self.last_work,admission.take_output(&mut self.mutation_source,clone_grant))?.progress();
                    if self.mutation_source.is_some(){self.coordinated=true;self.mutation_admission=None;self.phase=Phase::Source;}
                    progress
                }else{capture_work(&mut self.last_work,admission.advance(clone_grant))?.progress()};
                Ok(self.record(progress))
            }
            Phase::Source => {
                let demand = RetainedCloneSource::<S>::borrowed_constructor_demand::<SnapshotRead<S>>();
                if demand.admit(clone_grant).is_err() || clone_grant.maximum_copy_bytes < RetainedCloneSource::<S>::borrowed_constructor_copy_bytes::<SnapshotRead<S>>() {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                if self.source_admission.is_none() {
                    let base = self.base.take().ok_or_else(|| fault(prefix, "base"))?;
                    self.coordinated=true;
                    match base.source_admission() {
                        Ok(admission) => self.source_admission = Some(admission),
                        Err((error, base)) => { *self.base = Some(base); return Err((error).into()); }
                    }
                }
                let admission = self.source_admission.as_mut().ok_or_else(|| fault(prefix, "source-admission"))?;
                let progress = if admission.phase() == "ready" {
                    let progress = capture_work(&mut self.last_work,admission.take_output(&mut self.source, clone_grant))?.progress();
                    if self.source.is_some() { self.coordinated=true;self.source_admission = None; self.phase = Phase::Cursor; }
                    progress
                } else { capture_work(&mut self.last_work,admission.advance(clone_grant))?.progress() };
                Ok(self.record(progress))
            }
            Phase::Cursor => {
                let mut demand=S::projection_birth_demand();
                demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(prefix,"projection-birth-depth"))?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:clone_grant.maximum_depth-1,..clone_grant};
                let(cursor,progress)=capture_work(&mut self.last_work,S::begin_projection(child))?;
                *self.cursor=Some(cursor);
                let progress=semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"structural immutable projection birth")?;
                self.phase=Phase::Project;
                Ok(self.record(progress))
            }
            Phase::Project => {
                let cursor=self.cursor.as_mut().ok_or_else(||fault(prefix,"projection"))?;
                let source=self.source.as_ref().ok_or_else(||fault(prefix,"source"))?;
                let original=source.try_borrow()?;
                let mut demand=cursor.demands(original,body,maximum_work_units.checked_sub(2).unwrap_or(0))?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(prefix,"projection-depth"))?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:clone_grant.maximum_depth-1,..clone_grant};
                let step=capture_work(&mut self.last_work,cursor.advance(original,body,maximum_work_units.checked_sub(2).unwrap_or(0),child))?;
                let progress=semio_framework_value::retained_clone::admit_retained_clone_progress(child,step.progress(),"structural immutable snapshot projection")?;
                if matches!(step,RetainedCloneStep::Complete(_)){
                    if !cursor.ready(){return Err((fault(prefix,"projection-ready")).into());}
                    self.phase=Phase::TakeProjection;
                }
                Ok(self.record(progress))
            }
            Phase::TakeProjection => {
                let cursor=self.cursor.as_mut().ok_or_else(||fault(prefix,"projection"))?;
                let mut demand=cursor.take_demand()?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(prefix,"projection-transfer-depth"))?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:clone_grant.maximum_depth-1,..clone_grant};
                let Some((post,progress))=capture_work(&mut self.last_work,cursor.take(child))?else{return Err((fault(prefix,"projection-result")).into());};
                *self.projected=Some(post);
                self.coordinated=true;
                let progress=semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"structural immutable projection output")?;
                cursor.begin_close();
                self.phase=Phase::PlaceProjection;
                Ok(self.record(progress))
            }
            Phase::PlaceProjection => {
                let turn=self.place_projected(clone_grant);let progress=capture_work(&mut self.last_work,turn)?;
                if self.projected.is_none(){self.coordinated=true;self.phase=Phase::CloseProjection;}
                Ok(self.record(progress))
            }
            Phase::CloseProjection => {
                let cursor=self.cursor.as_mut().ok_or_else(||fault(prefix,"projection"))?;
                if cursor.terminal_is_empty(){
                    let copy=size_of::<S::Projection>();
                    if clone_grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                    *self.cursor=None;
                    self.coordinated=true;self.phase=Phase::Edit;
                    return Ok(self.record(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                }
                let mut demand=cursor.close_demands(clone_grant.maximum_copy_bytes)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(prefix,"projection-close-depth"))?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:clone_grant.maximum_depth-1,..clone_grant};
                let step=capture_work(&mut self.last_work,cursor.close_step(child))?;
                let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,cursor.terminal_is_empty(),"structural immutable projection close")?;
                Ok(self.record(step.progress()))
            }
            Phase::Edit => {
                let mode=self.edit.post_mode();
                if let Some(progress)=capture_work(&mut self.last_work,self.post_read.prepare(&mut self.post,mode,body,clone_grant))?{return Ok(self.record(progress));}
                let post=self.post_read.access(&mut self.post,mode)?;
                let before=self.source.as_ref().ok_or_else(||fault(prefix,"before-source"))?.try_borrow()?;
                let mutation=self.mutation_source.as_ref().ok_or_else(||fault(prefix,"mutation-source"))?.try_borrow()?;
                let demand=self.edit.advance_demands(before,mutation,post.read(),body,maximum_work_units)?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let result=self.edit.advance(before,mutation,post,body,clone_grant,receiving.ok_or_else(||fault(prefix,"native-edit-control"))?);
                self.last_work=self.last_work.saturating_add(self.edit.last_turn_work());
                match result? {
                    PagedOneItemEditStep::Progress(progress) => {
                        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, progress, "structural edit")?;
                        Ok(self.record(progress))
                    }
                    PagedOneItemEditStep::Complete(progress, inverse) => self.finish_edit(clone_grant, progress, vec![inverse]).map_err(Into::into),
                    PagedOneItemEditStep::CompleteRows(progress, inverse) => self.finish_edit(clone_grant, progress, inverse).map_err(Into::into),
                    PagedOneItemEditStep::CompleteOutcome(progress, inverse, messages) => {
                        self.outcome_messages = Some(protocol::MutationMessageLedgerRetirement::new(String::new(), messages));
                        self.finish_edit(clone_grant, progress, inverse)
                    }
                    PagedOneItemEditStep::RefusedOutcome(progress, messages) => {
                        self.outcome_messages = Some(protocol::MutationMessageLedgerRetirement::new(String::new(), messages));
                        self.coordinated=true;
                        self.cancelled = true;
                        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, progress, "structural semantic refusal")?;
                        Err(ValueError::literal(ValueRefusalKind::InvalidValue, "structural mutation produced a refused semantic outcome").with_retained_progress(progress).into())
                    }
                }
            }
            Phase::CloseEdit => {
                if self.edit.terminal_is_empty(){if !self.post_read.terminal_is_empty(){let progress=capture_work(&mut self.last_work,self.post_read.prepare(&mut self.post,PagedOneItemPostMode::Mutable,body,clone_grant))?.ok_or_else(||fault(prefix,"completed-post-return"))?;return Ok(self.record(progress));}self.coordinated=true;self.phase=Phase::TakeMutationSource;return Ok(self.record(RetainedCloneProgress{copied_items:1,..Default::default()}));}
                let factory=original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?;
                let mut demand=self.edit.close_demands(factory,clone_grant.maximum_copy_bytes)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||fault(prefix,"edit-close-depth"))?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:clone_grant.maximum_depth-1,..clone_grant};
                self.coordinated=true;
                let control=receiving.ok_or_else(||fault(prefix,"edit-close-receiving"))?;
                let selected=self.edit.close_work_demand(factory,child.maximum_copy_bytes)?;
                let demand=self.edit.close_demands(factory,child.maximum_copy_bytes)?;
                let turn=control.receive_retirement(demand,&mut self.edit_close_effect,demand.capacity_bytes!=0,selected,|issued,actual|{let mut no_allocation=None;let result=self.edit.close_step(factory,issued.restricted_by(child),actual.unwrap_or(&mut no_allocation));semio_framework_value::RetainedWorkOutcome{result,work:self.edit.last_turn_work()}});
                self.last_work=self.last_work.checked_add(turn.work).ok_or_else(||fault(prefix,"edit-close-work-overflow"))?;
                Ok(self.record(turn.result?))
            }
            Phase::TakeMutationSource => {
                let source=self.mutation_source.as_mut().ok_or_else(||fault(prefix,"mutation-source"))?;
                let demand=RetirementDemand{copy_bytes:source.next_take_copy_byte_demand()?,capacity_bytes:source.next_take_capacity_byte_demand(clone_grant.maximum_copy_bytes)?,release_bytes:source.next_take_release_byte_demand()?,depth:source.next_take_depth_demand()?};
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(clone_grant,demand){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                match capture_work(&mut self.last_work,source.take_authority(clone_grant))?{
                    RetainedCloneSourceTake::Pending(progress)=>Ok(self.record(progress)),
                    RetainedCloneSourceTake::Ready(authority,progress)=>{self.coordinated=true;*self.mutation_authority=Some(authority);*self.mutation_source=None;self.phase=Phase::TakeMutationPair;Ok(self.record(progress))}
                }
            }
            Phase::TakeMutationPair => {
                let copy=FactoryOwnedSourceAuthority::<M>::transfer_copy_byte_demand();
                if clone_grant.maximum_copy_bytes<copy || clone_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let authority=self.mutation_authority.take().ok_or_else(||fault(prefix,"mutation-authority"))?;
                self.coordinated=true;
                match authority.take_original(clone_grant){
                    Ok((mutation,factory,progress))=>{*self.mutation=Some(mutation);*self.mutation_retirement=Some(factory);self.phase=Phase::Messages;Ok(self.record(progress))}
                    Err((error,authority))=>{*self.mutation_authority=Some(authority);Err(error.into())}
                }
            }
            Phase::Messages => self.advance_messages(clone_grant).map_err(Into::into),
            Phase::Seal => self.begin_seal(grant).map_err(Into::into),
            Phase::Sealing => self.advance_sealer(grant,receiving.ok_or_else(||fault(prefix,"native-sealer-control"))?).map_err(Into::into),
        }
    }

    fn phase_work_demand(&self,body:usize,maximum_work_units:u64)->Result<Option<u64>,ValueError>{
        let work=match self.phase{
            Phase::MutationAuthority|Phase::TakeMutationPair=>Some(1),
            Phase::MutationSource=>Some(match self.mutation_admission.as_ref(){None=>1,Some(owner)if owner.phase()=="ready"=>1,Some(owner)=>owner.advance_work_demand()?}),
            Phase::Source=>Some(match self.source_admission.as_ref(){None=>1,Some(owner)if owner.phase()=="ready"=>1,Some(owner)=>owner.advance_work_demand()?}),
            Phase::Cursor=>Some(S::projection_birth_work_demand()?),
            Phase::Project=>{let owner=self.cursor.as_ref().ok_or_else(||fault(E::PREFIX,"projection"))?;let source=self.source.as_ref().ok_or_else(||fault(E::PREFIX,"source"))?.try_borrow()?;Some(owner.advance_work_demand(source,body,maximum_work_units.checked_sub(2).unwrap_or(0))?)},
            Phase::TakeProjection=>Some(self.cursor.as_ref().ok_or_else(||fault(E::PREFIX,"projection"))?.take_work_demand()?),
            Phase::PlaceProjection=>Some(1),
            Phase::CloseProjection=>{let owner=self.cursor.as_ref().ok_or_else(||fault(E::PREFIX,"projection"))?;Some(if owner.terminal_is_empty(){1}else{owner.close_work_demand(body)?})},
            Phase::Edit=>{let mode=self.edit.post_mode();if self.post_read.demand(&self.post,mode,body)?.is_some(){Some(self.post_read.work_demand(&self.post,mode,body)?)}else{None}},
            Phase::CloseEdit=>{if self.edit.terminal_is_empty(){if self.post_read.terminal_is_empty(){Some(1)}else{Some(self.post_read.work_demand(&self.post,PagedOneItemPostMode::Mutable,body)?)}}else{Some(self.edit.close_work_demand(original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?,body)?)}},
            Phase::TakeMutationSource=>Some(self.mutation_source.as_ref().ok_or_else(||fault(E::PREFIX,"mutation-source"))?.next_take_work_demand(body)?),
            Phase::Messages=>Some(match self.outcome_publication.as_ref(){Some(owner)if !owner.ready()=>owner.advance_work_demand()?,_=>1}),
            Phase::Seal=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original seal admission has no selected work quote")),
            Phase::Sealing=>None,
        };
        work.map(|work|work.checked_add(2).ok_or_else(||fault(E::PREFIX,"parent-work-overflow"))).transpose()
    }

    fn advance_messages(&mut self, grant: RetainedCloneGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        if let Some(publication) = self.outcome_publication.as_mut() {
            if publication.ready() {
                let copy = size_of::<Option<protocol::MutationMessagePublication>>();
                if grant.maximum_copy_bytes < copy { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
                self.coordinated=true;self.phase = Phase::Seal;
                return Ok(self.record(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }));
            }
            let mut demand = publication.demands()?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| fault(E::PREFIX, "message-depth"))?;
            if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            self.coordinated=true;
            let turn=publication.advance(child);self.last_work=self.last_work.checked_add(turn.work).ok_or_else(||fault(E::PREFIX,"message-work-overflow"))?;let step=turn.result?;
            let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "structural diagnostic report preparation")?;
            return Ok(self.record(progress));
        }
        let Some(messages) = self.outcome_messages.as_mut() else {
            self.coordinated=true;self.phase = Phase::Seal;
            return Ok(self.record(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        let frame = size_of::<protocol::MutationMessagePublication>();
        let copy = frame.checked_add(messages.transfer_copy_byte_demand()).ok_or_else(|| fault(E::PREFIX, "message-layout"))?;
        if grant.maximum_copy_bytes < copy { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - frame, ..grant };
        let Some((original, progress)) = messages.take_messages_controlled(child)? else { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); };
        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "structural original diagnostic transfer")?;
        self.coordinated=true;self.outcome_publication = Some(protocol::MutationMessagePublication::new(original));
        Ok(self.record(RetainedCloneProgress { copied_items: 1, copied_bytes: progress.copied_bytes.checked_add(frame).ok_or_else(|| fault(E::PREFIX, "message-receipt"))?, ..progress }))
    }

    fn retain_failed_seal(&mut self, authority: Arc<ArtifactStoreOneItemLiveAuthority>, edit: Edit<M>, post: Arc<S>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshots: Arc<dyn SnapshotRetirementFactory<S>>) {
        retain_seal_returns(&mut self.authority,&mut self.returned_edit,&mut self.returned_post,&mut self.mutation_retirement,&mut self.snapshot_retirement,(authority,edit,post,mutations,snapshots));
        self.cancelled=true;
    }
    fn begin_seal(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let prefix = E::PREFIX;
        let retained = grant.retained_grant();
        let constructor = ArtifactStoreOneItemSealer::<S, M>::constructor_demand();
        let inverse_bytes = self.pending_inverse.as_ref().map_or(0, |rows| rows.capacity().saturating_mul(size_of::<M>()));
        let wrapped = size_of::<M>().checked_add(inverse_bytes).and_then(|bytes| bytes.checked_add(size_of::<S>())).and_then(|bytes| bytes.checked_add(32)).and_then(|bytes| bytes.checked_add(constructor.capacity_bytes)).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "structural seal layout overflow"))?;
        if retained.maximum_items == 0 || retained.maximum_depth < constructor.depth || retained.maximum_capacity_bytes < wrapped || retained.maximum_copy_bytes < ArtifactStoreOneItemSealer::<S, M>::constructor_copy_byte_demand() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let post = self.post.take().ok_or_else(|| fault(prefix, "post"))?;
        self.coordinated=true;
        let inverse = self.pending_inverse.take().ok_or_else(|| fault(prefix, "inverse"))?;
        let mutation = self.mutation.take().ok_or_else(|| fault(prefix, "mutation"))?;
        let authority = self.authority.take().ok_or_else(|| fault(prefix, "authority"))?;
        let mut edit = authority.next_edit(mutation, inverse);
        edit.actor = Some(authority.actor().into());
        edit.mutation_meta[0].author_id = Some(ActorId(authority.actor().into()));
        let mutation_retirement = self.mutation_retirement.take().ok_or_else(|| fault(prefix, "mutation-issuer"))?;
        let snapshot_retirement = self.snapshot_retirement.take().ok_or_else(|| fault(prefix, "snapshot-issuer"))?;
        match authority.begin_one_item_seal(edit, Arc::new(post), mutation_retirement, snapshot_retirement, retained) {
            Ok((sealer, progress)) => {
                *self.sealer = Some(sealer);
                self.seal_base_checkpoint = Some(self.checkpoint);
                self.phase = Phase::Sealing;
                Ok(self.record(RetainedCloneProgress { retained_capacity_bytes: progress.retained_capacity_bytes.saturating_add(size_of::<M>()).saturating_add(inverse_bytes).saturating_add(2 * size_of::<usize>() + size_of::<S>()), ..progress }))
            }
            Err((error, authority, edit, post, mutation_retirement, snapshot_retirement)) => {
                self.retain_failed_seal(authority, edit, post, mutation_retirement, snapshot_retirement);
                Err(error)
            }
        }
    }
}

impl<S, M, E> ArtifactStoreOneItemPreparation<S, M> for PagedOneItemPreparation<S, M, E>
where
    S: SnapshotProjection<Output = S>,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, semio_framework_value::OriginalReceivingPrimary> {
        self.last_work=0;self.coordinated=false;
        if !grant.permits_one()||self.cancelled||self.closing{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        let maximum_work_units=receiving.available_work_units()?;if maximum_work_units==0{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)}let quote=self.phase_work_demand(body,maximum_work_units)?;
        if let Some(work)=quote{
            if !receiving.has_retirement_failure_storage(){
                let turn=receiving.prepare_failure_storage(grant.retained_grant());self.last_work=turn.work;
                return turn.result.map(|step|ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,step.progress()));
            }
            let turn=receiving.run_prepared_work(work,|_|{
                let result=self.advance_body(body,maximum_work_units,grant,None);
                let actual=self.last_work.saturating_add(u64::from(self.coordinated));
                semio_framework_value::RetainedWorkOutcome{result,work:actual}
            },|step|step.ownership_progress());
            return match turn{Some(turn)=>{self.last_work=turn.work;turn.result},None=>Ok(ArtifactStoreOneItemPreparationStep::Blocked)};
        }
        let before=receiving.actual_work();
        let result=self.advance_body(body,maximum_work_units,grant,Some(receiving));
        let observed=receiving.actual_work().saturating_sub(before);
        let reported=self.last_work.saturating_add(u64::from(self.coordinated));
        self.last_work=semio_os_store_one_item::original_one_item_turn_work(reported,observed,false)?;
        result
    }

    fn last_turn_work(&self)->u64{self.last_work}

    fn outcome_messages(&self) -> &[semio_os_spr::MutationMessage] {
        if let Some(publication) = self.outcome_publication.as_ref() { return publication.original_messages(); }
        if let Some(prepared) = self.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared) { return prepared.outcome_messages(); }
        self.outcome_messages.as_ref().map_or(&[], protocol::MutationMessageLedgerRetirement::retained_messages)
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn has_pending_receiving_recovery(&self)->bool{self.edit_close_effect.has_pending_recovery()||self.edit.has_pending_receiving_recovery()}
    fn pending_receiving_demand(&self)->Option<RetirementDemand>{self.edit_close_effect.pending_demand().or_else(||self.edit.pending_receiving_demand())}
    fn pending_receiving_recovery(&mut self)->Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>>{if self.edit_close_effect.has_pending_recovery(){self.edit_close_effect.pending_recovery().map(semio_framework_value::native_decoding::construction::NativeRecoveryLoan::Retirement)}else{self.edit.pending_receiving_recovery()}}
    fn finish_receiving_recovery(&mut self,control:&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{if self.edit_close_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut self.edit_close_effect)}else{self.edit.finish_receiving_recovery(control)}}
    fn begin_close(&mut self) {
        self.closing = true;
        self.edit.begin_close();
        if let Some(messages) = self.outcome_publication.as_mut() { messages.begin_close(); }
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        if self.terminal() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !self.closing || !grant.permits_one() {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let funded = grant.retained_grant();
        let demand = self.close_demands(funded.maximum_copy_bytes)?;
        if funded.maximum_depth < demand.depth {
            return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "structural preparation exceeds original nested depth")).into());
        }
        if funded.maximum_copy_bytes < demand.copy_bytes || funded.maximum_capacity_bytes < demand.capacity_bytes || funded.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: funded.maximum_depth.saturating_sub(1), ..funded };
        let unit = |copied_bytes: usize| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() });
        if let Some(messages) = self.outcome_publication.as_mut() {
            if messages.terminal_is_empty(){self.outcome_publication=None;return Ok(unit(demand.copy_bytes));}
            let step = messages.close_step(child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(messages) = self.outcome_messages.as_mut() {
            if messages.terminal_is_empty(){self.outcome_messages=None;return Ok(unit(demand.copy_bytes));}
            let step = messages.close_step(child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.external_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.external_retirement, child).result.map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.edit.terminal_is_empty() {
            let factory=original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?;
            return self.edit.close_step(factory,child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(Into::into);
        }
        if self.projected.is_some(){return self.place_projected(child).result.map(RetainedCloneStep::Progress).map_err(Into::into);}
        if !self.post_read.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.post_read.prepare(&mut self.post,PagedOneItemPostMode::Mutable,child.maximum_copy_bytes,child).result?.ok_or_else(||fault(E::PREFIX,"post-source-return"))?));}
        if let Some(sealer) = self.sealer.as_mut() {
            if !sealer.terminal_is_empty() {
                return sealer.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(Into::into);
            }
            self.sealer.take();
            return Ok(unit(size_of::<ArtifactStoreOneItemSealer<S,M>>()));
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.terminal_is_empty() {
                return cursor.close_step(child).result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *self.cursor = None;
            return Ok(unit(size_of::<S::Projection>()));
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(child).result.map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original)=self.post.take(){
            *self.returned_post=Some(Arc::new(original));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<S>()+size_of::<Arc<S>>(),retained_capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<S>(),..Default::default()}));
        }
        if let Some(inverse) = self.inverse_retirement.as_mut() {
            if inverse.terminal_is_empty(){*self.inverse_retirement=None;return Ok(unit(demand.copy_bytes));}
            let factory=original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?;
            let step = inverse.close_step(factory, child).result?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original) = self.pending_inverse.take() {
            return match semio_framework_value_factory::vector::FactoryVectorRetirement::admit(original, child) {
                Ok((inverse, progress)) => { *self.inverse_retirement = Some(inverse); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, original)) => { *self.pending_inverse = Some(original); Err(error.into()) }
            };
        }
        if let Some(step)=close_returned_edit(&mut self.returned_edit,&mut self.returned_edit_retirement,&mut self.mutation_retirement,child)?{return Ok(step);}
        if let Some(step)=close_returned_post(&mut self.returned_post,&self.snapshot_retirement,&mut self.external_retirement,child)?{return Ok(step);}
        if let Some(admission) = self.source_admission.as_mut() {
            if admission.phase() == "ready" {
                let step = admission.take_output(&mut self.source, child).result?;
                if admission.terminal_is_empty() { self.source_admission = None; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            let mut inputs = None;
            let step = admission.return_inputs(&mut inputs, child).result?;
            if let Some((base, ())) = inputs { *self.base = Some(base); }
            if admission.terminal_is_empty() { self.source_admission = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(admission)=self.mutation_admission.as_mut(){
            if admission.phase()=="ready"{
                let step=admission.take_output(&mut self.mutation_source,child).result?;
                if admission.terminal_is_empty(){self.mutation_admission=None;}
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            let mut inputs=None;
            let step=admission.return_inputs(&mut inputs,child).result?;
            if let Some((authority,()))=inputs{*self.mutation_authority=Some(authority);}
            if admission.terminal_is_empty(){self.mutation_admission=None;}
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source)=self.mutation_source.as_mut(){
            if !source.terminal_is_empty(){return source.close_step(child).result.map(|step|RetainedCloneStep::Progress(step.progress()));}
            *self.mutation_source=None;return Ok(unit(size_of::<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>()));
        }
        if let Some(step)=self.retirement.begin_granted(&mut self.mutation_authority,child).result?{return Ok(step);}
        if let Some(source) = self.source.as_mut() {
            if !source.terminal_is_empty() {
                return source.close_step(child).result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *self.source = None;
            return Ok(unit(size_of::<RetainedCloneSource<S,SnapshotRead<S>>>()));
        }
        if let Some(original) = self.mutation.take() {
            let moved=2*size_of::<M>();let provider=RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-moved,..child};
            return match self.mutation_retirement.as_ref().ok_or_else(|| fault(E::PREFIX, "mutation-issuer"))?.retire_owned(original, provider) .result {
                Ok((owner,mut receipt)) => {
                    *self.external_retirement=Some(owner);
                    let valid=receipt.fits(provider)&&receipt.retained_capacity_bytes==demand.capacity_bytes;
                    receipt.copied_bytes=receipt.copied_bytes.checked_add(moved).ok_or_else(||fault(E::PREFIX,"mutation-transfer-copy"))?;receipt.copied_items=receipt.copied_items.max(1);
                    if !valid{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original mutation issuer changed admitted frame extent").with_retained_progress(receipt)).into());}
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error,original)) => {
                    *self.mutation=Some(original);
                    let mut receipt=error.retained_progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(moved).ok_or_else(||fault(E::PREFIX,"mutation-return-copy"))?;receipt.copied_items=receipt.copied_items.max(1);
                    Err(error.with_retained_progress(receipt))
                }
            };
        }
        if self.base.is_some() {
            return artifact_retirement_admit_owned(&mut self.base, &mut self.external_retirement, child).result.map(|step| RetainedCloneStep::Progress(step.progress())).map_err(Into::into);
        }
        if let Some(original) = self.authority.take() {
            return match original.retire(child) {
                Ok((owner, progress)) => {
                    *self.external_retirement = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original)) => {
                    *self.authority = Some(original);
                    Err(error)
                }
            };
        }
        if let Some(original) = self.mutation_retirement.take() {
            self.factory_close[0] = Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(unit(demand.copy_bytes));
        }
        if let Some(original) = self.snapshot_retirement.take() {
            self.factory_close[1] = Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(unit(demand.copy_bytes));
        }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().ok_or_else(|| fault(E::PREFIX, "factory"))?;
            if !owner.terminal_is_empty() {
                return owner.step(child).result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *slot = None;
            return Ok(unit(0));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal()
    }
}

impl<S, M, E> PagedOneItemPreparation<S, M, E>
where
    S: SnapshotProjection<Output = S>,
    M: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    pub fn terminal(&self) -> bool {
        self.closing
            && self.edit.terminal_is_empty()
            && self.edit_close_effect.phase()==semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty
            && self.base.is_none()
            && self.source_admission.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.post.is_none()
            && self.projected.is_none()
            && self.post_read.terminal_is_empty()
            && self.pending_inverse.is_none()
            && self.inverse_retirement.is_none()
            && self.outcome_messages.is_none()
            && self.outcome_publication.is_none()
            && self.mutation.is_none()
            && self.mutation_source.is_none()
            && self.mutation_authority.is_none()
            && self.mutation_admission.is_none()
            && self.returned_edit.is_none()
            && self.returned_edit_retirement.is_none()
            && self.returned_post.is_none()
            && self.authority.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.sealer.is_none()
            && self.external_retirement.is_none()
            && self.retirement.is_empty()
            && self.factory_close.iter().all(Option::is_none)
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let nested = |mut value: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            value.depth = value.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "structural preparation depth overflow"))?;
            Ok(value)
        };
        let child = |demand: RetirementDemand| nested(demand);
        if let Some(messages) = self.outcome_publication.as_ref() { return if messages.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<protocol::MutationMessagePublication>(),depth:1,..Default::default()})}else{child(messages.close_demands()?)}; }
        if let Some(messages) = self.outcome_messages.as_ref() {
            if messages.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<protocol::MutationMessageLedgerRetirement>(),depth:1,..Default::default()});}
            return child(RetirementDemand { copy_bytes: messages.next_copy_byte_demand(), capacity_bytes: messages.next_capacity_byte_demand(body)?, release_bytes: messages.next_release_byte_demand()?, depth: messages.next_depth_demand()? });
        }
        if let Some(owner) = self.external_retirement.as_ref() {
            return child(artifact_retirement_box_demands(owner, body)?);
        }
        if !self.edit.terminal_is_empty() {
            return child(self.edit.close_demands(original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?,body)?);
        }
        if self.projected.is_some(){return child(self.placement_demand()?);}
        if !self.post_read.terminal_is_empty(){return child(self.post_read.demand(&self.post,PagedOneItemPostMode::Mutable,body)?.ok_or_else(||fault(E::PREFIX,"post-source-return-quote"))?);}
        if let Some(owner) = self.sealer.as_ref() {
            return if owner.terminal_is_empty() { Ok(RetirementDemand { copy_bytes:size_of::<ArtifactStoreOneItemSealer<S,M>>(), depth: 1, ..Default::default() }) } else { child(owner.retirement_demands(body)?) };
        }
        if let Some(owner) = self.cursor.as_ref() {
            return if owner.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes:size_of::<S::Projection>(), depth: 1, ..Default::default() })
            } else {
                child(owner.close_demands(body)?)
            };
        }
        if !self.retirement.is_empty() {
            return child(RetirementDemand { copy_bytes: self.retirement.next_copy_byte_demand()?, capacity_bytes: self.retirement.next_capacity_byte_demand(body)?, release_bytes: self.retirement.next_release_byte_demand()?, depth: self.retirement.next_depth_demand()? });
        }
        if self.post.is_some() {
            return child(RetirementDemand{copy_bytes:size_of::<S>()+size_of::<Arc<S>>(),capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<S>(),depth:1,..Default::default()});
        }
        if let Some(inverse) = self.inverse_retirement.as_ref() {
            if inverse.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value_factory::vector::FactoryVectorRetirement<M>>(),depth:1,..Default::default()});}
            let factory=original_mutation_factory(&self.mutation_retirement,&self.mutation_authority,&self.mutation_source)?;
            return child(inverse.demands(factory, body)?);
        }
        if self.pending_inverse.is_some() { return child(semio_framework_value_factory::vector::FactoryVectorRetirement::<M>::constructor_demand()); }
        if self.returned_edit.is_some()||self.returned_edit_retirement.is_some(){return child(returned_edit_demands(&self.returned_edit,&self.returned_edit_retirement,body)?);}
        if self.returned_post.is_some(){return child(returned_post_demands(&self.returned_post,&self.snapshot_retirement)?);}
        if let Some(admission) = self.source_admission.as_ref() {
            return if admission.phase() == "ready" { child(admission.next_demand()?) } else { child(admission.next_return_demand()?) };
        }
        if let Some(admission)=self.mutation_admission.as_ref(){
            return if admission.phase()=="ready"{child(admission.next_demand()?)}else{child(admission.next_return_demand()?)};
        }
        if let Some(owner)=self.mutation_source.as_ref(){
            return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>(),depth:1,..Default::default()})}else{child(RetirementDemand{copy_bytes:owner.next_close_copy_byte_demand()?,capacity_bytes:owner.next_close_capacity_byte_demand(body)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?})};
        }
        if self.mutation_authority.is_some(){return child(self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(true,&None,body)?);}
        if let Some(owner) = self.source.as_ref() {
            return if owner.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes:size_of::<RetainedCloneSource<S,SnapshotRead<S>>>(), depth: 1, ..Default::default() })
            } else {
                child(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
            };
        }
        if let Some(original) = self.mutation.as_ref() {
            let birth = self.mutation_retirement.as_ref().ok_or_else(|| fault("structural", "mutation-issuer"))?.retirement_birth_bytes(original);
            return child(RetirementDemand { copy_bytes:2*size_of::<M>(),capacity_bytes: birth, depth: 1, ..Default::default() });
        }
        if self.base.is_some() {
            return child(artifact_retirement_owned_birth_demands(&self.base)?);
        }
        if let Some(original) = self.authority.as_ref() {
            let birth = original.retirement_birth_demand();
            return child(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() });
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        if let Some(original) = self.factory_close.iter().find_map(Option::as_ref) {
            return if original.terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { child(original.demands(body)?) };
        }
        Ok(Default::default())
    }
}

impl<S: SnapshotProjection<Output = S>, M: Send + Sync + 'static, E: PagedOneItemEdit<S, M>> Drop for PagedOneItemPreparation<S, M, E> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || (self.edit.terminal_is_empty()
                    && self.base.is_none()
                    && self.source_admission.is_none()
                    && self.source.is_none()
                    && self.cursor.is_none()
                    && self.projected.is_none()
            && self.post.is_none()
                    && self.pending_inverse.is_none()
                    && self.inverse_retirement.is_none()
                    && self.outcome_messages.is_none()
                    && self.outcome_publication.is_none()
                    && self.mutation.is_none()
                    && self.mutation_source.is_none()
                    && self.mutation_authority.is_none()
                    && self.mutation_admission.is_none()
                    && self.returned_edit.is_none()
                    && self.returned_edit_retirement.is_none()
                    && self.returned_post.is_none()
                    && self.authority.is_none()
                    && self.mutation_retirement.is_none()
                    && self.snapshot_retirement.is_none()
                    && self.sealer.is_none()
                    && self.external_retirement.is_none()
                    && self.retirement.is_empty()
                    && self.factory_close.iter().all(Option::is_none)),
            "structural preparation dropped with live owners"
        );
    }
}

impl<S,M,E> semio_os_store_replay_operation::ArtifactReplayPreparationFactory<S,M> for PagedOneItemPreparationFactory<S,M,E>
where S:SnapshotProjection<Output=S>,M:ArtifactCanonicalJsonTree+semio_framework_value::retirement::RetireOwned+Mutation<S>+Send+Sync+'static,E:PagedOneItemEdit<S,M>{
 fn preparation_birth_demand(&self)->RetirementDemand{
  type Producer<S,M,E>=semio_os_store_replay_operation::native::NativeSemanticPreparation<S,M,semio_os_store_replay_operation::native::PagedProjection<S>,semio_os_store_replay_operation::native::PagedSemanticEdit<S,M,E>>;
  RetirementDemand{copy_bytes:size_of::<semio_os_store_replay_operation::native::ArtifactReplayPreparationRequest<S,M>>()+size_of::<Producer<S,M,E>>(),capacity_bytes:size_of::<Producer<S,M,E>>(),depth:1,..Default::default()}
 }
 fn begin(&self,request:semio_os_store_replay_operation::native::ArtifactReplayPreparationRequest<S,M>,grant:RetainedCloneGrant)->Result<(Box<dyn semio_os_store_replay_operation::ArtifactReplayPreparation<S,M>>,RetainedCloneProgress),(ValueError,semio_os_store_replay_operation::native::ArtifactReplayPreparationRequest<S,M>)>{
  let d=semio_os_store_replay_operation::ArtifactReplayPreparationFactory::preparation_birth_demand(self);
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"semantic paged producer birth not funded"),request));}
  let admitted=request.input.borrow().and_then(|input|{if !E::recognizes(input.get()){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"semantic paged operation has no installed issuer"));}E::preflight(input.get()).map(|_|()).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"semantic paged operation footprint refused"))});
  if let Err(error)=admitted{return Err((error,request));}
  let owner=semio_os_store_replay_operation::native::NativeSemanticPreparation::new(request,semio_os_store_replay_operation::native::PagedProjection::<S>::default(),semio_os_store_replay_operation::native::PagedSemanticEdit::<S,M,E>::default());
  Ok((Box::new(owner),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
 }
}
