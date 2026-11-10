//! 🫧️ Exact zero-payload transient closure through tracked read retirement.

use semio_plugin_ownership::transient_publication::transient_store_disposer;
use crate::store::TransientStore;
use crate::store;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use crate::NoTransient;
use crate::NoTransientMutation;
use semio_framework_value::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use std::sync::Arc;

type Store = TransientStore<NoTransient, NoTransientMutation>;
const _: () = assert!(size_of::<NoTransient>() == 0 && !std::mem::needs_drop::<NoTransient>());
const _: () = assert!(size_of::<NoTransientMutation>() == 0 && !std::mem::needs_drop::<NoTransientMutation>());

pub fn no_transient_store_disposer() -> Box<dyn ArtifactOwnedDisposer<Store>> {
    transient_store_disposer(Arc::new(OwnedValueRetirementFactory::<NoTransient>::default()))
}

pub fn no_transient_local_root_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<NoTransient>> {
    Arc::new(SharedValueRetirementFactory::<NoTransient>::default())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
