//! 🕸️ 🕸️ DAG play app commands command — `node-graph-edit`: the one verb both node-graph hosts dispatch for graph edits.

use crate::editor::dag::commands::delete_selection::delete_selection_result;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::editor::dag::DAG_PLAY_APP_ID;
use crate::mutations::{connect_nodes, dag_snapshot_mutations, disconnect_nodes, move_nodes, set_slider, DagSliderField};
use crate::op::DagMutation;
use crate::{DagNodeKind, DagSnapshot};
use semio_framework_artifact_infinite_dag::{dag_document_from_host_snapshot, DagHostSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord, NODE_DRAG_OPERATION};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 🎯️ One batched edit inside a `NodeGraphEdit`, closed and typed — the host wire rows of the shared `nodeGraphEdit`
/// vocabulary (`setHostSnapshot`, `deleteSelection`, `connect`, `disconnect`, the node-graph gesture record `move`, and the
/// slider overlay's `setSlider`).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub enum DagNodeGraphEditOp {
    #[dsl(key = "set-host-snapshot")]
    SetHostSnapshot { host_snapshot_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect")]
    Disconnect { synapse_id: String },
    #[dsl(key = "move")]
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    #[dsl(key = "set-slider")]
    SetSlider { widget_id: String, value: f64 },
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<DagNodeGraphEditOp>,
}

impl NodeGraphEdit {
    /// 🌉️ Decodes the `{operations: [...]}` arguments a host dispatches (a press's top-level `gesture`/`commit`/`abort`
    /// belong to the framework scrub machine and are not read here). An unknown operation or a malformed row is refused by
    /// name; a `move` row is exactly the node-graph gesture record.
    pub fn from_action_args(args: Option<&dsl::DslValue>) -> Result<Self, Fault> {
        let refuse = |message: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("dag.node-graph-edit.malformed"), message);
        let rows = match args.and_then(|args| args.get("operations")) {
            Some(dsl::DslValue::Array(rows)) => rows.as_slice(),
            None => &[],
            Some(_) => return Err(refuse("nodeGraphEdit operations must be an array".into())),
        };
        let text = |row: &dsl::DslValue, key: &str| row.get(key).and_then(dsl::DslValue::as_str).filter(|value| !value.is_empty()).map(str::to_string).ok_or_else(|| refuse(format!("a nodeGraphEdit row needs a non-empty {key}")));
        let operations = rows
            .iter()
            .map(|row| match row.get("operation").and_then(dsl::DslValue::as_str).unwrap_or("") {
                "setHostSnapshot" => Ok(DagNodeGraphEditOp::SetHostSnapshot { host_snapshot_json: text(row, "hostSnapshotJson")? }),
                "deleteSelection" => Ok(DagNodeGraphEditOp::DeleteSelection),
                "connect" => Ok(DagNodeGraphEditOp::Connect { source_node_id: text(row, "sourceNodeId")?, source_port_id: text(row, "sourcePortId")?, target_node_id: text(row, "targetNodeId")?, target_port_id: text(row, "targetPortId")? }),
                "disconnect" => Ok(DagNodeGraphEditOp::Disconnect { synapse_id: text(row, "synapseId")? }),
                NODE_DRAG_OPERATION => NodeDragRecord::from_row(row).map(|record| DagNodeGraphEditOp::Move { gesture_id: record.gesture_id, node_ids: record.node_ids, dx: record.dx, dy: record.dy }).map_err(refuse),
                "setSlider" => {
                    let value = row.get("value").and_then(dsl::DslValue::as_f64).filter(|value| value.is_finite()).ok_or_else(|| refuse("a setSlider row's value is a finite number".into()))?;
                    Ok(DagNodeGraphEditOp::SetSlider { widget_id: text(row, "widgetId")?, value })
                }
                other => Err(refuse(format!("nodeGraphEdit has no operation {other:?}"))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { operations })
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no
/// `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only
/// through that macro-generated path (`DagPlayApp::handle` always routes this command through `apply`
/// below instead), so its `DeleteSelection` sub-op degrades to treating the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(apply_to(payload, doc, cfg, &[]))
}

pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(apply_to(payload, doc, cfg, &interaction.selection("graph").ids))
}

/// 🧵️ The retained-tool twin of [`apply`] — see `delete_selection::apply_with_state` for why the raw
/// `protocol::InteractionState` is read instead of an `InteractionView`.
pub fn apply_with_state(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, interaction: &protocol::InteractionState) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let selected = interaction.selection.get("graph").map(|domain| domain.ids.clone()).unwrap_or_default();
    Ok(apply_to(payload, doc, cfg, &selected))
}

