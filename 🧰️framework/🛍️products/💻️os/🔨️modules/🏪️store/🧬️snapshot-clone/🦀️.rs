//! 🧩 Bounded retained-clone publication preparation.

use crate::os_spr::{ActorId, Edit, MutationId, MutationMeta, UndoPolicy};
use crate::os_store::{
    ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactCanonicalJson, ArtifactCanonicalJsonTree, ArtifactStoreBatchDigest, admit_artifact_batch_digest, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority,
    ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemSealer, ErasedSnapshotRetirement,
    HistoryLane, SnapshotRead, SnapshotRetirementFactory,
};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_close};
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, shared::SharedControlledRetirement}, FactoryAuthority};
use std::{marker::PhantomData, mem::size_of, sync::Arc};
use semio_framework_value::retirement::original_vector::OriginalVectorCursor;
#[path="🪪️metadata/🦀️.rs"]
mod metadata;
use metadata::NativeEditMetadata;

#[path = "🚚️handoff/🦀️.rs"]
mod handoff;
use handoff::RetainedCloneCursorHandoff;
#[path = "♻️pending/🦀️.rs"]
mod pending;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetainedCloneEditStep {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress),
}

impl RetainedCloneEditStep {
    pub fn progress(&self) -> RetainedCloneProgress {
        match self {
            Self::Progress(progress) | Self::Complete(progress) => *progress,
        }
    }
}

/// ✏️ Applies one domain mutation to an exclusive cloned owner through bounded turns.
pub trait RetainedCloneEditCursor<P: RetainedClone, M>: Send {
    fn advance(&mut self, base: RetainedCloneRef<'_, P>, post: &mut P, mutation: RetainedCloneRef<'_, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError>;
    fn take_inverse(&mut self) -> Option<Vec<M>>;
    /// 🌐️ Returns the original domain cursor's completed foreign-step decision.
    fn foreign_step_presence(&self) -> bool;
    fn cancel(&mut self);
    fn begin_close(&mut self) -> bool;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪪 Admits and creates the domain cursor while the Store retains publication authority.
pub trait RetainedCloneEdit<P: RetainedClone, M>: semio_framework_value::FactoryRetirement + Send + Sync + 'static {
    type Cursor: RetainedCloneEditCursor<P, M>;
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self) -> RetainedCloneBirthDemand;
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand;
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError>;
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RetainedClonePreparationFactory<P, M, E> {
    #[factory_child]
    edit: Arc<E>,
    #[factory_child]
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    #[factory_child]
    snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
    maximum_depth: usize,
    marker: PhantomData<fn() -> (P, M)>,
}

impl<P, M, E> RetainedClonePreparationFactory<P, M, E> {
    pub fn new(edit: Arc<E>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>, maximum_depth: usize) -> Result<Self, String> {
        if maximum_depth == 0 {
            return Err("retained clone preparation requires a nonzero structural depth envelope".into());
        }
        Ok(Self { edit, mutation_retirement, snapshot_retirement, maximum_depth, marker: PhantomData })
    }
}

impl<P, M, E> ArtifactStoreOneItemPreparationFactory<P, M> for RetainedClonePreparationFactory<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn begin_batch_digest(&self, edit:&mut Option<Box<Edit<M>>>, grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>,RetainedCloneProgress)>,ValueError>{admit_artifact_batch_digest(edit,grant)}
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.edit.preflight(mutation, lane)
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        if !P::controlled_retirement_supported() || !M::controlled_retirement_supported() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "snapshot clone originals require controlled typed retirement"));
        }
        Ok(RetainedCloneBirthDemand{capacity_bytes:size_of::<RetainedClonePreparation<P,M,E>>(),depth:1})
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let footprint = match self.edit.preflight(&request.mutation, request.lane) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            _ => return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "snapshot clone preparation footprint refused"), request)),
        };
        let ArtifactStoreOneItemPreparationRequest { operation: _, generation: _, base_revision: _, lane: _, authority, base, mutation, mutation_retirement, snapshot_retirement } = request;
        Ok((Box::new(RetainedClonePreparation::<P, M, E> {
            pending_base: Some(base),
            pending_mutation: Some(mutation),
            pending_mutation_authority: None,
            mutation_owner: None,
            edit: Some(Arc::clone(&self.edit)),
            pending_base_close: None,
            pending_mutation_close: None,
            mutation_owner_close: None,
            source: None,
            clone_cursor: None,
            clone_handoff: None,
            copied: None,
            edit_cursor: None,
            inverse: None,
            foreign_step_presence: None,
            publication_post: None,
            publication_post_close: None,
            publication_inverse: None,
            publication_metadata: None,
            publication_metadata_close: None,
            publication_edit: None,
            publication_edit_close: None,
            build_stage: 0,
            mutation: None,
            authority: Some(authority),
            sealer: None,
            mutation_retirement: Some(mutation_retirement),
            snapshot_retirement: Some(snapshot_retirement),
            copied_close: None,
            inverse_close: None,
            authority_close: None,
            factory_close: None,
            footprint,
            retained_capacity_bytes: progress.retained_capacity_bytes,
            maximum_depth: self.maximum_depth,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            ownership: RetainedCloneProgress::default(),
            seal_base: ArtifactStoreOneItemCheckpoint::default(),
            phase: RetainedClonePreparationPhase::Source,
            cancelled: false,
            closing: false,
        }), progress))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedClonePreparationPhase {
    Source,
    MutationSource,
    CloneCursor,
    EditCursor,
    Clone,
    CloseClone,
    Edit,
    CaptureForeignStepPresence,
    CloseEdit,
    Build,
    Seal,
    RecordForeignStepPresence,
    Prepared,
}

