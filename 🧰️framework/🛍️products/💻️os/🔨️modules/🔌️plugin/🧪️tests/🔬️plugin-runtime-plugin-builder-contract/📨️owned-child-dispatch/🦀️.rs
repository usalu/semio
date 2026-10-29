use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_builder::Ready;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_abi::ArtifactDeserializer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use semio_framework_plugin_abi::ArtifactSerializer;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_emission::AsyncTaskRun;
use semio_framework_plugin_decl_emission::AsyncTaskSource;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_abi::DerivedArtifactSpec;
use semio_framework_plugin_window_kits::LabelAxes;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_decl_emission::ToolCommandCatalog;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_config::WindowConfigReplaceMutation;
use semio_framework_plugin_window_kits::WindowKit;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_value::DslValue;
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_os_kernel::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{TestApp, contract_composed_app_raw, drain_and_close_composed_fixture, meta, prepare_original_fixture_child, register_test_child};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use serde_json::Value;
use semio_framework_job::InteractiveJob as _;

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_parent_and_children() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut app = contract_composed_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        if row["parentTouched"].as_bool().unwrap() { emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() })); }
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(prepare_original_fixture_child(&mut emit, law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        assert_eq!(emit.owned_child_emits.len(), 1);
        let result = app.test_dispatch_emit("compositeEdit", emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("direct dispatch retains owned publication under a genuine mounted operation");
        assert!(result.output.get("operationId").and_then(DslValue::as_str).is_some(), "direct dispatch returns its actual admitted operation identity");
        assert!(result.mutations.is_empty(), "publication remains staged at admission");
        drain_and_close_composed_fixture(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    }
}

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_output_when_authority_capture_refuses() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for refused in [false, true] {
        let row = &law["cases"][1];
        let mut app = contract_composed_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() }));
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(prepare_original_fixture_child(&mut emit, law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        let original = emit.owned_child_emits.as_ptr() as usize;
        assert_eq!(emit.owned_child_emits.len(), 1);
        let verb = if refused { "unregisteredOriginalChildVerb" } else { "compositeEdit" };
        let result = app.test_dispatch_emit(verb, emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("original output remains mounted even after authority capture refusal");
        let operation = result.output.get("operationId").and_then(DslValue::as_str).unwrap().parse::<u64>().unwrap();
        let (observation, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.tools.test_original_owned_emit_observation(operation).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observation, (1, 1, original, refused, refused));
        assert!(result.mutations.is_empty());
        drain_and_close_composed_fixture(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    }
}

mod reserved_emission_admission_laws { include!("🪪️admission/🦀️.rs"); }
