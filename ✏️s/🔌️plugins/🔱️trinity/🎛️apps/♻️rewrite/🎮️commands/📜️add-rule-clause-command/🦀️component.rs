//! 📜️ 📜️ Trinity Rewrite app command — `add-rule-clause-command`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::{Graph, JackSnapshot, PropertyValue};
use crate::artifacts::rewrite::schema::{ParameterKind, Rhs};
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};
use serde_json::Value;

fn parse_fixture_json(json: &str) -> Option<JackSnapshot> {
    JackSnapshot::from_json(json).ok()
}
fn apply_semantic_layout_edit(rule_layout: &mut std::collections::BTreeMap<String, crate::artifacts::rewrite::LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
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
fn parse_clause_ref(node_id: &str) -> Option<RuleClauseRef> {
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
fn remove_at<T>(items: &mut Vec<T>, index: usize) -> bool {
    if index < items.len() {
        items.remove(index);
        true
    } else {
        false
    }
}
fn add_rule_clause(state: &mut RewriteSnapshot, clause_kind: &str) -> bool {
    let Ok(mut lhs) = serde_json::from_str::<crate::artifacts::rewrite::schema::Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let left_var = lhs.pattern.left_var.clone();
    let changed = match clause_kind {
        "where" => {
            if lhs.where_clause.is_some() {
                false
            } else {
                lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
                true
            }
        }
        "create" => {
            rhs.create.push(crate::artifacts::rewrite::schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "merge" => {
            rhs.merge.push(crate::artifacts::rewrite::schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "set" => {
            rhs.set.push(crate::artifacts::rewrite::schema::AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
            true
        }
        "delete" => {
            rhs.delete.push(left_var);
            true
        }
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(crate::artifacts::rewrite::schema::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
            true
        }
        _ => false,
    };
    if changed {
        state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
        state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
    }
    changed
}
fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteSnapshot, selected_node_ids: &[String], surface_id: &str, operations: &[Value]) -> (bool, bool) {
    let mut changed = false;
    let mut clear_selection = false;
    for operation in operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) else {
                    continue;
                };
                if parse_fixture_json(fixture_json).is_none() {
                    continue;
                }
                if surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    state.before_fixture_json = fixture_json.into();
                    changed = true;
                } else if surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS {
                    let current = crate::apps::rewrite::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                } else if surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let current = crate::apps::rewrite::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                }
            }
            "deleteSelection" => {
                if selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
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
                            clear_selection = true;
                            changed = true;
                        }
                    }
                } else if surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let mut deleted = false;
                    for id in selected_node_ids {
                        deleted |= delete_rule_clause(state, id);
                    }
                    if deleted {
                        clear_selection = true;
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    (changed, clear_selection)
}
fn patch_fixture_nodes(fixture_json: &str, node_ids: &[String], field: &str, value: &str) -> Option<String> {
    let fixture = JackSnapshot::from_json(fixture_json).ok()?;
    let mut nodes = fixture.nodes();
    for node in nodes.iter_mut() {
        if !node_ids.iter().any(|id| id == &node.id) {
            continue;
        }
        match field {
            "name" => node.name = value.into(),
            "kind" => node.kind = value.into(),
            _ => {}
        }
    }
    let fixture = JackSnapshot::with_content(fixture.schema.clone(), fixture.name.clone(), fixture.manifest_id.clone(), fixture.manifest.clone(), fixture.camera.clone(), nodes, fixture.edges(), fixture.root_node_id.clone());
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

pub(crate) fn add_rule_clause_command(state: &RewriteSnapshot, kind: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut next = state.clone();
    if add_rule_clause(&mut next, kind) {
        Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
    } else {
        Ok(Emit::default())
    }
}
