//! 🧩 Bounded retained-clone publication preparation.

use crate::os_spr::{ActorId, Edit, MutationId, MutationMeta, UndoPolicy};
use crate::os_store::{
    ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactCanonicalJson, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority,
    ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemSealer, ErasedSnapshotRetirement,
    HistoryLane, SnapshotRead, SnapshotRetirementFactory,
};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_close};
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, shared::SharedControlledRetirement}, FactoryAuthority};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

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
    fn advance(&mut self, base: RetainedCloneRef<'_, P>, post: &mut P, mutation: RetainedCloneRef<'_, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, String>;
    fn take_inverse(&mut self) -> Option<Vec<M>>;
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
    M: ArtifactCanonicalJson + RetireOwned + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.edit.preflight(mutation, lane)
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        if !P::controlled_retirement_supported() || !M::controlled_retirement_supported() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "snapshot clone originals require controlled typed retirement"));
        }
        Ok(RetainedCloneBirthDemand{capacity_bytes:size_of::<RetainedClonePreparation<P,M,E>>(),depth:1})
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, M>)> {
        let footprint = match self.edit.preflight(&request.mutation, request.lane) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            _ => return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "snapshot clone preparation footprint refused"), request)),
        };
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let ArtifactStoreOneItemPreparationRequest { operation: _, generation: _, base_revision: _, lane: _, authority, base, mutation } = request;
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
            mutation: None,
            authority: Some(authority),
            sealer: None,
            mutation_retirement: Some(Arc::clone(&self.mutation_retirement)),
            snapshot_retirement: Some(Arc::clone(&self.snapshot_retirement)),
            copied_close: None,
            inverse_close: None,
            authority_close: None,
            factory_close: None,
            footprint,
            retained_capacity_bytes: progress.retained_capacity_bytes,
            maximum_depth: self.maximum_depth,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
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
    CloseEdit,
    Build,
    Seal,
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
    source: Option<RetainedCloneSource<P>>,
    clone_cursor: Option<P::Cursor>,
    clone_handoff: Option<RetainedCloneCursorHandoff<P>>,
    copied: Option<P>,
    edit_cursor: Option<E::Cursor>,
    inverse: Option<Vec<M>>,
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
    seal_base: ArtifactStoreOneItemCheckpoint,
    phase: RetainedClonePreparationPhase,
    cancelled: bool,
    closing: bool,
}

impl<P: RetainedClone, M: RetireOwned + Sync, E: RetainedCloneEdit<P, M>> RetainedClonePreparation<P, M, E> {
    fn handoff_clone_cursor(&mut self) -> Result<(), semio_framework_value::ValueError> {
        if self.clone_handoff.is_some() {
            return Ok(());
        }
        let cursor = self.clone_cursor.take().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation lost its cursor before close handoff"))?;
        self.clone_handoff = Some(RetainedCloneCursorHandoff::new(cursor));
        Ok(())
    }

