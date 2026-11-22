

use crate::demo_space_projection;
use semio_hub_space_app::SpaceCommand;
use semio_hub_space_app::unit_tests::context::{apply_mutations, seed_draw_plugin, seed_multi_port_plugins, studio_emit, test_surface_id};
use std::collections::HashSet;
use semio_hub_space_foundation::config::{ActiveNodeSetting, SpaceConfig, SpaceConfigMutation};
use semio_hub_space_commands_authoring::spawn_app::SpawnApp;
use semio_os::WorkflowMutation;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::MoveMediaNode(semio_hub_space_commands_authoring::move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(semio_hub_space_commands_authoring::remove_app_instance::RemoveAppInstance { node_id: Some("n1".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(semio_hub_space_commands_authoring::remove_app_instance::RemoveAppInstance { node_id: None }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::DeleteSelection(semio_hub_space_commands_authoring::delete_selection::DeleteSelection {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CopyAppInstance(semio_hub_space_commands_authoring::copy_app_instance::CopyAppInstance {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::DuplicateAppInstance(semio_hub_space_commands_authoring::duplicate_app_instance::DuplicateAppInstance {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PasteAppInstance(semio_hub_space_commands_authoring::paste_app_instance::PasteAppInstance {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RenameAppInstance(semio_hub_space_commands_authoring::rename_app_instance::RenameAppInstance { label: Some("Renamed".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchMediaNodes(semio_hub_space_commands_media::patch_media_nodes::PatchMediaNodes {
        node_ids: vec!["n1".into()],
        field: "position".into(),
        axis: Some("x".into()),
        value: "120".into(),
    }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchAppInstances(semio_hub_space_commands_authoring::patch_app_instances::PatchAppInstances {
        node_ids: vec!["n1".into()],
        field: "label".into(),
        value: "Batch Label".into(),
    }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ReorganizeWorkflow(semio_hub_space_commands_authoring::reorganize_workflow::ReorganizeWorkflow {}));
}

/// ⚖️ LAW: a palette drop is ONE relative `move-nodes` leaf — the offset from the node's base position — with no
/// coalesce key, and folding it lands the node exactly on the drop.
#[semio_framework_async_macros::async_test]
async fn move_media_node_emits_one_relative_move() {
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let base = projection.graph.nodes.first().expect("node").clone();
    let node_id = base.id.clone();
    let emit = studio_emit(&projection, &config, &SpaceCommand::MoveMediaNode(semio_hub_space_commands_authoring::move_media_node::MoveMediaNode { node_id: node_id.clone(), x: 120.0, y: 160.0 })).await.expect("handle");
    assert!(matches!(emit.artifact_mutations.as_slice(), [WorkflowMutation::MoveNodes(leaf)] if leaf.node_ids == [node_id.clone()] && leaf.dx == 120.0 - base.x && leaf.dy == 160.0 - base.y), "{:?}", emit.artifact_mutations);
    let node = apply_mutations(&projection, &emit.artifact_mutations).await.graph.nodes.into_iter().find(|row| row.id == node_id).expect("node");
    assert!((node.x - 120.0).abs() < 0.01);
    assert!((node.y - 160.0).abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn spawns_draw_app_instance() {
    seed_draw_plugin().await;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw").await, x: 80.0, y: 80.0 })).await.expect("handle");
    assert!(!emit.artifact_mutations.is_empty());
    let next = apply_mutations(&projection, &emit.artifact_mutations).await;
    assert_eq!(next.graph.nodes.len(), projection.graph.nodes.len() + 1);
    let expected_active = next.graph.nodes.last().map(|node| node.id.clone());
    assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: expected_active })]);
}

#[semio_framework_async_macros::async_test]
async fn spawns_draw_app_instance_at_drop_position() {
    seed_draw_plugin().await;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let existing: HashSet<String> = projection.graph.nodes.iter().map(|node| node.id.clone()).collect();
    let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw").await, x: 321.0, y: 654.0 })).await.expect("handle");
    let next = apply_mutations(&projection, &emit.artifact_mutations).await;
    let node = next.graph.nodes.iter().find(|node| node.plugin_id == "draw" && !existing.contains(&node.id)).expect("newly spawned draw node");
    assert!((node.x - 321.0).abs() < 0.01);
    assert!((node.y - 654.0).abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn patch_app_instances_updates_labels() {
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let emit =
        studio_emit(&projection, &config, &SpaceCommand::PatchAppInstances(semio_hub_space_commands_authoring::patch_app_instances::PatchAppInstances { node_ids: ids.clone(), field: "label".into(), value: "Batch Label".into() })).await.expect("handle");
    let next = apply_mutations(&projection, &emit.artifact_mutations).await;
    let labels: Vec<String> = next.graph.nodes.iter().filter(|node| ids.contains(&node.id)).map(|node| node.label.clone()).collect();
    assert!(labels.iter().all(|label| label == "Batch Label"));
}

#[semio_framework_async_macros::async_test]
async fn spawns_puzzle5d_and_shooting_with_multi_port_registrations() {
    seed_multi_port_plugins().await;
    let mut projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "puzzle.5d".into(), app_id: test_surface_id("puzzle5d").await, x: 200.0, y: 100.0 })).await.expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "shooting".into(), app_id: test_surface_id("shooting").await, x: 300.0, y: 100.0 })).await.expect("handle");
    projection = apply_mutations(&projection, &emit.artifact_mutations).await;
    let puzzle_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "puzzle.5d").expect("puzzle node");
    let shooting_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "shooting").expect("shooting node");
    assert_eq!(puzzle_node.outputs.len(), 3, "document:out + out-a + out-b");
    assert_eq!(shooting_node.inputs.len(), 2, "document:in + scene-in");
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_on_spawn() {
    use semio_plugin::artifact_app_laws;
    seed_draw_plugin().await;
    let mut app = semio_hub_space_app::unit_tests::context::app_with_registry().await;
    let before = app.snapshot().expect("projection").graph.nodes.len();
    artifact_app_laws::assert_undo_redo_round_trip(
        &mut app,
        SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw").await, x: 80.0, y: 80.0 }),
        |app| app.snapshot().expect("projection").graph.nodes.len(),
        before,
        before + 1,
    )
    .await;
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn space_bound_spawn_publishes_only_after_the_correlated_artifact_is_ready() {
    use semio_hub_space_commands_authoring::{open_instance::OpenInstance, spawn_app::finish_spawn_app::FinishSpawnApp};
    use semio_plugin::{AppOperationContext, ArtifactView, ConfigView, Effect};

    seed_draw_plugin().await;
    let data = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let field = |key| data.get(key).and_then(semio_framework_pack_json::Value::as_str).unwrap().to_string();
    let coords = |key| data.get(key).and_then(semio_framework_pack_json::Value::as_f64).unwrap();
    let config = SpaceConfig { space_id: Some(field("spaceId")), workflow_artifact_id: Some("workflow-123".into()), ..Default::default() };
    let projection = demo_space_projection().await;
    let history = semio_hub_space_app::unit_tests::context::empty_history();
    let operation = AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "workflow-123".into(),
        operation_id: 1,
        generation: 1,
        canonical_base_revision: [0; 32],
        retained: Default::default(),
        authoring_seed: "spawn-1".into(),
    };
    let expected_node_id = operation.entity_id("node", "spawnApp").unwrap();
    let doc = ArtifactView::with_operation(&projection, &history, operation);
    let app_id = test_surface_id("draw").await;
    let request = SpawnApp { plugin_id: field("pluginId"), app_id: app_id.clone(), x: coords("x"), y: coords("y") };
    let emitted = SpaceCommand::SpawnApp(request).dispatch(&doc, &ConfigView { snapshot: &config, window: None }).unwrap();
    assert!(emitted.artifact_mutations.is_empty());
    assert!(matches!(emitted.effects.as_slice(), [Effect::ReplayShellCommand { action_id, args: Some(args) }] if
        action_id == "os.create-workflow-artifact" && args.get("nodeId").and_then(semio_framework_value::DslValue::as_str) == Some(expected_node_id.as_str())));

    let payload = FinishSpawnApp {
        plugin_id: field("pluginId"), app_id: app_id.clone(), node_id: expected_node_id.clone(),
        x: coords("x"), y: coords("y"), artifact_id: field("artifactId"), artifact_schema: field("schema"),
        space_id: field("spaceId"), name: field("name"),
    };
    let completion = SpaceCommand::FinishSpawnApp(payload.clone());
    store::os_store::test_support::assert_op_line_round_trip(&completion);
    let emit = studio_emit(&projection, &config, &completion).await.unwrap();
    let committed = apply_mutations(&projection, &emit.artifact_mutations).await;
    let node = committed.graph.nodes.iter().find(|node| node.id == expected_node_id).unwrap();
    assert_eq!((node.x, node.y), (coords("x"), coords("y")));
    let persisted: serde_json::Value = serde_json::from_str(&node.artifact_ref).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    assert_eq!(persisted, expected["document"]);
    assert_ne!(node.id, field("artifactId"));
    assert!(studio_emit(&committed, &config, &completion).await.unwrap().artifact_mutations.is_empty());
    let conflicting = SpaceCommand::FinishSpawnApp(FinishSpawnApp { artifact_id: "different-artifact".into(), ..payload.clone() });
    assert!(studio_emit(&committed, &config, &conflicting).await.is_err());
    let stale = SpaceConfig { space_id: Some("another-space".into()), ..Default::default() };
    assert!(studio_emit(&projection, &stale, &completion).await.is_err());

    let reopened = studio_emit(&committed, &config, &SpaceCommand::OpenInstance(OpenInstance { node_id: Some(expected_node_id.clone()) })).await.unwrap();
    assert!(matches!(reopened.effects.as_slice(), [Effect::OpenPluginInstance { app_id: opened_app, os_instance_id: Some(node_id), document: Some(target), .. }] if
        opened_app == &app_id && node_id == &expected_node_id && target.document_id == field("artifactId")
        && target.schema == field("schema") && target.space_id.as_deref() == Some(field("spaceId").as_str())));
}
