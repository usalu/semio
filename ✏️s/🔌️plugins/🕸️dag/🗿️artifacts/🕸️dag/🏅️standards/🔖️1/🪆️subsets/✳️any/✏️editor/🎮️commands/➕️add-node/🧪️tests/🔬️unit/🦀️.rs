use super::*;
use crate::editor::dag::commands::{patch_dag_nodes, remove_node, rename_dag_node};
use crate::editor::dag::unit_tests::context;
use crate::editor::dag::DagCommand;
use semio_framework_artifact_infinite_dag::DagNodeKind;
use semio_framework_plugin::artifact_app_laws::{meta, settle_history_verb};
use semio_framework_plugin::PluginApp;

// 🧪️ Every fixture below drives the MOUNTED wrapper: `dispatch_typed` only QUEUES a typed
// operation, so `context::dispatch` settles its publication before the document is read, and each
// test closes its app through the bounded close protocol (a live `ArtifactStore` asserts in `Drop`).

#[semio_framework_async_macros::async_test]
async fn add_node_action_updates_document_with_the_new_node() {
    let mut app = context::new_app().await;
    context::dispatch(&mut app, DagCommand::AddNode(AddNode { kind: "slider".into(), x: None, y: None })).await;
    let nodes = context::live_scene(&app).await.nodes;
    assert!(nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn rename_dag_node_rewrites_nodes_and_edges() {
    let mut app = context::new_app().await;
    let old_id = context::live_scene(&app).await.nodes.first().map(|node| node.id.clone()).expect("node");
    context::dispatch(&mut app, DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: old_id.clone(), value: "renamed-node".into() })).await;
    let scene = context::live_scene(&app).await;
    let nodes = &scene.nodes;
    assert!(scene.edges.iter().all(|edge| crate::schema::split_endpoint(&edge.source).0 != old_id && crate::schema::split_endpoint(&edge.target).0 != old_id), "the graph cascades the id change to every endpoint");
    assert!(nodes.iter().any(|node| node.id == "renamed-node"));
    assert!(nodes.iter().all(|node| node.id != old_id));
    context::close(&mut app);
}

/// 🧪️ A mounted wrapper never answers `InvocationResult.mutations` (the edit lands through the
/// retained publication lane, not the invocation answer), so "no-op" is proved where it is
/// observable: the projected document is byte-identical before and after.
#[semio_framework_async_macros::async_test]
async fn rename_dag_node_is_a_no_op_for_an_empty_or_duplicate_id() {
    let mut app = context::new_app().await;
    let (first_id, second_id) = {
        let nodes = context::live_scene(&app).await.nodes;
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let before = context::live_scene(&app).await;
    context::dispatch(&mut app, DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id.clone(), value: "   ".into() })).await;
    assert_eq!(context::live_scene(&app).await, before, "a blank rename target leaves the document untouched");
    context::dispatch(&mut app, DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id, value: second_id })).await;
    assert_eq!(context::live_scene(&app).await, before, "renaming onto an existing id leaves the document untouched");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn remove_node_deletes_node_and_connected_edges() {
    let mut app = context::new_app().await;
    let node_id = context::live_scene(&app).await.nodes.first().map(|node| node.id.clone()).expect("node");
    context::dispatch(&mut app, DagCommand::RemoveNode(remove_node::RemoveNode { node_id: node_id.clone() })).await;
    let document = context::live_scene(&app).await;
    assert!(document.nodes.iter().all(|node| node.id != node_id));
    assert!(document.edges.iter().all(|edge| {
        let (from, _) = crate::schema::split_endpoint(&edge.source);
        let (to, _) = crate::schema::split_endpoint(&edge.target);
        from != node_id && to != node_id
    }));
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn add_node_then_undo_restores_document() {
    let mut app = context::new_app().await;
    let before = context::live_scene(&app).await.nodes.len();
    context::dispatch(&mut app, DagCommand::AddNode(AddNode { kind: "note".into(), x: None, y: None })).await;
    assert_eq!(context::live_scene(&app).await.nodes.len(), before + 1);
    settle_history_verb(&mut app, "undo", meta("local").instance_id).await;
    assert_eq!(context::live_scene(&app).await.nodes.len(), before);
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn patch_slider_value_lands_as_absolute_child_property_edits() {
    let mut app = context::new_app().await;
    context::dispatch(&mut app, DagCommand::AddNode(AddNode { kind: "slider".into(), x: None, y: None })).await;
    let node_id = context::live_scene(&app).await.nodes.iter().find(|node| matches!(node.kind, DagNodeKind::Slider { .. })).map(|node| node.id.clone()).expect("slider");
    for value in [1.0, 2.0, 5.0] {
        context::dispatch(&mut app, DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec![node_id.clone()], field: "value".into(), value: value.to_string() })).await;
    }
    let slider_value = context::live_scene(&app)
        .await
        .nodes
        .into_iter()
        .find(|node| node.id == node_id)
        .and_then(|node| match &node.kind {
            DagNodeKind::Slider { value, .. } => Some(*value),
            _ => None,
        })
        .expect("slider value");
    assert_eq!(slider_value, 5.0);
    context::close(&mut app);
}
