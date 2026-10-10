//! 📝️ Exact store owners for the zero-payload draft lane, and the zero-payload retirement every
//! statically empty lane (`NoDraft`, `NoConfig`) shares.

use crate::app::{bounded_document_store_disposer, ArtifactOwnedDisposer, NoDraft, NoDraftMutation};
use crate::store;
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use std::sync::Arc;

/// 🫙️ Releases one statically empty value as one owner and zero payload bytes, under ANY positive
/// item grant — a zero-payload value has no byte minimum to wait for, so it never answers
/// an empty receipt to a sub-page grant.
struct ZeroPayloadRetirement<T> {
    value: ManuallyDrop<Option<T>>,
}

impl<T: Send + 'static> store::ErasedSnapshotRetirement for ZeroPayloadRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.value.is_none() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_depth < 1 {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "zero-payload retirement exceeds admitted depth"));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        drop(self.value.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.value.is_some())) }
}

impl<T> Drop for ZeroPayloadRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.value.is_none(), "zero-payload retirement reached Drop before exact terminal emptiness");
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ZeroPayloadRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

fn admit_zero_payload_birth<V>(grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<ZeroPayloadRetirement<V>>(), depth: 1 }.admit(grant)
}

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for ZeroPayloadRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _value: &T) -> usize { std::mem::size_of::<ZeroPayloadRetirement<T>>() }

    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
        match admit_zero_payload_birth::<T>(grant) {
            Ok(progress) => Ok((Box::new(ZeroPayloadRetirement { value: ManuallyDrop::new(Some(value)) }), progress)),
            Err(error) => Err((error, value)),
        }
    }
}

impl<T: Send + Sync + 'static> store::SnapshotRetirementFactory<T> for ZeroPayloadRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<T>) -> usize { std::mem::size_of::<ZeroPayloadRetirement<std::sync::Arc<T>>>() }

    fn retire(&self, snapshot: Arc<T>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<T>)> {
        match admit_zero_payload_birth::<Arc<T>>(grant) {
            Ok(progress) => Ok((Box::new(ZeroPayloadRetirement { value: ManuallyDrop::new(Some(snapshot)) }), progress)),
            Err(error) => Err((error, snapshot)),
        }
    }
}

/// 🫙️ Exact owner catalogue for a store whose snapshot and mutation types are statically empty.
pub(super) fn zero_payload_store_owners<P, M>() -> Result<store::DocumentStoreOwners<P, M>, ValueError>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + store::OpBinary + store::OpText + Send + 'static,
{
    let capacity_bytes = store::DocumentStoreOwners::<P, M>::source_birth_bytes::<ZeroPayloadRetirementFactory<P>, ZeroPayloadRetirementFactory<P>, ZeroPayloadRetirementFactory<M>, store::ArtifactStoreCursorDisposer<P, M>>()?;
    store::fund_document_store_owners(RetainedCloneBirthDemand { capacity_bytes, depth: 1 }, |grant| {
        store::DocumentStoreOwners::admit_source_constructor(grant, || (ZeroPayloadRetirementFactory::<P>(std::marker::PhantomData), ZeroPayloadRetirementFactory::<P>(std::marker::PhantomData), ZeroPayloadRetirementFactory::<M>(std::marker::PhantomData), store::ArtifactStoreCursorDisposer::<P, M>::new()))
    })
}

pub fn no_draft_store_owners() -> Result<store::DocumentStoreOwners<NoDraft, NoDraftMutation>, ValueError> {
    zero_payload_store_owners::<NoDraft, NoDraftMutation>()
}

pub fn no_draft_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::DraftStore<NoDraft, NoDraftMutation>>> {
    bounded_document_store_disposer::<NoDraft, NoDraftMutation>()
}
