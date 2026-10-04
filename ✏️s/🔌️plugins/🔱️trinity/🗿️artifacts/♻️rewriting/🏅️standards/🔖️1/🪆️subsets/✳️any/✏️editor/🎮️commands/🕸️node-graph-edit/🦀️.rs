//! 📜️ Trinity Rewriting app command — `node-graph-edit`. Every row of a `nodeGraphEdit` dispatch — the closed node-graph record
//! vocabulary every renderer sends, decoded by the ONE shared decoder of `🛠️tool-machine` ([`node_graph_edit_rows`], design §13.3 of
//! ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) — becomes an intent leaf of the canvas it was drawn on. On the BEFORE canvas (the
//! working graph) every leaf is a child-lane leaf of the composed `workingGraph` child in the shared `s.stdio.semio@v1/graph`
//! vocabulary (design §20.15): a released drag is `drag-nodes`, a drawn wire `create-edge`, a cut wire `delete-edge`, a delete the
//! `delete-edge`s of every incident wire then `delete-node`, followed by `delete-edge` for the wires it names apart from the deleted
//! nodes'. On a rule side (LHS/RHS) a drag is `drag-rule-nodes` and a delete the clause deletions of its nodes; a rule wire is derived
//! from its clauses, so it is cut with them and never drawn or cut alone. A batch that released a drag commits through the ONE
//! node-drag machine ([`node_drag_emit`]) as ONE `ToolTransaction` — one edit, one history row; every other batch is one plain
//! edit. The tool is never history; its leaves are. Inline sliders and variadic ports do not exist on these graphs, so those rows are
//! refused by name, as is every row but a drag on a read-only canvas (a drag there moves nothing).

use crate::apply_rewrite_rule_mutation;
use crate::content::{admit, read, working_child_emit, WORKING_CHILD_SLOT};
use crate::standards::v1::subsets::any::schema::mutations::drag_rule_nodes;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::app::ChildContentView;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};
use semio_framework_tool_machine::{node_drag_emit, node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{create_edge::CreateEdge, delete_edge::DeleteEdge, delete_node::DeleteNode, drag_nodes::DragNodes, SemioGraphMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphSnapshot};

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

/// 🔌️ The node and port a row endpoint names on the working graph: a bare node for a node-level endpoint, else `node@port`.
fn endpoint(node_id: &str, port_id: &str) -> (GraphNodeId, Option<String>) {
    (GraphNodeId::new(node_id), (!port_id.is_empty()).then(|| port_id.to_string()))
}

/// 🔑️ The key an endpoint is written as in a wire's id: `node@port`, or the bare node.
fn endpoint_key((node, port): &(GraphNodeId, Option<String>)) -> String {
    port.as_ref().map_or_else(|| node.value.clone(), |port| semio_s_artifact_trinity_jack::port_key(&node.value, port))
}

/// 🏷️ The edge kind a wire drawn on the working graph carries: the first edge kind the rule's working-graph manifest declares, else
/// the kind its existing wires carry; `None` when the graph names neither.
fn working_edge_kind(state: &RewritingSnapshot, work: &SemioGraphSnapshot) -> Option<String> {
    state.working_graph.manifest.edge_kinds.first().map(|kind| kind.name.clone()).or_else(|| work.edges.first().map(|edge| edge.kind.clone()))
}

/// 🧮️ What ONE working-canvas row means over the working child `work` the previous rows left: the drag records it releases and
/// the child leaves it makes.
fn working_row(state: &RewritingSnapshot, work: &SemioGraphSnapshot, row: NodeGraphEditRow) -> Result<(Vec<NodeDragRecord>, Vec<SemioGraphMutation>), Fault> {
    Ok(match row {
        NodeGraphEditRow::Move(record) => (vec![record], Vec::new()),
        NodeGraphEditRow::SetSlider { widget_id, .. } => return Err(refuse(format!("the graph has no inline slider {widget_id:?}"))),
        NodeGraphEditRow::InsertPort { node_id, .. } => return Err(refuse(format!("node {node_id:?} has no variadic port"))),
        NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
            let kind = working_edge_kind(state, work).ok_or_else(|| refuse("the working graph names no edge kind a wire could carry"))?;
            let (source, target) = (endpoint(&source_node_id, &source_port_id), endpoint(&target_node_id, &target_port_id));
            let id = GraphEdgeId::new(format!("{}->{}", endpoint_key(&source), endpoint_key(&target)));
            let drawn = (source != target).then(|| SemioGraphMutation::CreateEdge(CreateEdge { id, source: source.0, target: target.0, kind, label: String::new(), source_port: source.1, target_port: target.1, properties: Vec::new(), at: None }));
            (Vec::new(), drawn.into_iter().collect())
        }
        NodeGraphEditRow::Disconnect { synapse_id } => (Vec::new(), vec![SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(synapse_id) })]),
        NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
            let incident = |edge_source: &str, edge_target: &str| node_ids.iter().any(|id| id == edge_source || id == edge_target);
            let mut leaves = Vec::new();
            for node in &node_ids {
                leaves.extend(work.edges.iter().filter(|edge| edge.source.value == *node || edge.target.value == *node).map(|edge| SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge.id.clone() })));
                leaves.push(SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(node.clone()) }));
            }
            let named_apart = synapse_ids.into_iter().filter(|id| !work.edges.iter().any(|edge| edge.id.value == *id && incident(&edge.source.value, &edge.target.value)));
            leaves.extend(named_apart.map(|id| SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(id) })));
            (Vec::new(), leaves)
        }
    })
}

