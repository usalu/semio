//! 📜️ 📜️ Trinity Rewrite app command — `node-graph-edit`.

use crate::artifacts::jack::{Graph, JackSnapshot};
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::schema::Rhs;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};
use serde_json::Value;

/// 🧭️ One addressable rule-clause node in the LHS/RHS semantic graphs (`lhs-where`, `rhs-create-N`,
/// `rhs-merge-N`, `rhs-set-N`, `rhs-delete-N`, `rhs-parameter-N`) — parsed back from its synthetic
/// node id by `parse_clause_ref`.
enum RuleClauseRef {
    LhsWhere,
    RhsCreate(usize),
    RhsMerge(usize),
    RhsSet(usize),
    RhsDelete(usize),
    RhsParameter(usize),
}

async fn parse_fixture_json(json: &str) -> Option<JackSnapshot> {
    JackSnapshot::from_json(json).ok()
}
async fn apply_semantic_layout_edit(rule_layout: &mut std::collections::BTreeMap<String, crate::artifacts::rewrite::LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
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
            rule_layout.insert(node.id.clone(), crate::artifacts::rewrite::LayoutPoint { x: node.x, y: node.y });
            changed = true;
        }
    }
    changed
}
async fn parse_clause_ref(node_id: &str) -> Option<RuleClauseRef> {
    if node_id == "lhs-where" {
        return Some(RuleClauseRef::LhsWhere);
    }
    let (prefix, index) = node_id.rsplit_once('-')?;
    let index: usize = index.parse().ok()?;
    match prefix {
        "rhs-create" => Some(RuleClauseRef::RhsCreate(index)),
        "rhs-merge" => Some(RuleClauseRef::RhsMerge(index)),
        "rhs-set" => Some(RuleClauseRef::RhsSet(index)),
        "rhs-delete" => Some(RuleClauseRef::RhsDelete(index)),
        "rhs-parameter" => Some(RuleClauseRef::RhsParameter(index)),
        _ => None,
    }
}
async fn remove_at<T>(items: &mut Vec<T>, index: usize) -> bool {
    if index < items.len() {
        items.remove(index);
        true
    } else {
        false
    }
}
async fn delete_rule_clause(state: &mut RewriteSnapshot, node_id: &str) -> bool {
    let Some(clause_ref) = parse_clause_ref(node_id) else {
        return false;
    };
    let Ok(mut lhs) = serde_json::from_str::<crate::artifacts::rewrite::schema::Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let changed = match clause_ref {
        RuleClauseRef::LhsWhere => {
            let had = lhs.where_clause.is_some();
            lhs.where_clause = None;
            had
        }
        RuleClauseRef::RhsCreate(index) => remove_at(&mut rhs.create, index),
        RuleClauseRef::RhsMerge(index) => remove_at(&mut rhs.merge, index),
        RuleClauseRef::RhsSet(index) => remove_at(&mut rhs.set, index),
        RuleClauseRef::RhsDelete(index) => remove_at(&mut rhs.delete, index),
        RuleClauseRef::RhsParameter(index) => {
            if index < rhs.parameters.len() {
                let removed = rhs.parameters.remove(index);
                state.parameter_bindings.remove(&removed.name);
                true
            } else {
                false
            }
        }
    };
    if changed {
        state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
        state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
        state.rule_layout.remove(node_id);
    }
    changed
}
async fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteSnapshot, selected_node_ids: &[String], surface_id: &str, operations: &[Value]) -> bool {
    let mut changed = false;
    for operation in operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) else {
                    continue;
                };
                if parse_fixture_json(fixture_json).is_none() {
                    continue;
                }
                if surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    state.before_fixture_json = fixture_json.into();
                    changed = true;
                } else if surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS {
                    let current = crate::editor::rewrite::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                } else if surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let current = crate::editor::rewrite::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                }
            }
            "deleteSelection" => {
                if selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    if let Some(fixture) = parse_fixture_json(&state.before_fixture_json) {
                        let mut nodes = fixture.nodes();
                        nodes.retain(|node| !selected_node_ids.contains(&node.id));
                        let mut edges = fixture.edges();
                        edges.retain(|edge| {
                            let from = crate::artifacts::jack::port_node_id(&edge.source).unwrap_or(&edge.source);
                            let to = crate::artifacts::jack::port_node_id(&edge.target).unwrap_or(&edge.target);
                            !selected_node_ids.iter().any(|id| id == from || id == to)
                        });
                        let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, edges, fixture.root_node_id.clone());
                        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                            state.before_fixture_json = json;
                            changed = true;
                        }
                    }
                } else if surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let mut deleted = false;
                    for id in selected_node_ids {
                        deleted |= delete_rule_clause(state, id);
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
pub(crate) async fn node_graph_edit(state: &RewriteSnapshot, selected_node_ids: &[String], surface_id: &str, operations_json: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
    let mut next = state.clone();
    let changed = apply_rewrite_node_graph_edit_operations(&mut next, selected_node_ids, surface_id, &operations);
    if !changed {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: rewrite_snapshot_mutations(state, &next), ..Default::default() })
}
