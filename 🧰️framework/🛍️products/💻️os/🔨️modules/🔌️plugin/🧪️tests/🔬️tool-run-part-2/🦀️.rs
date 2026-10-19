//! ⏯️ Part 2 of 2 of the runtime laws of the tool run ledger and driver (cut from `🔬️tool-run`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_views::NoDraftMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_tests_06::tool_run_support_1::{ToyFixtureCaller, ToyRunApp, close, fixture, number, pump_until, run_action, start, text, toy_app, toy_fixture_caller_policy, toy_meta};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_os_kernel::io;
use semio_framework::*;
use pack::snapshot::ArtifactPack;
use semio_framework::{FaultCode, FaultFrom};
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_tool_run::ToolRunState;
use store::os_store::backbone::Backbone;


#[test]
fn original_mounted_child_frontier_closes_after_actual_ack_under_original_policy(){
 let grant=toy_fixture_caller_policy().0.close;
 semio_framework_plugin_retained_command::typed_operation::test_original_mounted_child_frontier::<ToyRunApp>(||vec![SetCount{value:17}.into(),SetLabel{value:"original child metadata 🧵\0".repeat(8192)}.into()],grant,semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),semio_framework_plugin_testing_laws::artifact_app_laws::close_fixture_result_page);
}

#[test]
fn a_mounted_operation_reports_the_structural_refusal_it_retains(){
 semio_framework_plugin_retained_command::typed_operation::test_retained_structural_refusal::<ToyRunApp>();
}

#[semio_framework_async_macros::async_test]
async fn tool_run_freeze_policy_rejects_local_artifact_emits_with_busy() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    start(&mut app, text(&fixture["freezeToolId"]), &mut caller).await;
    let result = app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations: vec![SetCount { value: 1 }.into()], ..Default::default() }, &toy_meta(), &mut caller.identity).await;
    let fault = result.err().expect("a freezing run rejects local artifact emits");
    assert_eq!(fault.code, FaultCode::new(text(&fixture["busy"]["code"])));
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}