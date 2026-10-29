//! 🏷️ Candidate publication and composition pins: the authoritative store candidate publication helpers and the composition pins retirement with its drop guard. See [`super`].

use super::*;

/// 🔐️ Performs the sole document-store swap only inside its fresh authority decision.
#[expect(clippy::result_large_err, reason = "A rejected publication returns the unchanged candidate store, preserving its existing bounded retirement authority.")]
pub fn publish_document_store_candidate_if_authoritative<P, Mutation>(
    live: &mut ArtifactStore<P, Mutation>,
    candidate: ArtifactStore<P, Mutation>,
    validate: impl FnOnce() -> Result<(), Fault>,
) -> Result<ArtifactStore<P, Mutation>, (Fault, ArtifactStore<P, Mutation>)>
where
    P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::StoreState,
    Mutation: store::StoreMutationFor<P>,
{
    if let Err(fault) = validate() {
        return Err((fault, candidate));
    }
    Ok(std::mem::replace(live, candidate))
}

/// 🔄️ Publishes the replacement owner's heap candidate without moving either complete Store
/// through the maintenance stack; the returned box owns the displaced live Store in place.
#[doc(hidden)]
pub fn publish_boxed_document_store_candidate_if_authoritative<P, Mutation>(
    live: &mut ArtifactStore<P, Mutation>,
    mut candidate: Box<ArtifactStore<P, Mutation>>,
    validate: impl FnOnce() -> Result<(), Fault>,
) -> Result<Box<ArtifactStore<P, Mutation>>, (Fault, Box<ArtifactStore<P, Mutation>>)>
where
    P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::StoreState,
    Mutation: store::StoreMutationFor<P>,
{
    if let Err(fault) = validate() {
        return Err((fault, candidate));
    }
    std::mem::swap(live, candidate.as_mut());
    Ok(candidate)
}

#[doc(hidden)]
pub struct CompositionPinsRetirement {
    pub pins: ManuallyDrop<Option<Vec<vcs::CompositionPin>>>,
    pub active: ManuallyDrop<Option<vcs::CompositionPin>>,
    pub field: u8,
}

impl CompositionPinsRetirement {
    pub fn new(pins: Vec<vcs::CompositionPin>) -> Self {
        Self { pins: ManuallyDrop::new(Some(pins)), active: ManuallyDrop::new(None), field: 0 }
    }

    fn active_string_mut(&mut self) -> Option<&mut String> {
        let pin = self.active.as_mut()?;
        match self.field {
            0 => Some(&mut pin.child_ref.artifact_id),
            1 => Some(&mut pin.child_ref.dialect.artifact_kind),
            2 => Some(&mut pin.child_ref.dialect.standard),
            3 => Some(&mut pin.child_ref.dialect.subset),
            4 => Some(&mut pin.checkpoint_id),
            _ => None,
        }
    }

    /// 📐️ Quotes one original pin transfer, native field, or separate vector backing.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(pin) = self.active.as_ref() {
            let value = match self.field { 0 => Some(&pin.child_ref.artifact_id), 1 => Some(&pin.child_ref.dialect.artifact_kind), 2 => Some(&pin.child_ref.dialect.standard), 3 => Some(&pin.child_ref.dialect.subset), 4 => Some(&pin.checkpoint_id), _ => None };
            return Ok(value.map_or(RetirementDemand { copy_bytes: size_of_val(&*self.active), depth: 1, ..Default::default() }, |value| RetirementDemand { copy_bytes: size_of::<String>() + size_of::<u8>(), release_bytes: value.capacity(), depth: 1, ..Default::default() }));
        }
        let Some(pins) = self.pins.as_ref() else { return Ok(RetirementDemand::default()); };
        if !pins.is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<vcs::CompositionPin>() + size_of_val(&*self.active) + size_of::<usize>() + size_of::<u8>(), depth: 1, ..Default::default() }); }
        let release_bytes = pins.capacity().checked_mul(size_of::<vcs::CompositionPin>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original composition-pin backing extent overflowed"))?;
        Ok(RetirementDemand { copy_bytes: size_of_val(&*self.pins), release_bytes, depth: 1, ..Default::default() })
    }

    /// 🧹️ Retains every original until all five caller currencies admit its physical unit.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands().map_err(|error| plugin_sdk_fault(error.into_message()))?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.active.is_some() {
            if let Some(value) = self.active_string_mut() { drop(std::mem::take(value)); self.field += 1; } else { drop(self.active.take()); }
        } else if self.pins.as_ref().is_some_and(|pins| !pins.is_empty()) {
            *self.active = self.pins.as_mut().and_then(Vec::pop);
            self.field = 0;
        } else { drop(self.pins.take()); }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }))
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.pins.is_none() && self.active.is_none()
    }
}

impl Drop for CompositionPinsRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "composition-pin retirement reached Drop before every displaced identity owner was bounded-closed");
    }
}
