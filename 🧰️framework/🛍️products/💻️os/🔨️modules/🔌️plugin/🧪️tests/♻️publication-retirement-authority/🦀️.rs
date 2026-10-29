//! ♻️ Every typed-operation publication lane retires a REJECTED authority without faulting per turn.
//!
//! Regression cover for ticket 26/09/09/PROCEDURAL-3D-END-TO-END
//! (`📓️invoke-extension-rejected-authority-2026-09-11.md`): all seven lanes used to answer every
//! incomplete `close_step` of a faulted `Closing` publication with
//! `<lane> publication is retiring a rejected authority`, so the host saw
//! `typed-operation failed: …` on every turn of the drain and `invokeExtension` never completed.
#[cfg(test)]
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;

use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView};
use semio_framework_plugin_composition_publication::PendingArtifactStorePublication;
use semio_framework_plugin_retained_command::typed_operation::PendingPublicationOutcome;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_window_config::{WindowConfigMutation, WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_framework_plugin_window_transient::window_transient::{WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientOwnerRegistry};
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_testing_laws::artifact_app_laws::close_registered_fixture_app;
use semio_framework_plugin_ownership::store_lifecycle::{bounded_config_store_disposer, bounded_config_store_one_item_preparation_factory, bounded_document_store_disposer};
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::change_publication_transient::ChangePublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;
use semio_framework_os_kernel as store;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework::{Fault, ViewModel, ViewWindowInstance};
use semio_framework_2d_compute::EngineHandles;
use std::sync::Arc;


#[test]
fn mounted_original_close_fault_retains_refused_payload_and_retires_actual_frames(){
 use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep};
 use semio_framework_diagnostic::{FaultOrigin,FaultScope,FaultCause,FaultCode};
 use semio_framework_trace::observe_heap_allocations_on_this_thread;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧵️retained-command/🪟️mounted/♻️frontier/⚠️fault/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let origin=if row["origin"]=="app"{FaultOrigin::App}else{FaultOrigin::Framework};
  let(original,birth)=observe_heap_allocations_on_this_thread(||{
   let mut fault=Fault::new(origin,row["code"].as_str().unwrap().to_owned(),row["message"].as_str().unwrap()).with_param("route","drawingClipboard");
   *fault.scope=FaultScope{plugin_id:Some("draw".into()),app_id:Some("drawing".into()),instance_id:Some("7".into()),module:Some("clipboard".into()),body_key:Some("canvas".into())};
   fault.causes=vec![FaultCause{message:"original nested failure".into(),code:Some(FaultCode::new("drawing.original-child"))}];fault
  });
  let pointer=original.message.as_ptr();let original_live=birth.requested_bytes-birth.released_bytes;
  let operation=semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(),semio_framework_job::RevisionId(0),semio_framework_job::Generation(0),17);
  let mut mounted=semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation::<RetirementApp>{
   verb:String::new(),meta:semio_framework_plugin_decl_emission::ActionMeta{actor:Default::default(),instance_id:7,view_state:None},operation,canonical_revision:[0;32],artifact_generation:0,config_generation:0,draft_generation:0,presence_generation:0,transient_generation:0,
   window_config_authority:None,window_transient_authority:None,publication_lanes:&[],contract:Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),session:None,session_rejected:None,producer_source:None,producer_request:None,producer_request_retirement:None,reserved_producer:None,completion:None,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:None,cancellation_retirement:None,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},worker_resume_pending:false,terminal_seen:true,publication:None,tool_stage:semio_framework_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Capture,tool_press_result:None,pending_artifact_publication:None,pending_stream_abort:None,stream_abort_completed:false,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,pending_child_publication:None,owned_child_group:None,owned_child_committed:false,owned_child_result_pending:false,captured_child_content:None,captured_child_content_generation:0,result_page:None, diagnostics:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_ownership_progress:None,original_retirement_receipt:None,actor_capture:None,publication_attempt:0,ui_pending:false,progress:None,progress_pending:false,user_cancel_requested:false,published_artifact:false,published_config:false,published_window_config:false,command_logged:true,interaction_revalidated:false,
   terminal_fault:row["existingCode"].as_str().map(|code|semio_framework_plugin_decl_app::borrowed_report(FaultOrigin::Framework,code,semio_framework_diagnostic::Severity::Error,"earlier original terminal report",false)),retained_close_fault:Some(original),retained_close_fault_retirement:None,retained_close_fault_refusal:None,stage:semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage::Retiring
  };
  let mut capacity=0;let mut released=0;let mut frame_released=false;
  for _ in 0..32768{
   if !mounted.close_fault_pending(){break;}
   let demand=mounted.close_fault_demands(32).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(32),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for axis in 0..5{
    let mut denied=grant;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4 if demand.depth>0=>denied.maximum_depth-=1,_=>continue};
    let(step,heap)=observe_heap_allocations_on_this_thread(||mounted.close_fault_step(denied).unwrap());assert_eq!(heap,Default::default());assert_eq!(step.progress(),Default::default());assert!(mounted.close_fault_pending());assert!(!mounted.terminal_is_empty());if let Some(fault)=mounted.retained_close_fault.as_ref(){assert_eq!(fault.message.as_ptr(),pointer);}
   }
   let terminal_frame=mounted.retained_close_fault_retirement.as_ref().is_some_and(|owner|owner.terminal_is_empty());
   let(step,heap)=observe_heap_allocations_on_this_thread(||mounted.close_fault_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(heap.released_bytes,step.progress().released_bytes);capacity+=heap.requested_bytes;released+=heap.released_bytes;
   if terminal_frame{assert!(matches!(step,RetainedCloneStep::Progress(_)));assert_eq!(heap.released_bytes,demand.release_bytes);frame_released=true;}
  }
  assert!(frame_released&&mounted.terminal_is_empty());assert_eq!(released,original_live+capacity);let report=mounted.terminal_fault.as_ref().unwrap();let mut frame=[0;480];let len=report.framed_page_bytes(&mut frame);let independent:serde_json::Value=serde_json::from_slice(&frame[..len]).unwrap();assert_eq!(independent["code"],row["existingCode"].as_str().unwrap_or(row["code"].as_str().unwrap()));if row["existingCode"].is_null(){assert_eq!(independent["message"],row["message"]);}
 }
}

