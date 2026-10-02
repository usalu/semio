//! 📜️ Trinity Rewriting app command — `node-graph-edit`. Every row of a `nodeGraphEdit` dispatch — the closed node-graph record
//! vocabulary every renderer sends, decoded by the ONE shared decoder of `🛠️tool-machine` ([`node_graph_edit_rows`], design §13.3 of
//! ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) — becomes an intent leaf of the canvas it was drawn on. On the BEFORE canvas (the
//! working graph) a released drag is `drag-working-nodes`, a drawn wire `connect-working-ports`, a cut wire `disconnect-working-edges`,
//! a delete `delete-working-nodes` followed by `disconnect-working-edges` for the wires it names apart from the deleted nodes'. On a
//! rule side (LHS/RHS) a drag is `drag-rule-nodes` and a delete the clause deletions of its nodes; a rule wire is derived from its
//! clauses, so it is cut with them and never drawn or cut alone. A batch that released a drag commits through the ONE node-drag
//! machine ([`node_drag_commit`]) as ONE `ToolTransaction` — one edit, one history row; every other batch is one plain edit. The
//! tool is never history; its leaves are. Inline sliders and variadic ports do not exist on these graphs, so those rows are refused
//! by name, as is every row but a drag on a read-only canvas (a drag there moves nothing).

use crate::apply_rewrite_rule_mutation;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::mutations::{connect_working_ports, delete_working_nodes, disconnect_working_edges, drag_rule_nodes, drag_working_nodes};
use crate::RewritingSnapshot;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_framework_tool_machine::{node_drag_commit, node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow};
use semio_s_artifact_trinity_jack::{port_key, port_node_id, JackSnapshot};

/// 🪪️ The editor app id every drag transaction's `tool` is scoped by: `<appId>#nodeGraphEdit`.
pub(crate) const TRINITY_REWRITING_EDITOR_APP_ID: &str = "s.trinity.rewriting@1/*#editor";

/// 🖼️ The canvas a row was drawn on: the editable working graph, an editable rule side, or a read-only view.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Canvas {
    Working,
    Rule,
    ReadOnly,
}

impl Canvas {
    fn of(surface_id: &str) -> Self {
        match surface_id {
            crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE => Self::Working,
            crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS | crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_RHS => Self::Rule,
            _ => Self::ReadOnly,
        }
    }
}

/// 🚫️ The named refusal of one row this command cannot map.
fn refuse(reason: impl std::fmt::Display) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("trinity.rewriting.node-graph.row"), format!("nodeGraphEdit refusal: {reason}"))
}

/// 🔌️ The port key a row endpoint names on the working graph: `node@port`, or the bare node for a node-level endpoint.
fn endpoint(node_id: &str, port_id: &str) -> String {
    match port_id.is_empty() {
        true => node_id.to_string(),
        false => port_key(node_id, port_id),
    }
}

/// 🏷️ The edge kind a wire drawn on the working graph carries: the first edge kind its resolved manifest (embedded, else named by
/// `manifestId`) declares, else the kind its existing wires carry; `None` when the graph names neither.
fn working_edge_kind(fixture_json: &str) -> Option<String> {
    let mut graph = JackSnapshot::from_json(fixture_json).ok()?;
    let declared = graph.resolve_manifest().ok().and_then(|_| graph.manifest.edge_kinds.first().map(|kind| kind.name.clone()));
    declared.or_else(|| graph.edges().first().map(|edge| edge.kind.clone()))
}

/// 🪚️ The wires of a working-graph delete that survive its node deletion: every named wire none of whose endpoints is a deleted node.
fn surviving_wires(fixture_json: &str, node_ids: &[String], synapse_ids: Vec<String>) -> Vec<String> {
    let Ok(graph) = JackSnapshot::from_json(fixture_json) else { return synapse_ids };
    let edges = graph.edges();
    let deleted = |key: &str| node_ids.iter().any(|id| id == port_node_id(key).unwrap_or(key));
    synapse_ids.into_iter().filter(|id| !edges.iter().any(|edge| &edge.id == id && (deleted(&edge.source) || deleted(&edge.target)))).collect()
}

/// ✋️ The relative leaf one released drag means on `canvas`; nothing on a read-only canvas or for a record that moves nothing.
fn drag_leaf(canvas: Canvas, record: &NodeDragRecord) -> Option<RewriteRuleMutation> {
    match (canvas, record.moves()) {
        (Canvas::Working, true) => Some(drag_working_nodes(record.node_ids.clone(), record.dx, record.dy)),
        (Canvas::Rule, true) => Some(drag_rule_nodes(record.node_ids.clone(), record.dx, record.dy)),
        _ => None,
    }
}