struct RetainedClonePreparation<P: RetainedClone, M: RetireOwned + Sync, E: RetainedCloneEdit<P, M>> {
    pending_base: Option<SnapshotRead<P>>,
    pending_base_close: Option<ControlledRetirement<SnapshotRead<P>>>,
    pending_mutation_close: Option<ControlledRetirement<M>>,
    mutation_owner_close: Option<SharedControlledRetirement<M>>,
    pending_mutation: Option<M>,
    pending_mutation_authority: Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>,
    mutation_owner: Option<Arc<M>>,
    edit: Option<Arc<E>>,
    source: Option<RetainedCloneSource<P,SnapshotRead<P>>>,
    clone_cursor: Option<P::Cursor>,
    clone_handoff: Option<RetainedCloneCursorHandoff<P>>,
    copied: Option<P>,
    edit_cursor: Option<E::Cursor>,
    inverse: Option<Vec<M>>,
    foreign_step_presence: Option<bool>,
    publication_post: Option<Arc<P>>,
    publication_post_close: Option<SharedControlledRetirement<P>>,
    publication_inverse: Option<Box<OriginalVectorCursor<M>>>,
    publication_metadata: Option<Box<NativeEditMetadata<M>>>,
    publication_metadata_close: Option<ControlledRetirement<Box<NativeEditMetadata<M>>>>,
    publication_edit: Option<Edit<M>>,
    publication_edit_close: Option<ControlledRetirement<Edit<M>>>,
    build_stage: u8,
    mutation: Option<RetainedCloneSource<M>>,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    sealer: Option<ArtifactStoreOneItemSealer<P, M>>,
    mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    copied_close: Option<ControlledRetirement<P>>,
    inverse_close: Option<ControlledRetirement<Vec<M>>>,
    authority_close: Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>,
    factory_close: Option<FactoryAuthority>,
    footprint: ArtifactStoreOneItemFootprint,
    retained_capacity_bytes: usize,
    maximum_depth: usize,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    ownership: RetainedCloneProgress,
    seal_base: ArtifactStoreOneItemCheckpoint,
    phase: RetainedClonePreparationPhase,
    cancelled: bool,
    closing: bool,
}

impl<P: RetainedClone, M: ArtifactCanonicalJsonTree + RetireOwned + Sync, E: RetainedCloneEdit<P, M>> RetainedClonePreparation<P, M, E> {
    fn handoff_clone_cursor(&mut self) -> Result<(), semio_framework_value::ValueError> {
        if self.clone_handoff.is_some() {
            return Ok(());
        }
        let cursor = self.clone_cursor.take().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation lost its cursor before close handoff"))?;
        self.clone_handoff = Some(RetainedCloneCursorHandoff::new(cursor));
        Ok(())
    }

    fn record_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        self.ownership=progress;
        let bytes = progress.copied_bytes.checked_add(progress.retained_capacity_bytes).and_then(|bytes| bytes.checked_add(progress.released_bytes)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation byte progress overflow").with_retained_progress(self.ownership))?;
        self.retained_capacity_bytes = self.retained_capacity_bytes.checked_add(progress.retained_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation retained capacity overflow").with_retained_progress(self.ownership))?;
        if self.retained_capacity_bytes > self.footprint.retained_bytes {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation exceeded its admitted retained capacity").with_retained_progress(self.ownership));
        }
        self.checkpoint.cursor = self.checkpoint.cursor.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation cursor overflow").with_retained_progress(self.ownership))?;
        self.checkpoint.completed_items =
            self.checkpoint.completed_items.checked_add(u32::try_from(progress.copied_items).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation item progress overflow").with_retained_progress(self.ownership))?).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation item progress overflow").with_retained_progress(self.ownership))?;
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.checked_add(u64::try_from(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation byte progress overflow").with_retained_progress(self.ownership))?).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation byte progress overflow").with_retained_progress(self.ownership))?;
        Ok(())
    }

