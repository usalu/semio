//! 📜️ 📜️ Trinity Rewriting app command — `node-graph-edit`.

use semio_s_artifact_trinity_jack::JackWorkingScene;

use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use pack::JsonValue as Value;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_trinity_jack::{Graph, JackSnapshot};

fn parse_fixture_json(json: &str) -> Option<JackSnapshot> {
    JackSnapshot::from_json(json).ok()
}
fn apply_semantic_layout_edit(rule_layout: &mut std::collections::BTreeMap<String, crate::LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
    let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
        return false;
    };
    let mut changed = false;
    let edited_nodes = edited.nodes();
    let current_nodes = current.nodes();
    for node in &edited_nodes {
        let Some(prev) = current_nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            rule_layout.insert(node.id.clone(), crate::LayoutPoint { x: node.x, y: node.y });
            changed = true;
        }
    }
    changed
}
fn apply_rewriting_node_graph_edit_operations(state: &mut RewritingSnapshot, selected_node_ids: &[String], surface_id: &str, operations: &[Value]) -> bool {
    let mut changed = false;
    for operation in operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setHostSnapshot" => {
                let Some(host_snapshot_json) = operation.get("hostSnapshotJson").and_then(|value| value.as_str()) else {
                    continue;
                };
                if parse_fixture_json(host_snapshot_json).is_none() {
                    continue;
                }
                if surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE {
                    state.before_fixture_json = host_snapshot_json.into();
                    changed = true;
                } else if surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS {
                    let current = crate::editor::rewriting::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, host_snapshot_json);
                } else if surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_RHS {
                    let current = crate::editor::rewriting::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, host_snapshot_json);
                }
            }
            "deleteSelection" => {
                if selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE {
                    if let Some(fixture) = parse_fixture_json(&state.before_fixture_json) {
                        let mut nodes = fixture.nodes();
                        nodes.retain(|node| !selected_node_ids.contains(&node.id));
                        let mut edges = fixture.edges();
                        edges.retain(|edge| {
                            let from = semio_s_artifact_trinity_jack::port_node_id(&edge.source).unwrap_or(&edge.source);
                            let to = semio_s_artifact_trinity_jack::port_node_id(&edge.target).unwrap_or(&edge.target);
                            !selected_node_ids.iter().any(|id| id == from || id == to)
                        });
                        let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), JackWorkingScene { nodes: nodes, edges: edges }, fixture.root_node_id.clone());
                        if let Ok(json) = Graph::from_snapshot(fixture).and_then(|graph| graph.host_snapshot_json()) {
                            state.before_fixture_json = json;
                            changed = true;
                        }
                    }
                } else if surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS || surface_id == crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_RHS {
                    let mut deleted = false;
                    for id in selected_node_ids {
                        deleted |= crate::editor::rewriting::delete_rule_clause::delete_rule_clause(state, id);
                    }
                    if deleted {
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    changed
}
/// 🕹️ `selected_node_ids` now comes from `interaction.selection("graph").ids` (framework-owned) —
/// deleting a selected id here is enough on its own: the framework re-validates/prunes the "graph"
/// domain's selection against the fresh `interaction_topology` right after this document dispatch
/// lands, so no explicit selection-clearing mutation is emitted anymore.
pub(crate) fn node_graph_edit(state: &RewritingSnapshot, selected_node_ids: &[String], surface_id: &str, operations_json: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    let operations: Vec<Value> = pack::from_json_str(operations_json).unwrap_or_default();
    let mut next = state.clone();
    let changed = apply_rewriting_node_graph_edit_operations(&mut next, selected_node_ids, surface_id, &operations);
    if !changed {
        return Emit::default();
    }
    Emit { artifact_mutations: rewriting_snapshot_mutations(state, &next), ..Default::default() }
}
