//! 📜️ Trinity Rewrite app — document-mutating rule commands (`nodeGraphEdit`, `setLhsJson`,
//! `setRhsJson`, `setParameter`, `addRuleClause`, `resetRule`, `patchNodes`) — dispatched as VCS
//! operations with a true inverse (every mutation flows through the single LWW `SetState`).

use crate::apps::rewrite::config::RewriteConfigOperation;
use crate::artifacts::jack::{Graph, GraphFixture, PropertyValue};
use crate::artifacts::rewrite::engine::{ParameterKind, Rhs};
use crate::artifacts::rewrite::op::RewriteRuleOperation;
use crate::artifacts::rewrite::RewriteRuleState;
use semio_framework_plugin::{Emit, Fault};
use serde_json::Value;

fn parse_fixture_json(json: &str) -> Option<GraphFixture> {
    GraphFixture::from_json(json).ok()
}

fn apply_semantic_layout_edit(rule_layout: &mut std::collections::BTreeMap<String, crate::artifacts::rewrite::LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
    let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
        return false;
    };
    let mut changed = false;
    for node in &edited.nodes {
        let Some(prev) = current.nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            rule_layout.insert(node.id.clone(), crate::artifacts::rewrite::LayoutPoint { x: node.x, y: node.y });
            changed = true;
        }
    }
    changed
}

enum RuleClauseRef {
    LhsWhere,
    RhsCreate(usize),
    RhsMerge(usize),
    RhsSet(usize),
    RhsDelete(usize),
    RhsParameter(usize),
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

pub(crate) fn delete_rule_clause(state: &mut RewriteRuleState, node_id: &str) -> bool {
    let Some(clause_ref) = parse_clause_ref(node_id) else {
        return false;
    };
    let Ok(mut lhs) = serde_json::from_str::<crate::artifacts::rewrite::engine::Lhs>(&state.lhs_json) else {
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

/// ➕️ Appends a default instance of `clause_kind` to the rule (rewrite.where/create/merge/set/delete/parameter).
fn add_rule_clause(state: &mut RewriteRuleState, clause_kind: &str) -> bool {
    let Ok(mut lhs) = serde_json::from_str::<crate::artifacts::rewrite::engine::Lhs>(&state.lhs_json) else {
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
            rhs.create.push(crate::artifacts::rewrite::engine::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "merge" => {
            rhs.merge.push(crate::artifacts::rewrite::engine::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "set" => {
            rhs.set.push(crate::artifacts::rewrite::engine::AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
            true
        }
        "delete" => {
            rhs.delete.push(left_var);
            true
        }
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(crate::artifacts::rewrite::engine::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
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

/// 🖊️ Applies node-graph editor operations (drag layout / delete-selection) to `state` in place,
/// returning `(document_changed, should_clear_selection)`.
fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteRuleState, selected_node_ids: &[String], surface_id: &str, operations: &[Value]) -> (bool, bool) {
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
                    if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                        fixture.nodes.retain(|node| !selected_node_ids.contains(&node.id));
                        fixture.edges.retain(|edge| {
                            let from = crate::artifacts::jack::port_node_id(&edge.source).unwrap_or(&edge.source);
                            let to = crate::artifacts::jack::port_node_id(&edge.target).unwrap_or(&edge.target);
                            !selected_node_ids.iter().any(|id| id == from || id == to)
                        });
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
    let mut fixture = GraphFixture::from_json(fixture_json).ok()?;
    for node in fixture.nodes.iter_mut() {
        if !node_ids.iter().any(|id| id == &node.id) {
            continue;
        }
        match field {
            "name" => node.name = value.into(),
            "kind" => node.kind = value.into(),
            _ => {}
        }
    }
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

pub(crate) fn node_graph_edit(state: &RewriteRuleState, selected_node_ids: &[String], surface_id: &str, operations_json: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
    let mut next = state.clone();
    let (changed, clear_selection) = apply_rewrite_node_graph_edit_operations(&mut next, selected_node_ids, surface_id, &operations);
    if !changed {
        return Ok(Emit::default());
    }
    let config_operations = if clear_selection { vec![RewriteConfigOperation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
    Ok(Emit { document_operations: vec![RewriteRuleOperation::SetState { state: next }], config_operations, ..Default::default() })
}

pub(crate) fn set_lhs_json(state: &RewriteRuleState, value: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let mut next = state.clone();
    next.lhs_json = value.to_string();
    if &next == state {
        Ok(Emit::default())
    } else {
        Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
    }
}

pub(crate) fn set_rhs_json(state: &RewriteRuleState, value: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let mut next = state.clone();
    next.rhs_json = value.to_string();
    next.parameter_bindings = crate::apps::rewrite::default_parameter_bindings(&next.rhs_json);
    if &next == state {
        Ok(Emit::default())
    } else {
        Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
    }
}

pub(crate) fn set_parameter(state: &RewriteRuleState, name: &str, value: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    if name.is_empty() {
        return Ok(Emit::default());
    }
    let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return Ok(Emit::default());
    };
    let kind = rhs.parameters.iter().find(|param| param.name == name).map(|param| param.kind.clone());
    let parsed = match kind {
        Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
        Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
        Some(ParameterKind::String) | None => Some(PropertyValue::String(value.to_string())),
    };
    match parsed {
        Some(parsed) => {
            let mut next = state.clone();
            next.parameter_bindings.insert(name.to_string(), parsed);
            if &next == state {
                Ok(Emit::default())
            } else {
                Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
            }
        }
        None => Ok(Emit::default()),
    }
}

pub(crate) fn add_rule_clause_command(state: &RewriteRuleState, kind: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let mut next = state.clone();
    if add_rule_clause(&mut next, kind) {
        Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
    } else {
        Ok(Emit::default())
    }
}

pub(crate) fn reset_rule(state: &RewriteRuleState) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let next = crate::apps::rewrite::default_rule_state();
    let camera = crate::apps::rewrite::seed_before_pane_camera(&next);
    let config_operations = vec![RewriteConfigOperation::SetBeforePaneCamera { camera }];
    if &next == state {
        Ok(Emit::config(config_operations))
    } else {
        Ok(Emit { document_operations: vec![RewriteRuleOperation::SetState { state: next }], config_operations, ..Default::default() })
    }
}

pub(crate) fn patch_nodes(state: &RewriteRuleState, node_ids: &[String], field: &str, value: &str) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, Fault> {
    let trimmed = value.trim();
    if node_ids.is_empty() || field.is_empty() || trimmed.is_empty() {
        return Ok(Emit::default());
    }
    match patch_fixture_nodes(&state.before_fixture_json, node_ids, field, trimmed) {
        Some(patched) => {
            let mut next = state.clone();
            next.before_fixture_json = patched;
            if &next == state {
                Ok(Emit::default())
            } else {
                Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
            }
        }
        None => Ok(Emit::default()),
    }
}
