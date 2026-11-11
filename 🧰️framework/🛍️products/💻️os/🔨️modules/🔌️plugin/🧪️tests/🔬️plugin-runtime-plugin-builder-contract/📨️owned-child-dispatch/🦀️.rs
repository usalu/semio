use semio_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_plugin_decl_emission::Emit;
use semio_plugin_builder::Ready;
use semio_plugin_testing_laws::artifact_app_laws;
use semio_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_plugin_abi::ArtifactAnalysis;
use semio_plugin_abi::ArtifactAnalyzer;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_abi::ArtifactBuilder;
use semio_plugin_abi::ArtifactChildren;
use semio_plugin_abi::ArtifactComposer;
use semio_plugin_abi::ArtifactComposition;
use semio_plugin_abi::ArtifactDecomposer;
use semio_plugin_abi::ArtifactDeserializer;
use semio_plugin_decl_surface::ArtifactEditor;
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_plugin_decl_app::tool_contract::ArtifactReservedJob;
use semio_plugin_abi::ArtifactSerializer;
use store::ArtifactStoreInitializationAuthority;
use semio_plugin_decl_surface::ArtifactViewer;
use semio_plugin_decl_emission::AsyncTaskRun;
use semio_plugin_decl_emission::AsyncTaskSource;
use semio_plugin_decl_views::child_view::ChildContentRetainers;
use semio_plugin_abi::DerivedArtifactSpec;
use semio_plugin_window_kits::LabelAxes;
use semio_plugin_decl_app::NaturalFileDecodeCursor;
use semio_plugin_reactor_app::PluginApp;
use semio_os_kernel::io::SubsetValidator;
use semio_plugin_decl_emission::ToolCommandCatalog;
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_plugin_window_config::WindowConfigOwner;
use semio_plugin_window_config::WindowConfigReplaceMutation;
use semio_plugin_window_kits::WindowKit;
use semio_plugin_window_transient::window_transient::WindowTransientOwner;
use pack::snapshot::ArtifactPack;
use semio_os_kernel::os_store::backbone::Backbone;
use semio_framework_value::DslValue;
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_os_kernel::{MemberFactory, Mutation, SpaceMember};
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{TestApp, contract_composed_app_raw, drain_and_close_composed_fixture, meta, prepare_original_fixture_child, register_test_child};
use semio_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use serde_json::Value;
use semio_framework_job::InteractiveJob as _;
use semio_plugin_job::original_emission::OriginalEmissionInput;
use semio_plugin_instance_app::VcsArtifactApp;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers;
use semio_plugin_testing_laws::artifact_app_laws::{self, OriginalFixtureScope};

async fn mount_owned_emit(app: &mut VcsArtifactApp<TestApp, TestMembers>, verb: &str, emit: Emit<TestMutation, TestConfigMutation>, receipt: &mut Option<semio_framework::kernel::InvocationResult>, scope: &mut OriginalFixtureScope) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    artifact_app_laws::ensure_fixture_cancellation_issuer(app, scope.cx())?;
    let mut input = Some(OriginalEmissionInput::<TestApp> { verb: verb.into(), meta: meta(), emit });
    for _ in 0..semio_plugin_testing_caller::OriginalFixtureCaller::maximum_turns() {
        if app.dispatch_ctx().dispatch_original_emission(&mut input, receipt, scope.cx()).await? { return Ok(()); }
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "owned child emission exceeded its supplied mount turn bound").into())
}

fn mounted_operation(receipt: &semio_framework::kernel::InvocationResult) -> u64 {
    receipt.output.get("operationId").and_then(DslValue::as_str).expect("the mount answers with its actual admitted operation identity").parse().unwrap()
}

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_parent_and_children() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut app = contract_composed_app_raw(semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        if row["parentTouched"].as_bool().unwrap() { emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() })); }
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(prepare_original_fixture_child(&mut emit, law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        assert_eq!(emit.owned_child_emits.len(), 1);
        let mut scope = OriginalFixtureScope::admit();
        let mut receipt = None;
        mount_owned_emit(&mut app, "compositeEdit", emit, &mut receipt, &mut scope).await.expect("the stepped mount retains owned publication under a genuine mounted operation");
        let admitted = receipt.as_ref().expect("the settled mount answers with its receipt");
        mounted_operation(admitted);
        assert!(admitted.mutations.is_empty(), "publication remains staged at admission");
        let mut retirement = None;
        assert!(artifact_app_laws::retire_fixture_owned(&mut receipt, &mut retirement, artifact_app_laws::fixture_mounted_policy().close, scope.cx()).unwrap());
        drain_and_close_composed_fixture(&mut app, artifact_app_laws::fixture_mounted_policy(), scope.cx());
        scope.retire();
        drain_and_close_composed_fixture(&mut app, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    }
}

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_output_when_authority_capture_refuses() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for refused in [false, true] {
        let row = &law["cases"][1];
        let mut app = contract_composed_app_raw(semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() }));
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(prepare_original_fixture_child(&mut emit, law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        let original = emit.owned_child_emits.as_ptr() as usize;
        assert_eq!(emit.owned_child_emits.len(), 1);
        let verb = if refused { "unregisteredOriginalChildVerb" } else { "compositeEdit" };
        let mut scope = OriginalFixtureScope::admit();
        let mut receipt = None;
        let mounted = mount_owned_emit(&mut app, verb, emit, &mut receipt, &mut scope).await;
        if refused {
            let failure = mounted.expect_err("an unregistered verb refuses before its mounted birth");
            assert!(failure.to_string().contains("registered native publication owner"));
            assert_eq!(app.tools.test_original_admission_emit_observation(), Some((1, 1, original)), "the refused admission keeps the original owned output");
            assert!(receipt.is_none());
        } else {
            mounted.expect("the registered verb mounts its original owned output");
            let operation = mounted_operation(receipt.as_ref().unwrap());
            let (observation, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.tools.test_original_owned_emit_observation(operation).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(observation, (1, 1, original, false, false));
            assert!(receipt.as_ref().unwrap().mutations.is_empty());
            let mut retirement = None;
            assert!(artifact_app_laws::retire_fixture_owned(&mut receipt, &mut retirement, artifact_app_laws::fixture_mounted_policy().close, scope.cx()).unwrap());
        }
        drain_and_close_composed_fixture(&mut app, artifact_app_laws::fixture_mounted_policy(), scope.cx());
        scope.retire();
    }
}

mod reserved_emission_admission_laws { include!("🪪️admission/🦀️.rs"); }
