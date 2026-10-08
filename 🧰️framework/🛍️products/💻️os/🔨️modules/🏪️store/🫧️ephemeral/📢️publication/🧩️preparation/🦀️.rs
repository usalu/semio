//! 🧩️ Shared ephemeral preparation lifecycle around domain-owned construction tasks.

use super::{
    ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead, ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest, ArtifactEphemeralOneItemPrepared,
    ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ErasedSnapshotRetirement, ReturnedSnapshotReadRetirement,
};
use std::{mem::ManuallyDrop, sync::Arc};

pub enum ArtifactEphemeralPreparationTaskStep<P> {
    Progress(ArtifactStoreOneItemCheckpoint),
    Prepared { root: P, checkpoint: ArtifactStoreOneItemCheckpoint },
    Blocked,
}

/// 🛠️ Constructs one root within each grant; consumed mutations remain in the task or result.
pub trait ArtifactEphemeralPreparationTask<P, M>: ErasedSnapshotRetirement {
    fn advance(&mut self, base: &P, mutation: &mut Option<M>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<P>, String>;
    fn begin_close(&mut self);
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

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralTaskPreparation<P, M> {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "ephemeral preparation depth overflow"))?; Ok(demand) };
        if let Some(owner) = self.retirement.as_ref() { return nested(crate::os_store::artifact_retirement_box_demands(owner, body)?); }
        if let Some(task) = self.task.as_ref() { return if task.terminal_is_empty() { Ok(RetirementDemand { release_bytes: std::mem::size_of_val(task.as_ref()), depth: 1, ..Default::default() }) } else { nested(RetirementDemand { copy_bytes: task.next_copy_byte_demand()?, capacity_bytes: task.next_capacity_byte_demand(body)?, release_bytes: task.next_release_byte_demand()?, depth: task.next_depth_demand()? }) }; }
        if let Some(mutation) = self.mutation.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.mutation_retirement.as_ref().expect("original mutation factory").retirement_birth_bytes(mutation), depth: 2, ..Default::default() }); }
        if self.prepared.is_some() || self.base.as_ref().is_some_and(|base| matches!(base.0, ArtifactEphemeralBaseOwner::Transient(_))) { return Ok(RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }); }
        if self.base.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(registry) = self.base_registry.as_ref() { if Arc::strong_count(registry) == 1 && registry.has_returned() { return nested(registry.returned_admission_demands::<P>(|_| RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(crate::os_store::SnapshotReadLeaseRefusal::into_value_error)?); } return crate::os_store::snapshot_registry_alias_demands(&self.base_registry); }
        if self.state_retirement.is_some() || self.mutation_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.iter().flatten().next() { return nested(factory.demands(body)?); }
        Ok(Default::default())
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralOneItemPreparationFactory<P, M> for ArtifactEphemeralTaskPreparationFactory<P, M> {
    fn preflight(&self, mutation: &M) -> Result<ArtifactStoreOneItemFootprint, String> {
        (self.preflight)(mutation)
    }

    fn begin(&self, request: ArtifactEphemeralOneItemPreparationRequest<P, M>) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<P, M>>, ArtifactEphemeralOneItemPreparationRequest<P, M>> {
        if !self.preflight(&request.mutation).is_ok_and(ArtifactStoreOneItemFootprint::is_admissible) {
            return Err(request);
        }
        let task = match (self.create_task)(request.base.as_ref(), &request.mutation) {
            Ok(task) => task,
            Err(_) => return Err(request),
        };
        Ok(Box::new(ArtifactEphemeralTaskPreparation::new(request, task, self.state_retirement.clone(), self.mutation_retirement.clone())))
    }
}

pub(super) struct ArtifactEphemeralTaskPreparation<P, M> {
    base: ManuallyDrop<Option<ArtifactEphemeralBaseRead<P>>>,
    mutation: ManuallyDrop<Option<M>>,
    task: ManuallyDrop<Option<Box<dyn ArtifactEphemeralPreparationTask<P, M>>>>,
    prepared: ManuallyDrop<Option<ArtifactEphemeralOneItemPrepared<P>>>,
    retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    state_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    base_registry: ManuallyDrop<Option<Arc<crate::os_store::SnapshotReadLeaseRegistry>>>,
    factory_close: [Option<semio_framework_value::FactoryAuthority>; 2],
    checkpoint: ArtifactStoreOneItemCheckpoint,
    constructed: bool,
    cancelled: bool,
    closing: bool,
}

impl<P, M> ArtifactEphemeralTaskPreparation<P, M> {
    pub(super) fn new(
        request: ArtifactEphemeralOneItemPreparationRequest<P, M>,
        task: Box<dyn ArtifactEphemeralPreparationTask<P, M>>,
        state_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
    ) -> Self {
        Self {
            base: ManuallyDrop::new(Some(request.base)),
            mutation: ManuallyDrop::new(Some(request.mutation)),
            task: ManuallyDrop::new(Some(task)),
            prepared: ManuallyDrop::new(None),
            retirement: ManuallyDrop::new(None),
            state_retirement: Some(state_retirement),
            mutation_retirement: Some(mutation_retirement),
            base_registry: ManuallyDrop::new(None),
            factory_close: Default::default(),
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            constructed: false,
            cancelled: false,
            closing: false,
        }
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactEphemeralOneItemPreparation<P, M> for ArtifactEphemeralTaskPreparation<P, M> {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || self.closing || grant.maximum_items == 0 {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        if self.constructed {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let task = self.task.as_mut().ok_or_else(|| "ephemeral preparation lost its construction task".to_string())?;
        let base = self.base.as_ref().ok_or_else(|| "ephemeral preparation lost its base read".to_string())?;
        let step = task.advance(base.as_ref(), &mut self.mutation, ArtifactStoreOneItemGrant { maximum_items: 1, ..grant })?;
        let checkpoint = match &step {
            ArtifactEphemeralPreparationTaskStep::Progress(checkpoint) | ArtifactEphemeralPreparationTaskStep::Prepared { checkpoint, .. } => Some(*checkpoint),
            ArtifactEphemeralPreparationTaskStep::Blocked => None,
        };
        let invalid = checkpoint.is_some_and(|next| {
            next.completed_items < self.checkpoint.completed_items
                || next.completed_items - self.checkpoint.completed_items > 1
                || next.completed_bytes < self.checkpoint.completed_bytes
                || next.completed_bytes - self.checkpoint.completed_bytes > grant.maximum_copy_bytes as u64
        });
        let result = match step {
            ArtifactEphemeralPreparationTaskStep::Progress(checkpoint) => {
                self.checkpoint = checkpoint;
                ArtifactStoreOneItemPreparationStep::Progress(checkpoint)
            }
            ArtifactEphemeralPreparationTaskStep::Prepared { root, checkpoint } => {
                *self.prepared = Some(ArtifactEphemeralOneItemPrepared { next_root: Arc::new(root) });
                self.constructed = true;
                self.checkpoint = checkpoint;
                ArtifactStoreOneItemPreparationStep::Prepared(checkpoint)
            }
            ArtifactEphemeralPreparationTaskStep::Blocked => ArtifactStoreOneItemPreparationStep::Blocked,
        };
        if invalid {
            return Err("ephemeral construction task exceeded its exact grant".into());
        }
        Ok(result)
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<P>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<P>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(task) = self.task.as_mut() {
            task.begin_close();
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "ephemeral preparation close was not started")); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "ephemeral preparation exceeds admitted close depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant.retained_grant() };
        if self.retirement.is_some() { return crate::os_store::artifact_retirement_box_close_step(&mut self.retirement, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(task) = self.task.as_mut() {
            if !task.terminal_is_empty() { let step = task.close_step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, task.terminal_is_empty(), "ephemeral construction task")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
            self.task.take(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(original) = self.mutation.take() { return match self.mutation_retirement.as_ref().expect("original mutation factory").retire_owned(original, child) { Ok((owner, receipt)) => { *self.retirement = Some(owner); if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "ephemeral mutation constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, original)) => { *self.mutation = Some(original); Err(error) } }; }
        if let Some(prepared) = self.prepared.take() { return match ReturnedSnapshotReadRetirement::admit(prepared.next_root, self.state_retirement.as_ref().expect("original state factory").clone(), child) { Ok((owner, receipt)) => { *self.retirement = Some(owner); Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, root, alias)) => { *self.prepared = Some(ArtifactEphemeralOneItemPrepared { next_root: root }); drop(alias); Err(error) } }; }
        if let Some(base) = self.base.as_mut() {
            match &mut base.0 {
                ArtifactEphemeralBaseOwner::Transient(root) => {
                    let original = self.base.take().expect("observed original transient base");
                    let ArtifactEphemeralBaseOwner::Transient(root) = original.0 else { unreachable!() };
                    return match ReturnedSnapshotReadRetirement::admit(root, self.state_retirement.as_ref().expect("original state factory").clone(), child) { Ok((owner, receipt)) => { *self.retirement = Some(owner); Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, root, alias)) => { *self.base = Some(ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Transient(root))); drop(alias); Err(error) } };
                }
                ArtifactEphemeralBaseOwner::Presence(read) | ArtifactEphemeralBaseOwner::TransientRead(read) => {
                    if read.owner.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
                    if let Some(mut lease) = read.lease.take() { lease.return_now(); *self.base_registry = Some(lease.registry); }
                    self.base.take(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
            }
        }
        if let Some(registry) = self.base_registry.as_ref() {
            if Arc::strong_count(registry) == 1 && registry.has_returned() { return crate::os_store::advance_returned_snapshot_read(registry, &mut self.retirement, self.state_retirement.as_ref().expect("original state factory"), child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            return crate::os_store::snapshot_registry_alias_close_step(&mut self.base_registry, grant.retained_grant()).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.mutation_retirement.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(factory) = self.state_retirement.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().unwrap(); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "ephemeral preparation factory")?; if factory.terminal_is_empty() { *slot = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool { self.base.is_none() && self.mutation.is_none() && self.task.is_none() && self.prepared.is_none() && self.retirement.is_none() && self.base_registry.is_none() && self.state_retirement.is_none() && self.mutation_retirement.is_none() && self.factory_close.iter().all(Option::is_none) }

}

impl<P, M> Drop for ArtifactEphemeralTaskPreparation<P, M> {
    fn drop(&mut self) {
        assert!(self.base.is_none() && self.mutation.is_none() && self.task.is_none() && self.prepared.is_none() && self.retirement.is_none() && self.base_registry.is_none() && self.state_retirement.is_none() && self.mutation_retirement.is_none() && self.factory_close.iter().all(Option::is_none), "ephemeral preparation dropped before terminal-empty ownership");
    }
}
