//! 🧵️ Original DAG fields retain ownership through independently admitted canonical retirement.
#[derive(Clone,Copy)]
pub struct DagMemberStoreProducer;

use crate::{DagMutation, DagSnapshot};
use crate::os_store::{ArtifactStoreCursorDisposer, document_store_owners::{DocumentStoreOwners, MemberStoreProducer}};
use semio_framework_value::{ValueError, factory::{OwnedValueRetirementFactory, SharedValueRetirementFactory}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};

impl MemberStoreProducer<DagSnapshot,DagMutation> for DagMemberStoreProducer {
    /// 📦️ A DAG document opens as an owned member by receiving its original text record directly into native fields.
    type SnapshotOpen = crate::os_store::OriginalRecordMemberSnapshotOpen<DagSnapshot>;

    /// 🧮️ Catalog source construction copies no bytes: it donates fixed factories and reports only retained capacity.
    fn member_store_owners_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<DagSnapshot, DagMutation>::source_birth_bytes::<SharedValueRetirementFactory<DagSnapshot>, OwnedValueRetirementFactory<DagSnapshot>, OwnedValueRetirementFactory<DagMutation>, ArtifactStoreCursorDisposer<DagSnapshot, DagMutation>>()?, depth: 1 })
    }

    fn member_store_owners(&self,grant: RetainedCloneGrant) -> Result<(DocumentStoreOwners<DagSnapshot, DagMutation>, RetainedCloneProgress), crate::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<DagSnapshot, DagMutation>> {
        DocumentStoreOwners::admit_source_constructor(grant, || (
            SharedValueRetirementFactory::<DagSnapshot>::default(),
            OwnedValueRetirementFactory::<DagSnapshot>::default(),
            OwnedValueRetirementFactory::<DagMutation>::default(),
            ArtifactStoreCursorDisposer::<DagSnapshot, DagMutation>::new(),
        ))
    }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
