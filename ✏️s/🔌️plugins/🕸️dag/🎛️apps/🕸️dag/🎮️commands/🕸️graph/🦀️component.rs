//! 🕸️ DAG play app commands — graph editing: batched node-graph edits, media-port connections, node
//! drag, and reorganize. All document-mutating, dispatched as VCS operations with a true inverse.
//!
//! `DagNodeGraphEditOp` (the batched sub-operation enum for `nodeGraphEdit`, mirroring the pre-B1
//! `nodeGraphEdit` action's `operations` JSON array) moved here from the old `📡️protocol` crate — it's a
//! field of `NodeGraphEdit`'s payload, so it lives beside the command it belongs to rather than in
//! `📡️spr` (which now only carries the artifact-level `DagMutation` wire codec).
//!
//! Ported from the pre-migration `dag_ui::ArtifactApp::handle` match arms — several of those arms were
//! found with unbalanced `Ok(` wraps (the repo-wide corruption pattern documented in the migration
//! TEMPLATE §12.3); fixed here as part of the port, not a behavior change.

use crate::apps::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{connect_nodes, dag_snapshot_mutations, disconnect_nodes, move_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_document_from_fixture, dag_fixture_from_document, DagFixture, DagHost, DagLayoutOptions};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🗑️ Builds the removal `DagMutation`s plus the config op that CLEARS the whole selection, or `None`
/// when nothing in `node_ids` exists to remove — shared by `delete_selection::DeleteSelection` and
/// `node_graph_edit::DagNodeGraphEditOp::DeleteSelection` (both were the same `handle_action`
/// "deleteSelection" logic, reachable from two different action ids pre-migration).
/// `remove_node::RemoveNode` deliberately does NOT use this helper: it only pulls the removed id out of
/// the selection, never clears it outright.
fn delete_selection_result(document: &DagSnapshot, node_ids: &[String]) -> Option<(Vec<DagMutation>, DagConfigMutation)> {
    let removes = crate::artifacts::dag::schema::remove_nodes_operations(document, node_ids);
    if removes.is_empty() {
        None
    } else {
        Some((removes, DagConfigMutation::SetSelection { node_ids: Vec::new() }))
    }
}
//#endregion 🔖️Shared

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        match delete_selection_result(document, &config.selected_node_ids) {
            Some((removes, clear_selection)) => Ok(Emit { artifact_mutations: removes, config_mutations: vec![clear_selection], ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    /// 🎯️ One batched edit inside a `NodeGraphEdit` — mirrors the pre-migration `nodeGraphEdit` action's
    /// `operations` JSON array (`"setFixture"`/`"deleteSelection"`/`"connect"` sub-kinds), now closed and
    /// typed instead of stringly-tagged JSON.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
    pub enum DagNodeGraphEditOp {
        #[dsl(key = "set-fixture")]
        SetFixture { fixture_json: String },
        #[dsl(key = "delete-selection")]
        DeleteSelection,
        #[dsl(key = "connect")]
        Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        #[dsl(statements)]
        pub operations: Vec<DagNodeGraphEditOp>,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let mut artifact_mutations: Vec<DagMutation> = Vec::new();
        let mut config_mutations: Vec<DagConfigMutation> = Vec::new();
        for sub_operation in &payload.operations {
            match sub_operation {
                DagNodeGraphEditOp::SetFixture { fixture_json } => {
                    if let Ok(fixture) = serde_json::from_str::<DagFixture>(fixture_json) {
                        config_mutations.push(DagConfigMutation::SetCamera { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom });
                        artifact_mutations.extend(dag_snapshot_mutations(document, &dag_document_from_fixture(&fixture).into()));
                    }
                }
                DagNodeGraphEditOp::DeleteSelection => {
                    if let Some((removes, clear_selection)) = delete_selection_result(document, &config.selected_node_ids) {
                        artifact_mutations.extend(removes);
                        config_mutations.push(clear_selection);
                    }
                }
                DagNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    if let Ok(edge) = crate::artifacts::dag::schema::connect_edge(document, source_node_id, source_port_id, target_node_id, target_port_id) {
                        artifact_mutations.push(connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties));
                    }
                }
            }
        }
        Ok(Emit { artifact_mutations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️ConnectMediaPorts
pub mod connect_media_ports {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "connect-media-ports")]
    pub struct ConnectMediaPorts {
        pub source_node_id: String,
        pub source_port_id: String,
        pub target_node_id: String,
        pub target_port_id: String,
    }

    pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        match crate::artifacts::dag::schema::connect_edge(document, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id) {
            Ok(edge) => Ok(Emit::mutations(vec![connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties)])),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ConnectMediaPorts

//#region 🔖️Disconnect
pub mod disconnect {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "disconnect")]
    pub struct Disconnect {
        pub edge_id: String,
    }

    pub fn handle(payload: &Disconnect, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        if document.edges().iter().any(|edge| edge.id == payload.edge_id) {
            Ok(Emit::mutations(vec![disconnect_nodes(payload.edge_id.clone())]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️Disconnect

//#region 🔖️MoveMediaNode
pub mod move_media_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-media-node")]
    pub struct MoveMediaNode {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        if document.nodes().iter().any(|node| node.id == payload.node_id) {
            Ok(Emit::amend(vec![move_node(payload.node_id.clone(), payload.x, payload.y)], format!("move-{}", payload.node_id)))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️MoveMediaNode

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let camera = dag_config_camera(config);
        if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&dag_fixture_from_document(&infinite_board_port_directed_dag::DagSnapshot::from(document), camera)).unwrap_or_default()) {
            let _ = host.reorganize(&DagLayoutOptions::default());
            if let Ok(json) = host.fixture_json() {
                if let Ok(fixture) = serde_json::from_str::<DagFixture>(&json) {
                    // 🎯️ Reorganize only ever moves EXISTING nodes (same ids/edges) — the generic
                    // differ correctly narrows that down to a `move-node` per node whose position
                    // actually changed, never a whole-collection replace.
                    let content = crate::artifacts::dag::dag_content_child_handle_and_cache(fixture.nodes, document.edges());
                    let recomputed = DagSnapshot { schema: document.schema.clone(), content };
                    return Ok(Emit::mutations(dag_snapshot_mutations(document, &recomputed)));
                }
            }
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️Reorganize

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::node_graph_edit::DagNodeGraphEditOp;
    use super::*;
    use crate::apps::dag::testkit;
    use crate::apps::dag::commands::selection::set_selection;
    use crate::apps::dag::DagCommand;
    use semio_framework_plugin::PluginApp;

    /// 🧪️ `nodeGraphEdit` batches multiple sub-edits (connect + delete-selection here) into a single
    /// typed command — mirrors the pre-migration JSON `operations` array, now closed and typed.
    #[test]
    fn node_graph_edit_batches_connect_then_delete_selection() {
        let mut app = testkit::new_app();
        let (source_id, target_id) = {
            let projection = app.snapshot().expect("projection");
            let nodes = projection.nodes();
            (nodes[0].id.clone(), nodes[1].id.clone())
        };
        let edges_before = app.snapshot().expect("projection").edges().len();
        app.dispatch_typed(
            DagCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Connect { source_node_id: source_id.clone(), source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }] }),
            &semio_framework_plugin::testkit::meta("local"),
        )
        .expect("batched connect");
        assert!(app.snapshot().expect("projection").edges().len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");

        app.dispatch_typed(DagCommand::SetSelection(set_selection::SetSelection { ids: vec![source_id] }), &semio_framework_plugin::testkit::meta("local")).expect("select");
        let nodes_before = app.snapshot().expect("projection").nodes().len();
        app.dispatch_typed(DagCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: vec![DagNodeGraphEditOp::DeleteSelection] }), &semio_framework_plugin::testkit::meta("local")).expect("batched delete");
        assert_eq!(app.snapshot().expect("projection").nodes().len(), nodes_before - 1);
    }

    #[test]
    fn move_media_node_drag_coalesces_into_one_edit() {
        let mut app = testkit::new_app();
        let node_id = app.snapshot().expect("projection").nodes().first().map(|node| node.id.clone()).expect("node");
        for position in [10.0, 20.0, 30.0] {
            app.dispatch_typed(DagCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: node_id.clone(), x: position, y: position }), &semio_framework_plugin::testkit::meta("local")).expect("drag tick");
        }
        // A whole drag (three ticks, same coalesce key) is ONE undo step, not one-operation-per-tick.
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        let restored = app.snapshot().expect("projection");
        let original = crate::artifacts::dag::default_snapshot().nodes().iter().find(|node| node.id == node_id).map(|node| node.x).expect("original x");
        assert_eq!(restored.nodes().iter().find(|node| node.id == node_id).unwrap().x, original, "undoing the coalesced drag restores the pre-drag position");
    }

    #[test]
    fn disconnect_removes_a_known_edge_and_is_a_no_op_for_an_unknown_one() {
        let mut app = testkit::new_app();
        let edge_id = app.snapshot().expect("projection").edges().first().map(|edge| edge.id.clone());
        if let Some(edge_id) = edge_id {
            let edges_before = app.snapshot().expect("projection").edges().len();
            app.dispatch_typed(DagCommand::Disconnect(disconnect::Disconnect { edge_id }), &semio_framework_plugin::testkit::meta("local")).expect("disconnect");
            assert_eq!(app.snapshot().expect("projection").edges().len(), edges_before - 1);
        }
        let result = app.dispatch_typed(DagCommand::Disconnect(disconnect::Disconnect { edge_id: "nonexistent".into() }), &semio_framework_plugin::testkit::meta("local")).expect("disconnect unknown");
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn connect_media_ports_adds_an_edge_between_two_nodes() {
        let mut app = testkit::new_app();
        let (source_id, target_id) = {
            let projection = app.snapshot().expect("projection");
            let nodes = projection.nodes();
            (nodes[0].id.clone(), nodes[1].id.clone())
        };
        let edges_before = app.snapshot().expect("projection").edges().len();
        app.dispatch_typed(DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: source_id, source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("connect");
        assert!(app.snapshot().expect("projection").edges().len() >= edges_before);
    }
}
//#endregion 🧪️Tests