/// 🧮️ What ONE row means on `canvas` over the rule `state` the previous rows left: the drag records it releases and the other leaves
/// it makes.
fn row_effect(state: &RewritingSnapshot, canvas: Canvas, row: NodeGraphEditRow) -> Result<(Vec<NodeDragRecord>, Vec<RewriteRuleMutation>), Fault> {
    Ok(match (canvas, row) {
        (_, NodeGraphEditRow::Move(record)) => (vec![record], Vec::new()),
        (_, NodeGraphEditRow::SetSlider { widget_id, .. }) => return Err(refuse(format!("the graph has no inline slider {widget_id:?}"))),
        (_, NodeGraphEditRow::InsertPort { node_id, .. }) => return Err(refuse(format!("node {node_id:?} has no variadic port"))),
        (Canvas::ReadOnly, _) => return Err(refuse("the canvas is read-only")),
        (Canvas::Working, NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id }) => {
            let kind = working_edge_kind(&state.before_fixture_json).ok_or_else(|| refuse("the working graph names no edge kind a wire could carry"))?;
            (Vec::new(), vec![connect_working_ports(endpoint(&source_node_id, &source_port_id), endpoint(&target_node_id, &target_port_id), kind)])
        }
        (Canvas::Working, NodeGraphEditRow::Disconnect { synapse_id }) => (Vec::new(), vec![disconnect_working_edges(vec![synapse_id])]),
        (Canvas::Working, NodeGraphEditRow::Delete { node_ids, synapse_ids }) => {
            let wires = surviving_wires(&state.before_fixture_json, &node_ids, synapse_ids);
            let nodes = (!node_ids.is_empty()).then(|| delete_working_nodes(node_ids));
            (Vec::new(), nodes.into_iter().chain((!wires.is_empty()).then(|| disconnect_working_edges(wires))).collect())
        }
        (Canvas::Rule, NodeGraphEditRow::Delete { node_ids, .. }) if !node_ids.is_empty() => (Vec::new(), crate::editor::rewriting::delete_rule_clause::delete_rule_clauses(state, &node_ids)),
        (Canvas::Rule, NodeGraphEditRow::Connect { .. } | NodeGraphEditRow::Disconnect { .. } | NodeGraphEditRow::Delete { .. }) => return Err(refuse("a rule wire is derived from its clauses: edit or delete the clause it joins")),
    })
}

/// ⏰️ The host clock a drag commit runs on, so a transaction id minted at its upsert is unique per admission AND per moment.
fn drag_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🕹️ The emit of one `nodeGraphEdit` batch on the canvas `surface_id`: the rows (`operations_json`, the JSON array the host sent)
/// decode as one closed batch — any malformed row refuses all of them — and every row is read on the rule the previous ones left;
/// a leaf the running rule refuses is dropped. A batch that released a drag is ONE tool transaction of all its leaves, the ref
/// minted from the admission's `authoring_seed`, the host clock and `<appId>#nodeGraphEdit`, the press named by the first record; a
/// view without command authority (no seed) publishes the leaves plainly; a batch without a drag is a plain edit.
pub(crate) fn node_graph_edit(state: &RewritingSnapshot, surface_id: &str, operations_json: &str, authoring_seed: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let operations = pack::parse_json(operations_json).map_err(|error| refuse(format!("the operations are not JSON: {error}")))?;
    let rows = node_graph_edit_rows(&protocol::DslValue::object([("operations".into(), pack::json_to_dsl_value(&operations))])).map_err(refuse)?;
    let canvas = Canvas::of(surface_id);
    let mut running = state.clone();
    let (mut records, mut leaves) = (Vec::new(), Vec::new());
    for row in rows {
        let (released, made) = row_effect(&running, canvas, row)?;
        for leaf in released.iter().filter_map(|record| drag_leaf(canvas, record)).chain(made) {
            if apply_rewrite_rule_mutation(&mut running, &leaf).is_ok() {
                leaves.push(leaf);
            }
        }
        records.extend(released);
    }
    let Some(gesture) = records.first().map(|record| record.gesture_id.clone()) else {
        return Ok(Emit::mutations(leaves));
    };
    let Some((transaction, leaves)) = node_drag_commit(format!("{TRINITY_REWRITING_EDITOR_APP_ID}#nodeGraphEdit"), protocol::ActorId(authoring_seed.to_string()), &gesture, leaves, drag_tool_clock()) else {
        return Ok(Emit::default());
    };
    Ok(match authoring_seed.is_empty() {
        true => Emit::mutations(leaves),
        false => Emit::commit_transaction(transaction, leaves),
    })
}
