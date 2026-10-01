
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
    use crate::engine::space::unit_tests::context::{apply_mutations, studio_emit};
    use pack::json::Object;
    use semio_framework_os::{OsWorkflowCamera, os_workflow_to_flow_host_snapshot_json};
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node = projection.graph.nodes.first().expect("node").clone();
    let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
    let mut fixture = os_workflow_to_flow_host_snapshot_json(&projection.graph, &camera);
    if let Some(layout) = fixture.get_mut("layout").and_then(serde_json::Value::as_object_mut) {
        let mut position = serde_json::Map::new();
        position.insert("x".into(), serde_json::Value::from(500.0 + node.width / 2.0));
        position.insert("y".into(), serde_json::Value::from(300.0 + node.height / 2.0));
        layout.insert(node.id.clone(), serde_json::Value::Object(position));
    }
    let mut operations_entry = Object::new();
    operations_entry.insert("operation", pack::JsonValue::from("setHostSnapshot"));
    operations_entry.insert("hostSnapshotJson", pack::JsonValue::from(fixture.to_string()));
    let operations_json = pack::json_array([pack::JsonValue::Object(operations_entry)]).to_string();
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json })).await.expect("handle");
    assert!(matches!(emit.artifact_mutations.as_slice(), [WorkflowMutation::MoveNodes(leaf)] if leaf.node_ids == [node.id.clone()]), "a snapshot drag is ONE relative leaf: {:?}", emit.artifact_mutations);
    let moved = apply_mutations(&projection, &emit.artifact_mutations).await.graph.nodes.into_iter().find(|row| row.id == node.id).expect("node");
    assert!((moved.x - 500.0).abs() < 0.01);
    assert!((moved.y - 300.0).abs() < 0.01);
    assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]);
}

//#region ✋️GestureLaws
/// 🕹️ The `nodeGraphEdit` operations array a host dispatches, as its JSON text.
fn rows(rows: serde_json::Value) -> NodeGraphEdit {
    NodeGraphEdit { operations_json: rows.to_string() }
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record, as both hosts write it) is ONE relative `move-nodes` leaf
/// over the record's nodes the workflow holds; a ghost node is dropped from the record, and a drag that moves nothing
/// leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_relative_move() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::unit_tests::context::{apply_mutations, studio_emit};
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let mut record_ids = ids.clone();
    record_ids.push("ghost".into());
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": record_ids, "dx": 40.0, "dy": -12.5 }])))).await.expect("handle");
    assert!(matches!(emit.artifact_mutations.as_slice(), [WorkflowMutation::MoveNodes(leaf)] if leaf.node_ids == ids && leaf.dx == 40.0 && leaf.dy == -12.5), "{:?}", emit.artifact_mutations);
    let moved = apply_mutations(&projection, &emit.artifact_mutations).await;
    for id in &ids {
        let (before, after) = (projection.graph.nodes.iter().find(|node| &node.id == id).expect("base"), moved.graph.nodes.iter().find(|node| &node.id == id).expect("moved"));
        assert_eq!((after.x, after.y), (before.x + 40.0, before.y - 12.5));
    }
    for nothing in [serde_json::json!([{ "operation": "move", "gestureId": "node-drag:2", "nodeIds": [ids[0]], "dx": 0.0, "dy": 0.0 }]), serde_json::json!([{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["ghost"], "dx": 5.0, "dy": 0.0 }])] {
        let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(nothing))).await.expect("handle");
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "zero trace: {:?}", emit.artifact_mutations);
    }
}

/// ⚖️ LAW: a snapshot diff groups position changes by offset — two nodes moved together are ONE `move-nodes` leaf, a
/// node moved alone by another offset is its own leaf.
#[test]
fn snapshot_moves_group_by_offset() {
    let node = |id: &str, x: f64, y: f64| semio_framework_os::WorkflowNode { id: id.into(), plugin_id: String::new(), app_id: String::new(), label: String::new(), yields: String::new(), artifact_ref: String::new(), config_ref: String::new(), x, y, width: 10.0, height: 10.0, inputs: Vec::new(), outputs: Vec::new() };
    let nodes = vec![node("a", 0.1, 0.0), node("b", 0.7, 0.0), node("c", 5.0, 5.0)];
    let moves = vec![MoveNode { node_id: "a".into(), x: 0.1 + 0.2, y: 0.0 }, MoveNode { node_id: "b".into(), x: 0.7 + 0.2, y: 0.0 }, MoveNode { node_id: "c".into(), x: 5.0, y: 9.0 }];
    let leaves = space_move_leaves(&nodes, &moves);
    assert_eq!(leaves.len(), 2, "{leaves:?}");
    assert!(matches!(&leaves[0], WorkflowMutation::MoveNodes(leaf) if leaf.node_ids == ["a", "b"] && (leaf.dx - 0.2).abs() < 1e-12));
    assert!(matches!(&leaves[1], WorkflowMutation::MoveNodes(leaf) if leaf.node_ids == ["c"] && leaf.dy == 4.0));
}
//#endregion ✋️GestureLaws
