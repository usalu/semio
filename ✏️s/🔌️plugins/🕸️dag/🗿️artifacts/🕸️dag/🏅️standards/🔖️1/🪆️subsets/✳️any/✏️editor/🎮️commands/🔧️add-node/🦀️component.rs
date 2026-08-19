//! 🔧️ 🔧️ DAG play app commands command — `add-node`.

use crate::artifacts::dag::mutations::create_node;
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🕹️ No longer auto-selects the newly-added node — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub async fn handle(payload: &AddNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let id = crate::artifacts::dag::schema::next_node_id(document);
    let node = crate::artifacts::dag::schema::default_node_for_kind(&payload.kind, &id, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0));
    Ok(Emit::mutations(vec![create_node(node)]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit;
    use crate::editor::dag::commands::{patch_dag_nodes, remove_node, rename_dag_node};
    use crate::editor::dag::DagCommand;
    use infinite_board_port_directed_dag::DagNodeKind;
    use semio_framework_plugin::PluginApp;

    #[test]
    async fn add_node_action_updates_document_with_the_new_node() {
        let mut app = testkit::new_app();
        app.dispatch_typed(DagCommand::AddNode(AddNode { kind: "slider".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add node");
        let document = app.snapshot().expect("projection");
        let nodes = document.nodes();
        assert!(nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
    }

    #[test]
    async fn rename_dag_node_rewrites_nodes_and_edges() {
        let mut app = testkit::new_app();
        let old_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: old_id.clone(), value: "renamed-node".into() }), &semio_framework_plugin::testkit::meta("local")).expect("rename");
        let document = app.snapshot().expect("projection");
        let nodes = document.nodes();
        assert!(nodes.iter().any(|node| node.id == "renamed-node"));
        assert!(nodes.iter().all(|node| node.id != old_id));
    }

    #[test]
    async fn rename_dag_node_is_a_no_op_for_an_empty_or_duplicate_id() {
        let mut app = testkit::new_app();
        let (first_id, second_id) = {
            let nodes = app.snapshot().expect("projection").nodes();
            (nodes[0].id.clone(), nodes[1].id.clone())
        };
        let result = app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id.clone(), value: "   ".into() }), &semio_framework_plugin::testkit::meta("local")).expect("rename blank");
        assert!(result.mutations.is_empty());
        let result = app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id, value: second_id }), &semio_framework_plugin::testkit::meta("local")).expect("rename duplicate");
        assert!(result.mutations.is_empty());
    }

    #[test]
    async fn remove_node_deletes_node_and_connected_edges() {
        let mut app = testkit::new_app();
        let node_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::RemoveNode(remove_node::RemoveNode { node_id: node_id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("remove");
        let document = app.snapshot().expect("projection");
        assert!(document.nodes().iter().all(|node| node.id != node_id));
        assert!(document.edges().iter().all(|edge| {
            let (from, _) = crate::artifacts::dag::schema::split_endpoint(&edge.source);
            let (to, _) = crate::artifacts::dag::schema::split_endpoint(&edge.target);
            from != node_id && to != node_id
        }));
    }

    #[test]
    async fn add_node_then_undo_restores_document() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection").nodes().len();
        app.dispatch_typed(DagCommand::AddNode(AddNode { kind: "note".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add");
        assert_eq!(app.snapshot().expect("projection").nodes().len(), before + 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("projection").nodes().len(), before);
    }

    #[test]
    async fn patch_slider_value_coalesces_into_one_edit() {
        let mut app = testkit::new_app();
        app.dispatch_typed(DagCommand::AddNode(AddNode { kind: "slider".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add slider");
        let node_id = app.snapshot().expect("projection").nodes().iter().find(|node| matches!(node.kind, DagNodeKind::Slider { .. })).map(|node| node.id.clone()).expect("slider");
        for value in [1.0, 2.0, 5.0] {
            app.dispatch_typed(DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec![node_id.clone()], field: "value".into(), value: value.to_string() }), &semio_framework_plugin::testkit::meta("local")).expect("patch slider");
        }
        let slider_value = app
            .snapshot()
            .expect("projection")
            .nodes()
            .into_iter()
            .find(|node| node.id == node_id)
            .and_then(|node| match &node.kind {
                DagNodeKind::Slider { value, .. } => Some(*value),
                _ => None,
            })
            .expect("slider value");
        assert_eq!(slider_value, 5.0);
    }
}
//#endregion 🧪️Tests
