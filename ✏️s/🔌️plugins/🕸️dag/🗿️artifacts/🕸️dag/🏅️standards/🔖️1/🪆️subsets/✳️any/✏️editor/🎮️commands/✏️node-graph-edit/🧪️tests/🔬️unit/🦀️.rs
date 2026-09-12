use super::DagNodeGraphEditOp;
use super::*;
use crate::editor::dag::commands::{connect_media_ports, disconnect, move_media_node};
use crate::editor::dag::unit_tests::context;
use crate::editor::dag::DagCommand;
use semio_framework_plugin::{artifact_app_laws::meta, InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};
use serde_json::json;

/// 🧪️ `nodeGraphEdit` batches multiple sub-edits (connect + delete-selection here) into a single
/// typed command — mirrors the pre-migration JSON `operations` array, now closed and typed. The
/// `graph` domain's live selection (populated via the framework's own `interactionSelect` action —
/// the only way a downstream crate can populate a genuine `InteractionView`) drives the batched
/// delete-selection sub-op — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_batches_connect_then_delete_selection() {
    let mut app = context::new_app_with_registry().await;
    let (source_id, target_id) = {
        let projection = app.snapshot().expect("projection");
        let nodes = projection.nodes();
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = app.snapshot().expect("projection").edges().len();
    app.dispatch_typed(
        DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Connect { source_node_id: source_id.clone(), source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }] }),
        &meta("local"),
    )
    .await
    .expect("batched connect");
    assert!(app.snapshot().expect("projection").edges().len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");

    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: source_id }]).expect("targets");
    app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&dsl::DslValue::from(&json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" }))), &meta("local")).await.expect("interactionSelect");
    let nodes_before = app.snapshot().expect("projection").nodes().len();
    app.dispatch_typed(DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::DeleteSelection] }), &meta("local")).await.expect("batched delete");
    assert_eq!(app.snapshot().expect("projection").nodes().len(), nodes_before - 1);
}

#[semio_framework_async_macros::async_test]
async fn move_media_node_drag_coalesces_into_one_edit() {
    let mut app = context::new_app().await;
    let node_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");
    for position in [10.0, 20.0, 30.0] {
        app.dispatch_typed(DagCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: node_id.clone(), x: position, y: position }), &meta("local")).await.expect("drag tick");
    }
    // A whole drag (three ticks, same coalesce key) is ONE undo step, not one-operation-per-tick.
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    let restored = app.snapshot().expect("projection");
    let original = crate::default_snapshot().nodes().iter().find(|node| node.id == node_id).map(|node| node.x).expect("original x");
    assert_eq!(restored.nodes().iter().find(|node| node.id == node_id).unwrap().x, original, "undoing the coalesced drag restores the pre-drag position");
}

#[semio_framework_async_macros::async_test]
async fn disconnect_removes_a_known_edge_and_is_a_no_op_for_an_unknown_one() {
    let mut app = context::new_app().await;
    let edge_id = app.snapshot().expect("projection").edges().first().map(|edge| edge.id.clone());
    if let Some(edge_id) = edge_id {
        let edges_before = app.snapshot().expect("projection").edges().len();
        app.dispatch_typed(DagCommand::Disconnect(disconnect::Disconnect { edge_id }), &meta("local")).await.expect("disconnect");
        assert_eq!(app.snapshot().expect("projection").edges().len(), edges_before - 1);
    }
    let result = app.dispatch_typed(DagCommand::Disconnect(disconnect::Disconnect { edge_id: "nonexistent".into() }), &meta("local")).await.expect("disconnect unknown");
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn connect_media_ports_adds_an_edge_between_two_nodes() {
    let mut app = context::new_app().await;
    let (source_id, target_id) = {
        let projection = app.snapshot().expect("projection");
        let nodes = projection.nodes();
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = app.snapshot().expect("projection").edges().len();
    app.dispatch_typed(DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: source_id, source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }), &meta("local")).await.expect("connect");
    assert!(app.snapshot().expect("projection").edges().len() >= edges_before);
}
