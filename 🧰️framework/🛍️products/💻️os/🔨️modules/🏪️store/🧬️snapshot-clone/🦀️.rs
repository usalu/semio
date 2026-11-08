//! 🧩 Bounded retained-clone publication preparation.

use semio_os_spr::{ActorId, Edit, MutationId, MutationMeta, UndoPolicy};
use semio_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_os_store_canonical_json::ArtifactCanonicalJson;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_os_store_one_item_publication::native_source::pay;
use semio_os_store_one_item_publication::batch_digest::{ArtifactStoreBatchDigest, admit_artifact_batch_digest};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, RetainedAllocationEffect, OriginalRetirementPoll, RetirementPollExit};
use semio_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared};
use semio_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemSealer;
use semio_os_store_command::HistoryLane;
use semio_framework_value_snapshot_read::SnapshotRead;
use semio_framework_value::retained_clone::{OriginalEffectReply, RetainedClone, RetainedCloneCursor, RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceAdmission, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_close};
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, arc_custody::SharedControlledRetirement}, FactoryAuthority};
use std::{marker::PhantomData, mem::size_of, sync::Arc};
use semio_framework_value_original_vector::OriginalVectorCursor;
#[path="🪪️metadata/🦀️.rs"]
mod metadata;
use metadata::NativeEditMetadata;
#[path="📦️assembly/🦀️.rs"]
pub mod assembly;
pub use assembly::{OriginalPublicationAssembly,OriginalPublicationInputs};

#[path = "🚚️handoff/🦀️.rs"]
mod handoff;
use handoff::RetainedCloneCursorHandoff;
#[path = "♻️pending/🦀️.rs"]
mod pending;
#[path = "♻️custody/🦀️.rs"]
pub mod custody;
pub use custody::RetainedEditCustody;
#[path = "✏️config-apply/🦀️.rs"]
pub mod config_apply;
#[cfg(any(test, feature = "law"))]
#[path = "⚖️law/🦀️.rs"]
mod law;
#[cfg(feature = "law")]
pub use law::{drive_one_item_preparation_law, one_item_preparation_law_request, OneItemPreparationLaw};
pub use config_apply::{CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH,CONFIG_EDIT_RETAINED_BASELINE_BYTES,ConfigApplyCursor,ConfigApplyEdit,ConfigApplyMutation,config_apply_preparation_factory};

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
    fn advance_demands(&self, base: RetainedCloneRef<'_, P>, post: &P, mutation: RetainedCloneRef<'_, M>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_work_demand(&self,base:RetainedCloneRef<'_,P>,post:&P,mutation:RetainedCloneRef<'_,M>,body:usize)->Result<u64,ValueError>;
    fn advance(&mut self, base: RetainedCloneRef<'_, P>, post: &mut P, mutation: RetainedCloneRef<'_, M>, body:usize,effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneEditStep, ValueError>;
    fn take_inverse(&mut self) -> Option<Vec<M>>;
    /// 🌐️ Returns the original domain cursor's completed foreign-step decision.
    fn foreign_step_presence(&self) -> bool;
    fn cancel(&mut self);
    fn begin_close(&mut self) -> bool;
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪪 Admits and creates the domain cursor while the Store retains publication authority.
pub trait RetainedCloneEdit<P: RetainedClone, M>: semio_framework_value::FactoryRetirement + Send + Sync + 'static {
    type Cursor: RetainedCloneEditCursor<P, M>;
    /// 🧊️ Refuses with authored static prose so a denied native operation creates no text owner.
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, &'static str>;
    fn begin_demand(&self) -> RetainedCloneBirthDemand;
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand;
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError>;
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RetainedClonePreparationFactory<P, M, E> {
    #[factory_child]
    pub edit: Arc<E>,
    #[factory_child]
    pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    #[factory_child]
    pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
    pub maximum_depth: usize,
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
        self.edit.preflight(mutation, lane).map_err(str::to_owned)
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        if !P::controlled_retirement_supported() || !M::controlled_retirement_supported() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "snapshot clone originals require controlled typed retirement"));
        }
        Ok(RetainedCloneBirthDemand{capacity_bytes:size_of::<RetainedClonePreparation<P,M,E>>(),depth:1})
    }
    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<P, M, M>>().checked_add(std::mem::size_of::<RetainedClonePreparation<P,M,E>>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


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
            source_admission: None,
            mutation_admission: None,
            pending_mutation: Some(mutation),
            pending_mutation_authority: None,
            edit: Some(Arc::clone(&self.edit)),
            pending_base_close: None,
            pending_mutation_close: None,
            source: None,
            clone_cursor: None,
            clone_handoff: None,
            copied: None,
            edit_cursor: None,
            inverse: None,
            foreign_step_presence: None,
            assembly:OriginalPublicationAssembly::default(),
            build_stage: 0,
            mutation: None,
            authority: Some(authority),
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
            last_work:0,
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
    TakeClone,
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
    source_admission: Option<RetainedCloneSourceAdmission<P,SnapshotRead<P>>>,
    mutation_admission: Option<RetainedCloneSourceAdmission<M,M,SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>>,
    pending_base_close: Option<ControlledRetirement<SnapshotRead<P>>>,
    pending_mutation_close: Option<ControlledRetirement<M>>,
    pub pending_mutation: Option<M>,
    pending_mutation_authority: Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>,
    pub edit: Option<Arc<E>>,
    pub source: Option<RetainedCloneSource<P,SnapshotRead<P>>>,
    clone_cursor: Option<P::Cursor>,
    clone_handoff: Option<RetainedCloneCursorHandoff<P>>,
    pub copied: Option<P>,
    edit_cursor: Option<E::Cursor>,
    pub inverse: Option<Vec<M>>,
    pub foreign_step_presence: Option<bool>,
    assembly:OriginalPublicationAssembly<P,M>,
    build_stage: u8,
    pub mutation: Option<RetainedCloneSource<M>>,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    pub snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    copied_close: Option<ControlledRetirement<P>>,
    inverse_close: Option<ControlledRetirement<Vec<M>>>,
    pub authority_close: Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>,
    pub factory_close: Option<FactoryAuthority>,
    pub footprint: ArtifactStoreOneItemFootprint,
    pub retained_capacity_bytes: usize,
    pub maximum_depth: usize,
    pub checkpoint: ArtifactStoreOneItemCheckpoint,
    pub ownership: RetainedCloneProgress,
    last_work:u64,
    seal_base: ArtifactStoreOneItemCheckpoint,
    pub phase: RetainedClonePreparationPhase,
    pub cancelled: bool,
    pub closing: bool,
}

