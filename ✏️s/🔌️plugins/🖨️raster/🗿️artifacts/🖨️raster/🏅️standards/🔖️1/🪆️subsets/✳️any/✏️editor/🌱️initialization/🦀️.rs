//! 🌱️ Raster's original empty document, native Pack projection and genuine semantic Store catalog.
#[path="🎟️registration/🦀️.rs"]
pub mod registration;
pub use registration::{RasterNativeRegistration,RasterInstanceDeclarations};
use crate::{RasterPlayApp,config::initialization::RasterConfigGenesis,store_preparation::RasterStorePreparationFactory};
use semio_s_artifact_raster_raster_model::{snapshot::RasterSnapshot as State,RASTER_DOCUMENT_SCHEMA,RASTER_DIALECT};
use semio_s_artifact_raster_raster_mutations::RasterMutation as Mutation;
use semio_s_artifact_raster_raster_host::{RasterSnapshotRetirementFactory,RasterMutationRetirementFactory};
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::RetainedCloneBirthDemand};
use semio_framework_dsl_record::{BorrowedDslRecord,BorrowedRecordSpec,native_encoding::{FieldProjectionSource,FieldProjectionView}};
use std::mem::size_of;
use store::os_store::document_store_owners::{DocumentStoreOwnerProducer,DocumentStoreOwners,DocumentStoreOwnersAdmissionError};

type Owners=DocumentStoreOwners<State,Mutation>;
type Disposer=store::ArtifactStoreCursorDisposer<State,Mutation>;
type NativeRecord=semio_s_artifact_raster_raster_model::io::text::snapshot::record::RasterNativeDocument;
const IDS:[u16;8]=[NativeRecord::RECORD.fields[0].id,NativeRecord::RECORD.fields[1].id,NativeRecord::RECORD.fields[2].id,NativeRecord::RECORD.fields[3].id,NativeRecord::RECORD.fields[4].id,NativeRecord::RECORD.fields[5].id,NativeRecord::RECORD.fields[6].id,NativeRecord::RECORD.fields[7].id];
struct InitialRecord;
static INITIAL_RECORD:InitialRecord=InitialRecord;
impl FieldProjectionSource for InitialRecord{
 fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{Ok(match path{[]=>FieldProjectionView::Record(&IDS),[0]=>FieldProjectionView::Text(RASTER_DOCUMENT_SCHEMA),[1]=>FieldProjectionView::Text("raster"),[2]=>FieldProjectionView::Text("Untitled"),[3..=7]=>FieldProjectionView::List(0),_=>return Err(semio_framework_dsl_record::native_encoding::projection_path_error())})}
}

pub struct RasterDocumentCatalog;
impl DocumentStoreOwnerProducer<State,Mutation> for RasterDocumentCatalog{
 fn source_demands()->Result<RetirementDemand,ValueError>{Owners::shared_semantic_source_demands::<RasterSnapshotRetirementFactory,RasterSnapshotRetirementFactory,RasterMutationRetirementFactory,Disposer,RasterStorePreparationFactory>(RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0)}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(Owners,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<State,Mutation>>{Owners::admit_source_constructor_with_shared_semantic_preparation(grant,RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0,||(RasterSnapshotRetirementFactory,RasterSnapshotRetirementFactory,RasterMutationRetirementFactory,Disposer::new(),RasterStorePreparationFactory)).map(|(mut owners,progress)|{owners.operation_wire=semio_framework_os_store_operation_wire::original_edit::with_original_edit_message(store::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(|operation|Some(operation)));(owners.with_batch_digest(),progress)})}
}

pub struct RasterDocumentGenesis;
impl OriginalStoreGenesisOwner for RasterDocumentGenesis{
 type State=State;type Mutation=Mutation;type Catalog=RasterDocumentCatalog;type Initial=FixedStoreInitial<Self>;
 const SCHEMA:&'static str=RASTER_DOCUMENT_SCHEMA;
 const MAXIMUM_INITIAL_PACK_BYTES:usize=4096;
 fn pack_envelope_id()->Option<&'static str>{Some(<State as store::snapshot::ArtifactDsl>::envelope_id())}
 fn artifact_dialect()->Option<semio_framework_artifact_reference::Dialect>{Some(RASTER_DIALECT)}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{[RASTER_DIALECT.artifact_kind,RASTER_DIALECT.standard.0,RASTER_DIALECT.subset.0,label]}
 fn initial_state_pack_source(state:&State)->Result<(&dyn FieldProjectionSource,BorrowedRecordSpec),ValueError>{if state.schema!=RASTER_DOCUMENT_SCHEMA||state.id!="raster"||state.title.as_deref()!=Some("Untitled")||!state.layers.is_empty()||!state.assets.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Raster original genesis differs from its authored empty Pack source"))}Ok((&INITIAL_RECORD,NativeRecord::RECORD))}
}
impl OriginalFixedStoreInitial for RasterDocumentGenesis{
 type State=State;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{let bytes=RASTER_DOCUMENT_SCHEMA.len()+"raster".len()+"Untitled".len();Ok(RetirementDemand{copy_bytes:2*size_of::<State>()+bytes,capacity_bytes:bytes,depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Raster authored initial state is unfunded"))}let state=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();let capacity=state.schema.capacity()+state.id.capacity()+state.title.as_ref().map_or(0,String::capacity);Ok((state,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))}
}
impl semio_framework_plugin::OriginalArtifactEditorBirth for RasterPlayApp{
 type DocumentGenesis=RasterDocumentGenesis;type ConfigGenesis=RasterConfigGenesis;type DraftGenesis=semio_framework_plugin::NoDraftGenesisOwner;
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
