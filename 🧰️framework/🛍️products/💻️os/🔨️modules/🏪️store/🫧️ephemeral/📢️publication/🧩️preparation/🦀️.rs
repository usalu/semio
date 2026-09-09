//! 🧩️ Shared ephemeral preparation lifecycle around domain-owned construction tasks.

use super::{ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead, ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory,
    ArtifactEphemeralOneItemPreparationRequest, ArtifactEphemeralOneItemPrepared, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint,
    ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ErasedSnapshotRetirement, ReturnedSnapshotReadRetirement, SnapshotRetirementStep};
use std::{mem::ManuallyDrop, sync::Arc};

pub enum ArtifactEphemeralPreparationTaskStep<P> {
    Progress(ArtifactStoreOneItemCheckpoint),
    Prepared { root: P, checkpoint: ArtifactStoreOneItemCheckpoint },
    Blocked,
}

/// 🛠️ Constructs one root within each grant; consumed mutations remain in the task or result.
pub trait ArtifactEphemeralPreparationTask<P, M>: Send {
    fn advance(&mut self, base: &P, mutation: &mut Option<M>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<P>, String>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏗️ Domain construction supplies its task; Store retains base, mutation and result ownership.
pub struct ArtifactEphemeralTaskPreparationFactory<P, M> {
    preflight: fn(&M) -> Result<ArtifactStoreOneItemFootprint, String>,
    create_task: fn(&P, &M) -> Result<Box<dyn ArtifactEphemeralPreparationTask<P, M>>, String>,
    state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}

impl<P, M> ArtifactEphemeralTaskPreparationFactory<P, M> {
    pub fn new(
        preflight: fn(&M) -> Result<ArtifactStoreOneItemFootprint, String>,
        create_task: fn(&P, &M) -> Result<Box<dyn ArtifactEphemeralPreparationTask<P, M>>, String>,
        state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    ) -> Self {
        Self { preflight, create_task, state_retirement, mutation_retirement }
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralOneItemPreparationFactory<P, M> for ArtifactEphemeralTaskPreparationFactory<P, M> {
    fn preflight(&self, mutation: &M) -> Result<ArtifactStoreOneItemFootprint, String> { (self.preflight)(mutation) }

    fn begin(&self, request: ArtifactEphemeralOneItemPreparationRequest<P, M>) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<P, M>>, ArtifactEphemeralOneItemPreparationRequest<P, M>> {
        if !self.preflight(&request.mutation).is_ok_and(ArtifactStoreOneItemFootprint::is_admissible) { return Err(request); }
        let task = match (self.create_task)(request.base.as_ref(), &request.mutation) { Ok(task) => task, Err(_) => return Err(request) };
        Ok(Box::new(ArtifactEphemeralTaskPreparation::new(request, task, self.state_retirement.clone(), self.mutation_retirement.clone())))
    }
}

pub(super) struct ArtifactEphemeralTaskPreparation<P, M> {
    base: ManuallyDrop<Option<ArtifactEphemeralBaseRead<P>>>,
    mutation: ManuallyDrop<Option<M>>,
    task: ManuallyDrop<Option<Box<dyn ArtifactEphemeralPreparationTask<P, M>>>>,
    prepared: ManuallyDrop<Option<ArtifactEphemeralOneItemPrepared<P>>>,
    retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    constructed: bool,
    cancelled: bool,
    closing: bool,
}

impl<P, M> ArtifactEphemeralTaskPreparation<P, M> {
    pub(super) fn new(
        request: ArtifactEphemeralOneItemPreparationRequest<P, M>, task: Box<dyn ArtifactEphemeralPreparationTask<P, M>>,
        state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    ) -> Self {
        Self { base: ManuallyDrop::new(Some(request.base)), mutation: ManuallyDrop::new(Some(request.mutation)), task: ManuallyDrop::new(Some(task)),
            prepared: ManuallyDrop::new(None), retirement: ManuallyDrop::new(None), state_retirement, mutation_retirement,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(), constructed: false, cancelled: false, closing: false }
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralOneItemPreparation<P, M> for ArtifactEphemeralTaskPreparation<P, M> {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || self.closing || grant.maximum_items == 0 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.constructed { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let task = self.task.as_mut().ok_or_else(|| "ephemeral preparation lost its construction task".to_string())?;
        let base = self.base.as_ref().ok_or_else(|| "ephemeral preparation lost its base read".to_string())?;
        let step = task.advance(base.as_ref(), &mut self.mutation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })?;
        let checkpoint = match &step {
            ArtifactEphemeralPreparationTaskStep::Progress(checkpoint) | ArtifactEphemeralPreparationTaskStep::Prepared { checkpoint, .. } => Some(*checkpoint),
            ArtifactEphemeralPreparationTaskStep::Blocked => None,
        };
        let invalid = checkpoint.is_some_and(|next| next.completed_items < self.checkpoint.completed_items || next.completed_items - self.checkpoint.completed_items > 1
            || next.completed_bytes < self.checkpoint.completed_bytes || next.completed_bytes - self.checkpoint.completed_bytes > grant.maximum_bytes as u64);
        let result = match step {
            ArtifactEphemeralPreparationTaskStep::Progress(checkpoint) => { self.checkpoint = checkpoint; ArtifactStoreOneItemPreparationStep::Progress(checkpoint) }
            ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint } => {
                *self.prepared = Some(ArtifactEphemeralOneItemPrepared { next_root: Arc::new(root) });
                self.constructed = true;
                self.checkpoint = checkpoint;
                ArtifactStoreOneItemPreparationStep::Prepared(checkpoint)
            }
            ArtifactEphemeralPreparationTaskStep::Blocked => ArtifactStoreOneItemPreparationStep::Blocked,
        };
        if invalid { return Err("ephemeral construction task exceeded its exact grant".into()); }
        Ok(result)
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<P>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<P>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) {
        if self.closing { return; }
        self.closing = true;
        if let Some(task) = self.task.as_mut() { task.begin_close(); }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if grant.maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if !self.closing { return Err("ephemeral preparation close was not started".into()); }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, grant.maximum_bytes)? {
                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("ephemeral preparation retirement completed with retained owners".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_bytes => Err("ephemeral preparation retirement exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(task) = self.task.as_mut() {
            return match task.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })? {
                SnapshotRetirementStep::Complete if task.terminal_is_empty() => {
                    drop(self.task.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("ephemeral construction task completed with retained owners".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_bytes => Err("ephemeral construction task retirement exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(mutation) = self.mutation.take() {
            *self.retirement = Some(self.mutation_retirement.retire_owned(mutation));
        } else if let Some(prepared) = self.prepared.take() {
            *self.retirement = Some(Box::new(ReturnedSnapshotReadRetirement::new(prepared.next_root, self.state_retirement.clone())));
        } else if let Some(base) = self.base.take() {
            match base.0 {
                ArtifactEphemeralBaseOwner::Transient(root) => *self.retirement = Some(Box::new(ReturnedSnapshotReadRetirement::new(root, self.state_retirement.clone()))),
                ArtifactEphemeralBaseOwner::Presence(read) | ArtifactEphemeralBaseOwner::TransientRead(read) => drop(read),
            }
        } else { return Ok(SnapshotRetirementStep::Complete); }
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool { self.base.is_none() && self.mutation.is_none() && self.task.is_none() && self.prepared.is_none() && self.retirement.is_none() }
}

impl<P, M> Drop for ArtifactEphemeralTaskPreparation<P, M> {
    fn drop(&mut self) {
        assert!(self.base.is_none() && self.mutation.is_none() && self.task.is_none() && self.prepared.is_none() && self.retirement.is_none(), "ephemeral preparation dropped before terminal-empty ownership");
    }
}
