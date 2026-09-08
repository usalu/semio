
use super::*;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json: "[]".into() }));
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::testkit::{apply_mutations, studio_emit};
    use pack::json::Object;
    use semio_framework_os::{OsWorkflowCamera, os_workflow_to_flow_fixture};
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node = projection.graph.nodes.first().expect("node").clone();
    let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
    let mut fixture = os_workflow_to_flow_fixture(&projection.graph, &camera);
    if let Some(layout) = fixture.get_mut("layout").and_then(serde_json::Value::as_object_mut) {
        let mut position = serde_json::Map::new();
        position.insert("x".into(), serde_json::Value::from(500.0 + node.width / 2.0));
        position.insert("y".into(), serde_json::Value::from(300.0 + node.height / 2.0));
        layout.insert(node.id.clone(), serde_json::Value::Object(position));
    }
    let mut operations_entry = Object::new();
    operations_entry.insert("operation", pack::JsonValue::from("setFixture"));
    operations_entry.insert("fixtureJson", pack::JsonValue::from(fixture.to_string()));
    let mut operations_root = Object::new();
    operations_root.insert("operations", pack::json_array([pack::JsonValue::Object(operations_entry)]));
    let operations_json = pack::JsonValue::Object(operations_root).to_string();
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json })).await.expect("handle");
    let moved = apply_mutations(&projection, &emit.artifact_mutations).await.graph.nodes.into_iter().find(|row| row.id == node.id).expect("node");
    assert!((moved.x - 500.0).abs() < 0.01);
    assert!((moved.y - 300.0).abs() < 0.01);
    assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]);
}
