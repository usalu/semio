//! 🪟️ A genuine scalar native App uses the existing Window State, operation receiver and semantic factory.
use semio_framework_os_kernel as store;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as pack;
use semio_framework_os_kernel as dsl;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::*;
use semio_framework_plugin_window_config::*;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::{sync::Arc,mem::size_of};
#[path="../../../../🪟️window/🎚️config/🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🧸️fixture/🦀️.rs"]mod scalar;
pub(crate) use scalar::{State,Mutation,Owner};
semio_framework_os_kernel::original_ordinal_operation_receiving!(Mutation);
impl semio_framework_schema_composition::ArtifactCompositionFields for State{fn visit_child_refs<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,_:&mut V)->Result<(),V::Error>{Ok(())}}
impl store::SemanticMutation<State> for Mutation{
 fn kinds()->&'static[store::SemanticDescriptor]{&[store::SemanticDescriptor{verb:"set",entity:"zoom",kind:"set-zoom",record:"Zoom"}]}
 fn semantics(&self)->&'static store::SemanticDescriptor{&Self::kinds()[0]}
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Set Zoom","Zoom setzen")}
 fn target(&self)->Vec<String>{Vec::new()}
}
/// 🏭️ Three scalar lanes retain the Window owner's actual native semantic catalog.
pub(crate) struct ScalarGenesis<const CONFIG:bool>;
impl<const CONFIG:bool> OriginalStoreGenesisOwner for ScalarGenesis<CONFIG>{
 type State=State;type Mutation=Mutation;type Catalog=WindowConfigInitializationOwners<Owner>;type Initial=FixedStoreInitial<Self>;
 const SCHEMA:&'static str=Owner::SCHEMA;const MAXIMUM_INITIAL_PACK_BYTES:usize=4096;
 fn pack_envelope_id()->Option<&'static str>{Some(<State as pack::snapshot::ArtifactDsl>::envelope_id())}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["","","",label]}
 fn history_shape()->store::HistoryShape{if CONFIG{store::HistoryShape::Config}else{store::HistoryShape::Document}}
 fn initial_state_pack_source(state:&State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Owner::initial_state_pack_source(state)}
}
impl<const CONFIG:bool> OriginalFixedStoreInitial for ScalarGenesis<CONFIG>{type State=State;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Owner::initial_state_demands()}
 fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{Owner::initial_state(grant)}
}
pub(crate) use semio_framework_plugin_interaction::original_genesis::InteractionGenesis;
/// 🧪️ This App's document, config and draft use the original native scalar vocabulary without a cold mutation capability.
#[derive(Default,semio_framework_value::RetireOwned)]
pub(crate) struct NativeWindowApp;
impl ArtifactApp for NativeWindowApp{
 const DIALECT:semio_framework_artifact_reference::Dialect=semio_framework_artifact_reference::Dialect{artifact_kind:"test.window.genesis",standard:semio_framework_artifact_reference::StandardId("1"),subset:semio_framework_artifact_reference::SubsetId::ANY};
 const APP_ID:&'static str="test.window.genesis@1/*#native-run";const DOCUMENT_SCHEMA:&'static str=Owner::SCHEMA;
 type Snapshot=State;type Mutation=Mutation;type Config=State;type ConfigMutation=Mutation;type Draft=State;type DraftMutation=Mutation;type Presence=NoPresence;type PresenceMutation=NoPresenceMutation;type Transient=NoTransient;type TransientMutation=NoTransientMutation;type Command=NoConfigMutation;
 
 async fn handle(command:&Self::Command,_:&ArtifactView<'_,State>,_:&ConfigView<'_,State>,_:&InteractionView<'_>,_:Option<&semio_framework::ViewModel>,_:&DraftView<'_,State>,_:&semio_framework_2d_compute::EngineHandles)->semio_framework_plugin_decl_emission::ArtifactMutationOutcome<Mutation,Mutation,Mutation>{match *command{}}
 async fn render(_:&str,_:&ArtifactView<'_,State>,_:&ConfigView<'_,State>,_:&semio_framework::ViewModel)->semio_framework_plugin_interaction::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree>{Err(semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError::new("native-window-fixture","fixture declares its borrowed scalar through its original ToolRun presentation"))}
 fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{<WindowConfigInitializationOwners<Owner> as store::os_store::document_store_owners::DocumentStoreOwnerProducer<State,Mutation>>::source_demands()}
 fn build_document_store_owners(grant:RetainedCloneGrant)->Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<State,Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<State,Mutation>>>{Some(Owner::build_store_owners(grant))}
 fn build_artifact_store_one_item_preparation_factory()->Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<State,Mutation>>>{Some(Owner::build_one_item_preparation_factory())}
}
impl super::original_instance_stores::OriginalArtifactAppBirth for NativeWindowApp{type DocumentGenesis=ScalarGenesis<false>;type ConfigGenesis=ScalarGenesis<true>;type DraftGenesis=ScalarGenesis<false>;type InteractionGenesis=InteractionGenesis;fn controller_birth_demands(_:&str)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}fn admit_controller(_:&str,_:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{Ok((Self,RetainedCloneProgress{copied_items:1,..Default::default()}))}fn controller_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}fn controller_close_step(&mut self,_:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{Ok(true)}fn controller_terminal_is_empty(&self)->bool{true}}
