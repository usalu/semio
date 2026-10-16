//! 📥️ Trait impls of `🏪️store` whose trait or type belongs to `os-store-one-item-publication` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🦀️.rs`.
use std::sync::Arc;
use crate::ArtifactStoreBatchItemAuthority;
use semio_framework_os_store_one_item::ArtifactStoreOneItemFootprint;
use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;
use crate::ArtifactStoreOneItemPreparation;
use crate::ArtifactStoreOneItemPreparationRequest;
use crate::ArtifactStoreWireRetirement;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_command::HistoryLane;
use crate::lifted_publication::MemberStoreOneItemWire;
use crate::lifted_publication::MemberStoreOneItemWirePreparationFactory;
use semio_framework_os_spr::Mutation;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::ValueError;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;


impl<P: Send + Sync, Mutation: Send + 'static> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>> {
    type Input = MemberStoreOneItemWire;

    fn input_retirement_birth_bytes(&self, _input: &MemberStoreOneItemWire) -> usize { size_of::<ArtifactStoreWireRetirement>() }
    fn retire_input(&self, input: MemberStoreOneItemWire, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, MemberStoreOneItemWire)> { admit_artifact_retirement(input, grant, |input| ArtifactStoreWireRetirement { schema: Some(input.schema), bytes: Some(input.bytes) }) }
    fn into_retirement_authorities(self) -> [Option<semio_framework_value::FactoryAuthority>; 2] { let authority: Arc<dyn semio_framework_value::FactoryRetirement> = self; [Some(semio_framework_value::FactoryAuthority::new(authority)), None] }
    fn preflight(&self, input: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.as_ref().preflight(input, lane)
    }

    fn begin_demand(&self, input: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> { self.as_ref().begin_demand(input, lane) }
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>)> {
        self.as_ref().begin(request, grant)
    }
}
