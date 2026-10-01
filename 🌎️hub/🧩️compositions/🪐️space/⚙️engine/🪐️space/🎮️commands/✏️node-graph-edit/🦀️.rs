//! ✏️ ✏️ S Studio app command — `node-graph-edit`: the one verb both node-graph hosts dispatch for workflow graph edits.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, SpaceWindowCamera};
use crate::engine::space::S_PLAY_APP_ID;
use semio_framework_artifact_workflow_workflow::MoveNodes;
use semio_framework_os::workflow::{DisconnectEdge, MoveNode, RemoveNode};
use semio_framework_os::{apply_flow_host_snapshot_to_os_workflow, WorkflowMutation, WorkflowNode, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord, NODE_DRAG_OPERATION};

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 📏️ How far two drag offsets may differ and still be one drag: a host moves every dragged node by one world delta, and
/// recomputing `after - before` per node only differs by rounding.
const SPACE_DRAG_OFFSET_TOLERANCE: f64 = 1e-6;

/// 🧾️ `operations_json` is the JSON array of the host's `nodeGraphEdit` rows (`setHostSnapshot`, `move`, `connect`,
/// `disconnect`, `deleteSelection`), exactly as both hosts dispatch them.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🚚️ The relative `move-nodes` leaves that carry every node of `graph` to the absolute `moves`: nodes moved by one offset
/// (within [`SPACE_DRAG_OFFSET_TOLERANCE`]) share ONE leaf, in first-moved order, so a released drag of a selection is one
/// intent row and a relayout is one row per distinct offset.
pub(crate) fn space_move_leaves(nodes: &[WorkflowNode], moves: &[MoveNode]) -> Vec<WorkflowMutation> {
    let mut groups: Vec<(f64, f64, Vec<String>)> = Vec::new();
    for movement in moves {
        let Some(node) = nodes.iter().find(|node| node.id == movement.node_id) else { continue };
        let (dx, dy) = (movement.x - node.x, movement.y - node.y);
        if (dx, dy) == (0.0, 0.0) || !dx.is_finite() || !dy.is_finite() {
            continue;
        }
        match groups.iter_mut().find(|(gx, gy, _)| (gx - dx).abs() <= SPACE_DRAG_OFFSET_TOLERANCE && (gy - dy).abs() <= SPACE_DRAG_OFFSET_TOLERANCE) {
            Some((_, _, ids)) => ids.push(node.id.clone()),
            None => groups.push((dx, dy, vec![node.id.clone()])),
        }
    }
    groups.into_iter().map(|(dx, dy, node_ids)| WorkflowMutation::MoveNodes(MoveNodes { node_ids, dx, dy })).collect()
}