    fn merged_seal_checkpoint(&self, seal: ArtifactStoreOneItemCheckpoint) -> Result<ArtifactStoreOneItemCheckpoint, ValueError> {
        Ok(ArtifactStoreOneItemCheckpoint {
            cursor: self.seal_base.cursor.checked_add(seal.cursor).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone seal cursor overflow").with_retained_progress(self.ownership))?,
            completed_items: self.seal_base.completed_items.checked_add(seal.completed_items).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone seal item progress overflow").with_retained_progress(self.ownership))?,
            completed_bytes: self.seal_base.completed_bytes.checked_add(seal.completed_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone seal byte progress overflow").with_retained_progress(self.ownership))?,
            digest: seal.digest,
        })
    }

    fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        macro_rules! typed {
            ($value:expr) => { return Ok(RetirementDemand { copy_bytes: $value.next_close_copy_byte_demand()?, capacity_bytes: $value.next_close_capacity_byte_demand(body)?, release_bytes: $value.next_close_release_byte_demand()?, depth: $value.next_close_depth_demand()? }); };
        }
        macro_rules! controlled {
            ($value:expr) => { return Ok(RetirementDemand { copy_bytes: $value.next_copy_byte_demand()?, capacity_bytes: $value.next_capacity_byte_demand(body)?, release_bytes: $value.next_release_byte_demand()?, depth: $value.next_depth_demand()? }); };
        }
        if let Some(owner) = self.clone_handoff.as_ref() { controlled!(owner); }
        if let Some(owner) = self.clone_cursor.as_ref() { typed!(owner); }
        if let Some(owner)=self.edit_cursor.as_ref(){if !owner.terminal_is_empty(){typed!(owner);}}
        if let Some(owner)=self.sealer.as_ref(){return owner.retirement_demands(body);}
        if let Some(owner)=self.publication_metadata_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Box<NativeEditMetadata<M>>>>>(),depth:1,..Default::default()});}let copy=owner.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native metadata close depth overflow"))?});}
        if self.publication_metadata.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Box<NativeEditMetadata<M>>>()+size_of::<ControlledRetirement<Box<NativeEditMetadata<M>>>>(),depth:1,..Default::default()});}
        if let Some(owner)=self.publication_edit_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Edit<M>>>>(),depth:1,..Default::default()});}let copy=owner.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original assembled edit close depth overflow"))?});}
        if self.publication_edit.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Edit<M>>()+size_of::<ControlledRetirement<Edit<M>>>(),depth:1,..Default::default()});}
        if let Some(owner)=self.publication_inverse.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalVectorCursor<M>>>>(),release_bytes:size_of::<OriginalVectorCursor<M>>(),depth:1,..Default::default()});}let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original inverse vector parent depth overflow"))?;return Ok(demand);}
        if let Some(owner)=self.publication_post_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SharedControlledRetirement<P>>>(),depth:1,..Default::default()});}let copy=owner.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original publication post parent depth overflow"))?});}
        if self.publication_post.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<P>>()+size_of::<SharedControlledRetirement<P>>(),depth:1,..Default::default()});}
        if let Some(owner) = self.inverse_close.as_ref() { controlled!(owner); }
        if self.inverse.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Vec<M>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.copied_close.as_ref() { controlled!(owner); }
        if self.copied.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<P>(), depth: 1, ..Default::default() }); }
        if self.pending_mutation_close.is_some() || self.pending_mutation.is_some() { return pending::demands(&self.pending_mutation, &self.pending_mutation_close, body); }
        if let Some(owner)=self.pending_mutation_authority.as_ref(){controlled!(owner);}
        if let Some(owner)=self.mutation_owner_close.as_ref(){controlled!(owner);}
        if self.mutation_owner.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<M>>(),depth:1,..Default::default()});}
        if let Some(owner) = self.mutation.as_ref() { typed!(owner); }
        if self.pending_base_close.is_some() || self.pending_base.is_some() { return pending::demands(&self.pending_base, &self.pending_base_close, body); }
        if let Some(owner) = self.source.as_ref() { typed!(owner); }
        if let Some(owner) = self.authority_close.as_ref() { controlled!(owner); }
        if self.authority.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factory_close.as_ref() { return owner.demands(body); }
        if self.edit.is_some() || self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        Ok(Default::default())
    }

    fn build_sealer(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<bool, ValueError> {
        use semio_framework_value::{RetirementDemand,retirement::shared::shared_retirement_allocation_bytes};
        let g=grant.retained_grant();
        let demand=match self.build_stage{
            0=>RetirementDemand{copy_bytes:size_of::<Arc<M>>(),depth:1,..Default::default()},
            1=>{let source=self.mutation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward source missing before publication lease closure").with_retained_progress(self.ownership))?;if source.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}}else{let body=source.next_close_copy_byte_demand()?;RetirementDemand{copy_bytes:body,capacity_bytes:source.next_close_capacity_byte_demand(body)?,release_bytes:source.next_close_release_byte_demand()?,depth:source.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original forward source depth overflow").with_retained_progress(self.ownership))?}}},
            2=>RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneSource<M>>>(),depth:1,..Default::default()},
            3=>RetirementDemand{copy_bytes:size_of::<M>()+size_of::<Arc<M>>(),release_bytes:shared_retirement_allocation_bytes::<M>(),depth:1,..Default::default()},
            4=>RetirementDemand{copy_bytes:size_of::<P>()+size_of::<Arc<P>>(),capacity_bytes:shared_retirement_allocation_bytes::<P>(),depth:1,..Default::default()},
            5=>{let mut d=OriginalVectorCursor::<M>::constructor_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<OriginalVectorCursor<M>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original inverse vector transfer overflow").with_retained_progress(self.ownership))?;d.capacity_bytes=size_of::<OriginalVectorCursor<M>>();d},
            6=>NativeEditMetadata::<M>::constructor_demand(),
            7=>{let owner=self.publication_metadata.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata frame missing").with_retained_progress(self.ownership))?;owner.next_demand(self.authority.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata authority missing").with_retained_progress(self.ownership))?,self.publication_inverse.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse cursor missing").with_retained_progress(self.ownership))?)?},
            8=>RetirementDemand{copy_bytes:size_of::<Option<Box<NativeEditMetadata<M>>>>(),release_bytes:size_of::<NativeEditMetadata<M>>(),depth:1,..Default::default()},
            9=>{let owner=self.publication_inverse.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse backing missing").with_retained_progress(self.ownership))?;if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}}else{let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original inverse backing depth overflow").with_retained_progress(self.ownership))?;demand}},
            10=>RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalVectorCursor<M>>>>(),release_bytes:size_of::<OriginalVectorCursor<M>>(),depth:1,..Default::default()},
            11=>{let birth=ArtifactStoreOneItemSealer::<P,M>::constructor_demand();RetirementDemand{copy_bytes:ArtifactStoreOneItemSealer::<P,M>::constructor_copy_byte_demand()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<Arc<P>>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Arc<dyn SnapshotRetirementFactory<P>>>(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()}},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication assembly has no admitted stage").with_retained_progress(self.ownership)),
        };
        if g.maximum_items==0||demand.copy_bytes>g.maximum_copy_bytes||demand.capacity_bytes>g.maximum_capacity_bytes||demand.release_bytes>g.maximum_release_bytes||demand.depth>g.maximum_depth{return Ok(false);}
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
        match self.build_stage{
            0=>{self.mutation_owner=Some(self.mutation.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward source missing at publication detach").with_retained_progress(self.ownership))?.take_owner().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward owner missing at publication detach").with_retained_progress(self.ownership))?);self.build_stage=1;},
            1=>{let source=self.mutation.as_mut().unwrap();if source.terminal_is_empty(){self.build_stage=2;}else{progress=source.close_step(RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g})?.progress();}},
            2=>{self.mutation=None;self.build_stage=3;},
            3=>{let original=self.mutation_owner.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication forward Arc missing").with_retained_progress(self.ownership))?;if Arc::strong_count(original)!=1||Arc::weak_count(original)!=0{return Ok(false);}let original=self.mutation_owner.take().unwrap();match Arc::try_unwrap(original){Ok(original)=>self.pending_mutation=Some(original),Err(original)=>{self.mutation_owner=Some(original);return Ok(false);}}self.build_stage=4;},
            4=>{self.publication_post=Some(Arc::new(self.copied.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original exclusive post missing before shared publication birth").with_retained_progress(self.ownership))?));self.build_stage=5;},
            5=>{let original=self.inverse.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse vector missing before publication handoff").with_retained_progress(self.ownership))?;match OriginalVectorCursor::admit(original,g){Ok((owner,receipt))=>{self.publication_inverse=Some(Box::new(owner));progress=RetainedCloneProgress{copied_bytes:receipt.copied_bytes+size_of::<OriginalVectorCursor<M>>(),retained_capacity_bytes:size_of::<OriginalVectorCursor<M>>(),..receipt};self.build_stage=6;},Err((error,original))=>{self.inverse=Some(original);return Err(error);}}},
            6=>{let(owner,receipt)=NativeEditMetadata::<M>::admit(self.authority.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original authority missing at metadata admission").with_retained_progress(self.ownership))?,g)?;self.publication_metadata=Some(owner);progress=receipt;self.build_stage=7;},
            7=>{let owner=self.publication_metadata.as_mut().unwrap();if owner.ready(){let(original,receipt)=owner.take(g)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata take lacks grant").with_retained_progress(self.ownership))?;self.publication_edit=Some(original);progress=receipt;self.build_stage=8;}else{progress=owner.step(self.authority.as_ref().unwrap(),&mut self.pending_mutation,self.publication_inverse.as_mut().unwrap(),g)?;if progress==Default::default(){return Ok(false);}}},
            8=>{if !self.publication_metadata.as_ref().unwrap().terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata frame still retains fields before release").with_retained_progress(self.ownership));}self.publication_metadata=None;self.build_stage=9;},
            9=>{let owner=self.publication_inverse.as_mut().unwrap();if owner.terminal_is_empty(){self.build_stage=10;}else{owner.begin_close();progress=owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g})?.progress();}},
            10=>{if !self.publication_inverse.as_ref().unwrap().terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse frame retains backing before release").with_retained_progress(self.ownership));}self.publication_inverse=None;self.build_stage=11;},
            11=>{if self.authority.is_none()||self.publication_edit.is_none()||self.publication_post.is_none()||self.mutation_retirement.is_none()||self.snapshot_retirement.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication seal is missing an original owner").with_retained_progress(self.ownership));}let authority=self.authority.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication authority missing at seal").with_retained_progress(self.ownership))?;let original=self.publication_edit.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication edit missing at seal").with_retained_progress(self.ownership))?;let post=self.publication_post.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication post missing at seal").with_retained_progress(self.ownership))?;let mutation_retirement=self.mutation_retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original mutation factory missing at seal").with_retained_progress(self.ownership))?;let snapshot_retirement=self.snapshot_retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original snapshot factory missing at seal").with_retained_progress(self.ownership))?;match ArtifactStoreOneItemSealer::admit(authority,original,post,mutation_retirement,snapshot_retirement,g){Ok((owner,receipt))=>{self.sealer=Some(owner);progress=RetainedCloneProgress{copied_bytes:demand.copy_bytes,..receipt};self.phase=RetainedClonePreparationPhase::Seal;self.build_stage=12;},Err((error,authority,original,post,mutation_retirement,snapshot_retirement))=>{self.authority=Some(authority);self.publication_edit=Some(original);self.publication_post=Some(post);self.mutation_retirement=Some(mutation_retirement);self.snapshot_retirement=Some(snapshot_retirement);return Err(error);}}},
            _=>unreachable!(),
        }
        self.record_progress(admit_retained_clone_progress(g,progress,"original publication assembly").map_err(|error|error.with_retained_progress(progress))?)?;
        if self.phase==RetainedClonePreparationPhase::Seal{self.seal_base=self.checkpoint;}
        Ok(true)
    }


    fn advance_original(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        self.ownership=RetainedCloneProgress::default();
        if self.cancelled || self.closing || !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
        match self.phase {
            RetainedClonePreparationPhase::Source => {
                let bytes=RetainedCloneSource::<P>::borrowed_constructor_capacity_bytes::<SnapshotRead<P>>();
                if (RetainedCloneBirthDemand{capacity_bytes:bytes,depth:1}).admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                if clone_grant.maximum_copy_bytes<RetainedCloneSource::<P>::borrowed_constructor_copy_bytes::<SnapshotRead<P>>(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let base=self.pending_base.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original captured base").with_retained_progress(self.ownership))?;
                match base.admit_retained_clone_source(clone_grant){
                    Ok((source,progress))=>{self.source=Some(source);self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::MutationSource;},
                    Err((error,base))=>{self.pending_base=Some(base);return Err(error);},
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::MutationSource => {
                let demand=RetainedCloneSource::<M>::owned_constructor_demand::<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>();
                if demand.admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                if clone_grant.maximum_copy_bytes<RetainedCloneSource::<M>::constructor_copy_bytes(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let mutation=self.pending_mutation.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original owned mutation").with_retained_progress(self.ownership))?;
                let authority=self.pending_mutation_authority.take().unwrap_or_else(||SharedControlledRetirement::lease(Arc::clone(self.authority.as_ref().expect("retained original mutation publication authority"))));
                match RetainedCloneSource::admit_owned(mutation,authority,clone_grant){
                    Ok((source,progress))=>{self.mutation=Some(source);self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::CloneCursor;},
                    Err((error,mutation,authority))=>{self.pending_mutation=Some(mutation);self.pending_mutation_authority=Some(authority);return Err(error);},
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CloneCursor => {
                let demand=self.edit.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost native clone birth authority").with_retained_progress(self.ownership))?.snapshot_cursor_birth_demand();
                let progress=match demand.admit(clone_grant){Ok(progress)=>progress,Err(_)=>return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};
                self.clone_cursor=Some(P::retained_clone_cursor());self.record_progress(progress)?;

                self.phase=RetainedClonePreparationPhase::EditCursor;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::EditCursor => {
                let edit=self.edit.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original editor authority").with_retained_progress(self.ownership))?;
                let demand=edit.begin_demand();
                if demand.admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let(cursor,progress)=edit.begin(clone_grant)?;
                self.edit_cursor=Some(cursor);
                let progress=admit_retained_clone_progress(clone_grant,progress,"snapshot clone native edit constructor")?;
                if progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot clone editor constructor disagrees with its original admitted birth demand").with_retained_progress(self.ownership));}
                self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::Clone;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::Clone => {
                let step = self
                    .clone_cursor
                    .as_mut()
                    .ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its active clone cursor").with_retained_progress(self.ownership))?
                    .advance(self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its source").with_retained_progress(self.ownership))?.borrow(), clone_grant)
                    ?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation snapshot clone")?;
                self.record_progress(progress)?;
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.copied = Some(self.clone_cursor.as_mut().and_then(RetainedCloneCursor::take).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone cursor completed without its owner").with_retained_progress(self.ownership))?);
                    self.handoff_clone_cursor()?;
                    self.phase = RetainedClonePreparationPhase::CloseClone;
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CloseClone => {
                let handoff = self.clone_handoff.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its Store-owned clone cursor handoff").with_retained_progress(self.ownership))?;
                if handoff.terminal_is_empty() {
                    self.clone_handoff = None;
                    self.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    self.phase = RetainedClonePreparationPhase::Edit;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership));
                }
                let step = handoff.close_step(clone_grant)?;
                let step = admit_retained_clone_close(clone_grant, step, handoff.terminal_is_empty(), "retained clone preparation clone close")?;
                self.record_progress(step.progress())?;
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership) })
            }
            RetainedClonePreparationPhase::Edit => {
                let step = self.edit_cursor.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its admitted editor cursor").with_retained_progress(self.ownership))?.advance(
                    self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its edit source").with_retained_progress(self.ownership))?.borrow(),
                    self.copied.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its exclusive post snapshot").with_retained_progress(self.ownership))?,
                    self.mutation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its forward mutation").with_retained_progress(self.ownership))?.borrow(),
                    clone_grant,
                )?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation typed edit")?;
                self.record_progress(progress)?;
                if matches!(step,RetainedCloneEditStep::Complete(_)){self.phase=RetainedClonePreparationPhase::CaptureForeignStepPresence;}
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CaptureForeignStepPresence => {
                let copy=size_of::<Vec<M>>()+size_of::<bool>()+size_of::<RetainedClonePreparationPhase>();
                if clone_grant.maximum_copy_bytes<copy||clone_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let cursor=self.edit_cursor.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit decision cursor is absent").with_retained_progress(self.ownership))?;
                self.foreign_step_presence=Some(cursor.foreign_step_presence());self.inverse=Some(cursor.take_inverse().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit completed without inverse").with_retained_progress(self.ownership))?);let _=cursor.begin_close();self.phase=RetainedClonePreparationPhase::CloseEdit;
                self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CloseEdit => {
                let step = self.edit_cursor.as_mut().unwrap().close_step(clone_grant)?;
                let step = admit_retained_clone_close(clone_grant, step, self.edit_cursor.as_ref().is_none_or(|cursor|cursor.terminal_is_empty()), "retained clone preparation edit close")?;
                self.record_progress(step.progress())?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = RetainedClonePreparationPhase::Build; }
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership) })
            }
            RetainedClonePreparationPhase::Build => {
                if self.build_sealer(grant)? {
                    Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
                } else {
                    Ok(ArtifactStoreOneItemPreparationStep::Blocked)
                }
            }
            RetainedClonePreparationPhase::Seal => {
                let result = self.sealer.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its canonical sealer").with_retained_progress(self.ownership))?.advance(grant);
                let receipt=self.sealer.as_ref().unwrap().ownership_progress();
                self.ownership=receipt;
                admit_retained_clone_progress(clone_grant,receipt,"retained clone preparation original canonical source").map_err(|error|error.with_retained_progress(receipt))?;
                self.retained_capacity_bytes=self.retained_capacity_bytes.checked_add(receipt.retained_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone original canonical capacity overflow").with_retained_progress(self.ownership))?;
                if self.retained_capacity_bytes>self.footprint.retained_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone original canonical source exceeded admitted footprint").with_retained_progress(self.ownership));}
                let step = result?;
                let seal_checkpoint = match step {
                    ArtifactStoreOneItemPreparationStep::Progress(checkpoint,receipt) | ArtifactStoreOneItemPreparationStep::Prepared(checkpoint,receipt) => {admit_retained_clone_progress(clone_grant,receipt,"retained clone preparation original native seal")?;checkpoint},
                    ArtifactStoreOneItemPreparationStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                };
                self.checkpoint = self.merged_seal_checkpoint(seal_checkpoint)?;
                if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
                    self.phase=RetainedClonePreparationPhase::RecordForeignStepPresence;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership));
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::RecordForeignStepPresence => {
                let copy=size_of::<bool>()+size_of::<RetainedClonePreparationPhase>();if clone_grant.maximum_copy_bytes<copy||clone_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let presence=self.foreign_step_presence.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit foreign-step decision is absent").with_retained_progress(self.ownership))?;
                if !self.sealer.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original sealer is absent before declaration").with_retained_progress(self.ownership))?.record_foreign_step_presence(presence){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared owner is absent before declaration").with_retained_progress(self.ownership));}
                self.phase=RetainedClonePreparationPhase::Prepared;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::Prepared => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,Default::default())),
        }
    }

}

