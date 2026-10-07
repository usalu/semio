//! ♻️ Exact registered snapshot and operation-vector ownership for interactive replay cleanup.

use super::super::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, ReturnedSnapshotReadRetirement, SnapshotRetirementStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{mem::{ManuallyDrop, size_of}, sync::Arc};

pub(crate) trait ArtifactReplayRetirementFactory<P, M>: Send + Sync {
    fn snapshot(&self, owner: Arc<P>) -> Box<dyn ErasedSnapshotRetirement>;
    fn mutations(&self, owners: semio_framework_value::list::PagedList<M, {usize::MAX}>) -> Box<dyn ErasedSnapshotRetirement>;
}

struct RegisteredReplayRetirement<P, M> {
    snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactReplayRetirementFactory<P, M> for RegisteredReplayRetirement<P, M> {
    fn snapshot(&self, owner: Arc<P>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(ReturnedSnapshotReadRetirement::new(owner, self.snapshots.clone())) }
    fn mutations(&self, owners: semio_framework_value::list::PagedList<M, {usize::MAX}>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(ReplayMutationsRetirement { owners: ManuallyDrop::new(Some(owners)), active: None, factory: self.mutations.clone() }) }
}

pub(super) fn registered<P: Send + Sync + 'static, M: Send + 'static>(snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>) -> Arc<dyn ArtifactReplayRetirementFactory<P, M>> {
    Arc::new(RegisteredReplayRetirement { snapshots, mutations })
}

pub(super) fn chain(first: Box<dyn ErasedSnapshotRetirement>, second: Box<dyn ErasedSnapshotRetirement>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(ReplayPairRetirement { first: Some(first), second: Some(second) })
}

struct ReplayPairRetirement { first: Option<Box<dyn ErasedSnapshotRetirement>>, second: Option<Box<dyn ErasedSnapshotRetirement>> }

impl ErasedSnapshotRetirement for ReplayPairRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        let slot = if self.first.is_some() { &mut self.first } else { &mut self.second };
        let Some(active) = slot.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        let step = active.close_step(1, maximum_bytes)?;
        if step == SnapshotRetirementStep::Complete {
            if !active.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "replay cleanup pair child has live owners")); }
            *slot = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(step)
    }
    fn terminal_is_empty(&self) -> bool { self.first.is_none() && self.second.is_none() }
    fn next_close_byte_demand(&self) -> usize { self.first.as_ref().or(self.second.as_ref()).map_or(1, |active| active.next_close_byte_demand()) }
}

impl Drop for ReplayPairRetirement {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "replay cleanup pair dropped before terminal retirement"); }
}

struct ReplayMutationsRetirement<M: Send + 'static> {
    owners: ManuallyDrop<Option<semio_framework_value::list::PagedList<M, {usize::MAX}>>>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}

impl<M: Send + 'static> ErasedSnapshotRetirement for ReplayMutationsRetirement<M> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(active) = self.active.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            if step == SnapshotRetirementStep::Complete {
                if !active.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "replay mutation retirement child has live owners")); }
                self.active = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        let Some(owners) = self.owners.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        if let Some(owner) = owners.pop() {
            self.active = Some(self.factory.retire_owned(owner));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if owners.terminal_is_empty() {
            drop(self.owners.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let bytes = owners.next_release_allocation_bytes().map_err(ValueError::from)?;
        if bytes > maximum_bytes { return Ok(SnapshotRetirementStep::Blocked); }
        let progress = owners.release_empty_page(maximum_bytes).map_err(ValueError::from)?;
        Ok(SnapshotRetirementStep::Pending { released_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes })
    }

    fn terminal_is_empty(&self) -> bool { self.owners.is_none() && self.active.is_none() }

    fn next_close_byte_demand(&self) -> usize {
        self.active.as_ref().map_or_else(|| self.owners.as_ref().map_or(1, |owners| if owners.is_empty() { owners.next_release_allocation_bytes().unwrap_or(1).max(1) } else { 1 }), |active| active.next_close_byte_demand())
    }
}

impl<M: Send + 'static> Drop for ReplayMutationsRetirement<M> {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "replay mutations dropped before exact terminal retirement"); }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