/// 🧾️ Every row yields its leaf against the committed document:
/// - `move` — the RELATIVE `move-nodes` leaf over the record's nodes the workflow holds, committed through the ONE node-drag
///   machine as ONE tool transaction (design §13.3);
/// - `setHostSnapshot` — the structural diff of the graph it leaves behind, its position changes as intent (`move-nodes`
///   per drag offset); a snapshot that moved nodes is a drag and commits as ONE tool transaction too; its camera is view
///   state on the window config;
/// - `connect`, `disconnect`, `deleteSelection` — one-shot structural intents.
pub(crate) async fn edit_with_selection(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let projection = doc.snapshot;
    let edit_operations = pack::parse_json(&payload.operations_json).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default();
    let mut artifact_mutations = Vec::new();
    let mut config_mutations = Vec::new();
    let mut effects = Vec::new();
    let mut gesture: Option<String> = None;
    for edit in &edit_operations {
        match edit.get("operation").and_then(pack::JsonValue::as_str).unwrap_or("") {
            "setHostSnapshot" => {
                if let Some(host_snapshot_json) = edit.get("hostSnapshotJson").and_then(pack::JsonValue::as_str) {
                    if let Some(camera) = pack::parse_json(host_snapshot_json).ok().and_then(|fixture| fixture.get("camera").cloned()).and_then(|camera| dsl::from_dsl_value::<SpaceWindowCamera>(pack::json_to_dsl_value(&camera)).ok()) {
                        config_mutations.push(SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera });
                    }
                    let (moves, structural): (Vec<WorkflowMutation>, Vec<WorkflowMutation>) = apply_flow_host_snapshot_to_os_workflow(&projection.graph, host_snapshot_json).into_iter().partition(|operation| matches!(operation, WorkflowMutation::MoveNode(_)));
                    let moves: Vec<MoveNode> = moves.into_iter().filter_map(|operation| if let WorkflowMutation::MoveNode(movement) = operation { Some(movement) } else { None }).collect();
                    let leaves = space_move_leaves(&projection.graph.nodes, &moves);
                    if !leaves.is_empty() {
                        gesture.get_or_insert_with(|| "setHostSnapshot".to_string());
                    }
                    artifact_mutations.extend(leaves);
                    artifact_mutations.extend(structural);
                }
            }
            NODE_DRAG_OPERATION => {
                if let Ok(record) = NodeDragRecord::from_row(&pack::json_to_dsl_value(edit)) {
                    let node_ids: Vec<String> = record.node_ids.iter().filter(|id| projection.graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
                    let record = NodeDragRecord { node_ids, ..record };
                    if record.moves() {
                        gesture.get_or_insert_with(|| record.gesture_id.clone());
                        artifact_mutations.push(WorkflowMutation::MoveNodes(MoveNodes { node_ids: record.node_ids, dx: record.dx, dy: record.dy }));
                    }
                }
            }
            "connect" => {
                if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) = (
                    edit.get("sourceNodeId").and_then(pack::JsonValue::as_str),
                    edit.get("sourcePortId").and_then(pack::JsonValue::as_str),
                    edit.get("targetNodeId").and_then(pack::JsonValue::as_str),
                    edit.get("targetPortId").and_then(pack::JsonValue::as_str),
                ) {
                    match crate::engine::space::negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id).await {
                        Ok(contract) => artifact_mutations.push(crate::engine::space::connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract).await),
                        Err(effect) => effects.push(effect),
                    }
                }
            }
            "disconnect" => {
                if let Some(edge_id) = edit.get("synapseId").and_then(pack::JsonValue::as_str).filter(|edge_id| projection.graph.edges.iter().any(|edge| edge.id == *edge_id)) {
                    artifact_mutations.push(WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge_id.to_string() }));
                }
            }
            "deleteSelection" => {
                for node_id in selected {
                    artifact_mutations.push(WorkflowMutation::RemoveNode(RemoveNode { node_id: node_id.clone() }));
                }
            }
            _ => {}
        }
    }
    let emit = match gesture {
        Some(gesture) => space_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, &gesture, artifact_mutations),
        None => Emit::mutations(artifact_mutations),
    };
    Emit { config_mutations, effects, ..emit }
}

/// 🛠️ ONE tool transaction of `leaves` through the ONE node-drag machine of `🛠️tool-machine` (design §13.3): the ref
/// minted from the admission's authoring seed, the host clock and `<appId>#<verb>`, for the press `gesture`. A view
/// without command authority publishes the leaves plainly; nothing yielded is the empty emit (zero trace).
pub(crate) fn space_node_drag_emit(doc: &ArtifactView<'_, WorkflowSnapshot>, verb: &str, gesture: &str, leaves: Vec<WorkflowMutation>) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let authoring_seed = doc.operation_optional().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
    match node_drag_commit(format!("{S_PLAY_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.clone()), gesture, leaves, clock) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, leaves),
        Some((_, leaves)) => Emit::mutations(leaves),
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no `interaction`
/// slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only through that macro-generated path
/// (`SpaceApp::handle` always routes this command through `apply` below instead), so its `"deleteSelection"` sub-operation
/// degrades to treating the selection as empty; every other sub-operation is unaffected.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(crate::engine::space::engine::resolve_future(edit_with_selection(payload, doc, &[])))
}

pub async fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(edit_with_selection(payload, doc, &interaction.selection("graph").ids).await)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
