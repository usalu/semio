//! 📋️ Original vector rows keep their allocation and borrowed exact issuer through bounded retirement.
use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind, factory_ticket_demands, close_factory_ticket};
use crate::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::{ManuallyDrop, size_of};
#[path="🔑️identity/🦀️.rs"] mod identity;
pub use identity::FactoryIssuerIdentity;

/// 🧺️ The original vector allocation remains owned until every row and child ticket settles.
pub struct FactoryVectorRetirement<T: Send + 'static> {
    remaining: ManuallyDrop<Option<std::vec::IntoIter<T>>>,
    pending: ManuallyDrop<Option<T>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    allocation_bytes: usize,
    allocation_address: usize,
    issuer: Option<FactoryIssuerIdentity<T>>,
}

fn fits(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

impl<T: Send + 'static> FactoryVectorRetirement<T> {
    /// 🌱️ Capturing original vector headers constructs no child or copied row.
    pub const fn constructor_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<Self>(), capacity_bytes: 0, release_bytes: 0, depth: 1 } }

    /// 🎟️ Returns the untouched original on insufficient capture admission.
    pub fn admit(original: Vec<T>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, Vec<T>)> {
        let demand = Self::constructor_demand();
        if !fits(demand, grant) { return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original vector capture requires its complete fixed-header grant"),original)); }
        let Some(allocation_bytes) = original.capacity().checked_mul(size_of::<T>()) else { return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original vector allocation extent overflow"),original)); };
        let allocation_address = original.as_ptr() as usize;
        Ok((Self { remaining: ManuallyDrop::new(Some(original.into_iter())), pending: ManuallyDrop::new(None), active: ManuallyDrop::new(None), allocation_bytes, allocation_address, issuer: None }, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    /// 👁️ Borrows a refused original row without moving or recreating it.
    pub fn pending(&self) -> Option<&T> { self.pending.as_ref() }

    /// 🧷️ Retains the original allocation address independently of the iterator's advancing slice.
    pub fn allocation_address(&self) -> usize { self.allocation_address }

    /// 📐️ Retains the precise native allocation extent through every row frontier.
    pub fn allocation_bytes(&self) -> usize { self.allocation_bytes }

    /// 📏️ Quotes one next row capture, exact provider birth, child close or paid backing release.
    pub fn demands(&self, factory: &dyn ArtifactOwnedValueRetirementFactory<T>, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.check_issuer(factory)?;
        let capture=if self.issuer.is_none()&&!self.terminal_is_empty(){FactoryIssuerIdentity::<T>::capture_copy_bytes()}else{0};
        let mut demand=self.owner_demands(factory,maximum_copy_bytes.saturating_sub(capture))?;
        demand.copy_bytes=demand.copy_bytes.checked_add(capture).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original issuer capture extent overflow"))?;
        Ok(demand)
    }

    fn owner_demands(&self, factory: &dyn ArtifactOwnedValueRetirementFactory<T>, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.check_issuer(factory)?;
        if let Some(active) = self.active.as_ref() {
            let mut demand = factory_ticket_demands(active, maximum_copy_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit,"original vector child depth overflow"))?;
            return Ok(demand);
        }
        if let Some(row) = self.pending.as_ref() { return Ok(RetirementDemand { copy_bytes: size_of::<T>().saturating_mul(2), capacity_bytes: factory.retirement_birth_bytes(row), release_bytes: 0, depth: 2 }); }
        if self.remaining.as_ref().is_some_and(|rows| rows.len() != 0) { return Ok(RetirementDemand { copy_bytes: size_of::<T>().saturating_mul(2), depth: 1, ..Default::default() }); }
        if self.remaining.is_some() { return Ok(RetirementDemand { release_bytes: self.allocation_bytes, depth: 1, ..Default::default() }); }
        Ok(Default::default())
    }

    /// 🪜️ Reads original row order once, restores provider refusals and releases only empty paid backing.
    pub fn close_step(&mut self, factory: &dyn ArtifactOwnedValueRetirementFactory<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demands(factory, grant.maximum_copy_bytes)?;
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !fits(demand, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.issuer.is_none(){self.issuer=Some(FactoryIssuerIdentity::capture(factory));}
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return close_factory_ticket(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(row) = self.pending.take() {
            let child = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - demand.copy_bytes, ..child };
            return match factory.retire_owned(row, child) {
                Ok((active, mut progress)) => {
                    *self.active = Some(active);
                    progress.copied_bytes = progress.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit,"original vector provider copy receipt overflow"))?;
                    if !progress.fits(grant) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original vector provider exceeded its admitted capture or constructor").with_retained_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, row)) => {
                    *self.pending = Some(row);
                    let mut progress = error.retained_progress();
                    progress.copied_items = progress.copied_items.max(1);
                    progress.copied_bytes = progress.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit,"original vector refusal copy receipt overflow"))?;
                    Err(error.with_retained_progress(progress))
                },
            };
        }
        let remaining = self.remaining.as_mut().expect("original vector backing retained");
        if let Some(row) = remaining.next() {
            *self.pending = Some(row);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        drop(self.remaining.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: self.allocation_bytes, ..Default::default() }))
    }

    /// 🫙️ No hidden iterator capacity, original row or child ticket remains at terminal.
    pub fn terminal_is_empty(&self) -> bool { self.remaining.is_none() && self.pending.is_none() && self.active.is_none() }

    fn check_issuer(&self, factory: &dyn ArtifactOwnedValueRetirementFactory<T>) -> Result<(), ValueError> {
        if self.issuer.is_some_and(|original| !original.matches(factory)) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original vector disposal issuer changed")); }
        Ok(())
    }
}

impl<T: Send + 'static> Drop for FactoryVectorRetirement<T> {
    fn drop(&mut self) {
        let empty = self.terminal_is_empty();
        assert!(empty || std::thread::panicking(),"original vector factory retirement requires complete funded closure");
        if empty { unsafe { ManuallyDrop::drop(&mut self.remaining); ManuallyDrop::drop(&mut self.pending); ManuallyDrop::drop(&mut self.active); } }
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