/// 🧮️ What ONE rule-canvas row means over the rule `state` the previous rows left: the drag records it releases and the parent
/// leaves it makes.
fn rule_row(state: &RewritingSnapshot, canvas: Canvas, row: NodeGraphEditRow) -> Result<(Vec<NodeDragRecord>, Vec<RewriteRuleMutation>), Fault> {
    Ok(match (canvas, row) {
        (_, NodeGraphEditRow::Move(record)) => (vec![record], Vec::new()),
        (_, NodeGraphEditRow::SetSlider { widget_id, .. }) => return Err(refuse(format!("the graph has no inline slider {widget_id:?}"))),
        (_, NodeGraphEditRow::InsertPort { node_id, .. }) => return Err(refuse(format!("node {node_id:?} has no variadic port"))),
        (Canvas::Rule, NodeGraphEditRow::Delete { node_ids, .. }) if !node_ids.is_empty() => (Vec::new(), crate::editor::rewriting::delete_rule_clause::delete_rule_clauses(state, &node_ids)),
        (Canvas::Rule, NodeGraphEditRow::Connect { .. } | NodeGraphEditRow::Disconnect { .. } | NodeGraphEditRow::Delete { .. }) => return Err(refuse("a rule wire is derived from its clauses: edit or delete the clause it joins")),
        _ => return Err(refuse("the canvas is read-only")),
    })
}

/// 🕹️ The emit of one working-canvas batch: every row is read on the working child the previous ones left and a leaf the child
/// refuses is dropped; a released drag commits all leaves as ONE child tool transaction.
fn working_graph_edit(state: &RewritingSnapshot, children: &ChildContentView, rows: Vec<NodeGraphEditRow>, authoring_seed: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let mut work = SemioGraphSnapshot::clone(&*read(state, children)?);
    let (mut records, mut leaves) = (Vec::new(), Vec::new());
    for row in rows {
        let (released, made) = working_row(state, &work, row)?;
        let drags = released.iter().filter(|record| record.moves()).map(|record| SemioGraphMutation::DragNodes(DragNodes { targets: record.node_ids.iter().map(|id| GraphNodeId::new(id.clone())).collect(), dx: record.dx, dy: record.dy }));
        for leaf in drags.chain(made).collect::<Vec<_>>() {
            if admit(&mut work, &leaf) {
                leaves.push(leaf);
            }
        }
        records.extend(released);
    }
    let Some(gesture) = records.first().map(|record| record.gesture_id.clone()) else {
        return Ok(working_child_emit(state, &leaves));
    };
    Ok(Emit::node_drag_child::<SemioGraphSnapshot, SemioGraphMutation>(node_drag_emit(TRINITY_REWRITING_EDITOR_APP_ID, "nodeGraphEdit", authoring_seed, &gesture, leaves), WORKING_CHILD_SLOT, &state.working_graph.content.child_id))
}

/// 🕹️ The emit of one `nodeGraphEdit` batch on the canvas `surface_id`: the rows (`operations_json`, the JSON array the host sent)
/// decode as one closed batch — any malformed row refuses all of them. The working canvas edits the composed `workingGraph` child
/// (read through `children`); a rule canvas reads every row on the rule the previous ones left and drops a leaf the running rule
/// refuses. A batch that released a drag is ONE tool transaction of all its leaves, the ref minted from the admission's
/// `authoring_seed`, the host clock and `<appId>#nodeGraphEdit`, the press named by the first record; a view without command
/// authority (no seed) publishes the leaves plainly; a batch without a drag is a plain edit.
pub(crate) fn node_graph_edit(state: &RewritingSnapshot, children: &ChildContentView, surface_id: &str, operations_json: &str, authoring_seed: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let operations = semio_framework_pack_json::parse(operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| refuse(format!("the operations are not JSON: {error}")))?;
    let rows = node_graph_edit_rows(&semio_framework_value::DslValue::object([("operations".into(), semio_framework_pack_json::to_dsl_value(&operations))])).map_err(refuse)?;
    let canvas = Canvas::of(surface_id);
    if canvas == Canvas::Working {
        return working_graph_edit(state, children, rows, authoring_seed);
    }
    let mut running = state.clone();
    let (mut records, mut leaves) = (Vec::new(), Vec::new());
    for row in rows {
        let (released, made) = rule_row(&running, canvas, row)?;
        let drags = released.iter().filter(|record| canvas == Canvas::Rule && record.moves()).map(|record| drag_rule_nodes(record.node_ids.clone(), record.dx, record.dy));
        for leaf in drags.chain(made).collect::<Vec<_>>() {
            if apply_rewrite_rule_mutation(&mut running, &leaf).is_ok() {
                leaves.push(leaf);
            }
        }
        records.extend(released);
    }
    let Some(gesture) = records.first().map(|record| record.gesture_id.clone()) else {
        return Ok(Emit::mutations(leaves));
    };
    Ok(node_drag_emit(TRINITY_REWRITING_EDITOR_APP_ID, "nodeGraphEdit", authoring_seed, &gesture, leaves).into())
}