impl<P, M, E> ArtifactStoreOneItemPreparation<P, M> for RetainedClonePreparation<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        match self.advance_original(grant) {
            Ok(step)=>Ok(step),
            Err(error)=>{self.cancelled=true;if error.retained_progress()!=Default::default(){self.ownership=error.retained_progress();}Err(error.with_retained_progress(self.ownership))}
        }
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<P, M>> {
        self.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, M>> {
        self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(cursor)=self.edit_cursor.as_mut(){cursor.cancel();}
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(cursor) = self.clone_cursor.as_mut() { cursor.begin_close(); }
        if let Some(cursor)=self.edit_cursor.as_mut(){let _=cursor.begin_close();}
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = grant.retained_grant();
        let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_close_depth_demand()? {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "snapshot clone close exceeds granted depth"));
        }
        if let Some(handoff) = self.clone_handoff.as_mut() {
            let step = handoff.close_step(grant)?;
            if handoff.terminal_is_empty() { self.clone_handoff = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(cursor) = self.clone_cursor.as_mut() {
            let step = cursor.close_step(grant)?;
            if cursor.terminal_is_empty() { self.clone_cursor = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner)=self.edit_cursor.as_mut(){let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.edit_cursor=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=self.sealer.as_mut(){let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.sealer=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.publication_metadata_close.is_some(){let d=self.close_demand(grant.maximum_copy_bytes)?;if d.depth>grant.maximum_depth||d.copy_bytes>grant.maximum_copy_bytes||d.capacity_bytes>grant.maximum_capacity_bytes||d.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}let owner=self.publication_metadata_close.as_mut().unwrap();if owner.terminal_is_empty(){self.publication_metadata_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..empty}));}return owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.publication_metadata.is_some(){let d=self.close_demand(grant.maximum_copy_bytes)?;if d.depth>grant.maximum_depth||d.copy_bytes>grant.maximum_copy_bytes{return Ok(RetainedCloneStep::Progress(empty));}let original=self.publication_metadata.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.publication_metadata_close=Some(owner),Err((error,original))=>{self.publication_metadata=Some(original);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..empty}));}
        if self.publication_edit_close.is_some(){let d=self.close_demand(grant.maximum_copy_bytes)?;if d.depth>grant.maximum_depth||d.copy_bytes>grant.maximum_copy_bytes||d.capacity_bytes>grant.maximum_capacity_bytes||d.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}let owner=self.publication_edit_close.as_mut().unwrap();if owner.terminal_is_empty(){self.publication_edit_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..empty}));}return owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.publication_edit.is_some(){let d=self.close_demand(grant.maximum_copy_bytes)?;if d.depth>grant.maximum_depth||d.copy_bytes>grant.maximum_copy_bytes{return Ok(RetainedCloneStep::Progress(empty));}let original=self.publication_edit.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.publication_edit_close=Some(owner),Err((error,original))=>{self.publication_edit=Some(original);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..empty}));}
        if self.publication_inverse.is_some(){let demand=self.close_demand(grant.maximum_copy_bytes)?;if demand.depth>grant.maximum_depth||demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}let owner=self.publication_inverse.as_mut().unwrap();if owner.terminal_is_empty(){self.publication_inverse=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));}owner.begin_close();return owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.publication_post_close.is_some(){let demand=self.close_demand(grant.maximum_copy_bytes)?;if demand.depth>grant.maximum_depth||demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}let owner=self.publication_post_close.as_mut().unwrap();if owner.terminal_is_empty(){self.publication_post_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}return owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.publication_post.is_some(){let bytes=size_of::<Arc<P>>()+size_of::<SharedControlledRetirement<P>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}self.publication_post_close=Some(SharedControlledRetirement::new(self.publication_post.take().unwrap()));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..empty}));}
        if let Some(owner) = self.inverse_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.inverse_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.inverse.is_some() {
            let bytes = size_of::<Vec<M>>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let value = self.inverse.take().unwrap();
            match ControlledRetirement::new(value) {
                Ok(owner) => self.inverse_close = Some(owner),
                Err((error, value)) => { self.inverse = Some(value); return Err(error); }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        if let Some(owner) = self.copied_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.copied_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.copied.is_some() {
            let bytes = size_of::<P>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let value = self.copied.take().unwrap();
            match ControlledRetirement::new(value) {
                Ok(owner) => self.copied_close = Some(owner),
                Err((error, value)) => { self.copied = Some(value); return Err(error); }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        if let Some(step)=pending::close(&mut self.pending_mutation,&mut self.pending_mutation_close,grant)?{return Ok(step);}
        if let Some(owner)=self.pending_mutation_authority.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.pending_mutation_authority=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=self.mutation_owner_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.mutation_owner_close=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.mutation_owner.is_some(){let bytes=size_of::<Arc<M>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}self.mutation_owner_close=Some(SharedControlledRetirement::lease(self.mutation_owner.take().unwrap()));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..empty}));}
        if let Some(source) = self.mutation.as_mut() {
            let step = source.close_step(grant)?;
            if source.terminal_is_empty() { self.mutation = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(step)=pending::close(&mut self.pending_base,&mut self.pending_base_close,grant)?{return Ok(step);}
        if let Some(source) = self.source.as_mut() {
            let step = source.close_step(grant)?;
            if source.terminal_is_empty() { self.source = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.authority_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.authority_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.authority.is_some() {
            let bytes = size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            self.authority_close = Some(SharedControlledRetirement::lease(self.authority.take().unwrap()));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        if let Some(owner) = self.factory_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.factory_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            let bytes = size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let owner: Arc<dyn semio_framework_value::FactoryRetirement> = if let Some(owner)=self.edit.take(){owner}else if let Some(owner) = self.mutation_retirement.take() { owner } else { self.snapshot_retirement.take().unwrap() };
            self.factory_close = Some(FactoryAuthority::new(owner));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.clone_cursor.is_none()
            && self.clone_handoff.is_none()
            && self.edit_cursor.as_ref().is_none_or(|cursor|cursor.terminal_is_empty())
            && self.pending_base.is_none()
            && self.pending_mutation.is_none()
            && self.pending_mutation_authority.is_none()
            && self.mutation_owner.is_none()
            && self.edit.is_none()
            && self.pending_base.is_none()
            && self.pending_base_close.is_none()
            && self.pending_mutation.is_none()
            && self.pending_mutation_close.is_none()
            && self.pending_mutation_authority.is_none()
            && self.mutation_owner.is_none()
            && self.mutation_owner_close.is_none()
            && self.edit.is_none()
            && self.source.is_none()
            && self.copied.is_none()
            && self.inverse.is_none()
            && self.publication_inverse.is_none()
            && self.publication_post.is_none()
            && self.publication_post_close.is_none()
            && self.publication_metadata.is_none()
            && self.publication_metadata_close.is_none()
            && self.publication_edit.is_none()
            && self.publication_edit_close.is_none()
            && self.mutation.is_none()
           
            && self.authority.is_none()
            && self.sealer.is_none()
            && self.copied_close.is_none()
            && self.inverse_close.is_none()
            && self.authority_close.is_none()
            && self.factory_close.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
