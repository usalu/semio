//! 🕸️ 🕸️ DAG play app commands command — `node-graph-edit`: the one verb both node-graph hosts dispatch for graph edits.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::editor::dag::DAG_PLAY_APP_ID;
use crate::{DagMutation, DagSnapshot, SemioGraphMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{node_drag_emit, node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow, NodePortSide};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 🎯️ One batched edit inside a `NodeGraphEdit`, closed and typed — the shared node-graph gesture record vocabulary every
/// renderer dispatches (design §13.3): each row names its entities by id and carries an intent.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub enum DagNodeGraphEditOp {
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect")]
    Disconnect { synapse_id: String },
    #[dsl(key = "move")]
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    #[dsl(key = "set-slider")]
    SetSlider { widget_id: String, value: f64 },
    #[dsl(key = "insert-port")]
    InsertPort { node_id: String, side: String, index: u32 },
    #[dsl(key = "delete")]
    Delete { node_ids: Vec<String>, synapse_ids: Vec<String> },
}

impl From<NodeGraphEditRow> for DagNodeGraphEditOp {
    fn from(row: NodeGraphEditRow) -> Self {
        match row {
            NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => Self::Connect { source_node_id, source_port_id, target_node_id, target_port_id },
            NodeGraphEditRow::Disconnect { synapse_id } => Self::Disconnect { synapse_id },
            NodeGraphEditRow::Move(record) => Self::Move { gesture_id: record.gesture_id, node_ids: record.node_ids, dx: record.dx, dy: record.dy },
            NodeGraphEditRow::SetSlider { widget_id, value } => Self::SetSlider { widget_id, value },
            NodeGraphEditRow::InsertPort { node_id, side, index } => Self::InsertPort { node_id, side: if side == NodePortSide::Input { "input" } else { "output" }.to_string(), index },
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => Self::Delete { node_ids, synapse_ids },
        }
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<DagNodeGraphEditOp>,
}

impl NodeGraphEdit {
    /// 🌉️ Decodes the `{operations: [...]}` arguments a host dispatches through the ONE shared row decoder of
    /// `🛠️tool-machine` (a press's top-level `gesture`/`commit`/`abort` belong to the framework scrub machine). Any malformed
    /// row refuses the whole batch by name.
    pub fn from_action_args(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let refuse = |message: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("dag.node-graph-edit.malformed"), message);
        let rows = node_graph_edit_rows(args.ok_or_else(|| refuse("nodeGraphEdit needs its operations".into()))?).map_err(refuse)?;
        Ok(Self { operations: rows.into_iter().map(DagNodeGraphEditOp::from).collect() })
    }
}

/// 🧾️ Every row yields its id-keyed graph child leaf against the composed `content` child (design §13.3, §20.15):
/// - `move` — the RELATIVE `drag-nodes` leaf over the record's nodes the graph holds, committed through the ONE node-drag
///   machine as ONE composed-child tool transaction;
/// - `setSlider` — the ABSOLUTE `set-node-property` of the slider's `value` (plus `resize-node` when the widget refits); a
///   dragged knob carries its press as the dispatch's top-level `gesture`/`commit`, so the framework scrub machine keeps
///   every tick provisional and commits the release as ONE edit;
/// - `connect`, `disconnect` — one edge each; `delete` — the named wires, then every named node with the wires it holds;
/// - `insertPort` — refused: no dag host inserts variadic ports.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let scene = crate::dag_scene(doc)?;
    let mut leaves: Vec<SemioGraphMutation> = Vec::new();
    let mut gesture: Option<String> = None;
    for sub_operation in &payload.operations {
        match sub_operation {
            DagNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                if let Ok(edge) = crate::schema::connect_edge(&scene, source_node_id, source_port_id, target_node_id, target_port_id) {
                    leaves.push(crate::create_edge_leaf(&edge));
                }
            }
            DagNodeGraphEditOp::Disconnect { synapse_id } => {
                if scene.edges.iter().any(|edge| &edge.id == synapse_id) {
                    leaves.push(crate::delete_edge_leaf(synapse_id));
                }
            }
            DagNodeGraphEditOp::Move { gesture_id, node_ids, dx, dy } => {
                let record = NodeDragRecord { gesture_id: gesture_id.clone(), node_ids: node_ids.iter().filter(|id| scene.nodes.iter().any(|node| &node.id == *id)).cloned().collect(), dx: *dx, dy: *dy };
                if record.moves() {
                    gesture.get_or_insert_with(|| record.gesture_id.clone());
                    leaves.push(crate::drag_nodes_leaf(record.node_ids, record.dx, record.dy));
                }
            }
            DagNodeGraphEditOp::SetSlider { widget_id, value } => {
                leaves.extend(scene.nodes.iter().filter(|node| &node.id == widget_id).flat_map(|node| crate::schema::node_field_leaves(node, "value", &value.to_string())));
            }
            DagNodeGraphEditOp::InsertPort { node_id, .. } => {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("dag.node-graph-edit.unsupported"), format!("dag node {node_id:?} takes no inserted port")));
            }
            DagNodeGraphEditOp::Delete { node_ids, synapse_ids } => {
                let removes = crate::schema::remove_nodes_leaves(&scene, node_ids);
                let cut: Vec<String> = removes.iter().filter_map(|leaf| if let SemioGraphMutation::DeleteEdge(payload) = leaf { Some(payload.id.value.clone()) } else { None }).collect();
                leaves.extend(synapse_ids.iter().filter(|id| scene.edges.iter().any(|edge| &edge.id == *id) && !cut.contains(id)).map(|id| crate::delete_edge_leaf(id)));
                leaves.extend(removes);
            }
        }
    }
    Ok(match gesture {
        Some(gesture) => dag_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, &gesture, leaves),
        None => crate::dag_child_emit(doc.snapshot, leaves),
    })
}

/// 🛠️ ONE composed-child tool transaction of graph `leaves` through the ONE node-drag machine of `🛠️tool-machine` (design
/// §12, §13.3): the ref minted from the admission's authoring seed, the host clock and `<appId>#<verb>`, for the press
/// `gesture`, stamped on the `content` member edit. A view without command authority publishes the leaves plainly; nothing
/// yielded is the empty emit (zero trace).
pub(crate) fn dag_node_drag_emit(doc: &ArtifactView<'_, DagSnapshot>, verb: &str, gesture: &str, leaves: Vec<SemioGraphMutation>) -> Emit<DagMutation, DagConfigMutation> {
    let drag = node_drag_emit(DAG_PLAY_APP_ID, verb, doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()), gesture, leaves);
    Emit::node_drag_child::<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot, _>(drag, "content", &doc.snapshot.content.child_id)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