impl<P: RetainedClone, M: ArtifactCanonicalJsonTree + RetireOwned + Sync, E: RetainedCloneEdit<P, M>> RetainedClonePreparation<P, M, E> {
    fn handoff_clone_cursor(&mut self, effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.clone_handoff.is_some() {
            return Ok(Default::default());
        }
        let mut cursor = self.clone_cursor.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained clone preparation lost its cursor before close handoff"))?;
        let grant = effect.grant();
        let started = if cursor.close_started() { Ok(Default::default()) } else { cursor.begin_close_paid(&mut effect.nested(grant)).result };
        self.clone_handoff = Some(RetainedCloneCursorHandoff::new(cursor));
        started
    }

    /// 📏️ Charges a turn against the admitted footprint as live retained capacity: a turn first retains its birth (the peak), then frees its release.
    fn charge_footprint(&mut self, progress: RetainedCloneProgress, message: &'static str) -> Result<(), ValueError> {
        let live = self.retained_capacity_bytes.checked_add(progress.retained_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation retained capacity overflow").with_retained_progress(self.ownership))?;
        if live > self.footprint.retained_bytes {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,message).with_retained_progress(self.ownership));
        }
        self.retained_capacity_bytes = live.saturating_sub(progress.released_bytes);
        Ok(())
    }

    pub fn record_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        self.ownership=progress;
        self.charge_footprint(progress, "retained clone preparation exceeded its admitted retained capacity")?;
        self.record_document_progress(progress)
    }

    /// 📄️ Records a turn of the document-size snapshot clone: the document is owned by the lane, so its capacity is not part of the edit footprint.
    fn record_document_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        self.ownership=progress;
        let bytes = progress.copied_bytes.checked_add(progress.retained_capacity_bytes).and_then(|bytes| bytes.checked_add(progress.released_bytes)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained clone preparation byte progress overflow").with_retained_progress(self.ownership))?;
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

    pub fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
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
        if let Some(demand)=self.assembly.close_demands(body)?{return Ok(demand);}
        if let Some(owner) = self.inverse_close.as_ref() { controlled!(owner); }
        if self.inverse.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Vec<M>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.copied_close.as_ref() { controlled!(owner); }
        if self.copied.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<P>(), depth: 1, ..Default::default() }); }
        if self.pending_mutation_close.is_some() || self.pending_mutation.is_some() { return pending::demands(&self.pending_mutation, &self.pending_mutation_close, body); }
        if let Some(owner)=self.pending_mutation_authority.as_ref(){controlled!(owner);}
        if let Some(owner) = self.mutation.as_ref() { typed!(owner); }
        if let Some(admission)=self.source_admission.as_ref(){return if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_return_demand()};}
        if let Some(admission)=self.mutation_admission.as_ref(){return if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_return_demand()};}
        if self.pending_base_close.is_some() || self.pending_base.is_some() { return pending::demands(&self.pending_base, &self.pending_base_close, body); }
        if let Some(owner) = self.source.as_ref() { typed!(owner); }
        if let Some(owner) = self.authority_close.as_ref() { controlled!(owner); }
        if self.authority.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factory_close.as_ref() { return owner.demands(body); }
        if self.edit.is_some() || self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        Ok(Default::default())
    }

    fn build_sealer(&mut self,grant:ArtifactStoreOneItemGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
        use semio_framework_value::RetirementDemand;
        let g=grant.retained_grant();
        if self.build_stage>=4{
            let turn=self.assembly.advance(OriginalPublicationInputs{forward:&mut self.pending_mutation,post:&mut self.copied,inverse:&mut self.inverse,authority:&mut self.authority,mutations:&mut self.mutation_retirement,snapshots:&mut self.snapshot_retirement},effect);
            pay(effect,turn.work)?;
            let Some(progress)=turn.result?else{return Ok(false);};
            self.record_progress(progress)?;
            if self.assembly.sealer().is_some(){self.phase=RetainedClonePreparationPhase::Seal;self.seal_base=self.checkpoint;}
            return Ok(true);
        }
        let demand=match self.build_stage{
            0=>{let source=self.mutation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward source missing before publication detach").with_retained_progress(self.ownership))?;let body=source.next_take_copy_byte_demand()?;RetirementDemand{copy_bytes:body,capacity_bytes:source.next_take_capacity_byte_demand(body)?,release_bytes:source.next_take_release_byte_demand()?,depth:source.next_take_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original forward source depth overflow").with_retained_progress(self.ownership))?}},
            1=>{let source=self.mutation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward source missing before publication lease closure").with_retained_progress(self.ownership))?;if source.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}}else{let body=source.next_close_copy_byte_demand()?;RetirementDemand{copy_bytes:body,capacity_bytes:source.next_close_capacity_byte_demand(body)?,release_bytes:source.next_close_release_byte_demand()?,depth:source.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original forward source depth overflow").with_retained_progress(self.ownership))?}}},
            2=>RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneSource<M>>>(),depth:1,..Default::default()},
            3=>RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()},
            _=>return Err(ValueError::invariant("original forward source has no admitted frontier")),
        };
        if g.maximum_items==0||demand.copy_bytes>g.maximum_copy_bytes||demand.capacity_bytes>g.maximum_capacity_bytes||demand.release_bytes>g.maximum_release_bytes||demand.depth>g.maximum_depth{return Ok(false);}
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
        match self.build_stage{
            0=>{let source=self.mutation.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original forward source missing at publication detach").with_retained_progress(self.ownership))?;match match source.take_authority(&mut effect.nested(RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g})){semio_framework_value::RetainedWorkPoll::Pending{..}=>semio_framework_value::retained_clone::RetainedCloneSourceTake::Pending(Default::default()),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|failure|failure.into_value_error())?}{semio_framework_value::retained_clone::RetainedCloneSourceTake::Pending(step)=>{progress=step;},semio_framework_value::retained_clone::RetainedCloneSourceTake::Ready(original,step)=>{self.pending_mutation=Some(original);progress=step;self.build_stage=1;}}},
            1=>{let source=self.mutation.as_mut().ok_or_else(||ValueError::invariant("mutation absent"))?;if source.terminal_is_empty(){self.build_stage=2;}else{progress=settled_step(source.close_step(&mut effect.nested(RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g})))?.progress();}},
            2=>{self.mutation=None;self.build_stage=3;},
            3=>{self.build_stage=4;},
            _=>unreachable!(),
        }
        self.record_progress(admit_retained_clone_progress(g,progress,"original forward source handoff").map_err(|error|error.with_retained_progress(progress))?)?;
        Ok(true)
    }

    fn seal_turn(&mut self, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
                let result = self.assembly.sealer.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its canonical sealer").with_retained_progress(self.ownership))?.advance(grant,receiving);
                self.last_work=self.assembly.sealer.as_ref().unwrap().last_turn_work();
                let receipt=self.assembly.sealer.as_ref().unwrap().ownership_progress();
                self.ownership=receipt;
                admit_retained_clone_progress(clone_grant,receipt,"retained clone preparation original canonical source").map_err(|error|error.with_retained_progress(receipt))?;
                self.charge_footprint(receipt,"retained clone original canonical source exceeded admitted footprint")?;
                let step = result?;
                let seal_checkpoint = match step {
                    ArtifactStoreOneItemPreparationStep::Progress(checkpoint,receipt) | ArtifactStoreOneItemPreparationStep::Prepared(checkpoint,receipt) => {admit_retained_clone_progress(clone_grant,receipt,"retained clone preparation original native seal")?;checkpoint},
                    ArtifactStoreOneItemPreparationStep::Blocked => {return Ok(ArtifactStoreOneItemPreparationStep::Blocked)},
                };
                self.checkpoint = self.merged_seal_checkpoint(seal_checkpoint)?;
                if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
                    self.phase=RetainedClonePreparationPhase::RecordForeignStepPresence;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership));
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
    }

    fn advance_local(&mut self, body:usize, grant: ArtifactStoreOneItemGrant, effect:&mut RetainedAllocationEffect<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        self.ownership=RetainedCloneProgress::default();
        if self.cancelled || self.closing || !grant.permits_one() {
            {return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};
        }
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
        match self.phase {
            RetainedClonePreparationPhase::Source => {
                let bytes=RetainedCloneSource::<P>::borrowed_constructor_capacity_bytes::<SnapshotRead<P>>();
                if (RetainedCloneBirthDemand{capacity_bytes:bytes,depth:1}).admit(clone_grant).is_err(){{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                if clone_grant.maximum_copy_bytes<RetainedCloneSource::<P>::borrowed_constructor_copy_bytes::<SnapshotRead<P>>(){{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                if self.source_admission.is_none() {
                    let base=self.pending_base.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original captured base").with_retained_progress(self.ownership))?;
                    match base.source_admission(){
                        Ok(admission)=>self.source_admission=Some(admission),
                        Err((error,base))=>{self.pending_base=Some(base);return Err((error).into());},
                    }
                }
                let admission=self.source_admission.as_mut().ok_or_else(||ValueError::invariant("staged source admission was just installed"))?;
                let progress=if admission.phase()=="ready" {
                    let turn=admission.take_output(&mut self.source,clone_grant);pay(effect,turn.work)?;let progress=turn.result?.progress();
                    if self.source.is_some() { self.source_admission=None; self.phase=RetainedClonePreparationPhase::MutationSource; }
                    progress
                } else {admission.advance(&mut effect.nested(clone_grant)).result?.progress() };
                self.record_progress(progress)?;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::MutationSource => {
                let demand=RetainedCloneSource::<M>::owned_constructor_demand::<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>();
                if demand.admit(clone_grant).is_err(){{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                if clone_grant.maximum_copy_bytes<RetainedCloneSource::<M>::constructor_copy_bytes(){{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                if self.mutation_admission.is_none() {
                    let mutation=self.pending_mutation.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original owned mutation").with_retained_progress(self.ownership))?;
                    let authority=match self.pending_mutation_authority.take(){Some(authority)=>authority,None=>SharedControlledRetirement::lease(Arc::clone(self.authority.as_ref().ok_or_else(||ValueError::invariant("retained original mutation publication authority"))?))};
                    self.mutation_admission=Some(RetainedCloneSource::<M>::admission_owned(mutation,authority));
                }
                let admission=self.mutation_admission.as_mut().ok_or_else(||ValueError::invariant("staged mutation admission was just installed"))?;
                let progress=if admission.phase()=="ready" {
                    let turn=admission.take_output(&mut self.mutation,clone_grant);pay(effect,turn.work)?;let progress=turn.result?.progress();
                    if self.mutation.is_some() { self.mutation_admission=None; self.phase=RetainedClonePreparationPhase::CloneCursor; }
                    progress
                } else {admission.advance(&mut effect.nested(clone_grant)).result?.progress() };
                self.record_progress(progress)?;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CloneCursor => {
                let demand=self.edit.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost native clone birth authority").with_retained_progress(self.ownership))?.snapshot_cursor_birth_demand();
                let progress=match demand.admit(clone_grant){Ok(progress)=>progress,Err(_)=>{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)}};
                self.clone_cursor=Some(P::retained_clone_cursor());self.record_progress(progress)?;

                self.phase=RetainedClonePreparationPhase::EditCursor;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::EditCursor => {
                let edit=self.edit.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its original editor authority").with_retained_progress(self.ownership))?;
                let demand=edit.begin_demand();
                if demand.admit(clone_grant).is_err(){{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                let(cursor,progress)=edit.begin(clone_grant)?;
                self.edit_cursor=Some(cursor);
                let progress=admit_retained_clone_progress(clone_grant,progress,"snapshot clone native edit constructor")?;
                if progress.retained_capacity_bytes!=demand.capacity_bytes{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot clone editor constructor disagrees with its original admitted birth demand").with_retained_progress(self.ownership)).into());}
                self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::Clone;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::Clone => {
                let maximum=effect.remaining_work_units()?;
                let turn = self
                    .clone_cursor
                    .as_mut()
                    .ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its active clone cursor").with_retained_progress(self.ownership))?
                    .advance(self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its source").with_retained_progress(self.ownership))?.borrow(), body, maximum, &mut effect.nested(clone_grant));
                let step=turn.result.map_err(|failure|failure.into_value_error())?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation snapshot clone")?;
                if progress == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) {
                    return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "retained-clone.step-grant-too-small: the snapshot clone cursor cannot fund its next contiguous owner within one turn grant").with_retained_progress(self.ownership)).into());
                }
                self.record_document_progress(progress)?;
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.phase=RetainedClonePreparationPhase::TakeClone;
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::TakeClone=>{
                let cursor=self.clone_cursor.as_mut().ok_or_else(||ValueError::invariant("completed snapshot clone cursor is absent"))?;
                let(mut candidate,mut slot):(Option<P>,Option<Result<RetainedCloneStep,ValueError>>)=(None,None);
                {let reply=OriginalEffectReply::borrow(&mut slot)?;cursor.take(&mut candidate,&mut effect.nested(clone_grant),reply)?;}
                let step=slot.take().ok_or_else(||ValueError::invariant("snapshot clone cursor take left its reply pending"))??;
                let Some(value)=candidate else{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);};
                self.copied=Some(value);self.record_document_progress(step.progress())?;let started=self.handoff_clone_cursor(effect)?;self.record_document_progress(started)?;self.phase=RetainedClonePreparationPhase::CloseClone;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            },
            RetainedClonePreparationPhase::CloseClone => {
                let handoff = self.clone_handoff.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its Store-owned clone cursor handoff").with_retained_progress(self.ownership))?;
                if handoff.terminal_is_empty() {
                    self.clone_handoff = None;
                    self.record_document_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    self.phase = RetainedClonePreparationPhase::Edit;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership));
                }
                let step=settled_step(handoff.close_step(&mut effect.nested(clone_grant)))?;
                let step = admit_retained_clone_close(clone_grant, step, handoff.terminal_is_empty(), "retained clone preparation clone close")?;
                self.record_document_progress(step.progress())?;
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership) })
            }
            RetainedClonePreparationPhase::Edit => {
                let result = self.edit_cursor.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its admitted editor cursor").with_retained_progress(self.ownership))?.advance(
                    self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its edit source").with_retained_progress(self.ownership))?.borrow(),
                    self.copied.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its exclusive post snapshot").with_retained_progress(self.ownership))?,
                    self.mutation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained clone preparation lost its forward mutation").with_retained_progress(self.ownership))?.borrow(),
                    body, &mut effect.nested(clone_grant),
                );
                let step=result?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation typed edit")?;
                self.record_progress(progress)?;
                if matches!(step,RetainedCloneEditStep::Complete(_)){self.phase=RetainedClonePreparationPhase::CaptureForeignStepPresence;}
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CaptureForeignStepPresence => {
                let copy=size_of::<Vec<M>>()+size_of::<bool>()+size_of::<RetainedClonePreparationPhase>();
                if clone_grant.maximum_copy_bytes<copy||clone_grant.maximum_depth==0{{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                let cursor=self.edit_cursor.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit decision cursor is absent").with_retained_progress(self.ownership))?;
                self.foreign_step_presence=Some(cursor.foreign_step_presence());self.inverse=Some(cursor.take_inverse().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit completed without inverse").with_retained_progress(self.ownership))?);let _=cursor.begin_close();self.phase=RetainedClonePreparationPhase::CloseEdit;
                self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::CloseEdit => {
                let step=self.edit_cursor.as_mut().unwrap().close_step(&mut effect.nested(clone_grant))?;
                let step = admit_retained_clone_close(clone_grant, step, self.edit_cursor.as_ref().is_none_or(|cursor|cursor.terminal_is_empty()), "retained clone preparation edit close")?;
                self.record_progress(step.progress())?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = RetainedClonePreparationPhase::Build; }
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership) })
            }
            RetainedClonePreparationPhase::Build => {
                if self.build_sealer(grant,effect)? {
                    Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,self.ownership))
                } else {
                    Ok(ArtifactStoreOneItemPreparationStep::Blocked)
                }
            }
            RetainedClonePreparationPhase::Seal => Err(ValueError::invariant("native seal turns run under their own original funding")),
            RetainedClonePreparationPhase::RecordForeignStepPresence => {
                let copy=size_of::<bool>()+size_of::<RetainedClonePreparationPhase>();if clone_grant.maximum_copy_bytes<copy||clone_grant.maximum_depth==0{{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};}
                let presence=self.foreign_step_presence.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained edit foreign-step decision is absent").with_retained_progress(self.ownership))?;
                if !self.assembly.sealer.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original sealer is absent before declaration").with_retained_progress(self.ownership))?.record_foreign_step_presence(presence){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared owner is absent before declaration").with_retained_progress(self.ownership)).into());}
                self.phase=RetainedClonePreparationPhase::Prepared;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,self.ownership))
            }
            RetainedClonePreparationPhase::Prepared => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,Default::default())),
        }
    }

}

