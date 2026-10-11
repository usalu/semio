//! 🧵️ Original DAG fields retain ownership through independently admitted canonical retirement.
use crate::os_store::{ArtifactCanonicalAuthoringFactory,ArtifactStoreOneItemPreparationFactory,ArtifactOwnedValueRetirementFactory,ArtifactStoreCursorDisposer,ErasedSnapshotRetirement,MemberStoreOwner,DocumentStoreOwners,SnapshotRetirementFactory};
use crate::{DagMutation,DagSnapshot};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{OwnedValueRetirementFactory,SharedValueRetirementFactory}};
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

impl MemberStoreOwner<DagMutation> for DagSnapshot {
    /// 📦️ A DAG document opens as an owned member through its OWN `ArtifactPack` codec. It declared
    /// `UnsupportedMemberSnapshotOpen` until 2026-09-21, whose `step` has exactly one answer —
    /// `Rejected(MemberOpenDiagnostic::Decode)` at step 0 — so every composed replacement and every
    /// document archive carrying a real DAG member was refused before it began.
    type SnapshotOpen = crate::os_store::PackMemberSnapshotOpen<Self>;

    fn member_store_owners_birth_demand() -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<Self, DagMutation>::source_birth_bytes::<DagSnapshotRetirementFactory, DagOwnedSnapshotRetirementFactory, DagMutationRetirementFactory, ArtifactStoreCursorDisposer<Self, DagMutation>>()? + semio_framework_value::factory_arc_birth_bytes::<ArtifactCanonicalAuthoringFactory<DagSnapshot, DagMutation>>(), depth: 1 })
    }

    fn member_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<Self, DagMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), crate::os_store::DocumentStoreOwnersAdmissionError<Self, DagMutation>> {
        let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<ArtifactCanonicalAuthoringFactory<DagSnapshot, DagMutation>>(), depth: 1 };
        DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (DagSnapshotRetirementFactory, DagOwnedSnapshotRetirementFactory, DagMutationRetirementFactory, ArtifactStoreCursorDisposer::<Self, DagMutation>::new(), Arc::new(ArtifactCanonicalAuthoringFactory::<DagSnapshot, DagMutation>::new()) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DagSnapshot, DagMutation>>))
    }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