const RETIREMENT_AUTHORITY_FIXTURE_JSON: &str = include_str!("../🧫️fixtures/♻️publication-retirement-authority/🔣️.json");

/// 🪟️ The one window kind both window lanes of this fixture partition under.
const RETIREMENT_WINDOW_KIND: &str = "publication-retirement-window";

#[doc(hidden)]
pub fn fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(RETIREMENT_AUTHORITY_FIXTURE_JSON).expect("publication retirement authority fixture parses");
    assert_eq!(fixture["schema"], "framework.plugin.publication-retirement-authority.v1");
    fixture
}

#[doc(hidden)]
pub fn grant(fixture: &serde_json::Value) -> store::os_store::one_item::ArtifactStoreOneItemGrant {
    let bytes=fixture["grant"]["maximumBytes"].as_u64().expect("fixture grant bytes")as usize;
    store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:fixture["grant"]["maximumItems"].as_u64().expect("fixture grant items")as usize,maximum_copy_bytes:bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:bytes,maximum_depth:128}
}

//#region ♻️RetirementFixtureLeaves
fn presence_footprint(_: &PublicationPresenceMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<PublicationPresenceMutation>()))
}

fn presence_transfer(mutation: PublicationPresenceMutation) -> PublicationPresence {
    let PublicationPresenceMutation::ChangePublicationPresence(value) = mutation;
    PublicationPresence { revision: value.revision }
}

fn presence_preparation_factory() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<PublicationPresence, PublicationPresenceMutation>> {
    Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
        presence_footprint,
        presence_transfer,
        Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<PublicationPresence>::default()),
        Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<PublicationPresenceMutation>::default()),
    ))
}

fn transient_footprint(_: &PublicationTransientMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(size_of::<PublicationTransientMutation>()))
}

fn transient_transfer(mutation: PublicationTransientMutation) -> PublicationTransient {
    let PublicationTransientMutation::ChangePublicationTransient(value) = mutation;
    PublicationTransient { revision: value.revision }
}

fn transient_preparation_factory() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<PublicationTransient, PublicationTransientMutation>> {
    Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
        transient_footprint,
        transient_transfer,
        Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<PublicationTransient>::default()),
        Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<PublicationTransientMutation>::default()),
    ))
}

struct RetirementWindowConfigOwner;

impl WindowConfigOwner for RetirementWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = RETIREMENT_WINDOW_KIND;
    const SCHEMA: &'static str = "plugin.testkit.publication-retirement-window-config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = TestConfig;
    type Mutation = TestConfigMutation;
    type Edit=semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::preparation::SelectionRetainedEdit;
    const MAXIMUM_PREPARATION_DEPTH:usize=64;
    fn build_retained_edit()->std::sync::Arc<Self::Edit>{std::sync::Arc::new(semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::preparation::SelectionRetainedEdit)}

    fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_plugin_window_config::bounded_window_config_preparation_source_birth::<Self>()}
    fn build_store_owners(grant:semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,semio_framework_value::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>> {
        semio_framework_plugin_window_config::bounded_window_config_store_owners::<Self>(grant)
    }

    fn build_one_item_preparation_factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin_window_config::bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin_window_config::bounded_window_config_store_disposer::<Self>()
    }
}

