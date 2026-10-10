//! 🫧️ Exact zero-payload transient closure through tracked read retirement.

use super::transient_publication::transient_store_disposer;
use crate::store::TransientStore;
use crate::{store, ArtifactOwnedDisposer, NoTransient, NoTransientMutation};
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use std::sync::Arc;

type Store = TransientStore<NoTransient, NoTransientMutation>;
const _: () = assert!(size_of::<NoTransient>() == 0 && !std::mem::needs_drop::<NoTransient>());
const _: () = assert!(size_of::<NoTransientMutation>() == 0 && !std::mem::needs_drop::<NoTransientMutation>());

/// 🫧️ Exact shared and owned retirement for the statically empty transient state.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct NoTransientRetirementFactory;

impl store::SnapshotRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retirement_birth_bytes(&self, _: &Arc<NoTransient>) -> usize { semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<NoTransient>() }
    fn retire(&self, root: Arc<NoTransient>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<NoTransient>)> { semio_framework_value::retirement::shared::admit_shared_retirement(root, grant, true) }
}

impl store::ArtifactOwnedValueRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retirement_birth_bytes(&self, _: &NoTransient) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<NoTransient>() }
    fn retire_owned(&self, root: NoTransient, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, NoTransient)> { semio_framework_value::retirement::admit_owned_retirement(root, grant) }
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
