//! 🌱️ Energy document genesis retains its authored native fields and actual semantic Store authorities.
use semio_framework_os_kernel as store;
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,retained_clone::RetainedCloneBirthDemand};
use semio_s_artifact_energy_model_snapshot::EnergyModelSnapshot;
use semio_s_artifact_energy_model_mutations::EnergyModelMutation;
use std::mem::size_of;

/// 🔋️ Publication and replay share the genuine retained Energy mutation producer.
pub type EnergyModelStorePreparationFactory=store::PagedOneItemPreparationFactory<EnergyModelSnapshot,EnergyModelMutation,store::MutationApplyEdit<EnergyModelSnapshot,EnergyModelMutation>>;

#[path="🌱️state/🦀️.rs"]
mod initial_source;
pub use initial_source::EnergyNativeInitialSource;

type Owners=store::os_store::document_store_owners::DocumentStoreOwners<EnergyModelSnapshot,EnergyModelMutation>;
type SnapshotRetirement=store::os_store::document_store_owners::BoundedArtifactRetirementFactory<EnergyModelSnapshot>;
type MutationRetirement=store::os_store::document_store_owners::BoundedArtifactRetirementFactory<EnergyModelMutation>;
type Disposer=store::os_store::ArtifactStoreCursorDisposer<EnergyModelSnapshot,EnergyModelMutation>;
fn source_copy()->usize{2*size_of::<Disposer>()+2*size_of::<Owners>()}
/// 🏭️ Original document construction installs one semantic issuer for publication and replay.
pub struct EnergyModelCatalog;
impl store::os_store::document_store_owners::DocumentStoreOwnerProducer<EnergyModelSnapshot,EnergyModelMutation> for EnergyModelCatalog{
 fn source_demands()->Result<RetirementDemand,ValueError>{Owners::shared_semantic_source_demands::<SnapshotRetirement,SnapshotRetirement,MutationRetirement,Disposer,EnergyModelStorePreparationFactory>(RetainedCloneBirthDemand{capacity_bytes:0,depth:0},source_copy())}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(Owners,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<EnergyModelSnapshot,EnergyModelMutation>>{Owners::admit_source_constructor_with_shared_semantic_preparation(grant,RetainedCloneBirthDemand{capacity_bytes:0,depth:0},source_copy(),||(SnapshotRetirement::new(),SnapshotRetirement::new(),MutationRetirement::new(),Disposer::new(),EnergyModelStorePreparationFactory::default())).map(|(mut owners,p)|{owners.operation_wire=semio_framework_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(|operation|Some(operation)));(owners.with_batch_digest(),p)})}
}
/// 🧬️ The native document supplies its intrinsic Pack source directly from its sole persistent model.
pub struct EnergyModelGenesis;
impl OriginalStoreGenesisOwner for EnergyModelGenesis{
 type State=EnergyModelSnapshot;type Mutation=EnergyModelMutation;type Catalog=EnergyModelCatalog;type Initial=EnergyNativeInitialSource;
 const SCHEMA:&'static str=semio_s_artifact_energy_model_snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA;
 const MAXIMUM_INITIAL_PACK_BYTES:usize=65536;
 fn artifact_dialect()->Option<semio_framework_artifact_reference::Dialect>{Some(semio_s_artifact_energy_model_snapshot::MODEL_DIALECT)}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["","","",label]}
 fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Ok(semio_s_artifact_energy_model_snapshot::original::pack::source(state))}
}

#[cfg(test)]#[path="🧪️tests/🏪️store.rs"]mod tests;

#[cfg(test)]#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