struct RetirementWindowTransientOwner;

impl WindowTransientOwner for RetirementWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = RETIREMENT_WINDOW_KIND;
    type State = PublicationTransient;
    type Mutation = PublicationTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        semio_framework_plugin_testing_fixtures_publication::window_transient_owners::owners()
    }
}
//#endregion ♻️RetirementFixtureLeaves

//#region ♻️RetirementFixtureApp
/// 🧪️ The one fixture app that owns a REAL store on every publication lane at once, so the shared
/// retirement law can be driven against seven concrete publication types rather than a stand-in.
#[derive(Default)]
struct RetirementApp;

impl ArtifactApp for RetirementApp {
    const DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.test.publication-retirement", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };
    const APP_ID: &'static str = "testkit-publication-retirement";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-publication-retirement/v1";
    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = TestConfig;
    type DraftMutation = TestConfigMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = PublicationTransientMutation;

    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<RetirementWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<RetirementWindowTransientOwner>()
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("retirement-doc", 1_024))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("retirement-cfg", 1_024))
    }

    fn build_draft_store_one_item_preparation_factory() -> Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Draft, Self::DraftMutation>("retirement-draft", 1_024))
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(presence_preparation_factory())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(transient_preparation_factory())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_store_disposer::<Self::Presence,Self::PresenceMutation>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin_ownership::transient_publication::bounded_transient_store_disposer::<Self::Transient,Self::TransientMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<PublicationPresence>::default()))
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<PublicationPresence>::default()))
    }

    fn build_transient_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<PublicationTransient>::default()))
    }

    fn reference_birth()->Option<fn()->Result<TestSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(TestSnapshot::default())
    })}

    async fn handle(
        _command: &PublicationTransientMutation,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, TestConfig>,
        _engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, TestConfigMutation> {
        Ok(Default::default())
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}
//#endregion ♻️RetirementFixtureApp

//#region ♻️RetirementDrivers
/// 🔭️ Drives ONE rejected publication through the shared retirement law and reports how many
/// incomplete turns it answered with `Ok` before its terminal turn.
fn mounted_rejection(app:&semio_framework_plugin_instance_app::VcsArtifactApp<RetirementApp>,pending:PendingArtifactStorePublication<RetirementApp>)->semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation<RetirementApp>{
 let revision=app.stores.store.content_revision_now();
 let operation=semio_framework_job::Operation::new(semio_framework_job::OperationId(1),semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),semio_framework_job::Generation(app.stores.store.generation_now()),17);
 let lease=app.tools.tool_cancellations.clone().begin(semio_framework_plugin_operation::ToolOperationKey{app_instance_id:7,document:semio_framework_plugin_operation::ArtifactDocumentAuthority(7),operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation}).expect("original cancellation lease");
 semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation{
  verb:String::new(),meta:semio_framework_plugin_decl_emission::ActionMeta{actor:Default::default(),instance_id:7,view_state:None},operation,canonical_revision:revision,artifact_generation:app.stores.store.generation_now(),config_generation:app.stores.config_store.generation_now(),draft_generation:app.stores.draft_store.generation_now(),presence_generation:app.stores.presence_store.generation_now(),transient_generation:app.stores.transient_store.generation_now(),
  window_config_authority:None,window_transient_authority:None,publication_lanes:&[],contract:Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),session:None,session_rejected:None,producer_source:None,producer_request:None,producer_request_retirement:None,reserved_producer:None,completion:None,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:Some(lease),worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},worker_resume_pending:false,terminal_seen:true,publication:None,pending_artifact_publication:Some(pending),pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,pending_child_publication:None,owned_child_group:None,owned_child_committed:false,owned_child_result_pending:false,captured_child_content:None,captured_child_content_generation:0,result_page:None, diagnostics:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_ownership_progress:None,original_retirement_receipt:None,actor_capture:None,publication_attempt:0,ui_pending:false,progress:None,progress_pending:false,user_cancel_requested:false,published_artifact:false,published_config:false,published_window_config:false,command_logged:true,interaction_revalidated:false,retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault:None,stage:semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage::Publishing
 }
}
#[doc(hidden)]
pub fn granted(demand:semio_framework_value::RetirementDemand)->semio_framework_value::retained_clone::RetainedCloneGrant{
 semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
async fn drive_rejected_retirement(app:&mut semio_framework_plugin_instance_app::VcsArtifactApp<RetirementApp>,pending:PendingArtifactStorePublication<RetirementApp>,row:&serde_json::Value)->usize{
 let lane=pending.lane().0;
 let reason=row["supersededFault"].as_str().unwrap();
 assert!(pending.is_closing());
 assert_eq!(pending.fault(),Some(reason));
 let generations=(app.stores.store.generation_now(),app.stores.config_store.generation_now(),app.stores.draft_store.generation_now(),app.stores.presence_store.generation_now(),app.stores.transient_store.generation_now());
 let mut mounted=mounted_rejection(app,pending);
 let mut turns=0;
 for _ in 0..8192{
  app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).expect("publication pauses until retained outcome delivery");
  if !mounted.pending_publication_outcome.pending()&&mounted.pending_artifact_publication.is_none(){break;}
  assert!(mounted.result_page.is_none(),"no premature terminal while original publication body or frame remains");
  let demand=mounted.granted_retirement_demands(512).expect("actual frontier quote");
  let grant=granted(demand);
  let before=(mounted.pending_publication_outcome.phase,mounted.pending_artifact_publication.is_some(),mounted.terminal_fault.is_some());
  let step=mounted.granted_retirement_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap();
  assert_eq!(step.progress(),Default::default());
  assert!(before==(mounted.pending_publication_outcome.phase,mounted.pending_artifact_publication.is_some(),mounted.terminal_fault.is_some()),"denied grant preserves publication custody and report");
  if demand.copy_bytes!=0{
   let step=mounted.granted_retirement_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}).unwrap();
   assert_eq!(step.progress(),Default::default());
   assert!(before==(mounted.pending_publication_outcome.phase,mounted.pending_artifact_publication.is_some(),mounted.terminal_fault.is_some()),"denied grant preserves publication custody and report");
  }
  let step=mounted.granted_retirement_step(grant).expect("actual four-currency maintenance frontier");
  assert!(step.progress().copied_bytes<=grant.maximum_copy_bytes&&step.progress().retained_capacity_bytes<=grant.maximum_capacity_bytes&&step.progress().released_bytes<=grant.maximum_release_bytes);
  turns+=1;
 }
 assert!(mounted.pending_artifact_publication.is_none()&&!mounted.pending_publication_outcome.pending(),"original body and frame close before delivery");
 assert_eq!(generations,(app.stores.store.generation_now(),app.stores.config_store.generation_now(),app.stores.draft_store.generation_now(),app.stores.presence_store.generation_now(),app.stores.transient_store.generation_now()),"rejected private candidate never commits");
 if lane==semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane::WindowTransient{
  assert!(mounted.terminal_fault.is_none()&&mounted.result_page.is_none(),"window transient retains its deliberate lenient policy");
  mounted.stage=semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage::Retiring;
 }else{
  app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).expect("one final fault after physical retirement");
  let mut page=mounted.take_fixture_result_page(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).expect("single final fault");
  assert_eq!(page.lane,semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane::Fault);
  let fault=semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page(page.bytes());
  assert_eq!(fault.code.0,"interactive-job.publication-rejected");
  assert_eq!(fault.message,reason);
  assert!(mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).is_none(),"fault terminal cannot repeat before ACK");
  assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
