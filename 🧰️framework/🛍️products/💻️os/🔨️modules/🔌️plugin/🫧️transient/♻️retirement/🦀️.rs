//! 🫧️ Exact zero-payload transient closure through tracked read retirement.

use super::transient_publication::transient_store_disposer;
use crate::store::TransientStore;
use crate::{store, ArtifactOwnedDisposer, NoTransient, NoTransientMutation};
use std::sync::Arc;

type Store = TransientStore<NoTransient, NoTransientMutation>;
const _: () = assert!(size_of::<NoTransient>() == 0 && !std::mem::needs_drop::<NoTransient>());
const _: () = assert!(size_of::<NoTransientMutation>() == 0 && !std::mem::needs_drop::<NoTransientMutation>());

/// 🫧️ Exact shared and owned retirement for the statically empty transient state.
pub struct NoTransientRetirementFactory;

struct NoTransientRetirement(Option<Arc<NoTransient>>);
struct NoTransientOwnedRetirement(Option<NoTransient>);

impl store::SnapshotRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retire(&self, root: Arc<NoTransient>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NoTransientRetirement(Some(root)))
    }
}

impl store::ArtifactOwnedValueRetirementFactory<NoTransient> for NoTransientRetirementFactory {
    fn retire_owned(&self, root: NoTransient) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NoTransientOwnedRetirement(Some(root)))
    }
}

impl store::ErasedSnapshotRetirement for NoTransientRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.0.is_none() {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        drop(self.0.take());
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl Drop for NoTransientRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.0.is_none(), "empty transient must return its exact root before drop");
        }
    }
}

impl store::ErasedSnapshotRetirement for NoTransientOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.0.is_none() {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        self.0 = None;
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
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
