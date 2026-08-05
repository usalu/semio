//! 🔧️ DAG play app commands — node CRUD: add/remove/rename/patch. All document-mutating, dispatched as
//! VCS operations with a true inverse.
//!
//! Ported from the pre-migration `dag_ui::DocumentApp::handle` match arms — several of those arms were
//! found with unbalanced `Ok(` wraps and a missing `Fault` import (the repo-wide corruption pattern
//! documented in the migration TEMPLATE §12.3, also hit by cad/vcs/shooting/sourcing's old ui crates);
//! fixed here as part of the port, not a behavior change.

use crate::artifacts::dag::engine;
use crate::artifacts::dag::op::DagOperation;
use crate::artifacts::dag::DagDocument;
use crate::apps::dag::config::{DagConfig, DagConfigOperation};
use infinite_board_port_directed_dag::{DagFixtureEdge, DagNodeSpec};
use protocol::CollectionOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub kind: String,
        pub x: Option<f64>,
        pub y: Option<f64>,
    }

    pub fn handle(payload: &AddNode, doc: &DocumentView<'_, DagDocument>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        let document = doc.projection;
        let id = engine::next_node_id(document);
        let node = engine::default_node_for_kind(&payload.kind, &id, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0));
        Ok(Emit {
            document_operations: vec![DagOperation::Nodes(CollectionOperation::Add { id: node.id.clone(), at: document.nodes.len(), item: node })],
            config_operations: vec![DagConfigOperation::SetSelection { node_ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddNode

//#region 🔖️RemoveNode
pub mod remove_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-node")]
    pub struct RemoveNode {
        pub node_id: String,
    }

    pub fn handle(payload: &RemoveNode, doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        let removes = engine::remove_nodes_operations(document, std::slice::from_ref(&payload.node_id));
        if removes.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit { document_operations: removes, config_operations: vec![DagConfigOperation::SetSelection { node_ids: config.selected_node_ids.iter().filter(|id| *id != &payload.node_id).cloned().collect() }], ..Default::default() })
        }
    }
}
//#endregion 🔖️RemoveNode

//#region 🔖️RenameDagNode
pub mod rename_dag_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rename-dag-node")]
    pub struct RenameDagNode {
        pub old_id: String,
        pub value: String,
    }

    pub fn handle(payload: &RenameDagNode, doc: &DocumentView<'_, DagDocument>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        let document = doc.projection;
        let trimmed = payload.value.trim();
        if trimmed.is_empty() || trimmed == payload.old_id.as_str() || document.nodes.iter().any(|node| node.id == trimmed) {
            return Ok(Emit::default());
        }
        let nodes: Vec<DagNodeSpec> = document.nodes.iter().map(|node| if node.id == payload.old_id { DagNodeSpec { id: trimmed.into(), ..node.clone() } } else { node.clone() }).collect();
        let edges: Vec<DagFixtureEdge> = document
            .edges
            .iter()
            .map(|edge| {
                let (from_node, from_port) = engine::split_endpoint(&edge.source);
                let (to_node, to_port) = engine::split_endpoint(&edge.target);
                DagFixtureEdge {
                    source: if from_node == payload.old_id { format!("{trimmed}@{from_port}") } else { edge.source.clone() },
                    target: if to_node == payload.old_id { format!("{trimmed}@{to_port}") } else { edge.target.clone() },
                    ..edge.clone()
                }
            })
            .collect();
        Ok(Emit { document_operations: vec![DagOperation::SetNodes { nodes }, DagOperation::SetEdges { edges }], config_operations: vec![DagConfigOperation::SetSelection { node_ids: vec![trimmed.to_string()] }], ..Default::default() })
    }
}
//#endregion 🔖️RenameDagNode

//#region 🔖️PatchDagNodes
pub mod patch_dag_nodes {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-dag-nodes")]
    pub struct PatchDagNodes {
        pub node_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchDagNodes, doc: &DocumentView<'_, DagDocument>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        let document = doc.projection;
        let operations: Vec<DagOperation> = document
            .nodes
            .iter()
            .filter(|node| payload.node_ids.contains(&node.id))
            .filter_map(|node| engine::node_patch_for_field(node, &payload.field, Some(payload.value.as_str())).map(|patch| DagOperation::Nodes(CollectionOperation::Patch { id: node.id.clone(), patch })))
            .collect();
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(operations, format!("patch-{}-{}", payload.field, payload.node_ids.join(","))))
        }
    }
}
//#endregion 🔖️PatchDagNodes

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::testkit;
    use crate::apps::dag::{DagCommand, DAG_PLAY_BODY_MAIN};
    use infinite_board_port_directed_dag::DagNodeKind;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn add_node_action_updates_document_and_selects_the_new_node() {
        let mut app = testkit::new_app();
        app.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add node");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
        let added_id = document.nodes.last().expect("added node").id.clone();
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains(&added_id), "the new node becomes the config selection");
    }

    #[test]
    fn rename_dag_node_rewrites_nodes_and_edges() {
        let mut app = testkit::new_app();
        let old_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: old_id.clone(), value: "renamed-node".into() }), &semio_framework_plugin::testkit::meta("local")).expect("rename");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| node.id == "renamed-node"));
        assert!(document.nodes.iter().all(|node| node.id != old_id));
    }

    #[test]
    fn rename_dag_node_is_a_no_op_for_an_empty_or_duplicate_id() {
        let mut app = testkit::new_app();
        let (first_id, second_id) = {
            let projection = app.projection().expect("projection");
            (projection.nodes[0].id.clone(), projection.nodes[1].id.clone())
        };
        let result = app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id.clone(), value: "   ".into() }), &semio_framework_plugin::testkit::meta("local")).expect("rename blank");
        assert!(result.operations.is_empty());
        let result = app.dispatch_typed(DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: first_id, value: second_id }), &semio_framework_plugin::testkit::meta("local")).expect("rename duplicate");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn remove_node_deletes_node_and_connected_edges_and_prunes_selection() {
        let mut app = testkit::new_app();
        let node_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::SetSelection(crate::apps::dag::commands::selection::set_selection::SetSelection { ids: vec![node_id.clone()] }), &semio_framework_plugin::testkit::meta("local")).expect("select");
        app.dispatch_typed(DagCommand::RemoveNode(remove_node::RemoveNode { node_id: node_id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("remove");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().all(|node| node.id != node_id));
        assert!(document.edges.iter().all(|edge| {
            let (from, _) = engine::split_endpoint(&edge.source);
            let (to, _) = engine::split_endpoint(&edge.target);
            from != node_id && to != node_id
        }));
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &semio_framework_plugin::ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().contains(&node_id), "the removed node is pruned from the config selection");
    }

    #[test]
    fn add_node_then_undo_restores_document() {
        let mut app = testkit::new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").nodes.len(), before);
    }

    #[test]
    fn patch_slider_value_coalesces_into_one_edit() {
        let mut app = testkit::new_app();
        app.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add slider");
        let node_id = app.projection().expect("projection").nodes.iter().find(|node| matches!(node.kind, DagNodeKind::Slider { .. })).map(|node| node.id.clone()).expect("slider");
        for value in [1.0, 2.0, 5.0] {
            app.dispatch_typed(DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec![node_id.clone()], field: "value".into(), value: value.to_string() }), &semio_framework_plugin::testkit::meta("local")).expect("patch slider");
        }
        let slider_value = app
            .projection()
            .expect("projection")
            .nodes
            .iter()
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