mounted.close_fixture_original_page(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close);
 }
 app.tools.tool_operations.insert_admitted(1,mounted);
 turns
}

/// ✅️ Publishes one ephemeral mutation to completion, which is what moves the store past the
/// generation an already-begun sibling publication captured. It is NOT retired here: its displaced
/// root stays co-owned by the rejected sibling's own base read until that sibling has retired, so
/// the superseding owner is retired last.
fn publish_ephemeral<P: Send + Sync + 'static, M>(
    publication: &mut store::ArtifactEphemeralOneItemPublication<P, M>,
    grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    mut advance: impl FnMut(&mut store::ArtifactEphemeralOneItemPublication<P, M>, store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, String>,
) {
    for _ in 0..4_096 {
        if matches!(advance(publication, grant).expect("superseding ephemeral publication advances"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) {
            assert!(publication.acknowledge());
            return;
        }
    }
    panic!("superseding ephemeral publication never published");
}

/// ✅️ Retires a publication the store accepted — the same shared law, whose terminal turn owes the
/// host nothing because nothing rejected it.
fn retire_accepted(pending:&mut PendingArtifactStorePublication<RetirementApp>,_grant:store::os_store::one_item::ArtifactStoreOneItemGrant){
 pending.begin_close();
 for _ in 0..8192{
  if pending.terminal_is_empty(){return;}
  let demand=pending.retirement_demands(512).expect("accepted original quote");
  pending.close_step(granted(demand)).expect("accepted original four-currency retirement");
 }
 panic!("accepted original never reached terminal emptiness");
}

fn retirement_view() -> ViewModel {
    ViewModel {
        window_id: Some("publication-retirement-window-left".into()),
        window_instances: vec![ViewWindowInstance { id: "publication-retirement-window-left".into(), window_kind_id: RETIREMENT_WINDOW_KIND.into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

fn document_mutation(value: i32) -> TestMutation {
    SetCount { value }.into()
}

fn admit_window_config_publication(registry: &mut WindowConfigOwnerRegistry, operation: semio_framework_job::OperationId, authority: &semio_framework_plugin_window_config::WindowConfigAuthority, mutation: WindowConfigMutation, grant: semio_framework_value::RetainedCloneGrant) -> Box<dyn semio_framework_plugin_window_config::ErasedWindowConfigPublication> {
    let (actor, _) = registry.local_actor_id().0.admit_clone(grant).unwrap();
    let mut actor = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(actor).map_err(|(error, _)| error).unwrap());
    let mut mutations = vec![mutation];
    registry.admit_begin(operation, &mut actor, &mut mutations, authority, grant).unwrap().expect("window config publication admitted").0
}

#[doc(hidden)]
pub fn config_mutation(value: &str) -> TestConfigMutation {
    ChangeTestConfigSelection { selected: Some(value.to_string()) }.into()
}
//#endregion ♻️RetirementDrivers

/// 🪢️ The superseding owner is retired LAST: until the rejected sibling has drained, both
/// still co-own the root the superseding write displaced.
#[semio_framework_async_macros::async_test]
async fn every_publication_lane_retires_a_rejected_authority_without_faulting_each_turn() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let fixture = fixture();
    let grant = grant(&fixture);
    let rows = fixture["lanes"].as_array().expect("fixture lanes").clone();
    let minimum_turns = fixture["law"]["minimumIncompleteTurnsWhileRejected"].as_u64().expect("fixture minimum turns") as usize;
    assert_eq!(rows.len(), 7, "the law covers every publication lane");
    assert_eq!(fixture["law"]["incompleteRetirementTurnIsOk"], true);
    let operation = semio_framework_job::OperationId(1);
    let mut app = semio_framework_plugin_testing_laws::artifact_app_laws::new_app::<RetirementApp>(protocol::ActorId("fixture".into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    let view = retirement_view();
    let window_config_authority = app.stores.window_config_store.capture(Some(&view)).await.expect("window config capture").expect("registered window config owner");
    let window_transient_authority = app.stores.window_transient_store.capture(Some(&view)).expect("window transient capture").expect("registered window transient owner");

    for row in &rows {
        let mut superseding_owner: Option<PendingArtifactStorePublication<RetirementApp>> = None;
        let mut pending = match row["id"].as_str().expect("fixture lane id") {
            "artifact" => {
                let publication = app
                    .stores.store
                    .begin_apply_batch(operation, app.stores.store.generation_now(), app.stores.store.content_revision_now(), "fixture".into(), vec![document_mutation(1)], store::os_store::store_schema::HistoryLane::Document, app.stores.artifact_one_item_factory.as_ref(), None)
                    .unwrap_or_else(|rejected| panic!("artifact publication admitted: {}", rejected.into_owners().0));
                semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| app.stores.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![document_mutation(9)], transaction: None }, &mut identity).await).expect("superseding document write");
                let mut publication = publication;
                assert!(receiving_job.receive(crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.store.advance_apply_batch(&mut publication, grant,receiving)).is_err(), "the superseded document publication is rejected");
                PendingArtifactStorePublication::Artifact(publication)
            }
            "config" => {
                let publication = app
                    .stores.config_store
                    .begin_apply_batch(
                        operation,
                        app.stores.config_store.generation_now(),
                        app.stores.config_store.content_revision_now(),
                        "fixture".into(),
                        vec![config_mutation("first")],
                        store::os_store::store_schema::HistoryLane::Document,
                        app.stores.config_one_item_factory.as_ref(),
                        None,
                    )
                    .unwrap_or_else(|rejected| panic!("config publication admitted: {}", rejected.into_owners().0));
                semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| app.stores.config_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![config_mutation("superseding")], transaction: None }, &mut identity).await).expect("superseding config write");
                let mut publication = publication;
                assert!(receiving_job.receive(crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.config_store.advance_apply_batch(&mut publication, grant,receiving)).is_err(), "the superseded config publication is rejected");
                PendingArtifactStorePublication::Config(publication)
            }
            "draft" => {
                let publication = app
                    .stores.draft_store
                    .begin_apply_batch(
                        operation,
                        app.stores.draft_store.generation_now(),
                        app.stores.draft_store.content_revision_now(),
                        "fixture".into(),
                        vec![config_mutation("first")],
                        store::os_store::store_schema::HistoryLane::Document,
                        app.stores.draft_one_item_factory.as_ref(),
                        None,
                    )
                    .unwrap_or_else(|rejected| panic!("draft publication admitted: {}", rejected.into_owners().0));
                semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| app.stores.draft_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![config_mutation("superseding")], transaction: None }, &mut identity).await).expect("superseding draft write");
                let mut publication = publication;
                assert!(receiving_job.receive(crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.draft_store.advance_apply_batch(&mut publication, grant,receiving)).is_err(), "the superseded draft publication is rejected");
                PendingArtifactStorePublication::Draft(publication)
            }
            "presence" => {
                let generation = app.stores.presence_store.generation_now();
                let factory = app.stores.presence_one_item_factory.clone().expect("presence preparation factory");
                let retirement = app.stores.presence_local_root_retirement_factory.clone();
                let mut publication = app
                    .stores.presence_store
                    .begin_publish_one(operation, generation, ChangePublicationPresence { revision: 1 }.into(), Some(factory.as_ref()), retirement.clone())
                    .unwrap_or_else(|rejected| panic!("presence publication admitted: {}", rejected.into_owners().0));
                let mut superseding = app.stores.presence_store.begin_publish_one(operation, generation, ChangePublicationPresence { revision: 2 }.into(), Some(factory.as_ref()), retirement).expect("superseding presence publication admitted");
                publish_ephemeral(&mut superseding, grant, |publication, grant|  receiving_job.receive(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.presence_store.advance_publish_one(publication, grant,receiving)));
                superseding_owner = Some(PendingArtifactStorePublication::Presence(superseding));
                assert!(receiving_job.receive(crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.presence_store.advance_publish_one(&mut publication, grant,receiving)).is_err(), "the superseded presence publication is rejected");
                PendingArtifactStorePublication::Presence(publication)
            }
            "transient" => {
                let generation = app.stores.transient_store.generation_now();
                let factory = app.stores.transient_one_item_factory.clone().expect("transient preparation factory");
                let retirement = app.stores.transient_local_root_retirement_factory.clone();
                let mut publication = app
                    .stores.transient_store
                    .begin_publish_one(operation, generation, ChangePublicationTransient { revision: 1 }.into(), Some(factory.as_ref()), retirement.clone())
                    .unwrap_or_else(|rejected| panic!("transient publication admitted: {}", rejected.into_owners().0));
                let mut superseding = app.stores.transient_store.begin_publish_one(operation, generation, ChangePublicationTransient { revision: 2 }.into(), Some(factory.as_ref()), retirement).expect("superseding transient publication admitted");
                publish_ephemeral(&mut superseding, grant, |publication, grant|  receiving_job.receive(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.transient_store.advance_publish_one(publication, grant,receiving)));
                superseding_owner = Some(PendingArtifactStorePublication::Transient(superseding));
                assert!(receiving_job.receive(crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance,|receiving|app.stores.transient_store.advance_publish_one(&mut publication, grant,receiving)).is_err(), "the superseded transient publication is rejected");
                PendingArtifactStorePublication::Transient(publication)
            }
            "windowConfig" => {
                let mutation = |value: &str| WindowConfigMutation::of::<RetirementWindowConfigOwner>("publication-retirement-window-left", config_mutation(value));
                let mut publication = admit_window_config_publication(&mut app.stores.window_config_store, operation, &window_config_authority, mutation("first"), grant.retained_grant());
                let mut superseding = admit_window_config_publication(&mut app.stores.window_config_store, operation, &window_config_authority, mutation("superseding"), grant.retained_grant());
                for _ in 0..4_096 {
                    if matches!(app.stores.window_config_store.advance(superseding.as_mut(), grant).expect("superseding window config advances"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) {
                        assert!(superseding.acknowledge());
                        break;
                    }
                }
                superseding_owner = Some(PendingArtifactStorePublication::WindowConfig(superseding));
                assert!(app.stores.window_config_store.advance(publication.as_mut(), grant).is_err(), "the superseded window config publication is rejected");
                PendingArtifactStorePublication::WindowConfig(publication)
            }
            "windowTransient" => {
                let mutation = |revision| WindowTransientMutation::of::<RetirementWindowTransientOwner>("publication-retirement-window-left", ChangePublicationTransient { revision }.into());
                let mut publication = app.stores.window_transient_store.begin(operation, &window_transient_authority, mutation(11)).expect("window transient publication admitted");
                let mut superseding = app.stores.window_transient_store.begin(operation, &window_transient_authority, mutation(12)).expect("superseding window transient publication admitted");
                for _ in 0..4_096 {
                    if matches!(app.stores.window_transient_store.advance(superseding.as_mut(), grant).expect("superseding window transient advances"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) {
                        assert!(superseding.acknowledge());
                        break;
                    }
                }
                superseding_owner = Some(PendingArtifactStorePublication::WindowTransient(superseding));
                assert!(app.stores.window_transient_store.advance(publication.as_mut(), grant).is_err(), "the superseded window transient publication is rejected");
                PendingArtifactStorePublication::WindowTransient(publication)
            }
            other => panic!("fixture declares an unknown publication lane {other}"),
        };
        let retiring_turns = drive_rejected_retirement(&mut app,pending,row).await;
        assert!(retiring_turns >= minimum_turns, "{} drained its rejection over {retiring_turns} incomplete turns, every one of which used to be a fault", row["id"]);
        assert!(app.tools.tool_operations.get(1).is_some_and(|owner|owner.pending_artifact_publication.is_none()), "{} retired terminal-empty", row["id"]);
        if let Some(mut owner) = superseding_owner {
            retire_accepted(&mut owner, grant);
        }
    }
    drop((window_config_authority, window_transient_authority));
    close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🔁️ The authority captured at admission is now stale by construction, which is exactly why the
