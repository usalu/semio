//! 📝️ Exact store owners for the zero-payload draft lane.

use crate::app::{bounded_document_store_disposer, ArtifactOwnedDisposer, NoDraft, NoDraftMutation};
use crate::store;
use std::mem::ManuallyDrop;
use std::sync::Arc;

struct NoDraftRetirement<T> {
    value: ManuallyDrop<Option<T>>,
}

impl<T: Send + 'static> store::ErasedSnapshotRetirement for NoDraftRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(value) = self.value.take() {
            drop(value);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl<T> Drop for NoDraftRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.value.is_none(), "no-draft retirement reached Drop before exact terminal emptiness");
    }
}

struct NoDraftRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for NoDraftRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NoDraftRetirement { value: ManuallyDrop::new(Some(value)) })
    }
}

impl<T: Send + Sync + 'static> store::SnapshotRetirementFactory<T> for NoDraftRetirementFactory<T> {
    fn retire(&self, snapshot: Arc<T>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NoDraftRetirement { value: ManuallyDrop::new(Some(snapshot)) })
    }
}

pub fn no_draft_store_owners() -> store::DocumentStoreOwners<NoDraft, NoDraftMutation> {
    store::DocumentStoreOwners::new(
        Arc::new(NoDraftRetirementFactory(std::marker::PhantomData)),
        Arc::new(NoDraftRetirementFactory(std::marker::PhantomData)),
        Arc::new(NoDraftRetirementFactory(std::marker::PhantomData)),
        Box::new(store::ArtifactStoreCursorDisposer::<NoDraft, NoDraftMutation>::new()),
    )
}

pub fn no_draft_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::DraftStore<NoDraft, NoDraftMutation>>> {
    bounded_document_store_disposer::<NoDraft, NoDraftMutation>()
}
