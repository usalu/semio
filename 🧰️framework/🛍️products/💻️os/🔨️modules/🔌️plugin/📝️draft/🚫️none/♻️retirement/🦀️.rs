//! 📝️ Exact store owners for the zero-payload draft lane, and the zero-payload retirement every
//! statically empty lane (`NoDraft`, `NoConfig`) shares.

use crate::app::{bounded_document_store_disposer, ArtifactOwnedDisposer, NoDraft, NoDraftMutation};
use crate::store;
use std::mem::ManuallyDrop;
use std::sync::Arc;

/// 🫙️ Releases one statically empty value as one owner and zero payload bytes, under ANY positive
/// item grant — a zero-payload value has no byte minimum to wait for, so it never answers
/// `Pending { 0, 0 }` to a sub-page grant.
struct ZeroPayloadRetirement<T> {
    value: ManuallyDrop<Option<T>>,
}

impl<T: Send + 'static> store::ErasedSnapshotRetirement for ZeroPayloadRetirement<T> {
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

impl<T> Drop for ZeroPayloadRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.value.is_none(), "zero-payload retirement reached Drop before exact terminal emptiness");
    }
}

struct ZeroPayloadRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for ZeroPayloadRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(ZeroPayloadRetirement { value: ManuallyDrop::new(Some(value)) })
    }
}

impl<T: Send + Sync + 'static> store::SnapshotRetirementFactory<T> for ZeroPayloadRetirementFactory<T> {
    fn retire(&self, snapshot: Arc<T>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(ZeroPayloadRetirement { value: ManuallyDrop::new(Some(snapshot)) })
    }
}

/// 🫙️ Exact owner catalogue for a store whose snapshot and mutation types are statically empty.
pub(super) fn zero_payload_store_owners<P, M>() -> store::DocumentStoreOwners<P, M>
where
    P: Clone + store::ToValue + store::FromValue + store::ArtifactPack + Send + Sync + 'static,
    M: Clone + store::ToValue + store::FromValue + store::Mutation<P> + store::OpBinary + store::OpText + Send + 'static,
{
    store::DocumentStoreOwners::new(
        Arc::new(ZeroPayloadRetirementFactory(std::marker::PhantomData)),
        Arc::new(ZeroPayloadRetirementFactory(std::marker::PhantomData)),
        Arc::new(ZeroPayloadRetirementFactory(std::marker::PhantomData)),
        Box::new(store::ArtifactStoreCursorDisposer::<P, M>::new()),
    )
}

pub fn no_draft_store_owners() -> store::DocumentStoreOwners<NoDraft, NoDraftMutation> {
    zero_payload_store_owners::<NoDraft, NoDraftMutation>()
}

pub fn no_draft_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::DraftStore<NoDraft, NoDraftMutation>>> {
    bounded_document_store_disposer::<NoDraft, NoDraftMutation>()
}
