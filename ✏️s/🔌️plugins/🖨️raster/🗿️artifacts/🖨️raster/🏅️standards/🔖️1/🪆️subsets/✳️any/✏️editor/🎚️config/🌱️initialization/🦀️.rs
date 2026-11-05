//! 🌱️ Raster configuration declares its original default, packed coverage receiver and shared replay preparation.
use super::{RasterConfig as State,RasterConfigMutation as Mutation};
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::RetainedCloneBirthDemand};
use store::os_store::document_store_owners::{DocumentStoreOwnerProducer,DocumentStoreOwners,DocumentStoreOwnersAdmissionError,BoundedArtifactRetirementFactory};
use std::{mem::size_of,sync::Arc};

type Owners=DocumentStoreOwners<State,Mutation>;
type StateRetirement=BoundedArtifactRetirementFactory<State>;
type MutationRetirement=BoundedArtifactRetirementFactory<Mutation>;
type Disposer=store::os_store::ArtifactStoreCursorDisposer<State,Mutation>;
type Edit=store::snapshot_clone_preparation::ConfigApplyEdit<State,Mutation>;
pub type RasterConfigPreparationFactory=store::snapshot_clone_preparation::RetainedClonePreparationFactory<State,Mutation,Edit>;
fn children()->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<Edit>()+semio_framework_value::factory_arc_birth_bytes::<StateRetirement>()+semio_framework_value::factory_arc_birth_bytes::<MutationRetirement>(),depth:1}}
fn source_copy()->usize{2*size_of::<Disposer>()+2*size_of::<Owners>()+2*size_of::<RasterConfigPreparationFactory>()+size_of::<Arc<Edit>>()+size_of::<Arc<StateRetirement>>()+size_of::<Arc<MutationRetirement>>()}

pub struct RasterConfigCatalog;
impl DocumentStoreOwnerProducer<State,Mutation> for RasterConfigCatalog{
    fn source_demands()->Result<RetirementDemand,ValueError>{Owners::shared_semantic_source_demands::<StateRetirement,StateRetirement,MutationRetirement,Disposer,RasterConfigPreparationFactory>(children(),source_copy())}
    fn admit_sources(grant:RetainedCloneGrant)->Result<(Owners,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<State,Mutation>>{
        Owners::admit_source_constructor_with_shared_semantic_preparation(grant,children(),source_copy(),||{
            let preparation=RasterConfigPreparationFactory::new(Arc::new(Edit::new()),Arc::new(MutationRetirement::new()),Arc::new(StateRetirement::new()),store::snapshot_clone_preparation::CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH).expect("authored nonzero Raster config preparation depth");
            (StateRetirement::new(),StateRetirement::new(),MutationRetirement::new(),Disposer::new(),preparation)
        }).map(|(mut owners,progress)|{owners.operation_wire=semio_framework_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(|operation|Some(operation)));(owners.with_batch_digest(),progress)})
    }
}

pub struct RasterConfigGenesis;
impl OriginalStoreGenesisOwner for RasterConfigGenesis{
    type State=State;type Mutation=Mutation;type Catalog=RasterConfigCatalog;type Initial=FixedStoreInitial<Self>;
    const SCHEMA:&'static str="raster.config";
    const MAXIMUM_INITIAL_PACK_BYTES:usize=4096;
    fn pack_envelope_id()->Option<&'static str>{Some(<State as pack::snapshot::ArtifactDsl>::envelope_id())}
    fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["","","",label]}
    fn history_shape()->store::HistoryShape{store::HistoryShape::Config}
    fn initial_state_pack_source(state:&State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Ok((state,<State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
}
impl OriginalFixedStoreInitial for RasterConfigGenesis{
    type State=State;
    fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<State>()+19,capacity_bytes:19,depth:1,..Default::default()})}
    fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{let demand=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Raster config default is unfunded"));}Ok((State::default(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}
}

fn target(operation:&Mutation)->&'static str{match operation{Mutation::SetBrushSize(_)=>"brush-size",Mutation::SetBrushOpacity(_)=>"brush-opacity",Mutation::SetBrushColor(_)=>"brush-color",Mutation::SetBrushHardness(_)=>"brush-hardness",Mutation::SetPaintTarget(_)=>"paint-target",Mutation::SetMaskValue(_)=>"mask-value",Mutation::SetPixelSelection(_)=>"pixel-selection",Mutation::SetCompositeViewport(_)=>"composite-viewport",Mutation::SetCamera(_)=>"camera",Mutation::SetFillTolerance(_)=>"fill-tolerance",Mutation::SetFilterOptions(_)=>"filter-options",Mutation::SetImageGrid(_)=>"image-grid"}}
impl semio_framework_os_store_operation_wire::ArtifactOperationTargets for Mutation{
    fn len(&self)->usize{1}
    fn segment_len(&self,index:usize)->Result<usize,ValueError>{if index!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Raster config target ordinal is absent"));}Ok(target(self).len())}
    fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let length=semio_framework_os_store_operation_wire::ArtifactOperationTargets::segment_len(self,index)?;if offset>length{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Raster config target offset is absent"));}let bytes=&target(self).as_bytes()[offset..];let count=bytes.len().min(output.len());output[..count].copy_from_slice(&bytes[..count]);Ok(count)}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
