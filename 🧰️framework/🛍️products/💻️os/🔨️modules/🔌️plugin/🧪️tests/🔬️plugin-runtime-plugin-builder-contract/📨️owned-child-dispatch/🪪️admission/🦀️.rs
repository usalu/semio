use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use pack::snapshot::ArtifactPack;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_os_kernel::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{TestApp, contract_composed_app_raw, drain_and_close_composed_fixture, meta};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use serde_json::Value;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigReplaceMutation};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_job::InteractiveJob as _;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
#[test]
fn framework_reserved_emit_publication_contracts_match_neutral_routes(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let declarations=[("copy",FrameworkCopyJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("cut",FrameworkCutJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("paste",FrameworkPasteJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("import-media",FrameworkImportMediaJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("undo",FrameworkUndoJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("redo",FrameworkRedoJobFactory::<TestApp>::PUBLICATION_CONTRACTS)];
 for(row,(id,contracts))in law["routes"].as_array().unwrap().iter().zip(declarations){assert_eq!(row["id"].as_str().unwrap(),id);assert_eq!(contracts.len(),1);assert_eq!(contracts[0].tool_id,id);let actual=contracts[0].lanes.iter().map(|lane|format!("{lane:?}")).collect::<Vec<_>>();let expected=row["lanes"].as_array().unwrap().iter().map(|lane|lane.as_str().unwrap().to_string()).collect::<Vec<_>>();assert_eq!(actual,expected,"reserved route {id} declares its exact original durable emission authority");}
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_admission_preserves_saturated_foreign_slots_and_original_lease(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let occupied=row["occupied"].as_u64().unwrap()as usize;
  let mut app=contract_composed_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
  let admitted=app.dispatch_ctx().test_reserved_emit_admission_case(occupied,&meta());
  drain_and_close_composed_fixture(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
  assert_eq!(admitted,row["admitted"].as_bool().unwrap());
 }
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_cancelled_worker_keeps_original_permit_until_completion_handoff(){
 let mut app=contract_composed_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
 let retained=app.dispatch_ctx().test_reserved_emit_cancelled_worker_handoff(&meta()).await;
 drain_and_close_composed_fixture(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
 assert!(retained,"cancelled producer must return its original permit so completed original output can enter bounded mounted cancellation");
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_busy_completion_keeps_original_fixed_owner(){
 let mut app=contract_composed_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
 let retained=app.test_reserved_emit_busy_completion_owner(&meta()).await;
 drain_and_close_composed_fixture(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
 assert!(retained,"busy original completion remains in the same pre-admitted operation until bounded close");
}