/// window-transient emission branch refreshes it before it begins.
#[semio_framework_async_macros::async_test]
async fn window_transient_re_begin_needs_the_refreshed_live_generation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let fixture = fixture();
    let grant = grant(&fixture);
    let expected = fixture["windowTransientReBegin"].clone();
    assert_eq!(expected["windowKindId"], RETIREMENT_WINDOW_KIND);
    let window_id = expected["windowId"].as_str().expect("fixture window id");
    let operation = semio_framework_job::OperationId(2);
    let mut app = semio_framework_plugin_testing_laws::artifact_app_laws::new_app::<RetirementApp>(protocol::ActorId("fixture".into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    let view = retirement_view();
    let mut authority = app.stores.window_transient_store.capture(Some(&view)).expect("window transient capture").expect("registered window transient owner");
    let captured_generation = authority.generation;
    let mutation = |revision| WindowTransientMutation::of::<RetirementWindowTransientOwner>(window_id, ChangePublicationTransient { revision }.into());

    let mut rejected = app.stores.window_transient_store.begin(operation, &authority, mutation(expected["capturedRevision"].as_u64().expect("captured revision"))).expect("captured authority admits its first publication");
    let mut superseding = app.stores.window_transient_store.begin(operation, &authority, mutation(expected["supersedingRevision"].as_u64().expect("superseding revision"))).expect("superseding publication admitted");
    for _ in 0..4_096 {
        if matches!(app.stores.window_transient_store.advance(superseding.as_mut(), grant).expect("superseding window transient advances"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) {
            assert!(superseding.acknowledge());
            break;
        }
    }
    superseding.begin_close();
    for _ in 0..4_096 {
        if matches!(superseding.close_step(grant.retained_grant()).expect("superseding window transient retires"),semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(superseding.terminal_is_empty());
    assert!(app.stores.window_transient_store.advance(rejected.as_mut(), grant).is_err());

    let mut pending = PendingArtifactStorePublication::<RetirementApp>::WindowTransient(rejected);
    let row = fixture["lanes"].as_array().expect("fixture lanes").iter().find(|row| row["id"] == "windowTransient").expect("window transient row").clone();
    let turns = drive_rejected_retirement(&mut app,pending,&row).await;
    assert!(app.tools.tool_operations.get(1).is_some_and(|owner|owner.pending_artifact_publication.is_none()));

    let stale = app.stores.window_transient_store.begin(operation, &authority, mutation(expected["rejectedRevision"].as_u64().expect("rejected revision")));
    assert_eq!(stale.is_ok(), expected["staleAuthorityBeginAccepted"].as_bool().expect("stale begin expectation"));
    drop(stale);
    let mut refreshed=false;
    for _ in 0..4096 {
        let demand=app.stores.window_transient_store.refresh_demands(&authority,512).expect("original refresh demand");
        let turn=granted(demand);
        let step=app.stores.window_transient_store.refresh(&mut authority,turn).expect("window transient authority refreshes onto the live generation");
        let progress=match step { semio_framework_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Pending(progress)=>progress, semio_framework_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Ready(progress)=>{refreshed=true;progress} };
        assert!(progress.fits(turn));
        if refreshed {break;}
    }
    assert!(refreshed,"original authority completes only after funded displaced snapshot capture");
    assert_eq!(authority.generation > captured_generation, expected["refreshedGenerationExceedsCaptured"].as_bool().expect("refreshed generation expectation"));
    let mut re_begun = app.stores.window_transient_store.begin(operation, &authority, mutation(expected["reBeginRevision"].as_u64().expect("re-begin revision"))).expect("refreshed authority admits a new publication");
    assert!(expected["refreshedAuthorityBeginAccepted"].as_bool().expect("refreshed begin expectation"));
    for _ in 0..4_096 {
        if matches!(app.stores.window_transient_store.advance(re_begun.as_mut(), grant).expect("re-begun window transient advances"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) {
            assert!(re_begun.acknowledge());
            break;
        }
    }
    re_begun.begin_close();
    for _ in 0..4_096 {
        if matches!(re_begun.close_step(grant.retained_grant()).expect("re-begun window transient retires"),semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(re_begun.terminal_is_empty());
    drop(authority);
    eprintln!("window transient retired a rejected authority over {turns} Ok turns, refused a stale re-begin, and admitted the refreshed one at generation {}", app.stores.window_transient_store.capture(Some(&view)).unwrap().unwrap().generation);
    close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🫧️ LAW: a window-transient emission the registry refuses is never dropped. The typed-operation unit hands the
/// refusal back as its typed fault and keeps the mutation on the operation's emit, so the ladder retries it and a
/// deterministic refusal ends the operation with that fault — never a silent success with the window transient lost
/// (the refusal used to consume the mutation, and the retry then completed without it).
#[semio_framework_async_macros::async_test]
async fn a_refused_window_transient_emission_keeps_its_mutation_and_faults() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = semio_framework_plugin_testing_laws::artifact_app_laws::new_app::<RetirementApp>(protocol::ActorId("fixture".into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    let authority = app.stores.window_transient_store.capture(Some(&retirement_view())).expect("window transient capture").expect("registered window transient owner");
    let generation = authority.generation;
    let revision = app.stores.store.content_revision_now();
    let operation = semio_framework_job::Operation::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),
        semio_framework_job::Generation(app.stores.store.generation_now()),
        17,
    );
    let lease = app
        .tools.tool_cancellations
        .clone()
        .begin(semio_framework_plugin_operation::ToolOperationKey { app_instance_id: 7, document: semio_framework_plugin_operation::ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation })
        .expect("cancellation lease");
    let misaddressed = WindowTransientMutation::of::<RetirementWindowTransientOwner>("publication-retirement-window-right", ChangePublicationTransient { revision: 3 }.into());
    let mut mounted = semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation::<RetirementApp> {
        verb: "setTransient".into(),
        meta: semio_framework_plugin_decl_emission::ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: Some(retirement_view()) },
        operation,
        canonical_revision: revision,
        artifact_generation: operation.generation.0,
        config_generation: 0,
        draft_generation: 0,
        presence_generation: 0,
        transient_generation: 0,
        window_config_authority: None,
        window_transient_authority: Some(authority),
        publication_lanes: &[semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane::WindowTransient], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
        session: None,
        session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
        reserved_producer: None, completion: None,
        completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
        output_chunks: None,
        cancellation_lease: Some(lease),
        identity: None, identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
        worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None,
        terminal_seen: true, worker_resume_pending: false,
        publication: Some(semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue::Emit(Ok(semio_framework_plugin_decl_emission::Emit::default()), semio_framework_plugin_decl_app::app_emission::EphemeralEmit { window_transient: vec![misaddressed], ..Default::default() })),
        tool_stage:semio_framework_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Capture,tool_press_result:None,pending_artifact_publication:None, pending_stream_abort:None, stream_abort_completed:false, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
        pending_child_publication: None,
        owned_child_group: None,
        owned_child_committed: false,
        owned_child_result_pending: false,
        captured_child_content: Some(Arc::new(semio_framework_plugin_decl_views::child_view::ChildContentView::EMPTY)),
        captured_child_content_generation: 0,
        result_page: None, diagnostics:None,
        result_page_presented: false,
        result_sequence: 0,
        publication_progress: 0,
        publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,
        publication_attempt: 0,
        ui_pending: true,
        progress: None,
        progress_pending: false,
        user_cancel_requested: false,
        published_artifact: false,
        published_config: false,
        published_window_config: false,
        command_logged: false,
        interaction_revalidated: false,
        terminal_fault: None,
        retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None,
        stage: semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage::Publishing,
    };
    for attempt in 0..3 {
        let refused = app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).expect_err("a misaddressed window transient is refused");
        assert_eq!(refused.code.0, "window-transient.address", "attempt {attempt}: the refusal is the registry's typed fault");
        let Some(semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue::Emit(Ok(_), ephemeral)) = mounted.publication.as_ref() else { panic!("the emit stays installed") };
        assert_eq!(ephemeral.window_transient.iter().map(WindowTransientMutation::window_id).collect::<Vec<_>>(), ["publication-retirement-window-right"], "attempt {attempt}: the refused mutation is kept, never dropped");
        assert!(mounted.pending_artifact_publication.is_none() && mounted.result_page.is_none(), "attempt {attempt}: nothing was published or answered");
    }
    assert_eq!(app.stores.window_transient_store.capture(Some(&retirement_view())).expect("capture").expect("owner").generation, generation, "the window transient never moved");
    mounted.publication = None;
    mounted.window_transient_authority = None;
    drop(mounted);
    close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
