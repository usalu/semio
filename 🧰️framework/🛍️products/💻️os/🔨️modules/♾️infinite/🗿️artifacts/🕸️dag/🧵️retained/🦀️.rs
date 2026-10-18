//! 🧵️ Original DAG fields retain ownership through independently admitted canonical retirement.
#[derive(Clone,Copy)]
pub struct DagMemberStoreProducer;

use crate::os_store::{ArtifactOwnedValueRetirementFactory,ArtifactStoreCursorDisposer,ErasedSnapshotRetirement,document_store_owners::MemberStoreProducer,document_store_owners::DocumentStoreOwners,SnapshotRetirementFactory};
use crate::{DagMutation,DagSnapshot};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},factory::{OwnedValueRetirementFactory,SharedValueRetirementFactory}};
use std::sync::Arc;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DagSnapshotRetirementFactory;
impl SnapshotRetirementFactory<DagSnapshot> for DagSnapshotRetirementFactory {
 fn retirement_birth_bytes(&self,value:&Arc<DagSnapshot>)->usize{SharedValueRetirementFactory::<DagSnapshot>::default().retirement_birth_bytes(value)}
 fn retire(&self,value:Arc<DagSnapshot>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<DagSnapshot>)>{SharedValueRetirementFactory::<DagSnapshot>::default().retire(value,grant)}
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DagOwnedSnapshotRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<DagSnapshot> for DagOwnedSnapshotRetirementFactory {
 fn retirement_birth_bytes(&self,value:&DagSnapshot)->usize{OwnedValueRetirementFactory::<DagSnapshot>::default().retirement_birth_bytes(value)}
 fn retire_owned(&self,value:DagSnapshot,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,DagSnapshot)>{OwnedValueRetirementFactory::<DagSnapshot>::default().retire_owned(value,grant)}
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DagMutationRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<DagMutation> for DagMutationRetirementFactory {
 fn retirement_birth_bytes(&self,value:&DagMutation)->usize{OwnedValueRetirementFactory::<DagMutation>::default().retirement_birth_bytes(value)}
 fn retire_owned(&self,value:DagMutation,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,DagMutation)>{OwnedValueRetirementFactory::<DagMutation>::default().retire_owned(value,grant)}
}

impl MemberStoreProducer<DagSnapshot,DagMutation> for DagMemberStoreProducer {
    /// 📦️ A DAG document opens as an owned member through its OWN `ArtifactPack` codec. It declared
    /// `UnsupportedMemberSnapshotOpen` until 2026-09-21, whose `step` has exactly one answer —
    /// `Rejected(MemberOpenDiagnostic::Decode)` at step 0 — so every composed replacement and every
    /// document archive carrying a real DAG member was refused before it began.
    type SnapshotOpen = crate::os_store::ProducedMemberSnapshotOpen<DagSnapshot,crate::os_store::ArtifactPackSnapshotDecoder<DagSnapshot>>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<DagSnapshot, DagMutation>::source_birth_bytes::<DagSnapshotRetirementFactory, DagOwnedSnapshotRetirementFactory, DagMutationRetirementFactory, ArtifactStoreCursorDisposer<DagSnapshot, DagMutation>>()?, depth: 1 })
    }

    fn member_store_owners(&self,grant: RetainedCloneGrant) -> Result<(DocumentStoreOwners<DagSnapshot, DagMutation>, RetainedCloneProgress), crate::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<DagSnapshot, DagMutation>> {
        DocumentStoreOwners::admit_source_constructor(grant, || (DagSnapshotRetirementFactory, DagOwnedSnapshotRetirementFactory, DagMutationRetirementFactory, ArtifactStoreCursorDisposer::<DagSnapshot, DagMutation>::new()))
    }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
