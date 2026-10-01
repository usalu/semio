//! 📜️ 📜️ Trinity Rewriting app command — `node-graph-edit`.

use semio_s_artifact_trinity_jack::JackWorkingScene;

use crate::apply_rewrite_rule_mutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_rule_layout_point, edit_before_fixture};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use pack::JsonValue as Value;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_trinity_jack::{Graph, JackSnapshot};

fn parse_fixture_json(json: &str) -> Option<JackSnapshot> {
    JackSnapshot::from_json(json).ok()
}
/// 📐️ The `change-rule-layout-point` leaves of a semantic (LHS/RHS) graph the host handed back whole: one per node whose position
/// moved against the graph the window rendered.
fn semantic_layout_mutations(current_fixture_json: &str, edited_fixture_json: &str) -> Vec<RewriteRuleMutation> {
    let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
        return Vec::new();
    };
    let current_nodes = current.nodes();
    edited
        .nodes()
        .into_iter()
        .filter(|node| current_nodes.iter().any(|prev| prev.id == node.id && ((prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6)))
        .map(|node| change_rule_layout_point(node.id, crate::LayoutPoint { x: node.x, y: node.y }))
        .collect()
}

/// 🕸️ The working graph without the selected nodes and every edge touching them, or `None` when it does not decode.
fn working_graph_without(fixture_json: &str, selected_node_ids: &[String]) -> Option<String> {
    let fixture = parse_fixture_json(fixture_json)?;
    let mut nodes = fixture.nodes();
    nodes.retain(|node| !selected_node_ids.contains(&node.id));
    let mut edges = fixture.edges();
    edges.retain(|edge| {
        let from = semio_s_artifact_trinity_jack::port_node_id(&edge.source).unwrap_or(&edge.source);
        let to = semio_s_artifact_trinity_jack::port_node_id(&edge.target).unwrap_or(&edge.target);
        !selected_node_ids.iter().any(|id| id == from || id == to)
    });
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), JackWorkingScene { nodes, edges }, fixture.root_node_id.clone());
    Graph::from_snapshot(fixture).and_then(|graph| graph.host_snapshot_json()).ok()
}

/// 🧮️ The leaves ONE node-graph operation means on `state`.
fn operation_mutations(state: &RewritingSnapshot, selected_node_ids: &[String], surface_id: &str, operation: &Value) -> Vec<RewriteRuleMutation> {
    let before_surface = surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE;
    let rule_surface = surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS || surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_RHS;
    match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
        "setHostSnapshot" => {
            let Some(host_snapshot_json) = operation.get("hostSnapshotJson").and_then(|value| value.as_str()).filter(|json| parse_fixture_json(json).is_some()) else {
                return Vec::new();
            };
            if before_surface {
                return (host_snapshot_json != state.before_fixture_json).then(|| edit_before_fixture(host_snapshot_json.to_string())).into_iter().collect();
            }
            let current = match surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS {
                true => crate::editor::rewriting::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout),
                false => crate::editor::rewriting::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout),
            };
            match rule_surface {
                true => semantic_layout_mutations(&current, host_snapshot_json),
                false => Vec::new(),
            }
        }
        "deleteSelection" if selected_node_ids.is_empty() => Vec::new(),
        "deleteSelection" if before_surface => working_graph_without(&state.before_fixture_json, selected_node_ids).filter(|json| json != &state.before_fixture_json).map(edit_before_fixture).into_iter().collect(),
        "deleteSelection" if rule_surface => crate::editor::rewriting::delete_rule_clause::delete_rule_clauses(state, selected_node_ids),
        _ => Vec::new(),
    }
}

/// 🕹️ The leaves of one `nodeGraphEdit` batch, operation by operation, each read on the rule the previous ones left.
/// `selected_node_ids` comes from `interaction.selection("graph").ids` (framework-owned): the framework re-validates the
/// selection against the fresh topology right after this document dispatch lands, so no selection leaf is emitted.
pub(crate) fn node_graph_edit(state: &RewritingSnapshot, selected_node_ids: &[String], surface_id: &str, operations_json: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    let operations: Vec<Value> = pack::from_json_str(operations_json).unwrap_or_default();
    let mut running = state.clone();
    let mut leaves = Vec::new();
    for operation in &operations {
        for leaf in operation_mutations(&running, selected_node_ids, surface_id, operation) {
            if apply_rewrite_rule_mutation(&mut running, &leaf).is_ok() {
                leaves.push(leaf);
            }
        }
    }
    Emit::mutations(leaves)
}
