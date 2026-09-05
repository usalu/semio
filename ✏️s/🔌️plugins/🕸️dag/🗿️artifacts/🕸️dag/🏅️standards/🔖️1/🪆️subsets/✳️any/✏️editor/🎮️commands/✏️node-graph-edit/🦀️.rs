//! 🕸️ 🕸️ DAG play app commands command — `node-graph-edit`.

use crate::artifacts::dag::mutations::{connect_nodes, dag_snapshot_mutations};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::commands::delete_selection::delete_selection_result;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use infinite_board_port_directed_dag::{dag_document_from_fixture, DagFixture};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🎯️ One batched edit inside a `NodeGraphEdit` — mirrors the pre-migration `nodeGraphEdit` action's
/// `operations` JSON array (`"setFixture"`/`"deleteSelection"`/`"connect"` sub-kinds), now closed and
/// typed instead of stringly-tagged JSON.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub enum DagNodeGraphEditOp {
    #[dsl(key = "set-fixture")]
    SetFixture { fixture_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<DagNodeGraphEditOp>,
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no
/// `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only
/// through that macro-generated path (`DagPlayApp::handle` always routes this command through `apply`
/// below instead), so its `DeleteSelection` sub-op degrades to treating the selection as empty.
pub async fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    apply_to(payload, doc, cfg, &[])
}

pub async fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    apply_to(payload, doc, cfg, &interaction.selection("graph").ids)
}

async fn apply_to(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, selected: &[String]) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut artifact_mutations: Vec<DagMutation> = Vec::new();
    let mut config_mutations: Vec<DagConfigMutation> = Vec::new();
    for sub_operation in &payload.operations {
        match sub_operation {
            DagNodeGraphEditOp::SetFixture { fixture_json } => {
                if let Ok(fixture) = dsl::json::from_json_str::<DagFixture>(fixture_json) {
                    config_mutations.push(DagConfigMutation::SetCamera { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom });
                    artifact_mutations.extend(dag_snapshot_mutations(document, &dag_document_from_fixture(&fixture).into()));
                }
            }
            DagNodeGraphEditOp::DeleteSelection => {
                if let Some(removes) = delete_selection_result(document, selected) {
                    artifact_mutations.extend(removes);
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::DagNodeGraphEditOp;
    use super::*;
    use crate::editor::dag::commands::{connect_media_ports, disconnect, move_media_node};
    use crate::editor::dag::testkit;
    use crate::editor::dag::DagCommand;
    use semio_framework_plugin::{testkit::meta, InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};
    use serde_json::json;

    /// 🧪️ `nodeGraphEdit` batches multiple sub-edits (connect + delete-selection here) into a single
    /// typed command — mirrors the pre-migration JSON `operations` array, now closed and typed. The
    /// `graph` domain's live selection (populated via the framework's own `interactionSelect` action —
    /// the only way a downstream crate can populate a genuine `InteractionView`) drives the batched
    /// delete-selection sub-op — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    #[semio_framework_async_macros::async_test]
    async fn node_graph_edit_batches_connect_then_delete_selection() {
        let mut app = testkit::new_app_with_registry();
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
        .expect("batched connect");
        assert!(app.snapshot().expect("projection").edges().len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");

        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: source_id }]).expect("targets");
        app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" })), &meta("local")).expect("interactionSelect");
        let nodes_before = app.snapshot().expect("projection").nodes().len();
        app.dispatch_typed(DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::DeleteSelection] }), &meta("local")).expect("batched delete");
        assert_eq!(app.snapshot().expect("projection").nodes().len(), nodes_before - 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_media_node_drag_coalesces_into_one_edit() {
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

    #[semio_framework_async_macros::async_test]
    async fn disconnect_removes_a_known_edge_and_is_a_no_op_for_an_unknown_one() {
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

    #[semio_framework_async_macros::async_test]
    async fn connect_media_ports_adds_an_edge_between_two_nodes() {
        let mut app = testkit::new_app();
        let (source_id, target_id) = {
            let projection = app.snapshot().expect("projection");
            let nodes = projection.nodes();
            (nodes[0].id.clone(), nodes[1].id.clone())
        };
        let edges_before = app.snapshot().expect("projection").edges().len();
        app.dispatch_typed(
            DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: source_id, source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }),
            &semio_framework_plugin::testkit::meta("local"),
        )
        .expect("connect");
        assert!(app.snapshot().expect("projection").edges().len() >= edges_before);
    }
}
//#endregion 🧪️Tests
