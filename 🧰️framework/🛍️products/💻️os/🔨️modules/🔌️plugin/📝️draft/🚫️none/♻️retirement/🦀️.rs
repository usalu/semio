//! 📝️ Exact store owners for the zero-payload draft lane, and the zero-payload retirement every
//! statically empty lane (`NoDraft`, `NoConfig`) shares.

use semio_plugin_ownership::store_lifecycle::{bounded_document_store_disposer, ArtifactOwnedDisposer};
use crate::NoDraft;
use crate::NoDraftMutation;
use crate::store;
use semio_framework_value::{ValueError, OwnedValueRetirementFactory, SharedValueRetirementFactory, retained_clone::RetainedCloneBirthDemand, retirement::RetireOwned};

/// 🫙️ Exact owner catalogue for a store whose snapshot and mutation types are statically empty.
pub(super) fn zero_payload_store_owners<P, M>() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<P, M>, ValueError>
where
    P: RetireOwned + Sync + pack::snapshot::ArtifactPack + ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::StoreState,
    M: RetireOwned + store::ArtifactCanonicalJsonTree + store::StoreMutationFor<P>,
{
    let capacity_bytes = store::os_store::document_store_owners::DocumentStoreOwners::<P, M>::source_birth_bytes::<SharedValueRetirementFactory<P>, OwnedValueRetirementFactory<P>, OwnedValueRetirementFactory<M>, store::ArtifactStoreCursorDisposer<P, M>>()?;
    store::os_store::document_store_owners::fund_document_store_owners(RetainedCloneBirthDemand { capacity_bytes, depth: 1 }, |grant| {
        store::os_store::document_store_owners::DocumentStoreOwners::admit_source_constructor(grant, || (SharedValueRetirementFactory::<P>::default(), OwnedValueRetirementFactory::<P>::default(), OwnedValueRetirementFactory::<M>::default(), store::ArtifactStoreCursorDisposer::<P, M>::new()))
    })
}

pub fn no_draft_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<NoDraft, NoDraftMutation>, ValueError> {
    zero_payload_store_owners::<NoDraft, NoDraftMutation>()
}

pub fn no_draft_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::DraftStore<NoDraft, NoDraftMutation>>> {
    bounded_document_store_disposer::<NoDraft, NoDraftMutation>()
}