    fn record_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), String> {
        let bytes = progress.copied_bytes.checked_add(progress.retained_capacity_bytes).and_then(|bytes| bytes.checked_add(progress.released_bytes)).ok_or("retained clone preparation byte progress overflow")?;
        self.retained_capacity_bytes = self.retained_capacity_bytes.checked_add(progress.retained_capacity_bytes).ok_or("retained clone preparation retained capacity overflow")?;
        if self.retained_capacity_bytes > self.footprint.retained_bytes {
            return Err("retained clone preparation exceeded its admitted retained capacity".into());
        }
        self.checkpoint.cursor = self.checkpoint.cursor.checked_add(1).ok_or("retained clone preparation cursor overflow")?;
        self.checkpoint.completed_items =
            self.checkpoint.completed_items.checked_add(u32::try_from(progress.copied_items).map_err(|_| "retained clone preparation item progress overflow")?).ok_or("retained clone preparation item progress overflow")?;
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.checked_add(u64::try_from(bytes).map_err(|_| "retained clone preparation byte progress overflow")?).ok_or("retained clone preparation byte progress overflow")?;
        Ok(())
    }

    fn merged_seal_checkpoint(&self, seal: ArtifactStoreOneItemCheckpoint) -> Result<ArtifactStoreOneItemCheckpoint, String> {
        Ok(ArtifactStoreOneItemCheckpoint {
            cursor: self.seal_base.cursor.checked_add(seal.cursor).ok_or("retained clone seal cursor overflow")?,
            completed_items: self.seal_base.completed_items.checked_add(seal.completed_items).ok_or("retained clone seal item progress overflow")?,
            completed_bytes: self.seal_base.completed_bytes.checked_add(seal.completed_bytes).ok_or("retained clone seal byte progress overflow")?,
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

    fn build_sealer(&mut self, _grant: ArtifactStoreOneItemGrant) -> Result<bool, String> {
        Err("retained-clone.publication-assembly-unsupported: native metadata and post-root require granted publication construction".into())
    }


}

impl<P, M, E> ArtifactStoreOneItemPreparation<P, M> for RetainedClonePreparation<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJson + RetireOwned + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || self.closing || !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
        match self.phase {
            RetainedClonePreparationPhase::Source => {
                let bytes=RetainedCloneSource::<P>::constructor_capacity_bytes::<SnapshotRead<P>>();
                if (RetainedCloneBirthDemand{capacity_bytes:bytes,depth:1}).admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let base=self.pending_base.take().ok_or("retained clone preparation lost its original captured base")?;
                match base.admit_retained_clone_source(clone_grant){
                    Ok((source,progress))=>{self.source=Some(source);self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::MutationSource;},
                    Err((error,base))=>{self.pending_base=Some(base);return Err(error.into_message());},
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::MutationSource => {
                let demand=RetainedCloneSource::<M>::owned_constructor_demand::<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>();
                if demand.admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let mutation=self.pending_mutation.take().ok_or("retained clone preparation lost its original owned mutation")?;
                let authority=self.pending_mutation_authority.take().unwrap_or_else(||SharedControlledRetirement::lease(Arc::clone(self.authority.as_ref().expect("retained original mutation publication authority"))));
                match RetainedCloneSource::admit_owned(mutation,authority,clone_grant){
                    Ok((source,progress))=>{self.mutation=Some(source);self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::CloneCursor;},
                    Err((error,mutation,authority))=>{self.pending_mutation=Some(mutation);self.pending_mutation_authority=Some(authority);return Err(error.into_message());},
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::CloneCursor => {
                let demand=self.edit.as_ref().ok_or("retained clone preparation lost native clone birth authority")?.snapshot_cursor_birth_demand();
                let progress=match demand.admit(clone_grant){Ok(progress)=>progress,Err(_)=>return Ok(ArtifactStoreOneItemPreparationStep::Blocked)};
                self.clone_cursor=Some(P::retained_clone_cursor());self.record_progress(progress)?;

                self.phase=RetainedClonePreparationPhase::EditCursor;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::EditCursor => {
                let edit=self.edit.as_ref().ok_or("retained clone preparation lost its original editor authority")?;
                let demand=edit.begin_demand();
                if demand.admit(clone_grant).is_err(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let(cursor,progress)=edit.begin(clone_grant).map_err(ValueError::into_message)?;
                self.edit_cursor=Some(cursor);
                let progress=admit_retained_clone_progress(clone_grant,progress,"snapshot clone native edit constructor").map_err(ValueError::into_message)?;
                if progress.retained_capacity_bytes!=demand.capacity_bytes{return Err("snapshot clone editor constructor disagrees with its original admitted birth demand".into());}
                self.record_progress(progress)?;self.phase=RetainedClonePreparationPhase::Clone;
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::Clone => {
                let step = self
                    .clone_cursor
                    .as_mut()
                    .ok_or("retained clone preparation lost its active clone cursor")?
                    .advance(self.source.as_ref().ok_or("retained clone preparation lost its source")?.borrow(), clone_grant)
                    .map_err(semio_framework_value::ValueError::into_message)?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation snapshot clone").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(progress)?;
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.copied = Some(self.clone_cursor.as_mut().and_then(RetainedCloneCursor::take).ok_or("retained clone cursor completed without its owner")?);
                    self.handoff_clone_cursor().map_err(semio_framework_value::ValueError::into_message)?;
                    self.phase = RetainedClonePreparationPhase::CloseClone;
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::CloseClone => {
                let handoff = self.clone_handoff.as_mut().ok_or("retained clone preparation lost its Store-owned clone cursor handoff")?;
                if handoff.terminal_is_empty() {
                    self.clone_handoff = None;
                    self.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    self.phase = RetainedClonePreparationPhase::Edit;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
                }
                let step = handoff.close_step(clone_grant).map_err(semio_framework_value::ValueError::into_message)?;
                let step = admit_retained_clone_close(clone_grant, step, handoff.terminal_is_empty(), "retained clone preparation clone close").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(step.progress())?;
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::Edit => {
                let step = self.edit_cursor.as_mut().ok_or("retained clone preparation lost its admitted editor cursor")?.advance(
                    self.source.as_ref().ok_or("retained clone preparation lost its edit source")?.borrow(),
                    self.copied.as_mut().ok_or("retained clone preparation lost its exclusive post snapshot")?,
                    self.mutation.as_ref().ok_or("retained clone preparation lost its forward mutation")?.borrow(),
                    clone_grant,
                )?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation typed edit").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(progress)?;
                if matches!(step, RetainedCloneEditStep::Complete(_)) {
                    self.inverse = Some(self.edit_cursor.as_mut().unwrap().take_inverse().ok_or("retained clone edit completed without inverse mutations")?);
                    if let Some(cursor)=self.edit_cursor.as_mut(){let _=cursor.begin_close();}
                    self.phase = RetainedClonePreparationPhase::CloseEdit;
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::CloseEdit => {
                let step = self.edit_cursor.as_mut().unwrap().close_step(clone_grant).map_err(semio_framework_value::ValueError::into_message)?;
                let step = admit_retained_clone_close(clone_grant, step, self.edit_cursor.as_ref().is_none_or(|cursor|cursor.terminal_is_empty()), "retained clone preparation edit close").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(step.progress())?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = RetainedClonePreparationPhase::Build; }
                Ok(if step.progress() == RetainedCloneProgress::default() && !matches!(step, RetainedCloneStep::Complete(_)) { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::Build => {
                if self.build_sealer(grant)? {
                    Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
                } else {
                    Err(format!("retained-clone.step-grant-too-small: fixed publication assembly requires more than the admitted {}-byte per-turn allocation grant", grant.maximum_capacity_bytes))
                }
            }
            RetainedClonePreparationPhase::Seal => {
                let step = self.sealer.as_mut().ok_or("retained clone preparation lost its canonical sealer")?.advance(grant)?;
                let seal_checkpoint = match step {
                    ArtifactStoreOneItemPreparationStep::Progress(checkpoint) | ArtifactStoreOneItemPreparationStep::Prepared(checkpoint) => checkpoint,
                    ArtifactStoreOneItemPreparationStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                };
                self.checkpoint = self.merged_seal_checkpoint(seal_checkpoint)?;
                if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                    self.phase = RetainedClonePreparationPhase::Prepared;
                    return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
                }
                Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            RetainedClonePreparationPhase::Prepared => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
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