/// 🧾️ Every row yields its leaf against the committed document:
/// - `move` — the RELATIVE `move-nodes` leaf over the record's nodes the graph holds, committed through the ONE node-drag
///   machine as ONE tool transaction (design §13.3);
/// - `setHostSnapshot` — the structural diff of the graph it leaves behind, its position changes as intent (`move-nodes`
///   per drag offset); a snapshot that moved nodes is a drag and commits as ONE tool transaction too;
/// - `setSlider` — the ABSOLUTE `set-slider` value leaf; a dragged knob carries its press as the dispatch's top-level
///   `gesture`/`commit`, so the framework scrub machine keeps every tick provisional and commits the release as ONE edit;
/// - `connect`, `disconnect`, `deleteSelection` — one-shot structural intents.
fn apply_to(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, selected: &[String]) -> Emit<DagMutation, DagConfigMutation> {
    let document = doc.snapshot;
    let nodes = document.nodes();
    let mut artifact_mutations: Vec<DagMutation> = Vec::new();
    let mut config_mutations: Vec<DagConfigMutation> = Vec::new();
    let mut gesture: Option<String> = None;
    for sub_operation in &payload.operations {
        match sub_operation {
            DagNodeGraphEditOp::SetHostSnapshot { host_snapshot_json } => {
                if let Ok(fixture) = dsl::json::from_json_str::<DagHostSnapshot>(host_snapshot_json) {
                    config_mutations.push(DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom }));
                    let leaves = dag_snapshot_mutations(document, &dag_document_from_host_snapshot(&fixture).into());
                    if leaves.iter().any(|leaf| matches!(leaf, DagMutation::MoveNodes(_))) {
                        gesture.get_or_insert_with(|| "setHostSnapshot".to_string());
                    }
                    artifact_mutations.extend(leaves);
                }
            }
            DagNodeGraphEditOp::DeleteSelection => {
                if let Some(removes) = delete_selection_result(document, selected) {
                    artifact_mutations.extend(removes);
                }
            }
            DagNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                if let Ok(edge) = crate::schema::connect_edge(document, source_node_id, source_port_id, target_node_id, target_port_id) {
                    artifact_mutations.push(connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties));
                }
            }
            DagNodeGraphEditOp::Disconnect { synapse_id } => {
                if document.edges().iter().any(|edge| &edge.id == synapse_id) {
                    artifact_mutations.push(disconnect_nodes(synapse_id.clone()));
                }
            }
            DagNodeGraphEditOp::Move { gesture_id, node_ids, dx, dy } => {
                let record = NodeDragRecord { gesture_id: gesture_id.clone(), node_ids: node_ids.iter().filter(|id| nodes.iter().any(|node| &node.id == *id)).cloned().collect(), dx: *dx, dy: *dy };
                if record.moves() {
                    gesture.get_or_insert_with(|| record.gesture_id.clone());
                    artifact_mutations.push(move_nodes(record.node_ids, record.dx, record.dy));
                }
            }
            DagNodeGraphEditOp::SetSlider { widget_id, value } => {
                if nodes.iter().any(|node| &node.id == widget_id && matches!(node.kind, DagNodeKind::Slider { value: current, .. } if current != *value)) {
                    artifact_mutations.push(set_slider(widget_id.clone(), DagSliderField::Value, *value));
                }
            }
        }
    }
    let emit = match gesture {
        Some(gesture) => dag_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, &gesture, artifact_mutations),
        None => Emit::mutations(artifact_mutations),
    };
    Emit { config_mutations, ..emit }
}

/// 🛠️ ONE tool transaction of `leaves` through the ONE node-drag machine of `🛠️tool-machine` (design §13.3): the ref
/// minted from the admission's authoring seed, the host clock and `<appId>#<verb>`, for the press `gesture`. A view
/// without command authority publishes the leaves plainly; nothing yielded is the empty emit (zero trace).
pub(crate) fn dag_node_drag_emit(doc: &ArtifactView<'_, DagSnapshot>, verb: &str, gesture: &str, leaves: Vec<DagMutation>) -> Emit<DagMutation, DagConfigMutation> {
    let authoring_seed = doc.operation_optional().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
    match node_drag_commit(format!("{DAG_PLAY_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.clone()), gesture, leaves, clock) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, leaves),
        Some((_, leaves)) => Emit::mutations(leaves),
        None => Emit::default(),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
