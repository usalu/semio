//! 🌱️ Config creation declares its authored default, original Pack receiver and shared publication/replay issuer.
use super::{EnergyModelConfig as State,EnergyModelConfigMutation as Mutation};
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::RetainedCloneBirthDemand};
use semio_framework_os_store_codec_registry::native_snapshot::{OriginalArtifactPackReceiving,OriginalSnapshotRecordReceiving};
use std::{mem::size_of,sync::Arc};
use store::os_store::document_store_owners::{DocumentStoreOwnerProducer,DocumentStoreOwners,DocumentStoreOwnersAdmissionError,BoundedArtifactRetirementFactory};
type Owners=DocumentStoreOwners<State,Mutation>;
type StateRetirement=BoundedArtifactRetirementFactory<State>;
type MutationRetirement=BoundedArtifactRetirementFactory<Mutation>;
type Disposer=store::os_store::ArtifactStoreCursorDisposer<State,Mutation>;
type Edit=store::snapshot_clone_preparation::ConfigApplyEdit<State,Mutation>;
/// 🎚️ Both config semantic ports share the same genuine retained edit and captured retirement authorities.
pub type EnergyConfigPreparationFactory=store::snapshot_clone_preparation::RetainedClonePreparationFactory<State,Mutation,Edit>;
fn children()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<Edit>()+semio_framework_value::factory_arc_birth_bytes::<StateRetirement>()+semio_framework_value::factory_arc_birth_bytes::<MutationRetirement>(),depth:1}}
fn source_copy()->usize{2*size_of::<Disposer>()+2*size_of::<Owners>()+2*size_of::<EnergyConfigPreparationFactory>()+size_of::<Arc<Edit>>()+size_of::<Arc<StateRetirement>>()+size_of::<Arc<MutationRetirement>>()}
/// 🏭️ Original Config initialization births its actual semantic and retirement sources together.
pub struct EnergyConfigCatalog;
impl DocumentStoreOwnerProducer<State,Mutation> for EnergyConfigCatalog{
 fn source_demands()->Result<RetirementDemand,ValueError>{Owners::shared_semantic_source_demands::<StateRetirement,StateRetirement,MutationRetirement,Disposer,EnergyConfigPreparationFactory>(children(),source_copy())}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(Owners,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<State,Mutation>>{Owners::admit_source_constructor_with_shared_semantic_preparation(grant,children(),source_copy(),||{let preparation=EnergyConfigPreparationFactory::new(Arc::new(Edit::new()),Arc::new(MutationRetirement::new()),Arc::new(StateRetirement::new()),store::snapshot_clone_preparation::CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH).expect("authored nonzero config preparation depth");(StateRetirement::new(),StateRetirement::new(),MutationRetirement::new(),Disposer::new(),preparation)}).map(|(mut owners,p)|{owners.operation_wire=semio_framework_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(|operation|Some(operation)));(owners.with_batch_digest(),p)})}
}
/// 🌱️ The four authored settings have a finite original String birth and canonical envelope identity.
pub struct EnergyConfigGenesis;
impl OriginalStoreGenesisOwner for EnergyConfigGenesis{
 type State=State;type Mutation=Mutation;type Catalog=EnergyConfigCatalog;type Initial=FixedStoreInitial<Self>;
 const SCHEMA:&'static str="energy.model.config";const MAXIMUM_INITIAL_PACK_BYTES:usize=4096;
 fn pack_envelope_id()->Option<&'static str>{Some(<State as pack::snapshot::ArtifactDsl>::envelope_id())}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["","","",label]}
 fn history_shape()->store::HistoryShape{store::HistoryShape::Config}
 fn initial_state_pack_source(state:&State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Ok((state,<State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
}
impl OriginalFixedStoreInitial for EnergyConfigGenesis{
 type State=State;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<State>()+super::DEFAULT_RESULT_FIELD.len(),capacity_bytes:super::DEFAULT_RESULT_FIELD.len(),depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Energy config default is unfunded"))}Ok((State::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
}
impl OriginalArtifactPackReceiving for State{
 type Receiving=OriginalSnapshotRecordReceiving<Self>;
 fn original_pack_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self::Receiving>(),depth:1,..Default::default()}}
 fn admit_original_pack(source:&[u8],limits:protocol::PackLimits,native:&semio_framework_value::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self::Receiving,RetainedCloneProgress)>,ValueError>{Self::Receiving::admit(source,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),limits,native,grant)}
}
impl semio_framework_os_store_operation_wire::ArtifactOperationTargets for Mutation{
 fn len(&self)->usize{1}
 fn segment_len(&self,index:usize)->Result<usize,ValueError>{if index!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"config target ordinal is absent"))}Ok(match self{Self::ChangeSimulationSettings(_)=>19,Self::ChangeResultField(_)=>12})}
 fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let len=semio_framework_os_store_operation_wire::ArtifactOperationTargets::segment_len(self,index)?;let text=match self{Self::ChangeSimulationSettings(_)=>"simulation-settings",Self::ChangeResultField(_)=>"result-field"};if offset>len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"config target offset is absent"))}let bytes=&text.as_bytes()[offset..];let count=bytes.len().min(output.len());output[..count].copy_from_slice(&bytes[..count]);Ok(count)}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
