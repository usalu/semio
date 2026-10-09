//! 🫧️ Exact zero-payload transient closure through tracked read retirement.

use super::transient_publication::transient_store_disposer;
use crate::store::TransientStore;
use crate::{store, ArtifactOwnedDisposer, NoTransient, NoTransientMutation};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::sync::Arc;

type Store = TransientStore<NoTransient, NoTransientMutation>;
const _: () = assert!(size_of::<NoTransient>() == 0 && !std::mem::needs_drop::<NoTransient>());
const _: () = assert!(size_of::<NoTransientMutation>() == 0 && !std::mem::needs_drop::<NoTransientMutation>());

/// 🫧️ Exact shared and owned retirement for the statically empty transient state.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct NoTransientRetirementFactory;

struct NoTransientRetirement(Option<Arc<NoTransient>>);
struct NoTransientOwnedRetirement(Option<NoTransient>);

impl store::SnapshotRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<NoTransient>) -> usize { std::mem::size_of::<NoTransientRetirement>() }

    fn retire(&self, root: Arc<NoTransient>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<NoTransient>)> {
        match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<NoTransientRetirement>(), depth: 1 }).admit(grant) {
            Ok(progress) => Ok((Box::new(NoTransientRetirement(Some(root))), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

impl store::ArtifactOwnedValueRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retirement_birth_bytes(&self, _root: &NoTransient) -> usize { std::mem::size_of::<NoTransientOwnedRetirement>() }

    fn retire_owned(&self, root: NoTransient, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, NoTransient)> {
        match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<NoTransientOwnedRetirement>(), depth: 1 }).admit(grant) {
            Ok(progress) => Ok((Box::new(NoTransientOwnedRetirement(Some(root))), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

fn close_empty<T>(slot: &mut Option<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if slot.is_none() {
        return Ok(RetainedCloneStep::Complete(Default::default()));
    }
    if grant.maximum_depth < 1 {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "empty transient retirement exceeds admitted depth"));
    }
    if grant.maximum_items == 0 {
        return Ok(RetainedCloneStep::Progress(Default::default()));
    }
    drop(slot.take());
    Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
}

impl store::ErasedSnapshotRetirement for NoTransientRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        close_empty(&mut self.0, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.0.is_some())) }
}

impl Drop for NoTransientRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.0.is_none(), "empty transient must return its exact root before drop");
        }
    }
}

impl store::ErasedSnapshotRetirement for NoTransientOwnedRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        close_empty(&mut self.0, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.0.is_some())) }
}

impl Drop for NoTransientOwnedRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.0.is_none(), "empty transient owned value must retire before drop");
        }
    }
}

pub fn no_transient_store_disposer() -> Box<dyn ArtifactOwnedDisposer<Store>> {
    transient_store_disposer(Arc::new(NoTransientRetirementFactory))
}

pub fn no_transient_local_root_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<NoTransient>> {
    Arc::new(NoTransientRetirementFactory)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
