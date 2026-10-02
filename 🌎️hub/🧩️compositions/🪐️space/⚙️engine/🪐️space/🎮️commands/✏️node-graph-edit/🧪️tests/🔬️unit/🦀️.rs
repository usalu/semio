
use super::*;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json: "[]".into() }));
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

/// ⚖️ LAW: a `delete` row removes exactly the edges and app instances it names (never an ambient selection), edges first.
#[semio_framework_async_macros::async_test]
async fn a_delete_row_removes_the_named_edges_then_nodes() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::unit_tests::context::studio_emit;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node = projection.graph.nodes.first().expect("node").id.clone();
    let edge = projection.graph.edges.first().map(|edge| edge.id.clone());
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(serde_json::json!([{ "operation": "delete", "nodeIds": [node, "ghost"], "synapseIds": edge.iter().collect::<Vec<_>>() }])))).await.expect("handle");
    let disconnects = emit.artifact_mutations.iter().filter(|leaf| matches!(leaf, WorkflowMutation::DisconnectEdge(_))).count();
    assert_eq!(disconnects, usize::from(edge.is_some()), "{:?}", emit.artifact_mutations);
    assert!(matches!(emit.artifact_mutations.last(), Some(WorkflowMutation::RemoveNode(leaf)) if leaf.node_id == node), "{:?}", emit.artifact_mutations);
    assert_eq!(emit.artifact_mutations.len(), disconnects + 1, "the ghost node is skipped");
}

/// ⚖️ LAW: the studio guest decodes the renderer's committed node-graph rows through the ONE shared decoder: every refused
/// row is refused, every accepted row a workflow carries (`move`, `connect`, `disconnect`, `delete`) decodes, and the
/// `setSlider`/`insertPort` rows it has no widget for are refused.
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    let row = |value: &serde_json::Value| space_node_graph_row(&pack::parse_json(&value.to_string()).expect("row JSON"));
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let carried = !matches!(case["row"]["operation"].as_str(), Some("setSlider" | "insertPort"));
        assert_eq!(row(&case["row"]).is_ok(), carried, "accepted row {}", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(row(&case["row"]).is_err(), "refused row {} decoded", case["id"]);
    }
}

/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");
//#endregion ✋️GestureLaws
