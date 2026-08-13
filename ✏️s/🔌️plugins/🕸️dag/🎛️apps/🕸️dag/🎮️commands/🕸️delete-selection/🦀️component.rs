//! 🕸️ 🕸️ DAG play app commands command — `delete-selection`.

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
//#endregion 🔖️DeleteSelection

//#region 🔖️NodeGraphEdit
//#endregion 🔖️NodeGraphEdit

//#region 🔖️ConnectMediaPorts
//#endregion 🔖️ConnectMediaPorts

//#region 🔖️Disconnect
//#endregion 🔖️Disconnect

//#region 🔖️MoveMediaNode
//#endregion 🔖️MoveMediaNode

//#region 🔖️Reorganize
//#endregion 🔖️Reorganize

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
