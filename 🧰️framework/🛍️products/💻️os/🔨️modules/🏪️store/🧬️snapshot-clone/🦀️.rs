//! 🧩 Bounded retained-clone publication preparation.

use crate::os_spr::{ActorId, Edit, MutationId, MutationMeta, UndoPolicy};
use crate::os_store::{
    ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactCanonicalJson, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority,
    ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemSealer, ErasedSnapshotRetirement,
    HistoryLane, SnapshotRetirementFactory, SnapshotRetirementStep,
};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

#[path = "🚚️handoff/🦀️.rs"]
mod handoff;
use handoff::RetainedCloneCursorHandoff;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RetainedCloneEditStep {
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
pub(crate) trait RetainedCloneEditCursor<P: RetainedClone, M>: Send {
    fn advance(&mut self, base: RetainedCloneRef<'_, P>, post: &mut P, mutation: &M, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, String>;
    fn take_inverse(&mut self) -> Option<Vec<M>>;
    fn cancel(&mut self);
    fn begin_close(&mut self) -> bool;
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪪 Admits and creates the domain cursor while the Store retains publication authority.
pub(crate) trait RetainedCloneEdit<P: RetainedClone, M>: Send + Sync + 'static {
    type Cursor: RetainedCloneEditCursor<P, M>;
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin(&self) -> Self::Cursor;
}

pub(crate) struct RetainedClonePreparationFactory<P, M, E> {
    edit: Arc<E>,
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
    maximum_depth: usize,
    marker: PhantomData<fn() -> (P, M)>,
}

impl<P, M, E> RetainedClonePreparationFactory<P, M, E> {
    pub(crate) fn new(edit: Arc<E>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>, maximum_depth: usize) -> Result<Self, String> {
        if maximum_depth == 0 {
            return Err("retained clone preparation requires a nonzero structural depth envelope".into());
        }
        Ok(Self { edit, mutation_retirement, snapshot_retirement, maximum_depth, marker: PhantomData })
    }
}

impl<P, M, E> ArtifactStoreOneItemPreparationFactory<P, M> for RetainedClonePreparationFactory<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJson + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.edit.preflight(mutation, description, lane)
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, M>>, ArtifactStoreOneItemPreparationRequest<P, M>> {
        let footprint = match self.edit.preflight(&request.mutation, request.description.as_deref(), request.lane) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            _ => return Err(request),
        };
        let ArtifactStoreOneItemPreparationRequest { operation: _, generation: _, base_revision: _, lane: _, authority, description, base, mutation } = request;
        Ok(Box::new(RetainedClonePreparation::<P, M, E> {
            source: Some(RetainedCloneSource::from_authority(Arc::clone(base.owner.as_ref().expect("live snapshot read owner is present")), base)),
            clone_cursor: Some(P::retained_clone_cursor()),
            clone_handoff: None,
            copied: None,
            edit_cursor: self.edit.begin(),
            inverse: None,
            mutation: Some(mutation),
            description,
            authority: Some(authority),
            sealer: None,
            mutation_retirement: Some(Arc::clone(&self.mutation_retirement)),
            snapshot_retirement: Some(Arc::clone(&self.snapshot_retirement)),
            active_retirement: None,
            footprint,
            retained_capacity_bytes: 0,
            maximum_depth: self.maximum_depth,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            seal_base: ArtifactStoreOneItemCheckpoint::default(),
            phase: RetainedClonePreparationPhase::Clone,
            cancelled: false,
            closing: false,
        }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedClonePreparationPhase {
    Clone,
    CloseClone,
    Edit,
    CloseEdit,
    Build,
    Seal,
    Prepared,
}

struct RetainedClonePreparation<P: RetainedClone, M, E: RetainedCloneEdit<P, M>> {
    source: Option<RetainedCloneSource<P>>,
    clone_cursor: Option<P::Cursor>,
    clone_handoff: Option<RetainedCloneCursorHandoff<P>>,
    copied: Option<P>,
    edit_cursor: E::Cursor,
    inverse: Option<Vec<M>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    sealer: Option<ArtifactStoreOneItemSealer<P, M>>,
    mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    footprint: ArtifactStoreOneItemFootprint,
    retained_capacity_bytes: usize,
    maximum_depth: usize,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    seal_base: ArtifactStoreOneItemCheckpoint,
    phase: RetainedClonePreparationPhase,
    cancelled: bool,
    closing: bool,
}

impl<P: RetainedClone, M, E: RetainedCloneEdit<P, M>> RetainedClonePreparation<P, M, E> {
    fn handoff_clone_cursor(&mut self) -> Result<(), semio_framework_value::ValueError> {
        if self.clone_handoff.is_some() {
            return Ok(());
        }
        let cursor = self.clone_cursor.take().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation lost its cursor before close handoff"))?;
        self.clone_handoff = Some(RetainedCloneCursorHandoff::new(cursor));
        Ok(())
    }

    fn clone_grant(&self, grant: ArtifactStoreOneItemGrant) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_bytes, maximum_capacity_bytes: grant.maximum_bytes, maximum_depth: self.maximum_depth }
    }

    fn record_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), String> {
        let bytes = progress.copied_bytes.checked_add(progress.retained_capacity_bytes).ok_or("retained clone preparation byte progress overflow")?;
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

    fn record_retirement(&mut self, step: SnapshotRetirementStep, maximum_items: usize, maximum_bytes: usize, scope: &str) -> Result<(), String> {
        let step = admit_retained_clone_retirement(step, maximum_items, maximum_bytes, scope).map_err(semio_framework_value::ValueError::into_message)?;
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            self.record_progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })?;
        }
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

    fn build_capacity(&self) -> Result<usize, String> {
        let authority = self.authority.as_ref().ok_or("retained clone preparation lost its Store authority")?;
        let actor_bytes = authority.actor().len();
        let group_bytes = authority.group_id().map_or(0, str::len);
        let actor_capacity = actor_bytes.checked_mul(3).ok_or("retained clone preparation actor capacity overflow")?;
        size_of::<M>()
            .checked_add(size_of::<MutationMeta>())
            .and_then(|bytes| bytes.checked_add(size_of::<P>()))
            .and_then(|bytes| bytes.checked_add(ARTIFACT_STORE_ONE_ITEM_ID_BYTES * 6))
            .and_then(|bytes| bytes.checked_add(actor_capacity))
            .and_then(|bytes| bytes.checked_add(group_bytes))
            .and_then(|bytes| bytes.checked_add(authority.line_id().map_or(0, str::len)))
            .ok_or_else(|| "retained clone preparation build capacity overflow".into())
    }

    fn build_sealer(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<bool, String> {
        let required = self.build_capacity()?;
        if grant.maximum_items == 0 || grant.maximum_bytes < required {
            return Ok(false);
        }
        let authority = self.authority.take().ok_or("retained clone preparation lost its Store authority")?;
        let id = authority.edit_id();
        let mutation_id = authority.stamped_edit_id().map_or_else(|| MutationId(format!("{id}#0")), |identity| MutationId(identity.to_string()));
        let edit = Edit {
            line: authority.line_id().map(str::to_owned),
            id,
            actor: Some(authority.actor().to_string()),
            forwards: vec![self.mutation.take().ok_or("retained clone preparation lost its forward mutation")?],
            inverse: self.inverse.take().ok_or("retained clone preparation lost its inverse mutations")?,
            mutation_meta: vec![MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: Vec::new(),
                base_version: u64::try_from(authority.base_applied_edit_count()).map_err(|_| "retained clone preparation base version overflow")?,
                author_id: Some(ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: authority.group_id().map(str::to_string),
                origin: Default::default(),
                transaction: None,
            }],
            description: self.description.take(),
            verb: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let post = Arc::new(self.copied.take().ok_or("retained clone preparation lost its post snapshot")?);
        self.sealer = Some(authority.begin_one_item_seal(
            edit,
            post,
            Arc::clone(self.mutation_retirement.as_ref().ok_or("retained clone preparation lost its mutation retirement authority")?),
            Arc::clone(self.snapshot_retirement.as_ref().ok_or("retained clone preparation lost its snapshot retirement authority")?),
        ));
        self.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: required })?;
        self.seal_base = self.checkpoint;
        self.phase = RetainedClonePreparationPhase::Seal;
        Ok(true)
    }

    fn close_active_retirement(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<Option<SnapshotRetirementStep>, semio_framework_value::ValueError> {
        let Some(active) = self.active_retirement.as_mut() else { return Ok(None) };
        let maximum_items = grant.maximum_items.min(1);
        let step = admit_retained_clone_retirement(active.close_step(maximum_items, grant.maximum_bytes)?, maximum_items, grant.maximum_bytes, "retained clone preparation active retirement")?;
        if step == SnapshotRetirementStep::Complete {
            if !active.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation retirement completed with a live owner"));
            }
            self.active_retirement = None;
            return Ok(Some(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        Ok(Some(step))
    }
}

impl<P, M, E> ArtifactStoreOneItemPreparation<P, M> for RetainedClonePreparation<P, M, E>
where
    P: RetainedClone,
    M: ArtifactCanonicalJson + Send + Sync + 'static,
    E: RetainedCloneEdit<P, M>,
{
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || self.closing || !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let clone_grant = self.clone_grant(grant);
        match self.phase {
            RetainedClonePreparationPhase::Clone => {
                let step = self
                    .clone_cursor
                    .as_mut()
                    .ok_or("retained clone preparation lost its active clone cursor")?
                    .advance(self.source.as_ref().ok_or("retained clone preparation lost its source")?.borrow(), clone_grant)
                    .map_err(semio_framework_value::ValueError::into_message)?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation snapshot clone").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(progress)?;
                if matches!(step, RetainedCloneStep::Progress(value) if value == RetainedCloneProgress::default()) {
                    return Err(format!("retained-clone.step-grant-too-small: snapshot clone requires more than the admitted {}-byte per-turn allocation or copy grant", grant.maximum_bytes));
                }
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.copied = Some(self.clone_cursor.as_mut().and_then(RetainedCloneCursor::take).ok_or("retained clone cursor completed without its owner")?);
                    self.handoff_clone_cursor().map_err(semio_framework_value::ValueError::into_message)?;
                    self.phase = RetainedClonePreparationPhase::CloseClone;
                }
                Ok(if progress == RetainedCloneProgress::default() { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::CloseClone => {
                let maximum_items = grant.maximum_items.min(1);
                let handoff = self.clone_handoff.as_mut().ok_or("retained clone preparation lost its Store-owned clone cursor handoff")?;
                let step =
                    admit_retained_clone_retirement(handoff.close_step(maximum_items, grant.maximum_bytes).map_err(semio_framework_value::ValueError::into_message)?, maximum_items, grant.maximum_bytes, "retained clone preparation clone close")
                        .map_err(semio_framework_value::ValueError::into_message)?;
                let terminal = handoff.terminal_is_empty();
                self.record_retirement(step, maximum_items, grant.maximum_bytes, "retained clone preparation clone close")?;
                if step == SnapshotRetirementStep::Complete {
                    if !terminal {
                        return Err("retained clone cursor completed close with a live owner".into());
                    }
                    self.clone_handoff = None;
                    self.phase = RetainedClonePreparationPhase::Edit;
                }
                Ok(if step == SnapshotRetirementStep::Blocked { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::Edit => {
                let step = self.edit_cursor.advance(
                    self.source.as_ref().ok_or("retained clone preparation lost its edit source")?.borrow(),
                    self.copied.as_mut().ok_or("retained clone preparation lost its exclusive post snapshot")?,
                    self.mutation.as_ref().ok_or("retained clone preparation lost its forward mutation")?,
                    clone_grant,
                )?;
                let progress = admit_retained_clone_progress(clone_grant, step.progress(), "retained clone preparation typed edit").map_err(semio_framework_value::ValueError::into_message)?;
                self.record_progress(progress)?;
                if matches!(step, RetainedCloneEditStep::Progress(value) if value == RetainedCloneProgress::default()) {
                    return Err(format!("retained-clone.step-grant-too-small: typed edit requires more than the admitted {}-byte per-turn allocation or copy grant", grant.maximum_bytes));
                }
                if matches!(step, RetainedCloneEditStep::Complete(_)) {
                    self.inverse = Some(self.edit_cursor.take_inverse().ok_or("retained clone edit completed without inverse mutations")?);
                    let _ = self.edit_cursor.begin_close();
                    self.phase = RetainedClonePreparationPhase::CloseEdit;
                }
                Ok(if progress == RetainedCloneProgress::default() { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::CloseEdit => {
                let maximum_items = grant.maximum_items.min(1);
                let step = admit_retained_clone_retirement(
                    self.edit_cursor.close_step(maximum_items, grant.maximum_bytes).map_err(semio_framework_value::ValueError::into_message)?,
                    maximum_items,
                    grant.maximum_bytes,
                    "retained clone preparation edit close",
                )
                .map_err(semio_framework_value::ValueError::into_message)?;
                self.record_retirement(step, maximum_items, grant.maximum_bytes, "retained clone preparation edit close")?;
                if step == SnapshotRetirementStep::Complete {
                    if !self.edit_cursor.terminal_is_empty() {
                        return Err("retained clone edit completed close with a live owner".into());
                    }
                    self.phase = RetainedClonePreparationPhase::Build;
                }
                Ok(if step == SnapshotRetirementStep::Blocked { ArtifactStoreOneItemPreparationStep::Blocked } else { ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint) })
            }
            RetainedClonePreparationPhase::Build => {
                if self.build_sealer(grant)? {
                    Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
                } else {
                    Err(format!("retained-clone.step-grant-too-small: fixed publication assembly requires more than the admitted {}-byte per-turn allocation grant", grant.maximum_bytes))
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
        self.edit_cursor.cancel();
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if self.clone_handoff.is_none() {
            if let Some(cursor) = self.clone_cursor.take() {
                self.clone_handoff = Some(RetainedCloneCursorHandoff::new(cursor));
            }
        }
        let _ = self.edit_cursor.begin_close();
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(handoff) = self.clone_handoff.as_mut() {
            let maximum_items = grant.maximum_items.min(1);
            let step = admit_retained_clone_retirement(handoff.close_step(maximum_items, grant.maximum_bytes)?, maximum_items, grant.maximum_bytes, "retained clone preparation cursor close")?;
            if step == SnapshotRetirementStep::Complete {
                if !handoff.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation cursor completed close with a live owner"));
                }
                self.clone_handoff = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if !self.edit_cursor.terminal_is_empty() {
            let maximum_items = grant.maximum_items.min(1);
            let step = admit_retained_clone_retirement(self.edit_cursor.close_step(maximum_items, grant.maximum_bytes)?, maximum_items, grant.maximum_bytes, "retained clone preparation edit close")?;
            if step == SnapshotRetirementStep::Complete {
                if !self.edit_cursor.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation edit completed close with a live owner"));
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.close_step(grant)?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !sealer.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone canonical sealer completed with a live owner"));
            }
            self.sealer = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = self.close_active_retirement(grant)? {
            return Ok(step);
        }
        if let Some(inverse) = self.inverse.as_mut() {
            if let Some(mutation) = inverse.pop() {
                self.active_retirement = Some(
                    self.mutation_retirement
                        .as_ref()
                        .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation lost inverse retirement authority"))?
                        .retire_owned(mutation),
                );
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.inverse = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(copied) = self.copied.take() {
            self.active_retirement = Some(owned_retirement(copied));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.mutation.take() {
            self.active_retirement = Some(
                self.mutation_retirement
                    .as_ref()
                    .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained clone preparation lost mutation retirement authority"))?
                    .retire_owned(mutation),
            );
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(description) = self.description.take() {
            self.active_retirement = Some(owned_retirement(description));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            self.active_retirement = Some(authority.retire());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.source.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutation_retirement.take().is_some() || self.snapshot_retirement.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.clone_cursor.is_none()
            && self.clone_handoff.is_none()
            && self.edit_cursor.terminal_is_empty()
            && self.source.is_none()
            && self.copied.is_none()
            && self.inverse.is_none()
            && self.mutation.is_none()
            && self.description.is_none()
            && self.authority.is_none()
            && self.sealer.is_none()
            && self.active_retirement.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

use semio_framework_value::retirement::owned_retirement;
