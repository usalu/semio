//! ✏️ ✏️ S Studio app command — `node-graph-edit`: the one verb both node-graph hosts dispatch for workflow graph edits.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::engine::space::S_PLAY_APP_ID;
use semio_framework_artifact_workflow_workflow::MoveNodes;
use semio_framework_os::workflow::{DisconnectEdge, RemoveNode};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{node_drag_emit, NodeDragRecord, NodeGraphEditRow};

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 🧾️ `operations_json` is the JSON array of the host's `nodeGraphEdit` rows — the shared node-graph row vocabulary
/// (design §13.3) — exactly as both hosts dispatch them.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🧾️ Decodes one host row through the ONE shared node-graph row decoder of `🛠️tool-machine`; the `setSlider` and
/// `insertPort` rows a studio workflow has no widget for are refused by name, so the whole batch is refused.
pub(crate) fn space_node_graph_row(row: &semio_framework_pack_json::Value) -> Result<NodeGraphEditRow, Fault> {
    match NodeGraphEditRow::from_row(&semio_framework_pack_json::to_dsl_value(row)).map_err(|reason| Fault::from(format!("space nodeGraphEdit refusal: {reason}")))? {
        NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => Err(Fault::from("space nodeGraphEdit refusal: a studio workflow has no sliders and no variadic ports")),
        row => Ok(row),
    }
}

/// 🧾️ Every row yields its id-keyed leaf against the committed document:
/// - `move` — the RELATIVE `move-nodes` leaf over the record's nodes the workflow holds, committed through the ONE node-drag
///   machine as ONE tool transaction (design §13.3);
/// - `connect` — the negotiated edge between two ports; `disconnect` — one edge;
/// - `delete` — the named edges, then the named app instances (each with the edges it holds).
pub(crate) async fn edit(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let rows: Vec<NodeGraphEditRow> = match semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().cloned()) {
        Some(rows) => rows.iter().map(space_node_graph_row).collect::<Result<_, _>>()?,
        None => return Err(Fault::from("space nodeGraphEdit operations must be a JSON array")),
    };
    let mut artifact_mutations = Vec::new();
    let mut effects = Vec::new();
    let mut gesture: Option<String> = None;
    for row in &rows {
        match row {
            NodeGraphEditRow::Move(record) => {
                let node_ids: Vec<String> = record.node_ids.iter().filter(|id| projection.graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
                let record = NodeDragRecord { node_ids, ..record.clone() };
                if record.moves() {
                    gesture.get_or_insert_with(|| record.gesture_id.clone());
                    artifact_mutations.push(WorkflowMutation::MoveNodes(MoveNodes { node_ids: record.node_ids, dx: record.dx, dy: record.dy }));
                }
            }
            NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => match crate::engine::space::negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id).await {
                Ok(contract) => artifact_mutations.push(crate::engine::space::connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract).await),
                Err(effect) => effects.push(effect),
            },
            NodeGraphEditRow::Disconnect { synapse_id } => {
                if projection.graph.edges.iter().any(|edge| &edge.id == synapse_id) {
                    artifact_mutations.push(WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: synapse_id.clone() }));
                }
            }
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                artifact_mutations.extend(projection.graph.edges.iter().filter(|edge| synapse_ids.contains(&edge.id)).map(|edge| WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge.id.clone() })));
                artifact_mutations.extend(projection.graph.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| WorkflowMutation::RemoveNode(RemoveNode { node_id: node.id.clone() })));
            }
            NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => {}
        }
    }
    let emit = match gesture {
        Some(gesture) => space_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, &gesture, artifact_mutations),
        None => Emit::mutations(artifact_mutations),
    };
    Ok(Emit { effects, ..emit })
}

/// 🛠️ ONE tool transaction of `leaves` through the ONE node-drag machine of `🛠️tool-machine` (design §13.3): the ref
/// minted from the admission's authoring seed, the host clock and `<appId>#<verb>`, for the press `gesture`. A view
/// without command authority publishes the leaves plainly; nothing yielded is the empty emit (zero trace).
pub(crate) fn space_node_drag_emit(doc: &ArtifactView<'_, WorkflowSnapshot>, verb: &str, gesture: &str, leaves: Vec<WorkflowMutation>) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    node_drag_emit(S_PLAY_APP_ID, verb, doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()), gesture, leaves).into()
}

/// 🕹️ The command body, through the engine's future resolver.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    crate::engine::space::engine::resolve_future(edit(payload, doc))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