/// 🚦️ A pending child keeps its resident effect and reports no progress; a settled failure becomes the primary refusal.
fn settled_step(poll: OriginalRetirementPoll) -> Result<RetainedCloneStep, ValueError> {
    match poll {
        semio_framework_value::RetainedWorkPoll::Pending { .. } => Ok(RetainedCloneStep::Progress(Default::default())),
        semio_framework_value::RetainedWorkPoll::Complete(turn) => turn.result.map_err(|failure| failure.into_value_error()),
    }
}

impl<P, M, E> ArtifactStoreOneItemPreparation<P, M> for RetainedClonePreparation<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, semio_framework_value::OriginalReceivingPrimary> {
        self.last_work=0;self.ownership=Default::default();
        if self.cancelled||self.closing||!grant.permits_one(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        let before=receiving.actual_work();
        let result=if self.phase==RetainedClonePreparationPhase::Seal{self.seal_turn(grant,receiving)}else{
            let demand=if self.phase==RetainedClonePreparationPhase::Edit{self.edit_cursor.as_ref().ok_or_else(||ValueError::invariant("original editor cursor is absent"))?.next_work_demand(self.source.as_ref().ok_or_else(||ValueError::invariant("original edit source is absent"))?.borrow(),self.copied.as_ref().ok_or_else(||ValueError::invariant("original post is absent"))?,self.mutation.as_ref().ok_or_else(||ValueError::invariant("original mutation source is absent"))?.borrow(),body)?}else{1};
            let local=RetainedCloneGrant{maximum_items:1,..grant.retained_grant()};
            match receiving.with_effect(local,demand,|_,effect|{let result=self.advance_local(body,grant,effect);(result,effect.performed_work())})?{
                Some((result,work))=>{self.last_work=u64::try_from(work).unwrap_or(u64::MAX);result},
                None=>return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            }
        };
        let observed=u64::try_from(receiving.actual_work().saturating_sub(before)).unwrap_or(u64::MAX);let coordinated=result.as_ref().is_ok_and(|step|step.ownership_progress()!=Default::default());
        let reported=self.last_work;
        if self.phase!=RetainedClonePreparationPhase::Seal{self.last_work=reported.max(observed).max(u64::from(coordinated));}
        self.last_work=semio_os_store_one_item::original_one_item_turn_work(reported,observed,coordinated)?;
        match result {
            Ok(step)=>Ok(step),
            Err(error)=>{self.cancelled=true;if error.retained_progress()!=Default::default(){self.ownership=error.retained_progress();}Err(error.with_retained_progress(self.ownership).into())}
        }
    }
    fn last_turn_work(&self)->u64{self.last_work}
    fn has_pending_receiving_recovery(&self)->bool{false}
    fn pending_receiving_demand(&self)->Option<semio_framework_value::RetirementDemand>{None}
    fn pending_receiving_recovery(&mut self)->Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>>{None}
    fn finish_receiving_recovery(&mut self,_:&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot clone preparation opens every native effect atomically and never retains a pending receiving frame to recover").into()),work:0}}

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<P, M>> {
        if self.phase!=RetainedClonePreparationPhase::Prepared{return None}
        self.assembly.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, M>> {
        if self.phase!=RetainedClonePreparationPhase::Prepared{return None}
        self.assembly.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(cursor)=self.edit_cursor.as_mut(){cursor.cancel();}
        if let Some(sealer) = self.assembly.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(cursor)=self.edit_cursor.as_mut(){let _=cursor.begin_close();}
        if let Some(sealer) = self.assembly.sealer.as_mut() {
            sealer.begin_close();
        }
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.ownership=Default::default();
        let result = self.close_ladder(effect);
        RetirementPollExit::settle(effect, result)
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
            && self.source_admission.is_none()
            && self.mutation_admission.is_none()
            && self.pending_base.is_none()
            && self.pending_mutation.is_none()
            && self.pending_mutation_authority.is_none()
            && self.edit.is_none()
            && self.pending_base.is_none()
            && self.pending_base_close.is_none()
            && self.pending_mutation.is_none()
            && self.pending_mutation_close.is_none()
            && self.pending_mutation_authority.is_none()
            && self.edit.is_none()
            && self.source.is_none()
            && self.copied.is_none()
            && self.inverse.is_none()
            && self.assembly.terminal_is_empty()
            && self.mutation.is_none()
           
            && self.authority.is_none()
            && self.copied_close.is_none()
            && self.inverse_close.is_none()
            && self.authority_close.is_none()
            && self.factory_close.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
    }
}

impl<P, M, E> RetainedClonePreparation<P, M, E> where P: RetainedClone, M: ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static, E: RetainedCloneEdit<P, M> {
    fn close_ladder(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_close_depth_demand()? {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "snapshot clone close exceeds granted depth").into());
        }
        if let Some(handoff) = self.clone_handoff.as_mut() {
            let step=RetirementPollExit::resume(handoff.close_step(&mut effect.nested(grant)))?;
            if handoff.terminal_is_empty() { self.clone_handoff = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(cursor) = self.clone_cursor.as_mut() {
            if !cursor.close_started() { let turn=cursor.begin_close_paid(&mut effect.nested(grant)); return Ok(RetainedCloneStep::Progress(turn.result?)); }
            let step=RetirementPollExit::resume(cursor.close_step(&mut effect.nested(grant)))?;
            if cursor.terminal_is_empty() { self.clone_cursor = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner)=self.edit_cursor.as_mut(){let step=owner.close_step(&mut effect.nested(grant))?;if owner.terminal_is_empty(){self.edit_cursor=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if !self.assembly.terminal_is_empty(){return self.assembly.close_step(&mut effect.nested(grant));}
        if let Some(owner) = self.inverse_close.as_mut() {
            let step=RetirementPollExit::resume(owner.step(&mut effect.nested(grant)))?;
            if owner.terminal_is_empty() { self.inverse_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.inverse.is_some() {
            let bytes = size_of::<Vec<M>>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let value = self.inverse.take().unwrap();
            match ControlledRetirement::new(value) {
                Ok(owner) => self.inverse_close = Some(owner),
                Err((error, value)) => { self.inverse = Some(value); return Err((error).into()); }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        if let Some(owner) = self.copied_close.as_mut() {
            let step=RetirementPollExit::resume(owner.step(&mut effect.nested(grant)))?;
            if owner.terminal_is_empty() { self.copied_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.copied.is_some() {
            let bytes = size_of::<P>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let value = self.copied.take().unwrap();
            match ControlledRetirement::new(value) {
                Ok(owner) => self.copied_close = Some(owner),
                Err((error, value)) => { self.copied = Some(value); return Err((error).into()); }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        if let Some(admission)=self.mutation_admission.as_mut(){
            if admission.phase()=="ready" {
                let turn=admission.take_output(&mut self.mutation,grant);semio_os_store_one_item_publication::native_source::pay(effect,turn.work)?;let step=turn.result?;
                if admission.terminal_is_empty() { self.mutation_admission = None; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            let mut inputs=None;
            let turn=admission.return_inputs(&mut inputs,&mut effect.nested(grant));let step=turn.result?;
            if let Some((mutation,authority))=inputs { self.pending_mutation=Some(mutation); self.pending_mutation_authority=Some(authority); }
            if admission.terminal_is_empty() { self.mutation_admission = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(step)=pending::close(&mut self.pending_mutation,&mut self.pending_mutation_close,&mut effect.nested(grant))?{return Ok(step);}
        if let Some(owner)=self.pending_mutation_authority.as_mut(){let step=RetirementPollExit::resume(owner.step(&mut effect.nested(grant)))?;if owner.terminal_is_empty(){self.pending_mutation_authority=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(source) = self.mutation.as_mut() {
            let step=RetirementPollExit::resume(source.close_step(&mut effect.nested(grant)))?;
            if source.terminal_is_empty() { self.mutation = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(admission)=self.source_admission.as_mut(){
            if admission.phase()=="ready" {
                let turn=admission.take_output(&mut self.source,grant);semio_os_store_one_item_publication::native_source::pay(effect,turn.work)?;
                let step=turn.result?;
                if admission.terminal_is_empty() { self.source_admission = None; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            let mut inputs=None;
            let turn=admission.return_inputs(&mut inputs,&mut effect.nested(grant));let step=turn.result?;
            if let Some((base,()))=inputs { self.pending_base=Some(base); }
            if admission.terminal_is_empty() { self.source_admission = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(step)=pending::close(&mut self.pending_base,&mut self.pending_base_close,&mut effect.nested(grant))?{return Ok(step);}
        if let Some(source) = self.source.as_mut() {
            let step=RetirementPollExit::resume(source.close_step(&mut effect.nested(grant)))?;
            if source.terminal_is_empty() { self.source = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.authority_close.as_mut() {
            let step=RetirementPollExit::resume(owner.step(&mut effect.nested(grant)))?;
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
            let step=RetirementPollExit::resume(owner.step(&mut effect.nested(grant)))?;
            if owner.terminal_is_empty() { self.factory_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.edit.is_some() || self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            let bytes = size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let owner: Arc<dyn semio_framework_value::FactoryRetirement> = if let Some(owner)=self.edit.take(){owner}else if let Some(owner) = self.mutation_retirement.take() { owner } else { self.snapshot_retirement.take().unwrap() };
            self.factory_close = Some(FactoryAuthority::new(owner));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
